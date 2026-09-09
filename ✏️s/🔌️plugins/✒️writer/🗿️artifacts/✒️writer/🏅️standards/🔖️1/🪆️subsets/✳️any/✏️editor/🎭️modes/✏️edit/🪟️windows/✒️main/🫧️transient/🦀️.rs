//! 🫧️ Ephemeral local state for one concrete Writer main editor window.

#[path = "🧬️schema/🦀️.rs"]
mod schema;
pub use schema::*;

impl store::ArtifactDsl for WriterMainWindowTransient {
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Writer window transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for WriterMainWindowTransient {
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

impl protocol::MutationDiff<WriterMainWindowTransient> for WriterMainWindowTransient {
    fn apply(&self, _base: &WriterMainWindowTransient) -> protocol::MutationApplyResult<WriterMainWindowTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

pub struct WriterMainWindowTransientOwner;

#[path = "📢️publication/🦀️.rs"]
mod publication;

impl semio_framework_plugin::WindowTransientOwner for WriterMainWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = super::WRITER_PLAY_WINDOW_KIND;
    type State = WriterMainWindowTransient;
    type Mutation = WriterMainWindowTransientMutation;

    fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        publication::owners()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🧩️partial-construction/🦀️.rs"]
mod preparation_tests;

pub fn current<'a, T>(view: &'a semio_framework_plugin::TransientView<'_, T>) -> Option<&'a WriterMainWindowTransient> {
    view.window::<WriterMainWindowTransientOwner>()
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, mutation: WriterMainWindowTransientMutation) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    let window_id = view
        .window_id
        .as_deref()
        .ok_or_else(|| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("writer.main-window-required"), "Writer editor state requires a concrete main window"))?;
    let kind = view.window_instances.iter().find(|window| window.id == window_id).map(|window| window.window_kind_id.as_str());
    if kind != Some(super::WRITER_PLAY_WINDOW_KIND) {
        return Err(semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("writer.main-window-required"), "Writer editor state requires a main editor window"));
    }
    Ok(semio_framework_plugin::WindowTransientMutation::of::<WriterMainWindowTransientOwner>(window_id, mutation))
}
