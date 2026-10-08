//! 👥️ CAD presence — shareable live ephemeral state + mutations.

use protocol::Mutation;
use semio_framework_value_derive::{FromValue, ToValue};
use store::ArtifactPack;

#[path = "♻️retirement/🦀️.rs"]
pub mod retirement;

//#region 🔖️Presence
/// 👥️ Shareable live CAD view state — camera and engagement step. Peer mesh
/// selection/hover now broadcasts via the framework's typed `PresenceInteraction`, not here.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "cad.presence")]
#[dsl(layout = "lines")]
pub struct CadPresence {
    // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): mesh object/vertex/edge/face
    // selection AND hover broadcast automatically now via the framework's typed
    // `PresencePeer.interaction: Option<PresenceInteraction>` for the `"cad"` domain — every field
    // that used to mirror `CadConfig`'s selection/hover here is DELETED, not migrated in place.
    pub camera_position: [f64; 3],
    pub camera_target: [f64; 3],
    pub camera_zoom: f64,
    pub camera_fov: f64,
    pub engagement_step: String,
    pub engagement_pane: Option<String>,
}

impl Default for CadPresence {
    fn default() -> Self {
        Self { camera_position: [12.0, -12.0, 8.0], camera_target: [0.0, 0.0, 0.0], camera_zoom: 1.0, camera_fov: 50.0, engagement_step: "Idle".into(), engagement_pane: None }
    }
}

/// 🔺️ Sparse delta of the shareable presence: only the fields a mutation actually changes.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct CadPresenceDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera_position: Option<[f64; 3]>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera_target: Option<[f64; 3]>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera_zoom: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera_fov: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub engagement_step: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub engagement_pane: Option<crate::editor::cad::config::CadTextSet>,
}

impl protocol::MutationDiff<CadPresence> for CadPresenceDiff {
    fn apply(&self, base: &CadPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<CadPresence> {
        Ok(CadPresence {
            camera_position: self.camera_position.unwrap_or(base.camera_position),
            camera_target: self.camera_target.unwrap_or(base.camera_target),
            camera_zoom: self.camera_zoom.unwrap_or(base.camera_zoom),
            camera_fov: self.camera_fov.unwrap_or(base.camera_fov),
            engagement_step: self.engagement_step.clone().unwrap_or_else(|| base.engagement_step.clone()),
            engagement_pane: self.engagement_pane.as_ref().map_or_else(|| base.engagement_pane.clone(), |set| set.value.clone()),
        })
    }
    fn absorb(&mut self, other: Self) {
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(camera_position);
        take!(camera_target);
        take!(camera_zoom);
        take!(camera_fov);
        take!(engagement_step);
        take!(engagement_pane);
    }
}

impl protocol::DiffAlgebra<CadPresence> for CadPresenceDiff {
    fn inverse(&self, base: &CadPresence) -> Self {
        Self {
            camera_position: self.camera_position.map(|_| base.camera_position),
            camera_target: self.camera_target.map(|_| base.camera_target),
            camera_zoom: self.camera_zoom.map(|_| base.camera_zoom),
            camera_fov: self.camera_fov.map(|_| base.camera_fov),
            engagement_step: self.engagement_step.as_ref().map(|_| base.engagement_step.clone()),
            engagement_pane: self.engagement_pane.as_ref().map(|_| crate::editor::cad::config::CadTextSet { value: base.engagement_pane.clone() }),
        }
    }
    fn between(base: &CadPresence, other: &CadPresence) -> Self {
        Self {
            camera_position: (base.camera_position != other.camera_position).then_some(other.camera_position),
            camera_target: (base.camera_target != other.camera_target).then_some(other.camera_target),
            camera_zoom: (base.camera_zoom != other.camera_zoom).then_some(other.camera_zoom),
            camera_fov: (base.camera_fov != other.camera_fov).then_some(other.camera_fov),
            engagement_step: (base.engagement_step != other.engagement_step).then(|| other.engagement_step.clone()),
            engagement_pane: (base.engagement_pane != other.engagement_pane).then(|| crate::editor::cad::config::CadTextSet { value: other.engagement_pane.clone() }),
        }
    }
    fn is_empty(&self) -> bool {
        *self == Self::default()
    }
}

impl store::ArtifactDsl for CadPresence {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        if body.trim().is_empty() {
            return Ok(Self::default());
        }
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for CadPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
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
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslEnum)]
#[value(rename_all = "camelCase")]
pub enum CadPresenceMutation {
    #[dsl(key = "set")]
    Set {
        #[dsl(block)]
        presence: CadPresence,
    },
}

impl Mutation<CadPresence> for CadPresenceMutation {
    type Diff = CadPresenceDiff;

    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence",
        semantic_kind: "set-presence",
        display_name: "Set Presence",
        emoji: "👥️",
        aggregate_variant: "Set",
        payload_schema: "🧬️schema/🔣️.json",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied, protocol::MutationOutcomeClass::NoOp],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Self::Set { .. } => &Self::DESCRIPTORS[0],
        }
    }

    fn diff(&self, base: &CadPresence) -> protocol::MutationOutcome<CadPresenceDiff> {
        let Self::Set { presence } = self;
        let diff = CadPresenceDiff {
            camera_position: (base.camera_position != presence.camera_position).then_some(presence.camera_position),
            camera_target: (base.camera_target != presence.camera_target).then_some(presence.camera_target),
            camera_zoom: (base.camera_zoom != presence.camera_zoom).then_some(presence.camera_zoom),
            camera_fov: (base.camera_fov != presence.camera_fov).then_some(presence.camera_fov),
            engagement_step: (base.engagement_step != presence.engagement_step).then(|| presence.engagement_step.clone()),
            engagement_pane: (base.engagement_pane != presence.engagement_pane).then(|| crate::editor::cad::config::CadTextSet { value: presence.engagement_pane.clone() }),
        };
        if diff == CadPresenceDiff::default() {
            return protocol::MutationOutcome::new(diff).warning("mutation.no-op", "Presence is already up to date.");
        }
        protocol::MutationOutcome::new(diff)
    }

    fn inverse(&self, base: &CadPresence) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![Self::Set { presence: base.clone() }])
    }
}

impl protocol::OpText for CadPresenceMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                let record = semio_framework_dsl_record::parse(body, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        let body = semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl protocol::OpBinary for CadPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
