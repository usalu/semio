//! 🧮️ Lowpoly play app — view state (`LowpolyConfig`) and its patch operations
//! (`LowpolyConfigMutation`). Absorbs every field that used to live in the old ui crate's
//! `LowpolyPlayRuntime` app-struct `RefCell` (selection, active object, paint utility/layer, selection
//! method/mode, hover, world camera, sun, and show-edges) — session-only editor settings round-trip through the config
//! `ArtifactStore` exactly like document content, with a real `backwards` per
//! `LowpolyConfigMutation`, mirroring the `shooting_engine::ShootingConfig` pilot. Nested value types
//! (`LowpolySelection`, the world camera, hover target, sun, paint color) are flattened into scalar
//! fields rather than embedded as DSL blocks — `LowpolySelection`/`WorldSunConfig` aren't
//! `dsl::DslField`-capable today and flattening avoids widening that surface just for this migration.

use protocol::Mutation;
use semio_framework_plugin::WorldSunConfig;
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Config
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_os_kernel::DslArtifact, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "lowpoly.lowpolycfg")]
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
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for LowpolyConfig {
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
impl store::ArtifactPack for LowpolyConfig {
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

store::config_diff! {
    record: LowpolyConfig,
    diff: LowpolyConfigDiff,
    fields: {
        active_object_id: String,
        paint_utility: String,
        active_paint_layer: u32,
        utility_params_json: String,
        paint_color_r: u8,
        paint_color_g: u8,
        paint_color_b: u8,
        paint_color_a: u8,
        world_camera_position: [f64; 3],
        world_camera_target: [f64; 3],
        world_camera_fov: f64,
        engagement_input: String,
        show_edges: bool,
        sun_enabled: bool,
        sun_azimuth: f64,
        sun_elevation: f64,
        sun_intensity: f64,
        sun_color: String,
    },
}

/// 🌞️ Reads `LowpolyConfig`'s flattened sun fields back into a `WorldSunConfig` — the boundary where
/// the framework's shared sun toggle/slider helper (`apply_world3d_sun_action`) can operate on it.
pub fn lowpoly_sun_config(config: &LowpolyConfig) -> WorldSunConfig {
    WorldSunConfig { enabled: config.sun_enabled, azimuth: config.sun_azimuth, elevation: config.sun_elevation, intensity: config.sun_intensity, color: config.sun_color.clone() }
}
//#endregion 🔖️Config

//#region 🔖️ConfigMutations
/// 🧮️ B1: `LowpolyConfig`'s operation enum — one variant per settled interaction (mirrors the pre-B1 `LowpolyPlayRuntime` field
/// writes); each variant's inverse is the same variant carrying the base value of exactly the fields it owns.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum, value_derive::ToValue, value_derive::FromValue)]
pub enum LowpolyConfigMutation {
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
}

//#region 🔖️OpCodec
impl protocol::OpText for LowpolyConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown mutation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for LowpolyConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
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
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}

//#endregion 🔖️OpCodec

impl Mutation<LowpolyConfig> for LowpolyConfigMutation {
    type Diff = LowpolyConfigDiff;

    /// 🧷️ Provisional per-variant leaf metadata for this hand-written (non-derived) aggregate —
    /// `diff`/`inverse` dispatch here is a plain `match`, not the derive's per-leaf `MutationKind`
    /// shape. One entry per variant, in declaration order, mirroring `generation2d`'s identical
    /// precedent for its own hand-written config aggregate.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-active-object",
            semantic_kind: "set-active-object",
            display_name: "Set Active Object",
            emoji: "⚙️",
            aggregate_variant: "SetActiveObject",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-paint-utility",
            semantic_kind: "set-paint-utility",
            display_name: "Set Paint Utility",
            emoji: "⚙️",
            aggregate_variant: "SetPaintUtility",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-active-paint-layer",
            semantic_kind: "set-active-paint-layer",
            display_name: "Set Active Paint Layer",
            emoji: "⚙️",
            aggregate_variant: "SetActivePaintLayer",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-utility-params",
            semantic_kind: "set-utility-params",
            display_name: "Set Utility Params",
            emoji: "⚙️",
            aggregate_variant: "SetUtilityParams",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-paint-color",
            semantic_kind: "set-paint-color",
            display_name: "Set Paint Color",
            emoji: "⚙️",
            aggregate_variant: "SetPaintColor",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-world-camera",
            semantic_kind: "set-world-camera",
            display_name: "Set World Camera",
            emoji: "⚙️",
            aggregate_variant: "SetWorldCamera",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-engagement-input",
            semantic_kind: "set-engagement-input",
            display_name: "Set Engagement Input",
            emoji: "⚙️",
            aggregate_variant: "SetEngagementInput",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-show-edges",
            semantic_kind: "set-show-edges",
            display_name: "Set Show Edges",
            emoji: "⚙️",
            aggregate_variant: "SetShowEdges",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-sun",
            semantic_kind: "set-sun",
            display_name: "Set Sun",
            emoji: "⚙️",
            aggregate_variant: "SetSun",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            LowpolyConfigMutation::SetActiveObject { .. } => &Self::DESCRIPTORS[0],
            LowpolyConfigMutation::SetPaintUtility { .. } => &Self::DESCRIPTORS[1],
            LowpolyConfigMutation::SetActivePaintLayer { .. } => &Self::DESCRIPTORS[2],
            LowpolyConfigMutation::SetUtilityParams { .. } => &Self::DESCRIPTORS[3],
            LowpolyConfigMutation::SetPaintColor { .. } => &Self::DESCRIPTORS[4],
            LowpolyConfigMutation::SetWorldCamera { .. } => &Self::DESCRIPTORS[5],
            LowpolyConfigMutation::SetEngagementInput { .. } => &Self::DESCRIPTORS[6],
            LowpolyConfigMutation::SetShowEdges { .. } => &Self::DESCRIPTORS[7],
            LowpolyConfigMutation::SetSun { .. } => &Self::DESCRIPTORS[8],
        }
    }

    /// 📦️ Each variant sets only the slots of the fields it owns.
    fn diff(&self, base: &LowpolyConfig) -> protocol::MutationOutcome<LowpolyConfigDiff> {
        protocol::MutationOutcome::new(match self {
            LowpolyConfigMutation::SetActiveObject { object_id } => LowpolyConfigDiff { active_object_id: Some(object_id.clone()), ..Default::default() },
            LowpolyConfigMutation::SetPaintUtility { value } => LowpolyConfigDiff { paint_utility: Some(value.clone()), ..Default::default() },
            LowpolyConfigMutation::SetActivePaintLayer { value } => LowpolyConfigDiff { active_paint_layer: Some(*value), ..Default::default() },
            LowpolyConfigMutation::SetUtilityParams { json } => LowpolyConfigDiff { utility_params_json: Some(json.clone()), ..Default::default() },
            LowpolyConfigMutation::SetPaintColor { r, g, b, a } => LowpolyConfigDiff { paint_color_r: Some(*r), paint_color_g: Some(*g), paint_color_b: Some(*b), paint_color_a: Some(*a), ..Default::default() },
            LowpolyConfigMutation::SetWorldCamera { position, target, fov } => LowpolyConfigDiff { world_camera_position: Some(*position), world_camera_target: Some(*target), world_camera_fov: Some(*fov), ..Default::default() },
            LowpolyConfigMutation::SetEngagementInput { value } => LowpolyConfigDiff { engagement_input: Some(value.clone()), ..Default::default() },
            LowpolyConfigMutation::SetShowEdges { value } => LowpolyConfigDiff { show_edges: Some(*value), ..Default::default() },
            LowpolyConfigMutation::SetSun { enabled, azimuth, elevation, intensity, color } => LowpolyConfigDiff {
                sun_enabled: Some(*enabled),
                sun_azimuth: Some(*azimuth),
                sun_elevation: Some(*elevation),
                sun_intensity: Some(*intensity),
                sun_color: Some(color.clone()),
                ..Default::default()
            },
        })
    }

    fn inverse(&self, base: &LowpolyConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            LowpolyConfigMutation::SetActiveObject { .. } => LowpolyConfigMutation::SetActiveObject { object_id: base.active_object_id.clone() },
            LowpolyConfigMutation::SetPaintUtility { .. } => LowpolyConfigMutation::SetPaintUtility { value: base.paint_utility.clone() },
            LowpolyConfigMutation::SetActivePaintLayer { .. } => LowpolyConfigMutation::SetActivePaintLayer { value: base.active_paint_layer },
            LowpolyConfigMutation::SetUtilityParams { .. } => LowpolyConfigMutation::SetUtilityParams { json: base.utility_params_json.clone() },
            LowpolyConfigMutation::SetPaintColor { .. } => LowpolyConfigMutation::SetPaintColor { r: base.paint_color_r, g: base.paint_color_g, b: base.paint_color_b, a: base.paint_color_a },
            LowpolyConfigMutation::SetWorldCamera { .. } => LowpolyConfigMutation::SetWorldCamera { position: base.world_camera_position, target: base.world_camera_target, fov: base.world_camera_fov },
            LowpolyConfigMutation::SetEngagementInput { .. } => LowpolyConfigMutation::SetEngagementInput { value: base.engagement_input.clone() },
            LowpolyConfigMutation::SetShowEdges { .. } => LowpolyConfigMutation::SetShowEdges { value: base.show_edges },
            LowpolyConfigMutation::SetSun { .. } => LowpolyConfigMutation::SetSun {
                enabled: base.sun_enabled,
                azimuth: base.sun_azimuth,
                elevation: base.sun_elevation,
                intensity: base.sun_intensity,
                color: base.sun_color.clone(),
            },
        }])
    }
}
//#endregion 🔖️ConfigMutations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
