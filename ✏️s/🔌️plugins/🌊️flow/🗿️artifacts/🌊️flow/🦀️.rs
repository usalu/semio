//! 🌊️ Flow artifact — the document entity this plugin's apps edit.
//!
//! The persisted snapshot type is [`FlowSnapshot`] (this plugin). The framework crate
//! `semio-framework-os-flow` still owns a separate `semio_framework_artifact_flow_flow::FlowFixture` used by `FlowHost` and by
//! other plugins (e.g. procedural) that embed a flow graph; conversions live on `FlowSnapshot`.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`flow→C:flow`, the canonical editor for
//! stdio's `flow` subset): the old inline `widgets`/`synapses`/`layout` fields are replaced by a
//! composed `s.stdio.semio@v1/flow` CHILD slot (`🔖️ContentBridge` below) — this plugin no longer
//! defines its own node-graph content model, it composes stdio's `flow` subset instead. The rich
//! live editing types (`semio_framework_artifact_flow_flow::Widget`/`semio_framework_artifact_flow_flow::SynapseSpec`/`semio_framework_artifact_flow_flow::WidgetLayout`, the framework
//! kernel's own vocabulary `FlowHost` edits) still flow entirely through `FlowSnapshot::to_fixture`/
//! `from_fixture`, which now bridge through the composed child + `🔖️WorkingScene` cache rather than
//! plain struct fields.

extern crate semio_framework_schema as framework_schema;

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_flow_demo_tests;
extern crate semio_framework_value_derive as value_derive;
use semio_framework_artifact_playbook_playbook as playbook;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<FlowMutation, FlowConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
extern crate self as semio_s_artifact_flow_flow;

use semio_framework_artifact_flow_flow::{SynapseSpec, Widget, WidgetLayout};
use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};
use semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::{
    FlowEdge as SemioFlowEdge, FlowNode as SemioFlowNode, FlowParam as SemioFlowParam, PortRef as SemioPortRef, SemioFlowSnapshot, STDIO_SEMIOFLOW_DOCUMENT_SCHEMA,
};
use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;

#[path = "♻️retirement/🦀️.rs"]
pub mod retirement;

//#region 🔖️Types
pub use semio_framework_artifact_flow_flow::FLOW_DOCUMENT_SCHEMA;
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
        let id = schema::widget_id(widget);
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
    let id = schema::widget_id(widget).to_string();
    let kind = schema::widget_kind_label(widget).to_string();
    let position = layout.map(|entry| semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2 { x: entry.x, y: entry.y }).unwrap_or_default();
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
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
        .schema(schema::flow_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::flow_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .document_codec::<EditorApp<editor::flow::FlowPlayApp>>()
        .try_build()
}
//#endregion 🔖️Declaration

#[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_widget {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️create-widget/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️create-widget/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️create-widget/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️create-widget/🧪️tests/🚫️rejects-a-duplicate-widget-id/🦀️.rs"]
                                    mod tests_rejects_a_duplicate_widget_id;
                                }
                                #[path = "."]
                                pub mod delete_widget {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-widget/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-widget/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-widget/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-widget/🧪️tests/🚫️rejects-deleting-a-160f49/🦀️.rs"]
                                    mod tests_rejects_deleting_a_missing_widget;
                                }
                                #[path = "."]
                                pub mod reorder_widgets {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️reorder-widgets/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️reorder-widgets/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️reorder-widgets/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔢️reorder-widgets/🧪️tests/🗜️clamps-an-out-of-54a49e/🦀️.rs"]
                                    mod tests_clamps_an_out_of_range_index_onto_the_last_slot;
                                }
                                #[path = "."]
                                pub mod replace_widget {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️replace-widget/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️replace-widget/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️replace-widget/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁️replace-widget/🧪️tests/🟰️replaces-a-note-with-6fb6b5/🦀️.rs"]
                                    mod tests_replaces_a_note_with_an_identical_note;
                                }
                                #[path = "."]
                                pub mod connect_widgets {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️connect-widgets/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️connect-widgets/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️connect-widgets/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔌️connect-widgets/🧪️tests/🚫️refuses-a-parallel-96391d/🦀️.rs"]
                                    mod tests_refuses_a_parallel_synapse_as_a_no_op;
                                }
                                #[path = "."]
                                pub mod disconnect_widgets {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-widgets/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-widgets/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-widgets/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️disconnect-widgets/🧪️tests/🚫️rejects-disconnect-631cc8/🦀️.rs"]
                                    mod tests_rejects_disconnecting_a_missing_synapse;
                                }
                                #[path = "."]
                                pub mod reorder_synapses {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-synapses/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-synapses/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-synapses/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-synapses/🧪️tests/🟰️keeps-the-leading-fe4b61/🦀️.rs"]
                                    mod tests_keeps_the_leading_synapse_at_index_zero;
                                }
                                #[path = "."]
                                pub mod update_synapse_endpoints {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️update-synapse-endpoints/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️update-synapse-endpoints/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️update-synapse-endpoints/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️update-synapse-endpoints/🧪️tests/🟰️re-declares-the-5174a0/🦀️.rs"]
                                    mod tests_re_declares_the_same_endpoints;
                                }
                                #[path = "."]
                                pub mod move_widgets {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-widgets/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-widgets/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-widgets/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️move-widgets/🧪️tests/🟰️re-applies-the-current-4adf26/🦀️.rs"]
                                    mod tests_re_applies_the_current_layout_to_both_widgets;
                                }
                                // 🌉️ COMPOSITE — owns 🦠️mutation + 🧩️plan only (no 🔺️diff/↩️inverse: both fold from the plan).
                                #[path = "."]
                                pub mod duplicate_widget {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️duplicate-widget/🦀️.rs"]
                                    pub mod mutation;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️duplicate-widget/🧩️plan/🦀️.rs"]
                                    pub mod plan;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👯️duplicate-widget/🧪️tests/🚫️rejects-duplicating-6d209e/🦀️.rs"]
                                    mod tests_rejects_duplicating_onto_a_taken_id;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod csv {
                                            #[path = "."]
                                            pub mod v_rfc4180 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📊️csv/🔖️rfc4180/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op {
            pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
            pub use crate::standards::v1::subsets::any::schema::mutations::FlowMutation;
        }
        pub mod document_dsl {
            pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod pack {
            pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
        }
        pub mod diff {
            pub use crate::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::schema::diff::text::*;
            }
        }
        pub mod mutations {
            pub use crate::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }
        pub use crate::standards::v1::subsets::any::schema::diff::FlowDiff;
        pub use crate::standards::v1::subsets::any::schema::mutations::FlowMutation;
        pub use crate::standards::v1::subsets::any::schema::snapshot::FlowSnapshot;

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod flow {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-widget/🦀️.rs"]
            pub mod add_widget;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗️connect-media-ports/🦀️.rs"]
            pub mod connect_media_ports;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖱️context-menu-at/🦀️.rs"]
            pub mod context_menu_at;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✂️disconnect/🦀️.rs"]
            pub mod disconnect;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️duplicate-widget/🦀️.rs"]
            pub mod duplicate_widget;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👣️duplicate-widget-step/🦀️.rs"]
            pub mod duplicate_widget_step;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧮️evaluate/🦀️.rs"]
            pub mod evaluate;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏁️flow-eval-resolve/🦀️.rs"]
            pub mod flow_eval_resolve;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏱️flow-eval-tick/🦀️.rs"]
            pub mod flow_eval_tick;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎯️focus-selection/🦀️.rs"]
            pub mod focus_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚚️move-media-node/🦀️.rs"]
            pub mod move_media_node;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs"]
            pub mod node_graph_edit;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔭️node-graph-viewport/🦀️.rs"]
            pub mod node_graph_viewport;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔦️open-spotlight/🦀️.rs"]
            pub mod open_spotlight;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🩹️patch-flow-widgets/🦀️.rs"]
            pub mod patch_flow_widgets;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➖️remove-widget/🦀️.rs"]
            pub mod remove_widget;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏷️rename-flow-widget/🦀️.rs"]
            pub mod rename_flow_widget;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗺️reorganize/🦀️.rs"]
            pub mod reorganize;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️replace-image/🦀️.rs"]
            pub mod replace_image;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/▶️run-extension-action/🦀️.rs"]
            pub mod run_extension_action;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🛍️set-catalogue-sections/🦀️.rs"]
            pub mod set_catalogue_sections;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👥️set-contributions/🦀️.rs"]
            pub mod set_contributions;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📏️set-grid-factor/🦀️.rs"]
            pub mod set_grid_factor;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧲️set-grid-snap-enabled/🦀️.rs"]
            pub mod set_grid_snap_enabled;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-grid-visible/🦀️.rs"]
            pub mod set_grid_visible;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-graph-parameter/🦀️.rs"]
            pub mod set_graph_parameter;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔬️set-lod-mode/🦀️.rs"]
            pub mod set_lod_mode;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🙈️set-preview-off/🦀️.rs"]
            pub mod set_preview_off;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↔️set-proximity-distance/🦀️.rs"]
            pub mod set_proximity_distance;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✅️spotlight-commit/🦀️.rs"]
            pub mod spotlight_commit;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔌️toggle-extension/🦀️.rs"]
            pub mod toggle_extension;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️.rs"]
                        mod component;
                        pub use component::*;

                        #[path = "."]
                        pub mod options {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/🌐️grid/🦀️.rs"]
                            pub mod grid;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/🔭️lod/🦀️.rs"]
                            pub mod lod;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/☑️options/📏️proximity/🦀️.rs"]
                            pub mod proximity;
                        }
                    }

                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🗣️compiled/🦀️.rs"]
                    pub mod compiled;
                }
            }

            #[path = "."]
            pub mod generate {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod commands {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🎮️commands/➕️add-generation/🦀️.rs"]
                    pub mod add_generation;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🎮️commands/🗑️remove-generation/🦀️.rs"]
                    pub mod remove_generation;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🎮️commands/🏷️rename-generation/🦀️.rs"]
                    pub mod rename_generation;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🎮️commands/🎯️select-generation/🦀️.rs"]
                    pub mod select_generation;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🎮️commands/🎚️update-generation-values/🦀️.rs"]
                    pub mod update_generation_values;
                }

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs"]
                    pub mod form;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs"]
                    pub mod generations;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs"]
                    pub mod preview;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod flow {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌊️main/🦀️.rs"]
                    pub mod main;
                }
            }
        }
    }
}

