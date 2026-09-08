
use super::*;

#[semio_framework_async_macros::async_test]
async fn native_text_round_trip() {
    let doc = Vdi3805Snapshot::default();
    let text = serialize_native_text(&doc.catalog);
    let parsed = parse_native_text(&text, SecurityLimits::default()).expect("parse");
    assert_eq!(parsed.products.len(), doc.catalog.products.len());
    assert_eq!(parsed.file.manufacturer, doc.catalog.file.manufacturer);
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
async fn parse_native_text_parses_product_records() {
    let text = "3805;DEMO;420.10.1;2026-07-22;1\n100;DEMO;HV;VLV-1;2\n200;dn;50\n";
    let parsed = parse_native_text(text, SecurityLimits::default()).expect("parse");
    assert_eq!(parsed.products.len(), 1);
    assert_eq!(parsed.products[0].identity.article_number, "VLV-1");
    assert_eq!(parsed.products[0].sheet, SheetId(2));
}

#[semio_framework_async_macros::async_test]
async fn validate_structure_reports_missing_manufacturer() {
    let mut doc = Vdi3805Snapshot::default();
    doc.catalog.file.manufacturer = String::new();
    let issues = validate_structure(&doc.catalog);
    assert!(issues.iter().any(|d| d.field == "manufacturer" && d.severity == Severity::Error));
}

#[semio_framework_async_macros::async_test]
async fn validate_structure_reports_empty_products() {
    let mut doc = Vdi3805Snapshot::default();
    doc.catalog.products.clear();
    let issues = validate_structure(&doc.catalog);
    assert!(issues.iter().any(|d| d.field == "products" && d.severity == Severity::Warning));
}

#[semio_framework_async_macros::async_test]
async fn validate_structure_reports_missing_article_number_and_config_id() {
    let mut doc = Vdi3805Snapshot::default();
    doc.catalog.products[0].identity.article_number = String::new();
    doc.catalog.products[0].configuration.id = String::new();
    let issues = validate_structure(&doc.catalog);
    assert!(issues.iter().any(|d| d.severity == Severity::Error && d.field.starts_with("product.")));
    assert!(issues.iter().any(|d| d.severity == Severity::Warning && d.field.starts_with("configuration.")));
}

#[semio_framework_async_macros::async_test]
async fn validate_structure_reports_unknown_record_family() {
    let mut doc = Vdi3805Snapshot::default();
    doc.catalog.products[0].records.push(NativeRecord { family: RecordFamilyId("888".into()), fields: vec!["888".into()], extensions: ExtensionBag::default() });
    let issues = validate_structure(&doc.catalog);
    assert!(issues.iter().any(|d| d.severity == Severity::Info && d.field.contains("888")));
}

#[semio_framework_async_macros::async_test]
async fn linear_map_interpolates_and_handles_degenerate_domain() {
    assert!((linear_map(5.0, 0.0, 10.0, 0.0, 100.0) - 50.0).abs() < 1e-9);
    assert_eq!(linear_map(5.0, 3.0, 3.0, 7.0, 42.0), 7.0);
}

#[semio_framework_async_macros::async_test]
async fn diagnostic_constructors_and_report_mapping() {
    let diags = vec![Diagnostic::error("f1", "bad"), Diagnostic::warning("f2", "meh"), Diagnostic::info("f3", "fyi")];
    let report = diagnostics_to_report(&diags, "1", "validate");
    assert_eq!(report[0].status, CheckStatus::Fail);
    assert_eq!(report[1].status, CheckStatus::Pass);
    assert_eq!(report[2].status, CheckStatus::Pass);
}
