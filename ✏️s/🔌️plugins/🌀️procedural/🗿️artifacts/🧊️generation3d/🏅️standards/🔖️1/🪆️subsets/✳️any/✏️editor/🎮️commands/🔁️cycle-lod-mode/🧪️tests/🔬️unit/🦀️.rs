use super::*;
use crate::editor::generation3d::config::GENERATION_3D_LOD_MODES;
use crate::editor::generation3d::unit_tests::context::{self, app, dispatch};
use crate::editor::generation3d::Generation3dCommand;

/// ⚖️ LAW: the level-of-detail ladder cycles over exactly the rungs the flow window's picker offers,
/// and an unset `lod_mode` steps off the `medium` that picker displays for it.
#[test]
fn the_lod_mode_ladder_is_a_total_cycle() {
    let mut seen = Vec::new();
    let mut mode = GENERATION_3D_LOD_MODES[0].to_string();
    for _ in 0..GENERATION_3D_LOD_MODES.len() {
        seen.push(mode.clone());
        mode = next_lod_mode(&mode);
    }
    assert_eq!(seen, GENERATION_3D_LOD_MODES.iter().map(|mode| (*mode).to_string()).collect::<Vec<_>>(), "one cycle walks the ladder in order");
    assert_eq!(mode, GENERATION_3D_LOD_MODES[0], "the cycle wraps");
    assert_eq!(next_lod_mode(""), "fine", "an unset config displays `medium`, so its next step is `fine`");
    assert_eq!(next_lod_mode("nonsense"), GENERATION_3D_LOD_MODES[0], "an unknown mode re-enters at the first rung");
}

/// ⚖️ LAW: a view verb, like its show-mode twin.
#[semio_framework_async_macros::async_test]
async fn cycle_lod_mode_is_a_view_action_with_no_artifact_mutations() {
    let _serial = crate::editor::generation3d::unit_tests::serial_execution::lock();
    let mut app = app().await;
    let before = context::snapshot(&app);
    dispatch(&mut app, Generation3dCommand::CycleLodMode(CycleLodMode {})).await;
    assert_eq!(context::snapshot(&app), before, "cycleLodMode must not mutate the document");
}
