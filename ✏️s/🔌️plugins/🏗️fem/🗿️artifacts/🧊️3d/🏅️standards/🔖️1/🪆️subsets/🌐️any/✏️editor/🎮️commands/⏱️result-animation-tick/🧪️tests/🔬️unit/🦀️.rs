use super::*;
use crate::editor::fem3d::commands::set_result_animation::{SetResultAnimation, TICK_ACTION};
use crate::editor::fem3d::modes::edit::windows::results;
use crate::editor::fem3d::modes::edit::windows::results::config::Fem3dResultsAnimation;
use crate::editor::fem3d::unit_tests::context::{close, dispatch, fem3d_app, Fem3dApp};
use crate::editor::fem3d::Fem3dCommand;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, InvocationResult, NoConfig, ViewModel, ViewWindowInstance};

const WINDOW: &str = "results-left";

fn blank() -> SetResultAnimation {
    SetResultAnimation { phase: None, playing: None, speed: None, loop_mode: None, waveform: None, field: None, value: None, window_id: None }
}

/// 🏷️ A transport gesture as the results panel tags it — naming its window, so the retained route
/// captures the window's running clock alongside its config.
fn tagged() -> SetResultAnimation {
    SetResultAnimation { window_id: Some(WINDOW.into()), ..blank() }
}

fn results_view() -> ViewModel {
    ViewModel { window_id: Some(WINDOW.into()), window_instances: vec![ViewWindowInstance { id: WINDOW.into(), window_kind_id: results::FEM3D_WINDOW_RESULTS.into() }], ..Default::default() }
}

/// 🎚️ The transport the addressed results window ended up with, read back off its own config partition.
async fn published(app: &mut Fem3dApp) -> Fem3dResultsAnimation {
    semio_framework_plugin::artifact_app_laws::capture_fixture_window_config::<results::config::Fem3dResultsWindowConfigOwner, _, _>(app, &results_view()).await.expect("capture results window config").unwrap_or_default().animation
}

/// 🫧️ The running clock of the addressed results window, read back off its own transient partition.
fn clock(app: &mut Fem3dApp) -> Option<Fem3dPlaybackClock> {
    let snapshot = app.window_transient_snapshot(&results_view()).expect("capture results window transient");
    results::transient::captured_clock(snapshot.as_ref(), WINDOW)
}

/// 🔁️ Drives the publication the dispatch opened all the way into the stores, the way the plugin
/// host's own continuation does — a read before the settle would see the PREVIOUS frame.
async fn settle(app: &mut Fem3dApp) -> Vec<Effect> {
    semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut **app, 1).await.expect("settle the typed operation").effects
}

/// 🔁️ How many playback re-arms one gesture asked the host for. Counted off BOTH the dispatch's own
/// answer and the settled receipt, because either surface may be the one that carries them.
fn rearms(dispatched: &InvocationResult, settled: &[Effect]) -> usize {
    let count = |effects: &[Effect]| effects.iter().filter(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == TICK_ACTION)).count();
    count(&dispatched.requested_effects).max(count(settled))
}

/// ⏯️ Dispatches one playback gesture at the results window; answers its re-arm count.
async fn play(app: &mut Fem3dApp, command: SetResultAnimation) -> usize {
    let result = dispatch(app, Fem3dCommand::SetResultAnimation(command)).await;
    let settled = settle(app).await;
    rearms(&result, &settled)
}

/// ⏱️ One clock tick at the results window; answers its re-arm count.
async fn tick(app: &mut Fem3dApp) -> usize {
    let result = dispatch(app, Fem3dCommand::ResultAnimationTick(ResultAnimationTick { window_id: WINDOW.into() })).await;
    let settled = settle(app).await;
    rearms(&result, &settled)
}

/// ⏱️ LAW: a tick on a running window advances the CLOCK by exactly one frame and re-arms itself —
/// the frame lands in the window's transient, and neither the document nor the window config moves.
#[semio_framework_async_macros::async_test]
async fn result_animation_tick_advances_the_clock_and_rearms_while_playing() {
    let mut app = fem3d_app();
    play(&mut app, SetResultAnimation { phase: Some(0.0), playing: Some(true), speed: Some(1.0), ..blank() }).await;
    let before = app.snapshot().expect("snapshot");
    let config_generation = app.window_config_generation(&results_view()).await.expect("config generation");
    assert_eq!(clock(&mut app), None, "the clock only exists once a frame landed");
    assert_eq!(tick(&mut app).await, 1, "a playing tick owes the next frame");
    assert_eq!(app.snapshot().expect("snapshot"), before, "the playback clock never touches the document");
    let running = clock(&mut app).expect("a frame landed in the transient");
    assert!((running.phase - ANIMATION_TICK_SECONDS).abs() < 1e-9, "{running:?}");
    let state = published(&mut app).await;
    assert_eq!(state.phase, 0.0, "a frame never amends the window config");
    assert!(state.playing);
    assert_eq!(app.window_config_generation(&results_view()).await.expect("config generation"), config_generation);
    close(&mut app);
}

/// 🛑️ LAW: a tick that finds the window stopped PARKS a running clock — its phase becomes the
/// resting phase of the transport, the transient is cleared — and re-arms nothing; a tick with no
/// clock to park writes nothing at all. That is the ONE thing that keeps a paused window (or a
/// second chain that raced the first) from spinning the guest.
#[semio_framework_async_macros::async_test]
async fn result_animation_tick_parks_the_clock_when_stopped() {
    let mut app = fem3d_app();
    assert_eq!(tick(&mut app).await, 0);
    assert_eq!(published(&mut app).await, Fem3dResultsAnimation::default());
    assert_eq!(clock(&mut app), None);
    play(&mut app, SetResultAnimation { phase: Some(0.0), playing: Some(true), speed: Some(1.0), ..blank() }).await;
    assert_eq!(tick(&mut app).await, 1);
    assert_eq!(tick(&mut app).await, 1);
    assert_eq!(play(&mut app, SetResultAnimation { playing: Some(false), ..blank() }).await, 0, "the keyboard pause carries no window tag, so the clock is still running");
    assert!(clock(&mut app).is_some());
    assert_eq!(tick(&mut app).await, 0, "the tick arriving after the pause parks the clock and dies out");
    assert_eq!(clock(&mut app), None);
    let state = published(&mut app).await;
    assert!(!state.playing);
    assert!((state.phase - 2.0 * ANIMATION_TICK_SECONDS).abs() < 1e-9, "the transport rests where the animation was: {state:?}");
    assert_eq!(tick(&mut app).await, 0, "nothing left to park");
    close(&mut app);
}

/// 🔚️ LAW: a `Once` run parks at the end and stops ITSELF — the last tick stops the transport at
/// phase 1, clears the clock and owes no next frame.
#[semio_framework_async_macros::async_test]
async fn result_animation_tick_stops_itself_at_the_end_of_a_once_run() {
    let mut app = fem3d_app();
    play(&mut app, SetResultAnimation { phase: Some(0.99), playing: Some(true), speed: Some(4.0), loop_mode: Some("once".into()), ..blank() }).await;
    assert_eq!(tick(&mut app).await, 0, "the final frame of a once run must not re-arm");
    let state = published(&mut app).await;
    assert_eq!(state.phase, 1.0);
    assert!(!state.playing);
    assert_eq!(clock(&mut app), None);
    close(&mut app);
}

/// ⏸️ LAW: a transport gesture the panel tagged with its window rests exactly where the running
/// clock is — a pause keeps the frame on screen, a play resumes from it — and clears the clock so
/// the chain restarts from the published transport instead of a stale frame.
#[semio_framework_async_macros::async_test]
async fn tagged_transport_gestures_rest_where_the_clock_runs() {
    let mut app = fem3d_app();
    assert_eq!(play(&mut app, SetResultAnimation { phase: Some(0.0), playing: Some(true), speed: Some(1.0), ..tagged() }).await, 1);
    for _ in 0..3 {
        assert_eq!(tick(&mut app).await, 1);
    }
    assert_eq!(play(&mut app, SetResultAnimation { playing: Some(false), ..tagged() }).await, 0);
    assert_eq!(clock(&mut app), None, "the pause cleared the clock itself");
    let paused = published(&mut app).await;
    assert!(!paused.playing);
    assert!((paused.phase - 3.0 * ANIMATION_TICK_SECONDS).abs() < 1e-9, "{paused:?}");
    assert_eq!(tick(&mut app).await, 0, "no clock left for the straggling tick to park");
    assert_eq!(play(&mut app, SetResultAnimation { playing: Some(true), ..tagged() }).await, 1, "resuming arms one chain");
    assert_eq!(tick(&mut app).await, 1);
    let resumed = clock(&mut app).expect("the chain runs again");
    assert!((resumed.phase - 4.0 * ANIMATION_TICK_SECONDS).abs() < 1e-9, "the run resumes from the parked phase: {resumed:?}");
    assert_eq!(play(&mut app, SetResultAnimation { field: Some("speed".into()), value: Some("2".into()), ..tagged() }).await, 0, "retuning mid-playback arms nothing");
    assert_eq!(clock(&mut app), None, "the retune cleared the clock so the next frame starts from the published phase");
    let retuned = published(&mut app).await;
    assert!(retuned.playing);
    assert!((retuned.phase - 4.0 * ANIMATION_TICK_SECONDS).abs() < 1e-9, "the retune published the frame the user saw: {retuned:?}");
    assert_eq!(tick(&mut app).await, 1);
    let next = clock(&mut app).expect("the chain runs on");
    assert!((next.phase - 6.0 * ANIMATION_TICK_SECONDS).abs() < 1e-9, "one frame at the new speed from the published phase: {next:?}");
    close(&mut app);
}

/// 🪢️ LAW: a transport gesture amends the previous playback edit — the window-config store's
/// applied-edit capacity is fixed — and a running tick re-arms through the retained route.
#[semio_framework_async_macros::async_test]
async fn playback_publications_coalesce_and_the_retained_tick_rearms() {
    let doc = crate::standards::v1::subsets::any::schema::snapshot::text::fem3d_boot_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let view = ArtifactView::new(&doc, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let model = ViewModel { window_id: Some("results-law".into()), window_instances: vec![ViewWindowInstance { id: "results-law".into(), window_kind_id: results::FEM3D_WINDOW_RESULTS.into() }], ..Default::default() };
    let started = crate::editor::fem3d::commands::set_result_animation::handle_window(&SetResultAnimation { playing: Some(true), ..blank() }, &view, &cfg, &model).expect("play");
    assert_eq!(started.coalesce_key.as_deref(), Some(PLAYBACK_COALESCE_KEY), "a gesture amends the previous playback edit");
    assert!(handle_window(&ResultAnimationTick { window_id: "results-law".into() }, &view, &cfg, &model).is_err(), "the batch route has no transient to land a frame in");
    let mut app = fem3d_app();
    dispatch(&mut app, Fem3dCommand::SetResultAnimation(SetResultAnimation { playing: Some(true), ..blank() })).await;
    let ticked = dispatch(&mut app, Fem3dCommand::ResultAnimationTick(ResultAnimationTick { window_id: WINDOW.into() })).await;
    assert!(ticked.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == TICK_ACTION)), "a running tick re-arms");
    close(&mut app);
}

/// 🧾️ LAW: a playback run never touches the window-config store — its 64-item applied ledger and
/// its 1 024-slot displaced-owner queue are not in the frame's path at all. The frames land in the
/// transient partition, and the config generation after a run past the ledger capacity is the one
/// the play gesture left.
#[semio_framework_async_macros::async_test]
async fn result_animation_frames_never_touch_the_window_config_ledger() {
    let mut app = fem3d_app();
    play(&mut app, SetResultAnimation { phase: Some(0.0), playing: Some(true), speed: Some(0.25), loop_mode: Some("loop".into()), ..blank() }).await;
    let config_generation = app.window_config_generation(&results_view()).await.expect("config generation");
    let frames = semio_framework_os_kernel::os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY + 16;
    for frame in 0..frames {
        assert_eq!(tick(&mut app).await, 1, "frame {frame}: the clock keeps re-arming well past the ledger capacity");
    }
    assert_eq!(app.window_config_generation(&results_view()).await.expect("config generation"), config_generation, "no frame amended the window config");
    assert!(!semio_framework_plugin::PluginApp::maintenance_under_pressure(&*app), "no frame displaced a config-lane owner");
    let running = clock(&mut app).expect("the clock runs");
    let expected = (frames as f64 * ANIMATION_TICK_SECONDS * 0.25).rem_euclid(1.0);
    assert!((running.phase - expected).abs() < 1e-6, "every one of the {frames} frames landed: phase {} vs {expected}", running.phase);
    close(&mut app);
}

/// ⏳️ Laws that drive hundreds of frames through the mounted harness — under the `long` profile.
mod long {
    use super::*;

    /// 🧾️ LAW: the cost of a frame does not grow with the frames already played. The clock used to
    /// live in the window config: every frame amended ONE coalesced edit, but that edit accumulated
    /// every frame's operation and the store re-digested the whole edit on each amend, so the tick
    /// went from 0.2 ms to 240 ms inside 300 frames (and the browser's tick period tripled inside a
    /// minute). On the transient partition a frame is one root swap. 1 200 frames under the
    /// reactor's real maintenance budget — one fair step per frame — with the last fifth of the run
    /// costing no more than four times its first fifth, no maintenance pressure ever reported, and the
    /// window config untouched.
    #[semio_framework_async_macros::async_test]
    async fn result_animation_frame_cost_stays_flat_across_a_long_run() {
        let mut app = fem3d_app();
        play(&mut app, SetResultAnimation { phase: Some(0.0), playing: Some(true), speed: Some(0.25), loop_mode: Some("loop".into()), ..blank() }).await;
        let config_generation = app.window_config_generation(&results_view()).await.expect("config generation");
        let frames = 1_200usize;
        let fifth = frames / 5;
        let mut first = std::time::Duration::ZERO;
        let mut last = std::time::Duration::ZERO;
        for frame in 0..frames {
            let started = std::time::Instant::now();
            assert_eq!(tick(&mut app).await, 1, "frame {frame}: the clock keeps re-arming");
            let elapsed = started.elapsed();
            if frame < fifth {
                first += elapsed;
            } else if frame >= frames - fifth {
                last += elapsed;
            }
            semio_framework_plugin::PluginApp::maintenance_step(&mut *app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one fair maintenance step per frame");
            assert!(!semio_framework_plugin::PluginApp::maintenance_under_pressure(&*app), "frame {frame}: a frame never displaces a store owner");
        }
        assert_eq!(app.window_config_generation(&results_view()).await.expect("config generation"), config_generation, "no frame amended the window config");
        assert!(last <= first * 4, "the last {fifth} frames took {last:?} against {first:?} for the first {fifth}: the frame cost grows with the run");
        let running = clock(&mut app).expect("the clock runs");
        let expected = (frames as f64 * ANIMATION_TICK_SECONDS * 0.25).rem_euclid(1.0);
        assert!((running.phase - expected).abs() < 1e-6, "every one of the {frames} frames landed: phase {} vs {expected}", running.phase);
        close(&mut app);
    }
}
