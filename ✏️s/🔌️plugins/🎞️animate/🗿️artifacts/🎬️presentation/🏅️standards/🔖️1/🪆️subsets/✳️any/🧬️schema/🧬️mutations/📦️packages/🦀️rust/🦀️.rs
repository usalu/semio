//! 🧬️ mutations of `semio-s-artifact-animate-presentation` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::snapshot::schema::PresentationSnapshot;
pub use crate::standards::v1::subsets::any::schema::mutations::PresentationMutation;
pub use crate::snapshot::schema::default_snapshot;
pub use crate::snapshot::schema::demo_snapshot;
#[cfg(test)]
pub use crate::standards::v1::subsets::any::schema::diff::PresentationDiff;
pub use semio_s_artifact_animate_presentation_tile_grid::{FigureTileDraft, FigureTileFrame, FigureTileSource};
pub use semio_s_artifact_animate_presentation_inferences::{PRESENTATION_DOCUMENT_SCHEMA, default_figure_tile_source, genesis_presentation_child_pack, presentation_snapshot_with_tiles, presentation_working_scene};
pub use semio_s_artifact_animate_presentation_tests::{default_presentation_snapshot};
pub use crate::viewer::animate::artifact_items_2::{ANIMATE_DIALECT};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
