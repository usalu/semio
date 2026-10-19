//! 🌉️ Lowpoly action bridge: folds the host `{action, args}` vocabulary into the snake_case `FromValue` payloads of the command nodes and builds the typed `LowpolyCommand` from an action name.


//#region 🔖️ActionBridge
use super::*;
use semio_framework_value::DslValue;
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

/// 🔢️ The host's JSON round trip turns every integer into `Number::Float`; integer codecs decode exact
/// integers only, so whole finite floats get their integer variant back.
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

/// 🔁️ Snake-cases every key, applies `aliases` (source → destination, never overwriting a present
/// destination) and seeds `defaults` for keys still absent.
pub(super) fn fold(args: Option<&DslValue>, aliases: &[(&str, &str)], defaults: &[(&str, DslValue)]) -> Vec<(String, DslValue)> {
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
    entries
}

/// 📝️ Moves the merged control `value` into the JSON-text `value_json` field the reducers re-parse.
pub(super) fn value_json(mut entries: Vec<(String, DslValue)>) -> Vec<(String, DslValue)> {
    if !entries.iter().any(|(key, _)| key == "value_json") {
        if let Some((_, value)) = entries.iter().find(|(key, _)| key == "value" || key == "pressed").cloned() {
            put(&mut entries, "value_json", DslValue::String(semio_framework_pack_json::to_json_string(&value)));
        }
    }
    entries.retain(|(key, _)| key != "value" && key != "pressed");
    entries
}

/// 📝️ Prints a control value into a `String` field (`"0.5"`, `"true"`, or the text itself).
fn text_value(mut entries: Vec<(String, DslValue)>) -> Vec<(String, DslValue)> {
    if let Some(slot) = entries.iter_mut().find(|(key, _)| key == "value") {
        slot.1 = match slot.1.clone() {
            DslValue::String(text) => DslValue::String(text),
            DslValue::Null => DslValue::String(String::new()),
            other => DslValue::String(semio_framework_pack_json::to_json_string(&other)),
        };
    }
    entries
}

/// 📷️ Accepts `{position, target, fov}` or the World3d host's `{windowId, camera: {position, target, zoom, up}}`.
pub(super) fn camera(mut entries: Vec<(String, DslValue)>) -> Vec<(String, DslValue)> {
    if let Some((_, DslValue::Object(pose))) = entries.iter().find(|(key, _)| key == "camera").cloned() {
        for (key, value) in pose {
            if matches!(key.as_str(), "position" | "target" | "fov") && !entries.iter().any(|(existing, _)| existing == &key) {
                entries.push((key, value));
            }
        }
    }
    entries.retain(|(key, _)| matches!(key.as_str(), "position" | "target" | "fov"));
    let float = |value: &DslValue| match value {
        DslValue::Number(semio_framework_value::Number::Float(v)) => DslValue::Number(semio_framework_value::Number::Float(*v)),
        DslValue::Number(semio_framework_value::Number::UInt(v)) => DslValue::Number(semio_framework_value::Number::Float(*v as f64)),
        DslValue::Number(semio_framework_value::Number::Int(v)) => DslValue::Number(semio_framework_value::Number::Float(*v as f64)),
        other => other.clone(),
    };
    for key in ["position", "target"] {
        if let Some(slot) = entries.iter_mut().find(|(existing, _)| existing == key) {
            if let DslValue::Array(items) = &slot.1 {
                slot.1 = DslValue::Array(items.iter().map(float).collect());
            }
        }
    }
    if let Some(slot) = entries.iter_mut().find(|(existing, _)| existing == "fov") {
        slot.1 = float(&slot.1);
    } else {
        entries.push(("fov".into(), DslValue::Number(semio_framework_value::Number::Float(50.0))));
    }
    entries
}

/// 🧲️ The World3d gumball sends `{mode, ids}` naming the host's own selection (string target ids);
/// the reducers read the live mesh-domain selection instead, so both are dropped.
fn gumball(mut entries: Vec<(String, DslValue)>) -> Vec<(String, DslValue)> {
    entries.retain(|(key, _)| key != "ids" && key != "mode");
    entries
}

fn decode<T: semio_framework_value::FromValue>(action: &str, entries: Vec<(String, DslValue)>) -> Result<T, Fault> {
    T::from_value(DslValue::Object(entries)).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("lowpoly action '{action}' arguments do not decode: {error}")))
}

pub fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<LowpolyCommand, Fault> {
    const OBJECT: &[(&str, &str)] = &[("id", "object_id"), ("value", "object_id")];
    let plain = || fold(args, &[], &[]);
    let none = || Vec::new();
    let zero = || DslValue::Number(semio_framework_value::Number::Float(0.0));
    Ok(match action {
        "addPrimitive" => LowpolyCommand::AddPrimitive(decode(action, fold(args, &[("value", "kind")], &[]))?),
        "patchObject" => LowpolyCommand::PatchObject(decode(action, value_json(fold(args, &[("id", "object_id")], &[])))?),
        "extrude" => LowpolyCommand::Extrude(decode(action, fold(args, &[("value", "extrude_distance"), ("distance", "extrude_distance")], &[]))?),
        "inset" => LowpolyCommand::Inset(decode(action, fold(args, &[("value", "inset_amount"), ("amount", "inset_amount")], &[]))?),
        "bevel" => LowpolyCommand::Bevel(decode(action, fold(args, &[("value", "bevel_amount"), ("amount", "bevel_amount"), ("segments", "bevel_segments")], &[]))?),
        "loopCut" => LowpolyCommand::LoopCut(decode(action, fold(args, &[("value", "loop_cuts"), ("cuts", "loop_cuts")], &[]))?),
        "subdivide" => LowpolyCommand::Subdivide(decode(action, none())?),
        "triangulate" => LowpolyCommand::Triangulate(decode(action, none())?),
        "mirror" => LowpolyCommand::Mirror(decode(action, fold(args, &[("value", "axis")], &[]))?),
        "decimate" => LowpolyCommand::Decimate(decode(action, fold(args, &[("value", "decimate_ratio"), ("ratio", "decimate_ratio")], &[]))?),
        "flipFaces" => LowpolyCommand::FlipFaces(decode(action, fold(args, &[("ids", "face_ids")], &[("face_ids", DslValue::Array(Vec::new()))]))?),
        "merge" => LowpolyCommand::Merge(decode(action, none())?),
        "dissolve" => LowpolyCommand::Dissolve(decode(action, none())?),
        "snap" => LowpolyCommand::Snap(decode(action, none())?),
        "toggleSmooth" => LowpolyCommand::ToggleSmooth(decode(action, none())?),
        "unwrapActive" => LowpolyCommand::UnwrapActive(decode(action, none())?),
        "markUvSeam" => LowpolyCommand::MarkUvSeam(decode(action, fold(args, &[("value", "seam"), ("ids", "edge_ids")], &[]))?),
        "clearSeam" => LowpolyCommand::ClearSeam(decode(action, none())?),
        "translateSelection" => LowpolyCommand::TranslateSelection(decode(action, gumball(fold(args, &[("x", "dx"), ("y", "dy"), ("z", "dz")], &[("dx", zero()), ("dy", zero()), ("dz", zero())])))?),
        "rotateSelection" => LowpolyCommand::RotateSelection(decode(action, gumball(fold(args, &[], &[("ax", zero()), ("ay", DslValue::Number(semio_framework_value::Number::Float(1.0))), ("az", zero()), ("angle", zero())])))?),
        "scaleSelection" => LowpolyCommand::ScaleSelection(decode(action, gumball(fold(args, &[("x", "sx"), ("y", "sy"), ("z", "sz")], &[("sx", DslValue::Number(semio_framework_value::Number::Float(1.0))), ("sy", DslValue::Number(semio_framework_value::Number::Float(1.0))), ("sz", DslValue::Number(semio_framework_value::Number::Float(1.0)))])))?),
        "addPaintLayer" => LowpolyCommand::AddPaintLayer(decode(action, plain())?),
        "paintFill" => LowpolyCommand::PaintFill(decode(action, plain())?),
        "fillBucket" => LowpolyCommand::FillBucket(decode(action, plain())?),
        "importSnapshotJson" => LowpolyCommand::ImportSnapshotJson(decode(action, fold(args, &[("value", "json")], &[]))?),
        "replaceSnapshotJson" => LowpolyCommand::ReplaceSnapshotJson(decode(action, fold(args, &[("value", "json")], &[]))?),
        "exportMesh" => LowpolyCommand::ExportMesh(decode(action, fold(args, &[("value", "format")], &[("format", DslValue::String("obj".into()))]))?),
        "loadMeshRequest" => LowpolyCommand::LoadMeshRequest(decode(action, none())?),
        "importMeshFile" => LowpolyCommand::ImportMeshFile(decode(action, fold(args, &[("filename", "name"), ("contents", "payload")], &[]))?),
        "deleteSelection" => LowpolyCommand::DeleteSelection(decode(action, none())?),
        "duplicateObject" => LowpolyCommand::DuplicateObject(decode(action, fold(args, OBJECT, &[]))?),
        "engagementSubmit" => LowpolyCommand::EngagementSubmit(decode(action, text_value(fold(args, &[("text", "value"), ("input", "value")], &[])))?),
        "setActiveObject" => LowpolyCommand::SetActiveObject(decode(action, fold(args, OBJECT, &[]))?),
        "setActivePaintLayer" => LowpolyCommand::SetActivePaintLayer(decode(action, fold(args, &[("index", "layer_index"), ("value", "layer_index")], &[]))?),
        "setUtilityParam" => LowpolyCommand::SetUtilityParam(decode(action, value_json(plain()))?),
        "engagementInput" => LowpolyCommand::EngagementInput(decode(action, text_value(fold(args, &[("text", "value"), ("input", "value")], &[("value", DslValue::String(String::new()))])))?),
        "toggleShowEdges" => LowpolyCommand::ToggleShowEdges(decode(action, none())?),
        "toggleSun" => LowpolyCommand::ToggleSun(decode(action, none())?),
        "setSunAzimuth" => LowpolyCommand::SetSunAzimuth(decode(action, fold(args, &[], &[("value", zero())]))?),
        "setSunElevation" => LowpolyCommand::SetSunElevation(decode(action, fold(args, &[], &[("value", zero())]))?),
        "setSunIntensity" => LowpolyCommand::SetSunIntensity(decode(action, fold(args, &[], &[("value", zero())]))?),
        "setCamera" => LowpolyCommand::SetCamera(decode(action, camera(plain()))?),
        "paintSample" => LowpolyCommand::PaintSample(decode(action, plain())?),
        "paintStroke" => LowpolyCommand::PaintStroke(decode(action, plain())?),
        "paintAt" => LowpolyCommand::PaintAt(decode(action, plain())?),
        "canvasPointerDown" => LowpolyCommand::CanvasPointerDown(decode(action, plain())?),
        "canvasPointerMove" => LowpolyCommand::CanvasPointerMove(decode(action, plain())?),
        "canvasPointerUp" => LowpolyCommand::CanvasPointerUp(decode(action, plain())?),
        _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the lowpoly editor has no command for action '{action}'"))),
    })
}
//#endregion 🔖️ActionBridge
