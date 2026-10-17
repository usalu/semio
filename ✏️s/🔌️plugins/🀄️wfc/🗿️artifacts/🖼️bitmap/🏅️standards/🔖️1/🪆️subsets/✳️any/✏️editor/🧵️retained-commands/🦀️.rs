//! 🧵️ Retained command route of the WFC bitmap editor: the bounded publication contracts, the command work and the job factories. See [`super`].

use super::*;

//#region 🧵️RetainedCommands
/// 🧬️ The payload schema id every bitmap retained command job is admitted under.
const BITMAP_RETAINED_COMMAND_SCHEMA: &str = "s.wfc.bitmap/v1.tool-command.v1";

/// 🎒️ Wire ceiling for one bitmap tool dispatch. The largest payload is a `set-input-pixels` region
/// covering a whole 512-edge sample's row band; a full 128 × 128 repaint is 22 KiB of base64.
pub(super) const BITMAP_RETAINED_RAW_BYTES: usize = 65_536;

/// 📬️ Admission envelope for one published document edit — `set-input-pixels` publishes the widest
/// one, a whole re-laid-out sample buffer.
pub(super) const BITMAP_ARTIFACT_MUTATION_MAXIMUM_BYTES: usize = 262_144;

/// 🧮️ Durable items one dispatch may fold: every verb folds ONE row, the example picker none (it is a `LoadDocument`).
pub(super) const BITMAP_RETAINED_WORK_ITEMS: usize = 4_096;

/// 🚦️ Per-tool publication lanes, read straight off each command's own emit.
const BITMAP_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    artifact_route("change-seed"),
    artifact_route("resize-input"),
    artifact_route("set-input-pixels"),
    artifact_route("add-palette-color"),
    artifact_route("change-palette-color"),
    artifact_route("remove-palette-color"),
    artifact_route("resize-output"),
    artifact_route("change-model"),
    artifact_route("pin-pixel"),
    artifact_route("unpin-pixel"),
    window_config_route("set-active-color"),
    brush_route("paint-stroke"),
    ArtifactToolPublicationContract { tool_id: "solve", lanes: &[ArtifactToolPublicationLane::Transient] },
    ArtifactToolPublicationContract { tool_id: "commit-fill-solve", lanes: &[ArtifactToolPublicationLane::Transient] },
    host_only_route("setActiveExample"),
    artifact_route("pin-solution"),
    brush_route("canvasPointerDown"),
    brush_route("canvasPointerMove"),
    brush_route("canvasPointerUp"),
    host_only_route("canvasDoubleClick"),
    host_only_route("setCamera"),
];

const fn artifact_route(tool_id: &'static str) -> ArtifactToolPublicationContract {
    ArtifactToolPublicationContract { tool_id, lanes: &[ArtifactToolPublicationLane::Artifact] }
}

const fn window_config_route(tool_id: &'static str) -> ArtifactToolPublicationContract {
    ArtifactToolPublicationContract { tool_id, lanes: &[ArtifactToolPublicationLane::WindowConfig] }
}

const fn brush_route(tool_id: &'static str) -> ArtifactToolPublicationContract {
    ArtifactToolPublicationContract { tool_id, lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::WindowTransient] }
}

const fn host_only_route(tool_id: &'static str) -> ArtifactToolPublicationContract {
    ArtifactToolPublicationContract { tool_id, lanes: &[ArtifactToolPublicationLane::HostOnly] }
}

fn bitmap_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(BITMAP_RETAINED_RAW_BYTES, BITMAP_RETAINED_WORK_ITEMS, 1, BITMAP_ARTIFACT_MUTATION_MAXIMUM_BYTES, 7_500)
}

/// 🧵️ The one retained work every bitmap verb rides. `Solve` is the only arm that answers
/// `CompleteWithEphemeral`: it runs the artifact's own headless inference adapter and hands the
/// collapse to the transient lane the output window renders from.
pub(super) struct BitmapCommandWork {
    pub(super) tool_id: &'static str,
    pub(super) consumed: bool,
}

impl ArtifactCommandWork<EditorApp<BitmapEditor>> for BitmapCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(
        &self,
        command: &BitmapEditorCommand,
        _snapshot: &BitmapSnapshot,
        _interaction: &protocol::InteractionState,
        _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BitmapEditor>>>,
    ) -> Option<usize> {
        BITMAP_TOOL_IDS.contains(&bitmap_command_id(command)).then_some(1)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BitmapEditor>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BitmapEditor>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.consumed {
            return Err(Fault::from("wfc-bitmap-retained-work-repeated"));
        }
        self.consumed = true;
        if bitmap_command_id(input.command) != self.tool_id {
            return Err(Fault::from("wfc-bitmap-retained-route-mismatch"));
        }
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let cfg = ConfigView { snapshot: input.config, window: input.context.and_then(|context| context.window_config.as_ref()) };
        let view_state = input.context.and_then(|context| context.view_state.as_ref());
        let pointer = input.command.brush_pointer();
        if pointer.is_some() || matches!(input.command, BitmapEditorCommand::PaintStroke { .. }) {
            let detached = semio_framework_plugin::app::GestureSlot::detached();
            let slot = input.context.map_or(&detached, |context| context.gesture());
            let stroke = match &pointer {
                Some(pointer) => BitmapEditor::pointer_stroke(pointer, doc.snapshot, slot.open().is_some()),
                None => Some((input.command.clone(), false)),
            };
            let Some((stroke, interrupt)) = stroke else { return Ok(ArtifactCommandWorkStep::Complete(Emit::default())) };
            return Ok(ArtifactCommandWorkStep::Complete(BitmapEditor::paint_stroke(&stroke, &doc, &cfg, slot, interrupt)?));
        }
        let emit = BitmapEditor::dispatch(input.command, &doc, &cfg, view_state)?;
        match input.command {
            BitmapEditorCommand::Solve => Ok(ArtifactCommandWorkStep::Complete(Emit {
                effects: vec![fill_tool::start_fill_effect()],
                ui_scope: semio_framework::kernel::UiDirtyScope::Full,
                ..Default::default()
            })),
            BitmapEditorCommand::CommitFillSolve { pixels, contradiction, width, height } => Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                emit,
                ephemeral: EphemeralEmit {
                    transient: vec![BitmapTransientMutation::SetSolve(SetSolve {
                        output_pixels: if *contradiction || pixels.is_empty() { None } else { Some(pixels.clone()) },
                        contradiction: *contradiction,
                        output_width: *width,
                        output_height: *height,
                    })],
                    ..Default::default()
                },
            }),
            _ => Ok(ArtifactCommandWorkStep::Complete(emit)),
        }
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

/// 🏭️ The app-owned bounded tool-job factory. `qualified_tool_proof` refuses every typed command
/// that resolves to anything else, so this registration is what makes the whole roster reachable.
pub struct BitmapCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl BitmapCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: BITMAP_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }

    pub(super) fn register(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<BitmapEditor>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(Self::new(&controller))
    }
}

impl semio_framework::ToolJobFactory for BitmapCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<BitmapEditor>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<BitmapEditor>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        BITMAP_RETAINED_COMMAND_SCHEMA
    }
    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        bitmap_retained_contract()
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
        if input.declared_bytes() > BITMAP_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("bounded wfc bitmap command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for BitmapCommandJobFactory {
    type Owner = EditorApp<BitmapEditor>;
    const TOOL_IDS: &'static [&'static str] = BITMAP_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = WFC_BITMAP_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = BITMAP_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands
