use super::*;
use crate::editor::fem3d::commands::set_result_animation::{SetResultAnimation, TICK_ACTION};
use crate::editor::fem3d::modes::edit::windows::results;
use crate::editor::fem3d::modes::edit::windows::results::config::Fem3dResultsAnimation;
use crate::editor::fem3d::unit_tests::context::{close, dispatch, fem3d_app, Fem3dApp};
use crate::editor::fem3d::Fem3dCommand;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, InvocationResult, NoConfig, ViewModel, ViewWindowInstance};

fn blank() -> SetResultAnimation {
    SetResultAnimation { phase: None, playing: None, speed: None, loop_mode: None, waveform: None, field: None, value: None, window_id: None }
}

fn results_view() -> ViewModel {
    ViewModel {
        window_id: Some("results-left".into()),
        window_instances: vec![ViewWindowInstance { id: "results-left".into(), window_kind_id: results::FEM3D_WINDOW_RESULTS.into() }],
        ..Default::default()
    }
}

/// 🎚️ The playback state the addressed results window ended up in, read back off its own partition.
async fn published(app: &mut Fem3dApp) -> Fem3dResultsAnimation {
    semio_framework_plugin::artifact_app_laws::capture_fixture_window_config::<results::config::Fem3dResultsWindowConfigOwner, _, _>(app, &results_view()).await.expect("capture results window config").unwrap_or_default().animation
}


/// 🔁️ Drives the publication the dispatch opened all the way into the window-config store, the way
/// the plugin host's own continuation does — a read before the settle would see the PREVIOUS frame.
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
    let result = dispatch(app, Fem3dCommand::ResultAnimationTick(ResultAnimationTick {})).await;
    let settled = settle(app).await;
    rearms(&result, &settled)
}

/// ⏱️ LAW: a tick on a running window advances the phase by exactly one frame and re-arms itself.
#[semio_framework_async_macros::async_test]
async fn result_animation_tick_advances_and_rearms_while_playing() {
    let mut app = fem3d_app();
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
    let mut app = fem3d_app();
    assert_eq!(tick(&mut app).await, 0);
    assert_eq!(published(&mut app).await, Fem3dResultsAnimation::default());
    play(&mut app, SetResultAnimation { playing: Some(true), ..blank() }).await;
    play(&mut app, SetResultAnimation { playing: Some(false), ..blank() }).await;
    assert_eq!(tick(&mut app).await, 0, "a tick arriving after the pause dies out");
    close(&mut app);
}

/// 🔚️ LAW: a `Once` run parks at the end and stops ITSELF — the last tick owes no next frame.
#[semio_framework_async_macros::async_test]
async fn result_animation_tick_stops_itself_at_the_end_of_a_once_run() {
    let mut app = fem3d_app();
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
    let doc = crate::standards::v1::subsets::any::schema::snapshot::text::fem3d_boot_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let view = ArtifactView::new(&doc, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let model = ViewModel { window_id: Some("results-law".into()), window_instances: vec![ViewWindowInstance { id: "results-law".into(), window_kind_id: results::FEM3D_WINDOW_RESULTS.into() }], ..Default::default() };
    let started = crate::editor::fem3d::commands::set_result_animation::handle_window(&SetResultAnimation { phase: None, playing: Some(true), speed: None, loop_mode: None, waveform: None, field: None, value: None, window_id: None }, &view, &cfg, &model).expect("play");
    assert_eq!(started.coalesce_key.as_deref(), Some(PLAYBACK_COALESCE_KEY), "a gesture amends the previous playback edit");
    let mut app = fem3d_app();
    dispatch(&mut app, Fem3dCommand::SetResultAnimation(SetResultAnimation { phase: None, playing: Some(true), speed: None, loop_mode: None, waveform: None, field: None, value: None, window_id: None })).await;
    let ticked = dispatch(&mut app, Fem3dCommand::ResultAnimationTick(ResultAnimationTick {})).await;
    assert!(ticked.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == TICK_ACTION)), "a running tick re-arms");
    close(&mut app);
}

/// 🧾️ LAW: a playback run outlives the store's 64-item applied ledger. Every tick lands on the
/// results window's own config partition under the playback coalesce key, so a run of ticks is ONE
/// amended edit, not one ledger slot per frame — the retained window-config lane dropped that key
/// once and the browser's animation died after ~two seconds with `batched publication requires
/// preinstalled fixed applied and revision capacity`.
#[semio_framework_async_macros::async_test]
async fn result_animation_ticks_coalesce_into_one_ledger_edit() {
    let mut app = fem3d_app();
    play(&mut app, SetResultAnimation { phase: Some(0.0), playing: Some(true), speed: Some(0.25), loop_mode: Some("loop".into()), ..blank() }).await;
    let frames = semio_framework_os_kernel::os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY + 16;
    for frame in 0..frames {
        assert_eq!(tick(&mut app).await, 1, "frame {frame}: the clock keeps re-arming well past the ledger capacity");
    }
    let state = published(&mut app).await;
    assert!(state.playing, "{state:?}");
    let expected = (frames as f64 * ANIMATION_TICK_SECONDS * 0.25).rem_euclid(1.0);
    assert!((state.phase - expected).abs() < 1e-6, "every one of the {frames} frames landed: phase {} vs {expected}", state.phase);
    close(&mut app);
}

/// ⏳️ Laws that drive hundreds of frames through the mounted harness — under the `long` profile.
mod long {
    use super::*;

    /// 🧾️ LAW: a playback run outlives BOTH fixed caps a window-config partition carries under the
    /// reactor's real maintenance budget. Every tick lands on the results window's own partition under
    /// the playback coalesce key, so a run of ticks is ONE amended edit, not one slot of the 64-item
    /// applied ledger per frame — the retained window-config lane dropped that key once and the
    /// browser's animation died after ~two seconds with `batched publication requires preinstalled
    /// fixed applied and revision capacity`. And every amend displaces the previous snapshot/edit/
    /// envelope into the partition's 1 024-slot retirement queue, which only the host's maintenance
    /// pump drains: nothing drained the config-lane stores at all (died after ~340 frames with
    /// `displaced-owner fixed retirement authority is saturated`), and a fair stage in the 26-stage
    /// rotation drained one owner per rotation against three displaced per frame (died again after
    /// ~260 frames in the browser). The reactor now spends ONE fair step per turn and a burst of up to
    /// `LIVE_CLEANUP_PRESSURE_STEPS_PER_TURN` while the app reports pressure — this law pumps exactly
    /// that per frame, one turn per frame, for 1 200 frames: both ceilings are crossed, pressure is
    /// reached and relieved, and no frame is refused.
    #[semio_framework_async_macros::async_test]
    async fn result_animation_ticks_coalesce_and_retire_across_both_store_ceilings() {
        let mut app = fem3d_app();
        play(&mut app, SetResultAnimation { phase: Some(0.0), playing: Some(true), speed: Some(0.25), loop_mode: Some("loop".into()), ..blank() }).await;
        let frames = 1_200;
        let started = std::time::Instant::now();
        let mut tick_seconds = 0.0f64;
        let mut burst_seconds = 0.0f64;
        assert!(frames > semio_framework_os_kernel::os_vcs::ARTIFACT_HISTORY_LEDGER_CAPACITY, "the run crosses the applied ledger");
        let mut retired = 0usize;
        let mut pressured_frames = 0usize;
        let mut burst_steps_max = 0usize;
        for frame in 0..frames {
            let tick_started = std::time::Instant::now();
            assert_eq!(tick(&mut app).await, 1, "frame {frame}: the clock keeps re-arming past both ceilings");
            tick_seconds += tick_started.elapsed().as_secs_f64();
            let burst_started = std::time::Instant::now();
            let mut steps = 0usize;
            let mut pressured = false;
            for _ in 0..semio_framework_plugin::plugin_runtime::LIVE_CLEANUP_PRESSURE_STEPS_PER_TURN {
                if let semio_framework_plugin::PluginCloseStep::Pending { released_items, .. } = semio_framework_plugin::PluginApp::maintenance_step(&mut *app, 1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("maintenance step") {
                    if semio_framework_plugin::app::LAST_MAINTENANCE_STAGE.load(std::sync::atomic::Ordering::Relaxed) == u64::from(semio_framework_plugin::app::MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE) {
                        retired += released_items;
                    }
                }
                steps += 1;
                if !semio_framework_plugin::PluginApp::maintenance_under_pressure(&*app) {
                    break;
                }
                pressured = true;
            }
            burst_seconds += burst_started.elapsed().as_secs_f64();
            pressured_frames += usize::from(pressured);
            burst_steps_max = burst_steps_max.max(steps);
            if frame % 25 == 0 {
                eprintln!("[DEBUG] playback law frame={frame} steps={steps} pressured={pressured} retired={retired} elapsed_s={} tick_s={tick_seconds} burst_s={burst_seconds}", started.elapsed().as_secs_f64());
            }
        }
        assert!(pressured_frames > 0, "the run reached the displaced-owner pressure mark");
        assert!(pressured_frames < frames / 2, "pressure is relieved by the bursts, not permanent: {pressured_frames} of {frames} frames");
        assert!(retired > frames, "the config-lane drain retired the displaced owners the amends produced: {retired} over {frames} frames (widest burst {burst_steps_max} steps)");
        let state = published(&mut app).await;
        assert!(state.playing, "{state:?}");
        let expected = (frames as f64 * ANIMATION_TICK_SECONDS * 0.25).rem_euclid(1.0);
        assert!((state.phase - expected).abs() < 1e-6, "every one of the {frames} frames landed: phase {} vs {expected}", state.phase);
        close(&mut app);
    }
}
