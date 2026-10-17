//! 🧬️ Flow artifact schema — every field of the artifact with its state class.

use crate::{FlowContentChild, FlowSnapshot};

use framework_schema::ArtifactSchema;

//#region 🔖️Constants
/// 🖱️ Default proximity-select distance used by each concrete Flow main-window configuration,
/// homed here rather than app-side because this schema's own `FlowArtifact::from_snapshot` needs it too
/// and an artifact must never depend on an app.
pub const FLOW_DEFAULT_PROXIMITY_DISTANCE: f64 = 48.0;
/// 🔳️ Default canvas grid factor — see [`FLOW_DEFAULT_PROXIMITY_DISTANCE`] for why it lives here.
pub const FLOW_DEFAULT_GRID_FACTOR: f64 = 10.0;
//#endregion 🔖️Constants

//#region 🔖️Widgets
/// 🎛️ Every `Widget` variant carries its own `id: String` as its first field — this reaches through
/// the tag to read it generically.
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

/// 👯️ Clones a widget with every field but `id` copied verbatim — the `duplicate-widget` composite
/// mutation's plan uses this to mint the copy it hands to `create-widget`.
pub fn widget_with_id(widget: &Widget, id: String) -> Widget {
    let mut copy = widget.clone();
    match &mut copy {
        Widget::Neuron { id: widget_id, .. }
        | Widget::InputSlider { id: widget_id, .. }
        | Widget::InputNote { id: widget_id, .. }
        | Widget::InputImage { id: widget_id, .. }
        | Widget::Variable { id: widget_id, .. }
        | Widget::OutputPreview { id: widget_id, .. }
        | Widget::OutputAction { id: widget_id, .. }
        | Widget::OutputExport { id: widget_id, .. }
        | Widget::Cluster { id: widget_id, .. } => *widget_id = id,
    }
    copy
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
//#endregion 🔖️Widgets

//#region 🔹Artifact
/// 🧬️ flow document artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.flow.flow")]
pub struct FlowArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub content: FlowContentChild,
}
//#endregion 🔹Artifact

//#region 🔹Conversions
impl Default for FlowArtifact {
    fn default() -> Self {
        Self::from_snapshot(FlowSnapshot::default())
    }
}

impl FlowArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> FlowSnapshot {
        FlowSnapshot { schema: self.schema.clone(), content: self.content.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: FlowSnapshot) -> Self {
        Self { schema: snapshot.schema, content: snapshot.content }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: FlowSnapshot) {
        self.schema = snapshot.schema;
        self.content = snapshot.content;
    }
}
//#endregion 🔹Conversions

//#region 🔹Descriptor
/// 🧬️ Descriptor for `s.flow.flow` — twenty handcrafted schema leaves.
pub fn flow_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.flow.flow",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔹Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use semio_framework_artifact_flow_flow::CameraJson;
pub use semio_framework_artifact_flow_flow::Widget;
//#endregion 🔁️Re-exports

//#region 🧩️ArtifactItems
pub mod artifact_items {
//#region 🔖️Types
/// 🌊️ Owns the composed Flow document codec independently from its framework host.
pub const FLOW_DOCUMENT_SCHEMA: &str = "s.flow.flow";
//#endregion 🔖️Types
}
//#endregion 🧩️ArtifactItems

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
use semio_framework_artifact_flow_flow::{SynapseSpec, Widget, WidgetLayout};
use semio_s_artifact_stdio_semio_flow::schema::snapshot::{FlowEdge as SemioFlowEdge, FlowNode as SemioFlowNode, FlowParam as SemioFlowParam, PortRef as SemioPortRef, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA};
use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle type for the composed `s.stdio.semio@v1/flow` document — the flow plugin's
/// widgets/synapses/layout now live in this composed child's `nodes`/`edges` rather than inline on
/// `FlowSnapshot`.
pub type FlowContentChild = store::os_store::artifact_child::ArtifactChild<SemioFlowSnapshot>;

pub struct FlowContentHashWriter {
    hasher: semio_framework_hash::Sha256,
    written: usize,
    maximum_bytes: usize,
}

impl Write for FlowContentHashWriter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let next = self.written.checked_add(bytes.len()).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Flow content byte count overflow"))?;
        if next > self.maximum_bytes {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Flow content exceeds its retained byte cap"));
        }
        self.hasher.update(bytes);
        self.written = next;
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// 🎛️ Per-widget-variant fields flattened into id-ordered string key/value `FlowParam`s — structured
/// sub-values (`Dictionary`, port lists, `expanded` sets, cluster `Tree`/`FlowGui`) are JSON-encoded
/// into the string value, the same "honest string boundary" `SemioFlowSnapshot`'s own doc comment
/// describes for a generic flow DAG's per-node config. Every `Widget` field is covered — this is a
/// real lossless mapping, not a stub.
pub fn widget_params(widget: &Widget) -> Vec<SemioFlowParam> {
    fn p(key: &str, value: String) -> SemioFlowParam {
        SemioFlowParam { key: key.into(), value }
    }
    match widget {
        Widget::Neuron { neuron_kind, params, input_ports, output_ports, preview, .. } => vec![
            p("neuronKind", neuron_kind.clone()),
            p("params", semio_framework_pack_json::to_json_string(params)),
            p("inputPorts", serde_json::to_string(input_ports).unwrap_or_default()),
            p("outputPorts", serde_json::to_string(output_ports).unwrap_or_default()),
            p("preview", preview.to_string()),
        ],
        Widget::InputSlider { label, value, min, max, step, .. } => vec![p("label", label.clone()), p("value", value.to_string()), p("min", min.to_string()), p("max", max.to_string()), p("step", step.to_string())],
        Widget::InputNote { text, .. } => vec![p("text", text.clone())],
        Widget::InputImage { src, .. } => vec![p("src", src.clone())],
        Widget::Variable { name, schema, .. } => vec![p("name", name.clone()), p("schema", schema.clone())],
        Widget::OutputPreview { preview, expanded, .. } => vec![p("preview", semio_framework_pack_json::to_json_string(preview)), p("expanded", semio_framework_pack_json::to_json_string(expanded))],
        Widget::OutputAction { action, .. } => vec![p("action", action.clone())],
        Widget::OutputExport { format, .. } => vec![p("format", format.clone())],
        Widget::Cluster { name, tree, flow: nested, .. } => vec![p("name", name.clone()), p("tree", semio_framework_pack_json::to_json_string(tree)), p("flow", semio_framework_pack_json::to_json_string(nested))],
    }
}

/// 🌉 Inverse of [`widget_params`] — reconstructs the exact `Widget` variant from its `kind` tag and
/// flattened params; an unrecognized `kind` honestly surfaces as a note carrying the raw tag rather
/// than silently dropping the node.
pub fn widget_from_node(node: &SemioFlowNode) -> Widget {
    let params: HashMap<&str, &str> = node.params.iter().map(|param| (param.key.as_str(), param.value.as_str())).collect();
    let get = |key: &str| params.get(key).map(|value| value.to_string()).unwrap_or_default();
    let id = node.id.clone();
    match node.kind.as_str() {
        "neuron" => Widget::Neuron {
            id,
            neuron_kind: get("neuronKind"),
            params: semio_framework_pack_json::from_json_str(&get("params"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default(),
            input_ports: serde_json::from_str(&get("inputPorts")).unwrap_or_default(),
            output_ports: serde_json::from_str(&get("outputPorts")).unwrap_or_default(),
            preview: get("preview").parse().unwrap_or(true),
        },
        "inputSlider" => Widget::InputSlider { id, label: node.label.clone(), value: get("value").parse().unwrap_or(0.0), min: get("min").parse().unwrap_or(0.0), max: get("max").parse().unwrap_or(10.0), step: get("step").parse().unwrap_or(0.1) },
        "inputNote" => Widget::InputNote { id, text: get("text") },
        "inputImage" => Widget::InputImage { id, src: get("src") },
        "variable" => Widget::Variable { id, name: get("name"), schema: get("schema") },
        "outputPreview" => Widget::OutputPreview { id, preview: semio_framework_pack_json::from_json_str(&get("preview"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default(), expanded: semio_framework_pack_json::from_json_str(&get("expanded"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default() },
        "outputAction" => Widget::OutputAction { id, action: get("action") },
        "outputExport" => Widget::OutputExport { id, format: get("format") },
        "cluster" => Widget::Cluster { id, name: get("name"), tree: semio_framework_pack_json::from_json_str(&get("tree"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default(), flow: semio_framework_pack_json::from_json_str(&get("flow"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default() },
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
pub fn flow_content_snapshot_from_working(widgets: &[Widget], synapses: &[SynapseSpec], layout: &flow::OrderedMap<WidgetLayout>) -> SemioFlowSnapshot {
    let nodes = widgets
        .iter()
        .map(|widget| {
            let id = crate::schema::widget_id(widget);
            flow_content_node_from_working(widget, layout.get(id))
        })
        .collect();
    let edges = synapses
        .iter()
        .map(|synapse| SemioFlowEdge { id: synapse.id.clone(), from: SemioPortRef { node: synapse.from.clone(), port: synapse.from_port.clone() }, to: SemioPortRef { node: synapse.to.clone(), port: synapse.to_port.clone() }, kind: "data".into() })
        .collect();
    SemioFlowSnapshot { schema: STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(), nodes, edges }
}

/// 🌉 Maps one exact working widget and layout entry into its typed Semio child node.
pub fn flow_content_node_from_working(widget: &Widget, layout: Option<&WidgetLayout>) -> SemioFlowNode {
    let id = crate::schema::widget_id(widget).to_string();
    let kind = crate::schema::widget_kind_label(widget).to_string();
    let position = layout.map(|entry| semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2 { x: entry.x, y: entry.y }).unwrap_or_default();
    let label = match widget {
        Widget::InputSlider { label, .. } => label.clone(),
        _ => kind.clone(),
    };
    SemioFlowNode { id, kind, label, params: widget_params(widget), position }
}

/// 🌉 Inverse of [`flow_content_snapshot_from_working`].
pub fn working_from_flow_content_snapshot(content: &SemioFlowSnapshot) -> (Vec<Widget>, Vec<SynapseSpec>, flow::OrderedMap<WidgetLayout>) {
    let mut widgets = Vec::with_capacity(content.nodes.len());
    let mut layout = flow::OrderedMap::new();
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
pub fn flow_content_child_handle(widgets: &[Widget], synapses: &[SynapseSpec], layout: &flow::OrderedMap<WidgetLayout>) -> FlowContentChild {
    flow_content_child_handle_bounded(widgets, synapses, layout, usize::MAX).expect("Flow content serialization")
}

/// 🌊️ Mints a content-addressed child while enforcing the caller's exact serialization cap without staging JSON.
pub fn flow_content_child_handle_bounded(widgets: &[Widget], synapses: &[SynapseSpec], layout: &flow::OrderedMap<WidgetLayout>, maximum_bytes: usize) -> Result<FlowContentChild, String> {
    let digest = flow_content_digest(widgets, synapses, layout, maximum_bytes)?;
    Ok(flow_content_child_from_digest(digest, Arc::new(FlowWorkingScene { widgets: widgets.to_vec(), synapses: synapses.to_vec(), layout: layout.clone() })))
}

/// 🔐️ The canonical content digest alone, over borrowed scene parts — split out of
/// [`flow_content_child_handle_bounded`] so a caller that already OWNS the scene can move it into the
/// child's local owner instead of cloning it (an `OrderedMap` clone that is later dropped rather than
/// retired aborts the guest: "ordered-map root must be explicitly retired before drop").
pub fn flow_content_digest(widgets: &[Widget], synapses: &[SynapseSpec], layout: &flow::OrderedMap<WidgetLayout>, maximum_bytes: usize) -> Result<[u8; 32], String> {
    let mut writer = FlowContentHashWriter { hasher: semio_framework_hash::Sha256::new(), written: 0, maximum_bytes };
    writer.hasher.update(FLOW_CONTENT_ID_DOMAIN);
    let value = semio_framework_value::DslValue::object([
        ("widgets".to_string(), semio_framework_value::DslValue::Array(widgets.iter().map(semio_framework_value::ToValue::to_value).collect())),
        ("synapses".to_string(), semio_framework_value::DslValue::Array(synapses.iter().map(semio_framework_value::ToValue::to_value).collect())),
        ("layout".to_string(), semio_framework_value::ToValue::to_value(layout)),
    ]);
    let json: serde_json::Value = value.into();
    serde_json::to_writer(&mut writer, &json).map_err(|error| error.to_string())?;
    Ok(writer.hasher.finalize())
}

/// 🪪️ Portable content identity framing; scene bytes follow this NUL-terminated UTF-8 domain.
pub const FLOW_CONTENT_ID_DOMAIN: &[u8] = b"semio.flow.scene.sha256.v1\0";

/// 🪆️ Adopts the exact prepared scene allocation after its complete canonical digest is known.
pub fn flow_content_child_from_digest(digest: [u8; 32], scene: Arc<FlowWorkingScene>) -> FlowContentChild {
    let child_id = format!("flow-content-sha256-{}", semio_framework_hash::hex_lower(&digest));
    let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() };
    let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::os_store::artifact_child::ArtifactChild::new(child_id, target).with_local_owner(scene)
}
//#endregion 🔖️ContentBridge

//#region 🔖️WorkingScene
/// 🌱 Immutable artifact-instance owner of one composed content child's live
/// widgets/synapses/layout. The typed owner is retained by the exact `ArtifactChild`, omitted from
/// every wire codec, and dies with its final child/snapshot clone. Durable child-id reuse therefore
/// cannot replace or resolve another app instance's scene.
#[derive(Clone, Debug, Default, value_derive::ToValue)]
pub struct FlowWorkingScene {
    pub widgets: Vec<Widget>,
    pub synapses: Vec<SynapseSpec>,
    pub layout: flow::OrderedMap<WidgetLayout>,
}

/// ♻️ A scene value RETIRES its own layout root. `flow::OrderedMap`'s `Drop` is fail-closed
/// ("ordered-map root must be explicitly retired before drop"), and this type is handed out by value
/// to every reader — a diff that refuses early, an inverse that only reads a prior position, a test
/// that counts widgets. Retiring here is what makes "just drop it" correct for all of them; a caller
/// that TRANSFERS the parts on (into a new content child, into a `FlowHostSnapshot`) takes them
/// through [`FlowWorkingScene::into_parts`] instead, which leaves an empty root behind.
impl Drop for FlowWorkingScene {
    fn drop(&mut self) {
        let mut retirement = semio_framework_artifact_flow_flow::retained::FlowRetirement::default();
        retirement.push(semio_framework_artifact_flow_flow::retained::FlowOwner::Layouts(std::mem::take(&mut self.layout)));
        retirement.retire_cold();
    }
}

impl FlowWorkingScene {
    /// 🚚️ Moves the three owned parts out for a caller that transfers them onward, leaving this
    /// scene's own root empty so its retiring [`Drop`] has nothing left to close.
    pub fn into_parts(self) -> (Vec<Widget>, Vec<SynapseSpec>, flow::OrderedMap<WidgetLayout>) {
        let original=std::mem::ManuallyDrop::new(self);
        unsafe{(std::ptr::read(&original.widgets),std::ptr::read(&original.synapses),std::ptr::read(&original.layout))}
    }
}

/// 📝 Replaces one exact child handle's local scene owner without publishing process state.
pub fn cache_flow_content(handle: &mut FlowContentChild, widgets: Vec<Widget>, synapses: Vec<SynapseSpec>, layout: flow::OrderedMap<WidgetLayout>) {
    handle.set_local_owner(Arc::new(FlowWorkingScene { widgets, synapses, layout }));
}

/// 🔎 Reads the exact child-local live scene, failing soft only for a wire-decoded unresolved handle.
pub fn flow_working_scene_for_handle(handle: &FlowContentChild) -> FlowWorkingScene {
    handle.local_owner::<FlowWorkingScene>().map(|scene| (*scene).clone()).unwrap_or_default()
}

/// 🏗️ Mints a new content-addressed handle and MOVES the caller's scene into its exact
/// artifact-instance owner.
///
/// 🩸️ It used to mint the handle from borrowed parts and then let `widgets`/`synapses`/`layout` fall
/// out of scope — and `layout` is a `flow::OrderedMap`, whose fail-closed `Drop` panics with
/// "ordered-map root must be explicitly retired before drop". Every caller that owns a live scene
/// (`FlowSnapshot::from_host_snapshot`, and through it `rename-flow-widget`, `patchFlowWidgets`,
/// every `FlowMutation` that rebuilds the content child) therefore aborted the guest on a populated
/// document. The scene is now moved into the child's local owner, which `♻️retirement::retire_scene`
/// closes — no clone is minted and nothing is dropped.
pub fn flow_content_child_handle_and_cache(widgets: Vec<Widget>, synapses: Vec<SynapseSpec>, layout: flow::OrderedMap<WidgetLayout>) -> FlowContentChild {
    let digest = flow_content_digest(&widgets, &synapses, &layout, usize::MAX).expect("Flow content serialization");
    flow_content_child_from_digest(digest, Arc::new(FlowWorkingScene { widgets, synapses, layout }))
}
//#endregion 🔖️WorkingScene
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{FlowWorkingScene, flow_content_snapshot_from_working, flow_working_scene_for_handle, working_from_flow_content_snapshot};
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;
use std::sync::Arc;
pub use crate::standards::v1::subsets::any::schema::snapshot::FlowSnapshot;

//#region 🔖️SnapshotBridge
/// 🔎 Reads the current document's live widgets/synapses/layout off its `content` child handle — the
/// single read call site every mutation diff/inverse in this plugin uses instead of the old
/// `snapshot.widgets`/`.synapses`/`.layout` field access.
pub fn flow_working_scene(snapshot: &FlowSnapshot) -> FlowWorkingScene {
    flow_working_scene_for_handle(&snapshot.content)
}

/// 🌱️ The initial pack for the `content` child this document declares but no archive member carries —
/// derived from the working scene the parent already caches on its own content handle, which is the
/// same bytes `register_content_child` composes in the unit fixture.
///
/// 🩸️ Without it `ArtifactEditor::genesis_child_pack`'s default `None` leaves the child uncomposed in
/// a live shell, so `context.children.dialect("content", …)` never resolves, `FlowChildGroupWork`'s
/// `admitted_child` returns `None` at its second `?`, and every `Child`-lane verb is refused before
/// any capacity is measured — measured on :6216 as *"addWidget refused: dispatch-failed (user
/// window=flow-main) — retained command work refused the command before any capacity was measured"*
/// (ticket 26/09/18, slices PB1 §3 / PB3 §3.4), while the same command passes natively, where the
/// fixture registers the child by hand.
pub fn flow_genesis_content_pack(document: &FlowSnapshot, slot: &str, child_id: &str) -> Option<Vec<u8>> {
    if slot != "content" || child_id != document.content.child_id {
        return None;
    }
    let scene = document.content.local_owner::<FlowWorkingScene>()?;
    Some(<SemioFlowSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&flow_content_snapshot_from_working(&scene.widgets, &scene.synapses, &scene.layout)))
}

/// 🪆️ The flow document every reader and writer works on: the parent's exact `content` coordinate carrying the scene
/// of the composed child it names. That child is flow's only persisted scene — the parent pack holds just the
/// coordinate — so the parent's own working-scene owner, which only a parsed or genesis document carries and no edit
/// refreshes, rendered graphs without the widgets `addWidget` had added and let every parent-lane edit re-point the
/// coordinate at a child no store held (ticket 26/09/23, P8 `wp-p8.md` § flow). A coordinate whose child is not composed
/// yet (a document parsed from text, before genesis) reads the scene genesis will compose it from.
pub fn flow_composed_snapshot(snapshot: &FlowSnapshot, children: &semio_framework_plugin::ChildContentView) -> Result<FlowSnapshot, semio_framework_plugin::Fault> {
    let child_id = &snapshot.content.child_id;
    let scene = match children.dialect("content", child_id) {
        Some(dialect) if dialect.artifact_kind == "s.stdio.semio" && dialect.standard == "v1" && dialect.subset == "flow" => {
            let content = children.typed_read::<SemioFlowSnapshot>("content", child_id)?;
            let (widgets, synapses, layout) = working_from_flow_content_snapshot(&content);
            Arc::new(FlowWorkingScene { widgets, synapses, layout })
        }
        Some(dialect) => {
            return Err(semio_framework_plugin::Fault::new(
                semio_framework_plugin::FaultOrigin::App,
                semio_framework_plugin::FaultCode::new("flow.content-dialect"),
                format!("the flow content child \"{child_id}\" is {}@{}/{}, not s.stdio.semio@v1/flow", dialect.artifact_kind, dialect.standard, dialect.subset),
            ))
        }
        None => snapshot.content.local_owner::<FlowWorkingScene>().ok_or_else(|| {
            semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.content-unavailable"), format!("the flow content child \"{child_id}\" is not composed"))
        })?,
    };
    let mut content = snapshot.content.clone();
    content.set_local_owner(scene);
    Ok(FlowSnapshot { schema: snapshot.schema.clone(), content })
}
//#endregion 🔖️SnapshotBridge
}
//#endregion 🧩️ArtifactItems4
