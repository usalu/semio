//! 🕸️ mesh-edge preview overlay). Also hosts the screen-space draw helpers shared with the results window
//! 🧱️ Fem2d play app — the model window: the editable 2D structural canvas (nodes/members/supports,
//! (`semio_s_artifact_fem_2d_editor_results_window::editor::fem2d::modes::edit::windows::results`) — kept here rather than in the artifact's
//! `⚙️engine` because they take/return app-facing `semio_framework_plugin` scene types and their only
//! two consumers are these two sibling windows, both at app level.

#[path = "🎚️config/🦀️.rs"]
pub mod config;

/// 🔺️ Region mesh element identity, screen coordinates and node identities.
pub(crate) type Fem2dRegionMeshTriangle = (String, [(f64, f64); 3], [String; 3]);

use semio_s_fem_model::Dof;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_framework_os_kernel::Viewport2d;
use semio_framework_plugin::BuiltNode;
use semio_framework_plugin::Canvas2dScene;
#[cfg(test)]
use semio_framework_ui_scene::canvas2d_snapshot_with_page;
use semio_framework_ui_scene::canvas2d_snapshot_abort_write;
use semio_framework_ui_scene::canvas2d_snapshot_abort_write_step;
use semio_framework_ui_scene::canvas2d_snapshot_admit_page;
use semio_framework_ui_scene::canvas2d_snapshot_begin;
use semio_framework_ui_scene::canvas2d_snapshot_begin_close;
use semio_framework_ui_scene::canvas2d_snapshot_close_step;
use semio_framework_ui_scene::canvas2d_snapshot_seal;
use semio_framework_ui_scene::canvas2d_snapshot_terminal_is_empty;
use semio_framework_ui_scene::canvas2d_snapshot_write_terminal_is_empty;
use semio_framework_ui_scene::Canvas2dSnapshotDescriptor;
use semio_framework_ui_scene::Canvas2dSnapshotLease;
use semio_framework_ui_scene::Canvas2dSnapshotPage;
use semio_framework_ui_scene::Canvas2dSnapshotWriteToken;
use semio_framework_ui_scene::CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY;
use std::collections::HashMap;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_element_endpoints;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::find_node_2d;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::screen_2d;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::SCALE_2D;
use std::fmt::Write;

#[path = "🎨️visual-state-and-quality/🦀️.rs"]
mod visual_state_and_quality;
pub use visual_state_and_quality::{
    Fem2dLiveVisual, Fem2dRegionQualitySlots, FemVisualState, NodeLiveField, RegionVisualQuality,
    BODY_KEY, MOMENT_SCALE_2D, WINDOW_KIND_ID,
};
use visual_state_and_quality::{FEM2D_LIVE_REGION_CAPACITY, MESH_EDGE_COLOR};

#[path = "🗄️fixed-packet-pages-and-lease/🦀️.rs"]
mod fixed_packet_pages_and_lease;
use fixed_packet_pages_and_lease::{Fem2dFixedOrder, FEM2D_MOUNTED_VISUAL_OUTPUT_BYTES};
pub use fixed_packet_pages_and_lease::{
    Fem2dFixedPacketPages, Fem2dMountedVisualLease, Fem2dVisualFreshness, FEM2D_MOUNTED_VISUAL_PAGE_BYTES,
    FEM2D_MOUNTED_VISUAL_PAGE_COUNT,
};

#[path = "🧵️visual-job/🦀️.rs"]
mod visual_job;
use visual_job::{
    FEM2D_VISUAL_MAXIMUM_ELEMENTS, FEM2D_VISUAL_MAXIMUM_FIELDS, FEM2D_VISUAL_MAXIMUM_REGIONS,
};
pub use visual_job::{Fem2dVisualJob, Fem2dVisualJobStage, FEM2D_VISUAL_MAXIMUM_NODES};

#[path = "🖼️structure-and-live-layers/🦀️.rs"]
mod structure_and_live_layers;
#[cfg(test)]
use structure_and_live_layers::vector_layer;
pub use structure_and_live_layers::{
    fem2d_deformed_shape_layers, fem2d_live_visual_layers, fem2d_model_extent,
    fem2d_region_mesh_triangles, fem2d_structure_layers_with, render, render_with_progress,
};
pub use structure_and_live_layers::fem2d_structure_layers;
#[cfg(test)]
pub(crate) use structure_and_live_layers::{HOVER_COLOR_2D, SELECTION_COLOR_2D};

//#region 🧪️Tests
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️vector-json-contract/🦀️.rs"]
mod vector_json_contract;
#[cfg(test)]
#[path = "🧪️tests/🔬️emphasis/🦀️.rs"]
mod emphasis;
