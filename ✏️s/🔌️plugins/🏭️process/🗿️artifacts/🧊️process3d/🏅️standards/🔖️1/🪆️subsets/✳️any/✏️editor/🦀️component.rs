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

use crate::artifacts::process3d::op::Process3dMutation;
use crate::artifacts::process3d::{MachineCatalog, MachineCatalogs, Process3dSnapshot};
use crate::editor::process3d::commands::{camera, contribution, cursor, document, engagement, inspector, locale, media, step, stock, sun, utility, workshop, world};
use crate::editor::process3d::config::{Process3dConfig, Process3dConfigMutation};
use crate::editor::process3d::modes::edit;
use crate::editor::process3d::modes::edit::windows::workpiece;
use crate::editor::process3d::panels::{catalogue, document as document_panel, inspection, workshop as workshop_panel};
use crate::editor::process3d::presence::{Process3dPresence, Process3dPresenceMutation};
use crate::editor::process3d::terminology::process3d_labels;
use semio_framework::kernel::Effect;
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppActionRegistry, AppDefinition, ArtifactEditor, ArtifactKindSpec, ArtifactView, CommandDefinition, ConfigView, ContextMenuItemSpec, ContextMenuRequest, Dialect,
    DraftView, Editor, Emit, Fault, FaultCode, FaultOrigin, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, Label, LocalizedLabel, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, Menu,
    MergeMode, NoDraft, NoDraftMutation, OsMediaCapability, SelectionMethod, SelectionMode, SelectionSpec, UiNode, UiTreeItemNode, UtilityCategory, UtilityDefinition, WindowMeasure,
};
use serde_json::Value;
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
/// (`🎚️options/*`, `📌️panels/*`, `🎮️commands/*`) builds its `on_change`/item actions with.
pub fn process3d_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(PROCESS_3D_PLAY_CONTROLLER_ID).action(action, args)
}


/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref())
        .map(semio_framework_plugin::UiValue::Text)
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
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
    let mut builder = semio_framework_plugin::UiListBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder
            .push(value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new()
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder
            .push(key.to_owned(), value)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes
            .try_push(node)
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
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
        props.icon = Some(
            semio_framework_plugin::UiText::try_from_str(icon_id)
                .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.tree-item.icon", "fixed tree-item icon admission failed"))?,
        );
    }
    Ok(node)
}

/// 🔁️ Builds a `Effect::LoadDocument` for `document` — the sanctioned non-history "replace the
/// whole document" gesture (`ArtifactStore::reset`, applied host-side) every wholesale document-swap
/// command (`🎮️commands/📄️artifact`, `🎮️commands/🪵️stock`, `🎮️commands/📤️media`, `import_media`'s
/// `geometry:in`) uses instead of the banned whole-snapshot mutation. The spr is a fresh, edit-free
/// op-log — a genesis envelope with no history to encode.
pub fn reset_process3d_document_effect(document: &Process3dSnapshot) -> Effect {
    let pack = <Process3dSnapshot as ArtifactPack>::encode_pack(document);
    let envelope = store::create_document_envelope::<Process3dSnapshot, Process3dMutation>(crate::artifacts::process3d::PROCESS_3D_SCHEMA, "process3d", document.clone(), None);
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

    const DIALECT: Dialect = crate::artifacts::process3d::PROCESS3D_DIALECT;

    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::process3d::PROCESS_3D_SCHEMA;

    async fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::process3d::config::schema::app_schema_descriptor())
    }

    async fn initial_snapshot() -> Process3dSnapshot {
        crate::artifacts::process3d::schema::default_document()
    }

    async fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(process3d_io())
    }

    //#region 🔖️Media
    /// 🎞️ `brep:out` (see the artifact engine's `export_process3d_model`, STEP text) plus the inherited
    /// `document:out` default (the pack of `doc.snapshot`, replicated inline — overriding `export_media`
    /// shadows the trait's provided body for every port on this app, not just the new one).
    async fn export_media(port: &str, doc: &ArtifactView<'_, Process3dSnapshot>) -> Result<semio_framework_plugin::Media, MediaError> {
        match port {
            "brep:out" => match crate::artifacts::process3d::io::export_process3d_model(&crate::artifacts::process3d::process_working_scene_from_snapshot(doc.snapshot), doc.snapshot.resolved_up_to, "step")
                .map_err(|error| MediaError::Payload("brep:out".into(), error))?
            {
                Some(export) => {
                    let text = match export.data {
                        Value::String(text) => text,
                        other => serde_json::to_string(&other).unwrap_or_default(),
                    };
                    Ok(semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep }, payload: MediaPayload::Structured { schema: "3d.process".into(), json: text } })
                }
                None => Err(MediaError::Payload("brep:out".into(), "kernel replay failed".into())),
            },
            "document:out" => {
                let media_type = Self::io().await.map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
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

    /// 📥️ `geometry:in` (best-effort STEP-text import) replaces the whole document via a
    /// `Effect::LoadDocument` (whole-document replace has no in-history mutation); the inherited
    /// `document:in` default (which would decode a base64 pack via `whole_document_operation`) is
    /// unreachable now that `whole_document_operation` is `None`, so `document:in` is simply
    /// unimplemented here — overriding `import_media` shadows the trait's provided body for every port.
    async fn import_media(port: &str, media: &semio_framework_plugin::Media, _doc: &ArtifactView<'_, Process3dSnapshot>) -> Result<Emit<Process3dMutation, Process3dConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "geometry:in" => {
                let MediaPayload::Structured { schema, json } = &media.payload else {
                    return Err(MediaError::Payload("geometry:in".into(), "expected a structured payload".into()));
                };
                if schema != crate::artifacts::process3d::PROCESS_3D_SCHEMA && schema != "3d.process" {
                    return Err(MediaError::Payload("geometry:in".into(), format!("unrecognized schema: {schema}")));
                }
                // 📦️ `export_process3d_model("step")` hands back raw (non-base64) STEP text — the
                // "stdio.step" format is not binary — so this re-encodes it as base64 to satisfy
                // `import_process3d_model`'s `data:...,<base64>` expectation.
                use base64::Engine;
                let data_url = format!("data:application/octet-stream;base64,{}", base64::engine::general_purpose::STANDARD.encode(json.as_bytes()));
                match crate::artifacts::process3d::io::import_process3d_model("geometry-in.step", &data_url) {
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
    async fn command_id(command: &Process3dCommand) -> &'static str {
        command.command_id()
    }

    /// 🎯️ Exhaustive host-action bridge into the closed `Process3dCommand` enum. React and wgpu still
    /// emit manifest action ids plus JSON arguments; only this boundary interprets that transport shape.
    async fn command_from_action(action: &str, args: Option<&Value>) -> Result<Self::Command, Fault> {
        let field = |key: &str| args.and_then(|value| value.get(key));
        let string_field = |key: &str| field(key).and_then(Value::as_str).map(str::to_string);
        let number_field = |key: &str| field(key).and_then(Value::as_f64);
        let unsigned_field = |key: &str| field(key).and_then(|value| value.as_u64().or_else(|| value.as_f64().filter(|number| number.is_finite() && *number >= 0.0).map(|number| number as u64)));
        let signed_field = |key: &str| field(key).and_then(|value| value.as_i64().or_else(|| value.as_f64().filter(|number| number.is_finite()).map(|number| number as i64)));
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
                let json = string_field("json").or_else(|| field("document").and_then(|value| serde_json::to_string(value).ok())).unwrap_or_default();
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
                    .map(serde_json::from_value)
                    .transpose()
                    .map_err(|error| process3d_action_fault(action, format!("invalid 'machine': {error}")))?
                    .unwrap_or(crate::artifacts::process3d::WorkshopMachine { id: String::new(), label: String::new(), icon_id: String::new(), catalog_id: None, capabilities: Vec::new() }),
            })),
            "removeStep" => Ok(Process3dCommand::RemoveStep(remove_step::RemoveStep { id: string_field("id").unwrap_or_default() })),
            "removeSelectedStep" => Ok(Process3dCommand::RemoveSelectedStep(remove_selected_step::RemoveSelectedStep {})),
            "moveStep" => Ok(Process3dCommand::MoveStep(move_step::MoveStep { id: string_field("id").unwrap_or_default(), index: unsigned_field("index").unwrap_or_default() as usize })),
            "updateStep" => {
                let step_json = string_field("stepJson").or_else(|| string_field("step_json")).or_else(|| field("step").and_then(|value| serde_json::to_string(value).ok())).unwrap_or_default();
                Ok(Process3dCommand::UpdateStep(update_step::UpdateStep { step_json }))
            }
            "setStepEnabled" => Ok(Process3dCommand::SetStepEnabled(set_step_enabled::SetStepEnabled { id: string_field("id").unwrap_or_default(), enabled: field("enabled").and_then(Value::as_bool).unwrap_or(true) })),
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

    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): reads the framework-owned
    /// `"geometry"` domain selection once per dispatch and threads it through `Process3dDispatchCtx`
    /// — the one retained verb that operates ON the selection (`remove_selected_step`) reads it from
    /// there; every other command ignores it (mirrors `📐️cad`'s own `handle`).
    async fn handle(
        command: &Process3dCommand,
        doc: &ArtifactView<'_, Process3dSnapshot>,
        cfg: &ConfigView<'_, Process3dConfig>,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<Process3dMutation, Process3dConfigMutation, Self::DraftMutation>, Fault> {
        let selection = interaction.selection(PROCESS3D_INTERACTION_DOMAIN).await;
        let mut ctx = Process3dDispatchCtx { interaction: Process3dInteractionSnapshot { ids: selection.ids.clone() } };
        command.dispatch(doc, cfg, &mut ctx)
    }

    /// 🧮️ process3d exposes no genuinely settings-like sticky defaults — every `Process3dConfig` field
    /// is session-only view state, so this stays at the trait default.
    async fn config_spec() -> semio_framework_plugin::ConfigSpec {
        semio_framework_plugin::ConfigSpec::empty().await
    }

    async fn render(body_key: &str, doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let config = cfg.snapshot;
        let labels = process3d_labels(config);
        let base_body_key = body_key.split_once(':').map_or(body_key, |(base, _)| base);
        match base_body_key {
            PROCESS_3D_PLAY_BODY_MAIN => workpiece::render(doc.snapshot, config).map(semio_framework_plugin::built_to_component_tree),
            PROCESS_3D_PLAY_BODY_DOCUMENT => document_panel::render(doc.snapshot, labels).map(semio_framework_plugin::built_to_component_tree),
            PROCESS_3D_PLAY_BODY_CATALOGUE => catalogue::render(doc.snapshot, &config.contributions_json, labels).map(semio_framework_plugin::built_to_component_tree),
            PROCESS_3D_PLAY_BODY_WORKSHOP => workshop_panel::render(doc.snapshot, &config.contributions_json, labels).map(semio_framework_plugin::built_to_component_tree),
            PROCESS_3D_PLAY_BODY_INSPECTION => inspection::render(doc.snapshot, config, labels).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    async fn window_engagements(doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>) -> HashMap<String, semio_framework_plugin::WindowEngagement> {
        HashMap::from([(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into(), workpiece::engagement(doc.snapshot, cfg.snapshot, process3d_labels(cfg.snapshot)))])
    }

    async fn window_measures(_doc: &ArtifactView<'_, Process3dSnapshot>, cfg: &ConfigView<'_, Process3dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::from([(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into(), workpiece::window_measures(cfg.snapshot))])
    }

    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `context_menu` is no longer
    /// selection-gated — `ArtifactEditor::context_menu` has no `InteractionView` parameter, so it can no
    /// longer tell whether anything is selected (mirrors `📐️cad`'s own precedent) — always shows
    /// `removeSelectedStep`; it is itself a no-op via `remove_selected_step::handle` when nothing in
    /// the `"geometry"` domain is selected.
    async fn context_menu(_request: &ContextMenuRequest, _doc: &ArtifactView<'_, Process3dSnapshot>, _cfg: &ConfigView<'_, Process3dConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        Menu::of(registry).await.action("addStep").await.destructive("removeSelectedStep").await.separator().await.action("undo").await.action("redo").await.build().await
    }
}
//#endregion 🔖️Process3dPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline. `WindowKindDefinition.options.measures` stays empty: measures are config-derived per
/// frame by `ArtifactEditor::window_measures`, never frozen into the manifest.
pub fn create_process3d_app() -> AppDefinition {
    Editor::builder(crate::artifacts::process3d::PROCESS3D_DIALECT)
            .command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))]) })
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
                export_stdio_kinds: vec!["stdio.step", "stdio.obj", "stdio.stl", "stdio.gltf"],
                import_stdio_kinds: vec!["stdio.step", "stdio.obj", "stdio.stl"],
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
                ]).default_value("cut"),
            ])
            .action_args("setStock", vec![
                ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                    ActionArgOption::new("box", LocalizedLabel::native("Box", "Quader")),
                    ActionArgOption::new("cylinder", LocalizedLabel::native("Cylinder", "Zylinder")),
                    ActionArgOption::new("sphere", LocalizedLabel::native("Sphere", "Kugel")),
                ]).default_value("box"),
            ])
            .action_args("setActiveExample", vec![
                ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                    ActionArgOption::new(PROCESS3D_EXAMPLE_TIMBER, LocalizedLabel::native("Timber Beam Joinery", "Holzbalkenverbindung")),
                    ActionArgOption::new(PROCESS3D_EXAMPLE_PLATE, LocalizedLabel::native("Drilled Plate", "Gebohrte Platte")),
                ]).required().default_value(PROCESS3D_EXAMPLE_TIMBER),
            ])
            .action_args("exportModel", vec![
                ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), vec![
                    ActionArgOption::new("step", LocalizedLabel::native("STEP", "STEP")),
                    ActionArgOption::new("obj", LocalizedLabel::native("OBJ", "OBJ")),
                    ActionArgOption::new("stl", LocalizedLabel::native("STL", "STL")),
                    ActionArgOption::new("glb", LocalizedLabel::native("GLB", "GLB")),
                ]).required().default_value("step"),
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
            .config(semio_framework_plugin::resolve_ready(Process3dPlayApp::config_spec()))
            .io(process3d_io())
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a
            // bare `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so the old
            // `PROCESS3D_EXAMPLE_TIMBER`/`PROCESS3D_EXAMPLE_PLATE` app-level example registrations and
            // the no-op `.workflow("process3d", …)` call are dropped here, not silently: reported in
            // this packet's migration notes. The subset's own `📚️examples/🎬️demo` facet
            // (`crate::artifacts::process3d::examples::...`, real content, pre-existing) is the
            // modern, role-agnostic replacement surface for this.
            .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated)
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors `crate::artifacts::process3d::
/// artifact_kind()`'s literal for `"3d.process"` (schema/media type/export+import formats/presentation
/// fields copied verbatim), plus the two workflow ports: `geometry:in` (Many, unrequired — accepts
/// upstream geometry producers, e.g. cad/lowpoly) and `brep:out` (Many, unrequired, `kind_id:
/// "3d.process"` — reusing the artifact kind already declared, never a second `.artifact_kind(...)` call).
pub fn process3d_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: crate::artifacts::process3d::PROCESS_3D_SCHEMA.into(),
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
/// 🔓️ `pub` (not private): a variant payload of `crate::artifacts::process3d::MachineCatalogs`
/// (closed in the trait's own module, not here — see that enum's doc comment for why), which is
/// itself `pub` — a variant field can never be less visible than the enum wrapping it.
#[derive(Clone)]
pub struct ContributedMachineCatalog {
    catalog_id: String,
    label: String,
    icon_id: String,
    machines: Vec<crate::artifacts::process3d::WorkshopMachine>,
}

impl crate::artifacts::process3d::MachineCatalog for ContributedMachineCatalog {
    fn catalog_id(&self) -> &str {
        &self.catalog_id
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn icon_id(&self) -> &str {
        &self.icon_id
    }

    fn machines(&self) -> Vec<crate::artifacts::process3d::WorkshopMachine> {
        self.machines.clone()
    }
}

/// 🔌️ Refreshes contributed `process.machines` catalogs when the host pushes a new catalogue.
//#region 🔖️ProcessMachinesTopicPayload
/// 🗂️ `topic_contribution.payload` shape for the `"process.machines"` topic — the sole shape
/// `contributed_machine_catalogs` decodes. See `TopicContribution` in
/// `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProcessMachinesTopicPayload {
    app_id: String,
    module_id: String,
    label: String,
    icon_id: semio_framework::IconName,
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
// `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` (out of this packet's path_scope); bridged via
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
        let Ok(machines) = serde_json::from_str::<Vec<crate::artifacts::process3d::WorkshopMachine>>(&machines_json) else {
            continue;
        };
        if machines.len() > PROCESS_CONTRIBUTION_MAX_ITEMS {
            continue;
        }
        catalogs.push(ContributedMachineCatalog { catalog_id: module_id, label, icon_id: icon_id.to_string(), machines });
    }
    catalogs
}

fn builtin_installed_catalogs() -> Vec<MachineCatalogs> {
    vec![
        crate::artifacts::process3d::schema::GenericCatalog.into(),
        crate::artifacts::process3d::schema::wood_catalog().into(),
        crate::artifacts::process3d::schema::concrete_catalog().into(),
        crate::artifacts::process3d::schema::metal_catalog().into(),
        crate::artifacts::process3d::schema::robotic_catalog().into(),
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
pub fn catalog_machine(contributions_json: &str, catalog_id: &str, machine_id: &str) -> Option<crate::artifacts::process3d::WorkshopMachine> {
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
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    /// ✏️ `Process3dPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime
    /// `ArtifactApp` — `EditorApp<Process3dPlayApp>` (SDK adapter, contract §2.1) is the real
    /// `ArtifactApp` implementor `VcsArtifactApp` wraps, exactly the way
    /// `PluginBuilder::editor::<Process3dPlayApp>` builds it.
    pub type Process3dApp = VcsArtifactApp<EditorApp<Process3dPlayApp>>;

    /// ✏️ Adapts `create_process3d_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `testkit::assert_declared_actions_bridge_to_commands`/`new_app_with_registry`
    /// still expect — framework testkit gap, not modifiable here (`🧰️framework/**` is outside this
    /// packet's lease).
    pub fn process3d_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_process3d_app(), examples: Vec::new() }
    }

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.

    /// 🧪 Seeds wood/metal contribution catalogs so panel tests can install machines without the host.
    fn seed_domain_catalog_contributions(app: &mut Process3dApp) {
        use crate::artifacts::process3d::{Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, StockQuantity, WorkshopMachine};
        use semio_framework::{ProgramContributionEntry, TopicContribution};
        fn param(id: &str, label: &str, value: f64) -> CapabilityParameter {
            CapabilityParameter { id: id.into(), label: label.into(), value }
        }
        let wood_machines = vec![
            WorkshopMachine {
                id: "circularSaw".into(),
                label: "Circular Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "crosscut".into(),
                    label: "Crosscut".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                    parameters: vec![param("bladeDiameter", "Blade Diameter", 0.184), param("kerf", "Kerf", 0.002), param("maxCutDepth", "Max Cut Depth", 0.065)],
                    rules: vec![CapabilityRule::Max { quantity: StockQuantity::Height, parameter: "maxCutDepth".into(), margin: 0.0 }],
                }],
            },
            WorkshopMachine {
                id: "tableSaw".into(),
                label: "Table Saw".into(),
                icon_id: "scissors".into(),
                catalog_id: None,
                capabilities: vec![Capability {
                    id: "rip".into(),
                    label: "Rip".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                    parameters: vec![param("bladeDiameter", "Blade Diameter", 0.315), param("kerf", "Kerf", 0.0032), param("maxCutDepth", "Max Cut Depth", 0.102), param("fenceWidth", "Fence Width", 0.8)],
                    rules: vec![CapabilityRule::Max { quantity: StockQuantity::Height, parameter: "maxCutDepth".into(), margin: 0.0 }, CapabilityRule::Max { quantity: StockQuantity::Width, parameter: "fenceWidth".into(), margin: 0.0 }],
                }],
            },
        ];
        let metal_machines = vec![WorkshopMachine {
            id: "chopSaw".into(),
            label: "Chop Saw".into(),
            icon_id: "scissors".into(),
            catalog_id: None,
            capabilities: vec![Capability {
                id: "crosscut".into(),
                label: "Crosscut".into(),
                icon_id: "scissors".into(),
                recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                parameters: vec![param("bladeDiameter", "Blade Diameter", 0.35), param("kerf", "Kerf", 0.002), param("maxCutDepth", "Max Cut Depth", 0.12)],
                rules: vec![],
            }],
        }];
        let entries = vec![
            ProgramContributionEntry {
                plugin_id: "process-wood".into(),
                topic_contribution: Some(TopicContribution::new(
                    "process.machines",
                    serde_json::json!({
                        "appId": "process3d-play",
                        "moduleId": "wood",
                        "label": "Wood",
                        "iconId": "beam",
                        "machinesJson": serde_json::to_string(&wood_machines).unwrap(),
                    }),
                )),
            },
            ProgramContributionEntry {
                plugin_id: "process-metal".into(),
                topic_contribution: Some(TopicContribution::new(
                    "process.machines",
                    serde_json::json!({
                        "appId": "process3d-play",
                        "moduleId": "metal",
                        "label": "Metal",
                        "iconId": "wrench",
                        "machinesJson": serde_json::to_string(&metal_machines).unwrap(),
                    }),
                )),
            },
        ];
        let json = serde_json::to_string(&entries).unwrap();
        let _ = app;
    }

    pub fn app() -> Process3dApp {
        let mut app = new_app::<EditorApp<Process3dPlayApp>>();
        seed_domain_catalog_contributions(&mut app);
        app
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn app_with_registry() -> Process3dApp {
        let mut app = new_app_with_registry::<EditorApp<Process3dPlayApp>>(process3d_app_manifest_for_testkit);
        seed_domain_catalog_contributions(&mut app);
        app
    }

    pub fn dispatch(app: &mut Process3dApp, command: Process3dCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn action(app: &mut Process3dApp, action: &str, args: Option<&Value>) -> InvocationResult {
        app.handle_action(action, args, &meta("local")).expect("action dispatch")
    }

    pub fn render(app: &mut Process3dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    pub fn main_window_measures(app: &mut Process3dApp) -> Vec<WindowMeasure> {
        app.window_measures().get(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN).cloned().expect("main window measures")
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::process3d::testkit::{action, app, app_with_registry, dispatch, main_window_measures, process3d_app_manifest_for_testkit, render as render_body};
    use semio_framework_plugin::{testkit, ContextMenuRequest, ContextMenuSurfaceTarget, EditorApp, HistoryView, PluginApp, UiMenuRef, SET_ACTIVE_UTILITY_ACTION_ID};

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
    /// wire keyword must be distinct.
    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 33, "every Process3dCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — copied
    /// verbatim from the pre-migration `Process3dCommand`/`command_id()` match (the two vocabularies
    /// genuinely diverge for about a third of process3d's rows, unlike flow's single `setLocale`
    /// exception, so this pins the full table rather than deriving it from a kebab-case guess).
    #[semio_framework_async_macros::async_test]
    async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        let expected_wire_key = |id: &str| -> &'static str {
            match id {
                "setSnapshot" => "document",
                "setActiveExample" => "active-example",
                "addStep" => "add-step",
                "addWorkshopMachine" => "add-workshop-machine",
                "removeWorkshopMachine" => "remove-workshop-machine",
                "updateWorkshopMachine" => "update-workshop-machine",
                "removeStep" => "remove-step",
                "removeSelectedStep" => "remove-selected-step",
                "moveStep" => "move-step",
                "updateStep" => "update-step",
                "setStepEnabled" => "set-step-enabled",
                "setStock" => "stock",
                "patchInspector" => "patch-inspector",
                "setCursor" => "cursor",
                "stepCursor" => "step-cursor",
                "stepCursorBack" => "step-cursor-back",
                "stepCursorForward" => "step-cursor-forward",
                "engagementSubmit" => "engagement-submit",
                "worldPointerDown" => "world-pointer-down",
                "worldFaceDragEnd" => "world-face-drag-end",
                "importModelFile" => "import-model-file",
                "engagementInput" => "engagement-input",
                "engagementAbort" => "engagement-abort",
                "setCamera" => "camera",
                "toggleSun" => "toggle-sun",
                "setSunAzimuth" => "sun-azimuth",
                "setSunElevation" => "sun-elevation",
                "setSunIntensity" => "sun-intensity",
                "setLocale" => "locale",
                "setContributions" => "contributions",
                "exportModel" => "export-model",
                "loadModelRequest" => "load-model-request",
                other if other == SET_ACTIVE_UTILITY_ACTION_ID => "active-utility",
                other => panic!("no expected wire key recorded for command id {other} — add it to this table"),
            }
        };
        for command in every_command() {
            let id = command.command_id();
            let printed = protocol::OpText::print_op(&command);
            assert_eq!(printed.split(' ').next().unwrap_or_default(), expected_wire_key(id), "wire keyword drifted for command {id}: {printed:?}");
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) fn every_command() -> Vec<Process3dCommand> {
        vec![
            Process3dCommand::SetDocument(set_snapshot::SetDocument { json: serde_json::to_string(&crate::artifacts::process3d::empty_process3d_snapshot()).expect("json") }),
            Process3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: PROCESS3D_EXAMPLE_PLATE.into() }),
            Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: Some([1.0, 2.0, 3.0]) }),
            Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "wood".into(), machine_id: "circularSaw".into() }),
            Process3dCommand::RemoveWorkshopMachine(remove_workshop_machine::RemoveWorkshopMachine { id: "circularSaw".into() }),
            Process3dCommand::UpdateWorkshopMachine(update_workshop_machine::UpdateWorkshopMachine {
                machine: crate::artifacts::process3d::WorkshopMachine { id: "circularSaw".into(), label: "Circular Saw".into(), icon_id: "scissors".into(), catalog_id: Some("wood".into()), capabilities: vec![] },
            }),
            Process3dCommand::RemoveStep(remove_step::RemoveStep { id: "cut-1".into() }),
            Process3dCommand::RemoveSelectedStep(remove_selected_step::RemoveSelectedStep {}),
            Process3dCommand::MoveStep(move_step::MoveStep { id: "cut-1".into(), index: 2 }),
            Process3dCommand::UpdateStep(update_step::UpdateStep {
                step_json: serde_json::to_string(&crate::artifacts::process3d::ProcessStep {
                    id: "cut-1".into(),
                    label: "Cut".into(),
                    enabled: true,
                    origin: None,
                    measure: crate::artifacts::process3d::ProcessMeasure::Cut { tool: crate::artifacts::process3d::WorkingSolid::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: crate::artifacts::process3d::Pose::default() },
                })
                .expect("json"),
            }),
            Process3dCommand::SetStepEnabled(set_step_enabled::SetStepEnabled { id: "cut-1".into(), enabled: false }),
            Process3dCommand::SetStock(set_stock::SetStock { kind: "cylinder".into() }),
            Process3dCommand::PatchInspector(patch_inspector::PatchInspector { target: "beam".into(), field: "width".into(), number: Some(1.5), text: None }),
            Process3dCommand::SetCursor(set_cursor::SetCursor { value: Some(3) }),
            Process3dCommand::StepCursor(step_cursor::StepCursor { delta: -1 }),
            Process3dCommand::StepCursorBack(step_cursor_back::StepCursorBack {}),
            Process3dCommand::StepCursorForward(step_cursor_forward::StepCursorForward {}),
            Process3dCommand::EngagementSubmit(engagement_submit::EngagementSubmit {}),
            Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: [1.0, 2.0, 3.0] }),
            Process3dCommand::WorldFaceDragEnd(world_face_drag_end::WorldFaceDragEnd { normal: [0.0, 0.0, 1.0], start_point: [0.5, 0.5, 1.0], distance: -0.5, face_extent: Some([1.0, 1.0]) }),
            Process3dCommand::ImportModelFile(import_model_file::ImportModelFile { name: "beam.step".into(), payload: "data:application/octet-stream;base64,AAAA".into() }),
            Process3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "cut".into() }),
            Process3dCommand::EngagementInput(engagement_input::EngagementInput { value: "cut".into() }),
            Process3dCommand::EngagementAbort(engagement_abort::EngagementAbort {}),
            Process3dCommand::SetCamera(set_camera::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }),
            Process3dCommand::ToggleSun(toggle_sun::ToggleSun {}),
            Process3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 90.0 }),
            Process3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 45.0 }),
            Process3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 1.0 }),
            Process3dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            Process3dCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
            Process3dCommand::ExportModel(export_model::ExportModel { format: "step".into() }),
            Process3dCommand::LoadModelRequest(load_model_request::LoadModelRequest {}),
        ]
    }

    /// 🌉️ Every Process action emitted by React or wgpu must enter the same closed typed command
    /// vocabulary as native typed callers; undeclared strings fail at this single boundary.
    #[semio_framework_async_macros::async_test]
    async fn command_from_action_covers_every_declared_action_and_rejects_unknown_ones() {
        testkit::assert_declared_actions_bridge_to_commands::<EditorApp<Process3dPlayApp>>(process3d_app_manifest_for_testkit);
        assert!(Process3dPlayApp::command_from_action("nonsense", None).is_err());
    }

    /// 🖱️ The interaction payload shape that regressed retains its identifier through the transport
    /// bridge instead of being dropped into ad-hoc host state.
    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `setHover`/`contextMenuAt` were
    /// app-owned selection/hover commands, deleted along with `Process3dConfig::selected_id`/
    /// `hovered_id` — hover/selection now decode through the framework's auto-injected
    /// `interactionHover`/`interactionSelect` verbs instead of this app's own command vocabulary.
    #[semio_framework_async_macros::async_test]
    async fn interaction_actions_decode_into_typed_commands() {
        assert_eq!(
            Process3dPlayApp::command_from_action("setActiveExample", Some(&serde_json::json!({ "exampleId": PROCESS3D_EXAMPLE_PLATE }))).expect("example bridge"),
            Process3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: PROCESS3D_EXAMPLE_PLATE.into() })
        );
    }

    /// 📄️ Example switching exercises the complete registry-backed action path and emits the
    /// architecture's sanctioned whole-document load effect for the requested fixture.
    #[semio_framework_async_macros::async_test]
    async fn registry_backed_example_action_emits_the_requested_document() {
        let mut app = app_with_registry();
        let result = action(&mut app, "setActiveExample", Some(&serde_json::json!({ "exampleId": PROCESS3D_EXAMPLE_PLATE })));
        let Effect::LoadDocument { pack, .. } = result.requested_effects.first().expect("example action must load a document") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <Process3dSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode example document");
        assert_eq!(loaded, crate::artifacts::process3d::schema::plate_document());
    }

    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `worldPick` (a pure selection-setting
    /// command) is deleted along with `Process3dConfig::selected_face_id` — object/face picking now
    /// decodes through the framework's auto-injected `interactionSelect` verb, and the rendered
    /// world3d selection JSON carries no live selection ids anymore (see
    /// `🎭️modes/✏️edit/🪟️windows/🪚️workpiece`'s `process3d_selection_json` doc comment).
    #[semio_framework_async_macros::async_test]
    async fn world3d_render_carries_no_stale_selection_json_fields() {
        let mut app = app();
        let rendered = render_body(&mut app, PROCESS_3D_PLAY_BODY_MAIN);
        assert!(!rendered.contains("componentIds"), "componentIds is no longer emitted by the render boundary: {rendered}");
    }

    /// 🖱️ A world right-click has an app-owned menu to request; the host no longer falls through to
    /// an empty default.
    #[semio_framework_async_macros::async_test]
    async fn world_context_menu_exposes_process_commands() {
        let mut app = app_with_registry();
        let request = ContextMenuRequest {
            menu: UiMenuRef { id: "window".into(), args: None },
            surface: Some(ContextMenuSurfaceTarget { surface_id: "process.play".into(), kind: "world3d".into(), hits: Vec::new(), selection: Vec::new(), text: None }),
            window_instance_id: None,
            point: None,
        };
        let menu = app.context_menu(&request);
        let ids: Vec<&str> = menu.iter().map(|item| item.id.as_str()).collect();
        assert!(ids.contains(&"addStep"), "right-click menu must expose the primary Process command: {ids:?}");
        assert!(ids.contains(&"undo") && ids.contains(&"redo"), "right-click menu must expose history commands: {ids:?}");
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_process3d_app().definition).expect("app definition json");
        assert!(json.contains(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN), "window kind missing from the manifest");
        assert!(json.contains(edit::PROCESS3D_MODE_EDIT), "mode missing from the manifest");
        for body in [PROCESS_3D_PLAY_BODY_DOCUMENT, PROCESS_3D_PLAY_BODY_CATALOGUE, PROCESS_3D_PLAY_BODY_WORKSHOP, PROCESS_3D_PLAY_BODY_INSPECTION] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("3d.process"), "artifact kind missing from the manifest");
    }

    #[semio_framework_async_macros::async_test]
    async fn utility_registry_declares_four_flat_utilities_scoped_to_workpiece_window() {
        let definition = create_process3d_app().definition;
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, ["select", "cut", "drill", "attach"], "utilities declared in registry order");
        assert!(definition.utilities.iter().all(|utility| utility.group.is_none()), "process's select/cut/drill/attach are the window's entire top-level utility set, so none carry a visual group",);
        let window = definition.window_kinds.iter().find(|window| window.id == workpiece::PROCESS_3D_PLAY_WINDOW_MAIN).expect("workpiece window");
        let scoped: Vec<&str> = window.utilities.iter().map(|utility| utility.as_str()).collect();
        assert_eq!(scoped, ["select", "cut", "drill", "attach"], "all four utilities scoped to the workpiece window kind");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️IoTests
    /// 🔤️ `AppIo.export_formats`/`import_formats` (unlike `ArtifactKindSpec`) have no `export_stdio_kinds`/
    /// `import_stdio_kinds` string-id peer and are never read by `register_app_io`, so they stay empty
    /// here in step with `artifact_kind()`'s own now-empty lists (see that fn's doc).
    #[semio_framework_async_macros::async_test]
    async fn process3d_io_mirrors_the_declared_artifact_kind() {
        let io = process3d_io();
        assert_eq!(io.document_schema, crate::artifacts::process3d::PROCESS_3D_SCHEMA);
        assert_eq!(io.artifact.id, "3d.process");
        assert!(io.export_formats.is_empty());
        assert!(io.import_formats.is_empty());
    }

    /// 🔌️ WORKFLOWS-END-TO-END-TYPED-PORTS-REAL-SCHEMA-FLOW-CONFIG-ON-NODE Wave 2 port recipe:
    /// `geometry:in` and `brep:out` are declared with the right direction/kind/multiplicity.
    #[semio_framework_async_macros::async_test]
    async fn process3d_io_declares_geometry_in_and_brep_out_ports() {
        let io = process3d_io();
        let geometry_in = io.ports.iter().find(|port| port.id == "geometry:in").expect("geometry:in declared");
        assert_eq!(geometry_in.direction, semio_framework_plugin::MediaPortDirection::In);
        assert!(geometry_in.kind_id.is_none());
        assert!(!geometry_in.required);
        assert_eq!(geometry_in.multiplicity, semio_framework_plugin::PortMultiplicity::Many);

        let brep_out = io.ports.iter().find(|port| port.id == "brep:out").expect("brep:out declared");
        assert_eq!(brep_out.direction, semio_framework_plugin::MediaPortDirection::Out);
        assert_eq!(brep_out.kind_id.as_deref(), Some("3d.process"));
        assert!(!brep_out.required);
        assert_eq!(brep_out.multiplicity, semio_framework_plugin::PortMultiplicity::Many);
        assert_eq!(brep_out.media_type.class, MediaClass::ThreeD);
        assert_eq!(brep_out.media_type.form, MediaForm::Brep);
    }
    //#endregion 🔖️IoTests

    //#region 🔖️CrossCutting
    #[semio_framework_async_macros::async_test]
    async fn labels_resolve_native_by_default_and_in_german() {
        let mut config = Process3dConfig::default();
        assert_eq!(process3d_labels(&config).stock.as_str(), "Stock");
        config.locale = "de".into();
        assert_eq!(process3d_labels(&config).stock.as_str(), "Rohteil");
    }

    /// 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `AddStep` dispatches a
    /// `CreateStep` mutation, a documented no-op now (`steps` composes an `s.stdio.semio.flow`
    /// CHILD HANDLE — no resolver, see `ProcessWorkingScene`'s doc comment), so the step count
    /// never changes; `undo`/`redo` of a no-op are themselves no-ops, so the handle stays identical
    /// throughout.
    #[semio_framework_async_macros::async_test]
    async fn undo_after_add_step_leaves_the_steps_handle_unchanged() {
        let mut app = app();
        let before = app.snapshot().expect("snapshot").steps.clone();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None }),
            |app| app.snapshot().expect("snapshot").steps == before,
            true,
            true,
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn undo_after_add_workshop_machine_restores_previous_machine_count() {
        let mut app = app();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }),
            |app| app.snapshot().expect("snapshot").workshop.machines.len(),
            11,
            12,
        );
    }

    /// 🧬️ Swapping the stock kind resets the whole document (stock + cleared timeline), which has no
    /// in-history mutation (a whole-snapshot variant is banned outright), so `setStock` now surfaces as a
    /// `Effect::LoadDocument` rather than an `artifact_mutations` entry — `dispatch`'s in-process
    /// harness never applies `effects` to its own store, so this asserts on the emitted effect.
    #[semio_framework_async_macros::async_test]
    async fn arg_form_set_stock_emits_ops_reading_kind_arg() {
        let mut app = app();
        let result = dispatch(&mut app, Process3dCommand::SetStock(set_stock::SetStock { kind: "cylinder".into() }));
        assert!(result.mutations.is_empty(), "setStock replaces the whole document via an effect, not in-history mutations");
        let Effect::LoadDocument { pack, .. } = result.requested_effects.first().expect("setStock must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let document = <Process3dSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        let expected_solid = crate::artifacts::process3d::brep_child_handle("stock", &crate::artifacts::process3d::brep_snapshot_for_working_solid(&crate::artifacts::process3d::WorkingSolid::Cylinder { radius: 0.3, height: 1.0 }));
        assert_eq!(document.stock_solid, expected_solid, "setStock kind=cylinder must swap the stock solid to the real cylinder-content handle");
        let cleared_steps = crate::artifacts::process3d::flow_child_handle(&crate::artifacts::process3d::flow_snapshot_for_steps(&[], &Default::default()));
        assert_eq!(document.steps, cleared_steps, "swapping stock resets the step timeline");
    }

    fn set_utility(app: &mut crate::editor::process3d::testkit::Process3dApp, utility: &str) {
        dispatch(app, Process3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: utility.into() }));
    }

    /// 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `WorldPointerDown` dispatches
    /// `insert_step_mutations` → `CreateStep`, a documented no-op now (`steps` composes an
    /// `s.stdio.semio.flow` CHILD HANDLE — no resolver, see `ProcessWorkingScene`'s doc comment),
    /// so the placed step's real pose is no longer readable back off the persisted document. This
    /// asserts what remains real: the command still dispatches a mutation for a real world-space
    /// click.
    #[semio_framework_async_macros::async_test]
    async fn world_pointer_down_dispatches_a_mutation_for_a_real_click() {
        let mut app = app();
        set_utility(&mut app, "cut");
        let result = dispatch(&mut app, Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: [1.0, 2.0, 3.0] }));
        assert!(!result.mutations.is_empty(), "worldPointerDown must still dispatch a mutation for a real click");
    }

    #[semio_framework_async_macros::async_test]
    async fn world_pointer_down_resets_active_utility_to_select() {
        let mut app = app();
        set_utility(&mut app, "cut");
        let result = dispatch(&mut app, Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: [1.0, 2.0, 3.0] }));
        assert!(
            result.requested_effects.iter().any(|effect| matches!(effect, Effect::SetActiveUtility { utility_id, .. } if utility_id == "select")),
            "placing a step must hand the host a SetActiveUtility(select) effect so the click-to-place utility disengages",
        );
    }

    /// 🌉️ Same documented gap as `world_pointer_down_dispatches_a_mutation_for_a_real_click` — the
    /// per-click pose is no longer readable back off the persisted document, so this asserts that
    /// two distinct real clicks each still dispatch their own mutation.
    #[semio_framework_async_macros::async_test]
    async fn repeated_world_pointer_down_each_dispatch_a_mutation() {
        let mut app = app();
        set_utility(&mut app, "cut");
        let first = dispatch(&mut app, Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: [1.0, 0.0, 0.0] }));
        set_utility(&mut app, "cut");
        let second = dispatch(&mut app, Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: [2.0, 0.0, 0.0] }));
        assert!(!first.mutations.is_empty() && !second.mutations.is_empty(), "each real click must dispatch its own mutation");
    }

    /// 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `WorldFaceDragEnd` dispatches
    /// `insert_step_mutations` → `CreateStep`, a documented no-op now (`steps` composes an
    /// `s.stdio.semio.flow` CHILD HANDLE — no resolver, see `ProcessWorkingScene`'s doc comment),
    /// so the resulting document can no longer be read back to verify the replayed volume through
    /// the app-command pipeline (`PatchInspector`'s dimension patch is a documented no-op for the
    /// same reason — see `🎮️commands/🔎️inspector`'s own doc comment). The real, unaffected
    /// kernel-replay math (cut/attach volume deltas) is covered directly against a literal
    /// `ProcessWorkingScene` by `🧬️schema/💡️inferences`'s own
    /// `drill_reduces_volume_below_stock`/`attach_increases_volume_above_stock` tests; these two
    /// now assert only that the command still dispatches a mutation for a real face-drag gesture.
    #[semio_framework_async_macros::async_test]
    async fn world_face_drag_end_cut_dispatches_a_mutation() {
        let mut app = app();
        let result = dispatch(&mut app, Process3dCommand::WorldFaceDragEnd(world_face_drag_end::WorldFaceDragEnd { normal: [0.0, 0.0, 1.0], start_point: [0.5, 0.5, 1.0], distance: -0.5, face_extent: Some([1.0, 1.0]) }));
        assert!(!result.mutations.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn world_face_drag_end_attach_dispatches_a_mutation() {
        let mut app = app();
        let result = dispatch(&mut app, Process3dCommand::WorldFaceDragEnd(world_face_drag_end::WorldFaceDragEnd { normal: [0.0, 0.0, 1.0], start_point: [0.5, 0.5, 1.0], distance: 0.5, face_extent: Some([0.2, 0.2]) }));
        assert!(!result.mutations.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn world_face_drag_end_ignored_while_a_placement_utility_is_active() {
        let mut app = app();
        set_utility(&mut app, "cut");
        let result = dispatch(&mut app, Process3dCommand::WorldFaceDragEnd(world_face_drag_end::WorldFaceDragEnd { normal: [0.0, 0.0, 1.0], start_point: [0.5, 0.5, 1.0], distance: -0.5, face_extent: None }));
        assert!(result.mutations.is_empty(), "worldFaceDragEnd should be a no-operation while a placement utility is active, not the select utility");
    }

    #[semio_framework_async_macros::async_test]
    async fn toggle_sun_round_trips_through_config_and_defaults_off() {
        let mut app = app();
        let measures = app.window_measures();
        let sun_group = |measures: &HashMap<String, Vec<WindowMeasure>>| {
            measures[workpiece::PROCESS_3D_PLAY_WINDOW_MAIN]
                .iter()
                .find_map(|measure| match measure {
                    WindowMeasure::Group { id, children, .. } if id == "process3d-measure-sun" => Some(children.clone()),
                    _ => None,
                })
                .expect("sun measure group")
        };
        let children = sun_group(&measures);
        assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { pressed, .. } if !*pressed)));
        dispatch(&mut app, Process3dCommand::ToggleSun(toggle_sun::ToggleSun {}));
        let measures = app.window_measures();
        let children = sun_group(&measures);
        assert!(children.iter().any(|measure| matches!(measure, WindowMeasure::Toggle { pressed, .. } if *pressed)));
    }

    #[semio_framework_async_macros::async_test]
    async fn window_measures_surface_the_sun_group() {
        let mut app = app();
        let measures = main_window_measures(&mut app);
        assert_eq!(measures.len(), 1);
        assert!(matches!(&measures[0], WindowMeasure::Group { id, .. } if id == "process3d-measure-sun"));
    }

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        let mut app = app();
        assert!(render_body(&mut app, "process3d.play.nope").contains("Unknown body"));
    }

    #[semio_framework_async_macros::async_test]
    async fn window_body_accepts_the_framework_instance_suffix() {
        let mut app = app();
        let body_key = format!("{}:{}", workpiece::PROCESS_3D_PLAY_BODY_MAIN, workpiece::PROCESS_3D_PLAY_WINDOW_MAIN);
        let rendered = render_body(&mut app, &body_key);
        assert!(rendered.contains("processed"), "window-instance body key must render the Process world: {rendered}");
    }

    /// 🧪️ The registry-enforced app must accept every declared manifest action id without a kind-
    /// discipline error — proves the `app_commands!` rows and the manifest's `.operation`/`.shell_action`/
    /// `.action_with` declarations stay in sync.
    #[semio_framework_async_macros::async_test]
    async fn registry_enforced_app_accepts_a_declared_operation_action() {
        let mut app = app_with_registry();
        let result = dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None }));
        assert!(!result.mutations.is_empty());
    }

    //#region 🔖️MediaTests
    #[semio_framework_async_macros::async_test]
    async fn export_brep_out_returns_step_text_structured_payload() {
        let app = Process3dPlayApp;
        let document = crate::artifacts::process3d::schema::default_document();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let media = semio_framework_plugin::resolve_ready(Process3dPlayApp::export_media("brep:out", &doc)).expect("export brep:out");
        assert_eq!(media.media_type.class, MediaClass::ThreeD);
        assert_eq!(media.media_type.form, MediaForm::Brep);
        match media.payload {
            MediaPayload::Structured { schema, json } => {
                assert_eq!(schema, "3d.process");
                assert!(!json.is_empty());
            }
            MediaPayload::Binary { .. } => panic!("expected a Structured payload"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn export_unknown_port_is_not_implemented() {
        let app = Process3dPlayApp;
        let document = crate::artifacts::process3d::schema::default_document();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        assert!(matches!(semio_framework_plugin::resolve_ready(Process3dPlayApp::export_media("nonsense:out", &doc)), Err(MediaError::NotImplemented)));
    }

    #[semio_framework_async_macros::async_test]
    async fn import_geometry_in_rejects_unrecognized_schema() {
        let app = Process3dPlayApp;
        let document = crate::artifacts::process3d::schema::default_document();
        let history = HistoryView::empty();
        let doc = ArtifactView::new(&document, &history);
        let media = semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep }, payload: MediaPayload::Structured { schema: "unknown.schema".into(), json: "irrelevant".into() } };
        assert!(matches!(Process3dPlayApp::import_media("geometry:in", &media, &doc), Err(MediaError::Payload(port, _)) if port == "geometry:in"));
    }
    //#endregion 🔖️MediaTests

    //#region 🔖️BehaviorTests
    #[semio_framework_async_macros::async_test]
    async fn face_drag_orients_box_along_normal() {
        let (axis, angle) = axis_angle_from_up_to([0.0, 1.0, 0.0]);
        assert!((angle - std::f64::consts::FRAC_PI_2).abs() < 1e-9);
        assert!((axis[0] - (-1.0)).abs() < 1e-9 && axis[1].abs() < 1e-9 && axis[2].abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn face_drag_degenerate_antiparallel_normal_does_not_panic() {
        let (_, angle) = axis_angle_from_up_to([0.0, 0.0, -1.0]);
        assert!((angle - std::f64::consts::PI).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn process_machine_contributions_are_configuration_owned() {
        use semio_framework::{ProgramContributionEntry, TopicContribution};
        let machine = crate::artifacts::process3d::WorkshopMachine { id: "hot-saw".into(), label: "Hot Saw".into(), icon_id: "scissors".into(), catalog_id: None, capabilities: vec![] };
        let entry = ProgramContributionEntry {
            plugin_id: "process-module-test".into(),
            topic_contribution: Some(TopicContribution::new(
                "process.machines",
                serde_json::json!({
                    "appId": "process3d-play",
                    "moduleId": "hot-catalog",
                    "label": "Hot Catalog",
                    "iconId": "wrench",
                    "machinesJson": serde_json::to_string(&vec![machine]).unwrap(),
                }),
            )),
        };
        let json = serde_json::to_string(&vec![entry]).unwrap();
        assert!(installed_catalogs(&json).iter().any(|catalog| catalog.catalog_id() == "hot-catalog"));
        assert!(!installed_catalogs("[]").iter().any(|catalog| catalog.catalog_id() == "hot-catalog"));
    }

    #[semio_framework_async_macros::async_test]
    async fn process_contribution_envelope_accepts_exact_limits_and_rejects_plus_one() {
        let raw_max = format!("{{}}{}", " ".repeat(PROCESS_CONTRIBUTION_MAX_BYTES - 2));
        assert!(process_json_envelope_is_bounded(&raw_max));
        assert!(!process_json_envelope_is_bounded(&(raw_max + " ")));

        let depth_max = format!("{}0{}", "[".repeat(PROCESS_CONTRIBUTION_MAX_DEPTH), "]".repeat(PROCESS_CONTRIBUTION_MAX_DEPTH));
        assert!(process_json_envelope_is_bounded(&depth_max));
        let depth_plus_one = format!("{}0{}", "[".repeat(PROCESS_CONTRIBUTION_MAX_DEPTH + 1), "]".repeat(PROCESS_CONTRIBUTION_MAX_DEPTH + 1));
        assert!(!process_json_envelope_is_bounded(&depth_plus_one));

        let string_max = format!("\"{}\"", "x".repeat(PROCESS_CONTRIBUTION_MAX_STRING_BYTES));
        assert!(process_json_envelope_is_bounded(&string_max));
        let string_plus_one = format!("\"{}\"", "x".repeat(PROCESS_CONTRIBUTION_MAX_STRING_BYTES + 1));
        assert!(!process_json_envelope_is_bounded(&string_plus_one));

        let items_max = format!("[{}]", vec!["0"; PROCESS_CONTRIBUTION_MAX_ITEMS - 1].join(","));
        assert!(process_json_envelope_is_bounded(&items_max));
        let items_plus_one = format!("[{}]", vec!["0"; PROCESS_CONTRIBUTION_MAX_ITEMS].join(","));
        assert!(!process_json_envelope_is_bounded(&items_plus_one));
    }
    //#endregion 🔖️BehaviorTests
}
//#endregion 🧪️Tests
