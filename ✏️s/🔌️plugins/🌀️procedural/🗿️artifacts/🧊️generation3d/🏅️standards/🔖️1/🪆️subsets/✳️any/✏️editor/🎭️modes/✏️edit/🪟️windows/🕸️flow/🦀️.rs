//! 🕸️ Generation3d play app — the main flow-graph window (edit mode).

use crate::editor::generation3d::config::Generation3dConfig;
use crate::editor::generation3d::terminology::Generation3dLabels;
use crate::editor::generation3d::PreviewInteractionMarks;
use crate::editor::generation3d::GENERATION_3D_INTERACTION_CHANNEL;
use crate::editor::generation3d::GENERATION_3D_INTERACTION_DOMAIN;
use crate::editor::generation3d::GENERATION_3D_PLAY_APP_ID;
use crate::standards::v1::subsets::any::schema::{fixture_to_workflow, with_host};
use crate::Generation3dSnapshot;
use semio_framework_os_flow::{flow_backed_node_graph_extras, FlowEvalSession};
use semio_framework_os_kernel::Viewport2d;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, HasStackLayout, Trigger, UiAssemblyResult};
use semio_framework_plugin::{tree_item, tree_item_desc, ActionFactory, BuiltNode, LocalizedLabel, NodeGraphHover, NodeGraphScene, PanelTreeBuilder, PluginAssemblyError, SurfaceKind, WindowKindDefinition, WindowMeasure, WindowOptions};
use semio_framework_ui::wgpu::{NodeGraphEdgeRecord, NodeGraphFindItem, NodeGraphNodeRecord, NodeGraphOperatorChannelRecord, NodeGraphOperatorRecord, NodeGraphPortRecord};

//#region 🔖️Constants
pub const GENERATION_3D_PLAY_WINDOW_MAIN: &str = "procedural-main";
pub const GENERATION_3D_PLAY_BODY_MAIN: &str = "procedural.play.main";
const GENERATION_3D_PLAY_SURFACE_MAIN: &str = "procedural.play";
const GENERATION_3D_PLAY_OUTLINE_MAIN: &str = "procedural-play-graph";
const GENERATION_3D_PLAY_OUTLINE_NODES: &str = "procedural-play-graph.nodes";
const GENERATION_3D_PLAY_OUTLINE_WIRES: &str = "procedural-play-graph.wires";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: GENERATION_3D_PLAY_WINDOW_MAIN.into(),
        label: LocalizedLabel::native("Flow", "Workflow"),
        body_key: GENERATION_3D_PLAY_BODY_MAIN.into(),
        surface_kind: SurfaceKind::NodeGraph,
        icon_id: "flow-graph".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The LOD chrome measure for this window — collected fresh per frame, never frozen into the manifest.
pub fn window_measures(lod_mode: &str, on_change: impl Fn(&str, Option<serde_json::Value>) -> semio_framework_plugin::ActionDescriptor) -> Vec<WindowMeasure> {
    let current = if lod_mode.is_empty() { "medium" } else { lod_mode };
    vec![WindowMeasure::Select {
        id: "generation3d-measure-lod".into(),
        label: Some("LOD".into()),
        value: current.into(),
        // 🎚️ Rows come from the ONE ladder `cycleLodMode` walks, so the keyboard cycle and the
        // picker can never disagree (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
        items: crate::editor::generation3d::config::GENERATION_3D_LOD_MODES
            .iter()
            .map(|mode| {
                let label = match *mode {
                    "coarse" => "Coarse",
                    "fine" => "Fine",
                    _ => "Medium",
                };
                semio_framework_plugin::MeasureSelectItem { id: format!("generation3d-measure-lod-{mode}"), value: (*mode).into(), label: label.into() }
            })
            .collect(),
        on_change: on_change("setLodMode", None),
    }]
}
//#endregion 🔖️Definition

//#region 🔖️Outline
/// 🎯️ One `graph`-domain target batch in the JSON shape `interactionSelect`/`interactionHover` decode
/// (`parse_interaction_targets` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`) — a renderer
/// serialises its whole pick/hover batch into the single `targets` text arg, and a semantic row is just
/// a batch of one.
fn graph_targets_json(granularity: &str, id: &str) -> String {
    let mut target = dsl::json::Object::new();
    target.insert("granularity", dsl::json::Value::String(granularity.to_string()));
    target.insert("id", dsl::json::Value::String(id.to_string()));
    dsl::json::to_string(&dsl::json::Value::Array(vec![dsl::json::Value::Object(target)]))
}

/// 🚦 The `NodeEvalStatus` tag `flow_backed_node_graph_extras` reported for one widget, localized —
/// `build_flow_status_json` keys `{"<widgetId>": {"status": "ok" | "stale" | …}}`.
fn node_status_label(status_json: Option<&String>, node_id: &str, labels: &Generation3dLabels) -> Option<&'static str> {
    let parsed = dsl::json::parse(status_json?.as_str()).ok()?;
    let status = parsed.get(node_id)?.get("status")?.as_str()?;
    Some(match status {
        "stale" => labels.status_stale.as_str(),
        "queued" => labels.status_queued.as_str(),
        "computing" => labels.status_computing.as_str(),
        "error" => labels.status_error.as_str(),
        "blocked" => labels.status_blocked.as_str(),
        _ => labels.status_ok.as_str(),
    })
}

/// 🔌️ A node's ports as one row apiece, keyed by the topology's own `{nodeId}@{portId}` handle id
/// (`generation3d_port_ids_by_node`) so a port row and the canvas handle it mirrors are the same
/// `graph`-domain target. A port id is unique per node, NOT per direction — the packaged column
/// example's `extrusion-axis` carries `vector`/`x`/`y`/`z` as both an input and an output — so a port
/// reached from both sides is ONE row carrying both directions, which is exactly how many targets the
/// interaction topology has for it.
fn node_ports<'a>(node: &'a NodeGraphNodeRecord, labels: &Generation3dLabels) -> Vec<(&'a NodeGraphPortRecord, String)> {
    let mut ports: Vec<(&NodeGraphPortRecord, String)> = Vec::new();
    for (port, direction) in node.inputs.iter().map(|port| (port, labels.graph_input_port.as_str())).chain(node.outputs.iter().map(|port| (port, labels.graph_output_port.as_str()))) {
        match ports.iter_mut().find(|(known, _)| known.id == port.id) {
            Some((_, directions)) if !directions.contains(direction) => {
                directions.push_str(" · ");
                directions.push_str(direction);
            }
            Some(_) => {}
            None => ports.push((port, direction.to_string())),
        }
    }
    ports
}

fn port_row(port: &NodeGraphPortRecord, directions: String) -> UiAssemblyResult<BuiltNode> {
    let name = port.label.as_deref().filter(|label| !label.is_empty()).unwrap_or(port.id.as_str());
    tree_item_desc(&port.id, name, Some(directions))
}

/// 🧩️ One node row: the widget's own id verbatim (the `node` granularity target), its ports nested as
/// `handle` rows, and both interaction verbs bound so a pointer over the row and a click on it reach the
/// very same domain the canvas picks into.
fn node_row(node: &NodeGraphNodeRecord, status: Option<&'static str>, labels: &Generation3dLabels, factory: &ActionFactory) -> UiAssemblyResult<BuiltNode> {
    let ports = crate::ui_node_list(node_ports(node, labels).into_iter().map(|(port, directions)| port_row(port, directions)))?;
    let targets = graph_targets_json("node", &node.id);
    let (select, select_args) = factory.action(
        semio_framework_plugin::INTERACTION_SELECT_ACTION_ID,
        Some(crate::ui_value_map([("domainId", crate::ui_value_text(GENERATION_3D_INTERACTION_DOMAIN)?), ("merge", crate::ui_value_text("replace")?), ("method", crate::ui_value_text("pick")?), ("targets", crate::ui_value_text(&targets)?)])?),
    )?;
    let (hover, hover_args) = factory.action(
        semio_framework_plugin::INTERACTION_HOVER_ACTION_ID,
        Some(crate::ui_value_map([("channel", crate::ui_value_text(GENERATION_3D_INTERACTION_CHANNEL)?), ("domainId", crate::ui_value_text(GENERATION_3D_INTERACTION_DOMAIN)?), ("targets", crate::ui_value_text(&targets)?)])?),
    )?;
    let builder = semio_framework_ui_contract::tree_item(crate::ui_label(node.label.as_deref().unwrap_or(node.id.as_str()))?).default_open(true);
    let mut builder = builder.try_id(&node.id).map_err(|_| outline_error("node-row.id"))?;
    if let Some(status) = status {
        builder = builder.description(crate::ui_text(status)?);
    }
    builder = builder.try_on_with(Trigger::Activate, select, select_args.ok_or_else(|| outline_error("node-row.select-args"))?).map_err(|_| outline_error("node-row.select"))?;
    builder = builder.try_on_with(Trigger::HoverPreview, hover, hover_args.ok_or_else(|| outline_error("node-row.hover-args"))?).map_err(|_| outline_error("node-row.hover"))?;
    builder.try_children(ports).map_err(|_| outline_error("node-row.ports"))?.try_build().map_err(|_| outline_error("node-row.build"))
}

/// 🔗️ One wire row, keyed by the synapse id the `edge` granularity carries, labelled with both
/// endpoints so the graph's topology reads off the outline without the canvas.
fn wire_row(edge: &NodeGraphEdgeRecord) -> UiAssemblyResult<BuiltNode> {
    tree_item(&edge.id, format!("{}@{} → {}@{}", edge.source_node_id, edge.source_port_id, edge.target_node_id, edge.target_port_id))
}

/// 🕸️ The flow graph as semantic UI: every node of the open document with its ports, plus every wire
/// between them. The node-graph surface beside it paints the same records on the GPU, which no
/// screen-reader, no accessibility tree, and no headless probe can read — this tree is the graph's
/// renderer-neutral body, and the one the framework's `graph` interaction domain drives.
fn graph_outline(nodes: &[NodeGraphNodeRecord], edges: &[NodeGraphEdgeRecord], status_json: Option<&String>, labels: &Generation3dLabels) -> UiAssemblyResult<BuiltNode> {
    let factory = ActionFactory::new(GENERATION_3D_PLAY_APP_ID);
    let node_rows = crate::ui_node_list(nodes.iter().map(|node| node_row(node, node_status_label(status_json, &node.id, labels), labels, &factory)))?;
    let wire_rows = crate::ui_node_list(edges.iter().map(wire_row))?;
    PanelTreeBuilder::new(GENERATION_3D_PLAY_OUTLINE_MAIN)?
        .section_or_placeholder(GENERATION_3D_PLAY_OUTLINE_NODES, Some(crate::ui_label(labels.graph_nodes.as_str())?), true, node_rows, labels.graph_empty.as_str())?
        .section_or_placeholder(GENERATION_3D_PLAY_OUTLINE_WIRES, Some(crate::ui_label(labels.graph_wires.as_str())?), true, wire_rows, labels.graph_unwired.as_str())?
        .interaction_domain(GENERATION_3D_INTERACTION_DOMAIN)?
        .build()
}

fn outline_error(scope: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.flow-window", scope)
}
//#endregion 🔖️Outline


/// 🔌️ Channel code the operator catalogue speaks — `{nodeId}@{portId}` on a scene port, bare `portId` on a kind.
fn operator_channel_code(port_id: &str) -> &str {
    port_id.rsplit_once('@').map(|(_, code)| code).filter(|code| !code.is_empty()).unwrap_or(port_id)
}

fn operator_channel(code: &str, label: &str) -> NodeGraphOperatorChannelRecord {
    NodeGraphOperatorChannelRecord {
        code: code.to_string(),
        abbreviation: code.to_string(),
        name: label.to_string(),
        full_name: label.to_string(),
        operators: Vec::new(),
        default_json: None,
        label: Some(label.to_string()),
        cardinality: "!".into(),
    }
}

fn kind_extension_name(kind: &str) -> (String, String) {
    match kind.rsplit_once('.') {
        Some((extension, name)) => (extension.to_string(), name.to_string()),
        None => (String::new(), kind.to_string()),
    }
}

/// 🛍️ Document-derived operator records — one per neuron KIND the open graph actually holds, the way
/// the OS workflow window seeds `NodeGraphScene.operators` so the canvas can lay ports out without
/// waiting for the app-static catalogue (which may not have been contributed yet). Built-in `core.*`
/// widgets are omitted: the engine already knows them. Never the registered catalogue.
fn document_operator_records(dag: &semio_framework_artifact_infinite_dag::DagFixture, nodes: &[NodeGraphNodeRecord]) -> Vec<NodeGraphOperatorRecord> {
    let catalogue = semio_framework_os_flow::flow_operator_catalogue_records();
    let node_by_id: std::collections::BTreeMap<&str, &NodeGraphNodeRecord> = nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let mut kinds: Vec<String> = Vec::new();
    for spec in &dag.nodes {
        let Some(kind) = spec.operator_kind.as_deref().filter(|kind| !kind.is_empty() && !kind.starts_with("core.")) else { continue };
        if !kinds.iter().any(|known| known == kind) {
            kinds.push(kind.to_string());
        }
    }
    kinds
        .into_iter()
        .map(|kind| {
            if let Some(record) = catalogue.iter().find(|record| record.id == kind) {
                return record.clone();
            }
            let mut inputs = Vec::new();
            let mut outputs = Vec::new();
            let mut seen_in = std::collections::BTreeSet::new();
            let mut seen_out = std::collections::BTreeSet::new();
            for spec in &dag.nodes {
                if spec.operator_kind.as_deref() != Some(kind.as_str()) {
                    continue;
                }
                let Some(node) = node_by_id.get(spec.id.as_str()) else { continue };
                for port in &node.inputs {
                    let code = operator_channel_code(&port.id).to_string();
                    if seen_in.insert(code.clone()) {
                        inputs.push(operator_channel(&code, port.label.as_deref().unwrap_or(&code)));
                    }
                }
                for port in &node.outputs {
                    let code = operator_channel_code(&port.id).to_string();
                    if seen_out.insert(code.clone()) {
                        outputs.push(operator_channel(&code, port.label.as_deref().unwrap_or(&code)));
                    }
                }
            }
            let (extension, name) = kind_extension_name(&kind);
            NodeGraphOperatorRecord {
                id: kind,
                extension: extension.clone(),
                name: name.clone(),
                abbreviation: name.chars().next().map(|ch| ch.to_uppercase().to_string()).unwrap_or_else(|| "?".into()),
                icon: "box".into(),
                summary: String::new(),
                inputs,
                outputs,
                variadic_input: None,
                variadic_output: None,
                group: if extension.is_empty() { Vec::new() } else { vec![extension] },
            }
        })
        .collect()
}

fn graph_find_items(nodes: &[NodeGraphNodeRecord], category: &str) -> Vec<NodeGraphFindItem> {
    nodes
        .iter()
        .map(|node| NodeGraphFindItem { id: node.id.clone(), label: node.label.clone().unwrap_or_else(|| node.id.clone()), category: category.into() })
        .collect()
}

//#region 🔖️Render
pub fn render(document: &Generation3dSnapshot, config: &Generation3dConfig, session: &FlowEvalSession, marks: &PreviewInteractionMarks, labels: &Generation3dLabels) -> UiAssemblyResult<BuiltNode> {
    let fixture = &document.fixture;
    let (nodes, edges, operators) = with_host(fixture, |host| {
        let (nodes, edges) = fixture_to_workflow(&host.dag.fixture);
        let operators = document_operator_records(&host.dag.fixture, &nodes);
        (nodes, edges, operators)
    });
    let viewport = Viewport2d { x: fixture.camera.x, y: fixture.camera.y, zoom: fixture.camera.zoom };
    let flow_extras = flow_backed_node_graph_extras(fixture, &config.lod_mode, 0.0, true, false, semio_framework_ui_styling::metrics::board::GRID_FACTOR_DEFAULT, Some(session));
    let hover = marks.hovered_graph_target().map(|(node_id, port_id)| NodeGraphHover { node_id: Some(node_id), port_id });
    let outline = graph_outline(&nodes, &edges, flow_extras.status_json.as_ref(), labels)?;
    let surface = crate::scene_surface(
        GENERATION_3D_PLAY_SURFACE_MAIN,
        semio_framework_ui_contract::SurfaceKind::NodeGraph,
        &NodeGraphScene {
            editable: Some(true),
            capabilities_json: flow_extras.capabilities_json,
            lod_json: flow_extras.lod_json,
            fixture_json: flow_extras.fixture_json,
            eval_json: flow_extras.eval_json,
            status_json: flow_extras.status_json,
            operators,
            find_items: graph_find_items(&nodes, labels.graph_nodes.as_str()),
            selection: marks.graph_selection_ids(),
            highlighted: marks.graph_highlight_ids(),
            hover,
            ..NodeGraphScene::base(nodes, edges, viewport)
        },
    )?;
    let canvas = semio_framework_ui_contract::column()
        .grow(true)
        .try_id("procedural-play-main.canvas")
        .map_err(|_| outline_error("body.canvas-id"))?
        .try_child(surface)
        .map_err(|_| outline_error("body.canvas-child"))?
        .try_build()
        .map_err(|_| outline_error("body.canvas"))?;
    semio_framework_ui_contract::row()
        .grow(true)
        .try_id("procedural-play-main.body")
        .map_err(|_| outline_error("body.id"))?
        .try_child(outline)
        .map_err(|_| outline_error("body.outline"))?
        .try_child(canvas)
        .map_err(|_| outline_error("body.canvas-slot"))?
        .try_build()
        .map_err(|_| outline_error("body.build"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "./🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
