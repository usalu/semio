//! 🧮️ Process 3d play app — view state (`Process3dConfig`) and its operation enum
//! (`Process3dConfigMutation`), moved out of the old `⚙️engine`/`🔧️op` crates: this is APP state (view
//! state, never document content), so it belongs next to the app that owns it, not the artifact.
//!
//! B1: absorbs every field that used to live in the old UI crate's `Process3dRuntime` app-struct
//! `RefCell` (selection, hover, face pick, selection method, engagement input, camera, sun) plus the two
//! plugin-owned settings — session-only editor settings round-trip through the config `ArtifactStore`, with a real
//! `backwards` per [`Process3dConfigMutation`], mirroring the `shooting_engine::ShootingConfig` pilot.
//! The camera (was `Process3dCamera`) and sun (was `WorldSunConfig`) are flattened into scalar fields
//! rather than embedded as DSL blocks — neither type derives `dsl::DslRecord`, and `WorldSunConfig` is
//! shared framework state out of scope for this migration (mirrors `lowpoly_engine::LowpolyConfig`'s
//! identical flattening of its own world camera/sun).

use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};

/// 🧰️ The utility active when the config carries no explicit override.
pub const PROCESS3D_DEFAULT_UTILITY: &str = "select";

//#region 🔖️Config
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "process3dcfg")]
#[artifact(id = "process3d.config")]
#[dsl(layout = "lines")]
pub struct Process3dConfig {
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
    /// 🧩️ Host-pushed `ProgramContributionEntry[]` JSON for `process.machines` hot-swap installs.
    #[value(default = "default_contributions_json")]
    pub contributions_json: String,
    /// ⏱️ The replay cursor: how many timeline steps the viewer resolves, `None` = all of them. VIEW state — the
    /// document is the whole process; stepping through it is never an edit and never history.
    pub resolved_up_to: Option<usize>,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for Process3dConfig {
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
impl store::ArtifactPack for Process3dConfig {
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

fn default_contributions_json() -> String {
    "[]".into()
}

impl Default for Process3dConfig {
    fn default() -> Self {
        Self {
            engagement_input: String::new(),
            camera_position: [3.0, -3.0, 2.0],
            camera_target: [0.0, 0.0, 0.0],
            camera_fov: 45.0,
            sun_enabled: false,
            sun_azimuth: 45.0,
            sun_elevation: 35.0,
            sun_intensity: 0.85,
            sun_color: "#ffffff".into(),
            contributions_json: default_contributions_json(),
            resolved_up_to: None,
        }
    }
}

impl store::ConfigRecord for Process3dConfig {}

/// 🧱️ Carries an optional value as a present slot, so clearing it stays distinct from leaving it untouched on every wire.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Process3dOptionalCursor {
    pub value: Option<usize>,
}

/// 🔺️ Sparse field delta over [`Process3dConfig`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Process3dConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub engagement_input: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera_position: Option<[f64; 3]>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera_target: Option<[f64; 3]>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera_fov: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub sun_enabled: Option<bool>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub sun_azimuth: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub sun_elevation: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub sun_intensity: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub sun_color: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub contributions_json: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub resolved_up_to: Option<Process3dOptionalCursor>,
}

impl protocol::MutationDiff<Process3dConfig> for Process3dConfigDiff {
    fn apply(&self, base: &Process3dConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Process3dConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.engagement_input {
            next.engagement_input = value.clone();
        }
        if let Some(value) = &self.camera_position {
            next.camera_position = value.clone();
        }
        if let Some(value) = &self.camera_target {
            next.camera_target = value.clone();
        }
        if let Some(value) = &self.camera_fov {
            next.camera_fov = value.clone();
        }
        if let Some(value) = &self.sun_enabled {
            next.sun_enabled = value.clone();
        }
        if let Some(value) = &self.sun_azimuth {
            next.sun_azimuth = value.clone();
        }
        if let Some(value) = &self.sun_elevation {
            next.sun_elevation = value.clone();
        }
        if let Some(value) = &self.sun_intensity {
            next.sun_intensity = value.clone();
        }
        if let Some(value) = &self.sun_color {
            next.sun_color = value.clone();
        }
        if let Some(value) = &self.contributions_json {
            next.contributions_json = value.clone();
        }
        if let Some(value) = &self.resolved_up_to {
            next.resolved_up_to = value.value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.engagement_input.is_some() {
            self.engagement_input = other.engagement_input;
        }
        if other.camera_position.is_some() {
            self.camera_position = other.camera_position;
        }
        if other.camera_target.is_some() {
            self.camera_target = other.camera_target;
        }
        if other.camera_fov.is_some() {
            self.camera_fov = other.camera_fov;
        }
        if other.sun_enabled.is_some() {
            self.sun_enabled = other.sun_enabled;
        }
        if other.sun_azimuth.is_some() {
            self.sun_azimuth = other.sun_azimuth;
        }
        if other.sun_elevation.is_some() {
            self.sun_elevation = other.sun_elevation;
        }
        if other.sun_intensity.is_some() {
            self.sun_intensity = other.sun_intensity;
        }
        if other.sun_color.is_some() {
            self.sun_color = other.sun_color;
        }
        if other.contributions_json.is_some() {
            self.contributions_json = other.contributions_json;
        }
        if other.resolved_up_to.is_some() {
            self.resolved_up_to = other.resolved_up_to;
        }
    }
}

impl protocol::DiffAlgebra<Process3dConfig> for Process3dConfigDiff {
    fn inverse(&self, base: &Process3dConfig) -> Self {
        Self {
            engagement_input: self.engagement_input.as_ref().map(|_| base.engagement_input.clone()),
            camera_position: self.camera_position.as_ref().map(|_| base.camera_position.clone()),
            camera_target: self.camera_target.as_ref().map(|_| base.camera_target.clone()),
            camera_fov: self.camera_fov.as_ref().map(|_| base.camera_fov.clone()),
            sun_enabled: self.sun_enabled.as_ref().map(|_| base.sun_enabled.clone()),
            sun_azimuth: self.sun_azimuth.as_ref().map(|_| base.sun_azimuth.clone()),
            sun_elevation: self.sun_elevation.as_ref().map(|_| base.sun_elevation.clone()),
            sun_intensity: self.sun_intensity.as_ref().map(|_| base.sun_intensity.clone()),
            sun_color: self.sun_color.as_ref().map(|_| base.sun_color.clone()),
            contributions_json: self.contributions_json.as_ref().map(|_| base.contributions_json.clone()),
            resolved_up_to: self.resolved_up_to.as_ref().map(|_| Process3dOptionalCursor { value: base.resolved_up_to.clone() }),
        }
    }
    fn between(base: &Process3dConfig, other: &Process3dConfig) -> Self {
        Self {
            engagement_input: (base.engagement_input != other.engagement_input).then(|| other.engagement_input.clone()),
            camera_position: (base.camera_position != other.camera_position).then(|| other.camera_position.clone()),
            camera_target: (base.camera_target != other.camera_target).then(|| other.camera_target.clone()),
            camera_fov: (base.camera_fov != other.camera_fov).then(|| other.camera_fov.clone()),
            sun_enabled: (base.sun_enabled != other.sun_enabled).then(|| other.sun_enabled.clone()),
            sun_azimuth: (base.sun_azimuth != other.sun_azimuth).then(|| other.sun_azimuth.clone()),
            sun_elevation: (base.sun_elevation != other.sun_elevation).then(|| other.sun_elevation.clone()),
            sun_intensity: (base.sun_intensity != other.sun_intensity).then(|| other.sun_intensity.clone()),
            sun_color: (base.sun_color != other.sun_color).then(|| other.sun_color.clone()),
            contributions_json: (base.contributions_json != other.contributions_json).then(|| other.contributions_json.clone()),
            resolved_up_to: (base.resolved_up_to != other.resolved_up_to).then(|| Process3dOptionalCursor { value: other.resolved_up_to.clone() }),
        }
    }
    fn is_empty(&self) -> bool {
        self.engagement_input.is_none() && self.camera_position.is_none() && self.camera_target.is_none() && self.camera_fov.is_none() && self.sun_enabled.is_none() && self.sun_azimuth.is_none() && self.sun_elevation.is_none() && self.sun_intensity.is_none() && self.sun_color.is_none() && self.contributions_json.is_none() && self.resolved_up_to.is_none()
    }
}

//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ [`Process3dConfig`]'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `Process3dRuntime` field writes). Every field already carries its own setter, so `backwards()`
/// returns the SAME variant re-addressed at `base`'s old value — a targeted, in-kind inverse per
/// this ticket's ban on whole-record replace, rather than a generic whole-config snapshot.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum Process3dConfigMutation {
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
    #[dsl(key = "contributions")]
    SetContributions { json: String },
    #[dsl(key = "cursor")]
    SetCursor { value: Option<usize> },
}

//#region 🔖️OpCodec
impl protocol::OpText for Process3dConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_dsl_record::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        semio_framework_dsl_record::variants_text::print_op(self)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for Process3dConfigMutation {
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

impl Mutation<Process3dConfig> for Process3dConfigMutation {
    type Diff = Process3dConfigDiff;

    /// 🧾️ Leaf metadata for the view-state vocabulary. ⚠️ PROVISIONAL: none of the six `owner`
    /// paths below name a directory that exists on disk — this enum has no `🎚️config/<slug>` leaf
    /// triads of its own (every field lives flat in this file), so every entry is a metadata
    /// placeholder to satisfy `protocol::Mutation`, matching `🪵️sourcing`'s own config precedent.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⌨️set",
            semantic_kind: "set-engagement-input",
            display_name: "Set Engagement Input",
            emoji: "⌨️",
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
            owner: "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🎥️set-camera",
            semantic_kind: "set-camera",
            display_name: "Set Camera",
            emoji: "🎥️",
            aggregate_variant: "SetCamera",
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
            owner: "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/☀️set-sun",
            semantic_kind: "set-sun",
            display_name: "Set Sun",
            emoji: "☀️",
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
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🤝️set-contributions",
            semantic_kind: "set-contributions",
            display_name: "Set Contributions",
            emoji: "🤝️",
            aggregate_variant: "SetContributions",
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
            owner: "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⏱️set-cursor",
            semantic_kind: "set-cursor",
            display_name: "Set Replay Cursor",
            emoji: "⏱️",
            aggregate_variant: "SetCursor",
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
            Process3dConfigMutation::SetEngagementInput { .. } => &Self::DESCRIPTORS[0],
            Process3dConfigMutation::SetCamera { .. } => &Self::DESCRIPTORS[1],
            Process3dConfigMutation::SetSun { .. } => &Self::DESCRIPTORS[2],
            Process3dConfigMutation::SetContributions { .. } => &Self::DESCRIPTORS[3],
            Process3dConfigMutation::SetCursor { .. } => &Self::DESCRIPTORS[4],
        }
    }

    fn diff(&self, base: &Process3dConfig) -> protocol::MutationOutcome<Process3dConfigDiff> {
        protocol::MutationOutcome::new(match self {
            Process3dConfigMutation::SetEngagementInput { value } => Process3dConfigDiff { engagement_input: (base.engagement_input != *value).then(|| value.clone()), ..Default::default() },
            Process3dConfigMutation::SetCamera { position, target, fov } => Process3dConfigDiff {
                camera_position: (base.camera_position != *position).then_some(*position),
                camera_target: (base.camera_target != *target).then_some(*target),
                camera_fov: (base.camera_fov != *fov).then_some(*fov),
                ..Default::default()
            },
            Process3dConfigMutation::SetSun { enabled, azimuth, elevation, intensity, color } => Process3dConfigDiff {
                sun_enabled: (base.sun_enabled != *enabled).then_some(*enabled),
                sun_azimuth: (base.sun_azimuth != *azimuth).then_some(*azimuth),
                sun_elevation: (base.sun_elevation != *elevation).then_some(*elevation),
                sun_intensity: (base.sun_intensity != *intensity).then_some(*intensity),
                sun_color: (base.sun_color != *color).then(|| color.clone()),
                ..Default::default()
            },
            Process3dConfigMutation::SetContributions { json } => Process3dConfigDiff { contributions_json: (base.contributions_json != *json).then(|| json.clone()), ..Default::default() },
            Process3dConfigMutation::SetCursor { value } => Process3dConfigDiff { resolved_up_to: (base.resolved_up_to != *value).then(|| Process3dOptionalCursor { value: *value }), ..Default::default() },
        })
    }

    fn inverse(&self, base: &Process3dConfig) -> Self {
        Self {
            engagement_input: self.engagement_input.as_ref().map(|_| base.engagement_input.clone()),
            camera_position: self.camera_position.as_ref().map(|_| base.camera_position.clone()),
            camera_target: self.camera_target.as_ref().map(|_| base.camera_target.clone()),
            camera_fov: self.camera_fov.as_ref().map(|_| base.camera_fov.clone()),
            sun_enabled: self.sun_enabled.as_ref().map(|_| base.sun_enabled.clone()),
            sun_azimuth: self.sun_azimuth.as_ref().map(|_| base.sun_azimuth.clone()),
            sun_elevation: self.sun_elevation.as_ref().map(|_| base.sun_elevation.clone()),
            sun_intensity: self.sun_intensity.as_ref().map(|_| base.sun_intensity.clone()),
            sun_color: self.sun_color.as_ref().map(|_| base.sun_color.clone()),
            contributions_json: self.contributions_json.as_ref().map(|_| base.contributions_json.clone()),
            resolved_up_to: self.resolved_up_to.as_ref().map(|_| Process3dOptionalCursor { value: base.resolved_up_to.clone() }),
        }
    }
    fn between(base: &Process3dConfig, other: &Process3dConfig) -> Self {
        Self {
            engagement_input: (base.engagement_input != other.engagement_input).then(|| other.engagement_input.clone()),
            camera_position: (base.camera_position != other.camera_position).then(|| other.camera_position.clone()),
            camera_target: (base.camera_target != other.camera_target).then(|| other.camera_target.clone()),
            camera_fov: (base.camera_fov != other.camera_fov).then(|| other.camera_fov.clone()),
            sun_enabled: (base.sun_enabled != other.sun_enabled).then(|| other.sun_enabled.clone()),
            sun_azimuth: (base.sun_azimuth != other.sun_azimuth).then(|| other.sun_azimuth.clone()),
            sun_elevation: (base.sun_elevation != other.sun_elevation).then(|| other.sun_elevation.clone()),
            sun_intensity: (base.sun_intensity != other.sun_intensity).then(|| other.sun_intensity.clone()),
            sun_color: (base.sun_color != other.sun_color).then(|| other.sun_color.clone()),
            contributions_json: (base.contributions_json != other.contributions_json).then(|| other.contributions_json.clone()),
            resolved_up_to: (base.resolved_up_to != other.resolved_up_to).then(|| Process3dOptionalCursor { value: other.resolved_up_to.clone() }),
        }
    }
    fn is_empty(&self) -> bool {
        self.engagement_input.is_none() && self.camera_position.is_none() && self.camera_target.is_none() && self.camera_fov.is_none() && self.sun_enabled.is_none() && self.sun_azimuth.is_none() && self.sun_elevation.is_none() && self.sun_intensity.is_none() && self.sun_color.is_none() && self.contributions_json.is_none() && self.resolved_up_to.is_none()
    }
}

//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ [`Process3dConfig`]'s operation enum — one variant per settled interaction (mirrors the pre-B1
/// `Process3dRuntime` field writes). Every field already carries its own setter, so `backwards()`
/// returns the SAME variant re-addressed at `base`'s old value — a targeted, in-kind inverse per
/// this ticket's ban on whole-record replace, rather than a generic whole-config snapshot.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum Process3dConfigMutation {
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
    #[dsl(key = "contributions")]
    SetContributions { json: String },
    #[dsl(key = "cursor")]
    SetCursor { value: Option<usize> },
}

//#region 🔖️OpCodec
impl protocol::OpText for Process3dConfigMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_dsl_record::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        semio_framework_dsl_record::variants_text::print_op(self)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for Process3dConfigMutation {
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

impl Mutation<Process3dConfig> for Process3dConfigMutation {
    type Diff = Process3dConfigDiff;

    /// 🧾️ Leaf metadata for the view-state vocabulary. ⚠️ PROVISIONAL: none of the six `owner`
    /// paths below name a directory that exists on disk — this enum has no `🎚️config/<slug>` leaf
    /// triads of its own (every field lives flat in this file), so every entry is a metadata
    /// placeholder to satisfy `protocol::Mutation`, matching `🪵️sourcing`'s own config precedent.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⌨️set",
            semantic_kind: "set-engagement-input",
            display_name: "Set Engagement Input",
            emoji: "⌨️",
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
            owner: "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🎥️set-camera",
            semantic_kind: "set-camera",
            display_name: "Set Camera",
            emoji: "🎥️",
            aggregate_variant: "SetCamera",
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
            owner: "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/☀️set-sun",
            semantic_kind: "set-sun",
            display_name: "Set Sun",
            emoji: "☀️",
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
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🤝️set-contributions",
            semantic_kind: "set-contributions",
            display_name: "Set Contributions",
            emoji: "🤝️",
            aggregate_variant: "SetContributions",
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
            owner: "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⏱️set-cursor",
            semantic_kind: "set-cursor",
            display_name: "Set Replay Cursor",
            emoji: "⏱️",
            aggregate_variant: "SetCursor",
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
            Process3dConfigMutation::SetEngagementInput { .. } => &Self::DESCRIPTORS[0],
            Process3dConfigMutation::SetCamera { .. } => &Self::DESCRIPTORS[1],
            Process3dConfigMutation::SetSun { .. } => &Self::DESCRIPTORS[2],
            Process3dConfigMutation::SetContributions { .. } => &Self::DESCRIPTORS[3],
            Process3dConfigMutation::SetCursor { .. } => &Self::DESCRIPTORS[4],
        }
    }

    /// 📦️ Whole-config field-setter — every variant addresses the single always-present
    /// `Process3dConfig` by value, so there is no target to be missing; message-free outcome per
    /// the contract's root-scoped shrink-only allowlist.
    fn diff(&self, base: &Process3dConfig) -> protocol::MutationOutcome<Process3dConfig> {
        let mut next = base.clone();
        match self {
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
            Process3dConfigMutation::SetContributions { json } => {
                next.contributions_json = json.clone();
            }
            Process3dConfigMutation::SetCursor { value } => next.resolved_up_to = *value,
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Process3dConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        match self {
            Process3dConfigMutation::SetEngagementInput { .. } => vec![Process3dConfigMutation::SetEngagementInput { value: base.engagement_input.clone() }],
            Process3dConfigMutation::SetCamera { .. } => {
                vec![Process3dConfigMutation::SetCamera { position: base.camera_position, target: base.camera_target, fov: base.camera_fov }]
            }
            Process3dConfigMutation::SetSun { .. } => vec![Process3dConfigMutation::SetSun { enabled: base.sun_enabled, azimuth: base.sun_azimuth, elevation: base.sun_elevation, intensity: base.sun_intensity, color: base.sun_color.clone() }],
            Process3dConfigMutation::SetContributions { .. } => vec![Process3dConfigMutation::SetContributions { json: base.contributions_json.clone() }],
            Process3dConfigMutation::SetCursor { .. } => vec![Process3dConfigMutation::SetCursor { value: base.resolved_up_to }],
        }
    
    })())
}
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
mod law_tests {
    use super::*;

    /// ⚖️ The inverse diffs sum to the negative of the forward diff (L3).
    #[semio_framework_async_macros::async_test]
    async fn inverse_diffs_sum_to_the_negative_diff() {
        let base = Process3dConfig::default();
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&Process3dConfigMutation::SetEngagementInput { value: "x".into() }, &base).await;
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&Process3dConfigMutation::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 1.0], fov: 30.0 }, &base).await;
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&Process3dConfigMutation::SetSun { enabled: true, azimuth: 10.0, elevation: 20.0, intensity: 0.5, color: "#fff".into() }, &base).await;
        protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&Process3dConfigMutation::SetCursor { value: Some(2) }, &base).await;
    }
}
