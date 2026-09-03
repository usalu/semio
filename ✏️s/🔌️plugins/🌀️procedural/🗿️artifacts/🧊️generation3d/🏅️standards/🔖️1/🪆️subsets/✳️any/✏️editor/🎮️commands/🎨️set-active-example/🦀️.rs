//! 🎨️ 🎨️ Generation3d play app commands command — `set-active-example`.

use crate::artifacts::generation3d::op::{generation_mutation_to_generation3d, generation3d_fixture_operations, Generation3dMutation};
use crate::artifacts::generation3d::schema::{default_snapshot, example_snapshot, is_generation3d_example_id};
use crate::artifacts::generation3d::Generation3dSnapshot;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use flow::playbook::GenerationMutation;
use flow::{CameraJson, FlowEvalSession};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧾️ Resets the ephemeral generation-preview to match a freshly-loaded example, keeping every other
/// display option (preview camera, LOD, show mode, sun, active utility, locale)
/// unchanged. `graph`'s selection resets on its own — the framework prunes it against the new
/// fixture's `interaction_topology` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
fn config_after_example_load(previous: &Generation3dConfig, flow_camera: &CameraJson) -> Generation3dConfig {
    Generation3dConfig {
        camera: flow_camera.clone(),
        selected_generation_id: None,
        generation_preview_text: None,
        preview_camera: previous.preview_camera.clone(),
        lod_mode: previous.lod_mode.clone(),
        show_mode: previous.show_mode.clone(),
        sun_json: previous.sun_json.clone(),
        active_utility_id: previous.active_utility_id.clone(),
        locale: previous.locale.clone(),
        preview_eval_text: None,
    }
}

//#region 🔖️SetActiveExample
//#endregion 🔖️SetActiveExample

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String,
}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Generation3dSnapshot>, cfg: &ConfigView<'_, Generation3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    session.set_eval_json(String::new());
    let fixture = &doc.snapshot.fixture;
    let target = if payload.example_id.is_empty() {
        default_snapshot()
    } else if is_generation3d_example_id(&payload.example_id) {
        example_snapshot(&payload.example_id).unwrap_or_default()
    } else {
        return Ok(Emit::default());
    };
    let mut operations: Vec<Generation3dMutation> = doc.snapshot.generation.generations.iter().map(|generation| generation_mutation_to_generation3d(GenerationMutation::Remove { id: generation.id.clone() })).collect();
    operations.extend(generation3d_fixture_operations(fixture, &target.fixture));
    Ok(Emit { artifact_mutations: operations, config_mutations: vec![Generation3dConfigMutation::Snapshot { config: config_after_example_load(cfg.snapshot, &target.fixture.camera) }], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::generation3d::schema::PROCEDURAL_EXAMPLE_BOX_FILLET;
    use crate::editor::generation3d::testkit::{app, app_with_registry, dispatch};
    use crate::editor::generation3d::Generation3dCommand;
    use flow::Widget;
    use semio_framework_plugin::PluginApp;

    #[semio_framework_async_macros::async_test]
    async fn set_active_example_via_string_action_loads_fixture() {
        let _serial = crate::editor::generation3d::test_support::lock();
        let mut app = app_with_registry().await;
        app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCEDURAL_EXAMPLE_BOX_FILLET })), &semio_framework_plugin::testkit::meta("local")).await.expect("set example");
        let projection = app.snapshot().expect("snapshot");
        assert!(projection
            .fixture
            .widgets
            .iter()
            .any(|widget| crate::artifacts::generation3d::widget_id(widget).contains("fillet") || matches!(widget, Widget::Neuron { neuron_kind, .. } if neuron_kind.contains("fillet") || neuron_kind.contains("box"))));
    }

    #[semio_framework_async_macros::async_test]
    async fn unknown_example_id_is_a_no_op() {
        let _serial = crate::editor::generation3d::test_support::lock();
        let mut app = app().await;
        let before = app.snapshot().expect("snapshot");
        dispatch(&mut app, Generation3dCommand::SetActiveExample(SetActiveExample { example_id: "not-a-real-example".into() })).await;
        assert_eq!(app.snapshot().expect("snapshot"), before);
    }
}
//#endregion 🧪️Tests
