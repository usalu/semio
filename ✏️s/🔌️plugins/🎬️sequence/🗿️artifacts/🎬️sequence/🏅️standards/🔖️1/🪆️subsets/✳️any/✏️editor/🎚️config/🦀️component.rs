//! 🧮️ Sequence play app — view state (`SequenceConfig`) and its operation enum
//! (`SequenceConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/`
//! because nothing in it survives into the `.sequence` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so selection/camera/orientation edits are VCS'd exactly
//! like document content.

use crate::artifacts::sequence::SequenceCamera;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ B1: sequence's real `ArtifactApp::Config` — absorbs every former `SequencePlayRuntime` field
/// (`last_run_json`/`orientation`) plus the node-graph viewport camera (session-only, never a document
/// field) and the locale the pre-B1 host-pushed `ViewModel` used to carry (see
/// `crate::editor::sequence::terminology::sequence_play_labels`) — same "absorb every runtime field"
/// shape `shooting_engine::ShootingConfig` established for the pilot. 🕹️ ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `selected_step_ids` no longer lives here —
/// selection is framework-owned now, read via `InteractionView::selection("steps")`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
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
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for SequenceConfig {
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
impl store::ArtifactPack for SequenceConfig {
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

impl Default for SequenceConfig {
    fn default() -> Self {
        Self { last_run_json: String::new(), orientation: "leftRight".into(), camera: SequenceCamera::default(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(SequenceConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigMutations
/// 🧮️ B1: `SequenceConfig`'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `SequencePlayRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()`
/// returns — same "whole-config snapshot is the simplest correct inverse" shape as
/// `shooting_op::ShootingConfigMutation`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum SequenceConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: SequenceConfig,
    },
    #[dsl(key = "last-run")]
    SetLastRun { json: String },
    #[dsl(key = "orientation")]
    SetOrientation { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: SequenceCamera,
    },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for SequenceConfigMutation {
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
impl protocol::OpBinary for SequenceConfigMutation {
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

impl Mutation<SequenceConfig> for SequenceConfigMutation {
    type Diff = SequenceConfig;

    async fn diff(&self, base: &SequenceConfig) -> protocol::MutationOutcome<SequenceConfig> {
        let mut next = base.clone();
        match self {
            SequenceConfigMutation::Snapshot { config } => return protocol::MutationOutcome::new(config.clone()),
            SequenceConfigMutation::SetLastRun { json } => next.last_run_json = json.clone(),
            SequenceConfigMutation::SetOrientation { value } => next.orientation = value.clone(),
            SequenceConfigMutation::SetCamera { camera } => next.camera = camera.clone(),
            SequenceConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        protocol::MutationOutcome::new(next)
    }

    async fn inverse(&self, base: &SequenceConfig) -> Vec<Self> {
        vec![SequenceConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigMutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn sequence_config_default_matches_the_existing_runtime_defaults() {
        let config = SequenceConfig::default();
        assert!(config.last_run_json.is_empty());
        assert_eq!(config.orientation, "leftRight");
        assert_eq!(config.locale, "en-US");
    }

    #[semio_framework_async_macros::async_test]
    async fn sequence_config_dsl_round_trips() {
        let config = SequenceConfig { last_run_json: "{}".into(), orientation: "topBottom".into(), camera: SequenceCamera { x: 1.0, y: 2.0, zoom: 3.0 }, locale: "de-DE".into() };
        let text = store::ArtifactDsl::print_dsl(&config);
        let parsed = <SequenceConfig as store::ArtifactDsl>::parse_dsl(&text).expect("config dsl round trip");
        assert_eq!(parsed, config);
    }

    #[semio_framework_async_macros::async_test]
    async fn sequence_config_pack_round_trips() {
        let config = SequenceConfig { last_run_json: "{\"ok\":true}".into(), orientation: "leftRight".into(), camera: SequenceCamera::default(), locale: "en-US".into() };
        let bytes = store::ArtifactPack::encode_pack(&config);
        let decoded = <SequenceConfig as store::ArtifactPack>::decode_pack(&bytes).expect("config pack round trip");
        assert_eq!(decoded, config);
    }

    //#region 🔖️ConfigMutationTests
    async fn round_trip_config(config: &SequenceConfig, operation: &SequenceConfigMutation) -> SequenceConfig {
        let forward = operation.diff(config).diff().clone();
        let backwards = operation.inverse(config);
        assert_eq!(backwards.len(), 1);
        let restored = backwards[0].diff(&forward).diff().clone();
        assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[semio_framework_async_macros::async_test]
    async fn config_set_last_run_round_trips() {
        let config = SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigMutation::SetLastRun { json: "{\"ok\":true}".into() });
        assert_eq!(next.last_run_json, "{\"ok\":true}");
    }

    #[semio_framework_async_macros::async_test]
    async fn config_set_orientation_round_trips() {
        let config = SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigMutation::SetOrientation { value: "topBottom".into() });
        assert_eq!(next.orientation, "topBottom");
    }

    #[semio_framework_async_macros::async_test]
    async fn config_set_camera_round_trips() {
        let config = SequenceConfig::default();
        let camera = SequenceCamera { x: 5.0, y: 6.0, zoom: 2.0 };
        let next = round_trip_config(&config, &SequenceConfigMutation::SetCamera { camera: camera.clone() });
        assert_eq!(next.camera, camera);
    }

    #[semio_framework_async_macros::async_test]
    async fn config_set_locale_round_trips() {
        let config = SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigMutation::SetLocale { value: "de-DE".into() });
        assert_eq!(next.locale, "de-DE");
    }

    #[semio_framework_async_macros::async_test]
    async fn config_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::Snapshot { config: SequenceConfig::default() });
        store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::SetLastRun { json: "{}".into() });
        store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::SetOrientation { value: "leftRight".into() });
        store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::SetCamera { camera: SequenceCamera { x: 1.0, y: 2.0, zoom: 3.0 } });
        store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::SetLocale { value: "en-US".into() });
    }
    //#endregion 🔖️ConfigMutationTests
}
//#endregion 🧪️Tests
