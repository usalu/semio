//! 🧩️ Procedural3d play app commands — widget add/remove/patch and selection delete.

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigOperation};
use crate::artifacts::procedural3d::engine::{commit_fixture, host_from_fixture};
use crate::artifacts::procedural3d::op::{procedural3d_fixture_operations, Procedural3dOperation};
use crate::artifacts::procedural3d::Procedural3dDocument;
use flow_core::{FlowEvalSession, Widget};
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

//#region 🔖️DeleteSelection
pub mod delete_selection {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "delete-selection")]
    pub struct DeleteSelection {}

    pub fn handle(_payload: &DeleteSelection, doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        let fixture = &doc.projection.fixture;
        let selected = cfg.projection.selected_node_ids.clone();
        let mut host = host_from_fixture(fixture);
        let mut cleared = false;
        for id in &selected {
            if host.remove_widget(id).is_ok() {
                cleared = true;
            }
        }
        let operations = commit_fixture(fixture, &host.fixture);
        let config_operations = if cleared { vec![Procedural3dConfigOperation::SetSelection { node_ids: Vec::new() }] } else { Vec::new() };
        Ok(Emit { document_operations: operations, config_operations, ..Default::default() })
    }
}
//#endregion 🔖️DeleteSelection

//#region 🔖️RemoveWidget
pub mod remove_widget {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "remove-widget")]
    pub struct RemoveWidget {
        pub widget_id: String,
    }

    pub fn handle(payload: &RemoveWidget, doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        let fixture = &doc.projection.fixture;
        let target_id = &payload.widget_id;
        let mut host = host_from_fixture(fixture);
        if host.remove_widget(target_id).is_ok() {
            let operations = commit_fixture(fixture, &host.fixture);
            let mut remaining = cfg.projection.selected_node_ids.clone();
            remaining.retain(|id| id != target_id);
            Ok(Emit { document_operations: operations, config_operations: vec![Procedural3dConfigOperation::SetSelection { node_ids: remaining }], ..Default::default() })
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️RemoveWidget

//#region 🔖️AddWidget
pub mod add_widget {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-widget")]
    pub struct AddWidget {
        pub kind: String,
        pub x: Option<f64>,
        pub y: Option<f64>,
    }

    pub fn handle(payload: &AddWidget, doc: &DocumentView<'_, Procedural3dDocument>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        let fixture = &doc.projection.fixture;
        let descriptor = if let Some((base, neuron)) = payload.kind.split_once('|') {
            if base == "neuron" {
                json!({ "kind": "neuron", "neuronKind": neuron }).to_string()
            } else {
                json!({ "kind": payload.kind }).to_string()
            }
        } else {
            json!({ "kind": payload.kind }).to_string()
        };
        let x = payload.x.unwrap_or(120.0);
        let y = payload.y.unwrap_or(120.0);
        let mut host = host_from_fixture(fixture);
        if let Ok(id) = host.add_widget(&descriptor, x, y) {
            let operations = commit_fixture(fixture, &host.fixture);
            Ok(Emit { document_operations: operations, config_operations: vec![Procedural3dConfigOperation::SetSelection { node_ids: vec![id] }], ..Default::default() })
        } else {
            Ok(Emit::default())
        }
    }
}
//#endregion 🔖️AddWidget

//#region 🔖️PatchFlowWidgets
pub mod patch_flow_widgets {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-flow-widgets")]
    pub struct PatchFlowWidgets {
        pub widget_ids: Vec<String>,
        pub field: String,
        pub value: Option<f64>,
    }

    pub fn handle(payload: &PatchFlowWidgets, doc: &DocumentView<'_, Procedural3dDocument>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        let fixture = &doc.projection.fixture;
        let mut host = host_from_fixture(fixture);
        let baseline = host.fixture.clone();
        for widget in host.fixture.widgets.iter_mut() {
            if !payload.widget_ids.contains(&crate::artifacts::procedural3d::widget_id(widget).to_string()) {
                continue;
            }
            if let (Widget::InputSlider { value: slider_value, .. }, Some(new_value)) = (widget, payload.value) {
                if payload.field == "value" {
                    *slider_value = new_value;
                }
            }
        }
        Ok(Emit::operations(procedural3d_fixture_operations(&baseline, &host.fixture)))
    }
}
//#endregion 🔖️PatchFlowWidgets

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app, dispatch};
    use crate::apps::procedural3d::Procedural3dCommand;

    #[test]
    fn add_widget_action_appends_widget() {
        let mut app = app();
        let before = app.projection().expect("projection").fixture.widgets.len();
        dispatch(&mut app, Procedural3dCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), x: None, y: None }));
        assert!(app.projection().expect("projection").fixture.widgets.len() > before);
    }

    #[test]
    fn patch_flow_widgets_edits_slider_value() {
        let mut app = app();
        dispatch(&mut app, Procedural3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5) }));
        let value = app.projection().expect("projection").fixture.widgets.iter().find_map(|widget| match widget {
            Widget::InputSlider { id, value, .. } if id == "height" => Some(*value),
            _ => None,
        });
        assert_eq!(value, Some(9.5));
    }

    #[test]
    fn remove_widget_action_deletes_by_id() {
        let mut app = app();
        assert!(app.projection().expect("projection").fixture.widgets.iter().any(|widget| crate::artifacts::procedural3d::widget_id(widget) == "sides"));
        dispatch(&mut app, Procedural3dCommand::RemoveWidget(remove_widget::RemoveWidget { widget_id: "sides".into() }));
        assert!(!app.projection().expect("projection").fixture.widgets.iter().any(|widget| crate::artifacts::procedural3d::widget_id(widget) == "sides"));
    }
}
//#endregion 🧪️Tests
