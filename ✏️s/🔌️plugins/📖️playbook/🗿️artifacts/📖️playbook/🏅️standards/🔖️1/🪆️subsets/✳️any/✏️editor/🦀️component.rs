//! ✏️ Playbook editor — the mutation-capable surface for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `PlaybookPlayApp` implements
//! `ArtifactEditor`, never the runtime `ArtifactApp` directly — `EditorApp<PlaybookPlayApp>`
//! (framework SDK) is the sole runtime adapter `PluginBuilder::editor` wires up.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window render
//! in `🎭️modes/🏗️builder/🪟️windows/🏗️builder`, labels in `🗣️terminology`, view state in `🎚️config`,
//! shared compute in `⚙️engine`. This file is a routing table: `handle` → `PlaybookCommand::dispatch`,
//! `render` → body-key → node, plus `import_media`'s `"chapters:in"` importer (an editor-level override,
//! not a command).

use crate::editor::playbook::commands::{add_block, add_step, move_block, move_step, remove_block, remove_step, set_contributions, set_locale, update_playbook};
use crate::editor::playbook::config::{PlaybookConfig, PlaybookConfigMutation};
use crate::editor::playbook::modes::builder;
use crate::editor::playbook::modes::builder::windows::builder as builder_window;
use crate::artifacts::playbook::flatten_playbook_blocks;
use crate::artifacts::playbook::schema::default_block;
use crate::editor::playbook::engine::{playbook_io, PlaybookChapterPayload};
use crate::artifacts::playbook::op::{AddStep, PlaybookMutation};
use crate::artifacts::playbook::{artifact_kind, PlaybookSnapshot, PlaybookStep, PLAYBOOK_DIALECT, PLAYBOOK_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    NoDraft, NoDraftMutation, DraftView, ActionArgDef, ActionArgOption, ActionKind, ArtifactEditor, CommandDefinition, ConfigView, Dialect, Editor, ArtifactView, Emit, Fault, Label, LocalizedLabel, Media, MediaError, MediaPayload, UiNode,
    GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, MergeMode, SelectionMethod, SelectionMode, SelectionSpec,
    DomainTopology, InteractionTopology, TopologyNode,
};
use semio_framework_plugin::app::InteractionView;
use store::EngineHandles;

//#region 🔖️Constants
pub use builder_window::PLAYBOOK_PLAY_BODY_BUILDER;
pub use builder_window::PLAYBOOK_PLAY_WINDOW_BUILDER;

/// 📥️ The step `"chapters:in"` imports land in — created on first import, reused on every later one.
const PLAYBOOK_IMPORTED_STEP_ID: &str = "imported";
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `PlaybookPlayApp::Command` — the SOLE dispatch surface for playbook's own behavior, assembled
    /// from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// kebab-case `#[dsl(key = ..)]` the codec uses) — copied verbatim off the pre-migration
    /// `playbook_protocol::PlaybookCommand`'s `#[dsl(key)]` attributes. **Row order is the binary
    /// variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum PlaybookCommand for PlaybookSnapshot, PlaybookMutation, PlaybookConfig, PlaybookConfigMutation {
        "addStep" as "add-step" => add_step::AddStep,
        "removeStep" as "remove-step" => remove_step::RemoveStep,
        "moveStep" as "move-step" => move_step::MoveStep,
        "addBlock" as "add-block" => add_block::AddBlock,
        "removeBlock" as "remove-block" => remove_block::RemoveBlock,
        "moveBlock" as "move-block" => move_block::MoveBlock,
        "updatePlaybook" as "update-playbook" => update_playbook::UpdatePlaybook,
        "setLocale" as "locale" => set_locale::SetLocale,
        "setContributions" as "contributions" => set_contributions::SetContributions,
    }
}
//#endregion 🔖️Commands

//#region 🔖️Interaction
/// 🕹️ "blocks" — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14) interaction domain
/// this app declares: `HierarchyProvider::Topology` over the document's own step/block nesting (steps
/// are the "step" granularity, blocks are the "block" granularity, default) — replaces the deleted
/// `PlaybookConfig::selected_ids`/`set-selection` command/`PlaybookPresence::selected_ids`. Pick-only
/// (the block-list builder is a flat clickable list, no canvas marquee surface); not transitive —
/// the pre-migration `selected_ids` never auto-expanded a step selection onto its blocks, so this
/// keeps that exact semantic instead of inventing new cascading-selection behavior.
pub const PLAYBOOK_INTERACTION_BLOCKS: &str = "blocks";
pub const PLAYBOOK_INTERACTION_GRANULARITY_BLOCK: &str = "block";
pub const PLAYBOOK_INTERACTION_GRANULARITY_STEP: &str = "step";

/// 🌳️ `blocks` domain topology from the document's own step/block nesting — step ids and block ids
/// share the same flat id namespace `PlaybookConfig::selected_ids` used to (`remove-block`'s old manual
/// prune matched on either), so `validate_state` prunes a deleted step's OR block's id automatically
/// after every document dispatch (`revalidate_interaction_state_after_document_change`), replacing the
/// deleted hand-rolled prune in `remove_block::handle`.
async fn playbook_blocks_topology(spec: &PlaybookSnapshot) -> DomainTopology {
    let mut ordered = Vec::new();
    for step in spec.steps() {
        ordered.push(TopologyNode { id: step.id.clone(), granularity: PLAYBOOK_INTERACTION_GRANULARITY_STEP.into(), parent: None });
        for block in step.blocks {
            ordered.push(TopologyNode { id: block.id.clone(), granularity: PLAYBOOK_INTERACTION_GRANULARITY_BLOCK.into(), parent: Some(step.id.clone()) });
        }
    }
    DomainTopology { ordered }
}
//#endregion 🔖️Interaction

//#region 🔖️PlaybookPlayApp
/// 🧪️ B1: unit struct — the former app-struct `RefCell<Vec<String>>` selection now lives in
/// `PlaybookConfig` (see `ArtifactEditor::Config`), written through `PlaybookConfigMutation`s.
#[derive(Default)]
pub struct PlaybookPlayApp;

impl ArtifactEditor for PlaybookPlayApp {
    type Snapshot = PlaybookSnapshot;
    type Mutation = PlaybookMutation;
    type Config = PlaybookConfig;
    type ConfigMutation = PlaybookConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::playbook::presence::PlaybookPresence;
    type PresenceMutation = crate::editor::playbook::presence::PlaybookPresenceMutation;
    type Transient = semio_framework_plugin::app::NoTransient;
    type TransientMutation = semio_framework_plugin::app::NoTransientMutation;

    type Command = PlaybookCommand;

    const DIALECT: Dialect = PLAYBOOK_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = PLAYBOOK_DOCUMENT_SCHEMA;

    async fn app_schema() -> Option<schema::AppSchemaDescriptor> {
        Some(crate::editor::playbook::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> PlaybookSnapshot {
        crate::artifacts::playbook::empty_playbook_snapshot()
    }

    async fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(playbook_io())
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    async fn command_id(command: &PlaybookCommand) -> &'static str {
        command.command_id()
    }

    async fn handle(command: &PlaybookCommand, doc: &ArtifactView<'_, PlaybookSnapshot>, cfg: &ConfigView<'_, PlaybookConfig>, _interaction: &InteractionView<'_>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🕹️ `blocks` domain: `HierarchyProvider::Topology` from the document's own step/block nesting —
    /// see `playbook_blocks_topology`'s doc comment.
    async fn interaction_topology(doc: &ArtifactView<'_, PlaybookSnapshot>, _cfg: &ConfigView<'_, PlaybookConfig>) -> InteractionTopology {
        let mut domains = std::collections::BTreeMap::new();
        domains.insert(PLAYBOOK_INTERACTION_BLOCKS.to_string(), playbook_blocks_topology(doc.snapshot));
        InteractionTopology { domains }
    }

    /// 🎞️ `"chapters:in"` (Text×Document, `Many`) — decodes a `writer`-shaped chapter payload (see
    /// `writer_engine::WriterChapterPayload`/`PlaybookChapterPayload`) and inserts it as a `"note"` block
    /// (free-form `text` field, non-interactive) into a dedicated `"imported"` step, created on first
    /// import and reused on every later one (idempotent step creation).
    async fn import_media(port: &str, media: &Media, doc: &ArtifactView<'_, PlaybookSnapshot>) -> Result<Emit<PlaybookMutation, PlaybookConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "chapters:in" {
            return Err(MediaError::NotImplemented);
        }
        let MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "chapters:in importer only accepts a Structured payload".into()));
        };
        let chapter: PlaybookChapterPayload = serde_json::from_str(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let spec = doc.snapshot;
        let mut operations = Vec::new();
        if !spec.steps().iter().any(|step| step.id == PLAYBOOK_IMPORTED_STEP_ID) {
            operations.push(PlaybookMutation::AddStep(AddStep { step: PlaybookStep { id: PLAYBOOK_IMPORTED_STEP_ID.into(), title: "Imported".into(), description: None, blocks: Vec::new() }, index: None }));
        }
        let block_id = format!("chapter-{}", flatten_playbook_blocks(spec).len() + 1);
        let mut block = default_block(block_id, "note");
        block.label = chapter.title;
        block.text = Some(chapter.text);
        operations.push(crate::artifacts::playbook::op::add_block_operation(PLAYBOOK_IMPORTED_STEP_ID, block, None));
        Ok(Emit::mutations(operations))
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, PlaybookSnapshot>, cfg: &ConfigView<'_, PlaybookConfig>) -> UiNode {
        match body_key {
            PLAYBOOK_PLAY_BODY_BUILDER => builder_window::render(doc.snapshot, cfg.snapshot),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️PlaybookPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub async fn create_playbook_play_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(PLAYBOOK_DIALECT)
        .command(CommandDefinition { in_palette: false, ..CommandDefinition::new_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))]) })
        .document(["semio", "playbook"])
        .artifact_kind(artifact_kind())
        .mode_def(builder::definition())
        .default_mode_id(builder::PLAYBOOK_PLAY_MODE_BUILDER)
        .window_kind_def(builder_window::definition())
        .default_layout(builder::layout())
        .mutation("addStep", LocalizedLabel::native("Add Step", "Schritt hinzufügen"))
        .mutation("removeStep", LocalizedLabel::native("Remove Step", "Schritt entfernen"))
        .mutation("moveStep", LocalizedLabel::native("Move Step", "Schritt verschieben"))
        .mutation("addBlock", LocalizedLabel::native("Add Block", "Baustein hinzufügen"))
        .mutation("removeBlock", LocalizedLabel::native("Remove Block", "Baustein entfernen"))
        .mutation("moveBlock", LocalizedLabel::native("Move Block", "Baustein verschieben"))
        .mutation("updatePlaybook", LocalizedLabel::native("Update Playbook", "Playbook aktualisieren"))
        // 📝️ Staged argument form for the panel-visible create action (block kind is a choice).
        .action_args("addBlock", vec![
            ActionArgDef::select(
                "kind",
                LocalizedLabel::native("Kind", "Art"),
                crate::artifacts::playbook::PLAYBOOK_BUILTIN_KINDS.iter().map(|kind| ActionArgOption::new(*kind, LocalizedLabel::data(*kind))).collect(),
            )
            .default_value("text"),
        ])
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "blocks" interaction
        // domain — two granularities ("block" default, "step"), `HierarchyProvider::Topology` from
        // the document's own step/block nesting (`playbook_blocks_topology`/
        // `PlaybookPlayApp::interaction_topology`). Selection is pick-only (no canvas marquee
        // surface exists for this domain); the framework auto-injects interactionSelect/
        // interactionHover/clearSelection/selectAll/setSelectionMode/setInteractionGranularity,
        // replacing the deleted `setSelection` view action.
        .interaction(InteractionDefinition {
            id: PLAYBOOK_INTERACTION_BLOCKS.into(),
            label: LocalizedLabel::native("Blocks", "Bausteine"),
            granularities: vec![
                GranularityDefinition { id: PLAYBOOK_INTERACTION_GRANULARITY_BLOCK.into(), label: LocalizedLabel::native("Block", "Baustein"), icon_id: "square".into() },
                GranularityDefinition { id: PLAYBOOK_INTERACTION_GRANULARITY_STEP.into(), label: LocalizedLabel::native("Step", "Schritt"), icon_id: "list-ordered".into() },
            ],
            hierarchy: HierarchyProvider::Topology,
            hover: HoverSpec::default(),
            selection: SelectionSpec {
                modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                methods: vec![SelectionMethod::Pick],
                merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
                transitive: false,
                broadcast: true,
            },
        })
        .window_kind_interactions(PLAYBOOK_PLAY_WINDOW_BUILDER, vec![InteractionRef::new(PLAYBOOK_INTERACTION_BLOCKS)])
        // 🎯️ Typed channel surface (mirrors `writer_ui::create_writer_app`'s identical wiring) —
        // `crate::editor::playbook::engine::playbook_io()` is the single source of truth for both
        // the trait's `io()` override and this manifest declaration.
        .config(PlaybookPlayApp::config_spec())
        .io(playbook_io())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type PlaybookApp = VcsArtifactApp<EditorApp<PlaybookPlayApp>>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn playbook_app() -> PlaybookApp {
        new_app::<EditorApp<PlaybookPlayApp>>()
    }

    /// 🧪️ Adapts `create_playbook_play_app`'s `AppDefinition` (contract §2.4) into the `App {
    /// definition, examples }` shape `new_app_with_registry`/`assert_declared_actions_bridge_to_commands`
    /// still expect — framework testkit gap, not modifiable here (`🧰️framework/**` is outside this
    /// packet's lease).
    pub async fn playbook_manifest_for_testkit() -> App {
        App { definition: create_playbook_play_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline, and the
    /// `kind` default declared on `addBlock` materializes host-side.
    pub async fn playbook_app_with_registry() -> PlaybookApp {
        new_app_with_registry::<EditorApp<PlaybookPlayApp>>(playbook_manifest_for_testkit)
    }

    pub async fn dispatch(app: &mut PlaybookApp, command: PlaybookCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn render(app: &mut PlaybookApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::playbook::testkit::{dispatch, playbook_app};
    use crate::artifacts::playbook::op::AddBlock;
    use semio_framework_plugin::testkit;
    use semio_framework_plugin::{MediaClass, MediaForm};

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
    /// kebab-cased command id for every row except `setLocale`, preserved exactly (VERBATIM off the
    /// pre-migration `playbook_protocol::PlaybookCommand`'s own `#[dsl(key = ..)]` attribute) so the wire
    /// format stays byte-identical across the migration; see TEMPLATE.md §5.1.
    #[semio_framework_async_macros::async_test]
    async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        for command in every_command() {
            let id = command.command_id();
            let expected = match id {
                "setLocale" => "locale".to_string(),
                "setContributions" => "contributions".to_string(),
                _ => id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect(),
            };
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) async fn every_command() -> Vec<PlaybookCommand> {
        vec![
            PlaybookCommand::AddStep(add_step::AddStep {}),
            PlaybookCommand::RemoveStep(remove_step::RemoveStep { step_id: "s".into() }),
            PlaybookCommand::MoveStep(move_step::MoveStep { step_id: "s".into(), index: 2 }),
            PlaybookCommand::AddBlock(add_block::AddBlock { kind: "text".into(), step_id: None }),
            PlaybookCommand::RemoveBlock(remove_block::RemoveBlock { step_id: "s".into(), block_id: "b".into() }),
            PlaybookCommand::MoveBlock(move_block::MoveBlock { block_id: "b".into(), from_step_id: "s1".into(), to_step_id: "s2".into(), index: 0 }),
            PlaybookCommand::UpdatePlaybook(update_playbook::UpdatePlaybook { value: "Recipe".into() }),
            PlaybookCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            PlaybookCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
        ]
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_playbook_play_app()).expect("app definition json");
        assert!(json.contains(PLAYBOOK_PLAY_WINDOW_BUILDER), "window kind missing from the manifest: {json}");
        assert!(json.contains(builder::PLAYBOOK_PLAY_MODE_BUILDER), "mode missing from the manifest");
        assert!(json.contains("text.playbook"), "artifact kind missing from the manifest");
    }

    #[semio_framework_async_macros::async_test]
    async fn playbook_play_app_declares_builder_window_only() {
        let definition = create_playbook_play_app();
        assert_eq!(definition.window_kinds.len(), 1);
        assert_eq!(definition.window_kinds[0].id, PLAYBOOK_PLAY_WINDOW_BUILDER);
        assert_eq!(definition.window_kinds[0].body_key, PLAYBOOK_PLAY_BODY_BUILDER);
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

    /// 🌳️ `interaction_topology` walks every step and every one of its blocks into a `TopologyNode`, so
    /// `validate_state` can prune a deleted step's OR block's id out of a stale selection.
    #[semio_framework_async_macros::async_test]
    async fn interaction_topology_covers_every_step_and_block() {
        let mut app = playbook_app();
        dispatch(&mut app, PlaybookCommand::AddStep(add_step::AddStep {}));
        let step_id = app.snapshot().expect("projection").steps()[0].id.clone();
        dispatch(&mut app, PlaybookCommand::AddBlock(add_block::AddBlock { kind: "text".into(), step_id: Some(step_id.clone()) }));
        let spec = app.snapshot().expect("projection");
        let block_id = spec.steps().iter().find(|step| step.id == step_id).expect("step present").blocks[0].id.clone();
        let history = semio_framework_plugin::HistoryView::empty();
        let cfg = PlaybookConfig::default();
        let doc = ArtifactView::new(&spec, &history);
        let topology = PlaybookPlayApp::interaction_topology(&doc, &ConfigView { snapshot: &cfg });
        let blocks = topology.domains.get(PLAYBOOK_INTERACTION_BLOCKS).expect("blocks domain present in topology");
        assert!(blocks.ordered.iter().any(|node| node.id == step_id && node.granularity == PLAYBOOK_INTERACTION_GRANULARITY_STEP && node.parent.is_none()));
        assert!(blocks.ordered.iter().any(|node| node.id == block_id && node.granularity == PLAYBOOK_INTERACTION_GRANULARITY_BLOCK && node.parent.as_deref() == Some(step_id.as_str())));
    }
    //#endregion 🔖️Interaction

    //#region 🔖️CrossCutting
    #[semio_framework_async_macros::async_test]
    async fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = playbook_app();
        testkit::assert_undo_redo_round_trip(&mut app, PlaybookCommand::AddStep(add_step::AddStep {}), |app| app.snapshot().expect("materialize projection").steps().len(), 1, 2);
    }

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::editor::playbook::testkit::render;
        let mut app = playbook_app();
        assert!(render(&mut app, "playbook.play.nope").contains("Unknown body"));
    }

    /// 🧪️ The definitional proof: two independent instances start from the same document, apply
    /// DISJOINT edits (A adds a step, B adds a block to the pre-existing step), and exchanging operations
    /// over a backbone converges both sides onto the same projection — impossible under whole-document
    /// `setDocument` snapshots, where one side's write would clobber the other's.
    #[semio_framework_async_macros::async_test]
    async fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<EditorApp<PlaybookPlayApp>, (usize, usize)>(
            "mem://playbook-convergence",
            PlaybookCommand::AddStep(add_step::AddStep {}),
            PlaybookCommand::AddBlock(add_block::AddBlock { kind: "number".into(), step_id: None }),
            |app| {
                let projection = app.snapshot().expect("materialize projection");
                let steps = projection.steps();
                (steps.len(), steps[0].blocks.len())
            },
        );
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️PortTests
    #[semio_framework_async_macros::async_test]
    async fn playbook_io_declares_the_extra_chapters_in_port_and_its_own_kind() {
        let io = playbook_io();
        assert_eq!(io.artifact.id, "text.playbook");
        let ports = io.all_ports();
        let chapters_in = ports.iter().find(|port| port.id == "chapters:in").expect("chapters:in declared");
        assert_eq!(chapters_in.kind_id.as_deref(), Some("text.document"));
    }

    async fn chapter_media(text: &str, title: &str) -> Media {
        let payload = PlaybookChapterPayload { id: "jack".into(), title: title.into(), text: text.into(), language_id: "jack".into() };
        Media { media_type: semio_framework_plugin::MediaType { class: MediaClass::Text, form: MediaForm::Document }, payload: MediaPayload::Structured { schema: "text.document".into(), json: serde_json::to_string(&payload).unwrap() } }
    }

    #[semio_framework_async_macros::async_test]
    async fn import_media_creates_the_imported_step_and_a_note_block() {
        let spec = crate::artifacts::playbook::empty_playbook_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_view = ArtifactView::new(&spec, &history);
        let media = chapter_media("MATCH (a) RETURN a", "Jack Query");
        let emit = PlaybookPlayApp::import_media("chapters:in", &media, &doc_view).expect("import chapters:in");
        assert_eq!(emit.artifact_mutations.len(), 2, "creates the imported step, then the note block");
        assert!(matches!(&emit.artifact_mutations[0], PlaybookMutation::AddStep(payload) if payload.step.id == PLAYBOOK_IMPORTED_STEP_ID));
        match &emit.artifact_mutations[1] {
            PlaybookMutation::AddBlock(AddBlock { step_id, block, .. }) => {
                assert_eq!(step_id, PLAYBOOK_IMPORTED_STEP_ID);
                assert_eq!(block.kind, "note");
                assert_eq!(block.label, "Jack Query");
                assert_eq!(block.text.as_deref(), Some("MATCH (a) RETURN a"));
            }
            other => panic!("expected AddBlock, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn import_media_reuses_the_imported_step_on_a_second_import() {
        let base = crate::artifacts::playbook::empty_playbook_snapshot();
        let mut steps = base.steps();
        steps.push(PlaybookStep { id: PLAYBOOK_IMPORTED_STEP_ID.into(), title: "Imported".into(), description: None, blocks: Vec::new() });
        let spec = crate::artifacts::playbook::playbook_snapshot_with_steps(&base.schema, &base.id, &base.version, base.title.clone(), steps);
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_view = ArtifactView::new(&spec, &history);
        let media = chapter_media("second chapter", "Second");
        let emit = PlaybookPlayApp::import_media("chapters:in", &media, &doc_view).expect("import chapters:in");
        assert_eq!(emit.artifact_mutations.len(), 1, "the imported step already exists, only the block is added");
        assert!(matches!(&emit.artifact_mutations[0], PlaybookMutation::AddBlock(payload) if payload.step_id == PLAYBOOK_IMPORTED_STEP_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn import_media_rejects_unknown_ports_and_malformed_payloads() {
        let spec = crate::artifacts::playbook::empty_playbook_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc_view = ArtifactView::new(&spec, &history);
        assert!(matches!(PlaybookPlayApp::import_media("nonsense:in", &chapter_media("x", "y"), &doc_view), Err(MediaError::NotImplemented)));
        let bad_media = Media { media_type: semio_framework_plugin::MediaType { class: MediaClass::Text, form: MediaForm::Document }, payload: MediaPayload::Structured { schema: "text.document".into(), json: "not json".into() } };
        assert!(matches!(PlaybookPlayApp::import_media("chapters:in", &bad_media, &doc_view), Err(MediaError::Payload(..))));
    }
    //#endregion 🔖️PortTests
}
//#endregion 🧪️Tests
