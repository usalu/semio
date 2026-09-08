//! 👥️ Layout presence — shareable live ephemeral state + mutations.
//!
//! Selection/hover moved OUT of this facet into the framework-owned "elements" interaction domain,
//! which broadcasts automatically via the typed `PresencePeer.interaction` field (ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — no app-mirrored field needed here anymore.

use crate::{LayoutCamera, LayoutDropPreviewState};
use store::ArtifactPack;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Presence
/// 👥️ Shareable live subset of layout view state (active page, drop ghost, cameras).
#[derive(Clone, Debug, PartialEq, dsl::DslArtifact, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "layout.presence")]
#[dsl(layout = "lines")]
pub struct LayoutPresence {
    pub active_page_id: String,
    #[dsl(block)]
    pub drop_preview: LayoutDropPreviewState,
    #[dsl(block)]
    pub camera: LayoutCamera,
    #[dsl(block)]
    pub preview_camera: LayoutCamera,
}

impl Default for LayoutPresence {
    fn default() -> Self {
        Self { active_page_id: "page-1".into(), drop_preview: LayoutDropPreviewState::default(), camera: LayoutCamera::default(), preview_camera: LayoutCamera::default() }
    }
}

impl protocol::MutationDiff<LayoutPresence> for LayoutPresence {
    fn apply(&self, _base: &LayoutPresence) -> protocol::MutationApplyResult<LayoutPresence> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl store::ArtifactDsl for LayoutPresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for LayoutPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️Presence

/// 🧬️ Physical mutation declarations for the layout.presence channel.
#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️contract-vectors/🦀️.rs"]
mod contract_vectors;
