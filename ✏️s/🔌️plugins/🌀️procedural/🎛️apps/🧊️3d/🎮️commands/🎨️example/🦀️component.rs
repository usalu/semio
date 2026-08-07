//! 🎨️ Procedural3d play app commands — loading a bundled example fixture (document-mutating; clears
//! generations and resets ephemeral view state).

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigOperation};
use crate::artifacts::procedural3d::engine::{default_projection, example_projection, is_procedural3d_example_id};
use crate::artifacts::procedural3d::op::{procedural3d_fixture_operations, Procedural3dOperation};
use crate::artifacts::procedural3d::Procedural3dDocument;
use flow::{CameraJson, FlowEvalSession};
use playbook::GenerationOperation;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🧾️ Resets ephemeral selection/generation-preview to match a freshly-loaded example, keeping every
/// other display option (preview camera, LOD, show mode, selection method, sun, active utility,
/// locale, contributions) unchanged.
fn config_after_example_load(previous: &Procedural3dConfig, flow_camera: &CameraJson) -> Procedural3dConfig {
    Procedural3dConfig {
        camera: flow_camera.clone(),
        selected_node_ids: Vec::new(),
        hovered_node_id: None,
        selected_generation_id: None,
        generation_preview_text: None,
        preview_camera: previous.preview_camera.clone(),
        lod_mode: previous.lod_mode.clone(),
        show_mode: previous.show_mode.clone(),
        selection_method: previous.selection_method.clone(),
        sun_json: previous.sun_json.clone(),
        active_utility_id: previous.active_utility_id.clone(),
        locale: previous.locale.clone(),
        contributions_json: previous.contributions_json.clone(),
    }
}

//#region 🔖️SetActiveExample
pub mod set_active_example {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-example")]
    pub struct SetActiveExample {
        pub example_id: String,
    }

    pub fn handle(payload: &SetActiveExample, doc: &DocumentView<'_, Procedural3dDocument>, cfg: &ConfigView<'_, Procedural3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural3dOperation, Procedural3dConfigOperation>, Fault> {
        session.set_eval_json(String::new());
        let fixture = &doc.projection.fixture;
        let target = if payload.example_id.is_empty() {
            default_projection()
        } else if is_procedural3d_example_id(&payload.example_id) {
            example_projection(&payload.example_id).unwrap_or_default()
        } else {
            return Ok(Emit::default());
        };
        let mut operations: Vec<Procedural3dOperation> = doc.projection.generation.generations.iter().map(|generation| Procedural3dOperation::Generation(GenerationOperation::Remove { id: generation.id.clone() })).collect();
        operations.extend(procedural3d_fixture_operations(fixture, &target.fixture));
        Ok(Emit { document_operations: operations, config_operations: vec![Procedural3dConfigOperation::Snapshot { config: config_after_example_load(cfg.projection, &target.fixture.camera) }], ..Default::default() })
    }
}
//#endregion 🔖️SetActiveExample

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app, app_with_registry, dispatch};
    use crate::apps::procedural3d::Procedural3dCommand;
    use crate::artifacts::procedural3d::engine::PROCEDURAL_EXAMPLE_BOX_FILLET;
    use flow::Widget;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn set_active_example_via_string_action_loads_fixture() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app_with_registry();
        app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCEDURAL_EXAMPLE_BOX_FILLET })), &semio_framework_plugin::testkit::meta("local")).expect("set example");
        let projection = app.projection().expect("projection");
        assert!(projection.fixture.widgets.iter().any(|widget| crate::artifacts::procedural3d::widget_id(widget).contains("fillet") || matches!(widget, Widget::Neuron { neuron_kind, .. } if neuron_kind.contains("fillet") || neuron_kind.contains("box"))));
    }

    #[test]
    fn unknown_example_id_is_a_no_op() {
        let _serial = crate::artifacts::procedural3d::engine::test_support::lock();
        let mut app = app();
        let before = app.projection().expect("projection");
        dispatch(&mut app, Procedural3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "not-a-real-example".into() }));
        assert_eq!(app.projection().expect("projection"), before);
    }
}
//#endregion 🧪️Tests
