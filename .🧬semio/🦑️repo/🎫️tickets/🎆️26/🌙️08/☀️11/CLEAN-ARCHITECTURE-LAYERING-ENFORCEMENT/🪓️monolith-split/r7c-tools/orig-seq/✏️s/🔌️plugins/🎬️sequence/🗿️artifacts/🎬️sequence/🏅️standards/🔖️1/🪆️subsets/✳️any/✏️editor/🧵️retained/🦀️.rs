//! 🧵️ Sequence retained routes: the caps and the artifact, config and example tool routes that publish through retained work.

use super::*;

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `SequencePlayApp::Command` — the SOLE dispatch surface for sequence's own behavior,
    /// assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword
    /// (the kebab-case `#[dsl(key = ..)]` the codec uses) — every row's wire keyword happens to be the
    /// plain kebab-case of its id (no `flow`-style divergence here), but the two are still copied
    /// independently from the pre-migration `sequence_protocol` enum's `command_id()` match arm and
    /// `#[dsl(key = ..)]` attribute respectively, never derived one from the other. **Row order is the
    /// binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum SequenceCommand for SequenceSnapshot, SequenceMutation, NoConfig, NoConfigMutation {
        "addStep" as "add-step" => add_step::AddStep,
        "addStepToSlot" as "add-step-to-slot" => add_step_to_slot::AddStepToSlot,
        "addStepDropped" as "add-step-dropped" => add_step_dropped::AddStepDropped,
        "removeStep" as "remove-step" => remove_step::RemoveStep,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "moveStep" as "move-step" => move_step::MoveStep,
        "connectSteps" as "connect-steps" => connect_steps::ConnectSteps,
        "disconnectSteps" as "disconnect-steps" => disconnect_steps::DisconnectSteps,
        "setStepParams" as "set-step-params" => set_step_params::SetStepParams,
        "setStepCollapsed" as "set-step-collapsed" => set_step_collapsed::SetStepCollapsed,
        "reorganize" as "reorganize" => reorganize::Reorganize,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "setOrientation" as "set-orientation" => set_orientation::SetOrientation,
        "run" as "run" => run_command::Run,
        "stop" as "stop" => stop_command::Stop,
        "setViewport" as "set-viewport" => set_viewport::SetViewport,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
    }
}
//#endregion 🔖️Commands

//#region 📏️RetainedCaps
pub const SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS: usize = 256;
pub const SEQUENCE_STORE_MAXIMUM_BYTES: usize = 65_536;

pub fn sequence_bounded_child_emit_bytes(child_emits: &[ChildEmit], maximum_bytes: usize) -> Result<usize, String> {
    let mut retained_bytes = 0usize;
    let mut admit = |bytes: usize| {
        retained_bytes = retained_bytes.checked_add(bytes).ok_or_else(|| "Sequence child publication byte count overflowed".to_string())?;
        if retained_bytes > maximum_bytes {
            return Err("Sequence child publication exceeds its fixed retained envelope".to_string());
        }
        Ok(())
    };
    for child in child_emits {
        admit(child.slot.len())?;
        admit(child.child_id.len())?;
        admit(child.op_schema.0.len())?;
        for label in &child.labels {
            admit(label.retained_bytes())?;
        }
        for op in &child.ops {
            admit(op.len())?;
        }
    }
    Ok(retained_bytes)
}
//#endregion 📏️RetainedCaps

//#region 🧵️RetainedArtifactRoutes
const SEQUENCE_RETAINED_ARTIFACT_PAYLOAD_SCHEMA: &str = "sequence.play/retained-artifact-command.v1";
pub const SEQUENCE_RETAINED_ARTIFACT_TOOL_IDS: &[&str] = &["addStep", "addStepToSlot", "addStepDropped", "removeStep", "deleteSelection", "moveStep", "connectSteps", "disconnectSteps", "setStepParams", "setStepCollapsed"];
const SEQUENCE_RETAINED_ARTIFACT_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addStepToSlot", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addStepDropped", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "moveStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "connectSteps", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "disconnectSteps", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setStepParams", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setStepCollapsed", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Child] },
];

fn sequence_retained_id_admitted(value: &str) -> bool {
    !value.is_empty() && value.len() <= 128
}

pub fn sequence_retained_artifact_command_admitted(command: &SequenceCommand) -> bool {
    match command {
        SequenceCommand::AddStep(payload) => sequence_retained_id_admitted(&payload.kind) && payload.x.is_finite() && payload.y.is_finite(),
        SequenceCommand::AddStepToSlot(payload) => sequence_retained_id_admitted(&payload.kind) && sequence_retained_id_admitted(&payload.owner) && sequence_retained_id_admitted(&payload.slot_name) && payload.x.is_finite() && payload.y.is_finite(),
        SequenceCommand::AddStepDropped(payload) => sequence_retained_id_admitted(&payload.kind) && payload.picked_step_id.as_deref().is_none_or(sequence_retained_id_admitted) && payload.x.is_finite() && payload.y.is_finite(),
        SequenceCommand::RemoveStep(payload) => sequence_retained_id_admitted(&payload.id),
        SequenceCommand::DeleteSelection(_) => true,
        SequenceCommand::MoveStep(payload) => sequence_retained_id_admitted(&payload.node_id) && payload.x.is_finite() && payload.y.is_finite(),
        SequenceCommand::ConnectSteps(payload) => sequence_retained_id_admitted(&payload.source_node_id) && sequence_retained_id_admitted(&payload.target_node_id),
        SequenceCommand::DisconnectSteps(payload) => sequence_retained_id_admitted(&payload.from_id) && sequence_retained_id_admitted(&payload.to_id),
        SequenceCommand::SetStepParams(payload) => sequence_retained_id_admitted(&payload.id) && payload.params_json.len() <= SEQUENCE_RETAINED_RAW_BYTES,
        SequenceCommand::SetStepCollapsed(payload) => sequence_retained_id_admitted(&payload.id),
        _ => false,
    }
}

fn sequence_retained_artifact_emit(command: &SequenceCommand, snapshot: &SequenceSnapshot, scene: &SequenceWorkingScene, interaction: &protocol::InteractionState) -> Result<(Emit<SequenceMutation, NoConfigMutation>, Option<Dictionary>), Fault> {
    if scene.steps.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS || scene.edges.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
        return Err(sequence_fault("sequence.editor.capacity", "sequence-retained-scene-capacity"));
    }
    let mut discarded_params = None;
    let mut edit = edit_rules::SceneEdit::new(scene.clone());
    match command {
        SequenceCommand::AddStep(payload) => {
            edit.add_step(&payload.kind, payload.x, payload.y, None);
        }
        SequenceCommand::AddStepToSlot(payload) => {
            edit.add_step(&payload.kind, payload.x, payload.y, Some(SlotRef { owner: payload.owner.clone(), name: payload.slot_name.clone() }));
        }
        SequenceCommand::AddStepDropped(payload) => {
            edit.add_step_dropped(&payload.kind, payload.x, payload.y, payload.picked_step_id.as_deref());
        }
        SequenceCommand::RemoveStep(payload) => {
            edit.remove_steps(&edit_rules::removal_closure(scene, [payload.id.clone()]));
        }
        SequenceCommand::DeleteSelection(_) => {
            let selected = interaction.selection.get(SEQUENCE_INTERACTION_STEPS).map(|selection| selection.ids.clone()).unwrap_or_default();
            if selected.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
                return Err(sequence_fault("sequence.editor.capacity", "sequence-retained-selection-capacity"));
            }
            edit.remove_steps(&edit_rules::removal_closure(scene, selected));
        }
        SequenceCommand::MoveStep(payload) => {
            if let Some(step) = scene.steps.iter().find(|step| step.id == payload.node_id) {
                edit.drag(std::slice::from_ref(&payload.node_id), payload.x - step.x, payload.y - step.y);
            }
        }
        SequenceCommand::SetStepParams(payload) => {
            if let Ok(params) = semio_framework_pack_json::from_json_str::<StepParams>(&payload.params_json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
                if let Err(mut params) = edit.set_params(&payload.id, params) {
                    discarded_params = Some(std::mem::take(&mut params.0));
                }
            }
        }
        SequenceCommand::SetStepCollapsed(payload) => {
            edit.toggle_collapsed(&payload.id);
        }
        SequenceCommand::DisconnectSteps(payload) => {
            edit.disconnect(&payload.from_id, &payload.to_id);
        }
        SequenceCommand::ConnectSteps(payload) => {
            edit.connect(&payload.source_node_id, &payload.target_node_id, true);
        }
        _ => return Err(sequence_fault("app.command.tool-mismatch", "sequence-retained-artifact-route-mismatch")),
    }
    if edit.leaves.len() > SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS {
        return Err(sequence_fault("sequence.editor.capacity", "sequence-retained-artifact-output-capacity"));
    }
    Ok((sequence_child_leaves_emit(snapshot, edit.leaves), discarded_params))
}

#[derive(Default)]
pub struct SequenceRetainedSceneOwner {
    pub scene: Option<SequenceWorkingScene>,
    pub pending: Vec<Dictionary>,
    pub retirement: ValueRetirement,
}

impl SequenceRetainedSceneOwner {
    pub fn capture(&mut self, snapshot: &SequenceSnapshot, children: &semio_framework_plugin::app::ChildContentView) -> Result<(), Fault> {
        if self.scene.is_none() {
            self.scene = Some(sequence_working_scene_from_children(snapshot, children)?.into_inner());
        }
        Ok(())
    }

    pub fn scene(&self) -> Result<&SequenceWorkingScene, Fault> {
        self.scene.as_ref().ok_or_else(|| sequence_fault("sequence.content.unavailable", "sequence-retained-scene-required"))
    }

    pub fn release_one(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        if grant.maximum_items == 0 {
            return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()));
        }
        if let Some(step) = sequence_drain_retirement(&mut self.retirement, grant) {
            return step;
        }
        if let Some(step) = sequence_push_pending(&mut self.retirement, &mut self.pending, grant) {
            return step;
        }
        if let Some(scene) = self.scene.as_mut() {
            if let Some(step) = scene.steps.last_mut() {
                let progress = sequence_push_params(&mut self.retirement, step, grant)?;
                scene.steps.pop();
                return Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress));
            }
            if scene.edges.pop().is_some() {
                return sequence_unit();
            }
        }
        if self.scene.take().is_some() {
            return sequence_unit();
        }
        Ok(semio_framework_value::retained_clone::RetainedCloneStep::Complete(Default::default()))
    }

    pub fn close_demand(&self) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        let admits = !self.pending.is_empty() || self.scene.as_ref().is_some_and(|scene| !scene.steps.is_empty());
        sequence_owner_demand(&self.retirement, admits, self.scene.is_some())
    }

    pub fn empty(&self) -> bool {
        self.scene.is_none() && self.pending.is_empty() && self.retirement.terminal_is_empty()
    }
}

pub struct SequenceRetainedArtifactWork {
    pub tool_id: &'static str,
    pub workspace_identity: u64,
    pub cursor: usize,
    pub replay_target: Option<usize>,
    pub scene_owner: SequenceRetainedSceneOwner,
    pub completed: bool,
    pub closing: bool,
}

impl SequenceRetainedArtifactWork {
    pub fn new(tool_id: &'static str, operation: &semio_framework_plugin::AppOperationContext) -> Self {
        let scope = format!("{}:{}:{}:{}", operation.app_instance_id, operation.parent_document_id, operation.operation_id, operation.generation);
        let workspace_identity = scope.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325_u64, |state, byte| (state ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3));
        Self { tool_id, workspace_identity, cursor: 0, replay_target: None, scene_owner: SequenceRetainedSceneOwner::default(), completed: false, closing: false }
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<SequencePlayApp>> for SequenceRetainedArtifactWork {
    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<SequencePlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
    }
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }
    fn workspace_identity(&self) -> u64 {
        self.workspace_identity
    }
    fn extent(
        &self,
        _command: &SequenceCommand,
        snapshot: &SequenceSnapshot,
        interaction: &protocol::InteractionState,
        context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<SequencePlayApp>>>,
    ) -> Option<usize> {
        let scene = sequence_working_scene_from_children(snapshot, context?.children.as_ref()).ok()?;
        (scene.steps.len() <= SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS
            && scene.edges.len() <= SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS
            && interaction.selection.get(SEQUENCE_INTERACTION_STEPS).is_none_or(|selection| selection.ids.len() <= SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS))
        .then_some(SEQUENCE_RETAINED_MAXIMUM_UNITS)
    }

    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<SequencePlayApp>>,
    _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<SequencePlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config: _config, history: _history, interaction, hover: _hover, context, operation: _operation } = *input;
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.completed || self.cursor >= SEQUENCE_RETAINED_MAXIMUM_UNITS || !sequence_retained_artifact_command_admitted(command) {
            return Err(sequence_fault("sequence.retained.artifact-command", "sequence-retained-artifact-envelope"));
        }
        self.cursor += 1;
        if let Some(target) = self.replay_target {
            if self.cursor <= target {
                if self.cursor == target {
                    self.replay_target = None;
                }
                return Ok(ArtifactCommandWorkStep::Replay { stage: "sequence-artifact-replay", preview: b"{\"en\":\"Restoring Sequence edit\",\"de\":\"Sequenzbearbeitung wird wiederhergestellt\"}" });
            }
        }
        if self.cursor == 1 {
            return Ok(ArtifactCommandWorkStep::Progress { stage: "sequence-artifact-prepare", preview: b"{\"en\":\"Preparing Sequence edit\",\"de\":\"Sequenzbearbeitung wird vorbereitet\"}" });
        }
        let context = context.ok_or_else(|| sequence_fault("sequence.content.unavailable", "sequence-content-child-context-required"))?;
        self.scene_owner.capture(snapshot, context.children.as_ref())?;
        let scene = self.scene_owner.scene()?;
        let (emit, discarded_params) = sequence_retained_artifact_emit(command, snapshot, scene, interaction)?;
        if let Some(params) = discarded_params {
            self.scene_owner.pending.push(params);
        }
        let exact_child = emit.child_emits.first().is_none_or(|child| child.slot == "content" && child.child_id == snapshot.content.child_id);
        if !emit.config_mutations.is_empty() || !emit.draft_mutations.is_empty() || !emit.artifact_mutations.is_empty() || emit.child_emits.len() > 1 || !exact_child {
            return Err(sequence_fault("sequence.publication.lane", "sequence-retained-artifact-publication-lane"));
        }
        sequence_bounded_child_emit_bytes(&emit.child_emits, SEQUENCE_STORE_MAXIMUM_BYTES).map_err(|_| sequence_fault("sequence.editor.capacity", "sequence-retained-artifact-output-bytes"))?;
        self.completed = true;
        Ok(ArtifactCommandWorkStep::Complete(emit))
    }

    fn checkpoint_byte(&self, index: usize) -> Option<u8> {
        let mut target = [0_u8; 24];
        self.write_checkpoint(&mut target).ok()?;
        target.get(index).copied()
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 24 || &checkpoint[..4] != b"SRA1" || checkpoint[4] > 1 || checkpoint[5..8] != [0, 0, 0] {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-retained-artifact-checkpoint-invalid"));
        }
        let cursor = usize::try_from(u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-artifact-checkpoint-cursor"))?)).map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-artifact-checkpoint-cursor"))?;
        let identity = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-artifact-checkpoint-identity"))?);
        if identity != self.workspace_identity || cursor > SEQUENCE_RETAINED_MAXIMUM_UNITS || !self.scene_owner.empty() {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-retained-artifact-checkpoint-owner-mismatch"));
        }
        self.cursor = 0;
        self.replay_target = (cursor != 0).then_some(cursor);
        self.completed = false;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }
    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if grant.maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
        }
        self.replay_target = None;
        sequence_close_step(self.scene_owner.release_one(grant))
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.scene_owner.close_demand()?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, _maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.scene_owner.close_demand()?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.scene_owner.close_demand()?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.scene_owner.close_demand()?.depth) }
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.replay_target.is_none() && self.scene_owner.empty()
    }
}

impl SequenceRetainedArtifactWork {
    pub fn write_checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 24 {
            return Err(sequence_fault("sequence.editor.capacity", "sequence-retained-artifact-checkpoint-capacity"));
        }
        target[..24].fill(0);
        target[..4].copy_from_slice(b"SRA1");
        target[4] = u8::from(self.completed);
        target[8..16].copy_from_slice(&(self.cursor as u64).to_le_bytes());
        target[16..24].copy_from_slice(&self.workspace_identity.to_le_bytes());
        Ok(24)
    }
}

impl SequencePersistentWork {
    pub fn write_checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 24 {
            return Err(sequence_fault("sequence.editor.capacity", "sequence-persistent-checkpoint-capacity"));
        }
        target[..24].fill(0);
        target[..4].copy_from_slice(b"SRP1");
        target[8..16].copy_from_slice(&(self.progress as u64).to_le_bytes());
        target[16..24].copy_from_slice(&self.workspace_identity.to_le_bytes());
        Ok(24)
    }
}

impl SequenceRetainedConfigWork {
    pub fn write_checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 24 {
            return Err(sequence_fault("sequence.editor.capacity", "sequence-retained-checkpoint-capacity"));
        }
        target[..24].fill(0);
        target[..4].copy_from_slice(b"SRC1");
        target[4] = u8::from(self.completed);
        target[8..16].copy_from_slice(&(self.cursor as u64).to_le_bytes());
        target[16..24].copy_from_slice(&self.workspace_identity.to_le_bytes());
        Ok(24)
    }
}

impl SequenceRetainedExampleWork {
    pub fn write_checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 24 {
            return Err(sequence_fault("sequence.editor.capacity", "sequence-retained-checkpoint-capacity"));
        }
        target[..24].fill(0);
        target[..4].copy_from_slice(b"SRE1");
        target[4] = u8::from(self.completed);
        target[8..16].copy_from_slice(&(self.cursor as u64).to_le_bytes());
        target[16..24].copy_from_slice(&self.workspace_identity.to_le_bytes());
        Ok(24)
    }
}

pub struct SequenceRetainedArtifactJobFactory {
    pub keys: Vec<semio_framework::ToolFactoryKey>,
}
impl SequenceRetainedArtifactJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: SEQUENCE_RETAINED_ARTIFACT_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}
impl semio_framework::ToolJobFactory for SequenceRetainedArtifactJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<SequencePlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<SequencePlayApp>>;
    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        SEQUENCE_RETAINED_ARTIFACT_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(SEQUENCE_RETAINED_RAW_BYTES, SEQUENCE_RETAINED_MAXIMUM_UNITS, 1, SEQUENCE_STORE_MAXIMUM_BYTES, 2_000, 1, 1)
    }
    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }
    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > SEQUENCE_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|value| value.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((semio_framework::ToolJobFactoryError::new("Sequence retained artifact command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}
impl semio_framework_plugin::ArtifactOwnedToolJobFactory for SequenceRetainedArtifactJobFactory {
    type Owner = semio_framework_plugin::EditorApp<SequencePlayApp>;
    const TOOL_IDS: &'static [&'static str] = SEQUENCE_RETAINED_ARTIFACT_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SEQUENCE_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = SEQUENCE_RETAINED_ARTIFACT_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedArtifactRoutes

//#region 🧵️RetainedConfigRoutes
const SEQUENCE_RETAINED_PAYLOAD_SCHEMA: &str = "sequence.play/retained-config-command.v1";
pub const SEQUENCE_RETAINED_RAW_BYTES: usize = 4_096;
pub const SEQUENCE_RETAINED_MAXIMUM_UNITS: usize = 2;
pub const SEQUENCE_RETAINED_CONFIG_TOOL_IDS: &[&str] = &["setViewport", "setOrientation", "stop"];
const SEQUENCE_RETAINED_CONFIG_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] = &[
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setViewport", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setOrientation", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowConfig] },
    semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "stop", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::WindowTransient] },
];

pub fn sequence_retained_config_command_admitted(command: &SequenceCommand) -> bool {
    match command {
        SequenceCommand::SetViewport(_) | SequenceCommand::Stop(_) => true,
        SequenceCommand::SetOrientation(payload) => payload.value.len() <= 32,
        _ => false,
    }
}

pub struct SequenceRetainedConfigWork {
    pub tool_id: &'static str,
    pub workspace_identity: u64,
    pub cursor: usize,
    pub replay_target: Option<usize>,
    pub completed: bool,
    pub closing: bool,
}

impl SequenceRetainedConfigWork {
    pub fn new(tool_id: &'static str, operation: &semio_framework_plugin::AppOperationContext) -> Self {
        let scope = format!("{}:{}:{}:{}", operation.app_instance_id, operation.parent_document_id, operation.operation_id, operation.generation);
        let workspace_identity = scope.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325_u64, |state, byte| (state ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3));
        Self { tool_id, workspace_identity, cursor: 0, replay_target: None, completed: false, closing: false }
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<SequencePlayApp>> for SequenceRetainedConfigWork {
    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<SequencePlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
    }
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn workspace_identity(&self) -> u64 {
        self.workspace_identity
    }

    fn extent(
        &self,
        _command: &SequenceCommand,
        _snapshot: &SequenceSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<SequencePlayApp>>>,
    ) -> Option<usize> {
        Some(SEQUENCE_RETAINED_MAXIMUM_UNITS)
    }

    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<SequencePlayApp>>,
    _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<SequencePlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot: _snapshot, config: _config, history: _history, interaction: _interaction, hover: _hover, context, operation: _operation } = *input;
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.completed || self.cursor >= SEQUENCE_RETAINED_MAXIMUM_UNITS || !sequence_retained_config_command_admitted(command) {
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("sequence.retained.config-command"), "Sequence retained config command exceeded its exact route or payload envelope"));
        }
        self.cursor += 1;
        if let Some(target) = self.replay_target {
            if self.cursor <= target {
                if self.cursor == target {
                    self.replay_target = None;
                }
                return Ok(ArtifactCommandWorkStep::Replay { stage: "sequence-config-replay", preview: b"{\"en\":\"Restoring Sequence setting\",\"de\":\"Sequenzeinstellung wird wiederhergestellt\"}" });
            }
        }
        if self.cursor == 1 {
            return Ok(ArtifactCommandWorkStep::Progress { stage: "sequence-config-prepare", preview: b"{\"en\":\"Preparing Sequence setting\",\"de\":\"Sequenzeinstellung wird vorbereitet\"}" });
        }
        let context = context.ok_or_else(|| sequence_fault("sequence.window.unavailable", "sequence-window-context-required"))?;
        let view = context.view_state.as_ref().ok_or_else(|| sequence_fault("sequence.window.unavailable", "sequence-window-view-required"))?;
        let base = main::config::from_snapshot(context.window_config.as_ref());
        let mut config = base.clone();
        self.completed = true;
        match command {
            SequenceCommand::SetViewport(payload) => {
                config.camera = payload.camera.clone();
                Ok(ArtifactCommandWorkStep::Complete(Emit { window_config_mutations: main::config::addressed(view, &base, config)?, ..Default::default() }))
            }
            SequenceCommand::SetOrientation(payload) => {
                config.orientation = payload.value.clone();
                Ok(ArtifactCommandWorkStep::Complete(Emit { window_config_mutations: main::config::addressed(view, &base, config)?, ..Default::default() }))
            }
            SequenceCommand::Stop(_) => Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                emit: Emit::default(),
                ephemeral: semio_framework_plugin::EphemeralEmit {
                    presence: Vec::new(),
                    transient: Vec::new(),
                    window_transient: vec![script::transient::addressed(view, SequenceScriptWindowTransient::default())?],
                },
            }),
            _ => Err(sequence_fault("app.command.tool-mismatch", "sequence-window-route-rejected")),
        }
    }

    fn checkpoint_byte(&self, index: usize) -> Option<u8> {
        let mut target = [0_u8; 24];
        self.write_checkpoint(&mut target).ok()?;
        target.get(index).copied()
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 24 || &checkpoint[..4] != b"SRC1" || checkpoint[4] > 1 || checkpoint[5..8] != [0, 0, 0] {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-invalid"));
        }
        let cursor = usize::try_from(u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-cursor"))?)).map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-cursor"))?;
        let identity = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-identity"))?);
        if identity != self.workspace_identity || cursor > SEQUENCE_RETAINED_MAXIMUM_UNITS {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-owner-mismatch"));
        }
        self.cursor = 0;
        self.replay_target = (cursor != 0).then_some(cursor);
        self.completed = false;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if grant.maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
        }
        self.replay_target = None;
        semio_framework_job::InteractiveJobCloseStep::Complete { progress: Default::default() }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.replay_target.is_none()
    }
}

pub struct SequenceRetainedConfigJobFactory {
    pub keys: Vec<semio_framework::ToolFactoryKey>,
}

impl SequenceRetainedConfigJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: SEQUENCE_RETAINED_CONFIG_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for SequenceRetainedConfigJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<SequencePlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<SequencePlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        SEQUENCE_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(SEQUENCE_RETAINED_RAW_BYTES, SEQUENCE_RETAINED_MAXIMUM_UNITS, 1, 4_096, 2_000, 1, 1)
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > SEQUENCE_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|value| value.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((semio_framework::ToolJobFactoryError::new("Sequence retained config command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for SequenceRetainedConfigJobFactory {
    type Owner = semio_framework_plugin::EditorApp<SequencePlayApp>;
    const TOOL_IDS: &'static [&'static str] = SEQUENCE_RETAINED_CONFIG_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SEQUENCE_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = SEQUENCE_RETAINED_CONFIG_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedConfigRoutes

//#region 🧵️RetainedExampleRoutes
/// 🎬️ `setActiveExample`'s own retained route. It is a FOURTH factory rather than a row on any of
/// the three above because those three route document, child and window-config mutations through
/// their work impls, while this verb emits a host-applied effect and nothing else — the
/// `🔱️trinity/🔌️jack` one-verb-factory shape. Its lane is therefore `HostOnly`: it publishes on no
/// store at all.
const SEQUENCE_RETAINED_EXAMPLE_PAYLOAD_SCHEMA: &str = "sequence.play/retained-example-command.v1";
const SEQUENCE_RETAINED_EXAMPLE_RAW_BYTES: usize = 4_096;
pub const SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_UNITS: usize = 2;
const SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_ID_BYTES: usize = 256;
pub const SEQUENCE_RETAINED_EXAMPLE_TOOL_IDS: &[&str] = &["setActiveExample"];
const SEQUENCE_RETAINED_EXAMPLE_PUBLICATION_CONTRACTS: &[semio_framework_plugin::ArtifactToolPublicationContract] =
    &[semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] }];

pub fn sequence_retained_example_command_admitted(command: &SequenceCommand) -> bool {
    match command {
        SequenceCommand::SetActiveExample(payload) => payload.example_id.len() <= SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_ID_BYTES,
        _ => false,
    }
}

pub struct SequenceRetainedExampleWork {
    pub tool_id: &'static str,
    pub workspace_identity: u64,
    pub cursor: usize,
    pub replay_target: Option<usize>,
    pub completed: bool,
    pub closing: bool,
}

impl SequenceRetainedExampleWork {
    pub fn new(tool_id: &'static str, operation: &semio_framework_plugin::AppOperationContext) -> Self {
        let scope = format!("{}:{}:{}:{}", operation.app_instance_id, operation.parent_document_id, operation.operation_id, operation.generation);
        let workspace_identity = scope.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325_u64, |state, byte| (state ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3));
        Self { tool_id, workspace_identity, cursor: 0, replay_target: None, completed: false, closing: false }
    }
}

impl semio_framework_plugin::retained_command::ArtifactCommandWork<semio_framework_plugin::EditorApp<SequencePlayApp>> for SequenceRetainedExampleWork {
    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<SequencePlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
    }
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn workspace_identity(&self) -> u64 {
        self.workspace_identity
    }

    /// 📏️ Answered without touching the snapshot or the working scene: loading an example is what
    /// makes a scene exist, so measuring this verb against one would refuse it at exactly the boot
    /// moment the playground navbar dispatches it.
    fn extent(
        &self,
        command: &SequenceCommand,
        _snapshot: &SequenceSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<semio_framework_plugin::EditorApp<SequencePlayApp>>>,
    ) -> Option<usize> {
        sequence_retained_example_command_admitted(command).then_some(SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_UNITS)
    }

    fn step(
        &mut self,
        input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, semio_framework_plugin::EditorApp<SequencePlayApp>>,
    _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<semio_framework_plugin::retained_command::ArtifactCommandWorkStep<semio_framework_plugin::EditorApp<SequencePlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, .. } = *input;
        use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
        if self.completed || self.cursor >= SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_UNITS || !sequence_retained_example_command_admitted(command) {
            return Err(Fault::new(
                semio_framework_plugin::FaultOrigin::App,
                semio_framework_plugin::FaultCode::new("sequence.retained.example-command"),
                "Sequence retained example command exceeded its exact route or payload envelope",
            ));
        }
        self.cursor += 1;
        if let Some(target) = self.replay_target {
            if self.cursor <= target {
                if self.cursor == target {
                    self.replay_target = None;
                }
                return Ok(ArtifactCommandWorkStep::Replay { stage: "sequence-example-replay", preview: b"{\"en\":\"Restoring Sequence example\",\"de\":\"Sequenzbeispiel wird wiederhergestellt\"}" });
            }
        }
        if self.cursor == 1 {
            return Ok(ArtifactCommandWorkStep::Progress { stage: "sequence-example-prepare", preview: b"{\"en\":\"Preparing Sequence example\",\"de\":\"Sequenzbeispiel wird vorbereitet\"}" });
        }
        self.completed = true;
        match command {
            SequenceCommand::SetActiveExample(payload) => set_active_example::emit(&payload.example_id).map(ArtifactCommandWorkStep::Complete),
            _ => Err(sequence_fault("app.command.tool-mismatch", "sequence-example-route-rejected")),
        }
    }

    fn checkpoint_byte(&self, index: usize) -> Option<u8> {
        let mut target = [0_u8; 24];
        self.write_checkpoint(&mut target).ok()?;
        target.get(index).copied()
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 24 || &checkpoint[..4] != b"SRE1" || checkpoint[4] > 1 || checkpoint[5..8] != [0, 0, 0] {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-invalid"));
        }
        let cursor = usize::try_from(u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-cursor"))?)).map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-cursor"))?;
        let identity = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-identity"))?);
        if identity != self.workspace_identity || cursor > SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_UNITS {
            return Err(sequence_fault("sequence.resume.invalid", "sequence-retained-checkpoint-owner-mismatch"));
        }
        self.cursor = 0;
        self.replay_target = (cursor != 0).then_some(cursor);
        self.completed = false;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> semio_framework_job::InteractiveJobCloseStep {
        if !self.closing {
            return semio_framework_job::InteractiveJobCloseStep::Blocked;
        }
        if grant.maximum_items == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { progress: Default::default() };
        }
        self.replay_target = None;
        semio_framework_job::InteractiveJobCloseStep::Complete { progress: Default::default() }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.replay_target.is_none()
    }
}

pub struct SequenceRetainedExampleJobFactory {
    pub keys: Vec<semio_framework::ToolFactoryKey>,
}

impl SequenceRetainedExampleJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: SEQUENCE_RETAINED_EXAMPLE_TOOL_IDS.iter().map(|tool_id| semio_framework::ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for SequenceRetainedExampleJobFactory {
    type Payload = semio_framework_plugin::retained_command::ArtifactRetainedCommandPayload<semio_framework_plugin::EditorApp<SequencePlayApp>>;
    type Job = semio_framework_plugin::retained_command::ArtifactRetainedCommandJob<semio_framework_plugin::EditorApp<SequencePlayApp>>;

    fn keys(&self) -> &[semio_framework::ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        SEQUENCE_RETAINED_EXAMPLE_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> semio_framework::ToolExecutionContract {
        semio_framework::ToolExecutionContract::resumable(SEQUENCE_RETAINED_EXAMPLE_RAW_BYTES, SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_UNITS, 1, 65_536, 7_500, 1, 1)
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, semio_framework::ToolJobFactoryError> {
        Ok(semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (semio_framework::ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > SEQUENCE_RETAINED_EXAMPLE_RAW_BYTES || checkpoint.as_ref().is_some_and(|value| value.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((semio_framework::ToolJobFactoryError::new("Sequence retained example command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => semio_framework_plugin::retained_command::ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for SequenceRetainedExampleJobFactory {
    type Owner = semio_framework_plugin::EditorApp<SequencePlayApp>;
    const TOOL_IDS: &'static [&'static str] = SEQUENCE_RETAINED_EXAMPLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = SEQUENCE_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = SEQUENCE_RETAINED_EXAMPLE_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedExampleRoutes
