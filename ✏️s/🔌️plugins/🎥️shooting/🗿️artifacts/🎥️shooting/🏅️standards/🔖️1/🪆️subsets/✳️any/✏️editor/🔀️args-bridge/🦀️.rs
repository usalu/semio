//! 🔀️ Host-action bridge of the shooting editor: folds the shell `{action, args}` vocabulary into the closed `ShootingCommand` enum. See [`super`].

//#region 🔖️ActionBridge
use super::*;
use semio_framework_plugin::{FaultCode, FaultOrigin};

fn snake(key: &str) -> String {
    let mut out = String::with_capacity(key.len() + 4);
    for ch in key.chars() {
        if ch.is_ascii_uppercase() {
            out.push('_');
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn put(entries: &mut Vec<(String, DslValue)>, key: &str, value: DslValue) {
    entries.retain(|(existing, _)| existing != key);
    entries.push((key.to_string(), value));
}

/// 🔢️ The host's JSON round trip turns every integer into `Number::Float`; the `u64`/`i64` codecs
/// decode EXACT integers only, so whole finite floats get their integer variant back (`f64` fields
/// accept any `Number` variant, so nothing else changes).
fn integral(value: DslValue) -> DslValue {
    match value {
        DslValue::Number(semio_framework_value::Number::Float(float)) if float.is_finite() && float.fract() == 0.0 && float.abs() < 9.007_199_254_740_992e15 => {
            if float >= 0.0 { DslValue::Number(semio_framework_value::Number::UInt(float as u64)) } else { DslValue::Number(semio_framework_value::Number::Int(float as i64)) }
        }
        DslValue::Array(items) => DslValue::Array(items.into_iter().map(integral).collect()),
        DslValue::Object(entries) => DslValue::Object(entries.into_iter().map(|(key, value)| (key, integral(value))).collect()),
        other => other,
    }
}

/// 📝️ Prints a host control value into the `String` field the patch verbs carry (`"512"`, `"true"`,
/// or the text itself) — the reducers re-parse per field.
fn stringify(value: DslValue) -> DslValue {
    match value {
        DslValue::String(_) => value,
        DslValue::Null => DslValue::String(String::new()),
        other => DslValue::String(semio_framework_pack_json::to_json_string(&other)),
    }
}

/// 🔁️ Snake-cases every key of `args`, applies `aliases` (snake_case source → destination, first
/// present source wins and never overwrites a present destination) and seeds `defaults` for keys
/// still absent.
pub(super) fn fold(args: Option<&DslValue>, aliases: &[(&str, &str)], defaults: &[(&str, DslValue)]) -> DslValue {
    let mut entries: Vec<(String, DslValue)> = Vec::new();
    if let Some(DslValue::Object(object)) = args {
        for (key, value) in object {
            put(&mut entries, &snake(key), integral(value.clone()));
        }
    }
    for (from, into) in aliases {
        if entries.iter().any(|(key, _)| key == into) {
            continue;
        }
        if let Some((_, value)) = entries.iter().find(|(key, _)| key == from).cloned() {
            put(&mut entries, into, value);
        }
    }
    for (key, value) in defaults {
        if !entries.iter().any(|(existing, _)| existing == key) {
            entries.push(((*key).to_string(), value.clone()));
        }
    }
    DslValue::Object(entries)
}

/// 📷️ The viewport nests its pose under `camera` (`worldCameraSetCameraDispatchArgs`); a flat
/// `{position, target, …}` payload is admitted as the pose itself.
fn nest_camera(mut folded: DslValue) -> DslValue {
    if let DslValue::Object(entries) = &mut folded {
        if !entries.iter().any(|(key, _)| key == "camera") {
            let pose = DslValue::Object(entries.iter().filter(|(key, _)| matches!(key.as_str(), "position" | "target" | "zoom" | "fov" | "up" | "projection")).cloned().collect());
            entries.push(("camera".into(), pose));
        }
    }
    folded
}

fn with_text_value(mut folded: DslValue) -> DslValue {
    if let DslValue::Object(entries) = &mut folded {
        if let Some(slot) = entries.iter_mut().find(|(key, _)| key == "value") {
            slot.1 = stringify(slot.1.clone());
        }
    }
    folded
}

pub(super) fn decode<T: semio_framework_value::FromValue>(action: &str, value: DslValue) -> Result<T, Fault> {
    T::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("shooting action '{action}' arguments do not decode: {error}")))
}

pub fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<ShootingCommand, Fault> {
    const IDS: &[(&str, &str)] = &[("ids", "asset_ids"), ("asset_id", "asset_ids")];
    const SHOT_IDS: &[(&str, &str)] = &[("ids", "shot_ids"), ("shot_id", "shot_ids")];
    const PRESSED: &[(&str, &str)] = &[("pressed", "value")];
    let zero = || DslValue::Number(semio_framework_value::Number::Float(0.0));
    let one = || DslValue::Number(semio_framework_value::Number::Float(1.0));
    let text = |value: &str| DslValue::String(value.into());
    let plain = || fold(args, &[], &[]);
    Ok(match action {
        "importSnapshotJson" => ShootingCommand::ImportSnapshotJson(decode(action, fold(args, &[("value", "json"), ("document", "json")], &[]))?),
        "setActiveExample" => ShootingCommand::SetActiveExample(decode(action, fold(args, &[("id", "example_id"), ("value", "example_id")], &[("example_id", text(SHOOTING_EXAMPLE_DEFAULT_ID))]))?),
        "setActiveShot" => ShootingCommand::SetActiveShot(decode(action, fold(args, &[("value", "shot_id"), ("id", "shot_id")], &[]))?),
        "setActiveAsset" => ShootingCommand::SetActiveAsset(decode(action, fold(args, &[("value", "asset_id"), ("id", "asset_id")], &[]))?),
        "setShotCamera" => ShootingCommand::SetShotCamera(decode(action, nest_camera(fold(args, &[("id", "shot_id")], &[])))?),
        "saveCamera" => ShootingCommand::SaveCamera(decode(action, fold(args, &[("value", "label")], &[("label", text(""))]))?),
        "setSunAzimuth" => ShootingCommand::SetSunAzimuth(decode(action, plain())?),
        "setSunElevation" => ShootingCommand::SetSunElevation(decode(action, plain())?),
        "setSunIntensity" => ShootingCommand::SetSunIntensity(decode(action, plain())?),
        "setAmbientIntensity" => ShootingCommand::SetAmbientIntensity(decode(action, plain())?),
        "setMaterialRoughness" => ShootingCommand::SetMaterialRoughness(decode(action, plain())?),
        "setShadowEnabled" => ShootingCommand::SetShadowEnabled(decode(action, fold(args, PRESSED, &[]))?),
        "toggleSun" => ShootingCommand::ToggleSun(decode(action, fold(args, PRESSED, &[]))?),
        "setActiveShotLabel" => ShootingCommand::SetActiveShotLabel(decode(action, with_text_value(plain()))?),
        "setActiveShotFormat" => ShootingCommand::SetActiveShotFormat(decode(action, with_text_value(plain()))?),
        "setActiveShotShape" => ShootingCommand::SetActiveShotShape(decode(action, with_text_value(plain()))?),
        "patchShots" => ShootingCommand::PatchShots(decode(action, with_text_value(fold(args, SHOT_IDS, &[])))?),
        "patchAssets" => ShootingCommand::PatchAssets(decode(action, with_text_value(fold(args, IDS, &[])))?),
        "addShot" => ShootingCommand::AddShot(decode(action, fold(args, &[], &[("format", text("png")), ("shape", text("rectangle"))]))?),
        "addAsset" => ShootingCommand::AddAsset(decode(action, fold(args, &[], &[("format", text("glb"))]))?),
        "importAsset" => ShootingCommand::ImportAsset(decode(action, fold(args, &[("value", "payload")], &[]))?),
        "resetSnapshot" => ShootingCommand::ResetSnapshot(decode(action, plain())?),
        "translateSelection" => ShootingCommand::TranslateSelection(decode(action, fold(args, IDS, &[("asset_ids", DslValue::Array(Vec::new())), ("dx", zero()), ("dy", zero()), ("dz", zero())]))?),
        "rotateSelection" => ShootingCommand::RotateSelection(decode(action, fold(args, IDS, &[("asset_ids", DslValue::Array(Vec::new())), ("ax", zero()), ("ay", zero()), ("az", one()), ("angle", zero())]))?),
        "scaleSelection" => ShootingCommand::ScaleSelection(decode(action, fold(args, IDS, &[("asset_ids", DslValue::Array(Vec::new())), ("sx", one()), ("sy", one()), ("sz", one())]))?),
        "setCamera" => ShootingCommand::SetCamera(decode(action, nest_camera(plain()))?),
        "loadSavedCamera" => ShootingCommand::LoadSavedCamera(decode(action, fold(args, &[("value", "id"), ("camera_id", "id")], &[]))?),
        "setCenterModel" => ShootingCommand::SetCenterModel(decode(action, fold(args, &[("value", "pressed")], &[]))?),
        "setShotSelection" => ShootingCommand::SetShotSelection(decode(action, fold(args, SHOT_IDS, &[("shot_ids", DslValue::Array(Vec::new()))]))?),
        "worldPointerDown" => ShootingCommand::WorldPointerDown(decode(action, plain())?),
        "worldPointerMove" => ShootingCommand::WorldPointerMove(decode(action, plain())?),
        "saveDownload" => ShootingCommand::SaveDownload(decode(action, plain())?),
        "importAssetRequest" => ShootingCommand::ImportAssetRequest(decode(action, plain())?),
        "exportActiveShot" => ShootingCommand::ExportActiveShot(decode(action, plain())?),
        "exportAllShots" => ShootingCommand::ExportAllShots(decode(action, plain())?),
        _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the shooting editor has no command for action '{action}'"))),
    })
}
//#endregion 🔖️ActionBridge
