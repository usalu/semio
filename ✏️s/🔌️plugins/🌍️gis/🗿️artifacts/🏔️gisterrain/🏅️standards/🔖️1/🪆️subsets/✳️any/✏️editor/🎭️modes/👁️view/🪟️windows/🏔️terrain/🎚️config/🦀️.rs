//! 🧮️ Persisted-local camera configuration owned by one exact GIS Terrain window.
//!
//! Camera changes never enter document history. Each registered `gis3d-main` window stores and
//! reloads its own camera independently.

use semio_framework_value_derive::{FromValue, ToValue};
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Config
/// 🧮️ Free/live viewport camera for one concrete terrain window.
#[derive(Clone, Debug, PartialEq, dsl::DslArtifact, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(extension = "gisterrainwindowcfg")]
#[dsl(id = "gis.gisterrainwindowcfg")]
#[dsl(layout = "lines")]
pub struct GisTerrainWindowConfig {
    /// 🎥️ The free/live world camera (`{position,target,up,fov}` JSON).
    pub camera_json: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for GisTerrainWindowConfig {
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
impl store::ArtifactPack for GisTerrainWindowConfig {
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

/// 🎥️ A default overview camera scaled for a real-world DEM tile patch (hundreds of meters to a
/// few kilometers wide) — the generic `world3d_default_camera()` (position `[4,-4,3]`) assumes an
/// object-scale scene and would sit inside the ground here.
fn default_gis3d_camera_json() -> String {
    serde_json::json!({ "position": [800.0, -800.0, 600.0], "target": [0.0, 0.0, 0.0], "up": [0.0, 0.0, 1.0], "fov": 45.0 }).to_string()
}

impl Default for GisTerrainWindowConfig {
    fn default() -> Self {
        Self { camera_json: default_gis3d_camera_json() }
    }
}

store::impl_whole_record_config!(GisTerrainWindowConfig);

pub struct GisTerrainWindowConfigOwner;

impl semio_framework_plugin::WindowConfigOwner for GisTerrainWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = super::GIS3D_PLAY_WINDOW_MAIN;
    const SCHEMA: &'static str = "gis.gisterrainwindowcfg";
    const MAXIMUM_PUBLICATION_BYTES: usize = 8_192;
    type State = GisTerrainWindowConfig;
    type Mutation = GisTerrainWindowConfigMutation;

    fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> {
        semio_framework_plugin::bounded_window_config_store_owners::<Self>()
    }
    fn build_one_item_preparation_factory() -> std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
        semio_framework_plugin::bounded_window_config_preparation_factory::<Self>()
    }
    fn build_store_disposer() -> Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
        semio_framework_plugin::bounded_window_config_store_disposer::<Self>()
    }
}

pub fn current<C>(view: &semio_framework_plugin::ConfigView<'_, C>) -> GisTerrainWindowConfig {
    view.window::<GisTerrainWindowConfigOwner>().cloned().unwrap_or_default()
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, mutation: GisTerrainWindowConfigMutation) -> Result<semio_framework_plugin::WindowConfigMutation, semio_framework_plugin::Fault> {
    let window_id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("gis-terrain-window-context-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == window_id).map(|window| window.window_kind_id.as_str());
    if kind != Some(super::GIS3D_PLAY_WINDOW_MAIN) {
        return Err(semio_framework_plugin::Fault::from("gis-terrain-window-kind-required"));
    }
    Ok(semio_framework_plugin::WindowConfigMutation::of::<GisTerrainWindowConfigOwner>(window_id, mutation))
}
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
#[path = "🧬️schema/🔺️diff/🦀️.rs"]
mod configuration_diff;
pub use configuration_diff::{GisTerrainWindowConfigDelta, GisTerrainWindowConfigDiff};

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;

//#region 🔖️OpCodec
impl protocol::OpText for GisTerrainWindowConfigMutation {
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

impl protocol::OpBinary for GisTerrainWindowConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

//#endregion 🔖️OpCodec

//#region 🌉️TestBridge
/// 🔮️ One JSON report of applying a `set-camera` mutation to a `GisTerrainWindowConfig`, for a
/// language-neutral test adapter — the identical shape and purpose
/// `crate::gis_terrain_mutation_report_json` already establishes for the
/// terrain's own document-level mutations, applied here to gis3d's editor-config artifact (shard
/// G4, this ticket).
///
/// Every field of `GisTerrainWindowConfig` is a plain `String` — this bridge never
/// needs `serde_json::from_str::<GisTerrainWindowConfig>` at all, and so never needs this struct's own
/// `#[cfg_attr(test, derive(Serialize, Deserialize))]` (unavailable to a `sut`-feature adapter
/// crate, which links this crate as an ordinary dependency, not under `cfg(test)`). Every type in
/// this bridge's own signature is a `str`, matching `gis_terrain_mutation_report_json`'s own doc
/// comment on why that is the whole surface an adapter needs — a generated test host links only
/// `semio-repo-test-host` and, behind its `sut` feature, this crate, so neither `GisTerrainWindowConfigMutation`
/// nor `GisTerrainWindowConfig` can be named there, and hand-transcribing either into a Rust literal would be a
/// second copy of the committed specification vector, free to drift away from it.
///
/// `Mutation<GisTerrainWindowConfig>`/`MutationDiff<GisTerrainWindowConfig>` (via `#[derive(dsl::Mutations)]` on
/// `GisTerrainWindowConfigMutation` and `#[derive(dsl::MutationLeaf)]` on `SetCamera` are the
/// UNCONDITIONAL mutation-engine traits every leaf's own `diff`/`apply`/`inverse` already exercise
/// in this file's own `#[cfg(test)] mod tests` above — never gated by `cfg(test)`, unlike
/// `Serialize`/`Deserialize`/`ToValue`/`FromValue` — so this bridge reaches the exact same real
/// production behavior those unit tests already assert, through a route this crate's own default
/// build always compiles.
pub fn gis_terrain_window_config_mutation_report_json(camera_json: &str, kind: &str, value: &str) -> Result<String, String> {
    use protocol::{Mutation, MutationDiff};
    let base = GisTerrainWindowConfig { camera_json: camera_json.to_string() };
    let mutation: GisTerrainWindowConfigMutation = match kind {
        "set-camera" => GisTerrainWindowConfigMutation::SetCamera(SetCamera { camera_json: value.to_string() }),
        other => return Err(format!("gis_terrain_window_config_mutation_report_json: unknown kind {other:?}")),
    };
    let applied = mutation.diff(&base).diff().apply(&base).map_err(|error| error.to_string())?;
    let inverse_steps = mutation.inverse(&base);
    let mut undone = applied.clone();
    for step in &inverse_steps {
        undone = step.diff(&undone).diff().apply(&undone).map_err(|error| error.to_string())?;
    }
    let report = serde_json::json!({
        "base": {"cameraJson": base.camera_json},
        "snapshot": {"cameraJson": applied.camera_json},
        "inverseSnapshot": {"cameraJson": undone.camera_json},
    });
    Ok(report.to_string())
}
//#endregion 🌉️TestBridge

//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[path = "🧬️schema/🦀️.rs"]
pub mod schema;

#[cfg(test)]
#[path = "🧪️tests/🧬️direct-leaves/🦀️.rs"]
mod direct_leaf_contracts;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
