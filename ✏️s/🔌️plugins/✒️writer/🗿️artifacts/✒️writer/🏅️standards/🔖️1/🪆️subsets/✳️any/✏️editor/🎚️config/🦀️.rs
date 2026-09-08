//! 🧮️ Writer play app — view state (`WriterConfig`) and its operation enum (`WriterConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.writer` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so selection/hover/camera/editor-settings edits are VCS'd
//! exactly like document content. `WriterEditorSelection`/`WriterEditorSettings` were carried by the old
//! `⚙️engine` crate's `WriterConfig` before this migration — they move here alongside it, since neither
//! survives into the document either.

use crate::WriterCamera;
use serde::{Deserialize, Serialize};

pub use crate::{WriterEditorSelection, WriterEditorSettings};
/// 🧮️ B1: writer's real `ArtifactApp::Config` — absorbs every former `WriterPlayRuntime` app-struct
/// field that is genuinely app-specific (editor selection, format/lint signals, revision, editor
/// settings, engagement draft, and the session-only viewport camera — see `WriterCamera`'s doc
/// comment) plus `locale`, the one `ViewModel` field the writer UI actually reads
/// (`resolve_labels`/`is_de_locale` — see `crate::editor::writer::WriterPlayApp::render`), mirroring
/// `shooting_engine::ShootingConfig`'s B1 shape. AST selection/hover moved OUT (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM): the framework now owns them as the `ast`
/// interaction domain.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact, dsl::ToValue, dsl::FromValue)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "writer.config")]
#[dsl(layout = "lines")]
pub struct WriterConfig {
    /// 👁️ Editor text selection range — was `WriterPlayRuntime::editor_selection`. Editor-intrinsic
    /// (raw caret/range), NOT the `ast` interaction domain — kept here, undeleted, per ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM.
    #[dsl(block)]
    pub editor_selection: Option<WriterEditorSelection>,
    /// 🔔️ Bumped on every format pass — was `WriterPlayRuntime::format_signal`.
    pub format_signal: u32,
    /// 🔔️ Bumped on every lint pass — was `WriterPlayRuntime::lint_signal`.
    pub lint_signal: u32,
    /// 🔔️ Bumped on every ephemeral view mutation — was `WriterPlayRuntime::revision`.
    pub revision: u32,
    /// ⚙️ Editor chrome settings (line numbers, font/line/tab size) — was `WriterPlayRuntime::editor_settings`.
    #[dsl(block)]
    pub editor_settings: WriterEditorSettings,
    /// 💬️ In-progress engagement-bar input draft — was `WriterPlayRuntime::engagement_input`.
    pub engagement_input: String,
    /// 🎥️ Editor viewport pan/zoom — session-only, never a document field. Was `WriterPlayRuntime::camera`.
    #[dsl(block)]
    pub camera: WriterCamera,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for WriterConfig {
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
impl store::ArtifactPack for WriterConfig {
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

impl Default for WriterConfig {
    fn default() -> Self {
        Self { editor_selection: None, format_signal: 0, lint_signal: 0, revision: 0, editor_settings: WriterEditorSettings::default(), engagement_input: String::new(), camera: WriterCamera::default(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(WriterConfig);
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
