//! ⌨️ Command arguments and dispatch of the norm app surface: the argument readers and the dispatch helpers. See [`super`].

use super::*;

//#region 🔖️Commands
/// 📤️ Commits a bundle of targeted semantic mutations as one edit: a value-tree edit resolves to its one kind via [`NormEditRules::resolve`], bundled here
/// into a single undo entry whose history row is labelled by its leaves' `SemanticMutation::label` (design §20.4, §20.6).
pub fn commit_snapshot_fields<M>(mutations: Vec<M>) -> Result<Emit<M, NoConfigMutation>, Fault> {
    Ok(Emit::mutations(mutations))
}

/// 🎯️ Builds the args-side of an app's `command_from_action` bridge for `selected-check` — the shells
/// still speak `{action,args}` for chrome actions.
pub fn selected_check_index_arg(args: Option<&semio_framework_value::DslValue>) -> Option<u32> {
    args.and_then(|value| value.get("index")).and_then(semio_framework_value::DslValue::as_u64).map(|value| value as u32)
}

/// 🎯️ Path argument used by `setField` / `insertItem` / `removeItem`.
pub fn path_arg(args: Option<&semio_framework_value::DslValue>) -> String {
    args.and_then(|value| value.get("path"))
        .and_then(|value| if let semio_framework_value::DslValue::String(raw) = value { Some(raw.clone()) } else { Some(semio_framework_pack_json::to_json_string(value)) })
        .unwrap_or_default()
}

/// 🎯️ Index argument used by `insertItem` / `removeItem` / `applyRemedy.remedyIndex`.
pub fn index_arg(args: Option<&semio_framework_value::DslValue>, key: &str) -> usize {
    args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_u64).map(|value| value as usize).unwrap_or(0)
}

/// 🎯️ Optional value argument — host control merges under `value`; absent ⇒ JSON null.
pub fn value_arg_json(args: Option<&semio_framework_value::DslValue>) -> String {
    match args.and_then(|value| value.get("value")) {
        Some(value) => semio_framework_pack_json::to_json_string(value),
        None => "null".into(),
    }
}

/// 🎯️ `applyRemedy.checkId` (also accepts `check_id`).
pub fn check_id_arg(args: Option<&semio_framework_value::DslValue>) -> String {
    args.and_then(|value| value.get("checkId").or_else(|| value.get("check_id")))
        .and_then(|value| if let semio_framework_value::DslValue::String(raw) = value { Some(raw.clone()) } else { Some(semio_framework_pack_json::to_json_string(value)) })
        .unwrap_or_default()
}

fn dsl_value_from_json(json: &str) -> Result<semio_framework_value::DslValue, String> {
    semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// ✏️ Shared `setField` handler body used by every family's command leaf.
pub fn dispatch_set_field<D, M, F>(document: &D, path: &str, value_json: &str, resolve: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    F: FnOnce(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    let value = dsl_value_from_json(value_json).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.set-field-value"), error))?;
    handle_set_field(document, path, value, resolve)
}

/// ➕ Shared `insertItem` handler body.
pub fn dispatch_insert_item<D, M, F>(document: &D, path: &str, index: usize, value_json: Option<&str>, resolve: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    F: FnOnce(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    let value = match value_json {
        Some(json) if json != "null" => Some(dsl_value_from_json(json).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("norm.insert-item-value"), error))?),
        _ => None,
    };
    handle_insert_item(document, path, index, value, resolve)
}

/// ➖ Shared `removeItem` handler body.
pub fn dispatch_remove_item<D, M, F>(document: &D, path: &str, index: usize, resolve: F) -> Result<Emit<M, NoConfigMutation>, Fault>
where
    F: FnOnce(&D, &NormEdit) -> Result<Vec<M>, String>,
{
    handle_remove_item(document, path, index, resolve)
}

/// 🩹 Shared `applyRemedy` handler body.
pub fn dispatch_apply_remedy<Fam, Map>(document: &Fam::Document, check_id: &str, remedy_index: usize, resolve: Map) -> Result<Emit<Fam::Mutation, NoConfigMutation>, Fault>
where
    Fam: NormFamily,
    Fam::Document: Clone + semio_framework_value::ToValue,
    Map: FnOnce(&Fam::Document, &NormEdit) -> Result<Vec<Fam::Mutation>, String>,
{
    handle_apply_remedy::<Fam, Map>(document, check_id, remedy_index, resolve)
}
//#endregion 🔖️Commands

//#region 🫧️LiveCommands
/// 🧾️ The live-editing tool ids, appended to an app's `app_commands!` rows after [`NORM_RETAINED_TOOL_IDS`] (append only, no reordering).
pub const NORM_LIVE_TOOL_IDS: &[&str] = &["setFields", "previewRemedy", "clearPreview", "cancelEvaluation", "evaluateUnits", "viewportAct", "copyEntities", "duplicateEntities", "pasteEntities", "moveEntity", "deleteEntities"];

/// 🧱️ Largest number of field writes one `setFields` may carry; bigger edits are structural and go through `insertItem`.
pub const NORM_LIVE_MAX_WRITES: usize = 64;

/// 🧭️ Why an evaluation was requested; drives scheduling priority only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvalReason {
    Edit,
    Commit,
    Remedy,
    Manual,
}

/// 🫧️ One parsed live-editing command: `setFields` is the only one that touches the document, all others stay in the ephemeral lanes.
#[derive(Clone, Debug, PartialEq)]
pub enum LiveCommand {
    SetFields { writes: Vec<FieldWrite> },
    PreviewRemedy { check_id: String, index: usize, option: usize },
    PreviewInsert { spec: Option<String>, at: Option<[f64; 3]>, free: bool },
    ClearPreview,
    CancelEvaluation,
    EvaluateUnits { units: Option<Vec<String>>, reason: EvalReason },
    CopyEntities { collection: String, ids: Vec<String> },
    DuplicateEntities { collection: String, ids: Vec<String> },
    PasteEntities { clip_json: String, at: Option<usize> },
    MoveEntity { path: String, id: String, target: MoveTarget },
    DeleteEntities { collection: String, ids: Vec<String> },
}

/// ↕️ Where a reordered row goes: to a position, onto the row it was dropped on, or one step up or down (the keyboard, touch and screen-reader alternative to the drag).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MoveTarget {
    To(usize),
    Over(String),
    Step(isize),
}

impl EvalReason {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "edit" => Some(Self::Edit),
            "commit" => Some(Self::Commit),
            "remedy" => Some(Self::Remedy),
            "manual" => Some(Self::Manual),
            _ => None,
        }
    }
}

/// 🗃️ The writes of a bulk edit: `args.collection`, `args.ids` (one row id per line), `args.field` and the new `args.value` become one `{path, value}` per row (`beams[id=b1].xM`); `None` unless all four are there and at least one id is.
pub fn bulk_writes_arg(args: Option<&semio_framework_value::DslValue>) -> Option<semio_framework_value::DslValue> {
    let args = args?;
    let (collection, field, ids, value) = (args.get("collection")?.as_str()?, args.get("field")?.as_str()?, args.get("ids")?.as_str()?, args.get("value")?);
    let writes: Vec<semio_framework_value::DslValue> = ids
        .lines()
        .filter(|id| !id.is_empty())
        .map(|id| semio_framework_value::DslValue::object(vec![("path".to_string(), semio_framework_value::DslValue::String(format!("{collection}[id={id}].{field}"))), ("value".to_string(), value.clone())]))
        .collect();
    (!writes.is_empty()).then(|| semio_framework_value::DslValue::Array(writes))
}

/// ✍️ Reads `args.writes: [{path, value}]` (or the bulk form, see [`bulk_writes_arg`]); every entry needs a non-empty string path and a value.
pub fn writes_arg(args: Option<&semio_framework_value::DslValue>) -> Result<Vec<FieldWrite>, String> {
    let bulk = bulk_writes_arg(args);
    let items = args.and_then(|value| value.get("writes")).or(bulk.as_ref()).and_then(semio_framework_value::DslValue::as_array).ok_or("setFields needs a 'writes' array")?;
    items
        .iter()
        .map(|item| {
            let path = item.get("path").and_then(semio_framework_value::DslValue::as_str).filter(|path| !path.is_empty()).ok_or("every write needs a non-empty 'path'")?;
            let value = item.get("value").cloned().ok_or_else(|| format!("write '{path}' needs a 'value'"))?;
            Ok(FieldWrite { path: path.to_string(), value })
        })
        .collect()
}

/// 🏷️ Reads `args.collection` and the non-empty `args.ids` string array of an entity command.
pub fn entities_arg(args: Option<&semio_framework_value::DslValue>) -> Result<(String, Vec<String>), String> {
    let collection = args.and_then(|value| value.get("collection")).and_then(semio_framework_value::DslValue::as_str).filter(|collection| !collection.is_empty()).ok_or("an entity command needs a 'collection'")?.to_string();
    let ids: Vec<String> = args
        .and_then(|value| value.get("ids"))
        .and_then(semio_framework_value::DslValue::as_array)
        .ok_or("an entity command needs an 'ids' array")?
        .iter()
        .map(|id| id.as_str().map(str::to_string).ok_or_else(|| "every entity id must be a string".to_string()))
        .collect::<Result<_, _>>()?;
    if ids.is_empty() {
        return Err("an entity command needs at least one id".into());
    }
    Ok((collection, ids))
}

/// ↕️ Reads `args.path`, `args.id` and exactly one of `args.to` (a position), `args.over` (the id of the row dropped on) or `args.step` (`-1` up, `1` down).
pub fn move_arg(args: Option<&semio_framework_value::DslValue>) -> Result<LiveCommand, String> {
    let text = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_str).filter(|text| !text.is_empty()).map(str::to_string);
    let number = |key: &str| args.and_then(|value| value.get(key)).and_then(semio_framework_value::DslValue::as_f64);
    let path = text("path").ok_or("moveEntity needs a 'path'")?;
    let id = text("id").ok_or("moveEntity needs an 'id'")?;
    let target = match (number("to"), text("over"), number("step")) {
        (Some(to), None, None) if to >= 0.0 => MoveTarget::To(to as usize),
        (None, Some(over), None) => MoveTarget::Over(over),
        (None, None, Some(step)) if step == 1.0 || step == -1.0 => MoveTarget::Step(step as isize),
        (_, _, _) => return Err("moveEntity needs exactly one of 'to' (a position), 'over' (a row id) or 'step' (-1 or 1)".into()),
    };
    Ok(LiveCommand::MoveEntity { path, id, target })
}

/// 🧩️ Reads the optional `args.units: [UnitId]`; absent means the whole plan.
pub fn units_arg(args: Option<&semio_framework_value::DslValue>) -> Result<Option<Vec<String>>, String> {
    args.and_then(|value| value.get("units"))
        .filter(|units| !units.is_null())
        .map(|units| units.as_array().ok_or("'units' must be an array")?.iter().map(|unit| unit.as_str().map(str::to_string).ok_or_else(|| "every unit id must be a string".to_string())).collect())
        .transpose()
}

/// 🕹️ The gesture JSON of a `setFields` call: the `gesture` object, else the arguments themselves when a pointer phase or a key came with a handle (what the surface bindings and the host merge).
pub fn gesture_json_arg(args: Option<&semio_framework_value::DslValue>) -> Option<String> {
    let args = args?;
    args.get("gesture").map(semio_framework_pack_json::to_json_string).or_else(|| (args.get("handle").is_some() && (args.get("phase").is_some() || args.get("key").is_some())).then(|| semio_framework_pack_json::to_json_string(&typed_entry(args))))
}

/// ⌨️ A text `value` that a bound input committed with an Enter gesture becomes the typed `entry` of that gesture, so an inline cell and the dimension-line entry of a drag are one input path; any other arguments stay as they are.
fn typed_entry(args: &semio_framework_value::DslValue) -> semio_framework_value::DslValue {
    let enter = args.get("key").and_then(semio_framework_value::DslValue::as_str) == Some("Enter");
    match (enter, args.get("entry"), args.get("value").and_then(semio_framework_value::DslValue::as_str), args) {
        (true, None, Some(text), semio_framework_value::DslValue::Object(fields)) => semio_framework_value::DslValue::object(fields.iter().cloned().chain(std::iter::once(("entry".to_string(), semio_framework_value::DslValue::String(text.to_string())))).collect::<Vec<_>>()),
        _ => args.clone(),
    }
}

/// ➕ The insert JSON of a call: the `insert` object, else the tool named by `spec_key` (`insertSpec` for the ghost, `placeSpec` for the placement) with the merged cursor `at` and the freed snapping (`free` or Ctrl).
pub fn insert_json_arg(args: Option<&semio_framework_value::DslValue>, spec_key: &str) -> Option<String> {
    let args = args?;
    if let Some(insert) = args.get("insert") {
        return Some(semio_framework_pack_json::to_json_string(insert));
    }
    let spec = args.get(spec_key)?.clone();
    let mut members = vec![("spec".to_string(), spec)];
    if let Some(at) = args.get("at") {
        members.push(("at".to_string(), at.clone()));
    }
    let free = args.get("free").or_else(|| args.get("modifiers").and_then(|modifiers| modifiers.get("ctrl"))).and_then(semio_framework_value::DslValue::as_bool).unwrap_or(false);
    members.push(("free".to_string(), semio_framework_value::DslValue::Bool(free)));
    Some(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::object(members)))
}

/// 👻️ Reads `args.insert: {spec?, at?: [x, y, z], free?}`, the insert-tool cursor ghost; a missing `spec` keeps the armed tool, a missing `at` arms it at the origin, `free` (Ctrl) frees the snapping.
pub fn insert_arg(args: Option<&semio_framework_value::DslValue>) -> Result<(Option<String>, Option<[f64; 3]>, bool), String> {
    let insert = args.and_then(|value| value.get("insert")).ok_or("previewInsert needs an 'insert' object")?;
    let spec = insert.get("spec").and_then(semio_framework_value::DslValue::as_str).map(str::to_string);
    let at = insert
        .get("at")
        .map(|at| {
            let at = at.as_array().filter(|at| at.len() == 3).ok_or("the insert position must be [x, y, z]")?;
            let coordinate = |i: usize| at[i].as_f64().ok_or_else(|| "the insert position must be numbers".to_string());
            Ok::<_, String>([coordinate(0)?, coordinate(1)?, coordinate(2)?])
        })
        .transpose()?;
    let free = insert.get("free").and_then(semio_framework_value::DslValue::as_bool).unwrap_or(false);
    Ok((spec, at, free))
}

/// ↕️ The move target of a `moveEntity` payload (`to`, `over` and `step` as the command row carries them): exactly one of them.
pub fn move_target(to: Option<u32>, over: Option<&str>, step: Option<f64>) -> Result<MoveTarget, String> {
    match (to, over, step) {
        (Some(to), None, None) => Ok(MoveTarget::To(to as usize)),
        (None, Some(over), None) => Ok(MoveTarget::Over(over.to_owned())),
        (None, None, Some(step)) if step == 1.0 || step == -1.0 => Ok(MoveTarget::Step(step as isize)),
        (_, _, _) => Err("moveEntity needs exactly one of 'to', 'over' or 'step' (-1 or 1)".into()),
    }
}

impl LiveCommand {
    /// 🔡️ Parses a live tool id and its args; the failure names the offending argument.
    pub fn parse(id: &str, args: Option<&semio_framework_value::DslValue>) -> Result<Self, String> {
        match id {
            "setFields" => writes_arg(args).map(|writes| Self::SetFields { writes }),
            "previewRemedy" if args.and_then(|value| value.get("insert")).is_some() => insert_arg(args).map(|(spec, at, free)| Self::PreviewInsert { spec, at, free }),
            "previewRemedy" => Ok(Self::PreviewRemedy { check_id: check_id_arg(args), index: index_arg(args, "remedyIndex"), option: index_arg(args, "optionIndex") }),
            "clearPreview" => Ok(Self::ClearPreview),
            "cancelEvaluation" => Ok(Self::CancelEvaluation),
            "evaluateUnits" => {
                let reason = args.and_then(|value| value.get("reason")).and_then(semio_framework_value::DslValue::as_str).map_or(Some(EvalReason::Manual), EvalReason::parse).ok_or("unknown evaluation reason")?;
                Ok(Self::EvaluateUnits { units: units_arg(args)?, reason })
            }
            "copyEntities" => entities_arg(args).map(|(collection, ids)| Self::CopyEntities { collection, ids }),
            "duplicateEntities" => entities_arg(args).map(|(collection, ids)| Self::DuplicateEntities { collection, ids }),
            "pasteEntities" => {
                let clip_json = args.and_then(|value| value.get("clip")).map(semio_framework_pack_json::to_json_string).ok_or("pasteEntities needs a 'clip'")?;
                Ok(Self::PasteEntities { clip_json, at: args.and_then(|value| value.get("at")).and_then(semio_framework_value::DslValue::as_f64).map(|at| at as usize) })
            }
            "moveEntity" => move_arg(args),
            "deleteEntities" => entities_arg(args).map(|(collection, ids)| Self::DeleteEntities { collection, ids }),
            other => Err(format!("'{other}' is not a live command")),
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            Self::SetFields { .. } => NORM_LIVE_TOOL_IDS[0],
            Self::PreviewRemedy { .. } | Self::PreviewInsert { .. } => NORM_LIVE_TOOL_IDS[1],
            Self::ClearPreview => NORM_LIVE_TOOL_IDS[2],
            Self::CancelEvaluation => NORM_LIVE_TOOL_IDS[3],
            Self::EvaluateUnits { .. } => NORM_LIVE_TOOL_IDS[4],
            Self::CopyEntities { .. } => NORM_LIVE_TOOL_IDS[6],
            Self::DuplicateEntities { .. } => NORM_LIVE_TOOL_IDS[7],
            Self::PasteEntities { .. } => NORM_LIVE_TOOL_IDS[8],
            Self::MoveEntity { .. } => NORM_LIVE_TOOL_IDS[9],
            Self::DeleteEntities { .. } => NORM_LIVE_TOOL_IDS[10],
        }
    }
}

/// 📤️ Result of one live command: store mutations for `setFields`, ephemeral lane events for the rest.
pub enum LiveOutcome<M> {
    Mutations(Emit<M, NoConfigMutation>),
    Lanes(Vec<LaneEvent>),
}

/// 🫧️ Shared live-command dispatch. `resolve` is the family's `NormEditRules::resolve`; `setFields` resolves every write against the same
/// document and emits all mutations as ONE batched edit, so a drag release is one undo step. Placement leaves go through the shared placement commit: an entity without a row is ONE insert of its `standing` pose with the written leaves.
pub fn dispatch_live_command<Fam, Map>(document: &Fam::Document, command: &LiveCommand, standing: &dyn Fn(&str) -> Option<semio_s_plugin_norm_structural_scene::projection_kit::Placement>, resolve: Map) -> Result<LiveOutcome<Fam::Mutation>, Fault>
where
    Fam: NormFamily,
    Fam::Document: Clone + semio_framework_value::ToValue,
    Map: FnMut(&Fam::Document, &NormEdit) -> Result<Vec<Fam::Mutation>, String>,
{
    match command {
        LiveCommand::SetFields { writes } => crate::direct_manipulation::handle_placed_fields(document, writes, standing, resolve).map(LiveOutcome::Mutations),
        LiveCommand::PreviewRemedy { check_id, index, option } => {
            let report = Fam::live_report(document);
            let ghost = preview_remedy(&report, &semio_framework_value::ToValue::to_value(document), check_id, *index, *option)?;
            Ok(LiveOutcome::Lanes(vec![LaneEvent::GhostPreviewed(ghost)]))
        }
        LiveCommand::PreviewInsert { .. } => Ok(LiveOutcome::Lanes(Vec::new())),
        LiveCommand::ClearPreview => Ok(LiveOutcome::Lanes(vec![LaneEvent::DragCleared, LaneEvent::GhostCleared])),
        LiveCommand::CancelEvaluation => Ok(LiveOutcome::Lanes(vec![LaneEvent::EvalCancelled])),
        LiveCommand::CopyEntities { .. } | LiveCommand::DuplicateEntities { .. } | LiveCommand::PasteEntities { .. } | LiveCommand::MoveEntity { .. } | LiveCommand::DeleteEntities { .. } => Ok(LiveOutcome::Lanes(Vec::new())),
        LiveCommand::EvaluateUnits { units, .. } => Ok(LiveOutcome::Lanes(vec![LaneEvent::EvalStarted { units: units.clone().unwrap_or_default() }])),
    }
}
//#endregion 🫧️LiveCommands

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod live_command_tests;
