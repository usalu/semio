// Scratch probe (ticket 26/08/23/END-TO-END-TESTING-REFACTOR, wave 22): does quick-xml round-trip
// an XML declaration whose `standalone` pseudo-attribute is "no"?
use quick_xml::events::{BytesDecl, Event};
use quick_xml::{Reader, Writer};
use std::io::Cursor;

fn read_decl(source: &str) -> (String, Option<String>, Option<String>) {
    let mut reader = Reader::from_str(source);
    loop {
        match reader.read_event().expect("event") {
            Event::Decl(decl) => {
                let version = decl.version().expect("version").to_string();
                let encoding = decl.encoding().map(|r| r.expect("encoding").to_string());
                let standalone = decl.standalone().map(|r| r.expect("standalone").to_string());
                return (version, encoding, standalone);
            }
            Event::Eof => panic!("no declaration"),
            _ => {}
        }
    }
}

fn write_decl(version: &str, encoding: Option<&str>, standalone: Option<&str>) -> String {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    writer.write_event(Event::Decl(BytesDecl::new(version, encoding, standalone))).expect("write");
    writer.write_event(Event::Text(quick_xml::events::BytesText::from_escaped(""))).ok();
    String::from_utf8(writer.into_inner().into_inner()).expect("utf8")
}

fn main() {
    let source = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><a/>";
    println!("read yes  -> {:?}", read_decl(source));
    let written_no = write_decl("1.0", Some("UTF-8"), Some("no"));
    println!("write no  -> {:?}", written_no);
    let round = format!("{written_no}<a/>");
    println!("read back -> {:?}", read_decl(&round));
    let written_yes = write_decl("1.0", Some("UTF-8"), Some("yes"));
    println!("write yes -> {:?}", written_yes);
}
