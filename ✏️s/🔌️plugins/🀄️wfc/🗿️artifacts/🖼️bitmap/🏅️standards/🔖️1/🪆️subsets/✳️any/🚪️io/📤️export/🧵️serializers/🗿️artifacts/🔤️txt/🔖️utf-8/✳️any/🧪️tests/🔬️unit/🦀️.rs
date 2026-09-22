use super::*;

#[test]
fn serializing_rooms_16_to_txt_is_exact_and_non_empty() {
    let document = crate::examples::rooms_16::snapshot();
    let txt = serialize(&document);
    assert!(!txt.is_empty(), "txt must carry the DSL");
    assert_eq!(BitmapIntoTxt::FIDELITY, IoFidelity::Exact);
    assert_eq!(BitmapIntoTxt::INTO, TXT_DIALECT);
    let round = crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::deserialize(&txt).expect("txt round-trip");
    assert_eq!(round, document);
}
