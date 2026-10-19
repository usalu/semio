use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::Analysis;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_kit::editor::bim::kit::fault;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_dispatch_context::editor::bim::dispatch_context::BimDispatchCtx;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_bim::editor::bim::{BimCommand, BimModelApp};
#[allow(unused_imports)]
use semio_s_artifact_bim_model_mut_any_190::standards::v1::subsets::any::schema::mutations::ModelMutation;
#[allow(unused_imports)]
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, EditorApp, Emit, Fault, NoConfig, NoConfigMutation};
#[allow(unused_imports)]
use value_derive::{FromValue, ToValue};
use semio_s_artifact_bim_model_bim::editor::bim::commands::analyse_model::{handle, AnalyseModel};
use semio_s_artifact_bim_model_bim::editor::bim::fixture::support::{ctx, demo, run};
use semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance as inference;

#[semio_framework_async_macros::async_test]
async fn the_command_settles_the_session_and_changes_nothing_else() {
    let instance = semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::detached();
    let snapshot = demo();
    let mut context = ctx(&[]);
    context.gestures = Some(instance.clone());
    let emit = run(&snapshot, |doc, cfg| handle(&AnalyseModel { pressed: Some(true) }, doc, cfg, &mut context)).expect("analyses");
    assert!(emit.artifact_mutations.is_empty() && emit.effects.is_empty(), "an analysis only warms the session");
    semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::with_inference(Some(&instance), &snapshot, |_| ());
    assert!(inference::report(Some(&instance)).gated, "the next read answers from memory");
}

#[semio_framework_async_macros::async_test]
async fn the_analysis_is_stepped_with_progress_and_a_cancel_keeps_what_it_finished() {
    let instance = semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::detached();
    let snapshot = demo();
    let mut cancelled = Analysis::new(Some(&instance), 2);
    let first = cancelled.advance(&snapshot).expect("a step");
    assert!(!first.done && cancelled.fraction() > 0.0);
    cancelled.cancel(&snapshot);
    let report = semio_s_artifact_bim_model_inferences::standards::v1::subsets::any::schema::inferences::model_graph::instance::report(Some(&instance));
    assert!(report.cancelled && report.computed > 0, "{report:?}");
    let mut next = Analysis::new(Some(&instance), 1_000);
    while !next.is_done() {
        next.advance(&snapshot).expect("a step");
    }
    assert!(inference::report(Some(&instance)).reused >= report.computed, "the finished nodes were not computed again");
}
