//! 🎲️ Generation2d editor surface — the `ArtifactEditor` impl (dispatch-only), the aggregated command
//! enum and the manifest stitch (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.1).
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's own inferences. This file is a routing table: `handle`
//! → `Generation2dCommand::dispatch`, `render` → body-key → node, and a `🔖️Manifest` region that calls
//! one passthrough per node.
#![allow(clippy::result_large_err)]
use semio_framework_job_publication::{RetainedJobPublication,JobPublicationKind};
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

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
use semio_s_artifact_procedural_2d_editor_config::Generation2dConfigMutation;
use semio_s_artifact_procedural_2d_editor_config::SetSelectedGenerationSetting;
use semio_s_artifact_procedural_2d_editor_config::SetShowModeSetting;
use semio_s_artifact_procedural_2d_mode_edit::windows::flow as flow_window;
use semio_s_artifact_procedural_2d_mode_edit::windows::preview as edit_preview;
use semio_s_artifact_procedural_2d_mode_generate::windows::form;
use semio_s_artifact_procedural_2d_mode_generate::windows::generations;
use semio_s_artifact_procedural_2d_mode_generate::windows::preview as generate_preview;
use semio_s_artifact_procedural_2d_mode_edit as edit;
use semio_s_artifact_procedural_2d_mode_generate as generate;
use semio_s_artifact_procedural_2d_editor_panels::catalogue as catalogue_panel;
use semio_s_artifact_procedural_2d_editor_panels::document as document_panel;
use semio_s_artifact_procedural_2d_editor_panels::inspection as inspection_panel;
use semio_s_artifact_procedural_2d_terminology::generation2d_labels;
use semio_s_artifact_procedural_2d_terminology::Generation2dLabels;
use semio_s_artifact_procedural_2d_editor_transient::Generation2dTransient;
use semio_s_artifact_procedural_2d_editor_transient::Generation2dTransientMutation;
use semio_s_artifact_procedural_2d_editor_transient::SetGenerationPreview;
use semio_s_artifact_procedural_2d_mutations::Generation2dMutation;
use semio_s_artifact_procedural_2d_model::artifact_kind;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_s_artifact_procedural_2d_model::GENERATION2D_DIALECT;
use semio_s_artifact_procedural_2d_model::GENERATION_2D_SCHEMA;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_framework_job::{InteractiveJob, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, StepContext};
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, ArtifactRetainedWorkCapacity};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactReservedJob;
use semio_framework_plugin::ArtifactReservedToolInput;
use semio_framework_plugin::ArtifactReservedToolJob;
use semio_framework_plugin::ArtifactReservedToolJobRequest;
use semio_framework_plugin::ArtifactToolCompletion;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::CommandDefinition;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DomainTopology;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::Effect;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::FaultCode;
use semio_framework_plugin::FaultOrigin;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTopology;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::ToolRunJob;
use semio_framework_plugin::ToolRunJobPurpose;
use semio_framework_plugin::ToolRunJobRequest;
use semio_framework_plugin::TopologyNode;
use semio_s_artifact_procedural_2d_preview_eval::PreviewEvalRunLink;
use semio_s_artifact_procedural_2d_preview_eval::PreviewEvalRunOwner;
use semio_s_artifact_procedural_2d_preview_eval::PreviewEvalSessions;
use semio_s_artifact_procedural_2d_preview_eval::PreviewEvalTarget;
use semio_s_artifact_procedural_2d_preview_eval::PreviewEvalWindow;
use semio_s_artifact_procedural_2d_editor_commands::flow_eval_tick::generation2d_preview_target;
use semio_s_artifact_procedural_2d_editor_commands::flow_eval_tick::generation2d_preview_windows;
use semio_framework_2d::compute::EngineHandles;

#[path = "🛰️owner-and-io/🦀️.rs"]
mod owner_and_io;
pub use owner_and_io::*;

#[path = "🧵️retained-commands/🦀️.rs"]
mod retained_commands;
pub use retained_commands::*;

#[path = "🎁️contributions-route/🦀️.rs"]
mod contributions_route;
pub use contributions_route::*;

#[path = "📬️store-preparation/🦀️.rs"]
mod store_preparation;
use store_preparation::*;

#[path = "🎞️reserved-import/🦀️.rs"]
mod reserved_import;
use reserved_import::*;

#[path = "🕹️play-app-editor/🦀️.rs"]
mod play_app_editor;
pub use play_app_editor::*;

#[path = "🖱️graph-interaction-and-render/🦀️.rs"]
mod graph_interaction_and_render;
pub use graph_interaction_and_render::*;

//#endregion 🔖️Constants

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `Generation2dPlayApp::Command` — the SOLE dispatch surface for generation2d's own behavior.
    /// Each row states BOTH the manifest action id (`command_id()`) and the `dsl` wire keyword
    /// order is the binary variant ordinal: appending is safe, reordering is a wire-format break.**
    pub enum Generation2dCommand for Generation2dSnapshot, Generation2dMutation, Generation2dConfig, Generation2dConfigMutation, ctx = FlowEvalSession {
        "nodeGraphEdit" as "node-graph-edit" => node_graph_edit::NodeGraphEdit,
        "moveMediaNode" as "move-media-node" => move_media_node::MoveMediaNode,
        "addWidget" as "add-widget" => add_widget::AddWidget,
        "removeWidget" as "remove-widget" => remove_widget::RemoveWidget,
        "connectMediaPorts" as "connect-media-ports" => connect_media_ports::ConnectMediaPorts,
        "reorganize" as "reorganize" => reorganize::Reorganize,
        "addGeneration" as "add-generation" => add_generation::AddGeneration,
        "removeGeneration" as "remove-generation" => remove_generation::RemoveGeneration,
        "renameGeneration" as "rename-generation" => rename_generation::RenameGeneration,
        "updateGenerationValues" as "update-generation-values" => update_generation_values::UpdateGenerationValues,
        "nodeGraphViewport" as "node-graph-viewport" => node_graph_viewport::NodeGraphViewport,
        "setShowMode" as "set-show-mode" => set_show_mode::SetShowMode,
        "generate" as "generate" => enter_generate::Generate,
        "setEvalOutputs" as "set-eval-outputs" => set_eval_outputs::SetEvalOutputs,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp,
        "canvasWheel" as "canvas-wheel" => canvas_wheel::CanvasWheel,
        "selectGeneration" as "select-generation" => select_generation::SelectGeneration,
        "flowEvalTick" as "flow-eval-tick" => flow_eval_tick::FlowEvalTick,
        "flowEvalResolve" as "flow-eval-resolve" => flow_eval_resolve::FlowEvalResolve,
        "setContributions" as "set-contributions" => set_contributions::SetContributions,
        "setActiveExample" as "active-example" => set_active_example::SetActiveExample,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

//#region 📬️ConfigStorePreparation

//#region 🧪️PreparationLaws
#[cfg(test)]
#[path = "🧪️tests/🔬️generation2d-config-preparation-laws/🦀️.rs"]
mod generation2d_config_preparation_laws;
//#endregion 🧪️PreparationLaws
//#endregion 📬️ConfigStorePreparation

//#region 🔖️Manifest
/// 🧬️ Routes a generation command of the editor's command enum to the shared generation transition.
pub fn prepare_command(command: &Generation2dCommand, doc: &ArtifactView<'_, Generation2dSnapshot>, cfg: &ConfigView<'_, Generation2dConfig>) -> Option<generation::Generation2dGenerationCommandResult> {
    match command {
        Generation2dCommand::AddGeneration(_) => Some(generation::handle_generation("addGeneration", None, doc, cfg)),
        Generation2dCommand::RemoveGeneration(payload) => {
            let args = semio_framework_value::DslValue::object([("id".into(), semio_framework_value::DslValue::String(payload.id.clone()))]);
            Some(generation::handle_generation("removeGeneration", Some(&args), doc, cfg))
        }
        Generation2dCommand::RenameGeneration(payload) => {
            let args = semio_framework_value::DslValue::object([("id".into(), semio_framework_value::DslValue::String(payload.id.clone())), ("name".into(), semio_framework_value::DslValue::String(payload.name.clone()))]);
            Some(generation::handle_generation("renameGeneration", Some(&args), doc, cfg))
        }
        Generation2dCommand::UpdateGenerationValues(payload) => {
            let args = semio_framework_value::DslValue::object([
                ("generationId".into(), payload.generation_id.clone().map_or(semio_framework_value::DslValue::Null, semio_framework_value::DslValue::String)),
                ("questionId".into(), semio_framework_value::DslValue::String(payload.question_id.clone())),
                ("value".into(), payload.value.clone()),
            ]);
            Some(generation::handle_generation("updateGenerationValues", Some(&args), doc, cfg))
        }
        Generation2dCommand::SelectGeneration(payload) => {
            let args = semio_framework_value::DslValue::object([("id".into(), payload.id.clone().map_or(semio_framework_value::DslValue::Null, semio_framework_value::DslValue::String))]);
            Some(generation::handle_generation("selectGeneration", Some(&args), doc, cfg))
        }
        _ => None,
    }
}

pub fn create_generation2d_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(GENERATION2D_DIALECT)
        .document(["semio", "procedural", "2d"])
        .artifact_kind(artifact_kind())
        .icon_id("generation2d")
        .mode_def(edit::definition())
        .mode_def(generate::definition())
        .mode_layout(generate::GENERATION2D_PLAY_MODE_GENERATE, generate::GENERATION2D_PLAY_LAYOUT_GENERATE)
        .default_mode_id(edit::GENERATION2D_PLAY_MODE_EDIT)
        .window_kind_def(flow_window::definition())
        .window_kind_def(edit_preview::definition())
        .window_kind_def(generations::definition())
        .window_kind_def(form::definition())
        .window_kind_def(generate_preview::definition())
        // ⏯️ The read-only preview evaluation run: declared so the framework owns its lifecycle and
        // injects its reserved actions; both modes reference it because both mount a preview that starts it.
        .tool(semio_s_artifact_procedural_2d_preview_eval::preview_eval_tool_definition())
        .default_layout(edit::layout())
        .named_layout(generate::layout())
        .panel_tab_def(document_panel::definition())
        .panel_tab_def(catalogue_panel::definition())
        .panel_tab_def(inspection_panel::definition())
        // ✏️ Document-mutating operations — dispatched as VCS operations with a true inverse.
        // 🗂️ Referenced by `Generation2dPlayApp::context_menu` — categorized for grouped-context-menu disclosure.
        .action_with(categorized_action("nodeGraphEdit", LocalizedLabel::native("Edit Graph", "Graph bearbeiten"), ActionKind::Mutation, "selection"))
        .mutation("moveMediaNode", LocalizedLabel::native("Move Node", "Knoten verschieben"))
        .action_with(categorized_action("addWidget", LocalizedLabel::native("Add Widget", "Element hinzufügen"), ActionKind::Mutation, "create"))
        .mutation("removeWidget", LocalizedLabel::native("Remove Widget", "Element entfernen"))
        .action_destructive("removeWidget")
        .mutation("connectMediaPorts", LocalizedLabel::native("Connect Ports", "Ports verbinden"))
        .action_with(categorized_action("reorganize", LocalizedLabel::native("Reorganize", "Neu anordnen"), ActionKind::Mutation, "transform"))
        .action_with(categorized_action("addGeneration", LocalizedLabel::native("Add Generation", "Generation hinzufügen"), ActionKind::Mutation, "create"))
        .mutation("removeGeneration", LocalizedLabel::native("Remove Generation", "Generation entfernen"))
        .action_destructive("removeGeneration")
        .mutation("renameGeneration", LocalizedLabel::native("Rename Generation", "Generation umbenennen"))
        .mutation("updateGenerationValues", LocalizedLabel::native("Update Generation Values", "Generationswerte aktualisieren"))
        // 👁️ View actions — exact-window viewport, app show-mode display toggle, and evaluation scratch
        // (emit no operations). Selection/hover are the framework's `graph` interaction domain now
        // (`.interaction(...)` below) — the six framework verbs auto-inject.
        .action_with(ActionDefinition::new("nodeGraphViewport", LocalizedLabel::native("Set Viewport", "Ansicht festlegen"), ActionKind::View, "camera"))
        .action_with(categorized_action("setShowMode", LocalizedLabel::native("Set Show Mode", "Anzeigemodus festlegen"), ActionKind::View, "mode"))
        .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Beispiel setzen"))
        .action_with(categorized_action("generate", LocalizedLabel::native("Generate", "Generieren"), ActionKind::View, "actions"))
        .view_action("setEvalOutputs", LocalizedLabel::native("Set Eval Outputs", "Auswertungsausgaben festlegen"))
        .action_with(ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Canvas-Zeiger gedrückt"), ActionKind::View, "mouse-pointer"))
        .action_audience("canvasPointerDown", semio_framework_plugin::CapabilityAudience::Input)
        .action_with(ActionDefinition::new("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Canvas-Zeiger bewegt"), ActionKind::View, "mouse-pointer"))
        .action_audience("canvasPointerMove", semio_framework_plugin::CapabilityAudience::Input)
        .action_with(ActionDefinition::new("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Canvas-Zeiger losgelassen"), ActionKind::View, "mouse-pointer"))
        .action_audience("canvasPointerUp", semio_framework_plugin::CapabilityAudience::Input)
        .view_action("canvasWheel", LocalizedLabel::native("Canvas Wheel", "Canvas-Mausrad"))
        .action_audience("canvasWheel", semio_framework_plugin::CapabilityAudience::Input)
        .action_with(categorized_action("selectGeneration", LocalizedLabel::native("Select Generation", "Generation auswählen"), ActionKind::View, "methods"))
        .action_interactive_job("nodeGraphEdit", InteractiveJobClassification::Migrated)
        .action_interactive_job("moveMediaNode", InteractiveJobClassification::Migrated)
        .action_interactive_job("addWidget", InteractiveJobClassification::Migrated)
        .action_interactive_job("removeWidget", InteractiveJobClassification::Migrated)
        .action_interactive_job("connectMediaPorts", InteractiveJobClassification::Migrated)
        .action_interactive_job("reorganize", InteractiveJobClassification::Migrated)
        .action_interactive_job("addGeneration", InteractiveJobClassification::Migrated)
        .action_interactive_job("removeGeneration", InteractiveJobClassification::Migrated)
        .action_interactive_job("renameGeneration", InteractiveJobClassification::Migrated)
        .action_interactive_job("updateGenerationValues", InteractiveJobClassification::Migrated)
        .action_interactive_job("nodeGraphViewport", InteractiveJobClassification::Migrated)
        .action_interactive_job("setShowMode", InteractiveJobClassification::Migrated)
        .action_interactive_job("setActiveExample", InteractiveJobClassification::Migrated)
        .action_destructive("setActiveExample")
        .action_interactive_job("generate", InteractiveJobClassification::Migrated)
        .action_interactive_job("setEvalOutputs", InteractiveJobClassification::Migrated)
        .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
        .action_interactive_job("canvasPointerMove", InteractiveJobClassification::Migrated)
        .action_interactive_job("canvasPointerUp", InteractiveJobClassification::Migrated)
        .action_interactive_job("canvasWheel", InteractiveJobClassification::Migrated)
        .action_interactive_job("selectGeneration", InteractiveJobClassification::Migrated)
        .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalTick", LocalizedLabel::native("Evaluate Flow Tick", "Flow-Auswertungsschritt"), "runtime", ActionKind::View) }))
        .command(migrated_command(CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("flowEvalResolve", LocalizedLabel::native("Resolve Flow Evaluation", "Flow-Auswertung aufnehmen"), "runtime", ActionKind::View) }))
        .action_interactive_job("flowEvalTick", InteractiveJobClassification::Migrated)
        .action_interactive_job("flowEvalResolve", InteractiveJobClassification::Migrated)
        .command(migrated_command(CommandDefinition {
            in_palette: false,
            ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([
                ActionArgDef::text("json", LocalizedLabel::native("Contributions Page", "Beiträge-Seite")),
                ActionArgDef::text("page", LocalizedLabel::native("Page", "Seite")),
                ActionArgDef::text("pageCount", LocalizedLabel::native("Page Count", "Seitenanzahl")),
            ])
        }))
        .action_interactive_job("setContributions", InteractiveJobClassification::Migrated)
        // 📝️ Staged argument form for the palette-visible add-widget action (default materialized host-side).
        .action_args("addWidget", vec![
            ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                ActionArgOption::new("inputSlider", LocalizedLabel::native("Slider", "Schieberegler")),
                ActionArgOption::new("inputNote", LocalizedLabel::native("Note", "Notiz")),
                ActionArgOption::new("neuron", LocalizedLabel::native("Component", "Komponente")),
                ActionArgOption::new("outputPreview", LocalizedLabel::native("Preview", "Vorschau")),
                ActionArgOption::new("outputExport", LocalizedLabel::native("Export", "Export")),
            ]).default_value(&"inputSlider"),
        ])
        // 🕹️ First-class hover/selection (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM):
        // one domain over the flow-graph widget DAG, node/edge/handle granularities,
        // `HierarchyProvider::Topology` (see `Generation2dPlayApp::interaction_topology` above) —
        // transitive hover is the headline feature: hovering a Cluster group node highlights every
        // widget nested in its tree.
        .interaction(InteractionDefinition {
            id: "graph".into(),
            label: LocalizedLabel::native("Graph", "Graph"),
            granularities: vec![
                GranularityDefinition { id: "node".into(), label: LocalizedLabel::native("Node", "Knoten"), icon_id: "circle".into() },
                GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
                GranularityDefinition { id: "handle".into(), label: LocalizedLabel::native("Handle", "Griff"), icon_id: "move".into() },
            ],
            hierarchy: HierarchyProvider::Topology,
            hover: HoverSpec { transitive: true, ..HoverSpec::default() },
            selection: SelectionSpec {
                modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle],
                merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                transitive: false,
                broadcast: true,
            },
        })
        .window_kind_interactions(flow_window::GENERATION2D_PLAY_WINDOW_MAIN, vec![InteractionRef::new("graph")])
        .window_kind_interactions(edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW, vec![InteractionRef::new("graph")])
        .window_kind_interactions(generate_preview::GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW, vec![InteractionRef::new("graph")])
        // 📇️ Window-scoped action ownership — the generation2d twin of generation3d's own block
        // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END), asserted by
        // `every_emitted_action_is_declared_on_its_window_kind`. The node-graph host's two verbs on the
        // flow window, the canvas host's four pointer verbs on the two Canvas2d scene windows, the
        // generation tree's four row verbs on the Generations window, the form's one change verb on the
        // Form window. Everything else — `setShowMode`, `generate`, `addWidget`, `reorganize`,
        // `setEvalOutputs`, the framework's history ids — is app-scoped, stays unowned, and is therefore
        // copied onto every window by `build_definition`.
        .window_kind_action_refs(flow_window::GENERATION2D_PLAY_WINDOW_MAIN, vec!["nodeGraphEdit".into(), "nodeGraphViewport".into()])
        .window_kind_action_refs(edit_preview::GENERATION2D_PLAY_WINDOW_PREVIEW, vec!["canvasPointerDown".into(), "canvasPointerMove".into(), "canvasPointerUp".into(), "canvasWheel".into()])
        .window_kind_action_refs(generations::GENERATION2D_PLAY_WINDOW_GENERATIONS, vec!["addGeneration".into(), "selectGeneration".into(), "renameGeneration".into(), "removeGeneration".into()])
        .window_kind_action_refs(form::GENERATION2D_PLAY_WINDOW_GENERATE_FORM, vec!["updateGenerationValues".into()])
        .window_kind_action_refs(generate_preview::GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW, vec!["canvasPointerDown".into(), "canvasPointerMove".into(), "canvasPointerUp".into(), "canvasWheel".into()])
        .keybinding("mod+z", "undo")
        .keybinding("mod+shift+z", "redo")
        .config(Generation2dPlayApp::config_spec())
        .io(::semio_framework_async::poll::resolve_ready(generation2d_io()))
        // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`.editor::<E>(def: AppDefinition)` take a bare
        // `AppDefinition`, not the old `App { definition, examples }` — there is no `.example(...)`/
        // `.workflow(...)` on this builder, so the old `"default"` app-level example registration and
        // the no-op `.workflow("generation2d", …)` call are dropped here (not silently — reported in
        // this packet's migration notes). The subset's own `📚️examples` facet is the modern,
        // role-agnostic replacement surface for this.
        .action_describe("addGeneration", LocalizedLabel::native("Adds a new generation, a named set of input values for the 2D generator, to the generation list and selects it.", "Fügt der Generationsliste eine neue Generation hinzu, einen benannten Satz von Eingabewerten für den 2D-Generator, und wählt sie aus."))
        .action_describe("selectGeneration", LocalizedLabel::native("Selects the generation with the given id, whose input values the generator then evaluates and shows.", "Wählt die Generation mit der angegebenen Id aus, deren Eingabewerte der Generator dann auswertet und zeigt."))
        .action_describe("renameGeneration", LocalizedLabel::native("Renames one generation of the generation list.", "Benennt eine Generation der Generationsliste um."))
        .action_describe("removeGeneration", LocalizedLabel::native("Removes one generation by id from the generation list, with its input values.", "Entfernt eine Generation anhand ihrer Id samt ihrer Eingabewerte aus der Generationsliste."))
        .action_describe("updateGenerationValues", LocalizedLabel::native("Sets the value one input question takes in a generation (the selected one when no id is given) and re-evaluates the result.", "Setzt den Wert, den eine Eingabefrage in einer Generation annimmt (ohne Id in der ausgewählten), und wertet das Ergebnis neu aus."))
        .action_describe("addWidget", LocalizedLabel::native("Adds a new widget of the given kind (an input or an operator of the 2D generator graph) to the canvas.", "Fügt der Fläche ein neues Widget der angegebenen Art hinzu (eine Eingabe oder einen Operator des 2D-Generatorgraphen)."))
        .action_describe("removeWidget", LocalizedLabel::native("Removes one widget by id from the generator graph together with its connections.", "Entfernt ein Widget anhand seiner Id samt seiner Verbindungen aus dem Generatorgraphen."))
        .action_describe("reorganize", LocalizedLabel::native("Lays out every widget of the generator graph automatically from left to right, overwriting their manual positions.", "Ordnet alle Widgets des Generatorgraphen automatisch von links nach rechts an und überschreibt ihre manuellen Positionen."))
        .action_describe("setShowMode", LocalizedLabel::native("Sets what the editor shows, such as the generator graph or the generated result; only the view changes.", "Legt fest, was der Editor zeigt, etwa den Generatorgraphen oder das erzeugte Ergebnis; nur die Ansicht ändert sich."))
        .action_describe("setActiveExample", LocalizedLabel::native("Replaces the whole 2D generator with one of the plugin's bundled examples, by example id.", "Ersetzt den gesamten 2D-Generator durch eines der mitgelieferten Beispiele, anhand der Beispiel-Id."))
        .action_describe("moveMediaNode", LocalizedLabel::native("Moves one widget to the canvas position x, y.", "Verschiebt ein Widget an die Position x, y der Fläche."))
        .action_describe("connectMediaPorts", LocalizedLabel::native("Connects an output port of one widget to an input port of another; an incompatible connection changes nothing.", "Verbindet einen Ausgangsport eines Widgets mit einem Eingangsport eines anderen; eine unverträgliche Verbindung ändert nichts."))
        .action_describe("generate", LocalizedLabel::native("Switches the editor to its generate view, which shows the generation list and the result of the selected generation.", "Schaltet den Editor in seine Generieren-Ansicht, die die Generationsliste und das Ergebnis der ausgewählten Generation zeigt."))
        .action_describe("setEvalOutputs", LocalizedLabel::native("Hands the editor evaluated generator outputs (JSON) computed elsewhere so it can show them; the document is not changed.", "Übergibt dem Editor anderswo berechnete Generatorausgaben (JSON), damit er sie zeigt; das Dokument ändert sich nicht."))
        .action_audience("nodeGraphEdit", semio_framework_plugin::CapabilityAudience::Input)
        .action_audience("nodeGraphViewport", semio_framework_plugin::CapabilityAudience::Chrome)
        .action_destructive("reorganize")
        .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests

//#endregion 🧪️UnitTests

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo_session {
        #[path = "📚️examples/🎬️demo-session/🦀️.rs"]
        mod component;
        pub use component::*;
        #[cfg(test)]
        #[path = "📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
        mod tests;
    }
}
#[path = "🌉️wasm/🦀️.rs"]
pub mod wasm;
