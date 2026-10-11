//! 🧮️ Generation3d play app — view state (`Generation3dConfig`) and its operation enum
//! (`Generation3dConfigMutation`).
//!
//! This is APP state, not document state: selection, cameras, sun/LOD/show-mode display options, and
//! generation selection lives here rather than under `🗿️artifacts/`, since it does not survive
//! into the `.generation3d` document.

use semio_framework_artifact_flow_flow::CameraJson;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️PreviewCamera
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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

/// 👁️ The preview shading ladder, in the order the show-mode picker offers it and the order
/// `cycleShowMode` walks it. ONE table: the window measure builds its `MeasureSelectItem` rows from
/// it and `apply_show_mode_mesh` answers exactly these tags, so a keyboard cycle can never reach a
/// mode the picker does not offer (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub const GENERATION_3D_SHOW_MODES: [&str; 4] = ["shaded", "shaded+edges", "wireframe", "points"];

/// 🎚️ The level-of-detail ladder, same contract as [`GENERATION_3D_SHOW_MODES`] — coarsest first, so
/// `cycleLodMode` reads as "more detail" until it wraps.
pub const GENERATION_3D_LOD_MODES: [&str; 3] = ["coarse", "medium", "fine"];

/// 🔁️ The ladder step: the entry after `current`, wrapping, and the first entry for anything the
/// ladder does not name — including the empty string a never-set config carries.
fn next_in_ladder(ladder: &[&str], current: &str) -> String {
    let index = ladder.iter().position(|mode| *mode == current).map_or(0, |index| (index + 1) % ladder.len());
    ladder[index].to_string()
}

/// 🔁️ The show mode one `cycleShowMode` after `current`. An unset `show_mode` reads as the `shaded`
/// the preview window's measure displays for it, so the first cycle lands on `shaded+edges`.
pub fn next_show_mode(current: &str) -> String {
    next_in_ladder(&GENERATION_3D_SHOW_MODES, if current.is_empty() { GENERATION_3D_SHOW_MODES[0] } else { current })
}

/// 🔁️ The level of detail one `cycleLodMode` after `current`. An unset `lod_mode` reads as the
/// `medium` the flow window's measure displays for it, so the first cycle lands on `fine`.
pub fn next_lod_mode(current: &str) -> String {
    next_in_ladder(&GENERATION_3D_LOD_MODES, if current.is_empty() { "medium" } else { current })
}

/// 🌞️ Serialized default [`semio_framework_plugin::WorldSunConfig`] — the sun toggle/azimuth/
/// elevation/intensity display options, stored as raw JSON since `WorldSunConfig` is a framework type
/// without a `semio_framework_dsl_record_derive::DslRecord` impl (see [`Generation3dConfig::sun`]).
pub fn default_sun_json() -> String {
    semio_framework_pack_json::to_json_string(&semio_framework_plugin::WorldSunConfig::default())
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "generation3dcfg")]
#[artifact(id = "procedural.generation3dcfg")]
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

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for Generation3dConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
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
        semio_framework_pack_json::from_json_str(&self.sun_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_default()
    }
}

impl store::ConfigRecord for Generation3dConfig {}

//#endregion 🔖️Config

#[path = "🧬️schema/🧬️mutations/🦀️.rs"]
mod mutations;
pub use mutations::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "🚪️io/🦀️.rs"]
pub mod io;

/// 🩹 Owned-field diff of [`Generation3dConfig`]: exactly the fields a leaf sets.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Generation3dConfigPatch {
    pub lod_mode: Option<String>,
    pub show_mode: Option<String>,
    pub camera: Option<CameraJson>,
    pub preview_camera: Option<Generation3dPreviewCamera>,
    pub sun_json: Option<String>,
    pub selected_generation_id: Option<Generation3dSelectedGenerationChange>,
}

/// 🔺️ One change of the nullable `selected_generation_id`: the inner `None` clears it.
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct Generation3dSelectedGenerationChange {
    pub id: Option<String>,
}

impl protocol::MutationDiff<Generation3dConfig> for Generation3dConfigPatch {
    fn apply(&self, base: &Generation3dConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Generation3dConfig> {
        Ok(Generation3dConfig {
            lod_mode: self.lod_mode.clone().unwrap_or_else(|| base.lod_mode.clone()),
            show_mode: self.show_mode.clone().unwrap_or_else(|| base.show_mode.clone()),
            camera: self.camera.clone().unwrap_or_else(|| base.camera.clone()),
            preview_camera: self.preview_camera.clone().unwrap_or_else(|| base.preview_camera.clone()),
            sun_json: self.sun_json.clone().unwrap_or_else(|| base.sun_json.clone()),
            selected_generation_id: self.selected_generation_id.clone().map_or_else(|| base.selected_generation_id.clone(), |change| change.id),
            ..base.clone()
        })
    }
    fn absorb(&mut self, other: Self) {
        self.lod_mode = other.lod_mode.or_else(|| self.lod_mode.take());
        self.show_mode = other.show_mode.or_else(|| self.show_mode.take());
        self.camera = other.camera.or_else(|| self.camera.take());
        self.preview_camera = other.preview_camera.or_else(|| self.preview_camera.take());
        self.sun_json = other.sun_json.or_else(|| self.sun_json.take());
        self.selected_generation_id = other.selected_generation_id.or_else(|| self.selected_generation_id.take());
    }
}

impl protocol::DiffAlgebra<Generation3dConfig> for Generation3dConfigPatch {
    fn inverse(&self, base: &Generation3dConfig) -> Self {
        Self {
            lod_mode: self.lod_mode.as_ref().map(|_| base.lod_mode.clone()),
            show_mode: self.show_mode.as_ref().map(|_| base.show_mode.clone()),
            camera: self.camera.as_ref().map(|_| base.camera.clone()),
            preview_camera: self.preview_camera.as_ref().map(|_| base.preview_camera.clone()),
            sun_json: self.sun_json.as_ref().map(|_| base.sun_json.clone()),
            selected_generation_id: self.selected_generation_id.as_ref().map(|_| Generation3dSelectedGenerationChange { id: base.selected_generation_id.clone() }),
        }
    }
    fn is_empty(&self) -> bool {
        self.lod_mode.is_none() && self.show_mode.is_none() && self.camera.is_none() && self.preview_camera.is_none() && self.sun_json.is_none() && self.selected_generation_id.is_none()
    }
}
