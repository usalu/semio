//! 🕹️ Command surface of the VCS play app: the aggregated command enum, the app type, the bounded and resumable publication contracts and the retained tool job factories. See [`super`].

use super::*;

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `VcsPlayApp::Command` — the SOLE dispatch surface for the vcs demo app's own behavior. The six
    /// history actions (undo/redo/commitCheckpoint/createAlternative/switchAlternative/checkoutCheckpoint)
    /// never reach here — `VcsArtifactApp` intercepts those itself as host mechanics, not app behavior
    /// (see `shooting_protocol::ShootingCommand`'s identical doc). Field shapes mirror each action's old
    /// JSON `args` object exactly. **Row order is the binary variant ordinal: appending is safe,
    /// reordering is a wire-format break.**
    pub enum VcsCommand for VcsSnapshot, VcsDemoMutation, VcsDemoConfig, VcsDemoConfigMutation {
        "incrementCounter" as "increment-counter" => increment_counter::IncrementCounter,
        "renameVcs" as "rename-vcs" => rename_vcs::RenameVcs,
        "changeCounter" as "change-counter" => change_counter::ChangeCounter,
        "changeStatus" as "change-status" => change_status::ChangeStatus,
        "changeNotes" as "change-notes" => change_notes::ChangeNotes,
        "textEdit" as "text-edit" => text_edit::TextEdit,
        "edit" as "edit" => edit_command::Edit,
        "noMutation" as "no-operation" => no_operation::NoMutation,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "canvasWheel" as "canvas-wheel" => canvas_wheel::CanvasWheel,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
    }
}
//#endregion 🔖️Commands

//#region 🔖️VcsPlayApp
/// 🧪️ B1: unit struct — the former `VcsPlayApp::selected_checkpoint_ids` `RefCell` field passed through
/// `crate::editor::vcs::config::VcsDemoConfig` before becoming the framework-owned "history" interaction
/// domain (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM, see `VCS_INTERACTION_HISTORY`'s
/// doc comment); the config no longer duplicates host-owned view preferences.
#[derive(Default)]
pub struct VcsPlayApp;

//#region 🧵️RetainedCommands
pub const VCS_BOUNDED_TOOL_IDS: &[&str] = &["incrementCounter", "renameVcs", "changeCounter", "changeStatus", "changeNotes", "noMutation", "canvasPointerDown", "canvasPointerMove", "canvasPointerUp", "canvasWheel", "setActiveExample"];

pub const VCS_RESUMABLE_TOOL_IDS: &[&str] = &["textEdit", "edit"];

const VCS_BOUNDED_PAYLOAD_SCHEMA: &str = "vcs.vcs.tool-command.v1";

pub const VCS_BOUNDED_RAW_BYTES: usize = 8_192;

pub const VCS_BOUNDED_WORK_ITEMS: usize = 1;

pub const VCS_EDIT_MAXIMUM_TAGS: usize = 4_096;

pub const VCS_EDIT_MAXIMUM_OUTPUT_BYTES: usize = 16_384;

pub const VCS_EDIT_MAXIMUM_WORK_ITEMS: usize = 16_400;

pub const VCS_BOUNDED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "incrementCounter", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "renameVcs", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "changeCounter", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "changeStatus", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "changeNotes", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "noMutation", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasPointerUp", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "canvasWheel", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];

pub const VCS_RESUMABLE_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] =
    &[ArtifactToolPublicationContract { tool_id: "textEdit", lanes: &[ArtifactToolPublicationLane::Artifact] }, ArtifactToolPublicationContract { tool_id: "edit", lanes: &[ArtifactToolPublicationLane::Artifact] }];

fn vcs_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(VCS_BOUNDED_RAW_BYTES, 32, 32, 16_384, 7_500)
}

fn vcs_resumable_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(VCS_BOUNDED_RAW_BYTES, VCS_EDIT_MAXIMUM_WORK_ITEMS, 1, VCS_EDIT_MAXIMUM_OUTPUT_BYTES, 7_500, 1, 1)
}

pub fn vcs_bounded_extent(command: &VcsCommand, _snapshot: &VcsSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let bytes = match command {
        VcsCommand::IncrementCounter(_) | VcsCommand::NoMutation(_) | VcsCommand::CanvasPointerDown(_) | VcsCommand::CanvasPointerUp(_) | VcsCommand::CanvasWheel(_) => 0,
        // 🧵️ A batched move carries `samples.len()` pairs of f64 (design L4) — priced, never dropped.
        VcsCommand::CanvasPointerMove(payload) => payload.samples.len().checked_mul(2 * size_of::<f64>())?,
        VcsCommand::RenameVcs(payload) => payload.title.len(),
        VcsCommand::ChangeCounter(_) => 0,
        VcsCommand::ChangeStatus(payload) => payload.status.len(),
        VcsCommand::ChangeNotes(payload) => payload.notes.len(),
        VcsCommand::SetActiveExample(payload) if payload.example_id.len() <= 256 => 0,
        VcsCommand::SetActiveExample(_) => return None,
        VcsCommand::TextEdit(_) | VcsCommand::Edit(_) => return None,
    };
    (bytes <= VCS_BOUNDED_RAW_BYTES).then_some(VCS_BOUNDED_WORK_ITEMS)
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
pub fn vcs_bounded_reduce(
    command: &VcsCommand,
    snapshot: &VcsSnapshot,
    config: &VcsDemoConfig,
    history: &semio_framework_plugin::HistoryView,
    _interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<VcsPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation, NoDraftMutation>, Fault> {
    command.dispatch(&ArtifactView::with_operation(snapshot, history, operation.clone()), &ConfigView { snapshot: config, window: None })
}

pub fn vcs_edit_text(command: &VcsCommand) -> Option<&str> {
    match command {
        VcsCommand::TextEdit(payload) => Some(&payload.text),
        VcsCommand::Edit(payload) => Some(&payload.text),
        _ => None,
    }
}

pub fn vcs_edit_extent(command: &VcsCommand, snapshot: &VcsSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    let text = vcs_edit_text(command)?;
    (text.len() <= VCS_BOUNDED_RAW_BYTES && snapshot.tags.len() <= VCS_EDIT_MAXIMUM_TAGS).then_some(VCS_EDIT_MAXIMUM_WORK_ITEMS)
}

pub struct VcsBoundedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl VcsBoundedCommandJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: VCS_BOUNDED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for VcsBoundedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<VcsPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<VcsPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        VCS_BOUNDED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        vcs_bounded_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > VCS_BOUNDED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("VCS bounded command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for VcsBoundedCommandJobFactory {
    type Owner = EditorApp<VcsPlayApp>;
    const TOOL_IDS: &'static [&'static str] = VCS_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = VCS_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = VCS_BOUNDED_PUBLICATION_CONTRACTS;
}

pub struct VcsResumableCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl VcsResumableCommandJobFactory {
    pub fn new(controller_id: &str) -> Self {
        Self { keys: VCS_RESUMABLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for VcsResumableCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<VcsPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<VcsPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        VCS_BOUNDED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        vcs_resumable_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework_action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework_action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework_action_bus::RetainedToolWireInput, Option<semio_framework_action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > VCS_BOUNDED_RAW_BYTES || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("VCS resumable command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for VcsResumableCommandJobFactory {
    type Owner = EditorApp<VcsPlayApp>;
    const TOOL_IDS: &'static [&'static str] = VCS_RESUMABLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = VCS_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = VCS_RESUMABLE_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️VcsPlayApp
