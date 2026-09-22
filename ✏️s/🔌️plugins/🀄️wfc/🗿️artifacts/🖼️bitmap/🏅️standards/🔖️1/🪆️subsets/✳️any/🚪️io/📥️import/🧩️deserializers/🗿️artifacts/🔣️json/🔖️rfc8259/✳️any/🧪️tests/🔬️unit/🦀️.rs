use super::*;

#[test]
fn deserializing_canonical_json_rebuilds_rooms_16() {
    let document = crate::examples::rooms_16::snapshot();
    let json = crate::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::serialize(&document);
    let round = deserialize(&json).expect("json decode");
    assert_eq!(round, document);
    assert_eq!(JsonIntoBitmap::FIDELITY, IoFidelity::Exact);
    assert_eq!(JsonIntoBitmap::FROM, JSON_DIALECT);
}
