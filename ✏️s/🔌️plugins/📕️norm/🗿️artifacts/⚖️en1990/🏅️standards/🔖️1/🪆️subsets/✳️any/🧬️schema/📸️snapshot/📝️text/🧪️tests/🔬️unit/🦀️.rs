use super::*;

#[semio_framework_async_macros::async_test]
async fn default_snapshot_dsl_roundtrips() {
    let reference = crate::En1990Snapshot::default();
    let text = print_dsl(&reference);
    let document = parse_dsl(&text).expect("parse");
    assert_eq!(document, reference);
    assert_eq!(document.variables.len(), 2);
    assert_eq!(document.permanents.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn high_consequence_office_example_has_cc3() {
    let document = parse_dsl(EN1990_HIGH_CONSEQUENCE_OFFICE_EXAMPLE_TEXT).expect("example");
    assert_eq!(document.consequence_class, 3);
}
