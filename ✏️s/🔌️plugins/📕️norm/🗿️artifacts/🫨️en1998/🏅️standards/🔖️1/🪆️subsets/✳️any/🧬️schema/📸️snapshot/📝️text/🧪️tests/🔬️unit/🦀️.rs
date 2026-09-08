
use super::*;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&En1998Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn seismic_rc_frame_example_fixture_parses_and_round_trips() {
    let document = parse_dsl(EN1998_SEISMIC_RC_FRAME_EXAMPLE_TEXT).expect("parse seismic rc frame example");
    assert_eq!(document.annex, "en");
    assert_eq!(document.importance_class, "cc3");
    assert_eq!(document.structural_system, "dual_system");
    assert_eq!(document.en_spectrum_type, "type2");
    assert_eq!(document.retrofit_limit_state, "near_collapse");
    assert!(!document.multiple_resisting_systems);
    assert!(!document.tower_is_chimney);
    store::os_store::test_support::assert_dsl_round_trip(&document);
}
