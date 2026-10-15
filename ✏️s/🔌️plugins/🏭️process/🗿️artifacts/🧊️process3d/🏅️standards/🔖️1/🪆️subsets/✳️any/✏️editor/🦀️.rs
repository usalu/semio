//! 🖥️ Process 3d play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the workpiece
//! window's render/engagement in `🎭️modes/✏️edit/🪟️windows/🪚️workpiece`, panel trees in `📌️panels/*`,
//! labels in `🦀️terminology.rs`, view state in `🦀️config.rs`, shared compute in the artifact's `⚙️engine`.
//! This file is a routing table: `handle` → `Process3dCommand::dispatch`, `render` → body-key → node,
//! and a `🔖️Manifest` region that calls one `definition()` per node.
//!
//! 🧪️ B1: `Process3dPlayApp` is a unit struct — every former `Process3dRuntime` field (selection, hover,
//! face pick, selection method, engagement input, camera, sun) lives in `config::Process3dConfig`,
//! written via `config::Process3dConfigMutation`s; every action dispatches through the single typed
//! `Process3dCommand` channel via `ArtifactEditor::handle`.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

//#region 🧩️Modules
#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo_session {
        #[path = "📚️examples/🎬️demo-session/🦀️.rs"]
        mod component;
        pub use component::*;
    }
}
//#endregion 🧩️Modules

use semio_s_artifact_process_process3d_commands::{camera, contribution, cursor, document, engagement, inspector, media, step, stock, sun, workshop, world};
use semio_s_artifact_process_process3d_config::{Process3dConfig, Process3dConfigMutation, PROCESS3D_DEFAULT_UTILITY, Process3dConfigSetCamera, Process3dConfigSetContributions, Process3dConfigSetCursor, Process3dConfigSetEngagementInput, Process3dConfigSetSun};
use semio_s_artifact_process_process3d_modes;
use semio_s_artifact_process_process3d_modes::windows::workpiece;
use semio_s_artifact_process_process3d_panels::{catalogue, document as document_panel, inspection, workshop as workshop_panel};
use semio_s_artifact_process_process3d_presence::{Process3dPresence, Process3dPresenceMutation};
use semio_s_artifact_process_process3d_kit::editor::process3d::terminology::process3d_labels;
use semio_s_artifact_process_process3d_mutations::Process3dMutation;
use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceText as _;
use semio_s_artifact_process_process3d_model::{snapshot::Capability, snapshot::CapabilityRule, snapshot::MachineCatalog, snapshot::MachineCatalogs, snapshot::MeasureRecipe, snapshot::Process3dSnapshot, snapshot::ProcessMeasure, snapshot::ProcessStep, snapshot::StepOrigin, snapshot::Stock, snapshot::WorkingSolid, snapshot::WorkshopMachine};
use semio_framework::kernel::Effect;
use semio_framework::{DslValue, InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppActionRegistry;
use semio_framework_plugin::AppDefinition;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactKindSpec;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::CommandDefinition;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::ContextMenuItemSpec;
use semio_framework_plugin::ContextMenuRequest;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::Menu;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::OsMediaCapability;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::WindowMeasure;
use std::collections::HashMap;
use pack::snapshot::ArtifactPack;
use semio_framework_2d::compute::EngineHandles;

//#region 🔖️Constants

















pub use catalogue::PROCESS_3D_PLAY_BODY_CATALOGUE;
pub use document_panel::PROCESS_3D_PLAY_BODY_ARTIFACT;
pub use inspection::PROCESS_3D_PLAY_BODY_INSPECTION;
pub use workpiece::PROCESS_3D_PLAY_BODY_MAIN;
pub use workshop_panel::PROCESS_3D_PLAY_BODY_WORKSHOP;

























//#endregion 🔖️Constants

//#region 🔖️Commands


// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use camera::set_camera;
use contribution::set_contributions;
use cursor::{set_cursor, step_cursor, step_cursor_back, step_cursor_forward};
use document::{set_active_example, set_snapshot};
use engagement::{engagement_abort, engagement_input, engagement_submit};
use inspector::patch_inspector;
use media::{export_model, import_model_file, load_model_request};
use step::{add_step, move_step, remove_selected_step, remove_step, set_step_enabled, update_step};
use stock::set_stock;
use sun::{set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use workshop::{add_workshop_machine, remove_workshop_machine, update_workshop_machine};
use world::{world_face_drag_end, world_pointer_down};
//#endregion 🔖️Commands

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Process3dPlayApp::Command` — the SOLE dispatch surface for process3d's own behavior, assembled
    /// from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`) and the `dsl` wire keyword (the kebab `#[dsl(key = ..)]` the codec uses) — copied
    /// verbatim from the pre-migration `Process3dCommand`/`command_id()` match. **Row order is the binary
    /// variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum Process3dCommand for Process3dSnapshot, Process3dMutation, Process3dConfig, Process3dConfigMutation, ctx = Process3dDispatchCtx {
        "setSnapshot" as "document" => set_snapshot::SetDocument,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "addStep" as "add-step" => add_step::AddStep,
        "addWorkshopMachine" as "add-workshop-machine" => add_workshop_machine::AddWorkshopMachine,
        "removeWorkshopMachine" as "remove-workshop-machine" => remove_workshop_machine::RemoveWorkshopMachine,
        "updateWorkshopMachine" as "update-workshop-machine" => update_workshop_machine::UpdateWorkshopMachine,
        "removeStep" as "remove-step" => remove_step::RemoveStep,
        "removeSelectedStep" as "remove-selected-step" => remove_selected_step::RemoveSelectedStep,
        "moveStep" as "move-step" => move_step::MoveStep,
        "updateStep" as "update-step" => update_step::UpdateStep,
        "setStepEnabled" as "set-step-enabled" => set_step_enabled::SetStepEnabled,
        "setStock" as "stock" => set_stock::SetStock,
        "patchInspector" as "patch-inspector" => patch_inspector::PatchInspector,
        "setCursor" as "cursor" => set_cursor::SetCursor,
        "stepCursor" as "step-cursor" => step_cursor::StepCursor,
        "stepCursorBack" as "step-cursor-back" => step_cursor_back::StepCursorBack,
        "stepCursorForward" as "step-cursor-forward" => step_cursor_forward::StepCursorForward,
        "engagementSubmit" as "engagement-submit" => engagement_submit::EngagementSubmit,
        "worldPointerDown" as "world-pointer-down" => world_pointer_down::WorldPointerDown,
        "worldFaceDragEnd" as "world-face-drag-end" => world_face_drag_end::WorldFaceDragEnd,
        "importModelFile" as "import-model-file" => import_model_file::ImportModelFile,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "engagementAbort" as "engagement-abort" => engagement_abort::EngagementAbort,
        "setCamera" as "camera" => set_camera::SetCamera,
        "toggleSun" as "toggle-sun" => toggle_sun::ToggleSun,
        "setSunAzimuth" as "sun-azimuth" => set_sun_azimuth::SetSunAzimuth,
        "setSunElevation" as "sun-elevation" => set_sun_elevation::SetSunElevation,
        "setSunIntensity" as "sun-intensity" => set_sun_intensity::SetSunIntensity,
        "setContributions" as "contributions" => set_contributions::SetContributions,
        "exportModel" as "export-model" => export_model::ExportModel,
        "loadModelRequest" as "load-model-request" => load_model_request::LoadModelRequest,
    }
}
//#endregion 🔖️Commands
//#region 🔖️Process3dPlayApp
/// 🧪️ B1: unit struct — every former `Process3dRuntime` field now lives in `config::Process3dConfig`
/// (see `ArtifactEditor::Config`), written through `config::Process3dConfigMutation`s.
#[derive(Default)]
pub struct Process3dPlayApp;

/// 🖱️ The one context-menu implementation both `ArtifactEditor` entry points share — the destructive
/// `removeSelectedStep` row exists only for a non-empty `"geometry"` selection, so a right-click on
/// empty space never offers a verb that would silently no-op in `remove_selected_step::handle`.
fn process3d_context_menu_items(registry: &AppActionRegistry, view_state: &semio_framework_plugin::ViewModel, selected_ids: &[String]) -> Vec<ContextMenuItemSpec> {
    let menu = Menu::of(registry, view_state).action("addStep");
    let menu = if selected_ids.is_empty() { menu } else { menu.destructive("removeSelectedStep") };
    menu.separator().action("undo").action("redo").build()
}

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
const PROCESS3D_RETAINED_RAW_BYTES: usize = 8_192;
const PROCESS3D_RETAINED_WORK_ITEMS: usize = 64;
const PROCESS3D_SCAN_BYTES: usize = 256;
/// 📏️ Everything the retained config holds APART from the contributions pack — engagement text and
/// the sun colour. Unchanged; the contributions lane is priced separately below.
const PROCESS3D_CONFIG_FILTER_STORE_BYTES: usize = 16_384;

const PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES: usize = PROCESS3D_CONFIG_FILTER_STORE_BYTES + PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES;
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
fn process3d_retained_raw_bytes(tool_id: &str) -> usize {
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
fn process3d_retained_work_items(tool_id: &str) -> usize {
    process3d_retained_raw_bytes(tool_id).div_ceil(PROCESS3D_SCAN_BYTES).max(PROCESS3D_RETAINED_WORK_ITEMS)
}

/// 🛤️ Which retained work shape a tool id reduces through — the two are exhaustive over this app's
/// whole action surface, so an id that answers `None` here is genuinely not a command of this app.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Process3dCommandDisposition {
    Bounded,
    Config,
}

fn process3d_command_disposition(tool_id: &str) -> Option<Process3dCommandDisposition> {
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
fn process3d_resumable_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(semio_framework_plugin::CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES, 64, 1, PROCESS3D_RESUMABLE_OUTPUT_BYTES, 7_500, 1, 1)
}

fn process3d_string_units(value: &str) -> usize {
    value.len().div_ceil(PROCESS3D_SCAN_BYTES).max(1)
}

fn process3d_resumable_extent(command: &Process3dCommand, _snapshot: &Process3dSnapshot, config: &Process3dConfig, _interaction: &protocol::InteractionState) -> Option<usize> {
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
fn process3d_retained_reduce(
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

struct Process3dResumableCommandWork {
    tool_id: &'static str,
    extent: usize,
    cursor: usize,
    digest: u64,
    complete: bool,
    closing: bool,
}

impl Process3dResumableCommandWork {
    fn new(tool_id: &'static str, extent: usize) -> Self {
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

    fn close_step(&mut self, _grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        if self.closing {
            InteractiveJobCloseStep::Complete { progress: semio_framework_value::retained_clone::RetainedCloneProgress::default() }
        } else {
            InteractiveJobCloseStep::Blocked
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
}

struct Process3dBoundedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Process3dBoundedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
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

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > payload.maximum_raw_bytes || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Process3d bounded command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
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

struct Process3dResumableCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl Process3dResumableCommandJobFactory {
    fn new(controller_id: &str) -> Self {
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

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > payload.maximum_raw_bytes || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("Process3d resumable command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
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

//#region 📬️ConfigStorePreparation
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct Process3dConfigStorePreparationFactory;

struct Process3dConfigStorePreparation {
    owners: store::OneItemOwners<Process3dConfig, Process3dConfigMutation>,
    checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
}

fn process3d_config_retained_bytes(config: &Process3dConfig) -> Option<usize> {
    [config.engagement_input.len(), config.sun_color.len(), config.contributions_json.len()].into_iter().try_fold(0usize, usize::checked_add)
}

fn process3d_config_mutation_retained_bytes(mutation: &Process3dConfigMutation) -> usize {
    match mutation {
        Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ value }) => value.len(),
        Process3dConfigMutation::SetSun(Process3dConfigSetSun{ color, .. }) => color.len(),
        Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ json }) => json.len(),
        Process3dConfigMutation::SetCamera(Process3dConfigSetCamera{ .. }) | Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ .. }) => 0,
    }
}

fn admit_process3d_config_mutation(mutation: &Process3dConfigMutation) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = process3d_config_mutation_retained_bytes(mutation);
    let envelope = match mutation {
        Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ .. }) => PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES,
        _ => PROCESS3D_RETAINED_RAW_BYTES,
    };
    if retained_bytes > envelope {
        return Err("Process3d config mutation exceeds its fixed retained preparation envelope".into());
    }
    Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes))
}

fn prepare_process3d_config(base: &Process3dConfig, mutation: &Process3dConfigMutation) -> Result<(Process3dConfig, Vec<Process3dConfigMutation>), String> {
    admit_process3d_config_mutation(mutation)?;
    if process3d_config_retained_bytes(base).is_none_or(|bytes| bytes > PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES) {
        return Err("Process3d config base exceeds its fixed retained preparation envelope".into());
    }
    let inverse = match mutation {
        Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ .. }) => Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ value: base.engagement_input.clone() }),
        Process3dConfigMutation::SetCamera(Process3dConfigSetCamera{ .. }) => Process3dConfigMutation::SetCamera(Process3dConfigSetCamera{ position: base.camera_position, target: base.camera_target, fov: base.camera_fov }),
        Process3dConfigMutation::SetSun(Process3dConfigSetSun{ .. }) => Process3dConfigMutation::SetSun(Process3dConfigSetSun{ enabled: base.sun_enabled, azimuth: base.sun_azimuth, elevation: base.sun_elevation, intensity: base.sun_intensity, color: base.sun_color.clone() }),
        Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ .. }) => Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ json: base.contributions_json.clone() }),
        Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ .. }) => Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ value: base.resolved_up_to }),
    };
    let mut post = base.clone();
    match mutation {
        Process3dConfigMutation::SetEngagementInput(Process3dConfigSetEngagementInput{ value }) => post.engagement_input = value.clone(),
        Process3dConfigMutation::SetCamera(Process3dConfigSetCamera{ position, target, fov }) => {
            post.camera_position = *position;
            post.camera_target = *target;
            post.camera_fov = *fov;
        }
        Process3dConfigMutation::SetSun(Process3dConfigSetSun{ enabled, azimuth, elevation, intensity, color }) => {
            post.sun_enabled = *enabled;
            post.sun_azimuth = *azimuth;
            post.sun_elevation = *elevation;
            post.sun_intensity = *intensity;
            post.sun_color = color.clone();
        }
        Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{ json }) => post.contributions_json = json.clone(),
        Process3dConfigMutation::SetCursor(Process3dConfigSetCursor{ value }) => post.resolved_up_to = *value,
    }
    if process3d_config_retained_bytes(&post).is_none_or(|bytes| bytes > PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES) {
        return Err("Process3d config post-state exceeds its fixed retained preparation envelope".into());
    }
    Ok((post, vec![inverse]))
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Process3dConfig, Process3dConfigMutation> for Process3dConfigStorePreparationFactory {
    fn begin_batch_digest(
        &self,
        edit: &mut Option<Box<protocol::Edit<Process3dConfigMutation>>>,
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Process3dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &Process3dConfigMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Process3d config preparation rejected its lane".into());
        }
        admit_process3d_config_mutation(mutation)
    }

    fn begin_demand(&self, _mutation: &Process3dConfigMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Process3dConfigStorePreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Process3dConfig, Process3dConfigMutation, Process3dConfigMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<
        (Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Process3dConfig, Process3dConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress),
        (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Process3dConfig, Process3dConfigMutation, Process3dConfigMutation>),
    > {
        if request.lane != store::os_store::store_schema::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d config preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((Box::new(Process3dConfigStorePreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), cancelled: false }), progress))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Process3dConfig, Process3dConfigMutation> for Process3dConfigStorePreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use semio_framework_value::{retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if let Some(error) = self.owners.failure.as_ref() {
            return Err(error.clone());
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Process3d config preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Process3d config preparation lost its exact base root"))?;
        let mutation = self.owners.mutation.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Process3d config preparation lost its mutation owner"))?;
        let (post, inverse) = match prepare_process3d_config(base.get(), &mutation) {
            Ok(candidate) => candidate,
            Err(message) => {
                *self.owners.mutation = Some(mutation);
                *self.owners.failure = Some(ValueError::new(ValueRefusalKind::OwnershipLimit, message.clone()));
                return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, message));
            }
        };
        let Some(authority) = self.owners.authority.as_ref() else {
            *self.owners.mutation = Some(mutation);
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Process3d config preparation lost its Store authority"));
        };
        let edit = authority.next_edit(mutation, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Process3dConfig, Process3dConfigMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Process3dConfig, Process3dConfigMutation>> {
        self.owners.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.owners.begin_close();
    }

    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.owners.close_step(grant.retained_grant())
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}
//#endregion 📬️ConfigStorePreparation

//#region 📬️ArtifactStorePreparation
/// 🧺️ Fixed envelope for the process3d document's retained one-item publication lane. The document's
/// only unbounded axes are the durable `step_payloads` timeline, its re-minted `tool_solids` handles,
/// and the workshop's machines with their capability leaves — each counted here rather than encoded.
const PROCESS3D_DOCUMENT_MAXIMUM_ITEMS: usize = 4_096;
/// 🔤️ Longest single id, label, handle or recipe parameter name the retained lane admits, in either
/// the base or a mutation payload.
const PROCESS3D_DOCUMENT_TEXT_BYTES: usize = 256;
const PROCESS3D_DOCUMENT_MAXIMUM_BYTES: usize = 512 * 1_024;
/// 🎟️ What one `advance`/`close_step` turn costs, and the ONLY figure the grant is ever compared
/// against. The host drives this lane with a fixed `ArtifactStoreOneItemGrant { maximum_items: 1,
/// maximum_copy_bytes: TYPED_OPERATION_RESULT_PAGE_BYTES }` (4 KiB), so a gate that scaled with the
/// document — `grant.maximum_copy_bytes < measured_base_bytes` — would go `Blocked` forever the moment a
/// timeline or a workshop outgrew one page, stalling the operation instead of failing it. The base's
/// own size is a VALIDATION (`process3d_document_bytes`, rejected past
/// `PROCESS3D_DOCUMENT_MAXIMUM_BYTES`), never the gate.
const PROCESS3D_DOCUMENT_GRANT_BYTES: usize = 4_096;

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct Process3dArtifactPreparationFactory;

struct Process3dArtifactPreparation {
    owners: store::OneItemOwners<Process3dSnapshot, Process3dMutation>,
    checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
}

/// 📏️ One text field's own retained cost — rejected rather than truncated past the fixed envelope.
fn process3d_text_bytes(value: &str) -> Result<usize, semio_framework_value::ValueError> {
    if value.len() > PROCESS3D_DOCUMENT_TEXT_BYTES {
        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Process3d document carries a text field beyond its encoded text envelope"));
    }
    Ok(value.len())
}

fn process3d_child_bytes<S>(child: &store::os_store::artifact_child::ArtifactChild<S>) -> Result<usize, semio_framework_value::ValueError> {
    Ok(process3d_text_bytes(&child.child_id)?.saturating_add(process3d_text_bytes(&child.target.to_uri())?))
}

fn process3d_solid_bytes(solid: &WorkingSolid) -> Result<usize, semio_framework_value::ValueError> {
    match solid {
        WorkingSolid::Box { .. } | WorkingSolid::Cylinder { .. } | WorkingSolid::Sphere { .. } => Ok(0),
        WorkingSolid::ImportedMesh { mesh_url } => process3d_text_bytes(mesh_url),
        WorkingSolid::ImportedSolid { solid_handle } => process3d_text_bytes(solid_handle),
        WorkingSolid::Reference { reference_id } => process3d_text_bytes(reference_id),
    }
}

fn process3d_measure_bytes(measure: &ProcessMeasure) -> Result<usize, semio_framework_value::ValueError> {
    match measure {
        ProcessMeasure::Cut { tool, .. } => process3d_solid_bytes(tool),
        ProcessMeasure::Attach { component, .. } => process3d_solid_bytes(component),
        ProcessMeasure::Drill { .. } => Ok(0),
    }
}

fn process3d_origin_bytes(origin: Option<&StepOrigin>) -> Result<usize, semio_framework_value::ValueError> {
    match origin {
        Some(origin) => Ok(process3d_text_bytes(&origin.machine_id)?.saturating_add(process3d_text_bytes(&origin.capability_id)?)),
        None => Ok(0),
    }
}

fn process3d_step_bytes(step: &ProcessStep) -> Result<usize, semio_framework_value::ValueError> {
    Ok(process3d_text_bytes(&step.id)?.saturating_add(process3d_text_bytes(&step.label)?).saturating_add(process3d_origin_bytes(step.origin.as_ref())?).saturating_add(process3d_measure_bytes(&step.measure)?).saturating_add(size_of::<ProcessStep>()))
}

fn process3d_recipe_bytes(recipe: &MeasureRecipe) -> Result<usize, semio_framework_value::ValueError> {
    let parts: [&str; 3] = match recipe {
        MeasureRecipe::DiscCut { diameter, kerf } => [diameter, kerf, ""],
        MeasureRecipe::BladeCut { kerf, length, depth } => [kerf, length, depth],
        MeasureRecipe::PocketCut { diameter, depth } => [diameter, depth, ""],
        MeasureRecipe::BoreDrill { radius, depth } => [radius, depth, ""],
        MeasureRecipe::CylinderAttach { radius, length } => [radius, length, ""],
        MeasureRecipe::BoxAttach { width, depth, height } => [width, depth, height],
    };
    parts.iter().try_fold(0usize, |bytes, part| -> Result<usize, semio_framework_value::ValueError> { Ok(bytes.saturating_add(process3d_text_bytes(part)?)) })
}

fn process3d_capability_bytes(capability: &Capability) -> Result<usize, semio_framework_value::ValueError> {
    let parameters = capability.parameters.iter().try_fold(0usize, |bytes, parameter| -> Result<usize, semio_framework_value::ValueError> { Ok(bytes.saturating_add(process3d_text_bytes(&parameter.id)?).saturating_add(process3d_text_bytes(&parameter.label)?)) })?;
    let rules = capability.rules.iter().try_fold(0usize, |bytes, rule| -> Result<usize, semio_framework_value::ValueError> {
        let (CapabilityRule::Min { parameter, .. } | CapabilityRule::Max { parameter, .. }) = rule;
        Ok(bytes.saturating_add(process3d_text_bytes(parameter)?))
    })?;
    Ok(process3d_text_bytes(&capability.id)?
        .saturating_add(process3d_text_bytes(&capability.label)?)
        .saturating_add(process3d_text_bytes(&capability.icon_id)?)
        .saturating_add(process3d_recipe_bytes(&capability.recipe)?)
        .saturating_add(parameters)
        .saturating_add(rules)
        .saturating_add(size_of::<Capability>()))
}

fn process3d_capability_items(capability: &Capability) -> usize {
    1usize.saturating_add(capability.parameters.len()).saturating_add(capability.rules.len())
}

/// 📏️ One capability set costs its owner plus one work item per capability leaf it carries.
fn process3d_capabilities_items(capabilities: &[Capability]) -> usize {
    capabilities.iter().fold(1usize, |items, capability| items.saturating_add(process3d_capability_items(capability)))
}

fn process3d_capabilities_bytes(capabilities: &[Capability]) -> Result<usize, semio_framework_value::ValueError> {
    capabilities.iter().try_fold(0usize, |bytes, capability| -> Result<usize, semio_framework_value::ValueError> { Ok(bytes.saturating_add(process3d_capability_bytes(capability)?)) })
}

fn process3d_machine_bytes(machine: &WorkshopMachine) -> Result<usize, semio_framework_value::ValueError> {
    Ok(process3d_text_bytes(&machine.id)?
        .saturating_add(process3d_text_bytes(&machine.label)?)
        .saturating_add(process3d_text_bytes(&machine.icon_id)?)
        .saturating_add(machine.catalog_id.as_deref().map_or(Ok(0), process3d_text_bytes)?)
        .saturating_add(process3d_capabilities_bytes(&machine.capabilities)?)
        .saturating_add(size_of::<WorkshopMachine>()))
}

fn process3d_machine_items(machine: &WorkshopMachine) -> usize {
    process3d_capabilities_items(&machine.capabilities)
}

fn process3d_stock_bytes(stock: &Stock) -> Result<usize, semio_framework_value::ValueError> {
    Ok(process3d_text_bytes(&stock.id)?.saturating_add(process3d_text_bytes(&stock.label)?).saturating_add(process3d_solid_bytes(&stock.solid)?))
}

/// 📏️ The retained footprint of one document base — rejected rather than truncated when the timeline,
/// the workshop, or any single id/label/handle outgrows the fixed envelope.
fn process3d_document_bytes(document: &Process3dSnapshot) -> Result<usize, semio_framework_value::ValueError> {
    let items = document.step_payloads.len().saturating_add(document.tool_solids.len()).saturating_add(document.workshop.machines.iter().fold(0usize, |items, machine| items.saturating_add(process3d_machine_items(machine))));
    if items > PROCESS3D_DOCUMENT_MAXIMUM_ITEMS {
        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit, "Process3d document base exceeds its retained item envelope"));
    }
    let steps = document.step_payloads.iter().try_fold(0usize, |bytes, step| -> Result<usize, semio_framework_value::ValueError> { Ok(bytes.saturating_add(process3d_step_bytes(step)?)) })?;
    let machines = document.workshop.machines.iter().try_fold(0usize, |bytes, machine| -> Result<usize, semio_framework_value::ValueError> { Ok(bytes.saturating_add(process3d_machine_bytes(machine)?)) })?;
    let tools = document.tool_solids.iter().try_fold(0usize, |bytes, tool| -> Result<usize, semio_framework_value::ValueError> { Ok(bytes.saturating_add(process3d_child_bytes(tool)?)) })?;
    let bytes = process3d_text_bytes(&document.stock_id)?
        .saturating_add(process3d_text_bytes(&document.stock_label)?)
        .saturating_add(process3d_stock_bytes(&document.stock_payload)?)
        .saturating_add(process3d_child_bytes(&document.stock_solid)?)
        .saturating_add(process3d_child_bytes(&document.steps)?)
        .saturating_add(steps)
        .saturating_add(machines)
        .saturating_add(tools)
        .saturating_add(size_of::<Process3dSnapshot>());
    if bytes > PROCESS3D_DOCUMENT_MAXIMUM_BYTES {
        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Process3d document base exceeds its retained byte envelope"));
    }
    Ok(bytes)
}

/// 📏️ One semantic mutation's own retained bytes, shaped like what it actually addresses: a single step, machine,
/// stock field or cursor is one item carrying that target's own text, while a created machine or a replaced capability
/// set is one item per capability leaf it carries — the items bounded by `PROCESS3D_DOCUMENT_MAXIMUM_ITEMS`.
fn process3d_mutation_retained_bytes(mutation: &Process3dMutation) -> Result<usize, semio_framework_value::ValueError> {
    let (items, retained_bytes) = match mutation {
        Process3dMutation::CreateStep(payload) => (1, process3d_step_bytes(&payload.step)?),
        Process3dMutation::DeleteStep(payload) => (1, process3d_text_bytes(&payload.id)?),
        Process3dMutation::RenameStep(payload) => (1, process3d_text_bytes(&payload.id)?.saturating_add(process3d_text_bytes(&payload.new_label)?)),
        Process3dMutation::ChangeStepEnabled(payload) => (1, process3d_text_bytes(&payload.id)?),
        Process3dMutation::ChangeStepOrigin(payload) => (1, process3d_text_bytes(&payload.id)?.saturating_add(process3d_origin_bytes(payload.new_origin.as_ref())?)),
        Process3dMutation::ReplaceStepMeasure(payload) => (1, process3d_text_bytes(&payload.id)?.saturating_add(process3d_measure_bytes(&payload.new_measure)?)),
        Process3dMutation::ReorderSteps(payload) => (1, process3d_text_bytes(&payload.id)?),
        Process3dMutation::CreateMachine(payload) => (process3d_machine_items(&payload.machine), process3d_machine_bytes(&payload.machine)?),
        Process3dMutation::DeleteMachine(payload) => (1, process3d_text_bytes(&payload.id)?),
        Process3dMutation::RenameMachine(payload) => (1, process3d_text_bytes(&payload.id)?.saturating_add(process3d_text_bytes(&payload.new_label)?)),
        Process3dMutation::ChangeMachineIcon(payload) => (1, process3d_text_bytes(&payload.id)?.saturating_add(process3d_text_bytes(&payload.new_icon_id)?)),
        Process3dMutation::ReplaceMachineCapabilities(payload) => (process3d_capabilities_items(&payload.new_capabilities), process3d_text_bytes(&payload.id)?.saturating_add(process3d_capabilities_bytes(&payload.new_capabilities)?)),
        Process3dMutation::MoveStock(_) => (1, 0),
        Process3dMutation::ChangeStockLabel(payload) => (1, process3d_text_bytes(&payload.new_label)?),
        Process3dMutation::ReplaceStockSolid(payload) => (1, process3d_child_bytes(&payload.new_solid)?),
    };
    let retained_bytes = retained_bytes.saturating_add(size_of::<Process3dMutation>());
    if items > PROCESS3D_DOCUMENT_MAXIMUM_ITEMS {
        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit, "Process3d document mutation exceeds its fixed one-item preparation envelope"));
    }
    if retained_bytes > PROCESS3D_DOCUMENT_MAXIMUM_BYTES {
        return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "Process3d document mutation exceeds its fixed one-item preparation envelope"));
    }
    Ok(retained_bytes)
}

/// 🧮️ Runs the mutation's own semantic `diff`/`inverse` against `base` and applies the resulting diff,
/// so the retained lane and the batch lane can never diverge. An `Error`/`Fatal` outcome (a duplicate
/// step id, a missing target) is a REJECTION here, not a silent no-op: `MutationOutcome::error`/
/// `fatal` force an EMPTY diff, and publishing anyway would write a no-op edit into history.
fn prepare_process3d_document(base: &Process3dSnapshot, mutation: &Process3dMutation) -> Result<(Process3dSnapshot, Vec<Process3dMutation>), String> {
    process3d_mutation_retained_bytes(mutation).map_err(semio_framework_value::ValueError::into_message)?;
    process3d_document_bytes(base).map_err(semio_framework_value::ValueError::into_message)?;
    let outcome = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::diff(mutation, base);
    if let Some(message) = outcome.messages().iter().find(|message| matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)) {
        return Err(format!("Process3d document mutation was refused by its own vocabulary: {}", message.message));
    }
    let inverse = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::inverse(mutation, base).map_err(semio_framework_value::ValueError::into_message)?;
    let post = protocol::apply_diff(outcome.diff(), base).map_err(|error| format!("Process3d document mutation could not apply onto its exact base: {}", error.message))?;
    process3d_document_bytes(&post).map_err(semio_framework_value::ValueError::into_message)?;
    Ok((post, inverse))
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Process3dSnapshot, Process3dMutation> for Process3dArtifactPreparationFactory {
    fn begin_batch_digest(
        &self,
        edit: &mut Option<Box<protocol::Edit<Process3dMutation>>>,
        grant: semio_framework_value::retained_clone::RetainedCloneGrant,
    ) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<Process3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &Process3dMutation, lane: store::os_store::store_schema::HistoryLane) -> Result<store::os_store::one_item::ArtifactStoreOneItemFootprint, String> {
        if lane != store::os_store::store_schema::HistoryLane::Document {
            return Err("Process3d document preparation rejected its lane".into());
        }
        process3d_mutation_retained_bytes(mutation).map_err(semio_framework_value::ValueError::into_message)?;
        Ok(store::os_store::one_item::ArtifactStoreOneItemFootprint::for_leaf(mutation, PROCESS3D_DOCUMENT_GRANT_BYTES))
    }

    fn begin_demand(&self, _mutation: &Process3dMutation, _lane: store::os_store::store_schema::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: size_of::<Process3dArtifactPreparation>(), depth: 1 })
    }

    fn begin(
        &self,
        request: store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Process3dSnapshot, Process3dMutation, Process3dMutation>,
        grant: store::os_store::one_item::ArtifactStoreOneItemGrant,
    ) -> Result<
        (Box<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Process3dSnapshot, Process3dMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress),
        (semio_framework_value::ValueError, store::os_store::one_item_publication::ArtifactStoreOneItemPreparationRequest<Process3dSnapshot, Process3dMutation, Process3dMutation>),
    > {
        if self.preflight(&request.mutation, request.lane).is_err()
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Process3d document preparation rejected its original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) {
            Ok(demand) => demand,
            Err(error) => return Err((error, request)),
        };
        let progress = match demand.admit(grant.retained_grant()) {
            Ok(progress) => progress,
            Err(error) => return Err((error, request)),
        };
        Ok((
            Box::new(Process3dArtifactPreparation { owners: store::OneItemOwners::from_request(request), checkpoint: store::os_store::one_item::ArtifactStoreOneItemCheckpoint::default(), retained_bytes: 0, cancelled: false }),
            progress,
        ))
    }
}

impl store::os_store::one_item_publication::ArtifactStoreOneItemPreparation<Process3dSnapshot, Process3dMutation> for Process3dArtifactPreparation {
    fn advance(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        use semio_framework_value::{retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
        if !grant.permits_one() || self.cancelled || self.owners.is_closing() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if let Some(error) = self.owners.failure.as_ref() {
            return Err(error.clone());
        }
        if self.owners.refused.is_some() {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Process3d document preparation retains its original Store refusal"));
        }
        if self.owners.prepared.is_some() {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()));
        }
        if self.owners.candidate.is_none() {
            let base = self.owners.base.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Process3d document preparation lost its exact base root"))?.get();
            process3d_document_bytes(base)?;
            if grant.maximum_copy_bytes < PROCESS3D_DOCUMENT_GRANT_BYTES {
                return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
            }
            let mutation = self.owners.mutation.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Process3d document preparation lost its mutation owner"))?;
            match prepare_process3d_document(base, &mutation) {
                Ok((post, inverse)) => *self.owners.candidate = Some((post, inverse, mutation)),
                Err(message) => {
                    *self.owners.mutation = Some(mutation);
                    *self.owners.failure = Some(ValueError::new(ValueRefusalKind::OwnershipLimit, message.clone()));
                    return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, message));
                }
            }
            self.retained_bytes = PROCESS3D_DOCUMENT_GRANT_BYTES;
            self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: PROCESS3D_DOCUMENT_GRANT_BYTES as u64, digest: [0; 32] };
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint, RetainedCloneProgress::default()));
        }
        if grant.maximum_copy_bytes < self.retained_bytes {
            return Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.owners.authority.is_none() {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Process3d document preparation lost its Store authority"));
        }
        let (post, inverse, forward) = self.owners.candidate.take().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "Process3d document preparation lost its candidate"))?;
        let authority = self.owners.authority.as_ref().expect("observed original Store authority");
        let prepared = match authority.prepare_one_item(authority.next_edit(forward, inverse), std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => {
                *self.owners.refused = Some((edit, post));
                return Err(error);
            }
        };
        self.checkpoint = store::os_store::one_item::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::os_store::one_item::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, RetainedCloneProgress::default()))
    }

    fn checkpoint(&self) -> store::os_store::one_item::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Process3dSnapshot, Process3dMutation>> {
        self.owners.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::os_store::one_item_publication::ArtifactStoreOneItemPrepared<Process3dSnapshot, Process3dMutation>> {
        self.owners.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.owners.begin_close();
    }

    fn close_step(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
        self.owners.close_step(grant.retained_grant())
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, maximum_copy_bytes: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(maximum_copy_bytes)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.close_demands(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}
//#endregion 📬️ArtifactStorePreparation

//#region 🧾️ProofCatalogs
struct Process3dBoundedProofs;
impl Process3dBoundedProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Process3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.process.process3d@1/*#editor",
        artifact_schema: "process.3d",
        factory: "Process3dBoundedCommandJobFactory",
        factory_type: Process3dBoundedCommandJobFactory,
        tools: {
            "engagementAbort" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "setCamera" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "loadModelRequest" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "setSnapshot" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "setActiveExample" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "addStep" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "addWorkshopMachine" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "removeWorkshopMachine" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "updateWorkshopMachine" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "removeStep" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "removeSelectedStep" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "moveStep" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "updateStep" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "setStepEnabled" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "setStock" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "patchInspector" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "engagementSubmit" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "worldPointerDown" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "worldFaceDragEnd" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "importModelFile" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "exportModel" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "setCursor" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "stepCursor" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "stepCursorBack" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
            "stepCursorForward" => ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
        }
    }
}

struct Process3dResumableProofs;
impl Process3dResumableProofs {
    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<Process3dPlayApp>,
        owner_file: "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.process.process3d@1/*#editor",
        artifact_schema: "process.3d",
        factory: "Process3dResumableCommandJobFactory",
        factory_type: Process3dResumableCommandJobFactory,
        tools: {
            "engagementInput" => process3d_resumable_contract(),
            "toggleSun" => process3d_resumable_contract(),
            "setSunAzimuth" => process3d_resumable_contract(),
            "setSunElevation" => process3d_resumable_contract(),
            "setSunIntensity" => process3d_resumable_contract(),
            "setContributions" => process3d_resumable_contract(),
        }
    }
}
//#endregion 🧾️ProofCatalogs

/// 🧱️ Every window/panel body of the process3d editor, rendered against one already-resolved
/// `"geometry"` selection — shared by `render` (empty selection) and `render_with_request_context`
/// (the live selection) so there is exactly one body-key match in the app.
fn process3d_render_body(body_key: &str, doc: &Process3dSnapshot, config: &Process3dConfig, view_state: &semio_framework_plugin::ViewModel, selected_ids: &[String]) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let labels = process3d_labels(view_state);
    let active_utility = view_state.active_utility_id.as_deref().filter(|utility| !utility.is_empty()).unwrap_or(PROCESS3D_DEFAULT_UTILITY);
    let base_body_key = body_key.split_once(':').map_or(body_key, |(base, _)| base);
    // 🪟️ One `TreeWindows` per panel body: the host's open/scroll state for exactly the body being
    // rendered, read straight off `ViewModel.tree_windows` (ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING).
    let windows = semio_framework_plugin::TreeWindows::for_body(view_state, base_body_key);
    match base_body_key {
        PROCESS_3D_PLAY_BODY_MAIN => workpiece::render(doc, config, active_utility).map(semio_framework_plugin::built_to_component_tree),
        PROCESS_3D_PLAY_BODY_ARTIFACT => document_panel::render(doc, config.resolved_up_to, labels, &windows).map(semio_framework_plugin::built_to_component_tree),
        PROCESS_3D_PLAY_BODY_CATALOGUE => catalogue::render(doc, &config.contributions_json, labels, &windows).map(semio_framework_plugin::built_to_component_tree),
        PROCESS_3D_PLAY_BODY_WORKSHOP => workshop_panel::render(doc, &config.contributions_json, labels, &windows).map(semio_framework_plugin::built_to_component_tree),
        PROCESS_3D_PLAY_BODY_INSPECTION => inspection::render(doc, selected_ids, labels, &windows).map(semio_framework_plugin::built_to_component_tree),
        _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
    }
}

impl ArtifactEditor for Process3dPlayApp {
    /// 📢️ The localized notices of this editor's user-reachable refusals (design §20.12).
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        use semio_framework_ui_locale::LocalizedLabel;
        static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 2]> = std::sync::LazyLock::new(|| {
            [
            ("process3d.action.invalid", LocalizedLabel::native("This action's arguments are not valid.", "Die Argumente dieser Aktion sind ungültig.")),
            ("process3d.media.export", LocalizedLabel::native("The model cannot be exported.", "Das Modell lässt sich nicht exportieren.")),
            ]
        });
        &*NOTICES
    }

    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![semio_s_artifact_process_process3d_examples::source(), semio_s_artifact_process_process3d_examples::examples::concrete_forest::source()]
    }
    /// 🧩️ The roster the three composed `s.stdio.semio` members (`stockSolid`/`steps`/`toolSolids`)
    /// open through — a `NoMembers` editor cannot materialise the children `genesis_child_pack`
    /// derives, and every whole-document load then fails its archive closure as `Incomplete`.
    type Members = semio_s_artifact_stdio_semio::SemioMembers;
    type Snapshot = Process3dSnapshot;
    type Mutation = Process3dMutation;
    type Config = Process3dConfig;
    type ConfigMutation = Process3dConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = Process3dPresence;
    type PresenceMutation = Process3dPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = Process3dCommand;

    const DIALECT: Dialect = semio_s_artifact_process_process3d_model::snapshot::PROCESS3D_DIALECT;

    const DOCUMENT_SCHEMA: &'static str = semio_s_artifact_process_process3d_model::snapshot::PROCESS_3D_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Process3dArtifactPreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::os_store::one_item_publication::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(Process3dConfigStorePreparationFactory))
    }

    fn bounded_first_step_tool_proofs() -> Vec<semio_framework_plugin::ArtifactBoundedFirstStepProof> {
        Process3dBoundedProofs::bounded_first_step_tool_proofs().into_iter().chain(Process3dResumableProofs::bounded_first_step_tool_proofs()).collect()
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller_id = registry.controller_id().to_string();
        registry.register(Process3dBoundedCommandJobFactory::new(&controller_id))?;
        registry.register(Process3dResumableCommandJobFactory::new(&controller_id))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        let Some(disposition) = process3d_command_disposition(&request.tool_id) else {
            return Ok(None);
        };
        if request.command.command_id() != request.tool_id {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.tool-mismatch"), "Process 3D command does not match its exact registered tool"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = match disposition {
            Process3dCommandDisposition::Bounded => Box::new(BoundedArtifactCommandWork::new(tool_id, process3d_retained_reduce, |_, _, _| Some(1))),
            Process3dCommandDisposition::Config => {
                let extent = process3d_resumable_extent(&request.command, &request.snapshot, &request.config, &request.interaction_state).ok_or_else(|| Fault::from("process3d-retained-work-extent-overflow"))?;
                Box::new(Process3dResumableCommandWork::new(tool_id, extent))
            }
        };
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
            retained: request.retained,
        };
        let payload = ArtifactRetainedCommandPayload::new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                snapshot_read: request.snapshot_read,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            Process3dCommand::command_id,
            process3d_retained_raw_bytes(tool_id),
            process3d_retained_work_items(tool_id),
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }
    const REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY: bool = true;

    fn build_envelope_decode_owner_bundle() -> Option<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(semio_s_artifact_process_process3d_host::process3d_envelope_decode_owner_bundle())
    }

    fn build_document_store_initialization_job(
        envelope: store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        actor: protocol::ActorId,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::os_store::store_schema::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(semio_s_artifact_process_process3d_host::process3d_document_store_initialization_job(envelope, operation, generation, actor))
    }

    /// 🔐️ The exact publication gate the host consults once, right before its non-rejecting commit
    /// — so a lease the app admitted for itself (`process3d_admit_app_publication_authority`, every
    /// host-begun `Effect::LoadDocument`) is consumed here; a host-admitted lease stays the host's.
    fn validate_document_store_publication(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), Fault> {
        semio_s_artifact_process_process3d_host::process3d_validate_atomic_publication_authority(operation, generation, live_generation).map_err(|code| Fault::new(FaultOrigin::App, FaultCode::new(code), "Process3d atomic publication authority is absent or stale"))?;
        semio_s_artifact_process_process3d_host::process3d_release_app_publication_authority(operation);
        Ok(())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    /// 🧹️ Every owned store lane the close ladder drives needs its bounded disposer, the absent ones
    /// (`NoDraft`, `NoTransient`) included — `drive_artifact_owned_disposer` faults
    /// `interactive-job.close-owned-disposer-missing` on the first `None` and the app never reaches
    /// its terminal-empty shell (mirrors `📐️cad`/`🧩️puzzle`).
    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<NoDraft, NoDraftMutation>())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(semio_s_artifact_process_process3d_presence::Process3dPresenceRetirementFactory))
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(semio_s_artifact_process_process3d_presence::Process3dPresenceRetirementFactory))
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(semio_s_artifact_process_process3d_presence::process3d_presence_store_disposer())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(semio_s_artifact_process_process3d_config::schema::app_schema_descriptor())
    }

    fn child_restore_projection(snapshot: &Self::Snapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, Fault> {
        semio_s_artifact_process_process3d_model::snapshot::process3d_child_restore_projection(snapshot)
    }

    fn genesis_child_pack(snapshot: &Self::Snapshot, slot: &str, child_id: &str) -> Result<Option<Vec<u8>>,semio_framework_value::ValueError> {
 Ok((||{
        semio_s_artifact_process_process3d_model::snapshot::genesis_process3d_child_pack(snapshot, slot, child_id)
    
})())
}

    fn initial_snapshot() -> Process3dSnapshot {
        semio_s_artifact_process_process3d_model::standards::v1::subsets::any::io::text::snapshot::default_document()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(process3d_io())
    }

    //#region 🔖️Media
    /// 🎞️ `brep:out` (see the artifact engine's `export_process3d_model`, STEP text) plus the inherited
    /// `document:out` default (the pack of `doc.snapshot`, replicated inline — overriding `export_media`
    /// shadows the trait's provided body for every port on this app, not just the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, Process3dSnapshot>) -> Result<semio_framework_plugin::Media, MediaError> {
        match port {
            "brep:out" => match semio_s_artifact_process_process3d_io::export_process3d_model(&semio_s_artifact_process_process3d_model::snapshot::process_working_scene_from_snapshot(doc.snapshot), None, "step").map_err(|error| MediaError::Payload("brep:out".into(), error))? {
                Some(export) => {
                    let text = match export.data {
                        DslValue::String(text) => text,
                        other => semio_framework_pack_json::to_json_string(&other),
                    };
                    Ok(semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep }, payload: MediaPayload::Structured { schema: "3d.process".into(), json: text } })
                }
                None => Err(MediaError::Payload("brep:out".into(), "kernel replay failed".into())),
            },
            "artifact:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.artifact_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(semio_framework_plugin::Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 📥️ `geometry:in` (best-effort STEP-text import) replaces the whole document via a `Effect::LoadDocument`
    /// (`reset_process3d_document_effect`, outside undo history); there is no import mutation. Overriding `import_media`
    /// shadows the trait's provided body for every port, so `document:in` is simply unimplemented here.
    fn import_media(port: &str, media: &semio_framework_plugin::Media, _doc: &ArtifactView<'_, Process3dSnapshot>) -> Result<Emit<Process3dMutation, Process3dConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "geometry:in" => {
                let MediaPayload::Structured { schema, json } = &media.payload else {
                    return Err(MediaError::Payload("geometry:in".into(), "expected a structured payload".into()));
                };
                if schema != semio_s_artifact_process_process3d_model::snapshot::PROCESS_3D_SCHEMA && schema != "3d.process" {
                    return Err(MediaError::Payload("geometry:in".into(), format!("unrecognized schema: {schema}")));
                }
                // 📦️ `export_process3d_model("step")` hands back raw (non-base64) STEP text — the
                // "stdio.step" format is not binary — so this re-encodes it as base64 to satisfy
                // `import_process3d_model`'s `data:...,<base64>` expectation.
                let data_url = format!("data:application/octet-stream;base64,{}", base64_codec::base64_standard_encode(json.as_bytes()));
                match semio_s_artifact_process_process3d_io::import_process3d_model("geometry-in.step", &data_url) {
                    Some(snapshot) => Ok(Emit { effects: vec![reset_process3d_document_effect(&snapshot)], ..Default::default() }),
                    None => Err(MediaError::Payload("geometry:in".into(), "STEP import failed".into())),
                }
            }
            _ => Err(MediaError::NotImplemented),
        }
    }
    //#endregion 🔖️Media

    /// 🏷️ The manifest action id each command was declared under — supplied wholesale by
    /// `app_commands!`'s generated `command_id()`.
    fn command_id(command: &Process3dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Exhaustive host-action bridge into the closed `Process3dCommand` enum. React and wgpu still
    /// emit manifest action ids plus JSON arguments; only this boundary interprets that transport shape.
    fn command_from_action(action: &str, args: &mut Option<DslValue>, original_fault: &mut Option<Fault>) -> Result<Self::Command, Fault> {
        if let Some(fault) = original_fault.take() { return Err(fault); }
        let args = args.as_ref();
        let field = |key: &str| args.and_then(|value| value.get(key));
        let string_field = |key: &str| field(key).and_then(DslValue::as_str).map(str::to_string);
        let number_field = |key: &str| field(key).and_then(DslValue::as_f64);
        let unsigned_field = |key: &str| field(key).and_then(|value| value.as_f64()).filter(|number| number.is_finite() && *number >= 0.0).map(|number| number as u64);
        let signed_field = |key: &str| field(key).and_then(|value| value.as_f64()).filter(|number| number.is_finite()).map(|number| number as i64);
        let vec3_field = |key: &str| -> Option<[f64; 3]> {
            let values = field(key)?.as_array()?;
            if values.len() != 3 {
                return None;
            }
            Some([values[0].as_f64()?, values[1].as_f64()?, values[2].as_f64()?])
        };
        let vec2_field = |key: &str| -> Option<[f64; 2]> {
            let values = field(key)?.as_array()?;
            if values.len() != 2 {
                return None;
            }
            Some([values[0].as_f64()?, values[1].as_f64()?])
        };
        match action {
            "setSnapshot" => {
                let json = string_field("json").or_else(|| field("document").map(semio_framework_pack_json::to_json_string)).unwrap_or_default();
                Ok(Process3dCommand::SetDocument(set_snapshot::SetDocument { json }))
            }
            "setActiveExample" => Ok(Process3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: string_field("exampleId").or_else(|| string_field("id")).unwrap_or_else(|| PROCESS3D_EXAMPLE_TIMBER.into()) })),
            "addStep" => Ok(Process3dCommand::AddStep(add_step::AddStep {
                measure: string_field("measure"),
                machine_id: string_field("machineId").or_else(|| string_field("machine_id")),
                capability_id: string_field("capabilityId").or_else(|| string_field("capability_id")),
                position: vec3_field("position"),
            })),
            "addWorkshopMachine" => Ok(Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine {
                catalog_id: string_field("catalogId").or_else(|| string_field("catalog_id")).unwrap_or_default(),
                machine_id: string_field("machineId").or_else(|| string_field("machine_id")).unwrap_or_default(),
            })),
            "removeWorkshopMachine" => Ok(Process3dCommand::RemoveWorkshopMachine(remove_workshop_machine::RemoveWorkshopMachine { id: string_field("id").unwrap_or_default() })),
            "updateWorkshopMachine" => Ok(Process3dCommand::UpdateWorkshopMachine(update_workshop_machine::UpdateWorkshopMachine {
                machine: args
                    .and_then(|value| value.get("machine"))
                    .cloned()
                    .map(<WorkshopMachine as semio_framework_value::FromValue>::from_value)
                    .transpose()
                    .map_err(|error| process3d_action_fault(action, format!("invalid 'machine': {error}")))?
                    .unwrap_or(WorkshopMachine { id: String::new(), label: String::new(), icon_id: String::new(), catalog_id: None, capabilities: Vec::new() }),
            })),
            "removeStep" => Ok(Process3dCommand::RemoveStep(remove_step::RemoveStep { id: string_field("id").unwrap_or_default() })),
            "removeSelectedStep" => Ok(Process3dCommand::RemoveSelectedStep(remove_selected_step::RemoveSelectedStep {})),
            "moveStep" => Ok(Process3dCommand::MoveStep(move_step::MoveStep { id: string_field("id").unwrap_or_default(), index: unsigned_field("index").unwrap_or_default() as usize })),
            "updateStep" => {
                let step_json = string_field("stepJson").or_else(|| string_field("step_json")).or_else(|| field("step").map(semio_framework_pack_json::to_json_string)).unwrap_or_default();
                Ok(Process3dCommand::UpdateStep(update_step::UpdateStep { step_json }))
            }
            "setStepEnabled" => Ok(Process3dCommand::SetStepEnabled(set_step_enabled::SetStepEnabled {
                id: string_field("id").unwrap_or_default(),
                enabled: field("enabled").and_then(DslValue::as_bool).ok_or_else(|| process3d_action_fault(action, "requires the boolean 'enabled' it sets"))?,
            })),
            "setStock" => Ok(Process3dCommand::SetStock(set_stock::SetStock { kind: string_field("kind").or_else(|| string_field("value")).unwrap_or_else(|| "box".into()) })),
            "patchInspector" => Ok(Process3dCommand::PatchInspector(patch_inspector::PatchInspector {
                target: string_field("target").unwrap_or_default(),
                field: string_field("field").unwrap_or_default(),
                number: number_field("number").or_else(|| number_field("value")),
                text: string_field("text").or_else(|| string_field("value")),
            })),
            "setCursor" => Ok(Process3dCommand::SetCursor(set_cursor::SetCursor { value: unsigned_field("value") })),
            "stepCursor" => Ok(Process3dCommand::StepCursor(step_cursor::StepCursor { delta: signed_field("delta").unwrap_or_default() })),
            "stepCursorBack" => Ok(Process3dCommand::StepCursorBack(step_cursor_back::StepCursorBack {})),
            "stepCursorForward" => Ok(Process3dCommand::StepCursorForward(step_cursor_forward::StepCursorForward {})),
            "engagementSubmit" => Ok(Process3dCommand::EngagementSubmit(engagement_submit::EngagementSubmit {})),
            "worldPointerDown" => Ok(Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: vec3_field("position").unwrap_or_default() })),
            "worldFaceDragEnd" => Ok(Process3dCommand::WorldFaceDragEnd(world_face_drag_end::WorldFaceDragEnd {
                normal: vec3_field("normal").unwrap_or([0.0, 0.0, 1.0]),
                start_point: vec3_field("startPoint").or_else(|| vec3_field("start_point")).unwrap_or_default(),
                distance: number_field("distance").unwrap_or_default(),
                face_extent: vec2_field("faceExtent").or_else(|| vec2_field("face_extent")),
            })),
            "importModelFile" => Ok(Process3dCommand::ImportModelFile(import_model_file::ImportModelFile { name: string_field("name").unwrap_or_default(), payload: string_field("payload").unwrap_or_default() })),
            "engagementInput" => Ok(Process3dCommand::EngagementInput(engagement_input::EngagementInput { value: string_field("value").unwrap_or_default() })),
            "engagementAbort" => Ok(Process3dCommand::EngagementAbort(engagement_abort::EngagementAbort {})),
            "setCamera" => Ok(Process3dCommand::SetCamera(set_camera::SetCamera { position: vec3_field("position").unwrap_or([3.0, -3.0, 2.0]), target: vec3_field("target").unwrap_or_default(), fov: number_field("fov").unwrap_or(45.0) })),
            "toggleSun" => Ok(Process3dCommand::ToggleSun(toggle_sun::ToggleSun {})),
            "setSunAzimuth" => Ok(Process3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: number_field("value").unwrap_or_default() })),
            "setSunElevation" => Ok(Process3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: number_field("value").unwrap_or_default() })),
            "setSunIntensity" => Ok(Process3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: number_field("value").unwrap_or_default() })),
            "setContributions" => Ok(Process3dCommand::SetContributions(set_contributions::SetContributions { json: string_field("json").unwrap_or_else(|| "[]".into()) })),
            "exportModel" => Ok(Process3dCommand::ExportModel(export_model::ExportModel { format: string_field("format").unwrap_or_else(|| "step".into()) })),
            "loadModelRequest" => Ok(Process3dCommand::LoadModelRequest(load_model_request::LoadModelRequest {})),
            other => Err(process3d_action_fault(other, "not declared by the process app's typed command vocabulary")),
        }
    }

    fn host_configuration_mutation(action: &str, args: Option<&DslValue>) -> Result<Option<Self::ConfigMutation>, Fault> {
        Ok((action == "setContributions").then(|| Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{
            json: installable_contributions(args.and_then(|value| value.get("json")).and_then(DslValue::as_str).unwrap_or("[]"), PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES),
        })))
    }

    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): reads the framework-owned
    /// `"geometry"` domain selection once per dispatch and threads it through `Process3dDispatchCtx`
    /// — the one retained verb that operates ON the selection (`remove_selected_step`) reads it from
    /// there; every other command ignores it (mirrors `📐️cad`'s own `handle`).
    fn handle(
        command: &Process3dCommand,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        cfg: &ConfigView<'_, Process3dConfig>,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
        view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation, Self::DraftMutation>, Fault> {
        let selection = interaction.selection(PROCESS3D_INTERACTION_DOMAIN);
        let mut ctx = Process3dDispatchCtx { interaction: Process3dInteractionSnapshot { ids: selection.ids.clone() }, view_state: view_state.cloned() };
        command.dispatch(doc, cfg, &mut ctx)
    }

    /// 🧮️ process3d exposes no genuinely settings-like sticky defaults — every `Process3dConfig` field
    /// is session-only view state, so this stays at the trait default.
    fn config_spec() -> semio_framework_plugin::ConfigSpec {
        semio_framework_plugin::ConfigSpec::default()
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        process3d_render_body(body_key, doc.snapshot, cfg.snapshot, view_state, &[])
    }

    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): resolves the live `"geometry"`
    /// selection once per render and threads it into `process3d_render_body`, so the inspection
    /// panel finally sees a real selection instead of always rendering empty.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        cfg: &ConfigView<'_, Process3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
        cx:&mut semio_framework_plugin::StepContext<'_>,
    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        let rendered:semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree>=(||{
        let selected_ids = interaction.selection(PROCESS3D_INTERACTION_DOMAIN).ids.clone();
        process3d_render_body(body_key, doc.snapshot, cfg.snapshot, view_state, &selected_ids)

        })();
        rendered.map(Some).map_err(Into::into)
        }

    fn window_engagements(doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, semio_framework_plugin::WindowEngagement> {
        let active_utility = view_state.active_utility_id.as_deref().filter(|utility| !utility.is_empty()).unwrap_or(PROCESS3D_DEFAULT_UTILITY);
        HashMap::from([(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into(), workpiece::engagement(doc.snapshot, cfg.snapshot, active_utility, process3d_labels(view_state)))])
    }

    fn window_measures(_doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::from([(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into(), workpiece::window_measures(cfg.snapshot, process3d_labels(view_state).is_de()))])
    }

    /// 🖱️ Interaction-less twin of [`Self::context_menu_with_request_context`] — an empty
    /// `"geometry"` domain, so the destructive row is withheld.
    fn context_menu(_request: &ContextMenuRequest, _doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>, view_state: &semio_framework_plugin::ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        process3d_context_menu_items(registry, view_state, &[])
    }

    /// 🕹️ `removeSelectedStep` appears only for a real `"geometry"` selection — the authoritative
    /// framework-owned one, since `ContextMenuRequest.surface.selection` only carries what the clicked
    /// surface itself painted (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
    fn context_menu_with_request_context(
        _request: &ContextMenuRequest,
        _doc: &ArtifactView<'_, Process3dSnapshot>,
        _cfg: &ConfigView<'_, Process3dConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        process3d_context_menu_items(registry, view_state, &interaction.selection(PROCESS3D_INTERACTION_DOMAIN).ids)
    }
}
//#endregion 🔖️Process3dPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline. `WindowKindDefinition.options.measures` stays empty: measures are config-derived per
/// frame by `ArtifactEditor::window_measures`, never frozen into the manifest.
pub fn create_process3d_app() -> AppDefinition {
    Editor::builder(semio_s_artifact_process_process3d_model::snapshot::PROCESS3D_DIALECT)
            .command({
                let mut definition = CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))]) };
                definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
                definition
            })
            .document(["semio", "process", "3d"])
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "Bearbeiten"])
            .artifact_kind(ArtifactKindSpec {
                id: "3d.process".into(),
                label: LocalizedLabel::native("3D Process", "3D-Prozess"),
                source_format: "process.3d".into(),
                component_kind: "process3d".into(),
                dimension: "3d".into(),
                media_capability: OsMediaCapability::Brep,
                media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
                schema: "process.3d".into(),
                export_formats: vec![],
                import_formats: vec![],
                export_stdio_kinds: vec!["stdio.step".into(), "stdio.obj".into(), "stdio.stl".into(), "stdio.gltf".into()],
                import_stdio_kinds: vec!["stdio.step".into(), "stdio.obj".into(), "stdio.stl".into()],
            })
            .icon_id("hammer")
            .mode_def(edit::definition())
            .default_mode_id(edit::PROCESS3D_MODE_EDIT)
            .window_kind_def(workpiece::definition())
            .default_layout(edit::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(workshop_panel::definition())
            .panel_tab_def(inspection::definition())
            // 🔧️ Palette-visible create/mutate actions (staged arg forms attached below).
            .mutation("addStep", LocalizedLabel::native("Add Step", "Schritt hinzufügen"))
            .mutation("setStock", LocalizedLabel::native("Set Stock", "Rohteil festlegen"))
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_destructive("setActiveExample")
            .mutation("removeSelectedStep", LocalizedLabel::native("Remove Selected Step", "Ausgewählten Schritt entfernen"))
            .action_destructive("removeSelectedStep")
            // 🐚️ Palette-visible host round-trips.
            .shell_action("exportModel", LocalizedLabel::native("Export Model", "Modell exportieren"))
            .action_destructive("exportModel")
            .shell_action("loadModelRequest", LocalizedLabel::native("Load Model…", "Modell laden…"))
            // 🔧️ Internal document mutations dispatched by panel/viewport wiring (not palette-worthy).
            .action_with(internal_action("setSnapshot", LocalizedLabel::native("Set Document", "Dokument festlegen"), ActionKind::Mutation))
            .action_destructive("setSnapshot")
            .action_with(internal_action("addWorkshopMachine", LocalizedLabel::native("Add Machine", "Maschine hinzufügen"), ActionKind::Mutation))
            .action_with(internal_action("removeWorkshopMachine", LocalizedLabel::native("Remove Machine", "Maschine entfernen"), ActionKind::Mutation))
            .action_destructive("removeWorkshopMachine")
            .action_with(internal_action("updateWorkshopMachine", LocalizedLabel::native("Update Machine", "Maschine aktualisieren"), ActionKind::Mutation))
            .action_with(internal_action("importModelFile", LocalizedLabel::native("Import Model File", "Modelldatei importieren"), ActionKind::Mutation))
            .action_with(internal_action("removeStep", LocalizedLabel::native("Remove Step", "Schritt entfernen"), ActionKind::Mutation))
            .action_destructive("removeStep")
            .action_with(internal_action("moveStep", LocalizedLabel::native("Move Step", "Schritt verschieben"), ActionKind::Mutation))
            .action_with(internal_action("updateStep", LocalizedLabel::native("Update Step", "Schritt aktualisieren"), ActionKind::Mutation))
            .action_with(internal_action("setStepEnabled", LocalizedLabel::native("Set Step Enabled", "Schrittaktivierung festlegen"), ActionKind::Mutation))
            .action_with(internal_action("patchInspector", LocalizedLabel::native("Patch Inspector", "Inspektor aktualisieren"), ActionKind::Mutation))
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"), ActionKind::Mutation, "mouse-pointer") })
            .action_audience("worldPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(internal_action("worldFaceDragEnd", LocalizedLabel::native("World Face Drag End", "Welt-Flächenzug beendet"), ActionKind::Mutation))
            .action_audience("worldFaceDragEnd", semio_framework_plugin::CapabilityAudience::Input)
            // ⏱️ Replay-cursor navigation: VIEW verbs on the config lane (the viewer's cursor), never history.
            .action_with(internal_action("setCursor", LocalizedLabel::native("Set Cursor", "Cursor festlegen"), ActionKind::View))
            .action_with(internal_action("stepCursor", LocalizedLabel::native("Step Cursor", "Cursor schrittweise bewegen"), ActionKind::View))
            .action_with(internal_action("stepCursorBack", LocalizedLabel::native("Step Cursor Back", "Cursor zurück"), ActionKind::View))
            .action_with(internal_action("stepCursorForward", LocalizedLabel::native("Step Cursor Forward", "Cursor vorwärts"), ActionKind::View))
            // 🎛️ Engagement session command line (a separate system from utility selection).
            .action_with(internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::View))
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View, "hand") })
            // 👁️ Ephemeral view state — camera, sun. Selection/hover are the framework-owned
            // "geometry" interaction domain now (declared below via `.interaction`); the six
            // framework verbs (interactionSelect/interactionHover/clearSelection/selectAll/
            // setSelectionMode/setInteractionGranularity) auto-inject — never declared here.
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), ActionKind::View, "sun") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), ActionKind::View, "sun") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), ActionKind::View, "sun") })
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), ActionKind::View, "sun") })
            // 📝️ Staged argument forms for the palette-visible create/export actions.
            .action_args("addStep", vec![
                ActionArgDef::select("measure", LocalizedLabel::native("Measure", "Maßnahme"), vec![
                    ActionArgOption::new("cut", LocalizedLabel::native("Cut", "Schnitt")),
                    ActionArgOption::new("drill", LocalizedLabel::native("Drill", "Bohrung")),
                    ActionArgOption::new("attach", LocalizedLabel::native("Attach", "Anbau")),
                ]).default_value(&"cut"),
            ])
            .action_args("setStepEnabled", vec![
                ActionArgDef::text("id", LocalizedLabel::native("Step", "Schritt")).required(),
                ActionArgDef::toggle("enabled", LocalizedLabel::native("Enabled", "Aktiviert")).required(),
            ])
            .action_args("setStock", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("box", LocalizedLabel::native("Box", "Quader")),
                    ActionArgOption::new("cylinder", LocalizedLabel::native("Cylinder", "Zylinder")),
                    ActionArgOption::new("sphere", LocalizedLabel::native("Sphere", "Kugel")),
                ]).default_value(&"box"),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new(PROCESS3D_EXAMPLE_TIMBER, LocalizedLabel::native("Timber Beam Joinery", "Holzbalkenverbindung")),
                    ActionArgOption::new(PROCESS3D_EXAMPLE_PLATE, LocalizedLabel::native("Drilled Plate", "Gebohrte Platte")),
                    ActionArgOption::new(PROCESS3D_EXAMPLE_CONCRETE_FOREST, LocalizedLabel::native("Concrete Forest", "Betonwald")),
                ]).required().default_value(&PROCESS3D_EXAMPLE_TIMBER),
            ])
            .action_args("exportModel", vec![
                ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), vec![
                    ActionArgOption::new("step", LocalizedLabel::native("STEP", "STEP")),
                    ActionArgOption::new("obj", LocalizedLabel::native("OBJ", "OBJ")),
                    ActionArgOption::new("stl", LocalizedLabel::native("STL", "STL")),
                    ActionArgOption::new("glb", LocalizedLabel::native("GLB", "GLB")),
                ]).required().default_value(&"step"),
            ])
            // 🧰️ Flat top-level exclusive utility bar scoped to the workpiece window (active utility is
            // host-owned). These four are the window's entire utility set — not a sub-collection — so
            // each carries `group: None` and renders as its own flat utility bar icon.
            .utility(UtilityDefinition { category: Some(UtilityCategory::Selection), ..UtilityDefinition::new("select", LocalizedLabel::native("Select", "Auswählen"), "mouse-pointer") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("cut", LocalizedLabel::native("Cut", "Schneiden"), "scissors") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("drill", LocalizedLabel::native("Drill", "Bohren"), "circle-dot") })
            .utility(UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new("attach", LocalizedLabel::native("Attach", "Anbauen"), "plus") })
            .window_kind_utilities(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN, vec!["select".into(), "cut".into(), "drill".into(), "attach".into()])
            .interaction(process3d_interaction_definition())
            .window_kind_interactions(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN, vec![InteractionRef::new(PROCESS3D_INTERACTION_DOMAIN)])
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("bracketleft", "stepCursorBack")
            .keybinding("bracketright", "stepCursorForward")
            .keybinding("escape", "engagementAbort")
            .keybinding("delete", "removeSelectedStep")
            .keybinding("backspace", "removeSelectedStep")
            .config(Process3dPlayApp::config_spec())
            .io(process3d_io())
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a
            // bare `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so the old
            // `PROCESS3D_EXAMPLE_TIMBER`/`PROCESS3D_EXAMPLE_PLATE` app-level example registrations and
            // the no-op `.workflow("process3d", …)` call are dropped here, not silently: reported in
            // this packet's migration notes. The subset's own `📚️examples/🎬️demo` facet
            // (`crate::examples::...`, real content, pre-existing) is the
            // modern, role-agnostic replacement surface for this.
            // 🧵️ Every one of this app's 33 tool ids is UI-reachable, so the classification is set in one
            // sweep. `.action_interactive_job(id, …)` cannot express it: it only mutates `self.actions`
            // (`🔌️plugin/🦀️.rs:5166-5172`), so for the 32 ids declared as COMMANDS it was a silent no-op —
            // `migrated_tool_ids()` (`:12058`) then saw an empty set and `validate_tool_job_rows` rejected
            // every proof row with `interactive-job.catalog-authority`. `interactive_jobs` covers actions,
            // window actions, commands and mode commands alike. The exact split is pinned independently by
            // `📦️packages/🟦️typescript/📜️script.ts`'s route audit, which fails if it ever drifts.
            .interactive_jobs(InteractiveJobClassification::Migrated)
            .action_describe("addStep", LocalizedLabel::native("Adds a process step that applies one measure (such as a cut or a drill) with a workshop machine's capability at the given position on the stock.", "Fügt einen Prozessschritt hinzu, der eine Maßnahme (etwa einen Schnitt oder eine Bohrung) mit der Fähigkeit einer Werkstattmaschine an der angegebenen Position des Rohteils ausführt."))
            .action_describe("setStock", LocalizedLabel::native("Swaps the stock (the raw part, such as a box or cylinder) for one of the given kind and resets the whole process: every step and the timeline cursor are cleared.", "Tauscht das Rohteil (etwa Quader oder Zylinder) gegen eines der angegebenen Art und setzt den gesamten Prozess zurück: alle Schritte und der Zeitleisten-Cursor werden gelöscht."))
            .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole process document (stock, workshop and steps) with one of the plugin's bundled examples, by example id.", "Ersetzt das gesamte Prozessdokument (Rohteil, Werkstatt und Schritte) durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."))
            .action_describe("removeSelectedStep", LocalizedLabel::native("Removes the process step currently selected in the 3D view; nothing happens without a selection.", "Entfernt den aktuell in der 3D-Ansicht ausgewählten Prozessschritt; ohne Auswahl geschieht nichts."))
            .action_describe("exportModel", LocalizedLabel::native("Writes the processed model in the chosen format to a downloaded file on the user's machine.", "Schreibt das bearbeitete Modell im gewählten Format in eine heruntergeladene Datei auf dem Rechner des Nutzers."))
            .action_describe("loadModelRequest", LocalizedLabel::native("Opens the host's file picker for a STEP, OBJ, STL or GLB model; the chosen file is then imported as the new stock.", "Öffnet die Dateiauswahl des Hosts für ein STEP-, OBJ-, STL- oder GLB-Modell; die gewählte Datei wird dann als neues Rohteil importiert."))
            .action_describe("setSnapshot", LocalizedLabel::native("Replaces the whole process document with the supplied document; nothing of the previous stock, workshop or steps is kept.", "Ersetzt das gesamte Prozessdokument durch das übergebene; von Rohteil, Werkstatt und Schritten bleibt nichts erhalten."))
            .action_describe("addWorkshopMachine", LocalizedLabel::native("Adds a machine from the catalogue to the workshop under the given machine id, with the capabilities that catalogue entry provides.", "Fügt der Werkstatt eine Maschine aus dem Katalog unter der angegebenen Maschinen-Id hinzu, mit den Fähigkeiten dieses Katalogeintrags."))
            .action_describe("removeWorkshopMachine", LocalizedLabel::native("Removes one machine by id from the workshop.", "Entfernt eine Maschine anhand ihrer Id aus der Werkstatt."))
            .action_describe("updateWorkshopMachine", LocalizedLabel::native("Updates one workshop machine to the supplied definition, changing only its name, icon or capabilities where they differ.", "Aktualisiert eine Werkstattmaschine auf die übergebene Definition und ändert nur Name, Symbol oder Fähigkeiten, wo sie abweichen."))
            .action_describe("importModelFile", LocalizedLabel::native("Replaces the whole document with a stock read from the given model file (name and data-URL payload) and clears the process timeline.", "Ersetzt das gesamte Dokument durch ein Rohteil aus der angegebenen Modelldatei (Name und Data-URL-Inhalt) und leert die Prozesszeitleiste."))
            .action_describe("removeStep", LocalizedLabel::native("Removes one process step by id from the timeline.", "Entfernt einen Prozessschritt anhand seiner Id aus der Zeitleiste."))
            .action_describe("moveStep", LocalizedLabel::native("Moves one process step to a new position (index) in the timeline, changing the order in which it is applied.", "Verschiebt einen Prozessschritt an eine neue Position (Index) der Zeitleiste und ändert damit die Reihenfolge seiner Anwendung."))
            .action_describe("updateStep", LocalizedLabel::native("Updates one process step to the supplied definition, changing only the fields that differ.", "Aktualisiert einen Prozessschritt auf die übergebene Definition und ändert nur die abweichenden Felder."))
            .action_describe("setStepEnabled", LocalizedLabel::native("Sets one process step on or off, to exactly the value passed, so repeating it changes nothing; a disabled step stays in the timeline but is not applied to the stock.", "Schaltet einen Prozessschritt genau nach dem übergebenen Wert ein oder aus, sodass eine Wiederholung nichts ändert; ein deaktivierter Schritt bleibt in der Zeitleiste, wird aber nicht auf das Rohteil angewendet."))
            .action_describe("patchInspector", LocalizedLabel::native("Edits one inspector field of the stock or a workshop machine (a label, a stock position or a capability parameter) with a number or text value.", "Bearbeitet ein Inspektorfeld des Rohteils oder einer Werkstattmaschine (Bezeichnung, Rohteilposition oder Fähigkeitsparameter) mit einem Zahlen- oder Textwert."))
            .action_describe("setCursor", LocalizedLabel::native("Sets how many process steps the timeline cursor has applied, which the 3D view shows the stock after.", "Legt fest, wie viele Prozessschritte der Zeitleisten-Cursor angewendet hat, wonach die 3D-Ansicht das Rohteil zeigt."))
            .action_describe("stepCursor", LocalizedLabel::native("Moves the timeline cursor by the given number of steps forward or back (never below zero).", "Bewegt den Zeitleisten-Cursor um die angegebene Zahl von Schritten vor oder zurück (nie unter null)."))
            .action_describe("stepCursorBack", LocalizedLabel::native("Moves the timeline cursor one step back so the 3D view shows the stock before the last applied step.", "Bewegt den Zeitleisten-Cursor einen Schritt zurück, sodass die 3D-Ansicht das Rohteil vor dem zuletzt angewendeten Schritt zeigt."))
            .action_describe("stepCursorForward", LocalizedLabel::native("Moves the timeline cursor one step forward so the 3D view shows the stock after the next step.", "Bewegt den Zeitleisten-Cursor einen Schritt vor, sodass die 3D-Ansicht das Rohteil nach dem nächsten Schritt zeigt."))
            .action_destructive("setStock")
            .action_destructive("importModelFile")
            .build_definition()
}
//#endregion 🔖️Manifest


//#region 🔧️Behavior






/// 🔌️ Refreshes contributed `process.machines` catalogs when the host pushes a new catalogue.

















//#endregion 🔧️Behavior

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors `crate::
/// artifact_kind()`'s literal for `"3d.process"` (schema/media type/export+import formats/presentation
/// fields copied verbatim), plus the two workflow ports: `geometry:in` (Many, unrequired — accepts
/// upstream geometry producers, e.g. cad/lowpoly) and `brep:out` (Many, unrequired, `kind_id:
/// "3d.process"` — reusing the artifact kind already declared, never a second `.artifact_kind(...)` call).
pub fn process3d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        artifact_schema: semio_s_artifact_process_process3d_model::snapshot::PROCESS_3D_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
        ports: vec![
            semio_framework_plugin::MediaPortSpec {
                id: "geometry:in".into(),
                label: "Geometry".into(),
                direction: semio_framework_plugin::MediaPortDirection::In,
                media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Any },
                kind_id: None,
                required: false,
                multiplicity: semio_framework_plugin::PortMultiplicity::Many,
            },
            semio_framework_plugin::MediaPortSpec {
                id: "brep:out".into(),
                label: "Brep".into(),
                direction: semio_framework_plugin::MediaPortDirection::Out,
                media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
                kind_id: Some("3d.process".into()),
                required: false,
                multiplicity: semio_framework_plugin::PortMultiplicity::Many,
            },
        ],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "3d.process".into(), name: "3D Process".into(), dimension: "3d".into(), component_kind: "process3d".into() },
    }
}
//#endregion 🔖️Io
