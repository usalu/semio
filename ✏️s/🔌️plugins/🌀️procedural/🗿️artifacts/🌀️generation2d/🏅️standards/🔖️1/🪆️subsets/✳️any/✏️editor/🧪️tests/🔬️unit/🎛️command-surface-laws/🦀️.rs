/// 🎛️ Command surface laws of the generation2d editor routes: the retained route dispositions, the contributions ceiling, the run-start and preview-eval laws, the demo run job and the command ids with their wire forms.

//#region 🔖️CommandSurface
#[test]
fn retained_route_dispositions_are_exact_and_exhaustive() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    assert_eq!(GENERATION2D_BOUNDED_TOOL_IDS.len(), 22);
    assert_eq!(GENERATION2D_CONTRIBUTIONS_TOOL_IDS.len(), 1);
    assert_eq!(<Generation2dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 23, "both factories' proofs, aggregated");
    assert_eq!(Generation2dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.len(), 22);
    assert_eq!(Generation2dContributionsJobFactory::PUBLICATION_CONTRACTS.len(), 1);
    assert!(GENERATION2D_CONTRIBUTIONS_TOOL_IDS.iter().all(|tool_id| !GENERATION2D_BOUNDED_TOOL_IDS.contains(tool_id)), "a tool id may be owned by exactly one factory");
    assert!(GENERATION2D_CONTRIBUTIONS_RAW_BYTES > GENERATION2D_RETAINED_RAW_BYTES, "the contributions route exists precisely because the gesture quota cannot carry it");
    assert_eq!(generation2d_bounded_contract().shape, ToolExecutionShape::BoundedFirstStep);
    assert_eq!(generation2d_bounded_contract().cancellation, ToolCancellationPolicy::PerOperation);
    assert!(GENERATION2D_BOUNDED_TOOL_IDS.iter().all(|tool_id| Generation2dBoundedCommandJobFactory::PUBLICATION_CONTRACTS.iter().any(|contract| contract.tool_id == *tool_id)));
    for migrated in ["nodeGraphEdit", "moveMediaNode", "addWidget", "removeWidget", "connectMediaPorts", "reorganize", "setEvalOutputs"] {
        assert!(GENERATION2D_BOUNDED_TOOL_IDS.contains(&migrated), "{migrated} must own an exact retained reducer route");
        assert!(<Generation2dPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().iter().any(|proof| proof.tool_id() == migrated), "{migrated} must carry its bounded first-step proof row");
    }
    assert!(
        every_command().iter().all(|command| GENERATION2D_BOUNDED_TOOL_IDS.contains(&command.command_id()) || GENERATION2D_CONTRIBUTIONS_TOOL_IDS.contains(&command.command_id())),
        "every declared command routes through one of the two retained ladders"
    );
}

/// 📏️ The raw wire the host's `setContributions` push really carried into this app on 2026-09-19,
/// measured by the refusal it drew (`tool factory 's.procedural.generation2d@1/*#editor/setContributions'
/// rejected 62090 raw bytes before decoding; maximum is 12288`, ticket
/// 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP) — the size a declared ceiling has to admit.
const GENERATION2D_MEASURED_CONTRIBUTIONS_WIRE_BYTES: usize = 62_090;

/// ⚖️ LAW: the contributions route's declared wire ceiling ADMITS the pack the host really pushes —
/// whole, as page 0 of 1 — and is the transport's own assembled-command authority, not the JSON entry
/// point's 4 KiB string page. The factory's pre-decode gate and the served route read the same
/// constant, so admitting it here is admitting it at boot.
#[test]
fn contributions_route_declares_a_reachable_wire_ceiling() {
    assert_eq!(GENERATION2D_CONTRIBUTIONS_RAW_BYTES, semio_framework::kernel::COMMAND_MAXIMUM_BYTES, "the contributions route is bound by what the paged command ingress can assemble, not by the JSON entry point's string page");
    assert!(GENERATION2D_CONTRIBUTIONS_RAW_BYTES >= GENERATION2D_MEASURED_CONTRIBUTIONS_WIRE_BYTES, "the measured {GENERATION2D_MEASURED_CONTRIBUTIONS_WIRE_BYTES}-byte push must be admitted, the contract declares {GENERATION2D_CONTRIBUTIONS_RAW_BYTES}");
    let pack: String = std::iter::repeat_n('x', GENERATION2D_MEASURED_CONTRIBUTIONS_WIRE_BYTES).collect();
    let wire = semio_framework_pack_json::to_json_string(&("setContributions", Some(semio_framework_value::DslValue::object([
        ("json".to_string(), semio_framework_value::DslValue::String(pack)),
        ("page".to_string(), semio_framework_value::DslValue::uint(0)),
        ("pageCount".to_string(), semio_framework_value::DslValue::uint(1)),
    ]))));
    assert!(wire.len() <= GENERATION2D_CONTRIBUTIONS_RAW_BYTES, "a whole page-0-of-1 push encodes to {} bytes but the contract declares {GENERATION2D_CONTRIBUTIONS_RAW_BYTES}", wire.len());
    assert_eq!(generation2d_contributions_contract().max_raw_wire_bytes, GENERATION2D_CONTRIBUTIONS_RAW_BYTES);
}

async fn drive_preview_operation(app: &mut semio_framework_plugin::VcsArtifactApp<EditorApp<Generation2dPlayApp>>) -> Result<(u64, u64, u64), String> {
    use semio_framework_plugin::app::TypedOperationResultLane;
    let receipt = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, 1, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.map_err(|error| format!("{error:?}"))?;
    let count = |wanted: TypedOperationResultLane| receipt.lanes.iter().filter(|lane| **lane == wanted).count() as u64;
    Ok((count(TypedOperationResultLane::Artifact), count(TypedOperationResultLane::Config), count(TypedOperationResultLane::Transient)))
}

/// ⚖️ LAW: a generation command authors its document and selection lanes and NOTHING else — no tick loop
/// and no transient of its own. The generate preview's evaluation is the `previewEval` run's, so the
/// command carries exactly one run start on its own emit for the attached generate preview, and the run
/// publishes the evaluation into the ONE app transient every generate preview window renders.
#[semio_framework_async_macros::async_test]
async fn a_generation_command_carries_the_run_start_and_the_run_publishes_the_shared_generate_preview() {
    let mut app = app_with_registry().await;
    let result: Result<(), String> = async {
        let (generations_view, previews) = context::generate_shell_views();
        let before_generations = snapshot_read(&app).generation.as_state().generations.len();
        let action_meta = semio_framework_plugin::ActionMeta { view_state: Some(generations_view.clone()), ..semio_framework_plugin::artifact_app_laws::meta("preview-owner") };
        for widget_id in ["rect", "outline"] {
            app.dispatch_typed(Generation2dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: widget_id.into() }), &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.map_err(|error| format!("{error:?}"))?;
            semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, 1, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.map_err(|error| format!("{error:?}"))?;
        }
        let settled = context::drive_preview_run(&mut app, &action_meta, &[]).await;
        let before_transient = app.ephemeral_snapshot().await.transient_generation;
        app.dispatch_typed(Generation2dCommand::AddGeneration(add_generation::AddGeneration {}), &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.map_err(|error| format!("{error:?}"))?;
        let receipt = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, 1, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.map_err(|error| format!("{error:?}"))?;
        let count = |wanted: semio_framework_plugin::app::TypedOperationResultLane| receipt.lanes.iter().filter(|lane| **lane == wanted).count();
        let lanes = (count(semio_framework_plugin::app::TypedOperationResultLane::Artifact), count(semio_framework_plugin::app::TypedOperationResultLane::Config), count(semio_framework_plugin::app::TypedOperationResultLane::Transient));
        if lanes != (1, 1, 0) {
            return Err(format!("addGeneration must publish its artifact and selection lanes only, got {lanes:?}"));
        }
        let carried = context::run_actions(&receipt.effects);
        if carried != vec![semio_framework_tool_run::TOOL_RUN_START_ACTION_ID.to_string()] {
            return Err(format!("addGeneration must carry exactly one run start, got {carried:?} (the boot run settled as {settled:?})"));
        }
        if app.ephemeral_snapshot().await.transient_generation != before_transient {
            return Err("the gesture itself must publish no transient".into());
        }
        let run = context::drive_preview_run(&mut app, &action_meta, &receipt.effects).await;
        if !run.hop_windows.iter().any(|window| window == "preview-a" || window == "preview-b") {
            return Err(format!("the run must evaluate the generate previews: {run:?}"));
        }
        if run.state.as_deref() != Some("finalized") && run.state.as_deref() != Some("complete") {
            return Err(format!("the run must settle: {run:?}"));
        }
        if app.ephemeral_snapshot().await.transient_generation == before_transient {
            return Err("the run must publish the generate preview into the app transient".into());
        }
        if snapshot_read(&app).generation.as_state().generations.len() != before_generations + 1 {
            return Err("addGeneration did not preserve its document behavior".into());
        }
        let mut rendered = Vec::new();
        for context_view in &previews {
            let tree = app.render(generate_preview::GENERATION2D_PLAY_BODY_GENERATE_PREVIEW, None, context_view).await.map_err(|error| format!("{error:?}"))?;
            rendered.push(semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?);
        }
        if rendered[0] != rendered[1] {
            return Err("generation windows did not consume the same app-transient preview".into());
        }
        if rendered[0].contains(generation2d_labels(&previews[0]).preview_hint.as_str()) {
            return Err(format!("the generate preview still shows its hint after the run evaluated: {}", rendered[0]));
        }
        if include_str!("../../../🎚️config/🧬️schema/🔣️.json").contains("generationPreviewText") {
            return Err("config schema still owns computed preview output".into());
        }
        Ok(())
    }
    .await;
    close(app);
    result.expect("Generation2d generate preview run");
}

/// ⚖️ LAW (brief: start → complete → finalize, abort leaves the document byte-identical): the read-only
/// `previewEval` run started by the edit preview's first poll settles, is finalized (a read-only run is
/// finalized by the framework as soon as it completes) into NO document edit,
/// and an abort raised while it is running leaves the document pack, the config pack and the history
/// exactly as they were.
#[semio_framework_async_macros::async_test]
async fn the_preview_eval_run_finalizes_nothing_and_an_abort_leaves_the_document_byte_identical() {
    let mut app = app_with_registry().await;
    let result: Result<(), String> = async {
        let (flow_view, _) = context::edit_shell_views();
        let action_meta = semio_framework_plugin::ActionMeta { view_state: Some(flow_view.clone()), ..semio_framework_plugin::artifact_app_laws::meta("local") };
        for widget_id in ["rect", "outline"] {
            app.dispatch_typed(Generation2dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: widget_id.into() }), &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.map_err(|error| format!("{error:?}"))?;
            semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, 1, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.map_err(|error| format!("{error:?}"))?;
        }
        let document = app.document_pack().await.map_err(|error| format!("{error:?}"))?;
        let config = app.config_pack().await.map_err(|error| format!("{error:?}"))?;
        let history = app.artifact_generation_now();
        let run = context::drive_preview_run(&mut app, &action_meta, &[]).await;
        if run.hops == 0 || run.state.as_deref() != Some("finalized") {
            return Err(format!("the run must hop, complete and finalize: {run:?}"));
        }
        let after = (app.document_pack().await.map_err(|error| format!("{error:?}"))?, app.config_pack().await.map_err(|error| format!("{error:?}"))?, app.artifact_generation_now());
        if (after.0.pack.as_slice(), after.1.pack.as_slice(), after.2) != (document.pack.as_slice(), config.pack.as_slice(), history) {
            return Err("a finalized read-only run must author no document, config or history entry".into());
        }
        app.dispatch_typed(Generation2dCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), neuron_kind: None, format: None, action: None, x: None, y: None }), &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.map_err(|error| format!("{error:?}"))?;
        let receipt = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app, 1, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.map_err(|error| format!("{error:?}"))?;
        let edited = (app.document_pack().await.map_err(|error| format!("{error:?}"))?, app.artifact_generation_now());
        for effect in &receipt.effects {
            if let Effect::DispatchAction { action, args, .. } = effect {
                app.handle_action(action, args.as_ref(), &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.map_err(|error| format!("{action}: {error:?}"))?;
            }
        }
        let view = app.tool_run_ledger().view().ok_or("the gesture's run start must open a run")?;
        app.handle_action(
            semio_framework_tool_run::TOOL_RUN_ABORT_ACTION_ID,
            Some(&semio_framework_value::DslValue::object([
                (semio_framework_tool_run::TOOL_RUN_ARG_RUN_ID.to_string(), semio_framework_value::DslValue::String(view.identity.id.run.to_string())),
                (semio_framework_tool_run::TOOL_RUN_ARG_GENERATION.to_string(), semio_framework_value::DslValue::uint(u64::from(view.identity.generation))),
            ])),
            &action_meta,
        )
        .await
        .map_err(|error| format!("toolRunAbort: {error:?}"))?;
        let aborted = context::drive_preview_run(&mut app, &action_meta, &[]).await;
        if aborted.state.as_deref() != Some("aborted") {
            return Err(format!("the abort must end the run aborted and the poll must not restart it: {aborted:?}"));
        }
        let after_abort = (app.document_pack().await.map_err(|error| format!("{error:?}"))?, app.artifact_generation_now());
        if (after_abort.0.pack.as_slice(), after_abort.0.spr.as_slice(), after_abort.1) != (edited.0.pack.as_slice(), edited.0.spr.as_slice(), edited.1) {
            return Err("an aborted read-only run must leave the document byte-identical".into());
        }
        app.handle_action(semio_framework_tool_run::TOOL_RUN_DISMISS_ACTION_ID, Some(&semio_framework_value::DslValue::object([(semio_framework_tool_run::TOOL_RUN_ARG_RUN_ID.to_string(), semio_framework_value::DslValue::String(view.identity.id.run.to_string()))])), &action_meta).await.map_err(|error| format!("toolRunDismiss: {error:?}"))?;
        Ok(())
    }
    .await;
    close(app);
    result.expect("Generation2d previewEval lifecycle");
}

/// 🧳️ A standalone instance owner handle with the demo's edit preview attached through the real
/// `pending_effects` law, retired through its own close ladder by [`retire_owner`].
fn demo_owner(window: (&str, &'static str)) -> semio_framework_plugin::ArtifactInstanceOperationOwnerHandle {
    let handle = semio_framework_plugin::ArtifactInstanceOperationOwnerHandle::new(<Generation2dPlayApp as ArtifactEditor>::build_instance_operation_owner());
    handle
        .with_mut::<Generation2dInstanceOperationOwner, _>(|owner| {
            let (sessions, link) = owner.parts()?;
            let windows = [(window.0, window.1, PreviewEvalTarget::Document)];
            semio_s_artifact_procedural_2d_preview_eval::preview_eval_run_effects(sessions, link, &windows, &Default::default(), None, true);
            Ok(())
        })
        .expect("the demo owner attaches its preview");
    handle
}

fn retire_owner(handle: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) {
    semio_s_artifact_procedural_2d_testkit::retirement_driver::close_instance_owner::<Generation2dInstanceOperationOwner>(handle, "the demo owner");
}

/// ⚖️ LAW (`⏯️preview-eval-run.json` `demoRun`): the run job over the bundled demo — every hop performed by
/// the real `flowEvalTick` hop core, nothing contributed — traces every node it observes with its verdict,
/// counts one hop per unit of fuel, settles with the declared settle step, and no single `drive_step`
/// overruns the interactive ceiling.
#[test]
fn the_demo_run_job_traces_every_node_settles_and_stays_under_the_interactive_ceiling() {
    use semio_s_artifact_procedural_2d_testkit::retirement_driver::close_job;
    use semio_s_artifact_procedural_2d_testkit::retirement_driver::step_once;
    use semio_s_artifact_procedural_2d_testkit::retirement_driver::Stepped;
    use semio_framework_job::{Generation, OperationId};
    use semio_framework_tool_run::{ToolRunId, ToolRunIdentity, ToolRunTick, ToolRunTraceOp};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🧪️tests/🧫️fixtures/⏯️preview-eval-run.json")).expect("run fixture");
    let row = &fixture["demoRun"];
    let expected = &row["expected"];
    let (window_id, kind) = (row["window"]["id"].as_str().unwrap(), edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW);
    assert_eq!(row["window"]["kind"].as_str(), Some(kind));
    let handle = demo_owner((window_id, kind));
    let port = semio_framework_plugin::ToolRunJobPort::default();
    let identity = ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, [0; 32]);
    let mut job = semio_s_artifact_procedural_2d_preview_eval::PreviewEvalRunJob::<Generation2dInstanceOperationOwner>::new(handle.clone(), port.clone(), identity).expect("the run job attaches");
    let snapshot = semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshotRead::new(semio_s_artifact_procedural_2d_io_text::snapshot::default_snapshot());
    let history = context::empty_history_view();
    let config = Generation2dConfig::default();
    let tick = Generation2dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick { window_id: window_id.into(), window_kind_id: kind.into() });
    let cancel = semio_framework_job::root_cancel_token();
    let (mut sequence, mut worst_us, mut hops_performed) = (0_u64, 0_u64, 0_usize);
    let mut ticks: Vec<ToolRunTick> = Vec::new();
    let settled = loop {
        let started = semio_framework_job::default_now_us().expect("clock");
        let outcome = step_once(&mut job, 1, started + semio_framework_job::INTERACTIVE_LANE_WALL_US * 8, &cancel, OperationId(1), Generation(0), &mut sequence);
        worst_us = worst_us.max(semio_framework_job::default_now_us().expect("clock").saturating_sub(started));
        match outcome {
            Stepped::Yield => {}
            Stepped::Preview(bytes) => ticks.push(ToolRunTick::decode(&bytes).expect("tick decodes")),
            other => break other,
        }
        let armed = handle.with_mut::<Generation2dInstanceOperationOwner, _>(|owner| Ok(owner.parts()?.0.document.window_tick_is_armed(window_id))).expect("owner");
        if armed {
            hops_performed += 1;
            let doc = ArtifactView::new(&*snapshot, &history);
            let cfg = ConfigView { snapshot: &config, window: None };
            let (emit, transient) = handle.with_mut::<Generation2dInstanceOperationOwner, _>(|owner| generation2d_flow_eval_hop(owner, &tick, &doc, &doc, &cfg, 0)).expect("the hop runs");
            assert!(emit.extension_invocations.is_empty() && transient.is_empty(), "an uncontributed document target parks nothing and publishes no transient");
        }
        assert!(ticks.len() < 10_000, "the run must settle");
    };
    let mut final_nodes: std::collections::BTreeMap<u64, &'static str> = std::collections::BTreeMap::new();
    let mut verdicts = std::collections::BTreeSet::new();
    for op in ticks.iter().flat_map(|tick| tick.trace.iter()).flat_map(|page| page.ops.iter()) {
        if let ToolRunTraceOp::Upsert { key, verdict, reason, .. } = op {
            verdicts.insert(format!("{verdict:?}"));
            final_nodes.insert(*key, semio_s_artifact_procedural_2d_preview_eval::PreviewEvalRunReason::ALL[usize::from(*reason)].id());
        }
    }
    let expected_nodes: std::collections::BTreeMap<u64, &str> = expected["finalNodes"].as_object().unwrap().iter().map(|(node, reason)| (semio_s_artifact_procedural_2d_preview_eval::preview_eval_node_entity(node), reason.as_str().unwrap())).collect();
    let settle_reasons: Vec<&str> = ticks.iter().flat_map(|tick| tick.steps.iter()).map(|step| semio_s_artifact_procedural_2d_preview_eval::PreviewEvalRunReason::ALL[usize::from(step.reason)].id()).collect();
    let hops_counted = ticks.last().and_then(|tick| tick.progress.as_ref()).and_then(|progress| progress.counters.iter().find(|counter| counter.counter == semio_s_artifact_procedural_2d_preview_eval::PreviewEvalRunCounter::Hops.index())).map_or(0, |counter| counter.value);
    println!("[STATS] demoRun ticks={} hops_performed={hops_performed} hops_counted={hops_counted} worst_drive_step_us={worst_us} verdicts={verdicts:?} steps={settle_reasons:?}", ticks.len());
    assert_eq!(settled, Stepped::Complete, "the run completes");
    assert_eq!(final_nodes, expected_nodes, "every node's final traced verdict");
    assert_eq!(settle_reasons.last().copied(), expected["settleReason"].as_str(), "the settle step");
    assert!(hops_counted >= expected["hopsAtLeast"].as_u64().unwrap() && hops_counted as usize == hops_performed, "one counted hop per performed hop");
    assert!(worst_us <= expected["ceilingMicros"].as_u64().unwrap(), "worst drive_step {worst_us} µs overran the interactive ceiling");
    close_job(&mut job, "the demo run job");
    drop(snapshot);
    retire_owner(&handle);
}

/// ⚖️ LAW: closing an UNSETTLED run job — the host-driven abort — quiesces the sessions it was running: its
/// armed window owes nothing afterwards, so no poll restarts what the user stopped.
#[test]
fn closing_an_unsettled_run_job_quiesces_its_window() {
    use semio_s_artifact_procedural_2d_testkit::retirement_driver::close_job;
    use semio_s_artifact_procedural_2d_testkit::retirement_driver::step_once;
    use semio_framework_job::{Generation, OperationId};
    use semio_framework_tool_run::{ToolRunId, ToolRunIdentity};
    let window_id = "preview-1";
    let handle = demo_owner((window_id, edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW));
    let port = semio_framework_plugin::ToolRunJobPort::default();
    let mut job = semio_s_artifact_procedural_2d_preview_eval::PreviewEvalRunJob::<Generation2dInstanceOperationOwner>::new(handle.clone(), port, ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, [0; 32])).expect("the run job attaches");
    let mut sequence = 0;
    let started = semio_framework_job::default_now_us().expect("clock");
    let _ = step_once(&mut job, 1, started + semio_framework_job::INTERACTIVE_LANE_WALL_US * 8, &semio_framework_job::root_cancel_token(), OperationId(1), Generation(0), &mut sequence);
    let armed = handle.with_mut::<Generation2dInstanceOperationOwner, _>(|owner| Ok(owner.parts()?.0.document.window_tick_is_armed(window_id))).expect("owner");
    assert!(armed, "the first step dispatches the first hop");
    close_job(&mut job, "the unsettled run job");
    let (owed, still_armed) = handle.with_mut::<Generation2dInstanceOperationOwner, _>(|owner| {
        let (sessions, _) = owner.parts()?;
        Ok((sessions.document.window_tick_owed(window_id), sessions.document.window_tick_is_armed(window_id)))
    }).expect("owner");
    assert_eq!((owed, still_armed), (false, false), "an aborted run leaves its window quiescent");
    retire_owner(&handle);
}

#[test]
fn command_ids_are_unique_and_cover_every_row() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), GENERATION2D_BOUNDED_TOOL_IDS.len() + GENERATION2D_CONTRIBUTIONS_TOOL_IDS.len(), "every Generation2dCommand row must be covered by every_command()");
}

#[test]
fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — pinned
/// where the two vocabularies genuinely diverge. This is what a missing `#[dsl(keyword = ..)]` on a
/// payload struct silently breaks (the record prints with no keyword at all and fails to re-parse).
#[test]
fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    let expected_keywords = [
        "node-graph-edit",
        "move-media-node",
        "add-widget",
        "remove-widget",
        "connect-media-ports",
        "reorganize",
        "add-generation",
        "remove-generation",
        "rename-generation",
        "update-generation-values",
        "node-graph-viewport",
        "set-show-mode",
        "active-example",
        "generate",
        "set-eval-outputs",
        "canvas-pointer-down",
        "canvas-pointer-move",
        "canvas-pointer-up",
        "canvas-wheel",
        "select-generation",
        "flow-eval-tick",
        "flow-eval-resolve",
        "set-contributions",
    ];
    let commands = every_command();
    assert_eq!(commands.len(), expected_keywords.len(), "every_command() and expected_keywords must stay in the same declaration order");
    for (command, expected_keyword) in commands.iter().zip(expected_keywords) {
        let printed = protocol::OpText::print_op(command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_keyword, "wire keyword drifted for command {}: {printed:?}", command.command_id());
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<Generation2dCommand> {
    vec![
        Generation2dCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }),
        Generation2dCommand::MoveMediaNode(move_media_node::MoveMediaNode { node_id: "n1".into(), x: 1.0, y: 2.0 }),
        Generation2dCommand::AddWidget(add_widget::AddWidget { kind: "inputSlider".into(), neuron_kind: None, format: None, action: None, x: Some(10.0), y: None }),
        Generation2dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: "n1".into() }),
        Generation2dCommand::ConnectMediaPorts(connect_media_ports::ConnectMediaPorts { source_node_id: "n1".into(), source_port_id: "out".into(), target_node_id: "n2".into(), target_port_id: "in".into() }),
        Generation2dCommand::Reorganize(reorganize::Reorganize {}),
        Generation2dCommand::AddGeneration(add_generation::AddGeneration {}),
        Generation2dCommand::RemoveGeneration(remove_generation::RemoveGeneration { id: "g1".into() }),
        Generation2dCommand::RenameGeneration(rename_generation::RenameGeneration { id: "g1".into(), name: "Copy".into() }),
        Generation2dCommand::UpdateGenerationValues(update_generation_values::UpdateGenerationValues { generation_id: Some("g1".into()), question_id: "q1".into(), value: semio_framework_value::DslValue::float(5.0) }),
        Generation2dCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { viewport: semio_framework_os_kernel::Viewport2d::default() }),
        Generation2dCommand::SetShowMode(set_show_mode::SetShowMode { value: "wire".into() }),
        Generation2dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_procedural_2d_examples::demo::ID.into() }),
        Generation2dCommand::Generate(enter_generate::Generate {}),
        Generation2dCommand::SetEvalOutputs(set_eval_outputs::SetEvalOutputs { outputs_json: "{}".into() }),
        Generation2dCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {}),
        Generation2dCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { samples: vec![[1.0, 2.0], [3.0, 4.0]] }),
        Generation2dCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp { cancelled: false }),
        Generation2dCommand::CanvasWheel(canvas_wheel::CanvasWheel {}),
        Generation2dCommand::SelectGeneration(select_generation::SelectGeneration { id: Some("g1".into()) }),
        Generation2dCommand::FlowEvalTick(flow_eval_tick::FlowEvalTick { window_id: "preview-1".into(), window_kind_id: edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW.into() }),
        Generation2dCommand::FlowEvalResolve(flow_eval_resolve::FlowEvalResolve { window_id: "preview-1".into(), window_kind_id: edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW.into(), node_hash: 7, output_json: "{}".into(), ..Default::default() }),
        Generation2dCommand::SetContributions(set_contributions::SetContributions { json: "[]".into(), page: 0, page_count: 1 }),
    ]
}
//#endregion 🔖️CommandSurface
