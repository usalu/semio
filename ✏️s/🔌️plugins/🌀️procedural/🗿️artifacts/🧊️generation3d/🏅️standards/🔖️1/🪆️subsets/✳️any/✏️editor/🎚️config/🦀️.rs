//! 🧮️ Generation3d play app — view state (`Generation3dConfig`) and its operation enum
//! (`Generation3dConfigMutation`).
//!
//! This is APP state, not document state: selection, cameras, sun/LOD/show-mode display options, and
//! generation selection lives here rather than under `🗿️artifacts/`, since it does not survive
//! into the `.generation3d` document.

use protocol::Mutation;
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
    /// 🧮️ The edit-mode 3D preview's persisted flow-graph evaluation output (`FlowEvalSession::eval_json`) —
    /// `flowEvalTick` writes it every tick since the session itself is reconstructed fresh per dispatch
    /// (`ArtifactEditor::handle`/`render` take no `&self`), so this config field is the ONLY place the
    /// evaluated geometry survives between dispatches. See `edit::windows::preview::render`.
    pub preview_eval_text: Option<String>,
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

impl Default for Generation3dConfig {
    fn default() -> Self {
        Self {
            lod_mode: String::new(),
            show_mode: default_show_mode(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            preview_camera: Generation3dPreviewCamera::default(),
            sun_json: default_sun_json(),
            selected_generation_id: None,
            preview_eval_text: None,
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

//#region 🔖️ConfigOperations
/// 🧮️ [`Generation3dConfig`]'s operation enum — one variant per settled interaction, plus a generic
/// `Snapshot` every variant's `backwards()` returns.
// 🧯️ `Snapshot` genuinely needs to carry the whole config by value (it IS the inverse of every other
// variant); boxing it would only relocate the allocation for an enum that is never stored in bulk
// (one value per dispatch, immediately consumed), so the size lint is suppressed rather than chased.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslOps)]
pub enum Generation3dConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Generation3dConfig,
    },
    #[dsl(key = "lod-mode")]
    SetLodMode { value: String },
    #[dsl(key = "show-mode")]
    SetShowMode { value: String },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: CameraJson,
    },
    #[dsl(key = "preview-camera")]
    SetPreviewCamera {
        #[dsl(block)]
        camera: Generation3dPreviewCamera,
    },
    #[dsl(key = "sun")]
    SetSun { json: String },
    #[dsl(key = "selected-generation")]
    SetSelectedGeneration { selected_generation_id: Option<String> },
    #[dsl(key = "preview-eval")]
    SetPreviewEval { eval_text: Option<String> },
}

//#region 🔖️OpCodec
impl protocol::OpText for Generation3dConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
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
impl protocol::OpBinary for Generation3dConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
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
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = spec_fn();
        let body = &bytes[reader.position()..];
        let (record, _report) = store::pack_rt::decode_record_body(body, &spec, &store::PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        <Self as dsl::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: reader.position() as u64, detail: error.to_string() })
    }
}

//#endregion 🔖️OpCodec

impl Mutation<Generation3dConfig> for Generation3dConfigMutation {
    /// 🧷️ Provisional per-variant leaf metadata for this hand-written (non-derived) aggregate —
    /// `diff`/`inverse` dispatch here is a plain `match`, not the derive's per-leaf `MutationKind`
    /// shape. One entry per variant, in declaration order. ⚠️ PROVISIONAL: no variant below has an
    /// authored leaf directory on disk yet, so every `owner` names a path that does not exist —
    /// the same precedent puzzle3d's own config/presence aggregates set.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-snapshot",
            semantic_kind: "set-snapshot",
            display_name: "Set Snapshot",
            emoji: "⚙️",
            aggregate_variant: "Snapshot",
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
            owner: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-lod-mode",
            semantic_kind: "set-lod-mode",
            display_name: "Set Lod Mode",
            emoji: "⚙️",
            aggregate_variant: "SetLodMode",
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
            owner: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-show-mode",
            semantic_kind: "set-show-mode",
            display_name: "Set Show Mode",
            emoji: "⚙️",
            aggregate_variant: "SetShowMode",
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
            owner: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-camera",
            semantic_kind: "set-camera",
            display_name: "Set Camera",
            emoji: "⚙️",
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
            owner: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-preview-camera",
            semantic_kind: "set-preview-camera",
            display_name: "Set Preview Camera",
            emoji: "⚙️",
            aggregate_variant: "SetPreviewCamera",
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
            owner: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-sun",
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
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-selected-generation",
            semantic_kind: "set-selected-generation",
            display_name: "Set Selected Generation",
            emoji: "⚙️",
            aggregate_variant: "SetSelectedGeneration",
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
            owner: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/⚙️set-preview-eval",
            semantic_kind: "set-preview-eval",
            display_name: "Set Preview Eval",
            emoji: "⚙️",
            aggregate_variant: "SetPreviewEval",
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
            Generation3dConfigMutation::Snapshot { .. } => &Self::DESCRIPTORS[0],
            Generation3dConfigMutation::SetLodMode { .. } => &Self::DESCRIPTORS[1],
            Generation3dConfigMutation::SetShowMode { .. } => &Self::DESCRIPTORS[2],
            Generation3dConfigMutation::SetCamera { .. } => &Self::DESCRIPTORS[3],
            Generation3dConfigMutation::SetPreviewCamera { .. } => &Self::DESCRIPTORS[4],
            Generation3dConfigMutation::SetSun { .. } => &Self::DESCRIPTORS[5],
            Generation3dConfigMutation::SetSelectedGeneration { .. } => &Self::DESCRIPTORS[6],
            Generation3dConfigMutation::SetPreviewEval { .. } => &Self::DESCRIPTORS[7],
        }
    }

    type Diff = Generation3dConfig;

    fn diff(&self, base: &Generation3dConfig) -> protocol::MutationOutcome<Generation3dConfig> {
        let mut next = base.clone();
        match self {
            Generation3dConfigMutation::Snapshot { config } => return protocol::MutationOutcome::new(config.clone()),
            Generation3dConfigMutation::SetLodMode { value } => next.lod_mode = value.clone(),
            Generation3dConfigMutation::SetShowMode { value } => next.show_mode = value.clone(),
            Generation3dConfigMutation::SetCamera { camera } => next.camera = camera.clone(),
            Generation3dConfigMutation::SetPreviewCamera { camera } => next.preview_camera = camera.clone(),
            Generation3dConfigMutation::SetSun { json } => next.sun_json = json.clone(),
            Generation3dConfigMutation::SetSelectedGeneration { selected_generation_id } => next.selected_generation_id = selected_generation_id.clone(),
            Generation3dConfigMutation::SetPreviewEval { eval_text } => next.preview_eval_text = eval_text.clone(),
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &Generation3dConfig) -> Vec<Self> {
        vec![Generation3dConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
