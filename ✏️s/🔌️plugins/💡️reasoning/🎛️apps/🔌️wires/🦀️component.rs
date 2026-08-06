//! 🖥️ Wires play app — the `DocumentApp` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the window render
//! in `🎭️modes/✏️edit/🪟️windows/🕸️canvas`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view
//! state in `🦀️config.rs`, shared compute in the artifact's `⚙️engine`. This file is a routing table:
//! `handle` → `WiresCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls
//! one `definition()` per node.
//!
//! B1: `ReasoningWiresPlayApp` is a unit struct — every former `WiresPlayRuntime` field (selection,
//! in-flight drag) lives in `crate::apps::wires::config::WiresConfig`, written via
//! `crate::apps::wires::config::WiresConfigOperation`s (real `backwards`, no ad hoc runtime `RefCell`);
//! every action dispatches through the single typed `WiresCommand` channel via `DocumentApp::handle`.

use crate::apps::wires::commands::delete::delete_selection;
use crate::apps::wires::commands::example::{set_active_example, WIRES_PLAY_EXAMPLE_METABOLISM_ID};
use crate::apps::wires::commands::layout::{force_layout, reorganize};
use crate::apps::wires::commands::locale::set_locale;
use crate::apps::wires::commands::node::add_node;
use crate::apps::wires::commands::pointer::{canvas_pointer_down, canvas_pointer_move, canvas_pointer_up};
use crate::apps::wires::commands::relationship::add_relationship;
use crate::apps::wires::commands::selection::{document_select, set_selection};
use crate::apps::wires::config::{WiresConfig, WiresConfigOperation};
use crate::apps::wires::modes::edit;
use crate::apps::wires::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::artifacts::wires::op::MindmapWiresOperation;
use crate::artifacts::wires::MindmapWiresDocument;
use semio_framework_plugin::{NoDraft, NoDraftOperation, DraftView, ui_text, ActionDescriptor, App, ConfigView, DocumentApp, DocumentView, Emit, Fault, Label, LocalizedLabel, UiNode};
use store::EngineHandles;
use serde_json::Value;

//#region 🔖️Constants
pub const WIRES_PLAY_APP_ID: &str = "reasoning-wires-play";
pub use edit::windows::canvas::{WIRES_PLAY_BODY_COMPOSITE, WIRES_PLAY_WINDOW_CANVAS};
pub use catalogue_panel::WIRES_PLAY_BODY_CATALOGUE;
pub use document_panel::WIRES_PLAY_BODY_DOCUMENT;
pub use inspection_panel::WIRES_PLAY_BODY_PROPERTIES;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`) builds its `on_change`/item actions with.
pub fn wires_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(WIRES_PLAY_APP_ID).action(action, args)
}
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `ReasoningWiresPlayApp::Command` — the SOLE dispatch surface for this app's behavior,
    /// assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// kebab-case `#[dsl(key = ..)]` the binary/text codec uses) — they are genuinely different
    /// vocabularies; `setLocale`/`locale` is the row that proves it. **Row order is the binary variant
    /// ordinal: appending is safe, reordering is a wire-format break.**
    pub enum WiresCommand for MindmapWiresDocument, MindmapWiresOperation, WiresConfig, WiresConfigOperation {
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "addNode" as "add-node" => add_node::AddNode,
        "addRelationship" as "add-relationship" => add_relationship::AddRelationship,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "forceLayout" as "force-layout" => force_layout::ForceLayout,
        "reorganize" as "reorganize" => reorganize::Reorganize,
        "canvasPointerMove" as "pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "setSelection" as "set-selection" => set_selection::SetSelection,
        "documentSelect" as "document-select" => document_select::DocumentSelect,
        "canvasPointerDown" as "pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerUp" as "pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "setLocale" as "locale" => set_locale::SetLocale,
    }
}
//#endregion 🔖️Commands

//#region 🔖️ReasoningWiresPlayApp
/// 🧪️ B1: unit struct — every former `WiresPlayRuntime` field now lives in `WiresConfig`, written
/// through `WiresConfigOperation`s.
#[derive(Default)]
pub struct ReasoningWiresPlayApp;

impl DocumentApp for ReasoningWiresPlayApp {
    type Projection = MindmapWiresDocument;
    type Operation = MindmapWiresOperation;
    type Config = WiresConfig;
    type ConfigOperation = WiresConfigOperation;
    type Draft = NoDraft;
    type DraftOperation = NoDraftOperation;

    type Command = WiresCommand;

    const APP_ID: &'static str = WIRES_PLAY_APP_ID;

    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::wires::MINDMAP_WIRES_SCHEMA;

    fn initial_projection() -> MindmapWiresDocument {
        crate::artifacts::wires::empty_mindmap_wires_document()
    }

    /// 🏷️ Supplied wholesale by `app_commands!`'s generated `command_id()`.
    fn command_id(command: &WiresCommand) -> &'static str {
        command.command_id()
    }

    fn handle(command: &WiresCommand, doc: &DocumentView<'_, MindmapWiresDocument>, cfg: &ConfigView<'_, WiresConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<MindmapWiresOperation, WiresConfigOperation, Self::DraftOperation>, Fault> {
        command.dispatch(doc, cfg)
    }

    fn render(body_key: &str, doc: &DocumentView<'_, MindmapWiresDocument>, cfg: &ConfigView<'_, WiresConfig>) -> UiNode {
        let document = doc.projection;
        let labels = semio_framework_plugin::resolve_labels_for_locale::<crate::apps::wires::terminology::WiresLabels>(&cfg.projection.locale);
        match body_key {
            WIRES_PLAY_BODY_COMPOSITE => edit::windows::canvas::render(&document.board_fixture, &document.wires_fixture),
            WIRES_PLAY_BODY_DOCUMENT => document_panel::render(document, &cfg.projection.selected_ids, labels),
            WIRES_PLAY_BODY_CATALOGUE => catalogue_panel::render(&document.wires_fixture, labels),
            WIRES_PLAY_BODY_PROPERTIES => inspection_panel::render(document, &cfg.projection.selected_ids),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️ReasoningWiresPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
pub fn create_wires_app() -> App {
    App::from_builder(
        App::builder(WIRES_PLAY_APP_ID, LocalizedLabel::native("Mindmap Wires", "Mindmap-Wires"))
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
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .operation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .operation("addRelationship", LocalizedLabel::native("Add Relationship", "Beziehung hinzufügen"))
            .operation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
            .operation("forceLayout", LocalizedLabel::native("Force Layout", "Kraftbasiertes Layout"))
            .operation("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"))
            .operation("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegt"))
            // 👁️ Ephemeral view state — selection and in-flight drag.
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("documentSelect", LocalizedLabel::native("Document Select", "Dokument auswählen"))
            .view_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"))
            .view_action("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"))
            // 🎯️ Typed channel surface (B1 pure-trait conversion) — `config_spec()`'s single source of
            // truth (the trait default `ConfigSpec::empty()`: none of `WiresConfig`'s fields are
            // user-visible settings, they're ephemeral view state) reused here rather than duplicated.
            .config(ReasoningWiresPlayApp::config_spec()),
    )
    .example(WIRES_PLAY_EXAMPLE_METABOLISM_ID, LocalizedLabel::native("Metabolism", "Stoffwechsel"), serde_json::to_string(&crate::artifacts::wires::engine::metabolism_wires_example_document()).unwrap(), "network")
    .workflow("reasoning-wires", "Mindmap Wires", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app as new_test_app};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsDocumentApp, ViewModel};

    pub type WiresApp = VcsDocumentApp<ReasoningWiresPlayApp>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn new_app() -> WiresApp {
        new_test_app::<ReasoningWiresPlayApp>()
    }

    /// 🧪️ An app pre-loaded with the metabolism example document, for tests exercising a populated board.
    pub fn metabolism_app() -> WiresApp {
        let mut app = new_app();
        let document = crate::artifacts::wires::engine::metabolism_wires_example_document();
        let envelope = store::create_document_envelope::<MindmapWiresDocument, MindmapWiresOperation>(crate::artifacts::wires::MINDMAP_WIRES_SCHEMA, "reasoning-wires", document, None);
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
    use crate::apps::wires::testkit::{metabolism_app, new_app, render};

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
        assert_eq!(ids.len(), 12, "every WiresCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::test_support::assert_op_text_binary_equivalence(&command);
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
            ("setSelection", "set-selection"),
            ("documentSelect", "document-select"),
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
    #[test]
    fn commands_keep_their_pre_migration_wire_bytes() {
        let node = dsl::to_dsl_value(&serde_json::json!({ "id": "node-1", "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": "Alpha", "handles": [] })).unwrap();
        let _ = node;
        let cases: [(WiresCommand, &str, &str); 3] = [
            (WiresCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "metabolism".into() }), "active-example example-id=metabolism", "0100010a6d657461626f6c69736d01000600"),
            (WiresCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}), "pointer-up", "010a0000"),
            (WiresCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }), "locale value=de-DE", "010b010564652d444501000600"),
        ];
        for (command, text, hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text);
            assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
            store::test_support::assert_op_text_binary_equivalence(&command);
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
            WiresCommand::SetSelection(set_selection::SetSelection { ids: vec!["node-1".into(), "edge-1".into()] }),
            WiresCommand::DocumentSelect(document_select::DocumentSelect { ids: vec!["node-2".into()] }),
            WiresCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { id: Some("node-1".into()), x: 10.0, y: 20.0 }),
            WiresCommand::CanvasPointerUp(canvas_pointer_up::CanvasPointerUp {}),
            WiresCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_wires_app().definition).expect("app definition json");
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
        let document = crate::artifacts::wires::engine::metabolism_wires_example_document();
        assert_eq!(document.board_fixture.get("schema").and_then(|value| value.as_str()), Some(crate::artifacts::wires::MINDMAP_BOARD_SCHEMA));
        assert_eq!(crate::artifacts::wires::engine::fixture_nodes(&document.board_fixture).len(), 7);
    }

    #[test]
    fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        let mut app = new_app();
        assert!(render(&mut app, "reasoning.wires.nope").contains("Unknown body"));
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = new_app();
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(&mut app, WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }), |app| crate::artifacts::wires::engine::fixture_nodes(&app.projection().expect("projection").board_fixture).len(), 0, 1);
    }

    #[test]
    fn ingest_operations_is_idempotent() {
        semio_framework_plugin::testkit::assert_ingest_idempotent::<ReasoningWiresPlayApp, usize>(WiresCommand::AddNode(add_node::AddNode { kind: "identity".into() }), |app| crate::artifacts::wires::engine::fixture_nodes(&app.projection().expect("projection").board_fixture).len());
    }

    /// 🧪️ The definitional merge proof: A adds a node while B renames another node — disjoint edits
    /// on one backbone that must both survive on both instances (impossible under whole-document LWW).
    #[test]
    fn two_instances_converge_disjoint_graph_edits_via_backbone() {
        use crate::artifacts::wires::engine::find_board_node;
        use semio_framework_plugin::testkit::meta;
        use semio_framework_plugin::PluginApp;
        use store::MemoryBackbone;

        let mut instance_a = new_app();
        let mut instance_b = new_app();
        // Seed both from an identical base projection carrying node-1/node-2 (as initial state, not
        // as edits) so the only edits on the channel are A's and B's disjoint ones.
        let seed_node = |id: &str| dsl::to_dsl_value(&serde_json::json!({ "id": id, "nodeKind": "identity", "shape": "circle", "x": 0.0, "y": 0.0, "radius": 24.0, "text": id, "handles": [] })).expect("seed node");
        let mut base = crate::artifacts::wires::empty_mindmap_wires_document();
        base = store::apply_operation(&base, &MindmapWiresOperation::AddNode { node: seed_node("node-1") });
        base = store::apply_operation(&base, &MindmapWiresOperation::AddNode { node: seed_node("node-2") });
        let base_envelope = store::create_document_envelope::<MindmapWiresDocument, MindmapWiresOperation>(crate::artifacts::wires::MINDMAP_WIRES_SCHEMA, "reasoning-wires", base, None);
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

        let projection_a = instance_a.projection().expect("projection a");
        let projection_b = instance_b.projection().expect("projection b");
        // A's added node-3 survives on both.
        assert!(find_board_node(&projection_a, "node-3").is_some(), "A keeps its own node");
        assert!(find_board_node(&projection_b, "node-3").is_some(), "B converges on A's node");
        // B's move of node-2 survives on both.
        let x_of = |document: &MindmapWiresDocument| find_board_node(document, "node-2").map(crate::artifacts::wires::engine::node_position).unwrap().0;
        assert_eq!(x_of(&projection_a), 50.0, "A converges on B's move");
        assert_eq!(x_of(&projection_b), 50.0, "B keeps its own move");
    }
    //#endregion 🔖️CrossCutting
}
//#endregion 🧪️Tests
