//! ✏️ WFC 2D editor — the `ArtifactEditor` for `s.wfc.wfc2d@1/*`: two windows over one document, the
//! `wfc-graph` slot canvas and the read-only `wfc-2d-preview`.
//!
//! Every command maps 1:1 onto a real `Wfc2dMutation` builder from the schema tree — no synthetic
//! "set field" indirection, because the domain's own mutations are already exactly this granular.
//! The two command families that are NOT document mutations (`change-camera`, `change-active-tile`)
//! go to the per-pane config instead: a camera is per-window view state. The SOLVE goes to the app
//! transient — it is an INFERENCE, so letting it reach the document lanes would make a derived value
//! look persisted and would pollute undo — and reaches the preview window through
//! `ArtifactEditor::render_with_request_context`, the only render entry point that carries that lane.

#![allow(clippy::result_large_err)]

use crate::editor::wfc2d::config::{wfc2d_active_tile_id, Wfc2dConfig, Wfc2dConfigMutation};
use crate::editor::wfc2d::modes::edit;
use crate::editor::wfc2d::modes::edit::tools::{drag, fill};
use crate::editor::wfc2d::modes::edit::windows::{graph, preview};
use crate::editor::wfc2d::transient::{Wfc2dTransient, Wfc2dTransientMutation};
use crate::mutations::{change_seed, change_tile_media, change_tile_weight, connect_slots, create_rule, create_slot, create_tile, delete_rule, delete_slot, delete_tile, disconnect_slots, move_slot, pin_slot, resize_slot, unpin_slot};
use crate::schema::snapshot::{Wfc2dRule, Wfc2dSlot, Wfc2dSlotEdge, Wfc2dTile, Wfc2dTileMedia};
use crate::{Wfc2dMutation, Wfc2dSnapshot, WFC_2D_DIALECT, WFC_2D_DOCUMENT_SCHEMA};
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::ActionArgDef;
#[cfg(test)] use semio_framework_plugin::Effect;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ToolRunJob;
use semio_framework_plugin::ToolRunJobRequest;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::NoPresence;
use semio_framework_plugin::NoPresenceMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::TopologyNode;
use semio_framework_tool_machine::{node_graph_edit_rows, NodeDragRecord, NodeGraphEditRow};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_2d_compute::EngineHandles;

#[path = "🧾️command-enum-and-retained-routes/🦀️.rs"]
mod command_enum_and_retained_routes;
pub use command_enum_and_retained_routes::{
    solve_transient, wfc2d_command_id, Wfc2dEditorCommand, Wfc2dRetainedCommandJobFactory,
    WFC_2D_NODE_GRAPH_EDIT, WFC_2D_RETAINED_TOOL_IDS, WFC_2D_SET_ACTIVE_EXAMPLE,
};
use command_enum_and_retained_routes::{
    wfc2d_retained_extent, Wfc2dCommandWork, WFC_2D_RETAINED_RAW_BYTES, WFC_2D_RETAINED_WORK_ITEMS,
};

#[path = "🕸️graph-view-args-and-gestures/🦀️.rs"]
mod graph_view_args_and_gestures;
pub use graph_view_args_and_gestures::{
    wfc2d_example_document, wfc2d_node_graph_edit, Wfc2dEditor, Wfc2dGraphGesture, Wfc2dGraphView,
    WFC_2D_GRAPH_VIEW_SCALE,
};
use graph_view_args_and_gestures::{
    arg_bool, arg_f64, arg_optional_string, arg_string, arg_u64, require_edge, require_fresh,
    require_rule, require_slot, require_tile, wfc2d_load_document_effect,
};

#[path = "🧭️dispatch-and-artifact-editor/🦀️.rs"]
mod dispatch_and_artifact_editor;
pub use dispatch_and_artifact_editor::{dispatch, render_body, render_with_transient};

#[path = "🚀️editor-assembly/🦀️.rs"]
mod editor_assembly;
pub use editor_assembly::create_wfc2d_editor;

//#region 🧪️Tests
//#endregion 🧪️Tests
