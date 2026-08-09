//! 🧮️ Playbook play app — view state (`PlaybookConfig`) and its operation enum (`PlaybookConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.playbook` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so selection edits are VCS'd exactly like document content.
//! B1: absorbs the former app-struct `RefCell<Vec<String>>` selection state, plus `locale` (was read off
//! `view_state.locale`) — mirrors `writer_engine::WriterConfig`/`forms::config::FormsConfig`'s B1 shape.

use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ `PlaybookPlayApp::Config` — the pure-trait `DocumentApp::Config` for the playbook app.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "playbookcfg")]
#[dsl(layout = "lines")]
pub struct PlaybookConfig {
    /// 👁️ Selected step/block ids — was `PlaybookPlayApp::selected_ids`.
    pub selected_ids: Vec<String>,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
    /// 🧩️ Host-pushed `ProgramContributionEntry[]` JSON for `playbook.blockKind` hot-swap installs.
    #[serde(default = "default_contributions_json")]
    pub contributions_json: String,
}

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for PlaybookConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        "playbook.config"
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
impl store::DocumentPack for PlaybookConfig {
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


fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for PlaybookConfig {
    fn default() -> Self {
        Self { selected_ids: Vec::new(), locale: "en-US".into(), contributions_json: default_contributions_json() }
    }
}

store::impl_whole_record_config!(PlaybookConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ B1: `PlaybookConfig`'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `PlaybookPlayApp::selected_ids` field write), plus a generic `Snapshot` every variant's `backwards()`
/// returns — mirrors `writer_op::WriterConfigMutation`/`forms::config::FormsConfigMutation` exactly
/// (see either's doc comment for the whole-config-snapshot inverse rationale). Lives here, not in the
/// kernel `playbook` crate, since `PlaybookConfig` is this app's own config artifact, not shared domain
/// state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum PlaybookConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: PlaybookConfig,
    },
    #[dsl(key = "selected-ids")]
    SetSelectedIds { ids: Vec<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "contributions")]
    SetContributions { json: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for PlaybookConfigMutation {
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
impl protocol::OpBinary for PlaybookConfigMutation {
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


impl Mutation<PlaybookConfig> for PlaybookConfigMutation {
    type Diff = PlaybookConfig;

    fn diff(&self, base: &PlaybookConfig) -> PlaybookConfig {
        let mut next = base.clone();
        match self {
            PlaybookConfigMutation::Snapshot { config } => return config.clone(),
            PlaybookConfigMutation::SetSelectedIds { ids } => next.selected_ids = ids.clone(),
            PlaybookConfigMutation::SetLocale { value } => next.locale = value.clone(),
            PlaybookConfigMutation::SetContributions { json } => next.contributions_json = json.clone(),
        }
        next
    }

    fn inverse(&self, base: &PlaybookConfig) -> Vec<Self> {
        vec![PlaybookConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playbook_config_default_matches_the_existing_runtime_defaults() {
        let config = PlaybookConfig::default();
        assert!(config.selected_ids.is_empty());
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn playbook_config_dsl_round_trips_default_and_populated() {
        store::os_store::test_support::assert_config_round_trip(&PlaybookConfig::default());
        let populated = PlaybookConfig { selected_ids: vec!["step-1".into(), "block-1".into()], locale: "de-DE".into(), contributions_json: "[]".into() };
        store::os_store::test_support::assert_config_round_trip(&populated);
    }

    #[test]
    fn playbook_config_pack_round_trips() {
        let config = PlaybookConfig { selected_ids: vec!["block-1".into()], locale: "de-DE".into(), contributions_json: "[]".into() };
        let bytes = store::DocumentPack::encode_pack(&config);
        let decoded = <PlaybookConfig as store::DocumentPack>::decode_pack(&bytes).expect("decode playbook config pack");
        assert_eq!(decoded, config);
    }

    fn config_round_trip(base: &PlaybookConfig, operation: &PlaybookConfigMutation) -> PlaybookConfig {
        let forward = operation.diff(base);
        let backwards = operation.inverse(base);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = back.diff(&restored);
        }
        assert_eq!(&restored, base, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn config_mutations_apply_and_restore_every_field() {
        let base = PlaybookConfig::default();
        assert_eq!(config_round_trip(&base, &PlaybookConfigMutation::SetSelectedIds { ids: vec!["block-1".into()] }).selected_ids, vec!["block-1".to_string()]);
        assert_eq!(config_round_trip(&base, &PlaybookConfigMutation::SetLocale { value: "de-DE".into() }).locale, "de-DE");
        assert_eq!(config_round_trip(&base, &PlaybookConfigMutation::SetContributions { json: "[]".into() }).contributions_json, "[]");
    }

    #[test]
    fn playbook_config_operation_binary_matches_text() {
        store::os_store::test_support::assert_op_text_binary_equivalence(&PlaybookConfigMutation::SetLocale { value: "de-DE".into() });
        store::os_store::test_support::assert_op_text_binary_equivalence(&PlaybookConfigMutation::Snapshot { config: PlaybookConfig::default() });
    }
}
//#endregion 🧪️Tests
