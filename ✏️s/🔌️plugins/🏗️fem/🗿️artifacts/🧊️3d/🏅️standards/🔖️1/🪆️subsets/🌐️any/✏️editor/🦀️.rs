//! 📜️ manifest stitch. Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: `Fem3dPlayApp` now
//! 🖥️ FEM 3D play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! authors the `✏️editor` surface only — the read-only `👁️viewer` surface is a genuinely independent
//! sibling (`semio_s_artifact_fem_3d_viewer::viewer::fem3d::Fem3dViewer`), never constructed from this file.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, view state in concrete window config, shared compute in the artifact's
//! `⚙️engine`. This file is a routing table: `handle` → `Fem3dCommand::dispatch`, `render` → body-key →
//! window, and a `🔖️Manifest` region that calls one passthrough per node (scalar `.mode(..)`/
//! `.window_kind(..)` calls stay inline — fem3d builds neither a `ModeDefinition` nor a
//! `WindowKindDefinition` object anywhere, see `modes::edit`'s and the window nodes' own doc comments).

use semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results::results_window_instance_id;
use semio_s_artifact_fem_3d_io::standards::v1::subsets::any::io::reset_document_effect;
use crate::editor::fem3d::commands::gumball::rotate_selection;
use crate::editor::fem3d::commands::gumball::scale_selection;
use crate::editor::fem3d::commands::gumball::set_transform_gumball_flag;
use crate::editor::fem3d::commands::gumball::translate_selection;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_area_load;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_bar;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_combination;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_frame;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_load_case;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_material;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_member_udl;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_nodal_load;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_node;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_section;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_solid;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::add_support;
use crate::editor::fem3d::commands::focus_entity;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_combination;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_element;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_load;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_load_case;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_material;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_node;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_section;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_solid;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::patch_support;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::remove_selection;
use crate::editor::fem3d::commands::result_animation_tick;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::set_active_example;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::set_analysis_settings;
use crate::editor::fem3d::commands::set_camera;
use crate::editor::fem3d::commands::set_result_animation;
use crate::editor::fem3d::commands::set_result_display;
use semio_s_artifact_fem_3d_editor_commands::editor::fem3d::commands::set_self_weight;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::gumball::Fem3dGumballMotion;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::fem3d_interaction_definition;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::fem3d_transform_armed;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::Fem3dInteractionSnapshot;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::FEM3D_INTERACTION_DOMAIN;
use semio_s_artifact_fem_3d_editor_interaction::editor::fem3d::interaction::FEM3D_UTILITY_TRANSFORM;
use crate::editor::fem3d::modes::edit;
use semio_s_artifact_fem_3d_editor_model_window::editor::fem3d::modes::edit::windows::model as window_model;
use semio_s_artifact_fem_3d_editor_results_window::editor::fem3d::modes::edit::windows::results as window_results;
use semio_s_artifact_fem_3d_editor_panels::editor::fem3d::panels::artifact as artifact_panel;
use semio_s_artifact_fem_3d_editor_panels::editor::fem3d::panels::inspection as inspection_panel;
use semio_s_artifact_fem_3d_editor_panels::editor::fem3d::panels::results as results_panel;
use semio_s_artifact_fem_3d_editor_terminology::editor::fem3d::terminology::fem3d_labels;
use semio_s_fem_model::Dof;
use semio_s_fem_model::ElementResult;
use semio_s_artifact_fem_3d_mutations::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshot;
use semio_framework_pack_json::Value;
use semio_framework::InteractiveJobClassification;
use semio_framework::ToolExecutionContract;
use semio_framework::ToolFactoryKey;
use semio_framework::ToolJobFactory;
use semio_framework::ToolJobFactoryError;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::built_text_node;
use semio_framework_plugin::create_default_layout;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::AppDefinition;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::AppRenderOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigSpec;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
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
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::ViewModel;
use semio_framework_plugin::WindowMeasure;
use semio_framework_plugin::retained_command::ArtifactCommandInputs;
use semio_framework_plugin::retained_command::ArtifactCommandWork;
use semio_framework_plugin::retained_command::ArtifactCommandWorkStep;
use semio_framework_plugin::EphemeralEmit;
use std::collections::HashMap;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Fem3dPlayApp::Command` — the SOLE dispatch surface for fem3d's own behavior, assembled from
    /// the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`,
    /// the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the codec uses) — they are genuinely different vocabularies for 3 of these 18
    /// rows: `setActiveExample`/`active-example`, `setCamera`/`camera`, `setResultDisplay`/
    /// `result-display`. **Row order is the binary variant ordinal: appending is safe, reordering is a
    /// `Fem3dCommand` enum never had one (a pre-existing, intentional asymmetry between the two apps).
    pub enum Fem3dCommand for Fem3dSnapshot, Fem3dMutation, NoConfig, NoConfigMutation {
        "addNode" as "add-node" => add_node::AddNode,
        "addBar" as "add-bar" => add_bar::AddBar,
        "addFrame" as "add-frame" => add_frame::AddFrame,
        "addMaterial" as "add-material" => add_material::AddMaterial,
        "addSection" as "add-section" => add_section::AddSection,
        "addSupport" as "add-support" => add_support::AddSupport,
        "addNodalLoad" as "add-nodal-load" => add_nodal_load::AddNodalLoad,
        "addMemberUdl" as "add-member-udl" => add_member_udl::AddMemberUdl,
        "addAreaLoad" as "add-area-load" => add_area_load::AddAreaLoad,
        "addSolid" as "add-solid" => add_solid::AddSolid,
        "addLoadCase" as "add-load-case" => add_load_case::AddLoadCase,
        "addCombination" as "add-combination" => add_combination::AddCombination,
        "setSelfWeight" as "set-self-weight" => set_self_weight::SetSelfWeight,
        "setAnalysisSettings" as "set-analysis-settings" => set_analysis_settings::SetAnalysisSettings,
        "removeSelection" as "remove-selection" => remove_selection::RemoveSelection,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
        "setCamera" as "camera" => set_camera::SetCamera,
        "setResultDisplay" as "result-display" => set_result_display::SetResultDisplay,
        "patchNode" as "patch-node" => patch_node::PatchNode,
        "patchElement" as "patch-element" => patch_element::PatchElement,
        "patchMaterial" as "patch-material" => patch_material::PatchMaterial,
        "patchSection" as "patch-section" => patch_section::PatchSection,
        "patchSupport" as "patch-support" => patch_support::PatchSupport,
        "patchSolid" as "patch-solid" => patch_solid::PatchSolid,
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

#[path = "🗃️retained-commands/🦀️.rs"]
mod retained_commands;
pub(crate) use retained_commands::*;
pub use retained_commands::fem3d_route;

#[path = "📬️store-preparation/🦀️.rs"]
mod store_preparation;
use store_preparation::*;

#[path = "🔌️json-io-and-arg-helpers/🦀️.rs"]
mod json_io_and_arg_helpers;
pub use json_io_and_arg_helpers::*;

//#region 🔖️Fem3dPlayApp

#[path = "🖥️play-app/🦀️.rs"]
mod play_app;
pub use play_app::*;

//#region 🔖️UiHelpers

//#endregion 🔖️UiHelpers
//#endregion 🔖️Fem3dPlayApp

#[path = "📋️manifest/🦀️.rs"]
mod manifest;
pub use manifest::*;

//#region 🧪️UnitTests
#[cfg(test)]
#[path = "🧪️tests/🪟️window-config-ownership/🦀️.rs"]
mod window_config_ownership;
//#endregion 🧪️UnitTests
