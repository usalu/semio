//! 🫧️ Ephemeral local-only state of ONE exact WFC Bitmap Input window: the brush stroke in flight. A stroke
//! spans many dispatches (one per pointer batch), so its tool state — the statechart configuration and the open
//! `ToolTransaction` holding the one provisional `paint-input-stroke` leaf — lives here between them. Tool state is
//! never config and never history; only the committed leaf is (design §5, ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).

use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️State
/// 🖌️ A window's in-flight brush stroke: the statechart configuration by stable ids, the admission seed the
/// transaction was minted under, the open `TransactionRef`, and the one provisional leaf in value form.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapBrushToolState {
    pub states: Vec<String>,
    pub authoring_seed: String,
    pub transaction: protocol::TransactionRef,
    pub stroke: dsl::DslValue,
}

/// 🫧️ The input window's transient partition: the brush stroke in flight, `None` at rest. Boxed: a gesture state
/// exceeds the ephemeral ownership transfer's inline bound.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct BitmapInputWindowTransient {
    pub brush: Option<Box<BitmapBrushToolState>>,
}

store::artifact_retire_struct!(BitmapBrushToolState { states, authoring_seed, transaction, stroke });
store::artifact_retire_struct!(BitmapInputWindowTransient { brush });
//#endregion 🔖️State

//#region 🔖️Mutation
/// 🔁️ The one write the partition takes: its whole next state.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
pub enum BitmapInputWindowTransientMutation {
    Snapshot { transient: BitmapInputWindowTransient },
}

impl protocol::Mutation<BitmapInputWindowTransient> for BitmapInputWindowTransientMutation {
    type Diff = BitmapInputWindowTransient;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️input/🫧️transient",
        semantic_kind: "set-window-transient",
        display_name: "Set Bitmap Input Window Transient",
        emoji: "🫧️",
        aggregate_variant: "Snapshot",
        payload_schema: "wfcbitmap.inputwindowtransient",
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
    fn diff(&self, _base: &BitmapInputWindowTransient) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Snapshot { transient } => protocol::MutationOutcome::new(transient.clone()),
        }
    }
    fn inverse(&self, base: &BitmapInputWindowTransient) -> Vec<Self> {
        vec![Self::Snapshot { transient: base.clone() }]
    }
}

impl protocol::MutationDiff<BitmapInputWindowTransient> for BitmapInputWindowTransient {
    fn apply(&self, _base: &BitmapInputWindowTransient) -> protocol::MutationApplyResult<BitmapInputWindowTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl store::retirement::RetireOwned for BitmapInputWindowTransientMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        match self {
            Self::Snapshot { transient } => store::retirement::sequence(vec![store::retirement::leaf(0u8), store::retirement::RetireOwned::retirement(transient)]),
        }
    }
}
//#endregion 🔖️Mutation

//#region 🔖️Codecs
/// 📜️ JSON text form inside the partition's own envelope id.
impl store::ArtifactDsl for BitmapInputWindowTransient {
    const EXTENSION: &'static str = "wfcbitmapinputwindowtransient";
    fn envelope_id() -> &'static str {
        "s.wfc.bitmap.inputwindowtransient"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        dsl::json::to_json_string(self)
    }
}

/// 🎒️ Value-form pack.
impl store::ArtifactPack for BitmapInputWindowTransient {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        dsl::to_dsl_value(self).map_err(store::PackError::Schema)?.encode_pack_with(options)
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = dsl::DslValue::decode_pack_with(bytes, options)?;
        dsl::from_dsl_value(value).map_err(store::PackError::Schema)
    }
}

impl protocol::OpText for BitmapInputWindowTransientMutation {
    fn print_op(&self) -> String {
        dsl::json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for BitmapInputWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}
//#endregion 🔖️Codecs

//#region 🔖️Owner
/// 📏️ The retained bytes one partition write holds — refused above the store's one-item bound.
fn retained_bytes(transient: &BitmapInputWindowTransient) -> Option<usize> {
    let encoded = dsl::json::to_json_string(transient).len();
    let bytes = std::mem::size_of::<BitmapInputWindowTransient>().checked_add(encoded.checked_mul(2)?)?;
    (bytes <= store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).then_some(bytes)
}

fn preflight(mutation: &BitmapInputWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let BitmapInputWindowTransientMutation::Snapshot { transient } = mutation;
    let retained_bytes = retained_bytes(transient).ok_or_else(|| "Bitmap input window transient exceeds its retained publication envelope".to_string())?;
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn transfer(mutation: BitmapInputWindowTransientMutation) -> BitmapInputWindowTransient {
    match mutation {
        BitmapInputWindowTransientMutation::Snapshot { transient } => transient,
    }
}

/// 🪟️ The input window kind's transient owner.
pub struct BitmapInputWindowTransientOwner;

impl semio_framework_plugin::WindowTransientOwner for BitmapInputWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = super::WFC_BITMAP_WINDOW_INPUT;
    type State = BitmapInputWindowTransient;
    type Mutation = BitmapInputWindowTransientMutation;
    fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        let state = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::State>::default());
        let mutation = std::sync::Arc::new(store::retirement::OwnedValueRetirementFactory::<Self::Mutation>::default());
        let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(preflight, transfer, state.clone(), mutation.clone()));
        semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
    }
}

/// 🫧️ The input window's transient partition from a job context, or the resting default.
pub fn current(snapshot: Option<&semio_framework_plugin::WindowTransientSnapshot>) -> BitmapInputWindowTransient {
    snapshot.and_then(|value| value.get::<BitmapInputWindowTransientOwner>()).cloned().unwrap_or_default()
}

/// 🎯️ Addresses one partition write at the window being dispatched — refused when it is not an input window.
pub fn addressed(view: &semio_framework_plugin::ViewModel, transient: BitmapInputWindowTransient) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("wfc-bitmap-input-window-required"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("wfc-bitmap-window-stale"))?;
    if kind != super::WFC_BITMAP_WINDOW_INPUT {
        return Err(semio_framework_plugin::Fault::from("wfc-bitmap-input-window-kind-required"));
    }
    Ok(semio_framework_plugin::WindowTransientMutation::of::<BitmapInputWindowTransientOwner>(id, BitmapInputWindowTransientMutation::Snapshot { transient }))
}
//#endregion 🔖️Owner
