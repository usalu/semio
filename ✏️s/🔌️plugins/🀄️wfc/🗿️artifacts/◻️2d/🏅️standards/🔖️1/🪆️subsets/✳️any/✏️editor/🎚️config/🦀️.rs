//! 🧮️ WFC 2D editor — PER-WINDOW-INSTANCE view state (`Wfc2dConfig`) and its operation enum.
//!
//! This is APP state, not document state: nothing in it survives into the `.wfc2d` document. One
//! instance exists per pane, so the `wfc-graph` window and the `wfc-2d-preview` window each keep
//! their own camera, and the armed tile the graph window pins with is a per-pane choice too. It
//! round-trips through its own `ArtifactStore` exactly like document content, with a real inverse
//! per `Wfc2dConfigMutation` rather than never being VCS'd at all.

//#region 🔖️Config
/// 🧮️ `Wfc2dEditor::Config` — the camera this pane looks through, plus the tile `pin-slot` arms.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact(extension = "wfc2dcfg")]
#[artifact(id = "wfc.wfc2d.config")]
#[dsl(layout = "lines")]
pub struct Wfc2dConfig {
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
impl store::ArtifactDsl for Wfc2dConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted `ArtifactPack`: envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for Wfc2dConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️ArtifactCodec

impl Default for Wfc2dConfig {
    fn default() -> Self {
        Self { camera_x: 0.0, camera_y: 0.0, camera_zoom: 1.0, active_tile_id: String::new() }
    }
}

impl store::ConfigRecord for Wfc2dConfig {}

/// 🔺️ Field-sparse diff of [`Wfc2dConfig`]: each field is an optional absolute value.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", default)]
pub struct Wfc2dConfigDiff {
    pub camera_x: Option<f64>,
    pub camera_y: Option<f64>,
    pub camera_zoom: Option<f64>,
    pub active_tile_id: Option<String>,
}

impl protocol::DiffAlgebra<Wfc2dConfig> for Wfc2dConfigDiff {
    fn inverse(&self, base: &Wfc2dConfig) -> Self {
        Self {
            camera_x: self.camera_x.as_ref().map(|_| base.camera_x.clone()),
            camera_y: self.camera_y.as_ref().map(|_| base.camera_y.clone()),
            camera_zoom: self.camera_zoom.as_ref().map(|_| base.camera_zoom.clone()),
            active_tile_id: self.active_tile_id.as_ref().map(|_| base.active_tile_id.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.camera_x.is_none() && self.camera_y.is_none() && self.camera_zoom.is_none() && self.active_tile_id.is_none()
    }
}

impl protocol::MutationDiff<Wfc2dConfig> for Wfc2dConfigDiff {
    fn apply(&self, base: &Wfc2dConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Wfc2dConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.camera_x {
            next.camera_x.clone_from(value);
        }
        if let Some(value) = &self.camera_y {
            next.camera_y.clone_from(value);
        }
        if let Some(value) = &self.camera_zoom {
            next.camera_zoom.clone_from(value);
        }
        if let Some(value) = &self.active_tile_id {
            next.active_tile_id.clone_from(value);
        }
        Ok(next)
    }
    fn absorb(&mut self, later: Self) {
        if later.camera_x.is_some() {
            self.camera_x = later.camera_x;
        }
        if later.camera_y.is_some() {
            self.camera_y = later.camera_y;
        }
        if later.camera_zoom.is_some() {
            self.camera_zoom = later.camera_zoom;
        }
        if later.active_tile_id.is_some() {
            self.active_tile_id = later.active_tile_id;
        }
    }
}

/// 🀄️ The tile a pin gesture in this pane assigns: the armed id when it names a real tile, else the
/// document's first tile — never an empty id that `pin-slot` would refuse.
pub fn wfc2d_active_tile_id(config: &Wfc2dConfig, document: &crate::Wfc2dSnapshot) -> Option<String> {
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

#[path = "🚪️io/🦀️.rs"]
pub mod io;
