#[path = "/home/deck/semio/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/⚡️idf/🧬️.rs"]
mod codec;
fn main() {
 let args: Vec<String> = std::env::args().collect();
 let text = std::fs::read_to_string(&args[1]).unwrap();
 if args.get(2).is_some_and(|mode| mode == "export") {
  let document = semio_framework_pack_json::parse(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  let records = codec::epjson_to_records(&document, codec::DICTIONARY).unwrap();
  print!("{}", codec::write_records(&records).unwrap());
 } else {
  let records = codec::parse_records(&text).unwrap();
  let document = codec::records_to_epjson(&records, codec::DICTIONARY).unwrap();
  print!("{}", semio_framework_pack_json::to_string_pretty(&document));
 }
}
