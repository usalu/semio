use super::*;

#[test]
fn component_protocol_semio_is_protocol_dialect() {
    let g = semio_framework_dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
    assert_eq!(g.dialect, semio_framework_dsl::SemioDialect::Protocol);
    assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
    let _ = COMPONENT_PROTOCOL_PATH;
}
#[test]
fn verify_protocol_bytes_against_encoded_pack() {
    use crate::Fem3dSnapshot;
    let document = Fem3dSnapshot::default();
    let bytes = encode(&document);
    let g = semio_framework_dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
    ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes pack bytes");
}
