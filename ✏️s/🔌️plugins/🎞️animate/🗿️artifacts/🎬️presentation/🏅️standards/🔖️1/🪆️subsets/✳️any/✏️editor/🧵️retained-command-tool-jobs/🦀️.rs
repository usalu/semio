//! 🧵️ Retained command tool jobs: the retained tool ids, payload schema and capacities, the publication contracts, the retained contract, selection and extent reductions and the retained command job factory. See [`super`].

use super::*;

//#region 🧵️RetainedCommands
pub const ANIMATE_PRESENTATION_RETAINED_TOOL_IDS: &[&str] = &[
    "seedGrid",
    "addTile",
    "deleteTile",
    "deleteSelection",
    "renameTiles",
    "patchTileCrops",
    "setSource",
    "setFrame",
    "setActiveExample",
    "clearTiles",
    "engagementSubmit",
    "resetGrid",
    "engagementInput",
    "canvasPointerDown",
    "noMutation",
    "copyPrompt",
    "exportVideoFromDeck",
];

const ANIMATE_PRESENTATION_RETAINED_PAYLOAD_SCHEMA: &str = "animate.presentation.tool-command.v1";

pub(super) const ANIMATE_PRESENTATION_RETAINED_RAW_BYTES: usize = 8_192;

pub(super) const ANIMATE_PRESENTATION_RETAINED_WORK_ITEMS: usize = 1;

pub const ANIMATE_PRESENTATION_CONFIG_VALUE_BYTES: usize = 512;

pub(super) const ANIMATE_PRESENTATION_CONFIG_BASE_BYTES: usize = 512;

pub const ANIMATE_PRESENTATION_CONFIG_STEP_BYTES: usize = 4_096;

/// 🀄️ The largest tile roster one retained verb may build. A roster is staged as one `delete-tiles` row
/// plus one `create-tile` row per tile, so nothing downstream bounds its length — this constant is the only ceiling.
const ANIMATE_PRESENTATION_MAXIMUM_TILES: usize = 256;

pub const ANIMATE_PRESENTATION_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "seedGrid", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "addTile", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "deleteTile", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "renameTiles", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchTileCrops", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setSource", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setFrame", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "clearTiles", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "resetGrid", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "noMutation", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "copyPrompt", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "exportVideoFromDeck", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];

/// 🧾️ The ONE execution contract this app's retained factory publishes AND declares in its
/// `bounded_first_step_tool_proofs!` row. `validate_tool_job_rows`'s `typed_join` compares the two
/// for equality, so a second literal in the proof row is a rejection waiting to happen — a
/// `bounded_first_step(…)` literal there against this `resumable(…)` factory shape is exactly what
/// used to fail the whole catalog (`interactive-job.catalog-authority`, `typed_join=false`).
pub(super) fn animate_presentation_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(ANIMATE_PRESENTATION_RETAINED_RAW_BYTES, 64, 1, 65_536, 7_500, 1, 1)
}

/// 🕹️ The tile ids the framework-owned `tiles` interaction domain holds selected, read straight off
/// the retained job's own `InteractionState` — the retained lane used to hand every handler an EMPTY
/// selection, which silently turned `deleteSelection`/`renameTiles`/`patchTileCrops` into no-ops the
/// moment they became retained jobs. `ArtifactApp::handle` reads the same domain (`:830`).
fn animate_presentation_selected_ids(interaction: &protocol::InteractionState) -> Vec<String> {
    interaction.selection.get(PRESENTATION_INTERACTION_DOMAIN).map(|selection| selection.ids.clone()).unwrap_or_default()
}

/// 📏️ Every retained verb here is ONE semantic work item — the job reduces the whole command in a
/// single `BoundedArtifactCommandWork` step, and `ArtifactRetainedCommandPayload::new` is handed
/// `ANIMATE_PRESENTATION_RETAINED_WORK_ITEMS` (1) as its preflight ceiling, so an extent priced per
/// emitted ROW is refused outright (`retained command exceeds semantic work capacity` — measured on
/// `seedGrid` 2×2 before this was corrected). Fan-out is therefore bounded HERE, by refusing the
/// command whose payload would build more than this app's tile ceiling, not by the extent number.
pub(super) fn animate_presentation_retained_extent(command: &PresentationCommand, _snapshot: &PresentationSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let bounded = |length: usize| (length <= ANIMATE_PRESENTATION_RETAINED_RAW_BYTES).then_some(1);
    match command {
        PresentationCommand::SeedGrid(payload) => (payload.rows as usize).checked_mul(payload.columns as usize).filter(|cells| *cells > 0 && *cells <= ANIMATE_PRESENTATION_MAXIMUM_TILES).map(|_| 1),
        PresentationCommand::AddTile(_) | PresentationCommand::DeleteSelection(_) | PresentationCommand::ClearTiles(_) | PresentationCommand::ResetGrid(_) | PresentationCommand::NoOperation(_) | PresentationCommand::CopyPrompt(_) | PresentationCommand::SetFrame(_) => Some(1),
        PresentationCommand::DeleteTile(payload) => bounded(payload.id.len()),
        PresentationCommand::RenameTiles(payload) => (payload.value.len() <= ANIMATE_PRESENTATION_RETAINED_RAW_BYTES && payload.ids.len() <= ANIMATE_PRESENTATION_MAXIMUM_TILES).then_some(1),
        PresentationCommand::PatchTileCrops(payload) => (payload.field.len() <= ANIMATE_PRESENTATION_RETAINED_RAW_BYTES && payload.ids.len() <= ANIMATE_PRESENTATION_MAXIMUM_TILES).then_some(1),
        PresentationCommand::SetSource(payload) => bounded(payload.source.src.len()),
        PresentationCommand::SetActiveExample(payload) => bounded(payload.example_id.len()),
        PresentationCommand::EngagementSubmit(payload) => bounded(payload.value.len()),
        PresentationCommand::EngagementInput(payload) => (payload.value.len() <= ANIMATE_PRESENTATION_CONFIG_VALUE_BYTES).then_some(1),
        PresentationCommand::CanvasPointerDown(payload) => bounded(payload.layer_id.as_ref().map_or(0, String::len)),
        PresentationCommand::ExportVideoFromDeck(payload) => bounded(payload.scene_json.len()),
    }
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
pub(super) fn animate_presentation_retained_reduce(
    command: &PresentationCommand,
    snapshot: &PresentationSnapshot,
    config: &PresentationConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<AnimatePresentationPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<PresentationMutation, PresentationConfigMutation, NoDraftMutation>, Fault> {
    if !ANIMATE_PRESENTATION_RETAINED_TOOL_IDS.contains(&command.command_id()) || animate_presentation_retained_extent(command, snapshot, interaction).is_none() {
        return Err(Fault::from("animate-presentation-retained-route-mismatch"));
    }
    let document = ArtifactView::with_operation(snapshot, history, operation.clone());
    let config = ConfigView { snapshot: config, window: None };
    let mut context = PresentationDispatchCtx { selected_ids: animate_presentation_selected_ids(interaction) };
    command.dispatch(&document, &config, &mut context)
}

pub(super) struct AnimatePresentationRetainedCommandJobFactory {
    pub(super) keys: Vec<ToolFactoryKey>,
}

impl AnimatePresentationRetainedCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: ANIMATE_PRESENTATION_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for AnimatePresentationRetainedCommandJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<EditorApp<AnimatePresentationPlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<EditorApp<AnimatePresentationPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        ANIMATE_PRESENTATION_RETAINED_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        animate_presentation_retained_contract()
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > ANIMATE_PRESENTATION_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|value| value.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("Animate Presentation retained command rejects an oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl ArtifactOwnedToolJobFactory for AnimatePresentationRetainedCommandJobFactory {
    type Owner = EditorApp<AnimatePresentationPlayApp>;
    const TOOL_IDS: &'static [&'static str] = ANIMATE_PRESENTATION_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = PRESENTATION_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = ANIMATE_PRESENTATION_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands
