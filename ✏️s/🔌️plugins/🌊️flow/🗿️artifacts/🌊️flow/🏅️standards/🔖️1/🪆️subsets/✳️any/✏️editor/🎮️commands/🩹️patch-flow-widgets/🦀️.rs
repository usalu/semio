//! 🪟️ 🧩️ Flow play app commands command — `patch-flow-widgets`.

use crate::artifacts::flow::schema::widget_id;
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use crate::editor::flow::config::{FlowConfig, FlowConfigMutation};
use flow::{FlowEvalSession, Widget};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
pub struct PatchFlowWidgets {
    pub widget_ids: Vec<String>,
    pub field: String,
    pub value: String,
}

/// ✏️ Patches the slider value / note text on the selected widgets in the fixture, returning the
/// clone. `value` is the typed command field verbatim (a plain `&str`, not a `serde_json::Value` —
/// mirrors `dag_engine::node_patch_for_field`'s "typed command carries the raw UI input string
/// directly" convention) — numeric fields parse it themselves.
fn patched_widgets_fixture(snapshot: &FlowSnapshot, widget_ids: &[String], field: &str, raw_value: &str) -> FlowSnapshot {
    let mut fixture = snapshot.to_fixture();
    for widget in fixture.widgets.iter_mut() {
        if !widget_ids.iter().any(|id| id == widget_id(widget)) {
            continue;
        }
        match (field, widget) {
            ("value", Widget::InputSlider { value, .. }) => {
                if let Ok(parsed) = raw_value.parse::<f64>() {
                    *value = parsed;
                }
            }
            ("text", Widget::InputNote { text, .. }) => *text = raw_value.into(),
            _ => {}
        }
    }
    FlowSnapshot::from_fixture(fixture)
}

pub fn handle(payload: &PatchFlowWidgets, doc: &ArtifactView<'_, FlowSnapshot>, _cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let fixture = doc.snapshot;
    let next = patched_widgets_fixture(fixture, &payload.widget_ids, &payload.field, &payload.value);
    let operations = crate::artifacts::flow::schema::mutations::snapshot_operations(fixture, &next);
    if operations.is_empty() {
        Ok(Emit::default())
    } else {
        Ok(Emit::amend(operations, format!("patch-{}-{}", payload.field, payload.widget_ids.join(","))))
    }
}
