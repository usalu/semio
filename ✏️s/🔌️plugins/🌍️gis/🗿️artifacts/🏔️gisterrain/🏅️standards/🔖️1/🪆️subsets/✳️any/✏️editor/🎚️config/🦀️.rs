//! 🧮️ GIS 3D play app — the view-state config artifact and its operation enum.
//!
//! Session-only but real, undoable config: panning and selecting never enter the document's undo
//! history, but they still round-trip through the config `ArtifactStore` with a true `backwards`.
//! The terrain's one editable property (exaggeration) is document state and lives in
//! `crate::artifacts::gisterrain`.

use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ gis3d's `ArtifactEditor::Config` — the free/live viewport camera and world selection, plus
/// `locale`. Mirrors `crate::editor::gis2d::config::Gis2dConfig`'s identical shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
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
impl store::ArtifactPack for Gis3dConfig {
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

/// 🎥️ A default overview camera scaled for a real-world DEM tile patch (hundreds of meters to a
/// few kilometers wide) — the generic `world3d_default_camera()` (position `[4,-4,3]`) assumes an
/// object-scale scene and would sit inside the ground here.
fn default_gis3d_camera_json() -> String {
    serde_json::json!({ "position": [800.0, -800.0, 600.0], "target": [0.0, 0.0, 0.0], "up": [0.0, 0.0, 1.0], "fov": 45.0 }).to_string()
}

impl Default for Gis3dConfig {
    fn default() -> Self {
        Self { camera_json: default_gis3d_camera_json(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(Gis3dConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
#[path = "🧬️schema/🔺️diff/🦀️.rs"]
mod configuration_diff;
pub use configuration_diff::{Gis3dConfigDelta, Gis3dConfigDiff};

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;

//#region 🔖️OpCodec
impl protocol::OpText for Gis3dConfigMutation {
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

impl protocol::OpBinary for Gis3dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { dsl::variants_binary::encode_op(self) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { dsl::variants_binary::decode_op(bytes) }
}

//#endregion 🔖️OpCodec

//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🧬️direct-leaves/🦀️.rs"]
mod direct_leaf_contracts;

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mutation, MutationDiff};

    #[test]
    fn gis3d_config_serde_is_strict_and_requires_both_fields() {
        assert!(serde_json::from_str::<Gis3dConfig>(r#"{"cameraJson":"{}"}"#).is_err());
        assert!(serde_json::from_str::<Gis3dConfig>(r#"{"locale":"en-US"}"#).is_err());
        assert!(serde_json::from_str::<Gis3dConfig>(r#"{"cameraJson":null,"locale":"en-US"}"#).is_err());
        assert!(serde_json::from_str::<Gis3dConfig>(r#"{"cameraJson":"{}","locale":"en-US","extra":true}"#).is_err());
        assert!(serde_json::from_str::<Gis3dConfig>(r#"{"cameraJson":"{}","locale":"en-US"}"#).is_ok());
    }

    #[semio_framework_async_macros::async_test]
    async fn gis3d_config_default_matches_the_pre_migration_view_defaults() {
        let config = Gis3dConfig::default();
        assert!(config.camera_json.contains("800"));
        assert_eq!(config.locale, "en-US");
    }

    #[semio_framework_async_macros::async_test]
    async fn gis3d_config_dsl_round_trips_default_and_populated() {
        store::os_store::test_support::assert_dsl_round_trip(&Gis3dConfig::default());
        let populated = Gis3dConfig { locale: "de-DE".into(), ..Gis3dConfig::default() };
        store::os_store::test_support::assert_dsl_round_trip(&populated);
        store::os_store::test_support::assert_dsl_pack_equivalence(&populated);
    }

    #[semio_framework_async_macros::async_test]
    async fn gis3d_config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = Gis3dConfig::default();
        let operation = Gis3dConfigMutation::SetCamera(SetCamera { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() });
        let next = operation.diff(&base).diff().apply(&base).expect("apply");
        assert_eq!(next.camera_json, r#"{"position":[1.0,2.0,3.0]}"#);
        let backwards = operation.inverse(&base);
        assert_eq!(backwards, vec![Gis3dConfigMutation::SetCamera(SetCamera { camera_json: base.camera_json.clone() })]);
        assert_eq!(backwards[0].diff(&next).diff().apply(&next).expect("restore"), base);
    }

    #[semio_framework_async_macros::async_test]
    async fn gis3d_config_operation_lines_round_trip() {
        store::os_store::test_support::assert_op_line_round_trip(&Gis3dConfigMutation::SetCamera(SetCamera { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() }));
        store::os_store::test_support::assert_op_line_round_trip(&Gis3dConfigMutation::SetLocale(SetLocale { value: "de-DE".into() }));
    }
}
//#endregion 🧪️Tests
