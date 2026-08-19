//! 🧩️ 🧩️ Procedural3d play app commands command — `remove-widget`.

use crate::editor::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::artifacts::procedural3d::schema::{commit_fixture, host_from_fixture};
use crate::artifacts::procedural3d::op::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::FlowEvalSession;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "remove-widget")]
pub struct RemoveWidget {
    pub widget_id: String}

/// 🕹️ No longer prunes selection itself — the framework auto-prunes `graph`'s selection after any
/// document mutation that deletes a selected id (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub async fn handle(payload: &RemoveWidget, doc: &ArtifactView<'_, Procedural3dSnapshot>, _cfg: &ConfigView<'_, Procedural3dConfig>, _session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    let fixture = &doc.snapshot.fixture;
    let target_id = &payload.widget_id;
    let mut host = host_from_fixture(fixture);
    if host.remove_widget(target_id).is_ok() {
        let operations = commit_fixture(fixture, &host.fixture);
        Ok(Emit { artifact_mutations: operations, ..Default::default() })
    } else {
        Ok(Emit::default())
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural3d::testkit::{app, dispatch, drain_flow_eval_ticks};
    use crate::editor::procedural3d::Procedural3dCommand;
    use crate::editor::procedural3d::commands::{add_widget, patch_flow_widgets};

    #[semio_framework_async_macros::async_test]
    async fn add_widget_action_appends_widget() {
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app();
        let before = app.snapshot().expect("snapshot").fixture.widgets.len();
        dispatch(&mut app, Procedural3dCommand::AddWidget(add_widget::AddWidget { kind: "inputNote".into(), x: None, y: None }));
        assert!(app.snapshot().expect("snapshot").fixture.widgets.len() > before);
    }

    #[semio_framework_async_macros::async_test]
    async fn patch_flow_widgets_edits_slider_value() {
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app();
        dispatch(&mut app, Procedural3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5) }));
        let value = app.snapshot().expect("snapshot").fixture.widgets.iter().find_map(|widget| match widget {
            Widget::InputSlider { id, value, .. } if id == "height" => Some(*value),
            _ => None});
        assert_eq!(value, Some(9.5));
    }

    #[semio_framework_async_macros::async_test]
    async fn patch_flow_widgets_recomputes_preview_geometry() {
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app();
        drain_flow_eval_ticks(&mut app);
        let before_eval = flow::with_process_flow_eval_session(|session| session.eval_json().to_string());
        let before_fixture = app.snapshot().expect("snapshot").fixture.clone();
        let (before_meshes, _) = crate::editor::procedural3d::preview_payload_from_eval(&before_eval, &before_fixture, &Procedural3dConfig::default());

        dispatch(&mut app, Procedural3dCommand::PatchFlowWidgets(patch_flow_widgets::PatchFlowWidgets { widget_ids: vec!["height".into()], field: "value".into(), value: Some(9.5) }));
        drain_flow_eval_ticks(&mut app);
        let after_eval = flow::with_process_flow_eval_session(|session| session.eval_json().to_string());
        let after_fixture = app.snapshot().expect("snapshot").fixture.clone();
        let (after_meshes, _) = crate::editor::procedural3d::preview_payload_from_eval(&after_eval, &after_fixture, &Procedural3dConfig::default());

        assert_ne!(before_eval, after_eval, "slider mutation must invalidate the evaluated flow");
        assert_ne!(before_meshes, after_meshes, "slider mutation must change the tessellated preview mesh");
    }

    #[semio_framework_async_macros::async_test]
    async fn remove_widget_action_deletes_by_id() {
        let _serial = crate::editor::procedural3d::test_support::lock();
        let mut app = app();
        assert!(app.snapshot().expect("snapshot").fixture.widgets.iter().any(|widget| crate::artifacts::procedural3d::widget_id(widget) == "sides"));
        dispatch(&mut app, Procedural3dCommand::RemoveWidget(RemoveWidget { widget_id: "sides".into() }));
        assert!(!app.snapshot().expect("snapshot").fixture.widgets.iter().any(|widget| crate::artifacts::procedural3d::widget_id(widget) == "sides"));
    }
}
//#endregion 🧪️Tests
