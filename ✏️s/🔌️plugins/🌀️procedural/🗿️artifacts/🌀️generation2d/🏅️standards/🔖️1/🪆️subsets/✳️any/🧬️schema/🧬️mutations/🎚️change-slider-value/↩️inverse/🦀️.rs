//! ↩️ Inverse for `ChangeSliderValue` — the BASE slider widget restored whole (value AND the range a widening value moved),
//! an absolute row; nothing when the slider does not exist on the base.

use crate::standards::v1::subsets::any::schema::mutations::{replace_widget,Generation2dMutation};

use crate::Generation2dSnapshot;
use semio_framework_artifact_flow_flow::Widget;

pub fn inverse(payload: &super::ChangeSliderValue, base: &Generation2dSnapshot) -> Result<Vec<Generation2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    base.host_snapshot.widgets.iter().find(|widget| matches!(widget, Widget::InputSlider { id, .. } if *id == payload.id)).map(|widget| replace_widget(widget.clone())).into_iter().collect()

    })())
}
