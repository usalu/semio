//! 📷️ Shooting play app commands — shot selection, labeling, sticky defaults and bulk field patches.

use crate::artifacts::shooting::mutations::change_shot_format::mutation::ChangeShotFormat;
use crate::artifacts::shooting::mutations::change_shot_height::mutation::ChangeShotHeight;
use crate::artifacts::shooting::mutations::change_shot_shape::mutation::ChangeShotShape;
use crate::artifacts::shooting::mutations::change_shot_width::mutation::ChangeShotWidth;
use crate::artifacts::shooting::mutations::create_shot::mutation::CreateShot;
use crate::artifacts::shooting::mutations::rename_shot::mutation::RenameShot;
use crate::artifacts::shooting::mutations::set_active_shot::mutation::SetActiveShot as SetActiveShotMutation;
use crate::artifacts::shooting::op::ShootingMutation;
use crate::artifacts::shooting::ShootingShot;
use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::ShootingDispatchCtx;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// 🩹️ Builds the single-field `ShootingMutation` for a `patchShot`/`patchShots`/`setActiveShot*`
/// field write, addressed at `id` — shared by `set_active_shot_format`/`set_active_shot_shape` and
/// `patch_shots` below.
async fn shot_mutation_for_field(id: String, field: &str, value: &Value) -> Option<ShootingMutation> {
    match field {
        "label" => value.as_str().map(|v| ShootingMutation::RenameShot(RenameShot { id, new_label: v.into() })),
        "width" => value.as_u64().map(|v| ShootingMutation::ChangeShotWidth(ChangeShotWidth { id, new_width: v as u32 })),
        "height" => value.as_u64().map(|v| ShootingMutation::ChangeShotHeight(ChangeShotHeight { id, new_height: v as u32 })),
        "format" => value.as_str().map(|v| ShootingMutation::ChangeShotFormat(ChangeShotFormat { id, new_format: v.into() })),
        "shape" => value.as_str().map(|v| ShootingMutation::ChangeShotShape(ChangeShotShape { id, new_shape: v.into() })),
        _ => None,
    }
}

async fn active_shot_id(fixture: &crate::artifacts::shooting::ShootingSnapshot) -> Option<String> {
    crate::artifacts::shooting::schema::active_shot(fixture).map(|shot| shot.id.clone())
}

//#region 🔖️SetActiveShot
pub mod set_active_shot {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "active-shot")]
    pub struct SetActiveShot {
        pub shot_id: Option<String>,
    }

    pub async fn handle(
        payload: &SetActiveShot,
        _doc: &ArtifactView<'_, crate::artifacts::shooting::ShootingSnapshot>,
        _cfg: &ConfigView<'_, ShootingConfig>,
        _ctx: &mut ShootingDispatchCtx,
    ) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match payload.shot_id.as_deref().filter(|id| !id.is_empty()) {
            Some(id) => Ok(Emit::mutations(vec![ShootingMutation::SetActiveShot(SetActiveShotMutation { shot_id: Some(id.into()) })])),
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

    pub async fn handle(
        payload: &SetActiveShotLabel,
        doc: &ArtifactView<'_, crate::artifacts::shooting::ShootingSnapshot>,
        _cfg: &ConfigView<'_, ShootingConfig>,
        _ctx: &mut ShootingDispatchCtx,
    ) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match active_shot_id(doc.snapshot) {
            Some(shot_id) => Ok(Emit::mutations(vec![ShootingMutation::RenameShot(RenameShot { id: shot_id, new_label: payload.value.clone() })])),
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

    pub async fn handle(
        payload: &SetActiveShotFormat,
        doc: &ArtifactView<'_, crate::artifacts::shooting::ShootingSnapshot>,
        _cfg: &ConfigView<'_, ShootingConfig>,
        _ctx: &mut ShootingDispatchCtx,
    ) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match active_shot_id(doc.snapshot).and_then(|shot_id| shot_mutation_for_field(shot_id, "format", &json!(payload.value))) {
            Some(mutation) => Ok(Emit::mutations(vec![mutation])),
            None => Ok(Emit::default()),
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

    pub async fn handle(
        payload: &SetActiveShotShape,
        doc: &ArtifactView<'_, crate::artifacts::shooting::ShootingSnapshot>,
        _cfg: &ConfigView<'_, ShootingConfig>,
        _ctx: &mut ShootingDispatchCtx,
    ) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        match active_shot_id(doc.snapshot).and_then(|shot_id| shot_mutation_for_field(shot_id, "shape", &json!(payload.value))) {
            Some(mutation) => Ok(Emit::mutations(vec![mutation])),
            None => Ok(Emit::default()),
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

    pub async fn handle(
        payload: &PatchShots,
        _doc: &ArtifactView<'_, crate::artifacts::shooting::ShootingSnapshot>,
        _cfg: &ConfigView<'_, ShootingConfig>,
        _ctx: &mut ShootingDispatchCtx,
    ) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        if payload.shot_ids.is_empty() {
            return Ok(Emit::default());
        }
        let value = json!(payload.value);
        let mutations: Vec<ShootingMutation> = payload.shot_ids.iter().cloned().filter_map(|id| shot_mutation_for_field(id, &payload.field, &value)).collect();
        if mutations.is_empty() {
            Ok(Emit::default())
        } else {
            Ok(Emit::mutations(mutations))
        }
    }
}
//#endregion 🔖️PatchShots

//#region 🔖️AddShot
pub mod add_shot {
    use super::*;
    use crate::artifacts::shooting::schema::next_shooting_id;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "add-shot")]
    pub struct AddShot {
        pub format: String,
        pub shape: String,
    }

    pub async fn handle(payload: &AddShot, doc: &ArtifactView<'_, crate::artifacts::shooting::ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let id = next_shooting_id("shot");
        let shot = ShootingShot { id: id.clone(), label: format!("Shot {}", snapshot.shots.len() + 1), width: 256, height: 256, format: payload.format.clone(), shape: payload.shape.clone(), background: None, camera_id: None };
        Ok(Emit {
            artifact_mutations: vec![ShootingMutation::CreateShot(CreateShot { shot, index: Some(snapshot.shots.len()) }), ShootingMutation::SetActiveShot(SetActiveShotMutation { shot_id: Some(id.clone()) })],
            config_mutations: vec![ShootingConfigMutation::SetShotSelection { shot_ids: vec![id] }],
            ..Default::default()
        })
    }
}
//#endregion 🔖️AddShot

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::testkit::{dispatch, shooting_app};
    use crate::editor::shooting::ShootingCommand;

    #[semio_framework_async_macros::async_test]
    async fn set_active_shot_label_patches_active_shot() {
        let mut app = shooting_app();
        dispatch(&mut app, ShootingCommand::SetActiveShotLabel(set_active_shot_label::SetActiveShotLabel { value: "Hero Shot".into() }));
        assert_eq!(crate::artifacts::shooting::schema::active_shot(&app.snapshot().expect("snapshot")).unwrap().label, "Hero Shot");
    }

    #[semio_framework_async_macros::async_test]
    async fn add_shot_action_appends_shot() {
        let mut app = shooting_app();
        dispatch(&mut app, ShootingCommand::AddShot(add_shot::AddShot { format: "svg".into(), shape: "ellipse".into() }));
        assert!(app.snapshot().expect("snapshot").shots.iter().any(|shot| shot.format == "svg" && shot.shape == "ellipse"));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_shot_updates_fixture() {
        let mut app = shooting_app();
        let second_id = app.snapshot().expect("snapshot").shots.get(1).map(|shot| shot.id.clone()).expect("second shot");
        dispatch(&mut app, ShootingCommand::SetActiveShot(set_active_shot::SetActiveShot { shot_id: Some(second_id.clone()) }));
        assert_eq!(app.snapshot().expect("snapshot").active_shot_id, second_id);
    }
}
//#endregion 🧪️Tests
