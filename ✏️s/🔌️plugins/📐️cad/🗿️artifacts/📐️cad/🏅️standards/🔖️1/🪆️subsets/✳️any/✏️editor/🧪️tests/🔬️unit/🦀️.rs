
/// 🎟️ Unbounded retained grant: the pre-migration step budget carried no retained limit.
const UNBOUNDED_STEP_GRANT: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
use semio_framework_pack_json::json;

#[path = "🧰️command-fixtures-and-retained-lanes/🦀️.rs"]
mod command_fixtures_and_retained_lanes;
pub(crate) use command_fixtures_and_retained_lanes::*;

#[path = "🌲️forest-mesh-and-schema/🦀️.rs"]
mod forest_mesh_and_schema;

#[path = "🖼️rendered-scene-and-manifest/🦀️.rs"]
mod rendered_scene_and_manifest;

#[path = "👁️view-model/🦀️.rs"]
mod view_model;

#[path = "⚙️operations-and-file-effects/🦀️.rs"]
mod operations_and_file_effects;

#[path = "🎮️engagement-and-import/🦀️.rs"]
mod engagement_and_import;

#[path = "⏪️history-and-convergence/🦀️.rs"]
mod history_and_convergence;

#[path = "🧩️contributions/🦀️.rs"]
mod contributions;

#[path = "🎚️engagement-commit-and-render-probes/🦀️.rs"]
mod engagement_commit_and_render_probes;

#[path = "🧬️geometry-media-and-owners/🦀️.rs"]
mod geometry_media_and_owners;

#[path = "../🧰️support/🦀️.rs"]
mod context;

use context::*;
use semio_s_artifact_cad_cad_editor::*;
use semio_s_artifact_cad_cad_commands::{camera::set_camera, camera::set_projection, camera::set_projection_param};
use semio_s_artifact_cad_cad_commands::contribution::set_contributions;
use semio_s_artifact_cad_cad_commands::{engagement::engagement_abort, engagement::engagement_input, engagement::engagement_possible_select, engagement::engagement_repeat_last, engagement::engagement_submit, engagement::world_pointer_down, engagement::world_pointer_move};
use semio_s_artifact_cad_cad_commands::{io::import_cad_file, io::load_raw_request, io::save_current, io::save_in_play, io::save_selected};
use semio_s_artifact_cad_cad_commands::model_definition::set_active_example;
use semio_s_artifact_cad_cad_commands::{node::add_node, node::rename_node, node::set_node_selection};
use semio_s_artifact_cad_cad_commands::{object::add_object, object::delete_object, object::duplicate_object, object::patch_object, object::patch_selection};
use semio_s_artifact_cad_cad_commands::{reference::patch_cad_play_reference, reference::reference_hover, reference::set_reference_hidden, reference::set_reference_locked, reference::set_reference_selection};
use semio_s_artifact_cad_cad_commands::{sun::set_sun_azimuth, sun::set_sun_elevation, sun::set_sun_intensity, sun::toggle_sun};
use semio_s_artifact_cad_cad_commands::{transform::apply_transformation, transform::rotate_selection, transform::scale_selection, transform::translate_selection};
use semio_s_artifact_cad_cad_commands::utility::set_dislocate_option;
use semio_s_artifact_cad_cad_config::{cad_sun_config_to_world, CadConfig, CadConfigMutation, CadDislocateOptions, SetEdit, SetContributionsEdit};
use semio_s_artifact_cad_cad_interaction::{self, apply_event, can_commit, keyed_transitions, resolve_interaction_key, start_session, CadEngagementScratch};
use semio_s_artifact_cad_cad_modes;
use semio_s_artifact_cad_cad_transform::{cad_import_object_emit, CadPaneModels, CadToolEntry, CadToolLeaf, CadTransformRecord};
use semio_s_artifact_cad_cad_modes::window_transient;
use semio_s_artifact_cad_cad_transient::CadWorldWindowTransient;
use semio_s_artifact_cad_cad_modes::{windows::building, windows::energy, windows::shape, windows::structure_classic};
use semio_s_artifact_cad_cad_panels::{catalogue, document, inspection};
use semio_s_artifact_cad_cad_kit::{editor::cad::terminology::cad_is_de_locale, editor::cad::terminology::cad_labels};
use semio_s_artifact_cad_cad_mutations::CadMutation;
use semio_s_artifact_cad_cad_model::{geometry::cad_object_from_model_element, geometry::model_element_from_cad_object, geometry::objects_from_model_snapshot, geometry::CadObject, geometry::CAD_OBJECT_PSET};
use semio_s_artifact_cad_cad_io::{export_solids_as, CadSolidExport, CAD_SOLID_EXPORT_DIALECT_STEP};
use semio_s_artifact_cad_cad_inferences::{cad_brep_kernel, cad_camera_projection_config, ensure_object_solid_handle, forest_play_scene, next_cad_id, CAD_EXAMPLE_FOREST_LEFT, CAD_MODEL_DEFINITION_BUILDING, CAD_MODEL_DEFINITION_ENERGY, CAD_MODEL_DEFINITION_SHAPE, CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC};
use semio_s_artifact_cad_cad_model::{snapshot::artifact_kind, snapshot::CadCamera, snapshot::CadPaneId, snapshot::CadSnapshot, snapshot::CadWorkingScene, snapshot::CAD_DOCUMENT_SCHEMA};
use semio_s_artifact_cad_cad_play::CadComposedPanes;
use {semio_s_artifact_stdio_semio_model::schema::mutations::remove_element::RemoveElement, semio_s_artifact_stdio_semio_model::schema::mutations::set_element::SetElement, semio_s_artifact_stdio_semio_model::schema::mutations::SemioModelMutation};
use semio_s_artifact_stdio_semio_model::schema::mutations::insert_element::InsertElement;
use semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelElement;
use semio_framework::kernel::Effect;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::tree_item as framework_tree_item;
use semio_framework_plugin::tree_item_with_action;
use semio_framework_plugin::world3d_camera_projection_json;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ChildContentView;
use semio_framework_plugin::CommandDefinition;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::ContextMenuItemSpec;
use semio_framework_plugin::ContextMenuRequest;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::Menu;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiText;
use semio_framework_plugin::UiValue;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::ViewModel;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowMeasure;
use semio_framework_plugin::WorldSunConfig;
use semio_framework_3d_brep_engine::{Brep, GeometryHandle};
use semio_framework_plugin::app::{ArtifactEditor, Dialect, Editor};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_pack_json::Value;
use std::collections::HashMap;
use semio_framework_2d_compute::EngineHandles;
use semio_s_artifact_cad_cad_io::scene_from_spatial_payload;
use semio_s_artifact_cad_cad_inferences::{align_mesh_to_host_snapshot_centroid, default_document, object_mesh_data, run_derive_from_geometry, CAD_CONCRETE_FOREST_REFERENCE_URL, CAD_DEFAULT_TYPOLOGY_EXTENT, CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX, CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX, CAD_FOREST_REFERENCE_PLANE_Z, CAD_FOREST_REFERENCE_WIDTH_WORLD, CAD_FOREST_REFERENCE_Y_OFFSET_RATIO};
use semio_s_artifact_cad_cad_model::{snapshot::empty_cad_snapshot, snapshot::CadNode, snapshot::CadReference, snapshot::CAD_PLAY_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ActionKind, AppActionRegistry, EditorApp, PluginApp, SET_ACTIVE_UTILITY_ACTION_ID};
use store::{os_store::backbone::Backbone, os_store::backbone::BackboneMessage, os_store::backbone::MemoryBackbone};

include!("../📦️member-authority/🦀️.rs");
