//! 🧮️ Animate presentation app — view state (`PresentationConfig`) and its operation enum
//! (`PresentationConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/`
//! because nothing in it survives into the `.presentation` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so engagement edits are VCS'd exactly
//! like document content.

#[cfg(test)]
use protocol::Mutation;

//#region 🔖️Config
/// 🧮️ B1: animate presentation's real `ArtifactApp::Config` — absorbs every former
/// `AnimatePresentationPlayRuntime` field (`engagement_input`). Locale and terminology come from the
/// shared host `ViewModel` (see `crate::editor::animate::terminology`).
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "presentcfg")]
#[artifact(id = "presentation.config")]
#[dsl(layout = "lines")]
#[derive(Default)]
pub struct PresentationConfig {
    /// ⌨️ In-progress engagement-bar input draft — was `AnimatePresentationPlayRuntime::engagement_input`.
    pub engagement_input: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for PresentationConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for PresentationConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec



impl store::ConfigRecord for PresentationConfig {}

/// 🔺️ Sparse field delta over [`PresentationConfig`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct PresentationConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub engagement_input: Option<String>,
}

impl protocol::MutationDiff<PresentationConfig> for PresentationConfigDiff {
    fn apply(&self, base: &PresentationConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<PresentationConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.engagement_input {
            next.engagement_input = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.engagement_input.is_some() {
            self.engagement_input = other.engagement_input;
        }
    }
}

impl protocol::DiffAlgebra<PresentationConfig> for PresentationConfigDiff {
    fn inverse(&self, base: &PresentationConfig) -> Self {
        Self {
            engagement_input: self.engagement_input.as_ref().map(|_| base.engagement_input.clone()),
        }
    }
    fn between(base: &PresentationConfig, other: &PresentationConfig) -> Self {
        Self {
            engagement_input: (base.engagement_input != other.engagement_input).then(|| other.engagement_input.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.engagement_input.is_none()
    }
}

//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️contract-vectors/🦀️.rs"]
mod contract_vectors;

#[path = "🚪️io/🦀️.rs"]
pub mod io;
