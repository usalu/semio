//! 🧮️ Sequence play app — view state (`SequenceConfig`) and its operation enum
//! (`SequenceConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/`
//! because nothing in it survives into the `.sequence` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so selection/camera/orientation edits are VCS'd exactly
//! like document content.

use crate::artifacts::sequence::SequenceCamera;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ B1: sequence's real `DocumentApp::Config` — absorbs every former `SequencePlayRuntime` field
/// (`selected_step_ids`/`last_run_json`/`orientation`) plus the node-graph viewport camera
/// (session-only, never a document field) and the locale the pre-B1 host-pushed `ViewModel` used to
/// carry (see `crate::apps::sequence::terminology::sequence_play_labels`) — same "absorb every
/// runtime field" shape `shooting_engine::ShootingConfig` established for the pilot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "sequencecfg")]
#[dsl(id = "sequence.config")]
#[dsl(layout = "lines")]
pub struct SequenceConfig {
    /// 👁️ Selected step ids — was `SequencePlayRuntime::selected_step_ids`.
    pub selected_step_ids: Vec<String>,
    /// 🏃️ Last `run` command's `RunResult` JSON, rendered under the compiled script — was
    /// `SequencePlayRuntime::last_run_json`.
    pub last_run_json: String,
    /// 🌳️ Layered-layout flow direction (`"leftRight"`/`"topBottom"`) `reorganize` reads — was
    /// `SequencePlayRuntime::orientation`. Kept as a string rather than `DagLayoutOrientation`
    /// directly: that enum is foreign to this crate and only derives `Serialize`/`Deserialize`, not
    /// `dsl::DslField` (see `crate::apps::sequence::commands::layout`'s conversion helper).
    pub orientation: String,
    /// 🎥️ The node-graph viewport pan/zoom — session-only, never a document field. Was
    /// `SequencePlayRuntime::camera`.
    #[dsl(block)]
    pub camera: SequenceCamera,
    /// 🗣️ BCP-47 locale tag — was read off the host-pushed `ViewModel.locale`.
    pub locale: String,
}

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for SequenceConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted DocumentPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::DocumentPack for SequenceConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️DocumentCodec


impl Default for SequenceConfig {
    fn default() -> Self {
        Self { selected_step_ids: Vec::new(), last_run_json: String::new(), orientation: "leftRight".into(), camera: SequenceCamera::default(), locale: "en-US".into() }
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
    #[dsl(key = "selection")]
    SetSelection { step_ids: Vec<String> },
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
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for SequenceConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}

//#endregion 🔖️OpCodec


impl Mutation<SequenceConfig> for SequenceConfigMutation {
    type Diff = SequenceConfig;

    fn diff(&self, base: &SequenceConfig) -> SequenceConfig {
        let mut next = base.clone();
        match self {
            SequenceConfigMutation::Snapshot { config } => return config.clone(),
            SequenceConfigMutation::SetSelection { step_ids } => next.selected_step_ids = step_ids.clone(),
            SequenceConfigMutation::SetLastRun { json } => next.last_run_json = json.clone(),
            SequenceConfigMutation::SetOrientation { value } => next.orientation = value.clone(),
            SequenceConfigMutation::SetCamera { camera } => next.camera = camera.clone(),
            SequenceConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn inverse(&self, base: &SequenceConfig) -> Vec<Self> {
        vec![SequenceConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigMutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_config_default_matches_the_existing_runtime_defaults() {
        let config = SequenceConfig::default();
        assert!(config.selected_step_ids.is_empty());
        assert!(config.last_run_json.is_empty());
        assert_eq!(config.orientation, "leftRight");
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn sequence_config_dsl_round_trips() {
        let config = SequenceConfig { selected_step_ids: vec!["step-1".into()], last_run_json: "{}".into(), orientation: "topBottom".into(), camera: SequenceCamera { x: 1.0, y: 2.0, zoom: 3.0 }, locale: "de-DE".into() };
        let text = store::DocumentDsl::print_dsl(&config);
        let parsed = <SequenceConfig as store::DocumentDsl>::parse_dsl(&text).expect("config dsl round trip");
        assert_eq!(parsed, config);
    }

    #[test]
    fn sequence_config_pack_round_trips() {
        let config = SequenceConfig { selected_step_ids: vec!["step-2".into()], last_run_json: "{\"ok\":true}".into(), orientation: "leftRight".into(), camera: SequenceCamera::default(), locale: "en-US".into() };
        let bytes = store::DocumentPack::encode_pack(&config);
        let decoded = <SequenceConfig as store::DocumentPack>::decode_pack(&bytes).expect("config pack round trip");
        assert_eq!(decoded, config);
    }

    //#region 🔖️ConfigMutationTests
    fn round_trip_config(config: &SequenceConfig, operation: &SequenceConfigMutation) -> SequenceConfig {
        let forward = operation.diff(config);
        let backwards = operation.inverse(config);
        assert_eq!(backwards.len(), 1);
        let restored = backwards[0].diff(&forward);
        assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn config_set_selection_round_trips() {
        let config = SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigMutation::SetSelection { step_ids: vec!["step-1".into()] });
        assert_eq!(next.selected_step_ids, vec!["step-1".to_string()]);
    }

    #[test]
    fn config_set_last_run_round_trips() {
        let config = SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigMutation::SetLastRun { json: "{\"ok\":true}".into() });
        assert_eq!(next.last_run_json, "{\"ok\":true}");
    }

    #[test]
    fn config_set_orientation_round_trips() {
        let config = SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigMutation::SetOrientation { value: "topBottom".into() });
        assert_eq!(next.orientation, "topBottom");
    }

    #[test]
    fn config_set_camera_round_trips() {
        let config = SequenceConfig::default();
        let camera = SequenceCamera { x: 5.0, y: 6.0, zoom: 2.0 };
        let next = round_trip_config(&config, &SequenceConfigMutation::SetCamera { camera: camera.clone() });
        assert_eq!(next.camera, camera);
    }

    #[test]
    fn config_set_locale_round_trips() {
        let config = SequenceConfig::default();
        let next = round_trip_config(&config, &SequenceConfigMutation::SetLocale { value: "de-DE".into() });
        assert_eq!(next.locale, "de-DE");
    }

    #[test]
    fn config_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::Snapshot { config: SequenceConfig::default() });
        store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::SetSelection { step_ids: vec!["step-1".into(), "step-2".into()] });
        store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::SetLastRun { json: "{}".into() });
        store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::SetOrientation { value: "leftRight".into() });
        store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::SetCamera { camera: SequenceCamera { x: 1.0, y: 2.0, zoom: 3.0 } });
        store::os_store::test_support::assert_op_line_round_trip(&SequenceConfigMutation::SetLocale { value: "en-US".into() });
    }
    //#endregion 🔖️ConfigMutationTests
}
//#endregion 🧪️Tests
