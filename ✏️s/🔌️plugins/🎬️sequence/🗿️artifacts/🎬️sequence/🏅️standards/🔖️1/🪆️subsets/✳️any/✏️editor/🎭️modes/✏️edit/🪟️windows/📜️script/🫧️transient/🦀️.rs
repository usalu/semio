//! 🫧️ Ephemeral run result bound to one exact Sequence script window.

use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct SequenceScriptWindowTransient {
    pub last_run_json: String,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum SequenceScriptWindowTransientMutation {
    Snapshot { transient: SequenceScriptWindowTransient },
}

impl protocol::Mutation<SequenceScriptWindowTransient> for SequenceScriptWindowTransientMutation {
    type Diff = SequenceScriptWindowTransient;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📜️script/🫧️transient",
        semantic_kind: "set-window-transient",
        display_name: "Set Sequence Script Window Transient",
        emoji: "🫧️",
        aggregate_variant: "Snapshot",
        payload_schema: "sequence.scriptwindowtransient",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, _base: &SequenceScriptWindowTransient) -> protocol::MutationOutcome<Self::Diff> {
        match self { Self::Snapshot { transient } => protocol::MutationOutcome::new(transient.clone()) }
    }
    fn inverse(&self, base: &SequenceScriptWindowTransient) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self::Snapshot { transient: base.clone() }]
    
    })())
}
}

impl protocol::MutationDiff<SequenceScriptWindowTransient> for SequenceScriptWindowTransient {
    fn apply(&self, _base: &SequenceScriptWindowTransient) -> protocol::MutationApplyResult<SequenceScriptWindowTransient> { Ok(self.clone()) }
    fn absorb(&mut self, other: Self) { *self = other; }
}

impl store::ArtifactDsl for SequenceScriptWindowTransient {
    const EXTENSION: &'static str = "sequencescriptwindowtransient";
    fn envelope_id() -> &'static str { "s.sequence.sequence.scriptwindowtransient" }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let json = semio_framework_pack_json::parse(body, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error.into_value_error(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&json)).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(self)));
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Sequence transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SequenceScriptWindowTransient {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&semio_framework_value::ToValue::to_value(self))).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::from(error.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) { return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "Sequence transient pack envelope mismatch"))); }
        let json = semio_framework_pack_json::parse_bytes(&body, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| store::PackError::from(error.into_value_error()))?;
        semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&json)).map_err(|error| store::PackError::from(error))
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> { None }
}

impl protocol::OpText for SequenceScriptWindowTransientMutation {
    fn print_op(&self) -> String { semio_framework_pack_json::to_json_string(self) }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
impl protocol::OpBinary for SequenceScriptWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(semio_framework_value::ValueError::from(error))))?;
        semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| protocol::ProtocolError::Pack(store::PackError::from(error)))
    }
}

semio_framework_value::artifact_retire_struct!(SequenceScriptWindowTransient { last_run_json });
impl semio_framework_value::retirement::RetireOwned for SequenceScriptWindowTransientMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        match self {
            Self::Snapshot { transient } => semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::leaf(0u8), semio_framework_value::retirement::RetireOwned::retirement(transient)]),
        }
    }
}

fn preflight(mutation: &SequenceScriptWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let SequenceScriptWindowTransientMutation::Snapshot { transient } = mutation;
    let footprint = store::ArtifactStoreOneItemFootprint::for_ephemeral_item(transient.last_run_json.len());
    footprint.is_admissible().then_some(footprint).ok_or_else(|| "Sequence script transient exceeds its retained publication envelope".into())
}

fn transfer(mutation: SequenceScriptWindowTransientMutation) -> SequenceScriptWindowTransient {
    match mutation { SequenceScriptWindowTransientMutation::Snapshot { transient } => transient }
}

pub struct SequenceScriptWindowTransientOwner;

impl semio_framework_plugin::WindowTransientOwner for SequenceScriptWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = super::SEQUENCE_PLAY_WINDOW_SCRIPT;
    type State = SequenceScriptWindowTransient;
    type Mutation = SequenceScriptWindowTransientMutation;
    fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        let state = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
        let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(preflight, transfer, state.clone(), mutation.clone()));
        semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
    }
}

pub fn current(view: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>) -> SequenceScriptWindowTransient {
    view.window::<SequenceScriptWindowTransientOwner>().cloned().unwrap_or_default()
}

pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowTransientSnapshot>) -> SequenceScriptWindowTransient {
    snapshot.and_then(|snapshot| snapshot.get::<SequenceScriptWindowTransientOwner>()).cloned().unwrap_or_default()
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, transient: SequenceScriptWindowTransient) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("sequence-script-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("sequence-window-stale"))?;
    if kind != super::SEQUENCE_PLAY_WINDOW_SCRIPT {
        return Err(semio_framework_plugin::Fault::from("sequence-script-window-kind-required"));
    }
    Ok(semio_framework_plugin::WindowTransientMutation::of::<SequenceScriptWindowTransientOwner>(id, SequenceScriptWindowTransientMutation::Snapshot { transient }))
}
