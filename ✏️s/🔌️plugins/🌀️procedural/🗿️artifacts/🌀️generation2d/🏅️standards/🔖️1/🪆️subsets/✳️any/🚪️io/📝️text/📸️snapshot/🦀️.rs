//! 📜️ Generation2d artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! `FlowHostSnapshot`/`Widget`/`SynapseSpec`/`WidgetLayout`/`CameraJson` (from `flow`) and
//! `GenerationPlayState`/`FormGeneration`/`GenerationMutation` (from `playbook`) are all foreign to
//! this crate, so none can carry a `#[derive(dsl::Dsl...)]` themselves — Rust's orphan rule requires
//! the impl target type to live in the crate that also owns the trait or the type, and neither is
//! true here. The `*Dsl` types re-exported below (owned by `semio_s_procedural_core_generation_sqlite_native`, shared with generation3d) are LOCAL structural twins the real types convert to/from right
//! at the `parse_dsl`/`print_dsl`/`parse_op`/`print_op` boundary (same pattern as `fem_2d`'s `FemDof`
//! and `imperative_core`'s `ValueDsl`/`StepNodeDsl`/`PathDsl`) — `Generation2dSnapshot`/
//! `Generation2dMutation` themselves keep their ORIGINAL foreign field types unchanged.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
#[cfg(test)]
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_artifact_playbook_playbook::GenerationPlayState;
#[cfg(test)]
use semio_framework_artifact_playbook_playbook::FormGeneration;
use std::collections::BTreeMap;

/// 📦️ The `procedural2d-play` "default" example, embedded at compile time as handcrafted `.generation2d`
/// DSL text — shared by the manifest's `.example(...)` registration, the `default_snapshot` fallback,
/// and every test host_snapshot.
pub const GENERATION2D_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio");

//#region 🔖️DslMirror
pub use semio_s_procedural_core_generation_sqlite_native::{
    camera_from_dsl, camera_to_dsl, form_generation_from_dsl, form_generation_to_dsl, layout_from_dsl, layout_to_dsl, synapse_from_dsl, synapse_to_dsl, widget_from_dsl, widget_to_dsl, CameraJsonDsl, FormGenerationDsl, SynapseSpecDsl, WidgetDsl, WidgetLayoutDsl,
};

/// 🧾️ Local twin of `Generation2dSnapshot`, flattening `FlowHostSnapshot`/`GenerationPlayState`'s fields
/// into one top-level `#[derive(dsl::DslRecord)]` grammar.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(id = "procedural.generation2d", layout = "lines")]
pub struct Generation2dSnapshotDsl {
    schema: String,
    #[dsl(block)]
    camera: CameraJsonDsl,
    #[dsl(statements, block)]
    widgets: Vec<WidgetDsl>,
    #[dsl(table)]
    synapses: Vec<SynapseSpecDsl>,
    layout: BTreeMap<String, WidgetLayoutDsl>,
    #[dsl(key = "selected-generation")]
    selected_generation_id: Option<String>,
    preview_text: Option<String>,
    #[dsl(table)]
    generations: Vec<FormGenerationDsl>,
}
//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ P6 handcrafted ArtifactDsl/ArtifactPack (derive no longer emits these traits).
impl pack::snapshot::ArtifactDsl for Generation2dSnapshotDsl {
    const EXTENSION: &'static str = "generation2d";
    fn envelope_id() -> &'static str {
        "procedural.generation2d"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}


//#endregion 🔖️HandcraftedArtifactCodecs

pub fn generation2d_document_to_dsl(document: &Generation2dSnapshot) -> Generation2dSnapshotDsl {
    let host_snapshot = &document.host_snapshot;
    let generation = &document.generation;
    Generation2dSnapshotDsl {
        schema: host_snapshot.schema.clone(),
        camera: camera_to_dsl(&host_snapshot.camera),
        widgets: host_snapshot.widgets.iter().map(widget_to_dsl).collect(),
        synapses: host_snapshot.synapses.iter().map(synapse_to_dsl).collect(),
        layout: host_snapshot.layout.iter().map(|(id, entry)| (id.clone(), layout_to_dsl(entry))).collect(),
        selected_generation_id: generation.selected_generation_id.clone(),
        preview_text: generation.preview_text.clone(),
        generations: generation.generations.iter().map(form_generation_to_dsl).collect(),
    }
}

pub fn generation2d_document_from_dsl(parsed: Generation2dSnapshotDsl) -> Result<Generation2dSnapshot, semio_framework_diagnostic::TextError> {
    let widgets = parsed.widgets.into_iter().map(widget_from_dsl).collect::<Result<Vec<_>, _>>()?;
    let synapses = parsed.synapses.into_iter().map(synapse_from_dsl).collect();
    let layout = parsed.layout.into_iter().map(|(id, entry)| (id, layout_from_dsl(&entry))).collect();
    Ok(Generation2dSnapshot {
        host_snapshot: FlowHostSnapshot { schema: parsed.schema, camera: camera_from_dsl(&parsed.camera), widgets, synapses, layout },
        generation: GenerationPlayState { generations: parsed.generations.into_iter().map(form_generation_from_dsl).collect(), selected_generation_id: parsed.selected_generation_id, preview_text: parsed.preview_text }.into(),
    })
}

/// 📜️ `.generation2d` textual document — derive-engine grammar via `Generation2dSnapshotDsl`.
impl pack::snapshot::ArtifactDsl for Generation2dSnapshot {
    const EXTENSION: &'static str = "generation2d";

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let parsed = <Generation2dSnapshotDsl as pack::snapshot::ArtifactDsl>::parse_dsl(text)?;
        generation2d_document_from_dsl(parsed)
    }

    fn print_dsl(&self) -> String {
        <Generation2dSnapshotDsl as pack::snapshot::ArtifactDsl>::print_dsl(&generation2d_document_to_dsl(self))
    }
}

/// 📦️ `.generation2d` binary pack — same `Generation2dSnapshotDsl` mirror as `ArtifactDsl` above;
/// `dsl::DslArtifact`'s derive already gives `Generation2dSnapshotDsl` its own `ArtifactPack` impl.

//#endregion 🔖️DslMirror

/// 📖️ Parses `.generation2d` DSL text into a `Generation2dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Generation2dSnapshot, semio_framework_diagnostic::TextError> {
    <Generation2dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Generation2dSnapshot` back to `.generation2d` DSL text.
pub fn print_dsl(document: &Generation2dSnapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Generation2dSnapshotText = String;

struct Generation2dNative;

impl semio_s_procedural_core_generation_sqlite_native::GenerationNative for Generation2dNative {
    type Snapshot = Generation2dSnapshot;
    const ENVELOPE_ID: &'static str = "procedural.generation2d";
    const PACK_PREFIX: Option<&'static [u8]> = Some(b"P2D2");
    fn spec_producer() -> semio_framework_dsl_record::RecordSpecProducer {
        Generation2dSnapshotDsl::__dsl_spec_producer()
    }
    fn fields(record: &semio_framework_dsl_record::RecordValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<semio_s_procedural_core_generation_sqlite_native::GenerationFields, semio_framework_value::ValueError> {
        let parsed = Generation2dSnapshotDsl::__dsl_from_record_controlled(record, control)?;
        Ok(semio_s_procedural_core_generation_sqlite_native::GenerationFields {
            schema: parsed.schema,
            camera: parsed.camera,
            widgets: parsed.widgets,
            synapses: parsed.synapses,
            layout: parsed.layout,
            selected_generation_id: parsed.selected_generation_id,
            preview_text: parsed.preview_text,
            generations: parsed.generations,
        })
    }
    fn host(snapshot: &Generation2dSnapshot) -> &FlowHostSnapshot {
        &snapshot.host_snapshot
    }
    fn state(snapshot: &Generation2dSnapshot) -> &GenerationPlayState {
        snapshot.generation.as_state()
    }
    fn assemble(host_snapshot: FlowHostSnapshot, generation: semio_framework_artifact_playbook_playbook::GenerationPlayRoot) -> Generation2dSnapshot {
        Generation2dSnapshot { host_snapshot, generation }
    }
}

pub fn encode_sqlite_native(document: &Generation2dSnapshot, encoding: semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding, control: &mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>, native_owner: &mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>) -> Result<store::io_schema::IoPayload, semio_framework_value::ValueError> {
    semio_s_procedural_core_generation_sqlite_native::encode::<Generation2dNative>(document, encoding, control, native_owner)
}

pub fn decode_sqlite_native(payload: &store::io_schema::IoPayload, control: &mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>, native_owner: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>) -> Result<Generation2dSnapshot, semio_framework_value::ValueError> {
    semio_s_procedural_core_generation_sqlite_native::decode::<Generation2dNative>(payload, control, native_owner)
}
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use semio_s_artifact_procedural_2d_model::*;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_framework_os_infinite_directed_presentation::DagHostSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;
use semio_framework_os_flow::forms_bridge::apply_generation_values_to_host_snapshot;
use semio_framework_os_flow::render_scene_json;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_os_flow::{flow_host_with_session, flow_neuron_kind_info_map, FlowEvalSession, FlowHost};
use semio_framework_ui_declaration::ui::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};
use semio_framework_value_derive::{FromValue, ToValue};
use pack::snapshot::ArtifactDsl;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;

pub fn collect_drawing_handles_from_eval(value: &semio_framework_pack_json::Value, handles: &mut Vec<String>) {
    match value {
        semio_framework_pack_json::Value::Object(map) => {
            if map.get("$schema").and_then(|entry| entry.as_str()) == Some("draw.drawing") {
                if let Some(handle) = map.get("handle").and_then(|entry| entry.as_str()) {
                    handles.push(handle.into());
                }
            }
            for (_, entry) in map.iter() {
                collect_drawing_handles_from_eval(entry, handles);
            }
        }
        semio_framework_pack_json::Value::Array(items) => {
            for item in items {
                collect_drawing_handles_from_eval(item, handles);
            }
        }
        _ => {}
    }
}

pub fn affine_transform_array(value: &semio_framework_pack_json::Value) -> [f64; 6] {
    if let Some(matrix) = value.as_array() {
        let mut out = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        for (index, entry) in matrix.iter().take(6).enumerate() {
            out[index] = entry.as_f64().unwrap_or(if index == 0 || index == 3 { 1.0 } else { 0.0 });
        }
        return out;
    }
    if let Some(matrix) = value.get("0").and_then(|entry| entry.as_array()) {
        let wrapped = semio_framework_pack_json::Value::Array(matrix.clone());
        return affine_transform_array(&wrapped);
    }
    [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]
}

pub fn path_segments_from_node(node: &semio_framework_pack_json::Value) -> Vec<semio_framework_pack_json::Value> {
    if let Some(segments) = node.get("segments").and_then(|entry| entry.as_array()) {
        return segments.clone();
    }
    for key in ["path", "shape", "line", "polyline", "rect", "ellipse", "circle", "polygon"] {
        if let Some(inner) = node.get(key) {
            if let Some(segments) = inner.get("segments").and_then(|entry| entry.as_array()) {
                return segments.clone();
            }
        }
    }
    Vec::new()
}

pub fn scene_layers_from_drawing_handle(handle: &str, prefix: &str) -> Vec<semio_framework_pack_json::Value> {
    let scene_json = render_scene_json(handle);
    let Ok(scene) = semio_framework_pack_json::parse(&scene_json, semio_framework_pack_json::JsonMemberPolicy::Reject) else {
        return Vec::new();
    };
    if scene.get("error").is_some() {
        return Vec::new();
    }
    let Some(nodes) = scene.get("nodes").and_then(|entry| entry.as_array()) else {
        return Vec::new();
    };
    nodes
        .iter()
        .enumerate()
        .map(|(index, node)| {
            let node_body = node.get("node").unwrap_or(node);
            let transform: Vec<semio_framework_pack_json::Value> = affine_transform_array(node.get("transform").unwrap_or(&semio_framework_pack_json::Value::Null)).into_iter().map(semio_framework_pack_json::Value::from).collect();
            let mut object = semio_framework_pack_json::Object::new();
            object.insert("id", semio_framework_pack_json::Value::from(format!("{prefix}-{handle}-{index}")));
            object.insert("transform", semio_framework_pack_json::Value::from(transform));
            object.insert("segments", semio_framework_pack_json::Value::from(path_segments_from_node(node_body)));
            object.insert("fill", node.get("fill").cloned().unwrap_or(semio_framework_pack_json::Value::Null));
            object.insert("stroke", node.get("stroke").cloned().unwrap_or(semio_framework_pack_json::Value::Null));
            object.insert("opacity", semio_framework_pack_json::Value::from(node.get("opacity").and_then(|entry| entry.as_f64()).unwrap_or(1.0)));
            object.insert("blendMode", semio_framework_pack_json::Value::from("normal"));
            object.insert("visible", semio_framework_pack_json::Value::from(true));
            object.insert("needsKernel", semio_framework_pack_json::Value::from(false));
            semio_framework_pack_json::Value::Object(object)
        })
        .collect()
}

pub fn generation_preview_host(host_snapshot: &FlowHostSnapshot, values: &semio_framework_artifact_playbook_playbook::PlaybookValues) -> FlowHost {
    let snapshot_json = semio_framework_pack_json::to_json_string(host_snapshot);
    let object: semio_framework_pack_json::Object = values.iter().map(|(key, value)| (key.clone(), semio_framework_pack_json::from_dsl_value(value))).collect();
    let patched = apply_generation_values_to_host_snapshot(&snapshot_json, &object);
    let patched_fixture = FlowHost::parse_host_snapshot_json(&patched).unwrap_or_else(|_| host_snapshot.clone());
    FlowHost::from_host_snapshot(patched_fixture).with_history_store_factory(::semio_framework_artifact_flow_flow::FLOW_HISTORY_STORE_FACTORY)
}

/// 📤️ The drawings a program OUTPUTS: every drawing a synapse delivers into an output widget
/// (`output-preview`, `output-export`), read off the evaluation at the synapse's source port, in
/// synapse order without repeats. Intermediate drawings (a shape before its style) are not outputs.
pub fn output_drawing_handles(host_snapshot: &FlowHostSnapshot, outputs: &semio_framework_pack_json::Value) -> Vec<String> {
    use semio_framework_artifact_flow_flow::Widget;
    let is_output = |id: &str| host_snapshot.widgets.iter().any(|widget| matches!(widget, Widget::OutputPreview { id: output, .. } | Widget::OutputExport { id: output, .. } if output == id));
    let mut handles = Vec::new();
    for synapse in host_snapshot.synapses.iter().filter(|synapse| is_output(&synapse.to)) {
        let Some(out) = outputs.get(&synapse.from).and_then(|node| node.get("out")) else { continue };
        let delivered = if synapse.from_port.is_empty() { Some(out) } else { out.get(&synapse.from_port) };
        let mut found = Vec::new();
        if let Some(value) = delivered {
            collect_drawing_handles_from_eval(value, &mut found);
        }
        for handle in found {
            if !handles.contains(&handle) {
                handles.push(handle);
            }
        }
    }
    handles
}

/// 🖼️ [`output_drawing_handles`] as the scene layers the `drawing:out` port publishes.
pub fn generation_output_layers(host_snapshot: &FlowHostSnapshot, eval_json: &str) -> String {
    let layers: Vec<semio_framework_pack_json::Value> = semio_framework_pack_json::parse(eval_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|outputs| output_drawing_handles(host_snapshot, &outputs).iter().flat_map(|handle| scene_layers_from_drawing_handle(handle, "generation2d-drawing-out")).collect()).unwrap_or_default();
    semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::from(layers))
}

pub fn generation_preview_layers(eval_json: &str) -> String {
    let prefix = "generation2d-generate-preview";
    let mut layers = Vec::new();
    if let Ok(outputs) = semio_framework_pack_json::parse(eval_json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
        let mut handles = Vec::new();
        collect_drawing_handles_from_eval(&outputs, &mut handles);
        handles.sort();
        handles.dedup();
        for handle in handles {
            layers.extend(scene_layers_from_drawing_handle(&handle, prefix));
        }
    }
    semio_framework_pack_json::to_string(&semio_framework_pack_json::Value::from(layers))
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use semio_s_artifact_procedural_2d_model::*;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_framework_os_infinite_directed_presentation::DagHostSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;
use semio_framework_os_flow::forms_bridge::apply_generation_values_to_host_snapshot;
use semio_framework_os_flow::render_scene_json;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_os_flow::{flow_host_with_session, flow_neuron_kind_info_map, FlowEvalSession, FlowHost};
use semio_framework_ui_declaration::ui::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};
use semio_framework_value_derive::{FromValue, ToValue};
use pack::snapshot::ArtifactDsl;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;

/// 📄️ The `procedural2d-play` "default" document — parsed from the bundled `.generation2d` example
/// host_snapshot, falling back to the empty document if the fixture ever fails to parse.
pub fn default_snapshot() -> Generation2dSnapshot {
    Generation2dSnapshot::parse_dsl(crate::snapshot::GENERATION2D_EXAMPLE_TEXT).unwrap_or_default()
}
}
pub use snapshot_wire_codec::*;

pub fn evaluate_generation_preview(host_snapshot: &FlowHostSnapshot, values: &semio_framework_artifact_playbook_playbook::PlaybookValues) -> String {
    let mut host = generation_preview_host(host_snapshot, values);
    let evaluated = host.evaluate().unwrap_or_default();
    host.retire_cold();
    evaluated
}
