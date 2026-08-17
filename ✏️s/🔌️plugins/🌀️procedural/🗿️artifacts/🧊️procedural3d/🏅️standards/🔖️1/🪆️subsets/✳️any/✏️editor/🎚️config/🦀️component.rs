//! 🧮️ Procedural3d play app — view state (`Procedural3dConfig`) and its operation enum
//! (`Procedural3dConfigMutation`).
//!
//! This is APP state, not document state: selection, cameras, sun/LOD/show-mode display options, and
//! the derived generation preview live here rather than under `🗿️artifacts/`, since none of it survives
//! into the `.procedural3d` document.

use flow::CameraJson;
use protocol::Mutation;
use serde::{Deserialize, Serialize};

//#region 🔖️PreviewCamera
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct Procedural3dPreviewCamera {
    #[serde(default = "default_preview_cam_pos")]
    #[dsl(coord)]
    pub position: [f64; 3],
    #[serde(default = "default_preview_cam_target")]
    #[dsl(coord)]
    pub target: [f64; 3],
    #[serde(default = "default_preview_fov")]
    pub fov: f64}

impl Default for Procedural3dPreviewCamera {
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
/// without a `dsl::DslRecord` impl (see [`Procedural3dConfig::sun`]).
pub fn default_sun_json() -> String {
    serde_json::to_string(&semio_framework_plugin::WorldSunConfig::default()).unwrap_or_default()
}

//#endregion 🔖️PreviewCamera

//#region 🔖️Config
/// 🧮️ `Procedural3dPlayApp`'s real `ArtifactApp::Config` — the pure-trait config artifact. Absorbs
/// LOD/show display options, flow-graph + preview cameras, sun display options, active generation
/// selection/preview, the active transform-gumball utility, and locale — session-only view state
/// round-trips through the config `ArtifactStore` exactly like document content, with a real
/// `backwards` per [`Procedural3dConfigMutation`]. Selection/hover moved to the framework's own
/// `graph` interaction domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) —
/// see `create_procedural3d_app`'s `.interaction(...)` declaration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "procedural3dcfg")]
#[dsl(layout = "lines")]
pub struct Procedural3dConfig {
    /// 🎚️ Level-of-detail tessellation deflection.
    pub lod_mode: String,
    /// 👁️ Preview shading mode.
    pub show_mode: String,
    /// 📷️ The flow-graph node canvas camera.
    #[dsl(block)]
    pub camera: CameraJson,
    /// 📷️ The 3D preview viewport camera.
    #[dsl(block)]
    pub preview_camera: Procedural3dPreviewCamera,
    /// 🌞️ JSON-encoded `semio_framework_plugin::WorldSunConfig`.
    #[serde(default = "default_sun_json")]
    pub sun_json: String,
    /// 🧬️ The selected generation id.
    pub selected_generation_id: Option<String>,
    /// 🧬️ The evaluated preview text for the selected generation.
    pub generation_preview_text: Option<String>,
    /// 🧰️ The active transform-gumball utility for the preview window.
    pub active_utility_id: String,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for Procedural3dConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text};
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        )
        .expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

/// 📦️ Handcrafted ArtifactPack (P6): envelope-wrapped pack body via `__dsl_*` record lowering.
impl store::ArtifactPack for Procedural3dConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        )
        .map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

//#endregion 🔖️ArtifactCodec


impl Default for Procedural3dConfig {
    fn default() -> Self {
        Self {
            lod_mode: String::new(),
            show_mode: default_show_mode(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            preview_camera: Procedural3dPreviewCamera::default(),
            sun_json: default_sun_json(),
            selected_generation_id: None,
            generation_preview_text: None,
            active_utility_id: "move".into(),
            locale: "en-US".into()}
    }
}

impl Procedural3dConfig {
    /// 🌞️ Parses `sun_json` — falls back to `WorldSunConfig::default()` on any malformed/legacy value.
    pub fn sun(&self) -> semio_framework_plugin::WorldSunConfig {
        serde_json::from_str(&self.sun_json).unwrap_or_default()
    }
}

store::impl_whole_record_config!(Procedural3dConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ [`Procedural3dConfig`]'s operation enum — one variant per settled interaction, plus a generic
/// `Snapshot` every variant's `backwards()` returns.
// 🧯️ `Snapshot` genuinely needs to carry the whole config by value (it IS the inverse of every other
// variant); boxing it would only relocate the allocation for an enum that is never stored in bulk
// (one value per dispatch, immediately consumed), so the size lint is suppressed rather than chased.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Procedural3dConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Procedural3dConfig},
    #[dsl(key = "lod-mode")]
    SetLodMode { value: String },
    #[dsl(key = "show-mode")]
    SetShowMode { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: CameraJson},
    #[dsl(key = "preview-camera")]
    SetPreviewCamera {
        #[dsl(block)]
        camera: Procedural3dPreviewCamera},
    #[dsl(key = "sun")]
    SetSun { json: String },
    #[dsl(key = "generation")]
    SetGeneration { selected_generation_id: Option<String>, generation_preview_text: Option<String> },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String }}

//#region 🔖️OpCodec
impl protocol::OpText for Procedural3dConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
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

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for Procedural3dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant")})?;
        let spec = (variants[ordinal].1)();
        let body = store::pack_rt::encode_record_body(&spec, &record, &store::PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        store::pack_rt::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = store::pack_rt::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as dsl::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 1,
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len())})?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string()})
    }
}

//#endregion 🔖️OpCodec


impl Mutation<Procedural3dConfig> for Procedural3dConfigMutation {
    type Diff = Procedural3dConfig;

    fn diff(&self, base: &Procedural3dConfig) -> protocol::MutationOutcome<Procedural3dConfig> {
        let mut next = base.clone();
        match self {
            Procedural3dConfigMutation::Snapshot { config } => return protocol::MutationOutcome::new(config.clone()),
            Procedural3dConfigMutation::SetLodMode { value } => next.lod_mode = value.clone(),
            Procedural3dConfigMutation::SetShowMode { value } => next.show_mode = value.clone(),
            Procedural3dConfigMutation::SetCamera { camera } => next.camera = camera.clone(),
            Procedural3dConfigMutation::SetPreviewCamera { camera } => next.preview_camera = camera.clone(),
            Procedural3dConfigMutation::SetSun { json } => next.sun_json = json.clone(),
            Procedural3dConfigMutation::SetGeneration { selected_generation_id, generation_preview_text } => {
                next.selected_generation_id = selected_generation_id.clone();
                next.generation_preview_text = generation_preview_text.clone();
            }
            Procedural3dConfigMutation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            Procedural3dConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Procedural3dConfig) -> Vec<Self> {
        vec![Procedural3dConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn procedural3d_config_default_matches_the_former_runtime_defaults() {
        let config = Procedural3dConfig::default();
        assert_eq!(config.show_mode, "shaded");
        assert_eq!(config.active_utility_id, "move");
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.sun(), semio_framework_plugin::WorldSunConfig::default());
    }

    fn config_round_trip(base: &Procedural3dConfig, operation: &Procedural3dConfigMutation) -> Procedural3dConfig {
        let forward = operation.diff(base).into_parts().0;
        let backwards = operation.inverse(base);
        let mut restored = forward.clone();
        for back in &backwards {
            restored = back.diff(&restored).into_parts().0;
        }
        assert_eq!(&restored, base, "backwards() must exactly restore the pre-operation config");
        forward
    }

    #[test]
    fn config_set_camera_and_preview_camera_round_trip() {
        let base = Procedural3dConfig::default();
        let next = config_round_trip(&base, &Procedural3dConfigMutation::SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } });
        assert_eq!(next.camera, CameraJson { x: 1.0, y: 2.0, zoom: 3.0 });
        let camera = Procedural3dPreviewCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 60.0 };
        let next2 = config_round_trip(&next, &Procedural3dConfigMutation::SetPreviewCamera { camera: camera.clone() });
        assert_eq!(next2.preview_camera, camera);
    }

    #[test]
    fn config_set_sun_round_trip_as_raw_json() {
        let base = Procedural3dConfig::default();
        let next = config_round_trip(&base, &Procedural3dConfigMutation::SetSun { json: "{\"enabled\":true}".into() });
        assert_eq!(next.sun_json, "{\"enabled\":true}");
    }

    #[test]
    fn config_set_generation_round_trips() {
        let base = Procedural3dConfig::default();
        let next = config_round_trip(&base, &Procedural3dConfigMutation::SetGeneration { selected_generation_id: Some("generation-1".into()), generation_preview_text: Some("42".into()) });
        assert_eq!(next.selected_generation_id, Some("generation-1".to_string()));
        assert_eq!(next.generation_preview_text, Some("42".to_string()));
    }

    #[test]
    fn config_set_active_utility_and_locale_round_trip() {
        let base = Procedural3dConfig::default();
        let next = config_round_trip(&base, &Procedural3dConfigMutation::SetActiveUtility { utility_id: "rotate".into() });
        assert_eq!(next.active_utility_id, "rotate");
        let next2 = config_round_trip(&next, &Procedural3dConfigMutation::SetLocale { value: "de-DE".into() });
        assert_eq!(next2.locale, "de-DE");
    }

    #[test]
    fn config_op_text_round_trips_every_variant() {
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Procedural3dConfigMutation::SetLodMode { value: "coarse".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Procedural3dConfigMutation::SetShowMode { value: "wireframe".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Procedural3dConfigMutation::SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Procedural3dConfigMutation::SetPreviewCamera { camera: Procedural3dPreviewCamera { position: [1.0, 2.0, 3.0], target: [4.0, 5.0, 6.0], fov: 45.0 } });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Procedural3dConfigMutation::SetSun { json: "{}".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Procedural3dConfigMutation::SetGeneration { selected_generation_id: Some("g1".into()), generation_preview_text: None });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Procedural3dConfigMutation::SetActiveUtility { utility_id: "scale".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Procedural3dConfigMutation::SetLocale { value: "de-DE".into() });
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Procedural3dConfigMutation::Snapshot { config: Procedural3dConfig::default() });
    }
}
//#endregion 🧪️Tests
