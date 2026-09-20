use super::*;
use crate::editor::flow::commands::evaluate::evaluate_result;
use crate::editor::flow::modes::edit::windows::main::FLOW_PLAY_WINDOW_MAIN;

/// 🔢️ How many hops the starter graph's chain is allowed before a law calls it a spin. The live
/// defect this bounds ran 2015 hops in ~3 s and only stopped when it had exhausted the host's
/// transient read lease registry (ticket 26/09/18 §5.3), so any finite bound separates the two.
const FLOW_EVAL_TICK_TURN_BOUND: usize = 64;

/// ⚖️ LAW: a `flowEvalTick` chain armed over one window SETTLES — every hop arms at most one
/// successor, and the chain reaches a hop that arms none inside
/// [`FLOW_EVAL_TICK_TURN_BOUND`] turns.
#[test]
fn a_tick_chain_settles_within_a_bounded_number_of_turns() {
    let snapshot = FlowSnapshot::default();
    let config = FlowMainWindowConfig::default();
    let mut session = FlowEvalSession::new();
    let mut armed = evaluate_result(&snapshot, &config, &mut session, FLOW_PLAY_WINDOW_MAIN, FLOW_PLAY_WINDOW_MAIN).effects.len();
    let mut turns = 0usize;
    while armed > 0 && turns < FLOW_EVAL_TICK_TURN_BOUND {
        assert_eq!(armed, 1, "a hop arms at most one successor");
        turns += 1;
        armed = tick_result(&snapshot, &config, &mut session, FLOW_PLAY_WINDOW_MAIN, FLOW_PLAY_WINDOW_MAIN).effects.len();
    }
    assert_eq!(armed, 0, "the chain still owed a hop after {FLOW_EVAL_TICK_TURN_BOUND} turns");
    session.retire_cold();
}

/// ⚖️ LAW: a SETTLED window owes nothing on an unchanged snapshot. This is the host's refresh poll
/// (`FlowPlayApp::pending_effects`) asked repeatedly with no edit in between: it must mint no hop at
/// all. The pre-fix probe rebuilt a throwaway `FlowEvalSession` per poll, so its latch was empty by
/// construction and every poll minted one.
#[test]
fn a_settled_window_owes_no_hop_on_an_unchanged_snapshot() {
    let snapshot = FlowSnapshot::default();
    let config = FlowMainWindowConfig::default();
    let mut session = FlowEvalSession::new();
    let mut armed = evaluate_result(&snapshot, &config, &mut session, FLOW_PLAY_WINDOW_MAIN, FLOW_PLAY_WINDOW_MAIN).effects.len();
    let mut turns = 0usize;
    while armed > 0 && turns < FLOW_EVAL_TICK_TURN_BOUND {
        turns += 1;
        armed = tick_result(&snapshot, &config, &mut session, FLOW_PLAY_WINDOW_MAIN, FLOW_PLAY_WINDOW_MAIN).effects.len();
    }
    for poll in 0..16 {
        assert!(evaluate_result(&snapshot, &config, &mut session, FLOW_PLAY_WINDOW_MAIN, FLOW_PLAY_WINDOW_MAIN).effects.is_empty(), "refresh poll {poll} minted a hop for a settled window");
    }
    session.retire_cold();
}

/// ⚖️ LAW: a hop is ADDRESSED — it discharges the latch of the window its payload names and no
/// other. A field-less payload could not, which is why every hop landed on whichever window the
/// shell redispatched it under and no window's latch was ever discharged.
#[test]
fn a_hop_discharges_only_the_window_its_payload_names() {
    let mut session = FlowEvalSession::new();
    assert!(session.arm_owed_window_tick(FLOW_PLAY_WINDOW_MAIN), "a window that has never ticked owes its first hop");
    assert!(!session.arm_owed_window_tick(FLOW_PLAY_WINDOW_MAIN), "a window whose hop is already armed owes no second");
    assert!(session.arm_owed_window_tick("flow-main-2"), "a second window owes its own first hop");
    session.begin_window_tick(FLOW_PLAY_WINDOW_MAIN);
    assert!(!session.window_tick_is_armed(FLOW_PLAY_WINDOW_MAIN), "the named window's arm is discharged by its own hop");
    assert!(session.window_tick_is_armed("flow-main-2"), "a sibling window's arm is untouched");
    session.retire_cold();
}

/// ⚖️ LAW: a hop that parks extension answers arms NOTHING — the answers own the continuation, and
/// only the last of a fan-out to settle re-arms. Re-arming beside the parked answers is what turned
/// one graph into 298 invocations against 111 settles in 80 s of live console.
#[test]
fn parked_extension_answers_own_the_continuation() {
    let mut session = FlowEvalSession::new();
    session.note_window_tick_outcome(FLOW_PLAY_WINDOW_MAIN, true);
    session.note_window_extensions_in_flight(FLOW_PLAY_WINDOW_MAIN, 2);
    assert!(!session.arm_owed_window_tick(FLOW_PLAY_WINDOW_MAIN), "a window waiting on answers is not armed");
    assert!(!session.settle_window_extension(FLOW_PLAY_WINDOW_MAIN), "the first of two answers arms nothing");
    assert!(!session.settle_window_extension(FLOW_PLAY_WINDOW_MAIN) && session.arm_owed_window_tick(FLOW_PLAY_WINDOW_MAIN), "the last answer arms exactly one successor");
    assert_eq!(session.window_extensions_in_flight(FLOW_PLAY_WINDOW_MAIN), 0);
    session.retire_cold();
}
