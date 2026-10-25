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
use semio_framework_plugin::{NoConfig, NoConfigMutation};
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
use semio_framework_os_infinite_directed_presentation::DagDrawLod;
use semio_framework_plugin::app::{ChildEmitPreparation, InteractionView};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppActionRegistry;
use semio_framework_plugin::AppDefinition;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::CommandDefinition;
use semio_framework_plugin::ConfigView;
use semio_framework_plugin::ContextMenuItemSpec;
use semio_framework_plugin::ContextMenuRequest;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DomainTopology;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::Effect;
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
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::TopologyNode;
use semio_framework_plugin::WindowMeasure;
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

#[path = "🧵️retained/🦀️.rs"]
pub mod retained;

#[cfg(test)]
#[path = "🫧️transient/🧪️tests/🫧️transient/🦀️.rs"]
mod transient_retirement_tests;

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

#[path = "📤️host-only-retained-route/🦀️.rs"]
mod host_only_retained_route;
pub use host_only_retained_route::{
    flow_scalar_command_view, FlowHostEffectJobFactory, FLOW_HOST_ONLY_TOOL_IDS,
};
use host_only_retained_route::{FlowHostEffectPayload, flow_build_host_only_job};

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
    FlowInstanceOperationOwner, FLOW_CONTRIBUTIONS_TOOL_IDS,
};
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
semio_framework_plugin::app_commands! {
    /// 🎯️ `FlowPlayApp::Command` — the SOLE dispatch surface for flow's own behavior, assembled from the
    /// `🎮️commands/*` payload modules. Each row states BOTH the manifest action id (`command_id()`, the
    /// camelCase id declared in `🔖️Manifest` below) and the `dsl` wire keyword (the kebab-case
    /// `#[dsl(key = ..)]` the binary/text codec uses) — they are genuinely different vocabularies, and
    /// is safe, reordering is a wire-format break.**
    ///
    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `setSelection`/`clearSelection`/
    /// `selectAll`/`selectNode`/`nodeGraphSelect`/`nodeGraphHover`/`graphPointerDown` are deleted — the
    /// framework auto-injects `interactionSelect`/`interactionHover`/`clearSelection`/`selectAll`/
    /// `setSelectionMode`/`setInteractionGranularity` for the declared "graph" domain instead (see
    /// `🔖️Manifest`). `deleteSelection`/`focusSelection`/`nodeGraphEdit`/`spotlightCommit` read that
    /// domain's live selection via `InteractionView` — `FlowPlayApp::handle` routes them through their
    /// own `apply` (this macro's generated `dispatch(doc, cfg, session)` has no `interaction` slot).
    pub enum FlowCommand for FlowSnapshot, FlowMutation, NoConfig, NoConfigMutation, ctx = FlowEvalSession {
        "addWidget" as "add-widget" => add_widget::AddWidget,
        "removeWidget" as "remove-widget" => remove_widget::RemoveWidget,
        "duplicateWidget" as "duplicate-widget" => duplicate_widget::DuplicateWidget,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "disconnect" as "disconnect" => disconnect::Disconnect,
        "connectMediaPorts" as "connect-media-ports" => connect_media_ports::ConnectMediaPorts,
        "moveMediaNode" as "move-media-node" => move_media_node::MoveMediaNode,
        "reorganize" as "reorganize" => reorganize::Reorganize,
        "patchFlowWidgets" as "patch-flow-widgets" => patch_flow_widgets::PatchFlowWidgets,
        "renameFlowWidget" as "rename-flow-widget" => rename_flow_widget::RenameFlowWidget,
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "spotlightCommit" as "spotlight-commit" => spotlight_commit::SpotlightCommit,
        "runExtensionAction" as "run-extension-action" => run_extension_action::RunExtensionAction,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
        "evaluate" as "evaluate" => evaluate::Evaluate,
        "focusSelection" as "focus-selection" => focus_selection::FocusSelection,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "setLodMode" as "set-lod-mode" => set_lod_mode::SetLodMode,
        "setProximityDistance" as "set-proximity-distance" => set_proximity_distance::SetProximityDistance,
        "setGridVisible" as "set-grid-visible" => set_grid_visible::SetGridVisible,
        "setGridSnapEnabled" as "set-grid-snap-enabled" => set_grid_snap_enabled::SetGridSnapEnabled,
        "setGridFactor" as "set-grid-factor" => set_grid_factor::SetGridFactor,
        "contextMenuAt" as "context-menu-at" => context_menu_at::ContextMenuAt,
        "setPreviewOff" as "set-preview-off" => set_preview_off::SetPreviewOff,
        "openSpotlight" as "open-spotlight" => open_spotlight::OpenSpotlight,
        "replaceImage" as "replace-image" => replace_image::ReplaceImage,
        "setCatalogueSections" as "set-catalogue-sections" => set_catalogue_sections::SetCatalogueSections,
        "toggleExtension" as "toggle-extension" => toggle_extension::ToggleExtension,
        "addGeneration" as "add-generation" => add_generation::AddGeneration,
        "removeGeneration" as "remove-generation" => remove_generation::RemoveGeneration,
        "selectGeneration" as "select-generation" => select_generation::SelectGeneration,
        "renameGeneration" as "rename-generation" => rename_generation::RenameGeneration,
        "updateGenerationValues" as "update-generation-values" => update_generation_values::UpdateGenerationValues,
        "flowEvalTick" as "flow-eval-tick" => flow_eval_tick::FlowEvalTick,
        "flowEvalResolve" as "flow-eval-resolve" => flow_eval_resolve::FlowEvalResolve,
        "setContributions" as "set-contributions" => set_contributions::SetContributions,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
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
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::flow::create_flow_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "computation.flow".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Flow", "Fluss"),
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
