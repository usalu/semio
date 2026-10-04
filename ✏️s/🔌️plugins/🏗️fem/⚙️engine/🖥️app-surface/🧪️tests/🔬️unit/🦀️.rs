use super::*;

#[test]
fn next_id_retries_past_collisions() {
    let existing = vec!["n0".to_string(), "n2".to_string()];
    assert_eq!(next_id(existing.into_iter(), "n"), "n3");
}

#[test]
fn hex_to_rgb01_parses_pure_colors() {
    assert_eq!(hex_to_rgb01("#ffffff"), (1.0, 1.0, 1.0));
    assert_eq!(hex_to_rgb01("#000000"), (0.0, 0.0, 0.0));
    assert_eq!(hex_to_rgb01("#ff0000"), (1.0, 0.0, 0.0));
}

#[test]
fn von_mises_color_maps_extremes_midpoint_and_clamps() {
    assert_eq!(von_mises_color(0.0, 0.0, 100.0), VON_MISES_BANDS[0]);
    assert_eq!(von_mises_color(100.0, 0.0, 100.0), VON_MISES_BANDS[VON_MISES_BANDS.len() - 1]);
    assert_eq!(von_mises_color(50.0, 0.0, 100.0), VON_MISES_BANDS[VON_MISES_BANDS.len() / 2]);
    assert_eq!(von_mises_color(-10.0, 0.0, 100.0), VON_MISES_BANDS[0]);
    assert_eq!(von_mises_color(200.0, 0.0, 100.0), VON_MISES_BANDS[VON_MISES_BANDS.len() - 1]);
}

//#region 🔖️Playback
/// ⏯️ LAW: a freshly opened results window is parked on the FULL deformed shape and is not playing —
/// the pre-playback behaviour, unchanged for anyone who never touches the transport.
#[test]
fn results_animation_default_is_a_still_full_deformation() {
    let animation = FemResultsAnimation::default();
    assert!(!animation.playing);
    assert_eq!(animation.phase, 1.0);
    assert_eq!(animation.amplitude(), 1.0);
    assert_eq!(animation.speed, 0.5);
    assert_eq!(animation.loop_mode, FemLoopMode::Loop);
    assert_eq!(animation.waveform, FemWaveform::Ramp);
    assert!(!animation.reverse);
}

/// 〰️ LAW: `Sine` swings through both signs; `Ramp` never leaves `0..=1`.
#[test]
fn results_animation_waveforms_map_phase_to_amplitude() {
    let ramp = FemResultsAnimation { phase: 0.25, waveform: FemWaveform::Ramp, ..FemResultsAnimation::default() };
    assert_eq!(ramp.amplitude(), 0.25);
    let sine = FemResultsAnimation { waveform: FemWaveform::Sine, ..FemResultsAnimation::default() };
    assert!((FemResultsAnimation { phase: 0.25, ..sine }.amplitude() - 1.0).abs() < 1e-9);
    assert!(FemResultsAnimation { phase: 0.75, ..sine }.amplitude() < -0.999);
    assert!(FemResultsAnimation { phase: 0.0, ..sine }.amplitude().abs() < 1e-9);
}

/// ▶️ LAW: arming a finished `Once` run rewinds it, so the play button is never a dead control.
#[test]
fn results_animation_start_rewinds_a_finished_once_run() {
    let mut animation = FemResultsAnimation { loop_mode: FemLoopMode::Once, phase: 1.0, ..FemResultsAnimation::default() };
    animation.start();
    assert!(animation.playing);
    assert_eq!(animation.phase, 0.0);
    let mut looping = FemResultsAnimation { phase: 1.0, ..FemResultsAnimation::default() };
    looping.start();
    assert_eq!(looping.phase, 1.0, "a looping run keeps its phase when armed");
}
//#endregion 🔖️Playback
