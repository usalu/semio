//! 🧵️ Process3d retained command contracts, dispositions, bounded and resumable job factories.

use super::*;
use semio_s_artifact_process_process3d_kit::Process3dDispatchCtx;
use semio_s_artifact_process_process3d_kit::Process3dInteractionSnapshot;

//#region 🧵️RetainedCommands
/// 🧵️ Every UI-reachable command that reduces in one bounded first step. A command left off this list
/// (and off `PROCESS3D_RESUMABLE_TOOL_IDS`) is unreachable at runtime, not merely untested:
/// `validate_ui_dispatch_classification` rejects any dispatch whose registry classification is not
/// `Migrated`, and `qualified_tool_proof` refuses a typed command that owns no tool proof — so the
/// twenty-two ids that used to sit in a batch-only pending-rewrite list (the whole step timeline, the
/// stock, the workshop, the cursor, the engagement line, the world pointer and both media round-trips)
/// could not be invoked from the browser at all.
const PROCESS3D_BOUNDED_TOOL_IDS: &[&str] = &[
    "engagementAbort",
    "setCamera",
    "loadModelRequest",
    "setSnapshot",
    "setActiveExample",
    "addStep",
    "addWorkshopMachine",
    "removeWorkshopMachine",
    "updateWorkshopMachine",
    "removeStep",
    "removeSelectedStep",
    "moveStep",
    "updateStep",
    "setStepEnabled",
    "setStock",
    "patchInspector",
    "engagementSubmit",
    "worldPointerDown",
    "worldFaceDragEnd",
    "importModelFile",
    "exportModel",
    "setCursor",
    "stepCursor",
    "stepCursorBack",
    "stepCursorForward",
];
const PROCESS3D_RESUMABLE_TOOL_IDS: &[&str] = &["engagementInput", "toggleSun", "setSunAzimuth", "setSunElevation", "setSunIntensity", "setContributions"];
const PROCESS3D_RETAINED_PAYLOAD_SCHEMA: &str = "process.3d.tool-command.v1";
pub(super) const PROCESS3D_RETAINED_RAW_BYTES: usize = 8_192;
const PROCESS3D_RETAINED_WORK_ITEMS: usize = 64;
const PROCESS3D_SCAN_BYTES: usize = 256;
/// 📏️ Everything the retained config holds APART from the contributions pack — engagement text and
/// the sun colour. Unchanged; the contributions lane is priced separately below.
const PROCESS3D_CONFIG_FILTER_STORE_BYTES: usize = 16_384;

pub(super) const PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES: usize = PROCESS3D_CONFIG_FILTER_STORE_BYTES + PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES;
/// 📏️ What op-encoding one config mutation adds around its string field: the `OpBinary` format
/// byte, the variant-ordinal varint and the record body's field framing/length prefix.
const PROCESS3D_CONFIG_MUTATION_ENVELOPE_BYTES: usize = 64;
/// 📏️ The exact output cap of the resumable (config-lane) routes. Their largest output is ONE
/// `Process3dConfigMutation::SetContributions` whose `json` [`installable_contributions`] bounds to
/// [`PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES`], so the cap is that lane plus the op envelope.
///
/// 🏁️ It used to be the 16 KiB gesture output every bounded route shares, while the contributions
/// lane beside it admits 24 KiB: the real four-extension roster (19 823 B) distilled fine, fit the
/// config store, and was then refused at the host-configuration gate with `host configuration
/// command 'setContributions' exceeds its exact output cap` — so the app never installed its
/// machine catalogs (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP). The input cap already
/// named the pack's own bound; the output cap now names the lane's.
pub(crate) const PROCESS3D_RESUMABLE_OUTPUT_BYTES: usize = PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES + PROCESS3D_CONFIG_MUTATION_ENVELOPE_BYTES;

/// 📏️ The raw-wire ceiling one retained tool id is admitted against. `setContributions` is the ONE
/// retained command whose wire is a host pack, not a gesture — it carries the whole capability
/// closure the receiver's registry `consumes` row names — so it is priced on
/// [`semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES`], never on the gesture envelope.
pub(super) fn process3d_retained_raw_bytes(tool_id: &str) -> usize {
    if tool_id == "setContributions" {
        semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES
    } else {
        PROCESS3D_RETAINED_RAW_BYTES
    }
}

/// 🧮️ Work units one retained tool id may declare. The resumable pump scans `PROCESS3D_SCAN_BYTES`
/// per unit, so a tool's unit cap and its wire cap are one bound: a `setContributions` pack of
/// [`semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES`] is that many scans, and pricing
/// it on the gesture cap refused the real pack before it ever reached the config lane.
pub(super) fn process3d_retained_work_items(tool_id: &str) -> usize {
    process3d_retained_raw_bytes(tool_id).div_ceil(PROCESS3D_SCAN_BYTES).max(PROCESS3D_RETAINED_WORK_ITEMS)
}

/// 🛤️ Which retained work shape a tool id reduces through — the two are exhaustive over this app's
/// whole action surface, so an id that answers `None` here is genuinely not a command of this app.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Process3dCommandDisposition {
    Bounded,
    Config,
}

pub(super) fn process3d_command_disposition(tool_id: &str) -> Option<Process3dCommandDisposition> {
    if PROCESS3D_BOUNDED_TOOL_IDS.contains(&tool_id) {
        return Some(Process3dCommandDisposition::Bounded);
    }
    PROCESS3D_RESUMABLE_TOOL_IDS.contains(&tool_id).then_some(Process3dCommandDisposition::Config)
}

fn process3d_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(PROCESS3D_RETAINED_RAW_BYTES, PROCESS3D_RETAINED_WORK_ITEMS, 1, 16_384, 7_500)
}

/// 🧾️ The ONE execution contract every RESUMABLE process3d tool is admitted under. Its wire ceiling
/// is [`semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES`] because one of those tools —
/// `setContributions` — carries a host pack, not a gesture, and a factory declares one contract for
/// every tool it serves, which the proof catalogue must join exactly. Widening the ADMISSION does
/// not widen what a sun or engagement gesture may send: [`process3d_retained_raw_bytes`] is what
/// `process3d_resumable_extent` and the payload both bound, per tool id. Its output cap is
/// [`PROCESS3D_RESUMABLE_OUTPUT_BYTES`] for the same reason: the one config mutation that route emits.
pub(super) fn process3d_resumable_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES, 64, 1, PROCESS3D_RESUMABLE_OUTPUT_BYTES, 7_500, 1, 1)
}

fn process3d_string_units(value: &str) -> usize {
    value.len().div_ceil(PROCESS3D_SCAN_BYTES).max(1)
}

pub(super) fn process3d_resumable_extent(command: &Process3dCommand, _snapshot: &Process3dSnapshot, config: &Process3dConfig, _interaction: &protocol::InteractionState) -> Option<usize> {
    let value = match command {
        Process3dCommand::EngagementInput(payload) => payload.value.as_str(),
        Process3dCommand::SetContributions(payload) => payload.json.as_str(),
        Process3dCommand::ToggleSun(_) | Process3dCommand::SetSunAzimuth(_) | Process3dCommand::SetSunElevation(_) | Process3dCommand::SetSunIntensity(_) => config.sun_color.as_str(),
        _ => return None,
    };
    let extent = process3d_string_units(value);
    (extent <= process3d_retained_work_items(command.command_id()) && value.len() <= process3d_retained_raw_bytes(command.command_id())).then_some(extent)
}

#[expect(clippy::too_many_arguments, reason = "Implements the framework ArtifactCommandReducer callback signature.")]
pub(super) fn process3d_retained_reduce(
    command: &Process3dCommand,
    snapshot: &Process3dSnapshot,
    config: &Process3dConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Process3dPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<Process3dMutation, Process3dConfigMutation, NoDraftMutation>, Fault> {
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window: None };
    let selection = interaction.selection.get(PROCESS3D_INTERACTION_DOMAIN);
    let mut ctx = Process3dDispatchCtx { interaction: Process3dInteractionSnapshot { ids: selection.map(|selection| selection.ids.clone()).unwrap_or_default() }, view_state: context.and_then(|context| context.view_state.clone()) };
    command.dispatch(&doc, &cfg, &mut ctx)
}

fn process3d_resumable_bytes<'a>(command: &'a Process3dCommand, config: &'a Process3dConfig) -> Option<&'a [u8]> {
    match command {
        Process3dCommand::EngagementInput(payload) => Some(payload.value.as_bytes()),
        Process3dCommand::SetContributions(payload) => Some(payload.json.as_bytes()),
        Process3dCommand::ToggleSun(_) | Process3dCommand::SetSunAzimuth(_) | Process3dCommand::SetSunElevation(_) | Process3dCommand::SetSunIntensity(_) => Some(config.sun_color.as_bytes()),
        _ => None,
    }
}

fn process3d_tool_identity(tool_id: &str) -> u64 {
    tool_id.as_bytes().iter().fold(0xcbf2_9ce4_8422_2325, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x1000_0000_01b3))
}

pub(super) struct Process3dResumableCommandWork {
    pub(super) tool_id: &'static str,
    extent: usize,
    pub(super) cursor: usize,
    pub(super) digest: u64,
    complete: bool,
    closing: bool,
}

impl Process3dResumableCommandWork {
    pub(super) fn new(tool_id: &'static str, extent: usize) -> Self {
        Self { tool_id, extent, cursor: 0, digest: 0xcbf2_9ce4_8422_2325, complete: false, closing: false }
    }

    fn observe_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.digest = (self.digest ^ u64::from(*byte)).wrapping_mul(0x1000_0000_01b3);
        }
    }

    fn observe_input(&mut self, command: &Process3dCommand, config: &Process3dConfig) -> Result<(), Fault> {
        let bytes = process3d_resumable_bytes(command, config).ok_or_else(|| Fault::from("process3d-retained-route-not-resumable"))?;
        let start = self.cursor.checked_mul(PROCESS3D_SCAN_BYTES).ok_or_else(|| Fault::from("process3d-retained-cursor-overflow"))?.min(bytes.len());
        let end = start.saturating_add(PROCESS3D_SCAN_BYTES).min(bytes.len());
        self.observe_bytes(&bytes[start..end]);
        Ok(())
    }

    fn complete_emit(&self, command: &Process3dCommand, config: &Process3dConfig) -> Result<Emit<Process3dMutation, Process3dConfigMutation, NoDraftMutation>, Fault> {
        let mutation = match command {
            Process3dCommand::EngagementInput(payload) => Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ value: payload.value.clone() }),
            Process3dCommand::ToggleSun(_) => Process3dConfigMutation::SetSun(Process3dConfigSetSun{ enabled: !config.sun_enabled, azimuth: config.sun_azimuth, elevation: config.sun_elevation, intensity: config.sun_intensity, color: config.sun_color.clone() }),
            Process3dCommand::SetSunAzimuth(payload) => Process3dConfigMutation::SetSun(Process3dConfigSetSun{ enabled: config.sun_enabled, azimuth: payload.value, elevation: config.sun_elevation, intensity: config.sun_intensity, color: config.sun_color.clone() }),
            Process3dCommand::SetSunElevation(payload) => Process3dConfigMutation::SetSun(Process3dConfigSetSun{ enabled: config.sun_enabled, azimuth: config.sun_azimuth, elevation: payload.value, intensity: config.sun_intensity, color: config.sun_color.clone() }),
            Process3dCommand::SetSunIntensity(payload) => Process3dConfigMutation::SetSun(Process3dConfigSetSun{ enabled: config.sun_enabled, azimuth: config.sun_azimuth, elevation: config.sun_elevation, intensity: payload.value, color: config.sun_color.clone() }),
            Process3dCommand::SetContributions(payload) => Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ json: payload.json.clone() }),
            _ => return Err(Fault::from("process3d-retained-route-not-resumable")),
        };
        Ok(Emit::config(vec![mutation]))
    }
}

impl ArtifactCommandWork<EditorApp<Process3dPlayApp>> for Process3dResumableCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &Process3dCommand, snapshot: &Process3dSnapshot, interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<Process3dPlayApp>>>) -> Option<usize> {
        let _ = (command, snapshot, interaction);
        Some(self.extent)
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Process3dPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<Process3dPlayApp>>, semio_framework_job::OriginalTurnFailure> {
        let original: Result<_, Fault> = (|| {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot: _snapshot, config, history: _history, interaction: _interaction, hover: _hover, context: _context, operation: _operation } = *input;
        if self.complete {
            return Err(Fault::from("process3d-retained-work-repeated"));
        }
        if self.cursor > self.extent {
            return Err(Fault::from("process3d-retained-checkpoint-cursor-out-of-range"));
        }
        if self.cursor < self.extent {
            self.observe_input(command, config)?;
            self.cursor += 1;
            return Ok(ArtifactCommandWorkStep::Progress { stage: "process3d-config-prepare", preview: b"{\"en\":\"Preparing configuration\",\"de\":\"Konfiguration wird vorbereitet\"}" });
        }
        self.complete = true;
        self.complete_emit(command, config).map(ArtifactCommandWorkStep::Complete)
    
        })();
        original.map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn checkpoint_byte(&self,index:usize)->Option<u8>{match index{0..=3=>Some(b"P3C1"[index]),4=>Some(u8::from(self.complete)),5..=7=>Some(0),8..=15=>Some(((self.cursor as u64)>>((index-8)*8))as u8),16..=23=>Some((self.digest>>((index-16)*8))as u8),24..=31=>Some((process3d_tool_identity(self.tool_id)>>((index-24)*8))as u8),32..=39=>Some(((self.extent as u64)>>((index-32)*8))as u8),_=>None}}

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 40 || &checkpoint[..4] != b"P3C1" || checkpoint[4] > 1 || checkpoint[5] != 0 {
            return Err(Fault::from("process3d-retained-checkpoint-invalid"));
        }
        let identity = u64::from_le_bytes(checkpoint[24..32].try_into().map_err(|_| Fault::from("process3d-retained-checkpoint-identity"))?);
        if identity != process3d_tool_identity(self.tool_id) {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.tool-mismatch"), "Process 3D checkpoint belongs to another registered tool"));
        }
        let extent = u64::from_le_bytes(checkpoint[32..40].try_into().map_err(|_| Fault::from("process3d-retained-checkpoint-extent"))?);
        if extent != self.extent as u64 {
            return Err(Fault::from("process3d-retained-checkpoint-extent-mismatch"));
        }
        let cursor = u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| Fault::from("process3d-retained-checkpoint-cursor"))?);
        if cursor > usize::MAX as u64 {
            return Err(Fault::from("process3d-retained-checkpoint-cursor"));
        }
        self.cursor = cursor as usize;
        self.digest = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| Fault::from("process3d-retained-checkpoint-digest"))?);
        self.complete = checkpoint[4] == 1;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{
        semio_framework_job::close_original_granted_step(cx,|_grant|{
            Ok(if self.closing {
                InteractiveJobCloseStep::Complete { progress: semio_framework_value::retained_clone::RetainedCloneProgress::default() }
            } else {
                InteractiveJobCloseStep::Blocked
            })
        })
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
}

pub(super) struct Process3dBoundedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Process3dBoundedCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: PROCESS3D_BOUNDED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for Process3dBoundedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Process3dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Process3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        PROCESS3D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        process3d_bounded_contract()
    }

    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: |payload| payload.maximum_raw_bytes, checkpoint_resume: false);

}

impl ArtifactOwnedToolJobFactory for Process3dBoundedCommandJobFactory {
    type Owner = EditorApp<Process3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = PROCESS3D_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_process_process3d_model::snapshot::PROCESS_3D_SCHEMA;
    /// 🛤️ The lane each tool publishes on, read off what its `🎮️commands/*` handler actually emits.
    /// `HostOnly` is a whole-document replacement or a shell round-trip carried as an `Effect`
    /// (`setSnapshot`/`setActiveExample`/`setStock`/`importModelFile` build
    /// `reset_process3d_document_effect`, `exportModel` a `DownloadMediaExport`, `loadModelRequest` a
    /// `RequestFileOpen`) rather than a store edit; `Artifact` is every verb that emits a
    /// `Process3dMutation` against the timeline, the stock or the workshop, admitted by
    /// `Process3dArtifactPreparationFactory`; `Config` is the viewer's replay cursor and the engagement line — view
    /// state, never history — so the cursor verbs and `engagementSubmit` publish there only, and every verb that
    /// inserts or removes a step publishes BOTH lanes (the step, and the cursor moved past it).
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementAbort", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "loadModelRequest", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSnapshot", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addWorkshopMachine", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeWorkshopMachine", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "updateWorkshopMachine", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeSelectedStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "moveStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "updateStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setStepEnabled", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setStock", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchInspector", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "worldPointerDown", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "worldFaceDragEnd", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "importModelFile", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "exportModel", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCursor", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "stepCursor", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "stepCursorBack", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "stepCursorForward", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
    ];
}

pub(super) struct Process3dResumableCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Process3dResumableCommandJobFactory {
    pub(super) fn new(controller_id: &str) -> Self {
        Self { keys: PROCESS3D_RESUMABLE_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl ToolJobFactory for Process3dResumableCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<Process3dPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<Process3dPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        PROCESS3D_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        process3d_resumable_contract()
    }

    semio_framework_plugin::retained_command::retained_command_factory_admission!(maximum_raw_bytes: |payload| payload.maximum_raw_bytes, checkpoint_resume: true);

}

impl ArtifactOwnedToolJobFactory for Process3dResumableCommandJobFactory {
    type Owner = EditorApp<Process3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = PROCESS3D_RESUMABLE_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_process_process3d_model::snapshot::PROCESS_3D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "toggleSun", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSunAzimuth", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSunElevation", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSunIntensity", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
    ];
}
//#endregion 🧵️RetainedCommands
