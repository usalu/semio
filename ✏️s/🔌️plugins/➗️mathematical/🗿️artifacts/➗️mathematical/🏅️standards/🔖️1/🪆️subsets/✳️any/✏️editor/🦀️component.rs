//! 🧮️ Mathematical editor — `MathematicalPlayApp`'s `ArtifactEditor` impl (dispatch-only, ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1), the aggregated command enum and
//! the manifest stitch. B1: the pure-trait pilot for this plugin — `MathematicalPlayApp` is a unit
//! struct; the former `MathPlayRuntime` app-struct `RefCell` (the node-graph viewport camera) now lives in
//! `crate::editor::mathematical::config::MathematicalConfig`, written via `MathematicalConfigMutation`s (real
//! `backwards`, no ad hoc inverse tracking); every action dispatches through the single typed
//! `MathematicalCommand` channel via `ArtifactEditor::handle`.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, view state in `🎚️config/🦀️component.rs`. Shared compute with more than one
//! consumer across the taxonomy tree (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES dissolved
//! the former artifact-tree `⚙️engine`) lives HERE — `🔖️Io`, `🔖️Scene`, `🔖️GraphAlgorithms`, `🔖️Geometry` — since
//! an artifact is a `🧬️schema` + `🚪️io` system only, never an engine; behaviour belongs to the app.
//! This file is a routing table: `handle` → `MathematicalCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.
//!
//! The sibling read-only surface (`👁️viewer/🦀️component.rs`) never imports from this module — see
//! that file's own doc header.

use crate::editor::mathematical::commands::set_artifact;
use crate::editor::mathematical::commands::set_points;
use crate::editor::mathematical::commands::{node_graph_edit, node_graph_viewport, set_algorithm, set_directed};
use crate::editor::mathematical::commands::set_locale;
use crate::editor::mathematical::config::{MathematicalConfig, MathematicalConfigMutation};
use crate::editor::mathematical::presence::{MathematicalPresence, MathematicalPresenceMutation};
use crate::editor::mathematical::modes::edit;
use crate::editor::mathematical::modes::edit::windows::{geometry as geometry_window, graph as graph_window};
use crate::artifacts::mathematical::op::MathematicalMutation;
use crate::artifacts::mathematical::{MathematicalGeometry, MathematicalGraph, MathematicalSnapshot, MATHEMATICAL_DIALECT, MATH_DOCUMENT_SCHEMA};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{
    ArtifactEditor, Dialect, Editor, NoDraft, NoDraftMutation, DraftView, SurfaceKind, UiComponentSceneNode, UiPresence,
    ui_text, ActionArgDef, ActionArgOption, ConfigView, ArtifactView, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, UiNode,
};
use serde_json::{json, Value};
use store::EngineHandles;
use store::ArtifactPack;
use ui_wgpu::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord};

//#region 🔖️Constants
pub const MATH_APP_ID: &str = "mathematical-play";
pub use geometry_window::MATH_PLAY_BODY_GEOMETRY;
pub use graph_window::MATH_PLAY_BODY_GRAPH;
//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `create_mathematical_app` declares via `.artifact_kind(...)` (`computation.mathematical`), plus one
/// extra output port: `result:out`, the current graph+geometry projection as a generic data value
/// (WORKFLOWS-END-TO-END-TYPED-PORTS port recipe).
pub async fn mathematical_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: MATH_DOCUMENT_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
        ports: vec![semio_framework_plugin::MediaPortSpec {
            id: "result:out".into(),
            label: "Result".into(),
            direction: semio_framework_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            kind_id: Some("computation.mathematical".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        }],
        export_formats: Vec::new(),
        import_formats: Vec::new(),
        artifact: semio_framework_plugin::ArtifactPresentation { id: "computation.mathematical".into(), name: "Mathematical".into(), dimension: "graph".into(), component_kind: "mathematical".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️Scene
/// 🖼️ An empty `UiComponentSceneNode` shell for a body key, ready for its `node_graph`/`canvas_2d` field
/// to be filled in — shared by both `🎭️modes/✏️edit/🪟️windows/*` renderers.
pub async fn empty_component_scene(surface_id: &str, component_kind: SurfaceKind) -> UiComponentSceneNode {
    UiComponentSceneNode {
        surface_id: surface_id.into(),
        controller_id: MATH_APP_ID.into(),
        component_kind,
        pane_id: None,
        binding_id: None,
        presence: UiPresence::default(),
        canvas_2d: None,
        world_3d: None,
        node_graph: None,
        text_editor: None,
        table: None,
        paint_2d: None,
        virtual_file_system: None,
        tiled_map: None,
        board2d: None,
        icon_render: None,
        ink_canvas: None,
        graph_timeline: None,
        block_list: None,
        diff_view: None,
        event_feed: None,
        menu: None,
    }
}
//#endregion 🔖️Scene

//#region 🔖️GraphAlgorithms
/// 🕸️ Runs the selected algorithm over the current graph and returns a per-node label suffix overlay.
pub async fn algorithm_overlay(graph: &MathematicalGraph) -> std::collections::HashMap<String, String> {
    use graph::algorithms::{adjacency, bfs_distances, connected_components, strongly_connected_components, topo_sort, IdIndex};

    let index = IdIndex::from_ids(graph.nodes.iter().map(|n| n.id.as_str()));
    let edge_pairs: Vec<(usize, usize)> = graph.edges.iter().filter_map(|e| Some((index.index_of(&e.source)?, index.index_of(&e.target)?))).collect();
    let adj = adjacency(index.len(), &edge_pairs, graph.directed);
    let mut overlay = std::collections::HashMap::new();

    match graph.algorithm.as_str() {
        "topo" => match topo_sort(&adj) {
            Ok(order) => {
                for (rank, &i) in order.iter().enumerate() {
                    if let Some(id) = index.id_of(i) {
                        overlay.insert(id.to_string(), format!(" #{rank}"));
                    }
                }
            }
            Err(_) => {
                for node in &graph.nodes {
                    overlay.insert(node.id.clone(), " ⟲".into());
                }
            }
        },
        "components" => {
            for (i, label) in connected_components(&adj).into_iter().enumerate() {
                if let Some(id) = index.id_of(i) {
                    overlay.insert(id.to_string(), format!(" ⬤️{label}"));
                }
            }
        }
        "scc" => {
            for (group, component) in strongly_connected_components(&adj).into_iter().enumerate() {
                for i in component {
                    if let Some(id) = index.id_of(i) {
                        overlay.insert(id.to_string(), format!(" ⬤️{group}"));
                    }
                }
            }
        }
        "bfs" => {
            if let Some(seed) = graph.algorithm_seed.as_deref().and_then(|s| index.index_of(s)) {
                for (i, dist) in bfs_distances(&adj, seed).into_iter().enumerate() {
                    if let Some(id) = index.id_of(i) {
                        overlay.insert(id.to_string(), dist.map_or_else(|| " ∞".into(), |d| format!(" d{d}")));
                    }
                }
            }
        }
        _ => {}
    }
    overlay
}

pub async fn workflow_json(graph: &MathematicalGraph) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let overlay = algorithm_overlay(graph);
    let nodes: Vec<NodeGraphNodeRecord> = graph
        .nodes
        .iter()
        .map(|node| {
            let suffix = overlay.get(&node.id).cloned().unwrap_or_default();
            NodeGraphNodeRecord { id: node.id.clone(), label: Some(format!("{}{}", node.label, suffix)), x: node.x, y: node.y, width: 72.0, height: 40.0, inputs: Vec::new(), outputs: Vec::new(), ..Default::default() }
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> =
        graph.edges.iter().map(|edge| NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id: edge.source.clone(), source_port_id: "out".into(), target_node_id: edge.target.clone(), target_port_id: "in".into(), label: None }).collect();
    (nodes, edges)
}
//#endregion 🔖️GraphAlgorithms

//#region 🔖️Geometry
pub async fn geometry_layers_json(geometry: &MathematicalGeometry) -> String {
    let points: Vec<geometry::Point> = geometry.points.iter().map(|p| geometry::Point::new(p.x, p.y)).collect();
    let hull = geometry::convex_hull(&points);
    let centroid = geometry::polygon_centroid(&hull);

    let mut layers: Vec<Value> = Vec::new();
    for (i, p) in points.iter().enumerate() {
        layers.push(json!({ "kind": "circle", "id": format!("point-{i}"), "x": p.x() - 5.0, "y": p.y() - 5.0, "width": 10.0, "height": 10.0, "color": "#38bdf8" }));
    }
    if hull.len() >= 2 {
        let mut hull_points: Vec<[f64; 2]> = Vec::new();
        for i in 0..hull.len() {
            let a = hull[i];
            let b = hull[(i + 1) % hull.len()];
            hull_points.push([a.x(), a.y()]);
            hull_points.push([b.x(), b.y()]);
        }
        layers.push(json!({ "kind": "polyline", "id": "hull", "points": hull_points, "color": "#facc15" }));
    }
    layers.push(json!({ "kind": "circle", "id": "centroid", "x": centroid.x() - 4.0, "y": centroid.y() - 4.0, "width": 8.0, "height": 8.0, "color": "#f472b6" }));
    serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into())
}
//#endregion 🔖️Geometry

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `MathematicalPlayApp::Command` — the SOLE dispatch surface for mathematical's own behavior,
    /// assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// kebab-case `#[dsl(key = ..)]` the binary/text codec uses) — they are genuinely different
    /// vocabularies; `setLocale`/`locale` is the row that proves it. **Row order is the binary variant
    /// ordinal: appending is safe, reordering is a wire-format break.**
    pub enum MathematicalCommand for MathematicalSnapshot, MathematicalMutation, MathematicalConfig, MathematicalConfigMutation {
        "setDocument" as "set-artifact" => set_artifact::SetArtifact,
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
/// `crate::editor::mathematical::config::MathematicalConfig` (see `ArtifactEditor::Config`), written
/// through `MathematicalConfigMutation`s.
#[derive(Default)]
pub struct MathematicalPlayApp;

impl ArtifactEditor for MathematicalPlayApp {
    type Snapshot = MathematicalSnapshot;
    type Mutation = MathematicalMutation;
    type Config = MathematicalConfig;
    type ConfigMutation = MathematicalConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = MathematicalPresence;
    type PresenceMutation = MathematicalPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = MathematicalCommand;

    const DIALECT: Dialect = MATHEMATICAL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = MATH_DOCUMENT_SCHEMA;

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::mathematical::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> MathematicalSnapshot {
        MathematicalSnapshot::default()
    }

    async fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(mathematical_io())
    }

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`. `setLocale` has no manifest declaration (host-pushed,
    /// not a user-facing action).
    async fn command_id(command: &MathematicalCommand) -> &'static str {
        command.command_id()
    }

    async fn handle(command: &MathematicalCommand, doc: &ArtifactView<'_, MathematicalSnapshot>, cfg: &ConfigView<'_, MathematicalConfig>, _interaction: &InteractionView<'_>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<MathematicalMutation, MathematicalConfigMutation, Self::DraftMutation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🎞️ `"result:out"` exports the active algorithm's per-node overlay (topo order/connected
    /// components/SCC group/BFS distance — the port recipe's `computation.mathematical`-kinded output);
    /// `"document:out"` replicates `ArtifactApp::export_media`'s default whole-document-pack behavior
    /// (unreachable once this override exists).
    async fn export_media(port: &str, doc: &ArtifactView<'_, MathematicalSnapshot>) -> Result<Media, MediaError> {
        match port {
            "result:out" => {
                let graph = crate::artifacts::mathematical::mathematical_graph(doc.snapshot);
                let overlay = algorithm_overlay(&graph);
                let json = serde_json::to_string(&serde_json::json!({ "algorithm": graph.algorithm, "overlay": overlay })).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                Ok(Media { media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value }, payload: MediaPayload::Structured { schema: "computation.mathematical".into(), json } })
            }
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, MathematicalSnapshot>, cfg: &ConfigView<'_, MathematicalConfig>) -> UiNode {
        match body_key {
            MATH_PLAY_BODY_GRAPH => graph_window::render(&crate::artifacts::mathematical::mathematical_graph(doc.snapshot), &cfg.snapshot.camera),
            MATH_PLAY_BODY_GEOMETRY => geometry_window::render(&crate::artifacts::mathematical::mathematical_geometry(doc.snapshot)),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }
}
//#endregion 🔖️MathematicalPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline.
///
/// 🚧️ SDK GAP (contract §2.4): `EditorBuilder` has no `.example(...)`/`.workflow(...)` —
/// `PluginBuilder::editor::<E>(def: AppDefinition)` only takes the bare definition, so the old
/// `.example_source(crate::examples::art_mathematical_demo::source())` and
/// `.workflow("mathematical", "Mathematical", "graph")` calls are dropped here (not silently: noted
/// in the migration report). The subset's own `📚️examples/🎬️demo` facet
/// (`crate::artifacts::mathematical::examples::...`, real content, pre-existing) is the modern,
/// role-agnostic replacement surface for example registration.
pub async fn create_mathematical_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(MATHEMATICAL_DIALECT)
        .document(["semio", "mathematical"])
        .artifact_kind(crate::artifacts::mathematical::artifact_kind())
        .icon_id("math-app")
        .mode_def(edit::definition())
        .default_mode_id(edit::MATH_PLAY_MODE_EDIT)
        .window_kind_def(graph_window::definition())
        .window_kind_def(geometry_window::definition())
        .default_layout(edit::layout())
        // ✏️ Document-mutating actions — dispatched as VCS operations with true inverses.
        .mutation("setDocument", LocalizedLabel::native("Set Document", "Dokument festlegen"))
        .mutation("setAlgorithm", LocalizedLabel::native("Set Algorithm", "Algorithmus festlegen"))
        .mutation("setDirected", LocalizedLabel::native("Set Directed", "Gerichtet festlegen"))
        .mutation("nodeGraphEdit", LocalizedLabel::native("Node Graph Edit", "Knotengraph bearbeiten"))
        .view_action("nodeGraphViewport", LocalizedLabel::native("Node Graph Viewport", "Knotengraph-Ansicht"))
        .mutation("setPoints", LocalizedLabel::native("Set Points", "Punkte festlegen"))
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
        // WORKFLOWS-END-TO-END-TYPED-PORTS) — `mathematical_io()` (this file's own `🔖️Io` region) is
        // this port information's single source of truth, reused here rather than duplicated.
        .io(mathematical_io())
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
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    /// ✏️ `MathematicalPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<MathematicalPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<MathematicalPlayApp>` builds it.
    pub type MathApp = VcsArtifactApp<EditorApp<MathematicalPlayApp>>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub async fn math_app() -> MathApp {
        new_app::<EditorApp<MathematicalPlayApp>>()
    }

    /// ✏️ Adapts `create_mathematical_app`'s `AppDefinition` (contract §2.4) into the `App {
    /// definition, examples }` shape `testkit::assert_declared_actions_bridge_to_commands` still
    /// expects — framework testkit gap, not modifiable here (`🧰️framework/**` is outside this
    /// packet's lease).
    pub async fn mathematical_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_mathematical_app(), examples: Vec::new() }
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub async fn math_app_with_registry() -> MathApp {
        new_app_with_registry::<EditorApp<MathematicalPlayApp>>(mathematical_app_manifest_for_testkit)
    }

    pub async fn dispatch(app: &mut MathApp, command: MathematicalCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub async fn render(app: &mut MathApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::mathematical::testkit::{math_app, math_app_with_registry};

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
    /// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[test]
    async fn command_ids_are_unique_and_the_full_row_set_is_covered() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 7, "every MathematicalCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — the
    /// kebab-cased command id, except for the two documented divergences: `setLocale` → `locale`
    /// (an undeclared host-pushed command) and `setDocument` → `set-artifact` (the `app_commands!`
    /// row's own `"setDocument" as "set-artifact" => set_artifact::SetArtifact` explicitly pins a
    /// non-kebab wire keyword, matching `SetArtifact`'s own `#[dsl(keyword = "set-artifact")]`).
    /// **Pre-existing bug, independently traced**: `git log -1 --date=iso -- 🎮️commands/📄️set-artifact/
    /// 🦀️component.rs` shows `SetArtifact`'s explicit `set-artifact` keyword predates this ticket's
    /// own edits to this file (which only touched `render`/`export_media`); this test's hardcoded
    /// exception list simply never accounted for the second declared divergence. Fixed outright
    /// per this ticket's own "trivial, safe, unambiguous" guidance rather than left unresolved.
    #[test]
    async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        for command in every_command() {
            let id = command.command_id();
            let expected = match id {
                "setLocale" => "locale".to_string(),
                "setDocument" => "set-artifact".to_string(),
                _ => id.chars().flat_map(|c| if c.is_ascii_uppercase() { vec!['-', c.to_ascii_lowercase()] } else { vec![c] }).collect(),
            };
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected, "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) async fn every_command() -> Vec<MathematicalCommand> {
        vec![
            MathematicalCommand::SetArtifact(set_artifact::SetArtifact { graph: crate::artifacts::mathematical::dsl::math_graph_to_dsl(&crate::artifacts::mathematical::MathematicalGraph::default()), geometry: crate::artifacts::mathematical::MathematicalGeometry::default() }),
            MathematicalCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: "bfs".into(), seed: Some("a".into()) }),
            MathematicalCommand::SetDirected(set_directed::SetDirected { directed: true }),
            MathematicalCommand::NodeGraphEdit(node_graph_edit::NodeGraphEdit { operations_json: r#"[{"operation":"addNode","x":12.0,"y":34.0}]"#.into() }),
            MathematicalCommand::NodeGraphViewport(node_graph_viewport::NodeGraphViewport { camera: crate::artifacts::mathematical::MathematicalCamera { x: 5.0, y: 6.0, zoom: 2.0 } }),
            MathematicalCommand::SetPoints(set_points::SetPoints { geometry: crate::artifacts::mathematical::MathematicalGeometry::default() }),
            MathematicalCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
        ]
    }

    /// ⚖️ The row whose `Option` field makes `None`/`Some` distinct wire cases, pinned to the exact bytes
    /// captured from the pre-merge `mathematical_protocol` crate (see the ticket's
    /// `🧪️wire-baseline-before.txt`).
    #[test]
    async fn optional_field_rows_keep_their_pre_migration_bytes() {
        let cases: [(MathematicalCommand, &str, &str); 2] = [
            (MathematicalCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: "topo".into(), seed: None }), "set-algorithm algorithm=topo", "01010104746f706f01000600"),
            (MathematicalCommand::SetAlgorithm(set_algorithm::SetAlgorithm { algorithm: "bfs".into(), seed: Some("a".into()) }), "set-algorithm algorithm=bfs seed=a", "01010201610362667302000601010600"),
        ];
        for (command, text, hex) in cases {
            assert_eq!(protocol::OpText::print_op(&command), text);
            assert_eq!(protocol::OpBinary::encode_op(&command).expect("encode").iter().map(|b| format!("{b:02x}")).collect::<String>(), hex);
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_mathematical_app()).expect("app definition json");
        for id in [graph_window::MATH_PLAY_WINDOW_GRAPH, geometry_window::MATH_PLAY_WINDOW_GEOMETRY] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        assert!(json.contains(edit::MATH_PLAY_MODE_EDIT), "mode missing from the manifest");
        assert!(json.contains("computation.mathematical"), "artifact kind missing from the manifest");
    }

    #[test]
    async fn mathematical_io_is_declared_on_the_manifest() {
        let app = create_mathematical_app();
        assert_eq!(app.io.artifact.id, "computation.mathematical");
        assert_eq!(app.io.ports.len(), 1);
        assert_eq!(app.io.ports[0].id, "result:out");
    }

    #[test]
    async fn create_mathematical_app_builds_a_definition_for_the_editor_role() {
        let def = create_mathematical_app();
        assert_eq!(def.role, semio_framework::AppRole::Editor);
        assert_eq!(def.dialect, MATHEMATICAL_DIALECT.into());
    }

    #[test]
    async fn editor_dialect_matches_the_artifact_coordinate() {
        assert_eq!(<MathematicalPlayApp as ArtifactEditor>::DIALECT, MATHEMATICAL_DIALECT);
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    #[test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::editor::mathematical::testkit::render;
        let mut app = math_app();
        assert!(render(&mut app, "mathematical.play.nope").contains("Unknown body"));
    }

    #[test]
    async fn command_surface_is_registry_clean() {
        let _app = math_app_with_registry();
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️MathematicalIo
    #[test]
    async fn mathematical_io_declares_result_out_with_the_computation_mathematical_kind() {
        let io = mathematical_io();
        assert_eq!(io.document_schema, "semio.mathematical/v1");
        assert_eq!(io.artifact.id, "computation.mathematical");
        assert_eq!(io.ports.len(), 1);
        let port = &io.ports[0];
        assert_eq!(port.id, "result:out");
        assert_eq!(port.kind_id.as_deref(), Some("computation.mathematical"));
        assert_eq!(port.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(port.multiplicity, semio_framework::PortMultiplicity::Many);
        assert!(!port.required);
    }
    //#endregion 🔖️MathematicalIo

    //#region 🔖️GraphAlgorithms
    #[test]
    async fn topo_algorithm_overlay_orders_dag_nodes() {
        let graph = MathematicalGraph::default();
        let overlay = algorithm_overlay(&graph);
        assert!(overlay.get("a").unwrap().starts_with(" #0"));
        assert!(overlay.get("d").unwrap().starts_with(" #"));
    }

    #[test]
    async fn components_algorithm_overlay_groups_disconnected_node() {
        use crate::artifacts::mathematical::MathematicalNode;
        let mut graph = MathematicalGraph { algorithm: "components".into(), ..MathematicalGraph::default() };
        graph.nodes.push(MathematicalNode { id: "z".into(), label: "Z".into(), x: 0.0, y: 0.0 });
        let overlay = algorithm_overlay(&graph);
        assert_ne!(overlay.get("a"), overlay.get("z"));
    }

    #[test]
    async fn bfs_algorithm_overlay_reports_hop_distance() {
        let graph = MathematicalGraph { algorithm: "bfs".into(), algorithm_seed: Some("a".into()), ..MathematicalGraph::default() };
        let overlay = algorithm_overlay(&graph);
        assert_eq!(overlay.get("a").unwrap(), " d0");
        assert_eq!(overlay.get("b").unwrap(), " d1");
    }

    #[test]
    async fn workflow_json_round_trips_node_count() {
        let graph = MathematicalGraph::default();
        let (nodes, edges) = workflow_json(&graph);
        assert_eq!(nodes.len(), graph.nodes.len());
        assert_eq!(edges.len(), graph.edges.len());
    }
    //#endregion 🔖️GraphAlgorithms

    //#region 🔖️Geometry
    #[test]
    async fn geometry_layers_include_hull_and_centroid() {
        let geometry = MathematicalGeometry::default();
        let layers_json = geometry_layers_json(&geometry);
        assert!(layers_json.contains("\"hull\""));
        assert!(layers_json.contains("\"centroid\""));
    }
    //#endregion 🔖️Geometry
}
//#endregion 🧪️Tests
