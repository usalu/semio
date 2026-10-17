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
