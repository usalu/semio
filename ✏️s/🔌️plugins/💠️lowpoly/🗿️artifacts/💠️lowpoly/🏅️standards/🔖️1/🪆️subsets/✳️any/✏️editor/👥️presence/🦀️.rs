//! 👥️ Lowpoly presence — shareable live ephemeral state + mutations.

use protocol::Mutation;
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of lowpoly camera and paint state.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_os_kernel::DslArtifact, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "lowpoly.presence")]
#[dsl(layout = "lines")]
pub struct LowpolyPresence {
    pub world_camera_position: [f64; 3],
    pub world_camera_target: [f64; 3],
    pub world_camera_fov: f64,
    pub paint_utility: String,
}

impl Default for LowpolyPresence {
    fn default() -> Self {
        Self { world_camera_position: [2.5, 2.0, 2.5], world_camera_target: [0.0, 0.0, 0.0], world_camera_fov: 50.0, paint_utility: "brush".into() }
    }
}

/// 🔺️ Sparse field delta over [`LowpolyPresence`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct LowpolyPresenceDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub world_camera_position: Option<[f64; 3]>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub world_camera_target: Option<[f64; 3]>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub world_camera_fov: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub paint_utility: Option<String>,
}

impl protocol::MutationDiff<LowpolyPresence> for LowpolyPresenceDiff {
    fn apply(&self, base: &LowpolyPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<LowpolyPresence> {
        let mut next = base.clone();
        if let Some(value) = &self.world_camera_position {
            next.world_camera_position = value.clone();
        }
        if let Some(value) = &self.world_camera_target {
            next.world_camera_target = value.clone();
        }
        if let Some(value) = &self.world_camera_fov {
            next.world_camera_fov = value.clone();
        }
        if let Some(value) = &self.paint_utility {
            next.paint_utility = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.world_camera_position.is_some() {
            self.world_camera_position = other.world_camera_position;
        }
        if other.world_camera_target.is_some() {
            self.world_camera_target = other.world_camera_target;
        }
        if other.world_camera_fov.is_some() {
            self.world_camera_fov = other.world_camera_fov;
        }
        if other.paint_utility.is_some() {
            self.paint_utility = other.paint_utility;
        }
    }
}

impl protocol::DiffAlgebra<LowpolyPresence> for LowpolyPresenceDiff {
    fn inverse(&self, base: &LowpolyPresence) -> Self {
        Self {
            world_camera_position: self.world_camera_position.as_ref().map(|_| base.world_camera_position.clone()),
            world_camera_target: self.world_camera_target.as_ref().map(|_| base.world_camera_target.clone()),
            world_camera_fov: self.world_camera_fov.as_ref().map(|_| base.world_camera_fov.clone()),
            paint_utility: self.paint_utility.as_ref().map(|_| base.paint_utility.clone()),
        }
    }
    fn between(base: &LowpolyPresence, other: &LowpolyPresence) -> Self {
        Self {
            world_camera_position: (base.world_camera_position != other.world_camera_position).then(|| other.world_camera_position.clone()),
            world_camera_target: (base.world_camera_target != other.world_camera_target).then(|| other.world_camera_target.clone()),
            world_camera_fov: (base.world_camera_fov != other.world_camera_fov).then(|| other.world_camera_fov.clone()),
            paint_utility: (base.paint_utility != other.paint_utility).then(|| other.paint_utility.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.world_camera_position.is_none() && self.world_camera_target.is_none() && self.world_camera_fov.is_none() && self.paint_utility.is_none()
    }
}

impl store::ArtifactDsl for LowpolyPresence {
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

impl ArtifactPack for LowpolyPresence {
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
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub enum LowpolyPresenceMutation {
    #[dsl(key = "world-camera")]
    SetWorldCamera {
        #[dsl(coord)]
        position: [f64; 3],
        #[dsl(coord)]
        target: [f64; 3],
        fov: f64,
    },
    #[dsl(key = "paint-utility")]
    SetPaintUtility {
        value: String,
    },
}

impl Mutation<LowpolyPresence> for LowpolyPresenceMutation {
    type Diff = LowpolyPresenceDiff;

    /// 🧷️ Provisional per-variant leaf metadata for this hand-written (non-derived) aggregate — one
    /// entry for the sole `Snapshot` variant, mirroring `generation2d`'s identical precedent for its
    /// own hand-written presence aggregate.
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/set-world-camera",
        semantic_kind: "set-world-camera",
        display_name: "Set World Camera",
        emoji: "👥️",
        aggregate_variant: "SetWorldCamera",
        payload_schema: "🧬️schema/🔣️.json",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }]
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/set-paint-utility",
        semantic_kind: "set-paint-utility",
        display_name: "Set Paint Utility",
        emoji: "👥️",
        aggregate_variant: "SetPaintUtility",
        payload_schema: "🧬️schema/🔣️.json",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }]

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        match self {
            Self::SetWorldCamera { .. } => &Self::DESCRIPTORS[0],
            Self::SetPaintUtility { .. } => &Self::DESCRIPTORS[1],
        }
    }

    fn diff(&self, base: &LowpolyPresence) -> protocol::MutationOutcome<LowpolyPresenceDiff> {
        protocol::MutationOutcome::new(match self {
            Self::SetWorldCamera { position, target, fov } => LowpolyPresenceDiff { world_camera_position: (base.world_camera_position != *position).then_some(*position), world_camera_target: (base.world_camera_target != *target).then_some(*target), world_camera_fov: (base.world_camera_fov != *fov).then_some(*fov), ..Default::default() },
            Self::SetPaintUtility { value } => LowpolyPresenceDiff { paint_utility: (base.paint_utility != *value).then(|| value.clone()), ..Default::default() },
        })
    }

    fn inverse(&self, base: &LowpolyPresence) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![match self {
            Self::SetWorldCamera { .. } => Self::SetWorldCamera { position: base.world_camera_position, target: base.world_camera_target, fov: base.world_camera_fov },
            Self::SetPaintUtility { .. } => Self::SetPaintUtility { value: base.paint_utility.clone() },
        }])
    }
}

impl protocol::OpText for LowpolyPresenceMutation {
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

impl protocol::OpBinary for LowpolyPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation
