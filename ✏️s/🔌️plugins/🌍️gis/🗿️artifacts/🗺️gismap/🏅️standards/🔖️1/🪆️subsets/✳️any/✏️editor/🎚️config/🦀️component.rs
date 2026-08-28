//! 🧮️ GIS 2D play app — the view-state config artifact and its operation enum.
//!
//! Session-only but real, undoable config: it round-trips through the config `ArtifactStore` exactly
//! like document content, with a true `backwards` per operation. Nothing here is document state — the
//! map's positions/routes/regions live in `crate::artifacts::gismap`.

use protocol::Mutation;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Config
/// 🧮️ gis2d's `ArtifactEditor::Config` — per-layer visibility/stroke-weight, camera, render/vector/LOD
/// mode, plus `locale`. Layer AND feature selection/hover/method/mode moved to the framework-owned
/// `"features"` interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// read via `InteractionView::selection("features")`/`.hover("features", "pointer")`, never stored
/// here again. Per-layer maps are `BTreeMap` (not `HashMap`) because the DSL derive only binds
/// string-keyed maps through `dsl_schema::Shape::Map`'s `BTreeMap<String, V>` case.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "gis2dcfg")]
#[dsl(id = "gis.gis2dcfg")]
#[dsl(layout = "lines")]
pub struct Gis2dConfig {
    /// 👁️ Per-layer visibility; a missing entry defaults to visible.
    #[dsl(block)]
    pub layer_visibility: BTreeMap<String, bool>,
    /// 🎥️ The free/live map camera (`{x,y,zoom}` JSON).
    pub camera_json: String,
    /// 🖼️ `"image" | "vector" | "combined"`.
    pub render_mode: String,
    /// 🎨️ `"colored" | "figureGround" | "invertedFigure"`.
    pub vector_style: String,
    /// 🔽️ Active LOD tier id.
    pub lod_mode: String,
    /// 👁️ Per-layer stroke-weight multiplier; a missing entry defaults to `1.0`.
    #[dsl(block)]
    pub layer_stroke_scale: BTreeMap<String, f64>,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for Gis2dConfig {
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
impl store::ArtifactPack for Gis2dConfig {
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

fn default_gis2d_camera_json() -> String {
    r#"{"x":0,"y":0,"zoom":1}"#.into()
}

fn default_gis2d_render_mode() -> String {
    "combined".into()
}

fn default_gis2d_vector_style() -> String {
    "colored".into()
}

impl Default for Gis2dConfig {
    fn default() -> Self {
        Self {
            layer_visibility: BTreeMap::new(),
            camera_json: default_gis2d_camera_json(),
            render_mode: default_gis2d_render_mode(),
            vector_style: default_gis2d_vector_style(),
            // 🔽️ Mirrors `framework_surface::tiled_map::GIS_MAP_LOD_MODE_AUTOMATIC`, spelled out here so
            // the config type stays independent of the tiled-map surface crate.
            lod_mode: "automatic".into(),
            layer_stroke_scale: BTreeMap::new(),
            locale: "en-US".into(),
        }
    }
}

impl store::ConfigRecord for Gis2dConfig {}

/// 👁️ Whether a map layer is currently shown; a layer with no explicit entry defaults to visible.
pub fn layer_visible(cfg: &Gis2dConfig, layer_id: &str) -> bool {
    cfg.layer_visibility.get(layer_id).copied().unwrap_or(true)
}
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
#[path = "🧬️schema/🔺️diff/🦀️.rs"]
mod configuration_diff;
pub use configuration_diff::{Gis2dConfigDelta, Gis2dConfigDiff};

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;

fn required_nullable<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where D: serde::Deserializer<'de>, T: serde::Deserialize<'de> {
    <Option<T> as serde::Deserialize>::deserialize(deserializer)
}

//#region 🔖️OpCodec
impl protocol::OpText for Gis2dConfigMutation {
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

impl protocol::OpBinary for Gis2dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { dsl::variants_binary::encode_op(self) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> { dsl::variants_binary::decode_op(bytes) }
}

//#endregion 🔖️OpCodec

//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::MutationDiff;

    #[semio_framework_async_macros::async_test]
    async fn gis2d_config_default_matches_the_existing_action_arg_sticky_defaults() {
        let config = Gis2dConfig::default();
        assert_eq!(config.render_mode, "combined");
        assert_eq!(config.vector_style, "colored");
        assert_eq!(config.lod_mode, "automatic");
        assert_eq!(config.locale, "en-US");
    }

    #[semio_framework_async_macros::async_test]
    async fn gis2d_config_default_lod_mode_matches_the_tiled_map_surface_constant() {
        assert_eq!(Gis2dConfig::default().lod_mode, framework_surface::tiled_map::GIS_MAP_LOD_MODE_AUTOMATIC);
    }

    #[semio_framework_async_macros::async_test]
    async fn layer_visible_defaults_to_true_and_honours_explicit_entries() {
        let mut config = Gis2dConfig::default();
        assert!(layer_visible(&config, "water"), "a layer with no entry is visible");
        config.layer_visibility.insert("water".into(), false);
        assert!(!layer_visible(&config, "water"));
    }

    #[semio_framework_async_macros::async_test]
    async fn gis2d_config_dsl_round_trips_default_and_populated() {
        store::os_store::test_support::assert_dsl_round_trip(&Gis2dConfig::default());
        let mut populated = Gis2dConfig::default();
        populated.layer_visibility.insert("water".into(), false);
        populated.layer_stroke_scale.insert("roads".into(), 1.5);
        store::os_store::test_support::assert_dsl_round_trip(&populated);
        store::os_store::test_support::assert_dsl_pack_equivalence(&populated);
    }

    #[semio_framework_async_macros::async_test]
    async fn gis2d_config_operation_diff_writes_the_targeted_field_and_leaves_the_rest() {
        let base = Gis2dConfig::default();
        let next = Gis2dConfigMutation::SetRenderMode(SetRenderMode { value: "vector".into() }).diff(&base).diff().apply(&base).expect("apply");
        assert_eq!(next.render_mode, "vector");
        assert_eq!(next.vector_style, base.vector_style, "untouched fields survive the diff");
    }

    #[semio_framework_async_macros::async_test]
    async fn gis2d_config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = Gis2dConfig::default();
        let operation = Gis2dConfigMutation::SetLayerVisibility(SetLayerVisibility { layer_id: "water".into(), visible: Some(false) });
        let next = operation.diff(&base).diff().apply(&base).expect("apply");
        assert_eq!(next.layer_visibility.get("water"), Some(&false));
        let backwards = operation.inverse(&base);
        assert_eq!(backwards, vec![Gis2dConfigMutation::SetLayerVisibility(SetLayerVisibility { layer_id: "water".into(), visible: None })]);
        let restored = backwards[0].diff(&next).diff().apply(&next).expect("restore");
        assert_eq!(restored, base, "the per-field inverse restores the exact pre-operation config, including the absent map entry");
    }

    /// ⚖️ `SetLayerStrokeScale`'s inverse has the same absent-entry-vs-default subtlety as
    /// `SetLayerVisibility` above, covered separately since it defaults to `1.0` not `true`.
    #[semio_framework_async_macros::async_test]
    async fn gis2d_config_layer_stroke_scale_backwards_restores_an_absent_entry() {
        let base = Gis2dConfig::default();
        let operation = Gis2dConfigMutation::SetLayerStrokeScale(SetLayerStrokeScale { layer_id: "roads".into(), value: Some(2.0) });
        let next = operation.diff(&base).diff().apply(&base).expect("apply");
        assert_eq!(next.layer_stroke_scale.get("roads"), Some(&2.0));
        let backwards = operation.inverse(&base);
        let restored = backwards[0].diff(&next).diff().apply(&next).expect("restore");
        assert_eq!(restored, base);
        assert!(!restored.layer_stroke_scale.contains_key("roads"));
    }

    #[semio_framework_async_macros::async_test]
    async fn gis2d_config_operation_lines_round_trip() {
        store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetLayerVisibility(SetLayerVisibility { layer_id: "water".into(), visible: Some(false) }));
        store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetCamera(SetCamera { camera_json: r#"{"x":1,"y":2,"zoom":3}"#.into() }));
        store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetRenderMode(SetRenderMode { value: "vector".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetVectorStyle(SetVectorStyle { value: "figureGround".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetLodMode(SetLodMode { value: "automatic".into() }));
        store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetLayerStrokeScale(SetLayerStrokeScale { layer_id: "roads".into(), value: Some(1.5) }));
        store::os_store::test_support::assert_op_line_round_trip(&Gis2dConfigMutation::SetLocale(SetLocale { value: "de-DE".into() }));
    }
}

#[cfg(test)]
#[path = "🧪️tests/🧬️mutations/🦀️.rs"]
mod direct_mutation_tests;
//#endregion 🧪️Tests
