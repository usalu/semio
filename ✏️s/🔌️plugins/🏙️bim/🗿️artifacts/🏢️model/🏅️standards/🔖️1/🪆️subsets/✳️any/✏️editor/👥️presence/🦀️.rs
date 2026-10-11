//! 👥️ BIM presence: the shareable live state of one author, ephemeral and shared. Selection and hover travel through the framework's typed `PresencePeer.interaction`
//! broadcast; this facet carries only what is BIM specific: the storey the author works on, the plan camera and the line the author is typing into a window's entry field.

use protocol::Mutation;
use store::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of an author's view: working storey, plan camera, the line being typed into a window's entry field.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_os_kernel::DslArtifact)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "bim.presence")]
#[dsl(layout = "lines")]
pub struct BimPresence {
    pub engagement_input: String,
    pub storey: String,
    pub owned_worksets: Vec<String>,
    #[dsl(block)]
    pub camera: store::Viewport2d,
}

impl store::ArtifactPresenceSnapshot for BimPresence {}

impl Default for BimPresence {
    fn default() -> Self {
        Self { engagement_input: String::new(), storey: String::new(), owned_worksets: Vec::new(), camera: store::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 } }
    }
}

/// 🔺️ Sparse delta of the shareable presence: only the fields a mutation actually changes.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct BimPresenceDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub engagement_input: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub storey: Option<String>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera: Option<store::Viewport2d>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub owned_worksets: Option<Vec<String>>,
}

impl protocol::MutationDiff<BimPresence> for BimPresenceDiff {
    fn apply(&self, base: &BimPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<BimPresence> {
        Ok(BimPresence {
            engagement_input: self.engagement_input.clone().unwrap_or_else(|| base.engagement_input.clone()),
            storey: self.storey.clone().unwrap_or_else(|| base.storey.clone()),
            camera: self.camera.unwrap_or(base.camera),
            owned_worksets: self.owned_worksets.clone().unwrap_or_else(|| base.owned_worksets.clone()),
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.owned_worksets.is_some() { self.owned_worksets = other.owned_worksets.clone(); }
        if other.engagement_input.is_some() {
            self.engagement_input = other.engagement_input;
        }
        if other.storey.is_some() {
            self.storey = other.storey;
        }
        if other.camera.is_some() {
            self.camera = other.camera;
        }
    }
}

impl protocol::DiffAlgebra<BimPresence> for BimPresenceDiff {
    fn inverse(&self, base: &BimPresence) -> Self {
        Self { engagement_input: self.engagement_input.as_ref().map(|_| base.engagement_input.clone()), storey: self.storey.as_ref().map(|_| base.storey.clone()), camera: self.camera.map(|_| base.camera), owned_worksets: self.owned_worksets.as_ref().map(|_| base.owned_worksets.clone()) }
    }
    fn is_empty(&self) -> bool {
        self.engagement_input.is_none() && self.storey.is_none() && self.camera.is_none() && self.owned_worksets.is_none()
    }
}

impl semio_framework_value::retirement::RetireOwned for BimPresence {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        let store::Viewport2d { x, y, zoom } = self.camera;
        semio_framework_value::retirement::sequence(vec![
            semio_framework_value::retirement::RetireOwned::retirement(self.engagement_input),
            semio_framework_value::retirement::RetireOwned::retirement(self.storey),
            semio_framework_value::retirement::RetireOwned::retirement(self.owned_worksets),
            semio_framework_value::retirement::leaf(x),
            semio_framework_value::retirement::leaf(y),
            semio_framework_value::retirement::leaf(zoom),
        ])
    }
}

impl store::ArtifactDsl for BimPresence {
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

impl ArtifactPack for BimPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
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
/// 👥️ The one presence mutation: set the whole shareable record.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum)]
#[value(rename_all = "camelCase")]
pub enum BimPresenceMutation {
    #[dsl(key = "set")]
    Set {
        engagement_input: String,
        storey: String,
        owned_worksets: Vec<String>,
        #[dsl(block)]
        camera: store::Viewport2d,
    },
}

impl semio_framework_value::retirement::RetireOwned for BimPresenceMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        match self {
            Self::Set { engagement_input, storey, owned_worksets, camera: store::Viewport2d { x, y, zoom } } => semio_framework_value::retirement::sequence(vec![
                semio_framework_value::retirement::leaf(0u8),
                semio_framework_value::retirement::RetireOwned::retirement(engagement_input),
                semio_framework_value::retirement::RetireOwned::retirement(storey),
                semio_framework_value::retirement::RetireOwned::retirement(owned_worksets),
                semio_framework_value::retirement::leaf(x),
                semio_framework_value::retirement::leaf(y),
                semio_framework_value::retirement::leaf(zoom),
            ]),
        }
    }
}

impl Mutation<BimPresence> for BimPresenceMutation {
    type Diff = BimPresenceDiff;

    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence",
        semantic_kind: "set-presence",
        display_name: "Set Presence",
        emoji: "👥️",
        aggregate_variant: "Set",
        payload_schema: "🧬️schema/🔣️.json",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];

    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }

    fn diff(&self, base: &BimPresence) -> protocol::MutationOutcome<BimPresenceDiff> {
        match self {
            Self::Set { engagement_input, storey, owned_worksets, camera } => protocol::MutationOutcome::new(BimPresenceDiff {
                engagement_input: (&base.engagement_input != engagement_input).then(|| engagement_input.clone()),
                storey: (&base.storey != storey).then(|| storey.clone()),
                owned_worksets: (&base.owned_worksets != owned_worksets).then(|| owned_worksets.clone()),
                camera: (&base.camera != camera).then_some(*camera),
            }),
        }
    }

    fn inverse(&self, base: &BimPresence) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![Self::Set { engagement_input: base.engagement_input.clone(), storey: base.storey.clone(), owned_worksets: base.owned_worksets.clone(), camera: base.camera }])
    }
}

impl protocol::OpText for BimPresenceMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_dsl_record::variants_text::parse_op(line)
    }
    fn print_op(&self) -> String {
        semio_framework_dsl_record::variants_text::print_op(self)
    }
}

impl protocol::OpBinary for BimPresenceMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️PresenceMutation

impl BimPresence {
    /// 👥️ The presence of an author working on `storey`, keeping the rest of the record.
    pub fn on_storey(&self, storey: &str) -> BimPresenceMutation {
        BimPresenceMutation::Set { engagement_input: self.engagement_input.clone(), storey: storey.to_string(), owned_worksets: self.owned_worksets.clone(), camera: self.camera }
    }

    /// 👥️ The presence of an author typing `input` into a window entry field, keeping the rest of the record.
    pub fn typing(&self, input: &str) -> BimPresenceMutation {
        BimPresenceMutation::Set { engagement_input: input.to_string(), storey: self.storey.clone(), owned_worksets: self.owned_worksets.clone(), camera: self.camera }
    }

    /// 👥️ The presence of an author looking through `camera`, keeping the rest of the record.
    pub fn looking_through(&self, camera: store::Viewport2d) -> BimPresenceMutation {
        BimPresenceMutation::Set { engagement_input: self.engagement_input.clone(), storey: self.storey.clone(), owned_worksets: self.owned_worksets.clone(), camera }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

impl BimPresence {
    /// 🔐️ Ephemeral shared workset claims, independent of every authored model snapshot.
    pub fn claim_workset(&self, id: &str, claim: bool) -> BimPresenceMutation {
        let mut owned_worksets = self.owned_worksets.clone();
        owned_worksets.retain(|workset| workset != id);
        if claim { owned_worksets.push(id.to_owned()); owned_worksets.sort(); }
        BimPresenceMutation::Set { engagement_input: self.engagement_input.clone(), storey: self.storey.clone(), owned_worksets, camera: self.camera }
    }
}
