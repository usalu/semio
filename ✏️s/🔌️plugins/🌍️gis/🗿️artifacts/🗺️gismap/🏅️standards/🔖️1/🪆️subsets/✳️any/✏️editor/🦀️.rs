//! 🗺️ GIS 2D play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, the map canvas
//! and its chrome in `🎭️modes/✏️edit/🪟️windows/🗺️map` (+ its `🎚️options/*`), panel trees in
//! `📌️panels/*`, labels in `🦀️terminology.rs`, view state in `🦀️config.rs`, the shared `MapHost`
//! projection in `🦀️maphost.rs`, and document-side compute in `crate::schema`.
//! This app's typed media I/O surface (`gis2d_io`/ports/`gis2d_map_media`) lives below in `🔖️Io` —
//! relocated from the artifact's `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES),
//! since an `AppIo` surface is app behaviour, not artifact data.

use crate::editor::gis2d::commands::{example, features, inference, shell, view};
use crate::editor::gis2d::modes::edit::windows::map::config::{self as map_config, layer_visible, mutations as map_config_mutations, MapWindowConfig, MapWindowConfigMutation};
use crate::editor::gis2d::modes::edit;
use crate::editor::gis2d::modes::edit::windows::map;
use crate::editor::gis2d::panels::{artifact as document_panel, catalogue as catalogue_panel, inspection as inspection_panel};
use crate::editor::gis2d::terminology::gis2d_labels;
use crate::standards::v1::subsets::any::schema::mutations::GisMapMutation;
use crate::standards::v1::subsets::any::io::text::snapshot::{gis_map_document_from_descriptor_json};
use crate::{GisMapSnapshot, MapFeature, GIS_MAP_SCHEMA};
use crate::standards::v1::subsets::any::artifact_declaration::{artifact_kind};
use crate::tile_coordinates::gis_policy::GIS_MAP_LAYER_KEYS;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::tree_item;
use semio_framework_plugin::tree_item_with_action;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::Menu;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::WindowMeasure;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;
use serde_json::Value;
use std::collections::HashMap;
use pack::snapshot::ArtifactPack;
use semio_framework_2d::compute::EngineHandles;

pub use semio_s_artifact_gis_gismap_inferences::editor::gis2d::constants_and_io;
pub use constants_and_io::*;

#[path = "🕹️commands-and-retained-jobs/🦀️.rs"]
mod commands_and_retained_jobs;
pub use commands_and_retained_jobs::*;

#[path = "📬️one-item-preparation/🦀️.rs"]
mod one_item_preparation;
pub use one_item_preparation::*;

#[path = "🧭️play-app-editor/🦀️.rs"]
mod play_app_editor;
use play_app_editor::*;

#[path = "🧾️manifest/🦀️.rs"]
mod manifest;
pub use manifest::*;

//#region 🔖️Commands

// 🧷️ `app_commands!` addresses each payload module by a single identifier.
use example::set_active_example;
use features::{add_feature, delete_feature, move_feature, patch_positions, patch_route, patch_routes, rename_feature};
use inference::propose_bounds_region;
use shell::open_source;
use view::{fit_world, focus_feature, set_camera, set_layer_stroke_scale, set_lod_mode, set_render_mode, set_vector_style, toggle_layer_visibility};
//#endregion 🔖️Commands

//#region 🧪️UnitTests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub mod unit_tests;
//#endregion 🧪️UnitTests
