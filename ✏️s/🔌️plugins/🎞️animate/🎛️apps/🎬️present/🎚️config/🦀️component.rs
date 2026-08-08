//! 🧮️ Animate present app — view state (`PresentConfig`) and its operation enum
//! (`PresentConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/`
//! because nothing in it survives into the `.present` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so selection/engagement/locale edits are VCS'd exactly
//! like document content.

use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ B1: animate present's real `DocumentApp::Config` — absorbs every former
/// `AnimatePresentPlayRuntime` field (`selected_ids`/`engagement_input`) plus the locale the pre-B1
/// host-pushed `ViewModel` used to carry (see `crate::apps::present::terminology`) — same "absorb every
/// runtime field" shape `shooting_engine::ShootingConfig` established for the pilot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "presentcfg")]
#[dsl(layout = "lines")]
pub struct PresentConfig {
    /// 👁️ Selected tile ids — was `AnimatePresentPlayRuntime::selected_ids`.
    pub selected_ids: Vec<String>,
    /// ⌨️ In-progress engagement-bar input draft — was `AnimatePresentPlayRuntime::engagement_input`.
    pub engagement_input: String,
    /// 🗣️ BCP-47 locale tag — was read off the host-pushed `ViewModel.locale`.
    pub locale: String,
}

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for PresentConfig {
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
impl store::DocumentPack for PresentConfig {
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


impl Default for PresentConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), engagement_input: String::new(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(PresentConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigMutations
/// 🧮️ B1: `PresentConfig`'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `AnimatePresentPlayRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()`
/// returns — same "whole-config snapshot is the simplest correct inverse" shape as
/// `shooting_op::ShootingConfigMutation`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum PresentConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: PresentConfig,
    },
    #[dsl(key = "selection")]
    SetSelectedIds { ids: Vec<String> },
    #[dsl(key = "engagement-input")]
    SetEngagementInput { value: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for PresentConfigMutation {
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
impl protocol::OpBinary for PresentConfigMutation {
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


impl Mutation<PresentConfig> for PresentConfigMutation {
    type Diff = PresentConfig;

    fn diff(&self, base: &PresentConfig) -> PresentConfig {
        let mut next = base.clone();
        match self {
            PresentConfigMutation::Snapshot { config } => return config.clone(),
            PresentConfigMutation::SetSelectedIds { ids } => next.selected_ids = ids.clone(),
            PresentConfigMutation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            PresentConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn inverse(&self, base: &PresentConfig) -> Vec<Self> {
        vec![PresentConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigMutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn present_config_default_matches_the_existing_runtime_defaults() {
        let config = PresentConfig::default();
        assert!(config.selected_ids.is_empty());
        assert!(config.engagement_input.is_empty());
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn present_config_dsl_round_trips() {
        let config = PresentConfig { selected_ids: vec!["t1".into()], engagement_input: "2x2".into(), locale: "de-DE".into() };
        let text = store::DocumentDsl::print_dsl(&config);
        let parsed = <PresentConfig as store::DocumentDsl>::parse_dsl(&text).expect("config dsl round trip");
        assert_eq!(parsed, config);
    }

    #[test]
    fn present_config_pack_round_trips() {
        let config = PresentConfig { selected_ids: vec!["t2".into()], engagement_input: "add".into(), locale: "en-US".into() };
        let bytes = store::DocumentPack::encode_pack(&config);
        let decoded = <PresentConfig as store::DocumentPack>::decode_pack(&bytes).expect("config pack round trip");
        assert_eq!(decoded, config);
    }

    //#region 🔖️ConfigMutationTests
    fn round_trip_config(config: &PresentConfig, operation: &PresentConfigMutation) -> PresentConfig {
        let forward = operation.diff(config);
        let backwards = operation.inverse(config);
        assert_eq!(backwards.len(), 1);
        let restored = backwards[0].diff(&forward);
        assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn config_set_selected_ids_round_trips() {
        let config = PresentConfig::default();
        let next = round_trip_config(&config, &PresentConfigMutation::SetSelectedIds { ids: vec!["t1".into()] });
        assert_eq!(next.selected_ids, vec!["t1".to_string()]);
    }

    #[test]
    fn config_set_engagement_input_round_trips() {
        let config = PresentConfig::default();
        let next = round_trip_config(&config, &PresentConfigMutation::SetEngagementInput { value: "2x2".into() });
        assert_eq!(next.engagement_input, "2x2");
    }

    #[test]
    fn config_set_locale_round_trips() {
        let config = PresentConfig::default();
        let next = round_trip_config(&config, &PresentConfigMutation::SetLocale { value: "de-DE".into() });
        assert_eq!(next.locale, "de-DE");
    }

    #[test]
    fn config_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&PresentConfigMutation::Snapshot { config: PresentConfig::default() });
        store::test_support::assert_op_line_round_trip(&PresentConfigMutation::SetSelectedIds { ids: vec!["t1".into(), "t2".into()] });
        store::test_support::assert_op_line_round_trip(&PresentConfigMutation::SetEngagementInput { value: "add".into() });
        store::test_support::assert_op_line_round_trip(&PresentConfigMutation::SetLocale { value: "en-US".into() });
    }
    //#endregion 🔖️ConfigMutationTests
}
//#endregion 🧪️Tests
