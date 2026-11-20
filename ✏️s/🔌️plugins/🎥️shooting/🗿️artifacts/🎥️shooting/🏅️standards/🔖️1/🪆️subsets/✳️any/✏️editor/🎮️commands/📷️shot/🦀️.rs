//! 📷️ Shooting play app commands — shot selection, labeling, sticky defaults and bulk field patches.

use crate::editor::shooting::config::{ShootingConfig, ShootingConfigMutation};
use crate::editor::shooting::ui_helpers_and_io::ShootingDispatchCtx;
use crate::mutations::change_shot_format::ChangeShotFormat;
use crate::mutations::change_shot_height::ChangeShotHeight;
use crate::mutations::change_shot_shape::ChangeShotShape;
use crate::mutations::change_shot_width::ChangeShotWidth;
use crate::mutations::create_shot::CreateShot;
use crate::mutations::create_saved_camera::CreateSavedCamera;
use crate::mutations::patch_shot_camera::PatchShotCamera;
use crate::mutations::rename_shot::RenameShot;
use crate::mutations::set_active_shot::SetActiveShot as SetActiveShotMutation;
use crate::mutations::set_shot_auto_fit::SetShotAutoFit;
use crate::standards::v1::subsets::any::schema::mutations::ShootingMutation;
use crate::ShootingShot;
use semio_framework_pack_json::json;
use semio_framework_pack_json::Value;
use semio_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

/// 🩹️ Builds the single-field `ShootingMutation` for a `patchShot`/`patchShots`/`setActiveShot*`
/// field write, addressed at `id` — shared by `set_active_shot_format`/`set_active_shot_shape` and
/// `patch_shots` below.
fn shot_dimension(value: &Value) -> Option<u32> {
    value.as_u64().and_then(|value| u32::try_from(value).ok()).or_else(|| value.as_str()?.parse::<u32>().ok())
}

pub fn shot_mutation_for_field(id: String, field: &str, value: &Value) -> Option<ShootingMutation> {
    match field {
        "label" => value.as_str().map(|v| ShootingMutation::RenameShot(RenameShot { id, new_label: v.into() })),
        "width" => shot_dimension(value).map(|new_width| ShootingMutation::ChangeShotWidth(ChangeShotWidth { id, new_width })),
        "height" => shot_dimension(value).map(|new_height| ShootingMutation::ChangeShotHeight(ChangeShotHeight { id, new_height })),
        "format" => value.as_str().map(|v| ShootingMutation::ChangeShotFormat(ChangeShotFormat { id, new_format: v.into() })),
        "shape" => value.as_str().map(|v| ShootingMutation::ChangeShotShape(ChangeShotShape { id, new_shape: v.into() })),
        "autoFit" => {
            let auto_fit = match value.as_str()? {
                "scene" => None,
                "fit" => Some(true),
                "manual" => Some(false),
                _ => return None,
            };
            Some(ShootingMutation::SetShotAutoFit(SetShotAutoFit { id, auto_fit }))
        }
        _ => None,
    }
}

fn active_shot_id(snapshot: &crate::ShootingSnapshot) -> Option<String> {
    crate::active_shot(snapshot).map(|shot| shot.id.clone())
}

//#region 🔖️SetActiveShot
pub mod set_active_shot {
    use super::*;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
    #[dsl(keyword = "active-shot")]
    pub struct SetActiveShot {
        pub shot_id: Option<String>,
    }

    pub fn handle(payload: &SetActiveShot, _doc: &ArtifactView<'_, crate::ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "active-shot-label")]
    pub struct SetActiveShotLabel {
        pub value: String,
    }

    pub fn handle(payload: &SetActiveShotLabel, doc: &ArtifactView<'_, crate::ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "active-shot-format")]
    pub struct SetActiveShotFormat {
        pub value: String,
    }

    pub fn handle(payload: &SetActiveShotFormat, doc: &ArtifactView<'_, crate::ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "active-shot-shape")]
    pub struct SetActiveShotShape {
        pub value: String,
    }

    pub fn handle(payload: &SetActiveShotShape, doc: &ArtifactView<'_, crate::ShootingSnapshot>, _cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
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

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "patch-shots")]
    pub struct PatchShots {
        pub shot_ids: Vec<String>,
        pub field: String,
        pub value: String,
    }

    fn color(value: &str) -> Option<String> {
        let digits = value.strip_prefix('#')?;
        ((digits.len() == 3 || digits.len() == 6) && digits.bytes().all(|b| b.is_ascii_hexdigit())).then(|| value.to_owned())
    }

    fn patch_drawing(id: &str, field: &str, value: &str) -> Option<ShootingMutation> {
        let value = value.trim();
        let mut patch = crate::ShootingDrawingPatch::default();
        match field {
            "background" => patch.background = Some(value.parse().ok()?),
            "fill" => patch.fill = Some(value.parse().ok()?),
            "fillColor" => patch.fill_color = Some(crate::ShootingAssigned { value: if value.is_empty() { None } else { Some(color(value)?) } }),
            "fillOpacity" => patch.fill_opacity = Some(value.parse::<f64>().ok().filter(|v| (0.0..=1.0).contains(v))?),
            "visibleLines" => patch.visible_lines = Some(value.parse().ok()?),
            "hiddenLines" => patch.hidden_lines = Some(value.parse().ok()?),
            "mergeLines" => patch.merge_lines = Some(value.parse().ok()?),
            "visibleColor" => patch.visible_color = Some(color(value)?),
            "hiddenColor" => patch.hidden_color = Some(color(value)?),
            "visibleWidth" | "hiddenWidth" => {
                let width = value.parse::<f64>().ok().filter(|v| *v > 0.0 && *v <= 64.0)?;
                if field == "visibleWidth" { patch.visible_width = Some(width); } else { patch.hidden_width = Some(width); }
            }
            "lineEasing" => patch.line_easing = Some(value.parse::<f64>().ok().filter(|v| (0.0..=10.0).contains(v))?),
            _ => return None,
        }
        Some(ShootingMutation::SetShotDrawing(crate::mutations::set_shot_drawing::SetShotDrawing { id: id.to_owned(), patch }))
    }

    fn patch_camera(camera: &mut crate::ShootingCamera, field: &str, value: &str) -> bool {
        match field {
            "projection" if matches!(value, "perspective" | "orthographic") => camera.projection = Some(value.to_owned()),
            "zoom" | "fov" => {
                let Some(number) = value.parse::<f64>().ok().filter(|v| v.is_finite() && *v > 0.0 && (field == "zoom" || *v < 180.0)) else { return false };
                if field == "zoom" { camera.zoom = number; } else { camera.fov = number; }
            }
            "eyeX" | "eyeY" | "eyeZ" | "targetX" | "targetY" | "targetZ" | "upX" | "upY" | "upZ" => {
                let Some(number) = value.parse::<f64>().ok().filter(|v| v.is_finite()) else { return false };
                let axis = match field.as_bytes().last() { Some(b'X') => 0, Some(b'Y') => 1, _ => 2 };
                let coordinates = if field.starts_with("eye") {
                    &mut camera.position
                } else if field.starts_with("target") {
                    &mut camera.target
                } else {
                    camera.up.get_or_insert([0.0, 0.0, 1.0])
                };
                coordinates[axis] = number;
            }
            _ => return false,
        }
        PatchShotCamera::valid_camera(camera)
    }

    pub fn handle(payload: &PatchShots, doc: &ArtifactView<'_, crate::ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let field = payload.field.as_str();
        let value = json!(payload.value);
        let mut result = Emit::default();
        for id in &payload.shot_ids {
            let Some(shot) = doc.snapshot.shots.iter().find(|shot| &shot.id == id) else { continue };
            if let Some(mutation) = shot_mutation_for_field(id.clone(), field, &value).or_else(|| patch_drawing(id, field, &payload.value)) {
                result.artifact_mutations.push(mutation);
            } else {
                let fallback = crate::shooting_resolve_shot_camera(doc.snapshot, shot, &cfg.snapshot.camera);
                let mut camera = fallback.clone();
                if !patch_camera(&mut camera, field, &payload.value) { continue; }
                let manual_framing = field == "zoom"
                    || (field == "fov" && camera.projection.as_deref() != Some("orthographic"))
                    || field.starts_with("eye") || field.starts_with("target");
                result.artifact_mutations.push(ShootingMutation::PatchShotCamera(PatchShotCamera {
                    shot_id: id.clone(),
                    field: field.to_owned(),
                    camera,
                    fallback,
                    release: None,
                }));
                if manual_framing && shot.auto_fit != Some(false) {
                    result.artifact_mutations.push(ShootingMutation::SetShotAutoFit(SetShotAutoFit { id: id.clone(), auto_fit: Some(false) }));
                }
            }
        }
        Ok(result)
    }
}
//#endregion 🔖️PatchShots

//#region 🔖️AddShot
pub mod add_shot {
    use super::*;
    use crate::next_shooting_id;

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "add-shot")]
    pub struct AddShot {
        pub format: String,
        pub shape: String,
    }

    pub fn handle(payload: &AddShot, doc: &ArtifactView<'_, crate::ShootingSnapshot>, cfg: &ConfigView<'_, ShootingConfig>, _ctx: &mut ShootingDispatchCtx) -> Result<Emit<ShootingMutation, ShootingConfigMutation>, Fault> {
        let snapshot = doc.snapshot;
        let id = next_shooting_id("shot");
        let camera_id = next_shooting_id("camera");
        let saved_camera = crate::ShootingSavedCamera { id: camera_id.clone(), label: format!("Shot {}", snapshot.shots.len() + 1), camera: cfg.snapshot.camera.clone() };
        let shot = ShootingShot { id: id.clone(), label: format!("Shot {}", snapshot.shots.len() + 1), width: 256, height: 256, format: payload.format.clone(), shape: payload.shape.clone(), background: None, drawing: None, camera_id: Some(camera_id), auto_fit: None };
        Ok(Emit {
            artifact_mutations: vec![ShootingMutation::CreateSavedCamera(CreateSavedCamera { saved_camera, index: Some(snapshot.saved_cameras.len()) }), ShootingMutation::CreateShot(CreateShot { shot, index: Some(snapshot.shots.len()) }), ShootingMutation::SetActiveShot(SetActiveShotMutation { shot_id: Some(id.clone()) })],
            config_mutations: vec![ShootingConfigMutation::SetShotSelection(crate::editor::shooting::config::SetShotSelection { shot_ids: vec![id] })],
            ..Default::default()
        })
    }
}
//#endregion 🔖️AddShot

//#region 🧪️Tests

//#endregion 🧪️Tests


