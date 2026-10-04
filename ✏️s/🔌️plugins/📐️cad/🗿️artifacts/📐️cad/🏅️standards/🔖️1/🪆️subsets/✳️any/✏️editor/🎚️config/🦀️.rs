//! 🧮️ CAD application preferences shared by the artifact instance. Camera, sun, and Dislocate
//! preferences belong to exact concrete window owners beside the world windows.

use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Config
/// 🎛️ Handle preferences persisted by one exact CAD world-window configuration owner.
#[derive(Clone, Copy, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct CadDislocateOptions {
    pub move_enabled: bool,
    pub rotate_enabled: bool,
}

impl Default for CadDislocateOptions {
    fn default() -> Self {
        Self { move_enabled: true, rotate_enabled: true }
    }
}

/// 🌞️ Local `dsl::DslRecord`-able mirror of `semio_framework_plugin::WorldSunConfig` (foreign,
/// out-of-scope crate — cannot gain a `dsl` derive there). `cad_sun_config_from_world`/
/// `cad_sun_config_to_world` convert at the boundary; field-for-field identical otherwise.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct CadSunConfig {
    pub enabled: bool,
    pub azimuth: f64,
    pub elevation: f64,
    pub intensity: f64,
    pub color: String,
}

impl Default for CadSunConfig {
    fn default() -> Self {
        Self { enabled: false, azimuth: 45.0, elevation: 35.0, intensity: 0.85, color: "#ffffff".into() }
    }
}

pub fn cad_sun_config_from_world(sun: &semio_framework_plugin::WorldSunConfig) -> CadSunConfig {
    CadSunConfig { enabled: sun.enabled, azimuth: sun.azimuth, elevation: sun.elevation, intensity: sun.intensity, color: sun.color.clone() }
}

pub fn cad_sun_config_to_world(sun: &CadSunConfig) -> semio_framework_plugin::WorldSunConfig {
    semio_framework_plugin::WorldSunConfig { enabled: sun.enabled, azimuth: sun.azimuth, elevation: sun.elevation, intensity: sun.intensity, color: sun.color.clone() }
}

/// 🧮️ B1/WORKFLOWS-END-TO-END-TYPED-PORTS: cad's real `ArtifactApp::Config` — see the region doc
/// comment above for the full absorption story.
/// The per-frame engagement state (action line, REPL step, live session, repeat-last) is NOT config: it lives in the
/// addressed world window's transient (`🪟️windows/🫧️transient`, design §17.4 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "cadcfg")]
#[artifact(id = "cad.config")]
#[dsl(layout = "lines")]
pub struct CadConfig {
    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): mesh object/vertex/edge/face
    /// selection AND hover are now the framework-owned `"cad"` interaction domain
    /// (`InteractionView::selection("cad")`/`.hover("cad", "pointer")`) — `selected_object_ids`,
    /// `selection_method`, `hovered_object_id`, `hovered_target`, `active_object_id` (now
    /// `DomainSelection::anchor_id`) and `component_selection` are DELETED, not migrated in place.
    /// 👁️ Node (document-tree) selection stays app-owned — not a mesh-geometry granularity.
    pub selected_node_ids: Vec<String>,
    /// 🐁️ Hovered reference-overlay id (per-pane background image) — app-owned, distinct from the
    /// framework `"cad"` domain's mesh hover; was piggy-backed onto the deleted `hovered_object_id`
    /// via a `"reference:"` string prefix, now its own field.
    pub hovered_reference_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::active_example_id`.
    pub active_example_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::selected_reference_model_definition_id`.
    pub selected_reference_model_definition_id: Option<String>,
    /// 👁️ Was `CadPlayRuntime::selected_reference_id`.
    pub selected_reference_id: Option<String>,
    /// 🧩️ Host-pushed `ProgramContributionEntry[]` JSON for `cad.computer` hot-swap installs.
    #[value(default = "default_contributions_json")]
    pub contributions_json: String,
}

//#region 🔖️ArtifactCodec
/// 📜️ Handcrafted ArtifactDsl (P6): uses this type's `__dsl_*` helpers + parse/print, not derive emission.
impl store::ArtifactDsl for CadConfig {
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
impl store::ArtifactPack for CadConfig {
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

impl Default for CadConfig {
    fn default() -> Self {
        Self {
            selected_node_ids: Vec::new(),
            hovered_reference_id: None,
            active_example_id: None,
            selected_reference_model_definition_id: None,
            selected_reference_id: None,
            contributions_json: default_contributions_json(),
        }
    }
}

store::impl_whole_record_config!(CadConfig);
//#endregion 🔖️Config

//#region 🔖️ConfigOperations
/// 🧮️ WORKFLOWS-END-TO-END-TYPED-PORTS config recipe: `CadConfig`'s
/// operation enum. Unlike `CadMutation` (many narrow document-mutating variants), this is a single
/// whole-record `Snapshot`: application commands convert their scratch state into the next
/// `CadConfig` and diff it against the pre-command config, the same shape `reset_document_effect` uses for
/// a whole-DOCUMENT replace (SEMANTIC-MUTATIONS-OVERHAUL retired `CadMutation::SetSnapshot`
/// entirely; document-level whole-content replace is no longer an in-history mutation at all) —
/// application state (selection/hover/engagement) mutates in tight clusters (e.g.
/// `worldSelect` touches 5+ fields together), so per-field variants would just be wide-argument
/// snapshots in miniature with none of a real granular diff's benefit. `backwards()` restores the
/// exact pre-command `CadConfig`, giving real, exact undo without any per-field reverse-patch
/// bookkeeping — the same justification `shooting_op::ShootingConfigOperation` documents for its own
/// `Snapshot` fallback, generalized here to the sole variant.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum)]
pub enum CadConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Box<CadConfig>,
    },
    #[dsl(key = "contributions")]
    SetContributions { json: String },
}

//#region 🔖️OpCodec
impl protocol::OpText for CadConfigMutation {
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
impl protocol::OpBinary for CadConfigMutation {
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

impl Mutation<CadConfig> for CadConfigMutation {
    type Diff = CadConfig;

    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/📄snapshot",
            semantic_kind: "snapshot",
            display_name: "Snapshot",
            emoji: "📄",
            aggregate_variant: "Snapshot",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied, protocol::MutationOutcomeClass::NoOp],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
        protocol::MutationLeafDescriptor {
            schema_version: 1,
            owner: "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧮️set-contributions",
            semantic_kind: "set-contributions",
            display_name: "Set Contributions",
            emoji: "🧮️",
            aggregate_variant: "SetContributions",
            payload_schema: "🧬️schema/🔣️.json",
            text_opcode: None,
            binary_tag: None,
            invertibility: protocol::MutationInvertibility::ExplicitMutation,
            diff_participation: protocol::MutationDiffParticipation::Detect,
            outcome_classes: &[protocol::MutationOutcomeClass::Applied, protocol::MutationOutcomeClass::NoOp],
            composition: protocol::MutationComposition::Atomic,
            required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
        },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            CadConfigMutation::Snapshot { .. } => &Self::DESCRIPTORS[0],
            CadConfigMutation::SetContributions { .. } => &Self::DESCRIPTORS[1],
        }
    }

    fn diff(&self, base: &CadConfig) -> protocol::MutationOutcome<CadConfig> {
        match self {
            CadConfigMutation::Snapshot { config } => {
                if config.as_ref() == base {
                    return protocol::MutationOutcome::new(base.clone()).warning("mutation.no-op", "Config snapshot is already up to date.");
                }
                protocol::MutationOutcome::new(config.as_ref().clone())
            }
            CadConfigMutation::SetContributions { json } => {
                if &base.contributions_json == json {
                    return protocol::MutationOutcome::new(base.clone()).warning("mutation.no-op", "Contributions are already up to date.");
                }
                let mut next = base.clone();
                next.contributions_json = json.clone();
                let _ = crate::standards::v1::subsets::any::schema::inferences::validate_cad_computer_contributions(json);
                protocol::MutationOutcome::new(next)
            }
        }
    }

    fn inverse(&self, base: &CadConfig) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![CadConfigMutation::Snapshot { config: Box::new(base.clone()) }]
    
    })())
}
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
