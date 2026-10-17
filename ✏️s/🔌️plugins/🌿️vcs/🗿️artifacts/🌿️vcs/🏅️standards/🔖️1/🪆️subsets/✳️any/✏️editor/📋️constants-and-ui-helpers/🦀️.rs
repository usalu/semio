//! 📋️ Constants and UI value helpers of the VCS play app: the app ids, the interaction domain, the action and UI value constructors and the example document effect. See [`super`].

use super::*;

//#region 🔖️Constants
pub const VCS_PLAY_APP_ID: &str = "vcs-play";

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🎭️modes/*/🪟️windows/*`) builds its `on_change`/item actions with.
pub fn vcs_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(VCS_PLAY_APP_ID).action(action, args)
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

/// 🏷️ Resolves an `app_labels!`-checked `LabelText` (locale/terminology already folded) into the UI
/// contract's own fixed-capacity `Label` — the contract type deliberately has no `From<LabelText>`,
/// so every chrome node in this surface bridges here.
pub fn ui_fixed_label(label: semio_framework_ui_locale::LabelText) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(label.as_str()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI label admission failed"))
}
//#endregion 🔖️Constants

//#region 🔖️Interaction
/// 🕹️ "history" — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14) interaction domain
/// this app declares: multi-select highlighting over the seeded checkpoint history, `Flat` (checkpoints
/// have no selectable-entity nesting — the DAG's `parent_id` links only matter to the swimlane graph
/// layout, not to selection range/closure), one granularity `"commit"`. Distinct from the per-row
/// `checkoutCheckpoint`/`switchAlternative` click actions the document tree already declares (those are
/// navigation — they change the working checkpoint/alternative — not entity selection), which stay as
/// ordinary actions.
pub const VCS_INTERACTION_HISTORY: &str = "history";
//#endregion 🔖️Interaction

//#region 📚️ExampleDocument
pub const VCS_APP_ID: &str = "s.vcs.vcs@1/*#editor";

/// 🧬️ Whole-document replace is banned from the `Mutation` enum, so the example switch builds an
/// `Effect::LoadDocument` — the same lane `🗒️note`/`✒️writer`/`🏛️architect` use. The spr comes from
/// `store::os_store::text_format::empty_document_spr`, never from a minted `ArtifactEnvelope`: an envelope is a terminal
/// store shell whose `Drop` asserts its bounded retirement authority detached every nested owner
/// first, so building the effect that way panics (ticket 26/09/18, B1a fix #7).
pub fn vcs_example_document_effect() -> semio_framework_plugin::Effect {
    let pack = <VcsSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&crate::examples::demo::snapshot());
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr(VCS_APP_ID, VCS_DOCUMENT_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}
//#endregion 📚️ExampleDocument
