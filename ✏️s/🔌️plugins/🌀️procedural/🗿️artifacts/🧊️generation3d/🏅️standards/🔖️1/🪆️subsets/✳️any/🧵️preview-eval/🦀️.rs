//! 🧵️ Surface-neutral preview evaluation — the ONE read-only `previewEval` tool run both generation3d
//! surfaces run (`📋️tool-run-contract.md` §2, §3.7; ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS).
//!
//! ⏯️ The framework owns the lifecycle: a surface's `pending_effects` starts the run once an attached
//! preview window owes an evaluation, and start, pause, single step, abort and finalize are the
//! framework-reserved actions. [`PreviewEvalRunJob`] owns the scheduling: each algorithm unit is ONE
//! chain hop it hands the host through its [`ToolRunJobPort`] (`flowEvalTick` addressed at a preview
//! window), after which it waits until a hop settles. The hop advances the retained
//! [`FlowEvalSession`], publishes the evaluation into the addressed window's retained transient and
//! parks an [`semio_plugin::ExtensionInvocation`] (`evaluate` while operators are still
//! pending, then `tessellate`); `flowEvalResolve` / `flowTessellateResolve` fold the answers and wake
//! the job. Every observed node transition becomes a trace record keyed by the node's entity id. Abort
//! is host-driven by construction: the job's close quiesces the session and hands the host one
//! `flowEvalRelease` hop that reaches the geometry extension's own retained kernel jobs. No surface
//! may run a synchronous
//! `FlowEvalSession::tick` loop instead: the brep/math operators are CONTRIBUTED by the host at
//! runtime (`setContributions`) and are never linked into the guest, so an in-guest tick can only
//! ever fault (`📓️audit-window-inventory-2026-09-12.md` §4 P0 item 1).
//!
//! 🪆️ This module lives at the SUBSET level, beside `✏️editor` and `👁️viewer` rather than inside
//! either, because both run the identical chain and `policyViewerPurityBreaches` forbids a viewer
//! file reaching through `::editor::`. Everything a surface owns alone — its config type, its
//! interaction marks, its payload assembly, its window chrome — stays on that surface; everything
//! the chain itself is made of lives here exactly once (CLAUDE.md: repeated code MUST be close to
//! each other). Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.
#![allow(clippy::result_large_err)]

use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, StepContext};
use semio_s_artifact_procedural_3d_model::host_with_session;
use semio_os_flow::{FlowEvalPublication, FlowEvalSession};
use semio_plugin::ArtifactInstanceOperationOwnerHandle;
use semio_plugin::Effect;
use semio_plugin::ExtensionInvocation;
use semio_plugin::Fault;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::MeshData;
use semio_plugin::ToolDefinition;
use semio_plugin::ToolRunJobPort;
use semio_plugin::ToolRunView;
use semio_plugin::ViewModel;
use semio_framework_tool_run::{
    ToolRunCounter, ToolRunDefinition, ToolRunIdentity, ToolRunProgress, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunStepRing, ToolRunTickWriter, ToolRunTraceSubject, ToolRunVerdict, TOOL_RUN_ABORT_ACTION_ID, TOOL_RUN_ARG_GENERATION, TOOL_RUN_ARG_RUN_ID,
    TOOL_RUN_ARG_TOOL_ID, TOOL_RUN_FINALIZE_ACTION_ID, TOOL_RUN_START_ACTION_ID,
};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::{BTreeMap, HashSet};
#[path="📥️tessellation/🦀️.rs"]
mod tessellation_receiving;
pub use tessellation_receiving::{PreviewTessellateReceive,PreviewEvalReceive};

#[path = "🪟️window-addressing/🦀️.rs"]
mod window_addressing;
pub use window_addressing::*;

#[path = "🧊️preview-geometry/🦀️.rs"]
mod preview_geometry;
pub use preview_geometry::*;

#[path = "📦️preview-mesh-table/🦀️.rs"]
mod preview_mesh_table;
pub use preview_mesh_table::*;

#[path = "⏱️tick-and-resolve/🦀️.rs"]
mod tick_and_resolve;
pub use tick_and_resolve::*;

#[path = "📈️status-json/🦀️.rs"]
mod status_json;
pub use status_json::*;

//#region ⏱️Tick

pub use semio_os_flow::flow_inference_dependency_json as geometry_dependency_json;

//#endregion ⏱️Tick

//#region ⏯️Run
#[path = "⏯️tool-run/🦀️.rs"]
mod tool_run;
pub use tool_run::*;
//#endregion ⏯️Run

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

/// 🧭️ Which evaluation a window can publish and use to resolve preview geometry.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PreviewEvaluationSource {
    WithFallback,
    Current,
}

impl PreviewEvaluationSource {
    fn text(self, session: &FlowEvalSession) -> &str {
        match self {
            Self::WithFallback => session.painted_eval_json(),
            Self::Current => session.eval_json(),
        }
    }
}

/// 📃️ One parsed evaluation shared by the geometry, status and value projections of a preview.
pub struct PreviewEvaluation<'a> {
    source: &'a str,
    evaluation_source: PreviewEvaluationSource,
    document: semio_framework_pack_json::Value,
}

impl<'a> PreviewEvaluation<'a> {
    pub fn from_json(source: &'a str) -> Self {
        Self::from_source(source, PreviewEvaluationSource::WithFallback)
    }

    pub fn from_source(source: &'a str, evaluation_source: PreviewEvaluationSource) -> Self {
        let document = semio_framework_pack_json::parse(source, semio_framework_pack_json::JsonMemberPolicy::Reject)
            .unwrap_or(semio_framework_pack_json::Value::Null);
        Self { source, evaluation_source, document }
    }

    pub fn source(&self) -> &'a str {
        self.source
    }

    pub fn mesh_lookup<'session>(&self, session: &'session FlowEvalSession) -> PreviewMeshLookup<'session> {
        PreviewMeshLookup::for_source(session, self.evaluation_source)
    }

    pub fn geometry(&self) -> Option<&semio_framework_pack_json::Value> {
        self.document.as_object()?;
        self.document.get("error").and_then(semio_framework_pack_json::Value::as_str).is_none().then_some(&self.document)
    }
}

/// 🪟️ Carries the authoritative pointer window into the scene's hover projection.
pub fn stamp_preview_hover_source(object: &mut semio_framework_pack_json::Object, source: Option<&Option<String>>) {
    use semio_framework_pack_json::Value;
    if let Some(source) = source {
        object.insert("hoverSourceWindowId", source.clone().map_or(Value::Null, Value::String));
    }
}

/// 🗺️ Builds the shared Edit and View topology from authored widgets and visible canvas handles.
pub fn graph_interaction_topology(
    snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot,
    ports: &BTreeMap<String, Vec<String>>,
) -> semio_plugin::InteractionTopology {
    use semio_framework_artifact_flow_flow::{neural::Neuron, Widget};
    use semio_plugin::TopologyNode;
    use semio_s_artifact_procedural_3d_model::{widget_id, GraphTargetKind::{Edge, Handle, Node}};

    fn children(neuron: &Neuron, parent: &str, ordered: &mut Vec<TopologyNode>) {
        let id = Node.target(&neuron.id);
        ordered.push(TopologyNode { id: id.clone(), granularity: Node.granularity().into(), parent: Some(parent.into()) });
        if let Some(tree) = &neuron.tree {
            for child in &tree.neurons { children(child, &id, ordered); }
        }
    }

    let mut ordered = Vec::new();
    for widget in &snapshot.widgets {
        let raw = widget_id(widget);
        let id = Node.target(raw);
        ordered.push(TopologyNode { id: id.clone(), granularity: Node.granularity().into(), parent: None });
        for port in ports.get(raw).into_iter().flatten() {
            ordered.push(TopologyNode { id: Handle.target(port), granularity: Handle.granularity().into(), parent: Some(id.clone()) });
        }
        if let Widget::Cluster { tree, .. } = widget {
            for child in &tree.neurons { children(child, &id, &mut ordered); }
        }
    }
    for wire in &snapshot.synapses {
        ordered.push(TopologyNode { id: Edge.target(&wire.id), granularity: Edge.granularity().into(), parent: None });
    }
    semio_plugin::InteractionTopology { domains: [("graph".into(), semio_plugin::DomainTopology { ordered })].into() }
}

/// 🎯️ Raw IDs decoded once for matching surface geometry and authored wires.
#[derive(Default)]
pub struct PreviewGraphTargets<'a> {
    nodes: std::collections::BTreeSet<&'a str>,
    handles: std::collections::BTreeSet<&'a str>,
    edges: std::collections::BTreeSet<&'a str>,
}

impl<'a> PreviewGraphTargets<'a> {
    pub fn from_ids(ids: impl IntoIterator<Item = &'a String>) -> Self {
        use semio_s_artifact_procedural_3d_model::GraphTargetKind::{self, Edge, Handle, Node};
        let mut targets = Self::default();
        for id in ids {
            if let Some((kind, id)) = GraphTargetKind::parse(id) {
                match kind { Node => &mut targets.nodes, Handle => &mut targets.handles, Edge => &mut targets.edges }.insert(id);
            }
        }
        targets
    }

    pub fn marks(&self, widget: &str, port: &str) -> bool {
        self.nodes.contains(widget) || self.handles.contains(port)
    }
}

/// 🔌️ Geometry endpoints reached through a hovered wire or inlet.
#[derive(Default)]
pub struct PreviewWireHover<'a> {
    ports: std::collections::BTreeSet<String>,
    input_widgets: std::collections::BTreeSet<&'a str>,
    output_widgets: std::collections::BTreeSet<&'a str>,
}

impl<'a> PreviewWireHover<'a> {
    fn include_endpoint(&mut self, widget: &'a str, channel: &str, input: bool) {
        if channel.is_empty() {
            let widgets = if input { &mut self.input_widgets } else { &mut self.output_widgets };
            widgets.insert(widget);
        } else {
            self.ports.insert(semio_s_artifact_procedural_3d_model::graph_port_id(widget, channel, input));
        }
    }

    /// 🎯️ Checks the precise endpoint or a whole-dictionary endpoint on its own side.
    pub fn matches(&self, widget: &str, port: &str, input: bool) -> bool {
        let widgets = if input { &self.input_widgets } else { &self.output_widgets };
        widgets.contains(widget) || self.ports.contains(port)
    }
}

/// 🔗️ Projects hovered wires to both endpoints and hovered inlets to their producers.
pub fn preview_wire_hover<'a>(
    host: &'a semio_framework_artifact_flow_flow::FlowHostSnapshot,
    hovered: &PreviewGraphTargets<'_>,
) -> PreviewWireHover<'a> {
    let mut endpoints = PreviewWireHover::default();
    if hovered.edges.is_empty() && hovered.handles.is_empty() {
        return endpoints;
    }
    for wire in &host.synapses {
        if hovered.edges.contains(wire.id.as_str()) {
            endpoints.include_endpoint(&wire.from, &wire.from_port, false);
            endpoints.include_endpoint(&wire.to, &wire.to_port, true);
        } else if hovered.handles.contains(semio_s_artifact_procedural_3d_model::graph_port_id(&wire.to, &wire.to_port, true).as_str()) {
            endpoints.include_endpoint(&wire.from, &wire.from_port, false);
        }
    }
    endpoints
}

/// 🔦️ Resolves one published graph or component hover against directional channel IDs.
pub struct PreviewChannelHover<'a> {
    graph: PreviewGraphTargets<'a>,
    wires: PreviewWireHover<'a>,
    component_output: Option<String>,
}

impl<'a> PreviewChannelHover<'a> {
    pub fn new(
        host: &'a semio_framework_artifact_flow_flow::FlowHostSnapshot,
        graph_ids: &'a [String],
        component_output: Option<String>,
    ) -> Self {
        let graph = PreviewGraphTargets::from_ids(graph_ids);
        let wires = preview_wire_hover(host, &graph);
        Self { graph, wires, component_output: component_output.filter(|_| graph_ids.is_empty()) }
    }

    /// 🎯️ Tests the complete canonical handle, including its input or output direction.
    pub fn matches(&self, port_id: &str) -> bool {
        let Some(port) = semio_s_artifact_procedural_3d_model::parse_graph_port_id(port_id) else { return false };
        self.graph.marks(port.widget, port_id)
            || self.wires.matches(port.widget, port_id, port.input)
            || self.component_output.as_deref() == Some(port_id)
    }

    /// 📋️ Supplies the matching raw IDs in a presenter's own row order.
    pub fn matching<'row>(&self, candidates: impl IntoIterator<Item = &'row str>) -> Vec<String> {
        candidates.into_iter().filter(|id| self.matches(id)).map(str::to_owned).collect()
    }
}

/// 🔎️ Projects registered nodes and resolves each wire using its structured Flow endpoints.
pub fn dag_host_snapshot_to_workflow(host_snapshot: &semio_os_infinite_directed_presentation::DagHostSnapshot, synapses: &[semio_framework_artifact_flow_flow::SynapseSpec]) -> (Vec<semio_ui_declaration::ui::NodeGraphNodeRecord>, Vec<semio_ui_declaration::ui::NodeGraphEdgeRecord>) {
    use semio_s_artifact_procedural_3d_model::{graph_port_code, graph_port_id};
    let nodes: Vec<semio_ui_declaration::ui::NodeGraphNodeRecord> = host_snapshot
        .nodes
        .iter()
        .map(|node| semio_ui_declaration::ui::NodeGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| semio_ui_declaration::ui::NodeGraphPortRecord { id: graph_port_id(&node.id, &port.id, true), code: Some(port.id.clone()), label: Some(port.label.clone()), value_type: port.value_type.clone(), ..Default::default() }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| semio_ui_declaration::ui::NodeGraphPortRecord { id: graph_port_id(&node.id, &port.id, false), code: Some(port.id.clone()), label: Some(port.label.clone()), value_type: port.value_type.clone(), ..Default::default() }).collect(),
            ..Default::default()
        })
        .collect();
    let by_id: BTreeMap<_, _> = synapses.iter().map(|synapse| (synapse.id.as_str(), synapse)).collect();
    let edges = host_snapshot.edges.iter().filter_map(|edge| {
        let synapse = by_id.get(edge.id.as_str())?;
        let (source_node, source) = semio_os_infinite_directed_presentation::parse_dag_endpoint(&edge.source)?;
        let (target_node, target) = semio_os_infinite_directed_presentation::parse_dag_endpoint(&edge.target)?;
        if source_node != synapse.from || target_node != synapse.to { return None; }
        Some(semio_ui_declaration::ui::NodeGraphEdgeRecord {
            id: edge.id.clone(),
            source_node_id: synapse.from.clone(),
            source_port_id: graph_port_code(&source, false).into_owned(),
            target_node_id: synapse.to.clone(),
            target_port_id: graph_port_code(&target, true).into_owned(),
            label: None,
        })
    })
        .collect();
    (nodes, edges)
}

/// 🪪️ Reads exact retained channel packs without display filtering or another kernel evaluation.
pub fn export_meshes_from_evaluation(snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, session: &FlowEvalSession, widget_id: Option<&str>) -> Result<Vec<semio_plugin::MeshData>, semio_framework_diagnostic::TextError> {
    use semio_s_artifact_procedural_3d_mesh_bridge::io_error;
    if session.preview_chain_status().working || session.preview_eval_status().in_flight > 0 || session.preview_tessellate_status().in_flight > 0 { return Err(io_error("generation3d export is waiting for the current geometry evaluation")); }
    let sources = semio_s_artifact_procedural_3d_mesh_bridge::export_source_channels(snapshot, widget_id)?;
    let eval = semio_framework_pack_json::parse(session.eval_json(), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| io_error(error.to_string()))?;
    if let Some(error) = eval.get("error").and_then(semio_framework_pack_json::Value::as_str) { return Err(io_error(error)); }
    let mut meshes = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for (source, channel) in sources {
        for item in preview_channel_items_for_widget(&eval, &source).into_iter().filter(|item| channel.as_ref().is_none_or(|channel| *channel == item.channel)) {
            let key = if item.handle.is_empty() { format!("{source}@{}#{}", item.channel, item.index) } else { item.handle.clone() };
            if !seen.insert(key) { continue; }
            let mesh = match item.inline {
                Some(PreviewInlineGeometry::Vector { .. } | PreviewInlineGeometry::Plane { .. }) => continue,
                Some(inline) => inline_preview_mesh(&inline),
                None => session_preview_mesh(&item.handle, session).cloned(),
            }.ok_or_else(|| io_error(format!("generation3d export geometry '{source}@{}' is unavailable", item.channel)))?;
            meshes.push(mesh);
        }
    }
    if meshes.is_empty() { return Err(io_error("generation3d export has no connected geometry")); }
    Ok(meshes)
}
