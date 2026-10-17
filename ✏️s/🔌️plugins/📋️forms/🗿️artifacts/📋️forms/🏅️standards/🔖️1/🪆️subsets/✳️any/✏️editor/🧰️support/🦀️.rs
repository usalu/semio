//! 🧰️ Forms editor support — the action factory, the fixed-capacity UI admission helpers and the interaction domain ids that every window, panel and mode of the editor builds with, so none of them reaches back into the editor root.

use semio_framework_pack_json::Value;

//#region 🔖️Constants
pub const FORMS_PLAY_APP_ID: &str = "forms-play";
pub const FORMS_PLAY_WINDOW_TRY: &str = "forms-try";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`🪟️windows/*`, `📌️panels/*`) builds its `on_change`/item actions with.
pub fn forms_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(FORMS_PLAY_APP_ID).action(action, args)
}

/// 🏷️ Admits display text into a bounded semantic label.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    value.as_ref().try_into().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "forms label admission failed"))
}

/// 🎟️ Propagates a rejected semantic UI admission.
pub fn ui_admit<T, E>(value: Result<T, E>) -> semio_framework_plugin::UiAssemblyResult<T> {
    value.map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "forms UI admission failed"))
}

/// 🔤️ Admits text used by semantic controls.
pub fn ui_text_value(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiText> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "forms text admission failed"))
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}
//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🕹️ "fields" — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14) interaction domain
/// this app declares: `HierarchyProvider::Topology` over the document's own step/question nesting
/// (steps are the "section" granularity, questions are the "field" granularity), transitive (selecting/
/// hovering a step covers its questions).
pub const FORMS_INTERACTION_FIELDS: &str = "fields";
pub const FORMS_INTERACTION_GRANULARITY_FIELD: &str = "field";
pub const FORMS_INTERACTION_GRANULARITY_SECTION: &str = "section";
//#endregion 🔖️Interaction

//#region 🔖️Values
/// 🔠️ Parses a command's JSON-blob payload field (`value_json`/`values_json`/…), falling back to
/// `Value::Null` on malformed or absent JSON — every one of these fields is best-effort text carried
/// across the wire, not a validated protocol.
pub fn parse_value_json(value_json: &str) -> Value {
    semio_framework_pack_json::parse(value_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or(Value::Null)
}
//#endregion 🔖️Values
