//! ↩️ `change-slider-value` inverse — the BASE slider widget restored whole (value AND the range a widening value
//! moved), an absolute row; nothing when the slider does not exist on the base.

use crate::change_slider_value::ChangeSliderValue;
use crate::update_widget::UpdateWidget;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::Widget;

/// ↩️ The base widget, when it is a slider.
pub fn inverse<Op: From<crate::update_widget::UpdateWidget>>(payload: &ChangeSliderValue, base: &Generation3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    base.host_snapshot
        .widgets
        .iter()
        .find(|widget| matches!(widget, Widget::InputSlider { id, .. } if *id == payload.id))
        .map(|widget| Op::from(UpdateWidget { widget: widget.clone() }))
        .into_iter()
        .collect()

    })())
}
