use super::*;
use crate::editor::fem2d::modes::edit::windows::results;
use crate::editor::fem2d::unit_tests::context::{dispatch, fem2d_app, Fem2dApp};
use crate::editor::fem2d::Fem2dCommand;
use semio_framework_plugin::{InvocationResult, ViewModel, ViewWindowInstance};

fn animation(command: SetResultAnimation) -> Fem2dCommand {
    Fem2dCommand::SetResultAnimation(command)
}

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

/// ▶️ LAW: pressing play arms EXACTLY one clock; pressing it again while it runs arms none — two
/// chains on one window would double the frame rate and never slow down again.
#[semio_framework_async_macros::async_test]
async fn set_result_animation_arms_one_clock_when_playback_starts() {
    let mut app = fem2d_app();
    let started = dispatch(&mut app, animation(SetResultAnimation { playing: Some(true), ..blank() })).await;
    assert_eq!(rearms(&started), 1, "playing must arm the tick chain");
    assert!(started.mutations.is_empty(), "playback is window config only");
    assert!(published(&mut app).await.playing);
    let again = dispatch(&mut app, animation(SetResultAnimation { phase: Some(0.5), ..blank() })).await;
    assert_eq!(rearms(&again), 0, "a second gesture while playing must not arm a second clock");
    assert_eq!(published(&mut app).await.phase, 0.5);
    let stopped = dispatch(&mut app, animation(SetResultAnimation { playing: Some(false), ..blank() })).await;
    assert_eq!(rearms(&stopped), 0);
    assert!(!published(&mut app).await.playing);
}

/// ␣ LAW: the `space` chord dispatches the action with NO arguments at all — that gesture toggles.
#[semio_framework_async_macros::async_test]
async fn set_result_animation_without_arguments_toggles_play_pause() {
    let mut app = fem2d_app();
    let on = dispatch(&mut app, animation(blank())).await;
    assert_eq!(rearms(&on), 1);
    assert!(published(&mut app).await.playing);
    let off = dispatch(&mut app, animation(blank())).await;
    assert_eq!(rearms(&off), 0);
    assert!(!published(&mut app).await.playing);
}

/// 🎛️ LAW: the `{field, value}` vocabulary a persistent panel control speaks reaches every
/// transport field, including the relative phase step the two step buttons use.
#[semio_framework_async_macros::async_test]
async fn set_result_animation_applies_one_named_field() {
    let mut app = fem2d_app();
    for (field, value) in [("phase", "0.25"), ("speed", "2"), ("loopMode", "pingPong"), ("waveform", "sine"), ("reverse", "true")] {
        dispatch(&mut app, animation(SetResultAnimation { field: Some(field.into()), value: Some(value.into()), ..blank() })).await;
    }
    let state = published(&mut app).await;
    assert_eq!(state.phase, 0.25);
    assert_eq!(state.speed, 2.0);
    assert_eq!(state.loop_mode, results::config::Fem2dLoopMode::PingPong);
    assert_eq!(state.waveform, results::config::Fem2dWaveform::Sine);
    assert!(state.reverse);
    dispatch(&mut app, animation(SetResultAnimation { field: Some("phaseStep".into()), value: Some("0.5".into()), ..blank() })).await;
    assert!((published(&mut app).await.phase - 0.75).abs() < 1e-9);
    dispatch(&mut app, animation(SetResultAnimation { field: Some("playing".into()), value: Some("true".into()), ..blank() })).await;
    assert!(published(&mut app).await.playing);
}

/// 🚫️ LAW: an unknown field or an unparseable value is refused outright rather than silently
/// writing a default over the window's state.
#[semio_framework_async_macros::async_test]
async fn set_result_animation_refuses_unknown_fields_and_values() {
    let mut animation = results::config::Fem2dResultsAnimation::default();
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
    assert_eq!(delay_ms, results::config::ANIMATION_TICK_MS);
    assert_eq!(args.expect("re-arm args").get("windowId").and_then(dsl::DslValue::as_str), Some("results-left"));
}
