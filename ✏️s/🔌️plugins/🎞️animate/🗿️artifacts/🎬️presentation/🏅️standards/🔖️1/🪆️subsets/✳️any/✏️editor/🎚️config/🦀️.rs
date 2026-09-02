//! 🧮️ Animate presentation app — view state (`PresentationConfig`) and its operation enum
//! (`PresentationConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/`
//! because nothing in it survives into the `.presentation` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so selection/engagement/locale edits are VCS'd exactly
//! like document content.

use protocol::Mutation;

//#region 🔖️Config
/// 🧮️ B1: animate presentation's real `ArtifactApp::Config` — absorbs every former
/// `AnimatePresentationPlayRuntime` field (`selected_ids`/`engagement_input`) plus the locale the pre-B1
/// host-pushed `ViewModel` used to carry (see `crate::editor::animate::terminology`) — same "absorb every
/// runtime field" shape `shooting_engine::ShootingConfig` established for the pilot.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "presentcfg")]
#[dsl(id = "presentation.config")]
#[dsl(layout = "lines")]
pub struct PresentationConfig {
    /// ⌨️ In-progress engagement-bar input draft — was `AnimatePresentationPlayRuntime::engagement_input`.
    pub engagement_input: String,
    /// 🗣️ BCP-47 locale tag — was read off the host-pushed `ViewModel.locale`.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for PresentationConfig {
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
impl store::ArtifactPack for PresentationConfig {
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

impl Default for PresentationConfig {
    fn default() -> Self {
        Self { engagement_input: String::new(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(PresentationConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigMutations
/// 🧮️ B1: `PresentationConfig`'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `AnimatePresentationPlayRuntime` field writes). Every field already carries its own setter, so
/// `backwards()` returns the SAME variant re-addressed at `base`'s old value — a targeted, in-kind
/// inverse per this ticket's ban on whole-record replace, rather than a generic whole-config
/// snapshot.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum PresentationConfigMutation {
    #[dsl(key = "engagement-input")]
    SetEngagementInput { value: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for PresentationConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
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
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for PresentationConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
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
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
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

impl Mutation<PresentationConfig> for PresentationConfigMutation {
    type Diff = PresentationConfig;

    fn diff(&self, base: &PresentationConfig) -> protocol::MutationOutcome<PresentationConfig> {
        let mut next = base.clone();
        match self {
            PresentationConfigMutation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            PresentationConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &PresentationConfig) -> Vec<Self> {
        match self {
            PresentationConfigMutation::SetEngagementInput { .. } => vec![PresentationConfigMutation::SetEngagementInput { value: base.engagement_input.clone() }],
            PresentationConfigMutation::SetLocale { .. } => vec![PresentationConfigMutation::SetLocale { value: base.locale.clone() }],
        }
    }
}
//#endregion 🔖️ConfigMutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presentation_config_default_matches_the_existing_runtime_defaults() {
        let config = PresentationConfig::default();
        assert!(config.engagement_input.is_empty());
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn presentation_config_dsl_round_trips() {
        let config = PresentationConfig { engagement_input: "2x2".into(), locale: "de-DE".into() };
        let text = store::ArtifactDsl::print_dsl(&config);
        let parsed = <PresentationConfig as store::ArtifactDsl>::parse_dsl(&text).expect("config dsl round trip");
        assert_eq!(parsed, config);
    }

    #[test]
    fn presentation_config_pack_round_trips() {
        let config = PresentationConfig { engagement_input: "add".into(), locale: "en-US".into() };
        let bytes = store::ArtifactPack::encode_pack(&config);
        let decoded = <PresentationConfig as store::ArtifactPack>::decode_pack(&bytes).expect("config pack round trip");
        assert_eq!(decoded, config);
    }

    //#region 🔖️ConfigMutationTests
    fn round_trip_config(config: &PresentationConfig, operation: &PresentationConfigMutation) -> PresentationConfig {
        let forward = operation.diff(config).diff().clone();
        let backwards = operation.inverse(config);
        assert_eq!(backwards.len(), 1);
        let restored = backwards[0].diff(&forward).diff().clone();
        assert_eq!(&restored, config, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn config_set_engagement_input_round_trips() {
        let config = PresentationConfig::default();
        let next = round_trip_config(&config, &PresentationConfigMutation::SetEngagementInput { value: "2x2".into() });
        assert_eq!(next.engagement_input, "2x2");
    }

    #[test]
    fn config_set_locale_round_trips() {
        let config = PresentationConfig::default();
        let next = round_trip_config(&config, &PresentationConfigMutation::SetLocale { value: "de-DE".into() });
        assert_eq!(next.locale, "de-DE");
    }

    #[test]
    fn config_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&PresentationConfigMutation::SetEngagementInput { value: "add".into() });
        store::os_store::test_support::assert_op_line_round_trip(&PresentationConfigMutation::SetLocale { value: "en-US".into() });
    }
    //#endregion 🔖️ConfigMutationTests
}
//#endregion 🧪️Tests
