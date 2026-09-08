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

use crate::op::Process3dMutation;
use crate::{Capability, CapabilityRule, MachineCatalog, MachineCatalogs, MeasureRecipe, Process3dSnapshot, ProcessMeasure, ProcessStep, StepOrigin, Stock, WorkingSolid, WorkshopMachine};
use crate::editor::process3d::commands::{camera, contribution, cursor, document, engagement, inspector, locale, media, step, stock, sun, utility, workshop, world};
use crate::editor::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::editor::process3d::modes::edit;
use crate::editor::process3d::modes::edit::windows::workpiece;
use crate::editor::process3d::panels::{catalogue, document as document_panel, inspection, workshop as workshop_panel};
use crate::editor::process3d::presence::{Process3dPresence, Process3dPresenceMutation};
use crate::editor::process3d::terminology::process3d_labels;
use semio_framework::kernel::Effect;
use semio_framework::{DslValue, InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, AppActionRegistry, AppDefinition, AppOperationContext, ArtifactEditor, ArtifactKindSpec, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest,
    ArtifactToolFactoryRegistry, ArtifactView, CommandDefinition, ConfigView, ContextMenuItemSpec, ContextMenuRequest, Dialect, DraftView, Editor, EditorApp, Emit, Fault, FaultCode, FaultOrigin, GranularityDefinition, HierarchyProvider, HoverSpec,
    InteractionDefinition, InteractionRef, Label, LocalizedLabel, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, Menu, MergeMode, NoDraft, NoDraftMutation, OsMediaCapability, SelectionMethod, SelectionMode, SelectionSpec, UtilityCategory, UtilityDefinition, WindowMeasure,
};
use std::collections::HashMap;
use store::ArtifactPack;
use store::EngineHandles;

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_APP_ID: &str = "process3d-play";
const PROCESS_3D_PLAY_CONTROLLER_ID: &str = "process3d-play";
/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): the app's sole interaction domain — the
/// stock, every process step, and every installed workshop machine share the "object" granularity
/// (as their raw ids / `"machine:{id}"`), the processed mesh's picked faces use "face" (u32 ids
/// stringified at the `InteractionTarget` boundary). Flat hierarchy: no cross-object parent/child
/// structure to declare a topology for.
pub const PROCESS3D_INTERACTION_DOMAIN: &str = "geometry";

/// 🕹️ Owned snapshot of `InteractionView::selection(PROCESS3D_INTERACTION_DOMAIN).ids`, read once per
/// dispatch by `ArtifactEditor::handle` and threaded through `Process3dDispatchCtx` to the one command
/// handler that needs it (`remove_selected_step`) — mirrors `📐️cad`'s own `CadInteractionSnapshot`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Process3dInteractionSnapshot {
    pub ids: Vec<String>,
}

/// 🕹️ `app_commands!`'s `ctx` payload — every command handler now takes this fourth parameter (most
/// ignore it via `_ctx`), see `🔖️KeyedAndContextualForms` in the SDK's `app_commands!` doc comment.
pub struct Process3dDispatchCtx {
    pub interaction: Process3dInteractionSnapshot,
}

/// 🕹️ The `"geometry"` domain declaration: object granularity (stock/step/machine ids, the domain
/// default) plus face granularity (u32 mesh face ids, stringified at the `InteractionTarget`
/// boundary). Flat hierarchy — no cross-object parent/child structure.
fn process3d_interaction_definition() -> InteractionDefinition {
    InteractionDefinition {
        id: PROCESS3D_INTERACTION_DOMAIN.into(),
        label: LocalizedLabel::native("Geometry", "Geometrie"),
        granularities: vec![
            GranularityDefinition { id: "object".into(), label: LocalizedLabel::native("Object", "Objekt"), icon_id: "box".into() },
            GranularityDefinition { id: "face".into(), label: LocalizedLabel::native("Face", "Fläche"), icon_id: "square".into() },
        ],
        hierarchy: HierarchyProvider::Flat,
        hover: HoverSpec::default(),
        selection: SelectionSpec { modes: vec![SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
    }
}
pub const PROCESS3D_EXAMPLE_TIMBER: &str = "timber-beam-joinery";
pub const PROCESS3D_EXAMPLE_PLATE: &str = "drilled-plate";
pub use catalogue::PROCESS_3D_PLAY_BODY_CATALOGUE;
pub use document_panel::PROCESS_3D_PLAY_BODY_DOCUMENT;
pub use inspection::PROCESS_3D_PLAY_BODY_INSPECTION;
pub use workpiece::PROCESS_3D_PLAY_BODY_MAIN;
pub use workshop_panel::PROCESS_3D_PLAY_BODY_WORKSHOP;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`☑️options/*`, `📌️panels/*`, `🎮️commands/*`) builds its `on_change`/item actions with.
pub fn process3d_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(PROCESS_3D_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes.try_push(node).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
}

/// 🏷️ Admits resolved app text into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::Label> {
    semio_framework_plugin::plugin_app_close_prelude::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI label admission failed"))
}

/// 📇️ A non-palette action declaration (dispatched by UI wiring/keybindings, never surfaced in the
/// command palette) with the given execution kind.
fn internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::bounded_catalog(id, label, kind) }
}

/// 🧰️ Host effect that programmatically switches the workpiece window's active utility — the active
/// utility is also mirrored into `Process3dConfig::active_utility_id` (via `SetActiveUtility`) for
/// rendering, but the window chrome itself is still driven by this host effect. Shared by
/// `🎮️commands/🎛️engagement` and `🎮️commands/🌍️world`.
pub fn set_active_utility_effect(utility: &str) -> Effect {
    Effect::SetActiveUtility { window_id: workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into(), utility_id: utility.into() }
}

/// 🎨️ `tree_item_with_action` (SDK)? carries no icon slot, so this app-wide wrapper layers `icon_id` on
/// top via struct-update syntax — shared by the `🛍️catalogue` and `🛠️workshop` panels.
pub fn iconed_tree_item_with_action(
    id: impl AsRef<str>,
    label: impl AsRef<str>,
    icon_id: &str,
    action: semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut node = semio_framework_plugin::tree_item_with_action(id, ui_label(label)?, None, action?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(semio_framework_plugin::UiText::try_from_str(icon_id).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.tree-item.icon", "fixed tree-item icon admission failed"))?);
    }
    Ok(node)
}

/// 🔁️ Builds a `Effect::LoadDocument` for `document` — the sanctioned non-history "replace the
/// whole document" gesture (`ArtifactStore::reset`, applied host-side) every wholesale document-swap
/// command (`🎮️commands/🗿️artifact`, `🎮️commands/🪵️stock`, `🎮️commands/📤️media`, `import_media`'s
/// `geometry:in`) uses instead of the banned whole-snapshot mutation. The spr is a fresh, edit-free
/// op-log — a genesis envelope with no history to encode.
pub fn reset_process3d_document_effect(document: &Process3dSnapshot) -> Effect {
    let pack = <Process3dSnapshot as ArtifactPack>::encode_pack(document);
    let envelope = store::create_document_envelope::<Process3dSnapshot, Process3dMutation>(crate::PROCESS_3D_SCHEMA, "process3d", document.clone(), None);
    let spr = semio_framework_plugin::resolve_ready(store::print_document_spr(&envelope)).expect("process3d document spr encode is infallible for a fresh, edit-free envelope");
    Effect::LoadDocument { pack, spr }
}

/// 🚨 Typed host-action decoding fault with one stable app-specific code.
fn process3d_action_fault(action: &str, detail: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new("process3d.action.invalid"), format!("action '{action}': {}", detail.into()))
}

//#endregion 🔖️Constants

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
        "setActiveUtility" as "active-utility" => set_active_utility::SetActiveUtility,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "engagementAbort" as "engagement-abort" => engagement_abort::EngagementAbort,
        "setCamera" as "camera" => set_camera::SetCamera,
        "toggleSun" as "toggle-sun" => toggle_sun::ToggleSun,
        "setSunAzimuth" as "sun-azimuth" => set_sun_azimuth::SetSunAzimuth,
        "setSunElevation" as "sun-elevation" => set_sun_elevation::SetSunElevation,
        "setSunIntensity" as "sun-intensity" => set_sun_intensity::SetSunIntensity,
        "setLocale" as "locale" => set_locale::SetLocale,
        "setContributions" as "contributions" => set_contributions::SetContributions,
        "exportModel" as "export-model" => export_model::ExportModel,
        "loadModelRequest" as "load-model-request" => load_model_request::LoadModelRequest,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use camera::set_camera;
use contribution::set_contributions;
use cursor::{set_cursor, step_cursor, step_cursor_back, step_cursor_forward};
use document::{set_active_example, set_snapshot};
use engagement::{engagement_abort, engagement_input, engagement_submit};
use inspector::patch_inspector;
use locale::set_locale;
use media::{export_model, import_model_file, load_model_request};
use step::{add_step, move_step, remove_selected_step, remove_step, set_step_enabled, update_step};
use stock::set_stock;
use sun::{set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use utility::set_active_utility;
use workshop::{add_workshop_machine, remove_workshop_machine, update_workshop_machine};
use world::{world_face_drag_end, world_pointer_down};
//#endregion 🔖️Commands

//#region 🔖️Process3dPlayApp
/// 🧪️ B1: unit struct — every former `Process3dRuntime` field now lives in `config::Process3dConfig`
/// (see `ArtifactEditor::Config`), written through `config::Process3dConfigMutation`s.
#[derive(Default)]
pub struct Process3dPlayApp;

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
const PROCESS3D_RESUMABLE_TOOL_IDS: &[&str] = &[
    "setActiveUtility",
    "engagementInput",
    "toggleSun",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setLocale",
    "setContributions",
];
const PROCESS3D_RETAINED_PAYLOAD_SCHEMA: &str = "process.3d.tool-command.v1";
const PROCESS3D_RETAINED_RAW_BYTES: usize = 8_192;
const PROCESS3D_RETAINED_WORK_ITEMS: usize = 64;
const PROCESS3D_SCAN_BYTES: usize = 256;
const PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES: usize = 16_384;

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

fn process3d_resumable_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(PROCESS3D_RETAINED_RAW_BYTES, 64, 1, 16_384, 7_500, 1, 1)
}



fn process3d_string_units(value: &str) -> usize {
    value.len().div_ceil(PROCESS3D_SCAN_BYTES).max(1)
}

fn process3d_resumable_extent(command: &Process3dCommand, _snapshot: &Process3dSnapshot, config: &Process3dConfig, _interaction: &protocol::InteractionState) -> Option<usize> {
    let value = match command {
        Process3dCommand::SetActiveUtility(payload) => payload.utility_id.as_str(),
        Process3dCommand::EngagementInput(payload) => payload.value.as_str(),
        Process3dCommand::SetLocale(payload) => payload.value.as_str(),
        Process3dCommand::SetContributions(payload) => payload.json.as_str(),
        Process3dCommand::ToggleSun(_) | Process3dCommand::SetSunAzimuth(_) | Process3dCommand::SetSunElevation(_) | Process3dCommand::SetSunIntensity(_) => config.sun_color.as_str(),
        _ => return None,
    };
    let extent = process3d_string_units(value);
    (extent <= PROCESS3D_RETAINED_WORK_ITEMS && value.len() <= PROCESS3D_RETAINED_RAW_BYTES).then_some(extent)
}

fn process3d_retained_reduce(
    command: &Process3dCommand,
    snapshot: &Process3dSnapshot,
    config: &Process3dConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<Process3dMutation, Process3dConfigMutation, NoDraftMutation>, Fault> {
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config };
    let selection = interaction.selection.get(PROCESS3D_INTERACTION_DOMAIN);
    let mut ctx = Process3dDispatchCtx { interaction: Process3dInteractionSnapshot { ids: selection.map(|selection| selection.ids.clone()).unwrap_or_default() } };
    command.dispatch(&doc, &cfg, &mut ctx)
}

fn process3d_resumable_bytes<'a>(command: &'a Process3dCommand, config: &'a Process3dConfig) -> Option<&'a [u8]> {
    match command {
        Process3dCommand::SetActiveUtility(payload) => Some(payload.utility_id.as_bytes()),
        Process3dCommand::EngagementInput(payload) => Some(payload.value.as_bytes()),
        Process3dCommand::SetLocale(payload) => Some(payload.value.as_bytes()),
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
            Process3dCommand::SetActiveUtility(payload) => Process3dConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() },
            Process3dCommand::EngagementInput(payload) => Process3dConfigMutation::SetEngagementInput { value: payload.value.clone() },
            Process3dCommand::ToggleSun(_) => Process3dConfigMutation::SetSun { enabled: !config.sun_enabled, azimuth: config.sun_azimuth, elevation: config.sun_elevation, intensity: config.sun_intensity, color: config.sun_color.clone() },
            Process3dCommand::SetSunAzimuth(payload) => Process3dConfigMutation::SetSun { enabled: config.sun_enabled, azimuth: payload.value, elevation: config.sun_elevation, intensity: config.sun_intensity, color: config.sun_color.clone() },
            Process3dCommand::SetSunElevation(payload) => Process3dConfigMutation::SetSun { enabled: config.sun_enabled, azimuth: config.sun_azimuth, elevation: payload.value, intensity: config.sun_intensity, color: config.sun_color.clone() },
            Process3dCommand::SetSunIntensity(payload) => Process3dConfigMutation::SetSun { enabled: config.sun_enabled, azimuth: config.sun_azimuth, elevation: config.sun_elevation, intensity: payload.value, color: config.sun_color.clone() },
            Process3dCommand::SetLocale(payload) => Process3dConfigMutation::SetLocale { value: payload.value.clone() },
            Process3dCommand::SetContributions(payload) => Process3dConfigMutation::SetContributions { json: payload.json.clone() },
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

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<Process3dPlayApp>>) -> Result<ArtifactCommandWorkStep<EditorApp<Process3dPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { command, snapshot, config, history: _history, interaction, hover: _hover, context: _context, operation: _operation } = *input;
        if self.complete {
            return Err(Fault::from("process3d-retained-work-repeated"));
        }
        let extent = process3d_resumable_extent(command, snapshot, config, interaction).ok_or_else(|| Fault::from("process3d-retained-work-extent-overflow"))?;
        if extent != self.extent {
            return Err(Fault::from("process3d-retained-work-extent-drift"));
        }
        if self.cursor > extent {
            return Err(Fault::from("process3d-retained-checkpoint-cursor-out-of-range"));
        }
        if self.cursor < extent {
            self.observe_input(command, config)?;
            self.cursor += 1;
            return Ok(ArtifactCommandWorkStep::Progress { stage: "process3d-config-prepare", preview: b"{\"en\":\"Preparing configuration\",\"de\":\"Konfiguration wird vorbereitet\"}" });
        }
        self.complete = true;
        self.complete_emit(command, config).map(ArtifactCommandWorkStep::Complete)
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 40 {
            return Err(Fault::from("process3d-retained-checkpoint-capacity"));
        }
        target[..4].copy_from_slice(b"P3C1");
        target[4] = u8::from(self.complete);
        target[5] = 0;
        target[8..16].copy_from_slice(&(self.cursor as u64).to_le_bytes());
        target[16..24].copy_from_slice(&self.digest.to_le_bytes());
        target[24..32].copy_from_slice(&process3d_tool_identity(self.tool_id).to_le_bytes());
        target[32..40].copy_from_slice(&(self.extent as u64).to_le_bytes());
        Ok(40)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 40 || &checkpoint[..4] != b"P3C1" || checkpoint[4] > 1 || checkpoint[5] != 0 {
            return Err(Fault::from("process3d-retained-checkpoint-invalid"));
        }
        let identity = u64::from_le_bytes(checkpoint[24..32].try_into().map_err(|_| Fault::from("process3d-retained-checkpoint-identity"))?);
        if identity != process3d_tool_identity(self.tool_id) {
            return Err(Fault::from("process3d-retained-checkpoint-tool-mismatch"));
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

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        if self.closing {
            InteractiveJobCloseStep::Complete
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
        if input.declared_bytes() > PROCESS3D_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("Process3d bounded command rejects oversized wire or unsupported checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl ArtifactOwnedToolJobFactory for Process3dBoundedCommandJobFactory {
    type Owner = EditorApp<Process3dPlayApp>;
    const TOOL_IDS: &'static [&'static str] = PROCESS3D_BOUNDED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = crate::PROCESS_3D_SCHEMA;
    /// 🛤️ The lane each tool publishes on, read off what its `🎮️commands/*` handler actually emits.
    /// `HostOnly` is a whole-document replacement or a shell round-trip carried as an `Effect`
    /// (`setSnapshot`/`setActiveExample`/`setStock`/`importModelFile` build
    /// `reset_process3d_document_effect`, `exportModel` a `DownloadMediaExport`, `loadModelRequest` a
    /// `RequestFileOpen`) rather than a store edit; `Artifact` is every verb that emits a
    /// `Process3dMutation` against the timeline, the stock or the workshop, admitted by
    /// `Process3dArtifactPreparationFactory`; `engagementSubmit` publishes on BOTH lanes because its
    /// `back`/`forward`/`all` words move the document cursor while every word clears the config-lane
    /// engagement input.
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementAbort", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "loadModelRequest", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSnapshot", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveExample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addWorkshopMachine", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeWorkshopMachine", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "updateWorkshopMachine", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "removeSelectedStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "moveStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "updateStep", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setStepEnabled", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setStock", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchInspector", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "worldPointerDown", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "worldFaceDragEnd", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "importModelFile", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "exportModel", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCursor", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "stepCursor", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "stepCursorBack", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "stepCursorForward", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
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
        if input.declared_bytes() > PROCESS3D_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
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
    const DOCUMENT_SCHEMA: &'static str = crate::PROCESS_3D_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveUtility", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "toggleSun", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSunAzimuth", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSunElevation", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSunIntensity", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 📬️ConfigStorePreparation
struct Process3dConfigStorePreparationFactory;

struct Process3dConfigStorePreparation {
    base: Option<store::SnapshotRead<Process3dConfig>>,
    mutation: Option<Process3dConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Process3dConfig, Process3dConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

fn process3d_config_retained_bytes(config: &Process3dConfig) -> Option<usize> {
    [config.engagement_input.len(), config.sun_color.len(), config.active_utility_id.len(), config.locale.len(), config.contributions_json.len()].into_iter().try_fold(0usize, usize::checked_add)
}

fn process3d_config_mutation_retained_bytes(mutation: &Process3dConfigMutation) -> usize {
    match mutation {
        Process3dConfigMutation::SetEngagementInput { value } | Process3dConfigMutation::SetActiveUtility { utility_id: value } | Process3dConfigMutation::SetLocale { value } => value.len(),
        Process3dConfigMutation::SetSun { color, .. } => color.len(),
        Process3dConfigMutation::SetContributions { json } => json.len(),
        Process3dConfigMutation::SetCamera { .. } => 0,
    }
}

fn admit_process3d_config_mutation(mutation: &Process3dConfigMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = process3d_config_mutation_retained_bytes(mutation);
    if retained_bytes > PROCESS3D_RETAINED_RAW_BYTES {
        return Err("Process3d config mutation exceeds its fixed retained preparation envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn prepare_process3d_config(base: &Process3dConfig, mutation: Process3dConfigMutation) -> Result<(Process3dConfig, Vec<Process3dConfigMutation>, Process3dConfigMutation), String> {
    admit_process3d_config_mutation(&mutation)?;
    if process3d_config_retained_bytes(base).is_none_or(|bytes| bytes > PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES) {
        return Err("Process3d config base exceeds its fixed retained preparation envelope".into());
    }
    let inverse = match &mutation {
        Process3dConfigMutation::SetEngagementInput { .. } => Process3dConfigMutation::SetEngagementInput { value: base.engagement_input.clone() },
        Process3dConfigMutation::SetCamera { .. } => Process3dConfigMutation::SetCamera { position: base.camera_position, target: base.camera_target, fov: base.camera_fov },
        Process3dConfigMutation::SetSun { .. } => Process3dConfigMutation::SetSun { enabled: base.sun_enabled, azimuth: base.sun_azimuth, elevation: base.sun_elevation, intensity: base.sun_intensity, color: base.sun_color.clone() },
        Process3dConfigMutation::SetActiveUtility { .. } => Process3dConfigMutation::SetActiveUtility { utility_id: base.active_utility_id.clone() },
        Process3dConfigMutation::SetLocale { .. } => Process3dConfigMutation::SetLocale { value: base.locale.clone() },
        Process3dConfigMutation::SetContributions { .. } => Process3dConfigMutation::SetContributions { json: base.contributions_json.clone() },
    };
    let mut post = base.clone();
    match &mutation {
        Process3dConfigMutation::SetEngagementInput { value } => post.engagement_input = value.clone(),
        Process3dConfigMutation::SetCamera { position, target, fov } => {
            post.camera_position = *position;
            post.camera_target = *target;
            post.camera_fov = *fov;
        }
        Process3dConfigMutation::SetSun { enabled, azimuth, elevation, intensity, color } => {
            post.sun_enabled = *enabled;
            post.sun_azimuth = *azimuth;
            post.sun_elevation = *elevation;
            post.sun_intensity = *intensity;
            post.sun_color = color.clone();
        }
        Process3dConfigMutation::SetActiveUtility { utility_id } => post.active_utility_id = utility_id.clone(),
        Process3dConfigMutation::SetLocale { value } => post.locale = value.clone(),
        Process3dConfigMutation::SetContributions { json } => post.contributions_json = json.clone(),
    }
    if process3d_config_retained_bytes(&post).is_none_or(|bytes| bytes > PROCESS3D_CONFIG_STORE_MAXIMUM_BYTES) {
        return Err("Process3d config post-state exceeds its fixed retained preparation envelope".into());
    }
    Ok((post, vec![inverse], mutation))
}

fn process3d_config_store_edit(forward: Process3dConfigMutation, inverse: Vec<Process3dConfigMutation>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<Process3dConfigMutation> {
    let id = format!("process3d-config-retained-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

impl store::ArtifactStoreOneItemPreparationFactory<Process3dConfig, Process3dConfigMutation> for Process3dConfigStorePreparationFactory {
    fn preflight(&self, mutation: &Process3dConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Process3d config preparation rejected its lane or description envelope".into());
        }
        admit_process3d_config_mutation(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Process3dConfig, Process3dConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Process3dConfig, Process3dConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<Process3dConfig, Process3dConfigMutation>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Process3dConfigStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Process3dConfig, Process3dConfigMutation> for Process3dConfigStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "Process3d config preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "Process3d config preparation lost its mutation owner".to_string())?;
        let (post, inverse, forward) = prepare_process3d_config(base.get(), mutation)?;
        let authority = self.authority.as_ref().ok_or_else(|| "Process3d config preparation lost its Store authority".to_string())?;
        let edit = process3d_config_store_edit(forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Process3dConfig, Process3dConfigMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Process3dConfig, Process3dConfigMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Process3d config preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
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
/// maximum_bytes: TYPED_OPERATION_RESULT_PAGE_BYTES }` (4 KiB), so a gate that scaled with the
/// document — `grant.maximum_bytes < measured_base_bytes` — would go `Blocked` forever the moment a
/// timeline or a workshop outgrew one page, stalling the operation instead of failing it. The base's
/// own size is a VALIDATION (`process3d_document_bytes`, rejected past
/// `PROCESS3D_DOCUMENT_MAXIMUM_BYTES`), never the gate.
const PROCESS3D_DOCUMENT_GRANT_BYTES: usize = 4_096;

struct Process3dArtifactPreparationFactory;

struct Process3dArtifactPreparation {
    base: Option<store::SnapshotRead<Process3dSnapshot>>,
    mutation: Option<Process3dMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    candidate: Option<(Process3dSnapshot, Vec<Process3dMutation>, Process3dMutation)>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<Process3dSnapshot, Process3dMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    cancelled: bool,
    closing: bool,
}

/// 📏️ One text field's own retained cost — rejected rather than truncated past the fixed envelope.
fn process3d_text_bytes(value: &str) -> Result<usize, String> {
    if value.len() > PROCESS3D_DOCUMENT_TEXT_BYTES {
        return Err("Process3d document carries a text field beyond its encoded text envelope".into());
    }
    Ok(value.len())
}

fn process3d_child_bytes<S>(child: &store::ArtifactChild<S>) -> Result<usize, String> {
    Ok(process3d_text_bytes(&child.child_id)?.saturating_add(process3d_text_bytes(&child.target.to_uri())?))
}

fn process3d_solid_bytes(solid: &WorkingSolid) -> Result<usize, String> {
    match solid {
        WorkingSolid::Box { .. } | WorkingSolid::Cylinder { .. } | WorkingSolid::Sphere { .. } => Ok(0),
        WorkingSolid::ImportedMesh { mesh_url } => process3d_text_bytes(mesh_url),
        WorkingSolid::ImportedSolid { solid_handle } => process3d_text_bytes(solid_handle),
    }
}

fn process3d_measure_bytes(measure: &ProcessMeasure) -> Result<usize, String> {
    match measure {
        ProcessMeasure::Cut { tool, .. } => process3d_solid_bytes(tool),
        ProcessMeasure::Attach { component, .. } => process3d_solid_bytes(component),
        ProcessMeasure::Drill { .. } => Ok(0),
    }
}

fn process3d_origin_bytes(origin: Option<&StepOrigin>) -> Result<usize, String> {
    match origin {
        Some(origin) => Ok(process3d_text_bytes(&origin.machine_id)?.saturating_add(process3d_text_bytes(&origin.capability_id)?)),
        None => Ok(0),
    }
}

fn process3d_step_bytes(step: &ProcessStep) -> Result<usize, String> {
    Ok(process3d_text_bytes(&step.id)?
        .saturating_add(process3d_text_bytes(&step.label)?)
        .saturating_add(process3d_origin_bytes(step.origin.as_ref())?)
        .saturating_add(process3d_measure_bytes(&step.measure)?)
        .saturating_add(size_of::<ProcessStep>()))
}

fn process3d_recipe_bytes(recipe: &MeasureRecipe) -> Result<usize, String> {
    let parts: [&str; 3] = match recipe {
        MeasureRecipe::DiscCut { diameter, kerf } => [diameter, kerf, ""],
        MeasureRecipe::BladeCut { kerf, length, depth } => [kerf, length, depth],
        MeasureRecipe::PocketCut { diameter, depth } => [diameter, depth, ""],
        MeasureRecipe::BoreDrill { radius, depth } => [radius, depth, ""],
        MeasureRecipe::CylinderAttach { radius, length } => [radius, length, ""],
        MeasureRecipe::BoxAttach { width, depth, height } => [width, depth, height],
    };
    parts.iter().try_fold(0usize, |bytes, part| -> Result<usize, String> { Ok(bytes.saturating_add(process3d_text_bytes(part)?)) })
}

fn process3d_capability_bytes(capability: &Capability) -> Result<usize, String> {
    let parameters = capability.parameters.iter().try_fold(0usize, |bytes, parameter| -> Result<usize, String> { Ok(bytes.saturating_add(process3d_text_bytes(&parameter.id)?).saturating_add(process3d_text_bytes(&parameter.label)?)) })?;
    let rules = capability.rules.iter().try_fold(0usize, |bytes, rule| -> Result<usize, String> {
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

fn process3d_capabilities_bytes(capabilities: &[Capability]) -> Result<usize, String> {
    capabilities.iter().try_fold(0usize, |bytes, capability| -> Result<usize, String> { Ok(bytes.saturating_add(process3d_capability_bytes(capability)?)) })
}

fn process3d_machine_bytes(machine: &WorkshopMachine) -> Result<usize, String> {
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

fn process3d_stock_bytes(stock: &Stock) -> Result<usize, String> {
    Ok(process3d_text_bytes(&stock.id)?.saturating_add(process3d_text_bytes(&stock.label)?).saturating_add(process3d_solid_bytes(&stock.solid)?))
}

/// 📏️ The retained footprint of one document base — rejected rather than truncated when the timeline,
/// the workshop, or any single id/label/handle outgrows the fixed envelope.
fn process3d_document_bytes(document: &Process3dSnapshot) -> Result<usize, String> {
    let items = document.step_payloads.len().saturating_add(document.tool_solids.len()).saturating_add(document.workshop.machines.iter().fold(0usize, |items, machine| items.saturating_add(process3d_machine_items(machine))));
    if items > PROCESS3D_DOCUMENT_MAXIMUM_ITEMS {
        return Err("Process3d document base exceeds its retained item envelope".into());
    }
    let steps = document.step_payloads.iter().try_fold(0usize, |bytes, step| -> Result<usize, String> { Ok(bytes.saturating_add(process3d_step_bytes(step)?)) })?;
    let machines = document.workshop.machines.iter().try_fold(0usize, |bytes, machine| -> Result<usize, String> { Ok(bytes.saturating_add(process3d_machine_bytes(machine)?)) })?;
    let tools = document.tool_solids.iter().try_fold(0usize, |bytes, tool| -> Result<usize, String> { Ok(bytes.saturating_add(process3d_child_bytes(tool)?)) })?;
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
        return Err("Process3d document base exceeds its retained byte envelope".into());
    }
    Ok(bytes)
}

/// 📏️ One semantic mutation's own retained footprint, shaped like what it actually addresses: a
/// single step, machine, stock field or cursor is one work item carrying that target's own text,
/// while a created machine or a replaced capability set is one item per capability leaf it carries.
fn process3d_mutation_footprint(mutation: &Process3dMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let (work_items, retained_bytes) = match mutation {
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
        Process3dMutation::ChangeCursor(_) => (1, 0),
    };
    let retained_bytes = retained_bytes.saturating_add(size_of::<Process3dMutation>());
    if work_items > PROCESS3D_DOCUMENT_MAXIMUM_ITEMS || retained_bytes > PROCESS3D_DOCUMENT_MAXIMUM_BYTES {
        return Err("Process3d document mutation exceeds its fixed one-item preparation envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items, retained_bytes })
}

/// 🧮️ Runs the mutation's own semantic `diff`/`inverse` against `base` and applies the resulting diff,
/// so the retained lane and the batch lane can never diverge. An `Error`/`Fatal` outcome (a duplicate
/// step id, a missing target) is a REJECTION here, not a silent no-op: `MutationOutcome::error`/
/// `fatal` force an EMPTY diff, and publishing anyway would write a no-op edit into history.
fn prepare_process3d_document(base: &Process3dSnapshot, mutation: Process3dMutation) -> Result<(Process3dSnapshot, Vec<Process3dMutation>, Process3dMutation), String> {
    process3d_mutation_footprint(&mutation)?;
    process3d_document_bytes(base)?;
    let outcome = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::diff(&mutation, base);
    if let Some(message) = outcome.messages().iter().find(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal)) {
        return Err(format!("Process3d document mutation was refused by its own vocabulary: {}", message.message));
    }
    let inverse = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::inverse(&mutation, base);
    let post = protocol::MutationDiff::apply(outcome.diff(), base).map_err(|error| format!("Process3d document mutation could not apply onto its exact base: {}", error.message))?;
    process3d_document_bytes(&post)?;
    Ok((post, inverse, mutation))
}

fn process3d_document_store_edit(forward: Process3dMutation, inverse: Vec<Process3dMutation>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<Process3dMutation> {
    let id = format!("process3d-document-retained-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

impl store::ArtifactStoreOneItemPreparationFactory<Process3dSnapshot, Process3dMutation> for Process3dArtifactPreparationFactory {
    fn preflight(&self, mutation: &Process3dMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Process3d document preparation rejected its lane or description envelope".into());
        }
        process3d_mutation_footprint(mutation)?;
        Ok(store::ArtifactStoreOneItemFootprint { work_items: 2, retained_bytes: PROCESS3D_DOCUMENT_GRANT_BYTES })
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<Process3dSnapshot, Process3dMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<Process3dSnapshot, Process3dMutation>>, store::ArtifactStoreOneItemPreparationRequest<Process3dSnapshot, Process3dMutation>> {
        if self.preflight(&request.mutation, request.description.as_deref(), request.lane).is_err()
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(Process3dArtifactPreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            candidate: None,
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes: 0,
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<Process3dSnapshot, Process3dMutation> for Process3dArtifactPreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        if self.candidate.is_none() {
            let base = self.base.as_ref().ok_or_else(|| "Process3d document preparation lost its exact base root".to_string())?.get();
            process3d_document_bytes(base)?;
            if grant.maximum_bytes < PROCESS3D_DOCUMENT_GRANT_BYTES {
                return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
            }
            let mutation = self.mutation.take().ok_or_else(|| "Process3d document preparation lost its mutation owner".to_string())?;
            self.candidate = Some(prepare_process3d_document(base, mutation)?);
            self.retained_bytes = PROCESS3D_DOCUMENT_GRANT_BYTES;
            self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: PROCESS3D_DOCUMENT_GRANT_BYTES as u64, digest: [0; 32] };
            return Ok(store::ArtifactStoreOneItemPreparationStep::Progress(self.checkpoint));
        }
        if grant.maximum_bytes < self.retained_bytes {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        let (post, inverse, forward) = self.candidate.take().ok_or_else(|| "Process3d document preparation lost its candidate".to_string())?;
        let authority = self.authority.as_ref().ok_or_else(|| "Process3d document preparation lost its Store authority".to_string())?;
        let prepared = authority.prepare_one_item(process3d_document_store_edit(forward, inverse, self.description.take(), authority), std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 2, completed_items: 2, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<Process3dSnapshot, Process3dMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<Process3dSnapshot, Process3dMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || !grant.permits_one() {
            return Ok(store::SnapshotRetirementStep::Blocked);
        }
        if self.prepared.is_some() || self.candidate.is_some() {
            if grant.maximum_bytes < self.retained_bytes {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            if self.prepared.take().is_none() {
                self.candidate = None;
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if let Some(mutation) = self.mutation.as_ref() {
            let bytes = process3d_mutation_footprint(mutation)?.retained_bytes;
            if grant.maximum_bytes < PROCESS3D_DOCUMENT_GRANT_BYTES {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.mutation = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if let Some(description) = self.description.as_ref() {
            let bytes = description.len();
            if grant.maximum_bytes < bytes {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.description = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Process3d document preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.candidate.is_none() && self.prepared.is_none()
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
        document_schema: "process.3d",
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
        document_schema: "process.3d",
        factory: "Process3dResumableCommandJobFactory",
        factory_type: Process3dResumableCommandJobFactory,
        tools: {
            "setActiveUtility" => ToolExecutionContract::resumable(8_192, 64, 1, 16_384, 7_500, 1, 1),
            "engagementInput" => ToolExecutionContract::resumable(8_192, 64, 1, 16_384, 7_500, 1, 1),
            "toggleSun" => ToolExecutionContract::resumable(8_192, 64, 1, 16_384, 7_500, 1, 1),
            "setSunAzimuth" => ToolExecutionContract::resumable(8_192, 64, 1, 16_384, 7_500, 1, 1),
            "setSunElevation" => ToolExecutionContract::resumable(8_192, 64, 1, 16_384, 7_500, 1, 1),
            "setSunIntensity" => ToolExecutionContract::resumable(8_192, 64, 1, 16_384, 7_500, 1, 1),
            "setLocale" => ToolExecutionContract::resumable(8_192, 64, 1, 16_384, 7_500, 1, 1),
            "setContributions" => ToolExecutionContract::resumable(8_192, 64, 1, 16_384, 7_500, 1, 1),
        }
    }
}
//#endregion 🧾️ProofCatalogs

/// 🧱️ Every window/panel body of the process3d editor, rendered against one already-resolved
/// `"geometry"` selection — shared by `render` (empty selection) and `render_with_request_context`
/// (the live selection) so there is exactly one body-key match in the app.
fn process3d_render_body(body_key: &str, doc: &Process3dSnapshot, config: &Process3dConfig, selected_ids: &[String]) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let labels = process3d_labels(config);
    let base_body_key = body_key.split_once(':').map_or(body_key, |(base, _)| base);
    match base_body_key {
        PROCESS_3D_PLAY_BODY_MAIN => workpiece::render(doc, config).map(semio_framework_plugin::built_to_component_tree),
        PROCESS_3D_PLAY_BODY_DOCUMENT => document_panel::render(doc, labels).map(semio_framework_plugin::built_to_component_tree),
        PROCESS_3D_PLAY_BODY_CATALOGUE => catalogue::render(doc, &config.contributions_json, labels).map(semio_framework_plugin::built_to_component_tree),
        PROCESS_3D_PLAY_BODY_WORKSHOP => workshop_panel::render(doc, &config.contributions_json, labels).map(semio_framework_plugin::built_to_component_tree),
        PROCESS_3D_PLAY_BODY_INSPECTION => inspection::render(doc, selected_ids, labels).map(semio_framework_plugin::built_to_component_tree),
        _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
    }
}

impl ArtifactEditor for Process3dPlayApp {
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

    const DIALECT: Dialect = crate::PROCESS3D_DIALECT;

    const DOCUMENT_SCHEMA: &'static str = crate::PROCESS_3D_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(Process3dArtifactPreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
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
            return Err(Fault::from("process3d-command-tool-mismatch"));
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
        };
        let payload = ArtifactRetainedCommandPayload::try_new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs { command: *request.command, snapshot: request.snapshot, config: request.config, history: request.history, interaction_state: request.interaction_state, interaction_hover: request.interaction_hover, context: Some(request.context), operation: operation_context, completion: request.completion },
            Process3dCommand::command_id,
            PROCESS3D_RETAINED_RAW_BYTES,
            PROCESS3D_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }
    const REQUIRES_DOCUMENT_STORE_PUBLICATION_AUTHORITY: bool = true;

    fn build_envelope_decode_owner_bundle() -> Option<store::ArtifactEnvelopeDecodeOwnerBundle<Self::Snapshot, Self::Mutation>> {
        Some(crate::spr::process3d_envelope_decode_owner_bundle())
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(crate::spr::process3d_document_store_owners())
    }

    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_document_store_initialization_job(
        envelope: store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
    ) -> Result<semio_framework_plugin::ArtifactStoreInitializationJob<Self::Snapshot, Self::Mutation>, store::ArtifactEnvelope<Self::Snapshot, Self::Mutation>> {
        Ok(crate::spr::process3d_document_store_initialization_job(envelope, operation, generation))
    }

    fn validate_document_store_publication(operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, live_generation: semio_framework_job::Generation) -> Result<(), Fault> {
        crate::spr::process3d_validate_atomic_publication_authority(operation, generation, live_generation)
            .map_err(|code| Fault::new(FaultOrigin::App, FaultCode::new(code), "Process3d atomic publication authority is absent or stale"))
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(Box::new(semio_framework_plugin::ArtifactDocumentStoreDisposer::<Self::Snapshot, Self::Mutation>::new()))
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn app_schema() -> Option<::framework_schema::AppSchemaDescriptor> {
        Some(crate::editor::process3d::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> Process3dSnapshot {
        crate::schema::default_document()
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
            "brep:out" => match crate::io::export_process3d_model(&crate::process_working_scene_from_snapshot(doc.snapshot), doc.snapshot.resolved_up_to, "step")
                .map_err(|error| MediaError::Payload("brep:out".into(), error))?
            {
                Some(export) => {
                    let text = match export.data {
                        DslValue::String(text) => text,
                        other => semio_framework_os_kernel::json::to_json_string(&other),
                    };
                    Ok(semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep }, payload: MediaPayload::Structured { schema: "3d.process".into(), json: text } })
                }
                None => Err(MediaError::Payload("brep:out".into(), "kernel replay failed".into())),
            },
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(semio_framework_plugin::Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    /// 🌱️ `whole_document_operation` stays the trait default (`None`): per `📓️taxonomy.md`, whole-
    /// document replace has no in-history mutation at all (there is no import mutation by locked
    /// decision — every whole-document gesture below routes through `reset_process3d_document_effect`
    /// instead, a `Effect::LoadDocument`).
    ///
    /// 📥️ `geometry:in` (best-effort STEP-text import) replaces the whole document via a
    /// `Effect::LoadDocument` (whole-document replace has no in-history mutation); the inherited
    /// `document:in` default (which would decode a base64 pack via `whole_document_operation`) is
    /// unreachable now that `whole_document_operation` is `None`, so `document:in` is simply
    /// unimplemented here — overriding `import_media` shadows the trait's provided body for every port.
    fn import_media(port: &str, media: &semio_framework_plugin::Media, _doc: &ArtifactView<'_, Process3dSnapshot>) -> Result<Emit<Process3dMutation, Process3dConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "geometry:in" => {
                let MediaPayload::Structured { schema, json } = &media.payload else {
                    return Err(MediaError::Payload("geometry:in".into(), "expected a structured payload".into()));
                };
                if schema != crate::PROCESS_3D_SCHEMA && schema != "3d.process" {
                    return Err(MediaError::Payload("geometry:in".into(), format!("unrecognized schema: {schema}")));
                }
                // 📦️ `export_process3d_model("step")` hands back raw (non-base64) STEP text — the
                // "stdio.step" format is not binary — so this re-encodes it as base64 to satisfy
                // `import_process3d_model`'s `data:...,<base64>` expectation.
                let data_url = format!("data:application/octet-stream;base64,{}", base64_codec::base64_standard_encode(json.as_bytes()));
                match crate::io::import_process3d_model("geometry-in.step", &data_url) {
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
    fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<Self::Command, Fault> {
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
                let json = string_field("json").or_else(|| field("document").map(semio_framework_os_kernel::json::to_json_string)).unwrap_or_default();
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
                    .map(<WorkshopMachine as semio_framework_os_kernel::FromValue>::from_value)
                    .transpose()
                    .map_err(|error| process3d_action_fault(action, format!("invalid 'machine': {error}")))?
                    .unwrap_or(WorkshopMachine { id: String::new(), label: String::new(), icon_id: String::new(), catalog_id: None, capabilities: Vec::new() }),
            })),
            "removeStep" => Ok(Process3dCommand::RemoveStep(remove_step::RemoveStep { id: string_field("id").unwrap_or_default() })),
            "removeSelectedStep" => Ok(Process3dCommand::RemoveSelectedStep(remove_selected_step::RemoveSelectedStep {})),
            "moveStep" => Ok(Process3dCommand::MoveStep(move_step::MoveStep { id: string_field("id").unwrap_or_default(), index: unsigned_field("index").unwrap_or_default() as usize })),
            "updateStep" => {
                let step_json = string_field("stepJson").or_else(|| string_field("step_json")).or_else(|| field("step").map(semio_framework_os_kernel::json::to_json_string)).unwrap_or_default();
                Ok(Process3dCommand::UpdateStep(update_step::UpdateStep { step_json }))
            }
            "setStepEnabled" => Ok(Process3dCommand::SetStepEnabled(set_step_enabled::SetStepEnabled { id: string_field("id").unwrap_or_default(), enabled: field("enabled").and_then(DslValue::as_bool).unwrap_or(true) })),
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
            "setActiveUtility" => Ok(Process3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility {
                utility_id: string_field("utilityId").or_else(|| string_field("utility_id")).unwrap_or_else(|| crate::editor::process3d::config::PROCESS3D_DEFAULT_UTILITY.into()),
            })),
            "engagementInput" => Ok(Process3dCommand::EngagementInput(engagement_input::EngagementInput { value: string_field("value").unwrap_or_default() })),
            "engagementAbort" => Ok(Process3dCommand::EngagementAbort(engagement_abort::EngagementAbort {})),
            "setCamera" => Ok(Process3dCommand::SetCamera(set_camera::SetCamera { position: vec3_field("position").unwrap_or([3.0, -3.0, 2.0]), target: vec3_field("target").unwrap_or_default(), fov: number_field("fov").unwrap_or(45.0) })),
            "toggleSun" => Ok(Process3dCommand::ToggleSun(toggle_sun::ToggleSun {})),
            "setSunAzimuth" => Ok(Process3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: number_field("value").unwrap_or_default() })),
            "setSunElevation" => Ok(Process3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: number_field("value").unwrap_or_default() })),
            "setSunIntensity" => Ok(Process3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: number_field("value").unwrap_or_default() })),
            "setLocale" => Ok(Process3dCommand::SetLocale(set_locale::SetLocale { value: string_field("value").unwrap_or_else(|| "en-US".into()) })),
            "setContributions" => Ok(Process3dCommand::SetContributions(set_contributions::SetContributions { json: string_field("json").unwrap_or_else(|| "[]".into()) })),
            "exportModel" => Ok(Process3dCommand::ExportModel(export_model::ExportModel { format: string_field("format").unwrap_or_else(|| "step".into()) })),
            "loadModelRequest" => Ok(Process3dCommand::LoadModelRequest(load_model_request::LoadModelRequest {})),
            other => Err(process3d_action_fault(other, "not declared by the process app's typed command vocabulary")),
        }
    }

    fn host_configuration_mutation(action: &str, args: Option<&DslValue>) -> Result<Option<Self::ConfigMutation>, Fault> {
        Ok((action == "setContributions").then(|| Process3dConfigMutation::SetContributions { json: args.and_then(|value| value.get("json")).and_then(DslValue::as_str).unwrap_or("[]").to_string() }))
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
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation, Self::DraftMutation>, Fault> {
        let selection = interaction.selection(PROCESS3D_INTERACTION_DOMAIN);
        let mut ctx = Process3dDispatchCtx { interaction: Process3dInteractionSnapshot { ids: selection.ids.clone() } };
        command.dispatch(doc, cfg, &mut ctx)
    }

    /// 🧮️ process3d exposes no genuinely settings-like sticky defaults — every `Process3dConfig` field
    /// is session-only view state, so this stays at the trait default.
    fn config_spec() -> semio_framework_plugin::ConfigSpec {
        semio_framework_plugin::ConfigSpec::default()
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        process3d_render_body(body_key, doc.snapshot, cfg.snapshot, &[])
    }

    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): resolves the live `"geometry"`
    /// selection once per render and threads it into `process3d_render_body`, so the inspection
    /// panel finally sees a real selection instead of always rendering empty.
    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        cfg: &ConfigView<'_, Process3dConfig>,
        _transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let selected_ids = interaction.selection(PROCESS3D_INTERACTION_DOMAIN).ids.clone();
        process3d_render_body(body_key, doc.snapshot, cfg.snapshot, &selected_ids)
    }

    fn window_engagements(doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>) -> HashMap<String, semio_framework_plugin::WindowEngagement> {
        HashMap::from([(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into(), workpiece::engagement(doc.snapshot, cfg.snapshot, process3d_labels(cfg.snapshot)))])
    }

    fn window_measures(_doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::from([(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into(), workpiece::window_measures(cfg.snapshot))])
    }

    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `context_menu` is no longer
    /// selection-gated — `ArtifactEditor::context_menu` has no `InteractionView` parameter, so it can no
    /// longer tell whether anything is selected (mirrors `📐️cad`'s own precedent) — always shows
    /// `removeSelectedStep`; it is itself a no-op via `remove_selected_step::handle` when nothing in
    /// the `"geometry"` domain is selected.
    fn context_menu(_request: &ContextMenuRequest, _doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        Menu::of(registry).action("addStep").destructive("removeSelectedStep").separator().action("undo").action("redo").build()
    }
}
//#endregion 🔖️Process3dPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline. `WindowKindDefinition.options.measures` stays empty: measures are config-derived per
/// frame by `ArtifactEditor::window_measures`, never frozen into the manifest.
pub fn create_process3d_app() -> AppDefinition {
    Editor::builder(crate::PROCESS3D_DIALECT)
            .command({
                let mut definition = CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))]) };
                definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
                definition
            })
            .document(["semio", "process", "3d"])
            .artifact_kind(ArtifactKindSpec {
                id: "3d.process".into(),
                name: "3D Process".into(),
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
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .mutation("removeSelectedStep", LocalizedLabel::native("Remove Selected Step", "Ausgewählten Schritt entfernen"))
            // 🐚️ Palette-visible host round-trips.
            .shell_action("exportModel", LocalizedLabel::native("Export Model", "Modell exportieren"))
            .shell_action("loadModelRequest", LocalizedLabel::native("Load Model…", "Modell laden…"))
            // 🔧️ Internal document mutations dispatched by panel/viewport wiring (not palette-worthy).
            .action_with(internal_action("setSnapshot", LocalizedLabel::native("Set Document", "Dokument festlegen"), ActionKind::Mutation))
            .action_with(internal_action("addWorkshopMachine", LocalizedLabel::native("Add Machine", "Maschine hinzufügen"), ActionKind::Mutation))
            .action_with(internal_action("removeWorkshopMachine", LocalizedLabel::native("Remove Machine", "Maschine entfernen"), ActionKind::Mutation))
            .action_with(internal_action("updateWorkshopMachine", LocalizedLabel::native("Update Machine", "Maschine aktualisieren"), ActionKind::Mutation))
            .action_with(internal_action("importModelFile", LocalizedLabel::native("Import Model File", "Modelldatei importieren"), ActionKind::Mutation))
            .action_with(internal_action("removeStep", LocalizedLabel::native("Remove Step", "Schritt entfernen"), ActionKind::Mutation))
            .action_with(internal_action("moveStep", LocalizedLabel::native("Move Step", "Schritt verschieben"), ActionKind::Mutation))
            .action_with(internal_action("updateStep", LocalizedLabel::native("Update Step", "Schritt aktualisieren"), ActionKind::Mutation))
            .action_with(internal_action("setStepEnabled", LocalizedLabel::native("Set Step Enabled", "Schrittaktivierung festlegen"), ActionKind::Mutation))
            .action_with(internal_action("patchInspector", LocalizedLabel::native("Patch Inspector", "Inspektor aktualisieren"), ActionKind::Mutation))
            .action_with(internal_action("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"), ActionKind::Mutation))
            .action_with(internal_action("worldFaceDragEnd", LocalizedLabel::native("World Face Drag End", "Welt-Flächenzug beendet"), ActionKind::Mutation))
            // ⏱️ Document-cursor navigation operations (NOT framework History — they move the replay cursor).
            .action_with(internal_action("setCursor", LocalizedLabel::native("Set Cursor", "Cursor festlegen"), ActionKind::Mutation))
            .action_with(internal_action("stepCursor", LocalizedLabel::native("Step Cursor", "Cursor schrittweise bewegen"), ActionKind::Mutation))
            .action_with(internal_action("stepCursorBack", LocalizedLabel::native("Step Cursor Back", "Cursor zurück"), ActionKind::Mutation))
            .action_with(internal_action("stepCursorForward", LocalizedLabel::native("Step Cursor Forward", "Cursor vorwärts"), ActionKind::Mutation))
            // 🎛️ Engagement session command line (a separate system from utility selection).
            .action_with(internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Mutation))
            .action_with(internal_action("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View))
            .action_with(internal_action("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View))
            // 👁️ Ephemeral view state — camera, sun. Selection/hover are the framework-owned
            // "geometry" interaction domain now (declared below via `.interaction`); the six
            // framework verbs (interactionSelect/interactionHover/clearSelection/selectAll/
            // setSelectionMode/setInteractionGranularity) auto-inject — never declared here.
            .action_with(internal_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View))
            .action_with(internal_action("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), ActionKind::View))
            .action_with(internal_action("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), ActionKind::View))
            .action_with(internal_action("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), ActionKind::View))
            .action_with(internal_action("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), ActionKind::View))
            .action_with(internal_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"), ActionKind::View))
            // 📝️ Staged argument forms for the palette-visible create/export actions.
            .action_args("addStep", vec![
                ActionArgDef::select("measure", LocalizedLabel::native("Measure", "Maßnahme"), vec![
                    ActionArgOption::new("cut", LocalizedLabel::native("Cut", "Schnitt")),
                    ActionArgOption::new("drill", LocalizedLabel::native("Drill", "Bohrung")),
                    ActionArgOption::new("attach", LocalizedLabel::native("Attach", "Anbau")),
                ]).default_value(&"cut"),
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
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors `crate::
/// artifact_kind()`'s literal for `"3d.process"` (schema/media type/export+import formats/presentation
/// fields copied verbatim), plus the two workflow ports: `geometry:in` (Many, unrequired — accepts
/// upstream geometry producers, e.g. cad/lowpoly) and `brep:out` (Many, unrequired, `kind_id:
/// "3d.process"` — reusing the artifact kind already declared, never a second `.artifact_kind(...)` call).
pub fn process3d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: crate::PROCESS_3D_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
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

//#region 🔧️Behavior
/// 🧭️ Axis-angle rotation that maps world-up `[0,0,1]` onto an arbitrary unit `normal`, so a box
/// primitive's local Z axis (its `height` dimension) ends up flush with a picked face's normal. Pure
/// math with no snapshot/io coupling — only `🎮️commands/🌍️world`'s face-drag placement calls it, so
/// it lives here rather than `🧬️schema/💡️inferences`.
pub fn axis_angle_from_up_to(normal: [f64; 3]) -> ([f64; 3], f64) {
    const UP: [f64; 3] = [0.0, 0.0, 1.0];
    let dot = (UP[0] * normal[0] + UP[1] * normal[1] + UP[2] * normal[2]).clamp(-1.0, 1.0);
    if dot > 1.0 - 1e-9 {
        return ([0.0, 0.0, 1.0], 0.0);
    }
    if dot < -1.0 + 1e-9 {
        return ([1.0, 0.0, 0.0], std::f64::consts::PI);
    }
    let cross = [UP[1] * normal[2] - UP[2] * normal[1], UP[2] * normal[0] - UP[0] * normal[2], UP[0] * normal[1] - UP[1] * normal[0]];
    let len = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
    let axis = if len > 1e-9 { [cross[0] / len, cross[1] / len, cross[2] / len] } else { [0.0, 0.0, 1.0] };
    (axis, dot.acos())
}

/// 🧩️ One hot-installed machine catalog deserialized from the `"process.machines"` topic contribution.
/// 🔓️ `pub` (not private): a variant payload of `crate::MachineCatalogs`
/// (closed in the trait's own module, not here — see that enum's doc comment for why), which is
/// itself `pub` — a variant field can never be less visible than the enum wrapping it.
#[derive(Clone)]
pub struct ContributedMachineCatalog {
    catalog_id: String,
    label: String,
    icon_id: String,
    machines: Vec<WorkshopMachine>,
}

impl MachineCatalog for ContributedMachineCatalog {
    fn catalog_id(&self) -> &str {
        &self.catalog_id
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn icon_id(&self) -> &str {
        &self.icon_id
    }

    fn machines(&self) -> Vec<WorkshopMachine> {
        self.machines.clone()
    }
}

/// 🔌️ Refreshes contributed `process.machines` catalogs when the host pushes a new catalogue.
//#region 🔖️ProcessMachinesTopicPayload
/// 🗂️ `topic_contribution.payload` shape for the `"process.machines"` topic — the sole shape
/// `contributed_machine_catalogs` decodes. See `TopicContribution` in
/// `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`.
#[derive(semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase")]
struct ProcessMachinesTopicPayload {
    app_id: String,
    module_id: String,
    label: String,
    icon_id: String,
    machines_json: String,
}
//#endregion 🔖️ProcessMachinesTopicPayload

const PROCESS_CONTRIBUTION_MAX_BYTES: usize = 256 * 1024;
const PROCESS_CONTRIBUTION_MAX_DEPTH: usize = 32;
const PROCESS_CONTRIBUTION_MAX_ITEMS: usize = 4 * 1024;
const PROCESS_CONTRIBUTION_MAX_STRING_BYTES: usize = 4 * 1024;

fn process_json_envelope_is_bounded(input: &str) -> bool {
    if input.len() > PROCESS_CONTRIBUTION_MAX_BYTES {
        return false;
    }
    let mut depth = 0usize;
    let mut items = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut string_bytes = 0usize;
    let mut in_scalar = false;
    for byte in input.bytes() {
        if in_string {
            if escaped {
                string_bytes = string_bytes.saturating_add(1);
                escaped = false;
            } else if byte == b'\\' {
                string_bytes = string_bytes.saturating_add(1);
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            } else {
                string_bytes = string_bytes.saturating_add(1);
            }
            if string_bytes > PROCESS_CONTRIBUTION_MAX_STRING_BYTES {
                return false;
            }
            continue;
        }
        if in_scalar {
            if byte.is_ascii_whitespace() || matches!(byte, b',' | b']' | b'}') {
                in_scalar = false;
            } else {
                continue;
            }
        }
        match byte {
            b'"' => {
                items = items.saturating_add(1);
                in_string = true;
                string_bytes = 0;
            }
            b'{' | b'[' => {
                items = items.saturating_add(1);
                depth = depth.saturating_add(1);
                if depth > PROCESS_CONTRIBUTION_MAX_DEPTH {
                    return false;
                }
            }
            b'}' | b']' => {
                let Some(next) = depth.checked_sub(1) else {
                    return false;
                };
                depth = next;
            }
            b':' | b',' => {}
            byte if byte.is_ascii_whitespace() => {}
            _ => {
                items = items.saturating_add(1);
                in_scalar = true;
            }
        }
        if items > PROCESS_CONTRIBUTION_MAX_ITEMS {
            return false;
        }
    }
    !in_string && !escaped && depth == 0
}

// 🚫️async: E1 pure — bounded JSON decode into operation-owned values, zero suspension points.
// `TopicContribution::decode` itself is still `fn` in
// `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (out of this packet's path_scope); bridged via
// `semio_framework::io::resolve_ready` (see `imperative_extension_sdk`'s identical bridge and this
// packet's lease-request for the SDK owner to revert `decode` to sync directly).
fn contributed_machine_catalogs(contributions_json: &str) -> Vec<ContributedMachineCatalog> {
    if !process_json_envelope_is_bounded(contributions_json) {
        return Vec::new();
    }
    let mut catalogs = Vec::new();
    for entry in semio_framework::parse_contributions(contributions_json) {
        let Some(payload) = entry.topic_contribution.as_ref().filter(|topic| topic.topic == "process.machines").and_then(|topic| topic.decode::<ProcessMachinesTopicPayload>().ok()) else {
            continue;
        };
        let (app_id, module_id, label, icon_id, machines_json) = (payload.app_id, payload.module_id, payload.label, payload.icon_id, payload.machines_json);
        if app_id != PROCESS_3D_PLAY_APP_ID {
            continue;
        }
        if !process_json_envelope_is_bounded(&machines_json) {
            continue;
        }
        let Ok(machines) = semio_framework_os_kernel::json::from_json_str::<Vec<WorkshopMachine>>(&machines_json) else {
            continue;
        };
        if machines.len() > PROCESS_CONTRIBUTION_MAX_ITEMS {
            continue;
        }
        catalogs.push(ContributedMachineCatalog { catalog_id: module_id, label, icon_id, machines });
    }
    catalogs
}

fn builtin_installed_catalogs() -> Vec<MachineCatalogs> {
    vec![
        crate::schema::GenericCatalog.into(),
        crate::schema::wood_catalog().into(),
        crate::schema::concrete_catalog().into(),
        crate::schema::metal_catalog().into(),
        crate::schema::robotic_catalog().into(),
    ]
}

/// 🧩️ Every machine catalog installed in this build, in stable display order — the built-in generic
/// catalog first (so it renders as the default-open section), then every `process.machines` contribution
/// parsed from the invoking configuration's runtime-installable extensions under
/// `🏭️process/🧩️extensions/`.
pub fn installed_catalogs(contributions_json: &str) -> Vec<MachineCatalogs> {
    let mut catalogs = builtin_installed_catalogs();
    catalogs.extend(contributed_machine_catalogs(contributions_json).into_iter().map(MachineCatalogs::from));
    catalogs
}

/// 🔎️ One machine, by catalog + machine id, with `catalog_id` stamped onto the snapshot — the
/// "install into workshop" lookup for the workshop configurator's add-machine action.
pub fn catalog_machine(contributions_json: &str, catalog_id: &str, machine_id: &str) -> Option<WorkshopMachine> {
    let catalog = installed_catalogs(contributions_json).into_iter().find(|catalog| catalog.catalog_id() == catalog_id)?;
    let mut machine = catalog.machines().into_iter().find(|machine| machine.id == machine_id)?;
    machine.catalog_id = Some(catalog_id.to_string());
    Some(machine)
}
//#endregion 🔧️Behavior

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
