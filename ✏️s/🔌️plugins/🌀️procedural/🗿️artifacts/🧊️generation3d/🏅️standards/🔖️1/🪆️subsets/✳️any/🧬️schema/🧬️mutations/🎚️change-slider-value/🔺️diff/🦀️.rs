//! 🔺️ `change-slider-value` sparse diff — replaces the ONE addressed slider widget with its value set (and, for a
//! value outside its range, its range widened exactly like the canvas knob).

use crate::standards::v1::subsets::any::schema::diff::{diff_snapshot_from_helpers, Generation3dDiff, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::standards::v1::subsets::any::schema::mutations::change_slider_value::ChangeSliderValue;
use crate::standards::v1::subsets::any::schema::mutations::widget_index;
use crate::Generation3dSnapshot;
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
    let mut widget = base.host_snapshot.widgets[index].clone();
    if !semio_framework_artifact_flow_flow::set_widget_slider_value(&mut widget, payload.value) {
        return protocol::MutationOutcome::error("mutation.target-mismatch", format!("Slider \"{}\" has no range that holds {}.", payload.id, payload.value), [payload.id.clone()]);
    }
    protocol::MutationOutcome::new(diff_snapshot_from_helpers(base, &WidgetsDiff { removed: Vec::new(), set: vec![(index, widget)] }, &SynapsesDiff::default(), &LayoutDiff::default(), None, None))
}
