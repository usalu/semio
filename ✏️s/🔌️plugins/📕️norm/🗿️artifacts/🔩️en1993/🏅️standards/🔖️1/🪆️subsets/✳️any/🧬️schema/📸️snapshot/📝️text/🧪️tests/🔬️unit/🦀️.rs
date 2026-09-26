use super::*;

#[semio_framework_async_macros::async_test]
async fn document_dsl_round_trips() {
    store::os_store::test_support::assert_dsl_round_trip(&En1993Snapshot::default());
}

#[semio_framework_async_macros::async_test]
async fn dsl_round_trip_agrees_with_print_parse_wrappers() {
    let document = En1993Snapshot::default();
    let printed = print_dsl(&document);
    assert_eq!(parse_dsl(&printed).expect("parse printed document"), document);
}

#[semio_framework_async_macros::async_test]
async fn high_strength_connection_example_fixture_parses_and_round_trips() {
    use crate::document::AnnexChoice;
    use crate::standards::v1::subsets::any::schema::part_1_1;
    let document = parse_dsl(EN1993_HIGH_STRENGTH_CONNECTION_EXAMPLE_TEXT).expect("parse high strength connection example");
    assert_eq!(document.annex, AnnexChoice::En);
    let joint = document.joints.iter().find(|j| j.id == "joint-j1").expect("joint-j1");
    assert_eq!(joint.bolt_rows * joint.bolts_per_row, 4);
    assert_eq!(joint.bolt_class, "10.9");
    assert!((joint.bolt_diameter - 0.024).abs() < 1e-9);
    let fat = document.fatigue_details.iter().find(|f| f.id == "fat-1").expect("fat-1");
    assert_eq!(fat.method, "safe_life");
    let mat = document.materials.iter().find(|m| m.id == "mat-s460").expect("mat-s460");
    assert_eq!(mat.grade, "S460");
    assert_eq!(mat.subgrade, "K2");
    let section = document.sections.iter().find(|s| s.id == "sec-heb260").expect("sec-heb260");
    let class = part_1_1::section_class_rolled_i(section, mat.fy);
    assert!(class >= 1 && class <= 3, "expected class 1–3 for HEB 260 S460, got {class}");
    assert!(document.load_cases.iter().any(|lc| lc.kind == "permanent"));
    assert!(document.load_cases.iter().any(|lc| lc.kind == "imposed"));
    store::os_store::test_support::assert_dsl_round_trip(&document);
}


