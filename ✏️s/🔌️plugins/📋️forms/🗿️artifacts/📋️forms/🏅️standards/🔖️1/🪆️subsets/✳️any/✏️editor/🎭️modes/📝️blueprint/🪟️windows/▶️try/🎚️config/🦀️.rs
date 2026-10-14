//! 🎚️ Persisted local configuration for one exact Forms Try window.

use semio_framework_value_derive::{FromValue, ToValue};

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase")]
#[dsl(layout = "lines")]
#[artifact(id = "s.forms.forms.try-window-config", extension = "formstrywindowcfg")]
pub struct FormsTryWindowConfig {
    pub current_step_index: u32,
    #[value(skip_serializing_if = "Option::is_none")]
    pub submitted_response_id: Option<String>,
}





/// 📜️ Record-backed text form — the derived `__dsl_spec` grammar inside this window kind's semio
/// text envelope, the same shape every sibling window config prints.


/// 🎒️ Record-backed pack form. `record_spec` is what the retained window-config loader reads to
/// decode a mounted pack field-by-field; returning `None` here would fail every retained load of
/// this window kind with `WindowConfigPackLoadDiagnostic::TypedState`.

impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for FormsTryWindowConfig {

}

semio_framework_os_kernel::config_record! {
    record: FormsTryWindowConfig,
    diff: FormsTryWindowConfigDiff,
    set: FormsTryWindowConfigMutation,
    owner: "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/📝️blueprint/🪟️windows/▶️try",
    payload_schema: "forms.try-window-config",
    emoji: "🎚️",
    fields: {
        current_step_index: u32 => SetCurrentStepIndex "set-current-step-index",
        submitted_response_id: Option<String> => SetSubmittedResponseId "set-submitted-response-id",
    },
}





pub struct FormsTryWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for FormsTryWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::FORMS_PLAY_WINDOW_TRY;
    const SCHEMA: &'static str = "forms.try-window-config";
    const MAXIMUM_PUBLICATION_BYTES: usize = 4_096;
    type State = FormsTryWindowConfig;
    type Mutation = FormsTryWindowConfigMutation;
    type Edit = semio_framework_plugin::app::WindowConfigApplyEdit<FormsTryWindowConfig, FormsTryWindowConfigMutation>;
    const MAXIMUM_PREPARATION_DEPTH: usize = 64;
    fn build_retained_edit() -> std::sync::Arc<Self::Edit> { std::sync::Arc::new(semio_framework_plugin::app::WindowConfigApplyEdit::new()) }
    fn build_store_owners() -> Result<store::os_store::document_store_owners::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

pub fn register(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<FormsTryWindowConfigOwner>()
}

pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> FormsTryWindowConfig {
    snapshot.and_then(|snapshot| snapshot.get::<FormsTryWindowConfigOwner>()).cloned().unwrap_or_default()
}

pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> FormsTryWindowConfig { from_snapshot(view.window) }

pub fn addressed(view: &semio_framework_plugin::ViewModel, base: &FormsTryWindowConfig, config: FormsTryWindowConfig) -> Result<Vec<semio_framework_plugin::WindowConfigMutation>, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("forms-try-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("forms-try-window-stale"))?;
    if kind != super::FORMS_PLAY_WINDOW_TRY { return Err(semio_framework_plugin::Fault::from("forms-try-window-kind-required")); }
    Ok(FormsTryWindowConfigMutation::setting(base, &config).into_iter().map(|mutation| semio_framework_plugin::WindowConfigMutation::of::<FormsTryWindowConfigOwner>(id, mutation)).collect())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️window/🦀️.rs"]
mod tests;

//#region 🪢️TaxonomyMounts
#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
//#endregion 🪢️TaxonomyMounts

#[path="🚪️io/📝️text/🧬️mutations/🦀️.rs"]
mod field_set_text_io;

#[path="🚪️io/💾️binary/🧬️mutations/🦀️.rs"]
mod field_set_binary_io;

#[path="🚪️io/📝️text/📸️snapshot/🦀️.rs"]
mod text_snapshot_io;

#[path="🚪️io/💾️binary/📸️snapshot/🦀️.rs"]
mod binary_snapshot_io;
