//! 🎚️ Persisted-local configuration for one concrete Equation graph window.

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::*;

impl store::ArtifactDsl for EquationGraphWindowConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, rest)| rest);
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }

    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Equation graph window config envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for EquationGraphWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }

    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

store::impl_whole_record_config!(EquationGraphWindowConfig);

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

pub struct EquationGraphWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for EquationGraphWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::MATH_PLAY_WINDOW_GRAPH;
    const SCHEMA: &'static str = "mathematical.equationgraphwindowconfig";
    const MAXIMUM_PUBLICATION_BYTES: usize = 1024;
    type State = EquationGraphWindowConfig;
    type Mutation = EquationGraphWindowConfigMutation;

    fn build_store_owners() -> store::MemberStoreOwners<Self::State, Self::Mutation> {
        semio_framework_plugin::bounded_window_config_store_owners::<Self>()
    }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
        semio_framework_plugin::bounded_window_config_preparation_factory::<Self>()
    }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
        semio_framework_plugin::bounded_window_config_store_disposer::<Self>()
    }
}

pub fn current<'a, C>(view: &'a semio_framework_plugin::ConfigView<'_, C>) -> Option<&'a EquationGraphWindowConfig> {
    view.window::<EquationGraphWindowConfigOwner>()
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, mutation: EquationGraphWindowConfigMutation) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let window_id = view
        .window_id
        .as_deref()
        .ok_or_else(|| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("equation.graph-window-required"), "Equation viewport changes require a concrete graph window"))?;
    let kind = view.window_instances.iter().find(|window| window.id == window_id).map(|window| window.window_kind_id.as_str());
    if kind != Some(super::MATH_PLAY_WINDOW_GRAPH) {
        return Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("equation.graph-window-required"), "Equation viewport changes require a graph window"));
    }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<EquationGraphWindowConfigOwner>(window_id, mutation))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️mutation-vectors/🦀️.rs"]
mod mutation_vectors;

#[cfg(test)]
#[path = "🧪️tests/🔬️window-config-ownership/🦀️.rs"]
mod window_config_ownership;
