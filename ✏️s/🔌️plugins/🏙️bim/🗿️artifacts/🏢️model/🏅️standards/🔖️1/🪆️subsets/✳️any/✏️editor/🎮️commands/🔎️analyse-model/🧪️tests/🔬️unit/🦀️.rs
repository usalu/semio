use super::*;
use crate::editor::bim::unit_tests::support::{ctx, demo, run};
use crate::standards::v1::subsets::any::schema::inferences::model_graph::registry;

#[semio_framework_async_macros::async_test]
async fn the_command_settles_the_session_and_changes_nothing_else() {
    let snapshot = demo();
    let mut context = ctx(&[]);
    let emit = run(&snapshot, |doc, cfg| handle(&AnalyseModel { pressed: Some(true) }, doc, cfg, &mut context)).expect("analyses");
    assert!(emit.artifact_mutations.is_empty() && emit.effects.is_empty(), "an analysis only warms the session");
    registry::with_inference(None, &snapshot, |_| ());
    assert!(registry::report(None).gated, "the next read answers from memory");
}

#[semio_framework_async_macros::async_test]
async fn the_analysis_is_stepped_with_progress_and_a_cancel_keeps_what_it_finished() {
    let snapshot = demo();
    let mut cancelled = Analysis::new(Some(90), 2);
    let first = cancelled.advance(&snapshot).expect("a step");
    assert!(!first.done && cancelled.fraction() > 0.0);
    cancelled.cancel(&snapshot);
    let report = registry::report(Some(90));
    assert!(report.cancelled && report.computed > 0, "{report:?}");
    let mut next = Analysis::new(Some(90), 1_000);
    while !next.is_done() {
        next.advance(&snapshot).expect("a step");
    }
    assert!(registry::report(Some(90)).reused >= report.computed, "the finished nodes were not computed again");
    registry::close(90);
}
