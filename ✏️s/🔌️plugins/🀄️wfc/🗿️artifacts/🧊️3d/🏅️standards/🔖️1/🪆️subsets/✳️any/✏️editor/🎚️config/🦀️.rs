//! 🧮️ WFC 3D editor — PER-WINDOW-INSTANCE view state (`Wfc3dConfig`) and its operation enum.
//!
//! This is APP state, not document state: nothing in it survives into the `.wfc3d` document. One
//! instance exists per pane, so the `wfc-graph` window and the `wfc-3d-preview` window each keep
//! their own camera, and the armed tile the graph window pins with is a per-pane choice too. It
//! round-trips through its own `ArtifactStore` exactly like document content, with a real inverse per
//! `Wfc3dConfigMutation` rather than never being VCS'd at all.

use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Config
/// 🧮️ `Wfc3dEditor::Config` — the camera this pane looks through, plus the tile `pin-slot` arms.
/// `camera_*` are the 2d graph pane's pan/zoom; `orbit_*` are the 3d preview pane's orbit pose, kept
/// as plain scalars so one config type serves both panes without a variant per surface kind.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "wfc3dcfg")]
#[dsl(id = "wfc.wfc3d.config")]
#[dsl(layout = "lines")]
pub struct Wfc3dConfig {
    /// 🎥️ Viewport camera x for THIS pane.
    pub camera_x: f64,
    /// 🎥️ Viewport camera y for THIS pane.
    pub camera_y: f64,
    /// 🎥️ Viewport zoom for THIS pane.
    pub camera_zoom: f64,
    /// 🀄️ The tile a pin gesture in THIS pane assigns; empty means "the document's first tile".
    pub active_tile_id: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted `ArtifactDsl`: uses this type's `__dsl_*` helpers, not derive emission.
impl store::ArtifactDsl for Wfc3dConfig {
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

/// 📦️ Handcrafted `ArtifactPack`: envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for Wfc3dConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
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

impl Default for Wfc3dConfig {
    fn default() -> Self {
        Self { camera_x: 0.0, camera_y: 0.0, camera_zoom: 1.0, active_tile_id: String::new() }
    }
}

store::impl_whole_record_config!(Wfc3dConfig);

/// 🀄️ The tile a pin gesture in this pane assigns: the armed id when it names a real tile, else the
/// document's first tile — never an empty id that `pin-slot` would refuse.
pub fn wfc3d_active_tile_id(config: &Wfc3dConfig, document: &crate::Wfc3dSnapshot) -> Option<String> {
    if !config.active_tile_id.is_empty() && document.tiles.iter().any(|tile| tile.id == config.active_tile_id) {
        return Some(config.active_tile_id.clone());
    }
    document.tiles.first().map(|tile| tile.id.clone())
}
//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "🧬️schema/🦀️.rs"]
pub mod schema;
