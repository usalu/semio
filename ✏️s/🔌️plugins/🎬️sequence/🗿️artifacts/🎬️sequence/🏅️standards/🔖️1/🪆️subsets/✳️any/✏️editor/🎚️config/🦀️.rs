//! 🧮️ Sequence play app — view state (`SequenceConfig`) and its operation enum
//! (`SequenceConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/`
//! because nothing in it survives into the `.sequence` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so selection/camera/orientation edits are VCS'd exactly
//! like document content.

use crate::SequenceCamera;
#[cfg(test)]
use protocol::Mutation;
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ B1: sequence's real `ArtifactApp::Config` — absorbs every former `SequencePlayRuntime` field
/// (`last_run_json`/`orientation`) plus the node-graph viewport camera (session-only, never a document
/// field) and the locale the pre-B1 host-pushed `ViewModel` used to carry (see
/// `crate::editor::sequence::terminology::sequence_play_labels`) — same "absorb every runtime field"
/// shape `shooting_engine::ShootingConfig` established for the pilot. 🕹️ ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `selected_step_ids` no longer lives here —
/// selection is framework-owned now, read via `InteractionView::selection("steps")`.
#[derive(Clone, Debug, PartialEq, dsl::DslArtifact)]
#[derive(dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "sequencecfg")]
#[dsl(id = "sequence.config")]
#[dsl(layout = "lines")]
pub struct SequenceConfig {
    /// 🏃️ Last `run` command's `RunResult` JSON, rendered under the compiled script — was
    /// `SequencePlayRuntime::last_run_json`.
    pub last_run_json: String,
    /// 🌳️ Layered-layout flow direction (`"leftRight"`/`"topBottom"`) `reorganize` reads — was
    /// `SequencePlayRuntime::orientation`. Kept as a string rather than `DagLayoutOrientation`
    /// directly: that enum is foreign to this crate and only derives `Serialize`/`Deserialize`, not
    /// `dsl::DslField` (see `crate::editor::sequence::commands::layout`'s conversion helper).
    pub orientation: String,
    /// 🎥️ The node-graph viewport pan/zoom — session-only, never a document field. Was
    /// `SequencePlayRuntime::camera`.
    #[dsl(block)]
    pub camera: SequenceCamera,
    /// 🗣️ BCP-47 locale tag — was read off the host-pushed `ViewModel.locale`.
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for SequenceConfig {
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
impl store::ArtifactPack for SequenceConfig {
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

impl Default for SequenceConfig {
    fn default() -> Self {
        Self { last_run_json: String::new(), orientation: "leftRight".into(), camera: SequenceCamera::default(), }
    }
}

store::impl_whole_record_config!(SequenceConfig);
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
