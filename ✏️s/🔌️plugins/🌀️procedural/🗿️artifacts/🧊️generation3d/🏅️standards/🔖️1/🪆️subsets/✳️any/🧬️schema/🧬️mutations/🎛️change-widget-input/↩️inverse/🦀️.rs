//! ↩️ `change-widget-input` inverse — the BASE operator (or text source) restored whole, an absolute row; nothing when
//! the widget does not exist on the base.

use crate::standards::v1::subsets::any::schema::mutations::change_widget_input::ChangeWidgetInput;
use crate::standards::v1::subsets::any::schema::mutations::update_widget::UpdateWidget;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::Widget;

/// ↩️ The base widget, when it is an operator or a text source.
pub fn inverse(payload: &ChangeWidgetInput, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
    base.host_snapshot
        .widgets
        .iter()
        .find(|widget| matches!(widget, Widget::Neuron { id, .. } | Widget::InputNote { id, .. } if *id == payload.id))
        .map(|widget| Generation3dMutation::UpdateWidget(UpdateWidget { widget: widget.clone() }))
        .into_iter()
        .collect()
}
