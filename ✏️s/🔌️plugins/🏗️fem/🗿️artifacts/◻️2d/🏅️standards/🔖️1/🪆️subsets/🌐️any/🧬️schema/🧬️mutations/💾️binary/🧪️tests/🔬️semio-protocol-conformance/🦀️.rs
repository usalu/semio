
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::update_analysis_settings;

#[test]
fn component_protocol_semio_is_protocol_dialect() {
    let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
    assert_eq!(g.dialect, ::dsl::SemioDialect::Protocol);
    assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
    let _ = COMPONENT_PROTOCOL_PATH;
}

#[test]
fn verify_protocol_bytes_against_encoded_spr() {
    let operation = Fem2dMutation::UpdateAnalysisSettings(update_analysis_settings::UpdateAnalysisSettings { settings: crate::FemAnalysisSettings { modal_count: 5, buckling_count: 2, deformation_scale: 10.0 } });
    let bytes = encode_op(&operation).expect("encode op");
    let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
    ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes spr bytes");
}
