//! 🎚️ Persisted local configuration for one exact GIS Map window.
//!
//! Session-only but real, undoable config: it round-trips through the config `ArtifactStore` exactly
//! like document content, with a true `backwards` per operation. Nothing here is document state — the
//! map's positions/routes/regions live in `crate`.

use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️Config
/// 🧮️ One Map window's per-layer visibility/stroke-weight, camera, and render/vector/LOD
/// mode. Layer AND feature selection/hover/method/mode moved to the framework-owned
/// `"features"` interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// read via `InteractionView::selection("features")`/`.hover("features", "pointer")`, never stored
/// here again. Per-layer maps are `BTreeMap` (not `HashMap`) because the DSL derive only binds
/// string-keyed maps through `dsl_schema::Shape::Map`'s `BTreeMap<String, V>` case.
#[derive(Clone, Debug, PartialEq, dsl::DslArtifact, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "mapwindowcfg")]
#[dsl(id = "gis.mapwindowcfg")]
#[dsl(layout = "lines")]
pub struct MapWindowConfig {
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
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for MapWindowConfig {
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
impl store::ArtifactPack for MapWindowConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
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

impl Default for MapWindowConfig {
    fn default() -> Self {
        Self {
            layer_visibility: BTreeMap::new(),
            camera_json: default_gis2d_camera_json(),
            render_mode: default_gis2d_render_mode(),
            vector_style: default_gis2d_vector_style(),
            // 🔽️ Mirrors `semio_framework_surface::tiled_map::GIS_MAP_LOD_MODE_AUTOMATIC`, spelled out here so
            // the config type stays independent of the tiled-map surface crate.
            lod_mode: "automatic".into(),
            layer_stroke_scale: BTreeMap::new(),
        }
    }
}

impl store::ConfigRecord for MapWindowConfig {}

pub struct MapWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for MapWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = crate::editor::gis2d::modes::edit::windows::map::GIS2D_PLAY_WINDOW_MAIN;
    const SCHEMA: &'static str = "gis.mapwindowcfg";
    const MAXIMUM_PUBLICATION_BYTES: usize = 65_536;
    type State = MapWindowConfig;
    type Mutation = MapWindowConfigMutation;
    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> { semio_framework_plugin::bounded_window_config_store_owners::<Self>() }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> { semio_framework_plugin::bounded_window_config_preparation_factory::<Self>() }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> { semio_framework_plugin::bounded_window_config_store_disposer::<Self>() }
}

pub fn register(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<MapWindowConfigOwner>()
}

pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowConfigSnapshot>) -> MapWindowConfig {
    snapshot
        .filter(|snapshot| snapshot.window_kind_id() == crate::editor::gis2d::modes::edit::windows::map::GIS2D_PLAY_WINDOW_MAIN)
        .and_then(|snapshot| snapshot.get::<MapWindowConfigOwner>())
        .cloned()
        .unwrap_or_default()
}

pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> MapWindowConfig {
    from_snapshot(view.window)
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, mutation: MapWindowConfigMutation) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("gis-map-window-required"))?;
    let kind = view
        .window_instances
        .iter()
        .find(|window| window.id == id)
        .map(|window| window.window_kind_id.as_str())
        .ok_or_else(|| semio_framework_plugin::Fault::from("gis-map-window-stale"))?;
    if kind != crate::editor::gis2d::modes::edit::windows::map::GIS2D_PLAY_WINDOW_MAIN {
        return Err(semio_framework_plugin::Fault::from("gis-map-window-kind-required"));
    }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<MapWindowConfigOwner>(id, mutation))
}

/// ⭕️ Decodes a present nullable field while omission remains an error.
#[cfg(test)]
pub fn required_nullable<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    <Option<T> as Deserialize>::deserialize(deserializer)
}

/// 👁️ Whether a map layer is currently shown; a layer with no explicit entry defaults to visible.
pub fn layer_visible(cfg: &MapWindowConfig, layer_id: &str) -> bool {
    cfg.layer_visibility.get(layer_id).copied().unwrap_or(true)
}
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
#[path = "🧬️schema/🔺️diff/🦀️.rs"]
mod configuration_diff;
pub use configuration_diff::{MapWindowConfigDelta, MapWindowConfigDiff};

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;

//#region 🔖️OpCodec
impl protocol::OpText for MapWindowConfigMutation {
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

impl protocol::OpBinary for MapWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

//#endregion 🔖️OpCodec

//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧬️direct-leaves/🦀️.rs"]
mod direct_mutation_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️window-ownership/🦀️.rs"]
mod window_ownership_tests;
//#endregion 🧪️Tests
