use wit_component::WitPrinter;
use wit_parser::Resolve;
fn main() {
    let dir = std::env::args().nth(1).expect("directory");
    let mut resolve = Resolve::default();
    match resolve.push_dir(&dir) {
        Ok((pkg, _)) => {
            let mut printer = WitPrinter::default();
            printer.emit_docs(false);
            printer.print(&resolve, pkg, &[]).unwrap();
            println!("{}", printer.output);
        }
        Err(error) => {
            eprintln!("{error:#}");
            std::process::exit(1);
        }
    }
}
