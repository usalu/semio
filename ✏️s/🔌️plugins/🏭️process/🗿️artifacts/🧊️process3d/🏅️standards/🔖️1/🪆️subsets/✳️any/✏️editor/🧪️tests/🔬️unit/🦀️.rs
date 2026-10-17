extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
#[path = "../🧰️support/🦀️.rs"]
mod context;

use semio_s_artifact_process_process3d_editor::*;
use semio_s_artifact_process_process3d_kit::builtin_installed_catalogs;
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
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
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
use crate::{context::action, context::app, context::app_with_registry, context::dispatch, context::dispatch_with_utility, context::main_window_measures, context::main_window_view, context::process3d_app_manifest_for_tests, context::published_a_document_mutation, context::render as render_body, context::settled_dispatch, context::settled_dispatch_with_utility};
use semio_framework_plugin::{artifact_app_laws, ContextMenuRequest, ContextMenuSurfaceTarget, EditorApp, HistoryView, PluginApp, UiMenuRef, SET_ACTIVE_UTILITY_ACTION_ID};

#[path = "🏭️production-envelope/🦀️.rs"]
mod production_envelope;

//#region 🔖️CommandSurface

#[path = "🧵️command-surface/🦀️.rs"]
mod command_surface;

//#endregion 🔖️CommandSurface

//#region 🔖️ManifestSanity

#[path = "📋️manifest-and-io/🦀️.rs"]
mod manifest_and_io;

//#endregion 🔖️ManifestSanity

//#region 🔖️IoTests

//#endregion 🔖️IoTests

//#region 🔖️CrossCutting

#[path = "🎮️interaction-behavior/🦀️.rs"]
mod interaction_behavior;

//#endregion 🔖️CrossCutting

//#region 🔖️MediaTests

//#endregion 🔖️MediaTests

//#region 🔖️BehaviorTests

//#endregion 🔖️BehaviorTests

//#region 🧩️RealContributionsPackAdmission

#[path = "🧩️contributions-and-examples/🦀️.rs"]
mod contributions_and_examples;

//#endregion 🧩️RealContributionsPackAdmission

//#region 🧩️HostContributionsOutputCap
//#endregion 🧩️HostContributionsOutputCap

//#region 🚪️ExampleArchiveDoor

//#endregion 🚪️ExampleArchiveDoor
