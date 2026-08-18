//! ✏️ Wires editor — the read-write counterpart of `👁️viewer` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1). `ReasoningWiresPlayApp` implements
//! `ArtifactEditor`; `EditorApp<ReasoningWiresPlayApp>` (framework SDK) is the sole runtime
//! `ArtifactApp` adapter.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window render
//! in `🎭️modes/✏️edit/🪟️windows/🕸️canvas`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view
//! state in `🦀️config.rs`, shared document helpers in the artifact's `🧬️schema`, derived reads in its
//! `🧬️schema/💡️inferences`, and plugin registration (below — dissolved from the former `⚙️engine`, ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES). This file is a routing table: `handle` →
//! `WiresCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one
//! `definition()` per node.
//!
//! B1: `ReasoningWiresPlayApp` is a unit struct — every former `WiresPlayRuntime` field (selection,
//! in-flight drag) lives in `crate::editor::wires::config::WiresConfig`, written via
//! `crate::editor::wires::config::WiresConfigMutation`s (real `backwards`, no ad hoc runtime `RefCell`);
//! every action dispatches through the single typed `WiresCommand` channel via `ArtifactEditor::handle`.

use crate::editor::wires::commands::add_node;
use crate::editor::wires::commands::add_relationship;
use crate::editor::wires::commands::delete_selection;
use crate::editor::wires::commands::set_active_example;
use crate::editor::wires::commands::set_locale;
use crate::editor::wires::commands::{canvas_pointer_down, canvas_pointer_move, canvas_pointer_up};
use crate::editor::wires::commands::{force_layout, reorganize};
use crate::editor::wires::config::{WiresConfig, WiresConfigMutation};
use crate::editor::wires::modes::edit;
use crate::editor::wires::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::artifacts::wires::op::WiresMutation;
use crate::artifacts::wires::WiresSnapshot;
use semio_framework::kernel::Effect;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ui_text, ActionDescriptor, ArtifactEditor, ArtifactView, ConfigView, Dialect, DraftView, Editor, Emit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, Label, LocalizedLabel,
    MergeMode, NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, UiNode, INTERACTION_SELECT_ACTION_ID,
};
use serde_json::{json, Value};
use store::EngineHandles;

//#region 🔖️Constants
pub const WIRES_PLAY_APP_ID: &str = "reasoning-wires-play";
pub use catalogue_panel::WIRES_PLAY_BODY_CATALOGUE;
pub use document_panel::WIRES_PLAY_BODY_DOCUMENT;
pub use edit::windows::canvas::{WIRES_PLAY_BODY_COMPOSITE, WIRES_PLAY_WINDOW_CANVAS};
pub use inspection_panel::WIRES_PLAY_BODY_PROPERTIES;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub fn wires_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(WIRES_PLAY_APP_ID).action(action, args)
}

/// 🔁️ Builds a `Effect::LoadDocument` for `document` — the sanctioned non-history "replace the
/// whole document" gesture (`ArtifactStore::reset`, applied host-side) that
/// `🎮️commands/🧬️set-active-example::set_active_example` uses instead of a banned whole-snapshot mutation. The
/// spr is a fresh, edit-free op-log — a genesis envelope with no history to encode.
pub fn reset_wires_document_effect(document: &WiresSnapshot) -> Effect {
    let pack = <WiresSnapshot as store::ArtifactPack>::encode_pack(document);
    let envelope = store::create_document_envelope::<WiresSnapshot, WiresMutation>(crate::artifacts::wires::MINDMAP_WIRES_SCHEMA, "reasoning-wires", document.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("wires document spr encode is infallible for a fresh, edit-free envelope");
    Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🕹️ The one framework-owned interaction domain wires declares — identities (nodes) and
/// relationships (edges) on the mindmap canvas plus the document tree's identity/relationship rows
/// (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). `Flat`: the mindmap graph
/// (`infinite_board_normal_undirected`) is a normal undirected identity/relationship graph — no
/// parent/child structure exists anywhere in `WiresSnapshot`/the fixture schema to build a topology
/// from, unlike writer's AST or procedural's DAG, so this crate's own migration deliberately disagrees
/// with the original per-crate brief's "Topology over parent links" guess.
pub const WIRES_INTERACTION_GRAPH: &str = "graph";
pub const WIRES_GRANULARITY_NODE: &str = "node";
pub const WIRES_GRANULARITY_EDGE: &str = "edge";

/// 🕹️ Builds `interactionSelect`'s JSON args for one merge over `ids` at `granularity` — shared by
/// the canvas pointer/add commands (wrapped into a `Effect::DispatchAction`) and any document-tree
/// row whose click should select a real canvas identity/relationship.
pub fn wires_select_action_args(ids: &[String], granularity: &str, merge: &str) -> Value {
    let targets: Vec<Value> = ids.iter().map(|id| json!({ "granularity": granularity, "id": id })).collect();
    json!({ "domainId": WIRES_INTERACTION_GRAPH, "targets": serde_json::to_string(&targets).unwrap_or_default(), "merge": merge, "method": "pick" })
}

/// 🕹️ Wraps [`wires_select_action_args`] into the redispatch effect a canvas gesture's own `handle`
/// returns — `dispatch_action` intercepts the six framework interaction verbs BEFORE routing to
/// `ArtifactApp::handle`, so a plain config mutation can no longer express a selection change; the app
/// asks the host to redispatch `interactionSelect` instead (master doc: "surfaces do geometric
/// hit-testing and emit one batched `interactionSelect`").
pub fn wires_select_effect(ids: &[String], granularity: &str, merge: &str) -> Effect {
    Effect::DispatchAction {req: semio_framework_plugin::RequestId(112),  action: INTERACTION_SELECT_ACTION_ID.into(), args: semio_framework::optional_json_to_dsl(Some(wires_select_action_args(ids, granularity, merge))), delay_ms: 0 }
}
//#endregion 🔖️Interaction

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `ReasoningWiresPlayApp::Command` — the SOLE dispatch surface for this app's behavior,
    /// assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// kebab-case `#[dsl(key = ..)]` the binary/text codec uses) — they are genuinely different
    /// vocabularies; `setLocale`/`locale` is the row that proves it. **Row order is the binary variant
    /// ordinal: appending is safe, reordering is a wire-format break.**
    pub enum WiresCommand for WiresSnapshot, WiresMutation, WiresConfig, WiresConfigMutation {
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "addNode" as "add-node" => add_node::AddNode,
        "addRelationship" as "add-relationship" => add_relationship::AddRelationship,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "forceLayout" as "force-layout" => force_layout::ForceLayout,
        "reorganize" as "reorganize" => reorganize::Reorganize,
        "canvasPointerMove" as "pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerDown" as "pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerUp" as "pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "setLocale" as "locale" => set_locale::SetLocale,
    }
}
//#endregion 🔖️Commands

//#region 🔖️ReasoningWiresPlayApp
/// 🧪️ B1: unit struct — every former `WiresPlayRuntime` field now lives in `WiresConfig`, written
/// through `WiresConfigMutation`s.
#[derive(Default)]
pub struct ReasoningWiresPlayApp;

impl ArtifactEditor for ReasoningWiresPlayApp {
    type Snapshot = WiresSnapshot;
    type Mutation = WiresMutation;
    type Config = WiresConfig;
    type ConfigMutation = WiresConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::wires::presence::WiresPresence;
    type PresenceMutation = crate::editor::wires::presence::WiresPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = WiresCommand;

    const DIALECT: Dialect = crate::artifacts::wires::WIRES_DIALECT;

    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::wires::MINDMAP_WIRES_SCHEMA;

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::wires::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> WiresSnapshot {
        crate::artifacts::wires::empty_wires_snapshot()
    }

    /// 🏷️ Supplied wholesale by `app_commands!`'s generated `command_id()`.
    fn command_id(command: &WiresCommand) -> &'static str {
        command.command_id()
    }

    /// 🕹️ `deleteSelection` reads the "graph" interaction domain directly (bypassing the
    /// `app_commands!`-generated `dispatch`, whose per-row `$module::handle(payload, doc, cfg)`
    /// signature is framework-fixed and has no `interaction` slot) — ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    fn handle(
        command: &WiresCommand,
        doc: &ArtifactView<'_, WiresSnapshot>,
        cfg: &ConfigView<'_, WiresConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<WiresMutation, WiresConfigMutation, Self::DraftMutation>, Fault> {
        match command {
            WiresCommand::DeleteSelection(payload) => delete_selection::apply(payload, doc, cfg, interaction),
            _ => command.dispatch(doc, cfg),
        }
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, WiresSnapshot>, cfg: &ConfigView<'_, WiresConfig>) -> UiNode {
        let document = doc.snapshot;
        let labels = semio_framework_plugin::resolve_labels_for_locale::<crate::editor::wires::terminology::WiresLabels>(&cfg.snapshot.locale);
        match body_key {
            WIRES_PLAY_BODY_COMPOSITE => edit::windows::canvas::render(&crate::artifacts::wires::wires_working_board(document), &document.wires_fixture),
            WIRES_PLAY_BODY_DOCUMENT => document_panel::render(document, labels),
            WIRES_PLAY_BODY_CATALOGUE => catalogue_panel::render(&document.wires_fixture, labels),
            WIRES_PLAY_BODY_PROPERTIES => inspection_panel::render(document),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️ReasoningWiresPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
///
/// 🚧️ SDK GAP (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.4/§7.4):
/// `EditorBuilder` has no `.example(...)`/`.workflow(...)` methods — `AppBuilder`'s `App { definition,
/// examples }` split means `.editor::<E>(def: AppDefinition)` only ever takes the definition, so the
/// old metabolism example registration and the `"reasoning-wires"` workflow tag are dropped here, not
/// silently lost. The subset's own `📚️examples/🎬️demo` facet is the documented replacement mechanism
/// for the former; `metabolism_wires_example_snapshot()` itself still lives on and is exercised
/// directly by this file's own tests below.
pub fn create_wires_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::wires::WIRES_DIALECT)
        .document(["semio", "reasoning", "mindmap", "wires"])
        .artifact_kind(crate::artifacts::wires::artifact_kind())
        .icon_id("reasoning-wires")
        .mode_def(edit::definition())
        .default_mode_id(edit::WIRES_PLAY_MODE_EDIT)
        .window_kind_def(edit::windows::canvas::definition())
        .default_layout(edit::layout())
        .panel_tab_def(document_panel::definition())
        .panel_tab_def(catalogue_panel::definition())
        .panel_tab_def(inspection_panel::definition())
        // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
        .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
        .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
        .mutation("addRelationship", LocalizedLabel::native("Add Relationship", "Beziehung hinzufügen"))
        .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
        .mutation("forceLayout", LocalizedLabel::native("Force Layout", "Kraftbasiertes Layout"))
        .mutation("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"))
        .mutation("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegt"))
        // 👁️ Ephemeral view state — in-flight drag. Selection/hover are framework-owned now
        // (domain "graph") — no app-declared verbs; `interactionSelect`/`interactionHover`/
        // `clearSelection`/`selectAll`/`setSelectionMode`/`setInteractionGranularity` auto-inject
        // below via `.interaction(...)`.
        .view_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"))
        .view_action("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"))
        // 🕹️ Domain "graph": identities (node) and relationships (edge) — `Flat` (the mindmap graph
        // has no parent/child structure to build a topology from, see `WIRES_INTERACTION_GRAPH`'s
        // doc comment); single-select, pick-only, replace-only merge (matches the pre-migration
        // click-to-select behaviour this crate hand-rolled).
        .interaction(InteractionDefinition {
            id: WIRES_INTERACTION_GRAPH.into(),
            label: LocalizedLabel::native("Graph", "Graph"),
            granularities: vec![
                GranularityDefinition { id: WIRES_GRANULARITY_NODE.into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() },
                GranularityDefinition { id: WIRES_GRANULARITY_EDGE.into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
            ],
            hierarchy: HierarchyProvider::Flat,
            hover: HoverSpec::default(),
            selection: SelectionSpec { modes: vec![SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
        })
        .window_kind_interactions(WIRES_PLAY_WINDOW_CANVAS, vec![InteractionRef::new(WIRES_INTERACTION_GRAPH)])
        // 🎯️ Typed channel surface (B1 pure-trait conversion) — `config_spec()`'s single source of
        // truth (the trait default `ConfigSpec::empty()`: none of `WiresConfig`'s fields are
        // user-visible settings, they're ephemeral view state) reused here rather than duplicated.
        .config(ReasoningWiresPlayApp::config_spec())
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as new_test_app, new_app_with_registry};
    use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type WiresApp = VcsArtifactApp<EditorApp<ReasoningWiresPlayApp>>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn new_app() -> WiresApp {
        new_test_app::<EditorApp<ReasoningWiresPlayApp>>()
    }

    /// 🧪️ Framework testkit gap (SDK GAP, see this ticket's `📓️w0-f-report.md` handoff #3):
    /// `assert_declared_actions_bridge_to_commands`/`new_app_with_registry` still take `fn() -> App`,
    /// unchanged for this ticket, while `create_wires_app` now returns `AppDefinition` — this tiny
    /// local wrapper bridges the two shapes with an empty `examples` list (dropped per `create_wires_app`'s
    /// own doc comment).
    fn wires_manifest_for_testkit() -> App {
        App { definition: create_wires_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — required to resolve the "graph" interaction
    /// domain's declaration when dispatching a framework-injected verb like `interactionSelect`.
    pub fn app_with_registry() -> WiresApp {
        new_app_with_registry::<EditorApp<ReasoningWiresPlayApp>>(wires_manifest_for_testkit)
    }

    /// 🧪️ An app pre-loaded with the metabolism example document, for tests exercising a populated board.
    pub fn metabolism_app() -> WiresApp {
        let mut app = new_app();
        let document = crate::artifacts::wires::schema::metabolism_wires_example_snapshot()
            .expect("valid metabolism fixture mutations");
        let envelope = store::create_document_envelope::<WiresSnapshot, WiresMutation>(crate::artifacts::wires::MINDMAP_WIRES_SCHEMA, "reasoning-wires", document, None);
        let files = store::print_document_pack(&envelope).expect("print document pack");
        app.load_document_pack(&files).expect("load metabolism");
        app
    }

    pub fn dispatch(app: &mut WiresApp, command: WiresCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut WiresApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::wires::testkit::{metabolism_app, new_app, render};
    use semio_framework_plugin::EditorApp;

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every
    /// row's wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[test]
    fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 10, "every WiresCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — pinned
    /// per-row from the `app_commands!` table's `"id" as "wire-key"` declarations rather than derived
    /// (several rows genuinely diverge from a naive kebab-case of the id: `setLocale` → `locale`,
    /// `setActiveExample` → `active-example`, and all three `canvasPointer*` rows drop the `canvas-`
    /// prefix). This is what a missing `#[dsl(keyword = ..)]` on a payload struct silently breaks (the
    /// record prints with no keyword at all and no longer parses).
    #[test]
    fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        let expected_keys = [
            ("setActiveExample", "active-example"),
            ("addNode", "add-node"),
            ("addRelationship", "add-relationship"),
            ("deleteSelection", "delete-selection"),
            ("forceLayout", "force-layout"),
            ("reorganize", "reorganize"),
            ("canvasPointerMove", "pointer-move"),
            ("canvasPointerDown", "pointer-down"),
            ("canvasPointerUp", "pointer-up"),
            ("setLocale", "locale"),
        ];
        for command in every_command() {
            let id = command.command_id();
            let expected = expected_keys.iter().find(|(row_id, _)| *row_id == id).map(|(_, key)| *key).unwrap_or_else(|| panic!("no expected wire key recorded for command {id}"));
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    /// ⚖️ The wire bytes/text pinned from the pre-merge 7-crate baseline (see the ticket's
    /// `🧪️wire-baseline-before.txt`) — a regression here is a real format break, not a fixture mismatch.
    /// `setSelection`/`documentSelect` dissolved into the framework's own "graph" interaction domain
    /// (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) and no longer exist as `WiresCommand`
    /// rows, which shifts every later row's binary ordinal by 2 — `CanvasPointerUp`'s and `SetLocale`'s
    /// pinned hex below are updated for the new ordinals (8 and 9); `SetActiveExample` is unaffected
    /// (ordinal 0, before the deleted rows).
    #[test]
    fn commands_keep_their_pre_migration_wire_bytes() {
        let node = dsl::to_dsl_value(&serde_json::json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] })).unwrap();
        let _ = node;
        let cases: [(WiresCommand, &str, &str); 3] = [
            (WiresCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "metabolism".into() }), "active-example active-example example-id=metabolism", "0100010a6d657461626f6c69736d01000600"),
            (WiresCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}), "pointer-up pointer-up", "01080000"),
            (WiresCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }), "locale locale value=de-DE", "0109010564652d444501000600"),
        ];
        for (command, text, hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text);
            assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) fn every_command() -> Vec<WiresCommand> {
        vec![
            WiresCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "metabolism".into() }),
            WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }),
            WiresCommand::AddRelationship(add_relationship::AddRelationship { kind: "owns".into() }),
            WiresCommand::DeleteSelection(delete_selection::DeleteSelection {}),
            WiresCommand::ForceLayout(force_layout::ForceLayout {}),
            WiresCommand::Reorganize(reorganize::Reorganize {}),
            WiresCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 1.5, y: -2.5 }),
            WiresCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { id: Some("node-1".into()), x: 10.0, y: 20.0 }),
            WiresCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}),
            WiresCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️Interaction
    /// 🕹️ The "graph" domain is declared `HierarchyProvider::Flat`, single-select/pick/replace-only,
    /// and scoped to the canvas window (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
    #[test]
    fn graph_interaction_domain_is_declared_flat_and_scoped_to_the_canvas_window() {
        let definition = create_wires_app();
        let graph = definition.interactions.iter().find(|interaction| interaction.id == WIRES_INTERACTION_GRAPH).expect("graph interaction domain declared");
        assert!(matches!(graph.hierarchy, HierarchyProvider::Flat));
        assert_eq!(graph.granularities.len(), 2);
        assert!(!graph.selection.transitive, "graph has no hierarchy to close a transitive selection over");
        let canvas_window = definition.window_kinds.iter().find(|window| window.id == WIRES_PLAY_WINDOW_CANVAS).expect("canvas window kind declared");
        assert!(canvas_window.interactions.iter().any(|interaction_ref| interaction_ref.as_str() == WIRES_INTERACTION_GRAPH), "canvas window must reference the graph interaction domain");
    }

    /// 🕹️ `wires_select_action_args` shapes the exact JSON the framework's `interactionSelect` action
    /// expects: `domainId`/`targets` (a JSON-stringified `Vec<InteractionTarget>`)/`merge`/`method`.
    #[test]
    fn wires_select_action_args_shapes_interaction_select_payload() {
        let args = wires_select_action_args(&["node-1".to_string()], WIRES_GRANULARITY_NODE, "replace");
        assert_eq!(args["domainId"], WIRES_INTERACTION_GRAPH);
        assert_eq!(args["merge"], "replace");
        assert_eq!(args["method"], "pick");
        assert!(args["targets"].as_str().expect("targets json").contains("node-1"));
        assert!(args["targets"].as_str().expect("targets json").contains(WIRES_GRANULARITY_NODE));
    }
    //#endregion 🔖️Interaction

    //#region 🔖️ManifestSanity
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_wires_app()).expect("app definition json");
        assert!(json.contains(WIRES_PLAY_WINDOW_CANVAS), "window kind missing from the manifest: {json}");
        assert!(json.contains(edit::WIRES_PLAY_MODE_EDIT), "mode missing from the manifest");
        for body in [WIRES_PLAY_BODY_DOCUMENT, WIRES_PLAY_BODY_CATALOGUE, WIRES_PLAY_BODY_PROPERTIES] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("graph.wires"), "artifact kind missing from the manifest");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    #[test]
    fn wires_labels_resolve_native_by_default() {
        let mut app = metabolism_app();
        let json = render(&mut app, WIRES_PLAY_BODY_DOCUMENT);
        assert!(json.contains("Identities") && json.contains("Relationships"));
        let catalogue_json = render(&mut app, WIRES_PLAY_BODY_CATALOGUE);
        assert!(catalogue_json.contains("Identity kinds"));
        assert!(catalogue_json.contains("Relationship kinds"));
    }

    #[test]
    fn metabolism_board_fixture_uses_mindmap_schema() {
        let document = crate::artifacts::wires::schema::metabolism_wires_example_snapshot()
            .expect("valid metabolism fixture mutations");
        let board = crate::artifacts::wires::wires_working_board(&document);
        assert_eq!(board.get("schema").and_then(|value| value.as_str()), Some(crate::artifacts::wires::MINDMAP_BOARD_SCHEMA));
        assert_eq!(crate::artifacts::wires::schema::fixture_nodes(&board).len(), 7);
    }

    #[test]
    fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        let mut app = new_app();
        assert!(render(&mut app, "reasoning.wires.nope").contains("Unknown body"));
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = new_app();
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(
            &mut app,
            WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }),
            |app| crate::artifacts::wires::schema::fixture_nodes(&crate::artifacts::wires::wires_working_board(&app.snapshot().expect("snapshot"))).len(),
            0,
            1,
        );
    }

    #[test]
    fn ingest_operations_is_idempotent() {
        semio_framework_plugin::testkit::assert_ingest_idempotent::<EditorApp<ReasoningWiresPlayApp>, usize>(WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }), |app| {
            crate::artifacts::wires::schema::fixture_nodes(&crate::artifacts::wires::wires_working_board(&app.snapshot().expect("snapshot"))).len()
        });
    }

    /// 🧪️ The definitional merge proof: A adds a node while B renames another node — disjoint edits
    /// on one backbone that must both survive on both instances (impossible under whole-document LWW).
    #[test]
    fn two_instances_converge_disjoint_graph_edits_via_backbone() {
        use crate::artifacts::wires::standards::v1::subsets::any::schema::inferences::find_board_node;
        use semio_framework_plugin::testkit::meta;
        use semio_framework_plugin::PluginApp;
        use store::MemoryBackbone;

        let mut instance_a = new_app();
        let mut instance_b = new_app();
        // Seed both from an identical base projection carrying node-1/node-2 (as initial state, not
        // as edits) so the only edits on the channel are A's and B's disjoint ones.
        let seed_node = |id: &str| dsl::to_dsl_value(&serde_json::json!({ "id": id, "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": id, "handles": [] })).expect("seed node");
        let mut base = crate::artifacts::wires::empty_wires_snapshot();
        base = store::apply_mutation(&base, &crate::artifacts::wires::mutations::create_node(seed_node("node-1")))
            .expect("valid mutation")
            .0;
        base = store::apply_mutation(&base, &crate::artifacts::wires::mutations::create_node(seed_node("node-2")))
            .expect("valid mutation")
            .0;
        let base_envelope = store::create_document_envelope::<WiresSnapshot, WiresMutation>(crate::artifacts::wires::MINDMAP_WIRES_SCHEMA, "reasoning-wires", base, None);
        let base_files = store::print_document_pack(&base_envelope).expect("print document pack");
        instance_a.load_document_pack(&base_files).expect("load a");
        instance_b.load_document_pack(&base_files).expect("load b");
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://mindmap-convergence", "mem://mindmap-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        // A adds node-3 (a new node); B moves node-2 (a PatchNode) — disjoint edits on the graph.
        instance_a.dispatch_typed(WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }), &meta("actor-a")).expect("a adds node");
        instance_b.dispatch_typed(WiresCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { id: Some("node-2".into()), x: 0.0, y: 0.0 }), &meta("actor-b")).expect("b down");
        instance_b.dispatch_typed(WiresCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { x: 50.0, y: 60.0 }), &meta("actor-b")).expect("b move");
        instance_b.dispatch_typed(WiresCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}), &meta("actor-b")).expect("b up");

        instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).expect("pump b");

        let projection_a = instance_a.snapshot().expect("projection a");
        let projection_b = instance_b.snapshot().expect("projection b");
        // A's added node-3 survives on both.
        assert!(find_board_node(&projection_a, "node-3").is_some(), "A keeps its own node");
        assert!(find_board_node(&projection_b, "node-3").is_some(), "B converges on A's node");
        // B's move of node-2 survives on both.
        let x_of = |document: &WiresSnapshot| find_board_node(document, "node-2").map(|node| crate::artifacts::wires::schema::node_position(&node)).unwrap().0;
        assert_eq!(x_of(&projection_a), 50.0, "A converges on B's move");
        assert_eq!(x_of(&projection_b), 50.0, "B keeps its own move");
    }
    //#endregion 🔖️CrossCutting
}
//#endregion 🧪️Tests
