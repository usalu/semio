//! ✏️ BIM model editor: the `ArtifactEditor` implementation, the command table and the manifest stitch. The editor is a routing table: every command body lives in a `🎮️commands/*`
//! node, every window render in `🎭️modes/✏️edit/🪟️windows/*`, every panel in `📌️panels/*`, derived values in `model_graph::registry`, what an entity kind is in `🧩️entities`. A command only
//! emits mutations (or window config, presence and effects); nothing here applies a diff.

use crate::editor::bim::commands::{analyse_model, arm_utility, export_sheets, canvas_commit_draft, canvas_double_click, canvas_escape, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, create_entity, create_view, delete_selection, edit_schedule, apply_template, edit_classification, edit_template, search_classification, engagement_input, export_schedule_csv, export_model, engagement_submit, flip_walls, move_storey, place_elements, cursor_keys, remove_classification, remove_property, rename_entity, select_findings, set_camera, set_classification, set_field, set_property, set_view, split_wall, world_pointer_down, world_pointer_move};
use crate::editor::bim::config::app_schema_descriptor;
use crate::editor::bim::entities::kind_holding;
use crate::editor::bim::modes::edit;
use crate::editor::bim::modes::edit::windows::{plan, schedule, section, sheet, world};
use crate::editor::bim::panels::{classification as classification_panel, diagnostics as diagnostics_panel, library as library_panel, outliner as outliner_panel, properties as properties_panel};
use crate::editor::bim::presence::{BimPresence, BimPresenceMutation};
use crate::editor::bim::terminology::{bim_labels, BimLabels};
use crate::standards::v1::subsets::any::schema::mutations::ModelMutation;
use crate::{ModelSnapshot, BIM_MODEL_DIALECT, BIM_MODEL_DOCUMENT_SCHEMA};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_2d::compute::EngineHandles;
use semio_framework_artifact_reference::Dialect;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandInputs, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::DslValue;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::ViewModel;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use std::collections::HashMap;

//#region 🔖️Constants
pub const BIM_EDITOR_APP_ID: &str = "s.bim.model@1/*#editor";
const BIM_RETAINED_PAYLOAD_SCHEMA: &str = "bim.model.tool-command.v1";
const BIM_BOUNDED_RAW_BYTES: usize = 65_536;
const BIM_BOUNDED_WORK_ITEMS: usize = 4_096;
//#endregion 🔖️Constants

//#region 🔖️Dispatch
/// 🕹️ Per-dispatch state that is neither document nor app config: the framework selection of both domains, the addressed window with its own config, the armed utility and the
/// local presence. The generated `dispatch` has no way to thread an `InteractionView`, so the editor builds this once per command and hands it down. Handlers push the presence
/// they want shared into `presence_out`.
#[derive(Default)]
pub struct BimDispatchCtx {
    pub selected: Vec<String>,
    pub library_selected: Vec<String>,
    pub view: Option<ViewModel>,
    pub window_kind: String,
    pub utility: String,
    pub plan: plan::config::BimPlanWindowConfig,
    pub world: world::config::BimWorldWindowConfig,
    pub section: section::config::BimSectionWindowConfig,
    pub schedule: schedule::config::BimScheduleWindowConfig,
    pub sheet: sheet::config::BimSheetWindowConfig,
    pub presence: BimPresence,
    pub presence_out: Vec<BimPresenceMutation>,
    pub gestures: Option<semio_framework_plugin::ArtifactInstanceOperationOwnerHandle>,
    pub window_transient: crate::editor::bim::transient::BimWindowTransient,
    pub transient_out: Option<crate::editor::bim::transient::BimWindowTransient>,
}

impl BimDispatchCtx {
    /// 🕹️ The context of a command addressed at `view` with the config snapshot of its window.
    pub fn new(selected: Vec<String>, library_selected: Vec<String>, view: Option<&ViewModel>, window_config: Option<&semio_framework_plugin::WindowConfigSnapshot>, presence: Option<&BimPresence>) -> Self {
        Self {
            selected,
            library_selected,
            window_kind: view.and_then(crate::editor::bim::chrome::addressed_kind).unwrap_or_default().to_string(),
            utility: view.map_or(crate::editor::bim::utilities::DEFAULT_UTILITY, crate::editor::bim::utilities::active).to_string(),
            view: view.cloned(),
            plan: plan::config::from_snapshot(window_config),
            world: world::config::from_snapshot(window_config),
            section: section::config::from_snapshot(window_config),
            schedule: schedule::config::from_snapshot(window_config),
            sheet: sheet::config::from_snapshot(window_config),
            presence: presence.cloned().unwrap_or_default(),
            presence_out: Vec::new(),
            gestures: None,
            window_transient: Default::default(),
            transient_out: None,
        }
    }

    /// 🗣️ The labels of the addressed view; `None` when the command carries no view.
    pub fn labels(&self) -> Option<&'static BimLabels> {
        self.view.as_ref().map(bim_labels)
    }
}
//#endregion 🔖️Dispatch

//#region 🔖️CommandTable
/// 🧩️ THE command table: `manifest id as wire keyword => payload, [publication lanes]; kind, label, description`, the last two being fields of `BimLabels` (en and de live in the one `app_labels!` block). The command enum, the bounded tool roster, the
/// publication contracts, the bounded-first-step proofs and the manifest actions are all generated from these rows, so a new command is one row here, one payload node and one bridge arm.
macro_rules! bim_command_table {
    ($callback:ident) => {
        $callback! {
            "createEntity" as "create-entity" => create_entity::CreateEntity, [Artifact]; Mutation, cmd_create_entity, cmd_create_entity_describe;
            "createView" as "create-view" => create_view::CreateView, [Artifact, WindowConfig]; Mutation, cmd_create_view, cmd_create_view_describe;
            "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection, [Artifact]; Mutation, cmd_delete_selection, cmd_delete_selection_describe;
            "renameEntity" as "rename-entity" => rename_entity::RenameEntity, [Artifact]; Mutation, cmd_rename_entity, cmd_rename_entity_describe;
            "setField" as "set-field" => set_field::SetField, [Artifact]; Mutation, cmd_set_field, cmd_set_field_describe;
            "setView" as "set-view" => set_view::SetView, [WindowConfig, Presence]; View, cmd_set_view, cmd_set_view_describe;
            "editSchedule" as "edit-schedule" => edit_schedule::EditSchedule, [Artifact]; Mutation, cmd_edit_schedule, cmd_edit_schedule_describe;
            "exportScheduleCsv" as "export-schedule-csv" => export_schedule_csv::ExportScheduleCsv, [HostOnly]; View, cmd_export_schedule_csv, cmd_export_schedule_csv_describe;
            "analyseModel" as "analyse-model" => analyse_model::AnalyseModel, [HostOnly]; View, cmd_analyse_model, cmd_analyse_model_describe;
            "exportModel" as "export-model" => export_model::ExportModel, [HostOnly]; View, cmd_export_model, cmd_export_model_describe;
            "exportSheets" as "export-sheets" => export_sheets::ExportSheets, [HostOnly]; View, cmd_export_sheets, cmd_export_sheets_describe;
            "armViewport" as "arm-viewport" => arm_utility::ArmViewport, [HostOnly]; View, cmd_arm_viewport, cmd_arm_viewport_describe;
            "setCamera" as "camera" => set_camera::SetCamera, [WindowConfig, Presence]; View, cmd_set_camera, cmd_set_camera_describe;
            "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown, [Artifact, WindowTransient]; Mutation, cmd_canvas_pointer_down, cmd_canvas_pointer_down_describe;
            "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove, [Artifact, WindowTransient]; Mutation, cmd_canvas_pointer_move, cmd_canvas_pointer_move_describe;
            "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp, [Artifact, WindowTransient]; Mutation, cmd_canvas_pointer_up, cmd_canvas_pointer_up_describe;
            "canvasDoubleClick" as "canvas-double-click" => canvas_double_click::CanvasDoubleClick, [Artifact, WindowTransient]; Mutation, cmd_canvas_double_click, cmd_canvas_double_click_describe;
            "canvasCommitDraft" as "canvas-commit-draft" => canvas_commit_draft::CanvasCommitDraft, [Artifact, WindowTransient]; Mutation, cmd_canvas_commit_draft, cmd_canvas_commit_draft_describe;
            "canvasEscape" as "canvas-escape" => canvas_escape::CanvasEscape, [Artifact, WindowTransient, Presence]; Mutation, cmd_canvas_escape, cmd_canvas_escape_describe;
            "worldPointerDown" as "world-pointer-down" => world_pointer_down::WorldPointerDown, [Artifact, WindowTransient]; Mutation, cmd_world_pointer_down, cmd_world_pointer_down_describe;
            "worldPointerMove" as "world-pointer-move" => world_pointer_move::WorldPointerMove, [Artifact, WindowTransient]; Mutation, cmd_world_pointer_move, cmd_world_pointer_move_describe;
            "armSelect" as "arm-select" => arm_utility::ArmSelect, [HostOnly]; View, cmd_arm_select, cmd_arm_select_describe;
            "armWall" as "arm-wall" => arm_utility::ArmWall, [HostOnly]; View, cmd_arm_wall, cmd_arm_wall_describe;
            "armWallArc" as "arm-wall-arc" => arm_utility::ArmWallArc, [HostOnly]; View, cmd_arm_wall_arc, cmd_arm_wall_arc_describe;
            "armCurtainWall" as "arm-curtain-wall" => arm_utility::ArmCurtainWall, [HostOnly]; View, cmd_arm_curtain_wall, cmd_arm_curtain_wall_describe;
            "armColumn" as "arm-column" => arm_utility::ArmColumn, [HostOnly]; View, cmd_arm_column, cmd_arm_column_describe;
            "armBeam" as "arm-beam" => arm_utility::ArmBeam, [HostOnly]; View, cmd_arm_beam, cmd_arm_beam_describe;
            "armSlab" as "arm-slab" => arm_utility::ArmSlab, [HostOnly]; View, cmd_arm_slab, cmd_arm_slab_describe;
            "armRoof" as "arm-roof" => arm_utility::ArmRoof, [HostOnly]; View, cmd_arm_roof, cmd_arm_roof_describe;
            "armWindow" as "arm-window" => arm_utility::ArmWindow, [HostOnly]; View, cmd_arm_window, cmd_arm_window_describe;
            "armDoor" as "arm-door" => arm_utility::ArmDoor, [HostOnly]; View, cmd_arm_door, cmd_arm_door_describe;
            "armOpening" as "arm-opening" => arm_utility::ArmOpening, [HostOnly]; View, cmd_arm_opening, cmd_arm_opening_describe;
            "armStair" as "arm-stair" => arm_utility::ArmStair, [HostOnly]; View, cmd_arm_stair, cmd_arm_stair_describe;
            "armRailing" as "arm-railing" => arm_utility::ArmRailing, [HostOnly]; View, cmd_arm_railing, cmd_arm_railing_describe;
            "armRamp" as "arm-ramp" => arm_utility::ArmRamp, [HostOnly]; View, cmd_arm_ramp, cmd_arm_ramp_describe;
            "armSpace" as "arm-space" => arm_utility::ArmSpace, [HostOnly]; View, cmd_arm_space, cmd_arm_space_describe;
            "armGrid" as "arm-grid" => arm_utility::ArmGrid, [HostOnly]; View, cmd_arm_grid, cmd_arm_grid_describe;
            "armMeasure" as "arm-measure" => arm_utility::ArmMeasure, [HostOnly]; View, cmd_arm_measure, cmd_arm_measure_describe;
            "armMove" as "arm-move" => arm_utility::ArmMove, [HostOnly]; View, cmd_arm_move, cmd_arm_move_describe;
            "armRotate" as "arm-rotate" => arm_utility::ArmRotate, [HostOnly]; View, cmd_arm_rotate, cmd_arm_rotate_describe;
            "armSlabWalls" as "arm-slab-walls" => arm_utility::ArmSlabWalls, [HostOnly]; View, cmd_arm_slab_walls, cmd_arm_slab_walls_describe;
            "armCeiling" as "arm-ceiling" => arm_utility::ArmCeiling, [HostOnly]; View, cmd_arm_ceiling, cmd_arm_ceiling_describe;
            "armCeilingSpace" as "arm-ceiling-space" => arm_utility::ArmCeilingSpace, [HostOnly]; View, cmd_arm_ceiling_space, cmd_arm_ceiling_space_describe;
            "armSplitWall" as "arm-split-wall" => arm_utility::ArmSplitWall, [HostOnly]; View, cmd_arm_split_wall, cmd_arm_split_wall_describe;
            "armCopy" as "arm-copy" => arm_utility::ArmCopy, [HostOnly]; View, cmd_arm_copy, cmd_arm_copy_describe;
            "armMirror" as "arm-mirror" => arm_utility::ArmMirror, [HostOnly]; View, cmd_arm_mirror, cmd_arm_mirror_describe;
            "armArray" as "arm-array" => arm_utility::ArmArray, [HostOnly]; View, cmd_arm_array, cmd_arm_array_describe;
            "armArrayRadial" as "arm-array-radial" => arm_utility::ArmArrayRadial, [HostOnly]; View, cmd_arm_array_radial, cmd_arm_array_radial_describe;
            "armOffset" as "arm-offset" => arm_utility::ArmOffset, [HostOnly]; View, cmd_arm_offset, cmd_arm_offset_describe;
            "armTrim" as "arm-trim" => arm_utility::ArmTrim, [HostOnly]; View, cmd_arm_trim, cmd_arm_trim_describe;
            "armExtend" as "arm-extend" => arm_utility::ArmExtend, [HostOnly]; View, cmd_arm_extend, cmd_arm_extend_describe;
            "armAlign" as "arm-align" => arm_utility::ArmAlign, [HostOnly]; View, cmd_arm_align, cmd_arm_align_describe;
            "armSplit" as "arm-split" => arm_utility::ArmSplit, [HostOnly]; View, cmd_arm_split, cmd_arm_split_describe;
            "armDimension" as "arm-dimension" => arm_utility::ArmDimension, [HostOnly]; View, cmd_arm_dimension, cmd_arm_dimension_describe;
            "armTag" as "arm-tag" => arm_utility::ArmTag, [HostOnly]; View, cmd_arm_tag, cmd_arm_tag_describe;
            "armTextNote" as "arm-text-note" => arm_utility::ArmTextNote, [HostOnly]; View, cmd_arm_text_note, cmd_arm_text_note_describe;
            "armLeader" as "arm-leader" => arm_utility::ArmLeader, [HostOnly]; View, cmd_arm_leader, cmd_arm_leader_describe;
            "flipWalls" as "flip-walls" => flip_walls::FlipWalls, [Artifact]; Mutation, cmd_flip_walls, cmd_flip_walls_describe;
            "storeyUp" as "storey-up" => move_storey::StoreyUp, [Artifact]; Mutation, cmd_storey_up, cmd_storey_up_describe;
            "storeyDown" as "storey-down" => move_storey::StoreyDown, [Artifact]; Mutation, cmd_storey_down, cmd_storey_down_describe;
            "splitWall" as "split-wall-at" => split_wall::SplitWallAt, [Artifact]; Mutation, cmd_split_wall, cmd_split_wall_describe;
            "setProperty" as "set-property" => set_property::SetProperty, [Artifact]; Mutation, cmd_set_property, cmd_set_property_describe;
            "removeProperty" as "remove-property" => remove_property::RemoveProperty, [Artifact]; Mutation, cmd_remove_property, cmd_remove_property_describe;
            "setClassification" as "set-classification" => set_classification::SetClassification, [Artifact]; Mutation, cmd_set_classification, cmd_set_classification_describe;
            "removeClassification" as "remove-classification" => remove_classification::RemoveClassification, [Artifact]; Mutation, cmd_remove_classification, cmd_remove_classification_describe;
            "applyTemplate" as "apply-template" => apply_template::ApplyTemplate, [Artifact]; Mutation, cmd_apply_template, cmd_apply_template_describe;
            "editTemplate" as "edit-template" => edit_template::EditTemplate, [Artifact]; Mutation, cmd_edit_template, cmd_edit_template_describe;
            "editClassification" as "edit-classification" => edit_classification::EditClassification, [Artifact]; Mutation, cmd_edit_classification, cmd_edit_classification_describe;
            "searchClassification" as "search-classification" => search_classification::SearchClassification, [HostOnly]; View, cmd_search_classification, cmd_search_classification_describe;
            "placeElements" as "place-at" => place_elements::PlaceAt, [Artifact]; Mutation, cmd_place_elements, cmd_place_elements_describe;
            "selectFindings" as "select-findings" => select_findings::SelectFindings, [HostOnly]; View, cmd_select_findings, cmd_select_findings_describe;
            "cursorLeft" as "cursor-left" => cursor_keys::CursorLeft, [Artifact, WindowTransient]; Mutation, cmd_cursor_left, cmd_cursor_left_describe;
            "cursorRight" as "cursor-right" => cursor_keys::CursorRight, [Artifact, WindowTransient]; Mutation, cmd_cursor_right, cmd_cursor_right_describe;
            "cursorUp" as "cursor-up" => cursor_keys::CursorUp, [Artifact, WindowTransient]; Mutation, cmd_cursor_up, cmd_cursor_up_describe;
            "cursorDown" as "cursor-down" => cursor_keys::CursorDown, [Artifact, WindowTransient]; Mutation, cmd_cursor_down, cmd_cursor_down_describe;
            "cursorLeftFar" as "cursor-left-far" => cursor_keys::CursorLeftFar, [Artifact, WindowTransient]; Mutation, cmd_cursor_left_far, cmd_cursor_left_far_describe;
            "cursorRightFar" as "cursor-right-far" => cursor_keys::CursorRightFar, [Artifact, WindowTransient]; Mutation, cmd_cursor_right_far, cmd_cursor_right_far_describe;
            "cursorUpFar" as "cursor-up-far" => cursor_keys::CursorUpFar, [Artifact, WindowTransient]; Mutation, cmd_cursor_up_far, cmd_cursor_up_far_describe;
            "cursorDownFar" as "cursor-down-far" => cursor_keys::CursorDownFar, [Artifact, WindowTransient]; Mutation, cmd_cursor_down_far, cmd_cursor_down_far_describe;
            "cursorPlace" as "cursor-place" => cursor_keys::CursorPlace, [Artifact, WindowTransient]; Mutation, cmd_cursor_place, cmd_cursor_place_describe;
            "engagementInput" as "engagement-input" => engagement_input::EngagementInput, [WindowTransient, Presence]; View, cmd_engagement_input, cmd_engagement_input_describe;
            "engagementSubmit" as "engagement-submit" => engagement_submit::EngagementSubmit, [Artifact, WindowTransient, Presence]; Mutation, cmd_engagement_submit, cmd_engagement_submit_describe;
        }
    };
}

macro_rules! command_enum {
    ($($id:literal as $key:literal => $module:ident :: $payload:ident, [$($lane:ident),+]; $kind:ident, $label:ident, $describe:ident;)+) => {
        semio_framework_plugin::app_commands! {
            /// 🎯️ `BimModelApp::Command`: the sole dispatch surface of the editor, generated from the command table.
            pub enum BimCommand for ModelSnapshot, ModelMutation, NoConfig, NoConfigMutation, ctx = BimDispatchCtx {
                $($id as $key => $module::$payload),+
            }
        }
    };
}

macro_rules! tool_roster {
    ($($id:literal as $key:literal => $module:ident :: $payload:ident, [$($lane:ident),+]; $kind:ident, $label:ident, $describe:ident;)+) => {
        /// 🧵️ Every bounded first-step tool of the editor, in command-table order.
        pub const BIM_TOOL_IDS: &[&str] = &[$($id),+];
        const BIM_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[$(ArtifactToolPublicationContract { tool_id: $id, lanes: &[$(ArtifactToolPublicationLane::$lane),+] }),+];
    };
}

macro_rules! tool_proofs {
    ($($id:literal as $key:literal => $module:ident :: $payload:ident, [$($lane:ident),+]; $kind:ident, $label:ident, $describe:ident;)+) => {
        semio_framework_plugin::bounded_first_step_tool_proofs! {
            owner: EditorApp<BimModelApp>,
            owner_file: "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
            controller: "s.bim.model@1/*#editor",
            artifact_schema: "s.bim.model@1",
            factory: "BimCommandJobFactory",
            factory_type: BimCommandJobFactory,
            contract: ToolExecutionContract::bounded_first_step(BIM_BOUNDED_RAW_BYTES, BIM_BOUNDED_WORK_ITEMS, 1, 262_144, 7_500),
            tools: [$($id),+]
        }
    };
}

macro_rules! manifest_actions {
    ($($id:literal as $key:literal => $module:ident :: $payload:ident, [$($lane:ident),+]; $kind:ident, $label:ident, $describe:ident;)+) => {
        /// 📇️ The manifest action of every command row.
        fn command_actions() -> Vec<ActionDefinition> {
            vec![$(ActionDefinition::new($id, LocalizedLabel::native(BimLabels::NATIVE_EN.$label.as_str(), BimLabels::NATIVE_DE.$label.as_str()), ActionKind::$kind, "box").describe(LocalizedLabel::native(BimLabels::NATIVE_EN.$describe.as_str(), BimLabels::NATIVE_DE.$describe.as_str()))),+]
        }
    };
}

bim_command_table!(command_enum);
bim_command_table!(tool_roster);
bim_command_table!(manifest_actions);
//#endregion 🔖️CommandTable

//#region 🔖️ActionBridge
/// 🌉️ Host-action bridge into the closed `BimCommand` enum: the shells speak `{action, args}` with camelCase keys, every payload derives `FromValue` over snake_case, so this boundary
/// folds the keys, applies the per-verb aliases and decodes.
mod args_bridge {
    use super::*;
    use semio_framework_plugin::{FaultCode, FaultOrigin};

    fn snake(key: &str) -> String {
        key.chars().flat_map(|ch| if ch.is_ascii_uppercase() { vec!['_', ch.to_ascii_lowercase()] } else { vec![ch] }).collect()
    }

    fn put(entries: &mut Vec<(String, DslValue)>, key: &str, value: DslValue) {
        entries.retain(|(existing, _)| existing != key);
        entries.push((key.to_string(), value));
    }

    fn integral(value: DslValue) -> DslValue {
        match value {
            DslValue::Number(semio_framework_value::Number::Float(float)) if float.is_finite() && float.fract() == 0.0 && float.abs() < 9.007_199_254_740_992e15 => {
                if float >= 0.0 { DslValue::Number(semio_framework_value::Number::UInt(float as u64)) } else { DslValue::Number(semio_framework_value::Number::Int(float as i64)) }
            }
            DslValue::Array(items) => DslValue::Array(items.into_iter().map(integral).collect()),
            DslValue::Object(entries) => DslValue::Object(entries.into_iter().map(|(key, value)| (key, integral(value))).collect()),
            other => other,
        }
    }

    fn stringify(value: DslValue) -> DslValue {
        match value {
            DslValue::String(_) => value,
            DslValue::Null => DslValue::String(String::new()),
            other => DslValue::String(semio_framework_pack_json::to_json_string(&other)),
        }
    }

    fn fold(args: Option<&DslValue>, aliases: &[(&str, &str)], defaults: &[(&str, DslValue)]) -> DslValue {
        let mut entries: Vec<(String, DslValue)> = Vec::new();
        if let Some(DslValue::Object(object)) = args {
            for (key, value) in object {
                put(&mut entries, &snake(key), integral(value.clone()));
            }
        }
        for (from, into) in aliases {
            if entries.iter().any(|(key, _)| key == into) {
                continue;
            }
            if let Some((_, value)) = entries.iter().find(|(key, _)| key == from).cloned() {
                put(&mut entries, into, value);
            }
        }
        for (key, value) in defaults {
            if !entries.iter().any(|(existing, _)| existing == key) {
                entries.push(((*key).to_string(), value.clone()));
            }
        }
        DslValue::Object(entries)
    }

    fn value_as_text(mut folded: DslValue) -> DslValue {
        if let DslValue::Object(entries) = &mut folded {
            if let Some(slot) = entries.iter_mut().find(|(key, _)| key == "value") {
                slot.1 = stringify(slot.1.clone());
            }
        }
        folded
    }

    fn text_field(mut folded: DslValue, key: &str) -> DslValue {
        if let DslValue::Object(entries) = &mut folded {
            if let Some(slot) = entries.iter_mut().find(|(name, _)| name == key) {
                slot.1 = stringify(slot.1.clone());
            }
        }
        folded
    }

    fn ids_as_list(mut folded: DslValue) -> DslValue {
        if let DslValue::Object(entries) = &mut folded {
            if let Some(slot) = entries.iter_mut().find(|(key, _)| key == "ids") {
                if let DslValue::String(single) = &slot.1 {
                    slot.1 = DslValue::Array(vec![DslValue::String(single.clone())]);
                }
            }
        }
        folded
    }

    /// 🎥️ The host nests the pose under `camera` (or sends it flat); a pose with `position` is the world's orbit, one with `x` the canvas'.
    fn camera_pose(args: Option<&DslValue>) -> DslValue {
        let folded = fold(args, &[], &[]);
        let DslValue::Object(entries) = &folded else { return folded };
        let pose = match entries.iter().find(|(key, _)| key == "camera") {
            Some((_, DslValue::Object(inner))) => inner.clone(),
            _ => entries.clone(),
        };
        let slot = if pose.iter().any(|(key, _)| key == "position") { "camera3d" } else { "camera2d" };
        let kept: Vec<(String, DslValue)> = pose.into_iter().filter(|(key, _)| matches!(key.as_str(), "x" | "y" | "zoom" | "position" | "target" | "up")).collect();
        DslValue::Object(vec![(slot.to_string(), DslValue::Object(kept))])
    }

    fn only(folded: DslValue, keys: &[&str]) -> DslValue {
        match folded {
            DslValue::Object(entries) => DslValue::Object(entries.into_iter().filter(|(key, _)| keys.contains(&key.as_str())).collect()),
            other => other,
        }
    }

    fn decode<T: semio_framework_value::FromValue>(action: &str, value: DslValue) -> Result<T, Fault> {
        T::from_value(value).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("bim action '{action}' arguments do not decode: {error}")))
    }

    pub fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<BimCommand, Fault> {
        let text = |value: &str| DslValue::String(value.into());
        Ok(match action {
            "createEntity" => BimCommand::CreateEntity(decode(action, fold(args, &[("type", "kind"), ("value", "kind"), ("parent_id", "parent")], &[("parent", text("")), ("name", text(""))]))?),
            "createView" => BimCommand::CreateView(decode(action, fold(args, &[("type", "kind"), ("value", "kind"), ("parent_id", "parent")], &[("parent", text("")), ("name", text(""))]))?),
            "deleteSelection" => BimCommand::DeleteSelection(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new()))])))?),
            "renameEntity" => BimCommand::RenameEntity(decode(action, fold(args, &[("value", "name")], &[]))?),
            "setField" => BimCommand::SetField(decode(action, ids_as_list(value_as_text(fold(args, &[("id", "ids"), (outliner_panel::DRAG_ELEMENT_MIME, "ids")], &[("ids", DslValue::Array(Vec::new()))]))))?),
            "setView" => BimCommand::SetView(decode(action, value_as_text(fold(args, &[], &[("value", text(""))])))?),
            "editSchedule" => BimCommand::EditSchedule(decode(action, fold(args, &[], &[("key", text("")), ("value", text(""))]))?),
            "exportScheduleCsv" => BimCommand::ExportScheduleCsv(decode(action, only(fold(args, &[], &[("id", text(""))]), &["id", "pressed"]))?),
            "analyseModel" => BimCommand::AnalyseModel(decode(action, only(fold(args, &[], &[]), &["pressed"]))?),
            "exportModel" => BimCommand::ExportModel(decode(action, only(fold(args, &[], &[("format", text("ifc"))]), &["format", "pressed"]))?),
            "setCamera" => BimCommand::SetCamera(decode(action, camera_pose(args))?),
            "canvasPointerDown" => BimCommand::CanvasPointerDown(decode(action, fold(args, &[], &[]))?),
            "canvasPointerMove" => BimCommand::CanvasPointerMove(decode(action, fold(args, &[], &[]))?),
            "canvasPointerUp" => BimCommand::CanvasPointerUp(decode(action, fold(args, &[], &[]))?),
            "canvasDoubleClick" => BimCommand::CanvasDoubleClick(decode(action, fold(args, &[], &[]))?),
            "canvasCommitDraft" => BimCommand::CanvasCommitDraft(decode(action, fold(args, &[], &[]))?),
            "canvasEscape" => BimCommand::CanvasEscape(decode(action, fold(args, &[], &[]))?),
            "worldPointerDown" => BimCommand::WorldPointerDown(decode(action, fold(args, &[], &[]))?),
            "worldPointerMove" => BimCommand::WorldPointerMove(decode(action, fold(args, &[], &[]))?),
            "armSelect" => BimCommand::ArmSelect(decode(action, fold(args, &[], &[]))?),
            "armWall" => BimCommand::ArmWall(decode(action, fold(args, &[], &[]))?),
            "armWallArc" => BimCommand::ArmWallArc(decode(action, fold(args, &[], &[]))?),
            "armCurtainWall" => BimCommand::ArmCurtainWall(decode(action, fold(args, &[], &[]))?),
            "armColumn" => BimCommand::ArmColumn(decode(action, fold(args, &[], &[]))?),
            "armBeam" => BimCommand::ArmBeam(decode(action, fold(args, &[], &[]))?),
            "armSlab" => BimCommand::ArmSlab(decode(action, fold(args, &[], &[]))?),
            "armRoof" => BimCommand::ArmRoof(decode(action, fold(args, &[], &[]))?),
            "armWindow" => BimCommand::ArmWindow(decode(action, fold(args, &[], &[]))?),
            "armDoor" => BimCommand::ArmDoor(decode(action, fold(args, &[], &[]))?),
            "armOpening" => BimCommand::ArmOpening(decode(action, fold(args, &[], &[]))?),
            "armStair" => BimCommand::ArmStair(decode(action, fold(args, &[], &[]))?),
            "armRailing" => BimCommand::ArmRailing(decode(action, fold(args, &[], &[]))?),
            "armRamp" => BimCommand::ArmRamp(decode(action, fold(args, &[], &[]))?),
            "armSpace" => BimCommand::ArmSpace(decode(action, fold(args, &[], &[]))?),
            "armGrid" => BimCommand::ArmGrid(decode(action, fold(args, &[], &[]))?),
            "armMeasure" => BimCommand::ArmMeasure(decode(action, fold(args, &[], &[]))?),
            "armMove" => BimCommand::ArmMove(decode(action, fold(args, &[], &[]))?),
            "armRotate" => BimCommand::ArmRotate(decode(action, fold(args, &[], &[]))?),
            "armSlabWalls" => BimCommand::ArmSlabWalls(decode(action, fold(args, &[], &[]))?),
            "armCeiling" => BimCommand::ArmCeiling(decode(action, fold(args, &[], &[]))?),
            "armCeilingSpace" => BimCommand::ArmCeilingSpace(decode(action, fold(args, &[], &[]))?),
            "armSplitWall" => BimCommand::ArmSplitWall(decode(action, fold(args, &[], &[]))?),
            "armCopy" => BimCommand::ArmCopy(decode(action, fold(args, &[], &[]))?),
            "armMirror" => BimCommand::ArmMirror(decode(action, fold(args, &[], &[]))?),
            "armArray" => BimCommand::ArmArray(decode(action, fold(args, &[], &[]))?),
            "armArrayRadial" => BimCommand::ArmArrayRadial(decode(action, fold(args, &[], &[]))?),
            "armOffset" => BimCommand::ArmOffset(decode(action, fold(args, &[], &[]))?),
            "armTrim" => BimCommand::ArmTrim(decode(action, fold(args, &[], &[]))?),
            "armExtend" => BimCommand::ArmExtend(decode(action, fold(args, &[], &[]))?),
            "armAlign" => BimCommand::ArmAlign(decode(action, fold(args, &[], &[]))?),
            "armSplit" => BimCommand::ArmSplit(decode(action, fold(args, &[], &[]))?),
            "armDimension" => BimCommand::ArmDimension(decode(action, fold(args, &[], &[]))?),
            "armTag" => BimCommand::ArmTag(decode(action, fold(args, &[], &[]))?),
            "armTextNote" => BimCommand::ArmTextNote(decode(action, fold(args, &[], &[]))?),
            "armLeader" => BimCommand::ArmLeader(decode(action, fold(args, &[], &[]))?),
            "flipWalls" => BimCommand::FlipWalls(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new()))])))?),
            "storeyUp" => BimCommand::StoreyUp(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new()))])))?),
            "storeyDown" => BimCommand::StoreyDown(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new()))])))?),
            "splitWall" => BimCommand::SplitWallAt(decode(action, text_field(ids_as_list(fold(args, &[("id", "ids"), ("value", "at")], &[("ids", DslValue::Array(Vec::new())), ("at", text(""))])), "at"))?),
            "setProperty" => BimCommand::SetProperty(decode(action, value_as_text(ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new())), ("pset", text("")), ("property", text("")), ("value", text(""))]))))?),
            "removeProperty" => BimCommand::RemoveProperty(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new())), ("pset", text("")), ("property", text(""))])))?),
            "setClassification" => BimCommand::SetClassification(decode(action, ids_as_list(fold(args, &[("id", "ids"), ("value", "code")], &[("ids", DslValue::Array(Vec::new())), ("system", text("")), ("code", text(""))])))?),
            "removeClassification" => BimCommand::RemoveClassification(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new())), ("system", text(""))])))?),
            "applyTemplate" => BimCommand::ApplyTemplate(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new())), ("template", text(""))])))?),
            "editTemplate" => BimCommand::EditTemplate(decode(action, fold(args, &[], &[("index", text("")), ("field", text("")), ("value", text(""))]))?),
            "editClassification" => BimCommand::EditClassification(decode(action, fold(args, &[("value", "title")], &[("code", text("")), ("title", text("")), ("parent", text(""))]))?),
            "searchClassification" => BimCommand::SearchClassification(decode(action, fold(args, &[("value", "query")], &[("query", text(""))]))?),
            "placeElements" => BimCommand::PlaceAt(decode(action, text_field(ids_as_list(fold(args, &[("id", "ids"), ("value", "at")], &[("ids", DslValue::Array(Vec::new())), ("at", text(""))])), "at"))?),
            "selectFindings" => BimCommand::SelectFindings(decode(action, ids_as_list(fold(args, &[("id", "ids")], &[("ids", DslValue::Array(Vec::new()))])))?),
            "cursorLeft" => BimCommand::CursorLeft(decode(action, fold(args, &[], &[]))?),
            "cursorRight" => BimCommand::CursorRight(decode(action, fold(args, &[], &[]))?),
            "cursorUp" => BimCommand::CursorUp(decode(action, fold(args, &[], &[]))?),
            "cursorDown" => BimCommand::CursorDown(decode(action, fold(args, &[], &[]))?),
            "cursorLeftFar" => BimCommand::CursorLeftFar(decode(action, fold(args, &[], &[]))?),
            "cursorRightFar" => BimCommand::CursorRightFar(decode(action, fold(args, &[], &[]))?),
            "cursorUpFar" => BimCommand::CursorUpFar(decode(action, fold(args, &[], &[]))?),
            "cursorDownFar" => BimCommand::CursorDownFar(decode(action, fold(args, &[], &[]))?),
            "cursorPlace" => BimCommand::CursorPlace(decode(action, fold(args, &[], &[]))?),
            "engagementInput" => BimCommand::EngagementInput(decode(action, value_as_text(fold(args, &[], &[("value", text(""))])))?),
            "engagementSubmit" => BimCommand::EngagementSubmit(decode(action, value_as_text(fold(args, &[], &[("value", text(""))])))?),
            _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the bim editor has no command for action '{action}'"))),
        })
    }
}
//#endregion 🔖️ActionBridge

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface: the implicit document in/out ports only.
pub fn bim_io() -> AppIo {
    AppIo {
        artifact_schema: BIM_MODEL_DOCUMENT_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        ports: vec![],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "3d.bim-model".into(), name: "BIM Model".into(), dimension: "3d".into(), component_kind: "bim".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️RetainedCommands
fn bim_bounded_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(BIM_BOUNDED_RAW_BYTES, BIM_BOUNDED_WORK_ITEMS, 1, 262_144, 7_500)
}

fn bim_bounded_extent(command: &BimCommand) -> Option<usize> {
    BIM_TOOL_IDS.contains(&command.command_id()).then_some(1)
}

/// 🧵️ The single step of a bounded command: dispatches against the retained roots with the addressed window's config and the local presence, and publishes the presence the handler asked for.
struct BimCommandWork {
    tool_id: &'static str,
    owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
    completed: bool,
}

impl BimCommandWork {
    fn new(tool_id: &'static str, owner: semio_framework_plugin::ArtifactInstanceOperationOwnerHandle) -> Self {
        Self { tool_id, owner, completed: false }
    }
}

impl ArtifactCommandWork<EditorApp<BimModelApp>> for BimCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn extent(&self, command: &BimCommand, _snapshot: &ModelSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<BimModelApp>>>) -> Option<usize> {
        (!self.completed && command.command_id() == self.tool_id).then(|| bim_bounded_extent(command)).flatten()
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<BimModelApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<BimModelApp>>, Fault> {
        if self.completed || input.command.command_id() != self.tool_id {
            return Err(Fault::from("bim-command-work-terminal"));
        }
        let doc = ArtifactView::with_operation(input.snapshot, input.history, input.operation.clone());
        let window_config = input.context.and_then(|context| context.window_config.as_ref());
        let cfg = ConfigView { snapshot: input.config, window: window_config };
        let selection = |domain: &str| input.interaction.selection.get(domain).map(|selection| selection.ids.clone()).unwrap_or_default();
        let view = input.context.and_then(|context| context.view_state.as_ref());
        let presence = input.context.and_then(|context| context.presence_view()).map(|presence| presence.local.clone());
        let mut ctx = BimDispatchCtx::new(selection(crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN), selection(crate::editor::bim::interaction::BIM_LIBRARY_DOMAIN), view, window_config, presence.as_ref());
        ctx.gestures = Some(self.owner.clone());
        ctx.window_transient = crate::editor::bim::transient::from_snapshot(input.context.and_then(|context| context.window_transient.as_ref()));
        let emit = input.command.dispatch(&doc, &cfg, &mut ctx)?;
        crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::record_mutations(doc.operation_optional().map(|operation| operation.app_instance_id), input.snapshot, &emit.artifact_mutations);
        self.completed = true;
        let window_transient = match (ctx.transient_out.take(), view) {
            (Some(transient), Some(view)) => vec![crate::editor::bim::transient::addressed(view, transient)?],
            _ => Vec::new(),
        };
        Ok(if ctx.presence_out.is_empty() && window_transient.is_empty() { ArtifactCommandWorkStep::Complete(emit) } else { ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { presence: ctx.presence_out, transient: Vec::new(), window_transient } } })
    }
}

struct BimCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl BimCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: BIM_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for BimCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<BimModelApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<BimModelApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        BIM_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        bim_bounded_contract()
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
        if input.declared_bytes() > BIM_BOUNDED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("bounded Bim command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for BimCommandJobFactory {
    type Owner = EditorApp<BimModelApp>;
    const TOOL_IDS: &'static [&'static str] = BIM_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = BIM_MODEL_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = BIM_PUBLICATION_CONTRACTS;
}
//#endregion 🔖️RetainedCommands

//#region 🔖️Faults
/// 📣️ Declares the en/de notices of every `bim.*` refusal code a command can raise: each code names the `BimLabels` field that holds its two texts.
macro_rules! fault_notices {
    ($($code:literal => $label:ident;)+) => {
        /// 📣️ The notice of every refusal code, English and German from the one `app_labels!` block.
        pub fn bim_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
            static NOTICES: std::sync::LazyLock<Vec<(&'static str, LocalizedLabel)>> =
                std::sync::LazyLock::new(|| vec![$(($code, LocalizedLabel::native(BimLabels::NATIVE_EN.$label.as_str(), BimLabels::NATIVE_DE.$label.as_str()))),+]);
            NOTICES.as_slice()
        }
    };
}

fault_notices! {
    "bim.create.kind-unknown" => fault_create_kind_unknown;
    "bim.create.unsupported" => fault_create_unsupported;
    "bim.create.site-missing" => fault_create_site_missing;
    "bim.create.building-missing" => fault_create_building_missing;
    "bim.create.storey-missing" => fault_create_storey_missing;
    "bim.create.wall-type-missing" => fault_create_wall_type_missing;
    "bim.delete.unsupported" => fault_delete_unsupported;
    "bim.rename.target-missing" => fault_rename_target_missing;
    "bim.rename.unsupported" => fault_rename_unsupported;
    "bim.rename.rejected" => fault_rename_rejected;
    "bim.set.target-missing" => fault_set_target_missing;
    "bim.set.field-unknown" => fault_set_field_unknown;
    "bim.set.read-only" => fault_set_read_only;
    "bim.set.value-invalid" => fault_set_value_invalid;
    "bim.view.window-required" => fault_view_window_required;
    "bim.view.window-unsupported" => fault_view_window_unsupported;
    "bim.view.field-unknown" => fault_view_field_unknown;
    "bim.view.value-invalid" => fault_view_value_invalid;
    "bim.view.storey-missing" => fault_view_storey_missing;
    "bim.view.view-missing" => fault_view_view_missing;
    "bim.view.kind-unknown" => fault_view_kind_unknown;
    "bim.view.building-missing" => fault_view_building_missing;
    "bim.camera.window-required" => fault_camera_window_required;
    "bim.camera.pose-mismatch" => fault_camera_pose_mismatch;
    "bim.camera.invalid" => fault_camera_invalid;
    "bim.tool.storey-missing" => fault_tool_storey_missing;
    "bim.tool.type-missing" => fault_tool_type_missing;
    "bim.tool.rejected" => fault_tool_rejected;
    "bim.tool.input-invalid" => fault_tool_input_invalid;
    "bim.gesture.retained-route" => fault_gesture_retained_route;
    "bim.split.target-missing" => fault_split_target_missing;
    "bim.split.fraction-invalid" => fault_split_fraction_invalid;
    "bim.flip.wall-missing" => fault_flip_wall_missing;
    "bim.analyse.inference" => fault_analyse_inference;
    "bim.analyse.cancelled" => fault_analyse_cancelled;
    "bim.export.format-unknown" => fault_export_format_unknown;
    "bim.export.inference" => fault_export_inference;
    "bim.export.failed" => fault_export_failed;
    "bim.export.cancelled" => fault_export_cancelled;
    "bim.storey.target-missing" => fault_storey_target_missing;
    "bim.storey.no-neighbour" => fault_storey_no_neighbour;
    "bim.property.target-missing" => fault_property_target_missing;
    "bim.property.name-invalid" => fault_property_name_invalid;
    "bim.property.value-invalid" => fault_property_value_invalid;
    "bim.property.missing" => fault_property_missing;
    "bim.place.target-missing" => fault_place_target_missing;
    "bim.place.point-invalid" => fault_place_point_invalid;
    "bim.place.unsupported" => fault_place_unsupported;
    "bim.diagnostic.target-missing" => fault_diagnostic_target_missing;
    "bim.classification.target-missing" => fault_classification_target_missing;
    "bim.classification.invalid" => fault_classification_invalid;
    "bim.classification.missing" => fault_classification_missing;
    "bim.classification.system-missing" => fault_classification_system_missing;
    "bim.classification.edit-invalid" => fault_classification_edit_invalid;
    "bim.classification.entry-has-children" => fault_classification_entry_has_children;
    "bim.classification.no-match" => fault_classification_no_match;
    "bim.template.target-missing" => fault_template_target_missing;
    "bim.template.not-applicable" => fault_template_not_applicable;
    "bim.template.edit-invalid" => fault_template_edit_invalid;
    "bim.template.value-invalid" => fault_template_value_invalid;
}
//#endregion 🔖️Faults

//#region 🔖️BimModelApp
#[derive(Default)]
pub struct BimModelApp;

/// 🧬️ The window-kind to render-body table: which body key a window kind or panel renders.
fn selection_ids(interaction: &InteractionView<'_>, domain: &str) -> Vec<String> {
    interaction.selection(domain).ids.clone()
}

fn hover_ids(interaction: &InteractionView<'_>, domain: &str) -> Vec<String> {
    interaction.hover(domain, "pointer").ids.clone()
}

fn instance_of(doc: &ArtifactView<'_, ModelSnapshot>) -> Option<u32> {
    doc.render_operation().map(|operation| operation.app_instance_id)
}

/// 🚨️ The findings of the model by severity, from the instance's inference (the diagnostic index).
fn problems_of(doc: &ArtifactView<'_, ModelSnapshot>) -> crate::standards::v1::subsets::any::schema::inferences::diagnostics::SeverityCounts {
    crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::with_inference(instance_of(doc), doc.snapshot, |inference| inference.diagnostic_index.total)
}

#[allow(clippy::too_many_arguments)]
fn render_body(body_key: &str, doc: &ArtifactView<'_, ModelSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel, elements: &[String], library: &[String], hover: &[String], preview: &crate::editor::bim::gestures::session::Preview) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let snapshot = doc.snapshot;
    let labels = bim_labels(view_state);
    let utility = crate::editor::bim::utilities::active(view_state);
    let windows = || TreeWindows::for_body(view_state, body_key);
    let node = crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::with_inference(instance_of(doc), snapshot, |inference| match body_key {
        plan::BODY_KEY => {
            let config = plan::config::current(cfg);
            let revision = crate::render::plan::framing_revision(&plan::active_view(snapshot, &config).unwrap_or_default());
            let overlay = crate::editor::bim::gestures::overlay::plan_records(snapshot, elements, utility, preview, 1.0 / config.viewport.zoom.max(0.01));
            plan::render_over(snapshot, inference, &config, elements, hover, utility, revision, labels, &overlay)
        }
        world::BODY_KEY => {
            let config = world::config::current(cfg);
            world::render_over(snapshot, inference, &config, elements, hover, crate::render::plan::framing_revision(&format!("{}{}", config.isolated_storey, config.projection.kind)), preview)
        }
        section::BODY_KEY => {
            let config = section::config::current(cfg);
            let (start, end) = section::plane_of(snapshot, &config).map_or(([0.0, 0.0], [0.0, 0.0]), |plane| ([plane.start.x, plane.start.y], [plane.end.x, plane.end.y]));
            let overlay = crate::editor::bim::gestures::overlay::section_records(inference, start, end, utility, preview, 1.0 / config.viewport.zoom.max(0.01));
            let revision = crate::render::plan::framing_revision(&section::active_view(snapshot, &config).unwrap_or_default());
            section::render_over(snapshot, inference, &config, elements, utility, revision, labels, &overlay)
        }
        schedule::BODY_KEY => schedule::render(snapshot, inference, &schedule::config::current(cfg), labels),
        sheet::BODY_KEY => {
            let config = sheet::config::current(cfg);
            let active = sheet::active_sheet(snapshot, &config);
            let layout = active.as_ref().and_then(|id| inference.sheet_layouts.get(id));
            let overlay = crate::editor::bim::gestures::overlay::sheet_records(layout, elements, utility, preview, 1.0 / config.viewport.zoom.max(0.01));
            sheet::render_over(snapshot, inference, &config, elements, utility, crate::render::plan::framing_revision(&active.unwrap_or_default()), labels, &overlay)
        }
        outliner_panel::BODY_KEY => outliner_panel::render(snapshot, inference, labels, &windows()),
        properties_panel::BODY_KEY => properties_panel::render(snapshot, inference, elements, library, labels),
        library_panel::BODY_KEY => library_panel::render(snapshot, labels, &windows()),
        classification_panel::BODY_KEY => classification_panel::render(snapshot, labels, elements, library, &windows()),
        diagnostics_panel::BODY_KEY => diagnostics_panel::render(snapshot, inference, labels, &windows()),
        _ => semio_framework_plugin::built_text_node(Label::data(format!("{}: {body_key}", labels.unknown_body.as_str()))).map_err(|_| crate::editor::bim::kit::ui_capacity_error()),
    })?;
    Ok(semio_framework_plugin::built_to_component_tree(accessible_surface(node, body_key, labels)?))
}

/// ♿️ The accessible name of a window or panel body, and for a drawing surface how to drive it without a pointer; a body this table does not know keeps its own node.
fn accessible_surface(node: semio_framework_plugin::BuiltNode, body_key: &str, labels: &BimLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let (name, description) = match body_key {
        plan::BODY_KEY => (labels.window_plan, Some(labels.surface_plan_describe)),
        world::BODY_KEY => (labels.window_world, Some(labels.surface_world_describe)),
        section::BODY_KEY => (labels.window_section, Some(labels.surface_section_describe)),
        schedule::BODY_KEY => (labels.window_schedule, Some(labels.surface_schedule_describe)),
        sheet::BODY_KEY => (labels.window_sheet, Some(labels.surface_sheet_describe)),
        outliner_panel::BODY_KEY => (labels.panel_outliner, None),
        properties_panel::BODY_KEY => (labels.panel_properties, None),
        library_panel::BODY_KEY => (labels.panel_library, None),
        classification_panel::BODY_KEY => (labels.panel_classification, Some(labels.classification_surface_describe)),
        diagnostics_panel::BODY_KEY => (labels.panel_diagnostics, Some(labels.diag_surface_describe)),
        _ => return Ok(node),
    };
    crate::editor::bim::kit::accessible(node, name.as_str(), description.as_ref().map(|description| description.as_str()))
}

impl ArtifactEditor for BimModelApp {
    fn fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
        bim_fault_notices()
    }

    type Snapshot = ModelSnapshot;
    type Mutation = ModelMutation;
    type Config = NoConfig;
    type ConfigMutation = NoConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = BimPresence;
    type PresenceMutation = BimPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = BimCommand;

    const DIALECT: Dialect = BIM_MODEL_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = BIM_MODEL_DOCUMENT_SCHEMA;

    bim_command_table!(tool_proofs);

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(BimCommandJobFactory::new(&controller))
    }

    fn build_instance_operation_owner() -> Box<dyn semio_framework_plugin::ArtifactInstanceOperationOwner> {
        Box::<crate::editor::bim::gestures::GestureOwner>::default()
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !BIM_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id || bim_bounded_extent(&request.command).is_none() {
            return Err(crate::editor::bim::kit::fault("app.command.tool-mismatch", "Bim command does not match its exact registered tool"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = match &*request.command {
            BimCommand::ExportScheduleCsv(_) => Box::new(export_schedule_csv::ScheduleCsvWork::new(tool_id)),
            BimCommand::AnalyseModel(_) => Box::new(analyse_model::AnalyseWork::new(tool_id)),
            BimCommand::ExportModel(_) => Box::new(export_model::ModelExportWork::new(tool_id)),
            BimCommand::ExportSheets(_) => Box::new(export_sheets::SheetsExportWork::new(tool_id)),
            _ => Box::new(BimCommandWork::new(tool_id, request.instance_operation_owner.clone())),
        };
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id,
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::new(
            ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            BimCommand::command_id,
            BIM_BOUNDED_RAW_BYTES,
            BIM_BOUNDED_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(app_schema_descriptor())
    }

    fn initial_snapshot() -> ModelSnapshot {
        crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot()
    }

    fn io() -> Option<AppIo> {
        Some(bim_io())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("bim-artifact-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(semio_framework_plugin::bounded_config_store_one_item_preparation_factory::<Self::Config, Self::ConfigMutation>("bim-config-retained", store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))
    }

    fn build_document_store_owners() -> Option<store::DocumentStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::DocumentStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::DocumentStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(semio_framework_plugin::no_draft_store_owners())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::no_draft_store_disposer())
    }

    fn build_presence_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Presence, Self::PresenceMutation>())
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Presence>())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(semio_framework_plugin::PresenceStoreOwnedDisposer::new(std::sync::Arc::new(Self::Presence::default()), |value| value == &Self::Presence::default()).expect("default bim presence is the exact empty terminal")))
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(semio_framework_plugin::no_transient_store_disposer())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::no_transient_local_root_retirement_factory())
    }

    fn register_window_config_owners(registry: &mut semio_framework_plugin::WindowConfigOwnerRegistry) -> Result<(), Fault> {
        plan::config::register(registry)?;
        world::config::register(registry)?;
        section::config::register(registry)?;
        schedule::config::register(registry)?;
        sheet::config::register(registry)
    }

    fn register_window_transient_owners(registry: &mut semio_framework_plugin::WindowTransientOwnerRegistry) -> Result<(), Fault> {
        crate::editor::bim::transient::register(registry)
    }

    fn mounted_job_maintenance_step(_instance_id: u32, _maximum_items: usize, _maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        Ok(semio_framework_plugin::PluginCloseStep::Complete)
    }

    fn mounted_job_close_step(instance_id: u32, _maximum_items: usize, _maximum_bytes: usize) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::close(instance_id);
        Ok(semio_framework_plugin::PluginCloseStep::Complete)
    }

    fn mounted_jobs_terminal_is_empty(instance_id: u32) -> bool {
        crate::standards::v1::subsets::any::schema::inferences::model_graph::registry::terminal_is_empty(instance_id)
    }

    fn operation_progress_scope() -> semio_framework::kernel::UiDirtyScope {
        semio_framework::kernel::UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: vec![outliner_panel::BODY_KEY.to_string()], utilities: false, tools: false, engagements: false, measures: false, labels: false }
    }

    fn entity_label(snapshot: &ModelSnapshot, _kinds: &[String], id: &str) -> Option<LocalizedLabel> {
        let name = kind_holding(snapshot, id).and_then(|row| (row.name)(snapshot, id))?;
        Some(LocalizedLabel::native(&name, &name))
    }

    fn interaction_topology(doc: &ArtifactView<'_, ModelSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<protocol::InteractionTopology, semio_framework_value::ValueError> {
        Ok(crate::editor::bim::interaction::topology(doc.snapshot))
    }

    fn command_id(command: &BimCommand) -> &'static str {
        command.command_id()
    }

    fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<Self::Command, Fault> {
        args_bridge::command_from_action(action, args)
    }

    fn handle(
        command: &BimCommand,
        doc: &ArtifactView<'_, ModelSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        interaction: &InteractionView<'_>,
        view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<ModelMutation, NoConfigMutation, Self::DraftMutation>, Fault> {
        let mut ctx = BimDispatchCtx::new(selection_ids(interaction, crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN), selection_ids(interaction, crate::editor::bim::interaction::BIM_LIBRARY_DOMAIN), view_state, cfg.window, None);
        command.dispatch(doc, cfg, &mut ctx)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, ModelSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        render_body(body_key, doc, cfg, view_state, &[], &[], &[], &Default::default())
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, ModelSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        use crate::editor::bim::interaction::{BIM_ELEMENT_DOMAIN, BIM_LIBRARY_DOMAIN};
        let preview = crate::editor::bim::gestures::session::Preview::from_text(&crate::editor::bim::transient::current(transient).preview);
        render_body(body_key, doc, cfg, view_state, &selection_ids(interaction, BIM_ELEMENT_DOMAIN), &selection_ids(interaction, BIM_LIBRARY_DOMAIN), &hover_ids(interaction, BIM_ELEMENT_DOMAIN), &preview)
    }

    fn window_engagements(doc: &ArtifactView<'_, ModelSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel) -> HashMap<String, WindowEngagement> {
        crate::editor::bim::chrome::engagements(doc.snapshot, view_state, crate::editor::bim::chrome::plan_storey(doc.snapshot, cfg, view_state).as_deref(), 0, &problems_of(doc))
    }

    fn window_engagements_with_request_context(
        doc: &ArtifactView<'_, ModelSnapshot>,
        cfg: &ConfigView<'_, NoConfig>,
        view_state: &ViewModel,
        _transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,
        interaction: &InteractionView<'_>,
    ) -> HashMap<String, WindowEngagement> {
        let selected = selection_ids(interaction, crate::editor::bim::interaction::BIM_ELEMENT_DOMAIN).len();
        crate::editor::bim::chrome::engagements(doc.snapshot, view_state, crate::editor::bim::chrome::plan_storey(doc.snapshot, cfg, view_state).as_deref(), selected, &problems_of(doc))
    }

    fn window_measures(doc: &ArtifactView<'_, ModelSnapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        crate::editor::bim::chrome::measures(doc.snapshot, cfg, view_state)
    }
}
//#endregion 🔖️BimModelApp

//#region 🔖️Manifest
fn bim_action_args(id: &str) -> Vec<ActionArgDef> {
    let label = |pick: fn(&BimLabels) -> semio_framework_ui_locale::LabelText| LocalizedLabel::native(pick(&BimLabels::NATIVE_EN).as_str(), pick(&BimLabels::NATIVE_DE).as_str());
    let text = |name: &'static str, pick: fn(&BimLabels) -> semio_framework_ui_locale::LabelText| ActionArgDef::text(name, label(pick));
    let ids = || ActionArgDef::text_list("ids", label(|labels| labels.arg_entities));
    match id {
        "createEntity" => vec![text("kind", |labels| labels.arg_kind).required(), text("parent", |labels| labels.arg_container), text("name", |labels| labels.arg_name)],
        "createView" => vec![text("kind", |labels| labels.arg_kind).required(), text("parent", |labels| labels.arg_container), text("name", |labels| labels.arg_name)],
        "deleteSelection" | "flipWalls" | "storeyUp" | "storeyDown" => vec![ids()],
        "renameEntity" => vec![text("id", |labels| labels.arg_entity).required(), text("name", |labels| labels.arg_name).required()],
        "setField" => vec![ids(), text("field", |labels| labels.arg_parameter).required(), text("value", |labels| labels.arg_value).required()],
        "setView" => vec![text("field", |labels| labels.arg_setting).required(), text("value", |labels| labels.arg_value)],
        "editSchedule" => vec![text("id", |labels| labels.arg_entity).required(), text("part", |labels| labels.arg_part).required(), text("op", |labels| labels.arg_operation).required(), text("key", |labels| labels.arg_key), text("value", |labels| labels.arg_value)],
        "splitWall" => vec![ids(), text("at", |labels| labels.arg_fraction)],
        "setProperty" => vec![ids(), text("pset", |labels| labels.arg_property_set), text("property", |labels| labels.arg_property), text("value", |labels| labels.arg_value).required()],
        "removeProperty" => vec![ids(), text("pset", |labels| labels.arg_property_set).required(), text("property", |labels| labels.arg_property).required()],
        "selectFindings" => vec![ids()],
        "removeClassification" => vec![ids(), text("system", |labels| labels.arg_classification_system).required()],
        "setClassification" => vec![ids(), text("system", |labels| labels.arg_classification_system).required(), text("code", |labels| labels.arg_classification_code).required()],
        "applyTemplate" => vec![ids(), text("template", |labels| labels.arg_template).required()],
        "editTemplate" => vec![text("id", |labels| labels.arg_entity).required(), text("op", |labels| labels.arg_operation).required(), text("index", |labels| labels.arg_index), text("field", |labels| labels.arg_parameter), text("value", |labels| labels.arg_value)],
        "editClassification" => vec![text("id", |labels| labels.arg_entity).required(), text("op", |labels| labels.arg_operation).required(), text("code", |labels| labels.arg_classification_code), text("title", |labels| labels.arg_classification_title), text("parent", |labels| labels.arg_parent)],
        "searchClassification" => vec![text("system", |labels| labels.arg_classification_system).required(), text("query", |labels| labels.arg_query)],
        "placeElements" => vec![ids(), text("at", |labels| labels.arg_position).required()],
        "engagementInput" | "engagementSubmit" => vec![text("value", |labels| labels.arg_typed_line)],
        _ => Vec::new(),
    }
}

/// ⌨️ The keys of the commands that are no utility: history, the delete keys and the wall flip.
pub const COMMAND_KEYBINDINGS: &[(&str, &str)] = &[("mod+z", "undo"), ("mod+shift+z", "redo"), ("delete", "deleteSelection"), ("backspace", "deleteSelection"), ("shift+f", "flipWalls"), ("alt+arrowup", "storeyUp"), ("alt+arrowdown", "storeyDown")];

/// ⌨️ Every keybinding of the editor in registration order: the commands, the utilities and the keys of a gesture in progress. No key is bound twice.
pub fn all_keybindings() -> Vec<(&'static str, &'static str)> {
    COMMAND_KEYBINDINGS.iter().copied().chain(crate::editor::bim::utilities::keybindings()).chain(crate::editor::bim::gestures::GESTURE_KEYBINDINGS.iter().copied()).collect()
}

/// 🧱️ The editor manifest: one definition per taxonomy node.
pub fn create_bim_app() -> semio_framework_plugin::AppDefinition {
    let mut builder = Editor::builder(BIM_MODEL_DIALECT)
        .document(["semio", "bim"])
        .artifact_kind(crate::artifact_kind())
        .icon_id("building")
        .mode_def(edit::definition())
        .default_mode_id(edit::BIM_EDIT_MODE_EDIT)
        .window_kind_def(plan::definition())
        .window_kind_def(world::definition())
        .window_kind_def(section::definition())
        .window_kind_def(schedule::definition())
        .window_kind_def(sheet::definition())
        .default_layout(edit::layout())
        .panel_tab_def(outliner_panel::definition())
        .panel_tab_def(properties_panel::definition())
        .panel_tab_def(library_panel::definition())
        .panel_tab_def(classification_panel::definition())
        .panel_tab_def(diagnostics_panel::definition())
        .io(bim_io());
    for definition in crate::editor::bim::interaction::definitions() {
        builder = builder.interaction(definition);
    }
    for utility in crate::editor::bim::utilities::definitions() {
        builder = builder.utility(utility);
    }
    for action in command_actions() {
        let id = action.id.clone();
        let pointer = crate::editor::bim::gestures::POINTER_COMMAND_IDS.contains(&id.as_str());
        let described = ActionDefinition { in_palette: !pointer, ..action.with_args(bim_action_args(&id)) };
        builder = builder.action_with(described).action_interactive_job(&id, InteractiveJobClassification::Migrated);
        if pointer {
            builder = builder.action_audience(&id, semio_framework_plugin::CapabilityAudience::Input);
        }
    }
    builder = builder.action_destructive("deleteSelection");
    for (keys, action) in all_keybindings() {
        builder = builder.keybinding(keys, action);
    }
    builder.build_definition()
}
//#endregion 🔖️Manifest

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;

#[cfg(test)]
#[path = "🧪️tests/⌨️completeness/🦀️.rs"]
mod completeness_tests;
