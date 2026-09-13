use super::*;
use crate::editor::generation3d::config::GENERATION_3D_SHOW_MODES;
use crate::editor::generation3d::unit_tests::context::{self, app, dispatch};
use crate::editor::generation3d::Generation3dCommand;

/// ⚖️ LAW: the ladder is a cycle over exactly the modes the picker offers, and anything off the
/// ladder — the empty string a never-set config carries included — lands on its first entry.
#[test]
fn the_show_mode_ladder_is_a_total_cycle() {
    let mut seen = Vec::new();
    let mut mode = GENERATION_3D_SHOW_MODES[0].to_string();
    for _ in 0..GENERATION_3D_SHOW_MODES.len() {
        seen.push(mode.clone());
        mode = next_show_mode(&mode);
    }
    assert_eq!(seen, GENERATION_3D_SHOW_MODES.iter().map(|mode| (*mode).to_string()).collect::<Vec<_>>(), "one cycle walks the ladder in order");
    assert_eq!(mode, GENERATION_3D_SHOW_MODES[0], "the cycle wraps");
    assert_eq!(next_show_mode(""), GENERATION_3D_SHOW_MODES[1], "an unset config shows `shaded`, so its next step is the second rung");
    assert_eq!(next_show_mode("nonsense"), GENERATION_3D_SHOW_MODES[0], "an unknown mode re-enters at the first rung");
}

/// ⚖️ LAW: it is a VIEW verb — the whole point of binding it to a chord is that it costs the
/// document nothing.
#[semio_framework_async_macros::async_test]
async fn cycle_show_mode_is_a_view_action_with_no_artifact_mutations() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let before = context::snapshot(&app);
    dispatch(&mut app, Generation3dCommand::CycleShowMode(CycleShowMode {})).await;
    assert_eq!(context::snapshot(&app), before, "cycleShowMode must not mutate the document");
}
