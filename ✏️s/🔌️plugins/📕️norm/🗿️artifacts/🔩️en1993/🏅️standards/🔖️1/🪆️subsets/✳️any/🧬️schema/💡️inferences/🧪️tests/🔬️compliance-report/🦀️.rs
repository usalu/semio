use super::*;

#[semio_framework_async_macros::async_test]
async fn full_steel_member_e2e() {
    let report = check_full_steel_member(&En1993Snapshot::default());
    assert_eq!(report.checks.len(), 25);
}

#[semio_framework_async_macros::async_test]
async fn every_part_reaches_evaluate() {
    let report = check_full_steel_member(&En1993Snapshot::default());
    let families: std::collections::BTreeSet<&str> = report.checks.iter().map(|c| c.clause.family.as_str()).collect();
    for expected in
        ["EN 1993-1-1", "EN 1993-1-2", "EN 1993-1-3", "EN 1993-1-4", "EN 1993-1-5", "EN 1993-1-6", "EN 1993-1-8", "EN 1993-1-9", "EN 1993-1-10", "EN 1993-1-11", "EN 1993-1-12", "EN 1993-2", "EN 1993-3-1", "EN 1993-4-1", "EN 1993-5", "EN 1993-6"]
    {
        assert!(families.contains(expected), "missing checks sourced from {expected}");
    }
}
