//! 🧮️ FEM 3D app — view-state config + its operation enum (constitutional: engine's `Config` region +
//! op's `ConfigOperations` region, both moved here since config is app-level view state, not document
//! content).

use crate::artifacts::fem3d::FemCamera;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};

// #region 🔖️Config
/// 🧮️ B1: fem3d's real `ArtifactApp::Config` — absorbs both former `Fem3dPlayApp` `RefCell` fields
/// (`result_display`, `camera`); session-only view state now round-trips through the config
/// `ArtifactStore` exactly like document content, with a real `backwards` per `Fem3dConfigMutation`
/// instead of never being VCS'd at all. Mirrors `Fem2dConfig`'s identical B1 recipe, minus a `locale`
/// field (fem3d never carried a `ViewModel::locale` the way fem2d did).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "fem3dcfg")]
#[dsl(layout = "lines")]
pub struct Fem3dConfig {
    /// 👁️ The results window's selected case/combination id — was `fem_shared::ResultDisplay::source_id`.
    pub result_source_id: Option<String>,
    /// 👁️ The results window's display mode (`"static"`/`"modal"`/`"buckling"`) — was
    /// `fem_shared::DisplayMode`'s discriminant. Kept as a flat string rather than depending on
    /// `crate::app_surface::DisplayMode` from `Fem3dConfig` itself — the app translates to/from
    /// `crate::app_surface::DisplayMode` at the render boundary (see
    /// `crate::editor::fem3d::modes::edit::windows::results::config_result_display`).
    pub result_mode: String,
    /// 👁️ The selected modal/buckling mode index — was `fem_shared::DisplayMode::Modal`/`Buckling`'s payload.
    pub result_mode_index: u32,
    /// 🎥️ The world-3d camera (opaque host JSON) — was `Fem3dPlayApp::camera`.
    #[dsl(block)]
    pub camera: FemCamera,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for Fem3dConfig {
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
impl store::ArtifactPack for Fem3dConfig {
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

impl Default for Fem3dConfig {
    fn default() -> Self {
        Self { result_source_id: None, result_mode: "static".into(), result_mode_index: 0, camera: FemCamera::default() }
    }
}

store::impl_whole_record_config!(Fem3dConfig);
// #endregion 🔖️Config

// #region 🔖️ConfigOperations
/// 🧮️ B1: `Fem3dConfig`'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `Fem3dPlayApp` `RefCell` field writes), plus a generic `Snapshot` every variant's `backwards()`
/// returns — mirrors `Fem2dConfigMutation`'s identical B1 pilot recipe: since a config-only dispatch is
/// a plain `Apply` (not an `AmendLast`), each tick is its own distinct, real config edit, and "undo this
/// tick" is exactly "restore the whole-config snapshot from just before it". `Mutation::Diff` is the
/// WHOLE `Fem3dConfig` (not a granular patch type): `diff()` returns "the full config after this op", and
/// `MutationDiff<Fem3dConfig>::apply` for `Fem3dConfig` itself (`store::impl_whole_record_config!`) just
/// returns that snapshot verbatim, ignoring `base`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslOps)]
#[value(tag = "kind")]
pub enum Fem3dConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Fem3dConfig,
    },
    /// 👁️ Was the `setResultDisplay` view action writing `Fem3dPlayApp::result_display`.
    #[dsl(key = "result-display")]
    SetResultDisplay { source_id: Option<String>, mode: String, mode_index: u32 },
    /// 🎥️ Was the `setCamera` view action writing `Fem3dPlayApp::camera`.
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: FemCamera,
    },
}

//#region 🔖️OpCodec
impl protocol::OpText for Fem3dConfigMutation {
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
impl protocol::OpBinary for Fem3dConfigMutation {
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

impl Mutation<Fem3dConfig> for Fem3dConfigMutation {
    type Diff = Fem3dConfig;

    fn diff(&self, base: &Fem3dConfig) -> protocol::MutationOutcome<Fem3dConfig> {
        let mut next = base.clone();
        match self {
            Fem3dConfigMutation::Snapshot { config } => return protocol::MutationOutcome::new(config.clone()),
            Fem3dConfigMutation::SetResultDisplay { source_id, mode, mode_index } => {
                next.result_source_id = source_id.clone();
                next.result_mode = mode.clone();
                next.result_mode_index = *mode_index;
            }
            Fem3dConfigMutation::SetCamera { camera } => next.camera = camera.clone(),
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Fem3dConfig) -> Vec<Self> {
        vec![Fem3dConfigMutation::Snapshot { config: base.clone() }]
    }
}
// #endregion 🔖️ConfigOperations

// #region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fem3d_config_default_is_static_display_with_default_camera() {
        let config = Fem3dConfig::default();
        assert_eq!(config.result_mode, "static");
        assert!(config.result_source_id.is_none());
        assert_eq!(config.result_mode_index, 0);
        assert_eq!(config.camera, FemCamera::default());
    }

    /// 🧮️ `Fem3dConfig`'s `MutationDiff` is a whole-record replace, mirroring `Fem2dConfig`'s identical
    /// B1 pilot pattern: `apply` ignores `base` entirely.
    #[test]
    fn fem3d_config_operation_diff_is_a_whole_record_replace() {
        let base = Fem3dConfig::default();
        let replacement = Fem3dConfig { result_source_id: Some("dead".into()), result_mode: "modal".into(), result_mode_index: 2, camera: FemCamera { json: "{\"x\":1}".into() } };
        let applied = protocol::MutationDiff::apply(&replacement, &base).expect("valid config mutation diff");
        assert_eq!(applied, replacement);
        let mut absorbed = base.clone();
        protocol::MutationDiff::absorb(&mut absorbed, replacement.clone());
        assert_eq!(absorbed, replacement);
    }

    #[test]
    fn config_operation_backwards_always_restores_the_pre_operation_snapshot() {
        let base = Fem3dConfig::default();
        let camera = FemCamera { json: "{\"x\":1}".into() };
        let op = Fem3dConfigMutation::SetCamera { camera: camera.clone() };
        let next = op.diff(&base).diff().clone();
        assert_eq!(next.camera, camera);
        let backwards = op.inverse(&base);
        assert_eq!(backwards, vec![Fem3dConfigMutation::Snapshot { config: base.clone() }]);
        assert_eq!(backwards[0].diff(&next).diff(), &base);
    }

    #[test]
    fn set_result_display_config_operation_round_trips() {
        let base = Fem3dConfig::default();
        let op = Fem3dConfigMutation::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 2 };
        let next = op.diff(&base).diff().clone();
        assert_eq!(next.result_source_id.as_deref(), Some("dead"));
        assert_eq!(next.result_mode, "modal");
        assert_eq!(next.result_mode_index, 2);
    }

    #[test]
    fn fem3d_config_operation_text_round_trips_every_variant() {
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dConfigMutation::Snapshot { config: Fem3dConfig::default() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dConfigMutation::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 1 });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem3dConfigMutation::SetCamera { camera: FemCamera { json: "{\"x\":1}".into() } });
    }
}
// #endregion 🧪️Tests
