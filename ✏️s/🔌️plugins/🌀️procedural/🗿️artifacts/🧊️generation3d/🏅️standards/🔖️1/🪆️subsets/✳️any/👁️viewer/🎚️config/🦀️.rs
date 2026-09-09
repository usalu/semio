//! 🎚️ Generation3d viewer — the read-only surface's OWN persisted view state
//! (`Generation3dViewConfig`) and its authored mutation aggregate (`🧬️schema/🧬️mutations`).
//!
//! A viewer's `handle` returns `ViewEmit`, whose only mutation lane is this config — so show mode,
//! LOD, preview camera and sun are exactly the state a read-only surface may own, and nothing here
//! can ever reach the document. Hover/selection are deliberately ABSENT: they are the framework's
//! own `graph` interaction domain (`create_generation3d_viewer`'s `.interaction(...)`), read through
//! `InteractionView`, never stored by the app.
//!
//! Unlike the sibling surface's own hand-written config aggregate (whose `DESCRIPTORS` name leaf
//! directories that do not exist), every variant below is a REAL authored leaf directory under
//! `🧬️schema/🧬️mutations/`, so `dsl::Mutations`'s leaf-ownership check (`provenance.mutation_root ==
//! scope.mutation_root`, `descriptor.owner` an immediate child of the aggregate's mutation root)
//! holds by construction. MUST NOT import anything from the sibling `✏️editor` module
//! (`policyViewerPurityBreaches`).

use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️ViewCamera
/// 📷️ The read-only preview viewport camera — orbit/pan/zoom write the whole facet at once, the
/// way a viewport camera is never meaningfully set one field at a time.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Generation3dViewCamera {
    #[value(default = "default_view_camera_position")]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[value(default = "default_view_camera_target")]
    #[dsl(coord)]
    pub target: [f64; 3],
    #[value(default = "default_view_camera_fov")]
    pub fov: f64,
}

impl Default for Generation3dViewCamera {
    fn default() -> Self {
        Self { position: default_view_camera_position(), target: default_view_camera_target(), fov: default_view_camera_fov() }
    }
}

pub fn default_view_camera_position() -> [f64; 3] {
    [4.0, -4.0, 3.0]
}

pub fn default_view_camera_target() -> [f64; 3] {
    [0.0, 0.0, 0.0]
}

pub fn default_view_camera_fov() -> f64 {
    45.0
}

pub fn default_view_show_mode() -> String {
    "shaded".into()
}

/// 🌞️ Serialized default [`semio_framework_plugin::WorldSunConfig`] — stored as raw JSON because
/// `WorldSunConfig` is a framework type without a `dsl::DslRecord` impl.
pub fn default_view_sun_json() -> String {
    dsl::json::to_json_string(&semio_framework_plugin::WorldSunConfig::default())
}
//#endregion 🔖️ViewCamera

//#region 🔖️Config
/// 🎚️ `Generation3dViewer`'s real `ArtifactViewer::Config` — persisted local view state that
/// round-trips through the config `ArtifactStore` with a true `inverse` per leaf.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "generation3dviewcfg")]
#[dsl(id = "procedural.generation3dviewcfg")]
#[dsl(layout = "lines")]
pub struct Generation3dViewConfig {
    /// 🎚️ Level-of-detail tessellation deflection (`""`/`coarse`/`fine`).
    pub lod_mode: String,
    /// 👁️ Preview shading mode (`shaded`/`shaded+edges`/`wireframe`/`points`).
    pub show_mode: String,
    /// 📷️ The 3D preview viewport camera.
    #[dsl(block)]
    pub preview_camera: Generation3dViewCamera,
    /// 🌞️ JSON-encoded `semio_framework_plugin::WorldSunConfig`.
    #[value(default = "default_view_sun_json")]
    pub sun_json: String,
}

impl Default for Generation3dViewConfig {
    fn default() -> Self {
        Self { lod_mode: String::new(), show_mode: default_view_show_mode(), preview_camera: Generation3dViewCamera::default(), sun_json: default_view_sun_json() }
    }
}

impl Generation3dViewConfig {
    /// 🌞️ Parses `sun_json` — falls back to `WorldSunConfig::default()` on any malformed value.
    pub fn sun(&self) -> semio_framework_plugin::WorldSunConfig {
        dsl::json::from_json_str(&self.sun_json).unwrap_or_default()
    }

    /// 🧊️ Tessellation deflection for the current LOD — coarser LOD means fewer triangles per
    /// evaluated handle, the viewer's only payload-size lever.
    pub fn tolerance(&self) -> f64 {
        match self.lod_mode.as_str() {
            "coarse" => 0.15,
            "fine" => 0.02,
            _ => 0.05,
        }
    }

    /// 👁️ The effective shading mode, with the empty (never-set) value resolved to `shaded`.
    pub fn effective_show_mode(&self) -> &str {
        if self.show_mode.is_empty() {
            "shaded"
        } else {
            self.show_mode.as_str()
        }
    }
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl: uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for Generation3dViewConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack: envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for Generation3dViewConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
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

store::impl_whole_record_config!(Generation3dViewConfig);
//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
