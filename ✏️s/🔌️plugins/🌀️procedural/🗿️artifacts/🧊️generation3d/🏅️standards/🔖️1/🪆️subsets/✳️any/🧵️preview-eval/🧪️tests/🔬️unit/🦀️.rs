use super::*;
use semio_framework_tool_run::{ToolRunId, ToolRunTraceKind};
use std::collections::BTreeSet;
use std::sync::Arc;

/// 📜️ The language-agnostic run law both surfaces answer.
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

fn json_text(value: &serde_json::Value) -> String {
    value.as_str().map_or_else(|| value.to_string(), str::to_string)
}

/// ⚖️ LAW: a hop always carries BOTH halves of the window address, because the shell redispatches an
/// `Effect::DispatchAction` under whichever window is current and the retained route matches the
/// payload against the ViewModel roster, never against that window.
#[test]
fn a_hop_carries_the_window_id_and_kind_it_was_addressed_to() {
    let hops = &fixture()["hopRequest"];
    for (effect, action) in [(tick_effect("preview-1", "procedural-view-preview"), &hops["tickAction"]), (release_effect("preview-1", "procedural-view-preview"), &hops["releaseAction"])] {
        let Effect::DispatchAction { action: dispatched, args, .. } = effect else { panic!("a hop must be a dispatch effect") };
        assert_eq!(dispatched, text(action));
        let args = args.expect("an addressed hop carries args");
        assert_eq!(args.get("windowId").and_then(dsl::DslValue::as_str), Some("preview-1"));
        assert_eq!(args.get("windowKindId").and_then(dsl::DslValue::as_str), Some("procedural-view-preview"));
    }
}

/// ⚖️ LAW: only a roster entry whose kind the surface declared as a preview is an evaluation window,
/// and a surface that declares no preview kind has none.
#[test]
fn only_declared_preview_kinds_are_evaluation_windows() {
    use semio_framework_plugin::ViewWindowInstance;
    let view = ViewModel {
        window_instances: vec![ViewWindowInstance { id: "main".into(), window_kind_id: "procedural-main".into() }, ViewWindowInstance { id: "view-preview".into(), window_kind_id: "procedural-view-preview".into() }],
        ..Default::default()
    };
    let viewer_kinds: &[&'static str] = &["procedural-view-preview"];
    assert_eq!(attached_preview_windows(Some(&view), viewer_kinds), vec![("view-preview", "procedural-view-preview")]);
    assert!(attached_preview_windows(Some(&view), &[]).is_empty());
    assert!(attached_preview_windows(None, viewer_kinds).is_empty());
    assert_eq!(preview_kind("procedural-main", viewer_kinds), None);
}

/// ⚖️ LAW: the run declaration is the source of record verbatim, and the enums the job writes with
/// name exactly its stages, counters and reason codes with their verdicts.
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

fn mesh_state_of(states: &serde_json::Value, handle: &str) -> PreviewEvalMeshState {
    match states.get(handle).and_then(serde_json::Value::as_str) {
        Some("ready") => PreviewEvalMeshState::Ready,
        Some("diagnostics") => PreviewEvalMeshState::Diagnostics,
        _ => PreviewEvalMeshState::Pending,
    }
}

/// ⚖️ LAW: every `observations` row — the verdict each node carries, the mesh census, the stage and the
/// progress pair read off one published status and evaluation.
#[test]
fn every_observation_row_answers_its_verdicts_census_stage_and_progress() {
    for row in fixture()["observations"].as_array().unwrap() {
        let id = text(&row["id"]);
        let preview_widget_ids: Vec<String> = row["previewWidgetIds"].as_array().unwrap().iter().map(|value| text(value).to_string()).collect();
        let observation = observe_preview_eval(&json_text(&row["statusJson"]), &json_text(&row["evalJson"]), &preview_widget_ids, |handle| mesh_state_of(&row["meshStates"], handle), row["settled"].as_bool().unwrap());
        let expected = &row["expected"];
        let nodes: BTreeMap<String, &str> = observation.nodes.iter().map(|(node, reason)| (node.clone(), reason.id())).collect();
        let expected_nodes: BTreeMap<String, &str> = expected["nodes"].as_object().unwrap().iter().map(|(node, reason)| (node.clone(), text(reason))).collect();
        println!("[STATS] observation {id}: nodes={nodes:?} meshes={} tessellated={} stage={} progress={:?}", observation.meshes, observation.tessellated, observation.stage().id(), observation.progress());
        assert_eq!(nodes, expected_nodes, "{id}: node verdicts");
        assert_eq!((observation.meshes, observation.tessellated), (expected["meshes"].as_u64().unwrap(), expected["tessellated"].as_u64().unwrap()), "{id}: mesh census");
        assert_eq!(observation.stage().id(), text(&expected["stage"]), "{id}: stage");
        let progress = expected["progress"].as_array().unwrap();
        assert_eq!(observation.progress(), (progress[0].as_u64().unwrap(), progress[1].as_u64().unwrap()), "{id}: progress");
    }
}

fn replay_latch_steps(session: &mut FlowEvalSession, steps: &[serde_json::Value]) {
    let mut link = PreviewEvalRunLink::default();
    for step in steps {
        let window = text(&step["window"]);
        match text(&step["step"]) {
            "arm" => assert!(session.arm_window_tick(window), "{window}: arm"),
            "begin" => session.begin_window_tick(window),
            "outcome" => session.note_window_tick_outcome(window, tick_is_unfinished(step["more"].as_bool().unwrap(), step["parked"].as_u64().unwrap() as usize)),
            "inFlight" => session.note_window_extensions_in_flight(window, step["count"].as_u64().unwrap() as usize),
            "settle" => resolve_tessellate(&FlowTessellateResolve { window_id: window.into(), window_kind_id: "procedural-view-preview".into(), node_hash: 1, output_json: r#"{"done":true,"phase":"complete","unitsDone":3,"unitsTotal":3}"#.into() }, session),
            "owe" => owe_attached_previews(session, &mut link, &[(window, "procedural-view-preview")]),
            "abort" => {
                session.cancel_preview_evaluation(window);
            }
            other => panic!("the fixture named an unknown latch step {other}"),
        }
    }
}

/// ⚖️ LAW: every `scheduling` row — what the run does next over the attached windows' latches: dispatch
/// the first window that owes a hop nothing is chasing, wait while any hop or answer is outstanding,
/// settle when nothing is owed or awaited. Answers never arm anything themselves.
#[test]
fn every_scheduling_row_answers_dispatch_wait_or_settled() {
    for row in fixture()["scheduling"].as_array().unwrap() {
        let id = text(&row["id"]);
        let mut session = FlowEvalSession::new();
        let windows: Vec<(String, &'static str)> = row["windows"].as_array().unwrap().iter().map(|window| (text(window).to_string(), "procedural-view-preview")).collect();
        replay_latch_steps(&mut session, row["steps"].as_array().unwrap());
        let hop = next_preview_eval_hop(&session, &windows);
        println!("[STATS] scheduling {id}: {hop:?}");
        let expected = match text(&row["expected"]["hop"]) {
            "dispatch" => PreviewEvalHop::Dispatch(windows.iter().position(|(window, _)| window == text(&row["expected"]["window"])).expect("the expected window is attached")),
            "wait" => PreviewEvalHop::Wait,
            "settled" => PreviewEvalHop::Settled,
            other => panic!("unknown hop {other}"),
        };
        assert_eq!(hop, expected, "{id}");
        crate::flow_operators::retire_flow_eval_session(session);
    }
}

/// ⚖️ LAW: the kernel release reaches BOTH retained registries of the geometry extension through its
/// own capabilities, each answering on the one acknowledgement route; an unaddressable extension has
/// no actor to tell.
#[test]
fn the_kernel_release_reaches_both_extension_registries_or_nothing() {
    let expected = &fixture()["release"];
    let payload = FlowEvalRelease { window_id: "preview-1".into(), window_kind_id: "procedural-view-preview".into() };
    let invocations = release_invocations_for(&payload, Ok("flow-extension-brep".into()));
    assert_eq!(invocations.iter().map(|invocation| invocation.capability.as_str()).collect::<Vec<_>>(), expected["capabilities"].as_array().unwrap().iter().map(text).collect::<Vec<_>>());
    assert!(invocations.iter().all(|invocation| invocation.response_action == text(&expected["responseAction"]) && invocation.extension_id == "flow-extension-brep"));
    let miss = semio_framework_os_flow::FlowExtensionAddressMiss { extension_id: GENERATION_3D_GEOMETRY_EXTENSION_ID.into(), contributed: Vec::new() };
    assert_eq!(release_invocations_for(&payload, Err(miss)).len() as u64, expected["unaddressableEmits"].as_u64().unwrap());
}

fn run_view(row: &serde_json::Value) -> Option<ToolRunView> {
    let run = row.as_object()?;
    Some(ToolRunView::new(
        run.get("toolId").and_then(serde_json::Value::as_str).unwrap_or(PREVIEW_EVAL_TOOL_ID),
        ToolRunIdentity { id: ToolRunId { app_instance_id: 1, run: run.get("run").and_then(serde_json::Value::as_u64).unwrap_or(1) }, generation: run["generation"].as_u64().unwrap() as u32, base_revision: [0; 32] },
        ToolRunState::parse(text(&run["state"])).expect("a fixture run state"),
    ))
}

/// ⚖️ LAW: every `status` row — a preview window offers the framework abort of ITS OWN run exactly while
/// that run has work to stop, naming the run it stops.
#[test]
fn every_status_row_offers_the_run_abort_exactly_while_it_can_stop_work() {
    for row in fixture()["status"].as_array().unwrap() {
        let id = text(&row["id"]);
        let run = run_view(&row["run"]);
        let status: serde_json::Value = serde_json::from_str(&preview_progress_status_json_for(None, run.as_ref(), Ok("flow-extension-brep".into()))).expect("status json");
        println!("[STATS] status {id}: cancellable={} cancelAction={} cancelArgs={}", status["cancellable"], status["cancelAction"], status["cancelArgs"]);
        let expected = &row["expected"];
        assert_eq!(status["cancellable"], expected["cancellable"], "{id}: cancellable");
        assert_eq!(status["cancelAction"], expected["cancelAction"], "{id}: cancelAction");
        assert_eq!(status.get("cancelArgs").cloned().unwrap_or(serde_json::Value::Null), expected["cancelArgs"], "{id}: cancelArgs");
    }
    let miss = semio_framework_os_flow::FlowExtensionAddressMiss { extension_id: GENERATION_3D_GEOMETRY_EXTENSION_ID.into(), contributed: Vec::new() };
    let running = run_view(&serde_json::json!({ "run": 1, "generation": 0, "state": "running" }));
    let faulted: serde_json::Value = serde_json::from_str(&preview_progress_status_json_for(None, running.as_ref(), Err(miss))).expect("status json");
    assert_eq!((faulted["phase"].as_str(), faulted["cancellable"].as_bool()), (Some("faulted"), Some(false)), "an unaddressable kernel has no work an abort could stop");
}

/// ⚖️ LAW: every `runEffects` row — what a surface's `pending_effects` owes the run.
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
        let effects = preview_eval_run_effects(&mut session, &mut link, &windows, run.as_ref(), row["servable"].as_bool().unwrap_or(true));
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
        crate::flow_operators::retire_flow_eval_session(session);
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
        for effect in preview_eval_run_effects(session, link, windows, view.as_ref(), servable) {
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
                for effect in preview_eval_run_effects(&mut session, &mut link, &windows, complete.as_ref(), true) {
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
        crate::flow_operators::retire_flow_eval_session(session);
        crate::flow_operators::retire_flow_eval_session(spinning);
    }
}

/// ⚖️ LAW: the deflection ladder is ONE table — an editor mesh and a viewer mesh of the same handle
/// at the same LOD are tessellated to the same tolerance, so the two surfaces cannot drift.
#[test]
fn the_lod_deflection_ladder_is_shared_and_monotone() {
    assert!(preview_tolerance("coarse") > preview_tolerance("medium"));
    assert!(preview_tolerance("medium") > preview_tolerance("fine"));
    assert_eq!(preview_tolerance(""), preview_tolerance("medium"), "an unset LOD is medium");
}

/// ⚖️ LAW: every `jobSupersession` row — only the run job the link currently belongs to may quiesce the
/// session on close; a superseded job closes quietly, and a settled one closes without cancelling.
///
/// 🪪️ A `toolRunStart` retires the previous entry, so the OLD job's `close_step` runs AFTER the new job
/// has installed its own port and reset the link's settled marker. Reading those as its own, the closing
/// job cancelled the evaluation the FRESH run was in the middle of: the generate preview came back
/// `phase: "cancelled"` with an empty payload right after `addGeneration`
/// (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`).
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
    }
}

/// 📜️ The SHARED, language-neutral status-timeline fixture — the very rows the wgpu shell's Rust law
/// (`🔬️wgpu-shell-chrome-parity`) and the TypeScript `world3dComputeStatusV1` twin
/// (`🔬️engine-contract`) answer. This law closes the third side of the triangle: that the PRODUCER
/// actually emits a timeline of that shape, which is the half neither renderer can assert.
const SURFACE_CONTROLS_FIXTURE_JSON: &str = include_str!("../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧫️fixtures/🛑️surface-controls/🔣️.json");

fn published(session: &FlowEvalSession, run: Option<&ToolRunView>) -> serde_json::Value {
    serde_json::from_str(&preview_progress_status_json_for(Some(session), run, Ok(GENERATION_3D_GEOMETRY_EXTENSION_ID.to_string()))).expect("the projection publishes json")
}

/// ⚖️ LAW: a live evaluation publishes a RUN of non-idle frames with a monotone ratio and an offered
/// abort, and only a settled one publishes `idle` — at least as many non-idle frames as the shared
/// `progressTimeline` fixture declares.
///
/// 🩸️ Before the chain ledger this projection read only `pending_tessellate_by_hash` and
/// `eval_progress_by_hash`, both of which are EMPTY at every hop boundary — so every status a
/// preview window published during an evaluation was byte-identical
/// `phase: "idle", inFlight: 0, ratio: 1.0`, on wgpu and on React alike. No renderer can show
/// progress for work whose producer never says it is working
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-progress-visibility-2026-09-14.md`).
#[test]
fn a_live_evaluation_publishes_non_idle_frames_with_a_monotone_ratio_and_an_offered_abort() {
    let shared: serde_json::Value = serde_json::from_str(SURFACE_CONTROLS_FIXTURE_JSON).expect("the shared surface-controls fixture parses");
    let minimum = shared["progressTimeline"]["minimumNonIdleFrames"].as_u64().expect("the shared fixture declares a minimum") as usize;
    let run = run_view(&serde_json::json!({ "run": 1, "generation": 0, "state": "running" }));
    let mut host = semio_framework_os_flow::FlowHost::default();
    let mut session = FlowEvalSession::new();

    let idle = published(&session, None);
    assert_eq!(idle["phase"], "idle", "a session with nothing to do is idle");
    assert_eq!(idle["progress"]["ratio"], serde_json::json!(1.0));

    assert!(session.sync(&host), "the default demo graph has pending nodes, so a chain is armed");
    session.note_window_tick_outcome("preview-1", true);

    // ⛓️ The hop ladder the run job actually drives, sampled at every boundary a preview window
    // publishes at: arm the hop, park its `evaluate`/`tessellate` request, fold the answer, arm the
    // next. ONE tick chain spans many round trips — a budgeted operator answers `done: false`
    // several times for the same node before it produces anything — and the measured 6118 chain for
    // `sphere-cut-with-torus` is exactly that shape: five round trips, one settle
    // (`📓️wgpu-progress-visibility-2026-09-14.md`). Every one of these boundaries is a status the
    // surface published, and every one of them used to read `idle`.
    let mut frames = Vec::new();
    for round in 0..minimum {
        session.arm_window_tick("preview-1");
        session.begin_window_tick("preview-1");
        session.note_window_extensions_in_flight("preview-1", 1);
        frames.push(published(&session, run.as_ref()));
        let more = if round + 1 == minimum { session.tick(&mut host, None) } else { true };
        session.settle_window_extension("preview-1");
        session.note_window_tick_outcome("preview-1", more);
        frames.push(published(&session, run.as_ref()));
    }
    while session.tick(&mut host, None) {
        frames.push(published(&session, run.as_ref()));
    }
    session.note_window_tick_outcome("preview-1", false);

    let non_idle: Vec<&serde_json::Value> = frames.iter().filter(|frame| frame["phase"] != "idle").collect();
    let mut previous = f64::NEG_INFINITY;
    for frame in &non_idle {
        assert_eq!(frame["phase"], "computing", "a live chain names the phase it is in");
        assert_eq!(frame["phaseLabel"]["en"], "Computing");
        assert_eq!(frame["phaseLabel"]["de"], "Berechnen", "both tongues, no default language");
        assert_eq!(frame["cancellable"], true, "work in flight offers its abort");
        assert_eq!(frame["cancelAction"], "toolRunAbort");
        let ratio = frame["progress"]["ratio"].as_f64().expect("a published ratio");
        assert!(ratio >= previous - 1e-9, "the published ratio went backwards: {previous} → {ratio}");
        assert!(ratio < 1.0, "a live evaluation may never publish a complete ratio");
        assert!(frame["progress"]["nodesTotal"].as_u64().unwrap_or(0) > 0, "a live chain publishes the denominator its pill needs");
        previous = ratio;
    }
    assert!(non_idle.len() >= minimum, "{} non-idle frames, the shared fixture needs at least {minimum}", non_idle.len());

    let settled = published(&session, None);
    assert_eq!(settled["phase"], "idle", "the timeline ends settled");
    assert_eq!(settled["progress"]["ratio"], serde_json::json!(1.0));
    println!("[STATS] progress timeline: {} frames, {} non-idle, ratios {:?}", frames.len(), non_idle.len(), non_idle.iter().map(|frame| frame["progress"]["ratio"].as_f64().unwrap_or(-1.0)).collect::<Vec<_>>());
    session.retire_cold();
    host.retire_cold();
}

/// ⚖️ LAW: an abort taken while the chain is live settles on `cancelled`, never back on `idle` — the
/// `cancelled` lane of the same shared timeline fixture.
#[test]
fn an_abort_while_the_chain_is_live_settles_on_cancelled() {
    let run = run_view(&serde_json::json!({ "run": 1, "generation": 0, "state": "running" }));
    let mut host = semio_framework_os_flow::FlowHost::default();
    let mut session = FlowEvalSession::new();
    assert!(session.sync(&host));
    session.arm_window_tick("preview-1");
    session.begin_window_tick("preview-1");
    session.note_window_extensions_in_flight("preview-1", 1);
    let live = published(&session, run.as_ref());
    assert_eq!(live["phase"], "computing");
    assert_eq!(live["cancellable"], true);

    session.cancel_preview_evaluation("preview-1");
    let cancelled = published(&session, run.as_ref());
    assert_eq!(cancelled["phase"], "cancelled", "the gesture's own verdict outranks every ledger");
    assert_eq!(cancelled["phaseLabel"]["en"], "Cancelled");
    assert_eq!(cancelled["phaseLabel"]["de"], "Abgebrochen");
    println!("[STATS] abort timeline: live={} cancelled={}", live["phase"], cancelled["phase"]);
    session.retire_cold();
    host.retire_cold();
}
