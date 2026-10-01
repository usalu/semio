//! ↩️ `change-slider-value` inverse — the BASE slider widget restored whole (value AND the range a widening value
//! moved), an absolute row; nothing when the slider does not exist on the base.

use crate::standards::v1::subsets::any::schema::mutations::change_slider_value::ChangeSliderValue;
use crate::standards::v1::subsets::any::schema::mutations::update_widget::UpdateWidget;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::Widget;

/// ↩️ The base widget, when it is a slider.
pub fn inverse(payload: &ChangeSliderValue, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    base.host_snapshot
        .widgets
        .iter()
        .find(|widget| matches!(widget, Widget::InputSlider { id, .. } if *id == payload.id))
        .map(|widget| Generation3dMutation::UpdateWidget(UpdateWidget { widget: widget.clone() }))
        .into_iter()
        .collect()
}
