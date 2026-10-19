//! manifest stitch.
//! 🖥️ Fem2d play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, view state in concrete window config, shared compute in the artifact's `⚙️engine`.
//! This file is a routing table: `handle` → `Fem2dCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one passthrough per node (fem2d's mode/window declarations stay
//! scalar/inline — no `mode_def`/`window_kind_def` object is built anywhere in the pre-migration code).

use semio_s_fem_app_surface::ResultDisplay;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_area_load;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_bar;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_beam;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_combination;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_load_case;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_material;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_member_udl;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_nodal_load;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_node;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_region;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_section;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::add_support;
use crate::editor::fem2d::commands::canvas_pointer_down;
use crate::editor::fem2d::commands::canvas_pointer_move;
use crate::editor::fem2d::commands::canvas_pointer_up;
use crate::editor::fem2d::commands::focus_entity;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_combination;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_element;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_load;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_load_case;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_material;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_node;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_region;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_section;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_support;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::remove_selection;
use crate::editor::fem2d::commands::result_animation_tick;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::set_active_example;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::set_analysis_settings;
use crate::editor::fem2d::commands::set_camera;
use crate::editor::fem2d::commands::set_result_animation;
use crate::editor::fem2d::commands::set_result_display;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::set_self_weight;
use crate::editor::fem2d::commands::gumball::rotate_selection;
use crate::editor::fem2d::commands::gumball::scale_selection;
use crate::editor::fem2d::commands::gumball::set_transform_gumball_flag;
use crate::editor::fem2d::commands::gumball::translate_selection;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_interaction_definition;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::Fem2dInteractionSnapshot;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::FEM2D_INTERACTION_DOMAIN;
use semio_s_artifact_fem_2d_editor_panels::editor::fem2d::panels::artifact as artifact_panel;
use semio_s_artifact_fem_2d_editor_panels::editor::fem2d::panels::inspection as inspection_panel;
use semio_s_artifact_fem_2d_editor_panels::editor::fem2d::panels::results as results_panel;
use semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::fem2d_labels;
use semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results::results_window_instance_id;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use crate::editor::fem2d::modes::edit;
use semio_s_artifact_fem_2d_editor_model_window::editor::fem2d::modes::edit::windows::model as model_window;
use semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results as results_window;
use semio_s_fem_model::Dof;
use semio_s_fem_model::ElementResult;
use semio_s_artifact_fem_2d_mutations::standards::v1::subsets::any::schema::mutations::Fem2dMutation;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_framework_pack_json::Value;
use semio_s_artifact_fem_2d_io::standards::v1::subsets::any::io::reset_document_effect;
use semio_framework::InteractiveJobClassification;
use semio_framework::ToolExecutionContract;
use semio_framework::ToolFactoryKey;
use semio_framework::ToolJobFactory;
use semio_framework::ToolJobFactoryError;
use semio_framework_plugin::app::Dialect;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::built_text_node;
use semio_framework_plugin::create_default_layout;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigSpec;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::ViewModel;
use semio_framework_plugin::WindowMeasure;
use std::collections::HashMap;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Constants

//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Fem2dPlayApp::Command` — the SOLE dispatch surface for fem2d's own behavior, assembled from
    /// the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`,
    /// the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the codec uses) — genuinely different vocabularies; `"setActiveExample" as
    /// "active-example"` and `"setCamera" as "camera"` are two of the rows that prove it. **Row order is
    /// the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum Fem2dCommand for Fem2dSnapshot, Fem2dMutation, NoConfig, NoConfigMutation {
        "addNode" as "add-node" => add_node::AddNode,
        "addBar" as "add-bar" => add_bar::AddBar,
        "addBeam" as "add-beam" => add_beam::AddBeam,
        "addMaterial" as "add-material" => add_material::AddMaterial,
        "addSection" as "add-section" => add_section::AddSection,
        "addSupport" as "add-support" => add_support::AddSupport,
        "addNodalLoad" as "add-nodal-load" => add_nodal_load::AddNodalLoad,
        "addMemberUdl" as "add-member-udl" => add_member_udl::AddMemberUdl,
        "addAreaLoad" as "add-area-load" => add_area_load::AddAreaLoad,
        "addRegion" as "add-region" => add_region::AddRegion,
        "addLoadCase" as "add-load-case" => add_load_case::AddLoadCase,
        "addCombination" as "add-combination" => add_combination::AddCombination,
        "setSelfWeight" as "set-self-weight" => set_self_weight::SetSelfWeight,
        "setAnalysisSettings" as "set-analysis-settings" => set_analysis_settings::SetAnalysisSettings,
        "removeSelection" as "remove-selection" => remove_selection::RemoveSelection,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setResultDisplay" as "result-display" => set_result_display::SetResultDisplay,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "patchNode" as "patch-node" => patch_node::PatchNode,
        "patchElement" as "patch-element" => patch_element::PatchElement,
        "patchMaterial" as "patch-material" => patch_material::PatchMaterial,
        "patchSection" as "patch-section" => patch_section::PatchSection,
        "patchSupport" as "patch-support" => patch_support::PatchSupport,
        "patchRegion" as "patch-region" => patch_region::PatchRegion,
        "patchLoad" as "patch-load" => patch_load::PatchLoad,
        "patchLoadCase" as "patch-load-case" => patch_load_case::PatchLoadCase,
        "patchCombination" as "patch-combination" => patch_combination::PatchCombination,
        "setResultAnimation" as "result-animation" => set_result_animation::SetResultAnimation,
        "resultAnimationTick" as "result-animation-tick" => result_animation_tick::ResultAnimationTick,
        "focusEntity" as "focus-entity" => focus_entity::FocusEntity,
        "translateSelection" as "translate-selection" => translate_selection::TranslateSelection,
        "rotateSelection" as "rotate-selection" => rotate_selection::RotateSelection,
        "scaleSelection" as "scale-selection" => scale_selection::ScaleSelection,
        "setTransformGumballFlag" as "set-transform-gumball-flag" => set_transform_gumball_flag::SetTransformGumballFlag,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

#[path = "🧾️retained-command-contracts-and-work/🦀️.rs"]
mod retained_command_contracts_and_work;
use retained_command_contracts_and_work::{
    fem2d_document_items, fem2d_retained_contract, Fem2dRetainedWork, FEM2D_MAXIMUM_DOCUMENT_ITEMS,
    FEM2D_PUBLICATION_CONTRACTS, FEM2D_RETAINED_PAYLOAD_SCHEMA, FEM2D_RETAINED_RAW_BYTES,
    FEM2D_RETAINED_TOOL_IDS, FEM2D_RETAINED_WORK_ITEMS,
};

#[path = "🏭️retained-job-factories-and-preparation/🦀️.rs"]
mod retained_job_factories_and_preparation;
use retained_job_factories_and_preparation::{
    Fem2dArtifactPreparationFactory, Fem2dRetainedCommandJobFactory,
};
#[cfg(test)]
use retained_job_factories_and_preparation::Fem2dArtifactPreparation;

#[path = "📊️result-json-and-ports/🦀️.rs"]
mod result_json_and_ports;
use result_json_and_ports::{config_result_display, fem2d_dof, fem2d_dofs, results_map_json};
pub use result_json_and_ports::{fem2d_geometry_in_port, fem2d_io, fem2d_results_out_port};

#[path = "🎛️play-app-editor/🦀️.rs"]
mod play_app_editor;
pub use play_app_editor::Fem2dPlayApp;

#[path = "🚀️ui-helpers-and-app-assembly/🦀️.rs"]
mod ui_helpers_and_app_assembly;
pub use ui_helpers_and_app_assembly::create_fem2d_app;

//#region 📬️ArtifactStorePreparation

//#region 🧪️ArtifactPreparationLaws
#[cfg(test)]
#[path = "🧪️tests/🔬️fem2d-artifact-preparation-laws/🦀️.rs"]
mod fem2d_artifact_preparation_laws;

//#endregion 🧪️ArtifactPreparationLaws
//#endregion 📬️ArtifactStorePreparation

//#region 🧪️UnitTests
//#endregion 🧪️UnitTests
