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
    fn inverse(&self, base: &SequenceScriptWindowTransient) -> Vec<Self> {
        vec![Self::Snapshot { transient: base.clone() }]
    }
}

impl protocol::MutationDiff<SequenceScriptWindowTransient> for SequenceScriptWindowTransient {
    fn apply(&self, _base: &SequenceScriptWindowTransient) -> protocol::MutationApplyResult<SequenceScriptWindowTransient> { Ok(self.clone()) }
    fn absorb(&mut self, other: Self) { *self = other; }
}

impl store::ArtifactDsl for SequenceScriptWindowTransient {
    const EXTENSION: &'static str = "sequencescriptwindowtransient";
    fn envelope_id() -> &'static str { "s.sequence.sequence.scriptwindowtransient" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let json: serde_json::Value = serde_json::from_str(body).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_string_pretty(&value).expect("Sequence script transient JSON");
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Sequence transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SequenceScriptWindowTransient {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_vec(&value).map_err(|error| store::PackError::Schema(error.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) { return Err(store::PackError::Schema("Sequence transient pack envelope mismatch".into())); }
        let json: serde_json::Value = serde_json::from_slice(&body).map_err(|error| store::PackError::Schema(error.to_string()))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn record_spec() -> Option<dsl::RecordSpec> { None }
}

impl protocol::OpText for SequenceScriptWindowTransientMutation {
    fn print_op(&self) -> String { dsl::json::to_json_string(self) }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}
impl protocol::OpBinary for SequenceScriptWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

store::artifact_retire_struct!(SequenceScriptWindowTransient { last_run_json });
impl store::retirement::RetireOwned for SequenceScriptWindowTransientMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        match self {
            Self::Snapshot { transient } => store::retirement::sequence(vec![store::retirement::leaf(0u8), store::retirement::RetireOwned::retirement(transient)]),
        }
    }
}

fn preflight(mutation: &SequenceScriptWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let SequenceScriptWindowTransientMutation::Snapshot { transient } = mutation;
    let footprint = store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: transient.last_run_json.len() };
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
        let state = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
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
