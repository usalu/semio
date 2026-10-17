//! ✏️ animate of `semio-s-artifact-animate-presentation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;

pub use crate::snapshot::schema::PresentationSnapshot;
pub use crate::snapshot::schema::default_snapshot;
pub use crate::snapshot::schema::demo_snapshot;
pub use semio_s_artifact_animate_presentation_tile_grid::{FigureTileDraft, FigureTileFrame, FigureTileSource};
pub use semio_s_artifact_animate_presentation_inferences::{DEMO_FIGURE_SRC, PRESENTATION_DOCUMENT_SCHEMA, default_figure_tile_source, genesis_presentation_child_pack, presentation_snapshot_with_tiles, presentation_working_scene};
pub use semio_s_artifact_animate_presentation_tests::{default_presentation_snapshot};
pub use semio_s_artifact_animate_presentation_mutations::{ANIMATE_DIALECT};
pub use semio_s_artifact_animate_presentation_host::{demo_presentation_snapshot};
pub use crate::editor::animate::artifact_items_12::{artifact_kind};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
