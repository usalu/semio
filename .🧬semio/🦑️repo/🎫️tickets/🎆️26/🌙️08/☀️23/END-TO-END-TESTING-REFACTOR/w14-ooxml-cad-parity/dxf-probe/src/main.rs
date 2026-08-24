fn main() {
    for path in std::env::args().skip(1) {
        match dxf::Drawing::load_file(&path) {
            Ok(drawing) => {
                let names: Vec<String> = drawing.line_types().map(|l| format!("{}[{}]", l.name, l.description)).collect();
                println!("{path}\n  {} linetypes: {:?}", names.len(), names);
            }
            Err(e) => println!("{path}\n  ERROR {e}"),
        }
    }
}
