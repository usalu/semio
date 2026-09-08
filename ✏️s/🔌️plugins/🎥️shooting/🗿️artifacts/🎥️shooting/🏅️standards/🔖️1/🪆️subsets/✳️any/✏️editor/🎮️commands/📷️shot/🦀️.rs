//! 📷️ Shooting play app commands — shot selection, labeling, sticky defaults and bulk field patches.

use crate::mutations::change_shot_format::ChangeShotFormat;
use crate::mutations::change_shot_height::ChangeShotHeight;
use crate::mutations::change_shot_shape::ChangeShotShape;
use crate::mutations::change_shot_width::ChangeShotWidth;
use crate::mutations::create_shot::CreateShot;
use crate::mutations::rename_shot::RenameShot;
use crate::mutations::set_active_shot::SetActiveShot as SetActiveShotMutation;
use crate::op::ShootingMutation;
use crate::ShootingShot;
use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::ShootingDispatchCtx;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::json;
use dsl::os_pack::json::Value;
use semio_framework_value_derive::{FromValue, ToValue};

/// 🩹️ Builds the single-field `ShootingMutation` for a `patchShot`/`patchShots`/`setActiveShot*`
/// field write, addressed at `id` — shared by `set_active_shot_format`/`set_active_shot_shape` and
/// `patch_shots` below.
fn shot_dimension(value: &Value) -> Option<u32> {
    value.as_u64().and_then(|value| u32::try_from(value).ok()).or_else(|| value.as_str()?.parse::<u32>().ok())
}

fn shot_mutation_for_field(id: String, field: &str, value: &Value) -> Option<ShootingMutation> {
    match field {
        "label" => value.as_str().map(|v| ShootingMutation::RenameShot(RenameShot { id, new_label: v.into() })),
        "width" => shot_dimension(value).map(|new_width| ShootingMutation::ChangeShotWidth(ChangeShotWidth { id, new_width })),
        "height" => shot_dimension(value).map(|new_height| ShootingMutation::ChangeShotHeight(ChangeShotHeight { id, new_height })),
        "format" => value.as_str().map(|v| ShootingMutation::ChangeShotFormat(ChangeShotFormat { id, new_format: v.into() })),
        "shape" => value.as_str().map(|v| ShootingMutation::ChangeShotShape(ChangeShotShape { id, new_shape: v.into() })),
        _ => None,
    }
}

fn active_shot_id(fixture: &crate::ShootingSnapshot) -> Option<String> {
    crate::schema::active_shot(fixture).map(|shot| shot.id.clone())
}

//#region 🔖️SetActiveShot
pub mod set_active_shot {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "active-shot")]
    pub struct SetActiveShot {
        pub shot_id: Option<String>,
    }

    pub fn handle(
        payload: &SetActiveShot,
        _doc: &ArtifactView<'_, crate::ShootingSnapshot>,
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "active-shot-label")]
    pub struct SetActiveShotLabel {
        pub value: String,
    }

    pub fn handle(
        payload: &SetActiveShotLabel,
        doc: &ArtifactView<'_, crate::ShootingSnapshot>,
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "active-shot-format")]
    pub struct SetActiveShotFormat {
        pub value: String,
    }

    pub fn handle(
        payload: &SetActiveShotFormat,
        doc: &ArtifactView<'_, crate::ShootingSnapshot>,
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "active-shot-shape")]
    pub struct SetActiveShotShape {
        pub value: String,
    }

    pub fn handle(
        payload: &SetActiveShotShape,
        doc: &ArtifactView<'_, crate::ShootingSnapshot>,
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "patch-shots")]
    pub struct PatchShots {
        pub shot_ids: Vec<String>,
        pub field: String,
        pub value: String,
    }

    pub fn handle(
        payload: &PatchShots,
        _doc: &ArtifactView<'_, crate::ShootingSnapshot>,
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
    use crate::schema::next_shooting_id;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
    #[dsl(keyword = "add-shot")]
    pub struct AddShot {
        pub format: String,
        pub shape: String,
    }

    pub fn handle(payload: &AddShot, doc: &ArtifactView<'_, crate::ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let id = next_shooting_id("shot");
        let shot = ShootingShot { id: id.clone(), label: format!("Shot {}", snapshot.shots.len() + 1), width: 256, height: 256, format: payload.format.clone(), shape: payload.shape.clone(), background: None, camera_id: None };
        Ok(Emit {
            artifact_mutations: vec![ShootingMutation::CreateShot(CreateShot { shot, index: Some(snapshot.shots.len()) }), ShootingMutation::SetActiveShot(SetActiveShotMutation { shot_id: Some(id.clone()) })],
            config_mutations: vec![ShootingConfigMutation::SetShotSelection(crate::editor::shooting::config::SetShotSelection { shot_ids: vec![id] })],
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
        let mut app = shooting_app().await;
        dispatch(&mut app, ShootingCommand::SetActiveShotLabel(set_active_shot_label::SetActiveShotLabel { value: "Hero Shot".into() })).await;
        assert_eq!(crate::schema::active_shot(&app.snapshot().expect("snapshot")).unwrap().label, "Hero Shot");
    }

    #[semio_framework_async_macros::async_test]
    async fn add_shot_action_appends_shot() {
        let mut app = shooting_app().await;
        dispatch(&mut app, ShootingCommand::AddShot(add_shot::AddShot { format: "svg".into(), shape: "ellipse".into() })).await;
        assert!(app.snapshot().expect("snapshot").shots.iter().any(|shot| shot.format == "svg" && shot.shape == "ellipse"));
    }

    #[semio_framework_async_macros::async_test]
    async fn set_active_shot_updates_fixture() {
        let mut app = shooting_app().await;
        let second_id = app.snapshot().expect("snapshot").shots.get(1).map(|shot| shot.id.clone()).expect("second shot");
        dispatch(&mut app, ShootingCommand::SetActiveShot(set_active_shot::SetActiveShot { shot_id: Some(second_id.clone()) })).await;
        assert_eq!(app.snapshot().expect("snapshot").active_shot_id, second_id);
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod field_value_contract {
    use super::*;

    #[test]
    fn shooting_shot_field_values_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔢️field-values.json")).expect("neutral input vectors");
        let base = crate::schema::default_snapshot();
        let id = base.shots[0].id.clone();
        for vector in vectors["cases"].as_array().expect("cases") {
            let field = vector["field"].as_str().expect("field");
            let value = Value::String(vector["value"].as_str().expect("input string").into());
            let mutation = shot_mutation_for_field(id.clone(), field, &value);
            if vector["expected"].is_null() {
                assert!(mutation.is_none(), "invalid dimension must reject without truncation");
            } else {
                let (next, _) = store::apply_mutation(&base, &mutation.expect("valid input")).expect("apply shot field");
                let json: serde_json::Value = serde_json::from_str(&dsl::os_pack::to_json_string(&next)).expect("independent snapshot oracle");
                assert_eq!(json["shots"][0][field], vector["expected"]);
            }
        }
    }
}
