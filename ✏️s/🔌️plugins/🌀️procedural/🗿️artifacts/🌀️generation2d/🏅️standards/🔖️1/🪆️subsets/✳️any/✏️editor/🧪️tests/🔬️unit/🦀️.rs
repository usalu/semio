use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_s_artifact_procedural_2d_editor_commands::add_generation;
use semio_s_artifact_procedural_2d_editor_commands::add_widget;
use semio_s_artifact_procedural_2d_editor_commands::canvas_pointer_down;
use semio_s_artifact_procedural_2d_editor_commands::canvas_pointer_move;
use semio_s_artifact_procedural_2d_editor_commands::canvas_pointer_up;
use semio_s_artifact_procedural_2d_editor_commands::canvas_wheel;
use semio_s_artifact_procedural_2d_editor_commands::connect_media_ports;
use semio_s_artifact_procedural_2d_editor_commands::enter_generate;
use semio_s_artifact_procedural_2d_editor_commands::flow_eval_resolve;
use semio_s_artifact_procedural_2d_editor_commands::flow_eval_tick;
use semio_s_artifact_procedural_2d_editor_commands::generation;
use semio_s_artifact_procedural_2d_editor_commands::move_media_node;
use semio_s_artifact_procedural_2d_node_graph_edit as node_graph_edit;
use semio_s_artifact_procedural_2d_editor_commands::node_graph_viewport;
use semio_s_artifact_procedural_2d_editor_commands::remove_generation;
use semio_s_artifact_procedural_2d_editor_commands::remove_widget;
use semio_s_artifact_procedural_2d_editor_commands::rename_generation;
use semio_s_artifact_procedural_2d_editor_commands::reorganize;
use semio_s_artifact_procedural_2d_editor_commands::select_generation;
use semio_s_artifact_procedural_2d_editor_commands::set_active_example;
use semio_s_artifact_procedural_2d_editor_commands::set_contributions;
use semio_s_artifact_procedural_2d_editor_commands::set_eval_outputs;
use semio_s_artifact_procedural_2d_editor_commands::set_show_mode;
use semio_s_artifact_procedural_2d_editor_commands::update_generation_values;
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfig;
use semio_s_artifact_procedural_2d_mode_edit::windows::flow;
use semio_s_artifact_procedural_2d_mode_edit::windows::preview;
use semio_s_artifact_procedural_2d_mode_edit as edit;
use semio_s_artifact_procedural_2d_mode_generate::windows::form;
use semio_s_artifact_procedural_2d_mode_generate::windows::generations;
use semio_s_artifact_procedural_2d_mode_generate as generate;
use semio_s_artifact_procedural_2d_editor_panels::document;
use semio_s_artifact_procedural_2d_terminology::generation2d_labels;
use semio_s_artifact_procedural_2d_preview_eval::PreviewEvalTarget;
use semio_s_artifact_procedural_2d_model::GENERATION_2D_SCHEMA;
use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::Effect;
use semio_framework_plugin::Fault;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaType;

use semio_s_artifact_procedural_2d_editor::*;
use crate::app_fixture::{app, app_with_registry, close, snapshot_read};
use semio_framework_artifact_flow_flow::Widget;
use semio_framework_plugin::artifact_app_laws::assert_undo_redo_round_trip;
use semio_framework_plugin::PluginApp;

include!("🧰️production-fixtures-and-maintenance-law/🦀️.rs");

include!("🎛️command-surface-laws/🦀️.rs");

include!("📜️manifest-and-cross-cutting-laws/🦀️.rs");

include!("📏️surface-budget-and-context-menu-laws/🦀️.rs");

include!("🔌️port-laws/🦀️.rs");

include!("📇️window-action-archive-and-genesis-laws/🦀️.rs");
