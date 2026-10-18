//! 👁️ Generation3d viewer — the read-only counterpart of `✏️editor` for this subset (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.2). `Generation3dViewer` implements
//! `ArtifactViewer`, never `ArtifactEditor`/`ArtifactApp` — `ViewerApp<Generation3dViewer>`
//! (framework SDK) is the sole runtime adapter, so this file can never structurally emit an artifact
//! or draft mutation. MUST NOT import anything from the sibling `✏️editor` module
//! (`policyViewerPurityBreaches`).
//!
//! 🕹️ Read-only does not mean inert. A user opening a generation3d artifact read-only gets: a real
//! `World3dScene` of the evaluated geometry, live hover and selection over the framework's own
//! `graph` interaction domain, a preview camera they can orbit/pan/zoom, four shading modes, three
//! LOD steps and a full sun rig — every one of them a `Migrated` interactive job with an exact
//! app-owned bounded reducer, publishing on the CONFIG lane only. That publication contract is the
//! runtime half of the read-only guarantee whose compile-time half is `ViewEmit`.
#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_s_artifact_procedural_3d_preview_eval as preview_eval;
use semio_s_artifact_procedural_3d_viewer_commands::export_document;
use semio_s_artifact_procedural_3d_viewer_commands::flow_eval_release;
use semio_s_artifact_procedural_3d_viewer_commands::flow_eval_resolve;
use semio_s_artifact_procedural_3d_viewer_commands::flow_eval_tick;
use semio_s_artifact_procedural_3d_viewer_commands::flow_tessellate_cancel_resolve;
use semio_s_artifact_procedural_3d_viewer_commands::flow_tessellate_resolve;
use semio_s_artifact_procedural_3d_viewer_commands::set_active_example;
use semio_s_artifact_procedural_3d_viewer_commands::set_camera;
use semio_s_artifact_procedural_3d_viewer_commands::set_contributions;
use semio_s_artifact_procedural_3d_viewer_commands::set_lod_mode;
use semio_s_artifact_procedural_3d_viewer_commands::set_show_mode;
use semio_s_artifact_procedural_3d_viewer_commands::set_sun_azimuth;
use semio_s_artifact_procedural_3d_viewer_commands::set_sun_elevation;
use semio_s_artifact_procedural_3d_viewer_commands::set_sun_intensity;
use semio_s_artifact_procedural_3d_viewer_commands::toggle_sun;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfig;
use semio_s_artifact_procedural_3d_viewer_config::Generation3dViewConfigMutation;
use semio_s_artifact_procedural_3d_viewer_modes::view;
use semio_s_artifact_procedural_3d_viewer_modes::view::windows::preview;
use semio_s_artifact_procedural_3d_viewer_presence::Generation3dViewPresence;
use semio_s_artifact_procedural_3d_viewer_presence::Generation3dViewPresenceMutation;
use semio_s_artifact_procedural_3d_viewer_transient::Generation3dViewTransient;
use semio_s_artifact_procedural_3d_viewer_transient::Generation3dViewTransientMutation;
use semio_s_artifact_procedural_3d_mutations::Generation3dMutation;
use semio_s_artifact_procedural_3d_model::snapshot::Generation3dSnapshot;
use semio_s_artifact_procedural_3d_model::GENERATION3D_DIALECT;
use semio_s_artifact_procedural_3d_model::GENERATION_3D_SCHEMA;
use semio_framework::{ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::ActionDefinition;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionKind;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::ArtifactToolPublicationContract;
use semio_framework_plugin::ArtifactToolPublicationLane;
use semio_framework_plugin::ArtifactView;
use semio_framework_plugin::ConfigView;
use {semio_framework_artifact_reference::Dialect};
use semio_framework_plugin::DomainTopology;
use semio_framework_plugin::Emit;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::Fault;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_plugin::InteractionRef;
use semio_framework_plugin::InteractionTopology;
use semio_framework_plugin::InteractiveJobClassification;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::ToolRunJob;
use semio_framework_plugin::ToolRunJobPurpose;
use semio_framework_plugin::ToolRunJobRequest;
use semio_framework_plugin::TopologyNode;
use semio_framework_plugin::ViewEmit;
use semio_framework_plugin::Viewer;
use semio_framework_plugin::ViewerApp;
use semio_framework_plugin::WindowMeasure;
use std::collections::HashMap;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Constants

#[path = "🪪️view-constants-and-instance-operation-owner/🦀️.rs"]
mod view_constants_and_instance_operation_owner;
pub use view_constants_and_instance_operation_owner::*;

#[path = "🧲️retained-view-commands/🦀️.rs"]
mod retained_view_commands;
pub use retained_view_commands::*;

#[path = "🌐️flow-eval-chain-and-bounded-job-factory/🦀️.rs"]
mod flow_eval_chain_and_bounded_job_factory;
use flow_eval_chain_and_bounded_job_factory::*;

#[path = "🎨️contributions-and-example-routes/🦀️.rs"]
mod contributions_and_example_routes;
use contributions_and_example_routes::*;

#[path = "🔀️flow-eval-and-document-io-routes/🦀️.rs"]
mod flow_eval_and_document_io_routes;
use flow_eval_and_document_io_routes::*;

#[path = "🔭️viewer-and-interaction-topology/🦀️.rs"]
mod viewer_and_interaction_topology;
pub use viewer_and_interaction_topology::*;

#[path = "📜️viewer-manifest/🦀️.rs"]
mod viewer_manifest;
pub use viewer_manifest::*;

//#endregion 🔖️Constants

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod unit_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️status-contract/🦀️.rs"]
mod status_contract_tests;
