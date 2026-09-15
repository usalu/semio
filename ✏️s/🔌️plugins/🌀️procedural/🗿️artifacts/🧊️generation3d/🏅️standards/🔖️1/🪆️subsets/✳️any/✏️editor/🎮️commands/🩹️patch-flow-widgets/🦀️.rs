//! 🧩️ 🧩️ Generation3d play app commands command — `patch-flow-widgets`.

use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::schema::with_host;
use crate::standards::v1::subsets::any::schema::mutations::text::{generation3d_host_snapshot_operations, Generation3dMutation};
use crate::Generation3dSnapshot;
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-flow-widgets")]
pub struct PatchFlowWidgets {
    pub widget_ids: Vec<String>,
    pub field: String,
    pub value: Option<f64>,
    /// 🎚️ The press this value belongs to, when it came from a CONTINUOUS control (a dragged slider,
    /// a held spinner). Every value of one press folds into ONE undoable edit under this identity;
    /// absent means a discrete edit of its own. Minted by the renderer's continuous-gesture lane
    /// (`🗣️Interpreter/🟦️.tsx`'s `useContinuousTriggerLane`) — a whole one-second stream into the
    /// Inspection panel's number field cost 29 history entries without it
    /// (`📓️slider-preview-update-2026-09-15.md`).
    pub gesture: Option<String>,
}

/// 🎚️ The key one press's values fold under, or `None` for a discrete edit. Pure, so the law can
/// read the rule without building an `ArtifactView` (plugin code cannot construct one).
pub(crate) fn patch_coalesce_key(gesture: Option<&str>) -> Option<String> {
    gesture.filter(|key| !key.is_empty()).map(|key| format!("widget-field:{key}"))
}

pub fn handle(payload: &PatchFlowWidgets, doc: &ArtifactView<'_, Generation3dSnapshot>, _cfg: &ConfigView<'_, Generation3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let host_snapshot = &doc.snapshot.host_snapshot;
    let artifact_mutations = with_host(host_snapshot, |host| {
        let baseline = host.host_snapshot.clone();
        for widget in host.host_snapshot.widgets.iter_mut() {
            if !payload.widget_ids.contains(&crate::widget_id(widget).to_string()) {
                continue;
            }
            if let (Widget::InputSlider { value: slider_value, .. }, Some(new_value)) = (widget, payload.value) {
                if payload.field == "value" {
                    *slider_value = new_value;
                }
            }
        }
        let operations = generation3d_host_snapshot_operations(&baseline, &host.host_snapshot);
        baseline.retire_cold();
        operations
    });
    let coalesce_key = patch_coalesce_key(payload.gesture.as_deref());
    let ui_scope = if coalesce_key.is_some() { crate::editor::generation3d::commands::node_graph_edit::slider_gesture_ui_scope() } else { semio_framework::kernel::UiDirtyScope::default() };
    Ok(Emit { artifact_mutations, coalesce_key, ui_scope, ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
