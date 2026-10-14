//! 🧭️ Generation3d transform mutation leaves: drag, rotate and scale transforms. Every leaf is generic over the dispatch enum.
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_s_artifact_procedural_3d_model::{widget_id, widget_index};
use semio_s_artifact_procedural_3d_mut_widgets::{generation3d_partial, generation3d_targets_invariant, generation3d_with_params};

//#region 🔖️TransformGestures
/// 🎛️ The neuron kinds one gumball composition addresses: the B-Rep transform, the mesh transform and the mesh
/// component transform of the same operation.
pub const GENERATION3D_TRANSLATE_KINDS: [&str; 3] = ["brep.xform.translate", "brep.mesh.translate", "brep.mesh.translateComponents"];
pub const GENERATION3D_ROTATE_KINDS: [&str; 3] = ["brep.xform.rotate", "brep.mesh.rotate", "brep.mesh.rotateComponents"];
pub const GENERATION3D_SCALE_KINDS: [&str; 3] = ["brep.xform.scale", "brep.mesh.scale", "brep.mesh.scaleComponents"];
/// 🧮️ Borrows one native vector parameter, retaining the original per-axis fallback behavior.
pub fn generation3d_param_vector(params: &semio_framework_artifact_flow_flow::neural::Dictionary, key: &str, fallback: [f64; 3]) -> [f64; 3] {
    let vector = params.get(key).and_then(semio_framework_artifact_flow_flow::neural::Value::as_dictionary);
    let mut axes = fallback;
    for (axis, name) in ["x", "y", "z"].iter().enumerate() {
        if let Some(value) = vector.and_then(|vector| vector.get(name)).and_then(semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::neural_number) { axes[axis] = value; }
    }
    axes
}

/// 🔣️ Borrows one native numeric parameter, or the original fallback when absent.
pub fn generation3d_param_number(params: &semio_framework_artifact_flow_flow::neural::Dictionary, key: &str, fallback: f64) -> f64 {
    params.get(key).and_then(semio_framework_artifact_flow_flow::neural::Value::as_dictionary).and_then(|entry| entry.get("value")).and_then(semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::neural_number).unwrap_or(fallback)
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
    semio_s_artifact_procedural_3d_mut_widgets::change_widget_input::WidgetInputValue::Number(value).neural_value()
}

/// 🪄️ The sparse delta of one relative gumball leaf: every target that is an operator of `kinds` gets the params
/// `compose` derives from its BASE params, so an edited gesture re-derives the operator on whatever base it replays on.
/// Missing targets and targets of another kind are skipped (`mutation.partial`); none left is `target-missing` (no
/// target exists) or `target-mismatch`; an identity gesture is `mutation.no-op`.
pub fn generation3d_transform_diff(base: &Generation3dSnapshot, targets: &[String], kinds: &[&str], identity: bool, compose: impl Fn(&semio_framework_artifact_flow_flow::neural::Dictionary) -> Option<Vec<(&'static str, semio_framework_artifact_flow_flow::neural::Value)>>) -> protocol::MutationOutcome<Generation3dDiff> {
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
pub fn generation3d_transform_inverse<Op: From<semio_s_artifact_procedural_3d_mut_widgets::update_widget::UpdateWidget>>(base: &Generation3dSnapshot, targets: &[String], kinds: &[&str]) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    targets
        .iter()
        .filter_map(|id| base.host_snapshot.widgets.iter().find(|widget| widget_id(widget) == id))
        .filter(|widget| matches!(widget, semio_framework_artifact_flow_flow::Widget::Neuron { neuron_kind, .. } if kinds.contains(&neuron_kind.as_str())))
        .map(|widget| Op::from(semio_s_artifact_procedural_3d_mut_widgets::update_widget::UpdateWidget { widget: widget.clone() }))
        .collect()

    })())
}
//#endregion 🔖️TransformGestures

#[path = "."]
pub mod drag_transforms {
    #[path = "../✋️drag-transforms/🦀️.rs"]
    mod component;
    #[path = "../✋️drag-transforms/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../✋️drag-transforms/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod rotate_transforms {
    #[path = "../🔃️rotate-transforms/🦀️.rs"]
    mod component;
    #[path = "../🔃️rotate-transforms/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔃️rotate-transforms/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
#[path = "."]
pub mod scale_transforms {
    #[path = "../📏️scale-transforms/🦀️.rs"]
    mod component;
    #[path = "../📏️scale-transforms/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../📏️scale-transforms/↩️inverse/🦀️.rs"]
    pub mod inverse;
    pub use component::*;
}
