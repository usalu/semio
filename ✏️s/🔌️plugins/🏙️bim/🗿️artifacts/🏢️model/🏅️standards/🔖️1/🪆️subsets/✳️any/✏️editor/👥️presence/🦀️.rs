//! 👥️ BIM presence: the shareable live state of one author, ephemeral and shared. Selection and hover travel through the framework's typed `PresencePeer.interaction`
//! broadcast; this facet carries only what is BIM specific: the storey the author works on, the plan camera and the line the author is typing into a window's entry field.

use protocol::Mutation;
use pack::snapshot::ArtifactPack;

//#region 🔖️Presence
/// 👥️ Shareable live subset of an author's view: working storey, plan camera, the line being typed into a window's entry field.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value::SnapshotProjection, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", default, by_value_projection)]
#[artifact(extension = "bim.presence")]
#[dsl(layout = "lines")]
pub struct BimPresence {
    pub engagement_input: String,
    pub storey: String,
    pub owned_worksets: Vec<String>,
    #[dsl(block)]
    pub camera: store::Viewport2d,
}

impl store::ArtifactPresenceSnapshot for BimPresence {
    fn decode_presence_native(bytes: &[u8], owner: &mut store::io::control::NativeSnapshotDecodeOwner<'_, '_>) -> semio_framework_value::RetainedWorkOutcome<Self, store::io::control::NativeSnapshotFailure> {
        store::decode_presence_json(bytes, owner)
    }
}

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

impl protocol::MutationDiff<BimPresence> for BimPresenceDiff {}

impl protocol::ColdMutationDiff<BimPresence> for BimPresenceDiff {
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

    fn apply(&self, unpublished: &mut BimPresence, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        *unpublished = BimPresence {
            engagement_input: self.engagement_input.clone().unwrap_or_else(|| base.engagement_input.clone()),
            storey: self.storey.clone().unwrap_or_else(|| base.storey.clone()),
            camera: self.camera.unwrap_or(base.camera),
            owned_worksets: self.owned_worksets.clone().unwrap_or_else(|| base.owned_worksets.clone()),
        };
        Ok(())
    }
}

impl protocol::DiffAlgebra<BimPresence> for BimPresenceDiff {
    fn is_empty(&self) -> bool {
        self.engagement_input.is_none() && self.storey.is_none() && self.camera.is_none() && self.owned_worksets.is_none()
    }
}

impl protocol::ColdDiffAlgebra<BimPresence> for BimPresenceDiff {
    fn inverse(&self, base: &BimPresence) -> Self {
        Self { engagement_input: self.engagement_input.as_ref().map(|_| base.engagement_input.clone()), storey: self.storey.as_ref().map(|_| base.storey.clone()), camera: self.camera.map(|_| base.camera), owned_worksets: self.owned_worksets.as_ref().map(|_| base.owned_worksets.clone()) }
    }
}

impl pack::snapshot::ArtifactDsl for BimPresence {
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
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for BimPresence {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_os_kernel::os_store::serialization::ArtifactStoreSnapshot for BimPresence {
    fn reference_snapshot_codec()->Option<::semio_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}


}
//#endregion 🔖️Presence

//#region 🔖️PresenceMutation
/// 👥️ The one presence mutation: set the whole shareable record.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub enum BimPresenceMutation {
    #[dsl(key = "set")]
    Snapshot {
        #[dsl(block)]
        transient: BimPresence,
    },
}

protocol::retained_root_record!(
    state: BimPresence,
    mutation: BimPresenceMutation,
    diff: BimPresenceDiff,
    fields: {
        engagement_input: String,
        storey: String,
        camera: store::Viewport2d,
        owned_worksets: Vec<String>,
    }
);

impl Mutation<BimPresence> for BimPresenceMutation {
    type Diff = BimPresenceDiff;

    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence",
        semantic_kind: "set-presence",
        display_name: "Set Presence",
        emoji: "👥️",
        aggregate_variant: "Snapshot",
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
}

impl protocol::ColdMutation<BimPresence> for BimPresenceMutation {
    type ReferenceDiff = BimPresenceDiff;
    fn diff(&self, base: &BimPresence) -> protocol::MutationOutcome<BimPresenceDiff> {
        let Self::Snapshot { transient: presence } = self;
        protocol::MutationOutcome::new(BimPresenceDiff {
            engagement_input: (base.engagement_input != presence.engagement_input).then(|| presence.engagement_input.clone()),
            storey: (base.storey != presence.storey).then(|| presence.storey.clone()),
            owned_worksets: (base.owned_worksets != presence.owned_worksets).then(|| presence.owned_worksets.clone()),
            camera: (base.camera != presence.camera).then_some(presence.camera),
        })
    }
    fn inverse(&self, base: &BimPresence) -> Result<Vec<Self>, semio_framework_value::ValueError> {
        Ok(vec![Self::Snapshot { transient: base.clone() }])
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
        Ok(store::pack_rt::encode_pack_value(&semio_framework_value::ToValue::to_value(self)))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let value = store::pack_rt::decode_pack_value(bytes).map_err(protocol::ProtocolError::from)?;
        semio_framework_value::FromValue::from_value(store::pack_rt::renormalize_whole_number_floats(value))
            .map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
    }
}

::semio_os_kernel::original_pack_value_operation_receiving!(BimPresenceMutation);
::semio_os_kernel::artifact_persisted_encoded_binary_operation!(BimPresenceMutation);
//#endregion 🔖️PresenceMutation

impl BimPresence {
    /// 👥️ The presence of an author working on `storey`, keeping the rest of the record.
    pub fn on_storey(&self, storey: &str) -> BimPresenceMutation {
        BimPresenceMutation::Snapshot { transient: Self { storey: storey.to_owned(), ..self.clone() } }
    }

    /// 👥️ The presence of an author typing `input` into a window entry field, keeping the rest of the record.
    pub fn typing(&self, input: &str) -> BimPresenceMutation {
        BimPresenceMutation::Snapshot { transient: Self { engagement_input: input.to_owned(), ..self.clone() } }
    }

    /// 👥️ The presence of an author looking through `camera`, keeping the rest of the record.
    pub fn looking_through(&self, camera: store::Viewport2d) -> BimPresenceMutation {
        BimPresenceMutation::Snapshot { transient: Self { camera, ..self.clone() } }
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
        BimPresenceMutation::Snapshot { transient: Self { owned_worksets, ..self.clone() } }
    }
}
