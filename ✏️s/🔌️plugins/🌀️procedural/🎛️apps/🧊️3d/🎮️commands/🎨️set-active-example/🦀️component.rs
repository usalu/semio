//! 🎨️ 🎨️ Procedural3d play app commands command — `set-active-example`.

use crate::apps::procedural3d::config::{Procedural3dConfig, Procedural3dConfigMutation};
use crate::artifacts::procedural3d::schema::{default_snapshot, example_snapshot, is_procedural3d_example_id};
use crate::artifacts::procedural3d::op::{generation_mutation_to_procedural3d, procedural3d_fixture_operations, Procedural3dMutation};
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use flow::{CameraJson, FlowEvalSession};
use flow::playbook::GenerationMutation;
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

/// 🧾️ Resets the ephemeral generation-preview to match a freshly-loaded example, keeping every other
/// display option (preview camera, LOD, show mode, sun, active utility, locale)
/// unchanged. `graph`'s selection resets on its own — the framework prunes it against the new
/// fixture's `interaction_topology` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
fn config_after_example_load(previous: &Procedural3dConfig, flow_camera: &CameraJson) -> Procedural3dConfig {
    Procedural3dConfig {
        camera: flow_camera.clone(),
        selected_generation_id: None,
        generation_preview_text: None,
        preview_camera: previous.preview_camera.clone(),
        lod_mode: previous.lod_mode.clone(),
        show_mode: previous.show_mode.clone(),
        sun_json: previous.sun_json.clone(),
        active_utility_id: previous.active_utility_id.clone(),
        locale: previous.locale.clone()}
}

//#region 🔖️SetActiveExample
//#endregion 🔖️SetActiveExample

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "active-example")]
pub struct SetActiveExample {
    pub example_id: String}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Procedural3dSnapshot>, cfg: &ConfigView<'_, Procedural3dConfig>, session: &mut FlowEvalSession) -> Result<Emit<Procedural3dMutation, Procedural3dConfigMutation>, Fault> {
    session.set_eval_json(String::new());
    let fixture = &doc.snapshot.fixture;
    let target = if payload.example_id.is_empty() {
        default_snapshot()
    } else if is_procedural3d_example_id(&payload.example_id) {
        example_snapshot(&payload.example_id).unwrap_or_default()
    } else {
        return Ok(Emit::default());
    };
    let mut operations: Vec<Procedural3dMutation> = doc.snapshot.generation.generations.iter().map(|generation| generation_mutation_to_procedural3d(GenerationMutation::Remove { id: generation.id.clone() })).collect();
    operations.extend(procedural3d_fixture_operations(fixture, &target.fixture));
    Ok(Emit { artifact_mutations: operations, config_mutations: vec![Procedural3dConfigMutation::Snapshot { config: config_after_example_load(cfg.snapshot, &target.fixture.camera) }], ..Default::default() })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app, app_with_registry, dispatch};
    use crate::apps::procedural3d::Procedural3dCommand;
    use crate::artifacts::procedural3d::schema::PROCEDURAL_EXAMPLE_BOX_FILLET;
    use flow::Widget;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn set_active_example_via_string_action_loads_fixture() {
        let _serial = crate::apps::procedural3d::test_support::lock();
        let mut app = app_with_registry();
        app.handle_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCEDURAL_EXAMPLE_BOX_FILLET })), &semio_framework_plugin::testkit::meta("local")).expect("set example");
        let projection = app.snapshot().expect("snapshot");
        assert!(projection.fixture.widgets.iter().any(|widget| crate::artifacts::procedural3d::widget_id(widget).contains("fillet") || matches!(widget, Widget::Neuron { neuron_kind, .. } if neuron_kind.contains("fillet") || neuron_kind.contains("box"))));
    }

    #[test]
    fn unknown_example_id_is_a_no_op() {
        let _serial = crate::apps::procedural3d::test_support::lock();
        let mut app = app();
        let before = app.snapshot().expect("snapshot");
        dispatch(&mut app, Procedural3dCommand::SetActiveExample(SetActiveExample { example_id: "not-a-real-example".into() }));
        assert_eq!(app.snapshot().expect("snapshot"), before);
    }
}
//#endregion 🧪️Tests
