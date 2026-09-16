use super::*;
use protocol::{Mutation, MutationDiff, OpBinary, OpText};

#[test]
fn fem2d_window_config_results_matches_neutral_fixture_and_codecs() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧬️schema/🧫️fixtures/🪪️document-contract/🔣️.json")).expect("FEM window fixture");
    for candidate in fixture["valid"].as_array().expect("neutral valid cases") {
        let base: Fem2dResultsWindowConfig = dsl::json::from_json_str(&candidate.to_string()).expect("neutral FEM window config");
        let mutation = Fem2dResultsWindowConfigMutation::Snapshot { config: Box::new(base.clone()) };
        let after = mutation.diff(&base).diff().apply(&base).expect("FEM window diff");
        assert_eq!(after, base);
        assert_eq!(Fem2dResultsWindowConfigMutation::parse_op(&mutation.print_op()).expect("FEM text mutation"), mutation);
        assert_eq!(Fem2dResultsWindowConfigMutation::decode_op(&mutation.encode_op().expect("FEM binary mutation")).expect("FEM decoded mutation"), mutation);
        let text = store::ArtifactDsl::print_dsl(&base);
        assert_eq!(<Fem2dResultsWindowConfig as store::ArtifactDsl>::parse_dsl(&text).expect("FEM window text"), base);
        let bytes = store::ArtifactPack::encode_pack(&base);
        assert_eq!(<Fem2dResultsWindowConfig as store::ArtifactPack>::decode_pack(&bytes).expect("FEM window pack"), base);
    }
    let mut admitted_invalid = Vec::new();
    for row in fixture["invalid"].as_array().expect("neutral invalid cases") {
        let kind = row["kind"].as_str().expect("neutral invalid case kind");
        let mut candidate = fixture["valid"][0].clone();
        match kind {
            "unknown" => candidate["locale"] = serde_json::json!("de"),
            "camera-zero" => candidate["camera"]["zoom"] = serde_json::json!(0),
            "camera-negative" => candidate["camera"]["zoom"] = serde_json::json!(-1),
            "camera-null" => candidate["camera"] = serde_json::Value::Null,
            "camera-unknown" => candidate["camera"]["extra"] = serde_json::json!(true),
            "bad-mode" => candidate["resultMode"] = serde_json::json!("harmonic"),
            "animation-null" => candidate["animation"] = serde_json::Value::Null,
            "animation-unknown" => candidate["animation"]["extra"] = serde_json::json!(true),
            "bad-loop-mode" => candidate["animation"]["loopMode"] = serde_json::json!("bounce"),
            "bad-waveform" => candidate["animation"]["waveform"] = serde_json::json!("square"),
            _ => panic!("unknown neutral invalid case {kind}"),
        }
        if dsl::json::from_json_str::<Fem2dResultsWindowConfig>(&candidate.to_string()).is_ok() {
            admitted_invalid.push(kind);
        }
    }
    assert!(admitted_invalid.is_empty(), "FEM native admitted invalid neutral cases: {admitted_invalid:?}");
    eprintln!("[DEBUG] FEM 2D results window config matched neutral fixture and codecs");
}

//#region 🔖️Playback
/// ⏯️ LAW: a freshly opened results window is parked on the FULL deformed shape and is not playing —
/// the pre-playback behaviour, unchanged for anyone who never touches the transport.
#[test]
fn fem2d_results_animation_default_is_a_still_full_deformation() {
    let animation = Fem2dResultsAnimation::default();
    assert!(!animation.playing);
    assert_eq!(animation.phase, 1.0);
    assert_eq!(animation.amplitude(), 1.0);
    assert_eq!(animation.speed, 0.5);
    assert_eq!(animation.loop_mode, Fem2dLoopMode::Loop);
    assert_eq!(animation.waveform, Fem2dWaveform::Ramp);
    assert!(!animation.reverse);
}

/// 〰️ LAW: `Sine` swings through both signs; `Ramp` never leaves `0..=1`.
#[test]
fn fem2d_results_animation_waveforms_map_phase_to_amplitude() {
    let ramp = Fem2dResultsAnimation { phase: 0.25, waveform: Fem2dWaveform::Ramp, ..Fem2dResultsAnimation::default() };
    assert_eq!(ramp.amplitude(), 0.25);
    let sine = Fem2dResultsAnimation { waveform: Fem2dWaveform::Sine, ..Fem2dResultsAnimation::default() };
    assert!((Fem2dResultsAnimation { phase: 0.25, ..sine }.amplitude() - 1.0).abs() < 1e-9);
    assert!(Fem2dResultsAnimation { phase: 0.75, ..sine }.amplitude() < -0.999);
    assert!(Fem2dResultsAnimation { phase: 0.0, ..sine }.amplitude().abs() < 1e-9);
}

/// 🔁️ LAW: `Loop` wraps, `Once` parks at 1 and stops itself, `PingPong` bounces by flipping
/// `reverse` while `speed` stays positive.
#[test]
fn fem2d_results_animation_advances_by_its_loop_mode() {
    let base = Fem2dResultsAnimation { phase: 0.99, playing: true, speed: 1.0, ..Fem2dResultsAnimation::default() };
    let looped = Fem2dResultsAnimation { loop_mode: Fem2dLoopMode::Loop, ..base }.advanced(0.033);
    assert!(looped.playing && looped.phase < 0.1, "{looped:?}");
    let once = Fem2dResultsAnimation { loop_mode: Fem2dLoopMode::Once, ..base }.advanced(0.033);
    assert_eq!(once.phase, 1.0);
    assert!(!once.playing);
    let bounced = Fem2dResultsAnimation { loop_mode: Fem2dLoopMode::PingPong, ..base }.advanced(0.033);
    assert!(bounced.reverse && bounced.playing && bounced.phase < 1.0, "{bounced:?}");
    assert!(bounced.advanced(0.033).phase < bounced.phase, "a reversed ping-pong walks back down");
    let floored = Fem2dResultsAnimation { phase: 0.01, reverse: true, loop_mode: Fem2dLoopMode::PingPong, ..base }.advanced(0.033);
    assert!(!floored.reverse && floored.phase >= 0.0, "{floored:?}");
}

/// ▶️ LAW: arming a finished `Once` run rewinds it, so the play button is never a dead control.
#[test]
fn fem2d_results_animation_start_rewinds_a_finished_once_run() {
    let mut animation = Fem2dResultsAnimation { loop_mode: Fem2dLoopMode::Once, phase: 1.0, ..Fem2dResultsAnimation::default() };
    animation.start();
    assert!(animation.playing);
    assert_eq!(animation.phase, 0.0);
    let mut looping = Fem2dResultsAnimation { phase: 1.0, ..Fem2dResultsAnimation::default() };
    looping.start();
    assert_eq!(looping.phase, 1.0, "a looping run keeps its phase when armed");
}
//#endregion 🔖️Playback
