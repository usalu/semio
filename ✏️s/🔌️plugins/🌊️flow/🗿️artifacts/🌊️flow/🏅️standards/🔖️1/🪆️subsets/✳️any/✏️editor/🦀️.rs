//! 🖥️ Flow play app — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, chrome measures in those windows' `☑️options/*`, panel trees in `📌️panels/*`,
//! labels in `🗣️terminology/🦀️.rs`, view state in `🎚️config/🦀️.rs`, plugin registration
//! and `FlowHost` bridging (below — constitutional: general, an artifact must never depend on an app, so
//! both live here rather than under `🗿️artifacts`).
//! This file is a routing table: `handle` → `FlowCommand::dispatch`, `render` → body-key → node, and a
//! `🔖️Manifest` region that calls one `definition()` per node.

use semio_framework_artifact_playbook_playbook::flow_forms::PlaybookFlowFormProducer;
use crate::editor::flow::commands::{
    add_widget, connect_media_ports, context_menu_at, delete_selection, disconnect, duplicate_widget, evaluate, flow_eval_resolve, flow_eval_tick, focus_selection, move_media_node, node_graph_edit, node_graph_viewport,
    open_spotlight, patch_flow_widgets, remove_widget, rename_flow_widget, reorganize, replace_image, run_extension_action, set_active_example, set_catalogue_sections, set_grid_factor, set_grid_snap_enabled, set_grid_visible, set_lod_mode,
    set_contributions, set_preview_off, set_proximity_distance, spotlight_commit, toggle_extension,
};
use crate::editor::flow::modes::edit::windows::main::config::FlowMainWindowConfig;
use semio_plugin::{NoConfig, NoConfigMutation};
use crate::editor::flow::modes::edit::windows::{compiled, main};
use crate::editor::flow::modes::generate::commands::{add_generation, remove_generation, rename_generation, select_generation, update_generation_values};
use crate::editor::flow::modes::generate::windows::{form, generations, preview};
use crate::editor::flow::modes::{edit, generate};
use crate::editor::flow::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel};
use crate::editor::flow::presence::{FlowPresence, FlowPresenceMutation};
use crate::editor::flow::terminology::{flow_play_labels, FlowPlayLabels};
use crate::editor::flow::editor_vocabulary::{flow_graph_edge_target_id, flow_graph_node_target_id, flow_graph_selection_domains, FLOW_GRAPH_EDGE_TARGET_PREFIX, FLOW_GRAPH_NODE_TARGET_PREFIX, FLOW_GRAPH_OPERATION_RAW_BYTES, FLOW_INTERACTION_GRAPH, FLOW_PLAY_APP_ID, FLOW_PLAY_WINDOW_MAIN, FLOW_STORE_MAX_MUTATION_ITEMS, FLOW_STORE_MAX_SCENE_ITEMS, FLOW_STORE_MAX_TEXT_BYTES};
#[cfg(test)]
use crate::editor::flow::editor_vocabulary::{FLOW_PLAY_WINDOW_GENERATIONS, FLOW_PLAY_WINDOW_GENERATE_FORM, FLOW_PLAY_WINDOW_GENERATE_PREVIEW};
use crate::editor::flow::host::{cold_grant, flow_content_leaves_emit, flow_removal_leaves, focus_selection_camera, seed_host_catalogue, value_fault};
#[cfg(test)]
use crate::editor::flow::host::{host_from_snapshot, sync_host_selection_domains};
use crate::{FlowMutation, FlowSnapshot, FlowWorkingScene, FLOW_DOCUMENT_SCHEMA};
use flow::{flow_host_with_session, FlowEvalSession, FlowHost, FLOW_LOD_MODE_AUTOMATIC};
use semio_framework_artifact_flow_flow::{CameraJson, Widget};
#[cfg(test)] use semio_framework_artifact_flow_flow::{SynapseSpec, WidgetLayout};
use semio_os_infinite_directed_presentation::DagDrawLod;
use semio_plugin::app::{ChildEmitPreparation, InteractionView};
use semio_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_plugin::ActionArgDef;
use semio_plugin::ActionArgOption;
use semio_plugin::ActionDefinition;
use semio_plugin::ActionKind;
use semio_plugin::AppActionRegistry;
use semio_plugin::AppDefinition;
use semio_plugin::ArtifactEditor;
use semio_plugin::ArtifactView;
use semio_plugin::CommandDefinition;
use semio_plugin::ConfigView;
use semio_plugin::ContextMenuItemSpec;
use semio_plugin::ContextMenuRequest;
use {semio_framework_artifact_reference::Dialect};
use semio_plugin::DomainTopology;
use semio_plugin::DraftView;
use semio_plugin::Editor;
use semio_plugin::Effect;
use semio_plugin::Emit;
use semio_plugin::Fault;
use semio_plugin::GranularityDefinition;
use semio_plugin::HierarchyProvider;
use semio_plugin::HoverSpec;
use semio_plugin::InteractionDefinition;
use semio_plugin::InteractionRef;
use semio_plugin::InteractionTopology;
use semio_ui_locale::Label;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::MergeMode;
use semio_plugin::NoDraft;
use semio_plugin::NoDraftMutation;
use semio_plugin::SelectionMethod;
use semio_plugin::SelectionMode;
use semio_plugin::SelectionSpec;
use semio_plugin::TopologyNode;
use semio_plugin::WindowMeasure;
#[cfg(test)]
use serde_json::json;
use semio_framework::kernel::UiDirtyScope;
use semio_s_artifact_stdio_semio_flow::schema::mutations::{insert_edge, insert_node, SemioFlowMutation};
use semio_s_artifact_stdio_semio_flow::schema::snapshot::{FlowEdge, PortRef, SemioFlowSnapshot};
#[cfg(test)] use semio_s_artifact_stdio_semio_flow::schema::snapshot::FlowNode;
#[cfg(test)]
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use semio_framework_2d_compute::EngineHandles;

pub use semio_s_artifact_flow_flow_editor_retained::retained;

#[cfg(test)]
#[path = "🫧️transient/🧪️tests/🫧️transient/🦀️.rs"]
mod transient_retirement_tests;

#[cfg(test)]
#[path = "🧵️retained/🧮️evaluation/🧪️mounted.rs"]
mod retained_evaluation_mounted_tests;

#[cfg(test)]
#[path = "🧵️retained/📥️original/🧪️routes.rs"]
mod retained_original_route_tests;

//#region 🔖️Constants
pub use catalogue_panel::FLOW_PLAY_BODY_CATALOGUE;
pub use compiled::FLOW_PLAY_BODY_COMPILED;
pub use document_panel::FLOW_PLAY_BODY_ARTIFACT;
pub use form::FLOW_PLAY_BODY_GENERATE_FORM;
pub use generations::FLOW_PLAY_BODY_GENERATIONS;
pub use inspection_panel::FLOW_PLAY_BODY_INSPECTOR;
pub use main::FLOW_PLAY_BODY_MAIN;
pub use preview::FLOW_PLAY_BODY_GENERATE_PREVIEW;

#[path = "🙈️internal-actions-and-context-menu/🦀️.rs"]
mod internal_actions_and_context_menu;
use internal_actions_and_context_menu::{
    flow_content_child, flow_context_menu_items, flow_internal_action,
};

#[path = "🗃️direct-store-emit-and-work/🦀️.rs"]
mod direct_store_emit_and_work;
pub use direct_store_emit_and_work::{FlowDirectStoreWork, FLOW_DIRECT_STORE_TOOL_IDS};
use direct_store_emit_and_work::{
    duplicate_edge_id, duplicate_widget_id, flow_direct_store_emit, generation_window_transient,
    FLOW_DIRECT_STORE_RAW_BYTES,
};

#[path = "🧰️direct-store-command-work/🦀️.rs"]
mod direct_store_command_work;

#[path = "🏭️direct-store-factory-and-child-group-route/🦀️.rs"]
mod direct_store_factory_and_child_group_route;
use direct_store_factory_and_child_group_route::{
    flow_direct_store_contract, FLOW_CHILD_GROUP_RAW_BYTES,
};
pub use direct_store_factory_and_child_group_route::{
    FlowChildGroupJobFactory, FlowChildGroupWork, FlowDirectStoreJobFactory,
    FLOW_CHILD_GROUP_TOOL_IDS,
};

use direct_store_factory_and_child_group_route::flow_build_retained_job;

#[path = "📤️host-only-retained-route/🦀️.rs"]
mod host_only_retained_route;
pub use host_only_retained_route::{
    flow_scalar_command_view, FlowHostEffectJobFactory, FlowEvaluationJobFactory, FLOW_HOST_ONLY_TOOL_IDS, FLOW_EVALUATION_TOOL_IDS,
};
use host_only_retained_route::{FlowHostEffectPayload,flow_build_host_job,FlowEvaluationJobFactoryProofs};

#[path = "🕸️graph-operation-route/🦀️.rs"]
mod graph_operation_route;
pub use graph_operation_route::{
    flow_graph_operation_contract, FlowGraphOperationJobFactory, FLOW_GRAPH_OPERATION_CAPACITY,
    FLOW_GRAPH_OPERATION_TOOL_IDS,
};
use graph_operation_route::{
    FlowChildGroupJobFactoryProofs, FlowDirectStoreJobFactoryProofs,
    FlowGraphOperationJobFactoryProofs, FlowGraphOperationWork, FlowHostEffectJobFactoryProofs,
};

#[path = "🧩️contributions-route-and-instance-owner/🦀️.rs"]
mod contributions_route_and_instance_owner;
pub use contributions_route_and_instance_owner::{
    flow_contributions_registry_receipt, FlowContributionsJobFactory, FlowContributionsWork,
    FLOW_CONTRIBUTIONS_TOOL_IDS,
};
pub use semio_s_artifact_flow_flow_editor_retained::FlowInstanceOperationOwner;
use contributions_route_and_instance_owner::{
    FlowContributionsJobFactoryProofs, FLOW_CONTRIBUTIONS_RAW_BYTES,
};

#[path = "🎛️flow-play-app/🦀️.rs"]
mod flow_play_app;
pub use flow_play_app::FlowPlayApp;

#[path = "🚀️app-assembly/🦀️.rs"]
mod app_assembly;
pub use app_assembly::create_flow_app;

//#endregion 🔖️Constants

//#region 🔖️Commands
pub use semio_s_artifact_flow_flow_editor_retained::FlowCommand;
//#endregion 🔖️Commands

//#region 🧵️HostOnlyRetainedRoutes

#[path = "🧵️retained/📤️effect/🦀️.rs"]
mod original_host_effect;

//#endregion 🧵️HostOnlyRetainedRoutes

//#region 🧪️UnitTests

//#endregion 🧪️UnitTests

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🪢️TaxonomyMounts

//#endregion 🪢️TaxonomyMounts

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use crate::{FLOW_DOCUMENT_SCHEMA};
use semio_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::flow::create_flow_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "computation.flow".into(),
        label: semio_ui_locale::LocalizedLabel::native("Flow", "Fluss"),
        source_format: "flow.artifact".into(),
        component_kind: "flow".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Flow },
        schema: FLOW_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems5

use semio_s_artifact_flow_flow_editor_publication as child_publication;
