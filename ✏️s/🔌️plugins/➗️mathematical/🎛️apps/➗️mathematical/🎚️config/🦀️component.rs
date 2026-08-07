//! 🧮️ Mathematical play app — view state (`MathConfig`) and its operation enum
//! (`MathConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.mathematical` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so camera/locale edits are VCS'd exactly like document
//! content — absorbs the former app-struct `RefCell` (`MathPlayRuntime::camera`, the node-graph viewport)
//! plus the locale the UI used to read off the deleted `ViewModel`.

use crate::artifacts::mathematical::MathCamera;
use protocol::Operation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "mathematicalcfg")]
#[dsl(layout = "lines")]
pub struct MathConfig {
    /// 🎥️ Node-graph viewport camera — session-only, never a document field. Was
    /// `MathPlayRuntime::camera`.
    #[dsl(block)]
    pub camera: MathCamera,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for MathConfig {
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
impl store::DocumentPack for MathConfig {
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


impl Default for MathConfig {
    fn default() -> Self {
        Self { camera: MathCamera::default(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(MathConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ `MathConfig`'s operation enum — one variant per settled interaction (mirrors the pre-migration
/// `MathPlayRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()` returns —
/// mirrors `shooting_op::ShootingConfigOperation`'s "undo this tick is exactly restore the whole-config
/// snapshot from just before it" pattern: `Operation::Diff` is the WHOLE `MathConfig` (not a granular
/// patch type), `diff()` returns "the full config after this op", and
/// `protocol::OperationDiff<MathConfig>::apply` for `MathConfig` itself (see `store::impl_whole_record_config!`)
/// just returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum MathConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: MathConfig,
    },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: MathCamera,
    },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for MathConfigOperation {
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
impl protocol::OpBinary for MathConfigOperation {
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


impl Operation<MathConfig> for MathConfigOperation {
    type Diff = MathConfig;

    fn diff(&self, base: &MathConfig) -> MathConfig {
        let mut next = base.clone();
        match self {
            MathConfigOperation::Snapshot { config } => return config.clone(),
            MathConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            MathConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &MathConfig) -> Vec<Self> {
        vec![MathConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_config_default_is_the_identity_camera_and_english_locale() {
        let config = MathConfig::default();
        assert_eq!(config.camera, MathCamera::default());
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    fn math_config_dsl_round_trips() {
        let config = MathConfig { camera: MathCamera { x: 5.0, y: 6.0, zoom: 2.0 }, locale: "de-DE".into() };
        store::test_support::assert_dsl_round_trip(&config);
        store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[test]
    fn config_operation_snapshot_diff_ignores_base() {
        let base = MathConfig::default();
        let mut snapshot = base.clone();
        snapshot.locale = "de-DE".into();
        let operation = MathConfigOperation::Snapshot { config: snapshot.clone() };
        assert_eq!(Operation::diff(&operation, &base), snapshot);
    }

    #[test]
    fn config_operation_set_camera_round_trips() {
        let base = MathConfig::default();
        let camera = MathCamera { x: 5.0, y: 6.0, zoom: 2.0 };
        let operation = MathConfigOperation::SetCamera { camera: camera.clone() };
        let next = Operation::diff(&operation, &base);
        assert_eq!(next.camera, camera);
        let backwards = Operation::backwards(&operation, &base);
        assert_eq!(backwards, vec![MathConfigOperation::Snapshot { config: base }]);
        store::test_support::assert_op_line_round_trip(&operation);
    }

    #[test]
    fn config_operation_set_locale_round_trips() {
        store::test_support::assert_op_line_round_trip(&MathConfigOperation::SetLocale { value: "de-DE".into() });
    }
}
//#endregion 🧪️Tests
