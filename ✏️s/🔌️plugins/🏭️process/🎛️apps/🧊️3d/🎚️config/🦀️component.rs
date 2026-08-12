//! 🧮️ Process 3d play app — view state (`Process3dConfig`) and its operation enum
//! (`Process3dConfigMutation`), moved out of the old `⚙️engine`/`🔧️op` crates: this is APP state (view
//! state, never document content), so it belongs next to the app that owns it, not the artifact.
//!
//! B1: absorbs every field that used to live in the old UI crate's `Process3dRuntime` app-struct
//! `RefCell` (selection, hover, face pick, selection method, engagement input, camera, sun) plus the two
//! `ViewModel` fields process3d actually read (`active_utility_id`/`locale`) — session-only view state
//! now round-trips through the config `ArtifactStore` exactly like document content, with a real
//! `backwards` per [`Process3dConfigMutation`], mirroring the `shooting_engine::ShootingConfig` pilot.
//! The camera (was `Process3dCamera`) and sun (was `WorldSunConfig`) are flattened into scalar fields
//! rather than embedded as DSL blocks — neither type derives `dsl::DslRecord`, and `WorldSunConfig` is
//! shared framework state out of scope for this migration (mirrors `lowpoly_engine::LowpolyConfig`'s
//! identical flattening of its own world camera/sun).

use protocol::Mutation;
use serde::{Deserialize, Serialize};

/// 🧰️ The utility active when the config carries no explicit override.
pub const PROCESS3D_DEFAULT_UTILITY: &str = "select";

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "process3dcfg")]
#[dsl(id = "process3d.config")]
#[dsl(layout = "lines")]
pub struct Process3dConfig {
    /// 👁️ Was `Process3dRuntime::selected_id`.
    pub selected_id: Option<String>,
    /// 👁️ Was `Process3dRuntime::hovered_id`.
    pub hovered_id: Option<String>,
    /// 🖱️ Was `Process3dRuntime::selected_face_id`.
    pub selected_face_id: Option<u32>,
    /// 👁️ Was `Process3dRuntime::selection_method`.
    pub selection_method: String,
    /// 👁️ Was `Process3dRuntime::engagement_input`.
    pub engagement_input: String,
    /// 🎥️ Was `Process3dRuntime::camera` (`Process3dCamera`), flattened.
    #[dsl(coord)]
    pub camera_position: [f64; 3],
    #[dsl(coord)]
    pub camera_target: [f64; 3],
    pub camera_fov: f64,
    /// 🌞️ Was `Process3dRuntime::sun` (`WorldSunConfig`), flattened.
    pub sun_enabled: bool,
    pub sun_azimuth: f64,
    pub sun_elevation: f64,
    pub sun_intensity: f64,
    pub sun_color: String,
    /// 🧰️ Was read off the host-pushed `ViewModel::active_utility_id` (deleted for migrated apps).
    pub active_utility_id: String,
    /// 🗣️ Was read off `ViewModel::locale`.
    pub locale: String,
    /// 🧩️ Host-pushed `ProgramContributionEntry[]` JSON for `process.machines` hot-swap installs.
    #[serde(default = "default_contributions_json")]
    pub contributions_json: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for Process3dConfig {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
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
impl store::ArtifactPack for Process3dConfig {
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


fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for Process3dConfig {
    fn default() -> Self {
        Self {
            selected_id: None,
            hovered_id: None,
            selected_face_id: None,
            selection_method: "rectangle".into(),
            engagement_input: String::new(),
            camera_position: [3.0, -3.0, 2.0],
            camera_target: [0.0, 0.0, 0.0],
            camera_fov: 45.0,
            sun_enabled: false,
            sun_azimuth: 45.0,
            sun_elevation: 35.0,
            sun_intensity: 0.85,
            sun_color: "#ffffff".into(),
            active_utility_id: PROCESS3D_DEFAULT_UTILITY.into(),
            locale: "en-US".into(),
            contributions_json: default_contributions_json(),
        }
    }
}

impl Process3dConfig {
    /// 🧰️ Resolves the config-owned active utility, falling back to [`PROCESS3D_DEFAULT_UTILITY`] (only
    /// ever triggers if a config value somehow arrives empty).
    pub fn active_utility(&self) -> &str {
        if self.active_utility_id.is_empty() {
            PROCESS3D_DEFAULT_UTILITY
        } else {
            self.active_utility_id.as_str()
        }
    }
}

store::impl_whole_record_config!(Process3dConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ [`Process3dConfig`]'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `Process3dRuntime` field writes). Every field already carries its own setter, so `backwards()`
/// returns the SAME variant re-addressed at `base`'s old value — a targeted, in-kind inverse per
/// this ticket's ban on whole-record replace, rather than a generic whole-config snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Process3dConfigMutation {
    #[dsl(key = "selected-id")]
    SetSelectedId { value: Option<String> },
    #[dsl(key = "hovered-id")]
    SetHoveredId { value: Option<String> },
    #[dsl(key = "selected-face-id")]
    SetSelectedFaceId { value: Option<u32> },
    #[dsl(key = "engagement-input")]
    SetEngagementInput { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(coord)]
        position: [f64; 3],
        #[dsl(coord)]
        target: [f64; 3],
        fov: f64,
    },
    #[dsl(key = "sun")]
    SetSun { enabled: bool, azimuth: f64, elevation: f64, intensity: f64, color: String },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
    #[dsl(key = "contributions")]
    SetContributions { json: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for Process3dConfigMutation {
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
impl protocol::OpBinary for Process3dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed {
            what: "op variant",
            offset: 0,
            detail: format!("keyword {keyword:?} is not a declared variant"),
        })?;
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
            detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()),
        })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed {
            what: "op record",
            offset: reader.position() as u64,
            detail: error.to_string(),
        })
    }
}

//#endregion 🔖️OpCodec


impl Mutation<Process3dConfig> for Process3dConfigMutation {
    type Diff = Process3dConfig;

    fn diff(&self, base: &Process3dConfig) -> Process3dConfig {
        let mut next = base.clone();
        match self {
            Process3dConfigMutation::SetSelectedId { value } => next.selected_id = value.clone(),
            Process3dConfigMutation::SetHoveredId { value } => next.hovered_id = value.clone(),
            Process3dConfigMutation::SetSelectedFaceId { value } => next.selected_face_id = *value,
            Process3dConfigMutation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            Process3dConfigMutation::SetCamera { position, target, fov } => {
                next.camera_position = *position;
                next.camera_target = *target;
                next.camera_fov = *fov;
            }
            Process3dConfigMutation::SetSun { enabled, azimuth, elevation, intensity, color } => {
                next.sun_enabled = *enabled;
                next.sun_azimuth = *azimuth;
                next.sun_elevation = *elevation;
                next.sun_intensity = *intensity;
                next.sun_color = color.clone();
            }
            Process3dConfigMutation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            Process3dConfigMutation::SetLocale { value } => next.locale = value.clone(),
            Process3dConfigMutation::SetContributions { json } => {
                next.contributions_json = json.clone();
                crate::artifacts::process3d::engine::sync_process_machine_contributions(json);
            }
        }
        next
    }

    fn inverse(&self, base: &Process3dConfig) -> Vec<Self> {
        match self {
            Process3dConfigMutation::SetSelectedId { .. } => vec![Process3dConfigMutation::SetSelectedId { value: base.selected_id.clone() }],
            Process3dConfigMutation::SetHoveredId { .. } => vec![Process3dConfigMutation::SetHoveredId { value: base.hovered_id.clone() }],
            Process3dConfigMutation::SetSelectedFaceId { .. } => vec![Process3dConfigMutation::SetSelectedFaceId { value: base.selected_face_id }],
            Process3dConfigMutation::SetEngagementInput { .. } => vec![Process3dConfigMutation::SetEngagementInput { value: base.engagement_input.clone() }],
            Process3dConfigMutation::SetCamera { .. } => {
                vec![Process3dConfigMutation::SetCamera { position: base.camera_position, target: base.camera_target, fov: base.camera_fov }]
            }
            Process3dConfigMutation::SetSun { .. } => vec![Process3dConfigMutation::SetSun {
                enabled: base.sun_enabled,
                azimuth: base.sun_azimuth,
                elevation: base.sun_elevation,
                intensity: base.sun_intensity,
                color: base.sun_color.clone(),
            }],
            Process3dConfigMutation::SetActiveUtility { .. } => vec![Process3dConfigMutation::SetActiveUtility { utility_id: base.active_utility_id.clone() }],
            Process3dConfigMutation::SetLocale { .. } => vec![Process3dConfigMutation::SetLocale { value: base.locale.clone() }],
            Process3dConfigMutation::SetContributions { .. } => vec![Process3dConfigMutation::SetContributions { json: base.contributions_json.clone() }],
        }
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process3d_config_dsl_and_pack_round_trip() {
        use store::ArtifactPack;
        let config = Process3dConfig { selected_id: Some("stock".into()), hovered_id: Some("step-0".into()), selected_face_id: Some(3), sun_enabled: true, active_utility_id: "cut".into(), ..Process3dConfig::default() };
        store::os_store::test_support::assert_dsl_round_trip(&config);
        let bytes = config.encode_pack();
        assert_eq!(Process3dConfig::decode_pack(&bytes).expect("decode"), config);
    }

    #[test]
    fn process3d_config_operation_backwards_restores_the_same_field_from_base() {
        let base = Process3dConfig::default();
        let operation = Process3dConfigMutation::SetSelectedId { value: Some("step-0".into()) };
        let inverse = operation.inverse(&base);
        assert_eq!(inverse, vec![Process3dConfigMutation::SetSelectedId { value: base.selected_id }]);
    }

    #[test]
    fn process3d_config_operation_diff_applies_expected_fields() {
        let base = Process3dConfig::default();
        let next = Process3dConfigMutation::SetCamera { position: [1.0, 2.0, 3.0], target: [0.1, 0.2, 0.3], fov: 60.0 }.diff(&base);
        assert_eq!(next.camera_position, [1.0, 2.0, 3.0]);
        assert_eq!(next.camera_target, [0.1, 0.2, 0.3]);
        assert_eq!(next.camera_fov, 60.0);

        let next = Process3dConfigMutation::SetSun { enabled: true, azimuth: 10.0, elevation: 20.0, intensity: 0.5, color: "#123456".into() }.diff(&base);
        assert!(next.sun_enabled);
        assert_eq!(next.sun_azimuth, 10.0);
        assert_eq!(next.sun_elevation, 20.0);
        assert_eq!(next.sun_intensity, 0.5);
        assert_eq!(next.sun_color, "#123456");
    }

    #[test]
    fn process3d_config_op_text_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetSelectedId { value: Some("stock".into()) });
        store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetSelectedId { value: None });
        store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetHoveredId { value: Some("step-0".into()) });
        store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetSelectedFaceId { value: Some(3) });
        store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetSelectedFaceId { value: None });
        store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetEngagementInput { value: "cut".into() });
        store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetCamera { position: [1.0, 2.0, 3.0], target: [0.1, 0.2, 0.3], fov: 60.0 });
        store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetSun { enabled: true, azimuth: 10.0, elevation: 20.0, intensity: 0.5, color: "#123456".into() });
        store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetActiveUtility { utility_id: "cut".into() });
        store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetLocale { value: "de-DE".into() });
        store::os_store::test_support::assert_op_line_round_trip(&Process3dConfigMutation::SetContributions { json: "[]".into() });
    }

    #[test]
    fn process3d_config_default_matches_the_existing_runtime_defaults() {
        let config = Process3dConfig::default();
        assert_eq!(config.selection_method, "rectangle");
        assert_eq!(config.camera_position, [3.0, -3.0, 2.0]);
        assert_eq!(config.camera_target, [0.0, 0.0, 0.0]);
        assert_eq!(config.camera_fov, 45.0);
        assert!(!config.sun_enabled);
        assert_eq!(config.active_utility_id, "select");
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.active_utility(), "select");
    }
}
//#endregion 🧪️Tests
