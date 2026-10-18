//! 🖼️ Layout app engine — display-list construction, glyph layout, scene painting, hit-testing and
//! export (SVG/PDF/PNG/zip). Sibling topic file of `🦀️.rs` (headless compute is split across the
//! two purely because of size — see the master template's "+ sibling topic file for big engines" note).
//!
//! Relocated wholesale from the deleted artifact-tree `⚙️engine/🎬️scene` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): `LayoutEngine` owns `&mut self` rendering
//! state (font/layout contexts) threaded through canvas, pointer, wasm and window-render call sites
//! across this app — the textbook D5 Behavioral case, and one of the two `*Engine` structs in this
//! ticket that IS constructed outside its own file (see the region → destination map's exception
//! clause). `parse_layout_document`/`resolve_page` (pure, artifact-level) stayed at `🧬️schema`;
//! `export_scene`/`rect_path_segments`/`LayoutError` (io/codec-dispatch territory) stayed
//! at `🚪️io` — this file reaches both by qualified path, which is the normal app→artifact direction.

use semio_s_artifact_layout_layout_model::LayoutError;
use semio_s_artifact_layout_layout_model::resolve_page;
use semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire2_codec::parse_layout_document;
use semio_s_artifact_layout_layout_model::{Frame, LayoutBounds, LayoutRect, Page, ParagraphStyle, TextStory};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_framework_io_ports::{VectorNode, VectorScene, VectorSegment, VectorStyle};
use semio_framework_canvas::camera::{self, Camera, Viewport};
use semio_framework_canvas::{Affine, BezPath, Color, FillRule, Line, Point, Rect, RoundedRect, RoundedRectRadii, Scene, Stroke, Vec2};
#[cfg(test)]
use serde_json::Value;
use std::sync::OnceLock;
use ui_render::{FontDependencyId, FontFamilyChoice, ShapedText, TextAlignment, TextFontError, TextFontRole, TextFontSet, TextRunStyle, TextStyle, TextSystem};
#[cfg(not(target_arch = "wasm32"))]
use ui_render::{TextFontFiles, TextFontReadError};

#[path = "🎨️display-list-and-hit-testing/🦀️.rs"]
mod display_list_and_hit_testing;
pub use display_list_and_hit_testing::*;

#[path = "🔤️text-fonts/🦀️.rs"]
mod text_fonts;
pub use text_fonts::*;

#[path = "⚙️layout-engine-and-views/🦀️.rs"]
mod layout_engine_and_views;
pub use layout_engine_and_views::*;

#[path = "✍️drawing-labels-and-rasters/🦀️.rs"]
mod drawing_labels_and_rasters;
use drawing_labels_and_rasters::*;

#[path = "🖌️drawing-strokes-and-paths/🦀️.rs"]
mod drawing_strokes_and_paths;
use drawing_strokes_and_paths::*;

#[path = "🧩️assemble-display-list/🦀️.rs"]
mod display_list_assembly;
use display_list_assembly::*;

#[path = "🔠️style-spans-and-preview-marks/🦀️.rs"]
mod style_spans_and_preview_marks;
use style_spans_and_preview_marks::*;

#[path = "🎬️scene-painting-and-queries/🦀️.rs"]
mod scene_painting_and_queries;
pub use scene_painting_and_queries::*;

//#region 📤️Export
#[cfg(test)]
#[cfg(test)] use semio_s_artifact_layout_layout_engine_export::export_document_pdf_headless_batch;
#[cfg(test)]
#[cfg(test)] use semio_s_artifact_layout_layout_engine_export::export_document_png_headless_batch;
#[cfg(test)]
#[cfg(test)] use semio_s_artifact_layout_layout_engine_export::export_document_svg_headless_batch;
#[cfg(test)]
#[cfg(test)] use semio_s_artifact_layout_layout_engine_export::export_package_zip_headless_batch;
//#endregion 📤️Export

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
