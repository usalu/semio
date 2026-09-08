//! 🧮️ Shooting play app — view state (`ShootingConfig`) and its operation enum
//! (`ShootingConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/`
//! because nothing in it survives into the `.shooting` document. It still round-trips through a real
//! `ArtifactStore` (with a real `backwards`), so selection/camera/utility edits are VCS'd exactly like
//! document content.

use crate::ShootingCamera;
#[cfg(test)]
use protocol::Mutation;

//#region 🔖️Config
/// 🧮️ B1: shooting's real `ArtifactApp::Config` — the pure-trait pilot's config artifact. Absorbs
/// both the old sticky `ActionArgDef` defaults (`default_shot_format`/`shape`/`default_asset_format`)
/// AND everything that used to live in an app-struct `RefCell` runtime (selection, hover, selection
/// method, center-model toggle, fit-revision counter, camera draft label, and the free/live viewport
/// camera) — session-only view state now round-trips through the config `ArtifactStore` exactly like
/// document content, with a real `backwards` per [`ShootingConfigMutation`] instead of never being
/// VCS'd at all. `locale`/`active_utility_id` are the two view-state fields the shooting UI actually
/// reads (`resolve_labels`/the transform-gumball utility) — see `crate::editor::shooting::render`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "shooting.config")]
#[dsl(id = "shooting.config")]
#[dsl(layout = "lines")]
pub struct ShootingConfig {
    /// 🖼️ Mirrors `addShot`'s `format` `ActionArgDef` default (`"png"`).
    pub default_shot_format: String,
    /// 🖼️ Mirrors `addShot`'s `shape` `ActionArgDef` default (`"rectangle"`).
    pub default_shot_shape: String,
    /// 🧱️ Mirrors `addAsset`'s `format` `ActionArgDef` default (`"glb"`).
    pub default_asset_format: String,
    /// 👁️ Selected shot ids (gallery/document-tree multi-select) — genuinely app-specific, NOT part of
    /// the framework-owned `"assets"` interaction domain (ticket
    /// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM): asset selection/hover dissolved into that
    /// domain, but shots have no world-3d pick/marquee/hover surface of their own, so shot selection
    /// stays a plain config field, set via [`ShootingConfigMutation::SetShotSelection`].
    pub selected_shot_ids: Vec<String>,
    /// 👁️ "Center model in viewport" toggle.
    pub center_model: bool,
    /// 👁️ Bumped whenever the active asset changes to re-trigger a viewport fit.
    pub fit_revision: u32,
    /// 👁️ In-progress "save camera" label draft.
    pub camera_draft_label: String,
    /// 🎥️ The free/live viewport camera — session-only, never a document field.
    #[dsl(block)]
    pub camera: ShootingCamera,
    /// 🧰️ The active transform-gumball utility for the scene window.
    pub active_utility_id: String,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for ShootingConfig {
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
impl store::ArtifactPack for ShootingConfig {
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

impl Default for ShootingConfig {
    fn default() -> Self {
        Self {
            default_shot_format: "png".into(),
            default_shot_shape: "rectangle".into(),
            default_asset_format: "glb".into(),
            selected_shot_ids: Vec::new(),
            center_model: true,
            fit_revision: 0,
            camera_draft_label: String::new(),
            camera: ShootingCamera::default(),
            active_utility_id: "move".into(),
            locale: "en-US".into(),
        }
    }
}

store::impl_whole_record_config!(ShootingConfig);
//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn shooting_config_default_matches_the_existing_action_arg_sticky_defaults() {
        let config = ShootingConfig::default();
        assert_eq!(config.default_shot_format, "png");
        assert_eq!(config.default_shot_shape, "rectangle");
        assert_eq!(config.default_asset_format, "glb");
    }

    /// 🎞️ A fixture exercising every field — the dsl/pack round-trip law for `ShootingConfig`.
    #[semio_framework_async_macros::async_test]
    async fn shooting_config_dsl_pack_round_trip() {
        let config = ShootingConfig {
            selected_shot_ids: vec!["s1".into()],
            center_model: false,
            fit_revision: 3,
            camera_draft_label: "Hero".into(),
            camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..ShootingCamera::default() },
            active_utility_id: "rotate".into(),
            locale: "de-DE".into(),
            ..ShootingConfig::default()
        };
        store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[semio_framework_async_macros::async_test]
    async fn shooting_config_operation_text_binary_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::ReplaceConfig(ReplaceConfig { config: ShootingConfig { selected_shot_ids: vec!["s1".into()], locale: "de-DE".into(), ..ShootingConfig::default() } }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetShotSelection(SetShotSelection { shot_ids: vec!["s1".into(), "s2".into()] }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetCenterModel(SetCenterModel { value: true }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetFitRevision(SetFitRevision { value: 4 }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetCameraDraftLabel(SetCameraDraftLabel { value: "Hero".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetCamera(SetCamera { camera: ShootingCamera { position: [1.0, 2.0, 3.0], ..ShootingCamera::default() } }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetActiveUtility(SetActiveUtility { utility_id: "rotate".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetLocale(SetLocale { value: "de-DE".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&ShootingConfigMutation::SetDefaults(SetDefaults { shot_format: "svg".into(), shot_shape: "ellipse".into(), asset_format: "glb".into() }));
    }

    #[semio_framework_async_macros::async_test]
    async fn shooting_config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = ShootingConfig { selected_shot_ids: vec!["s1".into()], locale: "en-US".into(), ..ShootingConfig::default() };
        let operation = ShootingConfigMutation::SetShotSelection(SetShotSelection { shot_ids: vec!["s2".into()] });
        let forward = operation.diff(&base).into_parts().0;
        assert_eq!(forward.selected_shot_ids, vec!["s2".to_string()]);
        let backwards = operation.inverse(&base);
        assert_eq!(backwards, vec![ShootingConfigMutation::ReplaceConfig(ReplaceConfig { config: base.clone() })]);
        let restored = backwards[0].diff(&forward).into_parts().0;
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests

#[cfg(test)]
mod contract_vectors {
    use super::*;
    use protocol::{Mutation, MutationDiff, OpBinary, OpText};
    use dsl::os_pack as pack;

    #[test]
    fn shooting_configuration_contract_vectors_match_the_json_oracle() {
        let vectors: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔁️mutation-contracts.json")).expect("neutral contract vectors");
        let base: ShootingConfig = pack::from_json_str(&vectors["base"].to_string()).expect("owned base decoder");
        assert_eq!(<ShootingConfigMutation as Mutation<ShootingConfig>>::DESCRIPTORS.len(), vectors["cases"].as_array().expect("cases").len());
        for vector in vectors["cases"].as_array().expect("cases") {
            let mutation: ShootingConfigMutation = pack::from_json_str(&vector["mutation"].to_string()).expect("owned operation decoder");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&mutation)).expect("independent operation oracle"), vector["mutation"]);
            assert_eq!(mutation.descriptor().semantic_kind, vector["kind"].as_str().expect("semantic kind"));
            assert_eq!(ShootingConfigMutation::parse_op(&mutation.print_op()).expect("operation text"), mutation);
            assert_eq!(ShootingConfigMutation::decode_op(&mutation.encode_op().expect("operation binary")).expect("binary decode"), mutation);
            let outcome = mutation.diff(&base);
            assert!(outcome.messages().is_empty());
            let next = outcome.diff().apply(&base).expect("apply diff");
            assert_eq!(serde_json::from_str::<serde_json::Value>(&pack::to_json_string(&next)).expect("independent state oracle"), vector["expected"]);
            let restored = mutation.inverse(&base).into_iter().fold(next, |state, inverse| inverse.diff(&state).diff().apply(&state).expect("apply inverse"));
            assert_eq!(restored, base);
        }
    }
}
