use super::*;

#[test]
fn component_protocol_semio_is_protocol_dialect() {
    let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
    assert_eq!(g.dialect, ::dsl::SemioDialect::Protocol);
    assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
    let _ = COMPONENT_PROTOCOL_PATH;
}

#[test]
fn verify_protocol_bytes_against_encoded_pack() {
    let document = Fem2dSnapshot::default();
    let bytes = encode(&document);
    let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
    ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes pack bytes");
}
