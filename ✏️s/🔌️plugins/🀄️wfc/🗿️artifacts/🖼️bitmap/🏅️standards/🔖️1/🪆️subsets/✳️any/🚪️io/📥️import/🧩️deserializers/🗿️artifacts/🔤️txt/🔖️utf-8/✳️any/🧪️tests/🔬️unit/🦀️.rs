use super::*;

#[test]
fn deserializing_native_txt_rebuilds_rooms_16() {
    let document = crate::examples::rooms_16::snapshot();
    let txt = crate::standards::v1::subsets::any::io::export::serializers::artifacts::txt::v_utf_8::any::serialize(&document);
    let round = deserialize(&txt).expect("txt decode");
    assert_eq!(round, document);
    assert_eq!(TxtIntoBitmap::FIDELITY, IoFidelity::Exact);
    assert_eq!(TxtIntoBitmap::FROM, TXT_DIALECT);
}
