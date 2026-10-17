//! 🧵️ Retained command tools of the CAD play app: the tool ids, publication contracts, retained reduce and the command job factory. See [`super`].

use super::*;
use semio_s_artifact_cad_cad_kit::CAD_INTERACTION_DOMAIN;
use semio_s_artifact_cad_cad_kit::CadInteractionSnapshot;
use semio_s_artifact_cad_cad_modes::windows::config::addressed;

//#region 🔖️PlayApp
//#region 🧵️RetainedCommands
// 🤝️ `engagementSubmit`/`engagementPossibleSelect`/`worldPointerDown` are admitted against the document like every object
// gesture: an interaction step that reaches its commit state lands child leaves on the panes' model children (Child)
// besides the addressed window's engagement transient (WindowTransient, design §17.4, §20.15).
const CAD_RETAINED_ARTIFACT_TOOL_IDS: &[&str] = &["addNode", "renameNode", "patchCadPlayReference", "setReferenceHidden", "setReferenceLocked", "addObject", "patchObject", "patchSelection", "deleteObject", "duplicateObject", "translateSelection", "rotateSelection", "scaleSelection", "engagementSubmit", "engagementPossibleSelect", "worldPointerDown", "applyTransformation"];

const CAD_RETAINED_CONFIG_TOOL_IDS: &[&str] = &[
    "setCamera",
    "setProjection",
    "setProjectionParam",
    "setDislocateOption",
    "setNodeSelection",
    "setReferenceSelection",
    "referenceHover",
    "toggleSun",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setContributions",
    "setActiveExample",
    "importCadFile",
];

/// 🫧️ The engagement steps that commit nothing: they publish only the addressed window's transient (design §17.4).
const CAD_RETAINED_ENGAGEMENT_TOOL_IDS: &[&str] = &["engagementInput", "engagementRepeatLast", "engagementAbort", "worldPointerMove"];

/// 📤️ The exports read the scene and hand the host a download; they touch no store.
const CAD_RETAINED_EXPORT_TOOL_IDS: &[&str] = &["saveSelected", "saveInPlay", "saveCurrent"];

pub(super) const CAD_RETAINED_TOOL_IDS: &[&str] = &[
    "addNode",
    "renameNode",
    "patchCadPlayReference",
    "setReferenceHidden",
    "setReferenceLocked",
    "addObject",
    "patchObject",
    "patchSelection",
    "deleteObject",
    "duplicateObject",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "engagementSubmit",
    "worldPointerDown",
    "setCamera",
    "setProjection",
    "setProjectionParam",
    "setDislocateOption",
    "setNodeSelection",
    "setReferenceSelection",
    "referenceHover",
    "engagementInput",
    "engagementPossibleSelect",
    "engagementRepeatLast",
    "engagementAbort",
    "worldPointerMove",
    "toggleSun",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setContributions",
    "setActiveExample",
    "loadRawRequest",
    "applyTransformation",
    "importCadFile",
    "saveSelected",
    "saveInPlay",
    "saveCurrent",
];

const CAD_RETAINED_COMMAND_SCHEMA: &str = "cad.scene.tool-command.v1";

pub(super) const CAD_RETAINED_RAW_BYTES: usize = 8_192;

pub(super) const CAD_RETAINED_WORK_ITEMS: usize = 1;

pub(super) const CAD_CONFIG_STORE_MAXIMUM_BYTES: usize = 65_536;

pub(super) const CAD_CONFIG_STORE_MAXIMUM_ITEMS: usize = 256;

const CAD_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "addNode", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "renameNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchCadPlayReference", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setReferenceHidden", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setReferenceLocked", lanes: &[ArtifactToolPublicationLane::Artifact] },
    // 🪆️ Every object gesture publishes on the Child lane alone: its leaves land in the addressed panes' composed
    // `s.stdio.semio@v1/model` children, never in the parent document (design §20.15).
    ArtifactToolPublicationContract { tool_id: "addObject", lanes: &[ArtifactToolPublicationLane::Child] },
    ArtifactToolPublicationContract { tool_id: "patchObject", lanes: &[ArtifactToolPublicationLane::Child] },
    ArtifactToolPublicationContract { tool_id: "patchSelection", lanes: &[ArtifactToolPublicationLane::Child] },
    ArtifactToolPublicationContract { tool_id: "deleteObject", lanes: &[ArtifactToolPublicationLane::Child] },
    ArtifactToolPublicationContract { tool_id: "duplicateObject", lanes: &[ArtifactToolPublicationLane::Child] },
    ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[ArtifactToolPublicationLane::Child] },
    ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[ArtifactToolPublicationLane::Child] },
    ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[ArtifactToolPublicationLane::Child] },
    // 🤝️ Interaction steps: the window's engagement transient (WindowTransient) every step, plus the committed
    // objects' child leaves (Child) on the step that reaches the spec's commit state — never a config edit (design §17.4).
    ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[ArtifactToolPublicationLane::Child, ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "worldPointerDown", lanes: &[ArtifactToolPublicationLane::Child, ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setProjection", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setProjectionParam", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setDislocateOption", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setNodeSelection", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setReferenceSelection", lanes: &[ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::Interaction] },
    ArtifactToolPublicationContract { tool_id: "referenceHover", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "engagementPossibleSelect", lanes: &[ArtifactToolPublicationLane::Child, ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "engagementRepeatLast", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "engagementAbort", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "worldPointerMove", lanes: &[ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "toggleSun", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setSunAzimuth", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setSunElevation", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setSunIntensity", lanes: &[ArtifactToolPublicationLane::WindowConfig] },
    ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[ArtifactToolPublicationLane::Config] },
    // 🗃️ An example switch records the active example id in the session config and hands the host
    // ONE whole-document `LoadDocument` effect, which the store admits through
    // `build_document_store_initialization_job` — never an in-history artifact edit.
    ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[ArtifactToolPublicationLane::Config, ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "loadRawRequest", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "applyTransformation", lanes: &[ArtifactToolPublicationLane::Child] },
    // 🗃️ A whole spatial scene imports like an example switch: the window's reset engagement plus ONE host `LoadDocument`;
    // a single object file lands as ONE `insert-element` transaction on the addressed pane's model child (Child).
    ArtifactToolPublicationContract { tool_id: "importCadFile", lanes: &[ArtifactToolPublicationLane::Child, ArtifactToolPublicationLane::WindowTransient] },
    ArtifactToolPublicationContract { tool_id: "saveSelected", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "saveInPlay", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "saveCurrent", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];

/// 🧾️ The ONE execution contract every retained cad tool is admitted under. Its wire ceiling is
/// [`semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES`] because ONE of those tools —
/// `setContributions` — carries a host pack, not a gesture, and a factory declares one contract for
/// every tool it serves (`ArtifactOwnedToolJobFactory::execution_contract`), which the proof
/// catalogue must join exactly. Widening the ADMISSION does not widen what any gesture may actually
/// send: [`cad_retained_raw_bytes`] is what the payload reserves and what the wire factory refuses
/// past, per tool id.
pub(super) fn cad_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES, 64, CAD_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

/// 📏️ The raw-wire ceiling one retained tool id is admitted against — the twin of its registered
/// contract, which the wire factory and the payload builder must both honour or an admitted pack
/// is refused one layer deeper.
pub(super) fn cad_retained_raw_bytes(tool_id: &str) -> usize {
    if tool_id == "setContributions" || tool_id == "importCadFile" {
        semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES
    } else {
        CAD_RETAINED_RAW_BYTES
    }
}

pub(super) fn cad_retained_extent(command: &CadCommand, _snapshot: &CadSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    CAD_RETAINED_TOOL_IDS.contains(&command.command_id()).then_some(1)
}

/// 🧵️ One retained cad command over the operation's observed roots: the command's reducer over the dispatch context
/// the operation carries — the panes' composed model children the object writers decide against, the addressed
/// window's engagement transient in and its next state out (design §17.4, §20.15).
fn cad_retained_reduce(input: &ArtifactCommandInputs<'_, EditorApp<CadPlayApp>>) -> Result<(Emit<CadMutation, CadConfigMutation, NoDraftMutation>, Option<CadWorldWindowTransient>), Fault> {
    let ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config, history, interaction, hover, context, operation } = *input;
    let doc = match context {
        Some(context) => ArtifactView::with_children(snapshot, history, (*context.children).clone()).bound_to_operation(operation.clone()),
        None => ArtifactView::with_operation(snapshot, history, operation.clone()),
    };
    let cfg = ConfigView { snapshot: config, window: context.and_then(|context| context.window_config.as_ref()) };
    let selection = interaction.selection.get(CAD_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    let hovered_ids = hover.get(CAD_INTERACTION_DOMAIN).filter(|hover| hover.channel == CadInteractionSnapshot::POINTER_CHANNEL).map(|hover| hover.ids.clone()).unwrap_or_default();
    let retained_interaction = CadInteractionSnapshot { granularity: selection.granularity.clone(), ids: selection.ids.clone(), anchor_id: selection.anchor_id, hovered_ids };
    let mut ctx = CadDispatchCtx::new(retained_interaction, context.and_then(|context| context.view_state.clone()), window_transient::from_snapshot(context.and_then(|context| context.window_transient.as_ref())));
    let command_id = command.command_id();
    let emit = if CAD_RETAINED_ARTIFACT_TOOL_IDS.contains(&command_id) || CAD_RETAINED_EXPORT_TOOL_IDS.contains(&command_id) {
        admit_cad_snapshot(snapshot).map_err(Fault::from)?;
        command.dispatch(&doc, &cfg, &mut ctx)?
    } else if CAD_RETAINED_CONFIG_TOOL_IDS.contains(&command_id) {
        admit_cad_config(config).map_err(Fault::from)?;
        command.dispatch(&doc, &cfg, &mut ctx)?
    } else if CAD_RETAINED_ENGAGEMENT_TOOL_IDS.contains(&command_id) {
        command.dispatch(&doc, &cfg, &mut ctx)?
    } else {
        match command {
            CadCommand::LoadRawRequest(payload) => load_raw_request::handle(payload, &doc, &cfg, &mut ctx)?,
            _ => return Err(Fault::from("cad-retained-route-mismatch")),
        }
    };
    Ok((emit, ctx.next_window_transient))
}

/// 🧵️ The ONE retained work every cad tool runs ([`cad_retained_reduce`]): it completes with the addressed window's
/// next engagement transient as ephemeral local-only window state when the command changed it — a dispatch that
/// addresses no window (an agent, the palette) keeps none.
pub(super) struct CadRetainedCommandWork {
    pub(super) tool_id: &'static str,
    pub(super) consumed: bool,
}

impl ArtifactCommandWork<EditorApp<CadPlayApp>> for CadRetainedCommandWork {
    fn work_demands(&self, _input: &ArtifactCommandInputs<'_, EditorApp<CadPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand { copy_bytes: size_of::<Self>(), depth: 1, ..Default::default() })
    }
    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &CadCommand, snapshot: &CadSnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<CadPlayApp>>>) -> Option<usize> {
        cad_retained_extent(command, snapshot, interaction)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<CadPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<CadPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        if self.consumed {
            return Err(Fault::from("cad-retained-work-repeated"));
        }
        self.consumed = true;
        let (emit, transient) = cad_retained_reduce(input)?;
        let view = input.context.and_then(|context| context.view_state.as_ref()).filter(|view| view.window_id.is_some());
        Ok(match (transient, view) {
            (Some(transient), Some(view)) => ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: semio_framework_plugin::EphemeralEmit { window_transient: vec![window_transient::addressed(view, transient)?], ..Default::default() } },
            _ => ArtifactCommandWorkStep::Complete(emit),
        })
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }
}

pub(super) struct CadRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl CadRetainedCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: CAD_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for CadRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<CadPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<CadPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        CAD_RETAINED_COMMAND_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        cad_retained_contract()
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
        if input.declared_bytes() > payload.maximum_raw_bytes || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("CAD retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for CadRetainedCommandJobFactory {
    type Owner = EditorApp<CadPlayApp>;
    const TOOL_IDS: &'static [&'static str] = CAD_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = CAD_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = CAD_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands
//#endregion 🔖️PlayApp
