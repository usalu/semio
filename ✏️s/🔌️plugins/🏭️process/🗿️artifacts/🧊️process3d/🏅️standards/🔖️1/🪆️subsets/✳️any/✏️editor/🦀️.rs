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
use semio_s_artifact_process_process3d_modes as edit;
use semio_s_artifact_process_process3d_modes::windows::workpiece;
use semio_s_artifact_process_process3d_panels::{catalogue, document as document_panel, inspection, workshop as workshop_panel};
use semio_s_artifact_process_process3d_presence::{Process3dPresence, Process3dPresenceMutation};
use semio_s_artifact_process_process3d_kit::{editor::process3d::terminology::process3d_labels, internal_action, process3d_action_fault, process3d_interaction_definition, installable_contributions, reset_process3d_document_effect, PROCESS3D_CONFIG_CONTRIBUTIONS_BYTES, PROCESS3D_EXAMPLE_CONCRETE_FOREST, PROCESS3D_EXAMPLE_PLATE, PROCESS3D_EXAMPLE_TIMBER, PROCESS3D_INTERACTION_DOMAIN};
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
use semio_framework_2d_compute::EngineHandles;

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

#[path = "🧵️retained-commands/🦀️.rs"]
mod retained_commands;
pub(crate) use retained_commands::*;

#[path = "📬️store-preparation/🦀️.rs"]
mod store_preparation;
use store_preparation::*;

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

#[path = "🪟️editor-impl/🦀️.rs"]
mod editor_impl;
use editor_impl::*;

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
