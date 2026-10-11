//! 🧪️ The replay cursor is view state: every move lands on the config lane, clamps to the timeline and never
//! touches the document.

use super::*;
use crate::{Pose, ProcessMeasure, ProcessStep, ProcessWorkingScene, Stock, Workshop};
use semio_framework_plugin::HistoryView;

fn timeline(steps: usize) -> Process3dSnapshot {
    let step = |index: usize| ProcessStep { id: format!("step-{index}"), label: format!("Step {index}"), enabled: true, origin: None, measure: ProcessMeasure::Drill { radius: 0.01, depth: 0.05, pose: Pose::default() } };
    crate::process_working_scene_to_snapshot(&ProcessWorkingScene { stock: Stock::default(), steps: (0..steps).map(step).collect() }, Workshop::default())
}

fn moved(snapshot: &Process3dSnapshot, config: &Process3dConfig, command: impl FnOnce(&ArtifactView<'_, Process3dSnapshot>, &ConfigView<'_, Process3dConfig>, &mut crate::editor::process3d::Process3dDispatchCtx) -> Result<Emit<Process3dMutation, Process3dConfigMutation>, Fault>) -> Emit<Process3dMutation, Process3dConfigMutation> {
    let history = HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let cfg = ConfigView { snapshot: config, window: None };
    let mut ctx = crate::editor::process3d::Process3dDispatchCtx { interaction: Default::default(), view_state: None };
    command(&doc, &cfg, &mut ctx).expect("the cursor moves")
}

#[semio_framework_async_macros::async_test]
async fn an_unset_cursor_resolves_every_step_and_a_set_one_clamps() {
    let snapshot = timeline(4);
    assert_eq!(process3d_cursor(&snapshot, &Process3dConfig::default()), 4);
    assert_eq!(process3d_cursor(&snapshot, &Process3dConfig { resolved_up_to: Some(2), ..Process3dConfig::default() }), 2);
    assert_eq!(process3d_cursor(&snapshot, &Process3dConfig { resolved_up_to: Some(9), ..Process3dConfig::default() }), 4);
}

#[semio_framework_async_macros::async_test]
async fn every_cursor_verb_is_one_config_write_and_no_document_edit() {
    let snapshot = timeline(4);
    let config = Process3dConfig::default();
    let back = moved(&snapshot, &config, |doc, cfg, ctx| step_cursor_back::handle(&step_cursor_back::StepCursorBack {}, doc, cfg, ctx));
    assert!(back.artifact_mutations.is_empty(), "the cursor is never a document mutation");
    assert_eq!(back.config_mutations, vec![Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ value: Some(3) })], "back from every step resolves one fewer");
    let forward = moved(&snapshot, &Process3dConfig { resolved_up_to: Some(4), ..Process3dConfig::default() }, |doc, cfg, ctx| step_cursor_forward::handle(&step_cursor_forward::StepCursorForward {}, doc, cfg, ctx));
    assert!(forward.config_mutations.is_empty(), "forward past the timeline end clamps to where it already is");
    let set = moved(&snapshot, &config, |doc, cfg, ctx| set_cursor::handle(&set_cursor::SetCursor { value: Some(99) }, doc, cfg, ctx));
    assert_eq!(set.config_mutations, vec![Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ value: Some(4) })]);
    let step = moved(&snapshot, &Process3dConfig { resolved_up_to: Some(1), ..Process3dConfig::default() }, |doc, cfg, ctx| step_cursor::handle(&step_cursor::StepCursor { delta: -5 }, doc, cfg, ctx));
    assert_eq!(step.config_mutations, vec![Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ value: Some(0) })], "a step below zero clamps to the bare stock");
}
