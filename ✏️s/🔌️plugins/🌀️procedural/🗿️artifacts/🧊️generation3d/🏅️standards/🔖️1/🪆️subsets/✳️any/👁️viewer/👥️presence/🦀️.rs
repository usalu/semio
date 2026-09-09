//! 👥️ Generation3dViewPresence — the read-only surface's shareable live ephemeral state.
//!
//! A viewer publishes exactly what a co-viewer needs to follow along: where the read-only preview
//! camera is looking and how it is shading. Hover and selection broadcast on their own through the
//! framework's typed `PresenceInteraction` (declared by `create_generation3d_viewer`'s
//! `.interaction("graph")`), so they are deliberately absent here — the surface stores neither.
//! MUST NOT import anything from the sibling `✏️editor` module (`policyViewerPurityBreaches`).

use crate::viewer::generation3d::config::Generation3dViewCamera;
use semio_framework_value_derive::{FromValue, ToValue};
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of the read-only 3d preview surface.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "generation3dview.presence")]
#[dsl(layout = "lines")]
pub struct Generation3dViewPresence {
    /// 📷️ The 3D preview viewport camera this viewer is looking through.
    #[dsl(block)]
    pub preview_camera: Generation3dViewCamera,
    /// 👁️ Preview shading mode.
    pub show_mode: String,
}

impl Default for Generation3dViewPresence {
    fn default() -> Self {
        Self { preview_camera: Generation3dViewCamera::default(), show_mode: "shaded".into() }
    }
}

impl protocol::MutationDiff<Generation3dViewPresence> for Generation3dViewPresence {
    fn apply(&self, _base: &Generation3dViewPresence) -> protocol::MutationApplyResult<Generation3dViewPresence> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl store::ArtifactDsl for Generation3dViewPresence {
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

impl ArtifactPack for Generation3dViewPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
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

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;
