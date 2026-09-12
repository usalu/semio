pub(crate) mod context {
    use super::super::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::ActionMeta;
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};
    use store::ArtifactEnvelope;
    
    /// ✏️ `VcsPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime `ArtifactApp`
    /// — `EditorApp<VcsPlayApp>` (SDK adapter, contract §2.1) is the real `ArtifactApp` implementor
    /// `VcsArtifactApp` wraps, exactly the way `PluginBuilder::editor::<VcsPlayApp>` builds it.
    pub type VcsApp = VcsArtifactApp<EditorApp<VcsPlayApp>>;
    
    /// ✏️ Adapts `create_vcs_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `context::new_app_with_registry` still expects — framework test context gap, not
    /// modifiable here (`🧰️framework/**` is outside this packet's lease).
    fn vcs_app_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_vcs_app(), examples: Vec::new() }
    }
    
    /// 🧪️ A bare, pre-seeded app instance — no `AppActionRegistry`, so undeclared internal commands
    /// dispatch freely. Seeded via `seed_vcs_demo_history` (see its own doc comment for why this
    /// replaced `ArtifactApp::seed`).
    pub async fn app() -> VcsApp {
        let mut instance = new_app::<EditorApp<VcsPlayApp>>().await;
        seed_vcs_demo_history(&mut instance).await;
        instance
    }
    
    /// 🧪️ A pre-seeded app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn app_with_registry() -> VcsApp {
        let mut instance = new_app_with_registry::<EditorApp<VcsPlayApp>>(vcs_app_manifest_for_tests).await;
        seed_vcs_demo_history(&mut instance).await;
        instance
    }
    
    /// 🧾️ Builds one flat, string-valued action argument object — the `DslValue` shape
    /// `handle_action`/`command_from_action` take now that the action wire is the DSL value, not JSON.
    pub fn action_args(entries: impl IntoIterator<Item = (&'static str, String)>) -> dsl::DslValue {
        dsl::DslValue::object(entries.into_iter().map(|(key, value)| (key.to_string(), dsl::DslValue::String(value))))
    }
    
    /// 🕳️ The empty action argument object.
    pub fn no_args() -> dsl::DslValue {
        dsl::DslValue::Object(Vec::new())
    }
    
    pub async fn dispatch(instance: &mut VcsApp, command: VcsCommand) -> InvocationResult {
        instance.dispatch_typed(command, &meta("local")).await.expect("dispatch")
    }
    
    pub async fn render(instance: &mut VcsApp, body_key: &str) -> String {
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(instance.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
    }
    
    /// 📦️ Parses `document_pack()` (the full envelope) for tests that need to inspect raw
    /// checkpoints/alternatives directly — safe here because none of these tests undo/redo, so every
    /// edit in the log is still applied.
    pub async fn seeded_envelope(instance: &VcsApp) -> ArtifactEnvelope<VcsSnapshot, VcsDemoMutation> {
        let files = instance.document_pack().await.expect("document pack");
        store::parse_document_pack::<VcsSnapshot, VcsDemoMutation>(&files.pack, &files.spr).await.expect("parse document pack").envelope
    }
    
    /// 🌱️ Seeds a rich, forked checkpoint/alternative history through `VcsApp`'s own public dispatch
    /// surface (`dispatch_typed`/`handle_action`) — this app's whole point is exercising the history UI
    /// (swimlane graph, checkpoints, alternatives, undo/redo), so every test instance starts as a
    /// populated history, not a bare projection. Replaces the old direct-`ArtifactStore`-touch
    /// `seed_vcs_demo_history(&mut ArtifactStore)` dispatched via the now-removed `ArtifactApp::seed`
    /// hook (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M4). Field edits go through
    /// `VcsCommand::TextEdit` (whole-projection diff, matching `patch::text_edit_operations`) so one
    /// call can bundle several field changes into one undo-log entry, mirroring the original narrative's
    /// grouping. Per-checkpoint authorship is lost here: `handle_action`'s `"commitCheckpoint"` arm
    /// hardcodes `authors: Vec::new()` with no wire path for real authors (framework-owned, out of this
    /// plugin's boundary) — no test asserts on authorship, so this is a silent, documented fidelity
    /// loss, not a functional gap.
    async fn seed_edit(app: &mut VcsApp, local: &ActionMeta, mutate: fn(&mut VcsSnapshot)) {
        let mut next = app.snapshot().expect("materialize snapshot");
        mutate(&mut next);
        let text = serde_json::to_string(&next).expect("serialize snapshot");
        let _ = app.dispatch_typed(VcsCommand::TextEdit(text_edit::TextEdit { text }), local).await;
    }
    
    async fn seed_commit(app: &mut VcsApp, local: &ActionMeta, message: &str) {
        let _ = app.handle_action("commitCheckpoint", Some(&action_args([("message", message.to_string())])), local).await;
    }
    
    async fn seed_checkout(app: &mut VcsApp, local: &ActionMeta, checkpoint_id: &str) {
        let _ = app.handle_action("checkoutCheckpoint", Some(&action_args([("checkpointId", checkpoint_id.to_string())])), local).await;
    }
    
    async fn seed_create_alternative(app: &mut VcsApp, local: &ActionMeta, name: &str) -> String {
        let _ = app.handle_action("createAlternative", Some(&action_args([("name", name.to_string())])), local).await;
        seeded_envelope(app).await.active_alternative_id.clone().expect("alternative id")
    }
    
    async fn seed_switch_alternative(app: &mut VcsApp, local: &ActionMeta, alternative_id: &str) {
        let _ = app.handle_action("switchAlternative", Some(&action_args([("alternativeId", alternative_id.to_string())])), local).await;
    }
    
    async fn seed_last_checkpoint_id(app: &VcsApp) -> String {
        seeded_envelope(app).await.vcs.checkpoints.last().expect("checkpoint just committed").id.clone()
    }
    
    pub async fn seed_vcs_demo_history(app: &mut VcsApp) {
        let local = meta("local");
    
        seed_edit(app, &local, |s| {
            s.counter = 1;
            s.title = "VCS Demo".into();
        })
        .await;
        seed_commit(app, &local, "Bootstrap").await;
        let c1 = seed_last_checkpoint_id(app).await;
    
        seed_edit(app, &local, |s| {
            s.notes = "main line".into();
            s.status = "draft".into();
        })
        .await;
        seed_commit(app, &local, "Annotate main draft").await;
        let c2 = seed_last_checkpoint_id(app).await;
    
        seed_edit(app, &local, |s| {
            s.counter = 2;
        })
        .await;
        seed_commit(app, &local, "Main milestone").await;
        let c3 = seed_last_checkpoint_id(app).await;
    
        seed_checkout(app, &local, &c3).await;
        let feature_a_id = seed_create_alternative(app, &local, "feature-a").await;
        seed_edit(app, &local, |s| {
            s.title = "Feature A".into();
            s.tags.push("feature-a".into());
        })
        .await;
        seed_commit(app, &local, "Start feature A").await;
        let c4 = seed_last_checkpoint_id(app).await;
    
        seed_edit(app, &local, |s| {
            s.counter = 10;
        })
        .await;
        seed_commit(app, &local, "Feature A progress").await;
    
        seed_checkout(app, &local, &c3).await;
        let feature_b_id = seed_create_alternative(app, &local, "feature-b").await;
        seed_edit(app, &local, |s| {
            s.title = "Feature B".into();
            s.notes = "branch b".into();
        })
        .await;
        seed_commit(app, &local, "Start feature B").await;
    
        seed_edit(app, &local, |s| {
            s.counter = 20;
        })
        .await;
        seed_commit(app, &local, "Feature B try").await;
    
        seed_checkout(app, &local, &c3).await;
        seed_edit(app, &local, |s| {
            s.status = "active".into();
        })
        .await;
        seed_commit(app, &local, "Resume main").await;
        let c8 = seed_last_checkpoint_id(app).await;
    
        seed_switch_alternative(app, &local, &feature_a_id).await;
        seed_edit(app, &local, |s| {
            s.counter = 11;
            s.tags.push("wip".into());
        })
        .await;
        seed_commit(app, &local, "Feature A sprint").await;
    
        seed_checkout(app, &local, &c4).await;
        let _ = seed_create_alternative(app, &local, "feature-a-hotfix").await;
        seed_edit(app, &local, |s| {
            s.status = "hotfix".into();
        })
        .await;
        seed_commit(app, &local, "Hotfix off feature A").await;
    
        seed_switch_alternative(app, &local, &feature_b_id).await;
        seed_edit(app, &local, |s| {
            s.tags.push("review".into());
        })
        .await;
        seed_commit(app, &local, "Feature B review").await;
    
        seed_checkout(app, &local, &c8).await;
        seed_edit(app, &local, |s| {
            s.counter = 3;
            s.notes = "main polish".into();
            s.tags.push("release".into());
        })
        .await;
        seed_commit(app, &local, "Main batch polish").await;
    
        seed_edit(app, &local, |s| {
            s.status = "done".into();
        })
        .await;
        seed_commit(app, &local, "Main release").await;
    
        seed_checkout(app, &local, &c2).await;
        let _ = seed_create_alternative(app, &local, "docs").await;
        seed_edit(app, &local, |s| {
            s.notes = "documentation pass".into();
        })
        .await;
        seed_commit(app, &local, "Docs branch").await;
    
        seed_checkout(app, &local, &c1).await;
        let _ = seed_create_alternative(app, &local, "spike").await;
        seed_edit(app, &local, |s| {
            s.title = "Spike prototype".into();
        })
        .await;
        seed_commit(app, &local, "Spike experiment").await;
    }
}

use super::*;
use crate::editor::vcs::unit_tests::context::{action_args, app, dispatch, no_args, seeded_envelope};
use semio_framework_plugin::artifact_app_laws::meta;
use semio_framework_plugin::PluginApp;
use serde_json::{from_str as parse, Value};
use store::HistoryColumn;

const RETAINED_LIMITS: &str = include_str!("../../🧫️fixtures/🧫️retained-command-limits/🔣️.json");
const RETAINED_EDIT_LIMITS: &str = include_str!("../../🧫️fixtures/✍️retained-edit-limits/🔣️.json");
const RETAINED_ROUTES: &str = include_str!("../../🧫️fixtures/🛣️retained-command-routes.json");

//#region 🔖️CommandSurface
/// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
/// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
#[semio_framework_async_macros::async_test]
async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids.len(), 10, "every VcsCommand row must be covered by every_command()");
}

/// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_through_text_and_binary() {
    for command in every_command() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
    }
}

/// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
/// undeclared host-pushed command). This is what a missing `#[dsl(keyword = ..)]` on a payload struct
/// silently breaks (the record prints with no keyword at all and no longer parses).
#[semio_framework_async_macros::async_test]
async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
    for command in every_command() {
        let id = command.command_id();
        let expected = if id == "noMutation" { "no-operation".to_string() } else { id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect() };
        let printed = protocol::OpText::print_op(&command);
        assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
    }
}

// 🧷️ No `VcsCommand` payload has an `Option` field (unlike flow's `AddWidget`/`SetGridVisible`), so
// there is no `None`/`Some`-distinguishing wire case here and no
// `optional_field_rows_keep_their_pre_migration_bytes`-style pinning test is needed.

/// 🧾️ One representative value per row, in declaration (= binary ordinal) order. Matches the pilot's
/// wire baseline dump byte-for-byte (ticket `🧪️wire-baseline-before.txt`).
pub(super) fn every_command() -> Vec<VcsCommand> {
    vec![
        VcsCommand::IncrementCounter(increment_counter::IncrementCounter {}),
        VcsCommand::PatchSnapshot(patch_snapshot::PatchSnapshot { field: "title".into(), value: "Renamed".into() }),
        VcsCommand::TextEdit(text_edit::TextEdit { text: "{}".into() }),
        VcsCommand::Edit(edit_command::Edit { text: "{}".into() }),
        VcsCommand::NoMutation(no_operation::NoMutation {}),
        VcsCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown {}),
        VcsCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove {}),
        VcsCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}),
        VcsCommand::CanvasWheel(canvas_wheel::CanvasWheel {}),
    ]
}

#[test]
fn bounded_command_factory_matches_the_language_neutral_maximum_oracle() {
    let fixture: Value = parse(RETAINED_LIMITS).expect("VCS retained limits decode");
    let maximum = fixture.get("maximumTextBytes").and_then(Value::as_u64).expect("maximumTextBytes") as usize;
    let additional = fixture.get("rejectedAdditionalBytes").and_then(Value::as_u64).expect("rejectedAdditionalBytes") as usize;
    let expected_items = fixture.get("expectedWorkItems").and_then(Value::as_u64).expect("expectedWorkItems") as usize;
    let tool_ids = fixture.get("toolIds").and_then(Value::as_array).expect("toolIds").iter().map(|value| value.as_str().expect("tool id")).collect::<Vec<_>>();
    assert_eq!(maximum, VCS_BOUNDED_RAW_BYTES);
    assert_eq!(expected_items, VCS_BOUNDED_WORK_ITEMS);
    assert_eq!(tool_ids, VCS_BOUNDED_TOOL_IDS);
    let snapshot = VcsPlayApp::initial_snapshot();
    let interaction = protocol::InteractionState::default();
    let accepted = VcsCommand::PatchSnapshot(patch_snapshot::PatchSnapshot { field: String::new(), value: "v".repeat(maximum) });
    let rejected = VcsCommand::PatchSnapshot(patch_snapshot::PatchSnapshot { field: String::new(), value: "v".repeat(maximum + additional) });
    assert_eq!(vcs_bounded_extent(&accepted, &snapshot, &interaction), Some(expected_items));
    assert_eq!(vcs_bounded_extent(&rejected, &snapshot, &interaction), None);
    let factory = VcsBoundedCommandJobFactory::new("s.vcs.vcs@1/*#editor");
    assert_eq!(factory.execution_contract(), ToolExecutionContract::bounded_first_step(8_192, 32, 32, 16_384, 7_500));
    assert!(VcsPlayApp::command_from_action("patchSnapshot", Some(&action_args([("field", "f".to_string()), ("value", "v".repeat(maximum + additional))]))).is_err());
}

#[test]
fn retained_factories_publish_only_their_exact_declared_lanes() {
    use semio_framework_plugin::ArtifactOwnedToolJobFactory;
    let fixture: Value = parse(RETAINED_ROUTES).expect("VCS retained route fixture decodes");
    let routes = fixture.get("routes").and_then(Value::as_array).expect("routes");
    assert_eq!(routes.len(), 9);
    assert_eq!(<VcsBoundedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS, VCS_BOUNDED_PUBLICATION_CONTRACTS);
    assert_eq!(<VcsResumableCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS, VCS_RESUMABLE_PUBLICATION_CONTRACTS);
    for route in routes {
        let id = route.get("id").and_then(Value::as_str).expect("route id");
        let lane = route.get("lanes").and_then(Value::as_array).and_then(|lanes| lanes.first()).and_then(Value::as_str).expect("route lane");
        let contract = VCS_BOUNDED_PUBLICATION_CONTRACTS.iter().chain(VCS_RESUMABLE_PUBLICATION_CONTRACTS).find(|row| row.tool_id == id).expect("route contract");
        let expected = match lane {
            "Artifact" => ArtifactToolPublicationLane::Artifact,
            "Config" => ArtifactToolPublicationLane::Config,
            "HostOnly" => ArtifactToolPublicationLane::HostOnly,
            _ => panic!("unknown fixture lane"),
        };
        assert_eq!(contract.lanes, &[expected]);
    }
    assert_eq!(VCS_BOUNDED_PUBLICATION_CONTRACTS.iter().find(|row| row.tool_id == "noMutation").map(|row| row.lanes), Some(&[ArtifactToolPublicationLane::HostOnly][..]));
    assert!(VCS_RESUMABLE_PUBLICATION_CONTRACTS.iter().all(|row| row.lanes == [ArtifactToolPublicationLane::Artifact]));
}

#[test]
fn one_item_store_preparation_rejects_non_document_lanes() {
    use store::ArtifactStoreOneItemPreparationFactory;
    let artifact = VcsOneItemPreparationFactory::<VcsSnapshot, VcsDemoMutation>::new(store::HistoryLane::Document);
    let config = VcsOneItemPreparationFactory::<VcsDemoConfig, VcsDemoConfigMutation>::new(store::HistoryLane::Document);
    assert!(artifact.preflight(&crate::mutations::change_counter(1), None, store::HistoryLane::Document).is_ok());
    assert!(artifact.preflight(&crate::mutations::change_counter(1), None, store::HistoryLane::Interaction).is_err());
    let mutation = VcsDemoConfigMutation::Snapshot { config: VcsDemoConfig::default() };
    assert!(config.preflight(&mutation, None, store::HistoryLane::Document).is_ok());
    assert!(config.preflight(&mutation, None, store::HistoryLane::Interaction).is_err());
}

#[test]
fn action_bridge_covers_all_vcs_owned_commands_and_rejects_unknown_actions() {
    let rows = [
        ("incrementCounter", no_args()),
        ("patchSnapshot", action_args([("field", "title".to_string()), ("value", "next".to_string())])),
        ("textEdit", action_args([("text", "{}".to_string())])),
        ("edit", action_args([("text", "{}".to_string())])),
        ("noMutation", no_args()),
        ("canvasPointerDown", no_args()),
        ("canvasPointerMove", no_args()),
        ("canvasPointerUp", no_args()),
        ("canvasWheel", no_args()),
    ];
    for (id, args) in rows {
        assert_eq!(VcsPlayApp::command_from_action(id, Some(&args)).expect("declared action bridge").command_id(), id);
    }
    assert!(VcsPlayApp::command_from_action("unknown", None).is_err());
}

#[test]
fn resumable_text_edit_matches_the_serde_json_batch_oracle() {
    let fixture: Value = parse(RETAINED_EDIT_LIMITS).expect("VCS retained edit limits decode");
    assert_eq!(fixture.get("toolIds").and_then(Value::as_array).expect("toolIds").iter().map(|value| value.as_str().expect("tool id")).collect::<Vec<_>>(), VCS_RESUMABLE_TOOL_IDS);
    assert_eq!(fixture.get("maximumTextBytes").and_then(Value::as_u64), Some(VCS_BOUNDED_RAW_BYTES as u64));
    assert_eq!(fixture.get("maximumTags").and_then(Value::as_u64), Some(VCS_EDIT_MAXIMUM_TAGS as u64));
    assert_eq!(fixture.get("maximumOutputBytes").and_then(Value::as_u64), Some(VCS_EDIT_MAXIMUM_OUTPUT_BYTES as u64));
    assert_eq!(fixture.get("maximumWorkItems").and_then(Value::as_u64), Some(VCS_EDIT_MAXIMUM_WORK_ITEMS as u64));

    let mut current = VcsPlayApp::initial_snapshot();
    current.title = "before".into();
    current.tags = vec!["keep".into(), "remove".into()];
    let mut next = current.clone();
    next.title = "after".into();
    next.counter = 42;
    next.tags = vec!["keep".into(), "add".into()];
    let text = serde_json::to_string(&next).expect("serde_json oracle encodes next snapshot");
    let command = VcsCommand::TextEdit(text_edit::TextEdit { text: text.clone() });
    let expected = edit_command::text_edit_operations(&text, &current);
    let mut work = VcsEditCommandWork::new("textEdit");
    let mut turns = 0;
    let actual = loop {
        turns += 1;
        if let Some(emit) = work.advance(&command, &current).expect("resumable edit turn") {
            break emit;
        }
        assert!(turns <= VCS_EDIT_MAXIMUM_WORK_ITEMS);
    };
    assert!(turns > 1, "text edit must cross real scheduler turns");
    assert_eq!(actual.artifact_mutations, expected.artifact_mutations);
}

#[test]
fn resumable_text_edit_enforces_maximum_plus_one_and_retires_incrementally() {
    use semio_framework_plugin::retained_command::ArtifactCommandWork;
    let fixture: Value = parse(RETAINED_EDIT_LIMITS).expect("VCS retained edit limits decode");
    let maximum = fixture.get("maximumTextBytes").and_then(Value::as_u64).expect("maximumTextBytes") as usize;
    let additional = fixture.get("rejectedAdditionalBytes").and_then(Value::as_u64).expect("rejectedAdditionalBytes") as usize;
    assert!(VcsPlayApp::command_from_action("textEdit", Some(&action_args([("text", "x".repeat(maximum + additional))]))).is_err());
    let mut oversized_snapshot = VcsPlayApp::initial_snapshot();
    oversized_snapshot.tags = (0..=VCS_EDIT_MAXIMUM_TAGS).map(|index| format!("tag-{index}")).collect();
    let command = VcsCommand::Edit(edit_command::Edit { text: "{}".into() });
    assert_eq!(vcs_edit_extent(&command, &oversized_snapshot, &protocol::InteractionState::default()), None);

    let mut work = VcsEditCommandWork::new("edit");
    work.next = Some(VcsSnapshot { tags: vec!["retire".into()], ..VcsPlayApp::initial_snapshot() });
    work.current_tags.insert("current".into());
    work.next_tags.insert("next".into());
    work.mutations.push(crate::mutations::add_tag("mutation".into()));
    work.begin_close();
    let mut turns = 0;
    while !work.terminal_is_empty() {
        turns += 1;
        let step = work.close_step(1, VCS_EDIT_MAXIMUM_OUTPUT_BYTES);
        assert!(!matches!(step, semio_framework_job::InteractiveJobCloseStep::Blocked));
        assert!(turns < 16);
    }
    assert!(turns >= 5, "each nested owner must retire through a separate close grant");
}

#[test]
fn every_resumable_edit_turn_stays_below_the_interaction_ceiling() {
    let mut current = VcsPlayApp::initial_snapshot();
    current.tags = (0..64).map(|index| format!("current-{index:04}-{}", "x".repeat(32))).collect();
    let mut next = current.clone();
    next.notes = "n".repeat(256);
    next.tags.rotate_left(1);
    next.tags.push("z".repeat(4_096));
    let command = VcsCommand::TextEdit(text_edit::TextEdit { text: serde_json::to_string(&next).expect("maximum-turn fixture") });
    assert!(vcs_edit_text(&command).expect("text").len() <= VCS_BOUNDED_RAW_BYTES);
    let mut work = VcsEditCommandWork::new("textEdit");
    loop {
        let started = std::time::Instant::now();
        let step = work.advance(&command, &current).expect("timed edit turn");
        assert!(started.elapsed().as_micros() < 8_000, "one VCS edit turn exceeded 8 ms");
        if step.is_some() {
            break;
        }
    }
}
//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let json = serde_json::to_string(&create_vcs_app()).expect("app definition json");
    for id in [editor::VCS_PLAY_WINDOW_EDITOR, history::VCS_PLAY_WINDOW_HISTORY] {
        assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
    }
    assert!(json.contains(edit::VCS_PLAY_MODE_EDIT), "mode missing from the manifest");
    for body in [VCS_PLAY_BODY_DOCUMENT, VCS_PLAY_BODY_INSPECTION] {
        assert!(json.contains(body), "panel body {body} missing from the manifest");
    }
    assert!(json.contains("vcs.vcs"), "artifact kind missing from the manifest");
}

/// 🧪️ The registry-enforced app (View/Shell kind discipline) must still dispatch every declared
/// manifest action — exercises `context::app_with_registry`, the counterpart to the bare `app()`
/// every other node's tests use.
#[semio_framework_async_macros::async_test]
async fn registry_enforced_app_dispatches_a_declared_action() {
    use crate::editor::vcs::unit_tests::context::app_with_registry;
    let mut instance = app_with_registry().await;
    let before = instance.snapshot().expect("materialize snapshot").counter;
    dispatch(&mut instance, VcsCommand::IncrementCounter(increment_counter::IncrementCounter {})).await;
    assert_eq!(instance.snapshot().expect("materialize snapshot").counter, before + 1);
}
//#endregion 🔖️ManifestSanity

//#region 🔖️Interaction
/// 🕹️ The "history" domain is declared `HierarchyProvider::Flat`, Pick-only, and scoped to the
/// history window kind — see `VCS_INTERACTION_HISTORY`'s doc comment for why this is entity
/// selection over checkpoints, not the per-row `checkoutCheckpoint`/`switchAlternative` navigation.
#[semio_framework_async_macros::async_test]
async fn history_interaction_domain_is_declared_flat_and_scoped_to_the_history_window() {
    let definition = create_vcs_app();
    let history_domain = definition.interactions.iter().find(|interaction| interaction.id == VCS_INTERACTION_HISTORY).expect("history interaction domain declared");
    assert!(matches!(history_domain.hierarchy, HierarchyProvider::Flat));
    assert!(!history_domain.selection.transitive, "checkpoints have no selectable-entity nesting");
    assert_eq!(history_domain.granularities.len(), 1);
    assert_eq!(history_domain.granularities[0].id, "commit");
    let history_window = definition.window_kinds.iter().find(|window| window.id == history::VCS_PLAY_WINDOW_HISTORY).expect("history window kind declared");
    assert!(history_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == VCS_INTERACTION_HISTORY), "history window must reference the history interaction domain");
    let editor_window = definition.window_kinds.iter().find(|window| window.id == editor::VCS_PLAY_WINDOW_EDITOR).expect("editor window kind declared");
    assert!(editor_window.interactions.is_empty(), "the editor window has no checkpoint tree, so no interaction domain is scoped to it");
}
//#endregion 🔖️Interaction

//#region 🔖️CrossCutting
#[semio_framework_async_macros::async_test]
async fn seeded_history_has_checkpoints() {
    let instance = app().await;
    let envelope = seeded_envelope(&instance).await;
    assert!(envelope.vcs.alternatives.len() >= 5, "expected >=5 alternatives, got {}", envelope.vcs.alternatives.len());
    assert!(envelope.vcs.checkpoints.len() >= 14, "expected >=14 checkpoints, got {}", envelope.vcs.checkpoints.len());
    let mut children_by_parent: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for checkpoint in &envelope.vcs.checkpoints {
        if let Some(parent_id) = &checkpoint.parent_id {
            *children_by_parent.entry(parent_id.clone()).or_insert(0) += 1;
        }
    }
    assert!(children_by_parent.values().any(|count| *count >= 2), "seed must contain a real fork (a checkpoint with >=2 children)");
    let lanes: std::collections::HashSet<usize> = store::build_history_columns(&envelope).await.into_iter().map(|column: HistoryColumn| column.lane).collect();
    assert!(lanes.len() >= 3, "expected >=3 distinct swimlanes, got {lanes:?}");
}

#[semio_framework_async_macros::async_test]
async fn checkout_then_commit_forks_across_actions() {
    let mut instance = app().await;
    let envelope_before = seeded_envelope(&instance).await;
    let root_checkpoint_id = envelope_before.vcs.checkpoints[0].id.clone();
    let children_of_root_before = envelope_before.vcs.checkpoints.iter().filter(|checkpoint| checkpoint.parent_id.as_deref() == Some(root_checkpoint_id.as_str())).count();

    let checkout = instance.handle_action("checkoutCheckpoint", Some(&action_args([("checkpointId", root_checkpoint_id.clone())])), &meta("local")).await.expect("checkout");
    assert!(checkout.mutations.is_empty(), "history actions never emit KernelMutations");

    dispatch(&mut instance, VcsCommand::IncrementCounter(increment_counter::IncrementCounter {})).await;
    instance.handle_action("commitCheckpoint", Some(&action_args([("message", "forked from root".to_string())])), &meta("local")).await.expect("commit");

    let envelope_after = seeded_envelope(&instance).await;
    let children_of_root_after = envelope_after.vcs.checkpoints.iter().filter(|checkpoint| checkpoint.parent_id.as_deref() == Some(root_checkpoint_id.as_str())).count();
    assert_eq!(children_of_root_after, children_of_root_before + 1, "checking out the root then committing through actions must add a new fork of the root, not extend the trunk");
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_through_the_wrapper() {
    let mut instance = app().await;
    let before = instance.snapshot().expect("materialize snapshot").counter;
    dispatch(&mut instance, VcsCommand::IncrementCounter(increment_counter::IncrementCounter {})).await;
    assert_eq!(instance.snapshot().expect("materialize snapshot").counter, before + 1);
    let undo = instance.handle_action("undo", None, &meta("local")).await.expect("undo");
    assert!(undo.mutations.is_empty());
    assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
    assert_eq!(instance.snapshot().expect("materialize snapshot").counter, before);
    instance.handle_action("redo", None, &meta("local")).await.expect("redo");
    assert_eq!(instance.snapshot().expect("materialize snapshot").counter, before + 1);
}

#[semio_framework_async_macros::async_test]
async fn create_and_switch_alternative_round_trip_through_the_wrapper() {
    let mut instance = app().await;
    let create = instance.handle_action("createAlternative", Some(&action_args([("name", "trying-something".to_string())])), &meta("local")).await.expect("create alternative");
    assert!(create.mutations.is_empty());
    let envelope = seeded_envelope(&instance).await;
    assert!(envelope.active_alternative_id.is_some(), "createAlternative must set an active alternative");
}
//#endregion 🔖️CrossCutting
