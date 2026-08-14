//! 🪟️ 🧩️ Flow play app commands command — `add-widget`.

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::apps::flow::host_operations;
use crate::artifacts::flow::schema::widget_id;
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::{ FlowEvalSession, Widget};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
pub struct AddWidget {
    pub kind: String,
    pub neuron_kind: Option<String>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}

/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the new widget used to also become the
/// selection here — selection is framework-owned `InteractionState` now, only ever mutated by the
/// framework's own injected `interactionSelect` handling, never by an app command's `Emit` (mirrors
/// note's `add-block`).
pub fn handle(payload: &AddWidget, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
    let descriptor = match payload.kind.as_str() {
        "neuron" => json!({ "kind": "neuron", "neuronKind": payload.neuron_kind.as_deref().unwrap_or("math.add") }).to_string(),
        other => json!({ "kind": other }).to_string(),
    };
    let x = payload.x.unwrap_or(120.0);
    let y = payload.y.unwrap_or(120.0);
    let operations = host_operations(doc.snapshot, cfg.snapshot, session, |host| host.add_widget(&descriptor, x, y).is_ok());
    Ok(Emit::mutations(operations))
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{dispatch, flow_app};
    use crate::apps::flow::FlowCommand;

    #[test]
    fn add_widget_emits_operations_and_grows_the_widget_count() {
        let mut app = flow_app();
        let before = app.snapshot().expect("snapshot").to_fixture().widgets.len();
        let result = dispatch(&mut app, FlowCommand::AddWidget(AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(40.0), y: Some(40.0) }));
        assert!(!result.mutations.is_empty(), "addWidget must emit operations");
        assert_eq!(app.snapshot().expect("snapshot").to_fixture().widgets.len(), before + 1);
    }

    #[test]
    fn rename_rejects_blank_unchanged_and_taken_ids() {
        let mut app = flow_app();
        for value in ["", " ", "slider"] {
            let result = dispatch(&mut app, FlowCommand::RenameFlowWidget(crate::apps::flow::commands::rename_flow_widget::RenameFlowWidget { old_id: "slider".into(), value: value.into() }));
            assert!(result.mutations.is_empty(), "rename to {value:?} must be a no-operation");
        }
    }

    #[test]
    fn patch_flow_widgets_parses_the_raw_value_string_into_the_slider() {
        let mut app = flow_app();
        dispatch(&mut app, FlowCommand::PatchFlowWidgets(crate::apps::flow::commands::patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["slider".into()], field: "value".into(), value: "7.5".into() }));
        let patched = app.snapshot().expect("snapshot");
        let patched_widgets = patched.to_fixture().widgets;
        assert!(patched_widgets.iter().any(|widget| matches!(widget, Widget::InputSlider { id, value, .. } if id == "slider" && (value - 7.5).abs() < f64::EPSILON)), "slider must carry the parsed value: {patched_widgets:?}");
    }
}
//#endregion 🧪️Tests
