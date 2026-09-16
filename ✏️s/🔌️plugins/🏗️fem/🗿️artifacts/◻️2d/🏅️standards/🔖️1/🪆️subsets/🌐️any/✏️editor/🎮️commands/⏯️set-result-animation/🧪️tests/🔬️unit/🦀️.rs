use super::*;
use crate::editor::fem2d::modes::edit::windows::results;
use crate::editor::fem2d::unit_tests::context::{close, dispatch, fem2d_mounted_app, Fem2dApp};
use crate::editor::fem2d::Fem2dCommand;
use semio_framework_plugin::{Effect, InvocationResult, ViewModel, ViewWindowInstance};

fn animation(command: SetResultAnimation) -> Fem2dCommand {
    Fem2dCommand::SetResultAnimation(command)
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
    let result = dispatch(app, animation(command)).await;
    let settled = settle(app).await;
    rearms(&result, &settled)
}

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

/// ▶️ LAW: pressing play arms EXACTLY one clock; pressing it again while it runs arms none — two
/// chains on one window would double the frame rate and never slow down again.
#[semio_framework_async_macros::async_test]
async fn set_result_animation_arms_one_clock_when_playback_starts() {
    let mut app = fem2d_mounted_app();
    let started = play(&mut app, SetResultAnimation { playing: Some(true), ..blank() }).await;
    assert_eq!(started, 1, "playing must arm the tick chain");
    assert!(published(&mut app).await.playing);
    let again = play(&mut app, SetResultAnimation { phase: Some(0.5), ..blank() }).await;
    assert_eq!(again, 0, "a second gesture while playing must not arm a second clock");
    assert_eq!(published(&mut app).await.phase, 0.5);
    let stopped = play(&mut app, SetResultAnimation { playing: Some(false), ..blank() }).await;
    assert_eq!(stopped, 0);
    assert!(!published(&mut app).await.playing);
    close(&mut app);
}

/// 🧾️ LAW: playback is window-config only — the document never moves.
#[semio_framework_async_macros::async_test]
async fn set_result_animation_never_touches_the_document() {
    let mut app = fem2d_mounted_app();
    let before = app.snapshot().expect("snapshot");
    let result = dispatch(&mut app, animation(SetResultAnimation { playing: Some(true), ..blank() })).await;
    settle(&mut app).await;
    assert!(result.mutations.is_empty(), "setResultAnimation must not emit a document VCS operation");
    assert_eq!(app.snapshot().expect("snapshot"), before);
    close(&mut app);
}

/// ␣ LAW: the `space` chord dispatches the action with NO arguments at all — that gesture toggles.
#[semio_framework_async_macros::async_test]
async fn set_result_animation_without_arguments_toggles_play_pause() {
    let mut app = fem2d_mounted_app();
    let on = play(&mut app, blank()).await;
    assert_eq!(on, 1);
    assert!(published(&mut app).await.playing);
    let off = play(&mut app, blank()).await;
    assert_eq!(off, 0);
    assert!(!published(&mut app).await.playing);
    close(&mut app);
}

/// 🎛️ LAW: the `{field, value}` vocabulary a persistent panel control speaks reaches every
/// transport field, including the relative phase step the two step buttons use.
#[semio_framework_async_macros::async_test]
async fn set_result_animation_applies_one_named_field() {
    let mut app = fem2d_mounted_app();
    for (field, value) in [("phase", "0.25"), ("speed", "2"), ("loopMode", "pingPong"), ("waveform", "sine"), ("reverse", "true")] {
        play(&mut app, SetResultAnimation { field: Some(field.into()), value: Some(value.into()), ..blank() }).await;
    }
    let state = published(&mut app).await;
    assert_eq!(state.phase, 0.25);
    assert_eq!(state.speed, 2.0);
    assert_eq!(state.loop_mode, Fem2dLoopMode::PingPong);
    assert_eq!(state.waveform, Fem2dWaveform::Sine);
    assert!(state.reverse);
    play(&mut app, SetResultAnimation { field: Some("phaseStep".into()), value: Some("0.5".into()), ..blank() }).await;
    assert!((published(&mut app).await.phase - 0.75).abs() < 1e-9);
    play(&mut app, SetResultAnimation { field: Some("playing".into()), value: Some("true".into()), ..blank() }).await;
    assert!(published(&mut app).await.playing);
    close(&mut app);
}

/// 🚫️ LAW: an unknown field or an unparseable value is refused outright rather than silently
/// writing a default over the window's state.
#[semio_framework_async_macros::async_test]
async fn set_result_animation_refuses_unknown_fields_and_values() {
    let mut animation = Fem2dResultsAnimation::default();
    assert!(apply_field(&mut animation, "tempo", "1").is_err());
    assert!(apply_field(&mut animation, "speed", "fast").is_err());
    assert!(apply_field(&mut animation, "loopMode", "bounce").is_err());
    assert!(apply_field(&mut animation, "waveform", "square").is_err());
}

/// 🔁️ LAW: the re-arm names the tick action, carries the window it was armed for, and asks for the
/// fixed frame delay.
#[semio_framework_async_macros::async_test]
async fn rearm_effect_addresses_the_window_at_the_frame_delay() {
    let Effect::DispatchAction { action, args, delay_ms, .. } = rearm_effect("results-left") else { panic!("expected a dispatch-action re-arm") };
    assert_eq!(action, TICK_ACTION);
    assert_eq!(delay_ms, ANIMATION_TICK_MS);
    assert_eq!(args.expect("re-arm args").get("windowId").and_then(dsl::DslValue::as_str), Some("results-left"));
}

/// 🪟️ LAW: the `windowId` a panel control tags onto its own arguments reaches the handler and picks
/// the partition, so a split layout retunes the results pane the control belongs to — NOT whichever
/// pane happens to be focused. A tag naming a window that is not an open results window, or one that
/// disagrees with the configuration actually captured, is refused by name.
#[semio_framework_async_macros::async_test]
async fn set_result_animation_honours_the_window_a_control_tagged() {
    let snapshot = crate::standards::v1::subsets::any::schema::default_fem2d_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(&snapshot, &history);
    let config = NoConfig::default();
    let cfg = ConfigView { snapshot: &config, window: None };
    let split = ViewModel {
        window_id: Some("results-left".into()),
        window_instances: vec![
            ViewWindowInstance { id: "results-left".into(), window_kind_id: results::WINDOW_KIND_ID.into() },
            ViewWindowInstance { id: "results-right".into(), window_kind_id: results::WINDOW_KIND_ID.into() },
            ViewWindowInstance { id: "model-left".into(), window_kind_id: crate::editor::fem2d::modes::edit::windows::model::WINDOW_KIND_ID.into() },
        ],
        ..Default::default()
    };
    let tagged = |window_id: &str| SetResultAnimation { playing: Some(true), window_id: Some(window_id.into()), ..blank() };
    let emit = handle_window(&tagged("results-right"), &doc, &cfg, &split).expect("a tagged control addresses its own pane");
    assert_eq!(emit.window_config_mutations.iter().map(semio_framework_plugin::WindowConfigMutation::window_id).collect::<Vec<_>>(), ["results-right"]);
    let Effect::DispatchAction { args, .. } = emit.effects.first().expect("the tagged gesture arms its own clock") else { panic!("expected a dispatch-action re-arm") };
    assert_eq!(args.as_ref().and_then(|args| args.get("windowId")).and_then(dsl::DslValue::as_str), Some("results-right"));
    let untagged = handle_window(&SetResultAnimation { playing: Some(true), ..blank() }, &doc, &cfg, &split).expect("an untagged gesture falls back to the addressed window");
    assert_eq!(untagged.window_config_mutations.iter().map(semio_framework_plugin::WindowConfigMutation::window_id).collect::<Vec<_>>(), ["results-left"]);
    assert!(handle_window(&tagged("model-left"), &doc, &cfg, &split).is_err(), "a tag naming a model window is refused");
    assert!(handle_window(&tagged("results-gone"), &doc, &cfg, &split).is_err(), "a tag naming a closed window is refused");
}
