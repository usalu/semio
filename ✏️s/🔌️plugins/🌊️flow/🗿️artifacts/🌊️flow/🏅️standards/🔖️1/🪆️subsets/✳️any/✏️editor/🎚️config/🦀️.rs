//! 🧮️ Flow play app — view state (`FlowConfig`) and its operation enum (`FlowConfigMutation`).
//!
//! This is APP state, not document state: it lives at app level rather than under `🗿️artifacts/` because
//! nothing in it survives into the `.flow` document. It still round-trips through a real `ArtifactStore`
//! (with a real `backwards`), so selection/camera/grid edits are VCS'd exactly like document content.

use crate::artifacts::flow::schema::{FLOW_DEFAULT_GRID_FACTOR, FLOW_DEFAULT_PROXIMITY_DISTANCE};
use crate::playbook::GenerationPlayState;
use flow::{CameraJson, FLOW_LOD_MODE_AUTOMATIC};
use protocol::Mutation;
use std::collections::HashMap;

//#region 🔖️Config
/// 🧮️ `FlowPlayApp::Config` — the pure-trait `ArtifactApp::Config` for the flow app. Absorbs everything
/// that used to live in an app-struct `RefCell` (`FlowPlayRuntime`) AND the locale the flow UI read off
/// the deleted host-pushed `ViewModel` — session-only view/generate-mode state now round-trips through the
/// config `ArtifactStore` exactly like document content, with a real `backwards` per
/// [`FlowConfigMutation`] instead of never being VCS'd at all.
///
/// `automation_enabled_json`/`generation_json` hold JSON-encoded `HashMap<String, bool>`/
/// `playbook::GenerationPlayState` payloads rather than nested `#[dsl(block)]`/`#[dsl(table)]` fields:
/// none of those types derive `dsl::DslRecord`, mirroring `procedural_3d`'s identical `sun_json` escape
/// hatch for the same reason. Per-dispatch eval scratch uses a local `FlowEvalSession` in `handle` /
/// `pending_effects` / `render` (not process globals). `generation_json` stays config-tracked rather than becoming a
/// document operation (unlike the sibling `procedural_3d`/`procedural_2d` apps' `GenerationMutation`-backed
/// generations): flow's document model (`flow::FlowMutation`) is a shared kernel crate out of scope
/// for that conversion. `camera` stays a real `#[dsl(block)]` field since `flow::CameraJson` DOES
/// derive `dsl::DslRecord`.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "flowcfg")]
#[dsl(id = "flow.config")]
#[dsl(layout = "lines")]
pub struct FlowConfig {
    /// 👁️ Widget ids with their live-eval preview disabled.
    pub preview_off_node_ids: Vec<String>,
    /// 🎥️ The node-graph viewport camera.
    #[dsl(block)]
    pub camera: CameraJson,
    /// 🎚️ LOD mode id (or `flow::FLOW_LOD_MODE_AUTOMATIC`).
    pub lod_mode: String,
    /// 🖱️ Proximity-select distance.
    pub proximity_distance: f64,
    /// 🔳️ Canvas grid visibility.
    pub grid_visible: bool,
    /// 🧲️ Canvas grid snap toggle.
    pub grid_snap_enabled: bool,
    /// 🔳️ Canvas grid factor.
    pub grid_factor: f64,
    /// 📚️ JSON-encoded extra catalogue sections.
    pub catalogue_sections_json: String,
    /// 🧩️ JSON-encoded `(extension id) -> enabled` map.
    pub automation_enabled_json: String,
    /// 🧩️ Host-pushed ProgramContributionEntry[] JSON for flow.extension hot-swap installs.
    #[value(default = "default_contributions_json")]
    pub contributions_json: String,
    /// 🧬️ JSON-encoded `playbook::GenerationPlayState` (Generate-mode exploration surface).
    pub generation_json: String,
    /// 📋️ JSON checkpoint for observable duplicate-widget progress; empty when idle.
    pub duplicate_widget_progress_json: String,
    /// 🗣️ BCP-47 locale tag.
    pub locale: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for FlowConfig {
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
impl store::ArtifactPack for FlowConfig {
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

impl Default for FlowConfig {
    fn default() -> Self {
        Self {
            preview_off_node_ids: Vec::new(),
            camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            lod_mode: FLOW_LOD_MODE_AUTOMATIC.into(),
            proximity_distance: FLOW_DEFAULT_PROXIMITY_DISTANCE,
            grid_visible: true,
            grid_snap_enabled: false,
            grid_factor: FLOW_DEFAULT_GRID_FACTOR,
            catalogue_sections_json: "[]".into(),
            automation_enabled_json: String::new(),
            contributions_json: "[]".into(),
            generation_json: String::new(),
            duplicate_widget_progress_json: String::new(),
            locale: "en-US".into(),
        }
    }
}

impl FlowConfig {
    /// 🧩️ Parses `automation_enabled_json` — falls back to an empty map.
    pub fn automation_enabled(&self) -> HashMap<String, bool> {
        serde_json::from_str(&self.automation_enabled_json).unwrap_or_default()
    }

    /// 🧬️ Parses `generation_json` — falls back to `GenerationPlayState::default()`.
    pub fn generation(&self) -> GenerationPlayState {
        serde_json::from_str(&self.generation_json).unwrap_or_default()
    }
}

store::impl_whole_record_config!(FlowConfig);
//#endregion 🔖️Config

fn default_contributions_json() -> String {
    "[]".into()
}

//#region 🔖️ConfigMutations
/// 🧮️ [`FlowConfig`]'s operation enum — one variant per settled interaction, plus a generic `Snapshot`
/// every variant's `backwards()` returns: since a config-only "View" dispatch is a plain `Apply` (not an
/// `AmendLast`), each tick is its own distinct, real config edit, and "undo this tick" is exactly "restore
/// the whole-config snapshot from just before it" — the simplest correct inverse, needing no per-field
/// reverse-patch bookkeeping. `Mutation::Diff` is the WHOLE `FlowConfig` (not a granular patch type):
/// `diff()` returns "the full config after this op", and `store::impl_whole_record_config!` supplies the
/// `MutationDiff<FlowConfig>` that returns that snapshot verbatim, ignoring `base` — the same
/// "whole-record diff" shape the shooting/dag/procedural-3d config operations already use.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslOps)]
pub enum FlowConfigMutation {
    /// 🧩️ Host-pushed contributions catalogue JSON.
    #[dsl(key = "contributions")]
    SetContributions { json: String },
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: FlowConfig,
    },
    #[dsl(key = "preview-off")]
    SetPreviewOff { node_ids: Vec<String> },
    #[dsl(key = "camera")]
    SetCamera {
        #[dsl(block)]
        camera: CameraJson,
    },
    #[dsl(key = "lod-mode")]
    SetLodMode { value: String },
    #[dsl(key = "proximity-distance")]
    SetProximityDistance { value: f64 },
    #[dsl(key = "grid-visible")]
    SetGridVisible { value: bool },
    #[dsl(key = "grid-snap")]
    SetGridSnapEnabled { value: bool },
    #[dsl(key = "grid-factor")]
    SetGridFactor { value: f64 },
    #[dsl(key = "catalogue-sections")]
    SetCatalogueSections { sections_json: String },
    #[dsl(key = "extension-enabled")]
    SetAutomationEnabled { json: String },
    #[dsl(key = "generation")]
    SetGeneration { json: String },
    #[dsl(key = "duplicate-widget-progress")]
    SetDuplicateWidgetProgress { json: String },
    #[dsl(key = "cancel-duplicate-widget")]
    CancelDuplicateWidget { generation: u64 },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for FlowConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl protocol::OpBinary for FlowConfigMutation {
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

impl Mutation<FlowConfig> for FlowConfigMutation {
    type Diff = FlowConfig;

    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📡️set-contributions", semantic_kind: "contributions", display_name: "Set Contributions", emoji: "📡️", aggregate_variant: "SetContributions", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied, protocol::MutationOutcomeClass::Error], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📄snapshot", semantic_kind: "snapshot", display_name: "Snapshot", emoji: "📄", aggregate_variant: "Snapshot", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/👁️set-preview-off", semantic_kind: "preview-off", display_name: "Set Preview Off", emoji: "👁️", aggregate_variant: "SetPreviewOff", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🎥️set-camera", semantic_kind: "camera", display_name: "Set Camera", emoji: "🎥️", aggregate_variant: "SetCamera", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🎚️set-lod-mode", semantic_kind: "lod-mode", display_name: "Set Lod Mode", emoji: "🎚️", aggregate_variant: "SetLodMode", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🖱️set-proximity-distance", semantic_kind: "proximity-distance", display_name: "Set Proximity Distance", emoji: "🖱️", aggregate_variant: "SetProximityDistance", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🔳️set-grid-visible", semantic_kind: "grid-visible", display_name: "Set Grid Visible", emoji: "🔳️", aggregate_variant: "SetGridVisible", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧲️set-grid-snap-enabled", semantic_kind: "grid-snap", display_name: "Set Grid Snap Enabled", emoji: "🧲️", aggregate_variant: "SetGridSnapEnabled", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🔢️set-grid-factor", semantic_kind: "grid-factor", display_name: "Set Grid Factor", emoji: "🔢️", aggregate_variant: "SetGridFactor", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📚️set-catalogue-sections", semantic_kind: "catalogue-sections", display_name: "Set Catalogue Sections", emoji: "📚️", aggregate_variant: "SetCatalogueSections", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧩️set-automation-enabled", semantic_kind: "extension-enabled", display_name: "Set Automation Enabled", emoji: "🧩️", aggregate_variant: "SetAutomationEnabled", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️set-generation", semantic_kind: "generation", display_name: "Set Generation", emoji: "🧬️", aggregate_variant: "SetGeneration", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📋️set-duplicate-widget-progress", semantic_kind: "duplicate-widget-progress", display_name: "Set Duplicate Widget Progress", emoji: "📋️", aggregate_variant: "SetDuplicateWidgetProgress", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🛑️cancel-duplicate-widget", semantic_kind: "cancel-duplicate-widget", display_name: "Cancel Duplicate Widget", emoji: "🛑️", aggregate_variant: "CancelDuplicateWidget", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🗣️set-locale", semantic_kind: "locale", display_name: "Set Locale", emoji: "🗣️", aggregate_variant: "SetLocale", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            FlowConfigMutation::SetContributions { .. } => &Self::DESCRIPTORS[0],
            FlowConfigMutation::Snapshot { .. } => &Self::DESCRIPTORS[1],
            FlowConfigMutation::SetPreviewOff { .. } => &Self::DESCRIPTORS[2],
            FlowConfigMutation::SetCamera { .. } => &Self::DESCRIPTORS[3],
            FlowConfigMutation::SetLodMode { .. } => &Self::DESCRIPTORS[4],
            FlowConfigMutation::SetProximityDistance { .. } => &Self::DESCRIPTORS[5],
            FlowConfigMutation::SetGridVisible { .. } => &Self::DESCRIPTORS[6],
            FlowConfigMutation::SetGridSnapEnabled { .. } => &Self::DESCRIPTORS[7],
            FlowConfigMutation::SetGridFactor { .. } => &Self::DESCRIPTORS[8],
            FlowConfigMutation::SetCatalogueSections { .. } => &Self::DESCRIPTORS[9],
            FlowConfigMutation::SetAutomationEnabled { .. } => &Self::DESCRIPTORS[10],
            FlowConfigMutation::SetGeneration { .. } => &Self::DESCRIPTORS[11],
            FlowConfigMutation::SetDuplicateWidgetProgress { .. } => &Self::DESCRIPTORS[12],
            FlowConfigMutation::CancelDuplicateWidget { .. } => &Self::DESCRIPTORS[13],
            FlowConfigMutation::SetLocale { .. } => &Self::DESCRIPTORS[14],
        }
    }

    fn diff(&self, base: &FlowConfig) -> protocol::MutationOutcome<FlowConfig> {
        let mut next = base.clone();
        match self {
            FlowConfigMutation::Snapshot { config } => return protocol::MutationOutcome::new(config.clone()),
            FlowConfigMutation::SetPreviewOff { node_ids } => next.preview_off_node_ids = node_ids.clone(),
            FlowConfigMutation::SetCamera { camera } => next.camera = camera.clone(),
            FlowConfigMutation::SetLodMode { value } => next.lod_mode = value.clone(),
            FlowConfigMutation::SetProximityDistance { value } => next.proximity_distance = *value,
            FlowConfigMutation::SetGridVisible { value } => next.grid_visible = *value,
            FlowConfigMutation::SetGridSnapEnabled { value } => next.grid_snap_enabled = *value,
            FlowConfigMutation::SetGridFactor { value } => next.grid_factor = *value,
            FlowConfigMutation::SetCatalogueSections { sections_json } => next.catalogue_sections_json = sections_json.clone(),
            FlowConfigMutation::SetAutomationEnabled { json } => next.automation_enabled_json = json.clone(),
            FlowConfigMutation::SetGeneration { json } => next.generation_json = json.clone(),
            FlowConfigMutation::SetDuplicateWidgetProgress { json } => next.duplicate_widget_progress_json = json.clone(),
            FlowConfigMutation::CancelDuplicateWidget { generation } => {
                let active_generation = serde_json::from_str::<serde_json::Value>(&next.duplicate_widget_progress_json).ok().and_then(|value| value.get("generation").and_then(serde_json::Value::as_u64));
                if active_generation == Some(*generation) {
                    next.duplicate_widget_progress_json.clear();
                }
            }
            FlowConfigMutation::SetContributions { json } => {
                if let Err(reason) = flow::sync_host_flow_extension_contributions(json) {
                    return protocol::MutationOutcome::new(base.clone()).absorb_messages([protocol::MutationMessage::error(reason, reason)]);
                }
                next.contributions_json = json.clone();
            }
            FlowConfigMutation::SetLocale { value } => next.locale = value.clone(),
        }
        protocol::MutationOutcome::new(next)
    }

    fn inverse(&self, base: &FlowConfig) -> Vec<Self> {
        vec![FlowConfigMutation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigMutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn flow_config_default_matches_flow_play_runtime_defaults() {
        let config = FlowConfig::default();
        assert_eq!(config.camera, CameraJson { x: 0.0, y: 0.0, zoom: 1.0 });
        assert_eq!(config.lod_mode, FLOW_LOD_MODE_AUTOMATIC);
        assert_eq!(config.proximity_distance, FLOW_DEFAULT_PROXIMITY_DISTANCE);
        assert!(config.grid_visible);
        assert!(!config.grid_snap_enabled);
        assert_eq!(config.grid_factor, FLOW_DEFAULT_GRID_FACTOR);
        assert_eq!(config.catalogue_sections_json, "[]");
        assert_eq!(config.locale, "en-US");
        assert_eq!(config.automation_enabled(), HashMap::new());
        assert_eq!(config.generation(), GenerationPlayState::default());
    }

    /// 🎞️ A fixture exercising every field — the dsl/pack round-trip law for `FlowConfig`.
    #[semio_framework_async_macros::async_test]
    async fn flow_config_dsl_pack_round_trip() {
        let config = FlowConfig {
            preview_off_node_ids: vec!["n2".into()],
            camera: CameraJson { x: 12.5, y: -3.0, zoom: 2.25 },
            lod_mode: "micro".into(),
            proximity_distance: 96.0,
            grid_visible: false,
            grid_snap_enabled: true,
            grid_factor: 5.0,
            catalogue_sections_json: "[{\"id\":\"custom\"}]".into(),
            automation_enabled_json: "{\"auto-layout\":true}".into(),
            contributions_json: "[]".into(),
            generation_json: "{\"generations\":[]}".into(),
            duplicate_widget_progress_json: String::new(),
            locale: "de-DE".into(),
        };
        store::os_store::test_support::assert_dsl_pack_equivalence(&config);
    }

    #[semio_framework_async_macros::async_test]
    async fn flow_config_operation_text_binary_round_trips_every_variant() {
        store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::Snapshot { config: FlowConfig { locale: "de-DE".into(), ..FlowConfig::default() } });
        store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetPreviewOff { node_ids: vec!["n1".into()] });
        store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetCamera { camera: CameraJson { x: 1.0, y: 2.0, zoom: 3.0 } });
        store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetLodMode { value: "micro".into() });
        store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetProximityDistance { value: 48.0 });
        store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetGridVisible { value: true });
        store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetGridSnapEnabled { value: false });
        store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetGridFactor { value: 10.0 });
        store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetCatalogueSections { sections_json: "[]".into() });
        store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetAutomationEnabled { json: "{\"auto-layout\":true}".into() });
        store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetGeneration { json: "{\"generations\":[]}".into() });
        store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetDuplicateWidgetProgress { json: "{\"generation\":7}".into() });
        store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::CancelDuplicateWidget { generation: 7 });
        store::os_store::test_support::assert_op_line_round_trip(&FlowConfigMutation::SetLocale { value: "de-DE".into() });
    }

    #[semio_framework_async_macros::async_test]
    async fn flow_config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = FlowConfig { locale: "en-US".into(), ..FlowConfig::default() };
        let operation = FlowConfigMutation::SetPreviewOff { node_ids: vec!["n2".into()] };
        let forward = operation.diff(&base).into_parts().0;
        assert_eq!(forward.preview_off_node_ids, vec!["n2".to_string()]);
        let backwards = operation.inverse(&base);
        assert_eq!(backwards, vec![FlowConfigMutation::Snapshot { config: base.clone() }]);
        let restored = backwards[0].diff(&forward).into_parts().0;
        assert_eq!(restored, base);
    }
}
//#endregion 🧪️Tests
