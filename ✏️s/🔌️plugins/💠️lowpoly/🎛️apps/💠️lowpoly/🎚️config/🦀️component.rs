//! 🧮️ Lowpoly play app — view state (`LowpolyConfig`) and its patch operations
//! (`LowpolyConfigMutation`). Absorbs every field that used to live in the old ui crate's
//! `LowpolyPlayRuntime` app-struct `RefCell` (selection, active object, paint utility/layer, selection
//! method/mode, hover, world camera, sun, show-edges) plus the two `ViewModel` fields lowpoly actually
//! read (`active_utility_id`/`locale`) — session-only view state round-trips through the config
//! `ArtifactStore` exactly like document content, with a real `backwards` per
//! `LowpolyConfigMutation`, mirroring the `shooting_engine::ShootingConfig` pilot. Nested value types
//! (`LowpolySelection`, the world camera, hover target, sun, paint color) are flattened into scalar
//! fields rather than embedded as DSL blocks — `LowpolySelection`/`WorldSunConfig` aren't
//! `dsl::DslField`-capable today and flattening avoids widening that surface just for this migration.

use protocol::Mutation;
use semio_framework_plugin::WorldSunConfig;
use serde::{Deserialize, Serialize};

//#region 🔖️Config
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslArtifact)]
#[serde(rename_all = "camelCase", default)]
#[dsl(extension = "lowpoly.lowpolycfg")]
#[dsl(layout = "lines")]
pub struct LowpolyConfig {
    /// 👁️ Was `LowpolyPlayRuntime::active_object_id`.
    pub active_object_id: String,
    /// 👁️ Was `LowpolyPlayRuntime::paint_utility`.
    pub paint_utility: String,
    /// 👁️ Was `LowpolyPlayRuntime::active_paint_layer`.
    pub active_paint_layer: u32,
    /// 👁️ Was `LowpolyPlayRuntime::utility_params` (`serde_json::Value`) — carried as canonical JSON
    /// text since a raw `Value` field has no direct DSL binding.
    pub utility_params_json: String,
    /// 🎨️ Was `LowpolyPlayRuntime::paint_color` (`[u8; 4]`), flattened.
    pub paint_color_r: u8,
    pub paint_color_g: u8,
    pub paint_color_b: u8,
    pub paint_color_a: u8,
    /// 🎥️ Was `LowpolyPlayRuntime::world_camera` (`LowpolyWorldCamera`), flattened.
    #[dsl(coord)]
    pub world_camera_position: [f64; 3],
    #[dsl(coord)]
    pub world_camera_target: [f64; 3],
    #[dsl(angle = "deg")]
    pub world_camera_fov: f64,
    /// 👁️ Was `LowpolyPlayRuntime::engagement_input`.
    pub engagement_input: String,
    /// 👁️ Was `LowpolyPlayRuntime::show_edges`.
    pub show_edges: bool,
    /// 🌞️ Was `LowpolyPlayRuntime::sun` (`WorldSunConfig`), flattened.
    pub sun_enabled: bool,
    pub sun_azimuth: f64,
    pub sun_elevation: f64,
    pub sun_intensity: f64,
    pub sun_color: String,
    /// 🧰️ Was read off the host-pushed `ViewModel::active_utility_id` (deleted for migrated apps).
    pub active_utility_id: String,
    /// 🗣️ Was read off `ViewModel::locale`.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for LowpolyConfig {
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
impl store::ArtifactPack for LowpolyConfig {
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


impl Default for LowpolyConfig {
    fn default() -> Self {
        Self {
            active_object_id: String::new(),
            paint_utility: "brush".into(),
            active_paint_layer: 0,
            utility_params_json: default_utility_params_json(),
            paint_color_r: 255,
            paint_color_g: 64,
            paint_color_b: 64,
            paint_color_a: 255,
            world_camera_position: [18.0, -18.0, 12.0],
            world_camera_target: [0.0, 0.0, 0.0],
            world_camera_fov: 45.0,
            engagement_input: String::new(),
            show_edges: true,
            sun_enabled: false,
            sun_azimuth: 45.0,
            sun_elevation: 35.0,
            sun_intensity: 0.85,
            sun_color: "#ffffff".into(),
            active_utility_id: "move".into(),
            locale: "en-US".into(),
        }
    }
}

/// 🧰️ `LowpolyConfig::default`'s `utility_params_json` — mirrors the pre-B1
/// `LowpolyPlayRuntime::utility_params`'s default JSON object verbatim.
pub fn default_utility_params_json() -> String {
    serde_json::json!({
        "extrudeDistance": 0.25,
        "insetAmount": 0.1,
        "bevelAmount": 0.05,
        "bevelSegments": 1,
        "loopCuts": 1,
        "decimateRatio": 0.5,
        "snapGrid": 0.25,
        "mirrorAxis": 0,
        "brushSize": 16,
        "brushOpacity": 1,
        "brushHardness": 0.5,
    })
    .to_string()
}

store::impl_whole_record_config!(LowpolyConfig);

/// 🌞️ Reads `LowpolyConfig`'s flattened sun fields back into a `WorldSunConfig` — the boundary where
/// the framework's shared sun toggle/slider helper (`apply_world3d_sun_action`) can operate on it.
pub fn lowpoly_sun_config(config: &LowpolyConfig) -> WorldSunConfig {
    WorldSunConfig { enabled: config.sun_enabled, azimuth: config.sun_azimuth, elevation: config.sun_elevation, intensity: config.sun_intensity, color: config.sun_color.clone() }
}
//#endregion 🔖️Config

//#region 🔖️ConfigMutations
/// @emoji 🧮️ B1: `LowpolyConfig`'s operation enum — one variant per settled interaction (mirrors the
/// pre-B1 `LowpolyPlayRuntime` field writes), plus a generic `Snapshot` every variant's `backwards()`
/// returns — mirrors `shooting_op::ShootingConfigOperation`'s identical pattern: a config-only dispatch
/// is always a plain `Apply` (never `AmendLast`), so "undo this tick" = "restore the whole-config
/// snapshot from just before it", the simplest correct inverse.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[allow(clippy::large_enum_variant, reason = "Snapshot must carry the whole LowpolyConfig by value (not boxed) so its dsl(block)-derived wire encoding stays byte-identical to the pre-migration wire format; every variant is dispatched rarely (config-only ticks), never in a hot allocation path")]
pub enum LowpolyConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: LowpolyConfig,
    },
    #[dsl(key = "active-object")]
    SetActiveObject { object_id: String },
    #[dsl(key = "paint-utility")]
    SetPaintUtility { value: String },
    #[dsl(key = "active-paint-layer")]
    SetActivePaintLayer { value: u32 },
    #[dsl(key = "utility-params")]
    SetUtilityParams { json: String },
    #[dsl(key = "paint-color")]
    SetPaintColor { r: u8, g: u8, b: u8, a: u8 },
    #[dsl(key = "world-camera")]
    SetWorldCamera {
        #[dsl(coord)]
        position: [f64; 3],
        #[dsl(coord)]
        target: [f64; 3],
        fov: f64,
    },
    #[dsl(key = "engagement-input")]
    SetEngagementInput { value: String },
    #[dsl(key = "show-edges")]
    SetShowEdges { value: bool },
    #[dsl(key = "sun")]
    SetSun { enabled: bool, azimuth: f64, elevation: f64, intensity: f64, color: String },
    #[dsl(key = "active-utility")]
    SetActiveUtility { utility_id: String },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for LowpolyConfigMutation {
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
impl protocol::OpBinary for LowpolyConfigMutation {
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


impl Mutation<LowpolyConfig> for LowpolyConfigMutation {
    type Diff = LowpolyConfig;

    fn diff(&self, base: &LowpolyConfig) -> LowpolyConfig {
        let mut next = base.clone();
        match self {
            LowpolyConfigMutation::Snapshot { config } => return config.clone(),
            LowpolyConfigMutation::SetActiveObject { object_id } => next.active_object_id = object_id.clone(),
            LowpolyConfigMutation::SetPaintUtility { value } => next.paint_utility = value.clone(),
            LowpolyConfigMutation::SetActivePaintLayer { value } => next.active_paint_layer = *value,
            LowpolyConfigMutation::SetUtilityParams { json } => next.utility_params_json = json.clone(),
            LowpolyConfigMutation::SetPaintColor { r, g, b, a } => {
                next.paint_color_r = *r;
                next.paint_color_g = *g;
                next.paint_color_b = *b;
                next.paint_color_a = *a;
            }
            LowpolyConfigMutation::SetWorldCamera { position, target, fov } => {
                next.world_camera_position = *position;
                next.world_camera_target = *target;
                next.world_camera_fov = *fov;
            }
            LowpolyConfigMutation::SetEngagementInput { value } => next.engagement_input = value.clone(),
            LowpolyConfigMutation::SetShowEdges { value } => next.show_edges = *value,
            LowpolyConfigMutation::SetSun { enabled, azimuth, elevation, intensity, color } => {
                next.sun_enabled = *enabled;
                next.sun_azimuth = *azimuth;
                next.sun_elevation = *elevation;
                next.sun_intensity = *intensity;
                next.sun_color = color.clone();
            }
            LowpolyConfigMutation::SetActiveUtility { utility_id } => next.active_utility_id = utility_id.clone(),
            LowpolyConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn inverse(&self, base: &LowpolyConfig) -> Vec<Self> {
        vec![LowpolyConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigMutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use store::ArtifactPack;

    #[test]
    fn lowpoly_config_dsl_round_trips_default() {
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&LowpolyConfig::default());
    }

    #[test]
    fn lowpoly_config_dsl_round_trips_non_default() {
        let config = LowpolyConfig { active_object_id: "obj-2".into(), locale: "de-DE".into(), ..LowpolyConfig::default() };
        semio_framework_os_kernel::os_store::test_support::assert_dsl_round_trip(&config);
    }

    #[test]
    fn lowpoly_config_pack_round_trips() {
        let config = LowpolyConfig { active_object_id: "obj-9".into(), sun_enabled: true, ..LowpolyConfig::default() };
        let bytes = config.encode_pack();
        let restored = LowpolyConfig::decode_pack(&bytes).expect("decode");
        assert_eq!(restored, config);
    }

    #[test]
    fn config_op_backwards_always_snapshots_prior_state() {
        let base = LowpolyConfig { active_object_id: "obj-1".into(), ..LowpolyConfig::default() };
        let operation = LowpolyConfigMutation::SetActiveObject { object_id: "obj-2".into() };
        let after = operation.diff(&base);
        assert_eq!(after.active_object_id, "obj-2");
        let backwards = operation.inverse(&base);
        assert_eq!(backwards, vec![LowpolyConfigMutation::Snapshot { config: base.clone() }]);
        assert_eq!(backwards[0].diff(&after), base);
    }

    #[test]
    fn config_op_text_round_trip_set_active_object() {
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&LowpolyConfigMutation::SetActiveObject { object_id: "obj-2".into() });
    }

    #[test]
    fn config_op_text_round_trip_world_camera() {
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&LowpolyConfigMutation::SetWorldCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 });
    }

    #[test]
    fn config_op_text_round_trip_snapshot() {
        semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&LowpolyConfigMutation::Snapshot { config: LowpolyConfig::default() });
    }

    #[test]
    fn config_op_binary_round_trips_and_agrees_with_text() {
        let operation = LowpolyConfigMutation::SetActivePaintLayer { value: 3 };
        semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&operation);
    }
}
//#endregion 🧪️Tests
