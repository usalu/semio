use super::*;
use semio_os_flow::neural::ColdRetire;
use semio_framework_test_fixtures::fixture_text;
use semio_framework_tool_run::ToolRunId;
static PREVIEW_EVAL_RUN_FIXTURE_JSON: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../🧪️tests/🧫️fixtures/⏯️preview-eval-run.json"));
fn fixture() -> serde_json::Value {
    let fixture: serde_json::Value = serde_json::from_str((*PREVIEW_EVAL_RUN_FIXTURE_JSON)).expect("the preview evaluation run fixture parses");
    assert_eq!(fixture["format"], "semio.generation3d.preview-eval-run");
    assert_eq!(fixture["version"], 1);
    fixture
}


/// 🎟️ The grant every window-tick transition of a direct session law runs under: no axis the preview owner needs is short.
fn tick_grant() -> semio_framework_value::RetainedCloneGrant {
    semio_framework_value::RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 26, maximum_capacity_bytes: 1 << 28, maximum_release_bytes: 1 << 28, maximum_depth: 4_096 }
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
                session.arm_window_tick(window, tick_grant()).expect("the window tick arms");
                session.begin_window_tick(window, tick_grant()).expect("the window tick begins");
                session.note_window_tick_outcome(window, false, tick_grant()).expect("the window tick outcome is noted");
            }
        }
        let run = run_view(&row["run"]);
        if let (Some(run), Some(settled)) = (run.as_ref(), row["run"].get("settledGeneration").and_then(serde_json::Value::as_u64)) {
            link.settled = Some((run.identity.id.run, settled as u32));
        }
        link.restart_owed = row["restartOwed"].as_bool().unwrap_or(false);
        let effects = preview_eval_run_effects(&mut session, &mut link, &windows, run.as_ref().map(ToolRunView::poll_view), row["servable"].as_bool().unwrap_or(true), 0);
        let actions: Vec<&str> = effects.iter().map(|effect| match effect {
            Effect::DispatchAction { action, .. } => action.as_str(),
            other => panic!("{id}: the run owes only dispatches, found {other:?}"),
        }).collect();
        println!("[STATS] runEffects {id}: {actions:?}");
        assert_eq!(actions, row["expected"].as_array().unwrap().iter().map(text).collect::<Vec<_>>(), "{id}");
        for effect in &effects {
            let Effect::DispatchAction { action, args: Some(args), .. } = effect else { continue };
            match action.as_str() {
                TOOL_RUN_START_ACTION_ID => assert_eq!(args.get(TOOL_RUN_ARG_TOOL_ID).and_then(semio_framework_value::DslValue::as_str), Some(PREVIEW_EVAL_TOOL_ID), "{id}: start names the tool"),
                _ => assert_eq!(args.get(TOOL_RUN_ARG_RUN_ID).and_then(semio_framework_value::DslValue::as_str), run.as_ref().map(|run| run.identity.id.run.to_string()).as_deref(), "{id}: finalize names the run"),
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
        for effect in preview_eval_run_effects(session, link, windows, view.as_ref().map(ToolRunView::poll_view), servable, 0) {
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

        let stage = |session: &mut FlowEvalSession, link: &mut PreviewEvalRunLink| {
            if settled_windows {
                for (window_id, _) in &windows {
                    session.arm_window_tick(window_id, tick_grant()).expect("the window tick arms");
                    session.begin_window_tick(window_id, tick_grant()).expect("the window tick begins");
                    session.note_window_tick_outcome(window_id, false, tick_grant()).expect("the window tick outcome is noted");
                }
            }
            link.settled = has_settled.then_some((1, 0));
            if live {
                link.port = Some(semio_plugin::ToolRunJobPort::default());
            }
        };

        let mut session = FlowEvalSession::new();
        let mut link = PreviewEvalRunLink::default();
        stage(&mut session, &mut link);
        if settled_windows {
            let complete = run_view(&serde_json::json!({ "run": 1, "generation": 0, "state": "complete" }));
            let mut asked: Vec<&str> = Vec::new();
            for _ in 0..polls {
                for effect in preview_eval_run_effects(&mut session, &mut link, &windows, complete.as_ref().map(ToolRunView::poll_view), true, 0) {
                    let Effect::DispatchAction { action, .. } = effect else { panic!("{id}: the run owes only dispatches") };
                    asked.push(if action == TOOL_RUN_FINALIZE_ACTION_ID { "toolRunFinalize" } else { "toolRunStart" });
                }
            }
            assert_eq!(asked, if windows.is_empty() { Vec::new() } else { vec!["toolRunFinalize"] }, "{id}: a settled complete run is finalized exactly once, however often the host polls it");
            link.settled = has_settled.then_some((1, 0));
            link.restart_owed = false;
            link.requested = None;
        }

        let mut emit = semio_plugin::Emit::<u8>::default();
        emit.artifact_mutations = vec![0_u8; mutations];
        let owes = owe_attached_previews_for_mutations(&mut session, &mut link, &windows, servable, &mut emit, tick_grant()).expect("the previews are owed");
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
            assert!(session.arm_window_tick(window_id, tick_grant()).expect("the window tick arms").0, "{id}: a dispatched hop arms its window");
            session.begin_window_tick(window_id, tick_grant()).expect("the window tick begins");
            session.note_window_tick_outcome(window_id, false, tick_grant()).expect("the window tick outcome is noted");
            assert!(hops.len() <= roster.len(), "{id}: one landed mutation may never arm a window twice");
        }
        assert_eq!(hops, expected["hops"].as_array().unwrap().iter().map(text).collect::<Vec<_>>(), "{id}: hop schedule");

        let mut spinning = FlowEvalSession::new();
        let mut spinning_link = PreviewEvalRunLink::default();
        stage(&mut spinning, &mut spinning_link);
        let mut spinning_emit = semio_plugin::Emit::<u8>::default();
        spinning_emit.artifact_mutations = vec![0_u8; mutations];
        owe_attached_previews_for_mutations(&mut spinning, &mut spinning_link, &windows, servable, &mut spinning_emit, tick_grant()).expect("the previews are owed");
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
    use semio_plugin::plugin_app_close_prelude::{ActionKind, CommandView, HistoryView};
    let command = |seq: u64, applied: bool, edit: Option<&str>| CommandView {
        seq,
        action_id: "apply".into(),
        label: semio_ui_locale::LocalizedLabel::native("Update Widget", "Widget aktualisieren"),
        kind: ActionKind::Mutation,
        timestamp: String::new(),
        edit_id: edit.map(str::to_string),
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
    preview_eval_run_effects(&mut session, &mut link, &windows, None, true, edited);
    session.note_window_tick_outcome("procedural-preview", false, tick_grant()).expect("the window tick outcome is noted");
    link.requested = None;
    assert!(!session.window_tick_owed("procedural-preview"), "a paid window owes nothing while the document stands still");
    let effects = preview_eval_run_effects(&mut session, &mut link, &windows, None, true, undone);
    assert!(session.window_tick_owed("procedural-preview"), "an undo owes the attached preview a fresh evaluation");
    assert_eq!(effects.len(), 1, "and asks for the run that pays it: {effects:?}");
    let mut gesturing = FlowEvalSession::new();
    let mut gesture_link = PreviewEvalRunLink::default();
    preview_eval_run_effects(&mut gesturing, &mut gesture_link, &windows, None, true, edited);
    let mut emit = semio_plugin::Emit::<u8>::default();
    emit.artifact_mutations = vec![0_u8];
    owe_attached_previews_for_mutations(&mut gesturing, &mut gesture_link, &windows, true, &mut emit, tick_grant()).expect("the previews are owed");
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
        link.port = Some(semio_plugin::ToolRunJobPort::default());
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
    }}


#[test]
fn geometry_inference_dependencies_include_sources_and_wiring_without_presentation() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/📐️geometry-dependencies.json")).unwrap();
    let mut snapshot = semio_framework_artifact_flow_flow::FlowHostSnapshot::default();
    let original = geometry_dependency_json(&snapshot, fixture["neuronId"].as_str().unwrap());
    assert_eq!(serde_json::from_str::<serde_json::Value>(&original).unwrap(), fixture["expected"]);
    snapshot.camera.x = 17.0;
    snapshot.widgets.reverse();
    snapshot.synapses.reverse();
    assert_eq!(original, geometry_dependency_json(&snapshot, "add"));
    if let semio_framework_artifact_flow_flow::Widget::InputSlider { value, .. } = snapshot.widgets.iter_mut().find(|widget| semio_s_artifact_procedural_3d_model::widget_id(widget) == "slider").unwrap() { *value = 7.0; }
    assert_ne!(original, geometry_dependency_json(&snapshot, "add"));
    snapshot.retire_cold();
}

#[test]
fn bounded_cancel_replies_poll_the_same_capability_without_rearming_evaluation() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🛑️preview-cancel.json")).unwrap();
    let mut session = FlowEvalSession::new();
    for row in fixture["boundedCancelReplies"].as_array().unwrap() {
        let payload = FlowTessellateCancelResolve { window_id: "preview".into(), window_kind_id: "world3d".into(), output: semio_framework_value::DslValue::Object(["ok","pending"].iter().filter_map(|key| row["reply"][*key].as_bool().map(|flag| ((*key).to_string(), semio_framework_value::DslValue::Bool(flag)))).collect()), fault: None, ok: row["ok"].as_bool().unwrap(), original_input: None, input_valid: true };
        let invocations = resolve_tessellate_cancel_for(&payload, &mut session, Ok("geometry".into()));
        assert_eq!(!invocations.is_empty(), row["poll"].as_bool().unwrap());
        for invocation in invocations { assert_eq!(invocation.capability, "evaluateCancel"); assert_eq!(invocation.response_action, "flowTessellateCancelResolve"); let request: serde_json::Value = serde_json::from_str(&invocation.request_json).unwrap(); assert_eq!(request["windowId"], "preview"); }
    }
    session.retire_cold();
}

#[test]
fn inspector_evaluation_wire_uses_actual_neuron_identity_and_context_authority(){
 let law=fixture();let row=&law["evaluationWire"];for resume in [false,true]{let pending=semio_framework_artifact_flow_flow::neural::PendingExtensionEval{neuron_id:text(&row["neuronId"]).into(),extension_id:text(&row["extensionId"]).into(),operator_id:text(&row["operatorId"]).into(),input_json:text(&row["inputJson"]).into(),node_hash:row["nodeHash"].as_u64().unwrap()};let invocation=evaluate_invocation(pending,resume,text(&row["dependencyJson"]).into(),text(&row["windowId"]),text(&row["windowKindId"]));let actual:serde_json::Value=serde_json::from_str(&invocation.request_json).unwrap();let expected=serde_json::json!({"neuronId":row["neuronId"],"operatorId":row["operatorId"],"inputJson":if resume{""}else{text(&row["inputJson"])},"dependencyJson":if resume{""}else{text(&row["dependencyJson"])},"operatorVersion":format!("registry:{};geometry:1;policy:1",semio_os_flow::flow_extension_registry_generation()),"nodeHash":row["nodeHash"],"resume":resume,"windowId":row["windowId"],"windowKindId":row["windowKindId"],"extensionId":row["extensionId"]});assert_eq!(actual,expected);assert_eq!(actual.as_object().unwrap().len(),row["requiredKeys"].as_array().unwrap().len());for field in row["authorityFieldsAbsent"].as_array().unwrap(){assert!(actual.get(field.as_str().unwrap()).is_none());}assert_eq!(invocation.extension_id,row["extensionId"]);assert_eq!(invocation.capability,"evaluate");assert_eq!(invocation.response_action,"flowEvalResolve");drop(invocation);}
}

#[test]
fn inspector_close_original_context_reports_one_debit_and_whole_paired_failure(){
 use semio_framework_job::{OperationId,Generation,OriginalJobFixtureCaller};use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
 let law=fixture();let row=&law["originalCloseReceipt"];let source=RetainedCloneGrant{maximum_items:16,maximum_copy_bytes:4096,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:16};let mut caller=OriginalJobFixtureCaller::admit(OperationId(row["operation"].as_u64().unwrap()),Generation(row["generation"].as_u64().unwrap()),source,||Some(1),u64::MAX,64);for failed in [false,true]{let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:row["copy"].as_u64().unwrap()as usize,maximum_depth:1,..Default::default()};let progress=RetainedCloneProgress{copied_items:row["progressItems"].as_u64().unwrap()as usize,copied_bytes:row[if failed{"overdrawCopy"}else{"copy"}].as_u64().unwrap()as usize,..Default::default()};let step=if failed{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original preview close semantic cause").with_retained_progress(progress))}else{Ok(semio_framework_value::RetainedCloneStep::Progress(progress))};let mut seen=RetainedCloneProgress::default();let result=caller.turn(grant,|cx|{let before=cx.retained_progress();let result=semio_framework_job::close_original_retained_step(cx,semio_framework_value::RetirementDemand{copy_bytes:grant.maximum_copy_bytes,depth:grant.maximum_depth,..Default::default()},|_|semio_framework_value::RetainedWorkOutcome{result:step,work:row["spent"].as_u64().unwrap()});let after=cx.retained_progress();seen=RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes};result});assert_eq!(seen,progress);if failed{let failure=result.unwrap_err();assert_eq!(failure.primary_value().unwrap().kind,ValueRefusalKind::InvariantViolated);assert!(failure.receipt_refusal.is_some());assert_eq!(failure.primary.retained_progress(),progress);drop(failure);}else{assert_eq!(result.unwrap().progress(),progress);}}caller.retire(source);
}


#[test]
fn hovered_graph_connections_identify_their_preview_sources() {
    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../../✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🧪️tests/🧫️fixtures/🔬️unit/🔣️.json")).unwrap();
    let host = semio_os_flow::FlowHost::parse_host_snapshot_json(&law["hostSnapshot"].to_string()).unwrap();
    for case in law["expected"]["hoverSources"].as_array().unwrap() {
        let hovered = std::collections::BTreeSet::from([case["target"].as_str().unwrap().to_owned()]);
        let sources = preview_hover_sources(&host, &hovered);
        assert_eq!(serde_json::json!(sources), case["sources"]);
    }
    host.retire_cold();
}

#[test]
fn wireframe_synthesizes_missing_triangle_edges() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🪢️wireframe.json")).unwrap();
    let values = |key: &str| fixture[key].as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as f32).collect();
    let indices = fixture["indices"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u32).collect();
    let mesh = MeshData { positions: values("positions"), indices, paint_texture_base64: Some("surface".into()), ..Default::default() };
    let shaded = apply_show_mode_mesh(mesh.clone(), "shaded+edges");
    assert_eq!(shaded.edge_positions, values("edges"));
    assert_eq!(shaded.positions, values("positions"));
    assert!(shaded.paint_texture_base64.is_some());
    let projected_points = apply_show_mode_mesh(mesh.clone(), "points");
    assert_eq!(projected_points.positions, values("positions"));
    assert!(projected_points.paint_texture_base64.is_none());
    let result = apply_show_mode_mesh(mesh, "wireframe");
    assert_eq!(result.edge_positions, values("edges"));
    assert!(result.paint_texture_base64.is_none());
    assert!(mesh_has_preview_geometry(&result));
    let points = apply_show_mode_mesh(MeshData { edge_positions: values("edges"), ..Default::default() }, "points");
    use semio_framework_value::{DslValue,FromValue,ToValue};
    let refs: std::collections::BTreeMap<String,Vec<String>> = serde_json::from_value(fixture["componentRefs"].clone()).unwrap();
    let component_mesh = MeshData{
        positions:values("positions"),indices:vec![0,1,2,0,2,3],edge_positions:values("edges"),
        face_ids:vec![0;2],edge_ids:vec![0;5],vertex_ids:vec![0;4],
        component_references:FromValue::from_value(DslValue::Object(refs.into_iter().map(|(name,labels)|(name,labels.to_value())).collect())).unwrap(),
        ..Default::default()
    };
    for (mode,domain) in [("wireframe","edge"),("points","vertex")] {
        let shown=apply_show_mode_mesh(component_mesh.clone(),mode);
        assert_eq!(shown.component_references.len(),1);
        assert!(shown.component_references.get(domain).is_some());
        assert!(shown.face_ids.is_empty());
        assert_eq!(shown.edge_ids.is_empty(),mode=="points");
        assert_eq!(shown.vertex_ids.is_empty(),mode=="wireframe");
    }
    assert_eq!(points.positions, values("edges"));
    assert!(mesh_has_preview_geometry(&points));
    let point = &fixture["point"];
    let positions: Vec<f32> = serde_json::from_value(point["positions"].clone()).unwrap();
    let vertex_ids: Vec<u32> = serde_json::from_value(point["vertexIds"].clone()).unwrap();
    let labels: Vec<String> = serde_json::from_value(point["componentRefs"]["vertex"].clone()).unwrap();
    let references = FromValue::from_value(DslValue::Object([("vertex".into(), labels.to_value())].into_iter().collect())).unwrap();
    let isolated = apply_show_mode_mesh(MeshData {
        positions: positions.clone(), vertex_ids: vertex_ids.clone(), component_references: references, ..Default::default()
    }, "wireframe");
    assert!(mesh_has_preview_geometry(&isolated));
    assert_eq!(isolated.positions, positions);
    assert_eq!(isolated.vertex_ids, vertex_ids);
    assert_eq!(isolated.component_references.get("vertex"), Some(labels.as_slice()));
}


#[test]
fn nested_outputs_and_sink_inputs_keep_their_channel_address() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/channels.json")).unwrap();
    let evaluation = semio_framework_pack_json::parse(&fixture["evaluation"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for (widget, expected) in fixture["expected"].as_object().unwrap() {
        let actual: Vec<_> = preview_channel_items_for_widget(&evaluation, widget).into_iter().map(|item| {
            let kind = match item.inline { Some(PreviewInlineGeometry::Vector { .. }) => "vector", _ => &item.handle };
            serde_json::json!([item.channel, item.index, kind])
        }).collect();
        assert_eq!(serde_json::json!(actual), *expected, "{widget}");
    }
}

#[test]
fn spatial_vector_arrows_have_a_visible_tip_for_every_axis() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/channels.json")).unwrap();
    for row in fixture["arrows"].as_array().unwrap() {
        let direction: [f64; 3] = serde_json::from_value(row["direction"].clone()).unwrap();
        let [x, y, z] = direction;
        let mesh = vector_marker_mesh(x, y, z);
        let length = row["length"].as_f64().unwrap();
        if length == 0.0 {
            assert_eq!(mesh.positions, vec![0.0; 3]);
            assert_eq!(mesh.edge_positions.len(), 18);
            continue;
        }
        let edges = &mesh.edge_positions;
        assert_eq!(edges.len(), 30);
        assert_eq!(&edges[3..6], &direction.map(|v| v as f32));
        for fin in edges[6..].chunks_exact(6) {
            assert_eq!(&fin[..3], &edges[3..6]);
            let axial = (0..3).map(|i| fin[i + 3] as f64 * direction[i] / length).sum::<f64>();
            assert!((axial / length - 0.8).abs() < 1e-5);
        }
    }
}

#[test]
fn scalar_readout_rows_preserve_graph_targets() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔢️value-readout.json")).unwrap();
    let rows = preview_value_rows(&fixture["eval"].to_string());
    let expected: Vec<(String, String)> = serde_json::from_value(fixture["rows"].clone()).unwrap();
    for (id, _, target) in &rows {
        assert_eq!(serde_json::from_str::<serde_json::Value>(target).unwrap(), serde_json::json!([{"granularity": "handle", "id": id}]));
    }
    assert_eq!(rows.into_iter().map(|(id, value, _)| (id, value)).collect::<Vec<_>>(), expected);
}

#[test]
fn preview_mesh_sources_stay_channel_local_until_the_live_pack_arrives() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧭️mesh-provenance.json")).unwrap();
    let live = fixture["live"].as_str().unwrap();
    let previous = fixture["previous"].as_object().unwrap().iter().map(|(channel, handle)|
        (channel.clone(), serde_json::json!({"handle":handle}))).collect::<serde_json::Map<_, _>>();
    let previous = serde_json::json!({"widget":{"out":previous}});
    for row in fixture["cases"].as_array().unwrap() {
        let mut session = FlowEvalSession::new();
        if row["advanced"].as_bool() == Some(true) {
            session.previous_converged_eval_json = previous.to_string();
            session.converged_eval_json = serde_json::json!({"widget":{"out":{
                "a":{"handle":live},"b":{"handle":live}
            }}}).to_string();
        } else {
            session.converged_eval_json = previous.to_string();
        }
        for handle in row["available"].as_array().unwrap() {
            let handle = handle.as_str().unwrap();
            let x = fixture["packs"][handle].as_f64().unwrap() as f32;
            let mesh = MeshData { positions: vec![x,0.0,0.0,x,1.0,0.0,x,0.0,1.0], indices: vec![0,1,2], ..Default::default() };
            session.preview_mesh_by_handle.insert(handle.to_owned(), semio_os_flow::FlowPreviewMesh { tolerance: 0.05, mesh });
        }
        if row["diagnosed"].as_bool().unwrap() {
            session.preview_diagnostics_by_handle.insert(live.to_owned(), "[]".to_owned());
        }
        let item = PreviewChannelItem { channel: row["channel"].as_str().unwrap().into(), index: 0, handle: live.into(), inline: None };
        let lookup = PreviewMeshLookup::new(&session);
        let resolved = lookup.resolve("widget", &item);
        assert_eq!(resolved.as_ref().map(|(handle, _)| handle.as_ref()), row["expected"].as_str());
        assert_eq!(resolved.as_ref().map(|(_, mesh)| f64::from(mesh.positions[0])), row["position"].as_f64());
        assert_eq!(resolved.as_ref().is_some_and(|(handle, _)| handle.as_ref() == live), row["component"].as_bool().unwrap());
        session.retire_cold();
    }
}

#[test]
fn stale_preview_detail_closes_the_reply_and_preserves_the_active_request() {
    use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
    use semio_framework_value::{DslValue, ToValue};
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎚️preview-detail-ownership.json")).unwrap();
    let handle = law["handle"].as_str().unwrap();
    let tolerance = law["tolerance"].as_f64().unwrap();
    let resident_tolerance = law["cacheTolerance"].as_f64().unwrap();
    let new_tolerance = law["replacementTolerance"].as_f64().unwrap();
    let node_hash = law["staleHash"].as_u64().unwrap();
    let live_hash = law["liveHash"].as_u64().unwrap();
    let mut session = FlowEvalSession::new();
    let resident = MeshData { positions: vec![2.0, 0.0, 0.0], ..Default::default() };
    session.preview_mesh_by_handle.insert(handle.into(), semio_os_flow::FlowPreviewMesh { tolerance: resident_tolerance, mesh: resident });
    assert!(session.note_pending_tessellate(live_hash, handle.into(), new_tolerance));

    let mut payload = FlowTessellateResolve {
        window_id: "preview".into(), window_kind_id: "procedural-preview".into(),
        node_hash, handle: handle.into(), tolerance, fault: None, original_input: None, input_valid: true,
        output: DslValue::object([
            ("done".into(), DslValue::Bool(true)),
            ("cancellable".into(), DslValue::Bool(false)),
            ("phase".into(), DslValue::String("complete".into())),
            ("unitsDone".into(), DslValue::uint(1)),
            ("unitsTotal".into(), DslValue::uint(1)),
            ("windowId".into(), DslValue::String("preview".into())),
            ("windowKindId".into(), DslValue::String("procedural-preview".into())),
            ("nodeHash".into(), DslValue::uint(node_hash)),
            ("handle".into(), DslValue::String(handle.into())),
            ("tolerance".into(), DslValue::float(tolerance)),
            ("mesh".into(), MeshData { positions: vec![1.0, 0.0, 0.0], ..Default::default() }.to_value()),
        ]),
    };
    let stock = tick_grant();
    let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, ..stock };
    let mut caller = OriginalJobFixtureCaller::admit(
        OperationId(law["caller"]["operation"].as_u64().unwrap()), Generation(law["caller"]["generation"].as_u64().unwrap()),
        stock, || Some(1), u64::MAX, 4096,
    );
    let mut receiver = PreviewTessellateReceive::new();
    assert!(caller.turn(grant, |cx| receiver.receive(&mut payload, cx)).unwrap());
    for _ in 0..4096 {
        if caller.turn(grant, |cx| receiver.step(&payload, &mut session, cx)).unwrap() { break; }
    }
    assert!(receiver.is_folded() && receiver.terminal_is_empty());
    let expected: Vec<u64> = serde_json::from_value(law["remainingPending"].clone()).unwrap();
    assert_eq!(session.pending_tessellate_by_hash.keys().copied().collect::<Vec<_>>(), expected);
    let mesh = session.preview_mesh_for_tolerance(handle, resident_tolerance).unwrap();
    assert_eq!(mesh.positions, [2.0, 0.0, 0.0]);
    let host = semio_framework_artifact_flow_flow::FlowHostSnapshot::default();
    assert!(preview_mesh_retention_handles(&semio_framework_pack_json::Value::Null, &host, &session).contains(handle));
    session.retire_cold();
    caller.retire(stock);
}

#[test]
fn preview_tessellation_fault_settles_its_window_and_blocks_only_that_detail() {
    use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
    use semio_framework_value::DslValue;
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎚️preview-detail-ownership.json")).unwrap();
    let row = &law["fault"];
    let handle = law["handle"].as_str().unwrap();
    let window = row["window"].as_str().unwrap();
    let tolerance = row["tolerance"].as_f64().unwrap();
    let node_hash = semio_os_flow::preview_tessellate_node_hash(handle, tolerance.to_bits());
    let stock = tick_grant();
    let grant = semio_framework_value::RetainedCloneGrant { maximum_items: 1, ..stock };
    let mut session = FlowEvalSession::new();
    session.arm_window_tick(window, stock).unwrap();
    session.begin_window_tick(window, stock).unwrap();
    session.note_window_tick_outcome(window, true, stock).unwrap();
    session.note_window_extensions_in_flight(window, 1, stock).unwrap();
    session.note_preview_tessellate_tolerance(tolerance);
    assert!(session.note_pending_tessellate(node_hash, handle.into(), tolerance));
    let mut payload = FlowTessellateResolve {
        window_id: window.into(), window_kind_id: "procedural-preview".into(),
        node_hash, handle: handle.into(), tolerance, output: DslValue::Null, input_valid: true,
        fault: Some(DslValue::object([("message".into(), DslValue::String("surface failure".into()))])),
        original_input: None,
    };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(709), Generation(2), stock, || Some(1), u64::MAX, 4096);
    let mut receiver = PreviewTessellateReceive::new();
    assert!(caller.turn(grant, |cx| receiver.receive(&mut payload, cx)).unwrap());
    for _ in 0..4096 {
        if caller.turn(grant, |cx| receiver.step(&payload, &mut session, cx)).unwrap() { break; }
    }
    assert!(receiver.is_folded() && receiver.terminal_is_empty());
    assert_eq!(session.window_extensions_in_flight(window), 0);
    assert_eq!(session.preview_tessellate_status().phase, semio_os_flow::PreviewTessellatePhase::Failed);
    assert_eq!(u64::from(session.preview_tessellate_status().diagnostics), row["diagnostics"].as_u64().unwrap());
    assert!(!session.note_pending_tessellate(node_hash, handle.into(), tolerance));
    let next = row["nextTolerance"].as_f64().unwrap();
    session.note_preview_tessellate_tolerance(next);
    assert_eq!(session.preview_tessellate_status().diagnostics, 0);
    assert!(session.note_pending_tessellate(semio_os_flow::preview_tessellate_node_hash(handle, next.to_bits()), handle.into(), next));
    session.retire_cold();
    caller.retire(stock);
}

#[test]
fn plane_channel_preview_has_an_on_plane_outline_and_anchored_normal() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧭️plane-marker.json")).unwrap();
    let half_extent = fixture["halfExtent"].as_f64().unwrap();
    let normal_length = fixture["normalLength"].as_f64().unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let origin: [f64; 3] = serde_json::from_value(row["origin"].clone()).unwrap();
        let normal: [f64; 3] = serde_json::from_value(row["normal"].clone()).unwrap();
        let [x,y,z] = origin;
        let [nx,ny,nz] = normal;
        let text = serde_json::json!({"widget":{"out":{"plane":{
            "$schema":"plane","origin":{"$schema":"point","x":x,"y":y,"z":z},
            "normal":{"$schema":"vector","x":nx,"y":ny,"z":nz}
        }}}}).to_string();
        let eval = semio_framework_pack_json::parse(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let items = preview_channel_items_for_widget(&eval, "widget");
        assert_eq!(items.len(), 1);
        let item = &items[0];
        assert_eq!(item.channel, "plane");
        assert_eq!(item.inline, Some(PreviewInlineGeometry::Plane { origin, normal }));
        let mesh = inline_preview_mesh(item.inline.as_ref().unwrap());
        assert_eq!(mesh.is_some(), row["valid"].as_bool().unwrap());
        let Some(mesh) = mesh else { continue };
        assert_eq!(preview_mesh_role(item.inline.as_ref(), &mesh), "wire");
        let norm = nx.hypot(ny).hypot(nz);
        let offsets = |point: &[f32]| std::array::from_fn::<_, 3, _>(|axis| f64::from(point[axis]) - origin[axis]);
        for segment in mesh.edge_positions[..24].chunks_exact(6) {
            let start = offsets(&segment[..3]);
            let end = offsets(&segment[3..]);
            let distance = |offset: [f64; 3]| (0..3).map(|axis| offset[axis] * normal[axis] / norm).sum::<f64>();
            assert!(distance(start).abs() < 1e-6);
            assert!(distance(end).abs() < 1e-6);
            let edge_length: f64 = (0..3).map(|axis| (start[axis] - end[axis]).powi(2)).sum::<f64>().sqrt();
            assert!((edge_length - 2.0 * half_extent).abs() < 1e-6);
        }
        let base = offsets(&mesh.edge_positions[24..27]);
        let tip = offsets(&mesh.edge_positions[27..30]);
        assert!(base.into_iter().all(|offset| offset.abs() < 1e-6));
        for axis in 0..3 { assert!((tip[axis] - normal_length * normal[axis] / norm).abs() < 1e-6); }
        assert!(mesh_has_preview_geometry(&apply_show_mode_mesh(mesh, "points")));
    }
}
