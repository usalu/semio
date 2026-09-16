use super::*;
use crate::editor::fem2d::commands::set_result_animation::{SetResultAnimation, TICK_ACTION};
use crate::editor::fem2d::modes::edit::windows::results;
use crate::editor::fem2d::unit_tests::context::{dispatch, fem2d_app, Fem2dApp};
use crate::editor::fem2d::Fem2dCommand;
use semio_framework_plugin::{Effect, InvocationResult, ViewModel, ViewWindowInstance};

fn blank() -> SetResultAnimation {
    SetResultAnimation { phase: None, playing: None, speed: None, loop_mode: None, waveform: None, field: None, value: None }
}

fn results_view() -> ViewModel {
    ViewModel {
        window_id: Some("results-left".into()),
        window_instances: vec![ViewWindowInstance { id: "results-left".into(), window_kind_id: results::WINDOW_KIND_ID.into() }],
        ..Default::default()
    }
}

/// 🎚️ The playback state the addressed results window ended up in, read back off its own partition.
async fn published(app: &mut Fem2dApp) -> results::config::Fem2dResultsAnimation {
    semio_framework_plugin::artifact_app_laws::capture_fixture_window_config::<results::config::Fem2dResultsWindowConfigOwner, _, _>(app, &results_view()).await.expect("capture results window config").unwrap_or_default().animation
}

fn rearms(result: &InvocationResult) -> usize {
    result
        .requested_effects
        .iter()
        .filter(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == TICK_ACTION))
        .count()
}

async fn tick(app: &mut Fem2dApp) -> InvocationResult {
    dispatch(app, Fem2dCommand::ResultAnimationTick(ResultAnimationTick {})).await
}

/// ⏱️ LAW: a tick on a running window advances the phase by exactly one frame and re-arms itself.
#[semio_framework_async_macros::async_test]
async fn result_animation_tick_advances_and_rearms_while_playing() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetResultAnimation(SetResultAnimation { phase: Some(0.0), playing: Some(true), speed: Some(1.0), ..blank() })).await;
    let ticked = tick(&mut app).await;
    assert_eq!(rearms(&ticked), 1, "a playing tick owes the next frame");
    assert!(ticked.mutations.is_empty(), "the playback clock never touches the document");
    let state = published(&mut app).await;
    assert!((state.phase - results::config::ANIMATION_TICK_SECONDS).abs() < 1e-9, "{state:?}");
    assert!(state.playing);
}

/// 🛑️ LAW: a tick that finds the window stopped writes nothing and re-arms nothing — the ONE thing
/// that keeps a paused window (or a second chain that raced the first) from spinning the guest.
#[semio_framework_async_macros::async_test]
async fn result_animation_tick_is_inert_when_stopped() {
    let mut app = fem2d_app();
    let idle = tick(&mut app).await;
    assert_eq!(rearms(&idle), 0);
    assert!(idle.mutations.is_empty());
    assert_eq!(published(&mut app).await, results::config::Fem2dResultsAnimation::default());
    dispatch(&mut app, Fem2dCommand::SetResultAnimation(SetResultAnimation { playing: Some(true), ..blank() })).await;
    dispatch(&mut app, Fem2dCommand::SetResultAnimation(SetResultAnimation { playing: Some(false), ..blank() })).await;
    assert_eq!(rearms(&tick(&mut app).await), 0, "a tick arriving after the pause dies out");
}

/// 🔚️ LAW: a `Once` run parks at the end and stops ITSELF — the last tick owes no next frame.
#[semio_framework_async_macros::async_test]
async fn result_animation_tick_stops_itself_at_the_end_of_a_once_run() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetResultAnimation(SetResultAnimation { phase: Some(0.99), playing: Some(true), speed: Some(4.0), loop_mode: Some("once".into()), ..blank() })).await;
    let last = tick(&mut app).await;
    assert_eq!(rearms(&last), 0, "the final frame of a once run must not re-arm");
    let state = published(&mut app).await;
    assert_eq!(state.phase, 1.0);
    assert!(!state.playing);
}
