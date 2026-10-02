use super::*;
use semio_framework_os_flow::neural::ColdRetire;
use semio_framework_tool_run::ToolRunId;
const PREVIEW_EVAL_RUN_FIXTURE_JSON: &str = include_str!("../../../🧫️fixtures/⏯️preview-eval-run.json");
fn fixture() -> serde_json::Value {
    let fixture: serde_json::Value = serde_json::from_str(PREVIEW_EVAL_RUN_FIXTURE_JSON).expect("the preview evaluation run fixture parses");
    assert_eq!(fixture["format"], "semio.generation3d.preview-eval-run");
    assert_eq!(fixture["version"], 1);
    fixture
}

fn text(value: &serde_json::Value) -> &str {
    value.as_str().unwrap_or_else(|| panic!("fixture text expected, found {value}"))
}

fn run_view(row: &serde_json::Value) -> Option<ToolRunView> {
    let run = row.as_object()?;
    Some(ToolRunView::new(
        run.get("toolId").and_then(serde_json::Value::as_str).unwrap_or(PREVIEW_EVAL_TOOL_ID),
        ToolRunIdentity { id: ToolRunId { app_instance_id: 1, run: run.get("run").and_then(serde_json::Value::as_u64).unwrap_or(1) }, generation: run["generation"].as_u64().unwrap() as u32, base_revision: [0; 32] },
        ToolRunState::parse(text(&run["state"])).expect("a fixture run state"),
    ))
}

/// ⚖️ Every fixture run-effects row names its exact start, finalize, or unchanged outcome.
#[test]
fn every_run_effects_row_starts_finalizes_or_leaves_the_run() {
    for row in fixture()["runEffects"].as_array().unwrap() {
        let id = text(&row["id"]);
        let mut session = FlowEvalSession::new();
        let mut link = PreviewEvalRunLink::default();
        let windows: Vec<(&str, &'static str)> = row["windows"].as_array().unwrap().iter().map(|window| (text(window), "procedural-view-preview")).collect();
        if !row["owed"].as_bool().unwrap() {
            for (window, _) in &windows {
                session.arm_window_tick(window);
                session.begin_window_tick(window);
                session.note_window_tick_outcome(window, false);
            }
        }
        let run = run_view(&row["run"]);
        if let (Some(run), Some(settled)) = (run.as_ref(), row["run"].get("settledGeneration").and_then(serde_json::Value::as_u64)) {
            link.settled = Some((run.identity.id.run, settled as u32));
        }
        link.restart_owed = row["restartOwed"].as_bool().unwrap_or(false);
        let effects = preview_eval_run_effects(&mut session, &mut link, &windows, run.as_ref(), row["servable"].as_bool().unwrap_or(true), 0);
        let actions: Vec<&str> = effects.iter().map(|effect| match effect {
            Effect::DispatchAction { action, .. } => action.as_str(),
            other => panic!("{id}: the run owes only dispatches, found {other:?}"),
        }).collect();
        println!("[STATS] runEffects {id}: {actions:?}");
        assert_eq!(actions, row["expected"].as_array().unwrap().iter().map(text).collect::<Vec<_>>(), "{id}");
        for effect in &effects {
            let Effect::DispatchAction { action, args: Some(args), .. } = effect else { continue };
            match action.as_str() {
                TOOL_RUN_START_ACTION_ID => assert_eq!(args.get(TOOL_RUN_ARG_TOOL_ID).and_then(dsl::DslValue::as_str), Some(PREVIEW_EVAL_TOOL_ID), "{id}: start names the tool"),
                _ => assert_eq!(args.get(TOOL_RUN_ARG_RUN_ID).and_then(dsl::DslValue::as_str), run.as_ref().map(|run| run.identity.id.run.to_string()).as_deref(), "{id}: finalize names the run"),
            }
        }
        session.retire_cold();
    }
}

/// 🎞️ One `pending_effects` ladder replayed as the host runs it: a run action is a REQUEST, so the run
/// view keeps answering what it answered before for `start_lag_polls` further polls. Answers how many
/// `toolRunStart` invocations the ladder cost.
fn replay_pending_effects(session: &mut FlowEvalSession, link: &mut PreviewEvalRunLink, windows: &[(&str, &'static str)], polls: usize, start_lag_polls: usize, servable: bool, poll_start_view: &str) -> usize {
    let mut view = (poll_start_view != "none").then(|| run_view(&serde_json::json!({ "run": 1, "generation": 0, "state": poll_start_view })).expect("a fixture run view")).map(Some).unwrap_or(None);
    let mut starts = 0_usize;
    let mut landing: Option<(usize, ToolRunState)> = None;
    for _ in 0..polls {
        for effect in preview_eval_run_effects(session, link, windows, view.as_ref(), servable, 0) {
            let Effect::DispatchAction { action, .. } = effect else { panic!("the run owes only dispatches") };
            let state = match action.as_str() {
                TOOL_RUN_START_ACTION_ID => {
                    starts += 1;
                    ToolRunState::Running
                }
                TOOL_RUN_FINALIZE_ACTION_ID => ToolRunState::Finalized,
                other => panic!("the run owes only start and finalize, found {other}"),
            };
            landing = Some((start_lag_polls, state));
        }
        landing = match landing {
            Some((0, state)) => {
                view = Some(ToolRunView { state, ..view.unwrap_or_else(|| run_view(&serde_json::json!({ "run": 1, "generation": 0, "state": "running" })).expect("a fixture run view")) });
                None
            }
            Some((remaining, state)) => Some((remaining - 1, state)),
            None => None,
        };
    }
    starts
}

/// ⚖️ LAW: every `gestureRearm` row — a gesture that MOVED THE DOCUMENT owes every attached preview
/// window exactly one re-armed tick and carries, on its OWN emit, the run start that debt needs when no
/// live run is there to be woken; a gesture that authored no artifact mutation owes a settled run
/// nothing; and one gesture costs at most ONE `toolRunStart` however slowly the host's run view catches
/// up with what it was handed.
///
/// 🪪️ Three live defects, measured on 6018. (1) The inspector's `patchFlowWidgets` moved `height`
/// 6 → 7 in the document and the edit preview's published payload stayed byte-identical, because only a
/// hand-kept roster of tool ids owed the previews anything. (2) Once it owed, the debt was still never
/// read: the guest logged `owedAfter=["procedural-preview", "generation3d-generate-preview"]` and the
/// console then went silent for 60 s, because the host polls `pending_effects` once per `refreshUi` and
/// refreshes on ACTIVITY, and a settled, finalized run leaves none. (3) One quiet boot logged three
/// `toolRunStart` invocations inside 500 ms — and one busy session 1 053 — because `pending_effects`
/// re-read `owed` on every refresh and could not tell a start it had already asked for from one it
/// still owed (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`).
#[test]
fn every_gesture_rearm_row_owes_one_tick_per_preview_and_at_most_one_start() {
    for row in fixture()["gestureRearm"]["rows"].as_array().unwrap() {
        let id = text(&row["id"]);
        let windows: Vec<(&str, &'static str)> = row["windows"].as_array().unwrap().iter().map(|window| (text(window), "procedural-view-preview")).collect();
        let settled_windows = row["windowsSettled"].as_bool().unwrap();
        let live = row["runIsLive"].as_bool().unwrap();
        let has_settled = row["runHasSettled"].as_bool().unwrap();
        let servable = row["servable"].as_bool().unwrap();
        let polls = row["polls"].as_u64().unwrap() as usize;
        let lag = row["startLagPolls"].as_u64().unwrap() as usize;
        let poll_start_view = text(&row["pollStartView"]);
        let expected = &row["expected"];
        let mutations = row["artifactMutations"].as_u64().unwrap() as usize;

        // 🏁️ A window is SETTLED when it has ticked and reported nothing left to compute — the state a
        // finished evaluation leaves behind, and the state the inspector's edit found.
        let stage = |session: &mut FlowEvalSession, link: &mut PreviewEvalRunLink| {
            if settled_windows {
                for (window_id, _) in &windows {
                    session.arm_window_tick(window_id);
                    session.begin_window_tick(window_id);
                    session.note_window_tick_outcome(window_id, false);
                }
            }
            link.settled = has_settled.then_some((1, 0));
            if live {
                link.port = Some(semio_framework_plugin::ToolRunJobPort::default());
            }
        };

        let mut session = FlowEvalSession::new();
        let mut link = PreviewEvalRunLink::default();
        stage(&mut session, &mut link);
        if settled_windows {
            // 🏁️ A complete run is finalized ONCE, however often the host polls it while the view still
            // answers `complete` — the request latch, not the poll cadence, decides how often it asks.
            let complete = run_view(&serde_json::json!({ "run": 1, "generation": 0, "state": "complete" }));
            let mut asked: Vec<&str> = Vec::new();
            for _ in 0..polls {
                for effect in preview_eval_run_effects(&mut session, &mut link, &windows, complete.as_ref(), true, 0) {
                    let Effect::DispatchAction { action, .. } = effect else { panic!("{id}: the run owes only dispatches") };
                    asked.push(if action == TOOL_RUN_FINALIZE_ACTION_ID { "toolRunFinalize" } else { "toolRunStart" });
                }
            }
            assert_eq!(asked, if windows.is_empty() { Vec::new() } else { vec!["toolRunFinalize"] }, "{id}: a settled complete run is finalized exactly once, however often the host polls it");
            link.settled = has_settled.then_some((1, 0));
            link.restart_owed = false;
            link.requested = None;
        }

        let mut emit = semio_framework_plugin::Emit::<u8>::default();
        emit.artifact_mutations = vec![0_u8; mutations];
        let owes = owe_attached_previews_for_mutations(&mut session, &mut link, &windows, servable, &mut emit);
        assert_eq!(owes, expected["owes"].as_bool().unwrap(), "{id}: a gesture owes the previews exactly when it moved the document");
        let carried: Vec<&str> = emit.effects.iter().map(|effect| match effect {
            Effect::DispatchAction { action, .. } => action.as_str(),
            other => panic!("{id}: a gesture carries only dispatches, found {other:?}"),
        }).collect();
        assert_eq!(carried, expected["gestureEffects"].as_array().unwrap().iter().map(text).collect::<Vec<_>>(), "{id}: what the gesture's own emit carries — a settled, finalized run leaves nothing that would ask the host for another poll");
        let owed: Vec<&str> = windows.iter().filter(|(window_id, _)| session.window_tick_owed(window_id)).map(|(window_id, _)| *window_id).collect();
        assert_eq!(owed, expected["owedWindows"].as_array().unwrap().iter().map(text).collect::<Vec<_>>(), "{id}: owed windows");

        let roster: Vec<(String, &'static str)> = windows.iter().map(|(window_id, kind)| ((*window_id).to_string(), *kind)).collect();
        let mut hops: Vec<String> = Vec::new();
        while let PreviewEvalHop::Dispatch(index) = next_preview_eval_hop(&session, &roster) {
            let (window_id, _) = &roster[index];
            hops.push(window_id.clone());
            assert!(session.arm_window_tick(window_id), "{id}: a dispatched hop arms its window");
            session.begin_window_tick(window_id);
            session.note_window_tick_outcome(window_id, false);
            assert!(hops.len() <= roster.len(), "{id}: one landed mutation may never arm a window twice");
        }
        assert_eq!(hops, expected["hops"].as_array().unwrap().iter().map(text).collect::<Vec<_>>(), "{id}: hop schedule");

        // ⏯️ The storm half, on a fresh staging: the gesture again, then the host's own poll ladder with
        // its run view lagging behind every request it was handed.
        let mut spinning = FlowEvalSession::new();
        let mut spinning_link = PreviewEvalRunLink::default();
        stage(&mut spinning, &mut spinning_link);
        let mut spinning_emit = semio_framework_plugin::Emit::<u8>::default();
        spinning_emit.artifact_mutations = vec![0_u8; mutations];
        owe_attached_previews_for_mutations(&mut spinning, &mut spinning_link, &windows, servable, &mut spinning_emit);
        let starts = spinning_emit.effects.len() + replay_pending_effects(&mut spinning, &mut spinning_link, &windows, polls, lag, servable, poll_start_view);
        println!("[STATS] gestureRearm {id}: owes={owes} owed={owed:?} hops={hops:?} carried={carried:?} starts={starts}");
        assert_eq!(starts as u64, expected["starts"].as_u64().unwrap(), "{id}: one gesture owes at most one run start, however slowly the run view catches up");
        session.retire_cold();
        spinning.retire_cold();
    }
}

/// ⚖️ LAW: an UNDO owes the previews an evaluation, exactly as an edit does.
///
/// 🐛️ `undo`/`redo` are framework-reserved and never reach `dispatch_action`, so the emit-driven route
/// (`owe_attached_previews_for_mutations`) cannot see them: measured on :6023, `mod+z` after a slider
/// edit rewound `height` 8 → 6 in the published document while the preview kept delivering the 8-unit
/// extrusion, with neither a `toolRunStart` nor a `flowEvalTick` after the chord. The rule is the
/// APPLIED document-edit stack, which every route moves.
#[test]
fn a_history_verb_owes_the_previews_the_evaluation_a_gesture_would_have() {
    use semio_framework_plugin::plugin_app_close_prelude::{ActionKind, CommandView, HistoryView};
    let command = |seq: u64, applied: bool, edit: Option<&str>| CommandView {
        seq,
        action_id: "apply".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Update Widget", "Widget aktualisieren"),
        kind: ActionKind::Mutation,
        timestamp: String::new(),
        edit_id: edit.map(str::to_string),
        config_edit_id: None,
        child_edit_ids: Vec::new(),
        transition_id: None,
        author: None,
        op_lines: Vec::new(),
        op_count: 0,
        applied,
        revertible: true,
        count: 1,
        inverse: None,
        transaction: None,
        mutations: Vec::new(),
    };
    let view = |commands: Vec<CommandView>| HistoryView { commands, ..HistoryView::empty() };

    let edited = applied_document_edits_digest(&view(vec![command(2, true, Some("e2")), command(1, true, Some("e1"))]));
    let undone = applied_document_edits_digest(&view(vec![command(2, false, Some("e2")), command(1, true, Some("e1"))]));
    let redone = applied_document_edits_digest(&view(vec![command(2, true, Some("e2")), command(1, true, Some("e1"))]));
    assert_ne!(edited, undone, "an undo unapplies an edit, so the stack moves");
    assert_eq!(edited, redone, "a redo puts the same edit back, so the stack is the one it was");
    assert_ne!(undone, applied_document_edits_digest(&view(vec![command(1, true, Some("e1"))])), "a stack with an unapplied entry is not the stack without it");
    assert_eq!(
        applied_document_edits_digest(&view(vec![command(9, true, None), command(8, false, None)])),
        applied_document_edits_digest(&HistoryView::empty()),
        "cursor-motion and config rows carry no document edit and the evaluation does not read them",
    );

    let windows = [("procedural-preview", "procedural-preview")];
    let mut session = FlowEvalSession::new();
    let mut link = PreviewEvalRunLink::default();
    // 🚦️ First poll: the link learns the stack, the boot's own debt is paid and the run it asked for is
    // answered — which is the state a user is in when they press undo on a settled preview.
    preview_eval_run_effects(&mut session, &mut link, &windows, None, true, edited);
    session.note_window_tick_outcome("procedural-preview", false);
    link.requested = None;
    assert!(!session.window_tick_owed("procedural-preview"), "a paid window owes nothing while the document stands still");
    // ⏪️ The undo: no gesture, no emit, no app command — only the stack moving.
    let effects = preview_eval_run_effects(&mut session, &mut link, &windows, None, true, undone);
    assert!(session.window_tick_owed("procedural-preview"), "an undo owes the attached preview a fresh evaluation");
    assert_eq!(effects.len(), 1, "and asks for the run that pays it: {effects:?}");
    // 🔒️ …and it never asks for a SECOND run behind a gesture that already asked for one: the debt is
    // recorded, the request latch is not released, and the poll returns nothing.
    let mut gesturing = FlowEvalSession::new();
    let mut gesture_link = PreviewEvalRunLink::default();
    preview_eval_run_effects(&mut gesturing, &mut gesture_link, &windows, None, true, edited);
    let mut emit = semio_framework_plugin::Emit::<u8>::default();
    emit.artifact_mutations = vec![0_u8];
    owe_attached_previews_for_mutations(&mut gesturing, &mut gesture_link, &windows, true, &mut emit);
    assert_eq!(emit.effects.len(), 1, "the gesture carries its own run start");
    let second = preview_eval_run_effects(&mut gesturing, &mut gesture_link, &windows, None, true, undone);
    assert!(second.is_empty(), "the poll that sees the same move must ask for no second run, got {second:?}");
    assert!(gesturing.window_tick_owed("procedural-preview"), "the debt is still recorded");
    println!("[STATS] historyRearm edited={edited} undone={undone} effects={} second={}", effects.len(), second.len());
    gesturing.retire_cold();
    session.retire_cold();
}

/// ⚖️ LAW: the deflection ladder is ONE table — an editor mesh and a viewer mesh of the same handle
#[test]
fn every_job_supersession_row_lets_only_the_owning_job_quiesce_the_session() {
    for row in fixture()["jobSupersession"]["rows"].as_array().unwrap() {
        let id = text(&row["id"]);
        let mut link = PreviewEvalRunLink::default();
        link.job_run = row["linkRun"].as_u64();
        link.port = Some(semio_framework_plugin::ToolRunJobPort::default());
        link.settled = row["settled"].as_bool().unwrap().then_some((1, 0));
        let closing = row["closingRun"].as_u64().unwrap();
        let expected = &row["expected"];
        let owns = link.owned_by(closing);
        assert_eq!(owns, expected["owns"].as_bool().unwrap(), "{id}: ownership");

        // 🧯️ The close body, in the order `close_step` runs it: a job that does not own the link returns
        // before touching anything at all.
        let (mut cancels, mut clears_port) = (false, false);
        if owns {
            link.port = None;
            link.job_run = None;
            clears_port = true;
            cancels = link.settled.is_none();
        }
        println!("[STATS] jobSupersession {id}: owns={owns} cancels={cancels} clearsPort={clears_port}");
        assert_eq!(cancels, expected["cancels"].as_bool().unwrap(), "{id}: cancels the evaluation");
        assert_eq!(clears_port, expected["clearsPort"].as_bool().unwrap(), "{id}: clears the link's port");
        assert_eq!(link.port.is_none(), clears_port, "{id}: a superseded job may never take the live run's port away");
    }}
