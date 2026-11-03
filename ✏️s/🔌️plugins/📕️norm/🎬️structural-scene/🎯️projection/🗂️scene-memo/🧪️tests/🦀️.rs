use super::*;
use crate::mock_projection::*;
use norm::{AnnexChoice, CheckResult, ClauseId, LocalizedCopy, Quantity, QuantityKind, SubjectRef};

fn whole(d: &MockDoc) -> StructuralScene {
    SceneProjector::new(MockProjection::default()).project(d, &eval(&[]), &ctx(1)).expect("scene")
}

fn report(utilisation: f64) -> CheckReport {
    let mut report = CheckReport::default();
    let subject = SubjectRef::new("m0", "members[id=m0]", LocalizedCopy::new("m0", "m0"));
    report.push(CheckResult::assess("m0.c", "3", ClauseId::new("EN", "1", "1"), subject, LocalizedCopy::new("c", "c")).utilization(Quantity::new(QuantityKind::Moment, utilisation), Quantity::new(QuantityKind::Moment, 1.0)).annex(AnnexChoice::De).explanation(LocalizedCopy::new("x", "x")).build());
    report
}

#[test]
fn an_unchanged_revision_projects_once_and_shares_the_scene() {
    let mut memo = SceneMemo::<MockDoc>::default();
    let (d, r) = (doc(3), report(0.5));
    let first = memo.scene(&d, &r, Some("c1"), || whole(&d));
    let second = memo.scene(&d, &r, Some("c1"), || panic!("served from the memo"));
    assert!(Arc::ptr_eq(&first, &second));
    assert_eq!(memo.projections(), 1);
    assert!(memo.delta().is_none());
}

#[test]
fn a_changed_document_report_or_combination_projects_again_and_records_the_delta() {
    let mut memo = SceneMemo::<MockDoc>::default();
    let (mut d, r) = (doc(3), report(0.5));
    memo.scene(&d, &r, None, || whole(&d));
    d.members[1].length += 1.0;
    memo.scene(&d, &r, None, || whole(&d));
    let touched: std::collections::BTreeSet<String> = memo.delta().expect("delta").entities.iter().map(|patch| patch.id().to_owned()).collect();
    assert!(touched.contains("m1") && !touched.contains("m0") && !touched.contains("m2"), "{touched:?}");
    memo.scene(&d, &report(0.9), None, || whole(&d));
    memo.scene(&d, &report(0.9), Some("c2"), || whole(&d));
    assert_eq!(memo.projections(), 4);
}

live_whole_scene!(mod retained(MockDoc));

#[test]
fn the_declared_module_is_shared_by_every_window() {
    let (d, r) = (doc(2), report(0.2));
    let a = retained::scene(&d, &r, None, || whole(&d));
    let b = retained::scene(&d, &r, None, || panic!("memo"));
    assert!(std::sync::Arc::ptr_eq(&a, &b) && retained::delta().is_none());
}
