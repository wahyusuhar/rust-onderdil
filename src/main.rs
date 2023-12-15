use std::io;

#[derive(Debug)]
struct Onderdil {
    id:  u32,
    nama: String,
    harga: f64,
}

struct Database {
    onderdils: Vec<Onderdil>,
}

impl Database {
    fn new() -> Database {
        Database { onderdils: Vec::new() }
    }

    fn tambah_onderdil(&mut self, onderdil: Onderdil) {
        self.onderdils.push(onderdil);
    }

    fn lihat_onderdil(&self) {
        for onderdil in &self.onderdils {
            println!("{:?}", onderdil);
        }
    }

    fn edit_onderdil(&mut self, id: u32, nama_baru: String, harga_baru: f64) {
        for onderdil in &mut self.onderdils {
            if onderdil.id == id {
                onderdil.nama = nama_baru;
                onderdil.harga = harga_baru;
                println!("Onderdil berhasil diubah.");
                return;
            }
        }
        println!("Onderdil dengan ID {} tidak ditemukan.", id);
    }

    fn hapus_onderdil(&mut self, id: u32) {
        self.onderdils.retain(|onderdil| onderdil.id != id);
        println!("Onderdil dengan ID {} berhasil dihapus.", id);
    }
}

fn main() {
    let mut database = Database::new();
println!("
██████╗ ███╗   ██╗██████╗ ███████╗██████╗ ██████╗ ██╗██╗     
██╔═══██╗████╗  ██║██╔══██╗██╔════╝██╔══██╗██╔══██╗██║██║     
██║   ██║██╔██╗ ██║██║  ██║█████╗  ██████╔╝██║  ██║██║██║     
██║   ██║██║╚██╗██║██║  ██║██╔══╝  ██╔══██╗██║  ██║██║██║     
╚██████╔╝██║ ╚████║██████╔╝███████╗██║  ██║██████╔╝██║███████╗
 ╚═════╝ ╚═╝  ╚═══╝╚═════╝ ╚══════╝╚═╝  ╚═╝╚═════╝ ╚═╝╚══════╝                                                         
");
    loop {
        println!("Pilih operasi:");
        println!("1. Tambah Onderdil");
        println!("2. Lihat Onderdil");
        println!("3. Edit Onderdil");
        println!("4. Hapus Onderdil");
        println!("5. Keluar");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca baris");

        match input.trim().parse() {
            Ok(choice) => match choice {
                1 => {
                    println!("Masukkan ID:");
                    let id: u32 = input_id();
                    println!("Masukkan Nama Onderdil:");
                    let nama = input_string();
                    println!("Masukkan Harga Onderdil:");
                    let harga: f64 = input_harga();

                    let onderdil = Onderdil { id, nama, harga };
                    database.tambah_onderdil(onderdil);
                }
                2 => database.lihat_onderdil(),
                3 => {
                    println!("Masukkan ID Onderdil yang Akan Diubah:");
                    let id: u32 = input_id();
                    println!("Masukkan Nama Baru Onderdil:");
                    let nama = input_string();
                    println!("Masukkan Harga Baru Onderdil:");
                    let harga: f64 = input_harga();

                    database.edit_onderdil(id, nama, harga);
                }
                4 => {
                    println!("Masukkan ID Onderdil yang Akan Dihapus:");
                    let id: u32 = input_id();
                    database.hapus_onderdil(id);
                }
                5 => {
                    println!("Program selesai.");
                    break;
                }
                _ => println!("Pilihan tidak valid."),
            },
            Err(_) => println!("Masukan tidak valid. Masukkan angka."),
        }
    }
}

fn input_id() -> u32 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca baris");

        match input.trim().parse() {
            Ok(id) => return id,
            Err(_) => println!("Masukan ID tidak valid. Masukkan angka."),
        }
    }
}

fn input_harga() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca baris");

        match input.trim().parse() {
            Ok(harga) => return harga,
            Err(_) => println!("Masukan harga tidak valid. Masukkan angka."),
        }
    }
}

fn input_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Gagal membaca baris");
    input.trim().to_string()
}
