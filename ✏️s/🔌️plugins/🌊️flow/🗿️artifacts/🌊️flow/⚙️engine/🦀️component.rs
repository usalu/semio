//! ⚙️ Flow artifact — headless compute over the `FlowSnapshot` projection (constitutional: engine).
//!
//! Everything here is pure over `flow` types (`FlowHost`, `Widget`, `DagFixture`) plus the app's
//! view `FlowConfig`, and takes no label (`FlowPlayLabels`) parameter — labels are app chrome. The rule
//! for what lands here rather than next to a single caller: a helper with MORE THAN ONE consumer across
//! the taxonomy tree lives here; a helper with exactly one consumer lives in that consumer's component
//! file.

use crate::apps::flow::config::FlowConfig;
use crate::artifacts::flow::op::FlowMutation;
use crate::artifacts::flow::FlowSnapshot;
use flow::{
    dag::{DagDrawLod, DagFixture},
    flow_fixture_operations, flow_host_with_session,
    CameraJson, FlowEvalSession, FlowHost, Widget, FLOW_LOD_MODE_AUTOMATIC,
};
use semio_framework_plugin::HostEffect;
use serde_json::{json, Value};
use ui_wgpu::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};

//#region 🔖️Constants
pub const FLOW_WIDGET_DRAG_MIME: &str = "application/x-flow-widget";
/// 🖱️ Default proximity-select distance.
pub const FLOW_DEFAULT_PROXIMITY_DISTANCE: f64 = 48.0;
/// 🔳️ Default canvas grid factor.
pub const FLOW_DEFAULT_GRID_FACTOR: f64 = 10.0;
/// 🧵️ The self-chaining action id of the off-main-thread evaluation loop — dispatched as a
/// `HostEffect` by `🎮️commands/🧮️eval` and by `FlowPlayApp::pending_effects`.
pub const FLOW_EVAL_TICK_ACTION: &str = "flowEvalTick";
//#endregion 🔖️Constants

//#region 🔖️Register
/// 🗂️ Registers `FlowSnapshot`'s pack↔dsl codec under `FLOW_DOCUMENT_SCHEMA` so `framework/sync`'s folder
/// endpoints and any other schema-string-keyed caller can print/parse flow documents. Called from the
/// plugin root's `semio_plugin!{ setup: … }`.
pub fn register() {
    crate::artifacts::flow::io::register();

    register_artifact_schema();
    register_pilot_languages();
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<crate::apps::flow::FlowPlayApp>(crate::artifacts::flow::FLOW_DOCUMENT_SCHEMA);
}

/// 🧬️ Registers this artifact's fifteen schema leaves with the framework table.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::flow::schema::flow_artifact_schema_descriptor());
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary) for in-process execution.
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "flow.document",
        extension: Some("flow"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::flow::dsl::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::flow::dsl::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::flow::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::flow::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("flow.document"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "flow.op",
        extension: None,
        role: dsl::LanguageRole::Ops,
        grammar: Some(crate::artifacts::flow::op::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::flow::op::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::flow::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::flow::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("flow.op"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "flow.diff",
        extension: None,
        role: dsl::LanguageRole::Diff,
        grammar: Some(crate::artifacts::flow::schema::diff::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::flow::schema::diff::text::COMPONENT_GRAMMAR_PATH),
        protocol: None,
        protocol_path: None,
        hooks: dsl::passthrough_hooks("flow.diff"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "flow.pack",
        extension: None,
        role: dsl::LanguageRole::Pack,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::flow::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::flow::snapshot::pack::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("flow.pack"),
    });
    dsl::register_language(dsl::LanguageSpec {
        id: "flow.spr",
        extension: None,
        role: dsl::LanguageRole::Spr,
        grammar: None,
        grammar_path: None,
        protocol: Some(crate::artifacts::flow::spr::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::flow::spr::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("flow.spr"),
    });
}

//#endregion 🔖️Register

//#region 🔖️Host
pub fn seed_host_catalogue(host: &mut FlowHost, extra_sections_json: &str) {
    let mut sections = flow::flow_catalogue_sections();
    if let Ok(extra) = serde_json::from_str::<Vec<flow::CatalogueSection>>(extra_sections_json) {
        sections.extend(extra);
    }
    host.set_host_catalogue_json(&serde_json::to_string(&sections).unwrap_or_else(|_| "[]".into()));
}

/// 🎚️ Pushes the view-state canvas options (LOD mode, proximity distance, grid) onto a freshly built host.
pub fn apply_canvas_options(host: &mut FlowHost, config: &FlowConfig) {
    if config.lod_mode != FLOW_LOD_MODE_AUTOMATIC && DagDrawLod::from_id(&config.lod_mode).is_some() {
        host.dag.set_automatic_lod(false);
        host.dag.set_forced_draw_lod_label(&config.lod_mode);
    } else {
        host.dag.set_automatic_lod(true);
    }
    host.dag.set_proximity_distance(config.proximity_distance);
    host.set_grid_visible(config.grid_visible);
    host.set_grid_snap_enabled(config.grid_snap_enabled);
    let _ = host.set_grid_factor(config.grid_factor);
}

/// 🏗️ Rebuilds the stateful `FlowHost` from the document projection + view config + eval session — the
/// single entry point every command handler and every window renderer goes through.
pub fn host_from_snapshot(fixture: &FlowSnapshot, config: &FlowConfig, session: &FlowEvalSession) -> FlowHost {
    let mut host = flow_host_with_session(&fixture.to_fixture(), session);
    seed_host_catalogue(&mut host, &config.catalogue_sections_json);
    apply_canvas_options(&mut host, config);
    host
}

/// ✏️ Runs a stateful `FlowHost` mutation and diffs the result back into granular `FlowMutation`s —
/// returns an empty vec when `mutate` reports "nothing changed".
/// 🌉️ Diffs two snapshots into plugin `FlowMutation`s via the framework host bridge.
pub fn snapshot_operations(before: &FlowSnapshot, after: &FlowSnapshot) -> Vec<FlowMutation> {
    flow::flow_fixture_operations(&before.to_fixture(), &after.to_fixture())
        .into_iter()
        .map(crate::artifacts::flow::schema::mutations::from_framework_mutation)
        .collect()
}

pub fn host_operations(snapshot: &FlowSnapshot, config: &FlowConfig, session: &FlowEvalSession, mutate: impl FnOnce(&mut FlowHost) -> bool) -> Vec<FlowMutation> {
    let mut host = host_from_snapshot(snapshot, config, session);
    if !mutate(&mut host) {
        return Vec::new();
    }
    flow::flow_fixture_operations(&snapshot.to_fixture(), &host.fixture)
        .into_iter()
        .map(crate::artifacts::flow::schema::mutations::from_framework_mutation)
        .collect()
}

/// 🧵️ The `HostEffect` that arms/continues the off-main-thread `flowEvalTick` chain.
pub fn eval_tick_effect() -> HostEffect {
    HostEffect::DispatchAction { action: FLOW_EVAL_TICK_ACTION.into(), args: None, delay_ms: 0 }
}
//#endregion 🔖️Host

//#region 🔖️Selection
pub fn sync_host_selection(host: &mut FlowHost, selected: &[String]) {
    sync_host_selection_domains(host, selected, &[], &[]);
}

pub fn sync_host_selection_domains(host: &mut FlowHost, nodes: &[String], edges: &[String], handles: &[String]) {
    if nodes.is_empty() && edges.is_empty() && handles.is_empty() {
        let _ = host.dag.cancel_area_select();
        return;
    }
    let json = serde_json::json!({ "nodes": nodes, "edges": edges, "handles": handles });
    host.dag.set_selection_domains_json(&json.to_string());
}

/// 🔍️ The camera that frames the current node selection, or `None` when nothing is selected.
pub fn focus_selection_camera(fixture: &FlowSnapshot, config: &FlowConfig, session: &FlowEvalSession) -> Option<CameraJson> {
    if config.selected_node_ids.is_empty() {
        return None;
    }
    let mut host = host_from_snapshot(fixture, config, session);
    host.dag.set_viewport(1280, 800, 1.0);
    host.dag.set_selection(&config.selected_node_ids);
    host.focus_selection_camera(1.2)
}
//#endregion 🔖️Selection

//#region 🔖️Widgets
pub fn split_endpoint(endpoint: &str) -> (String, String) {
    endpoint.split_once('@').map_or_else(|| (endpoint.to_string(), "out".into()), |(node, port)| (node.to_string(), port.to_string()))
}

pub fn fixture_to_workflow(fixture: &DagFixture) -> (Vec<NodeGraphNodeRecord>, Vec<NodeGraphEdgeRecord>) {
    let nodes: Vec<NodeGraphNodeRecord> = fixture
        .nodes
        .iter()
        .map(|node| NodeGraphNodeRecord {
            id: node.id.clone(),
            label: Some(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
            x: node.x,
            y: node.y,
            width: node.width,
            height: node.height,
            inputs: node.inputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            outputs: node.outputs().iter().filter(|port| port.visible).map(|port| NodeGraphPortRecord { id: format!("{}@{}", node.id, port.id), label: Some(port.label.clone()), ..Default::default() }).collect(),
            ..Default::default()
        })
        .collect();
    let edges: Vec<NodeGraphEdgeRecord> = fixture
        .edges
        .iter()
        .map(|edge| {
            let (source_node_id, source_port_id) = split_endpoint(&edge.source);
            let (target_node_id, target_port_id) = split_endpoint(&edge.target);
            NodeGraphEdgeRecord { id: edge.id.clone(), source_node_id, source_port_id, target_node_id, target_port_id, label: None }
        })
        .collect();
    (nodes, edges)
}

pub fn widget_kind_label(widget: &Widget) -> &'static str {
    match widget {
        Widget::Neuron { .. } => "neuron",
        Widget::InputSlider { .. } => "inputSlider",
        Widget::InputNote { .. } => "inputNote",
        Widget::InputImage { .. } => "inputImage",
        Widget::Variable { .. } => "variable",
        Widget::OutputPreview { .. } => "outputPreview",
        Widget::OutputAction { .. } => "outputAction",
        Widget::OutputExport { .. } => "outputExport",
        Widget::Cluster { .. } => "cluster",
    }
}

pub fn widget_tree_label(widget: &Widget) -> String {
    match widget {
        Widget::Neuron { id, neuron_kind, .. } => format!("{id} ({neuron_kind})"),
        Widget::InputSlider { id, .. } => format!("{id} (slider)"),
        Widget::InputNote { id, .. } => format!("{id} (note)"),
        Widget::OutputPreview { id, .. } => format!("{id} (preview)"),
        Widget::Variable { id, name, .. } => format!("{id} ({name})"),
        widget => format!("{} ({})", widget_id(widget), widget_kind_label(widget)),
    }
}

pub fn widget_id(widget: &Widget) -> &str {
    match widget {
        Widget::Neuron { id, .. }
        | Widget::InputSlider { id, .. }
        | Widget::InputNote { id, .. }
        | Widget::InputImage { id, .. }
        | Widget::Variable { id, .. }
        | Widget::OutputPreview { id, .. }
        | Widget::OutputAction { id, .. }
        | Widget::OutputExport { id, .. }
        | Widget::Cluster { id, .. } => id,
    }
}

pub fn flow_widget_descriptor(kind: &str, neuron_kind: Option<&str>) -> Value {
    if kind == "neuron" {
        json!({ "kind": "neuron", "neuronKind": neuron_kind.unwrap_or(kind) })
    } else {
        json!({ "kind": kind })
    }
}

/// 🪢️ Wraps a widget descriptor into the `{mime: payload}` JSON shape `tree_item_with_action_draggable`
/// expects for its drag-data map.
pub fn flow_widget_drag_json(descriptor: &Value) -> Value {
    json!({ FLOW_WIDGET_DRAG_MIME: descriptor.to_string() })
}
//#endregion 🔖️Widgets

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_endpoint_defaults_port_to_out() {
        assert_eq!(split_endpoint("node@port"), ("node".to_string(), "port".to_string()));
        assert_eq!(split_endpoint("node"), ("node".to_string(), "out".to_string()));
    }

    #[test]
    fn widget_id_and_kind_label_agree_across_variants() {
        let widget = Widget::InputSlider { id: "slider".into(), value: 3.0, min: 0.0, max: 10.0, step: 0.1 };
        assert_eq!(widget_id(&widget), "slider");
        assert_eq!(widget_kind_label(&widget), "inputSlider");
        assert_eq!(widget_tree_label(&widget), "slider (slider)");
    }

    #[test]
    fn flow_widget_drag_json_wraps_descriptor_under_drag_mime() {
        let descriptor = flow_widget_descriptor("neuron", Some("math.add"));
        let drag = flow_widget_drag_json(&descriptor);
        assert!(drag.get(FLOW_WIDGET_DRAG_MIME).is_some());
    }

    #[test]
    fn flow_eval_session_neural_cache_is_per_instance_not_process_wide() {
        let a = FlowEvalSession::new();
        let b = FlowEvalSession::new();
        assert!(!std::sync::Arc::ptr_eq(&a.neural_cache(), &b.neural_cache()));
    }

    #[test]
    fn host_from_snapshot_deletes_edge_selected_by_synapse_domain() {
        let config = FlowConfig::default();
        let fixture = FlowSnapshot::default();
        let session = FlowEvalSession::new();
        let mut host = host_from_snapshot(&fixture, &config, &session);
        sync_host_selection_domains(&mut host, &[], &["s1".into()], &[]);
        assert!(host.has_selection(), "s1 must resolve through host_from_snapshot edge map");
        host.delete_selection().expect("deleteSelection");
        assert!(!host.fixture.synapses.iter().any(|synapse| synapse.id == "s1"));
    }
}
//#endregion 🧪️Tests


//#region 🔹ArtifactEngine
/// ⚙️ Stateful artifact engine owning the full `FlowArtifact` plus cached snapshot.
pub struct FlowEngine {
    artifact: crate::artifacts::flow::schema::FlowArtifact,
    snapshot: crate::artifacts::flow::FlowSnapshot,
}

impl FlowEngine {
    /// Seeds the engine from a persisted snapshot.
    pub fn new(snapshot: crate::artifacts::flow::FlowSnapshot) -> Self {
        let artifact = crate::artifacts::flow::schema::FlowArtifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    /// Consumes the engine and returns its persisted snapshot.
    pub fn into_snapshot(self) -> crate::artifacts::flow::FlowSnapshot {
        self.snapshot
    }
}

impl protocol::ArtifactEngine for FlowEngine {
    type Artifact = crate::artifacts::flow::schema::FlowArtifact;
    type Snapshot = crate::artifacts::flow::FlowSnapshot;
    type Mutation = crate::artifacts::flow::schema::mutations::FlowMutation;
    type Diff = crate::artifacts::flow::FlowDiff;

    fn artifact(&self) -> &Self::Artifact {
        &self.artifact
    }

    fn snapshot(&self) -> &Self::Snapshot {
        &self.snapshot
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot);
        self.snapshot = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot);
        self.artifact.set_snapshot(self.snapshot.clone());
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot)
    }
}
//#endregion 🔹ArtifactEngine
