//! 🧪️ The embedded grammar and protocol dialects of the fem2d snapshot codecs parse and recognize real pack bytes.

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::{binary, text};
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

#[test]
fn component_grammar_semio_is_grammar_dialect() {
    let g = semio_framework_dsl::parse_grammar(text::snapshot::COMPONENT_GRAMMAR_SEMIO).expect("parse grammar.semio");
    assert_eq!(g.dialect, semio_framework_dsl::SemioDialect::Grammar);
    assert!(!text::snapshot::COMPONENT_GRAMMAR_SEMIO.is_empty());
}

#[test]
fn component_protocol_semio_is_protocol_dialect() {
    let g = semio_framework_dsl::parse_grammar(binary::snapshot::COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
    assert_eq!(g.dialect, semio_framework_dsl::SemioDialect::Protocol);
    assert!(!binary::snapshot::COMPONENT_PROTOCOL_SEMIO.is_empty());
}

#[test]
fn verify_protocol_bytes_against_encoded_pack() {
    let bytes = binary::snapshot::encode(&Fem2dSnapshot::default());
    let g = semio_framework_dsl::parse_grammar(binary::snapshot::COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
    semio_os_kernel::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes pack bytes");
}
