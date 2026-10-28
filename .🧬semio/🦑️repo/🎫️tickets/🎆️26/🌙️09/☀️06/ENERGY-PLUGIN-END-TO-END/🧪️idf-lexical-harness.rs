#[path = "/home/deck/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/⚡️idf/🔤️.rs"]
mod lexical;
fn main() {
 let source = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
 let records = lexical::parse_records(&source).unwrap();
 print!("{}", lexical::write_records(&records).unwrap());
}
