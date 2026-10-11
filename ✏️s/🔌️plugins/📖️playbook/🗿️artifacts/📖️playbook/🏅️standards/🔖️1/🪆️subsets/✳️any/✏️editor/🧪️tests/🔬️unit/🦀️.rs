pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, new_app_with_registry_and_members, project_and_retire_fixture_tree, settle_history_verb, settle_registered_typed_operation};
    use semio_framework_plugin::{App, EditorApp, Effect, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type MountedPlaybookApp = VcsArtifactApp<EditorApp<PlaybookPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>;

    /// 🔒️ A mounted, self-closing playbook app: bound to the `meta("local")` instance the way the live
    /// host binds it before its first dispatch (`interactive-job.live-instance`), and retired through the
    /// bounded close protocol on drop (`artifact store reached Drop without its exact terminal-empty
    /// shallow-shell witness`). A panicking test leaves the witness alone so its first failure stays reported.
    pub struct PlaybookApp(MountedPlaybookApp);

    impl std::ops::Deref for PlaybookApp {
        type Target = MountedPlaybookApp;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl std::ops::DerefMut for PlaybookApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Drop for PlaybookApp {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                close_registered_fixture_app(&mut self.0);
            }
        }
    }

    /// 🧪️ A bound app instance with its concrete command registry, retained job proofs and composed member roster.
    pub async fn playbook_app() -> PlaybookApp {
        let mut app = new_app_with_registry_and_members::<EditorApp<PlaybookPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(playbook_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
        app.bind_instance_id(meta("local").instance_id).await;
        PlaybookApp(app)
    }

    /// 🧪️ Adapts `create_playbook_play_app`'s `AppDefinition` (contract §2.4) into the `App {
    /// definition, examples }` shape the registry-backed fixture constructors expect.
    pub fn playbook_manifest_for_tests() -> App {
        App { definition: create_playbook_play_app(), examples: Vec::new() }
    }

    semio_framework_plugin::history_edit_acceptance_law!("playbook", PlaybookPlayApp, playbook_manifest_for_tests, "../..");
    semio_framework_plugin::composed_reload_law!("playbook", PlaybookPlayApp, playbook_manifest_for_tests, "../..");
    semio_framework_plugin::composed_child_history_law!("playbook", PlaybookPlayApp, playbook_manifest_for_tests, [("addStep", "{}"), ("addBlock", r#"{"kind":"number"}"#)]);

    /// 🧩️ The LIVE playbook, composed from the parent projection and the `flow` CHILD store — the surface every step/block verb
    /// publishes on (the parent only carries the child's coordinate, design §20.15).
    pub async fn live_spec(app: &MountedPlaybookApp) -> crate::PlaybookSpec {
        use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;
        use store::{ArtifactPack, SpaceMember};
        let snapshot = app.snapshot().expect("playbook parent projection");
        let bytes = app.child_store("flow", &snapshot.flow.child_id).await.expect("playbook flow child").document_pack_bytes().await.expect("playbook flow child pack");
        let content = SemioFlowSnapshot::decode_pack(&bytes).expect("playbook flow child snapshot");
        crate::PlaybookSpec { schema: snapshot.schema.clone(), id: snapshot.id.clone(), version: snapshot.version.clone(), title: snapshot.title.clone(), steps: crate::steps_from_flow_content(&content).expect("decodable playbook steps") }
    }

    /// 🚚️ Dispatches one typed command, settles its retained operation and applies any `LoadDocument`
    /// effect exactly as the host would; a mounted app's `result.mutations` is always empty.
    pub async fn dispatch(app: &mut PlaybookApp, command: PlaybookCommand) -> InvocationResult {
        let mut result = app.dispatch_typed(command, &meta("local")).await.expect("dispatch");
        let receipt = settle_registered_typed_operation(&mut app.0, meta("local").instance_id).await.expect("retained playbook operation settles");
        result.requested_effects.extend(receipt.effects);
        for effect in &result.requested_effects {
            if let Effect::LoadDocument { pack, spr } = effect {
                let files = store::ArtifactPackFiles { pack: pack.clone(), spr: spr.clone(), ops: String::new() };
                semio_framework_plugin::artifact_app_laws::load_document(app, &files).await.expect("test host applies load-document effect");
            }
        }
        result
    }

    /// ↩️ Runs a framework-reserved history verb (`undo`/`redo`) through admission, commit and publication.
    pub async fn history_verb(app: &mut PlaybookApp, action: &str) {
        settle_history_verb(&mut app.0, action, meta("local").instance_id).await;
    }

    pub async fn render(app: &mut PlaybookApp, body_key: &str) -> String {
        project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render")).expect("render json")
    }
}

use super::*;
use crate::editor::playbook::unit_tests::context::{dispatch, live_spec, playbook_app};
use semio_framework_plugin::artifact_app_laws;
use semio_framework_plugin::{MediaClass, MediaForm, PluginApp};

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
    assert_eq!(ids.len(), 9, "every PlaybookCommand row must be covered by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
/// pre-migration `playbook_protocol::PlaybookCommand`'s own `#[dsl(key = ..)]` attribute) so the wire
/// format stays byte-identical across the migration; see TEMPLATE.md §5.1.
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    for command in every_command() {
        let id = command.command_id();
        let expected = match id {
            "setContributions" => "contributions".to_string(),
            "setActiveExample" => "active-example".to_string(),
            _ => id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect(),
        };
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
    }
}

/// 🌐️ The language-neutral job catalog has the same row order through an owned minimal
/// parser and the test-only third-party JSON oracle, matches the schema-generated command enum, and
/// names exactly the retained tool roster.
#[semio_framework_async_macros::async_test]
async fn interactive_job_catalog_matches_owned_and_json_oracle_projections() {
    let source = include_str!("../../🧫️fixtures/🧫️interactive-jobs/🔣️.json");
    let owned = source.lines().filter_map(|line| line.trim().strip_prefix("{\"id\":\"").and_then(|tail| tail.split_once('"')).map(|(id, _)| id.to_string())).collect::<Vec<_>>();
    let oracle = serde_json::from_str::<serde_json::Value>(source)
        .expect("language-neutral interactive job fixture")
        .get("tools")
        .and_then(serde_json::Value::as_array)
        .expect("tool rows")
        .iter()
        .map(|row| row.get("id").and_then(serde_json::Value::as_str).expect("tool id").to_string())
        .collect::<Vec<_>>();
    let generated = every_command().into_iter().map(|command| command.command_id().to_string()).collect::<Vec<_>>();
    assert_eq!(owned, oracle);
    assert_eq!(owned, generated);
    let mut retained = PLAYBOOK_RETAINED_TOOL_IDS.iter().map(|id| id.to_string()).collect::<Vec<_>>();
    let mut catalog = owned;
    retained.sort_unstable();
    catalog.sort_unstable();
    assert_eq!(catalog, retained, "every playbook verb is a retained Migrated tool");
}

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
pub(super) fn every_command() -> Vec<PlaybookCommand> {
    vec![
        PlaybookCommand::AddStep(add_step::AddStep {}),
        PlaybookCommand::RemoveStep(remove_step::RemoveStep { step_id: "s".into() }),
        PlaybookCommand::MoveStep(move_step::MoveStep { step_id: "s".into(), index: 2 }),
        PlaybookCommand::AddBlock(add_block::AddBlock { kind: "text".into(), step_id: None }),
        PlaybookCommand::RemoveBlock(remove_block::RemoveBlock { step_id: "s".into(), block_id: "b".into() }),
        PlaybookCommand::MoveBlock(move_block::MoveBlock { block_id: "b".into(), from_step_id: "s1".into(), to_step_id: "s2".into(), index: 0 }),
        PlaybookCommand::UpdatePlaybook(update_playbook::UpdatePlaybook { value: "Recipe".into() }),
        PlaybookCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
        PlaybookCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: crate::examples::demo::ID.into() }),
    ]
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_playbook_play_app()).expect("app definition json");
    for window in [PLAYBOOK_PLAY_WINDOW_BUILDER, PLAYBOOK_PLAY_WINDOW_STEPS, PLAYBOOK_PLAY_WINDOW_CHANGES, PLAYBOOK_PLAY_WINDOW_ACTIVITY, PLAYBOOK_PLAY_WINDOW_SOURCE, PLAYBOOK_PLAY_WINDOW_FILES] {
        assert!(json.contains(window), "window kind missing from the manifest: {window}");
    }
    assert!(json.contains(builder::PLAYBOOK_PLAY_MODE_BUILDER), "mode missing from the manifest");
    assert!(json.contains("text.playbook"), "artifact kind missing from the manifest");
}

#[semio_framework_async_macros::async_test]
async fn playbook_play_app_declares_the_authored_scene_showcase() {
    let definition = create_playbook_play_app();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🎭️modes/🏗️builder/🧫️fixtures/🎬️scene-showcase/🔣️.json")).unwrap();
    let actual: Vec<_> = definition.window_kinds.iter().map(|window| serde_json::json!({ "windowId": window.id, "bodyKey": window.body_key, "surfaceKind": window.surface_kind })).collect();
    assert_eq!(serde_json::json!(actual), fixture["windows"]);
}
//#endregion 🔖️ManifestSanity

//#region 🔖️Interaction
#[semio_framework_async_macros::async_test]
async fn blocks_interaction_domain_is_declared_topology_pick_only_on_the_builder_window() {
    let definition = create_playbook_play_app();
    let domain = definition.interactions.iter().find(|interaction| interaction.id == PLAYBOOK_INTERACTION_BLOCKS).expect("blocks interaction domain declared");
    assert!(matches!(domain.hierarchy, HierarchyProvider::Topology));
    assert_eq!(domain.selection.methods, vec![SelectionMethod::Pick]);
    assert!(!domain.selection.transitive, "selecting a step must not implicitly select its blocks — no pre-migration code ever did that");
    let builder_window = definition.window_kinds.iter().find(|window| window.id == PLAYBOOK_PLAY_WINDOW_BUILDER).expect("builder window declared");
    assert!(builder_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == PLAYBOOK_INTERACTION_BLOCKS), "builder window must reference the blocks interaction domain");
}

/// 🌳️ `interaction_topology` walks every step and every one of its blocks of the COMPOSED playbook into a `TopologyNode`, so
/// `validate_state` can prune a deleted step's OR block's id out of a stale selection.
#[semio_framework_async_macros::async_test]
async fn interaction_topology_covers_every_step_and_block() {
    let mut app = playbook_app().await;
    dispatch(&mut app, PlaybookCommand::AddStep(add_step::AddStep {})).await;
    let step_id = live_spec(&app).await.steps[1].id.clone();
    dispatch(&mut app, PlaybookCommand::AddBlock(add_block::AddBlock { kind: "text".into(), step_id: Some(step_id.clone()) })).await;
    let block_id = live_spec(&app).await.steps[1].blocks[0].id.clone();
    let spec = app.snapshot().expect("projection");
    let history = semio_framework_plugin::HistoryView::empty();
    let cfg = PlaybookConfig::default();
    let doc = ArtifactView::with_children(&spec, &history, app.test_child_content_view());
    let topology = PlaybookPlayApp::interaction_topology(&doc, &ConfigView { snapshot: &cfg, window: None }).expect("composed interaction topology");
    let blocks = topology.domains.get(PLAYBOOK_INTERACTION_BLOCKS).expect("blocks domain present in topology");
    assert!(blocks.ordered.iter().any(|node| node.id == step_id && node.granularity == PLAYBOOK_INTERACTION_GRANULARITY_STEP && node.parent.is_none()));
    assert!(blocks.ordered.iter().any(|node| node.id == block_id && node.granularity == PLAYBOOK_INTERACTION_GRANULARITY_BLOCK && node.parent.as_deref() == Some(step_id.as_str())));
}

/// 🚫️ An uncomposed `flow` child is a named fault, never an empty topology read through the handle.
#[test]
fn interaction_topology_refuses_an_uncomposed_flow_child() {
    let spec = crate::empty_playbook_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let cfg = PlaybookConfig::default();
    let error = PlaybookPlayApp::interaction_topology(&ArtifactView::new(&spec, &history), &ConfigView { snapshot: &cfg, window: None }).expect_err("no composed child");
    assert!(error.to_string().contains("playbook.flow.unavailable"), "{error}");
}
//#endregion 🔖️Interaction

//#region 🔖️CrossCutting
/// 👥️ `setContributions` is the FIRST command `#playbook` dispatches at boot, and it died with
/// `playbook presence local read requires a live exact local retirement owner` in the play grid's
/// strict acceptance (ticket 26/09/19). A presence DISPOSER is not a presence retirement OWNER:
/// `PresenceStore::local_read` fails closed while `local_retirement_factory` is `None`, so every
/// command whose ephemeral leg reads local presence is refused however correct the command is.
/// This drives the real verb through the real registered app, so the law fails if the owners are
/// ever dropped again — a declaration-only assertion would not catch a regression in `local_read`.
#[semio_framework_async_macros::async_test]
async fn set_contributions_boots_because_presence_declares_its_retirement_owners() {
    use semio_framework_plugin::ArtifactApp;

    assert!(
        <EditorApp<PlaybookPlayApp> as ArtifactApp>::build_presence_local_root_retirement_factory().is_some(),
        "playbook must declare a local presence retirement owner or every presence-reading command fails closed"
    );
    assert!(
        <EditorApp<PlaybookPlayApp> as ArtifactApp>::build_presence_peer_retirement_factory().is_some(),
        "playbook must declare a peer presence retirement owner"
    );

    let mut app = playbook_app().await;
    let outcome = app.dispatch_typed(PlaybookCommand::SetContributions(set_contributions::SetContributions { json: "{}".to_string() }), &artifact_app_laws::meta("local")).await;
    assert!(outcome.is_ok(), "setContributions must not be refused at boot: {:?}", outcome.err());
    artifact_app_laws::settle_registered_typed_operation(&mut *app, artifact_app_laws::meta("local").instance_id).await.expect("setContributions publishes");
}

/// ↩️ Spelled out over the `flow` child (the law helper's probe is a synchronous closure over the parent projection, which
/// carries no steps): undo retires the child-lane group, redo reapplies it.
#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trip_through_the_wrapper() {
    use crate::editor::playbook::unit_tests::context::history_verb;
    let mut app = playbook_app().await;
    assert_eq!(live_spec(&app).await.steps.len(), 1);
    dispatch(&mut app, PlaybookCommand::AddStep(add_step::AddStep {})).await;
    assert_eq!(live_spec(&app).await.steps.len(), 2, "addStep must land one step in the flow child");
    history_verb(&mut app, "undo").await;
    assert_eq!(live_spec(&app).await.steps.len(), 1, "undo must retire the child-lane group");
    history_verb(&mut app, "redo").await;
    assert_eq!(live_spec(&app).await.steps.len(), 2, "redo must reapply the child-lane group");
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
    use crate::editor::playbook::unit_tests::context::render;
    let mut app = playbook_app().await;
    assert!(render(&mut app, "playbook.play.nope").await.contains("Unknown body"));
}

/// 🧪️ The definitional proof: two independent registered instances start from the same document, apply DISJOINT child-lane
/// edits (A adds a step, B adds a block to the genesis step), and exchanging operations over a backbone converges both sides —
/// measured on the `flow` child, since neither edit touches the parent projection.
#[semio_framework_async_macros::async_test]
async fn two_instances_converge_disjoint_edits_via_backbone() {
    use semio_framework_plugin::artifact_app_laws::{meta, settle_registered_typed_operation};
    let mut instance_a = playbook_app().await;
    let mut instance_b = playbook_app().await;
    let (backbone_a, backbone_b) = store::MemoryBackbone::pair("mem://playbook-convergence", "mem://playbook-convergence").await;
    instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
    instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");
    let receiver = meta("actor-a").instance_id;
    instance_a.dispatch_typed(PlaybookCommand::AddStep(add_step::AddStep {}), &meta("actor-a")).await.expect("a applies its edit");
    settle_registered_typed_operation(&mut *instance_a, receiver).await.expect("a's edit publishes");
    instance_b.dispatch_typed(PlaybookCommand::AddBlock(add_block::AddBlock { kind: "number".into(), step_id: None }), &meta("actor-b")).await.expect("b applies its edit");
    settle_registered_typed_operation(&mut *instance_b, receiver).await.expect("b's edit publishes");
    instance_a.tick_backbone().await.expect("a folds b's events");
    instance_b.tick_backbone().await.expect("b folds a's events");
    let shape = |spec: crate::PlaybookSpec| spec.steps.iter().map(|step| (step.id.clone(), step.blocks.len())).collect::<Vec<_>>();
    let converged = shape(live_spec(&instance_a).await);
    assert_eq!(converged, shape(live_spec(&instance_b).await), "both instances must converge on the same composed playbook");
    assert_eq!(converged.len(), 2);
    assert_eq!(converged[0].1, 1, "b's block lands in the genesis step on both sides");
    instance_a.detach_backbone().await.expect("a releases its backbone");
    instance_b.detach_backbone().await.expect("b releases its backbone");
}
//#endregion 🔖️CrossCutting

//#region 🔖️PortTests
#[semio_framework_async_macros::async_test]
async fn playbook_io_declares_the_extra_chapters_in_port_and_its_own_kind() {
    let io = playbook_io();
    assert_eq!(io.artifact.id, "text.playbook");
    let ports = io.all_ports().await;
    let chapters_in = ports.iter().find(|port| port.id == "chapters:in").expect("chapters:in declared");
    assert_eq!(chapters_in.kind_id.as_deref(), Some("text.document"));
}

fn chapter_media(text: &str, title: &str) -> Media {
    let payload = PlaybookChapterPayload { id: "jack".into(), title: title.into(), text: text.into(), language_id: "jack".into() };
    Media { media_type: semio_framework_plugin::MediaType { class: MediaClass::Text, form: MediaForm::Document }, payload: MediaPayload::Structured { schema: "text.document".into(), json: semio_framework_pack_json::to_json_string(&payload) } }
}

/// 🎞️ The first import creates the `imported` step WITH its note block as ONE `flow` child edit (node + chain edge); the
/// reuse branch is the root's `playbook_add_block_leaves` (vectors in the artifact root's tests).
#[semio_framework_async_macros::async_test]
async fn import_media_creates_the_imported_step_on_the_flow_child() {
    let app = playbook_app().await;
    let spec = app.snapshot().expect("projection");
    let history = semio_framework_plugin::HistoryView::empty();
    let mut emit = PlaybookPlayApp::import_media("chapters:in", &chapter_media("MATCH (a) RETURN a", "Jack Query"), &ArtifactView::with_children(&spec, &history, app.test_child_content_view())).expect("import chapters:in");
    let mut prepared=false;
    for _ in 0..4096{
        match emit.prepare_child_one(1,65536).expect("bounded real child preparation"){
            semio_framework_plugin::app::ChildEmitPreparationStep::Ready=>{prepared=true;break;},
            semio_framework_plugin::app::ChildEmitPreparationStep::Pending=>{},
            semio_framework_plugin::app::ChildEmitPreparationStep::Refused(fault)=>panic!("actual fixture child refused: {}",fault.message),
        }
    }
    assert!(prepared,"closed fixture child prefix must complete within its authored bound");
    assert!(emit.artifact_mutations.is_empty(), "an import never writes the parent lane");
    assert_eq!(emit.child_emits.len(), 1);
    assert_eq!((emit.child_emits[0].slot.as_str(), emit.child_emits[0].child_id.as_str()), ("flow", spec.flow.child_id.as_str()));
    assert_eq!(emit.child_emits[0].ops.len(), 2, "one node insert and one chain edge");
    assert_eq!(emit.child_emits[0].op_schema.0, "node.insert-node");
}

#[semio_framework_async_macros::async_test]
async fn import_media_rejects_unknown_ports_and_malformed_payloads() {
    let spec = crate::empty_playbook_snapshot();
    let history = semio_framework_plugin::HistoryView::empty();
    let doc_view = ArtifactView::new(&spec, &history);
    assert!(matches!(PlaybookPlayApp::import_media("nonsense:in", &chapter_media("x", "y"), &doc_view), Err(MediaError::NotImplemented)));
    let bad_media = Media { media_type: semio_framework_plugin::MediaType { class: MediaClass::Text, form: MediaForm::Document }, payload: MediaPayload::Structured { schema: "text.document".into(), json: "not json".into() } };
    assert!(matches!(PlaybookPlayApp::import_media("chapters:in", &bad_media, &doc_view), Err(MediaError::Payload(..))));
    assert!(matches!(PlaybookPlayApp::import_media("chapters:in", &chapter_media("x", "y"), &doc_view), Err(MediaError::Payload(..))), "an uncomposed flow child refuses the import");
}
//#endregion 🔖️PortTests

//#region 🧵️RetainedToolCatalog
/// 🧾️ The exact three-way join the guest checks at boot, plus the store ownership a published edit
/// needs. playbook declared only `setContributions` retained — its six structural verbs were
/// `BatchOnlyPendingRewrite` and hard dead in the shell — and declared no store owners at all, so
/// the first real document publication answered `returned snapshot read requires its exact
/// owned-snapshot retirement factory` (ticket 26/09/18 slice B2b). The disposer is `mem::forget`ed
/// rather than dropped: a live `ArtifactStoreCursorDisposer` fails closed in `Drop` unless it has
/// been driven to terminal-empty through a real store, which an existence check has none of.
#[test]
fn every_retained_tool_id_is_migrated_contracted_and_backed_by_store_owners() {
    use semio_framework_plugin::{ArtifactEditor, ArtifactOwnedToolJobFactory};
    assert_eq!(PLAYBOOK_RETAINED_TOOL_IDS.len(), PLAYBOOK_RETAINED_PUBLICATION_CONTRACTS.len());
    for tool_id in PLAYBOOK_RETAINED_TOOL_IDS {
        assert!(PLAYBOOK_RETAINED_PUBLICATION_CONTRACTS.iter().any(|contract| contract.tool_id == *tool_id), "{tool_id} has no publication contract");
    }
    assert_eq!(<PlaybookRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::TOOL_IDS, PLAYBOOK_RETAINED_TOOL_IDS);
    let definition = create_playbook_play_app();
    for window in definition.window_kinds.iter() {
        for action in window.actions.iter().filter(|action| PLAYBOOK_RETAINED_TOOL_IDS.contains(&action.id.as_str())) {
            assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{} is retained but not Migrated", action.id);
        }
    }
    assert!(PlaybookPlayApp::build_artifact_store_one_item_preparation_factory().is_some(), "an Artifact-lane retained tool needs an artifact-lane preparation authority");
    assert!(PlaybookPlayApp::document_store_owners_source_demands().is_ok(), "a published artifact edit needs its exact owned-snapshot retirement factory");
    std::mem::forget(PlaybookPlayApp::build_document_store_disposer().expect("the document store needs its exact owned disposer"));
    assert!(PlaybookPlayApp::config_store_owners_source_demands().is_ok(), "the config lane needs its exact owners");
    std::mem::forget(PlaybookPlayApp::build_config_store_disposer().expect("the config store needs its exact owned disposer"));
}

/// 🌉️ The `{action,args}` bridge resolves every declared verb; without it the trait default refused
/// every id with `app.command.unsupported` and no Builder palette row could reach `dispatch`.
#[test]
fn command_from_action_resolves_every_declared_verb() {
    use semio_framework_plugin::ArtifactEditor;
    for tool_id in PLAYBOOK_RETAINED_TOOL_IDS {
        let command = PlaybookPlayApp::command_from_action(tool_id, None).unwrap_or_else(|error| panic!("{tool_id} has no bridge: {error:?}"));
        assert_eq!(command.command_id(), *tool_id);
    }
    assert!(PlaybookPlayApp::command_from_action("thereIsNoSuchVerb", None).is_err());
}

/// 🧬️ The example picker's whole-document load: a `HostOnly` retained verb whose demo parent names the ONE composed `flow`
/// child by a stable id the plugin's own catalogue answers (its genesis pack), so the archive's closure leg admits the load; a
/// foreign id is never answered. Six structural verbs publish on the `Child` lane only.
#[test]
fn set_active_example_is_host_only_and_the_flow_child_mints_its_genesis_pack() {
    use semio_framework_plugin::ArtifactEditor;
    assert!(PLAYBOOK_RETAINED_TOOL_IDS.contains(&"setActiveExample"), "the example verb must be a retained tool or the dispatch gate refuses it");
    let contract = PLAYBOOK_RETAINED_PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == "setActiveExample").expect("setActiveExample has a publication contract");
    assert_eq!(contract.lanes, &[ArtifactToolPublicationLane::HostOnly], "a whole-document load publishes through neither the artifact nor the config store");
    for verb in ["addStep", "removeStep", "moveStep", "addBlock", "removeBlock", "moveBlock"] {
        let contract = PLAYBOOK_RETAINED_PUBLICATION_CONTRACTS.iter().find(|contract| contract.tool_id == verb).expect("structural verb contract");
        assert_eq!(contract.lanes, &[ArtifactToolPublicationLane::Child], "{verb} edits the flow child only (design §20.15)");
    }
    let definition = create_playbook_play_app();
    let action = definition.actions.iter().find(|action| action.id == "setActiveExample").expect("setActiveExample is declared on the app roster");
    assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
    let demo = <PlaybookSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).expect("the demo asset parses as a playbook snapshot");
    assert_eq!(demo.flow.child_id, crate::examples::demo::FLOW_ID);
    assert_eq!(demo.flow.target.artifact_id, demo.flow.child_id, "the target must name its own child_id or ChildRestoreProjection refuses the whole load");
    let projection = PlaybookPlayApp::child_restore_projection(&demo).expect("the demo's loaded-parent child projection");
    assert!(projection.admits_member("flow", &demo.flow.target));
    let pack = PlaybookPlayApp::genesis_child_pack(&demo, "flow", &demo.flow.child_id).expect("valid genesis owner").expect("the demo flow child mints its genesis pack");
    let content = <semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot as store::ArtifactPack>::decode_pack(&pack).expect("the genesis pack decodes");
    assert_eq!(crate::steps_from_flow_content(&content).expect("decodable demo steps").len(), 3);
    assert!(PlaybookPlayApp::genesis_child_pack(&demo, "flow", "not-this-documents-child").expect("valid genesis owner").is_none(), "a foreign child id must not be answered");
    assert!(PlaybookPlayApp::genesis_child_pack(&demo, "document", &demo.flow.child_id).expect("valid genesis owner").is_none(), "the deleted document slot is never answered");
}

/// 📣️ Every refusal code this artifact raises has an en and de notice (design §20.12).
#[test]
fn every_playbook_refusal_code_has_a_localized_notice() {
    use semio_framework_plugin::ArtifactEditor;
    let notices = PlaybookPlayApp::fault_notices();
    assert_eq!(notices.len(), 7);
    for (code, label) in notices {
        assert!(code.starts_with("playbook.") && code.split('.').count() == 3, "{code} is a three-segment playbook code");
        for locale in [semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Locale::De] {
            assert!(!label.resolve(semio_framework_ui_locale::Terminology::Native, locale).is_empty(), "{code} has a {locale:?} notice");
        }
    }
}
//#endregion 🧵️RetainedToolCatalog
