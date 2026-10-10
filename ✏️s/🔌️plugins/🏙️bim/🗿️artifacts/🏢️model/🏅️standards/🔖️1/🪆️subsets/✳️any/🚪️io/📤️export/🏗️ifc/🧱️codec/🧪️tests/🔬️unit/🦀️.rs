use super::*;
use semio_s_artifact_stdio_ifc::part21::{Part21Header, Part21Instance, Part21Value};

fn header(schema: &str) -> Part21Header {
    let mut header = Part21Header::iso_10303_21_minimum();
    header.file_schema = vec![Part21Value::List(vec![Part21Value::Str(schema.into())])];
    header
}

fn document(schema: &str, ids: &[u64]) -> Part21Document {
    Part21Document { header: header(schema), instances: ids.iter().map(|id| Part21Instance { id: *id, entities: vec![("IFCPROJECT".to_string(), vec![Part21Value::Str("p".into())])] }).collect() }
}

#[test]
fn an_ifc4_document_round_trips_through_its_bytes() {
    let original = document("IFC4", &[1, 2]);
    let bytes = encode_ifc4(&original).expect("the document encodes");
    assert!(bytes.starts_with(b"ISO-10303-21"));
    assert_eq!(decode_ifc4(&bytes).expect("the bytes decode").instances, original.instances);
}

#[test]
fn the_ifc4_codec_refuses_another_schema_and_a_repeated_instance() {
    assert!(encode_ifc4(&document("IFC2X3", &[1])).is_err());
    assert!(encode_ifc4(&document("IFC4", &[1, 1])).unwrap_err().contains("#1"));
    let bytes = encode_document(document("IFC2X3", &[1])).expect("the 2x3 codec encodes");
    assert!(decode_ifc4(&bytes).is_err(), "an IFC2X3 file is no IFC4 file");
    assert!(decode_document(&bytes).is_ok());
    assert!(decode_ifc4(&encode_ifc4(&document("IFC4", &[1])).expect("encodes")).is_ok());
    assert!(decode_document(&encode_ifc4(&document("IFC4", &[1])).expect("encodes")).is_err(), "and an IFC4 file is no IFC2X3 file");
    assert!(decode_ifc4(&[0xff, 0xfe]).is_err());
}
