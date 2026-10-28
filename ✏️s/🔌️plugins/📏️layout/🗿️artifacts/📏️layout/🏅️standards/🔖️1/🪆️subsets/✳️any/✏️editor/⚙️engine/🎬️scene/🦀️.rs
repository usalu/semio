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
pub mod display_list_and_hit_testing;
use display_list_and_hit_testing::{DisplayColor, DisplayGlyph, DisplayGuide, DisplayImage, DisplayList, DisplayStroke, DisplayTextRun, StackPiece, bounds_to_display_rect, hit_test_page_frames, page_margin_guides, stack_pieces};
#[cfg(test)]
use display_list_and_hit_testing::DisplayRect;

#[path = "🔤️text-fonts/🦀️.rs"]
pub mod text_fonts;
use text_fonts::{LAYOUT_SANS, TEXT_FONTS, TextFontsInstallError};
#[cfg(not(target_arch = "wasm32"))]
use text_fonts::install_native_text_fonts;
#[cfg(test)]
use text_fonts::{TEXT_FONT_ASSET_ROOT, install_package_text_fonts, install_text_font_bytes, install_text_fonts, package_text_font_set, text_font_asset_declarations, text_font_asset_name};

#[path = "⚙️layout-engine-and-views/🦀️.rs"]
pub mod layout_engine_and_views;
use layout_engine_and_views::{LayoutEngine, build_display_list_for_page, default_paragraph, frame_layer_visible, layout_story_in_frame, outside_interactive_view};
#[cfg(test)]
use layout_engine_and_views::build_interactive_display_list;

#[path = "✍️drawing-labels-and-rasters/🦀️.rs"]
mod drawing_labels_and_rasters;
use drawing_labels_and_rasters::{DrawingLabel, background_scene, fit_from_points, place_drawing_labels, place_drawing_rasters, push_drawing_raster, raw_drawing_labels, raw_drawing_rasters, shape_drawing_label, stroke_points, unshaped_drawing_label};

#[path = "🖌️drawing-strokes-and-paths/🦀️.rs"]
mod drawing_strokes_and_paths;
pub use drawing_strokes_and_paths::drawing_kind;
use drawing_strokes_and_paths::{drawing_strokes, paint_display_stroke, place_drawing_in_frame, raw_drawing_strokes};

#[path = "🧩️assemble-display-list/🦀️.rs"]
mod display_list_assembly;
use display_list_assembly::assemble_display_list;

#[path = "🔠️style-spans-and-preview-marks/🦀️.rs"]
mod style_spans_and_preview_marks;
pub use style_spans_and_preview_marks::preview_mark;
use style_spans_and_preview_marks::{color_from, paint_preview_mark, placed_kind_glyphs, story_run_glyphs, style_spans};

#[path = "🎬️scene-painting-and-queries/🦀️.rs"]
pub mod scene_painting_and_queries;
#[cfg(test)]
use scene_painting_and_queries::{LayoutDropPreview, SceneQuery, build_scene_from_document_json, display_list_to_scene, hit_test_document_json, screen_to_world_json};

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

#[cfg(test)]
#[path = "🧪️tests/👁️layer-order/🦀️.rs"]
mod layer_order_hit_tests;

#[path="📄️paragraph/🦀️.rs"]
pub mod paragraph;
#[path = "🧵️flow/🦀️.rs"]
pub mod text_flow;

#[path="🧵️drawing/🦀️.rs"]
pub mod drawing_planning;

#[path="📇️metadata-index/🦀️.rs"]pub mod metadata_index;
