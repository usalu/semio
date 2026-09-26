pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::meta;
    use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
    use semio_s_artifact_stdio_semio::{create_semio_member, SemioMembers};
    use store::ArtifactPack;

    /// 🧪️ The registered, MOUNTED fixture app every editor test drives. It is a guard, not a bare
    /// alias: a registered app's `ArtifactStore` refuses `Drop` without its exact terminal-empty
    /// shallow-shell witness, so every test instance has to travel the framework's own close loop
    /// when it goes out of scope — a plain `VcsArtifactApp` binding panicked every render/panel test
    /// at the end of the test body instead of at its assertions.
    pub struct SequenceApp(pub(crate) VcsArtifactApp<EditorApp<SequencePlayApp>, SemioMembers>);

    impl std::ops::Deref for SequenceApp {
        type Target = VcsArtifactApp<EditorApp<SequencePlayApp>, SemioMembers>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for SequenceApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for SequenceApp {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0);
            }
        }
    }

    /// 🪪️ The live runtime instance every `meta(..)` dispatch is stamped with (`meta` always stamps 1).
    pub const SEQUENCE_TEST_INSTANCE: u32 = 1;

    pub async fn register_content_child(app: &mut SequenceApp) {
        let snapshot = app.snapshot().expect("Sequence parent snapshot");
        let materialized = neural_engine::ColdOwner::new(default_snapshot());
        let fixture = materialized.to_host_snapshot();
        let content = crate::sequence_content_snapshot_from_working(&fixture.steps, &fixture.edges);
        neural_engine::ColdRetire::retire_cold(fixture);
        let child_id = snapshot.content.child_id.clone();
        let dialect = snapshot.content.target.dialect.clone();
        assert_eq!(child_id, materialized.content.child_id, "Sequence concrete app and canonical child identities diverged");
        // 🌱️ `with_registry` seeds the app's declared genesis children itself (`seed_genesis_children`
        // → `genesis_sequence_child_pack`), so the `content` slot is usually already occupied and a
        // second `register_child` is refused with `interactive-job.child-member-duplicate`. The
        // fixture only has to FILL the slot when the host left it empty.
        if app.child_store("content", &child_id).await.is_some() {
            return;
        }
        let member = create_semio_member(&child_id, &dialect, &content.encode_pack()).await.expect("Sequence child member");
        app.register_child("content", child_id, dialect, member).await.expect("register Sequence content child");
    }

    /// 🧪️ An app instance carrying the real `AppActionRegistry` (identical to
    /// `new_app_with_registry_wired`). The registry-LESS `VcsArtifactApp::new` is unusable for this
    /// app: `with_registry_on_bus` joins `EditorApp<SequencePlayApp>`'s
    /// `bounded_first_step_tool_proofs!` roster against the registry's `Migrated` tool ids
    /// (`AppActionRegistry::validate_tool_job_rows`), and an empty registry declares none of them —
    /// construction panics with `interactive-job.catalog-authority` … `generated_migrated=false`,
    /// `migrated={}`. A registry-less wrapper could not dispatch anything anyway
    /// (`admit_command_wire_with_proof` refuses every verb that has no manifest declaration).
    pub async fn new_app() -> SequenceApp {
        new_app_with_registry_wired().await
    }

    /// 🧩️ `create_sequence_app` now returns `AppDefinition` (contract §2.4), not the runtime-shaped
    /// `App { definition, examples }` `new_app_with_registry` still expects (SDK gap, unchanged by
    /// this ticket — `context::assert_declared_actions_bridge_to_commands` carries the identical gap
    /// per `📓️w0-f-report.md` Gap 3) — wraps it with an empty `examples` list rather than porting one.
    pub(crate) fn sequence_manifest_for_tests() -> App {
        App { definition: create_sequence_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    /// 🪪️ MOUNTED: a registered app refuses every typed command whose `ActionMeta.instance_id` is not
    /// its bound live runtime instance (`interactive-job.live-instance`), and a freshly constructed
    /// wrapper has none — so the id `meta(..)` stamps is bound here, before the content child lands.
    pub async fn new_app_with_registry_wired() -> SequenceApp {
        let mut app = VcsArtifactApp::<EditorApp<SequencePlayApp>, SemioMembers>::with_registry(EditorApp::default(), AppActionRegistry::from_definition(&sequence_manifest_for_tests().definition)).await;
        app.bind_instance_id(SEQUENCE_TEST_INSTANCE).await;
        let mut app = SequenceApp(app);
        register_content_child(&mut app).await;
        app
    }

    /// 🧩️ The LIVE steps and edges, read from the `content` CHILD store — the surface every step/edge
    /// verb publishes on. A mounted app republishes the parent document when its retained operation
    /// settles, and the republished parent carries the composed handle WITHOUT a local owner, so
    /// `snapshot().to_host_snapshot()` faults `sequence child scene must be materialized before
    /// fixture projection: Absent` after the first settled edit. The child store is the same surface
    /// the windows read through `sequence_working_scene_from_children` (flow's `flow_child_node_count`
    /// is the sibling precedent). Cold-owned because it mints fresh step dictionaries nothing else owns.
    pub async fn live_host_snapshot(app: &SequenceApp) -> neural_engine::ColdOwner<SequenceHostSnapshot> {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
        use store::SpaceMember;
        let snapshot = app.snapshot().expect("Sequence parent projection");
        let child_id = snapshot.content.child_id.clone();
        let bytes = app.child_store("content", &child_id).await.expect("Sequence content child").document_pack_bytes().await.expect("Sequence content child pack");
        let content = SemioFlowSnapshot::decode_pack(&bytes).expect("Sequence content child snapshot");
        let (steps, edges) = crate::working_from_sequence_content_snapshot(&content);
        neural_engine::ColdOwner::new(SequenceHostSnapshot { schema: snapshot.schema.clone(), steps, edges })
    }

    /// 🪟️ One addressed window view. Sequence's window-config and window-transient verbs
    /// (`setViewport`/`setOrientation` on the main window, `run`/`stop` on the script window) refuse an
    /// unaddressed dispatch — `sequence-window-view-required` / `sequence-script-window-view-required` —
    /// and `addressed(view, ..)` resolves `view.window_id` against an instance OF ITS OWN KIND, so the
    /// two lanes need two metas (flow's `flow_main_window_meta` is the sibling precedent).
    fn window_meta(window_kind_id: &str) -> semio_framework_plugin::ActionMeta {
        let window = semio_framework_plugin::ViewWindowInstance { id: format!("{window_kind_id}#1"), window_kind_id: window_kind_id.into() };
        semio_framework_plugin::ActionMeta {
            view_state: Some(ViewModel {
                window_id: Some(window.id.clone()),
                active_window_kind_id: Some(window.window_kind_id.clone()),
                window_instances: vec![window],
                ..Default::default()
            }),
            ..meta("local")
        }
    }

    pub fn main_window_meta() -> semio_framework_plugin::ActionMeta {
        window_meta(main::SEQUENCE_PLAY_WINDOW_MAIN)
    }

    pub fn script_window_meta() -> semio_framework_plugin::ActionMeta {
        window_meta(script::SEQUENCE_PLAY_WINDOW_SCRIPT)
    }

    /// 🔁️ A mounted app ANSWERS before its retained typed operation has published: `dispatch_typed`
    /// only queues it, so a reader that skips the settle observes the pre-dispatch document. Drives
    /// the publication home the way the plugin host's continuation does and folds the settled
    /// receipt's effects into the answer.
    pub async fn dispatch(app: &mut SequenceApp, command: SequenceCommand) -> InvocationResult {
        dispatch_addressed(app, command, &main_window_meta()).await
    }

    /// 🔁️ [`dispatch`] against the SCRIPT window — the address `run`/`stop` publish their window
    /// transient at.
    pub async fn dispatch_in_script(app: &mut SequenceApp, command: SequenceCommand) -> InvocationResult {
        dispatch_addressed(app, command, &script_window_meta()).await
    }

    pub async fn dispatch_addressed(app: &mut SequenceApp, command: SequenceCommand, meta: &semio_framework_plugin::ActionMeta) -> InvocationResult {
        let mut result = app.0.dispatch_typed(command, meta).await.expect("dispatch");
        let settled = semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, SEQUENCE_TEST_INSTANCE).await.expect("settle the typed operation");
        result.requested_effects.extend(settled.effects);
        result
    }

    pub async fn render(app: &mut SequenceApp, body_key: &str) -> String {
        render_in(app, body_key, &ViewModel::default()).await
    }

    /// 🪟️ [`render`] through one addressed window view — a window-transient surface (the script
    /// window's last run result) is only readable from the instance it was published at.
    pub async fn render_in(app: &mut SequenceApp, body_key: &str, view: &ViewModel) -> String {
        let tree = app.render(body_key, None, view).await.expect("render");
        let tree = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("retire rendered tree");
        tree
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking is now the framework's
    /// injected `interactionSelect` verb, dispatched against the "steps" domain declared on this app —
    /// requires `new_app_with_registry_wired().await` (a bare `new_app().await` has no declared interaction
    /// domains to select against). `ids` are the steps' own raw document ids — the SAME ids the
    /// "steps" domain's topology/the document panel tree/the main node-graph canvas all use.
    /// 🪪️ `handle_action` only ADMITS the framework-reserved selection verb on a mounted app — the
    /// selection is not live until its reserved admission has been run to completion.
    pub async fn select_steps(app: &mut SequenceApp, ids: &[&str]) {
        let target_list: Vec<serde_json::Value> = ids.iter().map(|id| serde_json::json!({ "granularity": "step", "id": id })).collect();
        let targets = serde_json::to_string(&target_list).expect("targets json");
        let admitted = app
            .0
            .handle_action("interactionSelect", semio_framework_plugin::optional_json_to_dsl(Some(serde_json::json!({ "domainId": SEQUENCE_INTERACTION_STEPS, "targets": targets, "merge": "replace" }))).as_ref(), &meta("test"))
            .await
            .expect("interactionSelect");
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted).await.expect("interactionSelect admission settles");
    }
}

use super::*;
use crate::editor::sequence::unit_tests::context::{live_host_snapshot, new_app, new_app_with_registry_wired};
use semio_framework_plugin::{Locale, PluginApp, Terminology};

#[semio_framework_async_macros::async_test]
async fn default_snapshot_has_steps() {
    let fixture = neural_engine::ColdOwner::new(default_snapshot());
    assert_eq!(fixture.to_host_snapshot().steps.len(), 2);
}

/// ↩️ `artifact_app_laws::assert_undo_redo_round_trip`'s probe is a SYNCHRONOUS closure over the
/// parent projection, and sequence keeps its steps in a composed child that only the (async) child
/// store can answer for — so the law is spelled out here over that child, exactly as flow's
/// `undo_restores_fixture_after_add_widget` does for its own `content` child.
#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trip_through_the_wrapper() {
    use semio_framework_plugin::artifact_app_laws::{meta, settle_history_verb};
    let mut app = new_app().await;
    let receiver = meta("local").instance_id;
    assert_eq!(live_host_snapshot(&app).await.steps.len(), 2);
    context::dispatch(&mut app, SequenceCommand::AddStep(add_step::AddStep { kind: "log.print".into(), x: 0.0, y: 0.0 })).await;
    assert_eq!(live_host_snapshot(&app).await.steps.len(), 3, "addStep must land one step in the content child");
    settle_history_verb(&mut app.0, "undo", receiver).await;
    assert_eq!(live_host_snapshot(&app).await.steps.len(), 2, "undo must retire the child-lane group");
    settle_history_verb(&mut app.0, "redo", receiver).await;
    assert_eq!(live_host_snapshot(&app).await.steps.len(), 3, "redo must reapply the child-lane group");
}

/// 🧪️ The definitional regression proof: two independent instances start from the same fixture,
/// apply DISJOINT edits (A moves step-1, B moves step-2), and exchanging operations over a
/// `MemoryBackbone` converges both sides onto an identical projection.
///
/// 🧹️ The REGISTERED pair: sequence publishes bounded tool proofs, so a registry-less `paired_apps`
/// instance faults in the `interactive-job.catalog-authority` proof join (`generated_migrated=false`,
/// `migrated={}`) while it is constructed, before any edit lands.
///
/// 🧩️ Written out here rather than through `artifact_app_laws::assert_two_registered_instances_converge`:
/// that helper is typed `VcsArtifactApp<A>` — the `NoMembers` roster — and sequence composes its
/// steps/edges into an `s.stdio.semio@v1/flow` child, so a `NoMembers` instance dies at construction
/// with `derived child dialect 's.stdio.semio@v1/flow' is not declared by this app's member roster`.
/// The body is the helper's own, over this app's real `SemioMembers` pair.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    use semio_framework_plugin::artifact_app_laws::{meta, settle_registered_typed_operation};
    let mut instance_a = new_app_with_registry_wired().await;
    let mut instance_b = new_app_with_registry_wired().await;
    let (backbone_a, backbone_b) = store::MemoryBackbone::pair("mem://sequence-convergence", "mem://sequence-convergence").await;
    instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");
    // 🧩️ Measured on the CHILD, not on the parent projection: `moveStep` publishes on the `Child`
    // lane and leaves the parent's composed handle untouched, so a parent-side probe can never move.
    let genesis = live_host_snapshot(&instance_a).await.steps.iter().map(|step| (step.id.clone(), step.x)).collect::<Vec<_>>();
    let receiver = meta("actor-a").instance_id;
    instance_a.0.dispatch_typed(SequenceCommand::MoveStep(move_step::MoveStep { node_id: "step-1".into(), x: 111.0, y: 0.0 }), &meta("actor-a")).await.expect("a applies its edit");
    settle_registered_typed_operation(&mut instance_a.0, receiver).await.expect("a's edit publishes");
    instance_b.0.dispatch_typed(SequenceCommand::MoveStep(move_step::MoveStep { node_id: "step-2".into(), x: 222.0, y: 0.0 }), &meta("actor-b")).await.expect("b applies its edit");
    settle_registered_typed_operation(&mut instance_b.0, receiver).await.expect("b's edit publishes");
    instance_a.tick_backbone().await.expect("a folds b's events");
    instance_b.tick_backbone().await.expect("b folds a's events");
    let positions = |snapshot: &SequenceHostSnapshot| snapshot.steps.iter().map(|step| (step.id.clone(), step.x)).collect::<Vec<_>>();
    assert_eq!(positions(&*live_host_snapshot(&instance_a).await), positions(&*live_host_snapshot(&instance_b).await), "both instances must converge on the same snapshot");
    let admitted = instance_a.0.handle_action("commitCheckpoint", None, &meta("actor-a")).await.expect("a commits a checkpoint");
    semio_framework_plugin::app::settle_framework_reserved_admission(&mut instance_a.0, admitted).await.expect("a's checkpoint commit settles");
    settle_registered_typed_operation(&mut instance_a.0, receiver).await.expect("a's checkpoint publication settles");
    instance_b.tick_backbone().await.expect("b folds a's checkpoint");
    assert_eq!(positions(&*live_host_snapshot(&instance_a).await), positions(&*live_host_snapshot(&instance_b).await), "a replicated checkpoint keeps both instances converged");
    assert_ne!(positions(&*live_host_snapshot(&instance_a).await), genesis, "the replicated edits must actually land, not converge on the untouched genesis");
    // 🔌️ Both paired ends have to detach before either guard closes: a live backbone end blocks the
    // store's close loop.
    instance_a.detach_backbone().await.expect("a releases its backbone");
    instance_b.detach_backbone().await.expect("b releases its backbone");
}

#[semio_framework_async_macros::async_test]
async fn sequence_action_ids_resolve_to_labels_in_native_english_and_german() {
    let definition = create_sequence_app();
    for (id, label) in [("run", "Run"), ("stop", "Stop"), ("reorganize", "Reorganize")] {
        let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == id).expect("action");
        assert_eq!(action.label.resolve(Terminology::Native, Locale::En), label, "{id} action label");
    }
    for (id, label) in [("run", "Ausführen"), ("stop", "Stopp"), ("reorganize", "Neu anordnen")] {
        let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == id).expect("action");
        assert_eq!(action.label.resolve(Terminology::Native, Locale::De), label, "{id} action label");
    }
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    let mut app = new_app().await;
    assert!(context::render(&mut app, "sequence.play.nope").await.contains("Unknown body"));
}

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_sequence_app()).expect("app definition json");
    for id in [main::SEQUENCE_PLAY_WINDOW_MAIN, script::SEQUENCE_PLAY_WINDOW_SCRIPT, compiled::SEQUENCE_PLAY_WINDOW_COMPILED] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    assert!(json.contains(edit::SEQUENCE_PLAY_MODE_EDIT), "edit mode missing from the manifest");
    for body in [SEQUENCE_PLAY_BODY_ARTIFACT, SEQUENCE_PLAY_BODY_CATALOGUE, SEQUENCE_PLAY_BODY_INSPECTOR] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("computation.sequence"), "artifact kind missing from the manifest");
}
//#endregion 🔖️ManifestSanity

//#region 🔖️ContextMenuTests
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `ArtifactApp::context_menu`
/// carries no `InteractionView` (a documented framework gap — see `sequence_context_menu_items`'s
/// own doc comment), so this exercises that free function directly with a real `selected` slice
/// instead of going through the app's live (always-empty) `context_menu` trait method.
#[semio_framework_async_macros::async_test]
async fn context_menu_stays_within_nine_rows_and_ends_with_destructive_delete() {
    let registry = AppActionRegistry::from_definition(&create_sequence_app());
    let items = sequence_context_menu_items(&registry, false, None, &["step-1".to_string()]);
    assert!(items.len() <= 9, "expected <= 9 top-level rows, got {} ({items:?})", items.len());
    let last = items.last().expect("at least one row");
    assert_eq!(last.id, "delete-selection");
    assert_eq!(last.destructive, Some(true));
}
//#endregion 🔖️ContextMenuTests

//#region 🔖️PortTests
#[semio_framework_async_macros::async_test]
async fn sequence_io_declares_steps_in_and_document_ports() {
    let ports = SequencePlayApp::io().expect("io").all_ports().await;
    assert!(ports.iter().any(|port| port.id == "artifact:in"));
    assert!(ports.iter().any(|port| port.id == "artifact:out"));
    assert!(ports.iter().any(|port| port.id == "steps:in"));
}

#[semio_framework_async_macros::async_test]
async fn import_media_steps_in_inserts_a_new_step_from_an_object_payload() {
    let mut app = new_app_with_registry_wired().await;
    let before = live_host_snapshot(&app).await.steps.len();
    let media = Media {
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Any },
        payload: MediaPayload::Structured { schema: "computation.value".into(), json: json!({ "message": "from upstream" }).to_string() },
    };
    app.import_media("steps:in", media, &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("import steps:in");
    let after = live_host_snapshot(&app).await;
    assert_eq!(after.steps.len(), before + 1);
    let imported = after.steps.last().expect("imported step");
    assert_eq!(imported.kind, "computation.import");
    assert_eq!(imported.params.get("message").and_then(|value| value.as_atom()).and_then(|atom| atom.as_str()), Some("from upstream"));
}

#[semio_framework_async_macros::async_test]
async fn import_media_steps_in_wraps_a_bare_scalar_payload() {
    let mut app = new_app_with_registry_wired().await;
    let media = Media {
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Any },
        payload: MediaPayload::Structured { schema: "computation.value".into(), json: "42".into() },
    };
    app.import_media("steps:in", media, &semio_framework_plugin::artifact_app_laws::meta("local")).await.expect("import steps:in");
    let after = live_host_snapshot(&app).await;
    let imported = after.steps.last().expect("imported step");
    assert_eq!(imported.params.get("value").and_then(|value| value.as_atom()).and_then(|atom| atom.as_f64()), Some(42.0));
}

#[semio_framework_async_macros::async_test]
async fn import_media_rejects_unknown_port() {
    let mut app = new_app_with_registry_wired().await;
    let media = Media {
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::Computation, form: semio_framework_plugin::MediaForm::Any },
        payload: MediaPayload::Structured { schema: "computation.value".into(), json: "{}".into() },
    };
    assert!(app.import_media("not-a-port", media, &semio_framework_plugin::artifact_app_laws::meta("local")).await.is_err());
}
//#endregion 🔖️PortTests

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
/// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to
/// hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    // 🧾️ Measured against the enum's own generated roster rather than a hand-copied count, so a row
    // added to (or removed from) `app_commands!` can never leave `every_command()` silently stale.
    let mut declared: Vec<&str> = SequenceCommand::TOOL_JOB_IDS.to_vec();
    declared.sort_unstable();
    assert_eq!(sorted, declared, "every SequenceCommand row must be covered by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
/// kebab-cased command id, for every row (sequence has no `flow`-style id/keyword divergence).
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    for command in every_command() {
        let id = command.command_id();
        let expected: String = id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect();
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
    }
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<SequenceCommand> {
    vec![
        SequenceCommand::AddStep(add_step::AddStep { kind: "log.print".into(), x: 1.0, y: 2.0 }),
        SequenceCommand::AddStepToSlot(add_step_to_slot::AddStepToSlot { kind: "log.print".into(), x: 1.0, y: 2.0, owner: "step-1".into(), slot_name: "then".into() }),
        SequenceCommand::AddStepDropped(add_step_dropped::AddStepDropped { kind: "log.print".into(), x: 1.0, y: 2.0, picked_step_id: Some("step-1".into()) }),
        SequenceCommand::RemoveStep(remove_step::RemoveStep { id: "step-1".into() }),
        SequenceCommand::DeleteSelection(delete_selection::DeleteSelection {}),
        SequenceCommand::MoveStep(move_step::MoveStep { node_id: "step-1".into(), x: 5.0, y: 6.0 }),
        SequenceCommand::ConnectSteps(connect_steps::ConnectSteps { source_node_id: "step-1".into(), target_node_id: "step-2".into() }),
        SequenceCommand::DisconnectSteps(disconnect_steps::DisconnectSteps { from_id: "step-1".into(), to_id: "step-2".into() }),
        SequenceCommand::SetStepParams(set_step_params::SetStepParams { id: "step-1".into(), params_json: "{\"a\":1}".into() }),
        SequenceCommand::SetStepCollapsed(set_step_collapsed::SetStepCollapsed { id: "step-1".into() }),
        SequenceCommand::Reorganize(reorganize::Reorganize {}),
        SequenceCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: "[]".into() }),
        SequenceCommand::SetOrientation(set_orientation::SetOrientation { value: "topBottom".into() }),
        SequenceCommand::Run(run_command::Run {}),
        SequenceCommand::Stop(stop_command::Stop {}),
        SequenceCommand::SetViewport(set_viewport::SetViewport { camera: SequenceCamera { x: 1.0, y: 2.0, zoom: 3.0 } }),
        SequenceCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "demo".into() }),
    ]
}

/// ⚖️ Pinned to the exact hex captured from the pre-merge `sequence_protocol` crate — a
/// regression here is a real wire-format break, not a test-fixture mismatch.
#[semio_framework_async_macros::async_test]
async fn optional_field_row_keeps_its_pre_migration_bytes() {
    let some = SequenceCommand::AddStepDropped(add_step_dropped::AddStepDropped { kind: "log.print".into(), x: 1.0, y: 2.0, picked_step_id: Some("step-1".into()) });
    assert_eq!(protocol::OpText::print_op(&some), "add-step-dropped add-step-dropped kind=log.print x=1 y=2 picked-step-id=step-1");
    assert_eq!(protocol::OpBinary::encode_op(&some).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), "010202096c6f672e7072696e7406737465702d31040006000105000000000000f03f02050000000000000040030601");
    let none = SequenceCommand::AddStepDropped(add_step_dropped::AddStepDropped { kind: "log.print".into(), x: 1.0, y: 2.0, picked_step_id: None });
    assert_eq!(protocol::OpText::print_op(&none), "add-step-dropped add-step-dropped kind=log.print x=1 y=2");
    assert_eq!(protocol::OpBinary::encode_op(&none).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), "010201096c6f672e7072696e74030006000105000000000000f03f02050000000000000040");
}
//#endregion 🔖️CommandSurface

//#region 🔖️HostTests
use neural_engine::Atom;

#[semio_framework_async_macros::async_test]
async fn disconnect_steps_removes_edge() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    assert!(host.disconnect_steps("step-1", "step-2"));
    assert!(host.snapshot.edges.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn sync_from_dag_copies_node_positions() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    if let Some(node) = host.dag.host_snapshot.nodes.iter_mut().find(|node| node.id == "step-1") {
        node.x = 120.0;
        node.y = 80.0;
    }
    host.sync_from_dag();
    let step = host.snapshot.steps.iter().find(|step| step.id == "step-1").expect("step-1");
    assert_eq!(step.x, 120.0);
    assert_eq!(step.y, 80.0);
}

#[semio_framework_async_macros::async_test]
async fn sync_edges_from_dag_preserves_existing_edge_ids() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    let first_id = host.snapshot.edges[0].id.clone();
    host.sync_edges_from_dag();
    assert_eq!(host.snapshot.edges[0].id, first_id);
    host.sync_edges_from_dag();
    assert_eq!(host.snapshot.edges[0].id, first_id);
}

#[semio_framework_async_macros::async_test]
async fn connect_steps_rejects_fan_out() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    host.snapshot.edges.clear();
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", NeuralValue::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
    assert!(host.connect_steps("step-1", "step-2").is_ok());
    assert!(host.connect_steps("step-1", "step-3").is_err());
}

#[semio_framework_async_macros::async_test]
async fn build_path_includes_control_bodies() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new().insert("key", NeuralValue::Atom(Atom::String("flag".into()))), x: 560.0, y: 0.0, slot: None, collapsed: false });
    host.snapshot.steps.push(SequenceStep {
        id: "step-4".into(),
        kind: "log.print".into(),
        params: StepParams::new().insert("message", NeuralValue::Atom(Atom::String("yes".into()))),
        x: 560.0,
        y: 160.0,
        slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }),
        collapsed: false,
    });
    host.snapshot.edges.push(SequenceEdge { id: "edge-2".into(), from: "step-2".into(), to: "step-3".into() });
    let path = host.build_path();
    assert_eq!(path.steps.len(), 3);
    let control = path.steps.iter().find(|step| step.id == "step-3").expect("control step");
    assert!(control.bodies.contains_key("then"));
    assert_eq!(control.bodies.get("then").map(|body| body.steps.len()), Some(1));
}

#[semio_framework_async_macros::async_test]
async fn rebuild_dag_preserves_selection() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    host.dag.set_selection(&["step-1".into()]);
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", NeuralValue::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
    host.rebuild_dag();
    assert!(host.dag.selected_node_ids().contains(&"step-1".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn execution_ports_use_triangle_shape() {
    let host = neural_engine::ColdOwner::new(SequenceHost::default());
    let node = host.step_to_dag_node(&host.snapshot.steps[1]);
    assert_eq!(node.inputs()[0].shape, PortShape::Triangle);
    assert_eq!(node.outputs()[0].shape, PortShape::Triangle);
}

#[semio_framework_async_macros::async_test]
async fn function_steps_use_data_ports_without_visible_execution_pins() {
    let host = neural_engine::ColdOwner::new(SequenceHost::default());
    let step = SequenceStep { id: "step-fn".into(), kind: "math.add".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false };
    let node = host.step_to_dag_node(&step);
    assert!(node.inputs().iter().any(|port| port.id == "a" && port.visible));
    assert!(node.inputs().iter().any(|port| port.id == "prev" && !port.visible));
    assert!(node.outputs().iter().any(|port| port.id == "next" && !port.visible));
    assert!(!node.inputs().iter().any(|port| port.shape == PortShape::Triangle && port.visible));
}

#[semio_framework_async_macros::async_test]
async fn text_steps_use_data_ports_without_visible_execution_pins() {
    let host = neural_engine::ColdOwner::new(SequenceHost::default());
    let step = SequenceStep { id: "step-txt".into(), kind: "text.concat".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: None, collapsed: false };
    let node = host.step_to_dag_node(&step);
    assert!(node.inputs().iter().any(|port| port.id == "left" && port.visible));
    assert!(node.inputs().iter().any(|port| port.id == "into" && port.visible));
    assert!(node.inputs().iter().any(|port| port.id == "prev" && !port.visible));
    assert!(node.outputs().iter().any(|port| port.id == "next" && !port.visible));
    assert!(!node.inputs().iter().any(|port| port.shape == PortShape::Triangle && port.visible));
}

#[semio_framework_async_macros::async_test]
async fn replace_snapshot_preserves_next_serial_and_selection() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    let first = host.add_step("math.add", 40.0, 40.0);
    host.dag.set_selection(std::slice::from_ref(&first));
    let json = host.to_json().expect("fixture json");
    let round_trip: SequenceHostSnapshot = dsl::os_pack::from_json_str(&json).expect("parse");
    host.replace_snapshot(round_trip).expect("replace");
    let second = host.add_step("math.add", 80.0, 80.0);
    assert_ne!(first, second);
    assert!(host.snapshot.steps.iter().any(|step| step.id == first));
    assert!(host.snapshot.steps.iter().any(|step| step.id == second));
    assert!(host.dag.selected_node_ids().contains(&first));
}

#[semio_framework_async_macros::async_test]
async fn repeated_drops_after_replace_snapshot_use_distinct_ids() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    let first = host.add_step_dropped("math.add", 10.0, 10.0, None);
    let json = host.to_json().expect("fixture json");
    let round_trip: SequenceHostSnapshot = dsl::os_pack::from_json_str(&json).expect("parse");
    host.replace_snapshot(round_trip).expect("replace");
    let second = host.add_step_dropped("math.add", 20.0, 20.0, None);
    assert_ne!(first, second);
    assert_eq!(host.snapshot.steps.iter().filter(|step| step.kind == "math.add").count(), 2);
}

#[semio_framework_async_macros::async_test]
async fn add_step_dropped_targets_expanded_control_slot() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
    let id = host.add_step_dropped("log.print", 600.0, 180.0, Some("step-3"));
    let step = host.snapshot.steps.iter().find(|entry| entry.id == id).expect("added step");
    assert_eq!(step.slot.as_ref().map(|slot| slot.name.as_str()), Some("then"));
}

#[semio_framework_async_macros::async_test]
async fn execution_edges_use_sharp_sz_routing() {
    let host = neural_engine::ColdOwner::new(SequenceHost::default());
    let fixture = host.build_dag_host_snapshot();
    assert!(fixture.edges.iter().all(|edge| edge.route_style == EdgeRouteStyle::SharpSz));
}

#[semio_framework_async_macros::async_test]
async fn set_step_collapsed_toggles_control_step() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
    assert!(host.set_step_collapsed("step-3", true));
    assert!(host.snapshot.steps.iter().find(|step| step.id == "step-3").unwrap().collapsed);
}

#[semio_framework_async_macros::async_test]
async fn set_step_collapsed_rejects_unknown_id() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    assert!(!host.set_step_collapsed("nope", true));
}

#[semio_framework_async_macros::async_test]
async fn set_step_collapsed_rejects_non_control_step() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    assert!(!host.set_step_collapsed("step-1", true));
    assert!(!host.snapshot.steps.iter().find(|step| step.id == "step-1").unwrap().collapsed);
}

#[semio_framework_async_macros::async_test]
async fn remove_step_also_removes_slot_children() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
    host.snapshot.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 560.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
    assert!(host.remove_step("step-3"));
    assert!(!host.snapshot.steps.iter().any(|step| step.id == "step-3" || step.id == "step-4"));
}

#[semio_framework_async_macros::async_test]
async fn remove_step_returns_false_for_unknown_id() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    assert!(!host.remove_step("nope"));
}

#[semio_framework_async_macros::async_test]
async fn set_step_params_json_updates_step_params() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    host.set_step_params_json("step-1", r#"{"key":"renamed"}"#).expect("set params");
    let step = host.snapshot.steps.iter().find(|step| step.id == "step-1").unwrap();
    assert_eq!(step.params.get("key").and_then(|v| v.as_atom()).and_then(|a| a.as_str()), Some("renamed"));
}

#[semio_framework_async_macros::async_test]
async fn set_step_params_json_rejects_unknown_step() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    let err = host.set_step_params_json("nope", "{}").unwrap_err();
    assert!(matches!(err, SequenceCoreError::UnknownStep(id) if id == "nope"));
}

#[semio_framework_async_macros::async_test]
async fn set_step_params_json_rejects_invalid_json() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    let err = host.set_step_params_json("step-1", "not json").unwrap_err();
    assert!(matches!(err, SequenceCoreError::Json(_)));
}

#[semio_framework_async_macros::async_test]
async fn connect_steps_rejects_self_connect() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    assert!(matches!(host.connect_steps("step-1", "step-1").unwrap_err(), SequenceCoreError::SelfConnect));
}

#[semio_framework_async_macros::async_test]
async fn connect_steps_rejects_unknown_from_step() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    assert!(matches!(host.connect_steps("nope", "step-2").unwrap_err(), SequenceCoreError::StepNotFound(id) if id == "nope"));
}

#[semio_framework_async_macros::async_test]
async fn connect_steps_rejects_unknown_to_step() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    assert!(matches!(host.connect_steps("step-1", "nope").unwrap_err(), SequenceCoreError::StepNotFound(id) if id == "nope"));
}

#[semio_framework_async_macros::async_test]
async fn connect_steps_rejects_mismatched_slot_scope() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    host.snapshot.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 560.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
    assert!(matches!(host.connect_steps("step-2", "step-4").unwrap_err(), SequenceCoreError::MismatchedSlotScope));
}

#[semio_framework_async_macros::async_test]
async fn connect_steps_rejects_cycle() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", NeuralValue::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
    host.connect_steps("step-2", "step-3").expect("connect step-2 to step-3");
    assert!(matches!(host.connect_steps("step-3", "step-1").unwrap_err(), SequenceCoreError::CycleDetected));
}

#[semio_framework_async_macros::async_test]
async fn connect_steps_rewires_existing_incoming_edge() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "wait.delay".into(), params: StepParams::new().insert("ms", NeuralValue::Atom(Atom::Decimal(10.0))), x: 560.0, y: 0.0, slot: None, collapsed: false });
    host.connect_steps("step-3", "step-2").expect("rewire onto step-2");
    assert_eq!(host.snapshot.edges.len(), 1);
    assert_eq!(host.snapshot.edges[0].from, "step-3");
    assert_eq!(host.snapshot.edges[0].to, "step-2");
}

#[semio_framework_async_macros::async_test]
async fn disconnect_steps_returns_false_when_no_matching_edge() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    assert!(!host.disconnect_steps("step-2", "step-1"));
    assert_eq!(host.snapshot.edges.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn load_json_parses_valid_fixture() {
    let json = neural_engine::ColdOwner::new(SequenceHost::default()).to_json().expect("fixture json");
    let host = neural_engine::ColdOwner::new(SequenceHost::load_json(&json).expect("load json"));
    assert_eq!(host.snapshot.steps.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn load_json_rejects_unsupported_schema() {
    let result = SequenceHost::load_json(r#"{"schema":"other","steps":[],"edges":[]}"#);
    assert!(matches!(result, Err(SequenceCoreError::UnsupportedSchema(schema)) if schema == "other"));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_json_reports_imperative_catalogue_schema() {
    let host = neural_engine::ColdOwner::new(SequenceHost::default());
    assert!(host.catalogue_json().contains("\"imperative.catalogue\""));
}

#[semio_framework_async_macros::async_test]
async fn layout_expanded_slots_positions_slot_members_relative_to_owner() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
    host.snapshot.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 0.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
    host.layout_expanded_slots();
    let child = host.snapshot.steps.iter().find(|step| step.id == "step-4").unwrap();
    assert_eq!(child.x, 400.0);
    assert_eq!(child.y, 160.0);
}

#[semio_framework_async_macros::async_test]
async fn reorganize_syncs_step_positions_from_dag_layout() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    host.reorganize(&DagLayoutOptions::default()).expect("reorganize");
    for step in &host.snapshot.steps {
        let node = host.dag.host_snapshot.nodes.iter().find(|node| node.id == step.id).expect("node for step");
        assert_eq!(step.x, node.x);
        assert_eq!(step.y, node.y);
    }
}

#[semio_framework_async_macros::async_test]
async fn pick_step_id_at_screen_finds_step_under_cursor() {
    let host = neural_engine::ColdOwner::new(SequenceHost::default());
    let id = host.pick_step_id_at_screen(400.0, 300.0, 800, 600, 1.0);
    assert_eq!(id, Some("step-1".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn pick_step_id_at_screen_returns_none_when_missing_all_nodes() {
    let host = neural_engine::ColdOwner::new(SequenceHost::default());
    let id = host.pick_step_id_at_screen(-9000.0, -9000.0, 800, 600, 1.0);
    assert_eq!(id, None);
}

#[semio_framework_async_macros::async_test]
async fn add_step_dropped_falls_back_when_owner_collapsed() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: true });
    let id = host.add_step_dropped("log.print", 600.0, 180.0, Some("step-3"));
    let step = host.snapshot.steps.iter().find(|entry| entry.id == id).expect("added step");
    assert!(step.slot.is_none());
}

#[semio_framework_async_macros::async_test]
async fn add_step_dropped_falls_back_for_non_control_owner() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    let id = host.add_step_dropped("log.print", 300.0, 0.0, Some("step-2"));
    let step = host.snapshot.steps.iter().find(|entry| entry.id == id).expect("added step");
    assert!(step.slot.is_none());
}

#[semio_framework_async_macros::async_test]
async fn add_step_dropped_falls_back_for_unknown_owner_id() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    let id = host.add_step_dropped("log.print", 300.0, 0.0, Some("nope"));
    let step = host.snapshot.steps.iter().find(|entry| entry.id == id).expect("added step");
    assert!(step.slot.is_none());
}

#[semio_framework_async_macros::async_test]
async fn build_path_returns_unordered_slot_body_when_multiple_heads() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
    host.snapshot.steps.push(SequenceStep { id: "step-4".into(), kind: "log.print".into(), params: StepParams::new(), x: 0.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
    host.snapshot.steps.push(SequenceStep { id: "step-5".into(), kind: "log.print".into(), params: StepParams::new(), x: 280.0, y: 160.0, slot: Some(SlotRef { owner: "step-3".into(), name: "then".into() }), collapsed: false });
    let path = host.build_path();
    let control = path.steps.iter().find(|step| step.id == "step-3").expect("control step");
    let body = control.bodies.get("then").expect("then body");
    assert_eq!(body.steps.len(), 2);
    assert!(body.steps.iter().any(|step| step.id == "step-4"));
    assert!(body.steps.iter().any(|step| step.id == "step-5"));
}

#[semio_framework_async_macros::async_test]
async fn step_to_dag_node_shows_collapsed_indicator_for_collapsed_control_step() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    host.snapshot.steps.push(SequenceStep { id: "step-3".into(), kind: "control.if".into(), params: StepParams::new(), x: 560.0, y: 0.0, slot: None, collapsed: false });
    let expanded = host.step_to_dag_node(&host.snapshot.steps.iter().find(|step| step.id == "step-3").unwrap().clone());
    assert_eq!(expanded.abbreviation, "▾️0");
    host.set_step_collapsed("step-3", true);
    let collapsed = host.step_to_dag_node(&host.snapshot.steps.iter().find(|step| step.id == "step-3").unwrap().clone());
    assert_eq!(collapsed.abbreviation, "▸️0");
}

#[semio_framework_async_macros::async_test]
async fn set_ghost_step_and_clear_ghost_step_toggle_dag_ghost_node() {
    let mut host = neural_engine::ColdOwner::new(SequenceHost::default());
    assert!(host.dag.ghost_node().is_none());
    host.set_ghost_step("math.add", 10.0, 20.0);
    assert!(host.dag.ghost_node().is_some());
    host.clear_ghost_step();
    assert!(host.dag.ghost_node().is_none());
}

#[semio_framework_async_macros::async_test]
async fn run_executes_default_snapshot_and_records_scope() {
    let host = neural_engine::ColdOwner::new(SequenceHost::default());
    let result = host.run();
    assert_eq!(result.scope.get("counter").and_then(|v| v.as_atom()).and_then(|a| a.as_f64()), Some(0.0));
    assert!(!result.effects.is_empty());
    // 🧊️ `run` mints a fresh scope dictionary (and one per effect) that nothing else owns — dropping
    // the result unretired trips `final Dictionary ownership must be explicitly retired or owned by a
    // cold boundary`, which is why the crate publishes its own exact retirement for this shape.
    retire_run_result_cold(result);
}

#[semio_framework_async_macros::async_test]
async fn compile_text_renders_default_snapshot_steps() {
    let host = neural_engine::ColdOwner::new(SequenceHost::default());
    let text = host.compile_text();
    assert!(text.contains("state.set"));
    assert!(text.contains("log.print"));
}

#[semio_framework_async_macros::async_test]
async fn compiled_wire_literal_includes_step_ids() {
    let host = neural_engine::ColdOwner::new(SequenceHost::default());
    let literal = host.compiled_wire_literal();
    assert!(literal.contains("step-1"));
    assert!(literal.contains("step-2"));
}

#[semio_framework_async_macros::async_test]
async fn sequence_io_declares_the_steps_in_port() {
    let io = sequence_io();
    assert_eq!(io.artifact_schema, SEQUENCE_DOCUMENT_SCHEMA);
    assert_eq!(io.ports.len(), 1);
    let port = &io.ports[0];
    assert_eq!(port.id, "steps:in");
    assert_eq!(port.direction, semio_framework::MediaPortDirection::In);
    assert_eq!(port.multiplicity, semio_framework::PortMultiplicity::Many);
    assert!(!port.required);
}

#[semio_framework_async_macros::async_test]
async fn next_available_step_id_is_free_and_deterministic() {
    let fixture = neural_engine::ColdOwner::new(default_snapshot());
    let id = next_available_step_id(&fixture);
    assert!(!fixture.to_host_snapshot().steps.iter().any(|step| step.id == id));
    assert_eq!(id, next_available_step_id(&fixture), "pure function of the fixture, not a mutating counter");
}
//#endregion 🔖️HostTests
