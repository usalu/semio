//! 🫧️ Ephemeral local Flow state bound to the exact invoking window.

use crate::playbook::GenerationPlayState;

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct FlowWindowTransient {
    pub generation_json: String,
    pub duplicate_widget_progress_json: String,
}

impl FlowWindowTransient {
    pub fn generation(&self) -> GenerationPlayState {
        serde_json::from_str(&self.generation_json).unwrap_or_default()
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum FlowWindowTransientMutation {
    Snapshot { transient: FlowWindowTransient },
}

impl protocol::Mutation<FlowWindowTransient> for FlowWindowTransientMutation {
    type Diff = FlowWindowTransient;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🫧️transient",
        semantic_kind: "set-window-transient",
        display_name: "Set Flow Window Transient",
        emoji: "🫧️",
        aggregate_variant: "Snapshot",
        payload_schema: "flow.windowtransient",
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
    fn diff(&self, _base: &FlowWindowTransient) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Snapshot { transient } => protocol::MutationOutcome::new(transient.clone()),
        }
    }
    fn inverse(&self, base: &FlowWindowTransient) -> Vec<Self> {
        vec![Self::Snapshot { transient: base.clone() }]
    }
}

impl protocol::MutationDiff<FlowWindowTransient> for FlowWindowTransient {
    fn apply(&self, _base: &FlowWindowTransient) -> protocol::MutationApplyResult<FlowWindowTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl store::ArtifactDsl for FlowWindowTransient {
    const EXTENSION: &'static str = "flowwindowtransient";
    fn envelope_id() -> &'static str { "s.flow.flow.windowtransient" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let json: serde_json::Value = serde_json::from_str(body).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_string_pretty(&value).expect("Flow window transient JSON");
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Flow window envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for FlowWindowTransient {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_vec(&value).map_err(|error| store::PackError::Schema(error.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) { return Err(store::PackError::Schema("Flow window pack envelope mismatch".into())); }
        let json: serde_json::Value = serde_json::from_slice(&body).map_err(|error| store::PackError::Schema(error.to_string()))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn record_spec() -> Option<dsl::RecordSpec> { None }
}

impl protocol::OpText for FlowWindowTransientMutation {
    fn print_op(&self) -> String {
        dsl::json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for FlowWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

store::artifact_retire_struct!(FlowWindowTransient { generation_json, duplicate_widget_progress_json });

impl store::retirement::RetireOwned for FlowWindowTransientMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        match self {
            Self::Snapshot { transient } => store::retirement::sequence(vec![store::retirement::leaf(0u8), store::retirement::RetireOwned::retirement(transient)]),
        }
    }
}

fn preflight(mutation: &FlowWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let FlowWindowTransientMutation::Snapshot { transient } = mutation;
    let retained_bytes = transient.generation_json.len().checked_add(transient.duplicate_widget_progress_json.len()).ok_or_else(|| "Flow window transient footprint overflowed".to_string())?;
    let footprint = store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes };
    footprint.is_admissible().then_some(footprint).ok_or_else(|| "Flow window transient exceeds its retained publication envelope".into())
}

fn transfer(mutation: FlowWindowTransientMutation) -> FlowWindowTransient {
    match mutation {
        FlowWindowTransientMutation::Snapshot { transient } => transient,
    }
}

macro_rules! transient_owner {
    ($owner:ident, $kind:path) => {
        pub struct $owner;
        impl semio_framework_plugin::WindowTransientOwner for $owner {
            const WINDOW_KIND_ID: &'static str = $kind;
            type State = FlowWindowTransient;
            type Mutation = FlowWindowTransientMutation;
            fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
                let state = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::State>::default());
                let mutation = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
                let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(preflight, transfer, state.clone(), mutation.clone()));
                semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
            }
        }
    };
}

transient_owner!(FlowMainWindowTransientOwner, super::FLOW_PLAY_WINDOW_MAIN);
transient_owner!(FlowGenerationsWindowTransientOwner, crate::editor::flow::modes::generate::windows::generations::FLOW_PLAY_WINDOW_GENERATIONS);
transient_owner!(FlowFormWindowTransientOwner, crate::editor::flow::modes::generate::windows::form::FLOW_PLAY_WINDOW_GENERATE_FORM);
transient_owner!(FlowPreviewWindowTransientOwner, crate::editor::flow::modes::generate::windows::preview::FLOW_PLAY_WINDOW_GENERATE_PREVIEW);

pub fn register(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<FlowMainWindowTransientOwner>()?;
    registry.register::<FlowGenerationsWindowTransientOwner>()?;
    registry.register::<FlowFormWindowTransientOwner>()?;
    registry.register::<FlowPreviewWindowTransientOwner>()
}

fn from_owner<O: semio_framework_plugin::WindowTransientOwner<State = FlowWindowTransient, Mutation = FlowWindowTransientMutation>>(snapshot: &semio_framework_plugin::WindowTransientSnapshot) -> Option<FlowWindowTransient> {
    snapshot.get::<O>().cloned()
}

pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowTransientSnapshot>) -> FlowWindowTransient {
    let Some(snapshot) = snapshot else { return FlowWindowTransient::default() };
    match snapshot.window_kind_id() {
        super::FLOW_PLAY_WINDOW_MAIN => from_owner::<FlowMainWindowTransientOwner>(snapshot),
        crate::editor::flow::modes::generate::windows::generations::FLOW_PLAY_WINDOW_GENERATIONS => from_owner::<FlowGenerationsWindowTransientOwner>(snapshot),
        crate::editor::flow::modes::generate::windows::form::FLOW_PLAY_WINDOW_GENERATE_FORM => from_owner::<FlowFormWindowTransientOwner>(snapshot),
        crate::editor::flow::modes::generate::windows::preview::FLOW_PLAY_WINDOW_GENERATE_PREVIEW => from_owner::<FlowPreviewWindowTransientOwner>(snapshot),
        _ => None,
    }
    .unwrap_or_default()
}

pub fn current(view: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>) -> FlowWindowTransient {
    from_snapshot(view.window)
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, transient: FlowWindowTransient) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("flow-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("flow-window-stale"))?;
    let mutation = FlowWindowTransientMutation::Snapshot { transient };
    match kind {
        super::FLOW_PLAY_WINDOW_MAIN => Ok(semio_framework_plugin::WindowTransientMutation::of::<FlowMainWindowTransientOwner>(id, mutation)),
        crate::editor::flow::modes::generate::windows::generations::FLOW_PLAY_WINDOW_GENERATIONS => Ok(semio_framework_plugin::WindowTransientMutation::of::<FlowGenerationsWindowTransientOwner>(id, mutation)),
        crate::editor::flow::modes::generate::windows::form::FLOW_PLAY_WINDOW_GENERATE_FORM => Ok(semio_framework_plugin::WindowTransientMutation::of::<FlowFormWindowTransientOwner>(id, mutation)),
        crate::editor::flow::modes::generate::windows::preview::FLOW_PLAY_WINDOW_GENERATE_PREVIEW => Ok(semio_framework_plugin::WindowTransientMutation::of::<FlowPreviewWindowTransientOwner>(id, mutation)),
        _ => Err(semio_framework_plugin::Fault::from("flow-window-kind-required")),
    }
}
