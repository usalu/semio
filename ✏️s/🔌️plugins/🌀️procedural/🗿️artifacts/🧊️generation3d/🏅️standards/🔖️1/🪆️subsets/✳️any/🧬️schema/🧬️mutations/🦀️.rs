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
#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
//#endregion 📖️SemioGrammar

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::widget_id;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationMutation;
use store::{os_store::store_schema::ArtifactEnvelope, ArtifactStore};
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_os_flow::FlowHost;

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
    #[path = "🗑️delete-generation/🧪️tests/🔓️removes-an-unselected-middle-row/🦀️.rs"]
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
#[derive(Clone, Debug, PartialEq, dsl::Mutations, semio_framework_value::RetireOwned)]
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

/// 🧮️ Borrows one native vector parameter, retaining the original per-axis fallback behavior.
pub(crate) fn generation3d_param_vector(params: &semio_framework_artifact_flow_flow::neural::Dictionary, key: &str, fallback: [f64; 3]) -> [f64; 3] {
    let vector = params.get(key).and_then(semio_framework_artifact_flow_flow::neural::Value::as_dictionary);
    let mut axes = fallback;
    for (axis, name) in ["x", "y", "z"].iter().enumerate() {
        if let Some(value) = vector.and_then(|vector| vector.get(name)).and_then(change_widget_input::neural_number) { axes[axis] = value; }
    }
    axes
}

/// 🔣️ Borrows one native numeric parameter, or the original fallback when absent.
pub(crate) fn generation3d_param_number(params: &semio_framework_artifact_flow_flow::neural::Dictionary, key: &str, fallback: f64) -> f64 {
    params.get(key).and_then(semio_framework_artifact_flow_flow::neural::Value::as_dictionary).and_then(|entry| entry.get("value")).and_then(change_widget_input::neural_number).unwrap_or(fallback)
}

/// 🧩️ Builds one native vector or point dictionary used by transform composition.
pub fn generation3d_vector_value(schema: &str, axes: [f64; 3]) -> semio_framework_artifact_flow_flow::neural::Value {
    use semio_framework_artifact_flow_flow::neural::{Atom, Dictionary, Value};
    let mut dictionary = Dictionary::with_schema(schema);
    for (axis, value) in ["x", "y", "z"].iter().zip(axes) { dictionary = dictionary.insert(*axis, Value::Atom(Atom::Decimal(value))); }
    Value::Dictionary(dictionary)
}

/// 🔟️ Builds one native number dictionary used by transform composition.
pub fn generation3d_number_value(value: f64) -> semio_framework_artifact_flow_flow::neural::Value {
    change_widget_input::WidgetInputValue::Number(value).neural_value()
}

/// 🪡️ Merges native parameter entries and explicitly retires the temporary patch owner.
pub(crate) fn generation3d_with_params(widget: &semio_framework_artifact_flow_flow::Widget, entries: Vec<(&str, semio_framework_artifact_flow_flow::neural::Value)>) -> semio_framework_artifact_flow_flow::Widget {
    use semio_framework_artifact_flow_flow::neural::{ColdRetire, Dictionary};
    let mut patch = Dictionary::new();
    for (key, entry) in entries { patch = patch.insert(key, entry); }
    let semio_framework_artifact_flow_flow::Widget::Neuron { id, neuron_kind, params, input_ports, output_ports, preview } = widget else {
        patch.retire_cold();
        return widget.clone();
    };
    let merged = params.merge(&patch);
    patch.retire_cold();
    semio_framework_artifact_flow_flow::Widget::Neuron { id: id.clone(), neuron_kind: neuron_kind.clone(), params: merged, input_ports: input_ports.clone(), output_ports: output_ports.clone(), preview: *preview }
}

/// 🪄️ The sparse delta of one relative gumball leaf: every target that is an operator of `kinds` gets the params
/// `compose` derives from its BASE params, so an edited gesture re-derives the operator on whatever base it replays on.
/// Missing targets and targets of another kind are skipped (`mutation.partial`); none left is `target-missing` (no
/// target exists) or `target-mismatch`; an identity gesture is `mutation.no-op`.
pub(crate) fn generation3d_transform_diff(base: &Generation3dSnapshot, targets: &[String], kinds: &[&str], identity: bool, compose: impl Fn(&semio_framework_artifact_flow_flow::neural::Dictionary) -> Option<Vec<(&'static str, semio_framework_artifact_flow_flow::neural::Value)>>) -> protocol::MutationOutcome<Generation3dDiff> {
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
        let semio_framework_artifact_flow_flow::Widget::Neuron { neuron_kind, params, .. } = widget else {
            mismatched.push(id.clone());
            continue;
        };
        if !kinds.contains(&neuron_kind.as_str()) {
            mismatched.push(id.clone());
            continue;
        }
        match compose(params).map(|entries| generation3d_with_params(widget, entries)) {
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
    let patched = composed.into_iter().map(|(index, widget)| semio_s_artifact_procedural_3d_model::diff::Generation3dWidgetModification { id: widget_id(&base.host_snapshot.widgets[index]).to_string(), patch: semio_s_artifact_procedural_3d_model::diff::Generation3dWidgetPatch::Replace { widget } }).collect();
    protocol::MutationOutcome::new(Generation3dDiff { widgets: Some(semio_s_artifact_procedural_3d_model::diff::Generation3dWidgetsDelta { modified: patched, ..Default::default() }), ..Default::default() }).absorb_messages(messages)
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


//#region 🧭️GraphEditor
/// 🧭️ The explicit-intent recorder of every graph edit a command authors: each verb applies one `FlowHost` authoring
/// operation — which validates it, mints its ids and measures its nodes — and records the declared mutation leaves
/// that operation IS, cascades included, in the order they replay. No whole-snapshot diff stands between the host and
/// the leaves: `remove_widget` records the `disconnect-synapse` and `delete-widget-position` leaves of the wires and
/// the layout entry it takes with it, `connect_ports` records the `disconnect-synapse` of the wire it displaces, and a
/// port insertion records the `update-widget` and the `update-synapse` leaves of the wires it renumbers.
///
/// 🧩️ The host is the validator and the measurer (its port, type and cycle rules, its widget factory, its layout
/// algorithm); the leaves are the document edit. Replaying [`GraphEditor::finish`]'s leaves on the snapshot the host was
/// opened on reaches the host's own snapshot (`graph_editor_replay_reaches_the_host_snapshot`).
pub struct GraphEditor<'a> {
    host: &'a mut FlowHost,
    leaves: Vec<crate::Generation3dMutation>,
}

impl<'a> GraphEditor<'a> {
    /// 🧭️ Opens a recorder over `host`.
    pub fn new(host: &'a mut FlowHost) -> Self {
        Self { host, leaves: Vec::new() }
    }

    /// 👁️ The host's current graph, for the reads a command decides on.
    pub fn snapshot(&self) -> &FlowHostSnapshot {
        &self.host.host_snapshot
    }

    /// 👁️ The host itself, read-only, for the queries that need more than the graph (a widget's value form).
    pub fn host(&self) -> &FlowHost {
        self.host
    }

    /// 📐️ The rendered rectangles `[x, y, width, height]` of every node except `except` — what an automatic placement avoids.
    pub fn occupied(&self, except: &str) -> Vec<[f64; 4]> {
        self.host.dag.host_snapshot.nodes.iter().filter(|node| node.id != except).map(|node| [node.x, node.y, node.width, node.height]).collect()
    }

    /// 📐️ The rendered size `[width, height]` of node `id`.
    pub fn size_of(&self, id: &str) -> Option<[f64; 2]> {
        self.host.dag.host_snapshot.nodes.iter().find(|node| node.id == id).map(|node| [node.width, node.height])
    }

    /// 🧾️ The leaves recorded, in replay order. The caller owns them and retires them cold.
    pub fn finish(mut self) -> Vec<crate::Generation3dMutation> {
        std::mem::take(&mut self.leaves)
    }

    fn layout_leaf(&mut self, id: &str) {
        use crate::move_widget::MoveWidget;
        use crate::Generation3dMutation;
        let Some(layout) = self.host.host_snapshot.layout.get(id).cloned() else { return };
        if let Some(Generation3dMutation::MoveWidget(last)) = self.leaves.last_mut() {
            if last.id == id {
                last.layout = layout;
                return;
            }
        }
        self.leaves.push(Generation3dMutation::MoveWidget(MoveWidget { id: id.to_string(), layout }));
    }

    fn widget_leaf(&mut self, id: &str) {
        use crate::update_widget::UpdateWidget;
        use crate::Generation3dMutation;
        if let Some(widget) = self.host.host_snapshot.widgets.iter().find(|widget| widget_id(widget) == id) {
            self.leaves.push(Generation3dMutation::UpdateWidget(UpdateWidget { widget: widget.clone() }));
        }
    }

    fn synapse_leaves(&mut self, before: &[semio_framework_artifact_flow_flow::SynapseSpec]) {
        use crate::connect_synapse::ConnectSynapse;
        use crate::disconnect_synapse::DisconnectSynapse;
        use crate::update_synapse::UpdateSynapse;
        use crate::Generation3dMutation;
        let after = &self.host.host_snapshot.synapses;
        let mut leaves: Vec<Generation3dMutation> = before.iter().filter(|old| !after.iter().any(|new| new.id == old.id)).map(|old| Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: old.id.clone() })).collect();
        leaves.extend(after.iter().filter(|new| before.iter().any(|old| old.id == new.id && old != *new)).map(|new| Generation3dMutation::UpdateSynapse(UpdateSynapse { synapse: new.clone() })));
        leaves.extend(after.iter().enumerate().filter(|(_, new)| !before.iter().any(|old| old.id == new.id)).map(|(index, new)| Generation3dMutation::ConnectSynapse(ConnectSynapse { index, synapse: new.clone() })));
        self.leaves.extend(leaves);
    }

    /// 🧩️ Creates the admitted widget at `(x, y)` and records its placement.
    pub fn add_widget(&mut self, descriptor: &semio_framework_artifact_flow_flow::WidgetDescriptor, x: f64, y: f64) -> Result<String, String> {
        use crate::create_widget::CreateWidget;
        use crate::Generation3dMutation;
        let index = self.host.host_snapshot.widgets.len();
        let id = self.host.add_widget_descriptor(descriptor, x, y).map_err(|error| error.to_string())?;
        let widget = self.host.host_snapshot.widgets.iter().find(|widget| widget_id(widget) == id).cloned().ok_or_else(|| format!("created widget {id} is missing"))?;
        self.leaves.push(Generation3dMutation::CreateWidget(CreateWidget { index, widget }));
        self.layout_leaf(&id);
        Ok(id)
    }

    /// 📍️ Places widget `id` at `(x, y)`: one `move-widget`.
    pub fn move_widget(&mut self, id: &str, x: f64, y: f64) -> Result<(), String> {
        self.host.move_widget(id, x, y).map_err(|error| error.to_string())?;
        self.layout_leaf(id);
        Ok(())
    }

    /// ➖️ Removes widget `id` with everything that hangs on it, as `generation3d_widget_removal` spells the cascade.
    pub fn remove_widget(&mut self, id: &str) -> Result<(), String> {
        let leaves = crate::generation3d_widget_removal(&self.host.host_snapshot, id);
        self.host.remove_widget(id).map_err(|error| error.to_string())?;
        self.leaves.extend(leaves);
        Ok(())
    }

    /// 🔗️ Wires an output to an input: the `disconnect-synapse` of the wire the input drops (an input takes one wire) and
    /// the `connect-synapse` at the end of the list. Answers the new wire's id.
    pub fn connect_ports(&mut self, from_id: &str, from_port: &str, to_id: &str, to_port: &str) -> Result<String, String> {
        let before = self.host.host_snapshot.synapses.clone();
        let id = self.host.connect_ports(from_id, from_port, to_id, to_port).map_err(|error| error.to_string())?;
        self.synapse_leaves(&before);
        Ok(id)
    }

    /// ✂️ Cuts wire `id`: one `disconnect-synapse`.
    pub fn disconnect(&mut self, id: &str) -> Result<(), String> {
        use crate::disconnect_synapse::DisconnectSynapse;
        use crate::Generation3dMutation;
        self.host.disconnect(id).map_err(|error| error.to_string())?;
        self.leaves.push(Generation3dMutation::DisconnectSynapse(DisconnectSynapse { id: id.to_string() }));
        Ok(())
    }

    /// 🔀️ Splices `mid_id` between `anchor_id` and the consumers of its `anchor_out_port`: the `update-synapse` of every
    /// consumer wire it re-sources and the `connect-synapse` of the wire that feeds it.
    pub fn insert_between(&mut self, anchor_id: &str, anchor_out_port: &str, mid_id: &str, mid_in_port: &str, mid_out_port: &str) -> Result<(), String> {
        let before = self.host.host_snapshot.synapses.clone();
        self.host.insert_between(anchor_id, anchor_out_port, mid_id, mid_in_port, mid_out_port).map_err(|error| error.to_string())?;
        self.synapse_leaves(&before);
        Ok(())
    }

    /// ➕️ Opens an input port of variadic operator `id` at `index`: the `update-widget` of its new port list and the
    /// `update-synapse` of every wire the insertion renumbers.
    pub fn add_input_port(&mut self, id: &str, index: usize) -> Result<(), String> {
        let before = self.host.host_snapshot.synapses.clone();
        self.host.add_input_port(id, index).map_err(|error| error.to_string())?;
        self.widget_leaf(id);
        self.synapse_leaves(&before);
        Ok(())
    }

    /// ➕️ Opens an output port of variadic operator `id` at `index`; see [`GraphEditor::add_input_port`].
    pub fn add_output_port(&mut self, id: &str, index: usize) -> Result<(), String> {
        let before = self.host.host_snapshot.synapses.clone();
        self.host.add_output_port(id, index).map_err(|error| error.to_string())?;
        self.widget_leaf(id);
        self.synapse_leaves(&before);
        Ok(())
    }

    /// 👁️ Sets operator `id`'s preview flag: one `update-widget` when it flips. Answers whether it did.
    pub fn set_preview(&mut self, id: &str, shown: bool) -> bool {
        let mut flipped = false;
        for widget in &mut self.host.host_snapshot.widgets {
            if let Widget::Neuron { id: widget_id, preview, .. } = widget {
                if widget_id == id && *preview != shown {
                    *preview = shown;
                    flipped = true;
                }
            }
        }
        if flipped {
            self.widget_leaf(id);
        }
        flipped
    }

    /// 🗺️ Lays the graph out again with the host's layered layout: one `move-widget` per widget that moves.
    pub fn reorganize(&mut self, options: &semio_framework_os_infinite::board::schema::layout::DagLayoutOptions, control: &mut semio_framework_os_infinite::board::schema::layout::LayoutControl<'_>) -> Result<(), String> {
        let before = self.host.host_snapshot.layout.clone();
        let reorganized = self.host.reorganize(options, control).map_err(|error| error.to_string());
        let moved: Vec<String> = self.host.host_snapshot.layout.iter().filter(|(id, layout)| before.get(id) != Some(*layout)).map(|(id, _)| id.clone()).collect();
        let mut retirement = semio_framework_artifact_flow_flow::retained::FlowRetirement::default();
        retirement.push(semio_framework_artifact_flow_flow::retained::FlowOwner::Layouts(before));
        retirement.retire_cold();
        reorganized?;
        for id in moved {
            self.layout_leaf(&id);
        }
        Ok(())
    }
}

/// 🧊️ A recorder dropped with leaves still in hand — a command refused after it recorded — retires them cold: a recorded
/// `create-widget` owns a widget whose neural dictionary rejects a bare drop.
impl Drop for GraphEditor<'_> {
    fn drop(&mut self) {
        for leaf in self.leaves.drain(..) {
            leaf.retire_cold();
        }
    }
}
//#endregion 🧭️GraphEditor

impl crate::GraphEditor<'_> {
    /// 📥️ Admits a widget descriptor at the editor text boundary before recording native intent.
    pub fn add_widget_json(&mut self, text: &str, x: f64, y: f64) -> Result<String, String> {
        let descriptor = semio_framework_pack_json::from_json_str::<semio_framework_artifact_flow_flow::WidgetDescriptor>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
        self.add_widget(&descriptor, x, y)
    }
}

#[path = "../../../../../../🌉️apply/🦀️.rs"]
pub mod central_apply;
