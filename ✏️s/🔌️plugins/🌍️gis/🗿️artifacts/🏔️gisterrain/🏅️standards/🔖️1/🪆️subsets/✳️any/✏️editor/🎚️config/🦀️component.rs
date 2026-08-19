//! 🧮️ GIS 3D play app — the view-state config artifact and its operation enum.
//!
//! Session-only but real, undoable config: panning and selecting never enter the document's undo
//! history, but they still round-trip through the config `ArtifactStore` with a true `backwards`.
//! The terrain's one editable property (exaggeration) is document state and lives in
//! `crate::artifacts::gisterrain`.

use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ gis3d's `ArtifactEditor::Config` — the free/live viewport camera and world selection, plus
/// `locale`. Mirrors `crate::editor::gis2d::config::Gis2dConfig`'s identical shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "gis3dcfg")]
#[dsl(id = "gis.gis3dcfg")]
#[dsl(layout = "lines")]
pub struct Gis3dConfig {
    /// 🎥️ The free/live world camera (`{position,target,up,fov}` JSON).
    pub camera_json: String,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for Gis3dConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    async fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    async fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
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
    async fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for Gis3dConfig {
    async fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    async fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    async fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec


/// 🎥️ A default overview camera scaled for a real-world DEM tile patch (hundreds of meters to a
/// few kilometers wide) — the generic `world3d_default_camera()` (position `[4,-4,3]`) assumes an
/// object-scale scene and would sit inside the ground here.
async fn default_gis3d_camera_json() -> String {
    serde_json::json!({ "position": [800.0, -800.0, 600.0], "target": [0.0, 0.0, 0.0], "up": [0.0, 0.0, 1.0], "fov": 45.0 }).to_string()
}

impl Default for Gis3dConfig {
    async fn default() -> Self {
        Self { camera_json: default_gis3d_camera_json(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(Gis3dConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ `Gis3dConfig`'s operation enum — one variant per settled interaction; each variant's
/// `backwards()` re-emits the SAME variant with the old field value read from `base` (no
/// whole-config snapshot sentinel) — mirrors `crate::editor::gis2d::config::Gis2dConfigMutation`'s
/// identical shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Gis3dConfigMutation {
    #[dsl(key = "camera")]
    SetCamera { camera_json: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for Gis3dConfigMutation {
    async fn parse_op(line: &str) -> Result<Self, store::TextError> {
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
    async fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for Gis3dConfigMutation {
    async fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
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
    async fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
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


impl Mutation<Gis3dConfig> for Gis3dConfigMutation {
    type Diff = Gis3dConfig;

    async fn diff(&self, base: &Gis3dConfig) -> protocol::MutationOutcome<Gis3dConfig> {
        let mut next = base.clone();
        match self {
            Gis3dConfigMutation::SetCamera { camera_json } => {
                if &base.camera_json == camera_json {
                    return protocol::MutationOutcome::empty().warn("mutation.no-op", "Camera is already at the requested position.");
                }
                next.camera_json = camera_json.clone();
            }
            Gis3dConfigMutation::SetLocale { value } => {
                if &base.locale == value {
                    return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Locale is already \"{}\".", value));
                }
                next.locale = value.clone();
            }
        }
        protocol::MutationOutcome::new(next)
    }

    async fn inverse(&self, base: &Gis3dConfig) -> Vec<Self> {
        match self {
            Gis3dConfigMutation::SetCamera { .. } => vec![Gis3dConfigMutation::SetCamera { camera_json: base.camera_json.clone() }],
            Gis3dConfigMutation::SetLocale { .. } => vec![Gis3dConfigMutation::SetLocale { value: base.locale.clone() }],
        }
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn gis3d_config_default_matches_the_pre_migration_view_defaults() {
        let config = Gis3dConfig::default();
        assert!(config.camera_json.contains("800"));
        assert_eq!(config.locale, "en-US");
    }

    #[test]
    async fn gis3d_config_dsl_round_trips_default_and_populated() {
        store::os_store::test_support::assert_dsl_round_trip(&Gis3dConfig::default());
        let populated = Gis3dConfig { locale: "de-DE".into(), ..Gis3dConfig::default() };
        store::os_store::test_support::assert_dsl_round_trip(&populated);
        store::os_store::test_support::assert_dsl_pack_equivalence(&populated);
    }

    #[test]
    async fn gis3d_config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = Gis3dConfig::default();
        let operation = Gis3dConfigMutation::SetCamera { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() };
        let next = operation.diff(&base).diff().clone();
        assert_eq!(next.camera_json, r#"{"position":[1.0,2.0,3.0]}"#);
        let backwards = operation.inverse(&base);
        assert_eq!(backwards, vec![Gis3dConfigMutation::SetCamera { camera_json: base.camera_json.clone() }]);
        assert_eq!(backwards[0].diff(&next).diff().clone(), base);
    }

    #[test]
    async fn gis3d_config_operation_lines_round_trip() {
        store::os_store::test_support::assert_op_line_round_trip(&Gis3dConfigMutation::SetCamera { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() });
        store::os_store::test_support::assert_op_line_round_trip(&Gis3dConfigMutation::SetLocale { value: "de-DE".into() });
    }
}
//#endregion 🧪️Tests
