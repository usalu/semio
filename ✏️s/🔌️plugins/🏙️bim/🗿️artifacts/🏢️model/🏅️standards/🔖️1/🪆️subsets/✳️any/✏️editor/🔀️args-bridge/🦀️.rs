//! 🔀️ Host-action bridge of the BIM editor: folds the shell `{action, args}` vocabulary into the closed `BimCommand` enum. See [`super`].

use crate::editor::bim::panels::outliner as outliner_panel;
use semio_framework_plugin::DslValue;
use semio_framework_plugin::Fault;
//#region 🔖️ActionBridge
use crate::editor::bim::*;
use semio_framework_plugin::{FaultCode, FaultOrigin};

fn snake(key: &str) -> String {
    key.chars().flat_map(|ch| if ch.is_ascii_uppercase() { vec!['_', ch.to_ascii_lowercase()] } else { vec![ch] }).collect()
}

fn put(entries: &mut Vec<(String, DslValue)>, key: &str, value: DslValue) {
    entries.retain(|(existing, _)| existing != key);
    entries.push((key.to_string(), value));
}

pub(crate) fn integral(value: DslValue) -> DslValue {
    match value {
        DslValue::Number(semio_framework_value::Number::Float(float)) if float.is_finite() && float.fract() == 0.0 && float.abs() < 9.007_199_254_740_992e15 => {
            if float >= 0.0 { DslValue::Number(semio_framework_value::Number::UInt(float as u64)) } else { DslValue::Number(semio_framework_value::Number::Int(float as i64)) }
        }
        DslValue::Array(items) => DslValue::Array(items.into_iter().map(integral).collect()),
        DslValue::Object(entries) => DslValue::Object(entries.into_iter().map(|(key, value)| (key, integral(value))).collect()),
        other => other,
    }
}

pub(crate) fn stringify(value: DslValue) -> DslValue {
    match value {
        DslValue::String(_) => value,
        DslValue::Null => DslValue::String(String::new()),
        other => DslValue::String(semio_framework_pack_json::to_json_string(&other)),
    }
}

pub(crate) fn fold(args: Option<&DslValue>, aliases: &[(&str, &str)], defaults: &[(&str, DslValue)]) -> DslValue {
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

fn value_as_text(mut folded: DslValue) -> DslValue {
    if let DslValue::Object(entries) = &mut folded {
        if let Some(slot) = entries.iter_mut().find(|(key, _)| key == "value") {
            slot.1 = stringify(slot.1.clone());
        }
    }
    folded
}

fn text_field(mut folded: DslValue, key: &str) -> DslValue {
    if let DslValue::Object(entries) = &mut folded {
        if let Some(slot) = entries.iter_mut().find(|(name, _)| name == key) {
            slot.1 = stringify(slot.1.clone());
        }
    }
    folded
}

fn ids_as_list(mut folded: DslValue) -> DslValue {
    if let DslValue::Object(entries) = &mut folded {
        if let Some(slot) = entries.iter_mut().find(|(key, _)| key == "ids") {
            if let DslValue::String(single) = &slot.1 {
                slot.1 = DslValue::Array(vec![DslValue::String(single.clone())]);
            }
        }
    }
    folded
}

/// 🎥️ The host nests the pose under `camera` (or sends it flat); a pose with `position` is the world's orbit, one with `x` the canvas'.
pub(crate) fn camera_pose(args: Option<&DslValue>) -> DslValue {
    let folded = fold(args, &[], &[]);
    let DslValue::Object(entries) = &folded else { return folded };
    let pose = match entries.iter().find(|(key, _)| key == "camera") {
        Some((_, DslValue::Object(inner))) => inner.clone(),
        _ => entries.clone(),
    };
    let slot = if pose.iter().any(|(key, _)| key == "position") { "camera3d" } else { "camera2d" };
    let kept: Vec<(String, DslValue)> = pose.into_iter().filter(|(key, _)| matches!(key.as_str(), "x" | "y" | "zoom" | "position" | "target" | "up")).collect();
    DslValue::Object(vec![(slot.to_string(), DslValue::Object(kept))])
}

pub(crate) fn only(folded: DslValue, keys: &[&str]) -> DslValue {
    match folded {
        DslValue::Object(entries) => DslValue::Object(entries.into_iter().filter(|(key, _)| keys.contains(&key.as_str())).collect()),
        other => other,
    }
}

pub(crate) fn decode<T: semio_framework_value::FromValue>(action: &str, value: DslValue) -> Result<T, Fault> {
    T::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("bim action '{action}' arguments do not decode: {error}")))
}

pub fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<BimCommand, Fault> {
    let text = |value: &str| DslValue::String(value.into());
    Ok(match action {
        "createEntity" => BimCommand::CreateEntity(decode(action, fold(args, &[("type", "kind"), ("value", "kind"), ("parent_id", "parent")], &[("parent", text("")), ("name", text(""))]))?),
        "createView" => BimCommand::CreateView(decode(action, fold(args, &[("type", "kind"), ("value", "kind"), ("parent_id", "parent")], &[("parent", text("")), ("name", text(""))]))?),
        "deleteSelection" => BimCommand::DeleteSelection(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new()))])))?),
        "renameEntity" => BimCommand::RenameEntity(decode(action, fold(args, &[("value", "name")], &[]))?),
        "setField" => BimCommand::SetField(decode(action, ids_as_list(value_as_text(fold(args, &[("id", "ids"), (outliner_panel::DRAG_ELEMENT_MIME, "ids")], &[("ids", DslValue::Array(Vec::new()))]))))?),
        "setView" => BimCommand::SetView(decode(action, value_as_text(fold(args, &[], &[("value", text(""))])))?),
        "editFamily" => BimCommand::EditFamily(decode(action, fold(args, &[], &[("key", text("")), ("value", text(""))]))?),
        "editSchedule" => BimCommand::EditSchedule(decode(action, fold(args, &[], &[("key", text("")), ("value", text(""))]))?),
        "exportScheduleCsv" => BimCommand::ExportScheduleCsv(decode(action, only(fold(args, &[], &[("id", text(""))]), &["id", "pressed"]))?),
        "analyseModel" => BimCommand::AnalyseModel(decode(action, only(fold(args, &[], &[]), &["pressed"]))?),
        "exportModel" => BimCommand::ExportModel(decode(action, only(fold(args, &[], &[("format", text("ifc2x3"))]), &["format", "pressed"]))?),
        "exportSheets" => BimCommand::ExportSheets(decode(action, only(fold(args, &[], &[("format", text("pdf"))]), &["format", "sheet", "locale", "pressed"]))?),
        "setCamera" => BimCommand::SetCamera(decode(action, camera_pose(args))?),
        "canvasPointerDown" => BimCommand::CanvasPointerDown(decode(action, fold(args, &[], &[]))?),
        "canvasPointerMove" => BimCommand::CanvasPointerMove(decode(action, fold(args, &[], &[]))?),
        "canvasPointerUp" => BimCommand::CanvasPointerUp(decode(action, fold(args, &[], &[]))?),
        "canvasDoubleClick" => BimCommand::CanvasDoubleClick(decode(action, fold(args, &[], &[]))?),
        "canvasCommitDraft" => BimCommand::CanvasCommitDraft(decode(action, fold(args, &[], &[]))?),
        "canvasEscape" => BimCommand::CanvasEscape(decode(action, fold(args, &[], &[]))?),
        "worldPointerDown" => BimCommand::WorldPointerDown(decode(action, fold(args, &[], &[]))?),
        "worldPointerMove" => BimCommand::WorldPointerMove(decode(action, fold(args, &[], &[]))?),
        "armSelect" => BimCommand::ArmSelect(decode(action, fold(args, &[], &[]))?),
        "armWall" => BimCommand::ArmWall(decode(action, fold(args, &[], &[]))?),
        "armWallArc" => BimCommand::ArmWallArc(decode(action, fold(args, &[], &[]))?),
        "armCurtainWall" => BimCommand::ArmCurtainWall(decode(action, fold(args, &[], &[]))?),
        "armColumn" => BimCommand::ArmColumn(decode(action, fold(args, &[], &[]))?),
        "armBeam" => BimCommand::ArmBeam(decode(action, fold(args, &[], &[]))?),
        "armBeamArc" => BimCommand::ArmBeamArc(decode(action, fold(args, &[], &[]))?),
        "armColumnTilt" => BimCommand::ArmColumnTilt(decode(action, fold(args, &[], &[]))?),
        "armCurtainGrid" => BimCommand::ArmCurtainGrid(decode(action, fold(args, &[], &[]))?),
        "armCurtainCell" => BimCommand::ArmCurtainCell(decode(action, fold(args, &[], &[]))?),
        "armSlab" => BimCommand::ArmSlab(decode(action, fold(args, &[], &[]))?),
        "armRoof" => BimCommand::ArmRoof(decode(action, fold(args, &[], &[]))?),
        "armWindow" => BimCommand::ArmWindow(decode(action, fold(args, &[], &[]))?),
        "armDoor" => BimCommand::ArmDoor(decode(action, fold(args, &[], &[]))?),
        "armOpening" => BimCommand::ArmOpening(decode(action, fold(args, &[], &[]))?),
        "armStair" => BimCommand::ArmStair(decode(action, fold(args, &[], &[]))?),
        "armRailing" => BimCommand::ArmRailing(decode(action, fold(args, &[], &[]))?),
        "armRamp" => BimCommand::ArmRamp(decode(action, fold(args, &[], &[]))?),
        "armComponent" => BimCommand::ArmComponent(decode(action, fold(args, &[], &[]))?),
        "armRoute" => BimCommand::ArmRoute(decode(action, fold(args, &[], &[]))?),
        "placeComponent" => BimCommand::PlaceComponent(decode(action, fold(args, &[("value", "family"), ("id", "family")], &[]))?),
        "openFamily" => BimCommand::OpenFamily(decode(action, fold(args, &[("value", "family"), ("id", "family")], &[]))?),
        "searchFamilies" => BimCommand::SearchFamilies(decode(action, text_field(fold(args, &[("value", "query")], &[("query", text("")), ("category", text(""))]), "query"))?),
        "setOverride" => BimCommand::SetOverride(decode(action, value_as_text(fold(args, &[("id", "component")], &[("value", text(""))])))?),
        "gestureTurn" => BimCommand::GestureTurn(decode(action, fold(args, &[], &[]))?),
        "gestureTurnBack" => BimCommand::GestureTurnBack(decode(action, fold(args, &[], &[]))?),
        "gestureQuarter" => BimCommand::GestureQuarter(decode(action, fold(args, &[], &[]))?),
        "gestureMirror" => BimCommand::GestureMirror(decode(action, fold(args, &[], &[]))?),
        "gestureNext" => BimCommand::GestureNext(decode(action, fold(args, &[], &[]))?),
        "gesturePrevious" => BimCommand::GesturePrevious(decode(action, fold(args, &[], &[]))?),
        "gestureRaise" => BimCommand::GestureRaise(decode(action, fold(args, &[], &[]))?),
        "gestureLower" => BimCommand::GestureLower(decode(action, fold(args, &[], &[]))?),
        "gestureSystem" => BimCommand::GestureSystem(decode(action, fold(args, &[], &[]))?),
        "armSpace" => BimCommand::ArmSpace(decode(action, fold(args, &[], &[]))?),
        "armGrid" => BimCommand::ArmGrid(decode(action, fold(args, &[], &[]))?),
        "armMeasure" => BimCommand::ArmMeasure(decode(action, fold(args, &[], &[]))?),
        "armMove" => BimCommand::ArmMove(decode(action, fold(args, &[], &[]))?),
        "armRotate" => BimCommand::ArmRotate(decode(action, fold(args, &[], &[]))?),
        "armSlabWalls" => BimCommand::ArmSlabWalls(decode(action, fold(args, &[], &[]))?),
        "armCeiling" => BimCommand::ArmCeiling(decode(action, fold(args, &[], &[]))?),
        "armCeilingSpace" => BimCommand::ArmCeilingSpace(decode(action, fold(args, &[], &[]))?),
        "armSplitWall" => BimCommand::ArmSplitWall(decode(action, fold(args, &[], &[]))?),
        "armCopy" => BimCommand::ArmCopy(decode(action, fold(args, &[], &[]))?),
        "armMirror" => BimCommand::ArmMirror(decode(action, fold(args, &[], &[]))?),
        "armArray" => BimCommand::ArmArray(decode(action, fold(args, &[], &[]))?),
        "armArrayRadial" => BimCommand::ArmArrayRadial(decode(action, fold(args, &[], &[]))?),
        "armOffset" => BimCommand::ArmOffset(decode(action, fold(args, &[], &[]))?),
        "armTrim" => BimCommand::ArmTrim(decode(action, fold(args, &[], &[]))?),
        "armExtend" => BimCommand::ArmExtend(decode(action, fold(args, &[], &[]))?),
        "armAlign" => BimCommand::ArmAlign(decode(action, fold(args, &[], &[]))?),
        "armSplit" => BimCommand::ArmSplit(decode(action, fold(args, &[], &[]))?),
        "armDimension" => BimCommand::ArmDimension(decode(action, fold(args, &[], &[]))?),
        "armTag" => BimCommand::ArmTag(decode(action, fold(args, &[], &[]))?),
        "armTextNote" => BimCommand::ArmTextNote(decode(action, fold(args, &[], &[]))?),
        "armLeader" => BimCommand::ArmLeader(decode(action, fold(args, &[], &[]))?),
        "armViewport" => BimCommand::ArmViewport(decode(action, fold(args, &[], &[]))?),
        "armSweep" => BimCommand::ArmSweep(decode(action, fold(args, &[], &[]))?),
        "placeGridColumns" => BimCommand::PlaceGridColumns(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new())), ("storey", text(""))])))?),
        "flipWalls" => BimCommand::FlipWalls(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new()))])))?),
        "attachWalls" => BimCommand::AttachWalls(decode(action, ids_as_list(fold(args, &[("id", "ids"), ("value", "target")], &[("ids", DslValue::Array(Vec::new())), ("target", text(""))])))?),
        "storeyUp" => BimCommand::StoreyUp(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new()))])))?),
        "storeyDown" => BimCommand::StoreyDown(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new()))])))?),
        "splitWall" => BimCommand::SplitWallAt(decode(action, text_field(ids_as_list(fold(args, &[("id", "ids"), ("value", "at")], &[("ids", DslValue::Array(Vec::new())), ("at", text(""))])), "at"))?),
        "setProperty" => BimCommand::SetProperty(decode(action, value_as_text(ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new())), ("pset", text("")), ("property", text("")), ("value", text(""))]))))?),
        "removeProperty" => BimCommand::RemoveProperty(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new())), ("pset", text("")), ("property", text(""))])))?),
        "setClassification" => BimCommand::SetClassification(decode(action, ids_as_list(fold(args, &[("id", "ids"), ("value", "code")], &[("ids", DslValue::Array(Vec::new())), ("system", text("")), ("code", text(""))])))?),
        "removeClassification" => BimCommand::RemoveClassification(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new())), ("system", text(""))])))?),
        "applyTemplate" => BimCommand::ApplyTemplate(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new())), ("template", text(""))])))?),
        "editTemplate" => BimCommand::EditTemplate(decode(action, fold(args, &[], &[("index", text("")), ("field", text("")), ("value", text(""))]))?),
        "editClassification" => BimCommand::EditClassification(decode(action, fold(args, &[("value", "title")], &[("code", text("")), ("title", text("")), ("parent", text(""))]))?),
        "searchClassification" => BimCommand::SearchClassification(decode(action, fold(args, &[("value", "query")], &[("query", text(""))]))?),
        "placeElements" => BimCommand::PlaceAt(decode(action, text_field(ids_as_list(fold(args, &[("id", "ids"), ("value", "at")], &[("ids", DslValue::Array(Vec::new())), ("at", text(""))])), "at"))?),
        "selectFindings" => BimCommand::SelectFindings(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new()))])))?),
        "viewClash" => BimCommand::ViewClash(decode(action, only(fold(args, &[], &[("first", text("")), ("second", text("")), ("mode", text("zoom"))]), &["first", "second", "mode"]))?),
        "raiseIssue" => BimCommand::RaiseIssue(decode(action, only(fold(args, &[], &[("set", text("")), ("first", text("")), ("second", text(""))]), &["set", "first", "second"]))?),
        "captureViewpoint" => BimCommand::CaptureViewpoint(decode(action, only(fold(args, &[], &[("issue", text(""))]), &["issue"]))?),
        "restoreViewpoint" => BimCommand::RestoreViewpoint(decode(action, only(fold(args, &[], &[("issue", text(""))]), &["issue"]))?),
        "cursorLeft" => BimCommand::CursorLeft(decode(action, fold(args, &[], &[]))?),
        "cursorRight" => BimCommand::CursorRight(decode(action, fold(args, &[], &[]))?),
        "cursorUp" => BimCommand::CursorUp(decode(action, fold(args, &[], &[]))?),
        "cursorDown" => BimCommand::CursorDown(decode(action, fold(args, &[], &[]))?),
        "cursorLeftFar" => BimCommand::CursorLeftFar(decode(action, fold(args, &[], &[]))?),
        "cursorRightFar" => BimCommand::CursorRightFar(decode(action, fold(args, &[], &[]))?),
        "cursorUpFar" => BimCommand::CursorUpFar(decode(action, fold(args, &[], &[]))?),
        "cursorDownFar" => BimCommand::CursorDownFar(decode(action, fold(args, &[], &[]))?),
        "cursorPlace" => BimCommand::CursorPlace(decode(action, fold(args, &[], &[]))?),
        "engagementInput" => BimCommand::EngagementInput(decode(action, value_as_text(fold(args, &[], &[("value", text(""))])))?),
        "engagementSubmit" => BimCommand::EngagementSubmit(decode(action, value_as_text(fold(args, &[], &[("value", text(""))])))?),
        "applyConditions" => BimCommand::ApplyConditions(decode(action, ids_as_list(fold(args, &[("id", "ids"), ("value", "source")], &[("ids", DslValue::Array(Vec::new())), ("source", text(""))])))?),
        "clearConditions" => BimCommand::ClearConditions(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new()))])))?),
        _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the bim editor has no command for action '{action}'"))),
    })
}
//#endregion 🔖️ActionBridge
