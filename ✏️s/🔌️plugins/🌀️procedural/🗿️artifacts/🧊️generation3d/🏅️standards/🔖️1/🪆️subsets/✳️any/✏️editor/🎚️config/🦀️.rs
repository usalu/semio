//! 🧮️ Generation3d play app — view state (`Generation3dConfig`) and its operation enum
//! (`Generation3dConfigMutation`).
//!
//! This is APP state, not document state: selection, cameras, sun/LOD/show-mode display options, and
//! generation selection lives here rather than under `🗿️artifacts/`, since it does not survive
//! into the `.generation3d` document.

use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️PreviewCamera
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Generation3dPreviewCamera {
    #[value(default = "default_preview_cam_pos")]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[value(default = "default_preview_cam_target")]
    #[dsl(coord)]
    pub target: [f64; 3],
    #[value(default = "default_preview_fov")]
    pub fov: f64,
}

impl Default for Generation3dPreviewCamera {
    fn default() -> Self {
        Self { position: default_preview_cam_pos(), target: default_preview_cam_target(), fov: default_preview_fov() }
    }
}

pub fn default_preview_cam_pos() -> [f64; 3] {
    [4.0, -4.0, 3.0]
}

pub fn default_preview_cam_target() -> [f64; 3] {
    [0.0, 0.0, 0.0]
}

pub fn default_preview_fov() -> f64 {
    45.0
}

pub fn default_show_mode() -> String {
    "shaded".into()
}

/// 🌞️ Serialized default [`semio_framework_plugin::WorldSunConfig`] — the sun toggle/azimuth/
/// elevation/intensity display options, stored as raw JSON since `WorldSunConfig` is a framework type
/// without a `dsl::DslRecord` impl (see [`Generation3dConfig::sun`]).
pub fn default_sun_json() -> String {
    dsl::json::to_json_string(&semio_framework_plugin::WorldSunConfig::default())
}

//#endregion 🔖️PreviewCamera

//#region 🔖️Config
/// 🧮️ `Generation3dPlayApp`'s real `ArtifactApp::Config` — the pure-trait config artifact. Absorbs
/// LOD/show display options, flow-graph + preview cameras, sun display options, active generation
/// selection — app-local view state
/// round-trips through the config `ArtifactStore` exactly like document content, with a real
/// `backwards` per [`Generation3dConfigMutation`]. Selection/hover moved to the framework's own
/// `graph` interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// see `create_generation3d_app`'s `.interaction(...)` declaration.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "generation3dcfg")]
#[dsl(id = "procedural.generation3dcfg")]
#[dsl(layout = "lines")]
pub struct Generation3dConfig {
    /// 🎚️ Level-of-detail tessellation deflection.
    pub lod_mode: String,
    /// 👁️ Preview shading mode.
    pub show_mode: String,
    /// 📷️ The flow-graph node canvas camera.
    #[dsl(block)]
    pub camera: CameraJson,
    /// 📷️ The 3D preview viewport camera.
    #[dsl(block)]
    pub preview_camera: Generation3dPreviewCamera,
    /// 🌞️ JSON-encoded `semio_framework_plugin::WorldSunConfig`.
    #[value(default = "default_sun_json")]
    pub sun_json: String,
    /// 🧬️ The selected generation id.
    pub selected_generation_id: Option<String>,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for Generation3dConfig {
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
impl store::ArtifactPack for Generation3dConfig {
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

impl Default for Generation3dConfig {
    fn default() -> Self {
        Self {
            lod_mode: String::new(),
            show_mode: default_show_mode(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            preview_camera: Generation3dPreviewCamera::default(),
            sun_json: default_sun_json(),
            selected_generation_id: None,
        }
    }
}

impl Generation3dConfig {
    /// 🌞️ Parses `sun_json` — falls back to `WorldSunConfig::default()` on any malformed/legacy value.
    pub fn sun(&self) -> semio_framework_plugin::WorldSunConfig {
        dsl::json::from_json_str(&self.sun_json).unwrap_or_default()
    }
}

store::impl_whole_record_config!(Generation3dConfig);
//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
