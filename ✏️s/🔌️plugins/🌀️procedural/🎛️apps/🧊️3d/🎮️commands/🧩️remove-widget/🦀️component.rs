//! 🧩️ 🧩️ Procedural3d play app commands command — `remove-widget`.

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::artifacts::procedural3d::schema::{commit_fixture, host_from_fixture};
use crate::artifacts::procedural3d::op::{procedural3d_fixture_operations, Procedural3dMutation};
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::{FlowEvalSession, Widget};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-widget")]
pub struct RemoveWidget {
    pub widget_id: String}

pub fn handle(payload: &RemoveWidget, doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let target_id = &payload.widget_id;
    let mut host = host_from_fixture(fixture);
    if host.remove_widget(target_id).is_ok() {
        let operations = commit_fixture(fixture, &host.fixture);
        let mut remaining = cfg.snapshot.selected_node_ids.clone();
        remaining.retain(|id| id != target_id);
        Ok(Emit { artifact_mutations: operations, config_mutations: vec![Procedural3dConfigMutation::SetSelection { node_ids: remaining }], ..Default::default() })
    } else {
        Ok(Emit::default())
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app, dispatch};
    use crate::apps::procedural3d::Procedural3dCommand;

    #[test]
    fn add_widget_action_appends_widget() {
        let _serial = crate::apps::procedural3d::test_support::lock();
        let mut app = app();
        let before = app.snapshot().expect("snapshot").fixture.widgets.len();
        dispatch(&mut app, Procedural3dCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), x: None, y: None }));
        assert!(app.snapshot().expect("snapshot").fixture.widgets.len() > before);
    }

    #[test]
    fn patch_flow_widgets_edits_slider_value() {
        let _serial = crate::apps::procedural3d::test_support::lock();
        let mut app = app();
        dispatch(&mut app, Procedural3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5) }));
        let value = app.snapshot().expect("snapshot").fixture.widgets.iter().find_map(|widget| match widget {
            Widget::InputSlider { id, value, .. } if id == "height" => Some(*value),
            _ => None});
        assert_eq!(value, Some(9.5));
    }

    #[test]
    fn remove_widget_action_deletes_by_id() {
        let _serial = crate::apps::procedural3d::test_support::lock();
        let mut app = app();
        assert!(app.snapshot().expect("snapshot").fixture.widgets.iter().any(|widget| crate::artifacts::procedural3d::widget_id(widget) == "sides"));
        dispatch(&mut app, Procedural3dCommand::RemoveWidget(RemoveWidget { widget_id: "sides".into() }));
        assert!(!app.snapshot().expect("snapshot").fixture.widgets.iter().any(|widget| crate::artifacts::procedural3d::widget_id(widget) == "sides"));
    }
}
//#endregion 🧪️Tests
