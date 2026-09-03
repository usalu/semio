//! 👥️ Generation2dPresence — shareable live ephemeral state + mutations.
//!
//! Shareable live subset of the 2d procedural surface: graph camera, show-mode, generation pick.
//! Selection/hover broadcast automatically via the framework's typed `PresenceInteraction` (ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM) — see `create_generation2d_app`'s
//! `.interaction(...)` declaration.

use flow::CameraJson;
use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of procedural 2d view state (camera, show-mode, generation).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "generation2d.presence")]
#[dsl(layout = "lines")]
pub struct Generation2dPresence {
    /// 🗺️ The node-graph camera.
    #[dsl(block)]
    pub camera: CameraJson,
    /// 👁️ Display mode (`"preview"`/`"generate"`/`"wire"`).
    pub show_mode: String,
    /// 👁️ Active generation selection.
    pub selected_generation_id: Option<String>,
}

impl Default for Generation2dPresence {
    fn default() -> Self {
        Self { camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 }, show_mode: "preview".into(), selected_generation_id: None }
    }
}

impl protocol::MutationDiff<Generation2dPresence> for Generation2dPresence {
    fn apply(&self, _base: &Generation2dPresence) -> protocol::MutationApplyResult<Generation2dPresence> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl store::ArtifactDsl for Generation2dPresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for Generation2dPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
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
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslOps)]
#[value(rename_all = "camelCase")]
pub enum Generation2dPresenceMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        presence: Generation2dPresence,
    },
}

impl Mutation<Generation2dPresence> for Generation2dPresenceMutation {
    /// 🧷️ Provisional per-variant leaf metadata for this hand-written (non-derived) aggregate —
    /// `diff`/`inverse` dispatch here is a plain `match`, not the derive's per-leaf `MutationKind`
    /// shape. One entry per variant, in declaration order. ⚠️ PROVISIONAL: no variant below has an
    /// authored leaf directory on disk yet, so every `owner` names a path that does not exist —
    /// the same precedent puzzle3d's own config/presence aggregates set.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[
        protocol::MutationLeafDescriptor { schema_version: 1, owner: "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/👥️set-snapshot", semantic_kind: "set-snapshot", display_name: "Set Snapshot", emoji: "👥️", aggregate_variant: "Snapshot", payload_schema: "🔣️.schema.json", text_opcode: None, binary_tag: None, invertibility: protocol::MutationInvertibility::ExplicitMutation, diff_participation: protocol::MutationDiffParticipation::Detect, outcome_classes: &[protocol::MutationOutcomeClass::Applied], composition: protocol::MutationComposition::Atomic, required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema] },
    ];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Generation2dPresenceMutation::Snapshot { .. } => &Self::DESCRIPTORS[0],
        }
    }

    type Diff = Generation2dPresence;

    fn diff(&self, _base: &Generation2dPresence) -> protocol::MutationOutcome<Generation2dPresence> {
        match self {
            Self::Snapshot { presence } => protocol::MutationOutcome::new(presence.clone()),
        }
    }

    fn inverse(&self, base: &Generation2dPresence) -> Vec<Self> {
        vec![Self::Snapshot { presence: base.clone() }]
    }
}

impl protocol::OpText for Generation2dPresenceMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                let record = dsl::parse(body, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        let body = dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl protocol::OpBinary for Generation2dPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation
