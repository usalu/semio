//! ⌨️ Action decoding of the CAD play app: the flag value reader and the command from action mapping. See [`super`].

use super::*;

//#region 🔖️Commands
/// 🙈️ A reference set-verb (`setReferenceHidden{hidden}`, `setReferenceLocked{locked}`) sets exactly the boolean its
/// arguments carry — the row target's explicit next state — so a missing value is refused, never defaulted into a flip.
fn cad_flag_value_required(action: &str, flag: &str) -> Fault {
    Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("cad.action.flag-value-required"), format!("action '{action}' requires the boolean '{flag}' it sets"))
}

/// 🌉️ Converts the host shell's declared action id and JSON arguments into cad's closed typed
/// command vocabulary before the app dispatches through the binary command path.
pub(super) fn cad_command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<CadCommand, Fault> {
    let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_str).map(str::to_string);
    let f64_field = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_f64);
    let bool_field = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_bool);
    let str_vec_field = |key: &str| -> Vec<String> { args.and_then(|value| value.get(key)).and_then(|value| semio_framework_value::FromValue::from_value(value.clone()).ok()).unwrap_or_default() };
    let value_string = || -> Option<String> {
        args.and_then(|value| value.get("value")).and_then(|value| match value {
            semio_framework_value::DslValue::String(text) => Some(text.clone()),
            semio_framework_value::DslValue::Bool(flag) => Some(flag.to_string()),
            semio_framework_value::DslValue::Number(number) => Some(match number {
                semio_framework_value::Number::UInt(number) => number.to_string(),
                semio_framework_value::Number::Int(number) => number.to_string(),
                semio_framework_value::Number::Float(number) => number.to_string(),
            }),
            _ => None,
        })
    };
    let position_axis = |index: usize| args.and_then(|value| value.get("position")).and_then(semio_framework_value::DslValue::as_array).and_then(|array| array.get(index)).and_then(semio_framework_value::DslValue::as_f64);
    Ok(match action {
        "setActiveExample" => CadCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_field("exampleId").unwrap_or_default() }),
        "setDislocateOption" => CadCommand::SetDislocateOption(set_dislocate_option::SetDislocateOption { pane: str_field("pane"), option: str_field("option").unwrap_or_default(), pressed: bool_field("pressed") }),
        "setNodeSelection" => CadCommand::SetNodeSelection(set_node_selection::SetNodeSelection { node_ids: str_vec_field("nodeIds") }),
        "setCamera" => CadCommand::SetCamera(set_camera::SetCamera { pane: str_field("surfaceId"), camera: args.and_then(|value| value.get("camera")).and_then(|value| semio_framework_value::FromValue::from_value(value.clone()).ok()).unwrap_or_default() }),
        "setProjection" => CadCommand::SetProjection(set_projection::SetProjection {
            pane: str_field("surfaceId"),
            field: str_field("field"),
            value_str: args.and_then(|value| value.get("value")).and_then(semio_framework_value::DslValue::as_str).map(String::from),
            value_num: args.and_then(|value| value.get("value")).and_then(semio_framework_value::DslValue::as_f64),
            param: str_field("param"),
        }),
        "setProjectionParam" => CadCommand::SetProjectionParam(set_projection_param::SetProjectionParam {
            pane: str_field("surfaceId"),
            field: str_field("field"),
            value_str: args.and_then(|value| value.get("value")).and_then(semio_framework_value::DslValue::as_str).map(String::from),
            value_num: args.and_then(|value| value.get("value")).and_then(semio_framework_value::DslValue::as_f64),
            param: str_field("param"),
        }),
        "translateSelection" => {
            CadCommand::TranslateSelection(translate_selection::TranslateSelection { object_ids: str_vec_field("objectIds"), dx: f64_field("dx").unwrap_or(0.0), dy: f64_field("dy").unwrap_or(0.0), dz: f64_field("dz").unwrap_or(0.0) })
        }
        "rotateSelection" => CadCommand::RotateSelection(rotate_selection::RotateSelection {
            object_ids: str_vec_field("objectIds"),
            ax: f64_field("ax").unwrap_or(0.0),
            ay: f64_field("ay").unwrap_or(0.0),
            az: f64_field("az").unwrap_or(0.0),
            angle: f64_field("angle").unwrap_or(0.0),
        }),
        "scaleSelection" => CadCommand::ScaleSelection(scale_selection::ScaleSelection { object_ids: str_vec_field("objectIds"), sx: f64_field("sx").unwrap_or(1.0), sy: f64_field("sy").unwrap_or(1.0), sz: f64_field("sz").unwrap_or(1.0) }),
        "addObject" => CadCommand::AddObject(add_object::AddObject { typology: str_field("typology") }),
        "patchObject" => CadCommand::PatchObject(patch_object::PatchObject { object_id: str_field("objectId").unwrap_or_default(), field: str_field("field").unwrap_or_default(), value: value_string(), delta: f64_field("delta") }),
        "patchSelection" => CadCommand::PatchSelection(patch_selection::PatchSelection { object_ids: str_vec_field("objectIds"), field: str_field("field").unwrap_or_default(), value: value_string(), delta: f64_field("delta") }),
        "deleteObject" => CadCommand::DeleteObject(delete_object::DeleteObject { object_id: str_field("objectId").unwrap_or_default() }),
        "duplicateObject" => CadCommand::DuplicateObject(duplicate_object::DuplicateObject { object_id: str_field("objectId").unwrap_or_default() }),
        "addNode" => CadCommand::AddNode(add_node::AddNode { kind: str_field("kind").unwrap_or_else(|| "solid".into()) }),
        "renameNode" => CadCommand::RenameNode(rename_node::RenameNode { node_id: str_field("nodeId").unwrap_or_default(), value: str_field("value").unwrap_or_default() }),
        "applyTransformation" => CadCommand::ApplyTransformation(apply_transformation::ApplyTransformation { qid: str_field("qid").unwrap_or_default() }),
        "saveSelected" => CadCommand::SaveSelected(save_selected::SaveSelected {}),
        "saveInPlay" => CadCommand::SaveInPlay(save_in_play::SaveInPlay {}),
        "saveCurrent" => CadCommand::SaveCurrent(save_current::SaveCurrent { format: str_field("format") }),
        "loadRawRequest" => CadCommand::LoadRawRequest(load_raw_request::LoadRawRequest {}),
        "importCadFile" => {
            let payload = args.and_then(|value| value.get("payload").or_else(|| value.get("modelSpace"))).cloned().or_else(|| args.cloned());
            let payload = match payload {
                Some(semio_framework_value::DslValue::String(text)) => text,
                Some(other) => semio_framework_pack_json::to_json_string(&other),
                None => String::new(),
            };
            CadCommand::ImportCadFile(import_cad_file::ImportCadFile { name: str_field("name").unwrap_or_default(), payload })
        }
        "setReferenceSelection" => CadCommand::SetReferenceSelection(set_reference_selection::SetReferenceSelection { pane: str_field("pane"), model_definition_id: str_field("modelDefinitionId"), reference_id: str_field("referenceId") }),
        "referenceHover" => CadCommand::ReferenceHover(reference_hover::ReferenceHover { reference_id: str_field("referenceId") }),
        "patchCadPlayReference" => CadCommand::PatchCadPlayReference(patch_cad_play_reference::PatchCadPlayReference {
            model_definition_id: str_field("modelDefinitionId").unwrap_or_default(),
            reference_id: str_field("referenceId").unwrap_or_default(),
            field: str_field("field").unwrap_or_default(),
            value: value_string(),
            delta: f64_field("delta"),
        }),
        "setReferenceHidden" => CadCommand::SetReferenceHidden(set_reference_hidden::SetReferenceHidden {
            model_definition_id: str_field("modelDefinitionId").unwrap_or_default(),
            reference_id: str_field("referenceId").unwrap_or_default(),
            hidden: bool_field("hidden").ok_or_else(|| cad_flag_value_required(action, "hidden"))?,
        }),
        "setReferenceLocked" => CadCommand::SetReferenceLocked(set_reference_locked::SetReferenceLocked {
            model_definition_id: str_field("modelDefinitionId").unwrap_or_default(),
            reference_id: str_field("referenceId").unwrap_or_default(),
            locked: bool_field("locked").ok_or_else(|| cad_flag_value_required(action, "locked"))?,
        }),
        "engagementInput" => CadCommand::EngagementInput(engagement_input::EngagementInput { value: str_field("value").unwrap_or_default(), pane: str_field("pane") }),
        "engagementSubmit" => CadCommand::EngagementSubmit(engagement_submit::EngagementSubmit { pane: str_field("pane") }),
        "engagementPossibleSelect" => CadCommand::EngagementPossibleSelect(engagement_possible_select::EngagementPossibleSelect { pane: str_field("pane"), possible_id: str_field("possibleId").unwrap_or_default() }),
        "engagementRepeatLast" => CadCommand::EngagementRepeatLast(engagement_repeat_last::EngagementRepeatLast { pane: str_field("pane") }),
        "engagementAbort" => CadCommand::EngagementAbort(engagement_abort::EngagementAbort {}),
        "worldPointerDown" => CadCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { pane: str_field("pane"), surface_id: str_field("surfaceId"), x: position_axis(0), y: position_axis(1), z: position_axis(2) }),
        "worldPointerMove" => CadCommand::WorldPointerMove(world_pointer_move::WorldPointerMove { x: position_axis(0), y: position_axis(1), z: position_axis(2) }),
        "toggleSun" => CadCommand::ToggleSun(toggle_sun::ToggleSun {}),
        "setSunAzimuth" => CadCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: f64_field("value").unwrap_or(0.0) }),
        "setSunElevation" => CadCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: f64_field("value").unwrap_or(0.0) }),
        "setSunIntensity" => CadCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: f64_field("value").unwrap_or(0.0) }),
        "setContributions" => CadCommand::SetContributions(set_contributions::SetContributions { json: str_field("json").unwrap_or_else(|| "[]".into()) }),
        other => return Err(Fault::from(format!("unknown cad action '{other}'"))),
    })
}
//#endregion 🔖️Commands
