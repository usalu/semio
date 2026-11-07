//! 📜️ Generation3d artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! See `generation2d`'s sibling `🗣️dsl/🦀️.rs` docstring for why the `*Dsl` mirror types below
//! are LOCAL structural twins rather than derives on the foreign `flow`/`playbook` types directly.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::neural::{Atom, Dictionary, Value as NeuralValue};
use semio_framework_artifact_flow_flow::{CameraJson, FlowHostSnapshot, SynapseSpec, Widget, WidgetLayout};
use semio_framework_artifact_playbook_playbook::{FormGeneration, GenerationPlayState};
use std::collections::BTreeMap;

//#region 🔖️Examples
pub const GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT: &str = include_str!("../../../📚️examples/🍄️hexagonal-mushroom-column/🖼️assets/🍄️hexagonal-mushroom/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_RECT_EXTRUDE_TEXT: &str = include_str!("../../../📚️examples/📦️rectangle-extrude-volume/🖼️assets/📦️rectangle-extrude-volume/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_SPHERE_TORUS_TEXT: &str = include_str!("../../../📚️examples/🍩️sphere-cut-with-torus/🖼️assets/🍩️sphere-cut-with-torus/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_BOX_FILLET_TEXT: &str = include_str!("../../../📚️examples/📐️box-fillet-preview/🖼️assets/📐️box-fillet-preview/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_SPHERE_BOX_FUSE_TEXT: &str = include_str!("../../../📚️examples/🧲️sphere-box-fuse/🖼️assets/🧲️sphere-box-fuse/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_FACE_SWEEP_EXTRUDE_TEXT: &str = include_str!("../../../📚️examples/🧹️face-sweep-extrude/🖼️assets/🧹️face-sweep-extrude/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_RECTANGLE_WIRE_TEXT: &str = include_str!("../../../📚️examples/🪢️rectangle-wire-preview/🖼️assets/🪢️rectangle-wire-preview/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_BOX_SHELL_TEXT: &str = include_str!("../../../📚️examples/🐚️box-shell-preview/🖼️assets/🐚️box-shell-preview/🗣️.dsl.semio");
pub const GENERATION3D_EXAMPLE_MESH_WORKBENCH_TEXT: &str = include_str!("../../../📚️examples/🥽️mesh-workbench/🖼️assets/🥽️mesh-workbench/🗣️.dsl.semio");
//#endregion 🔖️Examples

//#region 🔖️DslMirror
pub use semio_s_artifact_procedural_3d_model::dsl_mirror::*;
//#endregion 🔖️DslMirror

/// 🔍️ Parses `.generation3d` DSL text into a `Generation3dSnapshot`.
pub fn parse_dsl(text: &str) -> Result<Generation3dSnapshot, semio_framework_diagnostic::TextError> {
    <Generation3dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `Generation3dSnapshot` back to `.generation3d` DSL text.
pub fn print_dsl(document: &Generation3dSnapshot) -> String {
    pack::snapshot::ArtifactDsl::print_dsl(document)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Generation3dSnapshotText = String;

pub use semio_s_artifact_procedural_3d_model::sqlite_codec::{decode_sqlite_native, encode_sqlite_native};
//#endregion 🚚️Carrier

mod source_examples {
use super::*;
use semio_s_artifact_procedural_3d_model::*;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use pack::snapshot::ArtifactDsl;
/// 📄️ The `procedural3d-play` "default" document — parsed from the bundled "hexagonal mushroom
/// column" example host_snapshot.
pub fn default_snapshot() -> Generation3dSnapshot {
    Generation3dSnapshot::parse_dsl(GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT).unwrap_or_default()
}
/// 🧾️ Builds the projection for a named bundled example; unknown ids return `None`.
pub fn example_snapshot(example_id: &str) -> Option<Generation3dSnapshot> {
    let dsl = match example_id {
        PROCEDURAL_EXAMPLE_HEX_COLUMN | "demo" => Some(GENERATION3D_EXAMPLE_HEX_COLUMN_TEXT),
        PROCEDURAL_EXAMPLE_RECT_EXTRUDE => Some(GENERATION3D_EXAMPLE_RECT_EXTRUDE_TEXT),
        PROCEDURAL_EXAMPLE_SPHERE_TORUS => Some(GENERATION3D_EXAMPLE_SPHERE_TORUS_TEXT),
        PROCEDURAL_EXAMPLE_BOX_FILLET => Some(GENERATION3D_EXAMPLE_BOX_FILLET_TEXT),
        PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE => Some(GENERATION3D_EXAMPLE_SPHERE_BOX_FUSE_TEXT),
        PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE => Some(GENERATION3D_EXAMPLE_FACE_SWEEP_EXTRUDE_TEXT),
        PROCEDURAL_EXAMPLE_RECTANGLE_WIRE => Some(GENERATION3D_EXAMPLE_RECTANGLE_WIRE_TEXT),
        PROCEDURAL_EXAMPLE_MESH_WORKBENCH => Some(GENERATION3D_EXAMPLE_MESH_WORKBENCH_TEXT),
        PROCEDURAL_EXAMPLE_BOX_SHELL => Some(GENERATION3D_EXAMPLE_BOX_SHELL_TEXT),
        _ => None,
    };
    dsl.and_then(|text| Generation3dSnapshot::parse_dsl(text).ok())
}
/// 📼️ Serializes an example's bare projection for registration via `App::example`.
pub fn example_document_json(example_id: &str) -> String {
    let snapshot = example_snapshot(example_id).unwrap_or_default();
    let json = semio_framework_pack_json::to_json_string(&snapshot);
    snapshot.retire_cold();
    json
}
}
pub use source_examples::*;

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use semio_s_artifact_procedural_3d_model::*;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_s_artifact_procedural_3d_model::widget_id;
use semio_framework_artifact_playbook_playbook::GenerationPlayRoot;
use semio_os_flow::forms_bridge::apply_generation_values_to_host_snapshot as apply_generation_values_to_host_snapshot_json;
use ::semio_framework_schema::ArtifactSchema;
use semio_os_flow::{flow_host_with_session, FlowEvalSession, FlowHost};
use semio_framework_value_derive::{FromValue, ToValue};
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_artifact_playbook_playbook::GenerationPlayState;

/// 🌉️ Bridges a `FormGeneration.values` map (`semio_framework_artifact_playbook_playbook::PlaybookValues`, see `FormGeneration`
/// in `📖️playbook/🦀️.rs`) into the `pack::json::Object` that `forms_bridge::apply_generation_values_to_host_snapshot`
/// actually takes.
pub(crate) fn generation_values_to_pack_object(values: &semio_framework_artifact_playbook_playbook::PlaybookValues) -> semio_framework_pack_json::Object {
    values.iter().map(|(key, value)| (key.clone(), semio_framework_pack_json::from_dsl_value(value))).collect()
}

/// 🔌️ The widget id behind a preview instance id. Instance ids are channel-qualified
/// (`{widgetId}@{channel}#{index}`) so both suffixes have to come off; a bare widget id, a port id
/// and an instance id therefore all resolve to the same widget.
pub fn widget_id_from_instance_id(instance_id: &str) -> &str {
    let base = instance_id.split('#').next().unwrap_or(instance_id);
    base.split('@').next().unwrap_or(base)
}

pub fn evaluate_generation_preview(host_snapshot: &FlowHostSnapshot, values: &semio_framework_artifact_playbook_playbook::PlaybookValues) -> String {
    let snapshot_json = semio_framework_pack_json::to_json_string(host_snapshot);
    let patched = apply_generation_values_to_host_snapshot_json(&snapshot_json, &generation_values_to_pack_object(values));
    let patched_fixture = FlowHost::parse_host_snapshot_json(&patched).unwrap_or_else(|_| host_snapshot.clone());
    let mut host = FlowHost::from_host_snapshot(patched_fixture).with_history_store_factory(::semio_framework_artifact_flow_flow::FLOW_HISTORY_STORE_FACTORY);
    host.set_neuron_kind_info_map(semio_os_flow::flow_neuron_kind_info_map());
    let evaluated = host.evaluate().unwrap_or_default();
    if host.retire_cold_or_park().is_err() {
        return String::new();
    }
    evaluated
}

}
pub use snapshot_codec::*;

#[path = "🗂️catalogue/🦀️.rs"]
pub mod catalogue;


