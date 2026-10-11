//! 🧩️ Generation3d command — `patch-flow-widgets`: one numeric field (a slider's `value`) set on several widgets at once.
//! The inspector's number field is a continuous control: it rides the framework scrub machine (design §13.1), so every tick
//! re-derives the ABSOLUTE `change-slider-value` leaves of the value against the committed document and the release
//! commits them as ONE edit. The handler reads the value only, never a gesture.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::change_slider_value::change_slider_value;
use crate::standards::v1::subsets::any::schema::mutations::Generation3dMutation;
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::{FlowHostSnapshot, Widget};
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "patch-flow-widgets")]
pub struct PatchFlowWidgets {
    pub widget_ids: Vec<String>,
    pub field: String,
    pub value: Option<f64>,
}

/// 🎚️ The absolute leaves of `payload` on `host_snapshot`: one `change-slider-value` per addressed slider whose value moves.
pub(crate) fn patch_leaves(host_snapshot: &FlowHostSnapshot, payload: &PatchFlowWidgets) -> Vec<Generation3dMutation> {
    let Some(value) = payload.value.filter(|value| payload.field == "value" && value.is_finite()) else { return Vec::new() };
    host_snapshot
        .widgets
        .iter()
        .filter_map(|widget| match widget {
            Widget::InputSlider { id, value: current, .. } if payload.widget_ids.contains(id) && *current != value => Some(change_slider_value(id.clone(), value)),
            _ => None,
        })
        .collect()
}

pub fn handle(payload: &PatchFlowWidgets, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    Ok(Emit { artifact_mutations: patch_leaves(&doc.snapshot.host_snapshot, payload), ui_scope: crate::editor::generation3d::commands::node_graph_edit::slider_gesture_ui_scope(), ..Default::default() })
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
