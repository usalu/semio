use super::*;

/// ⚖️ LAW: an armed re-dispatch always carries BOTH halves of the window address, because the
/// shell redispatches an `Effect::DispatchAction` under whichever window is current and the
/// retained route matches the payload against the ViewModel roster, never against that window.
#[test]
fn a_rearm_carries_the_window_id_and_kind_it_was_addressed_to() {
    let Effect::DispatchAction { action, args, .. } = rearm("preview-1", "procedural-view-preview", 105) else {
        panic!("a re-arm must be a dispatch effect");
    };
    assert_eq!(action, "flowEvalTick");
    let args = args.expect("an addressed re-arm carries args");
    assert_eq!(args.get("windowId").and_then(dsl::DslValue::as_str), Some("preview-1"));
    assert_eq!(args.get("windowKindId").and_then(dsl::DslValue::as_str), Some("procedural-view-preview"));
}

/// ⚖️ LAW: only a roster entry whose kind the surface declared as a preview may be armed, and a
/// surface that declares no preview kind at all arms nothing — the served-app stall was exactly an
/// address the route could only ever refuse.
#[test]
fn only_declared_preview_kinds_are_armed_from_a_roster() {
    use semio_framework_plugin::ViewWindowInstance;
    let view = ViewModel {
        window_instances: vec![
            ViewWindowInstance { id: "main".into(), window_kind_id: "procedural-main".into() },
            ViewWindowInstance { id: "view-preview".into(), window_kind_id: "procedural-view-preview".into() },
        ],
        ..Default::default()
    };
    let viewer_kinds: &[&'static str] = &["procedural-view-preview"];
    assert_eq!(attached_preview_windows(Some(&view), viewer_kinds), vec![("view-preview", "procedural-view-preview")]);
    assert!(attached_preview_windows(Some(&view), &[]).is_empty(), "a surface with no preview kind arms nothing");
    assert!(attached_preview_windows(None, viewer_kinds).is_empty(), "no roster at all arms nothing");
    assert_eq!(preview_kind("procedural-main", viewer_kinds), None, "a flow window holds no evaluation publication");
}

/// 📜️ The language-agnostic settle-path table both implementations answer.
const PREVIEW_CANCEL_FIXTURE_JSON: &str = include_str!("../../../🧫️fixtures/🛑️preview-cancel.json");

fn settle_path() -> dsl::json::Value {
    dsl::json::parse(PREVIEW_CANCEL_FIXTURE_JSON).expect("the cancel fixture parses").get("settlePath").cloned().expect("the fixture declares its settle path")
}

/// ⚖️ LAW: a tick that parked extension answers is NOT finished, whatever the evaluation itself
/// said — every `unfinishedRows` row of `🧫️fixtures/🛑️preview-cancel.json`. `!more` is precisely the
/// state that parks `tessellate`, so recording `more` alone told the session a window still owing
/// mesh round trips was done.
#[test]
fn a_tick_that_parked_extension_work_is_never_recorded_finished() {
    let rows = settle_path().get("unfinishedRows").cloned().expect("unfinishedRows");
    let rows = rows.as_array().expect("unfinishedRows is a table");
    assert_eq!(rows.len(), 4, "every declared row must be replayed");
    for row in rows {
        let id = row.get("id").and_then(dsl::json::Value::as_str).expect("row id");
        let more = row.get("more").and_then(dsl::json::Value::as_bool).expect("more");
        let parked = usize::try_from(row.get("parkedInvocations").and_then(dsl::json::Value::as_u64).expect("parkedInvocations")).expect("a row count fits");
        let expected = row.get("unfinished").and_then(dsl::json::Value::as_bool).expect("unfinished");
        println!("[STATS] settle-path row {id} more={more} parked={parked} unfinished={expected}");
        assert_eq!(tick_is_unfinished(more, parked), expected, "{id}");
    }
}

/// ⚖️ LAW: the LAST extension answer of a run arms the TERMINAL tick itself — the settle path is the
/// chain's own and identical on all three preview windows, never a debt handed to a host refresh
/// poll that a shell may narrow, coalesce, block or lose. On wgpu that poll's effects are exactly
/// what a blocked `apply_pending_step` never applies, and the viewer's two boolean examples hung at
/// `inFlight: 1` / `ratio 0.5853658536585366` forever
/// (`📓️wgpu-example-chain-2026-09-13.md` §6.1).
#[test]
fn the_last_tessellate_answer_arms_the_terminal_tick_itself() {
    let sequence = settle_path().get("latchSequence").cloned().expect("latchSequence");
    let sequence = sequence.as_array().expect("latchSequence is a table").clone();
    let flag = |step: &dsl::json::Value, key: &str| step.get(key).and_then(dsl::json::Value::as_bool).unwrap_or(false);
    let mut session = FlowEvalSession::new();
    let (window, kind) = ("preview-1", "procedural-view-preview");
    let mut emitted = 0usize;
    for step in &sequence {
        let name = step.get("step").and_then(dsl::json::Value::as_str).expect("step name");
        match name {
            "beginWindowTick" => session.begin_window_tick(window),
            "noteTickOutcome" => session.note_window_tick_outcome(window, tick_is_unfinished(false, 1)),
            "noteExtensionsInFlight" => session.note_window_extensions_in_flight(window, 1),
            "settleLastAnswer" => {
                let answered = FlowTessellateResolve { window_id: window.into(), window_kind_id: kind.into(), node_hash: 1, output_json: r#"{"done":true,"phase":"complete","unitsDone":3,"unitsTotal":3}"#.into() };
                emitted = resolve_tessellate(&answered, &mut session).len();
                assert_eq!(emitted == 1, flag(step, "emitsRearm"), "settleLastAnswer must arm the terminal tick from the chain itself");
            }
            other => panic!("the fixture named an unknown settle step {other}"),
        }
        let in_flight = u32::try_from(step.get("inFlight").and_then(dsl::json::Value::as_u64).expect("inFlight")).expect("a latch count fits");
        println!("[STATS] settle-path step {name} armed={} inFlight={} owed={}", session.window_tick_is_armed(window), session.window_extensions_in_flight(window), session.window_tick_owed(window));
        assert_eq!(session.window_tick_is_armed(window), flag(step, "armed"), "{name}: armed");
        assert_eq!(session.window_extensions_in_flight(window), in_flight, "{name}: inFlight");
        assert_eq!(session.window_tick_owed(window), flag(step, "tickOwed"), "{name}: tickOwed");
    }
    assert_eq!(emitted, 1, "the run's last answer owes exactly one terminal tick");
    crate::flow_operators::retire_flow_eval_session(session);
}

/// ⚖️ LAW: the terminal arm is LATCHED, not unconditional — an answer that lands on a window whose
/// own tick reported itself finished arms nothing, so a stale or foreign answer can never spin the
/// chain back up.
#[test]
fn an_answer_on_a_finished_window_arms_nothing() {
    let mut session = FlowEvalSession::new();
    let (window, kind) = ("preview-2", "procedural-view-preview");
    session.begin_window_tick(window);
    session.note_window_tick_outcome(window, tick_is_unfinished(false, 0));
    session.note_window_extensions_in_flight(window, 1);
    let answered = FlowTessellateResolve { window_id: window.into(), window_kind_id: kind.into(), node_hash: 2, output_json: r#"{"done":true,"phase":"complete","unitsDone":3,"unitsTotal":3}"#.into() };
    assert!(resolve_tessellate(&answered, &mut session).is_empty(), "a finished window owes no terminal tick");
    assert!(!session.window_tick_is_armed(window));
    crate::flow_operators::retire_flow_eval_session(session);
}

/// ⚖️ LAW: the deflection ladder is ONE table — an editor mesh and a viewer mesh of the same handle
/// at the same LOD are tessellated to the same tolerance, so the two surfaces cannot drift.
#[test]
fn the_lod_deflection_ladder_is_shared_and_monotone() {
    assert!(preview_tolerance("coarse") > preview_tolerance("medium"));
    assert!(preview_tolerance("medium") > preview_tolerance("fine"));
    assert_eq!(preview_tolerance(""), preview_tolerance("medium"), "an unset LOD is medium");
}
