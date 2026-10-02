//! 🫧️ Ephemeral local engagement state bound to one exact CAD world window (design §17.4 of ticket
//! 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): the action line, the REPL step, the live interaction session and the
//! interaction the window last finalized. Per-frame state never lives in config, so typing a line, a pointer move and a
//! possible-select publish nothing durable and list no history row; only a commit lands, as one transform-tool transaction.

use crate::editor::cad::modes::edit::windows::{building, energy, shape, structure_classic};

/// 🫧️ One CAD world window's engagement state — see the module doc and `🧬️schema/🔣️.json`.
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct CadWorldWindowTransient {
    pub engagement_input: String,
    pub engagement_step: String,
    pub engagement_pane: Option<String>,
    pub engagement_session_json: Option<String>,
    pub last_finalized_interaction_id: Option<String>,
}

impl Default for CadWorldWindowTransient {
    fn default() -> Self {
        Self { engagement_input: String::new(), engagement_step: "Idle".into(), engagement_pane: None, engagement_session_json: None, last_finalized_interaction_id: None }
    }
}

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(tag = "kind", rename_all = "kebab-case")]
pub enum CadWorldWindowTransientMutation {
    Snapshot { transient: CadWorldWindowTransient },
}

impl protocol::Mutation<CadWorldWindowTransient> for CadWorldWindowTransientMutation {
    type Diff = CadWorldWindowTransient;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🫧️transient",
        semantic_kind: "set-window-transient",
        display_name: "Set CAD World Window Transient",
        emoji: "🫧️",
        aggregate_variant: "Snapshot",
        payload_schema: "cad.worldwindowtransient",
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
    fn diff(&self, _base: &CadWorldWindowTransient) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Snapshot { transient } => protocol::MutationOutcome::new(transient.clone()),
        }
    }
    fn inverse(&self, base: &CadWorldWindowTransient) -> Vec<Self> {
        vec![Self::Snapshot { transient: base.clone() }]
    }
}

impl protocol::MutationDiff<CadWorldWindowTransient> for CadWorldWindowTransient {
    fn apply(&self, _base: &CadWorldWindowTransient) -> protocol::MutationApplyResult<CadWorldWindowTransient> {
        Ok(self.clone())
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

impl store::ArtifactDsl for CadWorldWindowTransient {
    const EXTENSION: &'static str = "cadworldwindowtransient";
    fn envelope_id() -> &'static str {
        "cad.worldwindowtransient"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        dsl::json::from_json_str(body).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = dsl::json::to_json_string(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid CAD world-window transient envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for CadWorldWindowTransient {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = dsl::json::to_json_string(self).into_bytes();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema("CAD world-window transient pack envelope mismatch".into()));
        }
        let text = std::str::from_utf8(&body).map_err(|error| store::PackError::Schema(error.to_string()))?;
        dsl::json::from_json_str(text).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        None
    }
}

impl protocol::OpText for CadWorldWindowTransientMutation {
    fn print_op(&self) -> String {
        dsl::json::to_json_string(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        dsl::json::from_json_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}

impl protocol::OpBinary for CadWorldWindowTransientMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        Ok(protocol::OpText::print_op(self).into_bytes())
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let text = std::str::from_utf8(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))?;
        dsl::json::from_json_str(text).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

semio_framework_value::artifact_retire_struct!(CadWorldWindowTransient { engagement_input, engagement_step, engagement_pane, engagement_session_json, last_finalized_interaction_id });

impl semio_framework_value::retirement::RetireOwned for CadWorldWindowTransientMutation {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        match self {
            Self::Snapshot { transient } => semio_framework_value::retirement::sequence(vec![semio_framework_value::retirement::leaf(0u8), semio_framework_value::retirement::RetireOwned::retirement(transient)]),
        }
    }
}

/// 🧺️ One window publication's retained footprint: every text it carries.
fn preflight(mutation: &CadWorldWindowTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let CadWorldWindowTransientMutation::Snapshot { transient } = mutation;
    let retained_bytes = [Some(transient.engagement_input.as_str()), Some(transient.engagement_step.as_str()), transient.engagement_pane.as_deref(), transient.engagement_session_json.as_deref(), transient.last_finalized_interaction_id.as_deref()]
        .into_iter()
        .flatten()
        .try_fold(0usize, |bytes, text| bytes.checked_add(text.len()))
        .ok_or_else(|| "CAD world-window transient footprint overflowed".to_string())?;
    let footprint = store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes };
    footprint.is_admissible().then_some(footprint).ok_or_else(|| "CAD world-window transient exceeds its retained publication envelope".into())
}

fn transfer(mutation: CadWorldWindowTransientMutation) -> CadWorldWindowTransient {
    match mutation {
        CadWorldWindowTransientMutation::Snapshot { transient } => transient,
    }
}

macro_rules! cad_world_window_transient_owner {
    ($owner:ident, $kind:path) => {
        pub struct $owner;
        impl semio_framework_plugin::WindowTransientOwner for $owner {
            const WINDOW_KIND_ID: &'static str = $kind;
            type State = crate::editor::cad::modes::edit::windows::transient::CadWorldWindowTransient;
            type Mutation = crate::editor::cad::modes::edit::windows::transient::CadWorldWindowTransientMutation;
            fn build_owners() -> semio_framework_plugin::WindowTransientOwnerBundle<Self::State, Self::Mutation> {
                crate::editor::cad::modes::edit::windows::transient::owner_bundle()
            }
        }
    };
}
pub(crate) use cad_world_window_transient_owner;

/// 🧰️ The exact retirement and ephemeral-transfer owners every CAD world window's transient lane is built from.
pub fn owner_bundle() -> semio_framework_plugin::WindowTransientOwnerBundle<CadWorldWindowTransient, CadWorldWindowTransientMutation> {
    let state = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<CadWorldWindowTransient>::default());
    let mutation = std::sync::Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::<CadWorldWindowTransientMutation>::default());
    let preparation = std::sync::Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(preflight, transfer, state.clone(), mutation.clone()));
    semio_framework_plugin::WindowTransientOwnerBundle::new(preparation, state, mutation)
}

pub fn register(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), semio_framework_plugin::Fault> {
    registry.register::<shape::transient::CadShapeWindowTransientOwner>()?;
    registry.register::<building::transient::CadBuildingWindowTransientOwner>()?;
    registry.register::<energy::transient::CadEnergyWindowTransientOwner>()?;
    registry.register::<structure_classic::transient::CadStructureClassicWindowTransientOwner>()
}

/// 🔎️ The engagement state of the window `snapshot` belongs to; at rest without one.
pub fn from_snapshot(snapshot: Option<&semio_framework_plugin::WindowTransientSnapshot>) -> CadWorldWindowTransient {
    let Some(snapshot) = snapshot else { return CadWorldWindowTransient::default() };
    match snapshot.window_kind_id() {
        shape::WINDOW_KIND_ID => snapshot.get::<shape::transient::CadShapeWindowTransientOwner>(),
        building::WINDOW_KIND_ID => snapshot.get::<building::transient::CadBuildingWindowTransientOwner>(),
        energy::WINDOW_KIND_ID => snapshot.get::<energy::transient::CadEnergyWindowTransientOwner>(),
        structure_classic::WINDOW_KIND_ID => snapshot.get::<structure_classic::transient::CadStructureClassicWindowTransientOwner>(),
        _ => None,
    }
    .cloned()
    .unwrap_or_default()
}

/// 🔎️ The engagement state of the window a render or projection runs for.
pub fn current<T>(view: &semio_framework_plugin::TransientView<'_, T>) -> CadWorldWindowTransient {
    from_snapshot(view.window)
}

/// 📮️ `transient` addressed to the exact window instance `view` names.
pub fn addressed(view: &semio_framework_plugin::ViewModel, transient: CadWorldWindowTransient) -> Result<semio_framework_plugin::WindowTransientMutation, semio_framework_plugin::Fault> {
    let id = view.window_id.as_deref().ok_or_else(|| semio_framework_plugin::Fault::from("cad.window.required: command has no addressed window instance"))?;
    let kind = view.window_instances.iter().find(|window| window.id == id).map(|window| window.window_kind_id.as_str()).ok_or_else(|| semio_framework_plugin::Fault::from("cad.window.stale: addressed window instance is not open"))?;
    let mutation = CadWorldWindowTransientMutation::Snapshot { transient };
    match kind {
        shape::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowTransientMutation::of::<shape::transient::CadShapeWindowTransientOwner>(id, mutation)),
        building::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowTransientMutation::of::<building::transient::CadBuildingWindowTransientOwner>(id, mutation)),
        energy::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowTransientMutation::of::<energy::transient::CadEnergyWindowTransientOwner>(id, mutation)),
        structure_classic::WINDOW_KIND_ID => Ok(semio_framework_plugin::WindowTransientMutation::of::<structure_classic::transient::CadStructureClassicWindowTransientOwner>(id, mutation)),
        _ => Err(semio_framework_plugin::Fault::from("cad.window.kind: addressed window is not a CAD world window")),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
