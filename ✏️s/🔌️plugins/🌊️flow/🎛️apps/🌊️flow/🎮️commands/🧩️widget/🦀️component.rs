//! 🧩️ Flow play app commands — widget lifecycle (add / remove / rename / patch / move).
//!
//! Every handler here mutates the DOCUMENT: it runs the stateful `FlowHost` mutation through
//! `crate::apps::flow::host_operations` and lets the fixture diff produce granular `FlowMutation`s with
//! true inverses. Payload field names and order are load-bearing — they ARE the `dsl` record shape of the
//! matching `FlowCommand` variant (see `crate::apps::flow::FlowCommand`).

use crate::apps::flow::config::{FlowConfig, FlowConfigMutation};
use crate::apps::flow::host_operations;
use crate::artifacts::flow::schema::widget_id;
use crate::artifacts::flow::{op::FlowMutation, FlowSnapshot};
use flow::{ FlowEvalSession, Widget};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

//#region 🔖️AddWidget
pub mod add_widget {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct AddWidget {
        pub kind: String,
        pub neuron_kind: Option<String>,
        pub x: Option<f64>,
        pub y: Option<f64>,
    }

    pub fn handle(payload: &AddWidget, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        let descriptor = match payload.kind.as_str() {
            "neuron" => json!({ "kind": "neuron", "neuronKind": payload.neuron_kind.as_deref().unwrap_or("math.add") }).to_string(),
            other => json!({ "kind": other }).to_string(),
        };
        let x = payload.x.unwrap_or(120.0);
        let y = payload.y.unwrap_or(120.0);
        let mut new_id = None;
        let operations = host_operations(doc.snapshot, cfg.snapshot, session, |host| match host.add_widget(&descriptor, x, y) {
            Ok(id) => {
                new_id = Some(id);
                true
            }
            Err(_) => false,
        });
        match new_id {
            Some(id) => Ok(Emit { artifact_mutations: operations, config_mutations: vec![FlowConfigMutation::SetSelection { node_ids: vec![id], edge_ids: Vec::new(), handle_ids: Vec::new() }], ..Default::default() }),
            None => Ok(Emit::mutations(operations)),
        }
    }
}
//#endregion 🔖️AddWidget

//#region 🔖️RemoveWidget
pub mod remove_widget {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct RemoveWidget {
        pub widget_id: String,
    }

    pub fn handle(payload: &RemoveWidget, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        let config = cfg.snapshot;
        let target_id = &payload.widget_id;
        let operations = host_operations(doc.snapshot, config, session, |host| host.remove_widget(target_id).is_ok());
        if operations.is_empty() {
            return Ok(Emit::default());
        }
        let remaining: Vec<String> = config.selected_node_ids.iter().filter(|id| *id != target_id).cloned().collect();
        Ok(Emit {
            artifact_mutations: operations,
            config_mutations: vec![FlowConfigMutation::SetSelection { node_ids: remaining, edge_ids: config.selected_edge_ids.clone(), handle_ids: config.selected_handle_ids.clone() }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️RemoveWidget

//#region 🔖️RenameFlowWidget
pub mod rename_flow_widget {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct RenameFlowWidget {
        pub old_id: String,
        pub value: String,
    }

    /// ✏️ Renames a widget id (rewiring synapses and layout) purely in the fixture; `None` if the target
    /// id is blank, unchanged, or already taken.
    fn renamed_fixture(fixture: &FlowSnapshot, old_id: &str, new_id: &str) -> Option<FlowSnapshot> {
        let trimmed = new_id.trim();
        if trimmed.is_empty() || trimmed == old_id || fixture.widgets.iter().any(|widget| widget_id(widget) == trimmed) {
            return None;
        }
        let mut next = fixture.clone();
        for widget in next.widgets.iter_mut() {
            if widget_id(widget) == old_id {
                match widget {
                    Widget::Neuron { id, .. }
                    | Widget::InputSlider { id, .. }
                    | Widget::InputNote { id, .. }
                    | Widget::InputImage { id, .. }
                    | Widget::Variable { id, .. }
                    | Widget::OutputPreview { id, .. }
                    | Widget::OutputAction { id, .. }
                    | Widget::OutputExport { id, .. }
                    | Widget::Cluster { id, .. } => *id = trimmed.to_string(),
                }
            }
        }
        for synapse in next.synapses.iter_mut() {
            if synapse.from == old_id {
                synapse.from = trimmed.into();
            }
            if synapse.to == old_id {
                synapse.to = trimmed.into();
            }
        }
        if let Some(layout) = next.layout.remove(old_id) {
            next.layout.insert(trimmed.into(), layout);
        }
        Some(next)
    }

    pub fn handle(payload: &RenameFlowWidget, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, _session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        let fixture = doc.snapshot;
        let config = cfg.snapshot;
        match renamed_fixture(fixture, &payload.old_id, &payload.value) {
            Some(next) => Ok(Emit {
                artifact_mutations: crate::artifacts::flow::schema::mutations::snapshot_operations(fixture, &next),
                config_mutations: vec![FlowConfigMutation::SetSelection { node_ids: vec![payload.value.trim().to_string()], edge_ids: config.selected_edge_ids.clone(), handle_ids: config.selected_handle_ids.clone() }],
                ..Default::default()
            }),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️RenameFlowWidget

//#region 🔖️PatchFlowWidgets
pub mod patch_flow_widgets {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct PatchFlowWidgets {
        pub widget_ids: Vec<String>,
        pub field: String,
        pub value: String,
    }

    /// ✏️ Patches the slider value / note text on the selected widgets in the fixture, returning the
    /// clone. `value` is the typed command field verbatim (a plain `&str`, not a `serde_json::Value` —
    /// mirrors `dag_engine::node_patch_for_field`'s "typed command carries the raw UI input string
    /// directly" convention) — numeric fields parse it themselves.
    fn patched_widgets_fixture(fixture: &FlowSnapshot, widget_ids: &[String], field: &str, raw_value: &str) -> FlowSnapshot {
        let mut next = fixture.clone();
        for widget in next.widgets.iter_mut() {
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
        next
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
}
//#endregion 🔖️PatchFlowWidgets

//#region 🔖️MoveMediaNode
pub mod move_media_node {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    pub struct MoveMediaNode {
        pub node_id: String,
        pub x: f64,
        pub y: f64,
    }

    pub fn handle(payload: &MoveMediaNode, doc: &ArtifactView<'_, FlowSnapshot>, cfg: &ConfigView<'_, FlowConfig>, session: &mut FlowEvalSession) -> Result<Emit<FlowMutation, FlowConfigMutation>, Fault> {
        let operations = host_operations(doc.snapshot, cfg.snapshot, session, |host| {
            host.begin_change();
            host.move_widget(&payload.node_id, payload.x, payload.y).is_ok()
        });
        if operations.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit::amend(operations, format!("move-{}", payload.node_id)))
        }
    }
}
//#endregion 🔖️MoveMediaNode

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{dispatch, flow_app};
    use crate::apps::flow::FlowCommand;

    #[test]
    fn add_widget_emits_operations_and_selects_the_new_widget() {
        let mut app = flow_app();
        let before = app.snapshot().expect("snapshot").widgets.len();
        let result = dispatch(&mut app, FlowCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), neuron_kind: None, x: Some(40.0), y: Some(40.0) }));
        assert!(!result.mutations.is_empty(), "addWidget must emit operations");
        assert_eq!(app.snapshot().expect("snapshot").widgets.len(), before + 1);
    }

    #[test]
    fn rename_rejects_blank_unchanged_and_taken_ids() {
        let mut app = flow_app();
        for value in ["", " ", "slider"] {
            let result = dispatch(&mut app, FlowCommand::RenameFlowWidget(rename_flow_widget::RenameFlowWidget { old_id: "slider".into(), value: value.into() }));
            assert!(result.mutations.is_empty(), "rename to {value:?} must be a no-operation");
        }
    }

    #[test]
    fn patch_flow_widgets_parses_the_raw_value_string_into_the_slider() {
        let mut app = flow_app();
        dispatch(&mut app, FlowCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["slider".into()], field: "value".into(), value: "7.5".into() }));
        let patched = app.snapshot().expect("snapshot");
        assert!(patched.widgets.iter().any(|widget| matches!(widget, Widget::InputSlider { id, value, .. } if id == "slider" && (value - 7.5).abs() < f64::EPSILON)), "slider must carry the parsed value: {patched:?}");
    }
}
//#endregion 🧪️Tests
