//! 🧮️ Equation play app — view state (`EquationConfig`) and its operation enum
//! (`EquationConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.equation` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so camera/locale edits are VCS'd exactly like document
//! content — absorbs the former app-struct `RefCell` (`MathPlayRuntime::camera`, the node-graph viewport)
//! plus the locale the UI used to read off the deleted `ViewModel`.

use crate::EquationCamera;
#[cfg(test)]
use protocol::Mutation;
// 🌱️ Additive `ToValue`/`FromValue` — required by `Mutation<P>`/`MutationDiff<P>`'s trait bound
// (`EquationConfigMutation` implements `Mutation<EquationConfig>` below); see
// `🦀️.rs`'s own docstring note on this crate's interim (not-yet-serde-free) state.
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, dsl::DslArtifact)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[dsl(id = "equation.config", layout = "lines")]
pub struct EquationConfig {
    /// 🎥️ Node-graph viewport camera — session-only, never a document field. Was
    /// `MathPlayRuntime::camera`.
    #[dsl(block)]
    pub camera: EquationCamera,
    /// 🗣️ BCP-47 locale tag — was read off `view_state.locale`.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for EquationConfig {
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
impl store::ArtifactPack for EquationConfig {
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

impl Default for EquationConfig {
    fn default() -> Self {
        Self { camera: EquationCamera::default(), locale: "en-US".into() }
    }
}

store::impl_whole_record_config!(EquationConfig);
//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::{EquationConfigMutation, SetCamera, SetLocale};

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn math_config_default_is_the_identity_camera_and_english_locale() {
        let config = EquationConfig::default();
        assert_eq!(config.camera, EquationCamera::default());
        assert_eq!(config.locale, "en-US");
    }

    #[semio_framework_async_macros::async_test]
    async fn math_config_dsl_round_trips() {
        let config = EquationConfig { camera: EquationCamera { x: 5.0, y: 6.0, zoom: 2.0 }, locale: "de-DE".into() };
        store::os_store::test_support::assert_dsl_round_trip(&config);
        store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[semio_framework_async_macros::async_test]
    async fn config_operation_set_camera_diff_writes_the_targeted_field() {
        let base = EquationConfig::default();
        let camera = EquationCamera { x: 5.0, y: 6.0, zoom: 2.0 };
        let operation = EquationConfigMutation::SetCamera(SetCamera { camera: camera.clone() });
        assert_eq!(Mutation::diff(&operation, &base).diff().camera, camera);
    }

    #[semio_framework_async_macros::async_test]
    async fn config_operation_set_camera_round_trips() {
        let base = EquationConfig::default();
        let camera = EquationCamera { x: 5.0, y: 6.0, zoom: 2.0 };
        let operation = EquationConfigMutation::SetCamera(SetCamera { camera: camera.clone() });
        let next = Mutation::diff(&operation, &base).diff().clone();
        assert_eq!(next.camera, camera);
        let backwards = Mutation::inverse(&operation, &base);
        assert_eq!(backwards, vec![EquationConfigMutation::SetCamera(SetCamera { camera: base.camera.clone() })]);
        assert_eq!(Mutation::diff(&backwards[0], &next).diff().clone(), base);
        store::os_store::test_support::assert_op_line_round_trip(&operation);
    }

    #[semio_framework_async_macros::async_test]
    async fn config_operation_set_locale_round_trips() {
        store::os_store::test_support::assert_op_line_round_trip(&EquationConfigMutation::SetLocale(SetLocale { value: "de-DE".into() }));
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod mutation_vectors {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};

    #[test]
    fn language_neutral_mutations_match_json_oracle_and_restore_base() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔁️mutations.json")).unwrap();
        for vector in vectors.as_array().unwrap() {
            let base: EquationConfig = dsl::json::from_json_str(&vector["base"].to_string()).unwrap();
            let mutation: EquationConfigMutation = dsl::json::from_json_str(&vector["mutation"].to_string()).unwrap();
            let oracle: EquationConfigMutation = serde_json::from_value(vector["mutation"].clone()).unwrap();
            assert_eq!(mutation, oracle);
            assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().unwrap());
            let next = mutation.diff(&base).diff().apply(&base).unwrap();
            assert_eq!(serde_json::to_value(&next).unwrap(), vector["after"]);
            assert_eq!(EquationConfigMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(), mutation);
            assert_eq!(EquationConfigMutation::parse_op(&mutation.print_op()).unwrap(), mutation);
            let mut restored = next;
            for inverse in mutation.inverse(&base) { restored = inverse.diff(&restored).diff().apply(&restored).unwrap(); }
            assert_eq!(restored, base);
        }
    }
}
