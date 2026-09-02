//! 🧮️ Architect play app — the view state (`Config`) and its operation surface.
//!
//! Everything the pre-B1 `RefCell<ArchitectPlayRuntime>` held (selection, active register, search,
//! cached report/analysis JSON, adjacency filter, graph camera) lives here, written via whole-snapshot
//! `ArchitectConfigMutation::Snapshot`s from the `🎮️commands/*` handlers.

use crate::artifacts::program::registers::AdjacencyKind;
use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::ProgramReport;
use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::SearchQuery;
use protocol::{Mutation, MutationDiff};

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
    async fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    async fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for ArchitectConfig {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.envelope_id())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    async fn record_spec() -> Option<dsl::RecordSpec> {
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
    async fn apply(&self, _base: &ArchitectConfig) -> protocol::MutationApplyResult<ArchitectConfig> {
        Ok({ self.clone() })
    }
    async fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

/// @emoji 🧮️ `ArchitectConfig`'s operation enum — a single whole-snapshot `Snapshot` variant is the
/// generic inverse every `🎮️commands/*` config edit uses (mirrors `norm::NormConfigOperation`
/// and `cad`'s `snapshot_of` helper; architect's config has no single hot-path field worth its own
/// granular operation variant the way `NormConfig::selected_check_index` did).
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslOps)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum ArchitectConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: ArchitectConfig,
    },
}

//#region 🔖️OpCodec
impl protocol::OpText for ArchitectConfigMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for ArchitectConfigMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}

//#endregion 🔖️OpCodec

impl Mutation<ArchitectConfig> for ArchitectConfigMutation {
    type Diff = ArchitectConfig;

    /// ✏️ Warning `mutation.no-op` if `config` already equals `base` (empty diff), else the
    /// whole-snapshot replacement.
    async fn diff(&self, base: &ArchitectConfig) -> protocol::MutationOutcome<ArchitectConfig> {
        match self {
            ArchitectConfigMutation::Snapshot { config } => {
                if config == base {
                    return protocol::MutationOutcome::empty().warn("mutation.no-op", "Config already matches the requested value.");
                }
                protocol::MutationOutcome::new(config.clone())
            }
        }
    }

    async fn inverse(&self, base: &ArchitectConfig) -> Vec<Self> {
        vec![ArchitectConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️Config

//#region 🔖️Readers
/// 🧮️ Reads `cfg.active_register`, defaulting to `"elements"` for a config that predates
/// `ArchitectPlayApp::initial_config`'s default (or was constructed bare in a test).
pub async fn active_register(cfg: &ArchitectConfig) -> &str {
    if cfg.active_register.is_empty() {
        "elements"
    } else {
        cfg.active_register.as_str()
    }
}

pub async fn parse_search_history(cfg: &ArchitectConfig) -> Vec<SearchQuery> {
    dsl::json::from_json_str(&cfg.search_history_json).unwrap_or_default()
}

pub async fn parse_active_report(cfg: &ArchitectConfig) -> Option<ProgramReport> {
    if cfg.active_report_json.is_empty() {
        return None;
    }
    dsl::json::from_json_str(&cfg.active_report_json).ok()
}

/// 🧮️ The whole-snapshot config edit every command handler emits.
pub async fn snapshot(next: ArchitectConfig) -> Vec<ArchitectConfigMutation> {
    vec![ArchitectConfigMutation::Snapshot { config: next }]
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
        let operation = ArchitectConfigMutation::Snapshot { config: next.clone() };
        assert_eq!(operation.diff(&base).diff(), &next);
        assert_eq!(operation.inverse(&base), vec![ArchitectConfigMutation::Snapshot { config: base }]);
    }

    #[semio_framework_async_macros::async_test]
    async fn an_empty_active_report_parses_to_none() {
        assert!(parse_active_report(&ArchitectConfig::default()).is_none());
        assert!(parse_search_history(&ArchitectConfig::default()).is_empty());
    }
}
//#endregion 🧪️Tests
