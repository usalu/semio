//! 🧮️ Architect play app — the view state (`Config`) and its operation surface.
//!
//! Everything the pre-B1 `RefCell<ArchitectPlayRuntime>` held (selection, active register, search,
//! cached report/analysis JSON, adjacency filter, graph camera) lives here, written via whole-snapshot
//! `ArchitectConfigMutation::ReplaceConfig` values from the `🎮️commands/*` handlers.

use crate::artifacts::program::registers::AdjacencyKind;
use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::ProgramReport;
use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::SearchQuery;
use protocol::MutationDiff;
#[cfg(test)]
use protocol::Mutation;

//#region 🔖️Config
/// @emoji 🧮️ B1: `ArchitectPlayApp`'s `ArtifactEditor::Config` — the pure replacement for the pre-B1
/// `RefCell<ArchitectPlayRuntime>` app-struct field (mirrors `norm::NormConfig`'s single-shared-shape
/// precedent for a monolithic, non-crate-split app).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslArtifact)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[dsl(extension = "architectcfg")]
#[dsl(layout = "lines")]
pub struct ArchitectConfig {
    pub active_register: String,
    pub search_query: String,
    /// 🔎️ `Vec<SearchQuery>` serialized as JSON — `SearchQuery` has no `dsl::DslField` binding of its
    /// own, so (like `positions_json`/`camera_json` on other migrated apps) it round-trips as text.
    pub search_history_json: String,
    /// 📋️ The currently rendered `ProgramReport` (the report window), serialized as JSON.
    pub active_report_json: String,
    /// 🐛️ Generic last-action-result debug dump (search hits / validation diagnostics / analysis
    /// result / report) — the pre-B1 `last_report_json` field, renamed since it no longer overlaps
    /// with `active_report_json` above.
    pub last_result_json: String,
    /// 🧮️ The last computed `AnalysisResult`, serialized as JSON — write-only state today (no render
    /// path reads it back), kept for state fidelity with the pre-B1 runtime.
    pub last_analysis_json: String,
    pub adjacency_kind_filter: Option<AdjacencyKind>,
    pub graph_camera_x: f64,
    pub graph_camera_y: f64,
    pub graph_camera_zoom: f64,
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

impl Default for ArchitectConfig {
    fn default() -> Self {
        Self {
            active_register: String::new(),
            search_query: String::new(),
            search_history_json: String::new(),
            active_report_json: String::new(),
            last_result_json: String::new(),
            last_analysis_json: String::new(),
            adjacency_kind_filter: None,
            graph_camera_x: 0.0,
            graph_camera_y: 0.0,
            graph_camera_zoom: 1.0,
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
/// 🧮️ Reads `cfg.active_register`, defaulting to `"elements"` for a config that predates
/// `ArchitectPlayApp::initial_config`'s default (or was constructed bare in a test).
pub fn active_register(cfg: &ArchitectConfig) -> &str {
    if cfg.active_register.is_empty() {
        "elements"
    } else {
        cfg.active_register.as_str()
    }
}

pub fn parse_search_history(cfg: &ArchitectConfig) -> Vec<SearchQuery> {
    dsl::json::from_json_str(&cfg.search_history_json).unwrap_or_default()
}

pub fn parse_active_report(cfg: &ArchitectConfig) -> Option<ProgramReport> {
    if cfg.active_report_json.is_empty() {
        return None;
    }
    dsl::json::from_json_str(&cfg.active_report_json).ok()
}

/// 🧮️ The whole-snapshot config edit every command handler emits.
pub fn snapshot(next: ArchitectConfig) -> Vec<ArchitectConfigMutation> {
    vec![ArchitectConfigMutation::ReplaceConfig(ReplaceConfig { config: next })]
}
//#endregion 🔖️Readers

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn active_register_falls_back_to_elements() {
        assert_eq!(active_register(&ArchitectConfig::default()), "elements");
        assert_eq!(active_register(&ArchitectConfig { active_register: "risks".into(), ..ArchitectConfig::default() }), "risks");
    }

    #[semio_framework_async_macros::async_test]
    async fn a_snapshot_operation_replaces_the_whole_config_and_inverts_to_the_base() {
        let base = ArchitectConfig::default();
        let next = ArchitectConfig { search_query: "hall".into(), ..ArchitectConfig::default() };
        let operation = ArchitectConfigMutation::ReplaceConfig(ReplaceConfig { config: next.clone() });
        assert_eq!(operation.diff(&base).diff(), &next);
        assert_eq!(operation.inverse(&base), vec![ArchitectConfigMutation::ReplaceConfig(ReplaceConfig { config: base })]);
    }

    #[semio_framework_async_macros::async_test]
    async fn an_empty_active_report_parses_to_none() {
        assert!(parse_active_report(&ArchitectConfig::default()).is_none());
        assert!(parse_search_history(&ArchitectConfig::default()).is_empty());
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod contract_vectors {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};
    use dsl::os_pack as pack;

    #[test]
    fn architect_configuration_contract_vectors_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔁️mutation-contracts.json")).expect("neutral contract vectors");
        let base: ArchitectConfig = pack::from_json_str(&vectors["base"].to_string()).expect("owned base decoder");
        assert_eq!(<ArchitectConfigMutation as Mutation<ArchitectConfig>>::DESCRIPTORS.len(), vectors["cases"].as_array().expect("cases").len());
        for vector in vectors["cases"].as_array().expect("cases") {
            let mutation: ArchitectConfigMutation = pack::from_json_str(&vector["mutation"].to_string()).expect("owned operation decoder");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation)).expect("independent operation oracle"), vector["mutation"]);
            assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().expect("semantic kind"));
            assert_eq!(ArchitectConfigMutation::parse_op(&mutation.print_op()).expect("operation text"), mutation);
            assert_eq!(ArchitectConfigMutation::decode_op(&mutation.encode_op().expect("operation binary")).expect("binary decode"), mutation);
            let outcome = mutation.diff(&base);
            assert!(outcome.messages().is_empty());
            let next = outcome.diff().apply(&base).expect("apply diff");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&next)).expect("independent state oracle"), vector["expected"]);
            let next_for_noop = next.clone();
            let restored = mutation.inverse(&base).into_iter().fold(next, |state, inverse| inverse.diff(&state).diff().apply(&state).expect("apply inverse"));
            assert_eq!(restored, base);
            let noop = mutation.diff(&next_for_noop);
            assert!(!noop.messages().is_empty());
        }
    }
}
