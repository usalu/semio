//! 🧩 Directed port graph board types shared by normal and dag leaves.

use std::collections::{BTreeMap, BTreeSet};

use crate::cavas::vello::kurbo::{Point, Vec2};
use crate::cavas::vello::peniko::Color;
use crate::cavas::camera::Camera;
use crate::NodeKindHandleTemplate;

// #region 🔖GraphPortMode
/// 🔌 Runtime port-model axis: ported graphs use handles; normal graphs connect node ids directly.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum GraphPortMode {
    #[default]
    Ported,
    Normal,
}

impl GraphPortMode {
    pub fn has_ports(self) -> bool {
        self == GraphPortMode::Ported
    }
}
// #endregion 🔖GraphPortMode

pub use mathematical_graph::NodeShape;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardElementStyleKind {
    Original,
    Neutral,
    Hovered,
    Selected,
    Highlighted,
    Disabled,
}

#[derive(Clone, Debug)]
pub struct NodeData {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub shape: NodeShape,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
    pub scale: f64,
    pub draggable: bool,
    pub selected: bool,
    pub visible: bool,
    pub root: bool,
    pub style: Option<String>,
    pub text: Option<String>,
    pub icon_kind: Option<String>,
    pub node_kind: String,
}

#[derive(Clone, Debug)]
pub struct WireKindDef {
    pub name: String,
    pub default_edge_kind: Option<String>,
}

#[derive(Clone, Debug)]
pub struct NodeKindDef {
    pub name: String,
    pub scale: f64,
    pub shape: NodeShape,
    pub handles: Vec<NodeKindHandleTemplate>,
    pub icon: Option<String>,
    pub color_fill: Option<Color>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveTool {
    Select,
    Brush,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeStrokePattern {
    Solid,
    Dashed,
    Dotted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeTipGeometry {
    Arrow,
    FineArrow,
    Diamond,
    Circle,
    Bar,
}

#[derive(Clone, Debug)]
pub struct EdgeTipDef {
    pub geometry: EdgeTipGeometry,
    pub filled: bool,
    pub scale: f64,
}

impl EdgeTipDef {
    pub fn from_catalog_row(eo: &serde_json::Map<String, serde_json::Value>) -> Option<Self> {
        let id = eo.get("id").and_then(|x| x.as_str()).unwrap_or("");
        if eo.get("geometry").is_none() {
            return Self::builtin_for_id(id);
        }
        let geometry = match eo.get("geometry").and_then(|x| x.as_str()).map(str::trim) {
            Some("arrow") => EdgeTipGeometry::Arrow,
            Some("fine-arrow") | Some("fine_arrow") => EdgeTipGeometry::FineArrow,
            Some("diamond") => EdgeTipGeometry::Diamond,
            Some("circle") => EdgeTipGeometry::Circle,
            Some("bar") => EdgeTipGeometry::Bar,
            _ => return None,
        };
        let filled = eo.get("filled").and_then(|x| x.as_bool()).unwrap_or_else(|| match geometry {
            EdgeTipGeometry::FineArrow | EdgeTipGeometry::Bar => false,
            EdgeTipGeometry::Diamond => eo.get("id").and_then(|x| x.as_str()).is_some_and(|id| id.contains("open")),
            _ => true,
        });
        let scale = eo
            .get("scale")
            .and_then(|x| x.as_f64())
            .filter(|v| v.is_finite() && *v > 0.0)
            .unwrap_or(1.0);
        Some(Self { geometry, filled, scale })
    }

    pub fn builtin_for_id(id: &str) -> Option<Self> {
        match id.trim().to_ascii_lowercase().as_str() {
            "arrow" | "filled-arrow" | "filled_arrow" => Some(Self { geometry: EdgeTipGeometry::Arrow, filled: true, scale: 1.0 }),
            "fine-arrow" | "fine_arrow" => Some(Self { geometry: EdgeTipGeometry::FineArrow, filled: false, scale: 1.0 }),
            "filled-diamond" | "filled_diamond" => Some(Self { geometry: EdgeTipGeometry::Diamond, filled: true, scale: 1.0 }),
            "open-diamond" | "open_diamond" => Some(Self { geometry: EdgeTipGeometry::Diamond, filled: false, scale: 1.0 }),
            _ => None,
        }
    }
}

pub fn builtin_edge_tips() -> BTreeMap<String, EdgeTipDef> {
    let ids = ["arrow", "filled-arrow", "fine-arrow", "filled-diamond", "open-diamond"];
    let mut m = BTreeMap::new();
    for id in ids {
        if let Some(def) = EdgeTipDef::builtin_for_id(id) {
            m.insert(id.to_string(), def);
        }
    }
    m
}

#[derive(Clone, Debug)]
pub struct EdgeKindDef {
    pub name: String,
    pub color: Option<Color>,
    pub stroke_width: f64,
    pub pattern: EdgeStrokePattern,
    pub source_tip: Option<String>,
    pub target_tip: Option<String>,
    pub directed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompatSpecificity {
    General = 0,
    Node = 1,
    Edge = 2,
    Handle = 3,
    Wire = 4,
}

#[derive(Clone, Debug)]
pub struct LinkCompatRule {
    pub source: String,
    pub target: String,
    pub bidirectional: bool,
    pub important: bool,
    pub specificity: CompatSpecificity,
}

#[derive(Clone, Debug)]
pub struct EdgeData {
    pub id: String,
    pub source: String,
    pub target: String,
    pub selected: bool,
    pub visible: bool,
    pub style: Option<String>,
    pub edge_kind: String,
    pub source_tip: Option<String>,
    pub target_tip: Option<String>,
}

#[derive(Clone, Debug)]
pub struct WireData {
    pub id: String,
    pub source: String,
    pub target: Option<String>,
    pub end_x: Option<f64>,
    pub end_y: Option<f64>,
    pub selected: bool,
    pub visible: bool,
    pub style: Option<String>,
    pub wire_kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectionOptions {
    pub method: String,
    pub mode: String,
    pub select_nodes: bool,
    pub select_edges: bool,
    pub select_handles: bool,
}

#[derive(Clone, Debug)]
pub enum Interaction {
    None,
    Pan {
        origin: Camera,
        start_screen: Point,
    },
    DragNodes {
        offset: Vec2,
        primary_id: String,
        start_positions: BTreeMap<String, (f64, f64)>,
        proximity_pair: Option<(String, String)>,
    },
    SelectionPending {
        initial_ids: BTreeSet<String>,
        start: Point,
        start_screen: Point,
    },
    Selection {
        initial_ids: BTreeSet<String>,
        points: Vec<Point>,
        screen_points: Vec<Point>,
        start: Point,
        start_screen: Point,
    },
    LinkAtSourceHandle {
        source_id: String,
        start_screen: Point,
    },
    LinkDragSnap {
        source_id: String,
        target_id: Option<String>,
        end_world: Point,
    },
    LinkTargetNode {
        source_id: String,
        target_node_id: String,
    },
    ExternalLinkPreview {
        source_id: String,
        end_world: Point,
        compatible_node_ids: Vec<String>,
        ring_node_id: Option<String>,
        ring_handle_ids: Vec<String>,
    },
}

impl Default for Interaction {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VelloThemePalette {
    pub raster_clear: Color,
    pub grid_minor_stroke: Color,
    pub edge_stroke: Color,
    pub edge_stroke_hovered: Color,
    pub edge_stroke_selected: Color,
    pub edge_stroke_selection_exit: Color,
    pub edge_stroke_disabled: Color,
    pub node_fill: Color,
    pub node_stroke: Color,
    pub node_fill_hovered: Color,
    pub node_stroke_hovered: Color,
    pub node_fill_selected: Color,
    pub node_stroke_selected: Color,
    pub node_fill_selection_exit: Color,
    pub node_stroke_selection_exit: Color,
    pub node_fill_disabled: Color,
    pub node_stroke_disabled: Color,
    pub indirect_handle_fill: Color,
    pub indirect_handle_stroke: Color,
    pub handle_fill: Color,
    pub handle_stroke: Color,
    pub handle_fill_hovered: Color,
    pub handle_stroke_hovered: Color,
    pub handle_fill_selected: Color,
    pub handle_stroke_selected: Color,
    pub handle_fill_selection_exit: Color,
    pub handle_stroke_selection_exit: Color,
    pub handle_fill_disabled: Color,
    pub handle_stroke_disabled: Color,
    pub wire_stroke: Color,
    pub wire_stroke_hovered: Color,
    pub wire_stroke_selected: Color,
    pub wire_stroke_highlighted: Color,
    pub wire_stroke_disabled: Color,
    pub selection_preview_fill: Color,
    pub selection_preview_stroke: Color,
    pub label_fill: Color,
    pub label_fill_hovered: Color,
    pub label_halo: Color,
}

impl VelloThemePalette {
    fn color_from_json_rgba8(arr: &[serde_json::Value]) -> Option<Color> {
        let r = u8::try_from(arr.get(0)?.as_u64().unwrap_or(0).min(255)).ok()?;
        let g = u8::try_from(arr.get(1)?.as_u64().unwrap_or(0).min(255)).ok()?;
        let b = u8::try_from(arr.get(2)?.as_u64().unwrap_or(0).min(255)).ok()?;
        let a = u8::try_from(arr.get(3).and_then(|x| x.as_u64()).unwrap_or(255).min(255)).ok()?;
        Some(Color::from_rgba8(r, g, b, a))
    }

    fn merge_color_field(next: &mut Color, v: &serde_json::Value, key: &str) {
        if let Some(arr) = v.get(key).and_then(|x| x.as_array()) {
            if let Some(c) = Self::color_from_json_rgba8(arr) {
                *next = c;
            }
        }
    }

    /// @emoji 🎨 Replaces this palette from the React host UI theme JSON payload.
    pub fn merge_from_json(&mut self, json: &str) -> Result<(), String> {
        let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let mut next = Self::default();
        Self::merge_color_field(&mut next.raster_clear, &v, "rasterClear");
        Self::merge_color_field(&mut next.grid_minor_stroke, &v, "gridMinorStroke");
        Self::merge_color_field(&mut next.edge_stroke, &v, "edgeStroke");
        Self::merge_color_field(&mut next.edge_stroke_hovered, &v, "edgeStrokeHovered");
        Self::merge_color_field(&mut next.edge_stroke_selected, &v, "edgeStrokeSelected");
        Self::merge_color_field(&mut next.edge_stroke_selection_exit, &v, "edgeStrokeSelectionExit");
        Self::merge_color_field(&mut next.edge_stroke_disabled, &v, "edgeStrokeDisabled");
        Self::merge_color_field(&mut next.node_fill, &v, "nodeFill");
        Self::merge_color_field(&mut next.node_stroke, &v, "nodeStroke");
        Self::merge_color_field(&mut next.node_fill_hovered, &v, "nodeFillHovered");
        Self::merge_color_field(&mut next.node_stroke_hovered, &v, "nodeStrokeHovered");
        Self::merge_color_field(&mut next.node_fill_selected, &v, "nodeFillSelected");
        Self::merge_color_field(&mut next.node_stroke_selected, &v, "nodeStrokeSelected");
        Self::merge_color_field(&mut next.node_fill_selection_exit, &v, "nodeFillSelectionExit");
        Self::merge_color_field(&mut next.node_stroke_selection_exit, &v, "nodeStrokeSelectionExit");
        Self::merge_color_field(&mut next.node_fill_disabled, &v, "nodeFillDisabled");
        Self::merge_color_field(&mut next.node_stroke_disabled, &v, "nodeStrokeDisabled");
        Self::merge_color_field(&mut next.indirect_handle_fill, &v, "indirectHandleFill");
        Self::merge_color_field(&mut next.indirect_handle_stroke, &v, "indirectHandleStroke");
        Self::merge_color_field(&mut next.handle_fill, &v, "handleFill");
        Self::merge_color_field(&mut next.handle_stroke, &v, "handleStroke");
        Self::merge_color_field(&mut next.handle_fill_hovered, &v, "handleFillHovered");
        Self::merge_color_field(&mut next.handle_stroke_hovered, &v, "handleStrokeHovered");
        Self::merge_color_field(&mut next.handle_fill_selected, &v, "handleFillSelected");
        Self::merge_color_field(&mut next.handle_stroke_selected, &v, "handleStrokeSelected");
        Self::merge_color_field(&mut next.handle_fill_selection_exit, &v, "handleFillSelectionExit");
        Self::merge_color_field(&mut next.handle_stroke_selection_exit, &v, "handleStrokeSelectionExit");
        Self::merge_color_field(&mut next.handle_fill_disabled, &v, "handleFillDisabled");
        Self::merge_color_field(&mut next.handle_stroke_disabled, &v, "handleStrokeDisabled");
        Self::merge_color_field(&mut next.wire_stroke, &v, "wireStroke");
        Self::merge_color_field(&mut next.wire_stroke_hovered, &v, "wireStrokeHovered");
        Self::merge_color_field(&mut next.wire_stroke_selected, &v, "wireStrokeSelected");
        Self::merge_color_field(&mut next.wire_stroke_highlighted, &v, "wireStrokeHighlighted");
        Self::merge_color_field(&mut next.wire_stroke_disabled, &v, "wireStrokeDisabled");
        Self::merge_color_field(&mut next.selection_preview_fill, &v, "selectionPreviewFill");
        Self::merge_color_field(&mut next.selection_preview_stroke, &v, "selectionPreviewStroke");
        Self::merge_color_field(&mut next.label_fill, &v, "labelFill");
        Self::merge_color_field(&mut next.label_fill_hovered, &v, "labelFillHovered");
        Self::merge_color_field(&mut next.label_halo, &v, "labelHalo");
        *self = next;
        Ok(())
    }
}

// #region 🔖Icons
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use crate::cavas::usvg;
use crate::cavas::vello::kurbo::{Affine, Rect};
use crate::cavas::vello::peniko::{Blob, Fill, ImageAlphaType, ImageBrush, ImageData, ImageFormat};
use crate::cavas::vello::Scene;

#[derive(Clone)]
pub enum CachedIconBody {
    Vector(Scene),
    Raster(Arc<ImageData>),
}

#[derive(Clone)]
struct CachedIconPaint {
    bx: f64,
    by: f64,
    bw: f64,
    bh: f64,
    body: CachedIconBody,
}

/// 🖼️ Shared SVG/raster icon decode cache for board and DAG hosts.
pub struct IconPaintCache {
    cache: RefCell<HashMap<String, CachedIconPaint>>,
    pub themed_icon_lookup: infinite_cavas::icon_codec::ThemedSvgLookup,
}

impl Default for IconPaintCache {
    fn default() -> Self {
        Self { cache: RefCell::new(HashMap::new()), themed_icon_lookup: |_| None }
    }
}

impl Clone for IconPaintCache {
    fn clone(&self) -> Self {
        Self { cache: RefCell::new(HashMap::new()), themed_icon_lookup: self.themed_icon_lookup }
    }
}

impl IconPaintCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&self) {
        self.cache.borrow_mut().clear();
    }

    fn icon_vector_cache_key(tag: &str, svg: &str, fg: Color, bg: Color) -> String {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        svg.hash(&mut hasher);
        let hx = hasher.finish();
        let f = fg.to_rgba8();
        let b = bg.to_rgba8();
        format!(
            "v8|{tag}|{hx:x}|{}|{:02x}{:02x}{:02x}{:02x}|{:02x}{:02x}{:02x}{:02x}",
            svg.len(),
            f.r,
            f.g,
            f.b,
            f.a,
            b.r,
            b.g,
            b.b,
            b.a
        )
    }

    fn icon_raster_cache_key(rgba: &Arc<[u8]>, w: u32, h: u32) -> String {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        rgba.as_ref().hash(&mut hasher);
        let hx = hasher.finish();
        format!("v8|r|{w}x{h}|{hx:x}|{}", rgba.len())
    }

    pub fn get_or_build(&self, encoded: &str, fg: Color, bg: Color, preserve_original_style: bool) -> Option<(f64, f64, f64, f64, CachedIconBody)> {
        let resolved = infinite_cavas::icon_codec::board_resolve_icon_kind(encoded, self.themed_icon_lookup);
        let key = match &resolved {
            infinite_cavas::icon_codec::BoardResolvedIcon::None => return None,
            infinite_cavas::icon_codec::BoardResolvedIcon::SvgThemed(s) | infinite_cavas::icon_codec::BoardResolvedIcon::SvgPlain(s) => {
                Self::icon_vector_cache_key(if preserve_original_style { "p" } else { "t" }, s.as_str(), fg, bg)
            }
            infinite_cavas::icon_codec::BoardResolvedIcon::RasterRgba8 { rgba, w, h } => Self::icon_raster_cache_key(rgba, *w, *h),
        };
        {
            let g = self.cache.borrow();
            if let Some(c) = g.get(&key) {
                return Some((c.bx, c.by, c.bw, c.bh, c.body.clone()));
            }
        }
        let (bx, by, bw, bh, body) = match resolved {
            infinite_cavas::icon_codec::BoardResolvedIcon::None => return None,
            infinite_cavas::icon_codec::BoardResolvedIcon::SvgThemed(s) => {
                let tree = usvg::Tree::from_str(s.trim(), infinite_cavas::svg_icon_vello09::usvg_options_icons()).ok()?;
                let (bx, by, bw, bh) = infinite_cavas::svg_icon_vello09::svg_icon_content_bounds(&tree);
                if !(bw > 0.0 && bh > 0.0 && bw.is_finite() && bh.is_finite()) {
                    return None;
                }
                let mut s = Scene::new();
                if preserve_original_style {
                    let _ = infinite_cavas::vello_svg::append_tree(&mut s, &tree);
                } else {
                    infinite_cavas::svg_icon_vello09::render_svg_tree_themed(&mut s, &tree, fg, bg);
                }
                (bx, by, bw, bh, CachedIconBody::Vector(s))
            }
            infinite_cavas::icon_codec::BoardResolvedIcon::SvgPlain(s) => {
                let svg_t = s.trim();
                let tree = usvg::Tree::from_str(svg_t, infinite_cavas::svg_icon_vello09::usvg_options_icons()).ok()?;
                let (bx, by, bw, bh) = infinite_cavas::svg_icon_vello09::svg_icon_content_bounds(&tree);
                if !(bw > 0.0 && bh > 0.0 && bw.is_finite() && bh.is_finite()) {
                    return None;
                }
                let mut s = Scene::new();
                if preserve_original_style {
                    let _ = infinite_cavas::vello_svg::append_tree(&mut s, &tree);
                } else {
                    infinite_cavas::svg_icon_vello09::render_svg_tree_themed(&mut s, &tree, fg, bg);
                }
                (bx, by, bw, bh, CachedIconBody::Vector(s))
            }
            infinite_cavas::icon_codec::BoardResolvedIcon::RasterRgba8 { rgba, w, h } => {
                let bx = 0.0_f64;
                let by = 0.0_f64;
                let bw = f64::from(w);
                let bh = f64::from(h);
                let img = ImageData {
                    data: Blob::new(Arc::new(rgba.as_ref().to_vec())),
                    format: ImageFormat::Rgba8,
                    alpha_type: ImageAlphaType::Alpha,
                    width: w,
                    height: h,
                };
                (bx, by, bw, bh, CachedIconBody::Raster(Arc::new(img)))
            }
        };
        let cached = CachedIconPaint { bx, by, bw, bh, body: body.clone() };
        self.cache.borrow_mut().insert(key, cached);
        Some((bx, by, bw, bh, body))
    }

    /// @emoji 🖼️ Paints an icon centered in a screen-space rectangle.
    pub fn append_icon_at_screen_rect(
        &self,
        scene: &mut Scene,
        icon_kind: &str,
        center: Point,
        avail_w: f64,
        avail_h: f64,
        fg: Color,
        bg: Color,
        preserve_original_style: bool,
    ) {
        let Some((bx, by, bw, bh, body)) = self.get_or_build(icon_kind, fg, bg, preserve_original_style) else {
            return;
        };
        if !(avail_w > 0.0 && avail_h > 0.0) {
            return;
        }
        let fit_inset = 0.76;
        let sx_half = avail_w * fit_inset * 0.5;
        let sy_half = avail_h * fit_inset * 0.5;
        let cx = bx + bw * 0.5;
        let cy = by + bh * 0.5;
        let scale = (2.0 * sx_half / bw).min(2.0 * sy_half / bh);
        let aff = Affine::translate((center.x - scale * cx, center.y - scale * cy)) * Affine::scale(scale);
        let clip_inset = 0.88;
        let hw = avail_w * clip_inset * 0.5;
        let hh = avail_h * clip_inset * 0.5;
        let clip_r = Rect::from_points(Point::new(center.x - hw, center.y - hh), Point::new(center.x + hw, center.y + hh));
        scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &clip_r);
        match &body {
            CachedIconBody::Vector(icon_scene) => {
                scene.append(icon_scene, Some(aff));
            }
            CachedIconBody::Raster(img) => {
                scene.draw_image(&ImageBrush::new((**img).clone()), aff);
            }
        }
        scene.pop_layer();
    }
}
// #endregion 🔖Icons

impl Default for VelloThemePalette {
    fn default() -> Self {
        Self {
            raster_clear: Color::from_rgba8(248, 248, 250, 255),
            grid_minor_stroke: Color::from_rgba8(160, 160, 170, 56),
            edge_stroke: Color::from_rgba8(120, 120, 130, 255),
            edge_stroke_hovered: Color::from_rgba8(123, 130, 125, 255),
            edge_stroke_selected: Color::from_rgba8(0, 17, 23, 255),
            edge_stroke_selection_exit: Color::from_rgba8(80, 140, 110, 255),
            edge_stroke_disabled: Color::from_rgba8(160, 160, 170, 56),
            node_fill: Color::from_rgba8(235, 238, 245, 255),
            node_stroke: Color::from_rgba8(123, 130, 125, 255),
            node_fill_hovered: Color::from_rgba8(123, 130, 125, 255),
            node_stroke_hovered: Color::from_rgba8(123, 130, 125, 255),
            node_fill_selected: Color::from_rgba8(60, 120, 220, 89),
            node_stroke_selected: Color::from_rgba8(0, 17, 23, 255),
            node_fill_selection_exit: Color::from_rgba8(196, 228, 213, 255),
            node_stroke_selection_exit: Color::from_rgba8(80, 140, 110, 255),
            node_fill_disabled: Color::from_rgba8(235, 238, 245, 255),
            node_stroke_disabled: Color::from_rgba8(160, 160, 170, 56),
            indirect_handle_fill: Color::from_rgba8(196, 228, 213, 255),
            indirect_handle_stroke: Color::from_rgba8(80, 140, 110, 255),
            handle_fill: Color::from_rgba8(123, 130, 125, 0),
            handle_stroke: Color::from_rgba8(123, 130, 125, 255),
            handle_fill_hovered: Color::from_rgba8(123, 130, 125, 255),
            handle_stroke_hovered: Color::from_rgba8(123, 130, 125, 255),
            handle_fill_selected: Color::from_rgba8(60, 120, 220, 89),
            handle_stroke_selected: Color::from_rgba8(0, 17, 23, 255),
            handle_fill_selection_exit: Color::from_rgba8(196, 228, 213, 255),
            handle_stroke_selection_exit: Color::from_rgba8(80, 140, 110, 255),
            handle_fill_disabled: Color::from_rgba8(248, 248, 250, 255),
            handle_stroke_disabled: Color::from_rgba8(160, 160, 170, 56),
            wire_stroke: Color::from_rgba8(120, 120, 130, 255),
            wire_stroke_hovered: Color::from_rgba8(123, 130, 125, 255),
            wire_stroke_selected: Color::from_rgba8(0, 17, 23, 255),
            wire_stroke_highlighted: Color::from_rgba8(80, 140, 110, 255),
            wire_stroke_disabled: Color::from_rgba8(160, 160, 170, 56),
            selection_preview_fill: Color::from_rgba8(60, 120, 220, 40),
            selection_preview_stroke: Color::from_rgba8(0, 17, 23, 180),
            label_fill: Color::from_rgba8(123, 130, 125, 255),
            label_fill_hovered: Color::from_rgba8(0, 17, 23, 255),
            label_halo: Color::from_rgba8(0, 0, 0, 0),
        }
    }
}
