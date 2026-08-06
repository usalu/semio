//! 🧮️ Mathematical play app — the `DocumentApp` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch. B1: the pure-trait pilot for this plugin — `MathematicalPlayApp` is a unit
//! struct; the former `MathPlayRuntime` app-struct `RefCell` (the node-graph viewport camera) now lives in
//! `crate::apps::mathematical::config::MathConfig`, written via `MathConfigOperation`s (real `backwards`,
//! no ad hoc inverse tracking); every action dispatches through the single typed `MathCommand` channel via
//! `DocumentApp::handle`.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, view state in `🎚️config/🦀️component.rs`, shared compute in the artifact's `⚙️engine`.
//! This file is a routing table: `handle` → `MathCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use crate::apps::mathematical::commands::document::set_document;
use crate::apps::mathematical::commands::geometry::set_points;
use crate::apps::mathematical::commands::graph::{node_graph_edit, node_graph_viewport, set_algorithm, set_directed};
use crate::apps::mathematical::commands::locale::set_locale;
use crate::apps::mathematical::config::{MathConfig, MathConfigOperation};
use crate::apps::mathematical::modes::edit;
use crate::apps::mathematical::modes::edit::windows::{geometry as geometry_window, graph as graph_window};
use crate::artifacts::mathematical::op::MathOperation;
use crate::artifacts::mathematical::{MathProjection, MATH_DOCUMENT_SCHEMA};
use semio_framework_plugin::{
    ui_text, ActionArgDef, ActionArgOption, App, ConfigView, DocumentApp, DocumentView, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, UiNode,
};
use store::DocumentPack;

//#region 🔖️Constants
pub const MATH_APP_ID: &str = "mathematical-play";
pub use geometry_window::MATH_PLAY_BODY_GEOMETRY;
pub use graph_window::MATH_PLAY_BODY_GRAPH;
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `MathematicalPlayApp::Command` — the SOLE dispatch surface for mathematical's own behavior,
    /// assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// kebab-case `#[dsl(key = ..)]` the binary/text codec uses) — they are genuinely different
    /// vocabularies; `setLocale`/`locale` is the row that proves it. **Row order is the binary variant
    /// ordinal: appending is safe, reordering is a wire-format break.**
    pub enum MathCommand for MathProjection, MathOperation, MathConfig, MathConfigOperation {
        "setDocument" as "set-document" => set_document::SetDocument,
        "setAlgorithm" as "set-algorithm" => set_algorithm::SetAlgorithm,
        "setDirected" as "set-directed" => set_directed::SetDirected,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "setPoints" as "set-points" => set_points::SetPoints,
        "setLocale" as "locale" => set_locale::SetLocale,
    }
}
//#endregion 🔖️Commands

//#region 🔖️MathematicalPlayApp
/// 🧪️ B1: unit struct — the former `MathPlayRuntime`/`self.runtime` field now lives in
/// `crate::apps::mathematical::config::MathConfig` (see `DocumentApp::Config`), written through
/// `MathConfigOperation`s.
#[derive(Default)]
pub struct MathematicalPlayApp;

impl DocumentApp for MathematicalPlayApp {
    type Projection = MathProjection;
    type Operation = MathOperation;
    type Config = MathConfig;
    type ConfigOperation = MathConfigOperation;
    type Command = MathCommand;

    fn app_id(&self) -> &str {
        MATH_APP_ID
    }

    fn document_schema(&self) -> &str {
        MATH_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> MathProjection {
        MathProjection::default()
    }

    fn io(&self) -> Option<semio_framework_plugin::AppIo> {
        Some(crate::artifacts::mathematical::engine::mathematical_io())
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`. `setLocale` has no manifest declaration (host-pushed,
    /// not a user-facing action).
    fn command_id(&self, command: &MathCommand) -> &str {
        command.command_id()
    }

    fn handle(&self, command: &MathCommand, doc: &DocumentView<'_, MathProjection>, cfg: &ConfigView<'_, MathConfig>) -> Result<Emit<MathOperation, MathConfigOperation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🎞️ `"result:out"` exports the active algorithm's per-node overlay (topo order/connected
    /// components/SCC group/BFS distance — the port recipe's `computation.mathematical`-kinded output);
    /// `"document:out"` replicates `DocumentApp::export_media`'s default whole-document-pack behavior
    /// (unreachable once this override exists).
    fn export_media(&self, port: &str, doc: &DocumentView<'_, MathProjection>) -> Result<Media, MediaError> {
        match port {
            "result:out" => {
                let overlay = crate::artifacts::mathematical::engine::algorithm_overlay(&doc.projection.graph);
                let json = serde_json::to_string(&serde_json::json!({ "algorithm": doc.projection.graph.algorithm, "overlay": overlay })).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.mathematical".into(), json } })
            }
            "document:out" => {
                let media_type = self.io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.projection.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, MathProjection>, cfg: &ConfigView<'_, MathConfig>) -> UiNode {
        match body_key {
            MATH_PLAY_BODY_GRAPH => graph_window::render(&doc.projection.graph, &cfg.projection.camera),
            MATH_PLAY_BODY_GEOMETRY => geometry_window::render(&doc.projection.geometry),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️MathematicalPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
pub fn create_mathematical_app() -> App {
    App::from_builder(
        App::builder(MATH_APP_ID, LocalizedLabel::native("Mathematical", "Mathematik"))
            .document(["semio", "mathematical"])
            .artifact_kind(crate::artifacts::mathematical::artifact_kind())
            .icon_id("math-app")
            .mode_def(edit::definition())
            .default_mode_id(edit::MATH_PLAY_MODE_EDIT)
            .window_kind_def(graph_window::definition())
            .window_kind_def(geometry_window::definition())
            .default_layout(edit::layout())
            // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
            .operation("setDocument", LocalizedLabel::native("Set Document", "Dokument festlegen"))
            .operation("setAlgorithm", LocalizedLabel::native("Set Algorithm", "Algorithmus festlegen"))
            .operation("setDirected", LocalizedLabel::native("Set Directed", "Gerichtet festlegen"))
            .operation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
            .view_action("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"))
            .operation("setPoints", LocalizedLabel::native("Set Points", "Punkte festlegen"))
            .view_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"))
            // 📝️ Staged argument forms for the graph analysis controls.
            .action_args("setAlgorithm", vec![
                ActionArgDef::select("algorithm", LocalizedLabel::native("Algorithm", "Algorithmus"), vec![
                    ActionArgOption::new("topo", LocalizedLabel::native("Topological Order", "Topologische Ordnung")),
                    ActionArgOption::new("components", LocalizedLabel::native("Connected Components", "Zusammenhangskomponenten")),
                    ActionArgOption::new("scc", LocalizedLabel::native("Strongly Connected Components", "Starke Zusammenhangskomponenten")),
                    ActionArgOption::new("bfs", LocalizedLabel::native("Breadth-First Distances", "Breitensuche-Distanzen")),
                ]).required(),
            ])
            .action_args("setDirected", vec![
                ActionArgDef::toggle("directed", LocalizedLabel::native("Directed", "Gerichtet")).default_value(true),
            ])
            // 🎯️ Typed channel surface (HEADLESS-APP-ENGINE-BINARY-COMMAND-PROTOCOL-FOUNDATIONS /
            // WORKFLOWS-END-TO-END-TYPED-PORTS) — `crate::artifacts::mathematical::engine::mathematical_io()`
            // is this port information's single source of truth, reused here rather than duplicated.
            .io(crate::artifacts::mathematical::engine::mathematical_io()),
    )
    .example("demo", LocalizedLabel::native("Demo", "Demo"), <MathProjection as store::DocumentDsl>::print_dsl(&MathProjection::default()), "cylinder")
    .workflow("mathematical", "Mathematical", "graph")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsDocumentApp, ViewState};

    pub type MathApp = VcsDocumentApp<MathematicalPlayApp>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn math_app() -> MathApp {
        new_app::<MathematicalPlayApp>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn math_app_with_registry() -> MathApp {
        new_app_with_registry::<MathematicalPlayApp>(create_mathematical_app)
    }

    pub fn dispatch(app: &mut MathApp, command: MathCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut MathApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewState::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::mathematical::testkit::{math_app, math_app_with_registry};

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
    /// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[test]
    fn command_ids_are_unique_and_the_full_row_set_is_covered() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 7, "every MathCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
    /// kebab-cased command id, except for the one documented divergence (`setLocale` → `locale`, an
    /// undeclared host-pushed command).
    #[test]
    fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        for command in every_command() {
            let id = command.command_id();
            let expected = if id == "setLocale" { "locale".to_string() } else { id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect() };
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) fn every_command() -> Vec<MathCommand> {
        vec![
            MathCommand::SetDocument(set_document::SetDocument { graph: crate::artifacts::mathematical::dsl::math_graph_to_dsl(&crate::artifacts::mathematical::MathGraph::default()), geometry: crate::artifacts::mathematical::MathGeometry::default() }),
            MathCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: "bfs".into(), seed: Some("a".into()) }),
            MathCommand::SetDirected(set_directed::SetDirected { directed: true }),
            MathCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: r#"[{"operation":"addNode","x":12.0,"y":34.0}]"#.into() }),
            MathCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { camera: crate::artifacts::mathematical::MathCamera { x: 5.0, y: 6.0, zoom: 2.0 } }),
            MathCommand::SetPoints(set_points::SetPoints { geometry: crate::artifacts::mathematical::MathGeometry::default() }),
            MathCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }

    /// ⚖️ The row whose `Option` field makes `None`/`Some` distinct wire cases, pinned to the exact bytes
    /// captured from the pre-merge `mathematical_protocol` crate (see the ticket's
    /// `🧪️wire-baseline-before.txt`).
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let cases: [(MathCommand, &str, &str); 2] = [
            (MathCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: "topo".into(), seed: None }), "set-algorithm algorithm=topo", "01010104746f706f01000600"),
            (MathCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: "bfs".into(), seed: Some("a".into()) }), "set-algorithm algorithm=bfs seed=a", "01010201610362667302000601010600"),
        ];
        for (command, text, hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text);
            assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
            store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_mathematical_app().definition).expect("app definition json");
        for id in [graph_window::MATH_PLAY_WINDOW_GRAPH, geometry_window::MATH_PLAY_WINDOW_GEOMETRY] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(edit::MATH_PLAY_MODE_EDIT), "mode missing from the manifest");
        assert!(json.contains("computation.mathematical"), "artifact kind missing from the manifest");
    }

    #[test]
    fn mathematical_io_is_declared_on_the_manifest() {
        let app = create_mathematical_app();
        assert_eq!(app.definition.io.artifact.id, "computation.mathematical");
        assert_eq!(app.definition.io.ports.len(), 1);
        assert_eq!(app.definition.io.ports[0].id, "result:out");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    #[test]
    fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::apps::mathematical::testkit::render;
        let mut app = math_app();
        assert!(render(&mut app, "mathematical.play.nope").contains("Unknown body"));
    }

    #[test]
    fn command_surface_is_registry_clean() {
        let _app = math_app_with_registry();
    }
    //#endregion 🔖️CrossCutting
}
//#endregion 🧪️Tests
