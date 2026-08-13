//! 🌊️ Flow artifact — the document entity this plugin's apps edit.
//!
//! The persisted snapshot type is [`FlowSnapshot`] (this plugin). The framework crate
//! `semio-framework-os-flow` still owns a separate `flow::FlowFixture` used by `FlowHost` and by
//! other plugins (e.g. procedural) that embed a flow graph; conversions live on `FlowSnapshot`.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`flow→C:flow`, the canonical editor for
//! stdio's `flow` subset): the old inline `widgets`/`synapses`/`layout` fields are replaced by a
//! composed `s.stdio.semio.flow` CHILD slot (`🔖️ContentBridge` below) — this plugin no longer
//! defines its own node-graph content model, it composes stdio's `flow` subset instead. The rich
//! live editing types (`flow::Widget`/`flow::SynapseSpec`/`flow::WidgetLayout`, the framework
//! kernel's own vocabulary `FlowHost` edits) still flow entirely through `FlowSnapshot::to_fixture`/
//! `from_fixture`, which now bridge through the composed child + `🔖️WorkingScene` cache rather than
//! plain struct fields.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::{
    FlowEdge as SemioFlowEdge, FlowNode as SemioFlowNode, FlowParam as SemioFlowParam, PortRef as SemioPortRef, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA,
};
use flow::{SynapseSpec, Widget, WidgetLayout};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};

//#region 🔖️Types
pub use crate::artifacts::flow::snapshot::schema::FlowSnapshot;
pub use flow::FLOW_DOCUMENT_SCHEMA;
//#endregion 🔖️Types

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle type for the composed `s.stdio.semio.flow` document — the flow plugin's
/// widgets/synapses/layout now live in this composed child's `nodes`/`edges` rather than inline on
/// `FlowSnapshot`.
pub type FlowContentChild = store::ArtifactChild<SemioFlowSnapshot>;

/// 🎛️ Per-widget-variant fields flattened into id-ordered string key/value `FlowParam`s — structured
/// sub-values (`Dictionary`, port lists, `expanded` sets, cluster `Tree`/`FlowGui`) are JSON-encoded
/// into the string value, the same "honest string boundary" `SemioFlowSnapshot`'s own doc comment
/// describes for a generic flow DAG's per-node config. Every `Widget` field is covered — this is a
/// real lossless mapping, not a stub.
fn widget_params(widget: &Widget) -> Vec<SemioFlowParam> {
    fn p(key: &str, value: String) -> SemioFlowParam {
        SemioFlowParam { key: key.into(), value }
    }
    match widget {
        Widget::Neuron { neuron_kind, params, input_ports, output_ports, preview, .. } => vec![
            p("neuronKind", neuron_kind.clone()),
            p("params", serde_json::to_string(params).unwrap_or_default()),
            p("inputPorts", serde_json::to_string(input_ports).unwrap_or_default()),
            p("outputPorts", serde_json::to_string(output_ports).unwrap_or_default()),
            p("preview", preview.to_string()),
        ],
        Widget::InputSlider { value, min, max, step, .. } => vec![p("value", value.to_string()), p("min", min.to_string()), p("max", max.to_string()), p("step", step.to_string())],
        Widget::InputNote { text, .. } => vec![p("text", text.clone())],
        Widget::InputImage { src, .. } => vec![p("src", src.clone())],
        Widget::Variable { name, schema, .. } => vec![p("name", name.clone()), p("schema", schema.clone())],
        Widget::OutputPreview { preview, expanded, .. } => vec![p("preview", serde_json::to_string(preview).unwrap_or_default()), p("expanded", serde_json::to_string(expanded).unwrap_or_default())],
        Widget::OutputAction { action, .. } => vec![p("action", action.clone())],
        Widget::OutputExport { format, .. } => vec![p("format", format.clone())],
        Widget::Cluster { name, tree, flow: nested, .. } => vec![p("name", name.clone()), p("tree", serde_json::to_string(tree).unwrap_or_default()), p("flow", serde_json::to_string(nested).unwrap_or_default())],
    }
}

/// 🌉 Inverse of [`widget_params`] — reconstructs the exact `Widget` variant from its `kind` tag and
/// flattened params; an unrecognized `kind` honestly surfaces as a note carrying the raw tag rather
/// than silently dropping the node.
fn widget_from_node(node: &SemioFlowNode) -> Widget {
    let params: HashMap<&str, &str> = node.params.iter().map(|param| (param.key.as_str(), param.value.as_str())).collect();
    let get = |key: &str| params.get(key).map(|value| value.to_string()).unwrap_or_default();
    let id = node.id.clone();
    match node.kind.as_str() {
        "neuron" => Widget::Neuron {
            id,
            neuron_kind: get("neuronKind"),
            params: serde_json::from_str(&get("params")).unwrap_or_default(),
            input_ports: serde_json::from_str(&get("inputPorts")).unwrap_or_default(),
            output_ports: serde_json::from_str(&get("outputPorts")).unwrap_or_default(),
            preview: get("preview").parse().unwrap_or(true),
        },
        "inputSlider" => Widget::InputSlider { id, value: get("value").parse().unwrap_or(0.0), min: get("min").parse().unwrap_or(0.0), max: get("max").parse().unwrap_or(10.0), step: get("step").parse().unwrap_or(0.1) },
        "inputNote" => Widget::InputNote { id, text: get("text") },
        "inputImage" => Widget::InputImage { id, src: get("src") },
        "variable" => Widget::Variable { id, name: get("name"), schema: get("schema") },
        "outputPreview" => Widget::OutputPreview { id, preview: serde_json::from_str(&get("preview")).unwrap_or_default(), expanded: serde_json::from_str(&get("expanded")).unwrap_or_default() },
        "outputAction" => Widget::OutputAction { id, action: get("action") },
        "outputExport" => Widget::OutputExport { id, format: get("format") },
        "cluster" => Widget::Cluster {
            id,
            name: get("name"),
            tree: serde_json::from_str(&get("tree")).unwrap_or_default(),
            flow: serde_json::from_str(&get("flow")).unwrap_or_default(),
        },
        other => Widget::InputNote { id, text: format!("[unknown widget kind {other:?}]") },
    }
}

/// 🌉 REAL bidirectional converter between the app's live `Widget`/`SynapseSpec`/`WidgetLayout`
/// editing state and the composed child's own `SemioFlowSnapshot` node/edge graph (the
/// "ModelBridge"/"DocumentBridge" pattern from `📓️wave3-reports/cad-report.md` and
/// `📓️wave3-reports/writer-report.md`) — every widget variant's fields round-trip through
/// [`widget_params`]/[`widget_from_node`]; `layout` merges directly into `FlowNode::position`;
/// `SynapseSpec` maps onto `FlowEdge` 1:1 (`kind` is a constant "data" tag on encode, discarded on
/// decode — lossless, since `SynapseSpec` carries no `kind` of its own to lose).
pub fn flow_content_snapshot_from_working(widgets: &[Widget], synapses: &[SynapseSpec], layout: &BTreeMap<String, WidgetLayout>) -> SemioFlowSnapshot {
    let nodes = widgets
        .iter()
        .map(|widget| {
            let id = crate::artifacts::flow::schema::widget_id(widget).to_string();
            let kind = crate::artifacts::flow::schema::widget_kind_label(widget).to_string();
            let position = layout.get(&id).map(|entry| semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2 { x: entry.x, y: entry.y }).unwrap_or_default();
            SemioFlowNode { id: id.clone(), kind: kind.clone(), label: kind, params: widget_params(widget), position }
        })
        .collect();
    let edges = synapses
        .iter()
        .map(|synapse| SemioFlowEdge {
            id: synapse.id.clone(),
            from: SemioPortRef { node: synapse.from.clone(), port: synapse.from_port.clone() },
            to: SemioPortRef { node: synapse.to.clone(), port: synapse.to_port.clone() },
            kind: "data".into(),
        })
        .collect();
    SemioFlowSnapshot { schema: STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(), nodes, edges }
}

/// 🌉 Inverse of [`flow_content_snapshot_from_working`].
pub fn working_from_flow_content_snapshot(content: &SemioFlowSnapshot) -> (Vec<Widget>, Vec<SynapseSpec>, BTreeMap<String, WidgetLayout>) {
    let mut widgets = Vec::with_capacity(content.nodes.len());
    let mut layout = BTreeMap::new();
    for node in &content.nodes {
        widgets.push(widget_from_node(node));
        layout.insert(node.id.clone(), WidgetLayout { x: node.position.x, y: node.position.y });
    }
    let synapses = content.edges.iter().map(|edge| SynapseSpec { id: edge.id.clone(), from: edge.from.node.clone(), from_port: edge.from.port.clone(), to: edge.to.node.clone(), to_port: edge.to.port.clone() }).collect();
    (widgets, synapses, layout)
}

/// 🕸️ Deterministic content-addressed CHILD handle for the flow content — same `(child_id, target)`
/// for identical `(widgets, synapses, layout)`, a different pair once the content actually changes;
/// mirrors writer's `document_child_handle`/cad's `cad_model_child_handle`.
pub fn flow_content_child_handle(widgets: &[Widget], synapses: &[SynapseSpec], layout: &BTreeMap<String, WidgetLayout>) -> FlowContentChild {
    use std::hash::{Hash, Hasher};
    let snapshot = flow_content_snapshot_from_working(widgets, synapses, layout);
    let content_json = serde_json::to_string(&snapshot).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("flow-content-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() };
    let target = store::os_io::ArtifactRef { artifact_id: "flow-content".into(), dialect };
    store::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️ContentBridge

//#region 🔖️WorkingScene
/// 🌱 Ephemeral, session-side working representation of the composed content child's live
/// widgets/synapses/layout — NEVER persisted, NEVER a durable field on `FlowSnapshot` itself
/// (matches the `EngineRep` contract: wholly derived, droppable at any instant, rebuilt from base).
/// Exists because no `LinkResolver`/child-dispatch seam is wired into `ArtifactApp::handle` yet
/// (checked directly against `🔌️plugin/🦀️component.rs` — same standing gap cad/lowpoly/writer's
/// reports all document); until one exists, the only way a persisted content-addressed HANDLE can
/// round-trip to real widgets/synapses/layout within one process is this cache, keyed by
/// `FlowContentChild::child_id` — mirrors `WriterWorkingScene`/`LowpolyScratch.mesh_workspace`.
///
/// ⚠️ Same documented gap as lowpoly's `StaleMeshWorkspace`/writer's `WriterWorkingScene`: store-
/// level undo/redo bypasses `ArtifactApp::handle` entirely, and a bare `parse_dsl`/`decode_pack` of
/// persisted bytes recovers only the opaque handle, never the content (the child's real payload
/// lives in its own, not-yet-resolvable, child store). `flow_working_scene`/`flow_working_scene_for_
/// handle` fail soft (an empty scene) rather than panicking. A real fix needs child-document
/// resolution, which no WASM-guest plugin in this repo has yet.
#[derive(Clone, Debug, Default)]
pub struct FlowWorkingScene {
    pub widgets: Vec<Widget>,
    pub synapses: Vec<SynapseSpec>,
    pub layout: BTreeMap<String, WidgetLayout>,
}

thread_local! {
    static FLOW_SCRATCH: RefCell<HashMap<String, FlowWorkingScene>> = RefCell::new(HashMap::new());
}

/// 📝 Seeds the scratch cache for a handle — call whenever new widgets/synapses/layout content is
/// about to become a document's `content` field (every mutation-diff/fixture builder in this plugin
/// does, via [`flow_content_child_handle_and_cache`]).
pub fn cache_flow_content(child_id: &str, widgets: Vec<Widget>, synapses: Vec<SynapseSpec>, layout: BTreeMap<String, WidgetLayout>) {
    FLOW_SCRATCH.with(|cache| cache.borrow_mut().insert(child_id.to_string(), FlowWorkingScene { widgets, synapses, layout }));
}

/// 🔎 Reads the cached live scene for a content child handle — an empty scene (never a panic) when
/// nothing has cached it yet (see this region's module doc comment for why that can happen).
pub fn flow_working_scene_for_handle(handle: &FlowContentChild) -> FlowWorkingScene {
    FLOW_SCRATCH.with(|cache| cache.borrow().get(&handle.child_id).cloned()).unwrap_or_default()
}

/// 🔎 Reads the current document's live widgets/synapses/layout off its `content` child handle — the
/// single read call site every mutation diff/inverse in this plugin uses instead of the old
/// `snapshot.widgets`/`.synapses`/`.layout` field access.
pub fn flow_working_scene(snapshot: &FlowSnapshot) -> FlowWorkingScene {
    flow_working_scene_for_handle(&snapshot.content)
}

/// 🏗️ Mints a new content-addressed handle AND seeds the scratch cache with its scene in one call —
/// the standard way every mutation-diff/fixture builder in this plugin creates a `content` field
/// value; never construct a handle without also caching, or [`flow_working_scene`] will read back
/// empty.
pub fn flow_content_child_handle_and_cache(widgets: Vec<Widget>, synapses: Vec<SynapseSpec>, layout: BTreeMap<String, WidgetLayout>) -> FlowContentChild {
    let handle = flow_content_child_handle(&widgets, &synapses, &layout);
    cache_flow_content(&handle.child_id, widgets, synapses, layout);
    handle
}
//#endregion 🔖️WorkingScene

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::flow::create_flow_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "computation.flow".into(),
        name: "Flow".into(),
        source_format: "flow.artifact".into(),
        component_kind: "flow".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType {
            class: MediaClass::Computation,
            form: MediaForm::Flow,
        },
        schema: "flow.artifact".into(),
        export_formats: vec![],
        import_formats: vec![],
            export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("flow.artifact") is deliberately NOT
    /// `FLOW_DOCUMENT_SCHEMA` ("flow.fixture") — the former names the artifact kind in the OS media
    /// catalogue, the latter keys the store envelope. Pinned so a future edit can't silently merge them.
    #[test]
    fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
        assert_eq!(artifact_kind().schema, "flow.artifact");
        assert_eq!(FLOW_DOCUMENT_SCHEMA, "flow.fixture");
    }

    #[test]
    fn default_snapshot_has_widgets() {
        assert!(!FlowSnapshot::default().to_fixture().widgets.is_empty());
    }

    #[test]
    fn widget_content_round_trips_through_the_composed_child_snapshot() {
        let fixture = flow::FlowFixture::default();
        let content = flow_content_snapshot_from_working(&fixture.widgets, &fixture.synapses, &fixture.layout);
        let (widgets, synapses, layout) = working_from_flow_content_snapshot(&content);
        assert_eq!(widgets, fixture.widgets);
        assert_eq!(synapses, fixture.synapses);
        for (id, entry) in &fixture.layout {
            assert_eq!(layout.get(id), Some(entry));
        }
    }
}
//#endregion 🧪️Tests
//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use std::sync::OnceLock;
    use semio_framework_plugin::{ComposerEntry, Dialect, ErasedComposeSource, ComposedArtifact, ComposeError, register_composer_entries};
    use crate::artifacts::flow::standards::v1::subsets::any::io::io_registry as v1;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries()
            .iter()
            .find(|e| e.writes == target)
            .ok_or_else(|| ComposeError { message: format!("FlowComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        (entry.compose)(sources)
    }

    pub fn register() {
        register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry
