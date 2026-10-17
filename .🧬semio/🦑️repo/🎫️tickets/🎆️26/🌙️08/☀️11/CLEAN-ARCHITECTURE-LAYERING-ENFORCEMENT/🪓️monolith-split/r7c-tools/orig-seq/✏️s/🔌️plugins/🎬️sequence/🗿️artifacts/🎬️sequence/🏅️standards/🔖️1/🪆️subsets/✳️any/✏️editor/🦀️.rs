//! 🖥️ Sequence play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum, the
//! manifest stitch, and this app's own typed media I/O surface + plugin registration + editing host
//! (below — constitutional: general, an artifact must never depend on an app, so all three live here
//! rather than under `🗿️artifacts`).
//!
//! Command bodies live in `🎮️commands/*`, window renders in `🎭️modes/✏️edit/🪟️windows/*`, panel trees in
//! `📌️panels/*`, labels in `🦀️terminology.rs`, view state in `🦀️config.rs`. `handle` →
//! `SequenceCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls one
//! `definition()` per node. `SequenceHost` (below) is the UI-editing engine shared by more than one
//! taxonomy node (commands, windows, panels, the wasm bridge) — a helper with exactly one consumer
//! lives in that consumer's own component file instead.

use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
use crate::editor::sequence::commands::connection::{connect_steps, disconnect_steps};
use crate::editor::sequence::commands::example::set_active_example;
use crate::editor::sequence::commands::layout::{reorganize, set_orientation};
use crate::editor::sequence::commands::node_graph::{node_graph_edit, set_viewport};
use crate::editor::sequence::commands::playback::{run_command, stop_command};
use crate::editor::sequence::commands::step::{add_step, add_step_dropped, add_step_to_slot, delete_selection, move_step, remove_step, set_step_collapsed, set_step_params};
use crate::editor::sequence::edit_rules;
use crate::editor::sequence::modes::edit;
use crate::editor::sequence::modes::edit::windows::{compiled, main, script};
use crate::editor::sequence::modes::edit::windows::main::config::SequenceMainWindowConfig;
use crate::editor::sequence::modes::edit::windows::script::transient::SequenceScriptWindowTransient;
use crate::editor::sequence::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::sequence::terminology::sequence_play_labels;
use crate::mutations::SequenceMutation;
use crate::{SequenceCamera, SequenceEdge, SequenceHostSnapshot, SequenceSnapshot, SequenceStep, SequenceWorkingScene, SlotRef, StepParams, SEQUENCE_DOCUMENT_SCHEMA};
use semio_framework_os_infinite::board::schema::layout::{DagLayoutOptions};
use dag::{would_create_cycle, DagHost};
use semio_framework_graph_manifest::PropertyBag;
use imperative_engine::{compile_to_text as imperative_compile_to_text, imperative_catalogue_json, imperative_module_registry, Executor, Path, RunResult, Step};
use infinite_board_port_directed_dag as dag;
use neural_engine::{ChannelSpec, ColdOwner, Dictionary, Registry, RegistryRetirement, SharedRegistry, Value as NeuralValue, ValueRetirement};
use semio_framework_os_infinite_directed_presentation::dag_host_snapshot_to_wire_literal;
use semio_framework_os_infinite_directed_presentation::{DagCamera, DagHostSnapshot, DagHostSnapshotEdge};

use semio_framework_os_infinite_directed_presentation::{DagNodeSpec, EdgeRouteStyle, IoPortSpec, PortShape};
use semio_framework_plugin::app::{ChildEmit,ChildEmitPreparation};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppActionRegistry;
use semio_framework_plugin::AppDefinition;
use semio_framework_plugin::AppIo;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::ContextMenuItemSpec;
use semio_framework_plugin::ContextMenuRequest;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DomainTopology;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::DslValue;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Emit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTopology;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::NoConfigMutation;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::TopologyNode;
use semio_framework_pack_json::{self as json, json};
use semio_framework_pack_json::Value;
use semio_framework_tool_machine::{node_drag_emit, NodeDragEmit, NodeGraphEditRow};
use std::collections::{BTreeMap, HashMap, VecDeque};
use semio_framework_2d::compute::EngineHandles;
use semio_s_artifact_stdio_semio_flow::schema::mutations::SemioFlowMutation;
use semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot;

#[path = "🧱️base/🦀️.rs"]
mod base;
pub use self::base::*;

#[path = "🖥️host/🦀️.rs"]
mod host;
pub use self::host::*;

#[path = "🧵️retained/🦀️.rs"]
mod retained;
pub use self::retained::*;

#[path = "🔁️persistent/🦀️.rs"]
mod persistent;
pub use self::persistent::*;

#[path = "🎞️import/🦀️.rs"]
mod import;
pub use self::import::*;

#[path = "🎹️app/🦀️.rs"]
mod app;
pub use self::app::*;

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must
/// be able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub mod unit_tests;
//#endregion 🧪️UnitTests
