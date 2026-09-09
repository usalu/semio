use super::*;
use std::collections::BTreeSet;

#[semio_framework_async_macros::async_test]
async fn evaluate_reaches_operative_sheet_families() {
    let report = evaluate(&Vdi3805Snapshot::default());
    let parts: BTreeSet<String> = report.checks.iter().map(|c| c.clause.part.clone()).filter(|p| p.chars().all(|ch| ch.is_ascii_digit())).collect();
    let registry = SchemaCatalog::current();
    for sheet in registry.operative_sheets() {
        let part = sheet.id.0.to_string();
        if sheet.status == SchemaStatus::Reserved {
            continue;
        }
        assert!(parts.contains(&part), "missing checks for sheet {part}");
    }
}

#[semio_framework_async_macros::async_test]
async fn reserved_sheet_returns_not_applicable() {
    let doc = Vdi3805Snapshot::default();
    let result = part_15::check(&doc);
    assert_eq!(result.status, document::CheckStatus::NotApplicable);
    let result = part_67::check(&doc);
    assert_eq!(result.status, document::CheckStatus::NotApplicable);
}

#[semio_framework_async_macros::async_test]
async fn historical_part_check_respects_strict_mode() {
    let mut doc = Vdi3805Snapshot { strict_mode: true, ..Vdi3805Snapshot::default() };
    let result = part_12::check(&doc);
    assert_eq!(result.status, document::CheckStatus::Fail);

    doc.strict_mode = false;
    let result = part_12::check(&doc);
    assert_eq!(result.status, document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn multi_profile_part_check_reports_metadata_when_no_product() {
    let doc = Vdi3805Snapshot::default();
    let result = part_08::check(&doc);
    assert_eq!(result.status, document::CheckStatus::Pass);
}

#[semio_framework_async_macros::async_test]
async fn evaluate_reports_strict_mode_check() {
    let doc = Vdi3805Snapshot { strict_mode: true, ..Vdi3805Snapshot::default() };
    let report = evaluate(&doc);
    assert!(report.checks.iter().any(|c| c.clause.section == "strict"));
}

#[semio_framework_async_macros::async_test]
async fn evaluate_skips_geometry_and_curve_checks_when_absent() {
    let mut doc = Vdi3805Snapshot::default();
    doc.geometry.clear();
    doc.curves.clear();
    let report = evaluate(&doc);
    assert!(!report.checks.iter().any(|c| c.clause.part == "geometry"));
    assert!(!report.checks.iter().any(|c| c.clause.part == "functions"));
}
