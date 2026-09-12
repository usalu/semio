//! 📊️ Shared persisted-local configuration for every concrete Norm Results window.

pub use super::mutations::{change_selected_check_index::ChangeSelectedCheckIndex, NormResultsWindowConfigMutation};
pub use super::schema::NormResultsWindowConfig;

//#region 🔖️Config
//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for NormResultsWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for NormResultsWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec

impl store::ConfigRecord for NormResultsWindowConfig {}

/// 🧮️ Resolved one-field configuration projection produced by its semantic mutation.
impl protocol::MutationDiff<NormResultsWindowConfig> for NormResultsWindowConfig {
    fn apply(&self, _base: &NormResultsWindowConfig) -> protocol::MutationApplyResult<NormResultsWindowConfig> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}
//#endregion 🔖️Config

/// 🪟️ Declares one family's concrete Results-window owner without duplicating its shared schema.
#[macro_export]
macro_rules! norm_results_window_config_owner {
    ($owner:ident, $window_kind_id:expr) => {
        pub struct $owner;

        impl semio_framework_plugin::WindowConfigOwner for $owner {
            const WINDOW_KIND_ID: &'static str = $window_kind_id;
            const SCHEMA: &'static str = "s.norm.results-window.config";
            const MAXIMUM_PUBLICATION_BYTES: usize = 4_096;
            type State = $crate::results_window_config::NormResultsWindowConfig;
            type Mutation = $crate::results_window_config::NormResultsWindowConfigMutation;

            fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> {
                semio_framework_plugin::bounded_window_config_store_owners::<Self>()
            }

            fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
                semio_framework_plugin::bounded_window_config_preparation_factory::<Self>()
            }

            fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
                semio_framework_plugin::bounded_window_config_store_disposer::<Self>()
            }
        }
    };
}

/// 👁️ Reads selection only from the exact Results-window snapshot captured for this call.
pub fn current<O>(view: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>) -> NormResultsWindowConfig
where
    O: semio_framework_plugin::WindowConfigOwner<State = NormResultsWindowConfig, Mutation = NormResultsWindowConfigMutation>,
{
    view.window::<O>().cloned().unwrap_or_default()
}

/// 🎯️ Addresses selection to a concrete Results instance, including a panel's focused window.
pub fn addressed<O>(
    view: &semio_framework_plugin::ViewModel,
    mutation: NormResultsWindowConfigMutation,
) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault>
where
    O: semio_framework_plugin::WindowConfigOwner<State = NormResultsWindowConfig, Mutation = NormResultsWindowConfigMutation>,
{
    let id = view
        .window_id
        .as_deref()
        .or(view.focused_window_id.as_deref())
        .ok_or_else(|| semio_framework_plugin::Fault::from("norm-results-window-required"))?;
    let kind = view
        .window_instances
        .iter()
        .find(|window| window.id == id)
        .map(|window| window.window_kind_id.as_str())
        .ok_or_else(|| semio_framework_plugin::Fault::from("norm-results-window-stale"))?;
    if kind != O::WINDOW_KIND_ID {
        return Err(semio_framework_plugin::Fault::from("norm-results-window-kind-required"));
    }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<O>(id, mutation))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
