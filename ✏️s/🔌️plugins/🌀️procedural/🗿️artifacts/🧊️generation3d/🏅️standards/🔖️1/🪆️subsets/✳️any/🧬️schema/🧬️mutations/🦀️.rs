//! ⚡️ Generation3d artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `Generation3dSnapshot`'s shape per `📓️derivation-rules.md`: an id-keyed widget
//! collection (`create`/`update`/`delete-widget`), a relationship/edge collection of synapses
//! (`connect`/`update`/`disconnect-synapse`), a per-widget position map (`move-widget` /
//! `delete-widget-position`), two document-level scalars (`update-camera`, `change-schema`), an
//! id-keyed generation collection bridged from `semio_framework_artifact_playbook_playbook::GenerationMutation`
//! (`create`/`delete`/`rename-generation`, `change-generation-value`) with its two document-level scalars
//! (`select-generation`, `change-generation-preview`), and six gesture intents (`change-slider-value`,
//! `drag`/`rotate`/`scale-transforms`, `move-nodes`, `change-widget-input`). Every variant wraps exactly
//! one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! `protocol::MutationKind<Generation3dSnapshot, Generation3dMutation>`; `#[derive(dsl::Mutations)]`
//! below generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to each
//! payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` for the reference shape.
//!
//! Every leaf folder's stem equals its `semanticKind` and every leaf is wired by [`Leaves`](self) below, in enum
//! order — no other file path-includes a leaf.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
//#endregion 📖️SemioGrammar

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::{widget_id, Generation3dSnapshot};
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationMutation;
use store::{ArtifactEnvelope, ArtifactStore};

//#region 🔖️AddressHelpers
/// 🔎️ BASE-state widget index lookup by id — shared by every widget triad leaf's inverse.
pub(crate) fn widget_index(host_snapshot: &FlowHostSnapshot, id: &str) -> Option<usize> {
    host_snapshot.widgets.iter().position(|widget| widget_id(widget) == id)
}

/// 🔎️ BASE-state synapse index lookup by id — shared by every synapse triad leaf's inverse.
pub(crate) fn synapse_index(host_snapshot: &FlowHostSnapshot, id: &str) -> Option<usize> {
    host_snapshot.synapses.iter().position(|synapse| synapse.id == id)
}
//#endregion 🔖️AddressHelpers

//#region 🔖️Leaves
// 🌱️ One self-wired module per mutation leaf folder, in enum order. A leaf folder's stem is its `semanticKind`.
#[path = "."]
pub mod create_widget {
    #[path = "🌱️create-widget/🦀️.rs"]
    mod component;
    #[path = "🌱️create-widget/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌱️create-widget/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🌱️create-widget/🧪️tests/📝️inserts/🦀️.rs"]
    mod tests_inserts_node_c_at_index_2;
}

#[path = "."]
pub mod update_widget {
    #[path = "🩹update-widget/🦀️.rs"]
    mod component;
    #[path = "🩹update-widget/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🩹update-widget/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🩹update-widget/🧪️tests/🎚️retunes/🦀️.rs"]
    mod tests_retunes_the_knob_slider_value;
}

#[path = "."]
pub mod delete_widget {
    #[path = "❌delete-widget/🦀️.rs"]
    mod component;
    #[path = "❌delete-widget/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "❌delete-widget/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "❌delete-widget/🧪️tests/🚫️removes/🦀️.rs"]
    mod tests_removes_node_a_and_leaves_wire_ab_dangling;
    #[cfg(test)]
    #[path = "❌delete-widget/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
    mod tests_removes_a_middle_row;
}

#[path = "."]
pub mod connect_synapse {
    #[path = "🔗️connect-synapse/🦀️.rs"]
    mod component;
    #[path = "🔗️connect-synapse/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔗️connect-synapse/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🔗️connect-synapse/🧪️tests/🔌️wires/🦀️.rs"]
    mod tests_wires_node_b_to_node_c_at_index_1;
}

#[path = "."]
pub mod update_synapse {
    #[path = "🔄️update-synapse/🦀️.rs"]
    mod component;
    #[path = "🔄️update-synapse/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔄️update-synapse/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🔄️update-synapse/🧪️tests/📡️repoints/🦀️.rs"]
    mod tests_repoints_wire_ab_onto_the_cap_port;
}

#[path = "."]
pub mod disconnect_synapse {
    #[path = "✂️disconnect-synapse/🦀️.rs"]
    mod component;
    #[path = "✂️disconnect-synapse/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "✂️disconnect-synapse/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "✂️disconnect-synapse/🧪️tests/✂️cuts/🦀️.rs"]
    mod tests_cuts_wire_ab_leaving_both_nodes;
    #[cfg(test)]
    #[path = "✂️disconnect-synapse/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
    mod tests_removes_a_middle_row;
}

#[path = "."]
pub mod move_widget {
    #[path = "📍️move-widget/🦀️.rs"]
    mod component;
    #[path = "📍️move-widget/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📍️move-widget/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "📍️move-widget/🧪️tests/📍️repositions/🦀️.rs"]
    mod tests_repositions_node_a_in_the_graph;
}

#[path = "."]
pub mod delete_widget_position {
    #[path = "🧹️delete-widget-position/🦀️.rs"]
    mod component;
    #[path = "🧹️delete-widget-position/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧹️delete-widget-position/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🧹️delete-widget-position/🧪️tests/🧹️unpins/🦀️.rs"]
    mod tests_unpins_the_node_a_position;
}

#[path = "."]
pub mod update_camera {
    #[path = "📷️update-camera/🦀️.rs"]
    mod component;
    #[path = "📷️update-camera/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📷️update-camera/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "📷️update-camera/🧪️tests/🔍️frames/🦀️.rs"]
    mod tests_frames_the_graph_at_double_zoom;
}

#[path = "."]
pub mod change_schema {
    #[path = "🔤️change-schema/🦀️.rs"]
    mod component;
    #[path = "🔤️change-schema/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔤️change-schema/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🔤️change-schema/🧪️tests/🏷️restamps/🦀️.rs"]
    mod tests_restamps_the_fixture_schema_id;
}

#[path = "."]
pub mod create_generation {
    #[path = "➕create-generation/🦀️.rs"]
    mod component;
    #[path = "➕create-generation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "➕create-generation/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "➕create-generation/🧪️tests/🌱️appends/🦀️.rs"]
    mod tests_appends_generation_2_and_moves_the_selection;
}

#[path = "."]
pub mod delete_generation {
    #[path = "🗑️delete-generation/🦀️.rs"]
    mod component;
    #[path = "🗑️delete-generation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🗑️delete-generation/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🗑️delete-generation/🧪️tests/🚫️removes/🦀️.rs"]
    mod tests_removes_the_selected_generation_2_and_falls_back;
    #[cfg(test)]
    #[path = "🗑️delete-generation/🧪️tests/📍️removes-a-middle-row/🦀️.rs"]
    mod tests_removes_a_middle_row;
    #[cfg(test)]
    #[path = "❌delete-widget/🧪️tests/🔓️removes-an-unselected-middle-row/🦀️.rs"]
    mod tests_removes_an_unselected_middle_row;
}

#[path = "."]
pub mod rename_generation {
    #[path = "🏷️rename-generation/🦀️.rs"]
    mod component;
    #[path = "🏷️rename-generation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏷️rename-generation/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🏷️rename-generation/🧪️tests/🏷️retitles/🦀️.rs"]
    mod tests_retitles_generation_1_via_new_name;
}

#[path = "."]
pub mod change_generation_value {
    #[path = "🔧️change-generation-value/🦀️.rs"]
    mod component;
    #[path = "🔧️change-generation-value/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔧️change-generation-value/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🔧️change-generation-value/🧪️tests/🏢️raises/🦀️.rs"]
    mod tests_raises_the_storeys_answer_in_generation_1;
}

#[path = "."]
pub mod change_slider_value {
    #[path = "🎚️change-slider-value/🦀️.rs"]
    mod component;
    #[path = "🎚️change-slider-value/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🎚️change-slider-value/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🎚️change-slider-value/🧪️tests/🎚️sets/🦀️.rs"]
    mod tests_sets_the_knob_slider_within_its_range;
}

#[path = "."]
pub mod drag_transforms {
    #[path = "✋️drag-transforms/🦀️.rs"]
    mod component;
    #[path = "✋️drag-transforms/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "✋️drag-transforms/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "✋️drag-transforms/🧪️tests/✋️drags/🦀️.rs"]
    mod tests_drags_the_translate_operator_by_the_gesture_offset;
}

#[path = "."]
pub mod rotate_transforms {
    #[path = "🔃️rotate-transforms/🦀️.rs"]
    mod component;
    #[path = "🔃️rotate-transforms/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔃️rotate-transforms/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🔃️rotate-transforms/🧪️tests/🔃️turns/🦀️.rs"]
    mod tests_turns_the_rotate_operator_half_a_radian_about_z;
}

#[path = "."]
pub mod scale_transforms {
    #[path = "📏️scale-transforms/🦀️.rs"]
    mod component;
    #[path = "📏️scale-transforms/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📏️scale-transforms/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "📏️scale-transforms/🧪️tests/📏️scales/🦀️.rs"]
    mod tests_scales_the_scale_operator_per_axis;
}

#[path = "."]
pub mod move_nodes {
    #[path = "🚚️move-nodes/🦀️.rs"]
    mod component;
    #[path = "🚚️move-nodes/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🚚️move-nodes/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🚚️move-nodes/🧪️tests/🚚️shifts/🦀️.rs"]
    mod tests_shifts_both_nodes_by_the_drag_offset;
}

#[path = "."]
pub mod change_widget_input {
    #[path = "🎛️change-widget-input/🦀️.rs"]
    mod component;
    #[path = "🎛️change-widget-input/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🎛️change-widget-input/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🎛️change-widget-input/🧪️tests/🎛️sets/🦀️.rs"]
    mod tests_sets_the_extrude_distance_input;
}

#[path = "."]
pub mod select_generation {
    #[path = "👆️select-generation/🦀️.rs"]
    mod component;
    #[path = "👆️select-generation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "👆️select-generation/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "👆️select-generation/🧪️tests/👆️picks/🦀️.rs"]
    mod tests_selects_generation_2;
}

#[path = "."]
pub mod change_generation_preview {
    #[path = "📝️change-generation-preview/🦀️.rs"]
    mod component;
    #[path = "📝️change-generation-preview/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📝️change-generation-preview/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "📝️change-generation-preview/🧪️tests/📝️retexts/🦀️.rs"]
    mod tests_restores_the_imported_preview_text;
}

//#endregion 🔖️Leaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the generation3d document, derived per
/// `📓️derivation-rules.md` from `Generation3dSnapshot`'s shape.
#[derive(Clone, Debug, PartialEq, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = Generation3dSnapshot, diff = Generation3dDiff, schema = "generation.3d")]
pub enum Generation3dMutation {
    CreateWidget(create_widget::CreateWidget),
    UpdateWidget(update_widget::UpdateWidget),
    DeleteWidget(delete_widget::DeleteWidget),
    ConnectSynapse(connect_synapse::ConnectSynapse),
    UpdateSynapse(update_synapse::UpdateSynapse),
    DisconnectSynapse(disconnect_synapse::DisconnectSynapse),
    MoveWidget(move_widget::MoveWidget),
    DeleteWidgetPosition(delete_widget_position::DeleteWidgetPosition),
    UpdateCamera(update_camera::UpdateCamera),
    ChangeSchema(change_schema::ChangeSchema),
    CreateGeneration(create_generation::CreateGeneration),
    DeleteGeneration(delete_generation::DeleteGeneration),
    RenameGeneration(rename_generation::RenameGeneration),
    ChangeGenerationValue(change_generation_value::ChangeGenerationValue),
    ChangeSliderValue(change_slider_value::ChangeSliderValue),
    DragTransforms(drag_transforms::DragTransforms),
    RotateTransforms(rotate_transforms::RotateTransforms),
    ScaleTransforms(scale_transforms::ScaleTransforms),
    MoveNodes(move_nodes::MoveNodes),
    ChangeWidgetInput(change_widget_input::ChangeWidgetInput),
    SelectGeneration(select_generation::SelectGeneration),
    ChangeGenerationPreview(change_generation_preview::ChangeGenerationPreview),
}

//#region 🏷️Kinds
/// 🏷️ The kebab-case spelling of every [`Generation3dMutation`] variant, in declaration order — the exact
/// vocabulary the `procedural-3d-1-any` mutation catalog (`../../🔣️oracle.json`) declares and
/// the `🧊️mutate-procedural-3d-1` exhaustive case measures itself against. The framework never parses Rust, so
/// `kinds_match_the_enum_and_the_catalog` below is what keeps this list honest against both.
pub const KINDS: &[&str] = &[
    "create-widget",
    "update-widget",
    "delete-widget",
    "connect-synapse",
    "update-synapse",
    "disconnect-synapse",
    "move-widget",
    "delete-widget-position",
    "update-camera",
    "change-schema",
    "create-generation",
    "delete-generation",
    "rename-generation",
    "change-generation-value",
    "change-slider-value",
    "drag-transforms",
    "rotate-transforms",
    "scale-transforms",
    "move-nodes",
    "change-widget-input",
    "select-generation",
    "change-generation-preview",
];
//#endregion 🏷️Kinds
//#endregion 🔖️Mutations

//#region 🔖️GestureLeaves
/// 🖊️ One number as the history labels print it: English with a decimal point, German with a decimal comma.
pub(crate) fn generation3d_label_number(value: f64) -> (String, String) {
    let english = format!("{}", (value * 1_000.0).round() / 1_000.0);
    let german = english.replace('.', ",");
    (english, german)
}

/// 🧺️ `count` items as the history labels name them, English and German.
pub(crate) fn generation3d_label_items(count: usize, english: &str, german: &str) -> (String, String) {
    (format!("{count} {english}"), format!("{count} {german}"))
}

/// 🧱️ The payload-intrinsic target law every relative gesture leaf states in its schema: at least one id, each once.
pub(crate) fn generation3d_targets_invariant(targets: &[String]) -> Result<(), &'static str> {
    if targets.is_empty() || targets.iter().any(String::is_empty) {
        return Err("a gesture leaf names at least one non-empty target");
    }
    if targets.iter().enumerate().any(|(at, id)| targets[..at].contains(id)) {
        return Err("a gesture leaf names each target once");
    }
    Ok(())
}

/// 🩹️ The `mutation.partial` warning of the targets a relative leaf skipped, or nothing.
pub(crate) fn generation3d_partial(skipped: Vec<String>, total: usize, reason: &str) -> Option<protocol::MutationMessage> {
    (!skipped.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {total} target(s) skipped ({reason}): {}", skipped.len(), skipped.join(", "))).at(skipped))
}

/// 🎛️ The neuron kinds one gumball composition addresses: the B-Rep transform, the mesh transform and the mesh
/// component transform of the same operation.
pub(crate) const GENERATION3D_TRANSLATE_KINDS: [&str; 3] = ["brep.xform.translate", "brep.mesh.translate", "brep.mesh.translateComponents"];
pub(crate) const GENERATION3D_ROTATE_KINDS: [&str; 3] = ["brep.xform.rotate", "brep.mesh.rotate", "brep.mesh.rotateComponents"];
pub(crate) const GENERATION3D_SCALE_KINDS: [&str; 3] = ["brep.xform.scale", "brep.mesh.scale", "brep.mesh.scaleComponents"];

/// 🧮️ The `x`/`y`/`z` of one vector param of a neuron's params in value form, or `fallback` where absent.
pub(crate) fn generation3d_param_vector(params: &semio_framework_value::DslValue, key: &str, fallback: [f64; 3]) -> [f64; 3] {
    let vector = params.get(key);
    let mut axes = fallback;
    for (axis, name) in ["x", "y", "z"].iter().enumerate() {
        if let Some(value) = vector.and_then(|vector| vector.get(name)).and_then(semio_framework_value::DslValue::as_f64) {
            axes[axis] = value;
        }
    }
    axes
}

/// 🔣️ The `value` of one number param of a neuron's params in value form, or `fallback` where absent.
pub(crate) fn generation3d_param_number(params: &semio_framework_value::DslValue, key: &str, fallback: f64) -> f64 {
    params.get(key).and_then(|entry| entry.get("value")).and_then(semio_framework_value::DslValue::as_f64).unwrap_or(fallback)
}

/// 🧩️ One typed vector literal (`{"$schema": schema, x, y, z}`) a transform operator param holds.
pub fn generation3d_vector_literal(schema: &str, axes: [f64; 3]) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::object([
        ("$schema".to_string(), semio_framework_value::DslValue::String(schema.into())),
        ("x".to_string(), semio_framework_value::DslValue::float(axes[0])),
        ("y".to_string(), semio_framework_value::DslValue::float(axes[1])),
        ("z".to_string(), semio_framework_value::DslValue::float(axes[2])),
    ])
}

/// 🔟️ One typed number literal (`{"$schema": "number", value}`) a transform operator param holds.
pub fn generation3d_number_literal(value: f64) -> semio_framework_value::DslValue {
    semio_framework_value::DslValue::object([("$schema".to_string(), semio_framework_value::DslValue::String("number".into())), ("value".to_string(), semio_framework_value::DslValue::float(value))])
}

/// 🪡️ `widget` with `entries` merged into its params: the one way a relative transform leaf builds an operator. `None`
/// when a param value is not a neural value; the patch is retired cold, never dropped.
pub(crate) fn generation3d_with_params(widget: &semio_framework_artifact_flow_flow::Widget, entries: Vec<(&str, semio_framework_value::DslValue)>) -> Option<semio_framework_artifact_flow_flow::Widget> {
    use semio_framework_artifact_flow_flow::neural::{ColdRetire, Dictionary, Value};
    let mut patch = Dictionary::new();
    for (key, entry) in entries {
        match <Value as semio_framework_value::FromValue>::from_value(entry) {
            Ok(value) => patch = patch.insert(key, value),
            Err(_) => {
                patch.retire_cold();
                return None;
            }
        }
    }
    let semio_framework_artifact_flow_flow::Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } = widget else {
        patch.retire_cold();
        return Some(widget.clone());
    };
    let merged = params.merge(&patch);
    patch.retire_cold();
    Some(semio_framework_artifact_flow_flow::Widget::Neuron { id: id.clone(), neuron_kind: neuron_kind.clone(), params: merged, input_ports: input_ports.clone(), output_ports: output_ports.clone(), preview: *preview })
}

/// 🪄️ The sparse delta of one relative gumball leaf: every target that is an operator of `kinds` gets the params
/// `compose` derives from its BASE params, so an edited gesture re-derives the operator on whatever base it replays on.
/// Missing targets and targets of another kind are skipped (`mutation.partial`); none left is `target-missing` (no
/// target exists) or `target-mismatch`; an identity gesture is `mutation.no-op`.
pub(crate) fn generation3d_transform_diff(base: &Generation3dSnapshot, targets: &[String], kinds: &[&str], identity: bool, compose: impl Fn(&semio_framework_value::DslValue) -> Option<Vec<(&'static str, semio_framework_value::DslValue)>>) -> protocol::MutationOutcome<Generation3dDiff> {
    if let Err(reason) = generation3d_targets_invariant(targets) {
        return protocol::MutationOutcome::fatal("mutation.invariant", reason, targets.to_vec());
    }
    let (mut missing, mut mismatched, mut composed) = (Vec::new(), Vec::new(), Vec::new());
    for id in targets {
        let Some(index) = widget_index(&base.host_snapshot, id) else {
            missing.push(id.clone());
            continue;
        };
        let widget = &base.host_snapshot.widgets[index];
        let semio_framework_artifact_flow_flow::Widget::Neuron { neuron_kind, .. } = widget else {
            mismatched.push(id.clone());
            continue;
        };
        if !kinds.contains(&neuron_kind.as_str()) {
            mismatched.push(id.clone());
            continue;
        }
        let params = semio_framework_value::ToValue::to_value(widget).get("params").cloned().unwrap_or(semio_framework_value::DslValue::Null);
        match compose(&params).and_then(|entries| generation3d_with_params(widget, entries)) {
            Some(next) => composed.push((index, next)),
            None => mismatched.push(id.clone()),
        }
    }
    let messages: Vec<protocol::MutationMessage> = [generation3d_partial(missing.clone(), targets.len(), "no such operator"), generation3d_partial(mismatched.clone(), targets.len(), "not an operator of this transform")].into_iter().flatten().collect();
    if composed.is_empty() {
        return if mismatched.is_empty() {
            protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} transform operator(s) exists", targets.len()), missing)
        } else {
            protocol::MutationOutcome::error("mutation.target-mismatch", format!("none of the {} target(s) is an operator this transform composes into", targets.len()), mismatched)
        };
    }
    if identity {
        for (_, widget) in composed {
            widget.retire_cold();
        }
        return protocol::MutationOutcome::empty().absorb_messages(messages.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "the gesture is the identity transform").at(targets.to_vec())]));
    }
    let patched = composed.into_iter().map(|(index, widget)| crate::standards::v1::subsets::any::schema::diff::Generation3dWidgetModification { id: widget_id(&base.host_snapshot.widgets[index]).to_string(), patch: crate::standards::v1::subsets::any::schema::diff::Generation3dWidgetPatch::Replace { widget } }).collect();
    protocol::MutationOutcome::new(crate::standards::v1::subsets::any::schema::diff::Generation3dDiff { widgets: Some(crate::standards::v1::subsets::any::schema::diff::Generation3dWidgetsDelta { modified: patched, ..Default::default() }), ..Default::default() }).absorb_messages(messages)
}

/// 🔙️ The exact inverse of one relative gumball leaf: every operator it would compose restored to its BASE widget —
/// absolute rows, never a negated delta.
pub(crate) fn generation3d_transform_inverse(base: &Generation3dSnapshot, targets: &[String], kinds: &[&str]) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    targets
        .iter()
        .filter_map(|id| base.host_snapshot.widgets.iter().find(|widget| widget_id(widget) == id))
        .filter(|widget| matches!(widget, semio_framework_artifact_flow_flow::Widget::Neuron { neuron_kind, .. } if kinds.contains(&neuron_kind.as_str())))
        .map(|widget| Generation3dMutation::UpdateWidget(update_widget::UpdateWidget { widget: widget.clone() }))
        .collect()

    })())
}
//#endregion 🔖️GestureLeaves

//#region 🔖️GenerationBridge
/// 🌉️ Bridges one `semio_framework_artifact_playbook_playbook::GenerationMutation` (the framework's own generation-editing
/// vocabulary — `Add`/`Remove`/`Rename`/`UpdateValues`) onto this facet's semantic
/// `Generation3dMutation` variants, so app-layer callers that already hold a `GenerationMutation`
/// (from `semio_framework_artifact_playbook_playbook::generation_operations`) need only swap the mapping function at the call
/// site, not learn this facet's internal triad-leaf module paths.
pub fn generation_mutation_to_generation3d(operation: GenerationMutation) -> Generation3dMutation {
    match operation {
        GenerationMutation::Add { generation } => Generation3dMutation::CreateGeneration(create_generation::CreateGeneration { generation, index: None }),
        GenerationMutation::Remove { id } => Generation3dMutation::DeleteGeneration(delete_generation::DeleteGeneration { id }),
        GenerationMutation::Rename { id, name } => Generation3dMutation::RenameGeneration(rename_generation::RenameGeneration { id, new_name: name }),
        GenerationMutation::UpdateValues { id, question_id, value } => Generation3dMutation::ChangeGenerationValue(change_generation_value::ChangeGenerationValue { id, question_id, new_value: value }),
    }
}
//#endregion 🔖️GenerationBridge

//#region 🔖️Cascades
/// 🗑️ The leaves that remove widget `id` with everything that hangs on it, in replay order: the `disconnect-synapse` of every
/// wire naming it, the `delete-widget-position` of its layout entry (before the widget goes — the position leaf addresses a
/// live widget) and the `delete-widget`. `delete-widget` itself does not cascade; this is where the cascade is spelled. No
/// leaves when the document holds no such widget.
pub fn generation3d_widget_removal(host_snapshot: &FlowHostSnapshot, id: &str) -> Vec<Generation3dMutation> {
    if widget_index(host_snapshot, id).is_none() {
        return Vec::new();
    }
    let mut leaves: Vec<Generation3dMutation> = host_snapshot.synapses.iter().filter(|synapse| synapse.from == id || synapse.to == id).map(|synapse| Generation3dMutation::DisconnectSynapse(disconnect_synapse::DisconnectSynapse { id: synapse.id.clone() })).collect();
    if host_snapshot.layout.contains_key(id) {
        leaves.push(Generation3dMutation::DeleteWidgetPosition(delete_widget_position::DeleteWidgetPosition { id: id.to_string() }));
    }
    leaves.push(Generation3dMutation::DeleteWidget(delete_widget::DeleteWidget { id: id.to_string() }));
    leaves
}

/// 🗑️ The leaves that delete a selection of wires and widgets: every selected wire is cut first, then every selected widget
/// goes with the wires it still holds; an id the document does not hold is skipped and no wire is cut twice.
pub fn generation3d_selection_removal(host_snapshot: &FlowHostSnapshot, selected: &[String]) -> Vec<Generation3dMutation> {
    let mut cut: Vec<String> = Vec::new();
    let mut leaves = Vec::new();
    for id in selected {
        if synapse_index(host_snapshot, id).is_some() && !cut.contains(id) {
            cut.push(id.clone());
            leaves.push(Generation3dMutation::DisconnectSynapse(disconnect_synapse::DisconnectSynapse { id: id.clone() }));
        }
    }
    let mut removed: Vec<&String> = Vec::new();
    for id in selected {
        if removed.contains(&id) {
            continue;
        }
        removed.push(id);
        for leaf in generation3d_widget_removal(host_snapshot, id) {
            if let Generation3dMutation::DisconnectSynapse(wire) = &leaf {
                if cut.contains(&wire.id) {
                    continue;
                }
                cut.push(wire.id.clone());
            }
            leaves.push(leaf);
        }
    }
    leaves
}
//#endregion 🔖️Cascades


pub type Generation3dEnvelope = ArtifactEnvelope<Generation3dSnapshot, Generation3dMutation>;
pub type Generation3dStore = ArtifactStore<Generation3dSnapshot, Generation3dMutation>;

//#region 🧊️Retirement
impl Generation3dMutation {
    /// 🧊️ Explicit cold-only disposal of one owned mutation. `create-widget`/`update-widget` carry a
    /// whole `Widget`, and a `Neuron`/`OutputPreview`/`Cluster` widget owns a `neural::Dictionary`
    /// (and an `OrderedSet`/`Tree`) that FAIL-CLOSE on a bare drop
    /// (`🧠️neural/⚙️engine/🦀️.rs`'s `Drop`), so a dropped mutation aborts the process. Every other
    /// variant is strings, numbers and plain `DslValue`s, which drop freely
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn retire_cold(self) {
        match self {
            Self::CreateWidget(create_widget::CreateWidget { widget, .. }) | Self::UpdateWidget(update_widget::UpdateWidget { widget }) => widget.retire_cold(),
            _ => {}
        }
    }
}
//#endregion 🧊️Retirement


pub fn inverse_generation3d_mutation(projection: &Generation3dSnapshot, mutation: &Generation3dMutation) -> Result<Vec<Generation3dMutation>, semio_framework_value::ValueError> {
    Ok({
    protocol::Mutation::inverse(mutation, projection)?

    })
}
//#endregion 🔖️Apply

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧪️gesture-leaves/🦀️.rs"]
mod gesture_leaves_tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/📄️document-restoration/🦀️.rs"]
mod document_restoration_tests;
