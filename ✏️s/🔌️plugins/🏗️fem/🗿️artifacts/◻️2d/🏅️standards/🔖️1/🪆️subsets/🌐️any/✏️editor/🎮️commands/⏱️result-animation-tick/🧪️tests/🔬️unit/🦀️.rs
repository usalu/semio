use super::*;
use crate::editor::fem2d::commands::set_result_animation::{SetResultAnimation, TICK_ACTION};
use crate::editor::fem2d::modes::edit::windows::results;
use crate::editor::fem2d::modes::edit::windows::results::config::Fem2dResultsAnimation;
use crate::editor::fem2d::unit_tests::context::{close, dispatch, fem2d_mounted_app, Fem2dApp};
use crate::editor::fem2d::Fem2dCommand;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, InvocationResult, NoConfig, ViewModel, ViewWindowInstance};

fn blank() -> SetResultAnimation {
    SetResultAnimation { phase: None, playing: None, speed: None, loop_mode: None, waveform: None, field: None, value: None, window_id: None }
}

fn results_view() -> ViewModel {
    ViewModel {
        window_id: Some("results-left".into()),
        window_instances: vec![ViewWindowInstance { id: "results-left".into(), window_kind_id: results::WINDOW_KIND_ID.into() }],
        ..Default::default()
    }
}

/// 🎚️ The playback state the addressed results window ended up in, read back off its own partition.
async fn published(app: &mut Fem2dApp) -> Fem2dResultsAnimation {
    semio_framework_plugin::artifact_app_laws::capture_fixture_window_config::<results::config::Fem2dResultsWindowConfigOwner, _, _>(app, &results_view()).await.expect("capture results window config").unwrap_or_default().animation
}


/// 🔁️ Drives the publication the dispatch opened all the way into the window-config store, the way
/// the plugin host's own continuation does — a read before the settle would see the PREVIOUS frame.
async fn settle(app: &mut Fem2dApp) -> Vec<Effect> {
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut **app, 1).await.expect("settle the typed operation").effects
}

/// 🔁️ How many playback re-arms one gesture asked the host for. Counted off BOTH the dispatch's own
/// answer and the settled receipt, because either surface may be the one that carries them.
fn rearms(dispatched: &InvocationResult, settled: &[Effect]) -> usize {
    let count = |effects: &[Effect]| effects.iter().filter(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == TICK_ACTION)).count();
    count(&dispatched.requested_effects).max(count(settled))
}

/// ⏯️ Dispatches one playback gesture at the results window; answers its re-arm count.
async fn play(app: &mut Fem2dApp, command: SetResultAnimation) -> usize {
    let result = dispatch(app, Fem2dCommand::SetResultAnimation(command)).await;
    let settled = settle(app).await;
    rearms(&result, &settled)
}

/// ⏱️ One clock tick at the results window; answers its re-arm count.
async fn tick(app: &mut Fem2dApp) -> usize {
    let result = dispatch(app, Fem2dCommand::ResultAnimationTick(ResultAnimationTick {})).await;
    let settled = settle(app).await;
    rearms(&result, &settled)
}

/// ⏱️ LAW: a tick on a running window advances the phase by exactly one frame and re-arms itself.
#[semio_framework_async_macros::async_test]
async fn result_animation_tick_advances_and_rearms_while_playing() {
    let mut app = fem2d_mounted_app();
    play(&mut app, SetResultAnimation { phase: Some(0.0), playing: Some(true), speed: Some(1.0), ..blank() }).await;
    let before = app.snapshot().expect("snapshot");
    assert_eq!(tick(&mut app).await, 1, "a playing tick owes the next frame");
    assert_eq!(app.snapshot().expect("snapshot"), before, "the playback clock never touches the document");
    let state = published(&mut app).await;
    assert!((state.phase - ANIMATION_TICK_SECONDS).abs() < 1e-9, "{state:?}");
    assert!(state.playing);
    close(&mut app);
}

/// 🛑️ LAW: a tick that finds the window stopped writes nothing and re-arms nothing — the ONE thing
/// that keeps a paused window (or a second chain that raced the first) from spinning the guest.
#[semio_framework_async_macros::async_test]
async fn result_animation_tick_is_inert_when_stopped() {
    let mut app = fem2d_mounted_app();
    assert_eq!(tick(&mut app).await, 0);
    assert_eq!(published(&mut app).await, Fem2dResultsAnimation::default());
    play(&mut app, SetResultAnimation { playing: Some(true), ..blank() }).await;
    play(&mut app, SetResultAnimation { playing: Some(false), ..blank() }).await;
    assert_eq!(tick(&mut app).await, 0, "a tick arriving after the pause dies out");
    close(&mut app);
}

/// 🔚️ LAW: a `Once` run parks at the end and stops ITSELF — the last tick owes no next frame.
#[semio_framework_async_macros::async_test]
async fn result_animation_tick_stops_itself_at_the_end_of_a_once_run() {
    let mut app = fem2d_mounted_app();
    play(&mut app, SetResultAnimation { phase: Some(0.99), playing: Some(true), speed: Some(4.0), loop_mode: Some("once".into()), ..blank() }).await;
    assert_eq!(tick(&mut app).await, 0, "the final frame of a once run must not re-arm");
    let state = published(&mut app).await;
    assert_eq!(state.phase, 1.0);
    assert!(!state.playing);
    close(&mut app);
}

/// 🪢️ LAW: a tick amends the previous playback edit — the window-config store's applied-edit
/// capacity is fixed, so an appending clock would stop after a few minutes.
#[semio_framework_async_macros::async_test]
async fn result_animation_tick_coalesces_its_publication() {
    let doc = crate::standards::v1::subsets::any::schema::default_fem2d_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let view = ArtifactView::new(&doc, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let model = ViewModel { window_id: Some("results-law".into()), window_instances: vec![ViewWindowInstance { id: "results-law".into(), window_kind_id: results::WINDOW_KIND_ID.into() }], ..Default::default() };
    let started = crate::editor::fem2d::commands::set_result_animation::handle_window(&SetResultAnimation { phase: None, playing: Some(true), speed: None, loop_mode: None, waveform: None, field: None, value: None, window_id: None }, &view, &cfg, &model).expect("play");
    assert_eq!(started.coalesce_key.as_deref(), Some(PLAYBACK_COALESCE_KEY), "a gesture amends the previous playback edit");
    let mut app = fem2d_mounted_app();
    dispatch(&mut app, Fem2dCommand::SetResultAnimation(SetResultAnimation { phase: None, playing: Some(true), speed: None, loop_mode: None, waveform: None, field: None, value: None, window_id: None })).await;
    let ticked = dispatch(&mut app, Fem2dCommand::ResultAnimationTick(ResultAnimationTick {})).await;
    assert!(ticked.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == TICK_ACTION)), "a running tick re-arms");
    close(&mut app);
}
