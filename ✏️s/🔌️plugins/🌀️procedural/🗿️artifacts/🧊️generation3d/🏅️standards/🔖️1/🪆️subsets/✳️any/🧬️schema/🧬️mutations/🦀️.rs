//! ⚡️ Generation3d artifact — closed semantic mutation dispatch enum (constitutional: op).
//!
//! Derived from `Generation3dSnapshot`'s shape per `📓️derivation-rules.md`: an id-keyed widget
//! collection (`create`/`update`/`delete-widget`), a relationship/edge collection of synapses
//! (`connect`/`update`/`disconnect-synapse`), a per-widget position map (`move-widget` /
//! `delete-widget-position`), two document-level scalars (`update-camera`, `change-schema`), and an
//! id-keyed generation collection bridged from `semio_framework_artifact_playbook_playbook::GenerationMutation`
//! (`create`/`delete`/`rename-generation`, `change-generation-value`). Every variant wraps exactly
//! one `🧬️mutations/<kind>/🦠️mutation` payload struct implementing
//! `protocol::MutationKind<Generation3dSnapshot, Generation3dMutation>`; `#[derive(dsl::Mutations)]`
//! below generates `impl protocol::Mutation`/`impl protocol::SemanticMutation` by delegating to each
//! payload's own `diff`/`inverse` — see `🧪️MutationsDeriveLaws` in
//! `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs` for the reference shape.
//!
//! `SetWidget`/`RemoveWidget`/`SetSynapse`/`RemoveSynapse`/`SetLayout`/`RemoveLayout`/`SetCamera`/
//! `SetSchema`/`Generation(GenerationMutation)` — the pre-migration generic vocabulary — are gone.
//! Eight triad-leaf directories keep their pre-migration `➖remove-*`/`🎛set-*` names: glue.rs
//! path-includes those exact files and this facet's writable boundary excludes glue.rs, so the
//! directories couldn't be renamed alongside their content — see the migration report's
//! `sharedFileRequests` for the exact rename once a later pass can touch glue.rs.

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
//#endregion 📖️SemioGrammar

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::{widget_id, Generation3dSnapshot};
use semio_framework_artifact_flow_flow::FlowHostSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationMutation;
use semio_framework_value_derive::{FromValue, ToValue};
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

//#region 🔖️NewLeaves
// 🌱️ Triad leaves that needed a fresh directory (no pre-migration slot to repurpose) — self-wired
// here since glue.rs is outside this facet's writable boundary; the eight leaves already carrying a
// semantic name (`delete_widget_position`/`disconnect_synapse`/`delete_widget`/`update_camera`/
// `move_widget`/`change_schema`/`update_synapse`/`update_widget`) stay wired by glue.rs's existing
// sibling `pub mod` blocks, unchanged — imported by those names just below.
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
    #[path = "🗑️delete/🦀️.rs"]
    mod component;
    #[path = "🗑️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🗑️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🗑️delete/🧪️tests/🚫️removes/🦀️.rs"]
    mod tests_removes_the_selected_generation_2_and_falls_back;
}

#[path = "."]
pub mod rename_generation {
    #[path = "🏷️rename/🦀️.rs"]
    mod component;
    #[path = "🏷️rename/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏷️rename/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🏷️rename/🧪️tests/🏷️retitles/🦀️.rs"]
    mod tests_retitles_generation_1_via_new_name;
}

#[path = "."]
pub mod change_generation_value {
    #[path = "🔧️change/🦀️.rs"]
    mod component;
    #[path = "🔧️change/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔧️change/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
    #[cfg(test)]
    #[path = "🔧️change/🧪️tests/🏢️raises/🦀️.rs"]
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
}
//#endregion 🔖️NewLeaves

//#region 🔖️RepurposedLeaves
// 🌱️ Triad leaves that repurpose a pre-migration `➖remove-*`/`🎛set-*` directory glue.rs already
// path-includes as a sibling of `component` (this file) under `pub mod mutations { ... }` — brought
// into this file's own scope the same way `cad`'s already-migrated `🧬️mutations/🦀️.rs`
// reaches its own siblings (`use super::create_object;` etc.): `pub use component::*` only lifts
// `component`'s items UP into `mutations`, it doesn't inject `mutations`'s OTHER children back down.
use super::change_schema;
use super::delete_widget;
use super::delete_widget_position;
use super::disconnect_synapse;
use super::move_widget;
use super::update_camera;
use super::update_synapse;
use super::update_widget;
//#endregion 🔖️RepurposedLeaves

//#region 🔖️Mutations
/// 🧬️ Closed semantic mutation vocabulary for the generation3d document, derived per
/// `📓️derivation-rules.md` from `Generation3dSnapshot`'s shape.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[value(tag = "mutation", rename_all = "camelCase")]
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

/// 🪡️ `widget` with `entries` merged into its params: the one way a relative transform leaf writes an operator. `None`
/// when a param value is not a neural value; the patch and the displaced params are retired cold, never dropped.
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
    let mut next = widget.clone();
    if let semio_framework_artifact_flow_flow::Widget::Neuron { params, .. } = &mut next {
        let merged = params.merge(&patch);
        std::mem::replace(params, merged).retire_cold();
    }
    patch.retire_cold();
    Some(next)
}

/// 🪄️ The sparse delta of one relative gumball leaf: every target that is an operator of `kinds` gets the params
/// `compose` derives from its BASE params, so an edited gesture re-derives the operator on whatever base it replays on.
/// Missing targets and targets of another kind are skipped (`mutation.partial`); none left is `target-missing` (no
/// target exists) or `target-mismatch`; an identity gesture is `mutation.no-op`.
pub(crate) fn generation3d_transform_diff(base: &Generation3dSnapshot, targets: &[String], kinds: &[&str], identity: bool, compose: impl Fn(&semio_framework_value::DslValue) -> Option<Vec<(&'static str, semio_framework_value::DslValue)>>) -> protocol::MutationOutcome<Generation3dDiff> {
    use crate::standards::v1::subsets::any::schema::diff::{diff_snapshot_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
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
    let widgets = WidgetsDiff { removed: Vec::new(), set: composed };
    let diff = diff_snapshot_from_helpers(base, &widgets, &SynapsesDiff::default(), &LayoutDiff::default(), None, None);
    for (_, widget) in widgets.set {
        widget.retire_cold();
    }
    protocol::MutationOutcome::new(diff).absorb_messages(messages)
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
        GenerationMutation::Add { generation } => Generation3dMutation::CreateGeneration(create_generation::CreateGeneration { generation }),
        GenerationMutation::Remove { id } => Generation3dMutation::DeleteGeneration(delete_generation::DeleteGeneration { id }),
        GenerationMutation::Rename { id, name } => Generation3dMutation::RenameGeneration(rename_generation::RenameGeneration { id, new_name: name }),
        GenerationMutation::UpdateValues { id, question_id, value } => Generation3dMutation::ChangeGenerationValue(change_generation_value::ChangeGenerationValue { id, question_id, new_value: value }),
    }
}
//#endregion 🔖️GenerationBridge

//#region 🔖️HostSnapshotDiffing
/// 🔀️ Diffs two fixtures into a minimal, invertible, mergeable semantic mutation set — signature
/// preserved from the pre-migration generic-vocabulary version (`🏗️builder`/app callers reach this
/// via `crate::standards::v1::subsets::any::schema::commit_host_snapshot`, unchanged) but every pushed
/// mutation is now a real semantic variant.
///
/// ⚠️ Order is load-bearing: every orphaned layout override is retired FIRST, while its widget is
/// still present. `delete_widget` leaves the widget's `layout` entry behind, and
/// `delete_widget_position` fail-closes with `mutation.target-missing` once the widget is gone — so
/// authoring the position removals after the widget removals silently kept every stale override, and
/// an example swap accumulated the previous example's layout keys forever
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub fn generation3d_host_snapshot_operations(before: &FlowHostSnapshot, after: &FlowHostSnapshot) -> Vec<Generation3dMutation> {
    let mut operations = Vec::new();
    for id in before.layout.keys() {
        if !after.layout.contains_key(id) {
            operations.push(Generation3dMutation::DeleteWidgetPosition(delete_widget_position::DeleteWidgetPosition { id: id.clone() }));
        }
    }
    let before_widget_ids: Vec<&str> = before.widgets.iter().map(widget_id).collect();
    let after_widget_ids: Vec<&str> = after.widgets.iter().map(widget_id).collect();
    let rebuilt_widgets = reordered_survivors(&before_widget_ids, &after_widget_ids);
    for widget in &before.widgets {
        let id = widget_id(widget);
        if !after_widget_ids.contains(&id) || rebuilt_widgets.contains(id) {
            operations.push(Generation3dMutation::DeleteWidget(delete_widget::DeleteWidget { id: id.to_string() }));
        }
    }
    for (index, widget) in after.widgets.iter().enumerate() {
        let id = widget_id(widget);
        let prior = if rebuilt_widgets.contains(id) { None } else { before.widgets.iter().find(|entry| widget_id(entry) == id) };
        match prior {
            Some(previous) if previous != widget => operations.push(Generation3dMutation::UpdateWidget(update_widget::UpdateWidget { widget: widget.clone() })),
            None => operations.push(Generation3dMutation::CreateWidget(create_widget::CreateWidget { index, widget: widget.clone() })),
            _ => {}
        }
    }
    let before_synapse_ids: Vec<&str> = before.synapses.iter().map(|entry| entry.id.as_str()).collect();
    let after_synapse_ids: Vec<&str> = after.synapses.iter().map(|entry| entry.id.as_str()).collect();
    let rebuilt_synapses = reordered_survivors(&before_synapse_ids, &after_synapse_ids);
    for synapse in &before.synapses {
        if !after_synapse_ids.contains(&synapse.id.as_str()) || rebuilt_synapses.contains(synapse.id.as_str()) {
            operations.push(Generation3dMutation::DisconnectSynapse(disconnect_synapse::DisconnectSynapse { id: synapse.id.clone() }));
        }
    }
    for (index, synapse) in after.synapses.iter().enumerate() {
        let prior = if rebuilt_synapses.contains(synapse.id.as_str()) { None } else { before.synapses.iter().find(|entry| entry.id == synapse.id) };
        match prior {
            Some(previous) if previous != synapse => operations.push(Generation3dMutation::UpdateSynapse(update_synapse::UpdateSynapse { synapse: synapse.clone() })),
            None => operations.push(Generation3dMutation::ConnectSynapse(connect_synapse::ConnectSynapse { index, synapse: synapse.clone() })),
            _ => {}
        }
    }
    for (id, layout) in &after.layout {
        if before.layout.get(id) != Some(layout) {
            operations.push(Generation3dMutation::MoveWidget(move_widget::MoveWidget { id: id.clone(), layout: layout.clone() }));
        }
    }
    if before.schema != after.schema {
        operations.push(Generation3dMutation::ChangeSchema(change_schema::ChangeSchema { new_schema: after.schema.clone() }));
    }
    operations
}

/// 🔢️ Ids of the entries that survive into `after` but must be RE-CREATED to reach its order:
/// every survivor outside a longest run whose target positions already ascend. The mutation
/// vocabulary has no reorder verb — `update-widget`/`update-synapse` replace in place — so a survivor
/// that crossed another survivor is authored as a delete plus a create at its target index. Without
/// this a fixture swap silently kept the PREVIOUS fixture's entry order
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
fn reordered_survivors(before_ids: &[&str], after_ids: &[&str]) -> std::collections::BTreeSet<String> {
    let survivors: Vec<(usize, &str)> = before_ids.iter().filter_map(|id| after_ids.iter().position(|entry| entry == id).map(|index| (index, *id))).collect();
    let mut run = vec![1usize; survivors.len()];
    let mut previous = vec![usize::MAX; survivors.len()];
    let mut longest = usize::MAX;
    for index in 0..survivors.len() {
        for candidate in 0..index {
            if survivors[candidate].0 < survivors[index].0 && run[candidate] + 1 > run[index] {
                run[index] = run[candidate] + 1;
                previous[index] = candidate;
            }
        }
        if longest == usize::MAX || run[index] > run[longest] {
            longest = index;
        }
    }
    let mut kept = std::collections::BTreeSet::new();
    while longest != usize::MAX {
        kept.insert(survivors[longest].1);
        longest = previous[longest];
    }
    survivors.iter().filter(|(_, id)| !kept.contains(id)).map(|(_, id)| (*id).to_string()).collect()
}
//#endregion 🔖️FixtureDiffing

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

//#region 🔖️Apply
/// 🎬️ Fallible in-place `vcs::apply_mutation` boundary. A diff builder that REJECTED the mutation
/// answers `MutationOutcome::{error,fatal}`, whose diff side is forced to `Default` (LAW 1,
/// `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1061-1069`) — applying that empty delta
/// would return the unchanged base as implicit success, exactly what [`protocol::MutationDiff`]'s
/// own contract forbids. The rejection is raised here instead, so a caller's `Ok` is a real witness
/// that the mutation landed. The refusal travels as the outcome's own messages, codes and levels unchanged; an apply-time
/// rejection joins them as the `Fatal` `mutation.apply.*` message `MutationOutcome::apply_to` would persist — a vocabulary
/// code is never re-typed as an apply error.
pub fn apply_generation3d_mutation(projection: &mut Generation3dSnapshot, mutation: &Generation3dMutation) -> Result<(), Vec<protocol::MutationMessage>> {
    let (delta, messages) = protocol::Mutation::diff(mutation, &*projection).into_parts();
    if messages.iter().any(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)) {
        delta.retire_cold();
        return Err(messages);
    }
    let applied = protocol::MutationDiff::apply(&delta, &*projection);
    delta.retire_cold();
    match applied {
        Ok(next) => {
            std::mem::replace(projection, next).retire_cold();
            Ok(())
        }
        Err(error) => Err(messages.into_iter().chain([protocol::MutationMessage::fatal(error.code, error.message).at(error.target)]).collect()),
    }
}

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
