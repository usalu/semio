//! 🖥️ Process 3d play app — the `DocumentApp` impl (dispatch-only), the aggregated command enum and
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
//! written via `config::Process3dConfigOperation`s; every action dispatches through the single typed
//! `Process3dCommand` channel via `DocumentApp::handle`.

use crate::apps::process3d::commands::{camera, cursor, document, engagement, inspector, locale, media, selection, step, stock, sun, utility, workshop, world};
use crate::apps::process3d::config::{Process3dConfig, Process3dConfigOperation};
use crate::apps::process3d::modes::edit;
use crate::apps::process3d::modes::edit::windows::workpiece;
use crate::apps::process3d::panels::{catalogue, document as document_panel, inspection, workshop as workshop_panel};
use crate::apps::process3d::terminology::process3d_labels;
use crate::artifacts::process3d::op::Process3dOperation;
use crate::artifacts::process3d::Process3dDocument;
use semio_framework_core::kernel::HostEffect;
use semio_framework_plugin::{NoDraft, NoDraftOperation, DraftView, 
    ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, ArtifactKindSpec, ConfigView, DocumentApp, DocumentView, Emit, Fault, Label, LocalizedLabel, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, OsMediaCapability,
    OsMediaFormat, UiNode, UiTreeItemNode, UtilityCategory, UtilityDefinition, WindowMeasure,
};
use store::EngineHandles;
use serde_json::Value;
use std::collections::HashMap;
use store::DocumentPack;

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_APP_ID: &str = "process3d-play";
const PROCESS_3D_PLAY_CONTROLLER_ID: &str = "process3d-play";
pub const PROCESS3D_EXAMPLE_TIMBER: &str = "timber-beam-joinery";
pub const PROCESS3D_EXAMPLE_PLATE: &str = "drilled-plate";
pub use workpiece::PROCESS_3D_PLAY_BODY_MAIN;
pub use document_panel::PROCESS_3D_PLAY_BODY_DOCUMENT;
pub use catalogue::PROCESS_3D_PLAY_BODY_CATALOGUE;
pub use workshop_panel::PROCESS_3D_PLAY_BODY_WORKSHOP;
pub use inspection::PROCESS_3D_PLAY_BODY_INSPECTION;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`🎚️options/*`, `📌️panels/*`, `🎮️commands/*`) builds its `on_change`/item actions with.
pub fn process3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(PROCESS_3D_PLAY_CONTROLLER_ID).action(action, args)
}

/// 📇️ A non-palette action declaration (dispatched by UI wiring/keybindings, never surfaced in the
/// command palette) with the given execution kind.
fn internal_action(id: &str, label: impl Into<LocalizedLabel>, kind: ActionKind) -> ActionDefinition {
    ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog(id, label, kind) }
}

/// 🧰️ Host effect that programmatically switches the workpiece window's active utility — the active
/// utility is also mirrored into `Process3dConfig::active_utility_id` (via `SetActiveUtility`) for
/// rendering, but the window chrome itself is still driven by this host effect. Shared by
/// `🎮️commands/🎛️engagement` and `🎮️commands/🌍️world`.
pub fn set_active_utility_effect(utility: &str) -> HostEffect {
    HostEffect::SetActiveUtility { window_id: workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into(), utility_id: utility.into() }
}

/// 🎨️ `tree_item_with_action` (SDK) carries no icon slot, so this app-wide wrapper layers `icon_id` on
/// top via struct-update syntax — shared by the `🛍️catalogue` and `🛠️workshop` panels.
pub fn iconed_tree_item_with_action(id: impl Into<String>, label: impl Into<Label>, icon_id: &str, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode { icon_id: Some(icon_id.into()), menu: None, ..semio_framework_plugin::tree_item_with_action(id, label, None, action) }
}
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Process3dPlayApp::Command` — the SOLE dispatch surface for process3d's own behavior, assembled
    /// from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`) and the `dsl` wire keyword (the kebab `#[dsl(key = ..)]` the codec uses) — copied
    /// verbatim from the pre-migration `Process3dCommand`/`command_id()` match. **Row order is the binary
    /// variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum Process3dCommand for Process3dDocument, Process3dOperation, Process3dConfig, Process3dConfigOperation {
        "setDocument" as "document" => set_document::SetDocument,
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
        "setSelection" as "set-selection" => set_selection::SetSelection,
        "setHover" as "set-hover" => set_hover::SetHover,
        "setCamera" as "camera" => set_camera::SetCamera,
        "worldPick" as "world-pick" => world_pick::WorldPick,
        "toggleSun" as "toggle-sun" => toggle_sun::ToggleSun,
        "setSunAzimuth" as "sun-azimuth" => set_sun_azimuth::SetSunAzimuth,
        "setSunElevation" as "sun-elevation" => set_sun_elevation::SetSunElevation,
        "setSunIntensity" as "sun-intensity" => set_sun_intensity::SetSunIntensity,
        "setLocale" as "locale" => set_locale::SetLocale,
        "exportModel" as "export-model" => export_model::ExportModel,
        "loadModelRequest" as "load-model-request" => load_model_request::LoadModelRequest,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name.
use camera::set_camera;
use cursor::{set_cursor, step_cursor, step_cursor_back, step_cursor_forward};
use document::{set_active_example, set_document};
use engagement::{engagement_abort, engagement_input, engagement_submit};
use inspector::patch_inspector;
use locale::set_locale;
use media::{export_model, import_model_file, load_model_request};
use selection::{set_hover, set_selection};
use step::{add_step, move_step, remove_selected_step, remove_step, set_step_enabled, update_step};
use stock::set_stock;
use sun::{set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use utility::set_active_utility;
use workshop::{add_workshop_machine, remove_workshop_machine, update_workshop_machine};
use world::{world_face_drag_end, world_pick, world_pointer_down};
//#endregion 🔖️Commands

//#region 🔖️Process3dPlayApp
/// 🧪️ B1: unit struct — every former `Process3dRuntime` field now lives in `config::Process3dConfig`
/// (see `DocumentApp::Config`), written through `config::Process3dConfigOperation`s.
#[derive(Default)]
pub struct Process3dPlayApp;

impl DocumentApp for Process3dPlayApp {
    type Projection = Process3dDocument;
    type Operation = Process3dOperation;
    type Config = Process3dConfig;
    type ConfigOperation = Process3dConfigOperation;
    type Draft = NoDraft;
    type DraftOperation = NoDraftOperation;

    type Command = Process3dCommand;

    const APP_ID: &'static str = PROCESS_3D_PLAY_APP_ID;

    const DOCUMENT_SCHEMA: &'static str = crate::artifacts::process3d::PROCESS_3D_SCHEMA;

    fn initial_projection() -> Process3dDocument {
        crate::artifacts::process3d::engine::default_document()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(crate::artifacts::process3d::engine::process3d_io())
    }

    //#region 🔖️Media
    /// 🎞️ `brep:out` (see the artifact engine's `export_process3d_model`, STEP text) plus the inherited
    /// `document:out` default (the pack of `doc.projection`, replicated inline — overriding `export_media`
    /// shadows the trait's provided body for every port on this app, not just the new one).
    fn export_media(port: &str, doc: &DocumentView<'_, Process3dDocument>) -> Result<semio_framework_plugin::Media, MediaError> {
        match port {
            "brep:out" => match crate::artifacts::process3d::engine::export_process3d_model(doc.projection, "step") {
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
                let media_type = Self::io().map_or(MediaType { class: MediaClass::Data, form: MediaForm::Value }, |io| io.document_media_type);
                let bytes = doc.projection.encode_pack();
                Ok(semio_framework_plugin::Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn whole_document_operation(projection: Process3dDocument) -> Option<Process3dOperation> {
        Some(Process3dOperation::SetDocument { document: projection })
    }

    /// 📥️ `geometry:in` (best-effort STEP-text import) plus the inherited `document:in` default (base64
    /// pack via `whole_document_operation`, replicated inline — overriding `import_media` shadows the
    /// trait's provided body for every port).
    fn import_media(port: &str, media: &semio_framework_plugin::Media, _doc: &DocumentView<'_, Process3dDocument>) -> Result<Emit<Process3dOperation, Process3dConfigOperation, Self::DraftOperation>, MediaError> {
        match port {
            "geometry:in" => {
                let MediaPayload::Structured { schema, json } = &media.payload else {
                    return Err(MediaError::Payload("geometry:in".into(), "expected a structured payload".into()));
                };
                if schema != crate::artifacts::process3d::PROCESS_3D_SCHEMA && schema != "3d.process" {
                    return Err(MediaError::Payload("geometry:in".into(), format!("unrecognized schema: {schema}")));
                }
                // 📦️ `export_process3d_model("step")` hands back raw (non-base64) STEP text — see
                // `OsMediaFormat::Step::is_binary() == false` — so this re-encodes it as base64 to
                // satisfy `import_process3d_model`'s `data:...,<base64>` expectation.
                use base64::Engine;
                let data_url = format!("data:application/octet-stream;base64,{}", base64::engine::general_purpose::STANDARD.encode(json.as_bytes()));
                match crate::artifacts::process3d::engine::import_process3d_model("geometry-in.step", &data_url) {
                    Some(document) => Ok(Emit::operations(vec![Process3dOperation::SetDocument { document }])),
                    None => Err(MediaError::Payload("geometry:in".into(), "STEP import failed".into())),
                }
            }
            "document:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                let projection = <Process3dDocument as DocumentPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
                match Self::whole_document_operation(projection) {
                    Some(operation) => Ok(Emit::operations(vec![operation])),
                    None => Err(MediaError::NotImplemented),
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

    fn handle(command: &Process3dCommand, doc: &DocumentView<'_, Process3dDocument>, cfg: &ConfigView<'_, Process3dConfig>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<Process3dOperation, Process3dConfigOperation, Self::DraftOperation>, Fault> {
        command.dispatch(doc, cfg)
    }

    /// 🧮️ process3d exposes no genuinely settings-like sticky defaults — every `Process3dConfig` field
    /// is session-only view state, so this stays at the trait default.
    fn config_spec() -> semio_framework_plugin::ConfigSpec {
        semio_framework_plugin::ConfigSpec::empty()
    }

    fn render(body_key: &str, doc: &DocumentView<'_, Process3dDocument>, cfg: &ConfigView<'_, Process3dConfig>) -> UiNode {
        let config = cfg.projection;
        let labels = process3d_labels(config);
        match body_key {
            PROCESS_3D_PLAY_BODY_MAIN => workpiece::render(doc.projection, config),
            PROCESS_3D_PLAY_BODY_DOCUMENT => document_panel::render(doc.projection, config, labels),
            PROCESS_3D_PLAY_BODY_CATALOGUE => catalogue::render(doc.projection, labels),
            PROCESS_3D_PLAY_BODY_WORKSHOP => workshop_panel::render(doc.projection, config, labels),
            PROCESS_3D_PLAY_BODY_INSPECTION => inspection::render(doc.projection, config, labels),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_engagements(doc: &DocumentView<'_, Process3dDocument>, cfg: &ConfigView<'_, Process3dConfig>) -> HashMap<String, semio_framework_plugin::WindowEngagement> {
        HashMap::from([(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into(), workpiece::engagement(doc.projection, cfg.projection, process3d_labels(cfg.projection)))])
    }

    fn window_measures(_doc: &DocumentView<'_, Process3dDocument>, cfg: &ConfigView<'_, Process3dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        HashMap::from([(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN.into(), workpiece::window_measures(cfg.projection))])
    }
}
//#endregion 🔖️Process3dPlayApp

//#region 🔖️Manifest
/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding declarations (which have no dedicated `_def` passthrough) are written
/// out inline. `WindowKindDefinition.options.measures` stays empty: measures are config-derived per
/// frame by `DocumentApp::window_measures`, never frozen into the manifest.
pub fn create_process3d_app() -> App {
    App::from_builder(
        App::builder(PROCESS_3D_PLAY_APP_ID, LocalizedLabel::native("Process 3D", "Process 3D"))
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
                export_formats: vec![OsMediaFormat::Step, OsMediaFormat::Obj, OsMediaFormat::Stl, OsMediaFormat::Glb],
                import_formats: vec![OsMediaFormat::Step, OsMediaFormat::Obj, OsMediaFormat::Stl],
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
            .operation("addStep", LocalizedLabel::native("Add Step", "Schritt hinzufügen"))
            .operation("setStock", LocalizedLabel::native("Set Stock", "Rohteil festlegen"))
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .operation("removeSelectedStep", LocalizedLabel::native("Remove Selected Step", "Ausgewählten Schritt entfernen"))
            // 🐚️ Palette-visible host round-trips.
            .shell_action("exportModel", LocalizedLabel::native("Export Model", "Modell exportieren"))
            .shell_action("loadModelRequest", LocalizedLabel::native("Load Model…", "Modell laden…"))
            // 🔧️ Internal document mutations dispatched by panel/viewport wiring (not palette-worthy).
            .action_with(internal_action("setDocument", LocalizedLabel::native("Set Document", "Dokument festlegen"), ActionKind::Operation))
            .action_with(internal_action("addWorkshopMachine", LocalizedLabel::native("Add Machine", "Maschine hinzufügen"), ActionKind::Operation))
            .action_with(internal_action("removeWorkshopMachine", LocalizedLabel::native("Remove Machine", "Maschine entfernen"), ActionKind::Operation))
            .action_with(internal_action("updateWorkshopMachine", LocalizedLabel::native("Update Machine", "Maschine aktualisieren"), ActionKind::Operation))
            .action_with(internal_action("importModelFile", LocalizedLabel::native("Import Model File", "Modelldatei importieren"), ActionKind::Operation))
            .action_with(internal_action("removeStep", LocalizedLabel::native("Remove Step", "Schritt entfernen"), ActionKind::Operation))
            .action_with(internal_action("moveStep", LocalizedLabel::native("Move Step", "Schritt verschieben"), ActionKind::Operation))
            .action_with(internal_action("updateStep", LocalizedLabel::native("Update Step", "Schritt aktualisieren"), ActionKind::Operation))
            .action_with(internal_action("setStepEnabled", LocalizedLabel::native("Set Step Enabled", "Schrittaktivierung festlegen"), ActionKind::Operation))
            .action_with(internal_action("patchInspector", LocalizedLabel::native("Patch Inspector", "Inspektor aktualisieren"), ActionKind::Operation))
            .action_with(internal_action("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"), ActionKind::Operation))
            .action_with(internal_action("worldFaceDragEnd", LocalizedLabel::native("World Face Drag End", "Welt-Flächenzug beendet"), ActionKind::Operation))
            // ⏱️ Document-cursor navigation operations (NOT framework History — they move the replay cursor).
            .action_with(internal_action("setCursor", LocalizedLabel::native("Set Cursor", "Cursor festlegen"), ActionKind::Operation))
            .action_with(internal_action("stepCursor", LocalizedLabel::native("Step Cursor", "Cursor schrittweise bewegen"), ActionKind::Operation))
            .action_with(internal_action("stepCursorBack", LocalizedLabel::native("Step Cursor Back", "Cursor zurück"), ActionKind::Operation))
            .action_with(internal_action("stepCursorForward", LocalizedLabel::native("Step Cursor Forward", "Cursor vorwärts"), ActionKind::Operation))
            // 🎛️ Engagement session command line (a separate system from utility selection).
            .action_with(internal_action("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Operation))
            .action_with(internal_action("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View))
            .action_with(internal_action("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View))
            // 👁️ Ephemeral view state — selection, hover, camera, face picking, sun.
            .action_with(internal_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"), ActionKind::View))
            .action_with(internal_action("setHover", LocalizedLabel::native("Set Hover", "Überfahren festlegen"), ActionKind::View))
            .action_with(internal_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View))
            .action_with(internal_action("worldPick", LocalizedLabel::native("World Pick", "Welt-Auswahl (Pick)"), ActionKind::View))
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
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .keybinding("bracketleft", "stepCursorBack")
            .keybinding("bracketright", "stepCursorForward")
            .keybinding("escape", "engagementAbort")
            .keybinding("delete", "removeSelectedStep")
            .keybinding("backspace", "removeSelectedStep")
            .config(Process3dPlayApp::config_spec())
            .io(crate::artifacts::process3d::engine::process3d_io()),
    )
    .example(PROCESS3D_EXAMPLE_TIMBER, LocalizedLabel::native("Timber Beam Joinery", "Holzbalkenverbindung"), crate::artifacts::process3d::engine::TIMBER_EXAMPLE_DSL, "file-text")
    .example(PROCESS3D_EXAMPLE_PLATE, LocalizedLabel::native("Drilled Plate", "Gebohrte Platte"), crate::artifacts::process3d::engine::PLATE_EXAMPLE_DSL, "file-text")
    .workflow("process3d", "Process 3D", "brep")
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{InvocationResult, PluginApp, VcsDocumentApp, ViewState};

    pub type Process3dApp = VcsDocumentApp<Process3dPlayApp>;

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn app() -> Process3dApp {
        new_app::<Process3dPlayApp>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn app_with_registry() -> Process3dApp {
        new_app_with_registry::<Process3dPlayApp>(create_process3d_app)
    }

    pub fn dispatch(app: &mut Process3dApp, command: Process3dCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut Process3dApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewState::default()).expect("render")).expect("render json")
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
    use crate::apps::process3d::testkit::{app, app_with_registry, dispatch, main_window_measures, render as render_body};
    use semio_framework_plugin::{testkit, HistoryView, PluginApp, SET_ACTIVE_UTILITY_ACTION_ID};

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
    /// wire keyword must be distinct.
    #[test]
    fn command_ids_are_unique_and_match_the_declared_manifest_actions() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 35, "every Process3dCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword — copied
    /// verbatim from the pre-migration `Process3dCommand`/`command_id()` match (the two vocabularies
    /// genuinely diverge for about a third of process3d's rows, unlike flow's single `setLocale`
    /// exception, so this pins the full table rather than deriving it from a kebab-case guess).
    #[test]
    fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        let expected_wire_key = |id: &str| -> &'static str {
            match id {
                "setDocument" => "document",
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
                "setSelection" => "set-selection",
                "setHover" => "set-hover",
                "setCamera" => "camera",
                "worldPick" => "world-pick",
                "toggleSun" => "toggle-sun",
                "setSunAzimuth" => "sun-azimuth",
                "setSunElevation" => "sun-elevation",
                "setSunIntensity" => "sun-intensity",
                "setLocale" => "locale",
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
            Process3dCommand::SetDocument(set_document::SetDocument { document: crate::artifacts::process3d::empty_process3d_projection() }),
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
                step: crate::artifacts::process3d::ProcessStep {
                    id: "cut-1".into(),
                    label: "Cut".into(),
                    enabled: true,
                    origin: None,
                    measure: crate::artifacts::process3d::ProcessMeasure::Cut { tool: crate::artifacts::process3d::SolidSpec::Box { width: 0.1, depth: 0.1, height: 0.1 }, pose: crate::artifacts::process3d::Pose::default() },
                },
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
            Process3dCommand::SetSelection(set_selection::SetSelection { id: Some("stock".into()) }),
            Process3dCommand::SetHover(set_hover::SetHover { id: Some("step-0".into()) }),
            Process3dCommand::SetCamera(set_camera::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }),
            Process3dCommand::WorldPick(world_pick::WorldPick { granularity: "face".into(), id: Some(7) }),
            Process3dCommand::ToggleSun(toggle_sun::ToggleSun {}),
            Process3dCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 90.0 }),
            Process3dCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 45.0 }),
            Process3dCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 1.0 }),
            Process3dCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            Process3dCommand::ExportModel(export_model::ExportModel { format: "step".into() }),
            Process3dCommand::LoadModelRequest(load_model_request::LoadModelRequest {}),
        ]
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_process3d_app().definition).expect("app definition json");
        assert!(json.contains(workpiece::PROCESS_3D_PLAY_WINDOW_MAIN), "window kind missing from the manifest");
        assert!(json.contains(edit::PROCESS3D_MODE_EDIT), "mode missing from the manifest");
        for body in [PROCESS_3D_PLAY_BODY_DOCUMENT, PROCESS_3D_PLAY_BODY_CATALOGUE, PROCESS_3D_PLAY_BODY_WORKSHOP, PROCESS_3D_PLAY_BODY_INSPECTION] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("3d.process"), "artifact kind missing from the manifest");
    }

    #[test]
    fn utility_registry_declares_four_flat_utilities_scoped_to_workpiece_window() {
        let definition = create_process3d_app().definition;
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, ["select", "cut", "drill", "attach"], "utilities declared in registry order");
        assert!(
            definition.utilities.iter().all(|utility| utility.group.is_none()),
            "process's select/cut/drill/attach are the window's entire top-level utility set, so none carry a visual group",
        );
        let window = definition.window_kinds.iter().find(|window| window.id == workpiece::PROCESS_3D_PLAY_WINDOW_MAIN).expect("workpiece window");
        let scoped: Vec<&str> = window.utilities.iter().map(|utility| utility.as_str()).collect();
        assert_eq!(scoped, ["select", "cut", "drill", "attach"], "all four utilities scoped to the workpiece window kind");
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    #[test]
    fn labels_resolve_native_by_default_and_in_german() {
        let mut config = Process3dConfig::default();
        assert_eq!(process3d_labels(&config).stock.as_str(), "Stock");
        config.locale = "de".into();
        assert_eq!(process3d_labels(&config).stock.as_str(), "Rohteil");
    }

    #[test]
    fn undo_after_add_step_restores_previous_step_count() {
        let mut app = app();
        testkit::assert_undo_redo_round_trip(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None }), |app| app.projection().expect("projection").steps.len(), 4, 5);
    }

    #[test]
    fn undo_after_add_workshop_machine_restores_previous_machine_count() {
        let mut app = app();
        testkit::assert_undo_redo_round_trip(
            &mut app,
            Process3dCommand::AddWorkshopMachine(add_workshop_machine::AddWorkshopMachine { catalog_id: "metal".into(), machine_id: "chopSaw".into() }),
            |app| app.projection().expect("projection").workshop.machines.len(),
            7,
            8,
        );
    }

    #[test]
    fn arg_form_set_stock_emits_ops_reading_kind_arg() {
        let mut app = app();
        let result = dispatch(&mut app, Process3dCommand::SetStock(set_stock::SetStock { kind: "cylinder".into() }));
        assert!(!result.operations.is_empty(), "the setStock arg form must materialize into document operations");
        let document = app.projection().expect("projection");
        assert!(matches!(document.stock.solid, crate::artifacts::process3d::SolidSpec::Cylinder { .. }), "setStock kind=cylinder must swap the stock solid");
        assert!(document.steps.is_empty(), "swapping stock resets the step timeline");
    }

    fn step_pose(step: &crate::artifacts::process3d::ProcessStep) -> [f64; 3] {
        match &step.measure {
            crate::artifacts::process3d::ProcessMeasure::Cut { pose, .. } | crate::artifacts::process3d::ProcessMeasure::Drill { pose, .. } | crate::artifacts::process3d::ProcessMeasure::Attach { pose, .. } => pose.position,
        }
    }

    fn set_utility(app: &mut crate::apps::process3d::testkit::Process3dApp, utility: &str) {
        dispatch(app, Process3dCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: utility.into() }));
    }

    #[test]
    fn world_pointer_down_reads_position_field_not_point() {
        let mut app = app();
        set_utility(&mut app, "cut");
        let result = dispatch(&mut app, Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: [1.0, 2.0, 3.0] }));
        assert!(!result.operations.is_empty(), "worldPointerDown must read the position the renderer actually sends");
        let document = app.projection().expect("projection");
        let last = document.steps.last().expect("inserted step");
        assert_eq!(step_pose(last), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn world_pointer_down_resets_active_utility_to_select() {
        let mut app = app();
        set_utility(&mut app, "cut");
        let result = dispatch(&mut app, Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: [1.0, 2.0, 3.0] }));
        assert!(
            result.requested_effects.iter().any(|effect| matches!(effect, HostEffect::SetActiveUtility { utility_id, .. } if utility_id == "select")),
            "placing a step must hand the host a SetActiveUtility(select) effect so the click-to-place utility disengages",
        );
    }

    #[test]
    fn repeated_world_pointer_down_places_steps_at_distinct_positions() {
        let mut app = app();
        set_utility(&mut app, "cut");
        dispatch(&mut app, Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: [1.0, 0.0, 0.0] }));
        set_utility(&mut app, "cut");
        dispatch(&mut app, Process3dCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { position: [2.0, 0.0, 0.0] }));
        let document = app.projection().expect("projection");
        let last_two: Vec<&crate::artifacts::process3d::ProcessStep> = document.steps.iter().rev().take(2).collect();
        assert_ne!(step_pose(last_two[0]), step_pose(last_two[1]), "repeated clicks at different points must produce distinct step poses");
    }

    #[test]
    fn world_face_drag_end_cut_reduces_volume_end_to_end() {
        let mut app = app();
        dispatch(&mut app, Process3dCommand::SetStock(set_stock::SetStock { kind: "box".into() }));
        let stock_volume = crate::artifacts::process3d::engine::processed_volume(&app.projection().expect("projection")).expect("stock volume");
        let result = dispatch(&mut app, Process3dCommand::WorldFaceDragEnd(world_face_drag_end::WorldFaceDragEnd { normal: [0.0, 0.0, 1.0], start_point: [0.5, 0.5, 1.0], distance: -0.5, face_extent: Some([1.0, 1.0]) }));
        assert!(!result.operations.is_empty());
        let document = app.projection().expect("projection");
        assert_eq!(document.steps.len(), 1);
        assert!(matches!(document.steps[0].measure, crate::artifacts::process3d::ProcessMeasure::Cut { .. }));
        let new_volume = crate::artifacts::process3d::engine::processed_volume(&document).expect("volume after cut");
        assert!(new_volume < stock_volume, "face-drag cut should reduce volume below stock ({new_volume} vs {stock_volume})");
    }

    #[test]
    fn world_face_drag_end_attach_increases_volume_end_to_end() {
        let mut app = app();
        dispatch(&mut app, Process3dCommand::SetStock(set_stock::SetStock { kind: "box".into() }));
        let stock_volume = crate::artifacts::process3d::engine::processed_volume(&app.projection().expect("projection")).expect("stock volume");
        let result = dispatch(&mut app, Process3dCommand::WorldFaceDragEnd(world_face_drag_end::WorldFaceDragEnd { normal: [0.0, 0.0, 1.0], start_point: [0.5, 0.5, 1.0], distance: 0.5, face_extent: Some([0.2, 0.2]) }));
        assert!(!result.operations.is_empty());
        let document = app.projection().expect("projection");
        assert_eq!(document.steps.len(), 1);
        assert!(matches!(document.steps[0].measure, crate::artifacts::process3d::ProcessMeasure::Attach { .. }));
        let new_volume = crate::artifacts::process3d::engine::processed_volume(&document).expect("volume after attach");
        assert!(new_volume > stock_volume, "face-drag attach should increase volume above stock ({new_volume} vs {stock_volume})");
    }

    #[test]
    fn world_face_drag_end_ignored_while_a_placement_utility_is_active() {
        let mut app = app();
        set_utility(&mut app, "cut");
        let result = dispatch(&mut app, Process3dCommand::WorldFaceDragEnd(world_face_drag_end::WorldFaceDragEnd { normal: [0.0, 0.0, 1.0], start_point: [0.5, 0.5, 1.0], distance: -0.5, face_extent: None }));
        assert!(result.operations.is_empty(), "worldFaceDragEnd should be a no-operation while a placement utility is active, not the select utility");
    }

    #[test]
    fn toggle_sun_round_trips_through_config_and_defaults_off() {
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

    #[test]
    fn window_measures_surface_the_sun_group() {
        let mut app = app();
        let measures = main_window_measures(&mut app);
        assert_eq!(measures.len(), 1);
        assert!(matches!(&measures[0], WindowMeasure::Group { id, .. } if id == "process3d-measure-sun"));
    }

    #[test]
    fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        let mut app = app();
        assert!(render_body(&mut app, "process3d.play.nope").contains("Unknown body"));
    }

    /// 🧪️ The registry-enforced app must accept every declared manifest action id without a kind-
    /// discipline error — proves the `app_commands!` rows and the manifest's `.operation`/`.shell_action`/
    /// `.action_with` declarations stay in sync.
    #[test]
    fn registry_enforced_app_accepts_a_declared_operation_action() {
        let mut app = app_with_registry();
        let result = dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None }));
        assert!(!result.operations.is_empty());
    }

    //#region 🔖️MediaTests
    #[test]
    fn export_brep_out_returns_step_text_structured_payload() {
        let app = Process3dPlayApp;
        let document = crate::artifacts::process3d::engine::default_document();
        let history = HistoryView::empty();
        let doc = DocumentView { projection: &document, history: &history };
        let media = app.export_media("brep:out", &doc).expect("export brep:out");
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

    #[test]
    fn export_unknown_port_is_not_implemented() {
        let app = Process3dPlayApp;
        let document = crate::artifacts::process3d::engine::default_document();
        let history = HistoryView::empty();
        let doc = DocumentView { projection: &document, history: &history };
        assert!(matches!(app.export_media("nonsense:out", &doc), Err(MediaError::NotImplemented)));
    }

    #[test]
    fn import_geometry_in_rejects_unrecognized_schema() {
        let app = Process3dPlayApp;
        let document = crate::artifacts::process3d::engine::default_document();
        let history = HistoryView::empty();
        let doc = DocumentView { projection: &document, history: &history };
        let media = semio_framework_plugin::Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep }, payload: MediaPayload::Structured { schema: "unknown.schema".into(), json: "irrelevant".into() } };
        assert!(matches!(app.import_media("geometry:in", &media, &doc), Err(MediaError::Payload(port, _)) if port == "geometry:in"));
    }
    //#endregion 🔖️MediaTests
}
//#endregion 🧪️Tests
