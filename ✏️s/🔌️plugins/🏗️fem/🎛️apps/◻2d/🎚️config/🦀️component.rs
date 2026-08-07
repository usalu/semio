//! 🧮️ Fem2d play app — view state (`Fem2dConfig`) and its operation enum (`Fem2dConfigOperation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.fem2d` document. It still round-trips through a real
//! `DocumentStore` (with a real `backwards`), so result-display/camera/locale edits are VCS'd exactly
//! like document content.

use crate::artifacts::fem2d::FemCamera;
use protocol::Operation;
use serde::{Deserialize, Serialize};

// #region 🔖️Config
/// 🧮️ B1: fem2d's real `DocumentApp::Config` — the pure-trait pilot's config artifact. Absorbs both
/// former `Fem2dPlayApp` `RefCell` fields (`result_display`, `camera`) plus the locale the deleted
/// `ViewModel` used to carry into label resolution — session-only view state now round-trips through
/// the config `DocumentStore` exactly like document content, with a real `backwards` per
/// [`Fem2dConfigOperation`] instead of never being VCS'd at all.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslDocument)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "fem2dcfg")]
#[dsl(layout = "lines")]
pub struct Fem2dConfig {
    /// 👁️ The results window's selected case/combination id — was `fem_shared::ResultDisplay::source_id`.
    pub result_source_id: Option<String>,
    /// 👁️ The results window's display mode (`"static"`/`"modal"`/`"buckling"`) — was
    /// `crate::app_surface::DisplayMode`'s discriminant. Kept as a flat string rather than depending on
    /// `crate::model::shared` from the artifact's `engine` (ui-scoped) — the app's window render
    /// translates to/from `crate::app_surface::DisplayMode` at the render boundary.
    pub result_mode: String,
    /// 👁️ The selected modal/buckling mode index — was `crate::app_surface::DisplayMode::Modal`/
    /// `Buckling`'s payload.
    pub result_mode_index: u32,
    /// 🎥️ The canvas camera (pan/zoom) — was `Fem2dPlayApp::camera`.
    #[dsl(block)]
    pub camera: FemCamera,
    /// 🗣️ BCP-47 locale tag — was read off the deleted `ViewModel::locale`.
    pub locale: String,
}

//#region 🔖️DocumentCodec
/// 📜️ Handcrafted DocumentDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::DocumentDsl for Fem2dConfig {
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
impl store::DocumentPack for Fem2dConfig {
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


impl Default for Fem2dConfig {
    fn default() -> Self {
        Self { result_source_id: None, result_mode: "static".into(), result_mode_index: 0, camera: FemCamera::default(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(Fem2dConfig);
// #endregion 🔖️Config

// #region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `Fem2dConfig`'s operation enum — one variant per settled interaction (mirrors the
/// pre-B1 `Fem2dPlayApp` `RefCell` field writes), plus a generic `Snapshot` every variant's
/// `backwards()` returns — mirrors `ShootingConfigOperation`'s identical B1 pilot recipe: since a
/// config-only dispatch is a plain `Apply` (not an `AmendLast`), each tick is its own distinct, real
/// config edit, and "undo this tick" is exactly "restore the whole-config snapshot from just before
/// it". `Operation::Diff` is the WHOLE `Fem2dConfig` (not a granular patch type): `diff()` returns "the
/// full config after this op", and `store::impl_whole_record_config!` supplies the
/// `OperationDiff<Fem2dConfig>` that returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Fem2dConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Fem2dConfig,
    },
    /// 👁️ Was the `setResultDisplay` view action writing `Fem2dPlayApp::result_display`.
    #[dsl(key = "result-display")]
    SetResultDisplay { source_id: Option<String>, mode: String, mode_index: u32 },
    /// 🎥️ Was the `setCamera` view action writing `Fem2dPlayApp::camera`.
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: FemCamera,
    },
    /// 🗣️ Was read off the deleted `ViewModel::locale`.
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for Fem2dConfigOperation {
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
impl protocol::OpBinary for Fem2dConfigOperation {
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


impl Operation<Fem2dConfig> for Fem2dConfigOperation {
    type Diff = Fem2dConfig;

    fn diff(&self, base: &Fem2dConfig) -> Fem2dConfig {
        let mut next = base.clone();
        match self {
            Fem2dConfigOperation::Snapshot { config } => return config.clone(),
            Fem2dConfigOperation::SetResultDisplay { source_id, mode, mode_index } => {
                next.result_source_id = source_id.clone();
                next.result_mode = mode.clone();
                next.result_mode_index = *mode_index;
            }
            Fem2dConfigOperation::SetCamera { camera } => next.camera = camera.clone(),
            Fem2dConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &Fem2dConfig) -> Vec<Self> {
        vec![Fem2dConfigOperation::Snapshot { config: base.clone() }]
    }
}
// #endregion 🔖️ConfigOperations

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fem2d_config_default_is_static_display_with_default_camera_and_locale() {
        let config = Fem2dConfig::default();
        assert_eq!(config.result_mode, "static");
        assert!(config.result_source_id.is_none());
        assert_eq!(config.result_mode_index, 0);
        assert_eq!(config.camera, FemCamera::default());
        assert_eq!(config.locale, "en-US");
    }

    /// 🧮️ `Fem2dConfig`'s `OperationDiff` is a whole-record replace, mirroring `ShootingConfig`'s
    /// identical B1 pilot pattern: `apply` ignores `base` entirely.
    #[test]
    fn fem2d_config_operation_diff_is_a_whole_record_replace() {
        let base = Fem2dConfig::default();
        let mut replacement = Fem2dConfig::default();
        replacement.locale = "de-DE".into();
        replacement.camera = FemCamera { x: 1.0, y: 2.0, zoom: 3.0 };
        let applied = protocol::OperationDiff::apply(&replacement, &base);
        assert_eq!(applied, replacement);
        let mut absorbed = base.clone();
        protocol::OperationDiff::absorb(&mut absorbed, replacement.clone());
        assert_eq!(absorbed, replacement);
    }

    #[test]
    fn config_operation_backwards_always_restores_the_pre_operation_snapshot() {
        let base = Fem2dConfig::default();
        let camera = FemCamera { x: 1.0, y: 2.0, zoom: 3.0 };
        let op = Fem2dConfigOperation::SetCamera { camera: camera.clone() };
        let next = op.diff(&base);
        assert_eq!(next.camera, camera);
        let backwards = op.backwards(&base);
        assert_eq!(backwards, vec![Fem2dConfigOperation::Snapshot { config: base.clone() }]);
        assert_eq!(backwards[0].diff(&next), base);
    }

    #[test]
    fn set_result_display_config_operation_round_trips() {
        let base = Fem2dConfig::default();
        let op = Fem2dConfigOperation::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 2 };
        let next = op.diff(&base);
        assert_eq!(next.result_source_id.as_deref(), Some("dead"));
        assert_eq!(next.result_mode, "modal");
        assert_eq!(next.result_mode_index, 2);
    }

    #[test]
    fn set_locale_config_operation_round_trips() {
        let base = Fem2dConfig::default();
        let op = Fem2dConfigOperation::SetLocale { value: "de-DE".into() };
        let next = op.diff(&base);
        assert_eq!(next.locale, "de-DE");
    }

    #[test]
    fn fem2d_config_operation_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&Fem2dConfigOperation::Snapshot { config: Fem2dConfig::default() });
        store::test_support::assert_op_line_round_trip(&Fem2dConfigOperation::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 1 });
        store::test_support::assert_op_line_round_trip(&Fem2dConfigOperation::SetCamera { camera: FemCamera { x: 1.0, y: 2.0, zoom: 1.5 } });
        store::test_support::assert_op_line_round_trip(&Fem2dConfigOperation::SetLocale { value: "de-DE".into() });
    }
}
// #endregion 🧪️Tests
