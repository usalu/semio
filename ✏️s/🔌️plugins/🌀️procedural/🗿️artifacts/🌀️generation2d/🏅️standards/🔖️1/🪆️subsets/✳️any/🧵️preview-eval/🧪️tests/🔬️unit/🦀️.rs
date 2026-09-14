use super::*;
use crate::editor::generation2d::unit_tests::context::retire_flow_eval_session;
use semio_framework_artifact_flow_flow::neural::{Atom, ColdRetire, Dictionary, Value as NeuralValue};
use semio_framework_tool_run::{ToolRunId, ToolRunTraceKind};
use std::collections::BTreeSet;
use std::sync::Arc;

/// 📜️ The language-agnostic run law the editor answers.
const PREVIEW_EVAL_RUN_FIXTURE_JSON: &str = include_str!("../../../🧫️fixtures/⏯️preview-eval-run.json");

fn fixture() -> serde_json::Value {
    let fixture: serde_json::Value = serde_json::from_str(PREVIEW_EVAL_RUN_FIXTURE_JSON).expect("the preview evaluation run fixture parses");
    assert_eq!(fixture["format"], "semio.generation2d.preview-eval-run");
    assert_eq!(fixture["version"], 1);
    fixture
}

fn text(value: &serde_json::Value) -> &str {
    value.as_str().unwrap_or_else(|| panic!("fixture text expected, found {value}"))
}

fn json_text(value: &serde_json::Value) -> String {
    value.as_str().map_or_else(|| value.to_string(), str::to_string)
}

fn target_of(value: &serde_json::Value) -> PreviewEvalTarget {
    match text(value) {
        "document" => PreviewEvalTarget::Document,
        "generation" => PreviewEvalTarget::Generation,
        other => panic!("unknown target {other}"),
    }
}

/// 🧳️ A pair of live sessions a law replays latch steps on, retired together.
struct Sessions {
    document: FlowEvalSession,
    generation: FlowEvalSession,
}

impl Sessions {
    fn new() -> Self {
        Self { document: FlowEvalSession::new(), generation: FlowEvalSession::new() }
    }

    fn parts(&mut self) -> PreviewEvalSessions<'_> {
        PreviewEvalSessions { document: &mut self.document, generation: &mut self.generation }
    }

    fn retire(self) {
        retire_flow_eval_session(self.document);
        retire_flow_eval_session(self.generation);
    }
}

fn roster(rows: &serde_json::Value) -> Vec<(String, &'static str, PreviewEvalTarget)> {
    rows.as_array().unwrap().iter().map(|row| (text(&row["id"]).to_string(), "generation2d-preview", target_of(&row["target"]))).collect()
}

fn borrowed<'a>(windows: &'a [(String, &'static str, PreviewEvalTarget)]) -> Vec<PreviewEvalWindow<'a>> {
    windows.iter().map(|(window_id, kind, target)| (window_id.as_str(), *kind, *target)).collect()
}

/// ⚖️ LAW: a hop always carries BOTH halves of the window address, because the shell redispatches an
/// `Effect::DispatchAction` under whichever window is current.
#[test]
fn a_hop_carries_the_window_id_and_kind_it_was_addressed_to() {
    let hops = &fixture()["hopRequest"];
    let Effect::DispatchAction { action, args, .. } = tick_effect("preview-1", "generation2d-preview") else { panic!("a hop must be a dispatch effect") };
    assert_eq!(action, text(&hops["tickAction"]));
    let args = args.expect("an addressed hop carries args");
    for (key, value) in hops["args"].as_array().unwrap().iter().map(text).zip(["preview-1", "generation2d-preview"]) {
        assert_eq!(args.get(key).and_then(dsl::DslValue::as_str), Some(value), "{key}");
    }
}

/// ⚖️ LAW: every `targets` row — the edit preview evaluates the document session, the generate preview a
/// generation session of its own, and no other window kind evaluates anything.
#[test]
fn every_target_row_names_the_session_its_window_kind_evaluates() {
    use semio_framework_plugin::ViewWindowInstance;
    for row in fixture()["targets"]["rows"].as_array().unwrap() {
        let kind = text(&row["kind"]);
        let expected = (!row["target"].is_null()).then(|| target_of(&row["target"]));
        assert_eq!(crate::editor::generation2d::generation2d_preview_target(kind), expected, "{kind}");
        let view = ViewModel { window_instances: vec![ViewWindowInstance { id: "w".into(), window_kind_id: kind.into() }], ..Default::default() };
        assert_eq!(crate::editor::generation2d::generation2d_preview_windows(Some(&view)).iter().map(|(_, _, target)| *target).collect::<Vec<_>>(), expected.into_iter().collect::<Vec<_>>(), "{kind}: roster");
    }
    assert!(attached_preview_windows(None, &[("generation2d-preview", PreviewEvalTarget::Document)]).is_empty(), "no roster before the first surface");
}

/// ⚖️ LAW: the run declaration is the source of record verbatim, and the enums the job writes with name
/// exactly its stages, counters and reason codes with their verdicts.
#[test]
fn the_run_declaration_matches_its_source_of_record_and_the_job_vocabulary() {
    let fixture = fixture();
    let expected = &fixture["vocabulary"];
    let definition = preview_eval_run_definition();
    definition.validate().expect("the run definition is valid");
    assert_eq!(definition.mutating, expected["mutating"].as_bool().unwrap());
    assert_eq!(serde_json::to_value(definition.rebase).unwrap(), expected["rebase"]);
    assert_eq!(serde_json::to_value(definition.reconfigure).unwrap(), expected["reconfigure"]);
    assert_eq!(definition.trace, ToolRunTraceKind::Entity);
    assert_eq!(serde_json::to_value(definition.trace).unwrap(), expected["trace"]);
    assert_eq!(definition.stages.iter().map(|stage| stage.id.as_str()).collect::<Vec<_>>(), PreviewEvalRunStage::ALL.iter().map(|stage| stage.id()).collect::<Vec<_>>());
    assert_eq!(definition.stages.iter().map(|stage| stage.id.as_str()).collect::<Vec<_>>(), expected["stages"].as_array().unwrap().iter().map(text).collect::<Vec<_>>());
    assert_eq!(definition.counters.iter().map(|counter| counter.id.as_str()).collect::<Vec<_>>(), PreviewEvalRunCounter::ALL.iter().map(|counter| counter.id()).collect::<Vec<_>>());
    assert_eq!(definition.counters.iter().map(|counter| counter.id.as_str()).collect::<Vec<_>>(), expected["counters"].as_array().unwrap().iter().map(text).collect::<Vec<_>>());
    let reasons = expected["reasons"].as_array().unwrap();
    assert_eq!(reasons.len(), PreviewEvalRunReason::ALL.len());
    for (row, reason) in reasons.iter().zip(PreviewEvalRunReason::ALL) {
        let declared = definition.reason(reason.code()).unwrap_or_else(|| panic!("reason {} is declared", reason.id()));
        assert_eq!((u64::from(reason.code()), reason.id()), (row["code"].as_u64().unwrap(), text(&row["id"])));
        assert_eq!((declared.id.as_str(), declared.verdict), (reason.id(), reason.verdict()));
        assert_eq!(serde_json::to_value(reason.verdict()).unwrap(), row["verdict"]);
        println!("[STATS] reason {} code={} verdict={:?}", reason.id(), reason.code(), reason.verdict());
    }
    let tool = preview_eval_tool_definition();
    assert_eq!((tool.id.as_str(), tool.run.as_ref()), (text(&fixture["toolId"]), Some(&definition)));
}

/// ⚖️ LAW: a node's trace entity is FNV-1a 64 of its id, checked against the `fnv` crate as the
/// third-party oracle and against the fixture's committed keys.
#[test]
fn a_node_entity_is_fnv1a_of_its_id_under_the_fnv_crate_oracle() {
    use std::hash::Hasher;
    for row in fixture()["entityKeys"].as_array().unwrap() {
        let node_id = text(&row["nodeId"]);
        let mut oracle = fnv::FnvHasher::default();
        oracle.write(node_id.as_bytes());
        assert_eq!(preview_eval_node_entity(node_id), oracle.finish(), "{node_id:?}: the fnv crate disagrees");
        assert_eq!(preview_eval_node_entity(node_id).to_string(), text(&row["entity"]), "{node_id:?}: the committed key");
    }
}

/// ⚖️ LAW: every `observations` row — the verdict each node carries, merged over every observed target
/// into its least settled reason, the stage and the settled/total pair.
#[test]
fn every_observation_row_answers_its_verdicts_stage_and_progress() {
    for row in fixture()["observations"].as_array().unwrap() {
        let id = text(&row["id"]);
        let statuses: Vec<String> = row["targets"].as_array().unwrap().iter().map(json_text).collect();
        let observation = observe_preview_eval(&statuses.iter().map(String::as_str).collect::<Vec<_>>());
        let expected = &row["expected"];
        let nodes: BTreeMap<String, &str> = observation.nodes.iter().map(|(node, reason)| (node.clone(), reason.id())).collect();
        let expected_nodes: BTreeMap<String, &str> = expected["nodes"].as_object().unwrap().iter().map(|(node, reason)| (node.clone(), text(reason))).collect();
        println!("[STATS] observation {id}: nodes={nodes:?} stage={} progress={:?}", observation.stage().id(), observation.progress());
        assert_eq!(nodes, expected_nodes, "{id}: node verdicts");
        assert_eq!(observation.stage().id(), text(&expected["stage"]), "{id}: stage");
        let progress = expected["progress"].as_array().unwrap();
        assert_eq!(observation.progress(), (progress[0].as_u64().unwrap(), progress[1].as_u64().unwrap()), "{id}: progress");
    }
}

fn replay_latch_steps(sessions: &mut Sessions, windows: &[(String, &'static str, PreviewEvalTarget)], steps: &[serde_json::Value]) {
    let mut link = PreviewEvalRunLink::default();
    for step in steps {
        let window = text(&step["window"]);
        let target = windows.iter().find(|(window_id, _, _)| window_id == window).map_or(PreviewEvalTarget::Document, |(_, _, target)| *target);
        if text(&step["step"]) == "owe" {
            owe_attached_previews(sessions.parts(), &mut link, &[(window, "generation2d-preview", target)]);
            continue;
        }
        let mut parts = sessions.parts();
        let session = parts.get_mut(target);
        match text(&step["step"]) {
            "arm" => assert!(session.arm_window_tick(window), "{window}: arm"),
            "begin" => session.begin_window_tick(window),
            "outcome" => session.note_window_tick_outcome(window, tick_is_unfinished(step["more"].as_bool().unwrap(), step["parked"].as_u64().unwrap() as usize)),
            "inFlight" => session.note_window_extensions_in_flight(window, step["count"].as_u64().unwrap() as usize),
            "settle" => {
                let output = Dictionary::new().insert("sum", NeuralValue::Atom(Atom::Integer(3)));
                resolve_eval(&FlowEvalResolve { window_id: window.into(), window_kind_id: "generation2d-preview".into(), node_hash: 1, output_json: dsl::json::to_json_string(&output), ok: true, ..Default::default() }, session);
                output.retire_cold();
            }
            "abort" => {
                session.cancel_preview_evaluation(window);
            }
            other => panic!("the fixture named an unknown latch step {other}"),
        }
    }
}

/// ⚖️ LAW: every `scheduling` row — what the run does next over the attached windows' latches, each read
/// off its own target session: dispatch the first window that owes a hop nothing is chasing, wait while
/// any hop or answer is outstanding, settle when nothing is owed or awaited.
#[test]
fn every_scheduling_row_answers_dispatch_wait_or_settled() {
    for row in fixture()["scheduling"].as_array().unwrap() {
        let id = text(&row["id"]);
        let mut sessions = Sessions::new();
        let windows = roster(&row["windows"]);
        replay_latch_steps(&mut sessions, &windows, row["steps"].as_array().unwrap());
        let hop = next_preview_eval_hop(&sessions.parts(), &windows);
        println!("[STATS] scheduling {id}: {hop:?}");
        let expected = match text(&row["expected"]["hop"]) {
            "dispatch" => PreviewEvalHop::Dispatch(windows.iter().position(|(window, _, _)| window == text(&row["expected"]["window"])).expect("the expected window is attached")),
            "wait" => PreviewEvalHop::Wait,
            "settled" => PreviewEvalHop::Settled,
            other => panic!("unknown hop {other}"),
        };
        assert_eq!(hop, expected, "{id}");
        sessions.retire();
    }
}

fn run_view(row: &serde_json::Value) -> Option<ToolRunView> {
    let run = row.as_object()?;
    Some(ToolRunView::new(
        run.get("toolId").and_then(serde_json::Value::as_str).unwrap_or(PREVIEW_EVAL_TOOL_ID),
        ToolRunIdentity { id: ToolRunId { app_instance_id: 1, run: run.get("run").and_then(serde_json::Value::as_u64).unwrap_or(1) }, generation: run["generation"].as_u64().unwrap() as u32, base_revision: [0; 32] },
        ToolRunState::parse(text(&run["state"])).expect("a fixture run state"),
    ))
}

fn settle_windows(sessions: &mut Sessions, windows: &[PreviewEvalWindow<'_>]) {
    for (window_id, _, target) in windows {
        let mut parts = sessions.parts();
        let session = parts.get_mut(*target);
        session.arm_window_tick(window_id);
        session.begin_window_tick(window_id);
        session.note_window_tick_outcome(window_id, false);
    }
}

fn unmoved(link: &mut PreviewEvalRunLink, windows: &[PreviewEvalWindow<'_>]) -> BTreeMap<PreviewEvalTarget, u64> {
    let current: BTreeMap<PreviewEvalTarget, u64> = windows.iter().map(|(_, _, target)| (*target, 1)).collect();
    for (target, digest) in &current {
        link.note_evaluated(*target, *digest);
    }
    current
}

fn left_windows(row: &serde_json::Value) -> Vec<PreviewEvalWindow<'_>> {
    row.as_array().unwrap().iter().map(|window| (text(window), "generation2d-preview", PreviewEvalTarget::Document)).collect()
}

/// ⚖️ LAW: every `runEffects` row — what the editor's `pending_effects` owes the run over an unmoved document.
#[test]
fn every_run_effects_row_starts_finalizes_or_leaves_the_run() {
    for row in fixture()["runEffects"].as_array().unwrap() {
        let id = text(&row["id"]);
        let mut sessions = Sessions::new();
        let mut link = PreviewEvalRunLink::default();
        let windows = left_windows(&row["windows"]);
        if !row["owed"].as_bool().unwrap() {
            settle_windows(&mut sessions, &windows);
        }
        let current = unmoved(&mut link, &windows);
        let run = run_view(&row["run"]);
        if let (Some(run), Some(settled)) = (run.as_ref(), row["run"].get("settledGeneration").and_then(serde_json::Value::as_u64)) {
            link.settled = Some((run.identity.id.run, settled as u32));
        }
        link.restart_owed = row["restartOwed"].as_bool().unwrap_or(false);
        let effects = preview_eval_run_effects(sessions.parts(), &mut link, &windows, &current, run.as_ref(), row["servable"].as_bool().unwrap_or(true));
        let actions: Vec<&str> = effects
            .iter()
            .map(|effect| match effect {
                Effect::DispatchAction { action, .. } => action.as_str(),
                other => panic!("{id}: the run owes only dispatches, found {other:?}"),
            })
            .collect();
        println!("[STATS] runEffects {id}: {actions:?}");
        assert_eq!(actions, row["expected"].as_array().unwrap().iter().map(text).collect::<Vec<_>>(), "{id}");
        for effect in &effects {
            let Effect::DispatchAction { action, args: Some(args), .. } = effect else { continue };
            match action.as_str() {
                TOOL_RUN_START_ACTION_ID => assert_eq!(args.get(TOOL_RUN_ARG_TOOL_ID).and_then(dsl::DslValue::as_str), Some(PREVIEW_EVAL_TOOL_ID), "{id}: start names the tool"),
                _ => assert_eq!(args.get(TOOL_RUN_ARG_RUN_ID).and_then(dsl::DslValue::as_str), run.as_ref().map(|run| run.identity.id.run.to_string()).as_deref(), "{id}: finalize names the run"),
            }
        }
        sessions.retire();
    }
}

/// ⚖️ LAW: every `documentMoved` row — a poll owes exactly the windows whose target's digest moved since
/// that target last evaluated, and nothing over an unmoved document.
#[test]
fn every_document_moved_row_owes_exactly_the_windows_of_a_moved_target() {
    let digests = |value: &serde_json::Value| -> BTreeMap<PreviewEvalTarget, u64> { value.as_object().unwrap().iter().map(|(target, digest)| (target_of(&serde_json::Value::String(target.clone())), digest.as_u64().unwrap())).collect() };
    for row in fixture()["documentMoved"]["rows"].as_array().unwrap() {
        let id = text(&row["id"]);
        let windows = roster(&row["windows"]);
        let attached = borrowed(&windows);
        let mut sessions = Sessions::new();
        settle_windows(&mut sessions, &attached);
        let mut link = PreviewEvalRunLink::default();
        for (target, digest) in digests(&row["evaluated"]) {
            link.note_evaluated(target, digest);
        }
        owe_moved_targets(&mut sessions.parts(), &link, &attached, &digests(&row["current"]));
        let parts = sessions.parts();
        let owed: Vec<&str> = attached.iter().filter(|(window_id, _, target)| parts.get(*target).window_tick_owed(window_id)).map(|(window_id, _, _)| *window_id).collect();
        println!("[STATS] documentMoved {id}: owed={owed:?}");
        assert_eq!(owed, row["expected"]["owedWindows"].as_array().unwrap().iter().map(text).collect::<Vec<_>>(), "{id}");
        sessions.retire();
    }
}

/// 🎞️ One `pending_effects` ladder replayed as the host runs it: a run action is a REQUEST, so the run
/// view keeps answering what it answered before for `start_lag_polls` further polls. Answers how many
/// `toolRunStart` invocations the ladder cost.
fn replay_pending_effects(sessions: &mut Sessions, link: &mut PreviewEvalRunLink, windows: &[PreviewEvalWindow<'_>], polls: usize, start_lag_polls: usize, servable: bool, poll_start_view: &str) -> usize {
    let current = unmoved(link, windows);
    let mut view = (poll_start_view != "none").then(|| run_view(&serde_json::json!({ "run": 1, "generation": 0, "state": poll_start_view })).expect("a fixture run view"));
    let mut starts = 0_usize;
    let mut landing: Option<(usize, ToolRunState)> = None;
    for _ in 0..polls {
        for effect in preview_eval_run_effects(sessions.parts(), link, windows, &current, view.as_ref(), servable) {
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

/// ⚖️ LAW: every `gestureRearm` row — a gesture that MOVED THE DOCUMENT owes every attached preview window
/// exactly one re-armed hop and carries, on its OWN emit, the run start that debt needs when no live run
/// is there to be woken; a gesture that authored no artifact mutation owes a settled run nothing; and one
/// gesture costs at most ONE `toolRunStart` however slowly the host's run view catches up.
#[test]
fn every_gesture_rearm_row_owes_one_hop_per_preview_and_at_most_one_start() {
    for row in fixture()["gestureRearm"]["rows"].as_array().unwrap() {
        let id = text(&row["id"]);
        let windows = left_windows(&row["windows"]);
        let settled_windows = row["windowsSettled"].as_bool().unwrap();
        let live = row["runIsLive"].as_bool().unwrap();
        let has_settled = row["runHasSettled"].as_bool().unwrap();
        let servable = row["servable"].as_bool().unwrap();
        let polls = row["polls"].as_u64().unwrap() as usize;
        let lag = row["startLagPolls"].as_u64().unwrap() as usize;
        let poll_start_view = text(&row["pollStartView"]);
        let expected = &row["expected"];
        let mutations = row["artifactMutations"].as_u64().unwrap() as usize;
        let stage = |sessions: &mut Sessions, link: &mut PreviewEvalRunLink| {
            if settled_windows {
                settle_windows(sessions, &windows);
            }
            link.settled = has_settled.then_some((1, 0));
            if live {
                link.port = Some(ToolRunJobPort::default());
            }
        };

        let mut sessions = Sessions::new();
        let mut link = PreviewEvalRunLink::default();
        stage(&mut sessions, &mut link);
        if settled_windows {
            let current = unmoved(&mut link, &windows);
            let complete = run_view(&serde_json::json!({ "run": 1, "generation": 0, "state": "complete" }));
            let mut asked: Vec<&str> = Vec::new();
            for _ in 0..polls {
                for effect in preview_eval_run_effects(sessions.parts(), &mut link, &windows, &current, complete.as_ref(), true) {
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
        let owes = owe_attached_previews_for_mutations(sessions.parts(), &mut link, &windows, servable, &mut emit);
        assert_eq!(owes, expected["owes"].as_bool().unwrap(), "{id}: a gesture owes the previews exactly when it moved the document");
        let carried: Vec<&str> = emit
            .effects
            .iter()
            .map(|effect| match effect {
                Effect::DispatchAction { action, .. } => action.as_str(),
                other => panic!("{id}: a gesture carries only dispatches, found {other:?}"),
            })
            .collect();
        assert_eq!(carried, expected["gestureEffects"].as_array().unwrap().iter().map(text).collect::<Vec<_>>(), "{id}: what the gesture's own emit carries");
        let owed: Vec<&str> = windows.iter().filter(|(window_id, _, _)| sessions.document.window_tick_owed(window_id)).map(|(window_id, _, _)| *window_id).collect();
        assert_eq!(owed, expected["owedWindows"].as_array().unwrap().iter().map(text).collect::<Vec<_>>(), "{id}: owed windows");

        let attached: Vec<(String, &'static str, PreviewEvalTarget)> = windows.iter().map(|(window_id, kind, target)| ((*window_id).to_string(), *kind, *target)).collect();
        let mut hops: Vec<String> = Vec::new();
        while let PreviewEvalHop::Dispatch(index) = next_preview_eval_hop(&sessions.parts(), &attached) {
            let (window_id, _, _) = &attached[index];
            hops.push(window_id.clone());
            assert!(sessions.document.arm_window_tick(window_id), "{id}: a dispatched hop arms its window");
            sessions.document.begin_window_tick(window_id);
            sessions.document.note_window_tick_outcome(window_id, false);
            assert!(hops.len() <= attached.len(), "{id}: one landed mutation may never arm a window twice");
        }
        assert_eq!(hops, expected["hops"].as_array().unwrap().iter().map(text).collect::<Vec<_>>(), "{id}: hop schedule");

        let mut spinning = Sessions::new();
        let mut spinning_link = PreviewEvalRunLink::default();
        stage(&mut spinning, &mut spinning_link);
        let mut spinning_emit = semio_framework_plugin::Emit::<u8>::default();
        spinning_emit.artifact_mutations = vec![0_u8; mutations];
        owe_attached_previews_for_mutations(spinning.parts(), &mut spinning_link, &windows, servable, &mut spinning_emit);
        let starts = spinning_emit.effects.len() + replay_pending_effects(&mut spinning, &mut spinning_link, &windows, polls, lag, servable, poll_start_view);
        println!("[STATS] gestureRearm {id}: owes={owes} owed={owed:?} hops={hops:?} carried={carried:?} starts={starts}");
        assert_eq!(starts as u64, expected["starts"].as_u64().unwrap(), "{id}: one gesture owes at most one run start, however slowly the run view catches up");
        sessions.retire();
        spinning.retire();
    }
}

/// ⚖️ LAW: every `jobSupersession` row — only the run job the link currently belongs to may quiesce the
/// sessions on close; a superseded job closes quietly, and a settled one closes without cancelling.
#[test]
fn every_job_supersession_row_lets_only_the_owning_job_quiesce_the_sessions() {
    for row in fixture()["jobSupersession"]["rows"].as_array().unwrap() {
        let id = text(&row["id"]);
        let mut link = PreviewEvalRunLink::default();
        link.job_run = row["linkRun"].as_u64();
        link.port = Some(ToolRunJobPort::default());
        link.settled = row["settled"].as_bool().unwrap().then_some((1, 0));
        let closing = row["closingRun"].as_u64().unwrap();
        let expected = &row["expected"];
        let owns = link.owned_by(closing);
        assert_eq!(owns, expected["owns"].as_bool().unwrap(), "{id}: ownership");
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
    }
}

/// ⚖️ LAW: a digest is a function of every part AND of where the parts split, so a moved generation
/// selection is never mistaken for an unmoved one.
#[test]
fn a_preview_eval_digest_separates_its_parts() {
    assert_eq!(preview_eval_digest(&["a", "b"]), preview_eval_digest(&["a", "b"]));
    assert_ne!(preview_eval_digest(&["ab", ""]), preview_eval_digest(&["a", "b"]));
    assert_ne!(preview_eval_digest(&["fixture", "g1"]), preview_eval_digest(&["fixture", "g2"]));
}
