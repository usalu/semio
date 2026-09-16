use super::*;
use crate::editor::fem3d::modes::edit::windows::results::config::{Fem3dLoopMode, Fem3dWaveform, ANIMATION_TICK_SECONDS};
use protocol::{Mutation, MutationDiff, OpBinary, OpText};
use std::collections::BTreeMap;
use store::{ArtifactDsl, ArtifactPack};

fn settings(loop_mode: Fem3dLoopMode) -> Fem3dResultsAnimation {
    Fem3dResultsAnimation { phase: 0.0, playing: true, speed: 0.5, loop_mode, waveform: Fem3dWaveform::Ramp, reverse: false }
}

//#region ⏱️ClockLaws
/// 🧾️ LAW: `Loop` wraps the phase modulo one and never ends the run.
#[test]
fn loop_clock_wraps_and_keeps_running() {
    let settings = settings(Fem3dLoopMode::Loop);
    let mut clock = Fem3dPlaybackClock::from_settings(&settings);
    for frame in 1..=100 {
        let (next, running) = clock.advanced(&settings, ANIMATION_TICK_SECONDS);
        assert!(running, "frame {frame}");
        assert!((0.0..1.0).contains(&next.phase), "frame {frame}: {}", next.phase);
        clock = next;
    }
    let expected = (100.0 * ANIMATION_TICK_SECONDS * 0.5).rem_euclid(1.0);
    assert!((clock.phase - expected).abs() < 1e-9, "{} vs {expected}", clock.phase);
}

/// 🧾️ LAW: `Once` parks exactly at 1 and ends the run on the frame that reaches it.
#[test]
fn once_clock_parks_at_one_and_ends_the_run() {
    let settings = Fem3dResultsAnimation { speed: 4.0, ..settings(Fem3dLoopMode::Once) };
    let mut clock = Fem3dPlaybackClock::from_settings(&settings);
    let mut frames = 0;
    loop {
        let (next, running) = clock.advanced(&settings, ANIMATION_TICK_SECONDS);
        clock = next;
        frames += 1;
        if !running {
            break;
        }
        assert!(frames < 100, "a 4 Hz ramp reaches its end within a second");
    }
    assert_eq!(clock.phase, 1.0);
    assert_eq!(frames, (1.0 / (4.0 * ANIMATION_TICK_SECONDS)).ceil() as usize);
}

/// 🧾️ LAW: `PingPong` reflects off both ends, flipping the direction and never leaving `0..=1`.
#[test]
fn ping_pong_clock_reflects_off_both_ends() {
    let settings = Fem3dResultsAnimation { speed: 4.0, ..settings(Fem3dLoopMode::PingPong) };
    let mut clock = Fem3dPlaybackClock::from_settings(&settings);
    let mut flips = 0;
    for _ in 0..60 {
        let (next, running) = clock.advanced(&settings, ANIMATION_TICK_SECONDS);
        assert!(running);
        assert!((0.0..=1.0).contains(&next.phase));
        flips += usize::from(next.reverse != clock.reverse);
        clock = next;
    }
    assert!(flips >= 3, "two seconds at 4 Hz bounce at least three times: {flips}");
}

/// 🧾️ LAW: parking folds the clock's phase and direction into the transport without touching the
/// settings, and a fresh clock starts from exactly the parked state.
#[test]
fn parking_round_trips_through_the_transport_settings() {
    let settings = Fem3dResultsAnimation { speed: 1.25, waveform: Fem3dWaveform::Sine, ..settings(Fem3dLoopMode::PingPong) };
    let clock = Fem3dPlaybackClock { phase: 0.375, reverse: true };
    let parked = clock.parked_into(&settings);
    assert_eq!(parked, Fem3dResultsAnimation { phase: 0.375, reverse: true, ..settings });
    assert_eq!(Fem3dPlaybackClock::from_settings(&parked), clock);
}
//#endregion ⏱️ClockLaws

//#region 🫧️PartitionLaws
#[test]
fn clock_publication_and_retirement_obey_tiny_grants() {
    use semio_framework_plugin::WindowTransientOwner;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔀️codec-contracts/🔣️.json")).unwrap();
    let owners = Fem3dResultsWindowTransientOwner::build_owners();
    for row in fixture["cases"].as_array().unwrap() {
        let before: Fem3dResultsWindowTransient = serde_json::from_value(row["before"].clone()).unwrap();
        let after: Fem3dResultsWindowTransient = serde_json::from_value(row["after"].clone()).unwrap();
        let mut store = store::TransientStore::<_, Fem3dResultsWindowTransientMutation>::new(before);
        let mut publication = store.begin_publish_one_leased(semio_framework_job::OperationId(1), 0, SetPlaybackClock { clock: after.clock }.into(), owners.preparation.as_ref(), owners.state_retirement.clone()).unwrap();
        let zero = store::ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 4096 };
        assert!(matches!(store.advance_publish_one(&mut publication, zero).unwrap(), store::ArtifactStoreOneItemAdvance::Blocked));
        let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 };
        for _ in 0..8 {
            if matches!(store.advance_publish_one(&mut publication, grant).unwrap(), store::ArtifactStoreOneItemAdvance::Published(_)) {
                break;
            }
        }
        assert_eq!(serde_json::to_value(store.current_root().as_ref()).unwrap(), row["after"]);
        assert!(publication.acknowledge());
        for _ in 0..4096 {
            match publication.close_step(grant).unwrap() {
                store::SnapshotRetirementStep::Complete => break,
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => assert!(released_items <= 1 && released_bytes <= 1),
                store::SnapshotRetirementStep::Blocked => panic!("an unaliased clock publication must close"),
            }
        }
        assert!(publication.terminal_is_empty());
    }
}

#[test]
fn clock_partition_matches_language_neutral_json_oracle() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪟️clock-partition/🔣️.json")).expect("fixture");
    let mut typed = BTreeMap::<String, Fem3dResultsWindowTransient>::new();
    let mut oracle = serde_json::Map::new();
    for step in fixture["steps"].as_array().expect("steps") {
        let window_id = step["windowId"].as_str().expect("window id").to_string();
        let clock = serde_json::from_value::<Option<Fem3dPlaybackClock>>(step["clock"].clone()).expect("clock");
        let mutation = Fem3dResultsWindowTransientMutation::from(SetPlaybackClock { clock });
        let state = typed.entry(window_id.clone()).or_default();
        Mutation::diff(&mutation, state).apply_to(state);
        oracle.insert(window_id, step["clock"].clone());
        store::os_store::test_support::assert_op_text_binary_equivalence(&mutation);
    }
    let typed = typed.into_iter().map(|(window_id, state)| (window_id, serde_json::to_value(state.clock).expect("typed clock"))).collect::<serde_json::Map<_, _>>();
    assert_eq!(serde_json::Value::Object(typed), fixture["expected"]);
    assert_eq!(serde_json::Value::Object(oracle), fixture["expected"]);
}

#[test]
fn clock_codecs_and_inverse_match_neutral_vectors() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔀️codec-contracts/🔣️.json")).unwrap();
    for row in oracle["cases"].as_array().unwrap() {
        let before: Fem3dResultsWindowTransient = dsl::json::from_json_str(&row["before"].to_string()).unwrap();
        let expected: Fem3dResultsWindowTransient = dsl::json::from_json_str(&row["after"].to_string()).unwrap();
        let operation: Fem3dResultsWindowTransientMutation = SetPlaybackClock { clock: expected.clock }.into();
        let outcome = operation.diff(&before);
        assert!(outcome.messages().is_empty());
        let applied = outcome.diff().apply(&before).unwrap();
        assert_eq!(applied, expected);
        let encoded: serde_json::Value = serde_json::from_str(&dsl::json::to_json_string(&applied)).unwrap();
        assert_eq!(encoded, row["after"]);
        assert_eq!(Fem3dResultsWindowTransient::parse_dsl(&applied.print_dsl()).unwrap(), applied);
        let packed = applied.encode_pack_with(&Default::default()).unwrap();
        assert_eq!(Fem3dResultsWindowTransient::decode_pack_with(&packed, &Default::default()).unwrap(), applied);
        let operation_wire = dsl::json::to_json_string(&operation);
        let independent: serde_json::Value = serde_json::from_str(&operation_wire).unwrap();
        assert_eq!(dsl::json::from_json_str::<Fem3dResultsWindowTransientMutation>(&serde_json::to_string(&independent).unwrap()).unwrap(), operation);
        assert_eq!(Fem3dResultsWindowTransientMutation::parse_op(&operation.print_op()).unwrap(), operation);
        assert_eq!(Fem3dResultsWindowTransientMutation::decode_op(&operation.encode_op().unwrap()).unwrap(), operation);
        let mut restored = applied;
        for inverse in operation.inverse(&before) {
            restored = inverse.diff(&restored).diff().apply(&restored).unwrap();
        }
        assert_eq!(restored, before);
    }
}
//#endregion 🫧️PartitionLaws
