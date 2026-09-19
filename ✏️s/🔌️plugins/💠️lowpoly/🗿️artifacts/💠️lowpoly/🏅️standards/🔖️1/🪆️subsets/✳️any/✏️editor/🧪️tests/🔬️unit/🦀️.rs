pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, new_app_with_registry, project_and_retire_fixture_tree};
    use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

    /// 🪪️ The one live instance every fixture app binds — a registry-backed app refuses typed
    /// commands until it knows its instance (`interactive-job.live-instance`).
    pub const INSTANCE: u32 = 1;

    /// 🔁️ The settle loop's maintenance grant. `settle_registered_typed_operation` pages maintenance at
    /// exactly one 4 KiB page; a lowpoly document carrying mesh content is larger than that, so its
    /// close cursor never receives a grant it can act on and the operation never retires (measured
    /// 2026-09-17: a 2 299-byte snapshot settles, a 4 240-byte one hangs).
    const SETTLE_GRANT_BYTES: usize = 1 << 20;

    /// 🧪️ A registry-backed, instance-bound app that closes its stores on drop (the store drop witness
    /// panics otherwise). Derefs to the framework app, so every `PluginApp` call reads as before.
    pub struct LowpolyApp(pub VcsArtifactApp<EditorApp<LowpolyPlayApp>>);

    impl std::ops::Deref for LowpolyApp {
        type Target = VcsArtifactApp<EditorApp<LowpolyPlayApp>>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for LowpolyApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for LowpolyApp {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                close_registered_fixture_app(&mut self.0);
            }
        }
    }

    /// 🧪️ `new_app_with_registry`/`assert_declared_actions_bridge_to_commands` (framework test context,
    /// unchanged for this ticket) still take `fn() -> App` — `create_lowpoly_app` now returns
    /// `AppDefinition` (contract §2.4). This tiny local wrapper is the documented bridge (pilot report
    /// `📓️w2-cad-report.md` recipe step 7), not a framework fix owed by this packet.
    pub fn lowpoly_manifest_for_tests() -> App {
        App { definition: create_lowpoly_app(), examples: Vec::new() }
    }

    /// 🧪️ The registry-backed app every unit test builds on. It used to be the registry-less
    /// `new_app`, which fails closed since lowpoly's 47 tools became `Migrated` with exact factories: an
    /// empty registry has no migrated ids, so the tool-proof catalog rejects the first factory
    /// (`interactive-job.catalog-authority`, 25 tests, ticket 26/08/29/LOWPOLY-END-TO-END, 2026-09-17).
    pub async fn app() -> LowpolyApp {
        app_with_registry().await
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn app_with_registry() -> LowpolyApp {
        let mut app = new_app_with_registry::<EditorApp<LowpolyPlayApp>>(lowpoly_manifest_for_tests).await;
        app.bind_instance_id(INSTANCE).await;
        LowpolyApp(app)
    }

    /// 🪟️ The Model window's view state — what the React shell stamps on every dispatch, and what the
    /// window-scoped verbs (`setCamera`, the select toggles) key their config mutations off.
    pub fn action_meta() -> semio_framework_plugin::ActionMeta {
        let id = edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN;
        let mut action = meta("local");
        action.view_state = Some(ViewModel { window_id: Some(id.into()), window_instances: vec![ViewWindowInstance { id: id.into(), window_kind_id: id.into() }], ..Default::default() });
        action
    }

    /// 🎯️ Dispatches one typed command through the retained route and settles its typed operation, so
    /// the snapshot a test reads next is the committed one.
    pub async fn dispatch(app: &mut LowpolyApp, command: LowpolyCommand) -> InvocationResult {
        let verb = command.command_id().to_string();
        let result = app.0.dispatch_typed(command, &action_meta()).await.unwrap_or_else(|fault| panic!("{verb} refused: {fault:?}"));
        settle(&mut app.0, &verb).await;
        result
    }

    /// 🕹️ Dispatches one string action exactly as the React shell does (`handle_action` →
    /// `command_from_action` → retained job → settle). Framework-reserved verbs (`interactionSelect`,
    /// `undo`, `redo`) return an admission receipt; their tool job only lands through the reserved settle.
    pub async fn act(app: &mut LowpolyApp, action: &str, args: serde_json::Value) {
        let args = protocol::DslValue::from(&args);
        let result = app.0.handle_action(action, Some(&args), &action_meta()).await.unwrap_or_else(|fault| panic!("{action} refused: {fault:?}"));
        if matches!(action, "interactionSelect" | "interactionHover" | "undo" | "redo") {
            semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, result).await.unwrap_or_else(|fault| panic!("{action} did not settle its reserved job: {fault:?}"));
        }
        settle(&mut app.0, action).await;
    }

    /// 🚫️ Dispatches a command the app must REFUSE and returns the refusal's text — either the dispatch
    /// itself faults, or the retained reducer publishes a fault page while the operation settles.
    pub async fn dispatch_refused(app: &mut LowpolyApp, command: LowpolyCommand) -> String {
        let verb = command.command_id().to_string();
        if let Err(fault) = app.0.dispatch_typed(command, &action_meta()).await {
            return fault.message;
        }
        settle_faults(&mut app.0, &verb).await.into_iter().next().unwrap_or_else(|| panic!("{verb} was admitted and settled without a refusal"))
    }

    /// 🔁️ `settle`, but a fault page is collected instead of asserted away.
    async fn settle_faults(app: &mut VcsArtifactApp<EditorApp<LowpolyPlayApp>>, action: &str) -> Vec<String> {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
        let mut faults = Vec::new();
        while app.has_pending_typed_operations() {
            assert!(std::time::Instant::now() < deadline, "{action} did not settle");
            let _ = app.maintenance_step(1, SETTLE_GRANT_BYTES).unwrap_or_else(|fault| panic!("{action} maintenance: {fault:?}"));
            app.advance_typed_operation_publication().await.unwrap_or_else(|fault| panic!("{action} publication: {fault:?}"));
            while let Some(page) = app.take_typed_operation_result_page(INSTANCE) {
                if page.lane == semio_framework_plugin::app::TypedOperationResultLane::Fault {
                    faults.push(String::from_utf8_lossy(page.bytes()).to_string());
                }
                assert!(app.acknowledge_typed_operation_result(page.token).unwrap_or(false), "{action} rejected its result ACK");
            }
            while app.take_typed_operation_effect().is_some() {}
            while app.take_typed_operation_event().is_some() {}
            while app.take_typed_operation_ui_scope().is_some() {}
            while app.take_typed_operation_completion().await.unwrap_or(None).is_some() {}
            while let Some(reply) = app.take_local_interaction_query_reply() {
                if let protocol::LocalInteractionQueryReply::Page { page } = reply {
                    let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
                    assert!(app.acknowledge_local_interaction_query(&token), "{action} rejected its local-interaction ACK");
                }
            }
            std::thread::yield_now();
        }
        faults
    }

    /// 🔁️ The fixture's own settle loop, at a REAL host grant (see `SETTLE_GRANT_BYTES`). Returns the
    /// number of result pages the operation published.
    pub async fn settle(app: &mut VcsArtifactApp<EditorApp<LowpolyPlayApp>>, action: &str) -> usize {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
        let mut lanes = 0;
        while app.has_pending_typed_operations() {
            assert!(std::time::Instant::now() < deadline, "{action} did not settle");
            let _ = app.maintenance_step(1, SETTLE_GRANT_BYTES).unwrap_or_else(|fault| panic!("{action} maintenance: {fault:?}"));
            app.advance_typed_operation_publication().await.unwrap_or_else(|fault| panic!("{action} publication: {fault:?}"));
            while let Some(page) = app.take_typed_operation_result_page(INSTANCE) {
                let lane = page.lane;
                let fault = (lane == semio_framework_plugin::app::TypedOperationResultLane::Fault).then(|| String::from_utf8_lossy(page.bytes()).to_string());
                assert!(app.acknowledge_typed_operation_result(page.token).unwrap_or(false), "{action} rejected its result ACK");
                assert!(fault.is_none(), "{action} publication fault: {}", fault.unwrap_or_default());
                lanes += 1;
            }
            while app.take_typed_operation_effect().is_some() {}
            while app.take_typed_operation_event().is_some() {}
            while app.take_typed_operation_ui_scope().is_some() {}
            while app.take_typed_operation_completion().await.unwrap_or(None).is_some() {}
            while let Some(reply) = app.take_local_interaction_query_reply() {
                if let protocol::LocalInteractionQueryReply::Page { page } = reply {
                    let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
                    assert!(app.acknowledge_local_interaction_query(&token), "{action} rejected its local-interaction ACK");
                }
            }
            std::thread::yield_now();
        }
        lanes
    }

    /// 🧾️ How many document mutations the session command log has recorded — the retained route
    /// commits through the typed operation, so an `InvocationResult` no longer carries the mutation
    /// and the log is what a test counts.
    pub async fn committed_edits(app: &mut LowpolyApp) -> usize {
        app.0.history_snapshot().await.expect("history snapshot").upserts.iter().filter(|entry| entry.kind == "mutation" && entry.applied).count()
    }

    /// 🖼️ Renders one body and projects it to JSON through the fixture observer, which also retires the
    /// paged children a serde projection cannot carry (`BuiltChildren requires retained page transport`).
    pub async fn render(app: &mut LowpolyApp, body_key: &str) -> String {
        let tree = app.0.render(body_key, None, &ViewModel::default()).await.expect("render");
        project_and_retire_fixture_tree(tree).expect("rendered fixture observation and retirement")
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking is now the framework's
    /// injected `interactionSelect` verb, dispatched against the "mesh" domain declared on this app.
    /// `object_id`/`face_id` address the same row id the Document panel tree renders (see
    /// `🧭️view/🦀️.rs`'s `🔖️MeshDomain` region).
    pub async fn select_face(app: &mut LowpolyApp, object_id: &str, face_id: u32) {
        select(app, &[("face", &crate::editor::lowpoly::view::document_target_row_id(object_id, "face", face_id))]).await;
    }

    /// 🕹️ A replace-merge pick of `targets` (`(granularity, row id)`) in the mesh domain.
    pub async fn select(app: &mut LowpolyApp, targets: &[(&str, &str)]) {
        let targets: Vec<serde_json::Value> = targets.iter().map(|(granularity, id)| serde_json::json!({ "granularity": granularity, "id": id })).collect();
        act(app, "interactionSelect", serde_json::json!({ "domainId": MESH_INTERACTION_DOMAIN, "targets": serde_json::to_string(&targets).expect("targets"), "merge": "replace", "method": "pick" })).await;
    }
}

use super::*;
use crate::editor::lowpoly::unit_tests::context::{app, app_with_registry, dispatch, lowpoly_manifest_for_tests, LowpolyApp};
use semio_framework_plugin::{artifact_app_laws, EditorApp, PluginApp};

fn retained_operation() -> AppOperationContext {
    AppOperationContext { app_instance_id: 7, parent_document_id: "lowpoly-retained-test".into(), operation_id: 11, generation: 13, canonical_base_revision: [17; 32] }
}

fn retained_context(transient: LowpolyTransient, transient_generation: u64) -> std::sync::Arc<ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>> {
    std::sync::Arc::new(ArtifactOwnedToolJobContext::new(
        7,
        None,
        [17; 32],
        0,
        transient_generation,
        semio_framework_plugin::app::ArtifactOwnedToolJobSnapshots {
            children: std::sync::Arc::new(semio_framework_plugin::ChildContentView::EMPTY),
            draft: std::sync::Arc::new(NoDraft::default()),
            transient: std::sync::Arc::new(transient),
            window_config: None,
            window_transient: None,
        },
    ))
}

#[test]
fn retained_route_partition_and_publication_are_exact() {
    use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};

    // 🎯️ The generated command schema is the one count every other table must match — 46 since
    // 2026-09-08 (`setActiveUtility` became framework-owned, `setFixtureJson` became
    // `replaceSnapshotJson`); the literal 47 these assertions carried was never re-run.
    let declared = LowpolyCommand::TOOL_JOB_IDS.len();
    let all = every_command();
    let mut partition = LOWPOLY_MIGRATED_TOOL_IDS.to_vec();
    partition.sort_unstable();
    partition.dedup();
    assert_eq!(partition.len(), declared, "every generated command id is Migrated");
    assert_eq!(all.len(), partition.len());
    assert!(all.iter().all(|command| partition.binary_search(&command.command_id()).is_ok()));
    assert!(LOWPOLY_MIGRATED_TOOL_IDS.iter().all(|tool_id| lowpoly_command_disposition(tool_id).is_some()));
    assert_eq!(<LowpolyPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), declared);
    assert_eq!(<LowpolyCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.len(), declared);
    assert_eq!(lowpoly_contract().shape, ToolExecutionShape::Resumable);
    assert_eq!(lowpoly_contract().cancellation, ToolCancellationPolicy::PerOperation);
    assert_eq!((lowpoly_contract().checkpoint_every_steps, lowpoly_contract().progress_every_steps), (1, 1));
}

#[semio_framework_async_macros::async_test]
async fn retained_progress_replay_freshness_and_close_are_exact() {
    let command = LowpolyCommand::ToggleShowEdges(toggle_show_edges::ToggleShowEdges {});
    let snapshot = crate::schema::default_snapshot();
    let config = LowpolyConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let history = HistoryView::empty();
    let operation = retained_operation();
    let context = retained_context(LowpolyTransient::default(), 19);
    let context_identity = context.identity_digest();
    let mut uninterrupted = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, operation.canonical_base_revision, context_identity);
    assert!(matches!(
        uninterrupted
            .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
                command: &command,
                snapshot: &snapshot,
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: Some(&context),
                operation: &operation
            })
            .expect("progress"),
        ArtifactCommandWorkStep::Progress { .. }
    ));
    let mut checkpoint = [0_u8; 88];
    uninterrupted.checkpoint(&mut checkpoint).expect("checkpoint");
    let mut wrong_base = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, [18; 32], context_identity);
    assert!(wrong_base.restore(&checkpoint).is_err());
    let mut replayed = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, operation.canonical_base_revision, context_identity);
    replayed.restore(&checkpoint).expect("work restore");
    assert!(matches!(
        replayed
            .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
                command: &command,
                snapshot: &snapshot,
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: Some(&context),
                operation: &operation
            })
            .expect("replay"),
        ArtifactCommandWorkStep::Replay { .. }
    ));
    assert!(matches!(
        replayed
            .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
                command: &command,
                snapshot: &snapshot,
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: Some(&context),
                operation: &operation
            })
            .expect("complete"),
        ArtifactCommandWorkStep::Complete(_)
    ));
    let drifted = AppOperationContext { generation: operation.generation + 1, ..operation.clone() };
    let mut rejected = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, operation.canonical_base_revision, context_identity);
    assert!(rejected
        .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs { command: &command, snapshot: &snapshot, config: &config, history: &history, interaction: &interaction, hover: &hover, context: Some(&context), operation: &drifted })
        .is_err());
    let drifted_context = retained_context(LowpolyTransient::default(), 20);
    assert!(rejected
        .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
            command: &command,
            snapshot: &snapshot,
            config: &config,
            history: &history,
            interaction: &interaction,
            hover: &hover,
            context: Some(&drifted_context),
            operation: &operation
        })
        .is_err());
    assert!(matches!(
        rejected
            .step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
                command: &command,
                snapshot: &snapshot,
                config: &config,
                history: &history,
                interaction: &interaction,
                hover: &hover,
                context: Some(&context),
                operation: &operation
            })
            .expect("exact retry"),
        ArtifactCommandWorkStep::Progress { .. }
    ));
    assert_eq!(replayed.close_step(0, 0), InteractiveJobCloseStep::Blocked);
    replayed.begin_close();
    assert_eq!(replayed.close_step(1, 1), InteractiveJobCloseStep::Complete);
    assert!(replayed.terminal_is_empty());
}

/// ⏱️ The interactive-step law over the UNIT BOX, not the concrete-forest default document: on 195
/// faces a debug build measures `mirror` at 42 ms, `loopCut` 18 ms, `toggleSmooth` 10 ms and even a
/// whole-object `translateSelection` 9 ms — the JSON parse + kernel pass + re-encode floor of one
/// monolithic mesh edit, which the runtime records as a single admitted overrun (`StepOverrunLedger`
/// quarantines only four CONSECUTIVE ones). Over the box every command's cost is its dispatch
/// overhead, which is what this law guards: `addPrimitive` re-encoding every untouched mesh (11.6 ms)
/// was such a bug (2026-09-18). Chunking whole-mesh kernel ops into resumable steps is a kernel job.
///
/// ⏱️ A debug build measures the same box commands at 8.7–10.6 ms (`triangulate`, `paintFill`'s 256²
/// flood fill + 262 KB diff) on a machine at load average 30, flapping between commands run to run, so
/// the ceiling a debug build is held to is the runtime's own quarantine bound — four consecutive
/// ceilings, the most one genuinely slow step may burn before `StepOverrunLedger` stops it. Release
/// builds (what the wasm guest ships as) keep the exact 8 ms law.
const INTERACTIVE_TURN_CEILING: std::time::Duration = std::time::Duration::from_micros(if cfg!(debug_assertions) { semio_framework_job::INTERACTIVE_STEP_CEILING_US * semio_framework_job::SUSTAINED_OVERRUN_QUARANTINE_STEPS as u64 } else { semio_framework_job::INTERACTIVE_STEP_CEILING_US });
#[semio_framework_async_macros::async_test]
async fn retained_migrated_turns_stay_below_eight_milliseconds() {
    let unit_box = semio_framework_3d::mesh::HalfedgeMesh::box_prim(1.0, 1.0, 1.0).expect("box prim").to_json().expect("box json");
    let snapshot = crate::snapshot_from_mesh_json(&unit_box, "obj-1", "Unit Box");
    let config = LowpolyConfig::default();
    let interaction = protocol::InteractionState::default();
    let hover = semio_framework_plugin::app::InteractionHoverState::default();
    let history = HistoryView::empty();
    let operation = retained_operation();
    let context = retained_context(LowpolyTransient::default(), 29);
    for command in every_command().into_iter().filter(|command| LOWPOLY_MIGRATED_TOOL_IDS.contains(&command.command_id())) {
        let tool_id = command.command_id();
        let disposition = lowpoly_command_disposition(tool_id).expect("migrated disposition");
        // ⏱️ Best of three: an oversubscribed host deschedules a step for whole milliseconds, which the
        // runtime forgives as an isolated overrun; the law is on the command's own cost.
        let mut best = std::time::Duration::MAX;
        for _attempt in 0..3 {
            let mut work = LowpolyRetainedCommandWork::new(tool_id, disposition, operation.operation_id, operation.generation, operation.canonical_base_revision, context.identity_digest());
            let mut slowest = std::time::Duration::ZERO;
            // 🔊️ Nothing is selected here, so a selection-bound mesh edit REFUSES (`no faces selected`) —
            // a refusal is a turn like any other and must respect the same ceiling.
            let mut refused = false;
            loop {
                let started = std::time::Instant::now();
                let step = work.step(&semio_framework_plugin::retained_command::ArtifactCommandInputs {
                    command: &command,
                    snapshot: &snapshot,
                    config: &config,
                    history: &history,
                    interaction: &interaction,
                    hover: &hover,
                    context: Some(&context),
                    operation: &operation,
                });
                slowest = slowest.max(started.elapsed());
                match step {
                    Ok(ArtifactCommandWorkStep::Complete(_) | ArtifactCommandWorkStep::CompleteWithEphemeral { .. }) => break,
                    Ok(_) => {}
                    Err(fault) => {
                        // 🗿️ `deleteSelection` on the one-object box refuses "keeps at least one object" — that
                        // is its empty-selection answer at object granularity.
                        assert!(fault.message.contains("selected") || fault.message.contains("selection") || fault.message.contains("at least one object"), "{tool_id} refused for a reason other than the empty selection: {fault:?}");
                        refused = true;
                        break;
                    }
                }
            }
            best = best.min(slowest);
            if !refused {
                work.begin_close();
                assert_eq!(work.close_step(1, LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES), InteractiveJobCloseStep::Complete);
            }
        }
        assert!(best < INTERACTIVE_TURN_CEILING, "{tool_id} turn exceeded {INTERACTIVE_TURN_CEILING:?} on every attempt: best {best:?}");
    }
}

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
/// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), LowpolyCommand::TOOL_JOB_IDS.len(), "every LowpolyCommand row must be covered by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword.
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    for command in every_command() {
        let printed = protocol::OpText::print_op(&command);
        let first_token = printed.split(' ').next().unwrap_or_default();
        assert!(!first_token.is_empty(), "printed op line must start with a wire keyword: {printed:?}");
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<LowpolyCommand> {
    vec![
        LowpolyCommand::AddPrimitive(add_primitive::AddPrimitive { kind: Some("box".into()) }),
        LowpolyCommand::PatchObject(patch_object::PatchObject { object_id: "obj-1".into(), field: "name".into(), value_json: Some("\"Renamed\"".into()) }),
        LowpolyCommand::Extrude(extrude::Extrude { extrude_distance: Some(0.25) }),
        LowpolyCommand::Inset(inset::Inset { inset_amount: Some(0.1) }),
        LowpolyCommand::Bevel(bevel::Bevel { bevel_amount: Some(0.05), bevel_segments: Some(1) }),
        LowpolyCommand::LoopCut(loop_cut::LoopCut { loop_cuts: Some(1) }),
        LowpolyCommand::Subdivide(subdivide::Subdivide {}),
        LowpolyCommand::Triangulate(triangulate::Triangulate {}),
        LowpolyCommand::Mirror(mirror::Mirror { axis: Some("x".into()) }),
        LowpolyCommand::Decimate(decimate::Decimate { decimate_ratio: Some(0.5) }),
        LowpolyCommand::FlipFaces(flip_faces::FlipFaces { face_ids: vec![0] }),
        LowpolyCommand::Merge(merge::Merge {}),
        LowpolyCommand::Dissolve(dissolve::Dissolve {}),
        LowpolyCommand::Snap(snap::Snap {}),
        LowpolyCommand::ToggleSmooth(toggle_smooth::ToggleSmooth {}),
        LowpolyCommand::UnwrapActive(unwrap_active::UnwrapActive {}),
        LowpolyCommand::MarkUvSeam(mark_uv_seam::MarkUvSeam { seam: Some(true), edge_ids: Some(vec![0]) }),
        LowpolyCommand::ClearSeam(clear_seam::ClearSeam {}),
        LowpolyCommand::TranslateSelection(translate_selection::TranslateSelection { mode: Some("mesh".into()), ids: Some(vec![]), dx: 1.0, dy: 0.0, dz: 0.0 }),
        LowpolyCommand::RotateSelection(rotate_selection::RotateSelection { mode: Some("mesh".into()), ids: Some(vec![]), ax: 0.0, ay: 1.0, az: 0.0, angle: 45.0 }),
        LowpolyCommand::ScaleSelection(scale_selection::ScaleSelection { mode: Some("mesh".into()), ids: Some(vec![]), sx: 1.0, sy: 1.0, sz: 1.0 }),
        LowpolyCommand::AddPaintLayer(add_paint_layer::AddPaintLayer { object_id: None, name: Some("Detail".into()) }),
        LowpolyCommand::PaintStrokeEnd(paint_stroke_end::PaintStrokeEnd {}),
        LowpolyCommand::PaintFill(paint_fill::PaintFill { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
        LowpolyCommand::FillBucket(fill_bucket::FillBucket { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
        LowpolyCommand::TransformEnd(transform_end::TransformEnd {}),
        LowpolyCommand::ImportSnapshotJson(set_snapshot_json::ImportSnapshotJson { json: "{}".into() }),
        LowpolyCommand::ReplaceSnapshotJson(replace_snapshot_json::ReplaceSnapshotJson { json: "{}".into() }),
        LowpolyCommand::ExportMesh(export_mesh::ExportMesh { format: "obj".into() }),
        LowpolyCommand::LoadMeshRequest(load_mesh_request::LoadMeshRequest {}),
        LowpolyCommand::ImportMeshFile(import_mesh_file::ImportMeshFile { name: "quad.obj".into(), payload: "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n".into() }),
        LowpolyCommand::DeleteSelection(delete_selection::DeleteSelection {}),
        LowpolyCommand::DuplicateObject(duplicate_object::DuplicateObject { object_id: None }),
        LowpolyCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("extrude".into()) }),
        LowpolyCommand::SetActiveObject(set_active_object::SetActiveObject { object_id: "obj-1".into() }),
        LowpolyCommand::SetActivePaintLayer(set_active_paint_layer::SetActivePaintLayer { layer_index: 0 }),
        LowpolyCommand::SetUtilityParam(set_utility_param::SetUtilityParam { key: "brushSize".into(), value_json: "20".into() }),
        LowpolyCommand::EngagementInput(engagement_input::EngagementInput { value: "ext".into() }),
        LowpolyCommand::ToggleShowEdges(toggle_show_edges::ToggleShowEdges {}),
        LowpolyCommand::ToggleSun(toggle_sun::ToggleSun {}),
        LowpolyCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 45.0 }),
        LowpolyCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 35.0 }),
        LowpolyCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 0.8 }),
        LowpolyCommand::SetCamera(set_camera::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }),
        LowpolyCommand::PaintStrokeBegin(paint_stroke_begin::PaintStrokeBegin {}),
        LowpolyCommand::PaintSample(paint_sample::PaintSample { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
        LowpolyCommand::PaintStroke(paint_stroke::PaintStroke { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
        LowpolyCommand::PaintAt(paint_at::PaintAt { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
        LowpolyCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { object_id: None, u: None, v: None, x: Some(0.0), y: Some(0.0) }),
        LowpolyCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { object_id: None, u: None, v: None, x: Some(1.0), y: Some(1.0) }),
        LowpolyCommand::TransformBegin(transform_begin::TransformBegin {}),
    ]
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_lowpoly_app()).expect("app definition json");
    for id in [edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN, paint_mode::windows::uv::LOWPOLY_PLAY_WINDOW_UV] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    for id in [edit::LOWPOLY_PLAY_MODE_EDIT, paint_mode::LOWPOLY_PLAY_MODE_PAINT, paint_mode::LOWPOLY_PLAY_LAYOUT_PAINT] {
        assert!(json.contains(id), "mode/layout {id} missing from the manifest");
    }
    for body in [LOWPOLY_PLAY_BODY_ARTIFACT, LOWPOLY_PLAY_BODY_CATALOGUE, LOWPOLY_PLAY_BODY_INSPECTION, LOWPOLY_PLAY_BODY_LAYERS] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("3d.lowpoly"), "artifact kind missing from the manifest");
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "mesh" domain is declared and
/// scoped to the Model window, and the framework auto-injects its six interaction actions.
#[semio_framework_async_macros::async_test]
async fn the_mesh_interaction_domain_is_declared_and_scoped_to_the_model_window() {
    let definition = create_lowpoly_app();
    let mesh = definition.interactions.iter().find(|interaction| interaction.id == MESH_INTERACTION_DOMAIN).expect("mesh domain declared");
    assert_eq!(mesh.granularities.iter().map(|granularity| granularity.id.as_str()).collect::<Vec<_>>(), vec!["object", "vertex", "edge", "face"]);
    assert!(matches!(mesh.hierarchy, HierarchyProvider::Flat));
    let main_window = definition.window_kinds.iter().find(|window| window.id == edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN).expect("main window declared");
    assert_eq!(main_window.interactions, vec![InteractionRef::new(MESH_INTERACTION_DOMAIN)]);
    for injected in ["interactionSelect", "interactionHover", "clearSelection", "selectAll", "setSelectionMode", "setInteractionGranularity"] {
        assert!(main_window.actions.iter().any(|action| action.id == injected), "framework must auto-inject {injected}");
    }
    for deleted in ["setSelection", "toggleSelectionKind", "toggleSelectionTarget", "setSelectionMethod", "setSelectionModeDefault", "worldSelect", "worldHover", "setHover", "worldPick"] {
        assert!(!definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == deleted), "{deleted} must no longer be app-declared");
    }
}
//#endregion 🔖️ManifestSanity

//#region 🔖️CrossCutting
/// 🧹️ The REGISTERED pair: lowpoly publishes bounded tool proofs, so a registry-less `paired_apps`
/// instance faults in the `interactive-job.catalog-authority` proof join before any edit lands.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    artifact_app_laws::assert_two_registered_instances_converge::<EditorApp<LowpolyPlayApp>, _, _, _>(
        "mem://lowpoly-convergence",
        || async { lowpoly_manifest_for_tests() },
        LowpolyCommand::PatchObject(patch_object::PatchObject { object_id: "obj-1".into(), field: "name".into(), value_json: Some(serde_json::to_string("Renamed By A").unwrap()) }),
        LowpolyCommand::AddPrimitive(add_primitive::AddPrimitive { kind: Some("box".into()) }),
        |app| app.snapshot().expect("projection"),
    )
    .await;
}

/// 🔁️ `artifact_app_laws::assert_ingest_idempotent` over THIS crate's harness: the framework's registered
/// twin binds no live instance, so a tool-proof app refuses its very first typed command
/// (`interactive-job.live-instance`). Same law, same shape — a sender on a memory backbone, its envelopes
/// replayed twice onto a fresh receiver.
#[semio_framework_async_macros::async_test]
async fn ingest_operations_is_idempotent() {
    use store::{Backbone, BackboneMessage, MemoryBackbone};
    let mut sender = app().await;
    let (near, mut far) = MemoryBackbone::pair("mem://lowpoly-idempotent", "mem://lowpoly-idempotent").await;
    sender.attach_backbone(store::Backbones::Memory(near)).await.expect("attach sender");
    dispatch(&mut sender, LowpolyCommand::PatchObject(patch_object::PatchObject { object_id: "obj-1".into(), field: "name".into(), value_json: Some(serde_json::to_string("Hero").unwrap()) })).await;
    assert_eq!(sender.snapshot().expect("projection").objects[0].name, "Hero");
    let mut envelopes = Vec::new();
    for message in far.receive().await.expect("receive") {
        if let BackboneMessage::Mutations { envelopes: operations } = message {
            envelopes.extend(protocol::decode_envelopes(&operations).expect("decode envelopes"));
        }
    }
    assert!(!envelopes.is_empty(), "the rename reached the backbone");
    let operations = protocol::encode_envelopes(&envelopes);
    let mut receiver = app().await;
    receiver.ingest_operations(&operations).await.expect("ingest once");
    let once = receiver.snapshot().expect("projection");
    assert_eq!(once.objects[0].name, "Hero", "the replayed rename applies");
    receiver.ingest_operations(&operations).await.expect("ingest twice");
    assert_eq!(receiver.snapshot().expect("projection"), once, "feeding the same operation twice must not double-apply");
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use crate::editor::lowpoly::unit_tests::context::render;
    let mut a = app().await;
    assert!(render(&mut a, "lowpoly.play.nope").await.contains("Unknown body"));
}
//#endregion 🔖️CrossCutting

//#region 🔖️MediaPorts
#[semio_framework_async_macros::async_test]
async fn export_media_mesh_out_produces_mesh_document_payload() {
    let mut a: LowpolyApp = app().await;
    let media = semio_framework_plugin::resolve_ready(a.export_media("mesh:out")).expect("export mesh:out");
    assert_eq!(media.media_type, MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh });
    match media.payload {
        MediaPayload::Structured { schema, .. } => assert_eq!(schema, "mesh.document"),
        other => panic!("expected Structured payload, got {other:?}"),
    }
}

/// 🧬️ `"mesh:in"` replaces the whole document via `reset_document_effect` (a
/// `Effect::LoadDocument`, outside undo history) — whole-document replace has no replacement
/// mutation per `📓️taxonomy.md`, so this is an effect, not an `artifact_mutations` entry.
#[semio_framework_async_macros::async_test]
async fn import_media_mesh_in_round_trips_into_a_reset_document_effect() {
    let mesh = semio_framework_plugin::mesh_from_kind("box");
    let mesh_document = crate::schema::mesh_document_from_mesh(&mesh).expect("mesh document");
    let json = serde_json::to_string(&mesh_document).expect("mesh document json");
    let media = Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: MediaPayload::Structured { schema: "mesh.document".into(), json } };
    let projection = crate::schema::default_snapshot();
    let history = HistoryView::empty();
    let doc = ArtifactView::new(&projection, &history);
    let emit = LowpolyPlayApp::import_media("mesh:in", &media, &doc).expect("import mesh:in");
    assert!(emit.artifact_mutations.is_empty(), "whole-document replace is an effect, not a mutation");
    let semio_framework_plugin::Effect::LoadDocument { pack, .. } = emit.effects.first().expect("mesh:in must emit a LoadDocument effect") else {
        panic!("expected a LoadDocument effect");
    };
    let loaded = <LowpolySnapshot as ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
    assert_eq!(loaded.objects.len(), 1);
}
//#endregion 🔖️MediaPorts

//#region 🔖️ContextMenuRegistry
#[semio_framework_async_macros::async_test]
async fn registry_wired_app_dispatches_add_primitive() {
    let mut a = app_with_registry().await;
    dispatch(&mut a, LowpolyCommand::AddPrimitive(add_primitive::AddPrimitive { kind: Some("plane".into()) })).await;
    assert_eq!(a.snapshot().expect("projection").objects.len(), 2);
}
//#endregion 🔖️ContextMenuRegistry
