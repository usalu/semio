//! 🧮️ Layout play app — view state (`LayoutConfig`) and its operation enum (`LayoutConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.layout` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so camera/drop-ghost edits are VCS'd exactly like
//! document content. Selection/hover moved OUT of this config into the framework-owned "elements"
//! interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).

use crate::LayoutCamera;
pub use crate::LayoutDropPreviewState;
#[cfg(test)]
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Config
/// 🧮️ B1: layout's real `ArtifactApp::Config` — absorbs every field that used to live on
/// `layout_ui::LayoutPlayApp`'s `RefCell<LayoutPlayRuntime>` (active page, drop-ghost, engagement
/// draft, and the two independent blueprint/preview camera poses) plus `locale`, the one `ViewModel`
/// field the layout UI actually reads — session-only view state now round-trips through the config
/// `ArtifactStore` exactly like document content, with a real `backwards` per `LayoutConfigMutation`
/// instead of never being VCS'd at all.
#[derive(Clone, Debug, PartialEq, dsl::DslArtifact, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "layout.config")]
#[dsl(id = "layout.config")]
#[dsl(layout = "lines")]
pub struct LayoutConfig {
    /// 👁️ Active page shown/edited on the Blueprint surface — was `LayoutPlayRuntime::active_page_id`.
    pub active_page_id: String,
    /// 👁️ Live catalogue drag-ghost — was `LayoutPlayRuntime::drop_preview` (`Option<LayoutDropPreviewState>`).
    #[dsl(block)]
    pub drop_preview: LayoutDropPreviewState,
    /// 👁️ In-progress engagement-bar input draft — was `LayoutPlayRuntime::engagement_input`.
    pub engagement_input: String,
    /// 📷️ The Blueprint surface's ephemeral camera pose — was `LayoutPlayRuntime::camera`.
    #[dsl(block)]
    pub camera: LayoutCamera,
    /// 📷️ The Preview surface's ephemeral camera pose — was `LayoutPlayRuntime::preview_camera`.
    #[dsl(block)]
    pub preview_camera: LayoutCamera,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for LayoutConfig {
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
impl store::ArtifactPack for LayoutConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
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

//#endregion 🔖️ArtifactCodec

impl Default for LayoutConfig {
    fn default() -> Self {
        Self { active_page_id: "page-1".into(), drop_preview: LayoutDropPreviewState::default(), engagement_input: String::new(), camera: LayoutCamera::default(), preview_camera: LayoutCamera::default() }
    }
}

store::impl_whole_record_config!(LayoutConfig);
//#endregion 🔖️Config

/// 🧬️ Physical mutation declarations for the layout.config channel.
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
