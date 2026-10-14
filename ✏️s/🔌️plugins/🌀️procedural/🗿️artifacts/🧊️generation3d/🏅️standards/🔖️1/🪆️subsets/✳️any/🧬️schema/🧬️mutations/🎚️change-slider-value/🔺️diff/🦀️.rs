//! 🔺️ `change-slider-value` sparse diff — replaces the ONE addressed slider widget with its value set (and, for a
//! value outside its range, its range widened exactly like the canvas knob).

use semio_s_artifact_procedural_3d_model::diff::Generation3dDiff;
use semio_s_artifact_procedural_3d_model::diff::Generation3dWidgetPatch;
use semio_s_artifact_procedural_3d_model::diff::Generation3dWidgetModification;
use semio_s_artifact_procedural_3d_model::diff::Generation3dWidgetsDelta;
use crate::change_slider_value::ChangeSliderValue;
use semio_s_artifact_procedural_3d_model::widget_index;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::Widget;

/// 🏗️ A missing slider is `target-missing`, another widget kind or a range that cannot hold the value is
/// `target-mismatch`, the value it already holds is `no-op`.
pub fn diff(payload: &ChangeSliderValue, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    if !payload.value.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Slider \"{}\" value is not finite.", payload.id), [payload.id.clone()]);
    }
    let Some(index) = widget_index(&base.host_snapshot, &payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Slider \"{}\" does not exist.", payload.id), [payload.id.clone()]);
    };
    let Widget::InputSlider { value, .. } = &base.host_snapshot.widgets[index] else {
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("Widget \"{}\" is not an input slider.", payload.id), [payload.id.clone()]);
    };
    if *value == payload.value {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", format!("Slider \"{}\" already holds {}.", payload.id, payload.value)).at([payload.id.clone()])]);
    }
    let Widget::InputSlider { value, min, max, step, .. } = &base.host_snapshot.widgets[index] else {
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("Widget \"{}\" is not an input slider.", payload.id), [payload.id.clone()]);
    };
    let Some((value, min, max, step)) = generation3d_slider_landing(*value, *min, *max, *step, payload.value) else {
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("Slider \"{}\" has no range that holds {}.", payload.id, payload.value), [payload.id.clone()]);
    };
    let patch = Generation3dWidgetPatch::Slider { value, min, max, step };
    protocol::MutationOutcome::new(Generation3dDiff { widgets: Some(Generation3dWidgetsDelta { modified: vec![Generation3dWidgetModification { id: payload.id.clone(), patch }], ..Default::default() }), ..Default::default() })
}

/// 🎚️ The slider fields after `target` lands on it (the range widened exactly like the canvas knob), computed on a probe copy; `None` when no range can hold the value.
pub fn generation3d_slider_landing(value: f64, min: f64, max: f64, step: f64, target: f64) -> Option<(f64, f64, f64, f64)> {
    let mut landing = semio_framework_artifact_flow_flow::Widget::InputSlider { id: String::new(), label: String::new(), value, min, max, step };
    if !semio_framework_artifact_flow_flow::set_widget_slider_value(&mut landing, target) {
        return None;
    }
    match landing {
        semio_framework_artifact_flow_flow::Widget::InputSlider { value, min, max, step, .. } => Some((value, min, max, step)),
        _ => None,
    }
}
