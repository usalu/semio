//! 🧱️ Generation3d editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and
//! the manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, panel trees in `📌️panels/*`, labels in `🦀️terminology.rs`, view state in
//! `🦀️config.rs`, shared compute in the artifact's `⚙️engine`.
#![allow(clippy::result_large_err)]
extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

use semio_s_artifact_procedural_3d_navigate_graph::activate_selection;
use semio_s_artifact_procedural_3d_navigate_graph::select_downstream_node;
use semio_s_artifact_procedural_3d_navigate_graph::select_next_node;
use semio_s_artifact_procedural_3d_navigate_graph::select_previous_node;
use semio_s_artifact_procedural_3d_navigate_graph::select_upstream_node;
use semio_s_artifact_procedural_3d_editor_commands::add_generation;
use semio_s_artifact_procedural_3d_add_widget as add_widget;
use semio_s_artifact_procedural_3d_editor_commands::cycle_lod_mode;
use semio_s_artifact_procedural_3d_editor_commands::cycle_show_mode;
use semio_s_artifact_procedural_3d_preview_eval as preview_eval;
use semio_s_artifact_procedural_3d_editor_commands::edit_mesh_selection;
use semio_s_artifact_procedural_3d_editor_commands::knife_mesh_selection;
use semio_s_artifact_procedural_3d_delete_selection as delete_selection;
use semio_s_artifact_procedural_3d_export_document as export_document;
use semio_s_artifact_procedural_3d_editor_commands::flow_eval_release;
use semio_s_artifact_procedural_3d_editor_commands::flow_eval_resolve;
use semio_s_artifact_procedural_3d_flow_eval_tick as flow_eval_tick;
use semio_s_artifact_procedural_3d_editor_commands::flow_tessellate_cancel_resolve;
use semio_s_artifact_procedural_3d_editor_commands::flow_tessellate_resolve;
use semio_s_artifact_procedural_3d_import_document as import_document;
use semio_s_artifact_procedural_3d_editor_commands::import_document_request;
use semio_s_artifact_procedural_3d_node_graph_edit as node_graph_edit;
use semio_s_artifact_procedural_3d_editor_commands::node_graph_viewport;
use semio_s_artifact_procedural_3d_editor_commands::patch_flow_widgets;
use semio_s_artifact_procedural_3d_editor_commands::remove_generation;
use semio_s_artifact_procedural_3d_editor_commands::remove_widget;
use semio_s_artifact_procedural_3d_editor_commands::rename_generation;
use semio_s_artifact_procedural_3d_editor_commands::reorganize;
use semio_s_artifact_procedural_3d_rotate_selection as rotate_selection;
use semio_s_artifact_procedural_3d_scale_selection as scale_selection;
use semio_s_artifact_procedural_3d_editor_commands::select_generation;
use semio_s_artifact_procedural_3d_editor_commands::set_active_example;
use semio_s_artifact_procedural_3d_editor_commands::set_camera;
use semio_s_artifact_procedural_3d_set_contributions as set_contributions;
use semio_s_artifact_procedural_3d_editor_commands::set_lod_mode;
use semio_s_artifact_procedural_3d_editor_commands::set_show_mode;
use semio_s_artifact_procedural_3d_editor_commands::set_sun_azimuth;
use semio_s_artifact_procedural_3d_editor_commands::set_sun_elevation;
use semio_s_artifact_procedural_3d_editor_commands::set_sun_intensity;
use semio_s_artifact_procedural_3d_editor_commands::toggle_sun;
use semio_s_artifact_procedural_3d_editor_commands::translate_selection;
use semio_s_artifact_procedural_3d_editor_commands::update_generation_values;
use semio_s_artifact_procedural_3d_set_widget_input as set_widget_input;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfig;
use semio_s_artifact_procedural_3d_editor_config::Generation3dConfigMutation;
use semio_s_artifact_procedural_3d_mode_edit::windows::flow as flow_window;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview as edit_preview;
use semio_s_artifact_procedural_3d_mode_generate::windows::form;
use semio_s_artifact_procedural_3d_mode_generate::windows::generations;
use semio_s_artifact_procedural_3d_mode_generate::windows::preview as generate_preview;
use semio_s_artifact_procedural_3d_mode_edit as edit;
use semio_s_artifact_procedural_3d_mode_generate as generate;
use semio_s_artifact_procedural_3d_editor_panels::catalogue as catalogue_panel;
use semio_s_artifact_procedural_3d_editor_panels::artifact as artifact_panel;
use semio_s_artifact_procedural_3d_editor_panels::inspection as inspection_panel;
use semio_s_artifact_procedural_3d_terminology::generation3d_labels;
use semio_s_artifact_procedural_3d_editor_transient::Generation3dTransient;
use semio_s_artifact_procedural_3d_editor_transient::Generation3dTransientMutation;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::artifact_kind;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_s_artifact_procedural_3d_model::GENERATION_3D_SCHEMA;
use semio_os_flow::{FlowEvalSession, FlowHost};
// 🚧️ SDK note (ticket 26/08/16 contract §2.1/§2.4): `ArtifactEditor`/`Editor`/`Dialect` are curated at
// `semio_plugin`'s crate root as of W0-F/W2-FIX — imported bare here, no `app::` prefix
// needed (unlike the earlier cad pilot, written before that gap closed). `app::InteractionView` is a
// separate, still-uncurated gap (unrelated to this ticket) — kept qualified.
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactoryError};
use semio_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, ArtifactRetainedWorkCapacity};
use semio_plugin::app::InteractionView;
use semio_plugin::ActionArgDef;
use semio_plugin::ActionArgOption;
use semio_plugin::ActionDefinition;
use semio_plugin::ActionDescriptor;
use semio_plugin::ActionKind;
use semio_plugin::AppOperationContext;
use semio_plugin::ArtifactEditor;
use semio_plugin::ArtifactOwnedToolJobRequest;
use semio_plugin::ArtifactToolFactoryRegistry;
use semio_plugin::ArtifactToolPublicationContract;
use semio_plugin::ArtifactToolPublicationLane;
use semio_plugin::ArtifactView;
use semio_plugin::CommandDefinition;
use semio_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_plugin::DomainTopology;
use semio_plugin::DraftView;
use semio_plugin::Editor;
use semio_plugin::EditorApp;
use semio_plugin::Effect;
use semio_plugin::Emit;
use semio_plugin::EphemeralEmit;
use semio_plugin::ExampleSource;
use semio_plugin::Fault;
use semio_plugin::FaultCode;
use semio_plugin::FaultOrigin;
use semio_plugin::GranularityDefinition;
use semio_plugin::HierarchyProvider;
use semio_plugin::HoverSpec;
use semio_plugin::InteractionDefinition;
use semio_plugin::InteractionRef;
use semio_plugin::InteractionTopology;
use semio_plugin::InteractiveJobClassification;
use semio_ui_locale::Label;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::MediaClass;
use semio_plugin::MediaError;
use semio_plugin::MediaForm;
use semio_plugin::MediaType;
use semio_plugin::MergeMode;
use semio_plugin::NoDraft;
use semio_plugin::NoDraftMutation;
use semio_plugin::SelectionMethod;
use semio_plugin::SelectionMode;
use semio_plugin::SelectionSpec;
use semio_plugin::ToolRunJob;
use semio_plugin::ToolRunJobPurpose;
use semio_plugin::ToolRunJobRequest;
use semio_plugin::TopologyNode;
use semio_plugin::UtilityDefinition;
use semio_plugin::WindowMeasure;
use std::collections::HashMap;
use semio_framework_2d_compute::EngineHandles;
use semio_s_artifact_procedural_3d_model::{GENERATION3D_EDITOR_APP_ID, generation3d_action};
pub use semio_s_artifact_procedural_3d_mode_edit::windows::preview::PreviewInteractionMarks;
use semio_s_artifact_procedural_3d_mode_edit::windows::preview::preview_selection_json;
use semio_s_artifact_procedural_3d_export_document::export_mesh_from_session;
use semio_s_artifact_procedural_3d_export_document::export_meshes_from_session;
use semio_s_artifact_procedural_3d_import_document::GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES;
use semio_s_artifact_procedural_3d_editor_commands::{generation::Generation3dGenerationCommandResult, generation::generation_command_result};

//#region 🔖️Constants

//#endregion 🔖️Constants

//#region 🔖️Commands

#[path = "🧿️commands-and-instance-operation-owner/🦀️.rs"]
mod commands_and_instance_operation_owner;
pub use commands_and_instance_operation_owner::*;

#[path = "🖼️play-app-and-retained-command-contracts/🦀️.rs"]
mod play_app_and_retained_command_contracts;
pub use play_app_and_retained_command_contracts::*;

#[path = "🛠️preview-and-flow-eval-window-command-work/🦀️.rs"]
mod preview_and_flow_eval_window_command_work;
pub use preview_and_flow_eval_window_command_work::*;

#[path = "🔺️retained-reduce-gumball-and-bounded-job-factory/🦀️.rs"]
mod retained_reduce_gumball_and_bounded_job_factory;
pub use retained_reduce_gumball_and_bounded_job_factory::*;

#[path = "🌊️flow-eval-and-document-io-routes/🦀️.rs"]
mod flow_eval_and_document_io_routes;
pub use flow_eval_and_document_io_routes::*;

#[path = "🤝️bounded-proofs-and-contributions-route/🦀️.rs"]
mod bounded_proofs_and_contributions_route;
pub use bounded_proofs_and_contributions_route::*;

#[path = "📬️artifact-store-preparation/🦀️.rs"]
mod artifact_store_preparation;
pub use artifact_store_preparation::*;

#[path = "🪟️artifact-editor-impl/🦀️.rs"]
mod artifact_editor_impl;

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported at file top under its own flat name.
//#endregion 🔖️Commands

//#region 🔖️Generation3dPlayApp

impl Generation3dPlayApp {
    /// 🗂️ The ONE context-menu implementation — both `ArtifactEditor::context_menu` and
    /// `context_menu_with_request_context` funnel here.
    ///
    /// Grouped disclosure: `reorganize` stays top-level, the transform trio appears only for a real
    /// selection, creation/removal/generation methods fold into taxonomy groups, and
    /// `delete-selection` stays a direct destructive item last.
    fn context_menu_body(
        request: &semio_plugin::ContextMenuRequest,
        doc: &ArtifactView<'_, Generation3dSnapshot>,
        _cfg: &ConfigView<'_, Generation3dConfig>,
        view_state: &semio_plugin::ViewModel,
        marks: &PreviewInteractionMarks,
        registry: &semio_plugin::AppActionRegistry,
    ) -> Vec<semio_plugin::ContextMenuItemSpec> {
        use semio_plugin::{node_graph_delete_selection_spec, selection_domains_from_surface, Menu, NodeGraphDeleteDispatch};
        let labels = generation3d_labels(view_state);
        let (selected_nodes, selected_edges) = marks.graph_selection_domains(&doc.snapshot.host_snapshot);
        let (nodes, edges) = selection_domains_from_surface(request.surface.as_ref(), &selected_nodes, &selected_edges);
        let has_selection = !nodes.is_empty() || !edges.is_empty();
        let mut menu = Menu::of(registry, view_state).action("reorganize");
        if has_selection {
            menu = menu.action("translateSelection").action("rotateSelection").action("scaleSelection");
        }
        menu = menu.group("create", |m| m.action("addWidget").action("addGeneration"));
        if has_selection {
            menu = menu.group("targets", |m| m.action("removeWidget").action("removeGeneration"));
        }
        menu = menu.group("methods", |m| m.action("renameGeneration").action("updateGenerationValues").action("patchFlowWidgets"));
        menu = menu.group(CONTEXT_MENU_TRANSFER_CATEGORY, |m| m.action("importDocumentRequest").action("exportDocument"));
        menu.item(node_graph_delete_selection_spec(labels.delete_selection.as_str(), view_state, &nodes, &edges, NodeGraphDeleteDispatch::ViaNodeGraphEdit)).build()
    }
}

//#endregion 🔖️Generation3dPlayApp

/// 🗂️ The context-menu group the two artifact round-trips (`Import Artifact…`, `Export Artifact`)
/// live under.
///
/// A `Menu::group(category)` row travels to the shell with `label: None` BY CONTRACT and the shell
/// resolves it from the closed 20-id ribbon-parent taxonomy (`RIBBON_PARENT_CATEGORIES`,
/// `🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs`). A category outside that table has no label to resolve
/// to and falls back to the raw id, so this menu used to show a user a row literally reading
/// `menu.group.io` in both locales — measured on the React serve 2026-09-14, ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END. `transfer` is the taxonomy's own name for moving a document in
/// and out; asserted against the table by `context_menu_groups_are_taxonomy_categories`.
const CONTEXT_MENU_TRANSFER_CATEGORY: &str = "transfer";

//#region 🔖️Manifest
/// 📚️ The eight bundled `📚️examples/🎬️<slug>` fixtures, in `schema::is_generation3d_example_id`'s
/// order. `Generation3dPlayApp::examples` returns this catalogue, and `.editor` stamps it onto
/// the manifest the navbar dropdown reads.
///
/// 🚫️ `✏️editor/📚️examples/🎬️demo-session` is deliberately NOT here, and its absence is a statement,
/// not an oversight (`📓️audit-user-journey-gaps-2026-09-13.md` §9 item 12 read it as one). That leaf
/// carries `.cmd.semio` command-REPLAY text, and `setActiveExample`'s only vocabulary is "load a
/// registered example document" — `🖨️raster` states the same rule for the same leaf, and `🧩️puzzle`
/// mounts and tests three of them without registering any. Held by
/// `the_example_picker_offers_the_flow_examples_and_never_the_command_session`.
pub fn examples() -> Vec<ExampleSource> {
    vec![
        semio_s_artifact_procedural_3d_examples::art_generation3d_hexagonal_mushroom_column::source(),
        semio_s_artifact_procedural_3d_examples::art_generation3d_mesh_workbench::source(),
        semio_s_artifact_procedural_3d_examples::art_generation3d_rectangle_extrude_volume::source(),
        semio_s_artifact_procedural_3d_examples::art_generation3d_sphere_cut_with_torus::source(),
        semio_s_artifact_procedural_3d_examples::art_generation3d_box_fillet_preview::source(),
        semio_s_artifact_procedural_3d_examples::art_generation3d_sphere_box_fuse::source(),
        semio_s_artifact_procedural_3d_examples::art_generation3d_face_sweep_extrude::source(),
        semio_s_artifact_procedural_3d_examples::art_generation3d_rectangle_wire_preview::source(),
        semio_s_artifact_procedural_3d_examples::art_generation3d_box_shell_preview::source(),
    ]
}
//#endregion 🔖️Manifest

//#region 🔖️ArtifactIo
/// 🔌️ Rehomed from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES) —
/// this app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `create_generation3d_app` declares via `.artifact_kind(...)`; `params:in`/`geometry:out` are the
/// workflow-specific ports beyond the implicit document in/out ports.
pub async fn generation3d_io() -> semio_plugin::AppIo {
    semio_plugin::AppIo::from_artifact(
        "generation.3d",
        MediaType { class: MediaClass::ThreeD, form: MediaForm::Flow },
        semio_plugin::ArtifactPresentation { id: "3d.generation".into(), name: "3D Generation".into(), dimension: "3d".into(), component_kind: "generation3d".into() },
    )
    .await
    .with_ports(vec![
        semio_plugin::MediaPortSpec {
            id: "params:in".into(),
            label: "Parameters".into(),
            direction: semio_plugin::MediaPortDirection::In,
            media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            kind_id: None,
            required: false,
            multiplicity: semio_framework::PortMultiplicity::One,
        },
        semio_plugin::MediaPortSpec {
            id: "geometry:out".into(),
            label: "Geometry".into(),
            direction: semio_plugin::MediaPortDirection::Out,
            media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
            kind_id: Some("3d.mesh".into()),
            required: false,
            multiplicity: semio_framework::PortMultiplicity::Many,
        },
    ])
    .await
}
//#endregion 🔖️ArtifactIo

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

#[path="../../../../../../../🫀️foundation/🌊️flow-source/🦀️.rs"]
mod retained_flow_source;

#[path="../../../../../../../🫀️foundation/🧬️generation/🌊️flow-overlay/🦀️.rs"]
mod retained_flow_overlay;

#[cfg(test)]
#[path="../../../../../../../🫀️foundation/🧬️generation/🌊️flow-overlay/🧪️laws/🦀️.rs"]
mod retained_flow_overlay_laws;

#[path="../../../../../../../🫀️foundation/🌊️flow-source/📥️provisional/🦀️.rs"]
mod retained_provisional_source;

#[path="../../../../../../../🫀️foundation/🌊️flow-source/📋️rows/🦀️.rs"]
mod retained_scratch_rows;
#[cfg(test)]
#[path="../../../../../../../🫀️foundation/🌊️flow-source/📋️rows/🧪️tests/🦀️.rs"]
mod retained_scratch_rows_laws;

#[path="../../../../../../../🫀️foundation/🌊️flow-source/🔎️rows/🦀️.rs"]
mod retained_scratch_scan;
#[cfg(test)]
#[path="../../../../../../../🫀️foundation/🌊️flow-source/🔎️rows/🧪️tests/🦀️.rs"]
mod retained_scratch_scan_laws;

#[path="../../../../../../../🫀️foundation/🌊️flow-source/✏️host-scratch/🦀️.rs"]
mod retained_host_scratch;
#[path="../../../../../../../🫀️foundation/🌊️flow-source/🎟️driver/🦀️.rs"]
mod retained_scratch_driver;
#[path="../../../../../../../🫀️foundation/🌊️flow-source/📋️projection/🦀️.rs"]
mod retained_projected_clone;
#[cfg(test)]
#[path="../../../../../../../🫀️foundation/🌊️flow-source/✏️host-scratch/🧪️laws/🦀️.rs"]
mod retained_host_scratch_laws;

#[cfg(test)]
fn host_scratch_test_domain()->retained_host_scratch::HostScratchDomain{retained_host_scratch::HostScratchDomain::Generation3d}

#[cfg(test)]
fn host_scratch_test_original(source:&semio_framework_artifact_flow_flow::FlowHostSnapshot,wire:&serde_json::Value)->semio_framework_artifact_flow_flow::FlowHostSnapshot{let mut snapshot=Generation3dSnapshot{host_snapshot:source.clone(),generation:Default::default()};let mutation:Generation3dMutation=semio_framework_pack_json::from_json_str(&wire.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let _=semio_s_artifact_procedural_3d_mutations::central_apply::apply_generation3d_mutation(&mut snapshot,&mutation);mutation.retire_cold();let result=std::mem::take(&mut snapshot.host_snapshot);snapshot.retire_cold();result}

#[cfg(test)]
#[path="../../../../../../../🫀️foundation/🌊️flow-source/📋️projection/🧪️laws/🦀️.rs"]
mod retained_projected_clone_laws;

#[path="🧵️retained-commands/📥️provisional/🎛️input/🧬️admit/🦀️.rs"]
mod retained_typed_input_admission;

#[cfg(test)]
#[path="🧵️retained-commands/📥️provisional/🎛️input/🧬️admit/🧪️laws/🦀️.rs"]
mod retained_typed_input_admission_laws;

use semio_s_artifact_procedural_3d_model::{GenerationPlayState,FormGeneration};

#[path="../../../../../../../🫀️foundation/🧬️generation/🧵️fold/🦀️.rs"]
mod retained_generation_fold;
#[cfg(test)]
#[path="../../../../../../../🫀️foundation/🌊️flow-source/🧪️caller/🦀️.rs"]
mod retained_scratch_testing;
#[cfg(test)]
#[path="../../../../../../../🫀️foundation/🧬️generation/🧵️fold/🧪️tests.rs"]
mod retained_generation_fold_laws;

#[cfg(test)]
fn generation_fold_test_original(source:&GenerationPlayState,wire:&serde_json::Value)->(GenerationPlayState,bool){let mut snapshot=Generation3dSnapshot{host_snapshot:Default::default(),generation:source.clone().into()};let mutation:Generation3dMutation=semio_framework_pack_json::from_json_str(&wire.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let accepted=semio_s_artifact_procedural_3d_mutations::central_apply::apply_generation3d_mutation(&mut snapshot,&mutation).is_ok();mutation.retire_cold();let result=snapshot.generation.as_state().clone();snapshot.retire_cold();(result,accepted)}

#[path="../../../../../../../🫀️foundation/🌊️flow-source/📥️provisional/✏️fold/🦀️.rs"]
mod retained_native_provisional_fold;

#[path="🧵️retained-commands/📥️provisional/✏️dispatch/🦀️.rs"]
mod retained_provisional_dispatch;

#[cfg(test)]
type HostScratchFixtureMutation=Generation3dMutation;

#[path="../../../../../../../🫀️foundation/🌊️flow-source/📥️provisional/🔁️replay/🦀️.rs"]
mod retained_provisional_replay;

#[path="🧵️retained-commands/📥️provisional/🎛️input/💎️literal/🦀️.rs"]
mod retained_typed_literal;

#[path="🧵️retained-commands/📥️provisional/🎛️input/⚖️current/🦀️.rs"]
mod retained_typed_current;

#[cfg(test)]
#[path="🧵️retained-commands/📥️provisional/🎛️input/⚖️current/🧪️laws/🦀️.rs"]
mod retained_typed_current_laws;

#[path="🧵️retained-commands/📥️provisional/🎛️input/🦀️.rs"]
mod retained_typed_input;
type NativeProvisionalAdditional=retained_typed_input::P3ProvisionalFold;

#[cfg(test)]
#[path="🧵️retained-commands/📥️provisional/🎛️input/🧪️laws/🦀️.rs"]
mod retained_typed_input_laws;

#[path="🧵️retained-commands/📥️provisional/🎛️input/📚️params/🔎️read/🦀️.rs"]
mod retained_native_param_read;

#[cfg(test)]
#[path="🧵️retained-commands/📥️provisional/🎛️input/📚️params/🔎️read/🧪️laws/🦀️.rs"]
mod retained_native_param_read_laws;

#[path="🧵️retained-commands/📥️provisional/🎛️input/📚️params/✏️patch/🦀️.rs"]
mod retained_native_param_patch;

#[path="🧵️retained-commands/📥️provisional/🧭️transform/🦀️.rs"]
mod retained_provisional_transform;

#[cfg(test)]
#[path="🧵️retained-commands/📥️provisional/🧭️transform/🧪️laws/🦀️.rs"]
mod retained_provisional_transform_laws;

#[path="../../../../../../../🫀️foundation/🌊️flow-source/🎬️scene/🦀️.rs"]
mod retained_provisional_scene;
#[path="../../../../../../../🫀️foundation/🌊️flow-source/🎬️scene/🧷️base.rs"]
mod retained_scene_base;

#[cfg(test)]
type ProvisionalSceneTestApp=EditorApp<Generation3dPlayApp>;
#[cfg(test)]
#[path="../../../../../../../🫀️foundation/🌊️flow-source/🎬️scene/🧪️.rs"]
mod retained_provisional_scene_laws;

#[path="../../../../../../../🫀️foundation/🌊️flow-source/📇️catalogue/🦀️.rs"]
mod retained_catalogue_lifecycle;
