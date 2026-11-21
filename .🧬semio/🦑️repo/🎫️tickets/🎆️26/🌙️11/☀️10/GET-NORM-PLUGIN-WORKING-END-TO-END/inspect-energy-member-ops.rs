use std::{env, fs};
use wasmparser::{Parser, Payload};

fn main() {
    let mut arguments = env::args().skip(1);
    let file = fs::read(arguments.next().unwrap()).unwrap();
    let region = env::var("ENERGY_CORE_REGION").ok().map(|region| {
        let (start, end) = region.split_once(':').unwrap();
        usize::from_str_radix(start, 16).unwrap()..usize::from_str_radix(end, 16).unwrap()
    });
    let file = region.map_or(file.as_slice(), |region| &file[region]);
    let wanted: Vec<_> = arguments.map(|value| value.parse::<usize>().unwrap()).collect();
    let mut index = 0;
    for part in Parser::new(0).parse_all(file) {
        if let Payload::CodeSectionEntry(body) = part.unwrap() {
            if wanted.contains(&index) {
                println!("function {index} {:?}", body.range());
                let mut ops = body.get_operators_reader().unwrap();
                while !ops.eof() {
                    let offset = ops.original_position();
                    println!("{offset:08x} {:?}", ops.read().unwrap());
                }
            }
            index += 1;
        }
    }
}
