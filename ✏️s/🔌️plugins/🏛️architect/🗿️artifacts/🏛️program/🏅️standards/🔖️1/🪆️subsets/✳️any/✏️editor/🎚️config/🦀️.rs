//! 🧮️ Architect play app — the view state (`Config`) and its operation surface.
//!
//! The unmounted search and analysis caches remain here until their result surface has a bounded owner.
//! Register, Adjacency, Graph, and Report state belongs to registered exact-window configurations.

use crate::standards::v1::subsets::any::schema::inferences::SearchQuery;
#[cfg(test)]
use protocol::Mutation;
use protocol::MutationDiff;

//#region 🔖️Config
/// 🧮️ B1: `ArchitectPlayApp`'s `ArtifactEditor::Config` — the pure replacement for the pre-B1
/// `RefCell<ArchitectPlayRuntime>` app-struct field (mirrors `norm::NormConfig`'s single-shared-shape
/// precedent for a monolithic, non-crate-split app).
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact(id = "architect.config")]
#[artifact(extension = "architectcfg")]
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
impl store::ArtifactPack for ArchitectConfig {
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

/// 🔺️ Sparse field diff of the architect config: each present field is written, the rest of the config is untouched.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
pub struct ArchitectConfigDiff {
    pub search_query: Option<String>,
    pub search_history_json: Option<String>,
    pub last_result_json: Option<String>,
    pub last_analysis_json: Option<String>,
}

impl MutationDiff<ArchitectConfig> for ArchitectConfigDiff {
    fn apply(&self, base: &ArchitectConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<ArchitectConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.search_query {
            next.search_query = value.clone();
        }
        if let Some(value) = &self.search_history_json {
            next.search_history_json = value.clone();
        }
        if let Some(value) = &self.last_result_json {
            next.last_result_json = value.clone();
        }
        if let Some(value) = &self.last_analysis_json {
            next.last_analysis_json = value.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.search_query.is_some() {
            self.search_query = other.search_query;
        }
        if other.search_history_json.is_some() {
            self.search_history_json = other.search_history_json;
        }
        if other.last_result_json.is_some() {
            self.last_result_json = other.last_result_json;
        }
        if other.last_analysis_json.is_some() {
            self.last_analysis_json = other.last_analysis_json;
        }
    }
}

impl protocol::DiffAlgebra<ArchitectConfig> for ArchitectConfigDiff {
    fn inverse(&self, base: &ArchitectConfig) -> Self {
        Self {
            search_query: self.search_query.as_ref().map(|_| base.search_query.clone()),
            search_history_json: self.search_history_json.as_ref().map(|_| base.search_history_json.clone()),
            last_result_json: self.last_result_json.as_ref().map(|_| base.last_result_json.clone()),
            last_analysis_json: self.last_analysis_json.as_ref().map(|_| base.last_analysis_json.clone()),
        }
    }

    fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;
//#endregion 🔖️Config

//#region 🔖️Readers
pub fn parse_search_history(cfg: &ArchitectConfig) -> Vec<SearchQuery> {
    semio_framework_pack_json::from_json_str(&cfg.search_history_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default()
}

/// 🧮️ The config edit every command handler emits: one `SetConfig` carrying exactly the fields of `next` that differ from `base`.
pub fn snapshot(base: &ArchitectConfig, next: ArchitectConfig) -> Vec<ArchitectConfigMutation> {
    let differing = |current: &String, requested: String| (*current != requested).then_some(requested);
    let set = SetConfig {
        search_query: differing(&base.search_query, next.search_query),
        search_history_json: differing(&base.search_history_json, next.search_history_json),
        last_result_json: differing(&base.last_result_json, next.last_result_json),
        last_analysis_json: differing(&base.last_analysis_json, next.last_analysis_json),
    };
    let untouched = set.search_query.is_none() && set.search_history_json.is_none() && set.last_result_json.is_none() && set.last_analysis_json.is_none();
    (!untouched).then(|| ArchitectConfigMutation::SetConfig(set)).into_iter().collect()
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

#[path = "🚪️io/🦀️.rs"]
pub mod io;
