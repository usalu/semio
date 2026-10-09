//! 🎚️ Persisted local configuration for one exact Forms Try window.

use semio_framework_value_derive::{FromValue, ToValue};

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact)]
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
impl store::ArtifactDsl for FormsTryWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Forms Try window config envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 🎒️ Record-backed pack form. `record_spec` is what the retained window-config loader reads to
/// decode a mounted pack field-by-field; returning `None` here would fail every retained load of
/// this window kind with `WindowConfigPackLoadDiagnostic::TypedState`.
impl store::ArtifactPack for FormsTryWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
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
    fn build_store_owners() -> Result<store::DocumentStoreOwners<Self::State, Self::Mutation>, semio_framework_value::ValueError> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
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
