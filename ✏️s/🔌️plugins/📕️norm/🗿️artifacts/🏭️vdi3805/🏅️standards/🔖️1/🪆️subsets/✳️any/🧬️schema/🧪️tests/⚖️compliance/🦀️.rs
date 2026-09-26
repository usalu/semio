use super::*;

#[semio_framework_async_macros::async_test]
async fn native_text_round_trip() {
    let doc = Vdi3805Snapshot::default();
    let text = serialize_native_text(&doc.catalog);
    let parsed = parse_native_text(&text, SecurityLimits::default()).expect("parse");
    assert_eq!(parsed.products.len(), doc.catalog.products.len());
    assert_eq!(parsed.file.manufacturer, doc.catalog.file.manufacturer);
    assert!(matches!(parsed.products[0].configuration.attributes, SheetAttributes::ValveHeating(_)));
}

#[semio_framework_async_macros::async_test]
async fn parse_native_text_rejects_empty_input() {
    let err = parse_native_text("", SecurityLimits::default()).unwrap_err();
    assert!(matches!(err, NormError::IncompleteInput { field } if field == "header"));
}

#[semio_framework_async_macros::async_test]
async fn parse_native_text_rejects_incomplete_header() {
    let err = parse_native_text("3805;DEMO;420.10.1\n", SecurityLimits::default()).unwrap_err();
    assert!(matches!(err, NormError::IncompleteInput { field } if field == "header_fields"));
}

#[semio_framework_async_macros::async_test]
async fn parse_native_text_rejects_invalid_building_system_number() {
    let err = parse_native_text("3805;DEMO;bad;2026-07-22;3\n", SecurityLimits::default()).unwrap_err();
    assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "building_system_number"));
}

#[semio_framework_async_macros::async_test]
async fn parse_native_text_rejects_non_numeric_record_count() {
    let err = parse_native_text("3805;DEMO;420.10.1;2026-07-22;abc\n", SecurityLimits::default()).unwrap_err();
    assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "record_count"));
}

#[semio_framework_async_macros::async_test]
async fn parse_native_text_rejects_too_many_records() {
    let limits = SecurityLimits { max_records: 0, ..SecurityLimits::default() };
    let text = "3805;DEMO;420.10.1;2026-07-22;1\n200;dn;50\n";
    let err = parse_native_text(text, limits).unwrap_err();
    assert!(matches!(err, NormError::InvalidValue { field, .. } if field == "records"));
}

#[semio_framework_async_macros::async_test]
async fn parse_native_text_parses_typed_product_records() {
    let text = "010;3805;DEMO;420.10.1;2026-07-22;UTF-8;2\n100;DEMO;HV;VLV-1;2\n200;geom.1;0.1;0.1;0.1;dn;50;kvs;4.5;pressure_class;PN16\n";
    let parsed = parse_native_text(text, SecurityLimits::default()).expect("parse");
    assert_eq!(parsed.products.len(), 1);
    assert_eq!(parsed.products[0].identity.article_number, "VLV-1");
    assert_eq!(parsed.products[0].sheet, SheetId(2));
    match &parsed.products[0].configuration.attributes {
        SheetAttributes::ValveHeating(a) => {
            assert_eq!(a.dn, 50);
            assert!((a.kvs_m3_h() - 4.5).abs() < 1e-9);
        }
        other => panic!("expected valve attributes, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn validate_structure_reports_missing_manufacturer() {
    let mut doc = Vdi3805Snapshot::default();
    doc.catalog.file.manufacturer = String::new();
    let issues = validate_structure(&doc);
    assert!(issues.iter().any(|d| d.field.contains("manufacturer") && d.severity == Severity::Error));
}

#[semio_framework_async_macros::async_test]
async fn validate_structure_reports_empty_products() {
    let mut doc = Vdi3805Snapshot::default();
    doc.catalog.products.clear();
    doc.catalog.file.record_count = 0;
    doc.catalog.file.record_count = 0;
    let issues = validate_structure(&doc);
    assert!(issues.iter().any(|d| d.field.contains("products") && d.severity == Severity::Error));
}

#[semio_framework_async_macros::async_test]
async fn validate_structure_reports_record_count_mismatch() {
    let mut doc = Vdi3805Snapshot::default();
    doc.catalog.file.record_count = 1;
    doc.catalog.file.record_count = 1;
    let issues = validate_structure(&doc);
    assert!(issues.iter().any(|d| d.field.contains("recordCount") && d.severity == Severity::Error));
}

#[semio_framework_async_macros::async_test]
async fn validate_structure_reports_dangling_geometry_ref() {
    let mut doc = Vdi3805Snapshot::default();
    doc.catalog.products[0].configuration.geometry_ref = Some("geom.missing".into());
    let issues = validate_structure(&doc);
    assert!(issues.iter().any(|d| d.field.contains("geometryRef") && d.severity == Severity::Error));
}

#[semio_framework_async_macros::async_test]
async fn linear_map_interpolates_and_handles_degenerate_domain() {
    assert!((linear_map(5.0, 0.0, 10.0, 0.0, 100.0) - 50.0).abs() < 1e-9);
    assert_eq!(linear_map(5.0, 3.0, 3.0, 7.0, 42.0), 7.0);
}

#[semio_framework_async_macros::async_test]
async fn round_trip_helpers_succeed_for_reference_fixture() {
    let doc = Vdi3805Snapshot::default();
    assert_json_round_trip(&doc.catalog).expect("json");
    assert_native_round_trip(&doc.catalog, crate::SecurityLimits::default()).expect("native");
}
