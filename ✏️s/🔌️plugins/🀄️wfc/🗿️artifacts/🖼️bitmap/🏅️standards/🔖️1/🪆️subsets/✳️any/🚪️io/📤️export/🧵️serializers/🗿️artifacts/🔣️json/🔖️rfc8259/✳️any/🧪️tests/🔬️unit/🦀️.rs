use super::*;

#[test]
fn serializing_rooms_16_to_json_is_exact_and_non_empty() {
    let document = crate::examples::rooms_16::snapshot();
    let json = serialize(&document);
    assert!(!json.is_empty(), "json text must carry the document");
    assert_eq!(BitmapIntoJson::FIDELITY, IoFidelity::Exact);
    assert_eq!(BitmapIntoJson::INTO, JSON_DIALECT);
    let round = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize(&json).expect("json round-trip");
    assert_eq!(round, document);
}
