//! 🔺️ `update-widget` sparse diff construction.

use crate::standards::v1::subsets::any::schema::diff::Generation3dDiff;
use crate::standards::v1::subsets::any::schema::diff::{Generation3dSynapseModification, Generation3dSynapsesDelta, Generation3dWidgetPatch, Generation3dWidgetModification, Generation3dWidgetsDelta, Generation3dSynapsePatch};
use crate::standards::v1::subsets::any::schema::mutations::update_widget::UpdateWidget;
use crate::standards::v1::subsets::any::schema::mutations::widget_index;
use crate::{widget_id, Generation3dSnapshot};
use semio_framework_artifact_flow_flow::Widget;

/// 🏗️ Builds the sparse fixture delta replacing one existing widget's body. The index is
/// irrelevant here — the widgets delta resolves an existing entry by id before ever consulting
/// the index, which only matters for a genuinely new (`create-widget`) insertion.
pub fn diff(payload: &UpdateWidget, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
    let id = widget_id(&payload.widget);
    let Some(index) = widget_index(&base.host_snapshot, id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Widget \"{id}\" does not exist."), [id.to_string()]);
    };
    if let Widget::InputSlider { value, min, max, step, .. } = &payload.widget {
        if !value.is_finite() || !min.is_finite() || !max.is_finite() || !step.is_finite() || min > max {
            return protocol::MutationOutcome::fatal("mutation.invariant", format!("Slider \"{id}\" has a non-finite or inverted value/min/max/step."), [id.to_string()]);
        }
    }
    if base.host_snapshot.widgets[index] == payload.widget {
        return protocol::MutationOutcome::new(Generation3dDiff::default()).warning("mutation.no-op", format!("Widget \"{id}\" is already in the requested state."));
    }
    let synapses = Generation3dSynapsesDelta {
        modified: crate::standards::v1::subsets::any::schema::mutations::update_widget::variable_synapses(&base.host_snapshot.widgets[index], &payload.widget, &base.host_snapshot.synapses).into_iter().map(|(_, synapse)| Generation3dSynapseModification { id: synapse.id.clone(), patch: Generation3dSynapsePatch(synapse) }).collect(),
        ..Default::default()
    };
    protocol::MutationOutcome::new(Generation3dDiff { widgets: Some(Generation3dWidgetsDelta { modified: vec![Generation3dWidgetModification { id: id.to_string(), patch: Generation3dWidgetPatch::Replace { widget: payload.widget.clone() } }], ..Default::default() }), synapses: Some(synapses).filter(|delta| !delta.modified.is_empty()), ..Default::default() })
}
