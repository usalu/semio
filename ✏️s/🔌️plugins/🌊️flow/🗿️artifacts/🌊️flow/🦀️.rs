//! 🌊️ Flow artifact — the document entity this plugin's apps edit.
//!
//! The persisted snapshot type is [`FlowSnapshot`] (this plugin). The framework crate
//! `semio-framework-os-flow` still owns a separate `flow::FlowFixture` used by `FlowHost` and by
//! other plugins (e.g. procedural) that embed a flow graph; conversions live on `FlowSnapshot`.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`flow→C:flow`, the canonical editor for
//! stdio's `flow` subset): the old inline `widgets`/`synapses`/`layout` fields are replaced by a
//! composed `s.stdio.semio@v1/flow` CHILD slot (`🔖️ContentBridge` below) — this plugin no longer
//! defines its own node-graph content model, it composes stdio's `flow` subset instead. The rich
//! live editing types (`flow::Widget`/`flow::SynapseSpec`/`flow::WidgetLayout`, the framework
//! kernel's own vocabulary `FlowHost` edits) still flow entirely through `FlowSnapshot::to_fixture`/
//! `from_fixture`, which now bridge through the composed child + `🔖️WorkingScene` cache rather than
//! plain struct fields.

use flow::{SynapseSpec, Widget, WidgetLayout};
use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::{
    FlowEdge as SemioFlowEdge, FlowNode as SemioFlowNode, FlowParam as SemioFlowParam, PortRef as SemioPortRef, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA,
};
use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;

#[path = "♻️retirement/🦀️.rs"]
pub mod retirement;

//#region 🔖️Types
pub use crate::artifacts::flow::snapshot::schema::FlowSnapshot;
pub use flow::FLOW_DOCUMENT_SCHEMA;
//#endregion 🔖️Types

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1/§2.1 — lives at the
/// ARTIFACT level (not under the sibling editor module) so a viewer file can read it without ever
/// importing through that module. `artifact_kind` matches this artifact's own `definition()` capability
/// row (`s.flow.schema.artifact` descriptor `b"s.flow.flow"`); `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location — the canonical surface ids are
/// `s.flow.flow@1/*#editor` / `s.flow.flow@1/*#viewer`.
pub const FLOW_DIALECT: Dialect = Dialect { artifact_kind: "s.flow.flow", standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️ContentBridge
/// 🕸️ Owned CHILD handle type for the composed `s.stdio.semio@v1/flow` document — the flow plugin's
/// widgets/synapses/layout now live in this composed child's `nodes`/`edges` rather than inline on
/// `FlowSnapshot`.
pub type FlowContentChild = store::ArtifactChild<SemioFlowSnapshot>;

struct FlowContentHashWriter {
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
fn widget_params(widget: &Widget) -> Vec<SemioFlowParam> {
    fn p(key: &str, value: String) -> SemioFlowParam {
        SemioFlowParam { key: key.into(), value }
    }
    match widget {
        Widget::Neuron { neuron_kind, params, input_ports, output_ports, preview, .. } => vec![
            p("neuronKind", neuron_kind.clone()),
            p("params", flow::os_pack::json::to_json_string(params)),
            p("inputPorts", serde_json::to_string(input_ports).unwrap_or_default()),
            p("outputPorts", serde_json::to_string(output_ports).unwrap_or_default()),
            p("preview", preview.to_string()),
        ],
        Widget::InputSlider { label, value, min, max, step, .. } => vec![p("label", label.clone()), p("value", value.to_string()), p("min", min.to_string()), p("max", max.to_string()), p("step", step.to_string())],
        Widget::InputNote { text, .. } => vec![p("text", text.clone())],
        Widget::InputImage { src, .. } => vec![p("src", src.clone())],
        Widget::Variable { name, schema, .. } => vec![p("name", name.clone()), p("schema", schema.clone())],
        Widget::OutputPreview { preview, expanded, .. } => vec![p("preview", flow::os_pack::json::to_json_string(preview)), p("expanded", flow::os_pack::json::to_json_string(expanded))],
        Widget::OutputAction { action, .. } => vec![p("action", action.clone())],
        Widget::OutputExport { format, .. } => vec![p("format", format.clone())],
        Widget::Cluster { name, tree, flow: nested, .. } => vec![p("name", name.clone()), p("tree", flow::os_pack::json::to_json_string(tree)), p("flow", flow::os_pack::json::to_json_string(nested))],
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
            params: flow::os_pack::json::from_json_str(&get("params")).unwrap_or_default(),
            input_ports: serde_json::from_str(&get("inputPorts")).unwrap_or_default(),
            output_ports: serde_json::from_str(&get("outputPorts")).unwrap_or_default(),
            preview: get("preview").parse().unwrap_or(true),
        },
        "inputSlider" => Widget::InputSlider { id, label: node.label.clone(), value: get("value").parse().unwrap_or(0.0), min: get("min").parse().unwrap_or(0.0), max: get("max").parse().unwrap_or(10.0), step: get("step").parse().unwrap_or(0.1) },
        "inputNote" => Widget::InputNote { id, text: get("text") },
        "inputImage" => Widget::InputImage { id, src: get("src") },
        "variable" => Widget::Variable { id, name: get("name"), schema: get("schema") },
        "outputPreview" => Widget::OutputPreview { id, preview: flow::os_pack::json::from_json_str(&get("preview")).unwrap_or_default(), expanded: flow::os_pack::json::from_json_str(&get("expanded")).unwrap_or_default() },
        "outputAction" => Widget::OutputAction { id, action: get("action") },
        "outputExport" => Widget::OutputExport { id, format: get("format") },
        "cluster" => Widget::Cluster { id, name: get("name"), tree: flow::os_pack::json::from_json_str(&get("tree")).unwrap_or_default(), flow: flow::os_pack::json::from_json_str(&get("flow")).unwrap_or_default() },
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
    let nodes = widgets.iter().map(|widget| {
        let id = crate::artifacts::flow::schema::widget_id(widget);
        flow_content_node_from_working(widget, layout.get(id))
    }).collect();
    let edges = synapses
        .iter()
        .map(|synapse| SemioFlowEdge { id: synapse.id.clone(), from: SemioPortRef { node: synapse.from.clone(), port: synapse.from_port.clone() }, to: SemioPortRef { node: synapse.to.clone(), port: synapse.to_port.clone() }, kind: "data".into() })
        .collect();
    SemioFlowSnapshot { schema: STDIO_SEMIOFLOW_DOCUMENT_SCHEMA.into(), nodes, edges }
}

/// 🌉 Maps one exact working widget and layout entry into its typed Semio child node.
pub fn flow_content_node_from_working(widget: &Widget, layout: Option<&WidgetLayout>) -> SemioFlowNode {
    let id = crate::artifacts::flow::schema::widget_id(widget).to_string();
    let kind = crate::artifacts::flow::schema::widget_kind_label(widget).to_string();
    let position = layout.map(|entry| semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::base::schema::geometry::SemioPoint2 { x: entry.x, y: entry.y }).unwrap_or_default();
    let label = match widget { Widget::InputSlider { label, .. } => label.clone(), _ => kind.clone() };
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
    let mut writer = FlowContentHashWriter { hasher: semio_framework_hash::Sha256::new(), written: 0, maximum_bytes };
    writer.hasher.update(FLOW_CONTENT_ID_DOMAIN);
    let value = dsl::DslValue::object([
        ("widgets".to_string(), dsl::DslValue::Array(widgets.iter().map(dsl::ToValue::to_value).collect())),
        ("synapses".to_string(), dsl::DslValue::Array(synapses.iter().map(dsl::ToValue::to_value).collect())),
        ("layout".to_string(), dsl::ToValue::to_value(layout)),
    ]);
    let json: serde_json::Value = value.into();
    serde_json::to_writer(&mut writer, &json).map_err(|error| error.to_string())?;
    Ok(flow_content_child_from_digest(writer.hasher.finalize(), Arc::new(FlowWorkingScene { widgets: widgets.to_vec(), synapses: synapses.to_vec(), layout: layout.clone() })))
}

/// 🪪️ Portable content identity framing; scene bytes follow this NUL-terminated UTF-8 domain.
pub const FLOW_CONTENT_ID_DOMAIN: &[u8] = b"semio.flow.scene.sha256.v1\0";

/// 🪆️ Adopts the exact prepared scene allocation after its complete canonical digest is known.
pub(crate) fn flow_content_child_from_digest(digest: [u8; 32], scene: Arc<FlowWorkingScene>) -> FlowContentChild {
    let child_id = format!("flow-content-sha256-{}", semio_framework_hash::hex_lower(&digest));
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() };
    let target = store::os_io::ArtifactRef { artifact_id: child_id.clone(), dialect };
    store::ArtifactChild::new(child_id, target).with_local_owner(scene)
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

/// 📝 Replaces one exact child handle's local scene owner without publishing process state.
pub fn cache_flow_content(handle: &mut FlowContentChild, widgets: Vec<Widget>, synapses: Vec<SynapseSpec>, layout: flow::OrderedMap<WidgetLayout>) {
    handle.set_local_owner(Arc::new(FlowWorkingScene { widgets, synapses, layout }));
}

/// 🔎 Reads the exact child-local live scene, failing soft only for a wire-decoded unresolved handle.
pub fn flow_working_scene_for_handle(handle: &FlowContentChild) -> FlowWorkingScene {
    handle.local_owner::<FlowWorkingScene>().map(|scene| (*scene).clone()).unwrap_or_default()
}

/// 🔎 Reads the current document's live widgets/synapses/layout off its `content` child handle — the
/// single read call site every mutation diff/inverse in this plugin uses instead of the old
/// `snapshot.widgets`/`.synapses`/`.layout` field access.
pub fn flow_working_scene(snapshot: &FlowSnapshot) -> FlowWorkingScene {
    flow_working_scene_for_handle(&snapshot.content)
}

/// 🏗️ Mints a new content-addressed handle with its exact artifact-instance scene owner.
pub fn flow_content_child_handle_and_cache(widgets: Vec<Widget>, synapses: Vec<SynapseSpec>, layout: flow::OrderedMap<WidgetLayout>) -> FlowContentChild {
    flow_content_child_handle(&widgets, &synapses, &layout)
}
//#endregion 🔖️WorkingScene

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::flow::create_flow_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "computation.flow".into(),
        name: "Flow".into(),
        source_format: "flow.artifact".into(),
        component_kind: "flow".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Flow },
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

    fn owner_handle(text: &str) -> FlowContentChild {
        let target = store::os_io::ArtifactRef { artifact_id: "flow-content-reused".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "flow".into() } };
        let scene = FlowWorkingScene { widgets: vec![Widget::InputNote { id: "note".into(), text: text.into() }], synapses: Vec::new(), layout: flow::OrderedMap::new() };
        FlowContentChild::new("flow-content-reused".into(), target).with_local_owner(Arc::new(scene))
    }

    fn owner_text(handle: &FlowContentChild) -> String {
        let owner = handle.local_owner::<FlowWorkingScene>().expect("typed Flow owner");
        let [Widget::InputNote { text, .. }] = owner.widgets.as_slice() else { panic!("one note fixture") };
        text.clone()
    }

    #[test]
    fn flow_scene_owner_fixture_is_language_neutral_and_bounded() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/⚖️flow-scene-owner-law.json")).expect("language-neutral Flow owner fixture");
        assert_eq!(fixture["ownedSlots"], 1);
        assert_eq!(fixture["maximumCases"], 5);
        assert_eq!(fixture["cases"].as_array().map(Vec::len), Some(5));
    }

    #[test]
    fn flow_scene_owner_holds_identity_isolation_aba_wire_omission_and_close() {
        let left = owner_handle("A");
        let clone = left.clone();
        let left_owner = left.local_owner::<FlowWorkingScene>().expect("left owner");
        let clone_owner = clone.local_owner::<FlowWorkingScene>().expect("clone owner");
        assert!(Arc::ptr_eq(&left_owner, &clone_owner));

        let right = owner_handle("B");
        assert_eq!(left.child_id, right.child_id, "hostile durable identity must collide");
        assert_eq!(owner_text(&left), "A");
        assert_eq!(owner_text(&right), "B");
        let stale_a = left.clone();
        drop(left);
        let reused_b = owner_handle("B");
        assert_eq!(owner_text(&stale_a), "A", "stale A must not resolve reused B");
        assert_eq!(owner_text(&reused_b), "B");

        let wire = serde_json::Value::from(dsl::ToValue::to_value(&stale_a));
        assert_eq!(wire.as_object().map(serde_json::Map::len), Some(2));
        assert!(wire.get("localOwner").is_none());
        let encoded = serde_json::to_vec(&wire).expect("independent JSON encoding");
        let decoded: FlowContentChild = flow::os_pack::json::from_json_str(std::str::from_utf8(&encoded).unwrap()).expect("child wire decode");
        assert!(decoded.local_owner::<FlowWorkingScene>().is_none());

        drop(left_owner);
        drop(clone_owner);
        drop(clone);
        drop(right);
        drop(stale_a);
        drop(reused_b);
        let terminal = owner_handle("close");
        let owner = terminal.local_owner::<FlowWorkingScene>().expect("close owner");
        let witness = Arc::downgrade(&owner);
        drop(owner);
        assert!(witness.upgrade().is_some());
        drop(terminal);
        assert!(witness.upgrade().is_none(), "one exact child slot must close its scene owner");
    }

    /// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("flow.artifact") is deliberately NOT
    /// `FLOW_DOCUMENT_SCHEMA` ("flow.fixture") — the former names the artifact kind in the OS media
    /// catalogue, the latter keys the store envelope. Pinned so a future edit can't silently merge them.
    #[semio_framework_async_macros::async_test]
    async fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
        assert_eq!(artifact_kind().schema, "flow.artifact");
        assert_eq!(FLOW_DOCUMENT_SCHEMA, "flow.fixture");
    }

    #[semio_framework_async_macros::async_test]
    async fn default_snapshot_has_widgets() {
        assert!(!FlowSnapshot::default().to_fixture().widgets.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn widget_content_round_trips_through_the_composed_child_snapshot() {
        let fixture = flow::FlowFixture::default();
        let content = flow_content_snapshot_from_working(&fixture.widgets, &fixture.synapses, &fixture.layout);
        let (widgets, synapses, layout) = working_from_flow_content_snapshot(&content);
        assert_eq!(widgets, fixture.widgets);
        assert_eq!(synapses, fixture.synapses);
        for (id, entry) in &fixture.layout {
            assert_eq!(layout.get(id), Some(entry));
        }
    }

    #[test]
    fn authored_slider_labels_survive_child_content_round_trip() {
        let cases: serde_json::Value = serde_json::from_str(include_str!("🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/🏷️slider-labels.json")).unwrap();
        for row in cases["cases"].as_array().unwrap() {
            let widget: Widget = dsl::FromValue::from_value(dsl::DslValue::from(row["widget"].clone())).unwrap();
            let content = flow_content_snapshot_from_working(&[widget.clone()], &[], &flow::OrderedMap::new());
            assert_eq!(content.nodes[0].label, row["expectedDagName"].as_str().unwrap());
            assert_eq!(working_from_flow_content_snapshot(&content).0, [widget]);
        }
    }
}
//#endregion 🧪️Tests
//#region 🔖️Declaration
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.flow.flow")?)
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.flow.flow.schema.artifact")?, ArtifactCapabilityKind::schema()).descriptor(b"s.flow.flow")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.flow.flow")?)?)?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.flow.flow.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.flow.flow.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.flow.flow.inference")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.flow.flow.composer.native")?, ArtifactCapabilityKind::composer()).descriptor(b"s.flow.flow@1/*")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.flow.flow@1/*")?)?)?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.flow.flow.composer.md")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.md@commonmark/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.md@commonmark/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.flow.flow.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.flow.flow.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"flow.fixture:flow")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "flow.fixture")?)?
                .claim(ArtifactIdentityClaim::codec_extension("flow.fixture", "flow")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.flow.flow.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"Flow")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Flow")?)?)?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.flow.flow.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"Flow")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Flow")?)?)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::EditorApp;
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::flow::schema::flow_artifact_schema_descriptor())
        .inferences([crate::artifacts::flow::standards::v1::subsets::any::schema::inferences::flow_artifact_inference_descriptor()])
        .composers(crate::artifacts::flow::standards::v1::subsets::any::io::io_registry::entries())
        .document_codec::<EditorApp<crate::editor::flow::FlowPlayApp>>()
        .try_build()
}
//#endregion 🔖️Declaration
