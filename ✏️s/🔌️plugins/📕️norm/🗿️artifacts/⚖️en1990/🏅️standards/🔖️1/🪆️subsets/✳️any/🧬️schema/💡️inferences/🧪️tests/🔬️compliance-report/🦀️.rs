use super::*;
use crate::standards::v1::subsets::any::schema::{check_combination_set, combination_uls, CombinationRule};

#[semio_framework_async_macros::async_test]
async fn evaluate_accidental_situation_numeric() {
    let doc = En1990Snapshot::default();
    let actions = action_set_from_document(&doc);
    let accidental_ed = combination_uls(&NaDe, DesignSituation::Accidental, CombinationRule::Uls610a, &actions, 0);
    assert!((accidental_ed - 168.0).abs() < 1e-9);
    let report = evaluate(&doc);
    let persistent = check_combination_set(&NaDe, DesignSituation::Persistent, &actions, doc.resistance_kn);
    let accidental = check_combination_set(&NaDe, DesignSituation::Accidental, &actions, doc.resistance_kn);
    assert_eq!(report.checks.len(), persistent.checks.len() + accidental.checks.len() + 2);
    assert!(report.checks.iter().any(|c| (c.computed.value / 1000.0 - accidental_ed).abs() < 1e-6));
}

#[semio_framework_async_macros::async_test]
async fn evaluate_seismic_situation_numeric() {
    let doc = En1990Snapshot::default();
    let report = evaluate(&doc);
    let seismic = report.checks.iter().find(|c| c.clause.section == "6.12b").expect("seismic 6.12b check present");
    assert!((seismic.computed.value / 1000.0 - 155.0).abs() < 1e-9);
}
