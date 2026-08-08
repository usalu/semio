//! 📷️ Shooting play app commands — shot selection, labeling, sticky defaults and bulk field patches.

use crate::apps::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::{ShootingShot, ShootingShotPatch};
use protocol::CollectionMutation;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// 🩹️ Builds the `ShootingShotPatch` for a `patchShot`/`patchShots`/`setActiveShot*` field write —
/// shared by `set_active_shot_format`/`set_active_shot_shape` and `patch_shots` below.
fn shot_patch_for_field(field: &str, value: &Value) -> Option<ShootingShotPatch> {
    match field {
        "label" => value.as_str().map(|v| ShootingShotPatch { label: Some(v.into()), ..Default::default() }),
        "width" => value.as_u64().map(|v| ShootingShotPatch { width: Some(v as u32), ..Default::default() }),
        "height" => value.as_u64().map(|v| ShootingShotPatch { height: Some(v as u32), ..Default::default() }),
        "format" => value.as_str().map(|v| ShootingShotPatch { format: Some(v.into()), ..Default::default() }),
        "shape" => value.as_str().map(|v| ShootingShotPatch { shape: Some(v.into()), ..Default::default() }),
        _ => None,
    }
}

fn active_shot_id(fixture: &crate::artifacts::shooting::ShootingFixture) -> Option<String> {
    crate::artifacts::shooting::engine::active_shot(fixture).map(|shot| shot.id.clone())
}

//#region 🔖️SetActiveShot
pub mod set_active_shot {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-shot")]
    pub struct SetActiveShot {
        pub shot_id: Option<String>,
    }

    pub fn handle(payload: &SetActiveShot, _doc: &DocumentView<'_, crate::artifacts::shooting::ShootingFixture>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match payload.shot_id.as_deref().filter(|id| !id.is_empty()) {
            Some(id) => Ok(Emit::mutations(vec![ShootingMutation::SetActiveShot { shot_id: Some(id.into()) }])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️SetActiveShot

//#region 🔖️SetActiveShotLabel
pub mod set_active_shot_label {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-shot-label")]
    pub struct SetActiveShotLabel {
        pub value: String,
    }

    pub fn handle(payload: &SetActiveShotLabel, doc: &DocumentView<'_, crate::artifacts::shooting::ShootingFixture>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match active_shot_id(doc.projection) {
            Some(shot_id) => Ok(Emit::mutations(vec![ShootingMutation::Shots(CollectionMutation::Patch { id: shot_id, patch: ShootingShotPatch { label: Some(payload.value.clone()), ..Default::default() } })])),
            None => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️SetActiveShotLabel

//#region 🔖️SetActiveShotFormat
pub mod set_active_shot_format {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-shot-format")]
    pub struct SetActiveShotFormat {
        pub value: String,
    }

    pub fn handle(payload: &SetActiveShotFormat, doc: &DocumentView<'_, crate::artifacts::shooting::ShootingFixture>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match (active_shot_id(doc.projection), shot_patch_for_field("format", &json!(payload.value))) {
            (Some(shot_id), Some(patch)) => Ok(Emit::mutations(vec![ShootingMutation::Shots(CollectionMutation::Patch { id: shot_id, patch })])),
            _ => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️SetActiveShotFormat

//#region 🔖️SetActiveShotShape
pub mod set_active_shot_shape {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-shot-shape")]
    pub struct SetActiveShotShape {
        pub value: String,
    }

    pub fn handle(payload: &SetActiveShotShape, doc: &DocumentView<'_, crate::artifacts::shooting::ShootingFixture>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match (active_shot_id(doc.projection), shot_patch_for_field("shape", &json!(payload.value))) {
            (Some(shot_id), Some(patch)) => Ok(Emit::mutations(vec![ShootingMutation::Shots(CollectionMutation::Patch { id: shot_id, patch })])),
            _ => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️SetActiveShotShape

//#region 🔖️PatchShots
pub mod patch_shots {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "patch-shots")]
    pub struct PatchShots {
        pub shot_ids: Vec<String>,
        pub field: String,
        pub value: String,
    }

    pub fn handle(payload: &PatchShots, _doc: &DocumentView<'_, crate::artifacts::shooting::ShootingFixture>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match shot_patch_for_field(&payload.field, &json!(payload.value)) {
            Some(patch) if !payload.shot_ids.is_empty() => Ok(Emit::mutations(payload.shot_ids.iter().cloned().map(|id| ShootingMutation::Shots(CollectionMutation::Patch { id, patch: patch.clone() })).collect())),
            _ => Ok(Emit::default()),
        }
    }
}
//#endregion 🔖️PatchShots

//#region 🔖️AddShot
pub mod add_shot {
    use super::*;
    use crate::artifacts::shooting::engine::next_shooting_id;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-shot")]
    pub struct AddShot {
        pub format: String,
        pub shape: String,
    }

    pub fn handle(payload: &AddShot, doc: &DocumentView<'_, crate::artifacts::shooting::ShootingFixture>, _cfg: &ConfigView<'_, ShootingConfig>) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let fixture = doc.projection;
        let id = next_shooting_id("shot");
        let shot = ShootingShot { id: id.clone(), label: format!("Shot {}", fixture.shots.len() + 1), width: 256, height: 256, format: payload.format.clone(), shape: payload.shape.clone(), background: None, camera_id: None };
        Ok(Emit {
            document_mutations: vec![ShootingMutation::Shots(CollectionMutation::Add { index: fixture.shots.len(), item: shot }), ShootingMutation::SetActiveShot { shot_id: Some(id.clone()) }],
            config_mutations: vec![ShootingConfigMutation::SetSelection { shot_ids: vec![id], asset_ids: Vec::new() }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️AddShot

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::shooting::testkit::{dispatch, shooting_app};
    use crate::apps::shooting::ShootingCommand;

    #[test]
    fn set_active_shot_label_patches_active_shot() {
        let mut app = shooting_app();
        dispatch(&mut app, ShootingCommand::SetActiveShotLabel(set_active_shot_label::SetActiveShotLabel { value: "Hero Shot".into() }));
        assert_eq!(crate::artifacts::shooting::engine::active_shot(&app.projection().expect("projection")).unwrap().label, "Hero Shot");
    }

    #[test]
    fn add_shot_action_appends_shot() {
        let mut app = shooting_app();
        dispatch(&mut app, ShootingCommand::AddShot(add_shot::AddShot { format: "svg".into(), shape: "ellipse".into() }));
        assert!(app.projection().expect("projection").shots.iter().any(|shot| shot.format == "svg" && shot.shape == "ellipse"));
    }

    #[test]
    fn set_active_shot_updates_fixture() {
        let mut app = shooting_app();
        let second_id = app.projection().expect("projection").shots.get(1).map(|shot| shot.id.clone()).expect("second shot");
        dispatch(&mut app, ShootingCommand::SetActiveShot(set_active_shot::SetActiveShot { shot_id: Some(second_id.clone()) }));
        assert_eq!(app.projection().expect("projection").active_shot_id, second_id);
    }
}
//#endregion 🧪️Tests
