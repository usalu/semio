//! 🧮️ Norm plugin — the ONE view-state config artifact every one of the fifteen norm apps uses.
//!
//! 📌️ Deliberately NOT a per-surface `✏️editor/🎚️config/🦀️component.rs`: all fifteen compliance apps have the
//! identical config shape (one field — which `CheckReport::checks` row the inspection panel points at),
//! so unlike `shooting`'s per-app `ShootingConfig` this is ONE type reused by every app rather than
//! fifteen byte-identical copies. It lives in `🫀️core` (the cross-artifact/cross-app kernel) because
//! that is the shallowest taxonomy node common to every consumer — the same "put shared declarations at
//! the shallowest common ancestor" rule the migration template states for shared window options.

use protocol::Mutation;
use serde::{Deserialize, Serialize};

pub use crate::document::NormHost;

//#region 🔖️Config
/// 🧮️ The shared `ArtifactApp::Config` for every norm family app — one field: which
/// `CheckReport::checks` row the inspection panel currently renders.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(id = "norm.config", extension = "normcfg")]
#[dsl(layout = "lines")]
pub struct NormConfig {
    /// 👁️ Which `CheckReport::checks` row the inspection panel renders — `None` (the default) means
    /// "the first check".
    pub selected_check_index: Option<u32>,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for NormConfig {
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
impl store::ArtifactPack for NormConfig {
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

impl store::ConfigRecord for NormConfig {}

/// 🧮️ Whole-record diff for `NormConfigMutation` — `apply` ignores `base` entirely, since
/// `NormConfigMutation::Snapshot` already carries the full post-op config.
impl protocol::MutationDiff<NormConfig> for NormConfig {
    async fn apply(&self, _base: &NormConfig) -> protocol::MutationApplyResult<NormConfig> {
        Ok({ self.clone() })
    }
    async fn absorb(&mut self, other: Self) {
        *self = other;
    }
}
//#endregion 🔖️Config

//#region 🔖️ConfigMutations
/// 🧮️ `NormConfig`'s mutation enum — `Snapshot` is the generic whole-config inverse every other
/// variant's `inverse()` returns (the simplest correct inverse for a config this small);
/// `SetSelectedCheckIndex` is the one real per-field edit every norm family app dispatches.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum NormConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: NormConfig,
    },
    #[dsl(key = "selected-check")]
    SetSelectedCheckIndex { index: Option<u32> },
}

//#region 🔖️OpCodec
impl protocol::OpText for NormConfigMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for NormConfigMutation {
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

impl Mutation<NormConfig> for NormConfigMutation {
    type Diff = NormConfig;

    async fn diff(&self, base: &NormConfig) -> protocol::MutationOutcome<NormConfig> {
        match self {
            NormConfigMutation::Snapshot { config } => {
                if config == base {
                    return protocol::MutationOutcome::new(base.clone()).warn("mutation.no-op", "Config snapshot is already up to date.");
                }
                protocol::MutationOutcome::new(config.clone())
            }
            NormConfigMutation::SetSelectedCheckIndex { index } => {
                if base.selected_check_index == *index {
                    return protocol::MutationOutcome::new(base.clone()).warn("mutation.no-op", "Selected check index is already this value.");
                }
                protocol::MutationOutcome::new(NormConfig { selected_check_index: *index })
            }
        }
    }

    async fn inverse(&self, base: &NormConfig) -> Vec<Self> {
        vec![NormConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigMutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn norm_config_dsl_round_trips() {
        store::os_store::test_support::assert_dsl_round_trip(&NormConfig::default());
        store::os_store::test_support::assert_dsl_round_trip(&NormConfig { selected_check_index: Some(3) });
    }

    #[semio_framework_async_macros::async_test]
    async fn norm_config_dsl_pack_equivalence() {
        store::os_store::test_support::assert_dsl_pack_equivalence(&NormConfig::default());
        store::os_store::test_support::assert_dsl_pack_equivalence(&NormConfig { selected_check_index: Some(7) });
    }

    #[semio_framework_async_macros::async_test]
    async fn norm_config_operation_snapshot_is_a_real_inverse() {
        let base = NormConfig { selected_check_index: Some(1) };
        let op = NormConfigMutation::SetSelectedCheckIndex { index: Some(5) };
        let next = op.diff(&base).diff().clone();
        assert_eq!(next.selected_check_index, Some(5));
        let backwards = op.inverse(&base);
        assert_eq!(backwards, vec![NormConfigMutation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&next).diff().clone();
        assert_eq!(restored, base);
    }

    #[semio_framework_async_macros::async_test]
    async fn norm_config_operation_op_text_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&NormConfigMutation::SetSelectedCheckIndex { index: Some(2) });
        store::os_store::test_support::assert_op_line_round_trip(&NormConfigMutation::SetSelectedCheckIndex { index: None });
        store::os_store::test_support::assert_op_line_round_trip(&NormConfigMutation::Snapshot { config: NormConfig { selected_check_index: Some(9) } });
    }

    /// 🧷️ Pins the config operations' exact pre-migration wire bytes (from the ticket's
    /// `🧪️wire-baseline-before.txt`) — `NormConfig` moved file but must not move format.
    #[semio_framework_async_macros::async_test]
    async fn config_mutations_keep_their_pre_migration_bytes() {
        let hex = |op: &NormConfigMutation| protocol::OpBinary::encode_op(op).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        assert_eq!(hex(&NormConfigMutation::Snapshot { config: NormConfig::default() }), "01000001000e0d00");
        assert_eq!(hex(&NormConfigMutation::Snapshot { config: NormConfig { selected_check_index: Some(9) } }), "01000001000e0d01000409");
        assert_eq!(hex(&NormConfigMutation::SetSelectedCheckIndex { index: Some(2) }), "01010001000402");
        assert_eq!(hex(&NormConfigMutation::SetSelectedCheckIndex { index: None }), "01010000");
    }
}
//#endregion 🧪️Tests
