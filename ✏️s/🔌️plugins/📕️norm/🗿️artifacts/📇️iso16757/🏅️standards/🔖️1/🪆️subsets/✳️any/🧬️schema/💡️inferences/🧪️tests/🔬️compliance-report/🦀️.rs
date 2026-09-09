use super::*;
use crate::document::CheckStatus;

#[semio_framework_async_macros::async_test]
async fn evaluate_exercises_all_parts_with_numeric_checks() {
    let report = evaluate(&Iso16757Snapshot::default());
    assert!(!report.checks.is_empty());
    let clauses: HashSet<String> = report.checks.iter().map(|c| format!("{} {}", c.clause.part, c.clause.section)).collect();
    assert!(clauses.iter().any(|c| c.starts_with("1 ")));
    assert!(clauses.iter().any(|c| c.starts_with("2 ")));
    assert!(clauses.iter().any(|c| c.starts_with("4 ")));
    assert!(clauses.iter().any(|c| c.starts_with("5 ")));
    let part_number_check = report.checks.iter().find(|c| c.clause.section == "6.10").expect("part number check");
    assert_eq!(part_number_check.status, CheckStatus::Pass);
    assert!((part_number_check.computed.value - 550.0).abs() < 1e-6);
}
