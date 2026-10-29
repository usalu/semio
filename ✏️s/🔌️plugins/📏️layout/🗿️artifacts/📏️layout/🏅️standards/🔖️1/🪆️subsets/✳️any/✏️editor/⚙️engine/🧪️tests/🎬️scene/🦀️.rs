//! 🧪️ Native scene, paragraph and Drawing laws exercise public engine interfaces and scoped test observations.
#![cfg(all(test, not(target_arch = "wasm32")))]
#![allow(unused_imports)]
extern crate semio_framework_os_kernel as protocol;
use semio_s_artifact_layout_layout_engine::scene::{self, text_fonts::*, layout_engine_and_views::*, display_list_and_hit_testing::*, scene_painting_and_queries::*, paragraph::*, text_flow::*, text_flow::projection::*, metadata_index::*, drawing_planning::*, drawing_planning::segments::*, drawing_planning::composition::*, drawing_planning::composition::canvas::*};
use semio_s_artifact_layout_layout_model::{Frame,LayoutBounds,LayoutRect,Page,ParagraphStyle,TextStory};
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire2_codec::parse_layout_document;
use semio_framework_canvas::camera::{Camera,Viewport};
use semio_framework_canvas::{Affine,BezPath,Color,FillRule,Line,Point,Rect,RoundedRect,RoundedRectRadii,Scene,Stroke,Vec2};
use semio_framework_io_ports::*;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep};
use semio_framework_2d_engine::PathSegment;
use serde_json::Value;
use ui_render::{TextSystem,TextStyle,TextRunStyle,TextAlignment,FontFamilyChoice,TextFontSet,TextFontFiles,TextFontRole};
use semio_s_artifact_layout_layout_engine_export::{export_document_pdf_headless_batch,export_document_png_headless_batch,export_document_svg_headless_batch,export_package_zip_headless_batch};
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[path="../../🎬️scene/🧪️tests/🔬️unit/🦀️.rs"] mod laws_0;
#[path="../../🎬️scene/🧪️tests/👁️layer-order/🦀️.rs"] mod laws_1;
#[path="../../🎬️scene/📄️paragraph/🧪️tests/🦀️.rs"] mod laws_2;
#[path="../../🎬️scene/🧵️drawing/🧪️tests/🧵️graph.rs"] mod laws_3;
#[path="../../🎬️scene/🧵️drawing/🖊️segments/🧪️tests/🦀️.rs"] mod laws_4;
#[path="../../🎬️scene/🧵️drawing/🖼️composition/🧪️tests/🦀️.rs"] mod laws_5;
#[path="../../🎬️scene/🧵️drawing/🖼️composition/📤️canvas/🧪️tests/🦀️.rs"] mod laws_6;
