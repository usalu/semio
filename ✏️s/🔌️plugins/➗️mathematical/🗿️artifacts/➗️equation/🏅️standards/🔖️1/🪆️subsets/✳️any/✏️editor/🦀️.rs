//! 🧮️ Equation editor — `EquationPlayApp`'s `ArtifactEditor` impl (dispatch-only, ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1), the aggregated command enum and
//! the manifest stitch. B1: the pure-trait pilot for this plugin — `EquationPlayApp` is a unit
//! struct; the former `MathPlayRuntime` app-struct `RefCell` (the node-graph viewport camera) now lives in
//! `crate::editor::equation::config::EquationGraphWindowConfig`, written via `EquationGraphWindowConfigMutation`s (real
//! `backwards`, no ad hoc inverse tracking); every action dispatches through the single typed
//! `EquationCommand` channel via `ArtifactEditor::handle`.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/✏️edit/🪟️windows/*`, view state in `🎚️config/🦀️.rs`. Shared compute with more than one
//! consumer across the taxonomy tree (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES dissolved
//! the former artifact-tree `⚙️engine`) lives HERE — `🔖️Io`, `🔖️Scene`, `🔖️GraphAlgorithms`, `🔖️Geometry` — since
//! an artifact is a `🧬️schema` + `🚪️io` system only, never an engine; behaviour belongs to the app.
//! This file is a routing table: `handle` → `EquationCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.
//!
//! The sibling read-only surface (`👁️viewer/🦀️.rs`) never imports from this module — see
//! that file's own doc header.

use crate::editor::equation::commands::edit_equation;
use crate::editor::equation::commands::edit_points;
use crate::editor::equation::commands::node_graph_edit::EquationEditOperation;
use crate::editor::equation::commands::{add_node, node_graph_edit, node_graph_viewport, set_active_example, set_algorithm, set_directed};
use crate::editor::equation::modes::edit;
use crate::editor::equation::modes::edit::windows::graph::config::{EquationCamera, EquationGraphWindowConfigMutation, EquationGraphWindowConfigOwner};
use crate::editor::equation::modes::edit::windows::{geometry as geometry_window, graph as graph_window};
use crate::op::EquationMutation;
use crate::{EquationGeometry, EquationGraph, EquationSnapshot, EQUATION_DIALECT, MATH_DOCUMENT_SCHEMA};
use semio_framework_pack_json::Value;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::plugin_app_close_prelude::ArtifactDisposal;
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
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
use std::sync::Arc;
use pack::snapshot::ArtifactPack;
use semio_framework_2d_compute::EngineHandles;
use ui_wgpu::wgpu::{NodeGraphEdgeRecord, NodeGraphNodeRecord, NodeGraphPortRecord};

//#region 🔖️Constants

#[path = "📄️document-io-algorithm-and-geometry-json/🦀️.rs"]
mod document_io_algorithm_and_geometry_json;
pub use document_io_algorithm_and_geometry_json::{
    algorithm_overlay, equation_io, geometry_layers_json, reset_equation_document_effect,
    workflow_json, EQUATION_EDGE_SOURCE_PORT, EQUATION_EDGE_TARGET_PORT, MATH_APP_ID,
};

#[path = "📏️retained-command-limits-and-work-state/🦀️.rs"]
mod retained_command_limits_and_work_state;
pub use retained_command_limits_and_work_state::{
    equation_command_extent, equation_edit_preflight, equation_operation_identity,
    EquationRetainedCommandWork, EQUATION_DEFAULT_EDIT_OPERATIONS, EQUATION_MAX_EDGES,
    EQUATION_MAX_EDIT_JSON_BYTES, EQUATION_MAX_EDIT_OPERATIONS, EQUATION_MAX_NODES,
    EQUATION_MAX_POINTS, EQUATION_TOOL_IDS,
};
use retained_command_limits_and_work_state::{
    equation_contract, equation_leaf_retained_bytes, EquationCloseUnit, EquationWorkPhase,
    EQUATION_PUBLICATION_CONTRACTS, EQUATION_RETAINED_PAYLOAD_SCHEMA, EQUATION_RETAINED_RAW_BYTES,
    EQUATION_RETAINED_WORK_ITEMS,
};
#[cfg(test)]
pub(crate) use retained_command_limits_and_work_state::EQUATION_MAX_DELETE_IDS;
pub(crate) use retained_command_limits_and_work_state::EQUATION_MAX_TEXT_BYTES;

#[path = "🛠️retained-command-work-and-job-factory/🦀️.rs"]
mod retained_command_work_and_job_factory;
pub use retained_command_work_and_job_factory::EquationCommandJobFactory;

#[path = "📬️store-preparation/🦀️.rs"]
mod store_preparation;
pub use store_preparation::EquationStorePreparationFactory;

#[path = "🎛️play-app-editor-and-constructor/🦀️.rs"]
mod play_app_editor_and_constructor;
pub use play_app_editor_and_constructor::{create_equation_app, EquationPlayApp};

pub use geometry_window::MATH_PLAY_BODY_GEOMETRY;
pub use graph_window::MATH_PLAY_BODY_GRAPH;
//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `EquationPlayApp::Command` — the SOLE dispatch surface for equation's own behavior,
    /// assembled from the `🎮️commands/*` payload modules. Each row states BOTH the manifest action id
    /// (`command_id()`, the camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the
    /// kebab-case `#[dsl(key = ..)]` the binary/text codec uses) — they are genuinely different
    /// ordinal: appending is safe, reordering is a wire-format break.**
    pub enum EquationCommand for EquationSnapshot, EquationMutation, NoConfig, NoConfigMutation {
        "editEquation" as "edit-equation" => edit_equation::EditEquation,
        "setAlgorithm" as "set-algorithm" => set_algorithm::SetAlgorithm,
        "setDirected" as "set-directed" => set_directed::SetDirected,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "editPoints" as "edit-points" => edit_points::EditPoints,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
        "addNode" as "add-node" => add_node::AddNode,
    }
}
//#endregion 🔖️Commands

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;
//#endregion 🧪️UnitTests

//#region 🪢️TaxonomyMounts

//#endregion 🪢️TaxonomyMounts

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use crate::{MATH_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::equation::create_equation_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "computation.equation".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Equation", "Gleichung"),
        source_format: MATH_DOCUMENT_SCHEMA.into(),
        component_kind: "equation".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Value },
        schema: MATH_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems5
