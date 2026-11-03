use super::*;
use norm::{AnnexChoice, ClauseId, LocalizedCopy, Quantity, QuantityKind, SubjectRef};

fn check(entity: &str, name: &str, utilisation: f64) -> CheckResult {
    let subject = SubjectRef::new(entity, format!("members[id={entity}]"), LocalizedCopy::new(entity, entity));
    CheckResult::assess(format!("{entity}.{name}"), "3", ClauseId::new("EN 1993-1-1", "6", "6.2.5"), subject, LocalizedCopy::new("Bending", "Biegung"))
        .utilization(Quantity::new(QuantityKind::Moment, utilisation * 100.0), Quantity::new(QuantityKind::Moment, 100.0))
        .annex(AnnexChoice::De)
        .explanation(LocalizedCopy::new("x", "x"))
        .build()
}

fn report(rows: &[(&str, &str, f64)]) -> CheckReport {
    let mut report = CheckReport::default();
    rows.iter().for_each(|(entity, name, utilisation)| report.push(check(entity, name, *utilisation)));
    report
}

fn ids(subjects: &[&str]) -> Vec<String> {
    subjects.iter().map(|s| (*s).to_owned()).collect()
}

#[test]
fn a_slice_keeps_the_report_order_of_the_requested_subjects() {
    let index = ReportIndex::of(&report(&[("a", "m", 0.1), ("b", "m", 0.2), ("a", "v", 0.3), ("c", "m", 0.4)]));
    let slice = index.slice(&ids(&["c", "a"]));
    assert_eq!(slice.checks.iter().map(|c| c.id.as_str()).collect::<Vec<_>>(), ["a.m", "a.v", "c.m"]);
    assert_eq!(slice.summary, CheckReportSummary::default());
}

#[test]
fn following_a_report_names_only_the_subjects_whose_rows_changed() {
    let mut index = ReportIndex::of(&report(&[("a", "m", 0.1), ("b", "m", 0.2)]));
    let (stamp_a, stamp_b) = (index.stamp(&ids(&["a"])), index.stamp(&ids(&["b"])));
    let changed = index.follow(&report(&[("a", "m", 0.1), ("b", "m", 0.9)]));
    assert_eq!(changed.into_iter().collect::<Vec<_>>(), ["b"]);
    assert_eq!(index.stamp(&ids(&["a"])), stamp_a);
    assert_ne!(index.stamp(&ids(&["b"])), stamp_b);
}

#[test]
fn a_row_inserted_before_a_subject_does_not_move_its_stamp() {
    let mut index = ReportIndex::of(&report(&[("b", "m", 0.2)]));
    let stamp = index.stamp(&ids(&["b"]));
    let changed = index.follow(&report(&[("a", "m", 0.1), ("b", "m", 0.2)]));
    assert_eq!(changed.into_iter().collect::<Vec<_>>(), ["a"]);
    assert_eq!(index.stamp(&ids(&["b"])), stamp);
    assert_eq!(index.report().checks.iter().map(|c| c.id.as_str()).collect::<Vec<_>>(), ["a.m", "b.m"]);
}

#[test]
fn a_removed_subject_changes_and_the_index_round_trips_the_report() {
    let full = report(&[("a", "m", 0.1), ("b", "m", 0.2)]);
    let mut index = ReportIndex::of(&full);
    assert_eq!(index.report(), full);
    let changed = index.follow(&report(&[("a", "m", 0.1)]));
    assert_eq!(changed.into_iter().collect::<Vec<_>>(), ["b"]);
    assert!(index.slice(&ids(&["b"])).checks.is_empty());
}

#[test]
fn an_unchanged_report_keeps_the_version() {
    let r = report(&[("a", "m", 0.1)]);
    let mut index = ReportIndex::of(&r);
    let version = index.version();
    assert!(index.follow(&r).is_empty());
    assert_eq!(index.version(), version);
}
