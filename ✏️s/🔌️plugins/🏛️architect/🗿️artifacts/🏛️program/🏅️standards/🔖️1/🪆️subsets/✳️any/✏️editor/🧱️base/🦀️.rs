//! 🧱️ Architect editor base — the app id, the UI assembly helpers, the interaction constants and the document reset effect that every editor surface shares, so no surface reaches the editor root for them.

use crate::standards::v1::subsets::any::schema::snapshot::{ProgramSnapshot, ARCHITECT_PROGRAM_SCHEMA};

//#region 🔖️Constants
pub const ARCHITECT_APP_ID: &str = "s.architect.program@1/*#editor";

/// 🏷️ Admits a semantic architect label.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| ui_capacity_error())
}

/// 📝️ Admits a fixed architect UI string.
pub fn ui_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::UiText> {
    semio_framework_ui_contract::UiText::try_from_str(value.as_ref()).ok_or_else(ui_capacity_error)
}

/// 🧱️ Finalizes a architect UI node with explicit identity.
pub fn ui_node<B: semio_framework_ui_contract::HasBase + semio_framework_ui_contract::Buildable>(builder: B, id: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    builder.try_id(id).map_err(|_| ui_capacity_error())?.try_build().map_err(|_| ui_capacity_error())
}

/// 👶️ Admits a complete collection of architect UI children.
pub fn ui_children<B: semio_framework_ui_contract::HasChildren>(builder: B, children: impl IntoIterator<Item = semio_framework_plugin::BuiltNode>) -> semio_framework_plugin::UiAssemblyResult<B> {
    builder.try_children(children).map_err(|_| ui_capacity_error())
}

/// 🚧️ Reports fixed-capacity UI admission failure.
pub fn ui_capacity_error() -> semio_framework_plugin::PluginAssemblyError {
    semio_framework_plugin::PluginAssemblyError::new("architect.ui.capacity", "architect UI admission failed")
}

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`📌️panels/*`, `🪟️windows/*`) builds its item/on-change actions with.
pub fn architect_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(ARCHITECT_APP_ID).action(action, args)
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
/// 🕹️ "program" — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14) interaction
/// domain this app declares: `HierarchyProvider::Flat` (the 68 registers are flat lists of
/// `EntityId`-keyed rows, no parent/child nesting), one granularity ("entity") covering every
/// register kind uniformly (mirrors note's single "block" granularity over several block kinds).
/// The document panel's element rows are the sole real pick surface today (`📌️panels/🗿️artifact`);
/// the register/graph/trace windows lost their per-selection detail entirely (`render` carries no
/// `InteractionView` — see each window's own doc comment) rather than being force-wired to a
/// surface (`BlockListScene`/`NodeGraphScene`) that cannot show it yet.
pub const ARCHITECT_INTERACTION_PROGRAM: &str = "program";
pub const ARCHITECT_INTERACTION_GRANULARITY_ENTITY: &str = "entity";
//#endregion 🔖️Interaction

//#region 🔖️ResetDocument
/// 🧬️ Whole-document replace is banned from the `Mutation` enum outright (the whole-document
/// replace variant — see `📓️taxonomy.md`'s forbidden vocabulary), so import/exchange flows build a
/// `Effect::LoadDocument` (outside undo history) instead of an `artifact_mutations` entry —
/// same mechanism `✏️s/🔌️plugins/🗒️note`'s `reset_document_effect` already established.
/// 🪦️ The spr is built with `empty_document_spr`, NOT by minting an `ArtifactEnvelope` and printing
/// it: an envelope is a terminal store shell whose `Drop` asserts that its app-owned bounded
/// retirement authority detached every nested owner first, and nothing here ever mounts or retires
/// it — so the envelope route panicked the moment the effect was built. `🗒️note`, `✒️writer` and
/// `📐️cad` all take this route; the log is edit-free by construction, which is exactly what a
/// whole-document replace carries.
pub fn reset_document_effect(document: &ProgramSnapshot) -> semio_framework_plugin::Effect {
    let pack = <ProgramSnapshot as pack::snapshot::ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr(ARCHITECT_APP_ID, ARCHITECT_PROGRAM_SCHEMA));
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument
