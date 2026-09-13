//! 🧮️ Architect play app — the view state (`Config`) and its operation surface.
//!
//! The unmounted search and analysis caches remain here until their result surface has a bounded owner.
//! Register, Adjacency, Graph, and Report state belongs to registered exact-window configurations.

use crate::standards::v1::subsets::any::schema::inferences::SearchQuery;
#[cfg(test)]
use protocol::Mutation;
use protocol::MutationDiff;

//#region 🔖️Config
/// @emoji 🧮️ B1: `ArchitectPlayApp`'s `ArtifactEditor::Config` — the pure replacement for the pre-B1
/// `RefCell<ArchitectPlayRuntime>` app-struct field (mirrors `norm::NormConfig`'s single-shared-shape
/// precedent for a monolithic, non-crate-split app).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslArtifact)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[dsl(id = "architect.config")]
#[dsl(extension = "architectcfg")]
#[dsl(layout = "lines")]
pub struct ArchitectConfig {
    pub search_query: String,
    /// 🔎️ `Vec<SearchQuery>` serialized as JSON — `SearchQuery` has no `dsl::DslField` binding of its
    /// own, so (like `positions_json`/`camera_json` on other migrated apps) it round-trips as text.
    pub search_history_json: String,
    /// 🐛️ Generic last-action-result debug dump (search hits / validation diagnostics / analysis
    /// result) retained outside the mounted window migration until a result owner exists.
    pub last_result_json: String,
    /// 🧮️ The last computed `AnalysisResult`, serialized as JSON — write-only state today (no render
    /// path reads it back), kept for state fidelity with the pre-B1 runtime.
    pub last_analysis_json: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for ArchitectConfig {
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
impl store::ArtifactPack for ArchitectConfig {
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

impl Default for ArchitectConfig {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            search_history_json: String::new(),
            last_result_json: String::new(),
            last_analysis_json: String::new(),
        }
    }
}

impl store::ConfigRecord for ArchitectConfig {}

impl MutationDiff<ArchitectConfig> for ArchitectConfig {
    fn apply(&self, _base: &ArchitectConfig) -> protocol::MutationApplyResult<ArchitectConfig> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;
//#endregion 🔖️Config

//#region 🔖️Readers
pub fn parse_search_history(cfg: &ArchitectConfig) -> Vec<SearchQuery> {
    dsl::json::from_json_str(&cfg.search_history_json).unwrap_or_default()
}

/// 🧮️ The whole-snapshot config edit every command handler emits.
pub fn snapshot(next: ArchitectConfig) -> Vec<ArchitectConfigMutation> {
    vec![ArchitectConfigMutation::ReplaceConfig(ReplaceConfig { config: next })]
}
//#endregion 🔖️Readers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️contract-vectors/🦀️.rs"]
mod contract_vectors;
