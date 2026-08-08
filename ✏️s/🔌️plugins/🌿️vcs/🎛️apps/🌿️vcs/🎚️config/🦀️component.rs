//! 🧮️ VCS play app — view state (`VcsDemoConfig`) and its operation enum (`VcsDemoConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.vcsdemo` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so selection/locale edits are VCS'd exactly like document
//! content. Absorbs the old `VcsPlayApp::selected_checkpoint_ids` `RefCell` field (multi-selected
//! checkpoint ids in the document tree) plus the `locale` field the UI used to read off the deleted
//! `ViewModel` (mirrors `shooting_engine::ShootingConfig`'s identical `locale` field/doc).

use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "vcscfg")]
#[dsl(layout = "lines")]
pub struct VcsDemoConfig {
    /// 👁️ Multi-selected checkpoint ids in the document tree — was `VcsPlayApp::selected_checkpoint_ids`.
    pub selected_checkpoint_ids: Vec<String>,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for VcsDemoConfig {
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
impl store::DocumentPack for VcsDemoConfig {
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


impl Default for VcsDemoConfig {
    fn default() -> Self {
        Self { selected_checkpoint_ids: Vec::new(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(VcsDemoConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigMutations
/// 🧮️ [`VcsDemoConfig`]'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `VcsPlayApp` field writes/deleted `ViewModel.locale`), plus a generic `Snapshot` every variant's
/// `backwards()` returns (see `shooting_op::ShootingConfigMutation`'s identical doc for why this
/// whole-config-snapshot-undo shape is correct and sufficient here).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum VcsDemoConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: VcsDemoConfig,
    },
    #[dsl(key = "selection")]
    SetSelection { checkpoint_ids: Vec<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for VcsDemoConfigMutation {
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
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for VcsDemoConfigMutation {
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


impl Mutation<VcsDemoConfig> for VcsDemoConfigMutation {
    type Diff = VcsDemoConfig;

    fn diff(&self, base: &VcsDemoConfig) -> VcsDemoConfig {
        let mut next = base.clone();
        match self {
            VcsDemoConfigMutation::Snapshot { config } => return config.clone(),
            VcsDemoConfigMutation::SetSelection { checkpoint_ids } => next.selected_checkpoint_ids = checkpoint_ids.clone(),
            VcsDemoConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn inverse(&self, base: &VcsDemoConfig) -> Vec<Self> {
        vec![VcsDemoConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigMutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vcs_demo_config_default_is_empty_selection_and_english_locale() {
        let config = VcsDemoConfig::default();
        assert!(config.selected_checkpoint_ids.is_empty());
        assert_eq!(config.locale, "en-US");
    }

    /// 🧮️ Round-trip law (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE): a
    /// non-default fixture must survive `DocumentDsl`/`DocumentPack` byte-for-byte.
    #[test]
    fn vcs_demo_config_dsl_pack_round_trips() {
        let config = VcsDemoConfig { selected_checkpoint_ids: vec!["checkpoint-1".into(), "checkpoint-2".into()], locale: "de-DE".into() };
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    /// 🧮️ Round-trip law per `VcsDemoConfigMutation` variant (WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-
    /// SCHEMA-FLOW-CONFIG-ON-NODE).
    #[test]
    fn vcs_demo_config_operation_op_text_round_trips() {
        store::test_support::assert_op_line_round_trip(&VcsDemoConfigMutation::Snapshot { config: VcsDemoConfig { selected_checkpoint_ids: vec!["checkpoint-1".into()], locale: "de-DE".into() } });
        store::test_support::assert_op_line_round_trip(&VcsDemoConfigMutation::SetSelection { checkpoint_ids: vec!["checkpoint-1".into(), "checkpoint-2".into()] });
        store::test_support::assert_op_line_round_trip(&VcsDemoConfigMutation::SetLocale { value: "de-DE".into() });
    }

    /// ⏪️ `backwards()` always returns a `Snapshot` of the pre-operation config, so applying it after
    /// the forward op exactly restores the original — the "whole-config-snapshot-undo" law.
    #[test]
    fn vcs_demo_config_operation_backwards_restores_the_base_config() {
        let base = VcsDemoConfig { selected_checkpoint_ids: vec!["checkpoint-1".into()], locale: "en-US".into() };
        let operation = VcsDemoConfigMutation::SetLocale { value: "de-DE".into() };
        let forward = operation.diff(&base);
        assert_eq!(forward.locale, "de-DE");
        let backwards = operation.inverse(&base);
        assert_eq!(backwards, vec![VcsDemoConfigMutation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&forward);
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
