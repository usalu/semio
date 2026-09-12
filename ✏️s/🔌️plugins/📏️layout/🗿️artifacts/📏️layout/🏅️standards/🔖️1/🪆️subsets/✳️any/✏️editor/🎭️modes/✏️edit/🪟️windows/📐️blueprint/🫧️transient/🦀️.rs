//! 🫧️ Ephemeral Layout interaction state bound to one exact Blueprint or Preview window.

use crate::LayoutDropPreviewState;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct LayoutWindowTransient {
    pub drop_preview: LayoutDropPreviewState,
    pub engagement_input: String,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum LayoutWindowTransientMutation {
    Snapshot { transient: LayoutWindowTransient },
}

impl protocol::Mutation<LayoutWindowTransient> for LayoutWindowTransientMutation {
    type Diff = LayoutWindowTransient;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📐️blueprint/🫧️transient",
        semantic_kind: "set-window-transient",
        display_name: "Set Layout Window Transient",
        emoji: "🫧️",
        aggregate_variant: "Snapshot",
        payload_schema: "layout.windowtransient",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::JsonSchema],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &Self::DESCRIPTORS[0] }
    fn diff(&self, _base: &LayoutWindowTransient) -> protocol::MutationOutcome<Self::Diff> {
        match self { Self::Snapshot { transient } => protocol::MutationOutcome::new(transient.clone()) }
    }
    fn inverse(&self, base: &LayoutWindowTransient) -> Vec<Self> {
        vec![Self::Snapshot { transient: base.clone() }]
    }
}

impl protocol::MutationDiff<LayoutWindowTransient> for LayoutWindowTransient {
    fn apply(&self, _base: &LayoutWindowTransient) -> protocol::MutationApplyResult<LayoutWindowTransient> { Ok(self.clone()) }
    fn absorb(&mut self, other: Self) { *self = other; }
}

impl store::retirement::RetireOwned for LayoutDropPreviewState {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        store::retirement::sequence(vec![
            store::retirement::RetireOwned::retirement(self.kind),
            store::retirement::RetireOwned::retirement(self.x),
            store::retirement::RetireOwned::retirement(self.y),
        ])
    }
}

impl store::ArtifactDsl for LayoutWindowTransient {
    const EXTENSION: &'static str = "layoutwindowtransient";
    fn envelope_id() -> &'static str { "s.layout.layout.windowtransient" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let json: serde_json::Value = serde_json::from_str(body).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_string_pretty(&value).expect("Layout window transient JSON");
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid Layout transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for LayoutWindowTransient {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let value: serde_json::Value = dsl::ToValue::to_value(self).into();
        let body = serde_json::to_vec(&value).map_err(|error| store::PackError::Schema(error.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) { return Err(store::PackError::Schema("Layout transient pack envelope mismatch".into())); }
        let json: serde_json::Value = serde_json::from_slice(&body).map_err(|error| store::PackError::Schema(error.to_string()))?;
        dsl::FromValue::from_value(json.into()).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn record_spec() -> Option<dsl::RecordSpec> { None }
}

impl protocol::OpText for LayoutWindowTransientMutation {
    fn print_op(&self) -> String { dsl::json::to_json_string(self) }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for LayoutWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> { Ok(protocol::OpText::print_op(self).into_bytes()) }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

store::artifact_retire_struct!(LayoutWindowTransient { drop_preview, engagement_input });
impl store::retirement::RetireOwned for LayoutWindowTransientMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        match self {
            Self::Snapshot { transient } => store::retirement::sequence(vec![store::retirement::leaf(0u8), store::retirement::RetireOwned::retirement(transient)]),
        }
    }
}

fn preflight(mutation: &LayoutWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let LayoutWindowTransientMutation::Snapshot { transient } = mutation;
    let retained_bytes = transient.engagement_input.len().checked_add(transient.drop_preview.kind.len()).ok_or_else(|| "Layout window transient footprint overflowed".to_string())?;
    let footprint = store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes };
    footprint.is_admissible().then_some(footprint).ok_or_else(|| "Layout window transient exceeds its retained publication envelope".into())
}

fn transfer(mutation: LayoutWindowTransientMutation) -> LayoutWindowTransient {
    match mutation { LayoutWindowTransientMutation::Snapshot { transient } => transient }
}

macro_rules! transient_owner {
    ($owner:ident, $kind:path) => {
        pub struct $owner;
        impl semio_framework_plugin::WindowTransientOwner for $owner {
            const WINDOW_KIND_ID: &'static str = $kind;
            type State = LayoutWindowTransient;
            type Mutation = LayoutWindowTransientMutation;
            fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
                let state = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::State>::default());
                let mutation = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
                let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(preflight, transfer, state.clone(), mutation.clone()));
                semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
            }
        }
    };
}

transient_owner!(LayoutBlueprintWindowTransientOwner, super::LAYOUT_PLAY_WINDOW_BLUEPRINT);
transient_owner!(LayoutPreviewWindowTransientOwner, crate::editor::layout::modes::edit::windows::preview::LAYOUT_PLAY_WINDOW_PREVIEW);

pub fn register(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<LayoutBlueprintWindowTransientOwner>()?;
    registry.register::<LayoutPreviewWindowTransientOwner>()
}

fn from_owner<O: semio_framework_plugin::WindowTransientOwner<State = LayoutWindowTransient, Mutation = LayoutWindowTransientMutation>>(snapshot: &semio_framework_plugin::WindowTransientSnapshot) -> Option<LayoutWindowTransient> {
    snapshot.get::<O>().cloned()
}

pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowTransientSnapshot>) -> LayoutWindowTransient {
    let Some(snapshot) = snapshot else { return LayoutWindowTransient::default() };
    match snapshot.window_kind_id() {
        super::LAYOUT_PLAY_WINDOW_BLUEPRINT => from_owner::<LayoutBlueprintWindowTransientOwner>(snapshot),
        crate::editor::layout::modes::edit::windows::preview::LAYOUT_PLAY_WINDOW_PREVIEW => from_owner::<LayoutPreviewWindowTransientOwner>(snapshot),
        _ => None,
    }.unwrap_or_default()
}

pub fn current(view: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>) -> LayoutWindowTransient {
    from_snapshot(view.window)
}

pub fn addressed(view: &semio_framework_plugin::ViewModel, transient: LayoutWindowTransient) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("layout-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("layout-window-stale"))?;
    let mutation = LayoutWindowTransientMutation::Snapshot { transient };
    match kind {
        super::LAYOUT_PLAY_WINDOW_BLUEPRINT => Ok(semio_framework_plugin::WindowTransientMutation::of::<LayoutBlueprintWindowTransientOwner>(id, mutation)),
        crate::editor::layout::modes::edit::windows::preview::LAYOUT_PLAY_WINDOW_PREVIEW => Ok(semio_framework_plugin::WindowTransientMutation::of::<LayoutPreviewWindowTransientOwner>(id, mutation)),
        _ => Err(semio_framework_plugin::Fault::from("layout-window-kind-required")),
    }
}
