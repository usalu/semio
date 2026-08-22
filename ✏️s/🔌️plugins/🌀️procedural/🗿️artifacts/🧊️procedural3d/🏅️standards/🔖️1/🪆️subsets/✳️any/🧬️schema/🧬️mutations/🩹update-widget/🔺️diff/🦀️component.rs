//! 🔺️ `update-widget` sparse diff construction.

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::diff::{diff_fixture_from_helpers, LayoutDiff, SynapsesDiff, WidgetsDiff};
use crate::artifacts::procedural3d::mutations::update_widget::mutation::UpdateWidget;
use crate::artifacts::procedural3d::mutations::widget_index;
use crate::artifacts::procedural3d::{widget_id, Procedural3dSnapshot};
use flow::Widget;

/// 🏗️ Builds the sparse fixture delta replacing one existing widget's body. The index is
/// irrelevant here — `apply_widgets_diff` resolves an existing entry by id before ever consulting
/// the index, which only matters for a genuinely new (`create-widget`) insertion.
pub fn diff(payload: &UpdateWidget, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
    let id = widget_id(&payload.widget);
    let Some(index) = widget_index(&base.fixture, id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{id}\" does not exist."), [id.to_string()]);
    };
    if let Widget::InputSlider { value, min, max, step, .. } = &payload.widget {
        if !value.is_finite() || !min.is_finite() || !max.is_finite() || !step.is_finite() || min > max {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Slider \"{id}\" has a non-finite or inverted value/min/max/step."), [id.to_string()]);
        }
    }
    if base.fixture.widgets[index] == payload.widget {
        return protocol::MutationOutcome::new(Procedural3dDiff::default()).warn("mutation.no-op", format!("Widget \"{id}\" is already in the requested state."));
    }
    protocol::MutationOutcome::new(diff_fixture_from_helpers(base, WidgetsDiff { removed: vec![], set: vec![(0, payload.widget.clone())] }, SynapsesDiff::default(), LayoutDiff::default(), None, None))
}
