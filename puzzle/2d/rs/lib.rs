//! 🧩 Puzzle 2d board: elements palette, icon codec, `BoardHost`, WASM session on `mathematical_graph` + `infinite_cavas`.
#![allow(clippy::missing_errors_doc, reason = "Puzzle board bundle is internal to puzzle 2d.")]

pub use infinite_cavas::{self as cavas, *};
pub use cavas::vello::kurbo::{CubicBez, Point, Vec2};
pub use mathematical_graph_normal_undirected::{
    apply_force_graph_layout_to_fixture_v1_json as apply_undirected_force_graph_layout_to_fixture_v1_json,
    apply_force_graph_layout_to_fixture_v1_value as apply_undirected_force_graph_layout_to_fixture_v1_value,
    apply_redraw_layout_to_fixture_v1_json as apply_normal_undirected_redraw_layout_to_fixture_v1_json, ForceGraphLayoutOptions,
};
pub use mathematical_graph_port_directed::{self as graph, handle_position, BoardEngine, BoardEvent, Camera, Edge, EdgeId, Handle, HandleId, InteractionMode, Node, NodeId, RenderSnapshot, Selection};
pub use graph::{
    apply_edge_handle_snap_to_fixture_v1_json, apply_force_graph_layout_to_fixture_v1_json, apply_force_graph_layout_to_fixture_v1_value,
    apply_redraw_layout_to_fixture_v1_json as apply_ported_redraw_layout_to_fixture_v1_json, GraphExtension,
};
pub use gis_map as map;
pub use reasoning_mindmap as mindmap;

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn is_undirected_fixture_schema(schema: &str) -> bool {
    matches!(schema, "reasoning.mindmap.fixture/v1" | "reasoning.wires.fixture/v1")
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn redraw_layout_fixture_json(fixture_json: &str, options_json: &str) -> Result<String, String> {
    let fixture: serde_json::Value = serde_json::from_str(fixture_json).map_err(|e| e.to_string())?;
    let schema = fixture.get("schema").and_then(|v| v.as_str()).unwrap_or("");
    let opts: serde_json::Value = serde_json::from_str(options_json).map_err(|e| e.to_string())?;
    let mode = opts.get("mode").and_then(|v| v.as_str()).unwrap_or("force-graph");
    if mode == "force-graph" && is_undirected_fixture_schema(schema) {
        apply_normal_undirected_redraw_layout_to_fixture_v1_json(fixture_json, options_json)
    } else {
        apply_ported_redraw_layout_to_fixture_v1_json(fixture_json, options_json)
    }
}

pub use vello_svg::usvg;
pub use vello_svg::vello;

mod elements_board_palette {
    use crate::vello::peniko::Color;
    include!(concat!(env!("OUT_DIR"), "/elements_styling_board.rs"));
}

mod board_metabolism_icons {
    include!(concat!(env!("OUT_DIR"), "/board_metabolism_icon_match.rs"));
}

fn resolve_node_icon_svg_from_encoding(encoded: &str) -> Option<String> {
    let t = encoded.trim();
    if t.is_empty() {
        return None;
    }
    if let Some(s) = board_metabolism_icons::board_metabolism_icon_svg(t) {
        return Some(s.to_string());
    }
    let lower = t.to_ascii_lowercase();
    if lower.starts_with("<?xml") || lower.contains("<svg") {
        return Some(t.to_string());
    }
    None
}

mod board_icon_codec {
    use base64::Engine as _;
    use std::path::PathBuf;
    use std::sync::{Arc, OnceLock};
    use typst::foundations::{Bytes, Datetime};
    use typst::layout::{Abs, PagedDocument};
    use typst::syntax::{FileId, Source, VirtualPath};
    use typst::text::Font;
    use typst::utils::LazyHash;
    use typst::Library;
    use typst::LibraryExt;
    use typst::World;

    #[derive(Debug)]
    pub enum BoardResolvedIcon {
        None,
        SvgThemed(String),
        SvgPlain(String),
        RasterRgba8 { rgba: Arc<[u8]>, w: u32, h: u32 },
    }

    struct RgbaImage {
        data: Arc<[u8]>,
        w: u32,
        h: u32,
    }

    fn decode_raster_icon_bytes(t: &str) -> Option<RgbaImage> {
        let s = t.trim().strip_prefix("image:").unwrap_or(t.trim()).trim();
        let rest = s.strip_prefix("data:image/png;base64,").or_else(|| s.strip_prefix("data:image/jpeg;base64,")).or_else(|| s.strip_prefix("data:image/jpg;base64,"))?;
        let raw = base64::engine::general_purpose::STANDARD.decode(rest.trim()).ok()?;
        let img = image::load_from_memory(&raw).ok()?;
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();
        if w == 0 || h == 0 {
            return None;
        }
        Some(RgbaImage { data: Arc::from(rgba.into_raw().into_boxed_slice()), w, h })
    }

    fn typst_asset_font_list() -> Vec<Font> {
        let mut out = Vec::new();
        for bytes in typst_assets::fonts() {
            let blob = Bytes::new(bytes);
            let mut idx = 0u32;
            loop {
                if let Some(f) = Font::new(blob.clone(), idx) {
                    out.push(f);
                    idx = idx.saturating_add(1);
                } else {
                    break;
                }
            }
        }
        out
    }

    fn typst_asset_font_list_plus_noto_color_emoji() -> Vec<Font> {
        let mut out = typst_asset_font_list();
        let emoji_blob = Bytes::new(crate::cavas::board_icon_assets::NOTO_COLOR_EMOJI_SUBSET_TTF);
        let mut idx = 0u32;
        loop {
            if let Some(f) = Font::new(emoji_blob.clone(), idx) {
                out.push(f);
                idx = idx.saturating_add(1);
            } else {
                break;
            }
        }
        out
    }

    fn board_typst_compile_markup_to_svg(markup: &str, fonts: &'static [Font], book: &'static LazyHash<typst::text::FontBook>) -> Option<String> {
        static LIB: OnceLock<LazyHash<Library>> = OnceLock::new();
        static MAIN: OnceLock<FileId> = OnceLock::new();
        let library = LIB.get_or_init(|| LazyHash::new(Library::default()));
        let main = *MAIN.get_or_init(|| FileId::new(None, VirtualPath::new("/board.typ")));
        let source = Source::new(main, markup.to_string());
        struct BoardTypstWorld<'a> {
            library: &'static LazyHash<Library>,
            book: &'static LazyHash<typst::text::FontBook>,
            main: FileId,
            source: Source,
            fonts: &'a [Font],
        }
        impl World for BoardTypstWorld<'_> {
            fn library(&self) -> &LazyHash<Library> {
                self.library
            }
            fn book(&self) -> &LazyHash<typst::text::FontBook> {
                self.book
            }
            fn main(&self) -> FileId {
                self.main
            }
            fn source(&self, id: FileId) -> typst::diag::FileResult<Source> {
                if id == self.main {
                    Ok(self.source.clone())
                } else {
                    Err(typst::diag::FileError::NotFound(PathBuf::from("board.typ")))
                }
            }
            fn file(&self, _id: FileId) -> typst::diag::FileResult<Bytes> {
                Err(typst::diag::FileError::NotFound(PathBuf::from("board.bin")))
            }
            fn font(&self, index: usize) -> Option<Font> {
                self.fonts.get(index).cloned()
            }
            fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
                None
            }
        }
        let w = BoardTypstWorld { library, book, main, source, fonts };
        let warned = typst::compile::<PagedDocument>(&w);
        let doc = warned.output.ok()?;
        if doc.pages.is_empty() {
            return None;
        }
        Some(typst_svg::svg_merged(&doc, Abs::pt(3.0)))
    }

    static TYPST_ASSET_FONTS: OnceLock<Vec<Font>> = OnceLock::new();
    static TYPST_ASSET_BOOK: OnceLock<LazyHash<typst::text::FontBook>> = OnceLock::new();
    // Noto Color Emoji (COLR) in the same `FontBook` as math broke `typst:` icon compiles; keep a second pool for `emoji:` only.
    static TYPST_ICON_EMOJI_FONTS: OnceLock<Vec<Font>> = OnceLock::new();
    static TYPST_ICON_EMOJI_BOOK: OnceLock<LazyHash<typst::text::FontBook>> = OnceLock::new();

    pub fn board_typst_markup_to_svg(markup: &str) -> Option<String> {
        let fonts = TYPST_ASSET_FONTS.get_or_init(typst_asset_font_list);
        let book = TYPST_ASSET_BOOK.get_or_init(|| LazyHash::new(typst::text::FontBook::from_fonts(fonts.iter())));
        board_typst_compile_markup_to_svg(markup, fonts.as_slice(), book)
    }

    fn board_typst_markup_to_svg_for_icon_emoji(markup: &str) -> Option<String> {
        let fonts = TYPST_ICON_EMOJI_FONTS.get_or_init(typst_asset_font_list_plus_noto_color_emoji);
        let book = TYPST_ICON_EMOJI_BOOK.get_or_init(|| LazyHash::new(typst::text::FontBook::from_fonts(fonts.iter())));
        board_typst_compile_markup_to_svg(markup, fonts.as_slice(), book)
    }

    pub fn board_resolve_icon_kind(encoded: &str) -> BoardResolvedIcon {
        let t = encoded.trim();
        if t.is_empty() {
            return BoardResolvedIcon::None;
        }
        if let Some(src) = t.strip_prefix("typst:") {
            let src = src.trim();
            if src.is_empty() {
                return BoardResolvedIcon::None;
            }
            let wrapped = format!("#set page(width: 96pt, height: 96pt, margin: 3pt, fill: none)\n{src}");
            return match board_typst_markup_to_svg(&wrapped) {
                Some(s) => BoardResolvedIcon::SvgPlain(s),
                None => BoardResolvedIcon::None,
            };
        }
        if let Some(em) = t.strip_prefix("emoji:") {
            let em = em.trim();
            if em.is_empty() {
                return BoardResolvedIcon::None;
            }
            let wrapped = format!("#set page(width: 88pt, height: 88pt, margin: 2pt, fill: none)\n#set align(center + horizon)\n#set text(size: 44pt, font: \"Noto Color Emoji\")\n{em}");
            return match board_typst_markup_to_svg_for_icon_emoji(&wrapped) {
                Some(s) => BoardResolvedIcon::SvgPlain(s),
                None => BoardResolvedIcon::None,
            };
        }
        if let Some(img) = decode_raster_icon_bytes(t) {
            return BoardResolvedIcon::RasterRgba8 { rgba: img.data, w: img.w, h: img.h };
        }
        if let Some(svg) = super::resolve_node_icon_svg_from_encoding(t) {
            if super::board_metabolism_icons::board_metabolism_icon_svg(t).is_some() {
                return BoardResolvedIcon::SvgThemed(svg);
            }
            return BoardResolvedIcon::SvgPlain(svg);
        }
        BoardResolvedIcon::None
    }
}

mod board_host {
    use crate::board_json_visible_option;
    use crate::elements_board_palette as board_palette;
    use crate::scene_json::*;
    use crate::usvg;
    use serde::Deserialize;
    use crate::vello::kurbo::{Affine, Circle, CubicBez, Point, Rect, Stroke, Vec2};
    use crate::vello::peniko::{Blob, Color, Fill, ImageAlphaType, ImageBrush, ImageData, ImageFormat};
    use crate::vello::Scene;
    use serde_json::json;
    use std::collections::{BTreeMap, BTreeSet};

    use crate::cavas::geom_sel::{
        cubic_bezier_axis_bounds, cubic_bezier_point, inflate_world_box, point_in_polygon, polygon_contains_world_box, polygon_intersects_world_box, segment_intersects_polygon, segment_intersects_world_box, world_box_contains_box,
        world_box_contains_point, world_box_from_points, world_boxes_overlap, WorldBox,
    };
    use crate::cavas::vcompute::{circle_handle_angle_toward, compute_edge_bezier_points, distance_between, distance_point_to_cubic_bezier, handle_position_on_circle, handle_position_on_rectangle, rectangle_handle_angle_toward};

    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};
    use std::sync::Arc;

    const GRID_WORLD_LARGE: f64 = 10.0;
    const GRID_WORLD_MEDIUM: f64 = 2.5;
    const GRID_WORLD_SMALL: f64 = 0.5;
    const GRID_WORLD_MICRO: f64 = 0.1;
    const GRID_FACTOR_DEFAULT: f64 = 10.0;
    const WORLD_CLIP_TILE_WORLD: f64 = 256.0;
    const MAX_WORLD_CLIP_TILES: u32 = 768;
    const EDGE_HIT_TOLERANCE_PX: f64 = 8.0;
    const HANDLE_HIT_TOLERANCE_PX: f64 = 10.0;
    const INDIRECT_HANDLE_MARKER_NODE_SCALE: f64 = 0.8;
    /// Radial offset from node rim to indirect-handle center, as a fraction of node half-extent (circle radius or half the shorter rectangle side).
    const INDIRECT_HANDLE_RING_GAP_NODE_SCALE: f64 = 0.7;
    const LINK_DRAG_MIN_DISTANCE_PX: f64 = 5.0;
    const LINK_HANDLE_SNAP_EXTRA_PX: f64 = 22.0;
    const LINK_COMMIT_SNAP_TIGHT_PX: f64 = 2.0;
    const DEFAULT_BRUSH_FLUSH_DISTANCE: f64 = 80.0;
    const DEFAULT_BRUSH_NODE_SIZE: f64 = 40.0;
    const SELECTION_LASSO_MIN_POINT_DISTANCE_PX: f64 = 3.0;
    const SELECTION_CLICK_MAX_DISTANCE_PX: f64 = 4.0;
    const BOUNDED_DRAG_HIT_PAD_PX: f64 = 8.0;
    pub use crate::cavas::camera::{CANVAS_CAMERA_ZOOM_MAX as BOARD_CAMERA_ZOOM_MAX, CANVAS_CAMERA_ZOOM_MIN as BOARD_CAMERA_ZOOM_MIN};
    const DEFAULT_WIRE_KIND_ID: &str = "wire.link";

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

    use crate::cavas::lod::{Lod, LodScale};

    const PUZZLE_2D_LODS: &[Lod; 6] = &[
        Lod {
            id: "minimap",
            name: "Minimap",
            description: "Whole-board silhouette; group selection and bounded drag only.",
            max_zoom: 0.15,
        },
        Lod {
            id: "overview",
            name: "Overview",
            description: "Topology and indirect handle rings; no per-node picks.",
            max_zoom: 0.35,
        },
        Lod {
            id: "compact",
            name: "Compact",
            description: "Dense graph layout with simplified chrome.",
            max_zoom: 0.55,
        },
        Lod {
            id: "normal",
            name: "Normal",
            description: "Standard editing: nodes, edges, and handle rings.",
            max_zoom: 1.25,
        },
        Lod {
            id: "detail",
            name: "Detail",
            description: "Node icons and richer strokes.",
            max_zoom: 2.5,
        },
        Lod {
            id: "micro",
            name: "Micro",
            description: "Maximum fidelity including handle icons.",
            max_zoom: f64::INFINITY,
        },
    ];

    const PUZZLE_2D_LOD_SCALE: LodScale = LodScale { lods: PUZZLE_2D_LODS };

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum BoardDrawLod {
        Minimap,
        Overview,
        Compact,
        Normal,
        Detail,
        Micro,
    }

    impl BoardDrawLod {
        fn label(self) -> &'static str {
            match self {
                Self::Minimap => "minimap",
                Self::Overview => "overview",
                Self::Compact => "compact",
                Self::Normal => "normal",
                Self::Detail => "detail",
                Self::Micro => "micro",
            }
        }

        fn from_id(id: &str) -> Option<Self> {
            Some(match id {
                "minimap" => Self::Minimap,
                "overview" => Self::Overview,
                "compact" => Self::Compact,
                "normal" => Self::Normal,
                "detail" => Self::Detail,
                "micro" => Self::Micro,
                _ => return None,
            })
        }

        fn from_scale_index(index: usize) -> Self {
            match index {
                0 => Self::Minimap,
                1 => Self::Overview,
                2 => Self::Compact,
                3 => Self::Normal,
                4 => Self::Detail,
                _ => Self::Micro,
            }
        }
    }

    pub fn puzzle_2d_lod_scale_json() -> String {
        let rows: Vec<serde_json::Value> = PUZZLE_2D_LODS
            .iter()
            .map(|lod| {
                serde_json::json!({
                    "id": lod.id,
                    "name": lod.name,
                    "description": lod.description,
                    "maxZoom": lod.max_zoom,
                })
            })
            .collect();
        serde_json::to_string(&rows).unwrap_or_else(|_| "[]".into())
    }

    #[derive(Clone)]
    enum CachedIconBody {
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NodeShape {
        Circle,
        Rectangle,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) enum BoardElementStyleKind {
        Original,
        Neutral,
        Hovered,
        Selected,
        Highlighted,
        Disabled,
    }

    /// @emoji 🎨 Whether drawable style resolves committed selection chrome or neutral cached geometry.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum StyleChromePass {
        CachedBase,
        InteractionOverlay,
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
        /// @emoji 🏷️ Runtime host encoding: catalog id from the baked icon table or inline SVG (`<?xml` / `<svg` …) parsed at detail LOD.
        pub icon_kind: Option<String>,
        pub node_kind: String,
    }

    #[derive(Clone, Debug)]
    pub struct HandleKindDef {
        pub name: String,
        pub color: Color,
        pub default_wire_kind: Option<String>,
        pub scale: f64,
    }

    #[derive(Clone, Debug)]
    pub struct WireKindDef {
        pub name: String,
        pub default_edge_kind: Option<String>,
    }

    #[derive(Clone, Debug)]
    pub struct NodeKindHandleTemplate {
        pub handle_kind: String,
        pub angle: f64,
        pub radius: Option<f64>,
    }

    #[derive(Clone, Debug)]
    pub struct NodeKindDef {
        pub name: String,
        pub scale: f64,
        pub shape: NodeShape,
        pub handles: Vec<NodeKindHandleTemplate>,
        /// @emoji 🏷️ Default WASM detail/micro icon encoding for instances of this node kind (`icon` in kind catalog JSON).
        pub icon: Option<String>,
        /// @emoji 🎨 Catalog fill when instance has no explicit fill override.
        pub color_fill: Option<Color>,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ActiveTool {
        Select,
        Brush,
    }

    #[derive(Clone, Debug)]
    struct BrushPreviewSnapshot {
        source_handle_id: String,
        node_kind_id: String,
        x: f64,
        y: f64,
        shape: NodeShape,
        radius: f64,
        width: f64,
        height: f64,
        handles: Vec<NodeKindHandleTemplate>,
        target_handle_index: usize,
        icon_kind: Option<String>,
    }

    #[derive(Clone, Debug)]
    struct FillVirtualNode {
        node_kind: String,
        x: f64,
        y: f64,
        shape: NodeShape,
        radius: f64,
        width: f64,
        height: f64,
    }

    #[derive(Clone, Debug)]
    struct FillVirtualHandle {
        node_id: String,
        handle_kind: String,
        template: NodeKindHandleTemplate,
    }

    #[derive(Clone, Debug, Default)]
    struct FillAccum {
        connected_handles: BTreeSet<String>,
        placements: Vec<(String, String, BrushPreviewSnapshot)>,
        virtual_nodes: HashMap<String, FillVirtualNode>,
        virtual_handles: HashMap<String, FillVirtualHandle>,
        virtual_bounds: Vec<WorldBox>,
        next_serial: u32,
    }

    #[derive(Clone, Debug)]
    struct FixtureDropPreviewSnapshot {
        node_kind_id: String,
        x: f64,
        y: f64,
        shape: NodeShape,
        radius: f64,
        width: f64,
        height: f64,
        icon_kind: Option<String>,
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
        fn from_catalog_row(eo: &serde_json::Map<String, serde_json::Value>) -> Option<Self> {
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

        fn builtin_for_id(id: &str) -> Option<Self> {
            match id.trim().to_ascii_lowercase().as_str() {
                "arrow" | "filled-arrow" | "filled_arrow" => Some(Self { geometry: EdgeTipGeometry::Arrow, filled: true, scale: 1.0 }),
                "fine-arrow" | "fine_arrow" => Some(Self { geometry: EdgeTipGeometry::FineArrow, filled: false, scale: 1.0 }),
                "filled-diamond" | "filled_diamond" => Some(Self { geometry: EdgeTipGeometry::Diamond, filled: true, scale: 1.0 }),
                "open-diamond" | "open_diamond" => Some(Self { geometry: EdgeTipGeometry::Diamond, filled: false, scale: 1.0 }),
                _ => None,
            }
        }
    }

    fn builtin_edge_tips() -> BTreeMap<String, EdgeTipDef> {
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
    pub struct HandleData {
        pub id: String,
        pub node_id: String,
        pub angle: f64,
        pub radius: f64,
        pub scale: f64,
        pub selected: bool,
        pub visible: bool,
        pub style: Option<String>,
        pub handle_kind: String,
        /// Parsed from descriptor `color` when set (overrides catalog fill).
        pub color_fill: Option<Color>,
        /// @emoji 🏷️ Runtime host encoding: `typst:`, `emoji:`, `image:data:…`, catalog id, or inline SVG for detail LOD.
        pub icon_kind: Option<String>,
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

    pub use crate::cavas::camera::Camera;

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
            /// @emoji 🧲 Preview/commit pair while an unconnected node overlaps a compatible target (`proximityConnect` on pointer-up).
            proximity_pair: Option<(String, String)>,
        },
        /// @emoji 🖱️ Background down before drag threshold; click-up deselects without preselect or exit chrome.
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
        /// @emoji 🔗 Host-driven link preview (cross-surface); pointer-up on ring handles can commit.
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
    }

    impl Default for VelloThemePalette {
        fn default() -> Self {
            Self {
                raster_clear: board_palette::RASTER_CLEAR,
                grid_minor_stroke: board_palette::GRID_MINOR_STROKE,
                edge_stroke: board_palette::EDGE_STROKE,
                edge_stroke_hovered: board_palette::NODE_STROKE,
                edge_stroke_selected: board_palette::EDGE_STROKE_SELECTED,
                edge_stroke_selection_exit: board_palette::INDIRECT_HANDLE_STROKE,
                edge_stroke_disabled: board_palette::GRID_MINOR_STROKE,
                node_fill: board_palette::NODE_FILL,
                node_stroke: board_palette::NODE_STROKE,
                node_fill_hovered: board_palette::NODE_FILL,
                node_stroke_hovered: board_palette::NODE_STROKE,
                node_fill_selected: board_palette::NODE_FILL_SELECTED,
                node_stroke_selected: board_palette::NODE_STROKE_SELECTED,
                node_fill_selection_exit: board_palette::INDIRECT_HANDLE_FILL,
                node_stroke_selection_exit: board_palette::INDIRECT_HANDLE_STROKE,
                node_fill_disabled: board_palette::NODE_FILL,
                node_stroke_disabled: board_palette::GRID_MINOR_STROKE,
                indirect_handle_fill: board_palette::INDIRECT_HANDLE_FILL,
                indirect_handle_stroke: board_palette::INDIRECT_HANDLE_STROKE,
                handle_fill: board_palette::HANDLE_FILL,
                handle_stroke: board_palette::HANDLE_STROKE,
                handle_fill_hovered: board_palette::HANDLE_FILL,
                handle_stroke_hovered: board_palette::HANDLE_STROKE,
                handle_fill_selected: board_palette::HANDLE_FILL_SELECTED,
                handle_stroke_selected: board_palette::HANDLE_STROKE_SELECTED,
                handle_fill_selection_exit: board_palette::INDIRECT_HANDLE_FILL,
                handle_stroke_selection_exit: board_palette::INDIRECT_HANDLE_STROKE,
                handle_fill_disabled: board_palette::HANDLE_FILL,
                handle_stroke_disabled: board_palette::GRID_MINOR_STROKE,
                wire_stroke: board_palette::EDGE_STROKE,
                wire_stroke_hovered: board_palette::NODE_STROKE,
                wire_stroke_selected: board_palette::EDGE_STROKE_SELECTED,
                wire_stroke_highlighted: board_palette::INDIRECT_HANDLE_STROKE,
                wire_stroke_disabled: board_palette::GRID_MINOR_STROKE,
                selection_preview_fill: board_palette::SELECTION_PREVIEW_FILL,
                selection_preview_stroke: board_palette::SELECTION_PREVIEW_STROKE,
            }
        }
    }

    #[derive(Clone)]
    pub struct BoardHost {
        pub camera: Camera,
        pub nodes: BTreeMap<String, NodeData>,
        pub handles: BTreeMap<String, HandleData>,
        pub edges: BTreeMap<String, EdgeData>,
        pub wires: BTreeMap<String, WireData>,
        /// Catalog keyed by `handle_kind` id (see `set_board_kind_catalogs_from_json`).
        pub handle_kinds: BTreeMap<String, HandleKindDef>,
        pub wire_kinds: BTreeMap<String, WireKindDef>,
        pub node_kinds: BTreeMap<String, NodeKindDef>,
        pub edge_kinds: BTreeMap<String, EdgeKindDef>,
        /// @emoji 🔺 Registry of edge tip shapes keyed by catalog id (built-ins seeded at init).
        pub edge_tips: BTreeMap<String, EdgeTipDef>,
        /// @emoji 🔗 Kind-compatibility rules for link gestures; empty = unrestricted.
        pub link_compat_rules: Vec<LinkCompatRule>,
        pub selection: BTreeSet<String>,
        /// @emoji 👁️ Live rectangle/lasso preview ids (committed selection stays in `selection` until pointer-up).
        pub preselect: BTreeSet<String>,
        /// @emoji 💠 During preselect: anchor selection \\ `preselect` (secondary chrome while dragging).
        pub preselect_removed: BTreeSet<String>,
        /// @emoji 💠 After commit: ids dropped in the last `select` transition only.
        pub selection_exit_highlight: BTreeSet<String>,
        pub selection_options: SelectionOptions,
        pub hovered_id: Option<String>,
        /// @emoji 🖱️ Transitive same-kind hover `(domain, kind_id)` when hovering a kind row or derived from `hovered_id`.
        pub hovered_kind: Option<(String, String)>,
        pub interaction: Interaction,
        pub width: u32,
        pub height: u32,
        pub dpr: f64,
        pub world_raster_tiling: String,
        pub events: Vec<serde_json::Value>,
        /// Screen-space preview polygon (CSS pixels) while area-selecting; cleared when idle.
        pub selection_screen_preview: Option<Vec<Point>>,
        /// @emoji ↔️ True when area-select drag is crossing (right-to-left); drives dashed preview stroke.
        pub selection_preview_crossing: bool,
        /// Screen-space polyline preview (CSS px) while dragging a handle link before drop.
        pub link_screen_preview: Option<Vec<Point>>,
        pub vello_theme: VelloThemePalette,
        /// @emoji 📐 Positive multiplier for LOD world grid steps (`10` / `5` / `1` base world units per band).
        pub grid_factor: f64,
        /// @emoji 🧲 When true, node drags snap to the finest visible LOD grid (step scales with `grid_factor`).
        pub grid_snap_enabled: bool,
        pub preserve_original_element_style: bool,
        /// @emoji 📶 When true (default), camera zoom selects draw LOD; when false, optional `forced_draw_lod` pins the tier when set.
        pub automatic_lod: bool,
        forced_draw_lod: Option<BoardDrawLod>,
        icon_vector_cache: RefCell<HashMap<String, CachedIconPaint>>,
        /// @emoji 📡 Dedupes {@code linkCompatibleNodes} emissions while a link wire is active.
        link_compat_nodes_emit_key: Option<String>,
        /// @emoji 📡 Dedupes {@code linkTargetRing} emissions while a link wire is active.
        link_target_ring_emit_key: Option<String>,
        /// @emoji 📡 Dedupes `select` emissions when ids are unchanged but modifier merge mode changes mid‑gesture.
        last_select_emit_sig: Option<(Vec<String>, Option<String>)>,
        /// @emoji 📡 Dedupes `preselect` emissions during area-select drag.
        last_preselect_emit_sig: Option<(Vec<String>, Vec<String>, Option<String>)>,
        /// @emoji 🧿 Bumped when drawable content changes (not camera); keys {@link BoardHost.world_content_cache}.
        content_scene_generation: u64,
        /// @emoji 🎨 World-space Vello content reused across pan/zoom when generation and LOD match.
        world_content_cache: RefCell<Option<(u64, BoardDrawLod, Scene)>>,
        /// @emoji 🔍 True while the wheel zoom gesture is active (skip grid + per-tile rebuild hot paths).
        wheel_zoom_active: bool,
        /// @emoji 📶 LOD tier pinned for the active wheel gesture so pan/zoom does not rebuild {@link BoardHost.world_content_cache} on every band crossing.
        wheel_zoom_render_lod: Option<BoardDrawLod>,
        /// @emoji 🖌️ Active viewport tool (`select` suppresses brush slot logic).
        active_tool: ActiveTool,
        brush_flush_distance: f64,
        brush_node_size: f64,
        brush_slot_source_id: Option<String>,
        brush_candidate_kinds: Vec<String>,
        brush_candidate_index: usize,
        brush_preview: Option<BrushPreviewSnapshot>,
        fixture_drop_preview: Option<FixtureDropPreviewSnapshot>,
        brush_candidates_emit_key: Option<String>,
        brush_preview_emit_key: Option<String>,
        brush_placement_serial: u64,
        brush_node_kind_weights: HashMap<String, f64>,
        brush_handle_kind_weights: HashMap<String, f64>,
        pub port_mode: GraphPortMode,
    }

    impl Default for BoardHost {
        fn default() -> Self {
            Self {
                camera: Camera::default(),
                nodes: BTreeMap::new(),
                handles: BTreeMap::new(),
                edges: BTreeMap::new(),
                wires: BTreeMap::new(),
                handle_kinds: BTreeMap::new(),
                wire_kinds: BTreeMap::new(),
                node_kinds: BTreeMap::new(),
                edge_kinds: BTreeMap::new(),
                edge_tips: builtin_edge_tips(),
                link_compat_rules: Vec::new(),
                selection: BTreeSet::new(),
                preselect: BTreeSet::new(),
                preselect_removed: BTreeSet::new(),
                selection_exit_highlight: BTreeSet::new(),
                selection_options: SelectionOptions { method: "rectangle".into(), mode: "replace".into(), select_nodes: true, select_edges: true, select_handles: true },
                hovered_id: None,
                hovered_kind: None,
                interaction: Interaction::None,
                width: 1,
                height: 1,
                dpr: 1.0,
                world_raster_tiling: "world-clip".into(),
                events: Vec::new(),
                selection_screen_preview: None,
                selection_preview_crossing: false,
                link_screen_preview: None,
                vello_theme: VelloThemePalette::default(),
                grid_factor: GRID_FACTOR_DEFAULT,
                grid_snap_enabled: false,
                preserve_original_element_style: false,
                automatic_lod: true,
                forced_draw_lod: None,
                icon_vector_cache: RefCell::new(HashMap::new()),
                link_compat_nodes_emit_key: None,
                link_target_ring_emit_key: None,
                last_select_emit_sig: None,
                last_preselect_emit_sig: None,
                content_scene_generation: 0,
                world_content_cache: RefCell::new(None),
                wheel_zoom_active: false,
                wheel_zoom_render_lod: None,
                active_tool: ActiveTool::Select,
                brush_flush_distance: DEFAULT_BRUSH_FLUSH_DISTANCE,
                brush_node_size: DEFAULT_BRUSH_NODE_SIZE,
                brush_slot_source_id: None,
                brush_candidate_kinds: Vec::new(),
                brush_candidate_index: 0,
                brush_preview: None,
                fixture_drop_preview: None,
                brush_candidates_emit_key: None,
                brush_preview_emit_key: None,
                brush_placement_serial: 0,
                brush_node_kind_weights: HashMap::new(),
                brush_handle_kind_weights: HashMap::new(),
                port_mode: GraphPortMode::Ported,
            }
        }
    }

    impl BoardHost {
        /// @emoji 📶 Draw LOD used while building the vector scene (pins during wheel zoom).
        fn draw_lod_for_frame(&self) -> BoardDrawLod {
            if self.wheel_zoom_active {
                if let Some(pinned) = self.wheel_zoom_render_lod {
                    return pinned;
                }
            }
            self.current_draw_lod()
        }

        fn board_draw_lod_label(lod: BoardDrawLod) -> &'static str {
            lod.label()
        }

        /// @emoji 🏷️ Camera, draw LOD, and visible node centers from the WASM host for the JS text overlay (must match the last GPU frame).
        pub fn overlay_paint_state_json(&self) -> String {
            let nodes: Vec<serde_json::Value> = self
                .nodes
                .values()
                .filter(|n| n.visible)
                .map(|n| serde_json::json!({ "id": n.id, "x": n.x, "y": n.y }))
                .collect();
            serde_json::json!({
                "camera": { "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom },
                "lod": Self::board_draw_lod_label(self.draw_lod_for_frame()),
                "nodes": nodes,
            })
            .to_string()
        }

        fn bump_content_scene_generation(&mut self) {
            self.content_scene_generation = self.content_scene_generation.wrapping_add(1);
            *self.world_content_cache.borrow_mut() = None;
        }

        #[cfg(test)]
        pub fn test_content_scene_generation(&self) -> u64 {
            self.content_scene_generation
        }

        fn viewport(&self) -> crate::cavas::camera::Viewport {
            crate::cavas::camera::Viewport { width: self.width, height: self.height, dpr: self.dpr }
        }

        fn camera_content_affine(&self) -> Affine {
            crate::cavas::camera::camera_content_affine(&self.camera, &self.viewport())
        }
    }

    impl BoardHost {
        fn color_from_json_rgba8(arr: &[serde_json::Value]) -> Option<Color> {
            let r = u8::try_from(arr.get(0)?.as_u64().unwrap_or(0).min(255)).ok()?;
            let g = u8::try_from(arr.get(1)?.as_u64().unwrap_or(0).min(255)).ok()?;
            let b = u8::try_from(arr.get(2)?.as_u64().unwrap_or(0).min(255)).ok()?;
            let a = u8::try_from(arr.get(3).and_then(|x| x.as_u64()).unwrap_or(255).min(255)).ok()?;
            Some(Color::from_rgba8(r, g, b, a))
        }

        fn grid_step_large_world(&self) -> f64 {
            GRID_WORLD_LARGE * self.grid_factor
        }
        fn grid_step_medium_world(&self) -> f64 {
            GRID_WORLD_MEDIUM * self.grid_factor
        }
        fn grid_step_small_world(&self) -> f64 {
            GRID_WORLD_SMALL * self.grid_factor
        }
        fn grid_step_micro_world(&self) -> f64 {
            GRID_WORLD_MICRO * self.grid_factor
        }

        pub fn new() -> Self {
            Self::default()
        }

        /// 🧠 Normal directed graph host: no handles, edges reference node ids.
        pub fn new_normal() -> Self {
            let mut host = Self::default();
            host.port_mode = GraphPortMode::Normal;
            host.selection_options.select_handles = false;
            host
        }

        fn has_ports(&self) -> bool {
            self.port_mode.has_ports()
        }

        fn node_rim_point_toward(&self, node: &NodeData, toward: Point) -> Option<Point> {
            let center = Point::new(node.x, node.y);
            match node.shape {
                NodeShape::Circle => {
                    let radius = self.scaled_node_radius(node);
                    let angle = circle_handle_angle_toward(center, toward);
                    Some(handle_position_on_circle(center, radius, angle))
                }
                NodeShape::Rectangle => {
                    let width = self.scaled_node_width(node);
                    let height = self.scaled_node_height(node);
                    let angle = rectangle_handle_angle_toward(center, width, height, toward);
                    Some(handle_position_on_rectangle(center, width, height, angle))
                }
            }
        }

        fn current_draw_lod(&self) -> BoardDrawLod {
            if !self.automatic_lod {
                if let Some(lod) = self.forced_draw_lod {
                    return lod;
                }
            }
            BoardDrawLod::from_scale_index(PUZZLE_2D_LOD_SCALE.resolve_index(self.camera.zoom))
        }

        fn lod_visible_grid_snap_step_world(&self) -> Option<f64> {
            match self.current_draw_lod() {
                BoardDrawLod::Minimap => None,
                BoardDrawLod::Overview | BoardDrawLod::Compact => Some(self.grid_step_large_world()),
                BoardDrawLod::Normal => Some(self.grid_step_medium_world()),
                BoardDrawLod::Detail => Some(self.grid_step_small_world()),
                BoardDrawLod::Micro => Some(self.grid_step_micro_world()),
            }
        }

        fn snap_world_scalar(&self, v: f64) -> f64 {
            if !self.grid_snap_enabled {
                return v;
            }
            let Some(step) = self.lod_visible_grid_snap_step_world() else {
                return v;
            };
            (v / step).round() * step
        }

        fn snap_world_pair(&self, x: f64, y: f64) -> (f64, f64) {
            (self.snap_world_scalar(x), self.snap_world_scalar(y))
        }

        pub fn set_grid_snap_enabled(&mut self, enabled: bool) {
            self.grid_snap_enabled = enabled;
        }

        pub fn set_automatic_lod(&mut self, enabled: bool) {
            self.automatic_lod = enabled;
            if enabled {
                self.forced_draw_lod = None;
            }
        }

        pub fn set_forced_draw_lod_label(&mut self, label: &str) {
            let t = label.trim();
            if t.is_empty() {
                self.forced_draw_lod = None;
                return;
            }
            self.forced_draw_lod = match BoardDrawLod::from_id(t) {
                Some(lod) => Some(lod),
                None => {
                    self.forced_draw_lod = None;
                    return;
                }
            };
        }

        pub fn set_grid_factor(&mut self, v: f64) -> Result<(), String> {
            if !v.is_finite() || v <= 0.0 || v > 1_000_000.0 {
                return Err("gridFactor must be finite and in (0, 1e6]".into());
            }
            self.grid_factor = v;
            Ok(())
        }

        /// @emoji 🔗 Applies or clears a host-driven link preview session (cross-surface mirror).
        pub fn set_external_link_preview_json(&mut self, json: &str) -> Result<(), String> {
            let v: serde_json::Value = serde_json::from_str(json).map_err(|e| format!("setLinkSessionJson: {e}"))?;
            let source = v.get("source").and_then(|s| s.as_str()).unwrap_or("").trim().to_string();
            if source.is_empty() {
                if matches!(self.interaction, Interaction::ExternalLinkPreview { .. }) {
                    self.interaction = Interaction::None;
                    self.clear_link_gesture_events();
                }
                return Ok(());
            }
            let end_x = v.get("endX").and_then(|x| x.as_f64()).unwrap_or(0.0);
            let end_y = v.get("endY").and_then(|y| y.as_f64()).unwrap_or(0.0);
            let compatible_node_ids: Vec<String> = v.get("compatiblePartIds").and_then(|a| a.as_array()).map(|arr| arr.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
            let ring_node_id = v.get("ringPartId").and_then(|n| n.as_str()).map(str::to_string);
            let ring_handle_ids: Vec<String> = v.get("ringAnchorIds").and_then(|a| a.as_array()).map(|arr| arr.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default();
            self.interaction = Interaction::ExternalLinkPreview { source_id: source.clone(), end_world: Point::new(end_x, end_y), compatible_node_ids, ring_node_id, ring_handle_ids };
            self.sync_link_gesture_events();
            Ok(())
        }

        /// @emoji 🔗 Clears host-driven link preview without touching local link drags.
        pub fn clear_external_link_preview(&mut self) {
            if matches!(self.interaction, Interaction::ExternalLinkPreview { .. }) {
                self.interaction = Interaction::None;
                self.clear_link_gesture_events();
            }
        }

        fn get_or_build_icon_paint(&self, encoded: &str, fg: Color, bg: Color, preserve_original_style: bool) -> Option<(f64, f64, f64, f64, CachedIconBody)> {
            let resolved = crate::board_icon_codec::board_resolve_icon_kind(encoded);
            let key = match &resolved {
                crate::board_icon_codec::BoardResolvedIcon::None => return None,
                crate::board_icon_codec::BoardResolvedIcon::SvgThemed(s) | crate::board_icon_codec::BoardResolvedIcon::SvgPlain(s) => Self::icon_vector_cache_key(if preserve_original_style { "p" } else { "t" }, s.as_str(), fg, bg),
                crate::board_icon_codec::BoardResolvedIcon::RasterRgba8 { rgba, w, h } => Self::icon_raster_cache_key(rgba, *w, *h),
            };
            {
                let g = self.icon_vector_cache.borrow();
                if let Some(c) = g.get(&key) {
                    return Some((c.bx, c.by, c.bw, c.bh, c.body.clone()));
                }
            }
            let (bx, by, bw, bh, body) = match resolved {
                crate::board_icon_codec::BoardResolvedIcon::None => return None,
                crate::board_icon_codec::BoardResolvedIcon::SvgThemed(s) => {
                    let tree = usvg::Tree::from_str(s.trim(), crate::svg_icon_vello09::usvg_options_board_icons()).ok()?;
                    let (bx, by, bw, bh) = crate::svg_icon_vello09::svg_icon_content_bounds(&tree);
                    if !(bw > 0.0 && bh > 0.0 && bw.is_finite() && bh.is_finite()) {
                        return None;
                    }
                    let mut s = Scene::new();
                    if preserve_original_style {
                        let _ = vello_svg::append_tree(&mut s, &tree);
                    } else {
                        crate::svg_icon_vello09::render_svg_tree_themed(&mut s, &tree, fg, bg);
                    }
                    (bx, by, bw, bh, CachedIconBody::Vector(s))
                }
                crate::board_icon_codec::BoardResolvedIcon::SvgPlain(s) => {
                    let svg_t = s.trim();
                    let tree = usvg::Tree::from_str(svg_t, crate::svg_icon_vello09::usvg_options_board_icons()).ok()?;
                    let (bx, by, bw, bh) = crate::svg_icon_vello09::svg_icon_content_bounds(&tree);
                    if !(bw > 0.0 && bh > 0.0 && bw.is_finite() && bh.is_finite()) {
                        return None;
                    }
                    let mut s = Scene::new();
                    if preserve_original_style {
                        let _ = vello_svg::append_tree(&mut s, &tree);
                    } else {
                        crate::svg_icon_vello09::render_svg_tree_themed(&mut s, &tree, fg, bg);
                    }
                    (bx, by, bw, bh, CachedIconBody::Vector(s))
                }
                crate::board_icon_codec::BoardResolvedIcon::RasterRgba8 { rgba, w, h } => {
                    let bx = 0.0_f64;
                    let by = 0.0_f64;
                    let bw = f64::from(w);
                    let bh = f64::from(h);
                    let img = ImageData { data: Blob::new(Arc::new(rgba.as_ref().to_vec())), format: ImageFormat::Rgba8, alpha_type: ImageAlphaType::Alpha, width: w, height: h };
                    (bx, by, bw, bh, CachedIconBody::Raster(Arc::new(img)))
                }
            };
            let cached = CachedIconPaint { bx, by, bw, bh, body: body.clone() };
            self.icon_vector_cache.borrow_mut().insert(key, cached);
            Some((bx, by, bw, bh, body))
        }

        pub fn clear_icon_vector_cache(&mut self) {
            self.icon_vector_cache.borrow_mut().clear();
        }

        fn icon_vector_cache_key(tag: &str, svg: &str, fg: Color, bg: Color) -> String {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            svg.hash(&mut hasher);
            let hx = hasher.finish();
            let f = fg.to_rgba8();
            let b = bg.to_rgba8();
            format!("v8|{tag}|{hx:x}|{}|{:02x}{:02x}{:02x}{:02x}|{:02x}{:02x}{:02x}{:02x}", svg.len(), f.r, f.g, f.b, f.a, b.r, b.g, b.b, b.a)
        }

        fn icon_raster_cache_key(rgba: &Arc<[u8]>, w: u32, h: u32) -> String {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            rgba.as_ref().hash(&mut hasher);
            let hx = hasher.finish();
            format!("v8|r|{w}x{h}|{hx:x}|{}", rgba.len())
        }

        pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
            self.width = width.max(1);
            self.height = height.max(1);
            self.dpr = dpr.max(1.0);
        }

        pub fn set_camera(&mut self, x: f64, y: f64, zoom: f64) {
            self.set_camera_internal(x, y, zoom, true);
        }

        /// @emoji 🔇 Updates viewport camera without enqueueing a `camera` drain row (wheel / imperative sync).
        pub fn set_camera_silent(&mut self, x: f64, y: f64, zoom: f64) {
            self.set_camera_internal(x, y, zoom, false);
        }

        fn set_camera_internal(&mut self, x: f64, y: f64, zoom: f64, emit_event: bool) {
            let zoom = crate::cavas::camera::clamp_zoom(zoom);
            if (self.camera.x - x).abs() < 1e-9 && (self.camera.y - y).abs() < 1e-9 && (self.camera.zoom - zoom).abs() < 1e-9 {
                return;
            }
            self.camera.x = x;
            self.camera.y = y;
            self.camera.zoom = zoom;
            if emit_event {
                self.push_event("camera", json!({ "x": self.camera.x, "y": self.camera.y, "zoom": self.camera.zoom }));
            }
        }

        pub fn set_selection_options(&mut self, method: &str, mode: &str, select_nodes: bool, select_edges: bool, select_handles: bool) {
            self.selection_options.method = method.into();
            self.selection_options.mode = if mode == "default" { "replace" } else { mode }.into();
            self.selection_options.select_nodes = select_nodes;
            self.selection_options.select_edges = select_edges;
            self.selection_options.select_handles = select_handles;
        }

        /// @emoji 🔗 JSON `[{ "source","target","bidirectional"?,"important"?,"specificity"? },…]` gates link gestures; empty clears restrictions.
        pub fn set_handle_link_compat_from_json(&mut self, json: &str) -> Result<(), String> {
            let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
            let arr = v.as_array().ok_or_else(|| "expected JSON array of compatibility objects".to_string())?;
            let mut next = Vec::new();
            for row in arr {
                let o = row.as_object().ok_or("compat row must be object")?;
                let source = o.get("source").and_then(|x| x.as_str()).ok_or_else(|| "compat row missing string source".to_string())?.trim().to_string();
                let target = o.get("target").and_then(|x| x.as_str()).ok_or_else(|| "compat row missing string target".to_string())?.trim().to_string();
                let bidirectional = o.get("bidirectional").and_then(|x| x.as_bool()).unwrap_or(false);
                let important = o.get("important").and_then(|x| x.as_bool()).unwrap_or(false);
                let spec_s = o.get("specificity").and_then(|x| x.as_str()).unwrap_or("handle");
                let specificity = Self::parse_compat_specificity(spec_s)?;
                next.push(LinkCompatRule { source, target, bidirectional, important, specificity });
            }
            self.link_compat_rules = next;
            Ok(())
        }

        fn parse_compat_specificity(raw: &str) -> Result<CompatSpecificity, String> {
            match raw.trim().to_ascii_lowercase().as_str() {
                "general" => Ok(CompatSpecificity::General),
                "node" => Ok(CompatSpecificity::Node),
                "edge" => Ok(CompatSpecificity::Edge),
                "handle" => Ok(CompatSpecificity::Handle),
                "wire" => Ok(CompatSpecificity::Wire),
                _ => Err(format!("compat specificity must be general|node|edge|handle|wire, got {raw:?}")),
            }
        }

        fn reject_kind_catalog_row_legacy_label(row: &serde_json::Map<String, serde_json::Value>, slice: &str) -> Result<(), String> {
            if row.contains_key("label") {
                return Err(format!("{slice} kind row must use name, not legacy label"));
            }
            Ok(())
        }

        /// @emoji 🧩 JSON object `{ handleKinds?, wireKinds?, nodeKinds?, edgeKinds? }` replacing prior catalogs (omit arrays to clear that slice).
        pub fn set_board_kind_catalogs_from_json(&mut self, json: &str) -> Result<(), String> {
            let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
            let o = v.as_object().ok_or("kind catalogs root must be object")?;
            if let Some(arr) = o.get("handleKinds").and_then(|x| x.as_array()) {
                let mut next = BTreeMap::new();
                for row in arr {
                    let ho = row.as_object().ok_or("handle kind row must be object")?;
                    Self::reject_kind_catalog_row_legacy_label(ho, "handle")?;
                    let id = ho.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or("handle kind id missing")?;
                    let name = ho.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                    let color_s = ho.get("color").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or("handle kind color missing")?;
                    let color = Self::parse_css_color(color_s).ok_or_else(|| format!("invalid handle kind color {color_s:?}"))?;
                    let default_wire_kind = ho.get("defaultWireKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                    let scale = ho.get("scale").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0).unwrap_or(1.0);
                    next.insert(id.to_string(), HandleKindDef { name, color, default_wire_kind, scale });
                }
                self.handle_kinds = next;
            }
            if let Some(arr) = o.get("wireKinds").and_then(|x| x.as_array()) {
                let mut next = BTreeMap::new();
                for row in arr {
                    let wo = row.as_object().ok_or("wire kind row must be object")?;
                    Self::reject_kind_catalog_row_legacy_label(wo, "wire")?;
                    let id = wo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or("wire kind id missing")?;
                    let name = wo.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                    let default_edge_kind = wo.get("defaultEdgeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                    next.insert(id.to_string(), WireKindDef { name, default_edge_kind });
                }
                self.wire_kinds = next;
            }
            if let Some(arr) = o.get("nodeKinds").and_then(|x| x.as_array()) {
                let mut next = BTreeMap::new();
                for row in arr {
                    let no = row.as_object().ok_or("node kind row must be object")?;
                    Self::reject_kind_catalog_row_legacy_label(no, "node")?;
                    let id = no.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or("node kind id missing")?;
                    let name = no.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                    let scale = no.get("scale").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0).unwrap_or(1.0);
                    let shape = match no.get("shape").and_then(|x| x.as_str()).map(str::trim) {
                        Some("rectangle") => NodeShape::Rectangle,
                        _ => NodeShape::Circle,
                    };
                    let mut handles: Vec<NodeKindHandleTemplate> = Vec::new();
                    if let Some(arr) = no.get("handles").and_then(|x| x.as_array()) {
                        for row in arr {
                            let ho = row.as_object().ok_or("node kind handle row must be object")?;
                            let handle_kind = ho.get("handleKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or("node kind handle handleKind missing")?;
                            let angle = ho.get("angle").and_then(|x| x.as_f64()).filter(|x| x.is_finite()).ok_or("node kind handle angle missing")?;
                            let radius = ho.get("radius").and_then(|x| x.as_f64()).filter(|x| x.is_finite() && *x > 0.0);
                            handles.push(NodeKindHandleTemplate { handle_kind: handle_kind.to_string(), angle, radius });
                        }
                    }
                    let icon = no
                        .get("icon")
                        .and_then(|x| x.as_str())
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string());
                    let color_fill = no
                        .get("color")
                        .and_then(|x| x.as_str())
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .and_then(Self::parse_css_hex_color);
                    next.insert(id.to_string(), NodeKindDef { name, scale, shape, handles, icon, color_fill });
                }
                self.node_kinds = next;
            }
            if let Some(arr) = o.get("edgeTips").and_then(|x| x.as_array()) {
                let mut tips = builtin_edge_tips();
                for row in arr {
                    let eo = row.as_object().ok_or("edge tip row must be object")?;
                    let id = eo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or("edge tip id missing")?;
                    let def = EdgeTipDef::from_catalog_row(eo).ok_or_else(|| format!("edge tip row {:?} invalid", id))?;
                    tips.insert(id.to_string(), def);
                }
                self.edge_tips = tips;
            }
            if let Some(arr) = o.get("edgeKinds").and_then(|x| x.as_array()) {
                let mut next = BTreeMap::new();
                for row in arr {
                    let eo = row.as_object().ok_or("edge kind row must be object")?;
                    Self::reject_kind_catalog_row_legacy_label(eo, "edge")?;
                    let id = eo.get("id").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).ok_or("edge kind id missing")?;
                    let name = eo.get("name").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).unwrap_or("").to_string();
                    let color = eo
                        .get("color")
                        .and_then(|x| x.as_str())
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .and_then(Self::parse_css_hex_color);
                    let stroke_width = eo
                        .get("stroke")
                        .and_then(|x| x.as_f64())
                        .filter(|v| v.is_finite() && *v > 0.0)
                        .or_else(|| eo.get("stroke").and_then(|x| x.as_str()).and_then(|s| s.trim().parse::<f64>().ok()).filter(|v| v.is_finite() && *v > 0.0))
                        .unwrap_or(2.0);
                    let pattern = match eo.get("pattern").and_then(|x| x.as_str()).map(str::trim) {
                        Some("dashed") => EdgeStrokePattern::Dashed,
                        Some("dotted") => EdgeStrokePattern::Dotted,
                        _ => EdgeStrokePattern::Solid,
                    };
                    let source_tip = Self::parse_catalog_tip_slot(eo.get("sourceTip").or_else(|| eo.get("source_tip")).and_then(|x| x.as_str()));
                    let target_tip = Self::parse_catalog_tip_slot(
                        eo.get("targetTip")
                            .or_else(|| eo.get("target_tip"))
                            .and_then(|x| x.as_str())
                            .or_else(|| eo.get("marker").and_then(|x| x.as_str())),
                    );
                    let directed = eo.get("directed").and_then(|x| x.as_bool()).unwrap_or(true);
                    next.insert(id.to_string(), EdgeKindDef { name, color, stroke_width, pattern, source_tip, target_tip, directed });
                }
                self.edge_kinds = next;
            }
            Ok(())
        }

        fn parse_catalog_tip_slot(value: Option<&str>) -> Option<String> {
            let s = value?.trim();
            if s.is_empty() || s.eq_ignore_ascii_case("none") {
                Some(String::new())
            } else {
                Some(s.to_string())
            }
        }

        fn lookup_edge_tip<'a>(&'a self, id: &str) -> Option<&'a EdgeTipDef> {
            if id.is_empty() {
                return None;
            }
            self.edge_tips.get(id)
        }

        fn resolve_tip_slot<'a>(&'a self, slot: Option<&str>) -> Option<&'a EdgeTipDef> {
            match slot {
                Some("") => None,
                Some(id) => self.lookup_edge_tip(id),
                None => None,
            }
        }

        fn parse_css_hex_color(s: &str) -> Option<Color> {
            let s = s.trim();
            let hex = s.strip_prefix('#')?;
            match hex.len() {
                3 => {
                    let mut full = String::new();
                    for ch in hex.chars() {
                        full.push(ch);
                        full.push(ch);
                    }
                    let v = u32::from_str_radix(&full, 16).ok()?;
                    let r = ((v >> 16) & 0xff) as u8;
                    let g = ((v >> 8) & 0xff) as u8;
                    let b = (v & 0xff) as u8;
                    Some(Color::from_rgba8(r, g, b, 255))
                }
                6 => {
                    let v = u32::from_str_radix(hex, 16).ok()?;
                    let r = ((v >> 16) & 0xff) as u8;
                    let g = ((v >> 8) & 0xff) as u8;
                    let b = (v & 0xff) as u8;
                    Some(Color::from_rgba8(r, g, b, 255))
                }
                8 => {
                    let v = u32::from_str_radix(hex, 16).ok()?;
                    let r = ((v >> 24) & 0xff) as u8;
                    let g = ((v >> 16) & 0xff) as u8;
                    let b = ((v >> 8) & 0xff) as u8;
                    let a = (v & 0xff) as u8;
                    Some(Color::from_rgba8(r, g, b, a))
                }
                _ => None,
            }
        }

        /// @emoji 🎨 Accepts `#rgb`/`#rrggbb`/`#rrggbbaa` or CSS `hsl()` / `hsla()` (comma or space syntax, optional `/` alpha).
        fn parse_css_color(s: &str) -> Option<Color> {
            if let Some(c) = Self::parse_css_hex_color(s) {
                return Some(c);
            }
            Self::parse_css_hsl_color(s)
        }

        fn parse_css_hsl_color(s: &str) -> Option<Color> {
            let low = s.trim().to_ascii_lowercase();
            let (legacy_alpha_form, inner) = if let Some(inner) = low.strip_prefix("hsla(").and_then(|x| x.strip_suffix(')')) {
                (true, inner)
            } else if let Some(inner) = low.strip_prefix("hsl(").and_then(|x| x.strip_suffix(')')) {
                (false, inner)
            } else {
                return None;
            };
            let inner = inner.trim();
            let (main, alpha_slash) = inner.split_once('/').map(|(a, b)| (a.trim(), Some(b.trim()))).unwrap_or((inner, None));
            let normalized = main.replace(',', " ");
            let parts: Vec<&str> = normalized.split_whitespace().collect();
            if parts.len() < 3 {
                return None;
            }
            let h = Self::parse_css_hsl_hue(parts[0])?;
            let sat = Self::parse_css_hsl_sl(parts[1])?;
            let light = Self::parse_css_hsl_sl(parts[2])?;
            let alpha = if let Some(a) = alpha_slash {
                Self::parse_css_alpha_channel(a)?
            } else if legacy_alpha_form && parts.len() >= 4 {
                Self::parse_css_alpha_channel(parts[3])?
            } else {
                1.0
            };
            let (r, g, b) = Self::hsl_to_rgb_bytes(h, sat, light);
            let a = (alpha * 255.0).round().clamp(0.0, 255.0) as u8;
            Some(Color::from_rgba8(r, g, b, a))
        }

        fn parse_css_hsl_hue(tok: &str) -> Option<f64> {
            let t = tok.trim();
            let n = t.strip_suffix("deg").map(str::trim).unwrap_or(t);
            let v: f64 = n.parse().ok()?;
            v.is_finite().then_some(v)
        }

        fn parse_css_hsl_sl(tok: &str) -> Option<f64> {
            let t = tok.trim();
            if let Some(p) = t.strip_suffix('%') {
                let v: f64 = p.trim().parse().ok()?;
                Some((v / 100.0).clamp(0.0, 1.0))
            } else {
                let v: f64 = t.parse().ok()?;
                Some(v.clamp(0.0, 1.0))
            }
        }

        fn parse_css_alpha_channel(tok: &str) -> Option<f64> {
            Self::parse_css_hsl_sl(tok)
        }

        fn board_hsl_hue_to_rgb_component(p: f64, q: f64, mut t: f64) -> f64 {
            if t < 0.0 {
                t += 1.0;
            }
            if t > 1.0 {
                t -= 1.0;
            }
            if t < 1.0 / 6.0 {
                p + (q - p) * 6.0 * t
            } else if t < 0.5 {
                q
            } else if t < 2.0 / 3.0 {
                p + (q - p) * (2.0 / 3.0 - t) * 6.0
            } else {
                p
            }
        }

        fn hsl_to_rgb_bytes(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
            let h_norm = ((h % 360.0 + 360.0) % 360.0) / 360.0;
            let s = s.clamp(0.0, 1.0);
            let l = l.clamp(0.0, 1.0);
            if s <= f64::EPSILON {
                let v = (l * 255.0).round().clamp(0.0, 255.0) as u8;
                return (v, v, v);
            }
            let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
            let p = 2.0 * l - q;
            let r = Self::board_hsl_hue_to_rgb_component(p, q, h_norm + 1.0 / 3.0);
            let g = Self::board_hsl_hue_to_rgb_component(p, q, h_norm);
            let b = Self::board_hsl_hue_to_rgb_component(p, q, h_norm - 1.0 / 3.0);
            ((r * 255.0).round().clamp(0.0, 255.0) as u8, (g * 255.0).round().clamp(0.0, 255.0) as u8, (b * 255.0).round().clamp(0.0, 255.0) as u8)
        }

        fn explicit_style_kind(style: Option<&str>) -> Option<BoardElementStyleKind> {
            match style.map(str::trim).filter(|s| !s.is_empty()) {
                Some("original") => Some(BoardElementStyleKind::Original),
                Some("neutral") => Some(BoardElementStyleKind::Neutral),
                Some("hovered") => Some(BoardElementStyleKind::Hovered),
                Some("selected") => Some(BoardElementStyleKind::Selected),
                Some("highlighted") => Some(BoardElementStyleKind::Highlighted),
                Some("disabled") => Some(BoardElementStyleKind::Disabled),
                _ => None,
            }
        }

        fn hovered_style_kind(&self, id: &str, domain: &str, element_kind: &str) -> Option<BoardElementStyleKind> {
            if self.is_preselect_active() {
                return None;
            }
            if self.selection.contains(id) {
                return None;
            }
            if self.hovered_id.as_deref() == Some(id) {
                return Some(BoardElementStyleKind::Hovered);
            }
            if let Some((hover_domain, hover_kind)) = self.hovered_kind.as_ref() {
                if hover_domain == domain && hover_kind == element_kind {
                    return Some(BoardElementStyleKind::Hovered);
                }
            }
            None
        }

        fn resolve_element_kind_hover(&self, id: &str) -> Option<(String, String)> {
            if let Some(node) = self.nodes.get(id) {
                return Some(("node".to_string(), node.node_kind.clone()));
            }
            if let Some(handle) = self.handles.get(id) {
                return Some(("handle".to_string(), handle.handle_kind.clone()));
            }
            if let Some(edge) = self.edges.get(id) {
                return Some(("edge".to_string(), edge.edge_kind.clone()));
            }
            if let Some(wire) = self.wires.get(id) {
                return Some(("wire".to_string(), wire.wire_kind.clone()));
            }
            None
        }

        fn ids_matching_kind_hover(&self) -> Vec<String> {
            let Some((domain, kind_id)) = self.hovered_kind.as_ref() else {
                return Vec::new();
            };
            let mut ids = Vec::new();
            match domain.as_str() {
                "node" => {
                    for node in self.nodes.values() {
                        if &node.node_kind == kind_id && !self.selection.contains(&node.id) {
                            ids.push(node.id.clone());
                        }
                    }
                }
                "handle" => {
                    for handle in self.handles.values() {
                        if &handle.handle_kind == kind_id && !self.selection.contains(&handle.id) {
                            ids.push(handle.id.clone());
                        }
                    }
                }
                "edge" => {
                    for edge in self.edges.values() {
                        if &edge.edge_kind == kind_id && !self.selection.contains(&edge.id) {
                            ids.push(edge.id.clone());
                        }
                    }
                }
                "wire" => {
                    for wire in self.wires.values() {
                        if &wire.wire_kind == kind_id && !self.selection.contains(&wire.id) {
                            ids.push(wire.id.clone());
                        }
                    }
                }
                _ => {}
            }
            ids
        }

        fn is_preselect_active(&self) -> bool {
            self.is_preselecting() || !self.preselect.is_empty()
        }

        /// @emoji 🎨 During area-select: preselect → Selected; anchor∖preselect → Highlighted; idle selection → Selected.
        fn resolve_interaction_style_kind(&self, id: &str) -> BoardElementStyleKind {
            if self.is_preselect_active() {
                if self.preselect.contains(id) {
                    return BoardElementStyleKind::Selected;
                }
                if self.selection.contains(id) {
                    return BoardElementStyleKind::Highlighted;
                }
                return BoardElementStyleKind::Neutral;
            }
            if self.selection.contains(id) {
                return BoardElementStyleKind::Selected;
            }
            BoardElementStyleKind::Neutral
        }

        fn resolve_node_style_kind(&self, n: &NodeData, pass: StyleChromePass) -> BoardElementStyleKind {
            if let Some(kind) = Self::explicit_style_kind(n.style.as_deref()) {
                return kind;
            }
            match pass {
                StyleChromePass::CachedBase => {
                    if self.preserve_original_element_style {
                        BoardElementStyleKind::Original
                    } else {
                        BoardElementStyleKind::Neutral
                    }
                }
                StyleChromePass::InteractionOverlay => {
                    if let Some(kind) = self.hovered_style_kind(n.id.as_str(), "node", n.node_kind.as_str()) {
                        return kind;
                    }
                    self.resolve_interaction_style_kind(n.id.as_str())
                }
            }
        }

        fn resolve_handle_style_kind(&self, h: &HandleData, pass: StyleChromePass) -> BoardElementStyleKind {
            if let Some(kind) = Self::explicit_style_kind(h.style.as_deref()) {
                return kind;
            }
            match pass {
                StyleChromePass::CachedBase => {
                    if self.preserve_original_element_style {
                        BoardElementStyleKind::Original
                    } else {
                        BoardElementStyleKind::Neutral
                    }
                }
                StyleChromePass::InteractionOverlay => {
                    if let Some(kind) = self.hovered_style_kind(h.id.as_str(), "handle", h.handle_kind.as_str()) {
                        return kind;
                    }
                    self.resolve_interaction_style_kind(h.id.as_str())
                }
            }
        }

        fn resolve_edge_style_kind(&self, e: &EdgeData, pass: StyleChromePass) -> BoardElementStyleKind {
            if let Some(kind) = Self::explicit_style_kind(e.style.as_deref()) {
                return kind;
            }
            match pass {
                StyleChromePass::CachedBase => BoardElementStyleKind::Neutral,
                StyleChromePass::InteractionOverlay => {
                    if let Some(kind) = self.hovered_style_kind(e.id.as_str(), "edge", e.edge_kind.as_str()) {
                        return kind;
                    }
                    self.resolve_interaction_style_kind(e.id.as_str())
                }
            }
        }

        fn resolve_wire_style_kind(&self, w: &WireData, pass: StyleChromePass) -> BoardElementStyleKind {
            if let Some(kind) = Self::explicit_style_kind(w.style.as_deref()) {
                return kind;
            }
            match pass {
                StyleChromePass::CachedBase => BoardElementStyleKind::Neutral,
                StyleChromePass::InteractionOverlay => {
                    if let Some(kind) = self.hovered_style_kind(w.id.as_str(), "wire", w.wire_kind.as_str()) {
                        return kind;
                    }
                    self.resolve_interaction_style_kind(w.id.as_str())
                }
            }
        }

        /// @emoji 💠 Entity ids that need selection/preselect/hover chrome painted above {@link BoardHost.world_content_cache}.
        fn interaction_overlay_entity_ids(&self) -> BTreeSet<String> {
            let mut ids = BTreeSet::new();
            if self.is_preselect_active() {
                ids.extend(self.preselect.iter().cloned());
                ids.extend(self.selection.iter().cloned());
                ids.extend(self.preselect_removed.iter().cloned());
            } else {
                ids.extend(self.selection.iter().cloned());
                ids.extend(self.selection_exit_highlight.iter().cloned());
            }
            if let Some(ref hover_id) = self.hovered_id {
                if !self.is_preselect_active() && !self.selection.contains(hover_id) {
                    ids.insert(hover_id.clone());
                }
            }
            if !self.is_preselect_active() {
                for id in self.ids_matching_kind_hover() {
                    ids.insert(id);
                }
            }
            ids
        }

        fn node_fill_for_style(theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
            match kind {
                BoardElementStyleKind::Hovered => theme.node_fill_hovered,
                BoardElementStyleKind::Selected => theme.node_fill_selected,
                BoardElementStyleKind::Highlighted => theme.node_fill_selection_exit,
                BoardElementStyleKind::Disabled => theme.node_fill_disabled,
                BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.node_fill,
            }
        }

        fn node_stroke_for_style(theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
            match kind {
                BoardElementStyleKind::Hovered => theme.node_stroke_hovered,
                BoardElementStyleKind::Selected => theme.node_stroke_selected,
                BoardElementStyleKind::Highlighted => theme.node_stroke_selection_exit,
                BoardElementStyleKind::Disabled => theme.node_stroke_disabled,
                BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.node_stroke,
            }
        }

        fn resolve_handle_fill_color(&self, h: &HandleData, theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
            match kind {
                BoardElementStyleKind::Hovered => theme.handle_fill_hovered,
                BoardElementStyleKind::Selected => theme.handle_fill_selected,
                BoardElementStyleKind::Highlighted => theme.handle_fill_selection_exit,
                BoardElementStyleKind::Disabled => theme.handle_fill_disabled,
                BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => {
                    if let Some(c) = h.color_fill {
                        return c;
                    }
                    if let Some(def) = self.handle_kinds.get(&h.handle_kind) {
                        return def.color;
                    }
                    theme.handle_fill
                }
            }
        }

        fn resolve_handle_stroke_color(&self, _h: &HandleData, theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
            match kind {
                BoardElementStyleKind::Hovered => theme.handle_stroke_hovered,
                BoardElementStyleKind::Selected => theme.handle_stroke_selected,
                BoardElementStyleKind::Highlighted => theme.handle_stroke_selection_exit,
                BoardElementStyleKind::Disabled => theme.handle_stroke_disabled,
                BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.handle_stroke,
            }
        }

        fn edge_stroke_for_style(theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
            match kind {
                BoardElementStyleKind::Hovered => theme.edge_stroke_hovered,
                BoardElementStyleKind::Selected => theme.edge_stroke_selected,
                BoardElementStyleKind::Highlighted => theme.edge_stroke_selection_exit,
                BoardElementStyleKind::Disabled => theme.edge_stroke_disabled,
                BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.edge_stroke,
            }
        }

        fn lerp_color(a: Color, b: Color, t: f64) -> Color {
            let t = t.clamp(0.0, 1.0);
            let ac = a.to_rgba8();
            let bc = b.to_rgba8();
            Color::from_rgba8(
                ((f64::from(ac.r) * (1.0 - t) + f64::from(bc.r) * t).round() as u8),
                ((f64::from(ac.g) * (1.0 - t) + f64::from(bc.g) * t).round() as u8),
                ((f64::from(ac.b) * (1.0 - t) + f64::from(bc.b) * t).round() as u8),
                ((f64::from(ac.a) * (1.0 - t) + f64::from(bc.a) * t).round() as u8),
            )
        }

        fn resolve_node_fill_color(&self, n: &NodeData, theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
            let theme_fill = Self::node_fill_for_style(theme, kind);
            match kind {
                BoardElementStyleKind::Hovered | BoardElementStyleKind::Selected | BoardElementStyleKind::Highlighted | BoardElementStyleKind::Disabled => theme_fill,
                BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => {
                    let kind_id = n.node_kind.trim();
                    if kind_id.is_empty() {
                        return theme_fill;
                    }
                    self.node_kinds.get(kind_id).and_then(|def| def.color_fill).unwrap_or(theme_fill)
                }
            }
        }

        fn edge_stroke_for_kind_pattern(pattern: EdgeStrokePattern, width: f64) -> Stroke {
            use vello::kurbo::Cap;
            let mut stroke = Stroke::new(width);
            match pattern {
                EdgeStrokePattern::Solid => {}
                EdgeStrokePattern::Dashed => {
                    stroke.dash_pattern = vec![width * 3.0, width * 2.0].into();
                }
                EdgeStrokePattern::Dotted => {
                    stroke.dash_pattern = vec![width * 0.35, width * 1.65].into();
                    stroke.start_cap = Cap::Round;
                    stroke.end_cap = Cap::Round;
                }
            }
            stroke
        }

        fn resolve_edge_stroke_paint(&self, e: &EdgeData, chrome_pass: StyleChromePass, lod_scale_width: f64) -> (Color, Stroke, f64) {
            let style_kind = self.resolve_edge_style_kind(e, chrome_pass);
            let chrome = Self::edge_stroke_for_style(&self.vello_theme, style_kind);
            let kind_def = self.edge_kinds.get(e.edge_kind.as_str());
            let base_color = kind_def.and_then(|d| d.color).unwrap_or(self.vello_theme.edge_stroke);
            let stroke_color = match style_kind {
                BoardElementStyleKind::Neutral | BoardElementStyleKind::Original => base_color,
                _ => Self::lerp_color(base_color, chrome, 0.55),
            };
            let catalog_w = kind_def.map(|d| d.stroke_width).unwrap_or(2.0);
            let width_mult = match style_kind {
                BoardElementStyleKind::Selected => 1.35,
                BoardElementStyleKind::Hovered => 1.2,
                _ => 1.0,
            };
            let width = lod_scale_width * (catalog_w / 2.0) * width_mult;
            let pattern = kind_def.map(|d| d.pattern).unwrap_or(EdgeStrokePattern::Solid);
            (stroke_color, Self::edge_stroke_for_kind_pattern(pattern, width), width)
        }

        fn resolve_edge_tips<'a>(&'a self, e: &EdgeData) -> (Option<&'a EdgeTipDef>, Option<&'a EdgeTipDef>) {
            let kind_def = self.edge_kinds.get(e.edge_kind.as_str());
            let source_slot = e.source_tip.as_deref().or_else(|| kind_def.and_then(|d| d.source_tip.as_deref()));
            let target_slot = e.target_tip.as_deref().or_else(|| kind_def.and_then(|d| d.target_tip.as_deref()));
            let mut source = self.resolve_tip_slot(source_slot);
            let mut target = self.resolve_tip_slot(target_slot);
            if target.is_none() && target_slot.is_none() {
                let directed = kind_def.map(|d| d.directed).unwrap_or(true);
                if directed {
                    target = self.lookup_edge_tip("arrow");
                }
            }
            (source, target)
        }

        fn append_edge_tip(scene: &mut Scene, tip: Point, dir: Vec2, color: Color, stroke_width: f64, tip_def: &EdgeTipDef) {
            use crate::vello::kurbo::BezPath;
            let len = dir.hypot();
            if len < 1e-9 {
                return;
            }
            let d = dir / len;
            let n = Vec2::new(-d.y, d.x);
            let sw = stroke_width.max(1.0) * tip_def.scale.max(0.25);
            match tip_def.geometry {
                EdgeTipGeometry::Arrow => {
                    let length = sw * 4.2;
                    let half_w = sw * 1.15;
                    let base = tip - d * length;
                    let mut path = BezPath::new();
                    path.move_to(tip);
                    path.line_to(base + n * half_w);
                    path.line_to(base - n * half_w);
                    path.close_path();
                    if tip_def.filled {
                        scene.fill(Fill::NonZero, Affine::IDENTITY, color, None, &path);
                    } else {
                        scene.stroke(&Stroke::new(sw.max(1.25)), Affine::IDENTITY, color, None, &path);
                    }
                }
                EdgeTipGeometry::FineArrow => {
                    let length = sw * 3.2;
                    let half_w = sw * 0.75;
                    let base = tip - d * length;
                    let mut path = BezPath::new();
                    path.move_to(tip);
                    path.line_to(base + n * half_w);
                    path.move_to(tip);
                    path.line_to(base - n * half_w);
                    let outline = Stroke::new((sw * 0.9).max(1.0));
                    scene.stroke(&outline, Affine::IDENTITY, color, None, &path);
                }
                EdgeTipGeometry::Diamond => {
                    let length = sw * 3.6;
                    let half_w = sw * 1.05;
                    let back = tip - d * length;
                    let mid = tip - d * (length * 0.5);
                    let mut path = BezPath::new();
                    path.move_to(tip);
                    path.line_to(mid + n * half_w);
                    path.line_to(back);
                    path.line_to(mid - n * half_w);
                    path.close_path();
                    if tip_def.filled {
                        scene.fill(Fill::NonZero, Affine::IDENTITY, color, None, &path);
                    } else {
                        scene.stroke(&Stroke::new(sw.max(1.25)), Affine::IDENTITY, color, None, &path);
                    }
                }
                EdgeTipGeometry::Circle => {
                    let r = sw * 1.4;
                    let center = tip - d * r;
                    let circle = Circle::new(center, r);
                    if tip_def.filled {
                        scene.fill(Fill::NonZero, Affine::IDENTITY, color, None, &circle);
                    } else {
                        scene.stroke(&Stroke::new(sw.max(1.25)), Affine::IDENTITY, color, None, &circle);
                    }
                }
                EdgeTipGeometry::Bar => {
                    let half = sw * 1.25;
                    let center = tip - d * (sw * 0.5);
                    let mut path = BezPath::new();
                    path.move_to(center + n * half);
                    path.line_to(center - n * half);
                    scene.stroke(&Stroke::new(sw.max(1.25)), Affine::IDENTITY, color, None, &path);
                }
            }
        }

        fn append_edge_tips_on_curve(scene: &mut Scene, curve: &CubicBez, color: Color, stroke_w: f64, source: Option<&EdgeTipDef>, target: Option<&EdgeTipDef>) {
            let inset = stroke_w * 0.35;
            if let Some(tip_def) = target {
                let mut tangent = curve.p3 - curve.p2;
                if tangent.hypot() < 1e-9 {
                    tangent = curve.p3 - curve.p1;
                }
                if tangent.hypot() >= 1e-9 {
                    let dir = tangent / tangent.hypot();
                    let tip = curve.p3 - dir * inset;
                    Self::append_edge_tip(scene, tip, tangent, color, stroke_w, tip_def);
                }
            }
            if let Some(tip_def) = source {
                let mut tangent = curve.p0 - curve.p1;
                if tangent.hypot() < 1e-9 {
                    tangent = curve.p0 - curve.p2;
                }
                if tangent.hypot() >= 1e-9 {
                    let dir = tangent / tangent.hypot();
                    let tip = curve.p0 - dir * inset;
                    Self::append_edge_tip(scene, tip, tangent, color, stroke_w, tip_def);
                }
            }
        }

        fn wire_stroke_for_style(theme: &VelloThemePalette, kind: BoardElementStyleKind) -> Color {
            match kind {
                BoardElementStyleKind::Hovered => theme.wire_stroke_hovered,
                BoardElementStyleKind::Selected => theme.wire_stroke_selected,
                BoardElementStyleKind::Highlighted => theme.wire_stroke_highlighted,
                BoardElementStyleKind::Disabled => theme.wire_stroke_disabled,
                BoardElementStyleKind::Original | BoardElementStyleKind::Neutral => theme.wire_stroke,
            }
        }

        fn handles_link_compatible_for_drag(&self, source: &HandleData, target: &HandleData) -> bool {
            if !Self::handle_port_shapes_compatible(source.handle_kind.as_str(), target.handle_kind.as_str()) {
                return false;
            }
            if self.link_compat_rules.is_empty() {
                return true;
            }
            let mut matched: Vec<&LinkCompatRule> = self.link_compat_rules.iter().filter(|rule| self.link_gesture_rule_applies(rule, source, target)).collect();
            if matched.is_empty() {
                return false;
            }
            if matched.iter().any(|r| r.important) {
                matched.retain(|r| r.important);
            } else {
                let max_rank = matched.iter().map(|r| r.specificity as i32).max().unwrap_or(0);
                matched.retain(|r| (r.specificity as i32) == max_rank);
            }
            !matched.is_empty()
        }

        fn compat_pair_matches(rule: &LinkCompatRule, a: &str, b: &str) -> bool {
            if rule.source == a && rule.target == b {
                return true;
            }
            if rule.bidirectional && rule.source == b && rule.target == a {
                return true;
            }
            false
        }

        fn handle_port_shape(handle_kind: &str) -> Option<&'static str> {
            if handle_kind.contains(" circular ") {
                Some("circular")
            } else if handle_kind.contains(" rectangular ") {
                Some("rectangular")
            } else {
                None
            }
        }

        fn handle_port_shapes_compatible(source_handle_kind: &str, target_handle_kind: &str) -> bool {
            match (
                Self::handle_port_shape(source_handle_kind),
                Self::handle_port_shape(target_handle_kind),
            ) {
                (Some(a), Some(b)) => a == b,
                _ => true,
            }
        }

        fn resolve_default_wire_kind_for_handle(&self, h: &HandleData) -> String {
            self.handle_kinds.get(&h.handle_kind).and_then(|d| d.default_wire_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string())
        }

        fn resolve_default_edge_kind_for_wire_kind(&self, wire_kind: &str) -> String {
            self.wire_kinds.get(wire_kind).and_then(|d| d.default_edge_kind.as_ref()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default()
        }

        fn link_gesture_rule_applies(&self, rule: &LinkCompatRule, source: &HandleData, target: &HandleData) -> bool {
            let w_src = self.resolve_default_wire_kind_for_handle(source);
            let w_tgt = self.resolve_default_wire_kind_for_handle(target);
            let e_src = self.resolve_default_edge_kind_for_wire_kind(&w_src);
            let e_tgt = self.resolve_default_edge_kind_for_wire_kind(&w_tgt);
            let sn = self.nodes.get(&source.node_id).map(|n| n.node_kind.as_str()).unwrap_or("");
            let tn = self.nodes.get(&target.node_id).map(|n| n.node_kind.as_str()).unwrap_or("");
            let sh = source.handle_kind.as_str();
            let th = target.handle_kind.as_str();
            match rule.specificity {
                CompatSpecificity::General => Self::compat_pair_matches(rule, sh, th),
                CompatSpecificity::Node => Self::compat_pair_matches(rule, sn, tn),
                CompatSpecificity::Edge => Self::compat_pair_matches(rule, e_src.as_str(), e_tgt.as_str()),
                CompatSpecificity::Handle => Self::compat_pair_matches(rule, sh, th),
                CompatSpecificity::Wire => Self::compat_pair_matches(rule, w_src.as_str(), th),
            }
        }

        fn default_edge_kind_for_created_link(&self, source: &HandleData, _target: &HandleData) -> String {
            let wk = self.resolve_default_wire_kind_for_handle(source);
            self.resolve_default_edge_kind_for_wire_kind(&wk)
        }

        fn resolve_default_wire_kind_for_handle_kind(&self, handle_kind: &str) -> String {
            self.handle_kinds
                .get(handle_kind)
                .and_then(|d| d.default_wire_kind.as_ref())
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string())
        }

        fn link_gesture_rule_applies_kind_strings(
            &self,
            rule: &LinkCompatRule,
            sn: &str,
            sh: &str,
            w_src: &str,
            e_src: &str,
            tn: &str,
            th: &str,
            _w_tgt: &str,
            e_tgt: &str,
        ) -> bool {
            match rule.specificity {
                CompatSpecificity::General => Self::compat_pair_matches(rule, sh, th),
                CompatSpecificity::Node => Self::compat_pair_matches(rule, sn, tn),
                CompatSpecificity::Edge => Self::compat_pair_matches(rule, e_src, e_tgt),
                CompatSpecificity::Handle => Self::compat_pair_matches(rule, sh, th),
                CompatSpecificity::Wire => Self::compat_pair_matches(rule, w_src, th),
            }
        }

        fn link_kinds_compatible_for_brush(&self, sn: &str, sh: &str, tn: &str, th: &str) -> bool {
            if !Self::handle_port_shapes_compatible(sh, th) {
                return false;
            }
            if self.link_compat_rules.is_empty() {
                return true;
            }
            let w_src = self.resolve_default_wire_kind_for_handle_kind(sh);
            let w_tgt = self.resolve_default_wire_kind_for_handle_kind(th);
            let e_src = self.resolve_default_edge_kind_for_wire_kind(&w_src);
            let e_tgt = self.resolve_default_edge_kind_for_wire_kind(&w_tgt);
            let mut matched: Vec<&LinkCompatRule> = self
                .link_compat_rules
                .iter()
                .filter(|rule| self.link_gesture_rule_applies_kind_strings(rule, sn, sh, &w_src, &e_src, tn, th, &w_tgt, &e_tgt))
                .collect();
            if matched.is_empty() {
                return false;
            }
            if matched.iter().any(|r| r.important) {
                matched.retain(|r| r.important);
            } else {
                let max_rank = matched.iter().map(|r| r.specificity as i32).max().unwrap_or(0);
                matched.retain(|r| (r.specificity as i32) == max_rank);
            }
            !matched.is_empty()
        }

        fn brush_slot_hit_radius_world(&self) -> f64 {
            (self.brush_node_size * 0.5).max(1.0)
        }

        /// @emoji 🖌️ Brush slot anchor follows indirect-handle layout at overview/normal LOD so hit targets match painted rings.
        fn brush_handle_anchor_world(&self, h: &HandleData) -> Option<Point> {
            if matches!(self.current_draw_lod(), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                self.indirect_handle_world_pos(h).or_else(|| self.handle_world_pos(h))
            } else {
                self.handle_world_pos(h)
            }
        }

        fn brush_slot_center_world(&self, h: &HandleData) -> Option<Point> {
            let n = self.nodes.get(&h.node_id)?;
            let hw = self.brush_handle_anchor_world(h)?;
            let nc = Point::new(n.x, n.y);
            let normal = crate::cavas::vcompute::normalize_or_zero(hw - nc);
            Some(hw + normal * self.brush_flush_distance)
        }

        /// @emoji 🖌️ World distance from pointer to brush slot when the pointer is on the slot, anchor, or sole-free node body.
        fn brush_slot_pointer_hit_distance(&self, world: Point, handle_id: &str, h: &HandleData) -> Option<f64> {
            let slot_center = self.brush_slot_center_world(h)?;
            let zoom = self.camera.zoom.max(1e-9);
            let slot_hit_r = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.brush_slot_hit_radius_world();
            let d_slot = distance_between(world, slot_center);
            if d_slot <= slot_hit_r {
                return Some(d_slot);
            }
            let anchor = self.brush_handle_anchor_world(h)?;
            let anchor_hit_r = (HANDLE_HIT_TOLERANCE_PX / zoom)
                + self.indirect_handle_marker_radius_world(h).max(self.effective_handle_radius(h));
            if distance_between(world, anchor) <= anchor_hit_r {
                return Some(d_slot);
            }
            if self.sole_eligible_indirect_handle_on_node(&h.node_id).as_deref() == Some(handle_id) {
                let n = self.nodes.get(&h.node_id)?;
                if self.point_in_node_world(n, world) {
                    return Some(d_slot);
                }
            }
            None
        }

        fn brush_nearest_slot_source(&self, world: Point) -> Option<String> {
            let mut best: Option<(f64, String)> = None;
            for (hid, h) in &self.handles {
                if !self.handle_effectively_visible(hid.as_str()) || self.handle_has_incident_edge(hid.as_str()) {
                    continue;
                }
                let Some(d) = self.brush_slot_pointer_hit_distance(world, hid.as_str(), h) else {
                    continue;
                };
                if best.as_ref().map(|(bd, _)| d < *bd).unwrap_or(true) {
                    best = Some((d, hid.clone()));
                }
            }
            best.map(|(_, id)| id)
        }

        fn brush_candidate_seed(source_handle_id: &str) -> u64 {
            source_handle_id.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(u64::from(b)))
        }

        fn brush_kind_weight(weights: &HashMap<String, f64>, id: &str, uniform_fallback: f64) -> f64 {
            weights
                .get(id)
                .copied()
                .filter(|w| w.is_finite() && *w > 0.0)
                .unwrap_or(uniform_fallback)
        }

        fn brush_next_seed(state: u64) -> u64 {
            state.wrapping_mul(6364136223846793005).wrapping_add(1)
        }

        fn brush_weighted_sample_index(weights: &[f64], seed: u64) -> usize {
            let wsum: f64 = weights.iter().sum();
            if wsum <= 0.0 {
                return 0;
            }
            let unit = (seed as f64) / (u64::MAX as f64);
            let mut r = unit * wsum;
            for (i, w) in weights.iter().enumerate() {
                if r <= *w || i + 1 == weights.len() {
                    return i;
                }
                r -= w;
            }
            weights.len().saturating_sub(1)
        }

        fn brush_weighted_order_strings(ids: &mut Vec<String>, seed: u64, weight_map: &HashMap<String, f64>) {
            if ids.len() < 2 {
                return;
            }
            let uniform = 1.0 / ids.len() as f64;
            let mut remaining: Vec<String> = std::mem::take(ids);
            let mut state = seed;
            while !remaining.is_empty() {
                let weights: Vec<f64> = remaining
                    .iter()
                    .map(|id| Self::brush_kind_weight(weight_map, id.as_str(), uniform))
                    .collect();
                state = Self::brush_next_seed(state);
                let pick = Self::brush_weighted_sample_index(&weights, state);
                ids.push(remaining.remove(pick));
            }
        }

        fn brush_compatible_node_kind_ids(&self, source: &HandleData) -> Vec<String> {
            let sn = self.nodes.get(&source.node_id).map(|n| n.node_kind.as_str()).unwrap_or("");
            let sh = source.handle_kind.as_str();
            let mut out: Vec<String> = Vec::new();
            for (kind_id, kind) in &self.node_kinds {
                if kind.handles.is_empty() {
                    continue;
                }
                let tn = kind_id.as_str();
                let compatible = kind.handles.iter().any(|t| self.link_kinds_compatible_for_brush(sn, sh, tn, t.handle_kind.as_str()));
                if compatible {
                    out.push(kind_id.clone());
                }
            }
            out
        }

        fn brush_template_world_pos(&self, center: Point, shape: NodeShape, radius: f64, width: f64, height: f64, angle: f64) -> Point {
            match shape {
                NodeShape::Circle => handle_position_on_circle(center, radius, angle),
                NodeShape::Rectangle => handle_position_on_rectangle(center, width, height, angle),
            }
        }

        fn brush_pick_target_handle_index(&self, source: &HandleData, node_kind_id: &str, kind: &NodeKindDef, cx: f64, cy: f64) -> Option<usize> {
            let sn = self.nodes.get(&source.node_id).map(|n| n.node_kind.as_str()).unwrap_or("");
            let sh = source.handle_kind.as_str();
            let tn = node_kind_id;
            let _ = (cx, cy);
            let mut compatible: Vec<(usize, f64)> = Vec::new();
            for (i, tmpl) in kind.handles.iter().enumerate() {
                if !self.link_kinds_compatible_for_brush(sn, sh, tn, tmpl.handle_kind.as_str()) {
                    continue;
                }
                let w = Self::brush_kind_weight(&self.brush_handle_kind_weights, tmpl.handle_kind.as_str(), 1.0);
                compatible.push((i, w));
            }
            if compatible.is_empty() {
                return None;
            }
            let weights: Vec<f64> = compatible.iter().map(|(_, w)| *w).collect();
            let seed = Self::brush_candidate_seed(node_kind_id)
                ^ source.id.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(u64::from(b)));
            let pick = Self::brush_weighted_sample_index(&weights, seed);
            Some(compatible[pick].0)
        }

        fn brush_build_preview(&self, source_handle_id: &str, node_kind_id: &str) -> Option<BrushPreviewSnapshot> {
            let source = self.handles.get(source_handle_id)?;
            let kind = self.node_kinds.get(node_kind_id)?;
            let center = self.brush_slot_center_world(source)?;
            let target_handle_index = self.brush_pick_target_handle_index(source, node_kind_id, kind, center.x, center.y)?;
            let radius = self.brush_node_size * 0.5 * kind.scale;
            let (width, height) = if kind.shape == NodeShape::Rectangle {
                (self.brush_node_size * kind.scale, self.brush_node_size * kind.scale)
            } else {
                (radius * 2.0, radius * 2.0)
            };
            Some(BrushPreviewSnapshot {
                source_handle_id: source_handle_id.to_string(),
                node_kind_id: node_kind_id.to_string(),
                x: center.x,
                y: center.y,
                shape: kind.shape,
                radius,
                width,
                height,
                handles: kind.handles.clone(),
                target_handle_index,
                icon_kind: kind.icon.clone(),
            })
        }

        fn brush_preview_json(preview: &BrushPreviewSnapshot) -> serde_json::Value {
            let mut node = json!({
                "nodeKind": preview.node_kind_id,
                "x": preview.x,
                "y": preview.y,
                "shape": if preview.shape == NodeShape::Rectangle { "rectangle" } else { "circle" },
            });
            if preview.shape == NodeShape::Rectangle {
                node["width"] = json!(preview.width);
                node["height"] = json!(preview.height);
            } else {
                node["radius"] = json!(preview.radius);
            }
            if let Some(ref icon) = preview.icon_kind {
                node["iconKind"] = json!(icon);
            }
            let handles: Vec<_> = preview
                .handles
                .iter()
                .map(|h| {
                    let mut row = json!({ "angle": h.angle, "handleKind": h.handle_kind });
                    if let Some(r) = h.radius {
                        row["radius"] = json!(r);
                    }
                    row
                })
                .collect();
            node["handles"] = json!(handles);
            json!({
                "node": node,
                "edge": {
                    "sourceHandleId": preview.source_handle_id,
                    "targetHandleIndex": preview.target_handle_index,
                }
            })
        }

        fn brush_place_json(preview: &BrushPreviewSnapshot, node_id: &str, edge_id: &str) -> serde_json::Value {
            let mut flat = json!({
                "nodeId": node_id,
                "edgeId": edge_id,
                "nodeKind": preview.node_kind_id,
                "sourceHandleId": preview.source_handle_id,
                "targetHandleIndex": preview.target_handle_index,
                "x": preview.x,
                "y": preview.y,
                "shape": if preview.shape == NodeShape::Rectangle { "rectangle" } else { "circle" },
            });
            if preview.shape == NodeShape::Rectangle {
                flat["width"] = json!(preview.width);
                flat["height"] = json!(preview.height);
            } else {
                flat["radius"] = json!(preview.radius);
            }
            if let Some(ref icon) = preview.icon_kind {
                flat["iconKind"] = json!(icon);
            }
            let handles: Vec<_> = preview
                .handles
                .iter()
                .map(|h| {
                    let mut row = json!({ "angle": h.angle, "handleKind": h.handle_kind });
                    if let Some(r) = h.radius {
                        row["radius"] = json!(r);
                    }
                    row
                })
                .collect();
            flat["handles"] = json!(handles);
            flat
        }

        fn brush_sync_preview_events(&mut self) {
            let key = self
                .brush_preview
                .as_ref()
                .map(|p| format!("{}|{}|{}|{}", p.source_handle_id, p.node_kind_id, p.x, p.y))
                .unwrap_or_default();
            if self.brush_preview_emit_key.as_deref() != Some(key.as_str()) {
                self.brush_preview_emit_key = Some(key.clone());
                if let Some(ref preview) = self.brush_preview {
                    self.push_event("brushPreview", Self::brush_preview_json(preview));
                } else {
                    self.push_event("brushPreview", json!({ "node": null, "edge": null }));
                }
            }
            let candidates_key = format!(
                "{}|{}|{}",
                self.brush_slot_source_id.as_deref().unwrap_or(""),
                self.brush_candidate_kinds.join(","),
                self.brush_candidate_index
            );
            if self.brush_candidates_emit_key.as_deref() != Some(candidates_key.as_str()) {
                self.brush_candidates_emit_key = Some(candidates_key);
                self.push_event(
                    "brushCandidates",
                    json!({
                        "sourceHandleId": self.brush_slot_source_id.clone().unwrap_or_default(),
                        "candidates": self.brush_candidate_kinds,
                        "index": self.brush_candidate_index,
                    }),
                );
            }
        }

        fn brush_clear_slot(&mut self) {
            let had_preview = self.brush_preview.is_some();
            self.brush_slot_source_id = None;
            self.brush_candidate_kinds.clear();
            self.brush_candidate_index = 0;
            self.brush_preview = None;
            if had_preview {
                self.bump_content_scene_generation();
                self.brush_preview_emit_key = None;
                self.brush_candidates_emit_key = None;
                self.brush_sync_preview_events();
            }
        }

        fn brush_allocate_placement_ids(&mut self) -> (String, String) {
            self.brush_placement_serial = self.brush_placement_serial.wrapping_add(1);
            let serial = self.brush_placement_serial;
            (format!("puzzle2d.brush.{serial}"), format!("puzzle2d.brush.edge.{serial}"))
        }

        fn brush_commit_preview(&mut self) {
            let Some(preview) = self.brush_preview.take() else {
                return;
            };
            let (node_id, edge_id) = self.brush_allocate_placement_ids();
            self.push_event("brushPlace", Self::brush_place_json(&preview, node_id.as_str(), edge_id.as_str()));
            self.bump_content_scene_generation();
            self.brush_preview_emit_key = None;
        }

        //#region 🪣Fill
        fn fill_preview_bounds(preview: &BrushPreviewSnapshot) -> WorldBox {
            match preview.shape {
                NodeShape::Rectangle => WorldBox {
                    min_x: preview.x - preview.width / 2.0,
                    min_y: preview.y - preview.height / 2.0,
                    max_x: preview.x + preview.width / 2.0,
                    max_y: preview.y + preview.height / 2.0,
                },
                NodeShape::Circle => WorldBox {
                    min_x: preview.x - preview.radius,
                    min_y: preview.y - preview.radius,
                    max_x: preview.x + preview.radius,
                    max_y: preview.y + preview.radius,
                },
            }
        }

        fn fill_handle_connected(&self, accum: &FillAccum, handle_id: &str) -> bool {
            accum.connected_handles.contains(handle_id) || self.handle_has_incident_edge(handle_id)
        }

        fn fill_collect_free_handles(&self, accum: &FillAccum) -> Vec<String> {
            let mut out = Vec::new();
            for (id, h) in &self.handles {
                if self.handle_effectively_visible(id.as_str()) && !self.fill_handle_connected(accum, id.as_str()) {
                    out.push(id.clone());
                }
                let _ = h;
            }
            for (id, vh) in &accum.virtual_handles {
                if !accum.connected_handles.contains(id) && accum.virtual_nodes.contains_key(&vh.node_id) {
                    out.push(id.clone());
                }
            }
            out
        }

        fn fill_source_node_and_handle_kind(&self, accum: &FillAccum, handle_id: &str) -> Option<(String, String)> {
            if let Some(h) = self.handles.get(handle_id) {
                let nk = self.nodes.get(&h.node_id)?.node_kind.clone();
                return Some((nk, h.handle_kind.clone()));
            }
            let vh = accum.virtual_handles.get(handle_id)?;
            let node_kind = accum.virtual_nodes.get(&vh.node_id)?.node_kind.clone();
            Some((node_kind, vh.handle_kind.clone()))
        }

        fn fill_virtual_handle_anchor_world(node: &FillVirtualNode, tmpl: &NodeKindHandleTemplate) -> Point {
            let center = Point::new(node.x, node.y);
            match node.shape {
                NodeShape::Circle => handle_position_on_circle(center, node.radius, tmpl.angle),
                NodeShape::Rectangle => handle_position_on_rectangle(center, node.width, node.height, tmpl.angle),
            }
        }

        fn fill_slot_center_world(&self, accum: &FillAccum, handle_id: &str) -> Option<Point> {
            if let Some(h) = self.handles.get(handle_id) {
                return self.brush_slot_center_world(h);
            }
            let vh = accum.virtual_handles.get(handle_id)?;
            let node = accum.virtual_nodes.get(&vh.node_id)?;
            let hw = Self::fill_virtual_handle_anchor_world(node, &vh.template);
            let nc = Point::new(node.x, node.y);
            let normal = crate::cavas::vcompute::normalize_or_zero(hw - nc);
            Some(hw + normal * self.brush_flush_distance)
        }

        fn fill_weight_for_handle(&self, accum: &FillAccum, handle_id: &str, uniform: f64) -> f64 {
            let hk = if let Some(h) = self.handles.get(handle_id) {
                h.handle_kind.as_str()
            } else {
                accum.virtual_handles.get(handle_id).map(|vh| vh.handle_kind.as_str()).unwrap_or("")
            };
            Self::brush_kind_weight(&self.brush_handle_kind_weights, hk, uniform)
        }

        fn fill_order_handles(&self, accum: &FillAccum, handles: &mut Vec<String>, seed: u64) {
            if handles.len() < 2 {
                return;
            }
            let uniform = 1.0 / handles.len() as f64;
            let mut remaining = std::mem::take(handles);
            let mut state = seed;
            while !remaining.is_empty() {
                let weights: Vec<f64> = remaining.iter().map(|id| self.fill_weight_for_handle(accum, id.as_str(), uniform)).collect();
                state = Self::brush_next_seed(state);
                let pick = Self::brush_weighted_sample_index(&weights, state);
                handles.push(remaining.remove(pick));
            }
        }

        fn fill_compatible_node_kind_ids(&self, accum: &FillAccum, source_handle_id: &str) -> Vec<String> {
            let Some((sn, sh)) = self.fill_source_node_and_handle_kind(accum, source_handle_id) else {
                return Vec::new();
            };
            let mut out: Vec<String> = Vec::new();
            for (kind_id, kind) in &self.node_kinds {
                if kind.handles.is_empty() {
                    continue;
                }
                let tn = kind_id.as_str();
                let compatible = kind
                    .handles
                    .iter()
                    .any(|t| self.link_kinds_compatible_for_brush(sn.as_str(), sh.as_str(), tn, t.handle_kind.as_str()));
                if compatible {
                    out.push(kind_id.clone());
                }
            }
            out
        }

        fn fill_pick_target_handle_index(&self, sn: &str, sh: &str, node_kind_id: &str, kind: &NodeKindDef, seed: u64) -> Option<usize> {
            let tn = node_kind_id;
            let mut compatible: Vec<(usize, f64)> = Vec::new();
            for (i, tmpl) in kind.handles.iter().enumerate() {
                if !self.link_kinds_compatible_for_brush(sn, sh, tn, tmpl.handle_kind.as_str()) {
                    continue;
                }
                let w = Self::brush_kind_weight(&self.brush_handle_kind_weights, tmpl.handle_kind.as_str(), 1.0);
                compatible.push((i, w));
            }
            if compatible.is_empty() {
                return None;
            }
            let weights: Vec<f64> = compatible.iter().map(|(_, w)| *w).collect();
            let pick = Self::brush_weighted_sample_index(&weights, seed);
            Some(compatible[pick].0)
        }

        fn fill_build_preview(&self, accum: &FillAccum, source_handle_id: &str, node_kind_id: &str, seed: u64) -> Option<BrushPreviewSnapshot> {
            let center = self.fill_slot_center_world(accum, source_handle_id)?;
            let (sn, sh) = self.fill_source_node_and_handle_kind(accum, source_handle_id)?;
            let kind = self.node_kinds.get(node_kind_id)?;
            let target_handle_index = self.fill_pick_target_handle_index(sn.as_str(), sh.as_str(), node_kind_id, kind, seed)?;
            let radius = self.brush_node_size * 0.5 * kind.scale;
            let (width, height) = if kind.shape == NodeShape::Rectangle {
                (self.brush_node_size * kind.scale, self.brush_node_size * kind.scale)
            } else {
                (radius * 2.0, radius * 2.0)
            };
            Some(BrushPreviewSnapshot {
                source_handle_id: source_handle_id.to_string(),
                node_kind_id: node_kind_id.to_string(),
                x: center.x,
                y: center.y,
                shape: kind.shape,
                radius,
                width,
                height,
                handles: kind.handles.clone(),
                target_handle_index,
                icon_kind: kind.icon.clone(),
            })
        }

        fn fill_collides(&self, accum: &FillAccum, preview: &BrushPreviewSnapshot) -> bool {
            let bounds = Self::fill_preview_bounds(preview);
            for n in self.nodes.values() {
                if world_boxes_overlap(bounds, self.node_world_bounds(n, 0.0)) {
                    return true;
                }
            }
            for vb in &accum.virtual_bounds {
                if world_boxes_overlap(bounds, *vb) {
                    return true;
                }
            }
            false
        }

        fn fill_apply_placement(accum: &mut FillAccum, preview: BrushPreviewSnapshot) {
            let serial = accum.next_serial;
            accum.next_serial += 1;
            let node_id = format!("puzzle2d.fill.{serial}");
            let edge_id = format!("puzzle2d.fill.edge.{serial}");
            let target_handle_id = format!("{node_id}:h{}", preview.target_handle_index);
            accum.connected_handles.insert(preview.source_handle_id.clone());
            accum.connected_handles.insert(target_handle_id);
            accum.virtual_bounds.push(Self::fill_preview_bounds(&preview));
            accum.virtual_nodes.insert(
                node_id.clone(),
                FillVirtualNode {
                    node_kind: preview.node_kind_id.clone(),
                    x: preview.x,
                    y: preview.y,
                    shape: preview.shape,
                    radius: preview.radius,
                    width: preview.width,
                    height: preview.height,
                },
            );
            for (i, tmpl) in preview.handles.iter().enumerate() {
                let hid = format!("{node_id}:h{i}");
                if accum.connected_handles.contains(&hid) {
                    continue;
                }
                accum.virtual_handles.insert(
                    hid,
                    FillVirtualHandle {
                        node_id: node_id.clone(),
                        handle_kind: tmpl.handle_kind.clone(),
                        template: tmpl.clone(),
                    },
                );
            }
            accum.placements.push((node_id, edge_id, preview));
        }

        /// @emoji 🪣 Deterministic frontier fill sequence (weighted distribution + AABB collision).
        pub fn brush_fill_json(&self, max_count: u32, seed: u64) -> String {
            let mut accum = FillAccum::default();
            let max = max_count.min(1000) as usize;
            let mut state = seed;
            while accum.placements.len() < max {
                let mut free = self.fill_collect_free_handles(&accum);
                if free.is_empty() {
                    break;
                }
                state = Self::brush_next_seed(state);
                self.fill_order_handles(&accum, &mut free, state);
                let mut placed = false;
                for source_handle_id in &free {
                    let mut kinds = self.fill_compatible_node_kind_ids(&accum, source_handle_id.as_str());
                    if kinds.is_empty() {
                        continue;
                    }
                    state = Self::brush_next_seed(state);
                    Self::brush_weighted_order_strings(&mut kinds, state, &self.brush_node_kind_weights);
                    for node_kind_id in &kinds {
                        state = Self::brush_next_seed(state);
                        let Some(preview) = self.fill_build_preview(&accum, source_handle_id.as_str(), node_kind_id.as_str(), state) else {
                            continue;
                        };
                        if self.fill_collides(&accum, &preview) {
                            continue;
                        }
                        Self::fill_apply_placement(&mut accum, preview);
                        placed = true;
                        break;
                    }
                    if placed {
                        break;
                    }
                }
                if !placed {
                    break;
                }
            }
            let placements: Vec<serde_json::Value> = accum
                .placements
                .iter()
                .map(|(node_id, edge_id, preview)| Self::brush_place_json(preview, node_id.as_str(), edge_id.as_str()))
                .collect();
            serde_json::json!({ "placements": placements }).to_string()
        }
        //#endregion 🪣Fill

        fn brush_preview_snapshot_from_session_json(node: &serde_json::Value, edge: &serde_json::Value, source_handle_id: &str) -> Option<BrushPreviewSnapshot> {
            let node_kind_id = node.get("nodeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
            let x = node.get("x").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
            let y = node.get("y").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
            let shape = match node.get("shape").and_then(|x| x.as_str()).map(str::trim) {
                Some("rectangle") => NodeShape::Rectangle,
                _ => NodeShape::Circle,
            };
            let (radius, width, height) = match shape {
                NodeShape::Circle => (node.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?, 0.0, 0.0),
                NodeShape::Rectangle => {
                    let w = node.get("width").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                    let h = node.get("height").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                    (0.0, w, h)
                }
            };
            let target_handle_index = edge.get("targetHandleIndex").and_then(|v| v.as_u64()).map(|v| v as usize)?;
            let mut handles: Vec<NodeKindHandleTemplate> = Vec::new();
            if let Some(arr) = node.get("handles").and_then(|x| x.as_array()) {
                for row in arr {
                    let ho = row.as_object()?;
                    let handle_kind = ho.get("handleKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
                    let angle = ho.get("angle").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
                    let radius = ho.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0);
                    handles.push(NodeKindHandleTemplate { handle_kind: handle_kind.to_string(), angle, radius });
                }
            }
            if handles.is_empty() {
                return None;
            }
            let icon_kind = node.get("iconKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
            Some(BrushPreviewSnapshot {
                source_handle_id: source_handle_id.to_string(),
                node_kind_id: node_kind_id.to_string(),
                x,
                y,
                shape,
                radius,
                width,
                height,
                handles,
                target_handle_index,
                icon_kind,
            })
        }

        /// @emoji 🖌️ Mirrors brush slot + preview from another authoring pane (no pointer input on this host).
        pub fn set_brush_session_mirror_json(&mut self, json: &str) -> Result<(), String> {
            if json.trim().is_empty() {
                self.brush_slot_source_id = None;
                self.brush_candidate_kinds.clear();
                self.brush_candidate_index = 0;
                self.brush_preview = None;
                self.brush_preview_emit_key = None;
                self.brush_candidates_emit_key = None;
                self.bump_content_scene_generation();
                return Ok(());
            }
            let v: serde_json::Value = serde_json::from_str(json).map_err(|e| format!("setBrushSessionJson: {e}"))?;
            let source = v.get("sourceHandleId").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
            self.brush_slot_source_id = source.clone();
            self.brush_candidate_kinds = v
                .get("candidates")
                .and_then(|x| x.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|x| x.as_str().map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string()))
                        .collect()
                })
                .unwrap_or_default();
            self.brush_candidate_index = v.get("index").and_then(|x| x.as_u64()).map(|i| i as usize).unwrap_or(0);
            if self.brush_candidate_kinds.is_empty() {
                self.brush_candidate_index = 0;
            } else {
                self.brush_candidate_index %= self.brush_candidate_kinds.len();
            }
            self.brush_preview = match (source.as_deref(), v.get("preview")) {
                (Some(source_id), Some(preview)) if !preview.is_null() => {
                    let node = preview.get("node").filter(|n| !n.is_null());
                    let edge = preview.get("edge").filter(|e| !e.is_null());
                    match (node, edge) {
                        (Some(node), Some(edge)) => Self::brush_preview_snapshot_from_session_json(node, edge, source_id),
                        _ => None,
                    }
                }
                _ => None,
            };
            self.brush_preview_emit_key = None;
            self.brush_candidates_emit_key = None;
            self.bump_content_scene_generation();
            Ok(())
        }

        fn brush_enter_slot(&mut self, source_handle_id: String) {
            if self.brush_slot_source_id.as_deref() == Some(source_handle_id.as_str()) {
                return;
            }
            self.brush_commit_preview();
            self.brush_slot_source_id = Some(source_handle_id.clone());
            let mut candidates = self
                .handles
                .get(source_handle_id.as_str())
                .map(|h| self.brush_compatible_node_kind_ids(h))
                .unwrap_or_default();
            Self::brush_weighted_order_strings(
                &mut candidates,
                Self::brush_candidate_seed(&source_handle_id),
                &self.brush_node_kind_weights,
            );
            self.brush_candidate_kinds = candidates;
            self.brush_candidate_index = 0;
            self.brush_rebuild_preview();
        }

        fn brush_rebuild_preview(&mut self) {
            let Some(ref source_id) = self.brush_slot_source_id else {
                self.brush_preview = None;
                self.brush_sync_preview_events();
                return;
            };
            let kind_id = self.brush_candidate_kinds.get(self.brush_candidate_index).cloned();
            self.brush_preview = kind_id.as_deref().and_then(|k| self.brush_build_preview(source_id, k));
            if self.brush_preview.is_some() {
                self.bump_content_scene_generation();
            }
            self.brush_preview_emit_key = None;
            self.brush_candidates_emit_key = None;
            self.brush_sync_preview_events();
        }

        fn brush_pointer_move(&mut self, world: Point) {
            if let Some(slot) = self.brush_nearest_slot_source(world) {
                self.brush_enter_slot(slot);
                self.set_hovered_id(self.brush_slot_source_id.clone());
            } else if self.brush_slot_source_id.is_some() {
                self.brush_commit_preview();
                self.brush_clear_slot();
                self.set_hovered_id(None);
            }
        }

        pub fn set_active_tool(&mut self, label: &str) {
            let next = if label == "brush" { ActiveTool::Brush } else { ActiveTool::Select };
            if self.active_tool == next {
                return;
            }
            if self.active_tool == ActiveTool::Brush {
                self.brush_commit_preview();
                self.brush_clear_slot();
            }
            self.active_tool = next;
            self.interaction = Interaction::None;
            self.bump_content_scene_generation();
        }

        pub fn set_brush_flush_distance(&mut self, distance: f64) {
            let d = if distance.is_finite() && distance >= 0.0 { distance } else { DEFAULT_BRUSH_FLUSH_DISTANCE };
            if (self.brush_flush_distance - d).abs() < 1e-9 {
                return;
            }
            self.brush_flush_distance = d;
            if self.active_tool == ActiveTool::Brush {
                self.brush_preview_emit_key = None;
                self.brush_rebuild_preview();
            }
        }

        pub fn set_brush_kind_weights(&mut self, json: &str) {
            if json.is_empty() {
                return;
            }
            let Ok(v) = serde_json::from_str::<serde_json::Value>(json) else {
                return;
            };
            self.brush_node_kind_weights.clear();
            self.brush_handle_kind_weights.clear();
            if let Some(obj) = v.get("nodeWeights").and_then(|x| x.as_object()) {
                for (k, val) in obj {
                    if let Some(w) = val.as_f64() {
                        if w.is_finite() && w >= 0.0 {
                            self.brush_node_kind_weights.insert(k.clone(), w);
                        }
                    }
                }
            }
            if let Some(obj) = v.get("handleWeights").and_then(|x| x.as_object()) {
                for (k, val) in obj {
                    if let Some(w) = val.as_f64() {
                        if w.is_finite() && w >= 0.0 {
                            self.brush_handle_kind_weights.insert(k.clone(), w);
                        }
                    }
                }
            }
            if self.active_tool != ActiveTool::Brush {
                return;
            }
            if let Some(source) = self.brush_slot_source_id.clone() {
                self.brush_slot_source_id = None;
                self.brush_enter_slot(source);
            } else {
                self.brush_preview_emit_key = None;
                self.brush_rebuild_preview();
            }
        }

        pub fn set_brush_node_size(&mut self, size: f64) {
            let s = if size.is_finite() && size > 0.0 { size } else { DEFAULT_BRUSH_NODE_SIZE };
            if (self.brush_node_size - s).abs() < 1e-9 {
                return;
            }
            self.brush_node_size = s;
            if self.active_tool == ActiveTool::Brush {
                self.brush_preview_emit_key = None;
                self.brush_rebuild_preview();
            }
        }

        pub fn brush_cycle_candidate(&mut self, forward: bool) {
            if self.brush_candidate_kinds.len() < 2 {
                return;
            }
            let len = self.brush_candidate_kinds.len();
            self.brush_candidate_index = if forward {
                (self.brush_candidate_index + 1) % len
            } else {
                (self.brush_candidate_index + len - 1) % len
            };
            self.brush_rebuild_preview();
        }

        pub fn brush_set_candidate_index(&mut self, index: usize) {
            if self.brush_candidate_kinds.is_empty() {
                return;
            }
            self.brush_candidate_index = index % self.brush_candidate_kinds.len();
            self.brush_rebuild_preview();
        }

        fn append_brush_node_icon_paint(
            &self,
            scene: &mut Scene,
            lod: BoardDrawLod,
            center: Point,
            shape: NodeShape,
            radius: f64,
            width: f64,
            height: f64,
            icon_kind: &str,
            fill: Color,
            stroke_c: Color,
        ) {
            if !matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro) {
                return;
            }
            let preserve_original_style = false;
            let Some((bx, by, bw, bh, body)) = self.get_or_build_icon_paint(icon_kind, stroke_c, fill, preserve_original_style) else {
                return;
            };
            let clip_inset = 0.88;
            let fit_inset = 0.76;
            let (sx_half, sy_half) = match shape {
                NodeShape::Circle => {
                    let s = self.draw_space_len(radius, false) * fit_inset;
                    (s, s)
                }
                NodeShape::Rectangle => (
                    self.draw_space_len(width, false) * fit_inset * 0.5,
                    self.draw_space_len(height, false) * fit_inset * 0.5,
                ),
            };
            let center_ds = self.draw_space_point(center, false);
            let cx = bx + bw * 0.5;
            let cy = by + bh * 0.5;
            let avail_w = 2.0 * sx_half;
            let avail_h = 2.0 * sy_half;
            let scale = (avail_w / bw).min(avail_h / bh);
            let aff = Affine::translate((center_ds.x - scale * cx, center_ds.y - scale * cy)) * Affine::scale(scale);
            match shape {
                NodeShape::Circle => {
                    let r_clip = self.draw_space_len(radius, false) * clip_inset;
                    let disc = Circle::new(center_ds, r_clip);
                    scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &disc);
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
                NodeShape::Rectangle => {
                    let hw = self.draw_space_len(width, false) * clip_inset * 0.5;
                    let hh = self.draw_space_len(height, false) * clip_inset * 0.5;
                    let clip_r = Rect::from_points(Point::new(center_ds.x - hw, center_ds.y - hh), Point::new(center_ds.x + hw, center_ds.y + hh));
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
        }

        fn paint_highlighted_node_preview(
            &self,
            scene: &mut Scene,
            _lod: BoardDrawLod,
            x: f64,
            y: f64,
            shape: NodeShape,
            radius: f64,
            width: f64,
            height: f64,
            icon_kind: Option<&str>,
        ) {
            let center = Point::new(x, y);
            let style = BoardElementStyleKind::Highlighted;
            let fill = Self::node_fill_for_style(&self.vello_theme, style);
            let stroke_c = Self::node_stroke_for_style(&self.vello_theme, style);
            let stroke = Stroke::new(2.0);
            match shape {
                NodeShape::Circle => {
                    let c = self.draw_space_point(center, false);
                    let r = self.draw_space_len(radius, false);
                    let circle = Circle::new(c, r);
                    scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &circle);
                    scene.stroke(&stroke, Affine::IDENTITY, stroke_c, None, &circle);
                }
                NodeShape::Rectangle => {
                    let hw = width * 0.5;
                    let hh = height * 0.5;
                    let p0 = self.draw_space_point(Point::new(x - hw, y - hh), false);
                    let p1 = self.draw_space_point(Point::new(x + hw, y + hh), false);
                    let rect = Rect::from_points(p0, p1);
                    scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &rect);
                    scene.stroke(&stroke, Affine::IDENTITY, stroke_c, None, &rect);
                }
            }
            if let Some(icon) = icon_kind.map(str::trim).filter(|s| !s.is_empty()) {
                self.append_brush_node_icon_paint(scene, BoardDrawLod::Detail, center, shape, radius, width, height, icon, fill, stroke_c);
            }
        }

        fn fixture_drop_preview_effective_dims(
            &self,
            preview: &FixtureDropPreviewSnapshot,
        ) -> (NodeShape, f64, f64, f64) {
            if let Some(kind) = self.node_kinds.get(preview.node_kind_id.as_str()) {
                let radius = self.brush_node_size * 0.5 * kind.scale;
                let (width, height) = if kind.shape == NodeShape::Rectangle {
                    (self.brush_node_size * kind.scale, self.brush_node_size * kind.scale)
                } else {
                    (radius * 2.0, radius * 2.0)
                };
                return (kind.shape, radius, width, height);
            }
            (preview.shape, preview.radius, preview.width, preview.height)
        }

        fn fixture_drop_preview_from_json(&self, node: &serde_json::Value) -> Option<FixtureDropPreviewSnapshot> {
            let node_kind_id = node.get("nodeKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty())?;
            let (x, y) = match (
                node.get("screenX").and_then(|v| v.as_f64()).filter(|v| v.is_finite()),
                node.get("screenY").and_then(|v| v.as_f64()).filter(|v| v.is_finite()),
            ) {
                (Some(sx), Some(sy)) => {
                    let world = self.screen_to_world(Point::new(sx, sy));
                    (world.x, world.y)
                }
                _ => {
                    let x = node.get("x").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
                    let y = node.get("y").and_then(|v| v.as_f64()).filter(|v| v.is_finite())?;
                    (x, y)
                }
            };
            let shape = match node.get("shape").and_then(|x| x.as_str()).map(str::trim) {
                Some("rectangle") => NodeShape::Rectangle,
                _ => NodeShape::Circle,
            };
            let (radius, width, height) = match shape {
                NodeShape::Circle => (node.get("radius").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?, 0.0, 0.0),
                NodeShape::Rectangle => {
                    let w = node.get("width").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                    let h = node.get("height").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0)?;
                    (0.0, w, h)
                }
            };
            let icon_kind = node.get("iconKind").and_then(|x| x.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
            Some(FixtureDropPreviewSnapshot {
                node_kind_id: node_kind_id.to_string(),
                x,
                y,
                shape,
                radius,
                width,
                height,
                icon_kind,
            })
        }

        /// @emoji 👻 Sets or clears the workbench palette fixture drop ghost node (independent of brush tool).
        pub fn set_fixture_drop_preview_json(&mut self, json: &str) -> Result<(), String> {
            if json.trim().is_empty() {
                self.fixture_drop_preview = None;
                self.bump_content_scene_generation();
                return Ok(());
            }
            let v: serde_json::Value = serde_json::from_str(json).map_err(|e| format!("setFixtureDropPreviewJson: {e}"))?;
            self.fixture_drop_preview = self.fixture_drop_preview_from_json(&v);
            if self.fixture_drop_preview.is_none() {
                return Err("setFixtureDropPreviewJson: preview payload missing nodeKind, screen/world point, or size".into());
            }
            self.bump_content_scene_generation();
            Ok(())
        }

        fn append_fixture_drop_preview_paint(&self, scene: &mut Scene, lod: BoardDrawLod) {
            let Some(ref preview) = self.fixture_drop_preview else {
                return;
            };
            let (shape, radius, width, height) = self.fixture_drop_preview_effective_dims(preview);
            let icon_kind = preview
                .icon_kind
                .as_deref()
                .filter(|s| !s.is_empty())
                .or_else(|| self.node_kinds.get(preview.node_kind_id.as_str()).and_then(|k| k.icon.as_deref()));
            self.paint_highlighted_node_preview(scene, lod, preview.x, preview.y, shape, radius, width, height, icon_kind);
        }

        fn append_brush_preview_paint(&self, scene: &mut Scene, lod: BoardDrawLod) {
            let Some(ref preview) = self.brush_preview else {
                return;
            };
            let _ = lod;
            self.paint_highlighted_node_preview(
                scene,
                lod,
                preview.x,
                preview.y,
                preview.shape,
                preview.radius,
                preview.width,
                preview.height,
                preview.icon_kind.as_deref(),
            );
            let center = Point::new(preview.x, preview.y);
            let source = match self.handles.get(preview.source_handle_id.as_str()) {
                Some(h) => h,
                None => return,
            };
            let src_pos = match self.handle_world_pos(source) {
                Some(p) => p,
                None => return,
            };
            let tmpl = match preview.handles.get(preview.target_handle_index) {
                Some(t) => t,
                None => return,
            };
            let tgt_pos = self.brush_template_world_pos(center, preview.shape, preview.radius, preview.width, preview.height, tmpl.angle);
            let Some(src_node) = self.nodes.get(&source.node_id) else {
                return;
            };
            let tgt_center = center;
            let curve = compute_edge_bezier_points(src_pos, tgt_pos, Point::new(src_node.x, src_node.y), tgt_center);
            let p0 = self.draw_space_point(curve.p0, false);
            let p1 = self.draw_space_point(curve.p1, false);
            let p2 = self.draw_space_point(curve.p2, false);
            let p3 = self.draw_space_point(curve.p3, false);
            let bez = CubicBez::new(p0, p1, p2, p3);
            scene.stroke(&Stroke::new(2.85), Affine::IDENTITY, self.vello_theme.wire_stroke_highlighted, None, &bez);
        }

        /// @emoji 🧩 Selects world-space clip tiling for Vello scene construction (`none` | `world-clip`).
        pub fn set_world_raster_tiling(&mut self, mode: &str) {
            let next = if mode == "world-clip" { "world-clip".into() } else { "none".into() };
            if self.world_raster_tiling == next {
                return;
            }
            self.world_raster_tiling = next;
        }

        pub fn set_original_element_style(&mut self, enabled: bool) {
            if self.preserve_original_element_style == enabled {
                return;
            }
            self.preserve_original_element_style = enabled;
            self.icon_vector_cache.borrow_mut().clear();
        }

        pub fn set_selection_screen_preview(&mut self, points: Option<Vec<Point>>) {
            if points.is_none() {
                self.selection_preview_crossing = false;
            }
            self.selection_screen_preview = points;
        }

        pub fn set_vello_theme_from_json(&mut self, json: &str) -> Result<(), String> {
            let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
            let mut next = self.vello_theme;
            if let Some(arr) = v.get("rasterClear").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.raster_clear = c;
                }
            }
            if let Some(arr) = v.get("gridMinorStroke").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.grid_minor_stroke = c;
                }
            }
            if let Some(arr) = v.get("edgeStroke").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.edge_stroke = c;
                }
            }
            if let Some(arr) = v.get("edgeStrokeHovered").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.edge_stroke_hovered = c;
                }
            }
            if let Some(arr) = v.get("edgeStrokeSelected").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.edge_stroke_selected = c;
                }
            }
            if let Some(arr) = v.get("edgeStrokeSelectionExit").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.edge_stroke_selection_exit = c;
                }
            }
            if let Some(arr) = v.get("edgeStrokeDisabled").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.edge_stroke_disabled = c;
                }
            }
            if let Some(arr) = v.get("nodeFill").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.node_fill = c;
                }
            }
            if let Some(arr) = v.get("nodeStroke").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.node_stroke = c;
                }
            }
            if let Some(arr) = v.get("nodeFillHovered").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.node_fill_hovered = c;
                }
            }
            if let Some(arr) = v.get("nodeStrokeHovered").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.node_stroke_hovered = c;
                }
            }
            if let Some(arr) = v.get("nodeFillSelected").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.node_fill_selected = c;
                }
            }
            if let Some(arr) = v.get("nodeStrokeSelected").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.node_stroke_selected = c;
                }
            }
            if let Some(arr) = v.get("nodeFillSelectionExit").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.node_fill_selection_exit = c;
                }
            }
            if let Some(arr) = v.get("nodeStrokeSelectionExit").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.node_stroke_selection_exit = c;
                }
            }
            if let Some(arr) = v.get("nodeFillDisabled").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.node_fill_disabled = c;
                }
            }
            if let Some(arr) = v.get("nodeStrokeDisabled").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.node_stroke_disabled = c;
                }
            }
            if let Some(arr) = v.get("indirectHandleFill").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.indirect_handle_fill = c;
                }
            }
            if let Some(arr) = v.get("indirectHandleStroke").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.indirect_handle_stroke = c;
                }
            }
            if let Some(arr) = v.get("handleFill").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.handle_fill = c;
                }
            }
            if let Some(arr) = v.get("handleStroke").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.handle_stroke = c;
                }
            }
            if let Some(arr) = v.get("handleFillHovered").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.handle_fill_hovered = c;
                }
            }
            if let Some(arr) = v.get("handleStrokeHovered").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.handle_stroke_hovered = c;
                }
            }
            if let Some(arr) = v.get("handleFillSelected").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.handle_fill_selected = c;
                }
            }
            if let Some(arr) = v.get("handleStrokeSelected").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.handle_stroke_selected = c;
                }
            }
            if let Some(arr) = v.get("handleFillSelectionExit").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.handle_fill_selection_exit = c;
                }
            }
            if let Some(arr) = v.get("handleStrokeSelectionExit").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.handle_stroke_selection_exit = c;
                }
            }
            if let Some(arr) = v.get("handleFillDisabled").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.handle_fill_disabled = c;
                }
            }
            if let Some(arr) = v.get("handleStrokeDisabled").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.handle_stroke_disabled = c;
                }
            }
            if let Some(arr) = v.get("wireStroke").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.wire_stroke = c;
                }
            }
            if let Some(arr) = v.get("wireStrokeHovered").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.wire_stroke_hovered = c;
                }
            }
            if let Some(arr) = v.get("wireStrokeSelected").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.wire_stroke_selected = c;
                }
            }
            if let Some(arr) = v.get("wireStrokeHighlighted").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.wire_stroke_highlighted = c;
                }
            }
            if let Some(arr) = v.get("wireStrokeDisabled").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.wire_stroke_disabled = c;
                }
            }
            if let Some(arr) = v.get("selectionPreviewFill").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.selection_preview_fill = c;
                }
            }
            if let Some(arr) = v.get("selectionPreviewStroke").and_then(|x| x.as_array()) {
                if let Some(c) = Self::color_from_json_rgba8(arr) {
                    next.selection_preview_stroke = c;
                }
            }
            self.vello_theme = next;
            self.icon_vector_cache.borrow_mut().clear();
            Ok(())
        }

        fn sync_selection_screen_overlay(&mut self, start_screen: Point, screen_points: &[Point]) {
            if screen_points.len() < 2 {
                self.selection_screen_preview = None;
                self.selection_preview_crossing = false;
                return;
            }
            let last = *screen_points.last().unwrap_or(&start_screen);
            self.selection_preview_crossing = last.x < start_screen.x;
            self.selection_screen_preview = Some(if self.selection_options.method == "lasso" {
                screen_points.to_vec()
            } else {
                vec![start_screen, Point::new(last.x, start_screen.y), last, Point::new(start_screen.x, last.y)]
            });
        }

        fn push_event(&mut self, name: &str, payload: serde_json::Value) {
            self.events.push(json!({ "name": name, "payload": payload }));
        }

        /// @emoji 🏁 Emits final node coordinates after a drag gesture so hosts can commit declarative fixture state once.
        fn push_node_drag_end_events(&mut self, start_positions: &BTreeMap<String, (f64, f64)>) {
            let mut moves = Vec::with_capacity(start_positions.len());
            for id in start_positions.keys() {
                let Some(node) = self.nodes.get(id) else {
                    continue;
                };
                moves.push(json!({ "id": id, "x": node.x, "y": node.y }));
            }
            if moves.is_empty() {
                return;
            }
            self.push_event("nodeDragEnd", json!({ "moves": moves }));
        }

        pub fn drain_events_json(&mut self) -> String {
            let out = serde_json::to_string(&self.events).unwrap_or_else(|_| "[]".into());
            self.events.clear();
            out
        }

        fn is_preselecting(&self) -> bool {
            matches!(&self.interaction, Interaction::Selection { .. })
        }

        /// @emoji 💠 Live area-select preview ids, or committed selection when not preselecting.
        fn selection_chrome_ids(&self) -> BTreeSet<String> {
            if self.is_preselecting() || !self.preselect.is_empty() {
                self.preselect.clone()
            } else {
                self.selection.clone()
            }
        }

        /// @emoji 🖱️ Empty selection on background click without exit/highlight chrome or preselect.
        fn clear_selection_on_background_click(&mut self) {
            if self.selection.is_empty() {
                return;
            }
            self.preselect.clear();
            self.preselect_removed.clear();
            self.last_preselect_emit_sig = None;
            self.last_select_emit_sig = None;
            self.selection_exit_highlight.clear();
            self.selection.clear();
            self.sync_selection_flags_to_objects();
            self.push_event("select", json!({ "ids": [], "exitHighlightIds": [] }));
        }

        fn sync_selection_flags_to_objects(&mut self) {
            let chrome = self.selection_chrome_ids();
            for n in self.nodes.values_mut() {
                n.selected = chrome.contains(&n.id);
            }
            for h in self.handles.values_mut() {
                h.selected = chrome.contains(&h.id);
            }
            for e in self.edges.values_mut() {
                e.selected = chrome.contains(&e.id);
            }
            for w in self.wires.values_mut() {
                w.selected = chrome.contains(&w.id);
            }
        }

        fn push_select_event(&mut self) {
            self.last_select_emit_sig = None;
            let mut sorted: Vec<_> = self.selection.iter().cloned().collect();
            sorted.sort();
            self.push_event("select", json!({ "ids": sorted, "exitHighlightIds": [] }));
        }

        pub fn set_selection_ids(&mut self, ids: &[String]) {
            let next: BTreeSet<String> = ids.iter().cloned().collect();
            if next == self.selection {
                return;
            }
            self.preselect.clear();
            self.preselect_removed.clear();
            self.last_preselect_emit_sig = None;
            self.selection_exit_highlight.clear();
            self.selection = next;
            self.sync_selection_flags_to_objects();
            self.push_select_event();
        }

        /// @emoji 🔇 Updates committed selection without emitting `select` (controlled React sync).
        pub fn set_selection_ids_silent(&mut self, ids: &[String]) {
            let next: BTreeSet<String> = ids.iter().cloned().collect();
            if next == self.selection {
                return;
            }
            self.preselect.clear();
            self.preselect_removed.clear();
            self.last_preselect_emit_sig = None;
            self.selection_exit_highlight.clear();
            self.selection = next;
            self.sync_selection_flags_to_objects();
        }

        /// @emoji 🔇 Mirrors area-select preview chrome without emitting `preselect` (shared multi-view sync).
        pub fn set_preselect_state_silent(&mut self, ids: &[String], removed_ids: &[String]) {
            let next: BTreeSet<String> = ids.iter().cloned().collect();
            let removed: BTreeSet<String> = removed_ids.iter().cloned().collect();
            if self.preselect == next && self.preselect_removed == removed {
                return;
            }
            self.preselect = next;
            self.preselect_removed = removed;
            self.sync_selection_flags_to_objects();
        }

        fn set_selection_ids_gestured(&mut self, ids: &[String], gesture: Option<&str>) {
            let next: BTreeSet<String> = ids.iter().cloned().collect();
            let mut sorted: Vec<_> = next.iter().cloned().collect();
            sorted.sort();
            let gesture_owned = gesture.map(std::borrow::ToOwned::to_owned);
            let sig = (sorted.clone(), gesture_owned.clone());
            if next == self.selection && self.last_select_emit_sig.as_ref() == Some(&sig) {
                return;
            }
            self.last_select_emit_sig = Some(sig);
            self.preselect.clear();
            self.preselect_removed.clear();
            self.last_preselect_emit_sig = None;
            if next != self.selection {
                self.selection_exit_highlight.clear();
                self.selection = next;
                self.sync_selection_flags_to_objects();
            }
            let mut payload = json!({ "ids": sorted, "exitHighlightIds": [] });
            if let Some(ref g) = gesture_owned {
                payload["gestureMergeMode"] = json!(g);
            }
            self.push_event("select", payload);
        }

        /// @emoji 👁️ Rectangle/lasso drag preview: `preselect` + `preselect_removed` (anchor \\ preselect); emits `preselect` only.
        fn apply_area_preselect(&mut self, anchor_ids: &BTreeSet<String>, ids: &[String], gesture: Option<&str>) {
            let next: BTreeSet<String> = ids.iter().cloned().collect();
            let sorted = Self::sorted_selection_ids(&next);
            let removed = Self::sorted_selection_ids(&anchor_ids.difference(&next).cloned().collect());
            let gesture_owned = gesture.map(std::borrow::ToOwned::to_owned);
            let sig = (sorted.clone(), removed.clone(), gesture_owned.clone());
            if self.preselect == next && self.last_preselect_emit_sig.as_ref() == Some(&sig) {
                return;
            }
            self.last_preselect_emit_sig = Some(sig);
            self.preselect = next;
            self.preselect_removed = anchor_ids.difference(&self.preselect).cloned().collect();
            self.set_hovered_id_silent(None);
            self.sync_selection_flags_to_objects();
            let mut payload = json!({ "ids": sorted, "removedIds": removed });
            if let Some(ref g) = gesture_owned {
                payload["gestureMergeMode"] = json!(g);
            }
            self.push_event("preselect", payload);
        }

        fn sorted_selection_ids(set: &BTreeSet<String>) -> Vec<String> {
            let mut v: Vec<_> = set.iter().cloned().collect();
            v.sort();
            v
        }

        /// @emoji 🧿 Ends a rectangle/lasso cycle: commits `selection`, clears preselect (highlight only lives in preselect).
        fn commit_area_select_from_initial(&mut self, initial_ids: &BTreeSet<String>, ids: &[String], gesture: Option<&str>) {
            let next: BTreeSet<String> = ids.iter().cloned().collect();
            let sorted = Self::sorted_selection_ids(&next);
            let anchor = Self::sorted_selection_ids(initial_ids);
            let gesture_owned = gesture.map(std::borrow::ToOwned::to_owned);
            self.last_select_emit_sig = None;
            self.last_preselect_emit_sig = None;
            self.preselect.clear();
            self.preselect_removed.clear();
            self.selection_exit_highlight.clear();
            self.selection = next;
            self.sync_selection_flags_to_objects();
            let mut payload = json!({ "ids": sorted, "anchorIds": anchor, "exitHighlightIds": [] });
            if let Some(ref g) = gesture_owned {
                payload["gestureMergeMode"] = json!(g);
            }
            self.push_event("select", payload);
        }

        /// @emoji 🧿 True during left‑button rectangle/lasso drag so callers can avoid descriptor round‑trips that fight the live marquee state.
        pub fn is_dragging_area_select(&self) -> bool {
            matches!(&self.interaction, Interaction::Selection { .. })
        }

        /// @emoji 🧿 True during area select, link gestures, node drag, or camera pan so JS can defer full `syncDescriptorJson` round-trips.
        pub fn defers_descriptor_sync_from_js(&self) -> bool {
            matches!(
                self.interaction,
                Interaction::LinkAtSourceHandle { .. }
                    | Interaction::LinkDragSnap { .. }
                    | Interaction::LinkTargetNode { .. }
                    | Interaction::ExternalLinkPreview { .. }
                    | Interaction::DragNodes { .. }
                    | Interaction::Pan { .. }
            )
        }

        pub fn world_to_screen(&self, p: Point) -> Point {
            crate::cavas::camera::world_to_screen(&self.camera, &self.viewport(), p)
        }

        pub fn screen_to_world(&self, p: Point) -> Point {
            crate::cavas::camera::screen_to_world(&self.camera, &self.viewport(), p)
        }

        fn node_kind_scale(&self, node_kind: &str) -> f64 {
            self.node_kinds.get(node_kind).map(|k| k.scale).unwrap_or(1.0)
        }

        fn handle_kind_scale(&self, handle_kind: &str) -> f64 {
            self.handle_kinds.get(handle_kind).map(|k| k.scale).unwrap_or(1.0)
        }

        fn effective_node_scale(&self, n: &NodeData) -> f64 {
            (n.scale * self.node_kind_scale(n.node_kind.as_str())).max(1e-9)
        }

        fn scaled_node_radius(&self, n: &NodeData) -> f64 {
            n.radius * self.effective_node_scale(n)
        }

        fn scaled_node_width(&self, n: &NodeData) -> f64 {
            n.width * self.effective_node_scale(n)
        }

        fn scaled_node_height(&self, n: &NodeData) -> f64 {
            n.height * self.effective_node_scale(n)
        }

        fn effective_handle_scale(&self, h: &HandleData) -> f64 {
            let node_scale = self.nodes.get(h.node_id.as_str()).map(|n| self.effective_node_scale(n)).unwrap_or(1.0);
            (node_scale * h.scale * self.handle_kind_scale(h.handle_kind.as_str())).max(1e-9)
        }

        pub(crate) fn effective_handle_radius(&self, h: &HandleData) -> f64 {
            h.radius * self.effective_handle_scale(h)
        }

        pub(crate) fn handle_world_pos(&self, h: &HandleData) -> Option<Point> {
            let n = self.nodes.get(&h.node_id)?;
            Some(match n.shape {
                NodeShape::Circle => handle_position_on_circle(Point::new(n.x, n.y), self.scaled_node_radius(n), h.angle),
                NodeShape::Rectangle => handle_position_on_rectangle(Point::new(n.x, n.y), self.scaled_node_width(n), self.scaled_node_height(n), h.angle),
            })
        }

        /// @emoji 📐 Node half-extent for indirect ring layout: circle radius or half the shorter rectangle side.
        fn indirect_node_half_extent(&self, n: &NodeData) -> f64 {
            match n.shape {
                NodeShape::Circle => self.scaled_node_radius(n),
                NodeShape::Rectangle => self.scaled_node_width(n).min(self.scaled_node_height(n)) * 0.5,
            }
        }

        /// @emoji 📐 Radial world offset from node rim to indirect-handle center (`INDIRECT_HANDLE_RING_GAP_NODE_SCALE`× half-extent) so ring–node proportions stay fixed when zooming.
        fn indirect_handle_ring_offset_world(&self, n: &NodeData) -> f64 {
            (self.indirect_node_half_extent(n) * INDIRECT_HANDLE_RING_GAP_NODE_SCALE).max(1e-9)
        }

        /// @emoji 📐 Ghost link handles sit on a rim offset by `INDIRECT_HANDLE_RING_GAP_NODE_SCALE`× node half-extent from the node body so ring spacing scales with the node at every zoom.
        pub(crate) fn indirect_handle_world_pos(&self, h: &HandleData) -> Option<Point> {
            let n = self.nodes.get(&h.node_id)?;
            let offset = self.indirect_handle_ring_offset_world(n);
            Some(match n.shape {
                NodeShape::Circle => handle_position_on_circle(Point::new(n.x, n.y), self.scaled_node_radius(n) + offset, h.angle),
                NodeShape::Rectangle => handle_position_on_rectangle(Point::new(n.x, n.y), self.scaled_node_width(n) + 2.0 * offset, self.scaled_node_height(n) + 2.0 * offset, h.angle),
            })
        }

        /// @emoji 📐 Indirect-connect marker radius in world units: `INDIRECT_HANDLE_MARKER_NODE_SCALE`× circle radius or × half the shorter rectangle side.
        pub(crate) fn indirect_handle_marker_radius_world(&self, h: &HandleData) -> f64 {
            let Some(n) = self.nodes.get(&h.node_id) else {
                return (self.effective_handle_radius(h) * INDIRECT_HANDLE_MARKER_NODE_SCALE).max(1e-9);
            };
            let handle_local_scale = (h.scale * self.handle_kind_scale(h.handle_kind.as_str())).max(1e-9);
            (self.indirect_node_half_extent(n) * INDIRECT_HANDLE_MARKER_NODE_SCALE * handle_local_scale).max(1e-9)
        }

        /// @emoji 🧭 Source handle id while a link wire is drawn (`LinkDragSnap` / `LinkTargetNode`).
        fn active_link_source_handle_id(&self) -> Option<&str> {
            match &self.interaction {
                Interaction::LinkDragSnap { source_id, .. } | Interaction::LinkTargetNode { source_id, .. } | Interaction::ExternalLinkPreview { source_id, .. } => Some(source_id.as_str()),
                _ => None,
            }
        }

        /// @emoji 🧭 Visible target node ids that expose at least one free handle compatible with `source_handle_id`.
        fn link_drag_compatible_target_node_ids(&self, source_handle_id: &str) -> Vec<String> {
            let Some(source) = self.handles.get(source_handle_id) else {
                return Vec::new();
            };
            let source_node_id = source.node_id.as_str();
            let mut out = Vec::new();
            let mut seen = std::collections::BTreeSet::new();
            for (hid, h) in &self.handles {
                if h.node_id == source_node_id || !self.handle_effectively_visible(hid.as_str()) {
                    continue;
                }
                if self.handle_has_incident_edge(hid.as_str()) {
                    continue;
                }
                if !self.handles_link_compatible_for_drag(source, h) {
                    continue;
                }
                if !self.nodes.get(&h.node_id).is_some_and(|n| n.visible) {
                    continue;
                }
                if seen.insert(h.node_id.clone()) {
                    out.push(h.node_id.clone());
                }
            }
            out.sort();
            out
        }

        /// @emoji 🧭 Count of visible free handles on `node_id` compatible with `source_handle_id`.
        fn link_compatible_handle_count_on_node(&self, source_handle_id: &str, node_id: &str) -> usize {
            let Some(source) = self.handles.get(source_handle_id) else {
                return 0;
            };
            if source.node_id == node_id {
                return 0;
            }
            self.handles.iter().filter(|(id, h)| h.node_id == node_id && self.handle_eligible_link_target_ring(id.as_str(), source_handle_id) && self.handles_link_compatible_for_drag(source, h)).count()
        }

        /// @emoji 🧭 Free compatible handle ids on `node_id` for an active link from `source_handle_id`.
        fn link_compatible_handle_ids_on_node(&self, source_handle_id: &str, node_id: &str) -> Vec<String> {
            let Some(source) = self.handles.get(source_handle_id) else {
                return Vec::new();
            };
            let mut out: Vec<String> = self
                .handles
                .iter()
                .filter_map(|(id, h)| {
                    if h.node_id != node_id {
                        return None;
                    }
                    if !self.handle_eligible_link_target_ring(id.as_str(), source_handle_id) {
                        return None;
                    }
                    self.handles_link_compatible_for_drag(source, h).then(|| id.clone())
                })
                .collect();
            out.sort();
            out
        }

        /// @emoji 🧭 Compatible target node under `world` while a link wire is active (node body hit).
        fn link_drag_ring_target_node_id(&self, source_handle_id: &str, world: Point) -> Option<String> {
            let nid = self.resolve_node_hit_world(world)?;
            if self.handles.get(source_handle_id)?.node_id == nid {
                return None;
            }
            self.node_has_any_free_link_compatible_handle(source_handle_id, nid.as_str()).then_some(nid)
        }

        /// @emoji 🧭 Resolves which single node draws the overview/normal indirect handle ring when that node has **more than one** eligible free handles (otherwise the sole handle is implicit).
        fn indirect_ring_node_id(&self, lod: BoardDrawLod) -> Option<String> {
            if !matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                return None;
            }
            if let Interaction::LinkTargetNode { source_id, target_node_id } = &self.interaction {
                if self.link_compatible_handle_count_on_node(source_id, target_node_id) > 1 {
                    return self.nodes.get(target_node_id).filter(|n| n.visible).map(|n| n.id.clone());
                }
                return None;
            }
            if let Interaction::ExternalLinkPreview { ring_node_id: Some(target_node_id), ring_handle_ids, .. } = &self.interaction {
                if ring_handle_ids.len() > 1 {
                    return self.nodes.get(target_node_id).filter(|n| n.visible).map(|n| n.id.clone());
                }
                return None;
            }
            if let Interaction::LinkDragSnap { source_id, end_world, .. } = &self.interaction {
                let ring_nid = self.link_drag_ring_target_node_id(source_id, *end_world)?;
                if self.link_compatible_handle_count_on_node(source_id, ring_nid.as_str()) > 1 {
                    return Some(ring_nid);
                }
                return None;
            }
            if self.active_link_source_handle_id().is_some() {
                return None;
            }
            let ring_nid = if self.selection.len() == 1 {
                self.selection.iter().next()?.clone()
            } else {
                return None;
            };
            let n = self.nodes.get(&ring_nid).filter(|n| n.visible)?;
            if self.eligible_indirect_handle_count_on_node(n.id.as_str()) > 1 {
                Some(ring_nid)
            } else {
                None
            }
        }

        fn eligible_indirect_handle_count_on_node(&self, node_id: &str) -> usize {
            self.handles.iter().filter(|(id, h)| h.node_id == node_id && self.handle_effectively_visible(id.as_str()) && self.handle_eligible_indirect_connect_ring(id.as_str())).count()
        }

        /// @emoji 🧭 Returns the handle id when `node_id` has exactly one visible free indirect-eligible handle.
        fn sole_eligible_indirect_handle_on_node(&self, node_id: &str) -> Option<String> {
            let mut found: Option<String> = None;
            for (id, h) in &self.handles {
                if h.node_id != node_id || !self.handle_effectively_visible(id.as_str()) || !self.handle_eligible_indirect_connect_ring(id.as_str()) {
                    continue;
                }
                if found.is_some() {
                    return None;
                }
                found = Some(id.clone());
            }
            found
        }

        /// @emoji 🧭 When the drop target has exactly one free handle compatible with `source_handle_id`, returns that handle id (otherwise `None`).
        fn node_sole_free_link_compatible_handle(&self, source_handle_id: &str, target_node_id: &str) -> Option<String> {
            let source = self.handles.get(source_handle_id)?;
            if source.node_id == target_node_id {
                return None;
            }
            let mut found: Option<String> = None;
            for (id, h) in &self.handles {
                if h.node_id != target_node_id || !self.handle_effectively_visible(id.as_str()) {
                    continue;
                }
                if self.handle_has_incident_edge(id.as_str()) {
                    continue;
                }
                if !self.handles_link_compatible_for_drag(source, h) {
                    continue;
                }
                if found.is_some() {
                    return None;
                }
                found = Some(id.clone());
            }
            found
        }

        fn point_in_node_world(&self, n: &NodeData, point: Point) -> bool {
            match n.shape {
                NodeShape::Rectangle => {
                    let hw = self.scaled_node_width(n) / 2.0;
                    let hh = self.scaled_node_height(n) / 2.0;
                    (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh
                }
                NodeShape::Circle => distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n),
            }
        }

        fn sole_indirect_handle_hit_link_target(&self, point: Point) -> Option<String> {
            let Interaction::LinkTargetNode { source_id, target_node_id } = &self.interaction else {
                return None;
            };
            let th = self.node_sole_free_link_compatible_handle(source_id, target_node_id)?;
            let n = self.nodes.get(target_node_id)?;
            if !n.visible {
                return None;
            }
            if !self.point_in_node_world(n, point) {
                return None;
            }
            Some(th)
        }

        fn sole_indirect_handle_hit_idle_selected_node(&self, point: Point) -> Option<String> {
            if !matches!(self.interaction, Interaction::None) {
                return None;
            }
            if self.selection.len() != 1 {
                return None;
            }
            let nid = self.selection.iter().next()?;
            if !self.nodes.contains_key(nid) {
                return None;
            }
            let sole = self.sole_eligible_indirect_handle_on_node(nid)?;
            let n = self.nodes.get(nid)?;
            if !n.visible {
                return None;
            }
            if !self.point_in_node_world(n, point) {
                return None;
            }
            Some(sole)
        }

        /// @emoji 🧭 True when `target_node_id` hosts at least one visible free handle that can pair with `source_handle_id` under link-compat rules.
        fn node_has_any_free_link_compatible_handle(&self, source_handle_id: &str, target_node_id: &str) -> bool {
            let Some(source) = self.handles.get(source_handle_id) else {
                return false;
            };
            if source.node_id == target_node_id {
                return false;
            }
            for (hid, h) in &self.handles {
                if h.node_id != target_node_id || !self.handle_effectively_visible(hid.as_str()) {
                    continue;
                }
                if self.handle_has_incident_edge(hid.as_str()) {
                    continue;
                }
                if self.handles_link_compatible_for_drag(source, h) {
                    return true;
                }
            }
            false
        }

        /// @emoji 💫 True when the handle may appear on a link-target ghost ring (`overview`/`normal` LOD).
        fn handle_eligible_link_target_ring(&self, handle_id: &str, source_handle_id: &str) -> bool {
            if !self.handle_effectively_visible(handle_id) || self.handle_has_incident_edge(handle_id) {
                return false;
            }
            let Some(source) = self.handles.get(source_handle_id) else {
                return false;
            };
            let Some(target) = self.handles.get(handle_id) else {
                return false;
            };
            if source.node_id == target.node_id {
                return false;
            }
            self.handles_link_compatible_for_drag(source, target)
        }

        fn indirect_ring_handle_eligible(&self, handle_id: &str, ring_node_id: &str) -> bool {
            if self.handles.get(handle_id).is_none_or(|h| h.node_id != ring_node_id) {
                return false;
            }
            if let Some(source_id) = self.active_link_source_handle_id() {
                self.handle_eligible_link_target_ring(handle_id, source_id)
            } else {
                self.handle_eligible_indirect_connect_ring(handle_id)
            }
        }

        fn link_drag_target_ring_hit(&self, source_id: &str, point: Point) -> Option<String> {
            if !matches!(self.current_draw_lod(), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                return None;
            }
            if let Interaction::ExternalLinkPreview { source_id: active_source, ring_handle_ids, .. } = &self.interaction {
                if active_source != source_id {
                    return None;
                }
                let zoom = self.camera.zoom;
                for hid in ring_handle_ids {
                    let Some(h) = self.handles.get(hid) else {
                        continue;
                    };
                    if !self.handle_eligible_link_target_ring(h.id.as_str(), source_id) {
                        continue;
                    }
                    let Some(pos) = self.indirect_handle_world_pos(h) else {
                        continue;
                    };
                    let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
                    if distance_between(point, pos) <= tol {
                        return Some(h.id.clone());
                    }
                }
                return None;
            }
            let node_id = self.link_drag_ring_target_node_id(source_id, point)?;
            if self.link_compatible_handle_count_on_node(source_id, node_id.as_str()) <= 1 {
                return None;
            }
            let zoom = self.camera.zoom;
            for h in self.handles.values().rev() {
                if h.node_id != node_id || !self.handle_eligible_link_target_ring(h.id.as_str(), source_id) {
                    continue;
                }
                let Some(pos) = self.indirect_handle_world_pos(h) else { continue };
                let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
                if distance_between(point, pos) <= tol {
                    return Some(h.id.clone());
                }
            }
            None
        }

        fn link_target_ring_snapshot(&self, source_handle_id: &str) -> (Option<String>, Vec<String>) {
            if let Interaction::ExternalLinkPreview { source_id, ring_node_id, ring_handle_ids, .. } = &self.interaction {
                if source_id == source_handle_id {
                    return (ring_node_id.clone(), ring_handle_ids.clone());
                }
            }
            let node_id = match &self.interaction {
                Interaction::LinkTargetNode { target_node_id, .. } => Some(target_node_id.clone()),
                Interaction::LinkDragSnap { end_world, .. } => self.link_drag_ring_target_node_id(source_handle_id, *end_world),
                _ => None,
            };
            let Some(nid) = node_id else {
                return (None, Vec::new());
            };
            if self.link_compatible_handle_count_on_node(source_handle_id, nid.as_str()) <= 1 {
                return (None, Vec::new());
            }
            (Some(nid.clone()), self.link_compatible_handle_ids_on_node(source_handle_id, nid.as_str()))
        }

        fn sync_link_gesture_events(&mut self) {
            if let Interaction::ExternalLinkPreview { source_id, compatible_node_ids, ring_node_id, ring_handle_ids, .. } = self.interaction.clone() {
                let compat_key = format!("{}|{}", source_id, compatible_node_ids.join(","));
                if self.link_compat_nodes_emit_key.as_deref() != Some(compat_key.as_str()) {
                    self.link_compat_nodes_emit_key = Some(compat_key);
                    self.push_event("linkCompatibleNodes", json!({ "source": source_id, "nodeIds": compatible_node_ids }));
                }
                let ring_key = format!("{}|{}|{}", source_id, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(","));
                if self.link_target_ring_emit_key.as_deref() != Some(ring_key.as_str()) {
                    self.link_target_ring_emit_key = Some(ring_key);
                    self.push_event(
                        "linkTargetRing",
                        json!({
                            "source": source_id,
                            "nodeId": ring_node_id,
                            "handleIds": ring_handle_ids,
                        }),
                    );
                }
                return;
            }
            let Some(source) = self.active_link_source_handle_id().map(str::to_string) else {
                self.clear_link_gesture_events();
                return;
            };
            let node_ids = self.link_drag_compatible_target_node_ids(&source);
            let compat_key = format!("{}|{}", source, node_ids.join(","));
            if self.link_compat_nodes_emit_key.as_deref() != Some(compat_key.as_str()) {
                self.link_compat_nodes_emit_key = Some(compat_key);
                self.push_event("linkCompatibleNodes", json!({ "source": source, "nodeIds": node_ids }));
            }
            let (ring_node_id, ring_handle_ids) = self.link_target_ring_snapshot(&source);
            let ring_key = format!("{}|{}|{}", source, ring_node_id.as_deref().unwrap_or(""), ring_handle_ids.join(","));
            if self.link_target_ring_emit_key.as_deref() != Some(ring_key.as_str()) {
                self.link_target_ring_emit_key = Some(ring_key);
                self.push_event(
                    "linkTargetRing",
                    json!({
                        "source": source,
                        "nodeId": ring_node_id,
                        "handleIds": ring_handle_ids,
                    }),
                );
            }
        }

        fn clear_link_gesture_events(&mut self) {
            if self.link_compat_nodes_emit_key.take().is_some() {
                self.push_event("linkCompatibleNodes", json!({ "source": "", "nodeIds": [] }));
            }
            if self.link_target_ring_emit_key.take().is_some() {
                self.push_event("linkTargetRing", json!({ "source": "", "nodeId": null, "handleIds": [] }));
            }
        }

        fn node_center_world(&self, node_id: &str) -> Option<Point> {
            let n = self.nodes.get(node_id)?;
            Some(Point::new(n.x, n.y))
        }

        fn edge_curve(&self, e: &EdgeData) -> Option<CubicBez> {
            if !self.has_ports() {
                let source_node = self.nodes.get(&e.source)?;
                let target_node = self.nodes.get(&e.target)?;
                let source_center = Point::new(source_node.x, source_node.y);
                let target_center = Point::new(target_node.x, target_node.y);
                let source_pos = self.node_rim_point_toward(source_node, target_center)?;
                let target_pos = self.node_rim_point_toward(target_node, source_center)?;
                return Some(compute_edge_bezier_points(source_pos, target_pos, source_center, target_center));
            }
            let source_handle = self.handles.get(&e.source)?;
            let target_handle = self.handles.get(&e.target)?;
            let source_node = self.nodes.get(&source_handle.node_id)?;
            let target_node = self.nodes.get(&target_handle.node_id)?;
            let source_pos = self.handle_world_pos(source_handle)?;
            let target_pos = self.handle_world_pos(target_handle)?;
            Some(compute_edge_bezier_points(source_pos, target_pos, Point::new(source_node.x, source_node.y), Point::new(target_node.x, target_node.y)))
        }

        fn link_drag_wire_curve_world(&self, source_id: &str, target_id: Option<&str>, end_world: Point) -> Option<CubicBez> {
            let source_handle = self.handles.get(source_id)?;
            let source_node = self.nodes.get(&source_handle.node_id)?;
            let source_pos = self.handle_world_pos(source_handle)?;
            let source_center = Point::new(source_node.x, source_node.y);
            let (target_pos, target_center) = if let Some(tid) = target_id {
                let th = self.handles.get(tid)?;
                let tn = self.nodes.get(&th.node_id)?;
                (self.handle_world_pos(th)?, Point::new(tn.x, tn.y))
            } else {
                (end_world, end_world)
            };
            Some(compute_edge_bezier_points(source_pos, target_pos, source_center, target_center))
        }

        fn active_link_wire_curve(&self) -> Option<CubicBez> {
            match &self.interaction {
                Interaction::LinkDragSnap { source_id, target_id, end_world } => self.link_drag_wire_curve_world(source_id.as_str(), target_id.as_deref(), *end_world),
                Interaction::LinkTargetNode { source_id, target_node_id } => self.link_drag_wire_curve_world(source_id.as_str(), None, self.node_center_world(target_node_id)?),
                Interaction::ExternalLinkPreview { source_id, end_world, .. } => self.link_drag_wire_curve_world(source_id.as_str(), None, *end_world),
                Interaction::DragNodes { proximity_pair: Some((src, tgt)), .. } => self.link_drag_wire_curve_world(src.as_str(), Some(tgt.as_str()), Point::ZERO),
                _ => None,
            }
        }

        fn wire_curve(&self, w: &WireData) -> Option<CubicBez> {
            let end_world = match (&w.target, w.end_x, w.end_y) {
                (None, Some(x), Some(y)) if x.is_finite() && y.is_finite() => Point::new(x, y),
                (Some(tid), _, _) => {
                    self.handles.get(tid)?;
                    return self.edge_curve(&EdgeData {
                        id: w.id.clone(),
                        source: w.source.clone(),
                        target: tid.clone(),
                        selected: w.selected,
                        visible: w.visible,
                        style: w.style.clone(),
                        edge_kind: String::new(),
                        source_tip: None,
                        target_tip: None,
                    });
                }
                _ => return None,
            };
            self.link_drag_wire_curve_world(w.source.as_str(), None, end_world)
        }

        fn apply_link_drag_snap_hover(&mut self, _source_handle_id: &str, world: Point, target_handle_id: Option<&str>) {
            if let Some(tid) = target_handle_id {
                self.set_hovered_id(Some(tid.to_string()));
            } else {
                self.update_hover_from_world(world);
            }
        }

        /// @emoji 🧭 Minimap/overview LOD: group selection and bounded drag only — no per-node/edge/handle picks.
        fn lod_disables_discrete_pick(&self) -> bool {
            matches!(self.current_draw_lod(), BoardDrawLod::Minimap | BoardDrawLod::Overview)
        }

        /// @emoji 🔗 Overview LOD: tight world-radius hit on a free handle so link drag can start without enabling broad `resolve_hit_world` handle picks.
        fn resolve_overview_free_link_handle_pointer_world(&self, point: Point) -> Option<String> {
            if !matches!(self.current_draw_lod(), BoardDrawLod::Overview) {
                return None;
            }
            if !self.selection_options.select_handles {
                return None;
            }
            const MAX_D_WORLD: f64 = 2.25;
            let mut best: Option<(f64, String)> = None;
            for h in self.handles.values() {
                if !self.handle_effectively_visible(h.id.as_str()) || self.handle_has_incident_edge(h.id.as_str()) {
                    continue;
                }
                let Some(pos) = self.handle_world_pos(h) else {
                    continue;
                };
                let d = distance_between(point, pos);
                if d <= MAX_D_WORLD && best.as_ref().map(|(bd, _)| d < *bd).unwrap_or(true) {
                    best = Some((d, h.id.clone()));
                }
            }
            best.map(|(_, id)| id)
        }

        /// @emoji 🧭 Minimap/overview LOD: pointer-down inside the selection AABB moves the group without a discrete hit.
        fn lod_uses_bounded_drag(&self) -> bool {
            matches!(self.current_draw_lod(), BoardDrawLod::Minimap | BoardDrawLod::Overview)
        }

        fn resolve_hover_world(&self, point: Point) -> Option<String> {
            let lod = self.current_draw_lod();
            let zoom = self.camera.zoom;
            if self.has_ports() && !matches!(lod, BoardDrawLod::Minimap) {
                if matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                    if let Some(hid) = self.sole_indirect_handle_hit_link_target(point) {
                        return Some(hid);
                    }
                    if let Interaction::LinkDragSnap { source_id, .. } = &self.interaction {
                        if let Some(hid) = self.link_drag_target_ring_hit(source_id, point) {
                            return Some(hid);
                        }
                    }
                }
                if let Some(ring_node_id) = self.indirect_ring_node_id(lod) {
                    for h in self.handles.values().rev() {
                        if h.node_id != ring_node_id || !self.handle_effectively_visible(h.id.as_str()) {
                            continue;
                        }
                        if !self.indirect_ring_handle_eligible(h.id.as_str(), ring_node_id.as_str()) {
                            continue;
                        }
                        let Some(pos) = self.indirect_handle_world_pos(h) else { continue };
                        let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
                        if distance_between(point, pos) <= tol {
                            return Some(h.id.clone());
                        }
                    }
                }
                if matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro) {
                    for h in self.handles.values().rev() {
                        if !self.handle_effectively_visible(h.id.as_str()) {
                            continue;
                        }
                        let Some(pos) = self.handle_world_pos(h) else { continue };
                        let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.effective_handle_radius(h);
                        if distance_between(point, pos) <= tol {
                            return Some(h.id.clone());
                        }
                    }
                }
                if matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                    if let Some(hid) = self.sole_indirect_handle_hit_idle_selected_node(point) {
                        return Some(hid);
                    }
                }
            }
            for n in self.nodes.values().rev() {
                if !n.visible {
                    continue;
                }
                match n.shape {
                    NodeShape::Rectangle => {
                        let hw = self.scaled_node_width(n) / 2.0;
                        let hh = self.scaled_node_height(n) / 2.0;
                        if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
                            return Some(n.id.clone());
                        }
                    }
                    NodeShape::Circle => {
                        if distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n) {
                            return Some(n.id.clone());
                        }
                    }
                }
            }
            for w in self.wires.values().rev() {
                if !self.wire_effectively_visible(w) {
                    continue;
                }
                if let Some(c) = self.wire_curve(w) {
                    if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                        return Some(w.id.clone());
                    }
                }
            }
            for e in self.edges.values().rev() {
                if !self.edge_effectively_visible(e) {
                    continue;
                }
                if let Some(c) = self.edge_curve(e) {
                    if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                        return Some(e.id.clone());
                    }
                }
            }
            None
        }

        pub fn resolve_hit_world(&self, point: Point) -> Option<String> {
            if self.lod_disables_discrete_pick() {
                return None;
            }
            let zoom = self.camera.zoom;
            let o = &self.selection_options;
            if self.has_ports() && o.select_handles {
                if matches!(self.current_draw_lod(), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                    if let Some(hid) = self.sole_indirect_handle_hit_link_target(point) {
                        return Some(hid);
                    }
                    if let Interaction::LinkDragSnap { source_id, .. } = &self.interaction {
                        if let Some(hid) = self.link_drag_target_ring_hit(source_id, point) {
                            return Some(hid);
                        }
                    }
                }
                if let Some(ring_node_id) = self.indirect_ring_node_id(self.current_draw_lod()) {
                    for h in self.handles.values().rev() {
                        if h.node_id != ring_node_id || !self.handle_effectively_visible(h.id.as_str()) {
                            continue;
                        }
                        if !self.indirect_ring_handle_eligible(h.id.as_str(), ring_node_id.as_str()) {
                            continue;
                        }
                        let Some(pos) = self.indirect_handle_world_pos(h) else { continue };
                        let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.indirect_handle_marker_radius_world(h);
                        if distance_between(point, pos) <= tol {
                            return Some(h.id.clone());
                        }
                    }
                }
                if matches!(self.current_draw_lod(), BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro) {
                    for h in self.handles.values().rev() {
                        if !self.handle_effectively_visible(h.id.as_str()) {
                            continue;
                        }
                        let Some(pos) = self.handle_world_pos(h) else { continue };
                        let tol = (HANDLE_HIT_TOLERANCE_PX / zoom) + self.effective_handle_radius(h);
                        if distance_between(point, pos) <= tol {
                            return Some(h.id.clone());
                        }
                    }
                }
                if matches!(self.current_draw_lod(), BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Normal) {
                    if let Some(hid) = self.sole_indirect_handle_hit_idle_selected_node(point) {
                        return Some(hid);
                    }
                }
            }
            if o.select_nodes {
                for n in self.nodes.values().rev() {
                    if !n.visible {
                        continue;
                    }
                    match n.shape {
                        NodeShape::Rectangle => {
                            let hw = self.scaled_node_width(n) / 2.0;
                            let hh = self.scaled_node_height(n) / 2.0;
                            if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
                                return Some(n.id.clone());
                            }
                        }
                        NodeShape::Circle => {
                            if distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n) {
                                return Some(n.id.clone());
                            }
                        }
                    }
                }
            }
            if o.select_edges {
                for e in self.edges.values().rev() {
                    if !self.edge_effectively_visible(e) {
                        continue;
                    }
                    if let Some(c) = self.edge_curve(e) {
                        if distance_point_to_cubic_bezier(point, c, 18) <= EDGE_HIT_TOLERANCE_PX / zoom {
                            return Some(e.id.clone());
                        }
                    }
                }
            }
            None
        }

        fn resolve_node_hit_world(&self, point: Point) -> Option<String> {
            for n in self.nodes.values().rev() {
                if !n.visible {
                    continue;
                }
                match n.shape {
                    NodeShape::Rectangle => {
                        let hw = self.scaled_node_width(n) / 2.0;
                        let hh = self.scaled_node_height(n) / 2.0;
                        if (point.x - n.x).abs() <= hw && (point.y - n.y).abs() <= hh {
                            return Some(n.id.clone());
                        }
                    }
                    NodeShape::Circle => {
                        if distance_between(point, Point::new(n.x, n.y)) <= self.scaled_node_radius(n) {
                            return Some(n.id.clone());
                        }
                    }
                }
            }
            None
        }

        fn merge_pick_into_selection(initial: &BTreeSet<String>, hit_id: &str, mode: &str) -> BTreeSet<String> {
            let mut next = initial.clone();
            match mode {
                "additive" => {
                    next.insert(hit_id.to_string());
                }
                "subtractive" => {
                    next.remove(hit_id);
                }
                "replace" => {
                    next.clear();
                    next.insert(hit_id.to_string());
                }
                _ => {
                    if next.contains(hit_id) {
                        next.remove(hit_id);
                    } else {
                        next.insert(hit_id.to_string());
                    }
                }
            }
            next
        }

        fn pick_merge_mode_for_modifiers(ctrl_or_meta: bool, shift: bool, option_mode: &str) -> String {
            if ctrl_or_meta && shift {
                return "invertive".into();
            }
            if ctrl_or_meta {
                return "subtractive".into();
            }
            if shift {
                return "additive".into();
            }
            option_mode.to_string()
        }

        pub fn sync_descriptor(&mut self, desc: &SceneDescriptorJson) -> Result<(), String> {
            if matches!(self.interaction, Interaction::LinkAtSourceHandle { .. } | Interaction::LinkDragSnap { .. } | Interaction::LinkTargetNode { .. } | Interaction::ExternalLinkPreview { .. }) {
                self.interaction = Interaction::None;
                self.clear_link_gesture_events();
            }
            let want_nodes: BTreeSet<_> = desc.nodes.iter().map(|n| n.id.clone()).collect();
            let want_handles: BTreeSet<_> = desc.handles.iter().map(|h| h.id.clone()).collect();
            let want_edges: BTreeSet<_> = desc.edges.iter().map(|e| e.id.clone()).collect();
            let want_wires: BTreeSet<_> = desc.wires.iter().map(|w| w.id.clone()).collect();
            self.edges.retain(|id, _| want_edges.contains(id));
            self.wires.retain(|id, _| want_wires.contains(id));
            self.handles.retain(|id, _| want_handles.contains(id));
            self.nodes.retain(|id, _| want_nodes.contains(id));
            for n in &desc.nodes {
                let shape = if n.shape.as_deref() == Some("rectangle") { NodeShape::Rectangle } else { NodeShape::Circle };
                let (radius, width, height) = match shape {
                    NodeShape::Circle => (n.radius.unwrap_or(0.0), 0.0, 0.0),
                    NodeShape::Rectangle => (0.0, n.width.unwrap_or(0.0), n.height.unwrap_or(0.0)),
                };
                let node_kind = n.node_kind.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default();
                self.nodes.insert(
                    n.id.clone(),
                    NodeData {
                        id: n.id.clone(),
                        x: n.x,
                        y: n.y,
                        shape,
                        radius,
                        width,
                        height,
                        scale: n.scale.filter(|v| v.is_finite() && *v > 0.0).unwrap_or(1.0),
                        draggable: n.draggable.unwrap_or(true),
                        selected: n.selected.unwrap_or(false),
                        visible: n.visible.unwrap_or(true),
                        root: n.root.unwrap_or(false),
                        style: n.style.clone(),
                        text: n.text.clone(),
                        icon_kind: n.icon_kind.clone(),
                        node_kind,
                    },
                );
            }
            for h in &desc.handles {
                let kind = h.handle_kind.as_deref().unwrap_or("").trim().to_string();
                let color_fill = match h.color.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
                    None => None,
                    Some(s) => Some(Self::parse_css_color(s).ok_or_else(|| format!("invalid color on handle {}: {s:?}", h.id))?),
                };
                let icon_kind = h.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.to_string());
                self.handles.insert(
                    h.id.clone(),
                    HandleData {
                        id: h.id.clone(),
                        node_id: h.node_id.clone(),
                        angle: h.angle,
                        radius: h.radius.unwrap_or(8.0),
                        scale: h.scale.filter(|v| v.is_finite() && *v > 0.0).unwrap_or(1.0),
                        selected: h.selected.unwrap_or(false),
                        visible: h.visible.unwrap_or(true),
                        style: h.style.clone(),
                        handle_kind: kind,
                        color_fill,
                        icon_kind,
                    },
                );
            }
            for e in &desc.edges {
                let existed = self.edges.contains_key(&e.id);
                let edge_kind = e.edge_kind.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_default();
                let source_tip = Self::parse_catalog_tip_slot(e.source_tip.as_deref());
                let target_tip = Self::parse_catalog_tip_slot(e.target_tip.as_deref());
                self.edges.insert(
                    e.id.clone(),
                    EdgeData {
                        id: e.id.clone(),
                        source: e.source.clone(),
                        target: e.target.clone(),
                        selected: e.selected.unwrap_or(false),
                        visible: e.visible.unwrap_or(true),
                        style: e.style.clone(),
                        edge_kind,
                        source_tip,
                        target_tip,
                    },
                );
                if !existed {
                    self.push_event("edgeCreate", json!({ "id": e.id, "source": e.source, "target": e.target }));
                }
            }
            for w in &desc.wires {
                let target = w.target.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
                let (end_x, end_y) = match &target {
                    Some(_) => (None, None),
                    None => {
                        let x = match w.end_x {
                            Some(v) if v.is_finite() => Some(v),
                            _ => None,
                        };
                        let y = match w.end_y {
                            Some(v) if v.is_finite() => Some(v),
                            _ => None,
                        };
                        if x.is_none() || y.is_none() {
                            continue;
                        }
                        (x, y)
                    }
                };
                let wire_kind = w
                    .wire_kind
                    .as_ref()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .or_else(|| self.handles.get(w.source.as_str()).map(|h| self.resolve_default_wire_kind_for_handle(h)))
                    .unwrap_or_else(|| DEFAULT_WIRE_KIND_ID.to_string());
                self.wires.insert(w.id.clone(), WireData { id: w.id.clone(), source: w.source.clone(), target, end_x, end_y, selected: w.selected.unwrap_or(false), visible: w.visible.unwrap_or(true), style: w.style.clone(), wire_kind });
            }
            if !self.is_preselect_active() {
                let mut new_selection = BTreeSet::new();
                for n in &desc.nodes {
                    if n.selected == Some(true) {
                        new_selection.insert(n.id.clone());
                    }
                }
                for h in &desc.handles {
                    if h.selected == Some(true) {
                        new_selection.insert(h.id.clone());
                    }
                }
                for e in &desc.edges {
                    if e.selected == Some(true) {
                        new_selection.insert(e.id.clone());
                    }
                }
                for w in &desc.wires {
                    if w.selected == Some(true) {
                        new_selection.insert(w.id.clone());
                    }
                }
                let prev_sel = self.selection.clone();
                if prev_sel != new_selection {
                    self.selection_exit_highlight.clear();
                }
                self.selection = new_selection;
                if prev_sel != self.selection {
                    self.push_select_event();
                }
            }
            self.sync_selection_flags_to_objects();
            self.bump_content_scene_generation();
            Ok(())
        }

        /// @emoji 📍 Applies peer-pane node drags without a full descriptor re-sync.
        pub fn set_node_positions(&mut self, moves: &[(String, f64, f64)]) {
            let mut geometry_changed = false;
            for (id, x, y) in moves {
                if !x.is_finite() || !y.is_finite() {
                    continue;
                }
                if let Some(node) = self.nodes.get_mut(id.as_str()) {
                    if (node.x - *x).abs() > 1e-9 || (node.y - *y).abs() > 1e-9 {
                        node.x = *x;
                        node.y = *y;
                        geometry_changed = true;
                    }
                }
            }
            if geometry_changed {
                self.bump_content_scene_generation();
            }
        }

        /// @emoji 📍 Parses `[{"id","x","y"},…]` and updates existing host nodes in place.
        pub fn set_node_positions_json(&mut self, json: &str) -> Result<(), String> {
            #[derive(Deserialize)]
            struct NodePositionMoveJson {
                id: String,
                x: f64,
                y: f64,
            }
            let rows: Vec<NodePositionMoveJson> = serde_json::from_str(json).map_err(|e| e.to_string())?;
            let moves: Vec<(String, f64, f64)> = rows.into_iter().map(|row| (row.id, row.x, row.y)).collect();
            self.set_node_positions(&moves);
            Ok(())
        }

        pub fn clear_scene(&mut self) {
            self.edges.clear();
            self.wires.clear();
            self.handles.clear();
            self.nodes.clear();
            self.selection.clear();
            self.preselect.clear();
            self.preselect_removed.clear();
            self.selection_exit_highlight.clear();
        }

        pub fn parse_fixture_v1(&mut self, raw: &serde_json::Value) -> bool {
            let f: FixtureV1Json = match serde_json::from_value(raw.clone()) {
                Ok(v) => v,
                Err(_) => return false,
            };
            self.port_mode = match f.schema.as_str() {
                "reasoning.mindmap.fixture/v1" => GraphPortMode::Normal,
                "puzzle.2d.fixture/v1" => GraphPortMode::Ported,
                _ => return false,
            };
            if !self.has_ports() {
                self.selection_options.select_handles = false;
            }
            self.set_camera(f.camera.x, f.camera.y, f.camera.zoom);
            self.clear_scene();
            let mut desc = SceneDescriptorJson::default();
            for entry in f.nodes {
                let Some(obj) = entry.as_object() else {
                    return false;
                };
                let Some(id) = obj.get("id").and_then(|v| v.as_str()) else {
                    return false;
                };
                let Some(x) = obj.get("x").and_then(|v| v.as_f64()) else {
                    return false;
                };
                let Some(y) = obj.get("y").and_then(|v| v.as_f64()) else {
                    return false;
                };
                if !x.is_finite() || !y.is_finite() {
                    return false;
                }
                let text = obj.get("text").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(String::from);
                if self.has_ports() {
                    let Some(handles_arr) = obj.get("handles").and_then(|v| v.as_array()) else {
                        return false;
                    };
                    let mut handles: Vec<HandleDescJson> = Vec::new();
                    for h in handles_arr {
                        let Some(ho) = h.as_object() else {
                            return false;
                        };
                        let Some(hid) = ho.get("id").and_then(|v| v.as_str()) else {
                            return false;
                        };
                        let Some(angle) = ho.get("angle").and_then(|v| v.as_f64()) else {
                            return false;
                        };
                        if !angle.is_finite() {
                            return false;
                        }
                        let handle_kind = ho.get("handleKind").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(String::from).unwrap_or_else(|| "port".into());
                        let handle_color = ho.get("color").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(String::from);
                        let handle_icon_kind = ho.get("iconKind").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                        let handle_scale = ho.get("scale").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0);
                        handles.push(HandleDescJson {
                            id: hid.into(),
                            node_id: id.into(),
                            angle,
                            radius: None,
                            scale: handle_scale,
                            selected: None,
                            style: None,
                            handle_kind: Some(handle_kind),
                            color: handle_color,
                            icon_kind: handle_icon_kind,
                            user_data: None,
                            visible: board_json_visible_option(ho),
                        });
                    }
                    desc.handles.extend(handles);
                } else if obj.get("handles").is_some() {
                    return false;
                }
                let shape_str = obj.get("shape").and_then(|v| v.as_str());
                let fixture_node_kind = obj.get("nodeKind").or_else(|| obj.get("node_kind")).and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let fixture_node_scale = obj.get("scale").and_then(|v| v.as_f64()).filter(|v| v.is_finite() && *v > 0.0);
                if shape_str == Some("rectangle") {
                    let Some(width) = obj.get("width").and_then(|v| v.as_f64()) else {
                        return false;
                    };
                    let Some(height) = obj.get("height").and_then(|v| v.as_f64()) else {
                        return false;
                    };
                    if width <= 0.0 || height <= 0.0 {
                        return false;
                    }
                    let root = obj.get("root").and_then(|v| v.as_bool());
                    let icon_kind = obj.get("iconKind").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                    desc.nodes.push(NodeDescJson {
                        id: id.into(),
                        x,
                        y,
                        draggable: None,
                        selected: None,
                        style: None,
                        text,
                        icon_kind,
                        node_kind: fixture_node_kind.clone(),
                        user_data: None,
                        visible: board_json_visible_option(obj),
                        root,
                        shape: Some("rectangle".into()),
                        radius: None,
                        width: Some(width),
                        height: Some(height),
                        scale: fixture_node_scale,
                    });
                } else {
                    let Some(radius) = obj.get("radius").and_then(|v| v.as_f64()) else {
                        return false;
                    };
                    if radius <= 0.0 {
                        return false;
                    }
                    let root = obj.get("root").and_then(|v| v.as_bool());
                    let icon_kind = obj.get("iconKind").and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                    desc.nodes.push(NodeDescJson {
                        id: id.into(),
                        x,
                        y,
                        draggable: None,
                        selected: None,
                        style: None,
                        text,
                        icon_kind,
                        node_kind: fixture_node_kind.clone(),
                        user_data: None,
                        visible: board_json_visible_option(obj),
                        root,
                        shape: Some("circle".into()),
                        radius: Some(radius),
                        width: None,
                        height: None,
                        scale: fixture_node_scale,
                    });
                }
            }
            for entry in f.edges {
                let Some(e) = entry.as_object() else {
                    return false;
                };
                let Some(id) = e.get("id").and_then(|v| v.as_str()) else {
                    return false;
                };
                let Some((source, target)) = fixture_edge_handle_ids_from_object(e) else {
                    return false;
                };
                if !self.has_ports() {
                    let node_ids: BTreeSet<&str> = desc.nodes.iter().map(|n| n.id.as_str()).collect();
                    if !node_ids.contains(source) || !node_ids.contains(target) {
                        return false;
                    }
                }
                let edge_kind = e.get("edgeKind").or_else(|| e.get("edge_kind")).and_then(|v| v.as_str()).map(str::trim).filter(|s| !s.is_empty()).map(|s| s.to_string());
                let source_tip = e
                    .get("sourceTip")
                    .or_else(|| e.get("source_tip"))
                    .and_then(|v| v.as_str())
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string());
                let target_tip = e
                    .get("targetTip")
                    .or_else(|| e.get("target_tip"))
                    .and_then(|v| v.as_str())
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string());
                desc.edges.push(EdgeDescJson {
                    id: id.into(),
                    source: source.into(),
                    target: target.into(),
                    edge_kind,
                    source_tip,
                    target_tip,
                    selected: None,
                    style: None,
                    user_data: None,
                    visible: board_json_visible_option(e),
                });
            }
            if self.sync_descriptor(&desc).is_err() {
                return false;
            }
            true
        }

        fn drawable_cull_pad_world(&self) -> f64 {
            16.0 / self.camera.zoom.max(1e-9)
        }

        fn visible_world_box(&self, pad_world: f64) -> WorldBox {
            let corners = [
                self.screen_to_world(Point::new(0.0, 0.0)),
                self.screen_to_world(Point::new(self.width as f64, 0.0)),
                self.screen_to_world(Point::new(self.width as f64, self.height as f64)),
                self.screen_to_world(Point::new(0.0, self.height as f64)),
            ];
            let base = world_box_from_points(&corners).unwrap_or(WorldBox { min_x: self.camera.x - 1.0, min_y: self.camera.y - 1.0, max_x: self.camera.x + 1.0, max_y: self.camera.y + 1.0 });
            inflate_world_box(base, pad_world)
        }

        fn world_tile_screen_clip_rect(&self, ix: i32, iy: i32, tile: f64) -> Rect {
            let wx0 = ix as f64 * tile;
            let wy0 = iy as f64 * tile;
            let wx1 = wx0 + tile;
            let wy1 = wy0 + tile;
            let ps = [self.world_to_screen(Point::new(wx0, wy0)), self.world_to_screen(Point::new(wx1, wy0)), self.world_to_screen(Point::new(wx1, wy1)), self.world_to_screen(Point::new(wx0, wy1))];
            let mut min_x = f64::INFINITY;
            let mut min_y = f64::INFINITY;
            let mut max_x = f64::NEG_INFINITY;
            let mut max_y = f64::NEG_INFINITY;
            for p in ps {
                min_x = min_x.min(p.x);
                min_y = min_y.min(p.y);
                max_x = max_x.max(p.x);
                max_y = max_y.max(p.y);
            }
            Rect::from_points(Point::new(min_x, min_y), Point::new(max_x, max_y)).inflate(1.0, 1.0)
        }

        fn handle_world_bounds_cull(&self, h: &HandleData) -> Option<WorldBox> {
            let pos = self.handle_world_pos(h)?;
            let pad = self.drawable_cull_pad_world() + self.effective_handle_radius(h).max(1.0);
            Some(inflate_world_box(WorldBox { min_x: pos.x, min_y: pos.y, max_x: pos.x, max_y: pos.y }, pad))
        }

        fn indirect_handle_world_bounds_cull(&self, h: &HandleData) -> Option<WorldBox> {
            let pos = self.indirect_handle_world_pos(h)?;
            let pad = self.drawable_cull_pad_world() + self.indirect_handle_marker_radius_world(h).max(1.0);
            Some(inflate_world_box(WorldBox { min_x: pos.x, min_y: pos.y, max_x: pos.x, max_y: pos.y }, pad))
        }

        fn edge_world_bounds_for_cull(&self, e: &EdgeData) -> Option<WorldBox> {
            let c = self.edge_curve(e)?;
            let axis = cubic_bezier_axis_bounds(c);
            let half_w_world = self.camera.zoom.max(0.75) / self.camera.zoom.max(1e-9);
            Some(inflate_world_box(axis, half_w_world + self.drawable_cull_pad_world()))
        }

        fn stroke_world_step_grid(&self, scene: &mut Scene, color: Color, stroke_px: f64, world_step: f64, min_step_screen: f64) {
            let step = world_step * self.camera.zoom;
            if step < min_step_screen {
                return;
            }
            let stroke = Stroke::new(stroke_px);
            let w = self.width as f64;
            let h = self.height as f64;
            let origin = self.world_to_screen(Point::new(0.0, 0.0));
            let x_off = ((origin.x % step) + step) % step;
            let y_off = ((origin.y % step) + step) % step;
            let mut p = crate::vello::kurbo::BezPath::new();
            let mut x = x_off;
            while x <= w {
                p.move_to(Point::new(x, 0.0));
                p.line_to(Point::new(x, h));
                x += step;
            }
            let mut y = y_off;
            while y <= h {
                p.move_to(Point::new(0.0, y));
                p.line_to(Point::new(w, y));
                y += step;
            }
            scene.stroke(&stroke, Affine::IDENTITY, color, None, &p);
        }

        fn draw_space_point(&self, world: Point, world_space: bool) -> Point {
            if world_space {
                world
            } else {
                self.world_to_screen(world)
            }
        }

        fn draw_space_len(&self, len_world: f64, world_space: bool) -> f64 {
            if world_space {
                len_world.max(1e-9)
            } else {
                (len_world * self.camera.zoom).max(1.0)
            }
        }

        fn append_handle_marker(&self, scene: &mut Scene, h: &HandleData, center: Point, radius_world: f64, draw_icon: bool, style_kind: BoardElementStyleKind, paint_override: Option<(Color, Color, f64)>, world_space: bool) {
            let c = self.draw_space_point(center, world_space);
            let r = self.draw_space_len(radius_world, world_space);
            let circle = Circle::new(c, r);
            let (fill, stroke_c, stroke_px) =
                if let Some((f, s, sw)) = paint_override { (f, s, sw) } else { (self.resolve_handle_fill_color(h, &self.vello_theme, style_kind), self.resolve_handle_stroke_color(h, &self.vello_theme, style_kind), 2.0_f64) };
            scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &circle);
            scene.stroke(&Stroke::new(stroke_px), Affine::IDENTITY, stroke_c, None, &circle);
            if draw_icon {
                if let Some(k) = h.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
                    let preserve_original_style = self.preserve_original_element_style || style_kind == BoardElementStyleKind::Original;
                    if let Some((bx, by, bw, bh, body)) = self.get_or_build_icon_paint(k, stroke_c, fill, preserve_original_style) {
                        let fit_inset = 0.62;
                        let s = self.draw_space_len(radius_world, world_space) * fit_inset;
                        let cx = bx + bw * 0.5;
                        let cy = by + bh * 0.5;
                        let avail = 2.0 * s;
                        let scale = (avail / bw).min(avail / bh);
                        let aff = Affine::translate((c.x - scale * cx, c.y - scale * cy)) * Affine::scale(scale);
                        let r_clip = self.draw_space_len(radius_world, world_space) * 0.82;
                        let disc = Circle::new(c, r_clip);
                        scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &disc);
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
            }
        }

        fn append_indirect_handle_ring(&self, scene: &mut Scene, tile_filter: Option<&WorldBox>, node_id: &str, chrome_pass: StyleChromePass, world_space: bool) {
            for h in self.handles.values() {
                if h.node_id != node_id || !self.handle_effectively_visible(h.id.as_str()) {
                    continue;
                }
                if !self.indirect_ring_handle_eligible(h.id.as_str(), node_id) {
                    continue;
                }
                if let Some(tb) = tile_filter {
                    let Some(hb) = self.indirect_handle_world_bounds_cull(h) else { continue };
                    if !world_boxes_overlap(*tb, hb) {
                        continue;
                    }
                }
                let Some(wp) = self.indirect_handle_world_pos(h) else { continue };
                let style_kind = self.resolve_handle_style_kind(h, chrome_pass);
                let stroke_px = 2.0_f64;
                let paint_override = if matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral) { Some((self.vello_theme.indirect_handle_fill, self.vello_theme.indirect_handle_stroke, stroke_px)) } else { None };
                self.append_handle_marker(scene, h, wp, self.indirect_handle_marker_radius_world(h), false, style_kind, paint_override, world_space);
            }
        }

        /// @emoji 📏 Screen-pixel edge stroke width (world-clip tiles and post-cache overlay).
        fn edge_screen_stroke_width_px(&self, lod: BoardDrawLod) -> f64 {
            match lod {
                BoardDrawLod::Minimap => 1.12_f64,
                BoardDrawLod::Overview | BoardDrawLod::Compact => (2.75_f64).max(2.0 * self.camera.zoom),
                _ => 2.0 * self.camera.zoom.max(0.75),
            }
        }

        /// @emoji 📏 Edge stroke in world units so {@link BoardHost.camera_content_affine} yields ~{@link Self::edge_screen_stroke_width_px}.
        fn edge_world_stroke_width(&self, lod: BoardDrawLod) -> f64 {
            let screen_px = self.edge_screen_stroke_width_px(lod);
            let z = self.camera.zoom.max(1e-9);
            (screen_px / z).max(1e-3)
        }

        fn append_nodes_handles_edges(&self, scene: &mut Scene, tile_filter: Option<&WorldBox>, lod: BoardDrawLod, world_space: bool) {
            self.append_nodes_and_handles(scene, tile_filter, lod, world_space, None, StyleChromePass::CachedBase);
            if !world_space {
                self.append_edges_wires_and_link(scene, tile_filter, lod, world_space, None, StyleChromePass::CachedBase);
            }
        }

        fn append_nodes_and_handles(
            &self,
            scene: &mut Scene,
            tile_filter: Option<&WorldBox>,
            lod: BoardDrawLod,
            world_space: bool,
            only_ids: Option<&BTreeSet<String>>,
            chrome_pass: StyleChromePass,
        ) {
            let pad = self.drawable_cull_pad_world();
            let draw_handles = self.has_ports() && matches!(lod, BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro);
            let draw_node_icons = matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro);
            let draw_handle_icons = lod == BoardDrawLod::Micro;
            let link_source = self.active_link_source_handle_id().map(str::to_string);
            let link_compat_nodes: std::collections::BTreeSet<String> = link_source.as_ref().map(|s| self.link_drag_compatible_target_node_ids(s).into_iter().collect()).unwrap_or_default();
            for n in self.nodes.values() {
                if !n.visible {
                    continue;
                }
                if let Some(ids) = only_ids {
                    if !ids.contains(&n.id) {
                        continue;
                    }
                }
                if let Some(tb) = tile_filter {
                    if !world_boxes_overlap(*tb, self.node_world_bounds(n, pad)) {
                        continue;
                    }
                }
                let link_compat = link_compat_nodes.contains(&n.id);
                let resolved_style_kind = self.resolve_node_style_kind(n, chrome_pass);
                let style_kind = if link_compat && matches!(resolved_style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral) { BoardElementStyleKind::Highlighted } else { resolved_style_kind };
                let draw_node_stroke = lod != BoardDrawLod::Minimap || !matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral);
                let stroke_c = Self::node_stroke_for_style(&self.vello_theme, style_kind);
                let fill = if lod == BoardDrawLod::Minimap && matches!(style_kind, BoardElementStyleKind::Original | BoardElementStyleKind::Neutral) {
                    stroke_c
                } else {
                    self.resolve_node_fill_color(n, &self.vello_theme, style_kind)
                };
                let sw = 2.0_f64;
                match n.shape {
                    NodeShape::Circle => {
                        let c = self.draw_space_point(Point::new(n.x, n.y), world_space);
                        let r = self.draw_space_len(self.scaled_node_radius(n), world_space);
                        let circle = Circle::new(c, r);
                        scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &circle);
                        if draw_node_stroke {
                            scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &circle);
                        }
                    }
                    NodeShape::Rectangle => {
                        let hw = self.scaled_node_width(n) / 2.0;
                        let hh = self.scaled_node_height(n) / 2.0;
                        let p0 = self.draw_space_point(Point::new(n.x - hw, n.y - hh), world_space);
                        let p1 = self.draw_space_point(Point::new(n.x + hw, n.y + hh), world_space);
                        let r = Rect::from_points(p0, p1);
                        scene.fill(Fill::NonZero, Affine::IDENTITY, fill, None, &r);
                        if draw_node_stroke {
                            scene.stroke(&Stroke::new(sw), Affine::IDENTITY, stroke_c, None, &r);
                        }
                    }
                }
                if draw_node_icons {
                    if let Some(k) = n.icon_kind.as_ref().map(|s| s.trim()).filter(|s| !s.is_empty()) {
                        let preserve_original_style = self.preserve_original_element_style || style_kind == BoardElementStyleKind::Original;
                        if let Some((bx, by, bw, bh, body)) = self.get_or_build_icon_paint(k, stroke_c, fill, preserve_original_style) {
                            let clip_inset = 0.88;
                            let fit_inset = 0.76;
                            let (sx_half, sy_half) = match n.shape {
                                NodeShape::Circle => {
                                    let s = self.draw_space_len(self.scaled_node_radius(n), world_space) * fit_inset;
                                    (s, s)
                                }
                                NodeShape::Rectangle => (
                                    self.draw_space_len(self.scaled_node_width(n), world_space) * fit_inset * 0.5,
                                    self.draw_space_len(self.scaled_node_height(n), world_space) * fit_inset * 0.5,
                                ),
                            };
                            let center = self.draw_space_point(Point::new(n.x, n.y), world_space);
                            let cx = bx + bw * 0.5;
                            let cy = by + bh * 0.5;
                            let avail_w = 2.0 * sx_half;
                            let avail_h = 2.0 * sy_half;
                            let scale = (avail_w / bw).min(avail_h / bh);
                            let aff = Affine::translate((center.x - scale * cx, center.y - scale * cy)) * Affine::scale(scale);
                            match n.shape {
                                NodeShape::Circle => {
                                    let r_clip = self.draw_space_len(self.scaled_node_radius(n), world_space) * clip_inset;
                                    let disc = Circle::new(center, r_clip);
                                    scene.push_clip_layer(Fill::NonZero, Affine::IDENTITY, &disc);
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
                                NodeShape::Rectangle => {
                                    let hw = self.draw_space_len(self.scaled_node_width(n), world_space) * clip_inset * 0.5;
                                    let hh = self.draw_space_len(self.scaled_node_height(n), world_space) * clip_inset * 0.5;
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
                        }
                    }
                }
            }
            for h in self.handles.values() {
                if !draw_handles || !self.handle_effectively_visible(h.id.as_str()) {
                    continue;
                }
                if let Some(ids) = only_ids {
                    if !ids.contains(&h.id) {
                        continue;
                    }
                }
                if let Some(tb) = tile_filter {
                    let Some(hb) = self.handle_world_bounds_cull(h) else { continue };
                    if !world_boxes_overlap(*tb, hb) {
                        continue;
                    }
                }
                let Some(wp) = self.handle_world_pos(h) else { continue };
                let style_kind = self.resolve_handle_style_kind(h, chrome_pass);
                self.append_handle_marker(scene, h, wp, self.effective_handle_radius(h), draw_handle_icons, style_kind, None, world_space);
            }
        }

        fn append_edges_wires_and_link(
            &self,
            scene: &mut Scene,
            tile_filter: Option<&WorldBox>,
            lod: BoardDrawLod,
            world_space: bool,
            only_ids: Option<&BTreeSet<String>>,
            chrome_pass: StyleChromePass,
        ) {
            let edge_sw = if world_space {
                self.edge_world_stroke_width(lod)
            } else {
                self.edge_screen_stroke_width_px(lod)
            };
            for e in self.edges.values() {
                if !self.edge_effectively_visible(e) {
                    continue;
                }
                if let Some(ids) = only_ids {
                    if !ids.contains(&e.id) {
                        continue;
                    }
                }
                if let Some(tb) = tile_filter {
                    let Some(eb) = self.edge_world_bounds_for_cull(e) else { continue };
                    if !world_boxes_overlap(*tb, eb) {
                        continue;
                    }
                }
                if let Some(c) = self.edge_curve(e) {
                    let p0 = self.draw_space_point(c.p0, world_space);
                    let p1 = self.draw_space_point(c.p1, world_space);
                    let p2 = self.draw_space_point(c.p2, world_space);
                    let p3 = self.draw_space_point(c.p3, world_space);
                    let curve = CubicBez::new(p0, p1, p2, p3);
                    let (stroke_color, edge_stroke, stroke_w) = self.resolve_edge_stroke_paint(e, chrome_pass, edge_sw);
                    scene.stroke(&edge_stroke, Affine::IDENTITY, stroke_color, None, &curve);
                    let (source_tip, target_tip) = self.resolve_edge_tips(e);
                    Self::append_edge_tips_on_curve(scene, &curve, stroke_color, stroke_w, source_tip, target_tip);
                }
            }
            let wire_sw = 2.25_f64;
            let wire_stroke = Stroke::new(wire_sw);
            for w in self.wires.values() {
                if !self.wire_effectively_visible(w) {
                    continue;
                }
                if let Some(ids) = only_ids {
                    if !ids.contains(&w.id) {
                        continue;
                    }
                }
                if let Some(c) = self.wire_curve(w) {
                    let p0 = self.draw_space_point(c.p0, world_space);
                    let p1 = self.draw_space_point(c.p1, world_space);
                    let p2 = self.draw_space_point(c.p2, world_space);
                    let p3 = self.draw_space_point(c.p3, world_space);
                    let curve = CubicBez::new(p0, p1, p2, p3);
                    let wc = Self::wire_stroke_for_style(&self.vello_theme, self.resolve_wire_style_kind(w, chrome_pass));
                    scene.stroke(&wire_stroke, Affine::IDENTITY, wc, None, &curve);
                }
            }
            let link_wire_sw = 2.85_f64;
            let link_wire_stroke = Stroke::new(link_wire_sw);
            let link_wire_color = self.vello_theme.node_stroke;
            if let Some(c) = self.active_link_wire_curve() {
                let p0 = self.draw_space_point(c.p0, world_space);
                let p1 = self.draw_space_point(c.p1, world_space);
                let p2 = self.draw_space_point(c.p2, world_space);
                let p3 = self.draw_space_point(c.p3, world_space);
                let curve = CubicBez::new(p0, p1, p2, p3);
                scene.stroke(&link_wire_stroke, Affine::IDENTITY, link_wire_color, None, &curve);
            }
        }

        fn append_cached_world_content(&self, scene: &mut Scene, lod: BoardDrawLod) {
            let gen = self.content_scene_generation;
            let cam_aff = self.camera_content_affine();
            let mut cache = self.world_content_cache.borrow_mut();
            let needs_rebuild = cache.as_ref().map(|c| c.0 != gen || c.1 != lod).unwrap_or(true);
            if needs_rebuild {
                let mut content = Scene::new();
                self.append_nodes_and_handles(&mut content, None, lod, true, None, StyleChromePass::CachedBase);
                *cache = Some((gen, lod, content));
            }
            if let Some(cached) = cache.as_ref() {
                scene.append(&cached.2, Some(cam_aff));
            }
            let edges_in_world_space = matches!(lod, BoardDrawLod::Overview | BoardDrawLod::Compact | BoardDrawLod::Minimap);
            if edges_in_world_space {
                let mut edge_layer = Scene::new();
                self.append_edges_wires_and_link(&mut edge_layer, None, lod, true, None, StyleChromePass::CachedBase);
                scene.append(&edge_layer, Some(cam_aff));
            } else {
                self.append_edges_wires_and_link(scene, None, lod, false, None, StyleChromePass::CachedBase);
            }
            let overlay_ids = self.interaction_overlay_entity_ids();
            if !overlay_ids.is_empty() {
                let mut overlay = Scene::new();
                self.append_nodes_and_handles(&mut overlay, None, lod, false, Some(&overlay_ids), StyleChromePass::InteractionOverlay);
                self.append_edges_wires_and_link(&mut overlay, None, lod, false, Some(&overlay_ids), StyleChromePass::InteractionOverlay);
                scene.append(&overlay, None);
            }
            if let Some(c) = self.active_link_wire_curve() {
                let link_wire_stroke = Stroke::new(2.85_f64);
                let link_wire_color = self.vello_theme.node_stroke;
                let p0 = self.draw_space_point(c.p0, false);
                let p1 = self.draw_space_point(c.p1, false);
                let p2 = self.draw_space_point(c.p2, false);
                let p3 = self.draw_space_point(c.p3, false);
                let curve = CubicBez::new(p0, p1, p2, p3);
                scene.stroke(&link_wire_stroke, Affine::IDENTITY, link_wire_color, None, &curve);
            }
            if self.has_ports() {
                if let Some(node_id) = self.indirect_ring_node_id(lod) {
                    self.append_indirect_handle_ring(scene, None, &node_id, StyleChromePass::CachedBase, false);
                }
            }
            if self.fixture_drop_preview.is_some() {
                self.append_fixture_drop_preview_paint(scene, lod);
            }
            if self.active_tool == ActiveTool::Brush || self.brush_preview.is_some() {
                self.append_brush_preview_paint(scene, lod);
            }
        }

        pub fn set_wheel_zoom_active(&mut self, active: bool) {
            if active && !self.wheel_zoom_active {
                self.wheel_zoom_render_lod = Some(self.current_draw_lod());
            }
            if !active {
                self.wheel_zoom_render_lod = None;
            }
            self.wheel_zoom_active = active;
        }

        pub fn build_vector_scene(&self) -> Scene {
            let mut inner = Scene::new();
            let lod = self.draw_lod_for_frame();
            if !self.wheel_zoom_active {
                let grid_color = self.vello_theme.grid_minor_stroke;
                if lod != BoardDrawLod::Minimap {
                    self.stroke_world_step_grid(&mut inner, grid_color, 1.0, self.grid_step_large_world(), 0.0);
                    match lod {
                        BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro => {
                            self.stroke_world_step_grid(&mut inner, grid_color, 0.72, self.grid_step_medium_world(), 0.0);
                        }
                        BoardDrawLod::Minimap | BoardDrawLod::Overview | BoardDrawLod::Compact => {}
                    }
                    if matches!(lod, BoardDrawLod::Detail | BoardDrawLod::Micro) {
                        self.stroke_world_step_grid(&mut inner, grid_color, 0.48, self.grid_step_small_world(), 0.0);
                    }
                    if lod == BoardDrawLod::Micro {
                        self.stroke_world_step_grid(&mut inner, grid_color, 0.32, self.grid_step_micro_world(), 0.0);
                    }
                }
            }
            if let Some(ref pts) = self.selection_screen_preview {
                if pts.len() >= 2 {
                    let mut path = crate::vello::kurbo::BezPath::new();
                    path.move_to(pts[0]);
                    for p in pts.iter().skip(1) {
                        path.line_to(*p);
                    }
                    path.close_path();
                    inner.fill(Fill::NonZero, Affine::IDENTITY, self.vello_theme.selection_preview_fill, None, &path);
                    let mut preview_stroke = Stroke::new(1.5);
                    if self.selection_preview_crossing {
                        preview_stroke.dash_pattern = vec![5.0, 4.0].into();
                    }
                    inner.stroke(&preview_stroke, Affine::IDENTITY, self.vello_theme.selection_preview_stroke, None, &path);
                }
            }
            self.append_cached_world_content(&mut inner, lod);
            let scale = self.dpr.max(1.0);
            if (scale - 1.0).abs() < f64::EPSILON {
                inner
            } else {
                let mut scene = Scene::new();
                scene.append(&inner, Some(Affine::scale(scale)));
                scene
            }
        }

        pub fn encoded_scene_hint(&self) -> usize {
            let s = self.build_vector_scene();
            s.encoding().path_tags.len()
        }

        pub fn update_hover_from_world(&mut self, world: Point) {
            let next = self.resolve_hover_world(world);
            self.set_hovered_id(next);
        }

        pub fn set_hovered_id(&mut self, id: Option<String>) {
            let next_kind = id.as_ref().and_then(|hover_id| self.resolve_element_kind_hover(hover_id));
            if self.hovered_id == id && self.hovered_kind == next_kind {
                return;
            }
            self.bump_content_scene_generation();
            self.hovered_id = id.clone();
            self.hovered_kind = next_kind.clone();
            self.push_event(
                "hover",
                json!({
                    "id": id,
                    "kind": next_kind.as_ref().map(|(domain, kind_id)| json!({ "domain": domain, "kindId": kind_id })),
                }),
            );
        }

        /// @emoji 🖱️ Sets transitive kind hover from a catalog row (clears direct `hovered_id`).
        pub fn set_hovered_kind(&mut self, domain: Option<String>, kind_id: Option<String>) {
            let next_kind = domain.zip(kind_id);
            if self.hovered_id.is_none() && self.hovered_kind == next_kind {
                return;
            }
            self.bump_content_scene_generation();
            self.hovered_id = None;
            self.hovered_kind = next_kind.clone();
            self.push_event(
                "hover",
                json!({
                    "id": null,
                    "kind": next_kind.as_ref().map(|(domain, kind_id)| json!({ "domain": domain, "kindId": kind_id })),
                }),
            );
        }

        /// @emoji 🔇 Updates hover chrome without emitting `hover` (controlled React sync).
        pub fn set_hovered_id_silent(&mut self, id: Option<String>) {
            let next_kind = id.as_ref().and_then(|hover_id| self.resolve_element_kind_hover(hover_id));
            if self.hovered_id == id && self.hovered_kind == next_kind {
                return;
            }
            self.hovered_id = id;
            self.hovered_kind = next_kind;
        }

        /// @emoji 🔇 Mirrors controlled kind hover without emitting `hover`.
        pub fn set_hovered_kind_silent(&mut self, domain: Option<String>, kind_id: Option<String>) {
            let next_kind = domain.zip(kind_id);
            if self.hovered_id.is_none() && self.hovered_kind == next_kind {
                return;
            }
            self.hovered_id = None;
            self.hovered_kind = next_kind;
        }

        pub fn wheel_screen(&mut self, sx: f64, sy: f64, delta_y: f64) {
            let viewport = self.viewport();
            crate::cavas::camera::wheel_screen(&mut self.camera, &viewport, sx, sy, delta_y);
            self.set_camera_silent(self.camera.x, self.camera.y, self.camera.zoom);
        }

        pub fn delete_selection(&mut self) {
            if !self.has_ports() {
                let edge_ids: Vec<_> = self.selection.iter().filter(|id| self.edges.contains_key(*id)).cloned().collect();
                for id in &edge_ids {
                    self.edges.remove(id);
                    self.push_event("edgeDelete", json!({ "id": id }));
                }
                let node_ids: Vec<_> = self.selection.iter().filter(|id| self.nodes.contains_key(*id)).cloned().collect();
                for nid in &node_ids {
                    let eids: Vec<_> = self.edges.iter().filter(|(_, e)| e.source == *nid || e.target == *nid).map(|(k, _)| k.clone()).collect();
                    for eid in eids {
                        self.edges.remove(&eid);
                        self.selection.remove(&eid);
                        self.push_event("edgeDelete", json!({ "id": eid }));
                    }
                    self.nodes.remove(nid);
                    self.push_event("nodeDelete", json!({ "id": nid }));
                }
                for id in edge_ids {
                    self.selection.remove(&id);
                }
                for id in node_ids {
                    self.selection.remove(&id);
                }
                self.selection_exit_highlight.clear();
                self.sync_selection_flags_to_objects();
                self.push_select_event();
                return;
            }
            let edge_ids: Vec<_> = self.selection.iter().filter(|id| self.edges.contains_key(*id)).cloned().collect();
            for id in &edge_ids {
                self.edges.remove(id);
                self.push_event("edgeDelete", json!({ "id": id }));
            }
            let mut node_ids: BTreeSet<String> = self.selection.iter().filter(|id| self.nodes.contains_key(*id)).cloned().collect();
            for id in self.selection.iter() {
                if let Some(handle) = self.handles.get(id) {
                    node_ids.insert(handle.node_id.clone());
                }
            }
            let node_ids: Vec<_> = node_ids.into_iter().collect();
            for nid in &node_ids {
                let handle_ids: Vec<_> = self.handles.iter().filter(|(_, h)| &h.node_id == nid).map(|(k, _)| k.clone()).collect();
                for hid in handle_ids {
                    let wids: Vec<_> = self.wires.iter().filter(|(_, w)| w.source == *hid || w.target.as_ref() == Some(&hid)).map(|(k, _)| k.clone()).collect();
                    for wid in &wids {
                        self.wires.remove(wid);
                        self.selection.remove(wid);
                    }
                    let eids: Vec<_> = self.edges.iter().filter(|(_, e)| e.source == hid || e.target == hid).map(|(k, _)| k.clone()).collect();
                    for eid in eids {
                        self.edges.remove(&eid);
                        self.selection.remove(&eid);
                        self.push_event("edgeDelete", json!({ "id": eid }));
                    }
                    self.handles.remove(&hid);
                    self.selection.remove(&hid);
                }
                self.nodes.remove(nid);
                self.push_event("nodeDelete", json!({ "id": nid }));
            }
            for id in edge_ids {
                self.selection.remove(&id);
            }
            for id in node_ids {
                self.selection.remove(&id);
            }
            self.selection_exit_highlight.clear();
            self.sync_selection_flags_to_objects();
            self.push_select_event();
        }

        fn link_snap_drag_tolerance_screen(&self, h: &HandleData) -> f64 {
            let z = self.camera.zoom.max(1e-9);
            HANDLE_HIT_TOLERANCE_PX + LINK_HANDLE_SNAP_EXTRA_PX + self.effective_handle_radius(h) * z
        }

        fn link_snap_commit_proximity_ok(&self, target_handle_id: &str, world: Point) -> bool {
            let Some(h) = self.handles.get(target_handle_id) else {
                return false;
            };
            if !self.handle_effectively_visible(target_handle_id) {
                return false;
            }
            let Some(pw) = self.handle_world_pos(h) else {
                return false;
            };
            let z = self.camera.zoom.max(1e-9);
            let d_screen = distance_between(self.world_to_screen(world), self.world_to_screen(pw));
            let tol_commit = HANDLE_HIT_TOLERANCE_PX + LINK_COMMIT_SNAP_TIGHT_PX + self.effective_handle_radius(h) * z;
            d_screen <= tol_commit
        }

        /// @emoji 🔗 True when any edge uses this handle as `source` or `target` (handle already participates in a link).
        fn handle_has_incident_edge(&self, handle_id: &str) -> bool {
            self.edges.values().any(|e| e.source == handle_id || e.target == handle_id)
        }

        fn node_has_any_incident_edge(&self, node_id: &str) -> bool {
            self.handles.values().filter(|h| h.node_id == node_id).any(|h| self.handle_has_incident_edge(h.id.as_str()))
        }

        fn lod_allows_node_proximity_connect(&self) -> bool {
            matches!(self.current_draw_lod(), BoardDrawLod::Normal | BoardDrawLod::Detail | BoardDrawLod::Micro)
        }

        /// @emoji 🧲 While dragging a node with no incident edges, overlapping bounds pick the nearest compatible free handle pair.
        fn node_drag_proximity_handle_pair(&self, moving_node_id: &str) -> Option<(String, String)> {
            if !self.lod_allows_node_proximity_connect() {
                return None;
            }
            if !self.node_effectively_visible(moving_node_id) {
                return None;
            }
            if self.node_has_any_incident_edge(moving_node_id) {
                return None;
            }
            let moving = self.nodes.get(moving_node_id)?;
            let moving_bounds = self.node_world_bounds(moving, 0.0);
            let mut best: Option<(f64, String, String)> = None;
            for (target_id, target) in &self.nodes {
                if target_id == moving_node_id || !self.node_effectively_visible(target_id.as_str()) {
                    continue;
                }
                let target_bounds = self.node_world_bounds(target, 0.0);
                if !world_boxes_overlap(moving_bounds, target_bounds) {
                    continue;
                }
                let moving_handles: Vec<_> = self.handles.iter().filter(|(id, h)| h.node_id == moving_node_id && self.handle_effectively_visible(id.as_str()) && !self.handle_has_incident_edge(id.as_str())).collect();
                let target_handles: Vec<_> = self.handles.iter().filter(|(id, h)| h.node_id == target_id.as_str() && self.handle_effectively_visible(id.as_str()) && !self.handle_has_incident_edge(id.as_str())).collect();
                for (src_id, src_h) in &moving_handles {
                    let Some(src_pos) = self.handle_world_pos(src_h) else {
                        continue;
                    };
                    for (tgt_id, tgt_h) in &target_handles {
                        let Some(tgt_pos) = self.handle_world_pos(tgt_h) else {
                            continue;
                        };
                        let d = distance_between(src_pos, tgt_pos);
                        let pair = if self.handles_link_compatible_for_drag(src_h, tgt_h) {
                            Some(((*src_id).clone(), (*tgt_id).clone()))
                        } else if self.handles_link_compatible_for_drag(tgt_h, src_h) {
                            Some(((*tgt_id).clone(), (*src_id).clone()))
                        } else {
                            None
                        };
                        if let Some((s, t)) = pair {
                            if best.as_ref().map(|(bd, _, _)| d < *bd).unwrap_or(true) {
                                best = Some((d, s.to_string(), t.to_string()));
                            }
                        }
                    }
                }
            }
            best.map(|(_, s, t)| (s, t))
        }

        fn node_effectively_visible(&self, node_id: &str) -> bool {
            self.nodes.get(node_id).is_some_and(|n| n.visible)
        }

        fn handle_effectively_visible(&self, handle_id: &str) -> bool {
            self.handles.get(handle_id).is_some_and(|h| h.visible && self.node_effectively_visible(h.node_id.as_str()))
        }

        fn edge_effectively_visible(&self, edge: &EdgeData) -> bool {
            if !self.has_ports() {
                return edge.visible && self.node_effectively_visible(edge.source.as_str()) && self.node_effectively_visible(edge.target.as_str());
            }
            edge.visible && self.handle_effectively_visible(edge.source.as_str()) && self.handle_effectively_visible(edge.target.as_str())
        }

        fn wire_effectively_visible(&self, wire: &WireData) -> bool {
            wire.visible && self.handle_effectively_visible(wire.source.as_str()) && wire.target.as_ref().map(|id| self.handle_effectively_visible(id.as_str())).unwrap_or(true)
        }

        /// @emoji 💫 True when the handle may be drawn or hit-tested on the indirect-connect ghost ring (`overview`/`normal` LOD).
        fn handle_eligible_indirect_connect_ring(&self, handle_id: &str) -> bool {
            self.handle_effectively_visible(handle_id) && !self.handle_has_incident_edge(handle_id)
        }

        /// @emoji 📍 Drag-phase link snap tests **screen px** to the handle anchor so detail/micro zoom keeps a stable hit halo; pointer-up re-checks with `link_snap_commit_proximity_ok` before `proximityConnect`.
        fn nearest_link_snap_handle_world(&self, source_handle_id: &str, world: Point) -> Option<String> {
            if matches!(self.current_draw_lod(), BoardDrawLod::Minimap) {
                return None;
            }
            let source_handle = self.handles.get(source_handle_id)?;
            if !self.handle_effectively_visible(source_handle_id) {
                return None;
            }
            let source_node_id = source_handle.node_id.as_str();
            let p_scr = self.world_to_screen(world);
            let mut best: Option<(f64, String)> = None;
            for (id, h) in &self.handles {
                if id == source_handle_id || !self.handle_effectively_visible(id.as_str()) {
                    continue;
                }
                if self.handle_has_incident_edge(id.as_str()) {
                    continue;
                }
                if h.node_id == source_node_id {
                    continue;
                }
                if !self.handles_link_compatible_for_drag(source_handle, h) {
                    continue;
                }
                let pw = self.handle_world_pos(h)?;
                let h_scr = self.world_to_screen(pw);
                let d_screen = distance_between(p_scr, h_scr);
                let tol_screen = self.link_snap_drag_tolerance_screen(h);
                if d_screen <= tol_screen && best.as_ref().map(|(bd, _)| d_screen < *bd).unwrap_or(true) {
                    best = Some((d_screen, id.clone()));
                }
            }
            best.map(|(_, id)| id)
        }

        fn try_commit_link_edge(&mut self, source_handle_id: &str, target_handle_id: &str, also_emit: Option<&'static str>) -> bool {
            if source_handle_id == target_handle_id {
                return false;
            }
            if !self.handle_effectively_visible(source_handle_id) || !self.handle_effectively_visible(target_handle_id) {
                return false;
            }
            let Some(source_row) = self.handles.get(source_handle_id) else {
                return false;
            };
            let Some(target_row) = self.handles.get(target_handle_id) else {
                return false;
            };
            if source_row.node_id == target_row.node_id {
                return false;
            }
            if !self.handles_link_compatible_for_drag(source_row, target_row) {
                return false;
            }
            if self.handle_has_incident_edge(source_handle_id) || self.handle_has_incident_edge(target_handle_id) {
                return false;
            }
            for e in self.edges.values() {
                if e.source == source_handle_id && e.target == target_handle_id {
                    return false;
                }
            }
            let mut n = self.edges.len().saturating_add(1);
            let id = loop {
                let candidate = format!("edge-link-{n}");
                if !self.edges.contains_key(&candidate) {
                    break candidate;
                }
                n = n.saturating_add(1);
            };
            let edge_kind = self.default_edge_kind_for_created_link(source_row, target_row);
            self.edges.insert(
                id.clone(),
                EdgeData {
                    id: id.clone(),
                    source: source_handle_id.to_string(),
                    target: target_handle_id.to_string(),
                    selected: false,
                    visible: true,
                    style: None,
                    edge_kind,
                    source_tip: None,
                    target_tip: None,
                },
            );
            self.push_event("edgeCreate", json!({ "id": id, "source": source_handle_id, "target": target_handle_id }));
            if let Some(name) = also_emit {
                self.push_event(name, json!({ "id": id, "source": source_handle_id, "target": target_handle_id }));
            }
            true
        }

        pub fn pointer_down_screen(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool) {
            self.set_selection_screen_preview(None);
            let screen = Point::new(sx, sy);
            let world = self.screen_to_world(screen);
            if self.active_tool == ActiveTool::Brush {
                if button == 1 {
                    self.interaction = Interaction::Pan { origin: self.camera.clone(), start_screen: screen };
                }
                return;
            }
            let hit = self.resolve_hit_world(world).or_else(|| self.resolve_overview_free_link_handle_pointer_world(world));
            if let Interaction::LinkTargetNode { source_id, target_node_id } = self.interaction.clone() {
                self.interaction = Interaction::None;
                self.clear_link_gesture_events();
                if button == 0 {
                    if let Some(th) = self.node_sole_free_link_compatible_handle(&source_id, &target_node_id) {
                        if hit.as_deref() == Some(target_node_id.as_str()) || hit.as_deref() == Some(th.as_str()) {
                            self.try_commit_link_edge(&source_id, &th, Some("indirectConnect"));
                            self.update_hover_from_world(world);
                            return;
                        }
                    }
                    if let Some(hid) = hit.as_ref().filter(|id| self.handles.get(*id).is_some_and(|h| h.node_id == target_node_id) && self.handle_eligible_link_target_ring(id.as_str(), source_id.as_str())) {
                        self.try_commit_link_edge(&source_id, hid, Some("indirectConnect"));
                        self.update_hover_from_world(world);
                        return;
                    }
                }
                self.update_hover_from_world(world);
                return;
            }
            if let Interaction::ExternalLinkPreview { source_id, ring_node_id, ring_handle_ids, .. } = self.interaction.clone() {
                if button == 0 {
                    if let Some(target_node_id) = ring_node_id {
                        if let Some(th) = self.node_sole_free_link_compatible_handle(&source_id, &target_node_id) {
                            if hit.as_deref() == Some(target_node_id.as_str()) || hit.as_deref() == Some(th.as_str()) {
                                self.interaction = Interaction::None;
                                self.clear_link_gesture_events();
                                self.try_commit_link_edge(&source_id, &th, Some("indirectConnect"));
                                self.update_hover_from_world(world);
                                return;
                            }
                        }
                    }
                    if let Some(hid) = hit.as_ref().filter(|id| ring_handle_ids.iter().any(|rh| rh == *id)) {
                        self.interaction = Interaction::None;
                        self.clear_link_gesture_events();
                        self.try_commit_link_edge(&source_id, hid, Some("indirectConnect"));
                        self.update_hover_from_world(world);
                        return;
                    }
                }
                self.update_hover_from_world(world);
                return;
            }
            let merge_from_modifiers = ctrl_or_meta || shift;
            let pick_mode = Self::pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
            if button == 0 && !merge_from_modifiers && self.try_begin_bounded_selection_drag_at(world) {
                return;
            }
            if button == 1 {
                self.interaction = Interaction::Pan { origin: self.camera.clone(), start_screen: screen };
                return;
            }
            if let Some(ref hid) = hit {
                if let Some(node) = self.nodes.get(hid) {
                    if node.draggable {
                        let nid = hid.clone();
                        let nx = node.x;
                        let ny = node.y;
                        let members_before: Vec<String> = self.selection.iter().filter(|id| self.nodes.get(*id).is_some_and(|n| n.draggable)).cloned().collect();
                        let drag_group_before = members_before.contains(&nid) && members_before.len() > 1;
                        let force_pick_merge = (pick_mode == "replace" && !drag_group_before) || pick_mode == "subtractive" || (pick_mode == "invertive" && merge_from_modifiers);
                        if !drag_group_before || force_pick_merge {
                            let next = Self::merge_pick_into_selection(&self.selection, &nid, pick_mode.as_str());
                            let ids: Vec<_> = next.iter().cloned().collect();
                            let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
                            self.set_selection_ids_gestured(&ids, gesture);
                        }
                        let members: Vec<String> = self.selection.iter().filter(|id| self.nodes.get(*id).is_some_and(|n| n.draggable)).cloned().collect();
                        let drag_group = members.contains(&nid) && members.len() > 1;
                        let mut start_positions = BTreeMap::new();
                        for id in if drag_group { members.as_slice() } else { std::slice::from_ref(&nid) } {
                            if let Some(n) = self.nodes.get(id) {
                                start_positions.insert(id.clone(), (n.x, n.y));
                            }
                        }
                        self.interaction = Interaction::DragNodes { primary_id: nid, offset: world - Point::new(nx, ny), start_positions, proximity_pair: None };
                        self.set_hovered_id(hit);
                        return;
                    }
                }
            }
            if let Some(ref hid) = hit {
                if button == 0 && self.handles.contains_key(hid) && !self.handle_has_incident_edge(hid.as_str()) {
                    let next = Self::merge_pick_into_selection(&self.selection, hid, pick_mode.as_str());
                    let ids: Vec<_> = next.iter().cloned().collect();
                    let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
                    self.set_selection_ids_gestured(&ids, gesture);
                    self.interaction = Interaction::LinkAtSourceHandle { source_id: hid.clone(), start_screen: screen };
                    self.set_hovered_id(Some(hid.clone()));
                    return;
                }
            }
            if hit.is_none() && button == 0 {
                self.interaction = Interaction::SelectionPending { initial_ids: self.selection.clone(), start: world, start_screen: screen };
                self.set_hovered_id(None);
                return;
            }
            self.interaction = Interaction::None;
            if let Some(id) = hit {
                let next = Self::merge_pick_into_selection(&self.selection, &id, pick_mode.as_str());
                let ids: Vec<_> = next.iter().cloned().collect();
                let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
                self.set_selection_ids_gestured(&ids, gesture);
                self.set_hovered_id(Some(id));
            } else {
                let gesture = merge_from_modifiers.then_some(pick_mode.as_str());
                self.set_selection_ids_gestured(&[], gesture);
                self.set_hovered_id(None);
            }
        }

        pub fn pointer_move_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool) {
            let screen = Point::new(sx, sy);
            let world = self.screen_to_world(screen);
            if self.active_tool == ActiveTool::Brush {
                match std::mem::replace(&mut self.interaction, Interaction::None) {
                    Interaction::Pan { origin, start_screen } => {
                        let delta = screen - start_screen;
                        let nx = origin.x - delta.x / origin.zoom;
                        let ny = origin.y - delta.y / origin.zoom;
                        self.set_camera(nx, ny, origin.zoom);
                        self.interaction = Interaction::Pan { origin, start_screen };
                    }
                    _ => {
                        self.interaction = Interaction::None;
                        self.brush_pointer_move(world);
                    }
                }
                return;
            }
            match std::mem::replace(&mut self.interaction, Interaction::None) {
                Interaction::DragNodes { primary_id, offset, start_positions, .. } => {
                    let primary_id = primary_id.clone();
                    let offset = offset;
                    let start_positions_cloned = start_positions.clone();
                    let (px0, py0) = start_positions.get(&primary_id).copied().unwrap_or((0.0, 0.0));
                    let nx = world.x - offset.x;
                    let ny = world.y - offset.y;
                    let mut dx = nx - px0;
                    let mut dy = ny - py0;
                    if self.grid_snap_enabled {
                        let (snx, sny) = self.snap_world_pair(nx, ny);
                        dx = snx - px0;
                        dy = sny - py0;
                    }
                    let mut geometry_changed = false;
                    for (id, (ox0, oy0)) in &start_positions {
                        if let Some(n) = self.nodes.get_mut(id) {
                            let mx = ox0 + dx;
                            let my = oy0 + dy;
                            if (n.x - mx).abs() > 1e-9 || (n.y - my).abs() > 1e-9 {
                                geometry_changed = true;
                            }
                            n.x = mx;
                            n.y = my;
                            self.push_event("nodeMove", json!({ "id": id, "x": mx, "y": my }));
                        }
                    }
                    if geometry_changed {
                        self.bump_content_scene_generation();
                    }
                    let proximity_pair = if start_positions.len() == 1 { self.node_drag_proximity_handle_pair(primary_id.as_str()) } else { None };
                    self.interaction = Interaction::DragNodes { primary_id, offset, start_positions: start_positions_cloned, proximity_pair };
                }
                Interaction::Pan { origin, start_screen } => {
                    let delta = screen - start_screen;
                    let nx = origin.x - delta.x / origin.zoom;
                    let ny = origin.y - delta.y / origin.zoom;
                    self.set_camera(nx, ny, origin.zoom);
                    self.interaction = Interaction::Pan { origin, start_screen };
                }
                Interaction::SelectionPending { initial_ids, start, start_screen } => {
                    if distance_between(start_screen, screen) < SELECTION_CLICK_MAX_DISTANCE_PX {
                        self.interaction = Interaction::SelectionPending { initial_ids, start, start_screen };
                    } else {
                        let points = vec![start, world];
                        let screen_points = vec![start_screen, screen];
                        let merge_mode = Self::pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                        let next = self.resolve_area_selection_with_initial(&initial_ids, start, &points, merge_mode.as_str());
                        let ids: Vec<_> = next.iter().cloned().collect();
                        let merge_from_modifiers = ctrl_or_meta || shift;
                        let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
                        self.apply_area_preselect(&initial_ids, &ids, gesture);
                        self.sync_selection_screen_overlay(start_screen, &screen_points);
                        self.interaction = Interaction::Selection { initial_ids, points, screen_points, start, start_screen };
                    }
                }
                Interaction::Selection { mut points, mut screen_points, start, initial_ids, start_screen } => {
                    let last_screen = screen_points.last().copied().unwrap_or(start_screen);
                    let add_point = self.selection_options.method == "lasso" || distance_between(screen, last_screen) >= SELECTION_LASSO_MIN_POINT_DISTANCE_PX;
                    if add_point {
                        points.push(world);
                        screen_points.push(screen);
                    } else if !points.is_empty() {
                        let last = points.len() - 1;
                        points[last] = world;
                        let ls = screen_points.len() - 1;
                        screen_points[ls] = screen;
                    }
                    let initial = initial_ids.clone();
                    let pts = points.clone();
                    let merge_mode = Self::pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                    let next = self.resolve_area_selection_with_initial(&initial, start, &pts, merge_mode.as_str());
                    let ids: Vec<_> = next.iter().cloned().collect();
                    let merge_from_modifiers = ctrl_or_meta || shift;
                    let gesture = merge_from_modifiers.then_some(merge_mode.as_str());
                    self.apply_area_preselect(&initial, &ids, gesture);
                    self.sync_selection_screen_overlay(start_screen, &screen_points);
                    self.interaction = Interaction::Selection { initial_ids, points, screen_points, start, start_screen };
                }
                Interaction::LinkAtSourceHandle { source_id, start_screen } => {
                    if distance_between(screen, start_screen) >= LINK_DRAG_MIN_DISTANCE_PX {
                        let optional_target_handle_id = self.nearest_link_snap_handle_world(&source_id, world);
                        self.apply_link_drag_snap_hover(&source_id, world, optional_target_handle_id.as_deref());
                        self.interaction = Interaction::LinkDragSnap { source_id: source_id.clone(), target_id: optional_target_handle_id, end_world: world };
                        self.sync_link_gesture_events();
                    } else {
                        self.interaction = Interaction::LinkAtSourceHandle { source_id, start_screen };
                        self.update_hover_from_world(world);
                    }
                }
                Interaction::LinkDragSnap { source_id, .. } => {
                    let optional_target_handle_id = self.nearest_link_snap_handle_world(&source_id, world);
                    self.apply_link_drag_snap_hover(&source_id, world, optional_target_handle_id.as_deref());
                    self.interaction = Interaction::LinkDragSnap { source_id: source_id.clone(), target_id: optional_target_handle_id, end_world: world };
                    self.sync_link_gesture_events();
                }
                Interaction::LinkTargetNode { source_id, target_node_id } => {
                    self.interaction = Interaction::LinkTargetNode { source_id, target_node_id };
                    self.update_hover_from_world(world);
                }
                Interaction::ExternalLinkPreview { source_id, end_world, compatible_node_ids, ring_node_id, ring_handle_ids } => {
                    self.interaction = Interaction::ExternalLinkPreview { source_id, end_world, compatible_node_ids, ring_node_id, ring_handle_ids };
                    self.update_hover_from_world(world);
                }
                Interaction::None => {
                    self.interaction = Interaction::None;
                    self.update_hover_from_world(world);
                }
            }
        }

        pub fn pointer_up_screen(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool) {
            let screen = Point::new(sx, sy);
            let world = self.screen_to_world(screen);
            if self.active_tool == ActiveTool::Brush {
                if matches!(self.interaction, Interaction::Pan { .. }) {
                    self.interaction = Interaction::None;
                }
                self.brush_commit_preview();
                self.brush_clear_slot();
                self.set_hovered_id(None);
                return;
            }
            let grabbed = std::mem::take(&mut self.interaction);
            match grabbed {
                Interaction::LinkDragSnap { source_id, target_id, .. } => {
                    if let Some(ref target_handle_id) = target_id {
                        if self.link_snap_commit_proximity_ok(target_handle_id, world) && self.try_commit_link_edge(&source_id, target_handle_id, Some("proximityConnect")) {
                            self.interaction = Interaction::None;
                            self.clear_link_gesture_events();
                            self.update_hover_from_world(world);
                            return;
                        }
                    }
                    if let Some(target_node_id) = self.resolve_node_hit_world(world) {
                        let source_node_id = self.handles.get(&source_id).map(|h| h.node_id.clone());
                        if source_node_id.as_deref() != Some(target_node_id.as_str()) {
                            if let Some(sole_target) = self.node_sole_free_link_compatible_handle(source_id.as_str(), target_node_id.as_str()) {
                                self.try_commit_link_edge(&source_id, &sole_target, Some("indirectConnect"));
                                self.clear_link_gesture_events();
                            } else {
                                self.interaction = Interaction::LinkTargetNode { source_id, target_node_id: target_node_id.clone() };
                                self.set_hovered_id(Some(target_node_id));
                                self.sync_link_gesture_events();
                            }
                            self.update_hover_from_world(world);
                            return;
                        }
                    }
                    self.interaction = Interaction::None;
                    self.clear_link_gesture_events();
                    self.update_hover_from_world(world);
                }
                Interaction::LinkAtSourceHandle { .. } => {
                    self.interaction = Interaction::None;
                    self.clear_link_gesture_events();
                    self.update_hover_from_world(world);
                }
                Interaction::DragNodes { start_positions, proximity_pair: Some((src, tgt)), .. } => {
                    let _ = self.try_commit_link_edge(&src, &tgt, Some("proximityConnect"));
                    self.push_node_drag_end_events(&start_positions);
                    self.interaction = Interaction::None;
                    self.update_hover_from_world(world);
                }
                Interaction::DragNodes { start_positions, .. } => {
                    self.push_node_drag_end_events(&start_positions);
                    self.interaction = Interaction::None;
                    self.update_hover_from_world(world);
                }
                Interaction::SelectionPending { initial_ids, start, start_screen } => {
                    let _ = (start, start_screen);
                    let merge_from_modifiers = ctrl_or_meta || shift;
                    if !merge_from_modifiers {
                        self.clear_selection_on_background_click();
                    } else {
                        let merge_mode = Self::pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                        let gesture = Some(merge_mode.as_str());
                        let next = self.resolve_area_selection_with_initial(&initial_ids, start, &[start], merge_mode.as_str());
                        let ids: Vec<_> = next.iter().cloned().collect();
                        self.set_selection_ids_gestured(&ids, gesture);
                    }
                    self.set_selection_screen_preview(None);
                    self.update_hover_from_world(world);
                }
                Interaction::Selection { mut points, mut screen_points, start, initial_ids, start_screen } => {
                    points.push(world);
                    screen_points.push(screen);
                    let end_screen = screen_points.last().copied().unwrap_or(start_screen);
                    let click_only = distance_between(start_screen, end_screen) < SELECTION_CLICK_MAX_DISTANCE_PX;
                    let merge_from_modifiers = ctrl_or_meta || shift;
                    let merge_mode = Self::pick_merge_mode_for_modifiers(ctrl_or_meta, shift, self.selection_options.mode.as_str());
                    let gesture = merge_from_modifiers.then(|| merge_mode.as_str());
                    if click_only {
                        self.commit_area_select_from_initial(&initial_ids, &[], gesture);
                    } else {
                        let next = self.resolve_area_selection_with_initial(&initial_ids, start, &points, merge_mode.as_str());
                        let ids: Vec<_> = next.iter().cloned().collect();
                        self.commit_area_select_from_initial(&initial_ids, &ids, gesture);
                    }
                    self.set_selection_screen_preview(None);
                    self.update_hover_from_world(world);
                }
                Interaction::ExternalLinkPreview { .. } => {
                    self.interaction = grabbed;
                    self.update_hover_from_world(world);
                }
                _ => {
                    self.interaction = Interaction::None;
                    self.update_hover_from_world(world);
                }
            }
        }

        pub fn pointer_leave_screen(&mut self) {
            if self.active_tool == ActiveTool::Brush {
                self.brush_commit_preview();
                self.brush_clear_slot();
                self.set_hovered_id(None);
                return;
            }
            if matches!(self.interaction, Interaction::None) {
                self.set_hovered_id(None);
            }
        }

        /// @emoji ↩️ Aborts an in‑flight rectangle/lasso drag and restores the selection snapshot from when the gesture began.
        pub fn cancel_area_select(&mut self) -> bool {
            let prev = std::mem::replace(&mut self.interaction, Interaction::None);
            match prev {
                Interaction::SelectionPending { .. } => {
                    self.set_selection_screen_preview(None);
                    true
                }
                Interaction::Selection { initial_ids, .. } => {
                    self.set_selection_screen_preview(None);
                    self.preselect.clear();
                    self.preselect_removed.clear();
                    self.last_preselect_emit_sig = None;
                    self.selection = initial_ids.clone();
                    self.sync_selection_flags_to_objects();
                    self.bump_content_scene_generation();
                    self.last_select_emit_sig = None;
                    let sorted = Self::sorted_selection_ids(&self.selection);
                    self.push_event("preselectCancel", json!({ "ids": sorted }));
                    true
                }
                other => {
                    self.interaction = other;
                    false
                }
            }
        }

        fn node_world_bounds(&self, n: &NodeData, pad: f64) -> WorldBox {
            let raw = match n.shape {
                NodeShape::Rectangle => {
                    let hw = self.scaled_node_width(n) / 2.0;
                    let hh = self.scaled_node_height(n) / 2.0;
                    WorldBox { min_x: n.x - hw, min_y: n.y - hh, max_x: n.x + hw, max_y: n.y + hh }
                }
                NodeShape::Circle => WorldBox { min_x: n.x - self.scaled_node_radius(n), min_y: n.y - self.scaled_node_radius(n), max_x: n.x + self.scaled_node_radius(n), max_y: n.y + self.scaled_node_radius(n) },
            };
            inflate_world_box(raw, pad)
        }

        fn selection_draggable_node_members(&self) -> Vec<String> {
            self.selection.iter().filter(|id| self.nodes.get(*id).is_some_and(|n| n.draggable)).cloned().collect()
        }

        fn selection_union_bounds_world(&self) -> Option<WorldBox> {
            let mut corners: Vec<Point> = Vec::new();
            for id in &self.selection {
                let Some(n) = self.nodes.get(id) else {
                    continue;
                };
                let b = self.node_world_bounds(n, 0.0);
                corners.push(Point::new(b.min_x, b.min_y));
                corners.push(Point::new(b.max_x, b.max_y));
            }
            world_box_from_points(&corners)
        }

        /// @emoji 📦 Starts a group drag when `world` lies inside the padded union bounds of the current selection (minimap/overview LOD).
        fn try_begin_bounded_selection_drag_at(&mut self, world: Point) -> bool {
            if !self.lod_uses_bounded_drag() {
                return false;
            }
            let members = self.selection_draggable_node_members();
            if members.is_empty() {
                return false;
            }
            let Some(bounds) = self.selection_union_bounds_world() else {
                return false;
            };
            let pad = BOUNDED_DRAG_HIT_PAD_PX / self.camera.zoom.max(1e-9);
            if !world_box_contains_point(inflate_world_box(bounds, pad), world) {
                return false;
            }
            let primary_id = members
                .iter()
                .min_by(|a, b| {
                    let da = self.nodes.get(*a).map(|n| distance_between(world, Point::new(n.x, n.y))).unwrap_or(f64::INFINITY);
                    let db = self.nodes.get(*b).map(|n| distance_between(world, Point::new(n.x, n.y))).unwrap_or(f64::INFINITY);
                    da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
                })
                .cloned()
                .unwrap_or_else(|| members[0].clone());
            let (px0, py0) = self.nodes.get(&primary_id).map(|n| (n.x, n.y)).unwrap_or((0.0, 0.0));
            let mut start_positions = BTreeMap::new();
            for id in &members {
                if let Some(n) = self.nodes.get(id) {
                    start_positions.insert(id.clone(), (n.x, n.y));
                }
            }
            self.interaction = Interaction::DragNodes { primary_id, offset: world - Point::new(px0, py0), start_positions, proximity_pair: None };
            self.set_hovered_id(None);
            true
        }

        fn selection_drag_shape_world(&self, start: Point, points: &[Point]) -> Option<(WorldBox, bool, Vec<Point>)> {
            let last = points.last().copied().unwrap_or(start);
            let enclosing = last.x >= start.x;
            if self.selection_options.method == "lasso" && points.len() >= 3 {
                let poly: Vec<Point> = points.to_vec();
                let b = world_box_from_points(&poly)?;
                return Some((b, enclosing, poly));
            }
            let b = world_box_from_points(&[start, last])?;
            let poly = vec![Point::new(b.min_x, b.min_y), Point::new(b.max_x, b.min_y), Point::new(b.max_x, b.max_y), Point::new(b.min_x, b.max_y)];
            Some((b, enclosing, poly))
        }

        fn selection_contains_node(&self, n: &NodeData, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
            let bounds = self.node_world_bounds(n, 0.0);
            let lasso = self.selection_options.method == "lasso";
            if enclosing {
                if lasso {
                    polygon_contains_world_box(polygon, bounds)
                } else {
                    world_box_contains_box(box_, bounds)
                }
            } else if lasso {
                polygon_intersects_world_box(polygon, bounds)
            } else {
                world_boxes_overlap(box_, bounds)
            }
        }

        fn selection_contains_handle(&self, h: &HandleData, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
            let Some(pos) = self.handle_world_pos(h) else {
                return false;
            };
            let pad = self.effective_handle_radius(h).max(1.0);
            let bounds = WorldBox { min_x: pos.x - pad, min_y: pos.y - pad, max_x: pos.x + pad, max_y: pos.y + pad };
            let lasso = self.selection_options.method == "lasso";
            if enclosing {
                if lasso {
                    polygon_contains_world_box(polygon, bounds)
                } else {
                    world_box_contains_box(box_, bounds)
                }
            } else if lasso {
                polygon_intersects_world_box(polygon, bounds)
            } else {
                world_boxes_overlap(box_, bounds)
            }
        }

        fn selection_contains_edge(&self, c: CubicBez, box_: WorldBox, enclosing: bool, polygon: &[Point]) -> bool {
            const STEPS: usize = 24;
            let mut samples = Vec::with_capacity(STEPS + 1);
            for i in 0..=STEPS {
                let t = i as f64 / STEPS as f64;
                samples.push(cubic_bezier_point(c, t));
            }
            let lasso = self.selection_options.method == "lasso";
            if enclosing {
                if lasso {
                    samples.iter().all(|&p| point_in_polygon(p, polygon))
                } else {
                    samples.iter().all(|&p| world_box_contains_point(box_, p))
                }
            } else if lasso {
                (1..samples.len()).any(|i| segment_intersects_polygon(samples[i - 1], samples[i], polygon))
            } else {
                (1..samples.len()).any(|i| segment_intersects_world_box(samples[i - 1], samples[i], box_))
            }
        }

        fn resolve_area_selection_with_initial(&self, initial: &BTreeSet<String>, start: Point, points: &[Point], merge_mode: &str) -> BTreeSet<String> {
            let Some((box_, enclosing, ref polygon)) = self.selection_drag_shape_world(start, points) else {
                return initial.clone();
            };
            let mut hits = BTreeSet::new();
            let o = &self.selection_options;
            if o.select_nodes {
                for n in self.nodes.values() {
                    if n.visible && self.selection_contains_node(n, box_, enclosing, polygon) {
                        hits.insert(n.id.clone());
                    }
                }
            }
            if o.select_handles {
                for h in self.handles.values() {
                    if self.handle_effectively_visible(h.id.as_str()) && self.selection_contains_handle(h, box_, enclosing, polygon) {
                        hits.insert(h.id.clone());
                    }
                }
            }
            if o.select_edges {
                for e in self.edges.values() {
                    if !self.edge_effectively_visible(e) {
                        continue;
                    }
                    if let Some(c) = self.edge_curve(e) {
                        if self.selection_contains_edge(c, box_, enclosing, polygon) {
                            hits.insert(e.id.clone());
                        }
                    }
                }
            }
            if merge_mode == "replace" {
                return hits;
            }
            let mut next = initial.clone();
            for id in &hits {
                match merge_mode {
                    "additive" => {
                        next.insert(id.clone());
                    }
                    "subtractive" => {
                        next.remove(id);
                    }
                    _ => {
                        if next.contains(id) {
                            next.remove(id);
                        } else {
                            next.insert(id.clone());
                        }
                    }
                }
            }
            next
        }
    }

    #[cfg(test)]
    impl BoardHost {
        pub(crate) fn test_resolve_node_style_kind(&self, node_id: &str) -> Option<BoardElementStyleKind> {
            self.nodes
                .get(node_id)
                .map(|n| self.resolve_node_style_kind(n, StyleChromePass::InteractionOverlay))
        }
    }

    impl crate::cavas::canvas_content::CanvasContent for BoardHost {
        fn build_scene(&self) -> Scene {
            self.build_vector_scene()
        }

        fn clear_color(&self) -> Color {
            self.vello_theme.raster_clear
        }
    }
}

pub use board_host::BoardHost;

// #region 🔖Puzzle2dExtension
/// 🧩 Puzzle 2d domain extension over the property graph canvas.
#[derive(Clone, Debug, Default)]
pub struct Puzzle2dExtension;

impl cavas::CanvasExtension for Puzzle2dExtension {
    fn extension_id(&self) -> &str {
        "puzzle.2d"
    }
}

impl graph::GraphExtension for Puzzle2dExtension {}
// #endregion 🔖Puzzle2dExtension

// #region 🔖WasmHost
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

#[cfg(target_arch = "wasm32")]
use js_sys::Promise;
#[cfg(target_arch = "wasm32")]
use std::cell::RefCell;
#[cfg(target_arch = "wasm32")]
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::future_to_promise;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardComputeEdgeBezier)]
pub fn board_compute_edge_bezier(source_px: f64, source_py: f64, source_cx: f64, source_cy: f64, target_px: f64, target_py: f64, target_cx: f64, target_cy: f64) -> Vec<f64> {
    let c = compute_edge_bezier_points(Point::new(source_px, source_py), Point::new(target_px, target_py), Point::new(source_cx, source_cy), Point::new(target_cx, target_cy));
    vec![c.p0.x, c.p0.y, c.p1.x, c.p1.y, c.p2.x, c.p2.y, c.p3.x, c.p3.y]
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardDistancePointCubic)]
pub fn board_distance_point_cubic(px: f64, py: f64, p0x: f64, p0y: f64, p1x: f64, p1y: f64, p2x: f64, p2y: f64, p3x: f64, p3y: f64, steps: u32) -> f64 {
    let curve = CubicBez::new(Point::new(p0x, p0y), Point::new(p1x, p1y), Point::new(p2x, p2y), Point::new(p3x, p3y));
    distance_point_to_cubic_bezier(Point::new(px, py), curve, steps.max(1) as usize)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardRayRectEdge)]
pub fn board_ray_rect_edge(hw: f64, hh: f64, ux: f64, uy: f64) -> Vec<f64> {
    let p = vcompute::ray_from_origin_to_axis_aligned_rectangle_edge(hw, hh, ux, uy);
    vec![p.x, p.y]
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardHandlePositionCircle)]
pub fn board_handle_position_circle(cx: f64, cy: f64, radius: f64, angle: f64) -> Vec<f64> {
    let p = vcompute::handle_position_on_circle(Point::new(cx, cy), radius, angle);
    vec![p.x, p.y]
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardHandlePositionRectangle)]
pub fn board_handle_position_rectangle(cx: f64, cy: f64, width: f64, height: f64, angle: f64) -> Vec<f64> {
    let p = vcompute::handle_position_on_rectangle(Point::new(cx, cy), width, height, angle);
    vec![p.x, p.y]
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardRedrawLayoutFixtureJson)]
pub fn board_redraw_layout_fixture_json(fixture_json: &str, options_json: &str) -> Result<String, JsValue> {
    redraw_layout_fixture_json(fixture_json, options_json).map_err(|e| JsValue::from_str(&e))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = boardRedrawHandlesFixtureJson)]
pub fn board_redraw_handles_fixture_json(fixture_json: &str) -> Result<String, JsValue> {
    graph::apply_edge_handle_snap_to_fixture_v1_json(fixture_json).map_err(|e| JsValue::from_str(&e))
}

// #region 🔖WasmSession
/// 🖥️ Single WASM entry: one {@link BoardHost}, optional WebGPU surface bound via {@link BoardSession::attach_canvas}.
#[cfg(target_arch = "wasm32")]
struct BoardSessionInner {
    host: BoardHost,
    gpu: cavas::gpu_session::CanvasGpuSession,
}

#[cfg(target_arch = "wasm32")]
impl BoardSessionInner {
    fn set_logical_size_and_maybe_resize_surface(&mut self, lw: u32, lh: u32, dpr: f64, pw: u32, ph: u32) {
        self.host.set_size(lw, lh, dpr);
        self.gpu.resize_surface(pw, ph);
    }

    fn render_frame_gpu(&mut self) -> Result<(), JsValue> {
        let scene = self.host.build_vector_scene();
        let clear = self.host.vello_theme.raster_clear;
        self.gpu.render_frame(&scene, clear)
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct BoardSession {
    state: Rc<RefCell<BoardSessionInner>>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl BoardSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { state: Rc::new(RefCell::new(BoardSessionInner { host: BoardHost::new(), gpu: cavas::gpu_session::CanvasGpuSession::default() })) }
    }

    /// 🧠 Construct a normal-graph session (no handles; edges connect node ids).
    #[wasm_bindgen(js_name = newNormal)]
    pub fn new_normal() -> Self {
        Self { state: Rc::new(RefCell::new(BoardSessionInner { host: BoardHost::new_normal(), gpu: cavas::gpu_session::CanvasGpuSession::default() })) }
    }

    #[wasm_bindgen(js_name = gpuReady)]
    pub fn gpu_ready(&self) -> bool {
        self.state.borrow().gpu.gpu_ready()
    }

    #[wasm_bindgen(js_name = isDraggingAreaSelect)]
    pub fn is_dragging_area_select(&self) -> bool {
        self.state.borrow().host.is_dragging_area_select()
    }

    #[wasm_bindgen(js_name = defersDescriptorSyncFromJs)]
    pub fn defers_descriptor_sync_from_js(&self) -> bool {
        self.state.borrow().host.defers_descriptor_sync_from_js()
    }

    /// @emoji 🌊 Binds WebGPU presentation to `canvas` once; `logical_w`/`logical_h` are CSS pixels, `dpr` scales the swapchain backing store; uses `future_to_promise` so wasm-bindgen does not hold `&mut BoardSession` across `await` (avoids `borrow_fail` vs `setSize` during GPU setup).
    #[wasm_bindgen(js_name = attach_canvas)]
    pub fn attach_canvas(&mut self, canvas: HtmlCanvasElement, logical_w: u32, logical_h: u32, dpr: f64) -> Promise {
        let inner = self.state.clone();
        if inner.borrow().gpu.gpu_ready() {
            return future_to_promise(async move { Err(JsValue::from_str("canvas surface already attached")) });
        }
        let lw = logical_w.max(1);
        let lh = logical_h.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        let canvas = canvas.clone();
        future_to_promise(async move {
            let (render_ctx, renderer, surface) = cavas::gpu_session::CanvasGpuSession::create_canvas_surface(canvas.clone(), pw, ph)
                .await
                .map_err(|err| JsValue::from_str(&err))?;
            let mut g = inner.borrow_mut();
            if g.gpu.gpu_ready() {
                return Err(JsValue::from_str("canvas surface already attached"));
            }
            g.host.set_size(lw, lh, dpr);
            g.gpu.finish_attach(canvas, render_ctx, renderer, surface);
            Ok(JsValue::UNDEFINED)
        })
    }

    #[wasm_bindgen(js_name = setSize)]
    pub fn set_size(&mut self, width: u32, height: u32, dpr: f64) {
        let lw = width.max(1);
        let lh = height.max(1);
        let dpr = dpr.max(1.0);
        let pw = ((lw as f64 * dpr).round() as u32).max(1);
        let ph = ((lh as f64 * dpr).round() as u32).max(1);
        let mut inner = self.state.borrow_mut();
        inner.set_logical_size_and_maybe_resize_surface(lw, lh, dpr, pw, ph);
    }

    #[wasm_bindgen(js_name = setSelectionScreenPreview)]
    pub fn set_selection_screen_preview(&mut self, flat_xy: &[f64]) {
        let mut inner = self.state.borrow_mut();
        if flat_xy.len() < 4 || flat_xy.len() % 2 != 0 {
            inner.host.set_selection_screen_preview(None);
            return;
        }
        let mut pts = Vec::with_capacity(flat_xy.len() / 2);
        for chunk in flat_xy.chunks_exact(2) {
            pts.push(Point::new(chunk[0], chunk[1]));
        }
        inner.host.set_selection_screen_preview(Some(pts));
    }

    #[wasm_bindgen(js_name = clearSelectionScreenPreview)]
    pub fn clear_selection_screen_preview(&mut self) {
        self.state.borrow_mut().host.set_selection_screen_preview(None);
    }

    #[wasm_bindgen(js_name = syncDescriptorJson)]
    pub fn sync_descriptor_json(&mut self, json: &str) -> Result<(), JsValue> {
        let mut raw: serde_json::Value = serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
        normalize_board_descriptor_hidden_to_visible(&mut raw);
        let desc: SceneDescriptorJson = serde_json::from_value(raw).map_err(|e| JsValue::from_str(&e.to_string()))?;
        self.state.borrow_mut().host.sync_descriptor(&desc).map_err(|e| JsValue::from_str(&e))?;
        Ok(())
    }

    #[wasm_bindgen(js_name = setNodePositionsJson)]
    pub fn set_node_positions_json_wasm(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_node_positions_json(json).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = setKindCatalogsJson)]
    pub fn set_board_kind_catalogs_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_board_kind_catalogs_from_json(json).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = setVelloThemeJson)]
    pub fn set_vello_theme_json(&mut self, json: &str) {
        let _ = self.state.borrow_mut().host.set_vello_theme_from_json(json);
    }

    #[wasm_bindgen(js_name = clearIconVectorCache)]
    pub fn clear_icon_vector_cache_wasm(&mut self) {
        self.state.borrow_mut().host.clear_icon_vector_cache();
    }

    #[wasm_bindgen(js_name = parseFixtureJson)]
    pub fn parse_fixture_json(&mut self, json: &str) -> bool {
        let raw: serde_json::Value = match serde_json::from_str(json) {
            Ok(v) => v,
            Err(_) => return false,
        };
        self.state.borrow_mut().host.parse_fixture_v1(&raw)
    }

    #[wasm_bindgen(js_name = setCamera)]
    pub fn set_camera_wasm(&mut self, x: f64, y: f64, zoom: f64) {
        self.state.borrow_mut().host.set_camera(x, y, zoom);
    }

    #[wasm_bindgen(js_name = pointerDownScreen)]
    pub fn pointer_down_screen_wasm(&mut self, sx: f64, sy: f64, button: u8, shift: bool, ctrl_or_meta: bool) {
        self.state.borrow_mut().host.pointer_down_screen(sx, sy, button, shift, ctrl_or_meta);
    }

    #[wasm_bindgen(js_name = pointerMoveScreen)]
    pub fn pointer_move_screen_wasm(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool) {
        self.state.borrow_mut().host.pointer_move_screen(sx, sy, shift, ctrl_or_meta);
    }

    #[wasm_bindgen(js_name = pointerUpScreen)]
    pub fn pointer_up_screen_wasm(&mut self, sx: f64, sy: f64, shift: bool, ctrl_or_meta: bool) {
        self.state.borrow_mut().host.pointer_up_screen(sx, sy, shift, ctrl_or_meta);
    }

    #[wasm_bindgen(js_name = pointerLeaveScreen)]
    pub fn pointer_leave_screen_wasm(&mut self) {
        self.state.borrow_mut().host.pointer_leave_screen();
    }

    #[wasm_bindgen(js_name = cancelAreaSelect)]
    pub fn cancel_area_select_wasm(&mut self) -> bool {
        self.state.borrow_mut().host.cancel_area_select()
    }

    #[wasm_bindgen(js_name = wheelScreen)]
    pub fn wheel_screen_wasm(&mut self, sx: f64, sy: f64, delta_y: f64) {
        self.state.borrow_mut().host.wheel_screen(sx, sy, delta_y);
    }

    #[wasm_bindgen(js_name = setWheelZoomActive)]
    pub fn set_wheel_zoom_active_wasm(&mut self, active: bool) {
        self.state.borrow_mut().host.set_wheel_zoom_active(active);
    }

    #[wasm_bindgen(js_name = deleteSelection)]
    pub fn delete_selection_wasm(&mut self) {
        self.state.borrow_mut().host.delete_selection();
    }

    #[wasm_bindgen(js_name = drainEventsJson)]
    pub fn drain_events_json_wasm(&mut self) -> String {
        self.state.borrow_mut().host.drain_events_json()
    }

    #[wasm_bindgen(js_name = cameraJson)]
    pub fn camera_json(&self) -> String {
        let inner = self.state.borrow();
        serde_json::json!({
            "x": inner.host.camera.x,
            "y": inner.host.camera.y,
            "zoom": inner.host.camera.zoom,
        })
        .to_string()
    }

    #[wasm_bindgen(js_name = overlayPaintStateJson)]
    pub fn overlay_paint_state_json_wasm(&self) -> String {
        self.state.borrow().host.overlay_paint_state_json()
    }

    #[wasm_bindgen(js_name = setSelectionOptions)]
    pub fn set_selection_options_wasm(&mut self, method: &str, mode: &str, select_nodes: bool, select_edges: bool, select_handles: bool) {
        self.state.borrow_mut().host.set_selection_options(method, mode, select_nodes, select_edges, select_handles);
    }

    #[wasm_bindgen(js_name = setHandleLinkCompatJson)]
    pub fn set_handle_link_compat_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_handle_link_compat_from_json(json).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = setWorldRasterTiling)]
    pub fn set_world_raster_tiling_wasm(&mut self, mode: &str) {
        self.state.borrow_mut().host.set_world_raster_tiling(mode);
    }

    #[wasm_bindgen(js_name = lodScaleJson)]
    pub fn lod_scale_json_wasm(&self) -> String {
        crate::board_host::puzzle_2d_lod_scale_json()
    }

    #[wasm_bindgen(js_name = setGridSnapEnabled)]
    pub fn set_grid_snap_enabled_wasm(&mut self, enabled: bool) {
        self.state.borrow_mut().host.set_grid_snap_enabled(enabled);
    }

    #[wasm_bindgen(js_name = setGridFactor)]
    pub fn set_grid_factor_wasm(&mut self, v: f64) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_grid_factor(v).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = setActiveTool)]
    pub fn set_active_tool_wasm(&mut self, label: &str) {
        self.state.borrow_mut().host.set_active_tool(label);
    }

    #[wasm_bindgen(js_name = setBrushFlushDistance)]
    pub fn set_brush_flush_distance_wasm(&mut self, distance: f64) {
        self.state.borrow_mut().host.set_brush_flush_distance(distance);
    }

    #[wasm_bindgen(js_name = setBrushKindWeights)]
    pub fn set_brush_kind_weights_wasm(&mut self, json: &str) {
        self.state.borrow_mut().host.set_brush_kind_weights(json);
    }

    #[wasm_bindgen(js_name = setBrushNodeSize)]
    pub fn set_brush_node_size_wasm(&mut self, size: f64) {
        self.state.borrow_mut().host.set_brush_node_size(size);
    }

    #[wasm_bindgen(js_name = brushCycleCandidate)]
    pub fn brush_cycle_candidate_wasm(&mut self, forward: bool) {
        self.state.borrow_mut().host.brush_cycle_candidate(forward);
    }

    #[wasm_bindgen(js_name = brushSetCandidateIndex)]
    pub fn brush_set_candidate_index_wasm(&mut self, index: u32) {
        self.state.borrow_mut().host.brush_set_candidate_index(index as usize);
    }

    #[wasm_bindgen(js_name = brushFillJson)]
    pub fn brush_fill_json_wasm(&self, max_count: u32, seed: u32) -> String {
        self.state.borrow().host.brush_fill_json(max_count, u64::from(seed))
    }

    #[wasm_bindgen(js_name = setBrushSessionJson)]
    pub fn set_brush_session_json_wasm(&mut self, json: &str) -> Result<(), JsValue> {
        self.state
            .borrow_mut()
            .host
            .set_brush_session_mirror_json(json)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = clearBrushSessionJson)]
    pub fn clear_brush_session_json_wasm(&mut self) {
        let _ = self.state.borrow_mut().host.set_brush_session_mirror_json("");
    }

    #[wasm_bindgen(js_name = setFixtureDropPreviewJson)]
    pub fn set_fixture_drop_preview_json_wasm(&mut self, json: &str) -> Result<(), JsValue> {
        self.state
            .borrow_mut()
            .host
            .set_fixture_drop_preview_json(json)
            .map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = clearFixtureDropPreview)]
    pub fn clear_fixture_drop_preview_wasm(&mut self) {
        let _ = self.state.borrow_mut().host.set_fixture_drop_preview_json("");
    }

    #[wasm_bindgen(js_name = setLinkSessionJson)]
    pub fn set_link_session_json_wasm(&mut self, json: &str) -> Result<(), JsValue> {
        self.state.borrow_mut().host.set_external_link_preview_json(json).map_err(|e| JsValue::from_str(&e))
    }

    #[wasm_bindgen(js_name = clearLinkSessionJson)]
    pub fn clear_link_session_json_wasm(&mut self) {
        self.state.borrow_mut().host.clear_external_link_preview();
    }

    #[wasm_bindgen(js_name = setOriginalElementStyle)]
    pub fn set_original_element_style_wasm(&mut self, enabled: bool) {
        self.state.borrow_mut().host.set_original_element_style(enabled);
    }

    #[wasm_bindgen(js_name = setAutomaticLod)]
    pub fn set_automatic_lod_wasm(&mut self, enabled: bool) {
        self.state.borrow_mut().host.set_automatic_lod(enabled);
    }

    #[wasm_bindgen(js_name = setForcedDrawLodLabel)]
    pub fn set_forced_draw_lod_label_wasm(&mut self, label: &str) {
        self.state.borrow_mut().host.set_forced_draw_lod_label(label);
    }

    #[wasm_bindgen(js_name = setSelectionIdsJson)]
    pub fn set_selection_ids_json(&mut self, json: &str) -> Result<(), JsValue> {
        let ids: Vec<String> = serde_json::from_str(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
        self.state.borrow_mut().host.set_selection_ids(&ids);
        Ok(())
    }

    #[wasm_bindgen(js_name = setSelectionIdsJsonSilent)]
    pub fn set_selection_ids_json_silent(&mut self, json: &str) -> Result<(), JsValue> {
        let ids: Vec<String> = serde_json::from_str(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
        self.state.borrow_mut().host.set_selection_ids_silent(&ids);
        Ok(())
    }

    #[wasm_bindgen(js_name = setPreselectStateJsonSilent)]
    pub fn set_preselect_state_json_silent(&mut self, json: &str) -> Result<(), JsValue> {
        #[derive(serde::Deserialize)]
        struct PreselectSync {
            ids: Vec<String>,
            #[serde(default, rename = "removedIds")]
            removed_ids: Vec<String>,
        }
        let body: PreselectSync = serde_json::from_str(json).map_err(|err| JsValue::from_str(&err.to_string()))?;
        self.state.borrow_mut().host.set_preselect_state_silent(&body.ids, &body.removed_ids);
        Ok(())
    }

    #[wasm_bindgen(js_name = setHoveredIdSilent)]
    pub fn set_hovered_id_silent_wasm(&mut self, id: Option<String>) {
        self.state.borrow_mut().host.set_hovered_id_silent(id);
    }

    #[wasm_bindgen(js_name = setHoveredKindSilent)]
    pub fn set_hovered_kind_silent_wasm(&mut self, domain: Option<String>, kind_id: Option<String>) {
        self.state.borrow_mut().host.set_hovered_kind_silent(domain, kind_id);
    }

    #[wasm_bindgen(js_name = encodedSceneHint)]
    pub fn encoded_scene_hint_wasm(&self) -> usize {
        self.state.borrow().host.encoded_scene_hint()
    }

    /// @emoji 🎨 Presents one frame when a GPU surface is attached; otherwise no-op `Ok`.
    #[wasm_bindgen(js_name = renderFrame)]
    pub fn render_frame(&mut self) -> Result<(), JsValue> {
        self.state.borrow_mut().render_frame_gpu()
    }
}
// #endregion 🔖WasmSession

// #region 🔖Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cavas::vello::kurbo::Point;

    #[test]
    fn computes_handle_positions_and_edge_curves() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_node(2, 300.0, 0.0, 40.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(20, 2, std::f64::consts::PI);
        engine.create_edge(100, 10, 20);

        let curve = engine.edge_curve(100).expect("edge curve should exist");
        assert!((curve.p0.x - 40.0).abs() < 0.001);
        assert!(curve.p0.y.abs() < 0.001);
        assert!((curve.p3.x - 260.0).abs() < 0.001);
        assert!(curve.p3.y.abs() < 0.001);
        let source_radial = curve.p0 - Point::ORIGIN;
        let arm0 = curve.p1 - curve.p0;
        let align0 = vcompute::normalize_or_zero(source_radial).dot(vcompute::normalize_or_zero(arm0));
        let target_approach = Point::new(300.0, 0.0) - curve.p3;
        let arm1 = curve.p3 - curve.p2;
        let align1 = vcompute::normalize_or_zero(target_approach).dot(vcompute::normalize_or_zero(arm1));
        assert!(align0 > 0.99);
        assert!(align1 > 0.99);
    }

    #[test]
    fn drags_nodes_without_rebuilding_the_scene_catalog() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 0.0, 0.0, 30.0, true);

        engine.pointer_down(0.0, 0.0, false);
        engine.pointer_move(60.0, 25.0);
        engine.pointer_up();

        let node = engine.nodes.get(&1).expect("node should remain in the engine");
        assert_eq!(node.center, Point::new(60.0, 25.0));

        let events = engine.drain_events();
        assert!(events.iter().any(|event| matches!(event, BoardEvent::SelectionChanged { node_ids, .. } if node_ids == &vec![1])));
        assert!(events.iter().any(|event| matches!(event, BoardEvent::NodeMoved { id: 1, x, y } if (*x - 60.0).abs() < 0.001 && (*y - 25.0).abs() < 0.001)));
    }

    #[test]
    fn hit_tests_handles_before_nodes_and_edges() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_node(2, 200.0, 0.0, 40.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(20, 2, std::f64::consts::PI);
        engine.create_edge(100, 10, 20);

        let handle_point = handle_position(engine.nodes.get(&1).unwrap(), engine.handles.get(&10).unwrap());
        engine.pointer_down(handle_point.x, handle_point.y, false);

        let events = engine.drain_events();
        assert!(events.iter().any(|event| matches!(event, BoardEvent::SelectionChanged { handle_ids, .. } if handle_ids == &vec![10])));
    }

    #[test]
    fn renders_snapshot_for_nodes_handles_and_edges() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 10.0, 20.0, 18.0, true);
        engine.create_node(2, 120.0, 20.0, 18.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(20, 2, std::f64::consts::PI);
        engine.create_edge(100, 10, 20);

        let snapshot = engine.render_snapshot();
        assert_eq!(snapshot.nodes.len(), 2);
        assert_eq!(snapshot.handles.len(), 2);
        assert_eq!(snapshot.edges.len(), 1);
    }

    #[test]
    fn engine_extend_pick_keeps_node_when_adding_handle() {
        let mut engine = BoardEngine::new();
        engine.create_node(1, 0.0, 0.0, 40.0, true);
        engine.create_node(2, 300.0, 0.0, 40.0, true);
        engine.create_handle(10, 1, 0.0);
        engine.create_handle(20, 2, std::f64::consts::PI);
        engine.create_edge(100, 10, 20);

        engine.pointer_down(0.0, 0.0, false);
        let _ = engine.drain_events();
        let hp = handle_position(engine.nodes.get(&1).unwrap(), engine.handles.get(&10).unwrap());
        engine.pointer_down(hp.x, hp.y, true);
        let events = engine.drain_events();
        let last = events.iter().rev().find_map(|event| match event {
            BoardEvent::SelectionChanged { node_ids, handle_ids, edge_ids } => Some((node_ids.clone(), handle_ids.clone(), edge_ids.clone())),
            _ => None,
        });
        let Some((node_ids, handle_ids, edge_ids)) = last else {
            panic!("expected SelectionChanged");
        };
        assert!(node_ids.contains(&1));
        assert!(handle_ids.contains(&10));
        assert!(edge_ids.is_empty());
    }
}

#[cfg(test)]
mod host_tests {
    use super::vcompute::compute_edge_bezier_points;
    use super::vcompute::distance_between;
    use super::vcompute::handle_position_on_circle;
    use super::vcompute::handle_position_on_rectangle;
    use super::{BoardHost, EdgeDescJson, HandleDescJson, NodeDescJson, SceneDescriptorJson, WireDescJson};
    use crate::board_host::{BoardElementStyleKind, EdgeStrokePattern, EdgeTipGeometry, NodeShape};
    use crate::board_host::GraphPortMode;
    use crate::board_host::Interaction;
    use crate::cavas::geom_sel::cubic_bezier_point;
    use crate::cavas::vello::kurbo::Point;
    use serde_json::json;

    fn set_detail_lod(h: &mut BoardHost) {
        h.set_camera(0.0, 0.0, 2.0);
    }

    fn set_micro_lod(h: &mut BoardHost) {
        h.set_camera(0.0, 60.0, 4.5);
    }

    fn set_overview_lod(h: &mut BoardHost) {
        h.set_camera(0.0, 0.0, 0.25);
    }

    fn sample_scene() -> SceneDescriptorJson {
        SceneDescriptorJson {
            nodes: vec![NodeDescJson {
                id: "a".into(),
                x: 0.0,
                y: 0.0,
                draggable: Some(true),
                selected: None,
                style: None,
                text: None,
                icon_kind: None,
                node_kind: None,
                user_data: None,
                visible: None,
                root: None,
                shape: Some("circle".into()),
                radius: Some(40.0),
                width: None,
                height: None,
                scale: None,
            }],
            handles: vec![
                HandleDescJson { id: "a:h0".into(), node_id: "a".into(), angle: 0.0, radius: None, selected: None, style: None, handle_kind: Some("port".into()), color: None, icon_kind: None, user_data: None, visible: None, scale: None },
                HandleDescJson {
                    id: "b:h0".into(),
                    node_id: "b".into(),
                    angle: std::f64::consts::PI,
                    radius: None,
                    selected: None,
                    style: None,
                    handle_kind: Some("port".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                    visible: None,
                    scale: None,
                },
            ],
            edges: vec![EdgeDescJson { id: "e1".into(), source: "a:h0".into(), target: "b:h0".into(), edge_kind: None,
                source_tip: None,
                target_tip: None,
                selected: None,
                style: None,
                user_data: None,
                visible: None }],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        }
    }

    #[test]
    fn board_host_defers_descriptor_sync_while_panning() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let _ = h.drain_events_json();
        h.pointer_down_screen(10.0, 10.0, 1, false, false);
        assert!(h.defers_descriptor_sync_from_js());
        h.pointer_move_screen(80.0, 60.0, false, false);
        assert!(h.defers_descriptor_sync_from_js());
        let _ = h.drain_events_json();
        h.pointer_up_screen(80.0, 60.0, false, false);
        assert!(!h.defers_descriptor_sync_from_js());
    }

    #[test]
    fn board_host_defers_descriptor_sync_while_dragging_nodes() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let _ = h.drain_events_json();
        let start = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(start.x, start.y, 0, false, false);
        assert!(matches!(h.interaction, Interaction::DragNodes { .. }));
        h.pointer_move_screen(start.x + 40.0, start.y, false, false);
        assert!(h.defers_descriptor_sync_from_js());
        let ev = h.drain_events_json();
        assert!(ev.contains("nodeMove"));
        h.pointer_up_screen(start.x + 40.0, start.y, false, false);
        assert!(!h.defers_descriptor_sync_from_js());
        let end = h.drain_events_json();
        assert!(end.contains("nodeDragEnd"));
    }

    #[test]
    fn board_host_set_node_positions_updates_existing_nodes_only() {
        let mut h = BoardHost::new();
        h.set_size(400, 300, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let gen_before = h.test_content_scene_generation();
        h.set_node_positions(&[("a".into(), 12.0, 34.0), ("missing".into(), 1.0, 2.0), ("a".into(), f64::NAN, 0.0)]);
        let node = h.nodes.get("a").expect("node a should remain");
        assert!((node.x - 12.0).abs() < 0.001);
        assert!((node.y - 34.0).abs() < 0.001);
        assert!(h.test_content_scene_generation() > gen_before, "moving nodes must invalidate cached world content");
        h.set_node_positions_json(r#"[{"id":"a","x":90.0,"y":110.0}]"#).unwrap();
        let node = h.nodes.get("a").expect("node a should remain");
        assert!((node.x - 90.0).abs() < 0.001);
        assert!((node.y - 110.0).abs() < 0.001);
    }

    #[test]
    fn board_host_overlay_paint_state_json_matches_host_camera_lod_and_node_centers() {
        let mut h = BoardHost::new();
        h.set_size(640, 480, 2.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        h.set_camera_silent(12.0, -8.0, 0.2);
        if let Some(n) = h.nodes.get_mut("a") {
            n.x = 33.0;
            n.y = 44.0;
        }
        let raw: serde_json::Value = serde_json::from_str(&h.overlay_paint_state_json()).expect("overlay paint state json");
        assert!((raw["camera"]["x"].as_f64().unwrap() - 12.0).abs() < 1e-9);
        assert!((raw["camera"]["y"].as_f64().unwrap() - (-8.0)).abs() < 1e-9);
        assert!((raw["camera"]["zoom"].as_f64().unwrap() - 0.2).abs() < 1e-9);
        assert_eq!(raw["lod"].as_str(), Some("overview"));
        let nodes = raw["nodes"].as_array().expect("nodes array");
        let a = nodes.iter().find(|row| row["id"].as_str() == Some("a")).expect("node a row");
        assert!((a["x"].as_f64().unwrap() - 33.0).abs() < 1e-9);
        assert!((a["y"].as_f64().unwrap() - 44.0).abs() < 1e-9);
    }

    #[test]
    fn board_host_node_drag_invalidates_cached_world_content() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let gen_before = h.test_content_scene_generation();
        let s = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 80.0, s.y + 40.0, false, false);
        assert!(h.test_content_scene_generation() > gen_before, "node drag must rebuild cached nodes/handles, not only edges");
        let node = h.nodes.get("a").expect("dragged node");
        assert!(node.x.abs() > 1.0 || node.y.abs() > 1.0, "pointer move should translate node a away from origin");
    }

    #[test]
    fn board_host_manual_lod_follow_zoom_still_encodes_graph() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let with_automatic = h.encoded_scene_hint();
        assert!(with_automatic > 0, "sample scene should encode vector paths");
        h.set_automatic_lod(false);
        h.set_forced_draw_lod_label("");
        let manual_follow_zoom = h.encoded_scene_hint();
        assert!(manual_follow_zoom > 0, "manual follow-zoom LOD must still draw nodes/edges (hint={manual_follow_zoom})");
        h.set_forced_draw_lod_label("overview");
        let pinned_overview = h.encoded_scene_hint();
        assert!(pinned_overview > 0, "pinned overview LOD must still draw graph");
        h.set_automatic_lod(true);
        let automatic_restored = h.encoded_scene_hint();
        assert_eq!(with_automatic, automatic_restored);
    }

    #[test]
    fn board_host_pick_selection_never_sets_exit_highlight() {
        let mut h = BoardHost::new();
        h.set_size(400, 300, 1.0);
        let mut d = sample_scene();
        d.selection_exit_highlight_ids = vec!["a".into(), "ghost".into()];
        h.sync_descriptor(&d).unwrap();
        let _ = h.drain_events_json();
        assert!(h.selection_exit_highlight.is_empty());
        h.set_selection_ids(&["a".into(), "e1".into()]);
        let ev = h.drain_events_json();
        assert!(h.selection_exit_highlight.is_empty());
        assert!(ev.contains("\"exitHighlightIds\":[]"));
        h.set_selection_ids(&["e1".into()]);
        let ev2 = h.drain_events_json();
        assert!(h.selection_exit_highlight.is_empty());
        assert!(ev2.contains("\"exitHighlightIds\":[]"));
    }

    #[test]
    fn board_host_vello_theme_keeps_explicit_element_state_colors() {
        let mut h = BoardHost::new();
        h.set_vello_theme_from_json(
            r#"{
				"nodeStrokeHovered": [1, 2, 3, 255],
				"edgeStrokeHovered": [4, 5, 6, 255],
				"handleStrokeHovered": [7, 8, 9, 255],
				"wireStrokeHovered": [10, 11, 12, 255]
			}"#,
        )
        .unwrap();
        assert_eq!(h.vello_theme.node_stroke_hovered.to_rgba8(), crate::cavas::vello::peniko::Color::from_rgba8(1, 2, 3, 255).to_rgba8());
        assert_eq!(h.vello_theme.edge_stroke_hovered.to_rgba8(), crate::cavas::vello::peniko::Color::from_rgba8(4, 5, 6, 255).to_rgba8());
        assert_eq!(h.vello_theme.handle_stroke_hovered.to_rgba8(), crate::cavas::vello::peniko::Color::from_rgba8(7, 8, 9, 255).to_rgba8());
        assert_eq!(h.vello_theme.wire_stroke_hovered.to_rgba8(), crate::cavas::vello::peniko::Color::from_rgba8(10, 11, 12, 255).to_rgba8());
    }

    #[test]
    fn board_host_cancel_area_select_restores_initial_selection() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into(), "b".into()]);
        let _ = h.drain_events_json();
        h.pointer_down_screen(5.0, 5.0, 0, false, false);
        assert!(!h.is_dragging_area_select());
        h.pointer_move_screen(20.0, 5.0, false, false);
        assert!(h.is_dragging_area_select());
        let _ = h.drain_events_json();
        assert!(h.cancel_area_select());
        assert!(!h.is_dragging_area_select());
        let ev = h.drain_events_json();
        assert!(ev.contains("preselectCancel"));
        assert!(!ev.contains("\"select\""));
        assert_eq!(h.selection.len(), 2);
        assert!(h.selection.contains("a") && h.selection.contains("b"));
        assert!(h.preselect.is_empty());
    }

    #[test]
    fn board_host_syncs_descriptor_and_hit_tests_handle_before_node() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hit = h.resolve_hit_world(hp);
        assert_eq!(hit.as_deref(), Some("a:h0"));
        assert!(h.encoded_scene_hint() > 10);
    }

    #[test]
    fn board_host_cached_content_includes_edge_vector_paths_at_overview_zoom() {
        let mut h = BoardHost::new();
        h.set_size(1200, 800, 1.0);
        h.sync_descriptor(&link_test_scene_a_to_b_linked()).unwrap();
        h.set_camera_silent(0.0, 0.0, 0.21);
        let with_edges = h.encoded_scene_hint();
        let mut without = link_test_scene_no_edge();
        h.sync_descriptor(&without).unwrap();
        let without_edges = h.encoded_scene_hint();
        assert!(
            with_edges > without_edges,
            "overview cached draw must encode edges (with={with_edges}, without={without_edges})"
        );
    }

    #[test]
    fn board_host_world_clip_changes_vector_encoding() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 600.0,
            y: 400.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        h.set_world_raster_tiling("none");
        let monolithic = h.encoded_scene_hint();
        h.set_world_raster_tiling("world-clip");
        let tiled = h.encoded_scene_hint();
        assert!(tiled >= monolithic);
    }

    #[test]
    fn board_host_silent_selection_keeps_cached_world_content_warm() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let gen_before = h.test_content_scene_generation();
        let neutral_hint = h.encoded_scene_hint();
        h.set_selection_ids_silent(&["a".into()]);
        assert_eq!(h.test_content_scene_generation(), gen_before, "selection chrome must paint via overlay without rebuilding cached geometry");
        assert!(h.encoded_scene_hint() > neutral_hint, "overlay must still encode selected chrome");
        assert_eq!(h.test_resolve_node_style_kind("a"), Some(BoardElementStyleKind::Selected));
    }

    #[test]
    fn board_host_selected_node_keeps_selected_style_when_hovered() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&sample_scene()).unwrap();
        h.set_selection_ids(&["a".into()]);
        h.set_hovered_id_silent(Some("a".into()));
        assert_eq!(h.test_resolve_node_style_kind("a"), Some(BoardElementStyleKind::Selected), "committed selection chrome should beat hover while pointer is over the node");
        h.set_selection_ids(&[]);
        h.set_hovered_id_silent(Some("a".into()));
        assert_eq!(h.test_resolve_node_style_kind("a"), Some(BoardElementStyleKind::Hovered), "unselected nodes should still use hover chrome");
    }

    #[test]
    fn board_host_dragging_selected_node_keeps_selected_style_at_detail_lod() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&sample_scene()).unwrap();
        h.set_selection_ids(&["a".into()]);
        let s = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        assert!(matches!(h.interaction, Interaction::DragNodes { .. }));
        assert_eq!(h.hovered_id.as_deref(), Some("a"));
        assert_eq!(h.test_resolve_node_style_kind("a"), Some(BoardElementStyleKind::Selected), "node drag should keep primary selected paint at detail LOD");
    }

    #[test]
    fn board_host_drag_emits_node_move() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let w = Point::new(0.0, 0.0);
        let s = h.world_to_screen(w);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false);
        h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("nodeMove"));
    }

    #[test]
    fn board_host_compact_discrete_hit_selects_and_drags_node() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 0.5);
        let mut desc = sample_scene();
        desc.handles.clear();
        desc.edges.clear();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        assert_eq!(h.resolve_hit_world(Point::new(0.0, 0.0)).as_deref(), Some("a"));
        assert!(h.resolve_hit_world(Point::new(150.0, 0.0)).is_none());
        let s = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false);
        h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("nodeMove"), "compact discrete node hit should drag, got: {ev}");
    }

    #[test]
    fn board_host_minimap_bounded_drag_moves_selection_inside_union_bounds() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_automatic_lod(false);
        h.set_forced_draw_lod_label("minimap");
        h.set_camera(0.0, 0.0, 0.1);
        let mut desc = sample_scene();
        desc.handles.clear();
        desc.edges.clear();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        h.set_selection_ids(&["a".into(), "b".into()]);
        let _ = h.drain_events_json();
        let gap = Point::new(150.0, 0.0);
        assert!(h.resolve_hit_world(gap).is_none());
        let s = h.world_to_screen(gap);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 50.0, s.y + 30.0, false, false);
        h.pointer_up_screen(s.x + 50.0, s.y + 30.0, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("nodeMove"), "expected bounded drag nodeMove, got: {ev}");
        let zoom = 0.1;
        let dx = 50.0 / zoom;
        let dy = 30.0 / zoom;
        let a = h.nodes.get("a").unwrap();
        let b = h.nodes.get("b").unwrap();
        assert!((a.x - dx).abs() < 1e-3 && (a.y - dy).abs() < 1e-3);
        assert!((b.x - (300.0 + dx)).abs() < 1e-3 && (b.y - dy).abs() < 1e-3);
    }

    #[test]
    fn board_host_overview_bounded_drag_moves_selection_inside_union_bounds() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_automatic_lod(false);
        h.set_forced_draw_lod_label("overview");
        set_overview_lod(&mut h);
        let mut desc = sample_scene();
        desc.handles.clear();
        desc.edges.clear();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        h.set_selection_ids(&["a".into(), "b".into()]);
        let _ = h.drain_events_json();
        let gap = Point::new(150.0, 0.0);
        assert!(h.resolve_hit_world(gap).is_none());
        let s = h.world_to_screen(gap);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 40.0, s.y + 20.0, false, false);
        h.pointer_up_screen(s.x + 40.0, s.y + 20.0, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("nodeMove"), "expected overview bounded drag, got: {ev}");
    }

    #[test]
    fn board_host_detail_lod_resolves_direct_handle_hit() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let probe = Point::new(hp.x + 2.0, hp.y);
        assert_eq!(h.resolve_hit_world(probe).as_deref(), Some("a:h0"));
    }

    #[test]
    fn board_host_multi_select_drag_moves_each_selected_node() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 100.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        h.set_selection_options("rectangle", "additive", true, true, true);
        h.set_selection_ids(&["a".into(), "b".into()]);
        let _ = h.drain_events_json();
        let w = Point::new(0.0, 0.0);
        let s = h.world_to_screen(w);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_move_screen(s.x + 10.0, s.y + 5.0, false, false);
        h.pointer_up_screen(s.x + 10.0, s.y + 5.0, false, false);
        let a = h.nodes.get("a").expect("node a");
        let b = h.nodes.get("b").expect("node b");
        assert!((a.x - 10.0).abs() < 1e-6);
        assert!((a.y - 5.0).abs() < 1e-6);
        assert!((b.x - 110.0).abs() < 1e-6);
        assert!((b.y - 5.0).abs() < 1e-6);
        let sorted: Vec<_> = h.selection.iter().cloned().collect();
        assert_eq!(sorted, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn board_host_selection_target_edges_skips_node_geometry() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_selection_options("rectangle", "invertive", false, true, false);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let inside_node_a = Point::new(0.0, 0.0);
        assert!(h.resolve_hit_world(inside_node_a).is_none());
        let on_edge = Point::new(150.0, 0.0);
        assert_eq!(h.resolve_hit_world(on_edge).as_deref(), Some("e1"));
    }

    #[test]
    fn board_host_additive_click_merges_edge_into_existing_selection() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_selection_options("rectangle", "additive", true, true, true);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        h.set_selection_ids(&["a".into()]);
        let _ = h.drain_events_json();
        let on_edge = Point::new(150.0, 0.0);
        let s = h.world_to_screen(on_edge);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        let mut got: Vec<_> = h.selection.iter().cloned().collect();
        got.sort();
        assert_eq!(got, vec!["a".to_string(), "e1".to_string()]);
    }

    #[test]
    fn board_host_selection_change_does_not_bump_content_scene_generation() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&sample_scene()).unwrap();
        let gen = h.test_content_scene_generation();
        let neutral_hint = h.encoded_scene_hint();
        h.set_selection_ids_silent(&["a".into()]);
        assert_eq!(h.test_content_scene_generation(), gen);
        assert!(h.encoded_scene_hint() > neutral_hint);
        h.set_selection_ids_silent(&[]);
        assert_eq!(h.test_content_scene_generation(), gen);
        assert_eq!(h.encoded_scene_hint(), neutral_hint);
    }

    #[test]
    fn board_host_background_click_deselect_skips_preselect_events() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        let desc = sample_scene();
        h.sync_descriptor(&desc).unwrap();
        h.set_selection_ids(&["a".into(), "e1".into()]);
        let _ = h.drain_events_json();
        let away = Point::new(5000.0, 5000.0);
        let s = h.world_to_screen(away);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        assert!(!h.is_dragging_area_select());
        h.pointer_move_screen(s.x + 1.0, s.y, false, false);
        let mid = h.drain_events_json();
        assert!(!mid.contains("preselect"), "background click path must not emit preselect");
        assert!(h.preselect_removed.is_empty());
        assert!(h.selection_exit_highlight.is_empty());
        assert!(h.selection.contains("a"));
        h.pointer_up_screen(s.x, s.y, false, false);
        assert!(h.selection.is_empty());
        assert!(h.selection_exit_highlight.is_empty());
        let fin = h.drain_events_json();
        assert!(fin.contains("select"));
        assert!(!fin.contains("preselect"));
        assert!(fin.contains("\"exitHighlightIds\":[]"));
    }

    #[test]
    fn board_host_background_click_without_drag_clears_selection() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        h.set_selection_ids(&["a".into(), "e1".into()]);
        let away = Point::new(5000.0, 5000.0);
        let s = h.world_to_screen(away);
        h.pointer_down_screen(s.x, s.y, 0, false, false);
        h.pointer_up_screen(s.x, s.y, false, false);
        assert!(h.selection.is_empty());
    }

    #[test]
    fn board_host_rectangle_area_select_includes_handles_with_nodes() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_selection_options("rectangle", "invertive", true, true, true);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let w0 = Point::new(-90.0, -70.0);
        let w1 = Point::new(90.0, 90.0);
        let s0 = h.world_to_screen(w0);
        let s1 = h.world_to_screen(w1);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        h.pointer_move_screen(s1.x, s1.y, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false);
        let mut got: Vec<_> = h.selection.iter().cloned().collect();
        got.sort();
        assert!(got.contains(&"a".to_string()));
        assert!(got.contains(&"a:h0".to_string()));
    }

    #[test]
    fn board_host_area_select_preselect_matches_selected_chrome() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let _ = h.drain_events_json();
        assert!(h.preselect_removed.is_empty());
        assert!(h.selection_exit_highlight.is_empty());
        let w_down = Point::new(350.0, -50.0);
        let w_mid = Point::new(270.0, 50.0);
        let w_end = Point::new(265.0, 48.0);
        let s_down = h.world_to_screen(w_down);
        h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
        assert!(!h.is_dragging_area_select());
        let _ = h.drain_events_json();
        let s_mid = h.world_to_screen(w_mid);
        let s_end = h.world_to_screen(w_end);
        h.pointer_move_screen(s_mid.x, s_mid.y, false, false);
        assert!(h.is_dragging_area_select());
        let _ = h.drain_events_json();
        assert!(h.preselect.contains("b"), "preview should include node b");
        assert!(h.preselect_removed.contains("a"));
        assert!(h.selection_exit_highlight.is_empty());
        assert!(!h.selection.contains("b"), "committed selection unchanged during preselect");
        let frozen = h.preselect_removed.clone();
        h.pointer_move_screen(s_end.x, s_end.y, false, false);
        let _ = h.drain_events_json();
        assert_eq!(frozen, h.preselect_removed);
        h.pointer_up_screen(s_end.x, s_end.y, false, false);
        let _ = h.drain_events_json();
        assert!(h.selection.contains("b"));
        assert!(!h.selection.contains("a"));
        assert!(h.preselect_removed.is_empty());
        assert!(h.selection_exit_highlight.is_empty());
    }

    #[test]
    fn board_host_area_select_from_empty_keeps_selection_until_commit() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        h.set_selection_ids(&[]);
        let _ = h.drain_events_json();
        let w_down = Point::new(350.0, -50.0);
        let w_mid = Point::new(270.0, 50.0);
        let s_down = h.world_to_screen(w_down);
        let s_mid = h.world_to_screen(w_mid);
        h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
        h.pointer_move_screen(s_mid.x, s_mid.y, false, false);
        let _ = h.drain_events_json();
        assert!(h.is_dragging_area_select());
        assert!(h.preselect.contains("b"));
        assert!(h.preselect_removed.is_empty());
        assert!(h.selection.is_empty());
        h.pointer_up_screen(s_mid.x, s_mid.y, false, false);
        let _ = h.drain_events_json();
        assert!(h.selection.contains("b"));
        assert!(h.preselect.is_empty());
    }

    #[test]
    fn board_host_minimap_preselect_matches_selected_chrome() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 0.1);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let neutral_hint = h.encoded_scene_hint();
        h.set_selection_ids(&["b".into()]);
        let _ = h.drain_events_json();
        let selected_hint = h.encoded_scene_hint();
        assert!(selected_hint > neutral_hint, "minimap selected chrome should add visible vector encoding over neutral state");
        h.set_selection_ids(&["a".into()]);
        let _ = h.drain_events_json();
        let w_down = Point::new(350.0, -50.0);
        let w_end = Point::new(265.0, 48.0);
        let s_down = h.world_to_screen(w_down);
        let s_end = h.world_to_screen(w_end);
        h.pointer_down_screen(s_down.x, s_down.y, 0, false, false);
        h.pointer_move_screen(s_end.x, s_end.y, false, false);
        assert!(h.is_dragging_area_select());
        assert!(h.preselect.contains("b"));
        h.set_selection_screen_preview(None);
        let preselect_hint = h.encoded_scene_hint();
        assert!(preselect_hint > neutral_hint, "minimap preselect should add visible selected chrome over neutral minimap rendering");
    }

    #[test]
    fn board_host_silent_preselect_applies_selected_chrome_without_area_drag() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 0.1);
        let mut desc = sample_scene();
        desc.nodes.push(NodeDescJson {
            id: "b".into(),
            x: 300.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        h.sync_descriptor(&desc).unwrap();
        let neutral_hint = h.encoded_scene_hint();
        assert!(!matches!(h.interaction, Interaction::Selection { .. }));
        h.set_preselect_state_silent(&["b".into()], &[]);
        assert!(h.nodes.get("b").is_some_and(|n| n.selected));
        assert!(h.nodes.get("a").is_some_and(|n| !n.selected));
        let preselect_hint = h.encoded_scene_hint();
        assert!(preselect_hint > neutral_hint, "silent minimap preselect should paint selected chrome without an active area-select interaction");
    }

    #[test]
    fn board_host_hover_tracks_visible_wires() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let mut desc = sample_scene();
        desc.edges.clear();
        desc.wires.push(WireDescJson { id: "w1".into(), source: "a:h0".into(), target: None, end_x: Some(220.0), end_y: Some(0.0), selected: None, style: None, wire_kind: None, user_data: None, visible: None });
        h.sync_descriptor(&desc).unwrap();
        let source = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let curve = compute_edge_bezier_points(source, Point::new(220.0, 0.0), Point::new(0.0, 0.0), Point::new(220.0, 0.0));
        let probe = cubic_bezier_point(curve, 0.5);
        h.update_hover_from_world(probe);
        assert_eq!(h.hovered_id.as_deref(), Some("w1"));
    }

    fn link_test_scene_no_edge() -> SceneDescriptorJson {
        SceneDescriptorJson {
            nodes: vec![
                NodeDescJson {
                    id: "a".into(),
                    x: 0.0,
                    y: 0.0,
                    draggable: Some(true),
                    selected: None,
                    style: None,
                    text: None,
                    icon_kind: None,
                    node_kind: None,
                    user_data: None,
                    visible: None,
                    root: None,
                    shape: Some("circle".into()),
                    radius: Some(40.0),
                    width: None,
                    height: None,
                    scale: None,
                },
                NodeDescJson {
                    id: "b".into(),
                    x: 280.0,
                    y: 0.0,
                    draggable: Some(true),
                    selected: None,
                    style: None,
                    text: None,
                    icon_kind: None,
                    node_kind: None,
                    user_data: None,
                    visible: None,
                    root: None,
                    shape: Some("circle".into()),
                    radius: Some(40.0),
                    width: None,
                    height: None,
                    scale: None,
                },
            ],
            handles: vec![
                HandleDescJson { id: "a:h0".into(), node_id: "a".into(), angle: 0.0, radius: None, selected: None, style: None, handle_kind: Some("parent".into()), color: None, icon_kind: None, user_data: None, visible: None, scale: None },
                HandleDescJson {
                    id: "b:h0".into(),
                    node_id: "b".into(),
                    angle: std::f64::consts::PI,
                    radius: None,
                    selected: None,
                    style: None,
                    handle_kind: Some("child".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                    visible: None,
                    scale: None,
                },
            ],
            edges: vec![],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        }
    }

    fn link_test_scene_no_edge_non_draggable_nodes() -> SceneDescriptorJson {
        let mut s = link_test_scene_no_edge();
        for n in &mut s.nodes {
            n.draggable = Some(false);
        }
        s
    }

    fn link_test_scene_node_a_two_free_handles() -> SceneDescriptorJson {
        let mut s = link_test_scene_no_edge();
        s.handles.push(HandleDescJson {
            id: "a:h1".into(),
            node_id: "a".into(),
            angle: std::f64::consts::FRAC_PI_2,
            radius: None,
            selected: None,
            style: None,
            handle_kind: Some("parent".into()),
            color: None,
            icon_kind: None,
            user_data: None,
            visible: None,
            scale: None,
        });
        s
    }

    fn link_test_scene_b_two_free_child_handles() -> SceneDescriptorJson {
        let mut s = link_test_scene_no_edge();
        s.handles.push(HandleDescJson { id: "b:h1".into(), node_id: "b".into(), angle: 0.0, radius: None, selected: None, style: None, handle_kind: Some("child".into()), color: None, icon_kind: None, user_data: None, visible: None, scale: None });
        s
    }

    fn link_test_scene_target_b_handle_busy() -> SceneDescriptorJson {
        let mut s = link_test_scene_no_edge();
        s.nodes.push(NodeDescJson {
            id: "c".into(),
            x: 560.0,
            y: 0.0,
            draggable: Some(true),
            selected: None,
            style: None,
            text: None,
            icon_kind: None,
            node_kind: None,
            user_data: None,
            visible: None,
            root: None,
            shape: Some("circle".into()),
            radius: Some(40.0),
            width: None,
            height: None,
            scale: None,
        });
        s.handles.push(HandleDescJson {
            id: "c:h0".into(),
            node_id: "c".into(),
            angle: std::f64::consts::PI,
            radius: None,
            selected: None,
            style: None,
            handle_kind: Some("child".into()),
            color: None,
            icon_kind: None,
            user_data: None,
            visible: None,
            scale: None,
        });
        s.edges.push(EdgeDescJson { id: "e-bc".into(), source: "b:h0".into(), target: "c:h0".into(), edge_kind: None,
                source_tip: None,
                target_tip: None,
                selected: None,
                style: None,
                user_data: None,
                visible: None });
        s
    }

    fn link_test_scene_a_to_b_linked() -> SceneDescriptorJson {
        let mut s = link_test_scene_no_edge();
        s.edges.push(EdgeDescJson { id: "e-ab".into(), source: "a:h0".into(), target: "b:h0".into(), edge_kind: None,
                source_tip: None,
                target_tip: None,
                selected: None,
                style: None,
                user_data: None,
                visible: None });
        s
    }

    fn link_test_scene_node_a_two_handles_one_busy() -> SceneDescriptorJson {
        let mut s = link_test_scene_a_to_b_linked();
        s.handles.push(HandleDescJson {
            id: "a:h1".into(),
            node_id: "a".into(),
            angle: std::f64::consts::FRAC_PI_2,
            radius: None,
            selected: None,
            style: None,
            handle_kind: Some("parent".into()),
            color: None,
            icon_kind: None,
            user_data: None,
            visible: None,
            scale: None,
        });
        s
    }

    #[test]
    fn board_host_node_drag_proximity_connect_overlapping_compatible_handles() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let center_b = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_down_screen(center_b.x, center_b.y, 0, false, false);
        let overlap = h.world_to_screen(Point::new(60.0, 0.0));
        h.pointer_move_screen(overlap.x, overlap.y, false, false);
        assert!(matches!(h.interaction, Interaction::DragNodes { proximity_pair: Some(_), .. }), "expected proximity preview wire while overlapping compatible nodes");
        h.pointer_up_screen(overlap.x, overlap.y, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"), "expected edgeCreate, got: {ev}");
        assert!(ev.contains("proximityConnect"), "expected proximityConnect, got: {ev}");
        assert!(ev.contains("b:h0"));
        assert!(ev.contains("a:h0"));
    }

    #[test]
    fn board_host_node_drag_skips_proximity_when_moving_node_has_incident_edge() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_a_to_b_linked()).unwrap();
        let _ = h.drain_events_json();
        let center_b = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_down_screen(center_b.x, center_b.y, 0, false, false);
        let overlap = h.world_to_screen(Point::new(60.0, 0.0));
        h.pointer_move_screen(overlap.x, overlap.y, false, false);
        assert!(matches!(h.interaction, Interaction::DragNodes { proximity_pair: None, .. }), "connected moving node must not preview node-drag proximity");
        h.pointer_up_screen(overlap.x, overlap.y, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("proximityConnect"), "expected no proximityConnect, got: {ev}");
    }

    #[test]
    fn board_host_link_drag_snap_emits_edge_create() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("proximityConnect"));
        assert!(ev.contains("a:h0"));
        assert!(ev.contains("b:h0"));
        let created: Vec<_> = h.edges.keys().filter(|k| k.starts_with("edge-link-")).cloned().collect();
        assert_eq!(created.len(), 1);
    }

    #[test]
    fn board_host_link_drag_snap_micro_zoom_rectangle_compatible_handles() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_micro_lod(&mut h);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id":"core.rect.bottom","name":"B","color":"#112233","defaultWireKind":"link.w"},
                    {"id":"core.rect.top","name":"T","color":"#112233","defaultWireKind":"link.w"}
                ],
                "wireKinds": [{"id":"link.w","name":"W","defaultEdgeKind":"link.e"}],
            })
            .to_string(),
        )
        .unwrap();
        h.set_handle_link_compat_from_json(r#"[{"source":"core.rect.bottom","target":"core.rect.top","specificity":"handle"}]"#).unwrap();
        let desc = SceneDescriptorJson {
            nodes: vec![
                NodeDescJson {
                    id: "a".into(),
                    x: 0.0,
                    y: 100.0,
                    draggable: Some(true),
                    selected: None,
                    style: None,
                    text: None,
                    icon_kind: None,
                    node_kind: None,
                    user_data: None,
                    visible: None,
                    root: None,
                    shape: Some("rectangle".into()),
                    radius: None,
                    width: Some(100.0),
                    height: Some(56.0),
                    scale: None,
                },
                NodeDescJson {
                    id: "b".into(),
                    x: 0.0,
                    y: 20.0,
                    draggable: Some(true),
                    selected: None,
                    style: None,
                    text: None,
                    icon_kind: None,
                    node_kind: None,
                    user_data: None,
                    visible: None,
                    root: None,
                    shape: Some("rectangle".into()),
                    radius: None,
                    width: Some(100.0),
                    height: Some(56.0),
                    scale: None,
                },
            ],
            handles: vec![
                HandleDescJson {
                    id: "a:h0".into(),
                    node_id: "a".into(),
                    angle: std::f64::consts::PI,
                    radius: None,
                    selected: None,
                    style: None,
                    handle_kind: Some("core.rect.bottom".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                    visible: None,
                    scale: None,
                },
                HandleDescJson { id: "b:h0".into(), node_id: "b".into(), angle: 0.0, radius: None, selected: None, style: None, handle_kind: Some("core.rect.top".into()), color: None, icon_kind: None, user_data: None, visible: None, scale: None },
            ],
            edges: vec![],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let pa = handle_position_on_rectangle(Point::new(0.0, 100.0), 100.0, 56.0, std::f64::consts::PI);
        let pb = handle_position_on_rectangle(Point::new(0.0, 20.0), 100.0, 56.0, 0.0);
        let s0 = h.world_to_screen(pa);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let mid = Point::new(0.0, 60.0);
        let s_mid = h.world_to_screen(mid);
        h.pointer_move_screen(s_mid.x, s_mid.y, false, false);
        let s1 = h.world_to_screen(pb);
        h.pointer_move_screen(s1.x, s1.y, false, false);
        assert!(matches!(h.interaction, Interaction::LinkDragSnap { ref target_id, .. } if target_id.as_deref() == Some("b:h0")), "expected drag snap onto b:h0 at micro zoom");
        h.pointer_up_screen(s1.x, s1.y, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"), "expected edgeCreate, got: {ev}");
        assert!(ev.contains("proximityConnect"), "expected proximityConnect, got: {ev}");
    }

    #[test]
    fn board_host_link_drag_snap_proximity_connect_in_overview_lod() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_overview_lod(&mut h);
        h.sync_descriptor(&link_test_scene_no_edge_non_draggable_nodes()).unwrap();
        let _ = h.drain_events_json();
        let center_a = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(center_a.x, center_a.y, 0, false, false);
        h.pointer_up_screen(center_a.x, center_a.y, false, false);
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"), "expected edgeCreate at overview LOD, got: {ev}");
        assert!(ev.contains("proximityConnect") || ev.contains("indirectConnect"), "expected proximityConnect or indirectConnect, got: {ev}");
    }

    #[test]
    fn board_host_parses_mindmap_fixture_without_handles() {
        let mut h = BoardHost::new_normal();
        let fixture = json!({
            "schema": "reasoning.mindmap.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "x": 0.0, "y": 0.0, "width": 48.0, "height": 48.0, "shape": "rectangle", "root": true },
                { "id": "b", "x": 120.0, "y": 0.0, "width": 40.0, "height": 40.0, "shape": "rectangle" }
            ],
            "edges": [
                { "id": "e1", "source": "a", "target": "b", "edgeKind": "wires.owns" }
            ]
        });
        assert!(h.parse_fixture_v1(&fixture));
        assert_eq!(h.port_mode, GraphPortMode::Normal);
        assert!(h.handles.is_empty());
        assert_eq!(h.edges.len(), 1);
        assert_eq!(h.edges.get("e1").unwrap().source, "a");
        assert_eq!(h.edges.get("e1").unwrap().target, "b");
        h.set_size(800, 600, 1.0);
        let scene = h.build_vector_scene();
        assert!(scene.encoding().path_tags.len() > 0);
    }

    #[test]
    fn board_host_ingests_edge_and_node_kind_catalog_visual_fields() {
        let mut h = BoardHost::new_normal();
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "edgeKinds": [
                    {"id":"wires.owns","name":"Owns","color":"#ff0000","stroke":"3","pattern":"dashed","targetTip":"filled-diamond","directed":false},
                    {"id":"wires.is","name":"Is","color":"#00ff00","pattern":"dotted","targetTip":"filled-arrow","directed":false}
                ],
                "nodeKinds": [
                    {"id":"capsule","name":"Capsule","shape":"circle","color":"#aabbcc"}
                ]
            })
            .to_string(),
        )
        .unwrap();
        let owns = h.edge_kinds.get("wires.owns").expect("owns edge kind");
        assert_eq!(owns.stroke_width, 3.0);
        assert_eq!(owns.pattern, EdgeStrokePattern::Dashed);
        assert_eq!(owns.target_tip.as_deref(), Some("filled-diamond"));
        assert!(!owns.directed);
        assert!(owns.color.is_some());
        let is = h.edge_kinds.get("wires.is").expect("is edge kind");
        assert_eq!(is.pattern, EdgeStrokePattern::Dotted);
        assert_eq!(is.target_tip.as_deref(), Some("filled-arrow"));
        assert!(!is.directed);
        let diamond = h.edge_tips.get("filled-diamond").expect("filled-diamond tip");
        assert_eq!(diamond.geometry, EdgeTipGeometry::Diamond);
        assert!(diamond.filled);
        let capsule = h.node_kinds.get("capsule").expect("capsule node kind");
        assert_eq!(capsule.shape, NodeShape::Circle);
        assert!(capsule.color_fill.is_some());
    }

    #[test]
    fn board_host_sync_descriptor_normal_graph_node_id_edges() {
        let mut h = BoardHost::new_normal();
        let desc = SceneDescriptorJson {
            nodes: vec![
                NodeDescJson {
                    id: "a".into(),
                    x: 0.0,
                    y: 0.0,
                    draggable: Some(true),
                    selected: None,
                    style: None,
                    text: None,
                    icon_kind: None,
                    node_kind: None,
                    user_data: None,
                    visible: None,
                    root: Some(true),
                    shape: Some("rectangle".into()),
                    radius: None,
                    width: Some(48.0),
                    height: Some(48.0),
                    scale: None,
                },
                NodeDescJson {
                    id: "b".into(),
                    x: 120.0,
                    y: 0.0,
                    draggable: Some(true),
                    selected: None,
                    style: None,
                    text: None,
                    icon_kind: None,
                    node_kind: None,
                    user_data: None,
                    visible: None,
                    root: None,
                    shape: Some("rectangle".into()),
                    radius: None,
                    width: Some(40.0),
                    height: Some(40.0),
                    scale: None,
                },
            ],
            handles: vec![],
            edges: vec![EdgeDescJson {
                id: "e1".into(),
                source: "a".into(),
                target: "b".into(),
                edge_kind: Some("wires.owns".into()),
                source_tip: None,
                target_tip: None,
                selected: None,
                style: None,
                user_data: None,
                visible: None,
            }],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        assert!(h.handles.is_empty());
        assert_eq!(h.edges.get("e1").unwrap().source, "a");
        assert_eq!(h.edges.get("e1").unwrap().target, "b");
        h.set_size(800, 600, 1.0);
        let scene = h.build_vector_scene();
        assert!(scene.encoding().path_tags.len() > 0);
    }

    #[test]
    fn board_host_hidden_handle_blocks_proximity_connect() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 280.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port", "hidden": true }]
                }
            ],
            "edges": []
        });
        assert!(h.parse_fixture_v1(&fixture));
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"), "hidden handle should block connect, got: {ev}");
        assert!(h.edges.is_empty());
    }

    #[test]
    fn board_host_hidden_node_blocks_indirect_connect() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "parent" }]
                },
                {
                    "id": "b",
                    "x": 280.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "hidden": true,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "child" }]
                }
            ],
            "edges": []
        });
        assert!(h.parse_fixture_v1(&fixture));
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let inside_a = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(inside_a.x, inside_a.y, 0, false, false);
        let inside_b = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(inside_b.x, inside_b.y, false, false);
        h.pointer_up_screen(inside_b.x, inside_b.y, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"), "hidden node should block indirect connect, got: {ev}");
        assert!(matches!(h.interaction, Interaction::None));
        assert!(h.edges.is_empty());
    }

    #[test]
    fn board_host_overview_lod_omits_direct_handle_resolve_hit() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_overview_lod(&mut h);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let probe = Point::new(hp.x + 3.0, hp.y);
        assert_ne!(h.resolve_hit_world(probe).as_deref(), Some("a:h0"));
    }

    #[test]
    fn board_host_link_rejects_incompatible_handle_kind_pairs() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_handle_link_compat_from_json(r#"[{"source":"child","target":"parent"}]"#).unwrap();
        let desc = link_test_scene_no_edge();
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"));
    }

    #[test]
    fn board_host_link_accepts_matching_handle_kind_pair() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        let desc = link_test_scene_no_edge();
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("proximityConnect"));
    }

    #[test]
    fn board_host_normal_lod_prefers_node_at_center_and_handle_off_rim() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        assert_eq!(h.resolve_hit_world(Point::new(0.0, 0.0)).as_deref(), Some("a"));
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let probe = Point::new(hp.x + 2.0, hp.y);
        assert_eq!(h.resolve_hit_world(probe).as_deref(), Some("a:h0"));
    }

    #[test]
    fn board_host_indirect_ring_resolve_skips_connected_handles() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_node_a_two_handles_one_busy()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let ha0 = h.handles.get("a:h0").unwrap();
        let ring_busy = h.indirect_handle_world_pos(ha0).unwrap();
        assert_ne!(h.resolve_hit_world(ring_busy).as_deref(), Some("a:h0"));
        assert_eq!(h.resolve_hit_world(Point::new(0.0, 0.0)).as_deref(), Some("a:h1"));
    }

    #[test]
    fn board_host_indirect_sole_compatible_drop_creates_edge_immediately() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let inside_a = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(inside_a.x, inside_a.y, 0, false, false);
        assert!(matches!(
            h.interaction,
            Interaction::LinkAtSourceHandle { ref source_id, .. } if source_id == "a:h0"
        ));
        let inside_b = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(inside_b.x, inside_b.y, false, false);
        assert!(matches!(h.interaction, Interaction::LinkDragSnap { .. }));
        h.pointer_up_screen(inside_b.x, inside_b.y, false, false);
        assert!(matches!(h.interaction, Interaction::None));
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("indirectConnect"));
        assert!(ev.contains("a:h0"));
        assert!(ev.contains("b:h0"));
    }

    #[test]
    fn board_host_indirect_two_compatible_child_handles_on_target_require_ring_pick() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_b_two_free_child_handles()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let sa = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(sa.x, sa.y, 0, false, false);
        let sb = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(sb.x, sb.y, false, false);
        h.pointer_up_screen(sb.x, sb.y, false, false);
        assert!(matches!(
            h.interaction,
            Interaction::LinkTargetNode { ref target_node_id, .. } if target_node_id == "b"
        ));
        let b0 = h.handles.get("b:h0").unwrap();
        let ring0 = h.indirect_handle_world_pos(b0).unwrap();
        let s1 = h.world_to_screen(ring0);
        h.pointer_down_screen(s1.x, s1.y, 0, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("indirectConnect"));
        assert!(ev.contains("a:h0"));
        assert!(ev.contains("b:h0"));
    }

    #[test]
    fn board_host_indirect_target_click_elsewhere_stops_wire() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_b_two_free_child_handles()).unwrap();
        h.set_selection_ids(&["a".into()]);
        let sa = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(sa.x, sa.y, 0, false, false);
        let target_center = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(target_center.x, target_center.y, false, false);
        h.pointer_up_screen(target_center.x, target_center.y, false, false);
        assert!(matches!(h.interaction, Interaction::LinkTargetNode { .. }));
        h.pointer_down_screen(20.0, 20.0, 0, false, false);
        assert!(matches!(h.interaction, Interaction::None));
        assert!(h.edges.is_empty());
    }

    #[test]
    fn board_host_indirect_ring_shown_when_node_has_two_free_handles() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_node_a_two_free_handles()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let ha0 = h.handles.get("a:h0").unwrap();
        let ring = h.indirect_handle_world_pos(ha0).unwrap();
        assert_eq!(h.resolve_hit_world(ring).as_deref(), Some("a:h0"));
    }

    #[test]
    fn board_host_indirect_ring_paints_without_rebuilding_world_cache() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_node_a_two_free_handles()).unwrap();
        let gen = h.test_content_scene_generation();
        let neutral_hint = h.encoded_scene_hint();
        h.set_selection_ids_silent(&["a".into()]);
        assert_eq!(h.test_content_scene_generation(), gen);
        let ha0 = h.handles.get("a:h0").unwrap();
        let ring = h.indirect_handle_world_pos(ha0).unwrap();
        assert_eq!(h.resolve_hit_world(ring).as_deref(), Some("a:h0"));
        assert!(h.encoded_scene_hint() > neutral_hint, "indirect ring must paint in the live overlay, not only in stale cached geometry");
        h.set_selection_ids_silent(&[]);
        assert_eq!(h.encoded_scene_hint(), neutral_hint);
    }

    #[test]
    fn board_host_link_drag_emits_compatible_nodes_and_target_ring_events() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_b_two_free_child_handles()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let sa = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(sa.x, sa.y, 0, false, false);
        let sb = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(sb.x, sb.y, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("linkCompatibleNodes"), "got: {ev}");
        assert!(ev.contains(r#""nodeIds":["b"]"#) || ev.contains(r#""nodeIds": ["b"]"#), "got: {ev}");
        assert!(ev.contains("linkTargetRing"), "got: {ev}");
        assert!(ev.contains("b:h0") && ev.contains("b:h1"), "got: {ev}");
        let ring = h.indirect_handle_world_pos(h.handles.get("b:h1").unwrap()).unwrap();
        assert_eq!(h.resolve_hit_world(ring).as_deref(), Some("b:h1"));
        h.pointer_up_screen(20.0, 20.0, false, false);
        let ev_end = h.drain_events_json();
        assert!(ev_end.contains("linkCompatibleNodes"));
        assert!(ev_end.contains(r#""nodeIds":[]"#) || ev_end.contains(r#""nodeIds": []"#));
        assert!(ev_end.contains("linkTargetRing"));
    }

    #[test]
    fn board_host_indirect_ring_gap_scales_with_node_across_zoom() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let ha = h.handles.get("a:h0").unwrap().clone();
        let node_r = 40.0_f64;
        let body = || handle_position_on_circle(Point::new(0.0, 0.0), node_r, 0.0);
        let gap_ratio = |host: &BoardHost| {
            let ring = host.indirect_handle_world_pos(&ha).unwrap();
            let gap_px = distance_between(host.world_to_screen(ring), host.world_to_screen(body()));
            gap_px / (node_r * host.camera.zoom)
        };
        h.set_camera(0.0, 0.0, 1.0);
        let ratio_z1 = gap_ratio(&h);
        let gap_px_z1 = node_r * ratio_z1;
        h.set_camera(0.0, 0.0, 4.25);
        let ratio_z2 = gap_ratio(&h);
        let gap_px_z2 = node_r * 4.25 * ratio_z2;
        assert!((ratio_z1 - ratio_z2).abs() < 1e-6, "rim-to-ring ratios differ: {ratio_z1} vs {ratio_z2}");
        assert!((ratio_z1 - 0.7).abs() < 1e-6);
        assert!((gap_px_z2 - gap_px_z1 * 4.25).abs() < 0.6, "screen gap should scale with zoom: {gap_px_z1} vs {gap_px_z2}");
    }

    #[test]
    fn board_host_indirect_handle_marker_radius_scales_with_node_extent() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let ha = h.handles.get("a:h0").unwrap();
        assert!((h.indirect_handle_marker_radius_world(ha) - 32.0).abs() < 1e-6);
    }

    #[test]
    fn board_host_handle_scale_combines_node_and_kind_scales() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [{"id":"slot-a","name":"Slot A","color":"#112233","scale":2.0}],
                "nodeKinds": [{"id":"kind-a","name":"Kind A","scale":1.5}],
            })
            .to_string(),
        )
        .unwrap();
        let mut desc = link_test_scene_no_edge();
        desc.nodes[0].node_kind = Some("kind-a".into());
        desc.nodes[0].scale = Some(2.0);
        desc.handles[0].handle_kind = Some("slot-a".into());
        desc.handles[0].scale = Some(0.5);
        h.sync_descriptor(&desc).unwrap();
        let ha = h.handles.get("a:h0").unwrap();
        assert_eq!(h.resolve_hit_world(Point::new(120.0, 0.0)).as_deref(), Some("a:h0"));
        assert!((h.indirect_handle_marker_radius_world(ha) - 96.0).abs() < 1e-6);
    }

    #[test]
    fn board_host_link_wire_specificity_allows_when_handle_row_absent() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [{"id":"parent","name":"P","color":"#112233","defaultWireKind":"flow.wire"}],
                "wireKinds": [{"id":"flow.wire","name":"W","defaultEdgeKind":"flow.edge"}],
            })
            .to_string(),
        )
        .unwrap();
        h.set_handle_link_compat_from_json(r#"[{"source":"flow.wire","target":"child","specificity":"wire"}]"#).unwrap();
        let desc = link_test_scene_no_edge();
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("proximityConnect"));
    }

    #[test]
    fn board_host_kind_catalog_accepts_modern_hsl_handle_colors() {
        let mut h = BoardHost::new();
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id":"space","name":"S","color":"hsl(206 52% 48%)"},
                    {"id":"comma","name":"C","color":"hsl(206, 52%, 48%)"},
                    {"id":"slash","name":"Sl","color":"hsl(206 52% 48% / 0.5)"},
                ],
            })
            .to_string(),
        )
        .unwrap();
        let c_space = h.handle_kinds.get("space").expect("space").color;
        let c_comma = h.handle_kinds.get("comma").expect("comma").color;
        let c_slash = h.handle_kinds.get("slash").expect("slash").color;
        assert_eq!(c_space, c_comma);
        assert_ne!(c_space, c_slash);
    }

    #[test]
    fn board_host_rejects_kind_catalog_rows_with_legacy_label() {
        let mut h = BoardHost::new();
        let err = h.set_board_kind_catalogs_from_json(&serde_json::json!({"handleKinds":[{"id":"h","label":"legacy","color":"#112233"}]}).to_string()).unwrap_err();
        assert!(err.contains("legacy label"));
    }

    #[test]
    fn board_host_link_important_pair_overrides_lower_specificity_filter() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [{"id":"parent","name":"P","color":"#112233","defaultWireKind":"flow.wire"}],
                "wireKinds": [{"id":"flow.wire","name":"W"}],
            })
            .to_string(),
        )
        .unwrap();
        h.set_handle_link_compat_from_json(
            r#"[
				{"source":"flow.wire","target":"nope","specificity":"wire"},
				{"source":"parent","target":"child","specificity":"general","important":true}
			]"#,
        )
        .unwrap();
        let desc = link_test_scene_no_edge();
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("edgeCreate"));
        assert!(ev.contains("proximityConnect"));
    }

    #[test]
    fn board_host_link_drag_does_not_snap_when_target_handle_busy() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_target_b_handle_busy()).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let hp_b = handle_position_on_circle(Point::new(280.0, 0.0), 40.0, std::f64::consts::PI);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        let s_mid = h.world_to_screen(Point::new(140.0, 0.0));
        h.pointer_move_screen(s_mid.x + 20.0, s_mid.y, false, false);
        let s1 = h.world_to_screen(hp_b);
        h.pointer_move_screen(s1.x, s1.y, false, false);
        h.pointer_up_screen(s1.x, s1.y, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"));
        assert_eq!(h.edges.len(), 1);
        assert!(h.edges.contains_key("e-bc"));
    }

    #[test]
    fn board_host_link_does_not_start_from_busy_source_handle() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&link_test_scene_a_to_b_linked()).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        assert!(matches!(h.interaction, Interaction::None));
        assert!(!h.drain_events_json().contains("edgeCreate"));
    }

    #[test]
    fn board_host_indirect_does_not_commit_on_busy_target_handle() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.sync_descriptor(&link_test_scene_target_b_handle_busy()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let sa = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_down_screen(sa.x, sa.y, 0, false, false);
        let target_center = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_move_screen(target_center.x, target_center.y, false, false);
        h.pointer_up_screen(target_center.x, target_center.y, false, false);
        assert!(matches!(
            h.interaction,
            Interaction::LinkTargetNode {
                ref source_id,
                ref target_node_id
            } if source_id == "a:h0" && target_node_id == "b"
        ));
        let _ = h.drain_events_json();
        let sb = h.world_to_screen(Point::new(280.0, 0.0));
        h.pointer_down_screen(sb.x, sb.y, 0, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"));
        assert_eq!(h.edges.len(), 1);
        assert!(matches!(h.interaction, Interaction::None));
    }

    #[test]
    fn board_host_link_short_drag_does_not_emit_edge_create() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_detail_lod(&mut h);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let hp_a = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let s0 = h.world_to_screen(hp_a);
        h.pointer_down_screen(s0.x, s0.y, 0, false, false);
        h.pointer_move_screen(s0.x + 2.0, s0.y, false, false);
        h.pointer_up_screen(s0.x + 2.0, s0.y, false, false);
        let ev = h.drain_events_json();
        assert!(!ev.contains("edgeCreate"));
    }

    #[test]
    fn board_host_brush_slot_emits_preview_and_place_on_leave() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_active_tool("brush");
        h.set_brush_flush_distance(40.0);
        h.set_brush_node_size(40.0);
        let catalogs = json!({
            "handleKinds": [{ "id": "port", "name": "Port", "color": "#888" }],
            "nodeKinds": [{
                "id": "brush.kind",
                "name": "Brush Kind",
                "handles": [{ "handleKind": "port", "angle": 3.141592653589793 }]
            }]
        });
        h.set_board_kind_catalogs_from_json(&catalogs.to_string()).unwrap();
        let desc = SceneDescriptorJson {
            nodes: vec![NodeDescJson {
                id: "a".into(),
                x: 0.0,
                y: 0.0,
                draggable: Some(true),
                selected: None,
                style: None,
                text: None,
                icon_kind: None,
                node_kind: Some("a.kind".into()),
                user_data: None,
                visible: None,
                root: None,
                shape: Some("circle".into()),
                radius: Some(40.0),
                width: None,
                height: None,
                scale: None,
            }],
            handles: vec![HandleDescJson {
                id: "a:h0".into(),
                node_id: "a".into(),
                angle: 0.0,
                radius: None,
                scale: None,
                selected: None,
                visible: None,
                style: None,
                handle_kind: Some("port".into()),
                color: None,
                icon_kind: None,
                user_data: None,
            }],
            edges: vec![],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let slot = hp + (hp - Point::new(0.0, 0.0)) * (40.0 / 40.0);
        let s = h.world_to_screen(slot);
        h.pointer_move_screen(s.x, s.y, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushPreview"), "expected brushPreview, got: {ev}");
        h.pointer_leave_screen();
        let ev2 = h.drain_events_json();
        assert!(ev2.contains("brushPlace"), "expected brushPlace on leave, got: {ev2}");
        assert!(ev2.contains("brush.kind"));
        assert!(ev2.contains("a:h0"));
        assert!(ev2.contains("nodeId"));
        assert!(ev2.contains("edgeId"));
    }

    #[test]
    fn board_host_brush_slot_commit_survives_pointer_move_out_of_slot() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_active_tool("brush");
        h.set_brush_flush_distance(40.0);
        h.set_brush_node_size(40.0);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [{ "handleKind": "child", "angle": 3.141592653589793 }]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let inside = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(inside.x, inside.y, false, false);
        let _ = h.drain_events_json();
        assert_eq!(h.nodes.len(), 2);
        let far = h.world_to_screen(Point::new(500.0, 500.0));
        h.pointer_move_screen(far.x, far.y, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushPlace"), "expected brushPlace when leaving slot, got: {ev}");
    }

    #[test]
    fn board_host_brush_fill_frontier_deterministic_and_collision_limited() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_brush_flush_distance(40.0);
        h.set_brush_node_size(40.0);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [
                        { "handleKind": "child", "angle": 0.0 },
                        { "handleKind": "child", "angle": 3.141592653589793 }
                    ]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let first = h.brush_fill_json(3, 42);
        let second = h.brush_fill_json(3, 42);
        assert_eq!(first, second, "fill must be deterministic for the same seed");
        let v: serde_json::Value = serde_json::from_str(&first).unwrap();
        let placements = v.get("placements").and_then(|x| x.as_array()).unwrap();
        assert!(!placements.is_empty(), "expected at least one fill placement");
        assert!(placements.len() <= 3);
        let many = h.brush_fill_json(1000, 99);
        let many_v: serde_json::Value = serde_json::from_str(&many).unwrap();
        let many_n = many_v.get("placements").and_then(|x| x.as_array()).map(|a| a.len()).unwrap_or(0);
        assert!(many_n < 1000, "collision should cap fill before 1000 on a tight scene");
    }

    #[test]
    fn board_host_fixture_drop_preview_json_paints_while_select_tool_active() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_active_tool("select");
        h.set_fixture_drop_preview_json(
            r#"{"nodeKind":"capsule_J","screenX":200.0,"screenY":150.0,"shape":"circle","radius":20.0,"iconKind":"capsule_J"}"#,
        )
        .unwrap();
        let ev = h.drain_events_json();
        assert!(!ev.contains("brushPlace"));
        assert!(h.encoded_scene_hint() > 0);
        h.set_fixture_drop_preview_json("").unwrap();
        assert!(h.encoded_scene_hint() > 0);
    }

    #[test]
    fn board_host_fixture_drop_preview_uses_catalog_shape_and_icon_at_overview_lod() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 0.05);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "nodeKinds": [{
                    "id": "capsule_J",
                    "name": "Capsule J",
                    "scale": 2.0,
                    "shape": "circle",
                    "icon": "capsule_J",
                    "handles": [{"handleKind": "door", "angle": 0.0}]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.set_fixture_drop_preview_json(
            r#"{"nodeKind":"capsule_J","screenX":120.0,"screenY":90.0,"shape":"circle","radius":10.0,"iconKind":"capsule_J"}"#,
        )
        .unwrap();
        let hint_with_preview = h.encoded_scene_hint();
        assert!(hint_with_preview > 0);
        h.set_fixture_drop_preview_json("").unwrap();
        let hint_cleared = h.encoded_scene_hint();
        assert!(hint_cleared != hint_with_preview || hint_with_preview > 0);
    }

    #[test]
    fn board_host_brush_session_mirror_json_shows_preview_without_pointer() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_active_tool("brush");
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [{"id": "parent", "name": "Parent", "color": "#888888"}],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [{"handleKind": "parent", "angle": 3.141592653589793}]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let session = serde_json::json!({
            "sourceHandleId": "a:h0",
            "candidates": ["brush.kind"],
            "index": 0,
            "preview": {
                "node": {
                    "nodeKind": "brush.kind",
                    "x": 120.0,
                    "y": 0.0,
                    "shape": "circle",
                    "radius": 20.0,
                    "handles": [{"handleKind": "parent", "angle": 3.141592653589793}]
                },
                "edge": { "sourceHandleId": "a:h0", "targetHandleIndex": 0 }
            }
        });
        h.set_brush_session_mirror_json(&session.to_string()).unwrap();
        let ev = h.drain_events_json();
        assert!(!ev.contains("brushPlace"));
        assert!(h.encoded_scene_hint() > 0);
    }

    #[test]
    fn board_host_brush_candidates_ordered_by_node_kind_weights() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_active_tool("brush");
        h.set_brush_flush_distance(40.0);
        h.set_brush_node_size(40.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [
                    {
                        "id": "light",
                        "name": "Light",
                        "handles": [{"handleKind": "child", "angle": 3.141592653589793}]
                    },
                    {
                        "id": "heavy",
                        "name": "Heavy",
                        "handles": [{"handleKind": "child", "angle": 3.141592653589793}]
                    }
                ]
            })
            .to_string(),
        )
        .unwrap();
        h.set_brush_kind_weights(r#"{"nodeWeights":{"heavy":0.99,"light":0.01},"handleWeights":{}}"#);
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let inside = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(inside.x, inside.y, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushCandidates"), "expected brushCandidates, got: {ev}");
        let v: serde_json::Value = serde_json::from_str(&ev).unwrap();
        let first = v.as_array().and_then(|rows| {
            rows.iter()
                .find(|row| row.get("name").and_then(|n| n.as_str()) == Some("brushCandidates"))
                .and_then(|row| row.get("payload"))
                .and_then(|p| p.get("candidates"))
                .and_then(|c| c.as_array())
                .and_then(|c| c.first())
                .and_then(|x| x.as_str())
        });
        assert_eq!(first, Some("heavy"));
    }

    #[test]
    fn board_host_fill_base_core_rectangular_excludes_cylindric_tambour() {
        const BASE_KIND: &str = "Base";
        const CYLINDRIC_TAMBOUR_KIND: &str = "Cylindric Tambour";
        const FIRST_STOREY_KIND: &str = "First Storey Tambour";
        let mut h = BoardHost::new();
        h.set_brush_flush_distance(80.0);
        h.set_brush_node_size(40.0);
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../fixture/nakagin-capsule-tower.2d.json")).unwrap();
        let compat_str = fixture
            .get("meta")
            .and_then(|m| m.get("kindCompatibility"))
            .map(|v| v.to_string())
            .unwrap_or_else(|| "[]".to_string());
        h.set_handle_link_compat_from_json(&compat_str).unwrap();
        let catalogs_str = fixture
            .get("meta")
            .and_then(|m| m.get("kindCatalogs"))
            .map(|kc| {
                serde_json::json!({
                    "handleKinds": kc.get("handles"),
                    "nodeKinds": kc.get("nodes"),
                })
                .to_string()
            })
            .unwrap_or_else(|| "{}".to_string());
        h.set_board_kind_catalogs_from_json(&catalogs_str).unwrap();
        let desc = SceneDescriptorJson {
            nodes: vec![NodeDescJson {
                id: "base".into(),
                x: 0.0,
                y: 0.0,
                draggable: Some(true),
                selected: None,
                style: None,
                text: None,
                icon_kind: Some("base".into()),
                node_kind: Some(BASE_KIND.into()),
                user_data: None,
                visible: None,
                root: None,
                shape: Some("circle".into()),
                radius: Some(20.0),
                width: None,
                height: None,
                scale: None,
            }],
            handles: vec![
                HandleDescJson {
                    id: "base:c0".into(),
                    node_id: "base".into(),
                    angle: -2.3561944901923453,
                    radius: Some(3.0),
                    scale: None,
                    selected: None,
                    visible: None,
                    style: None,
                    handle_kind: Some("core rectangular bottom".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                },
                HandleDescJson {
                    id: "base:c1".into(),
                    node_id: "base".into(),
                    angle: -0.7853981633974483,
                    radius: Some(3.0),
                    scale: None,
                    selected: None,
                    visible: None,
                    style: None,
                    handle_kind: Some("core rectangular bottom".into()),
                    color: None,
                    icon_kind: None,
                    user_data: None,
                },
            ],
            edges: vec![],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        let out: serde_json::Value = serde_json::from_str(&h.brush_fill_json(1, 7)).unwrap();
        let placements = out.get("placements").and_then(|x| x.as_array()).unwrap();
        assert_eq!(placements.len(), 1, "expected one fill placement on base");
        let node_kind = placements[0].get("nodeKind").and_then(|x| x.as_str()).unwrap_or("");
        assert_ne!(node_kind, CYLINDRIC_TAMBOUR_KIND, "cylindric tambour must not stack on rectangular core");
        assert_eq!(node_kind, FIRST_STOREY_KIND, "first storey tambour matches rectangular core stack");
    }

    #[test]
    fn board_host_brush_door_tambour_left_excludes_capital_with_metabolism_compat_rules() {
        const DOOR_TAMBOUR_LEFT: &str = "door tambour left";
        const CAPITAL_KIND: &str = "Capital";
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_active_tool("brush");
        h.set_brush_flush_distance(40.0);
        h.set_brush_node_size(40.0);
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../fixture/nakagin-capsule-tower.2d.json")).unwrap();
        let compat_str = fixture
            .get("meta")
            .and_then(|m| m.get("kindCompatibility"))
            .map(|v| v.to_string())
            .unwrap_or_else(|| "[]".to_string());
        h.set_handle_link_compat_from_json(&compat_str).unwrap();
        let catalogs_str = fixture
            .get("meta")
            .and_then(|m| m.get("kindCatalogs"))
            .map(|kc| {
                serde_json::json!({
                    "handleKinds": kc.get("handles"),
                    "nodeKinds": kc.get("nodes"),
                })
                .to_string()
            })
            .unwrap_or_else(|| "{}".to_string());
        h.set_board_kind_catalogs_from_json(&catalogs_str).unwrap();
        let desc = SceneDescriptorJson {
            nodes: vec![NodeDescJson {
                id: "tambour".into(),
                x: 0.0,
                y: 0.0,
                draggable: Some(true),
                selected: None,
                style: None,
                text: None,
                icon_kind: None,
                node_kind: Some("Tambour".into()),
                user_data: None,
                visible: None,
                root: None,
                shape: Some("circle".into()),
                radius: Some(40.0),
                width: None,
                height: None,
                scale: None,
            }],
            handles: vec![HandleDescJson {
                id: "tambour:h0".into(),
                node_id: "tambour".into(),
                angle: 0.0,
                radius: None,
                scale: None,
                selected: None,
                visible: None,
                style: None,
                handle_kind: Some(DOOR_TAMBOUR_LEFT.into()),
                color: None,
                icon_kind: None,
                user_data: None,
            }],
            edges: vec![],
            wires: vec![],
            selection_exit_highlight_ids: vec![],
        };
        h.sync_descriptor(&desc).unwrap();
        let _ = h.drain_events_json();
        let hp = handle_position_on_circle(Point::new(0.0, 0.0), 40.0, 0.0);
        let slot = hp + (hp - Point::new(0.0, 0.0)) * (40.0 / 40.0);
        let slot_screen = h.world_to_screen(slot);
        h.pointer_move_screen(slot_screen.x, slot_screen.y, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushCandidates"), "expected brushCandidates, got: {ev}");
        let v: serde_json::Value = serde_json::from_str(&ev).unwrap();
        let candidates = v
            .as_array()
            .and_then(|rows| {
                rows.iter()
                    .find(|row| row.get("name").and_then(|n| n.as_str()) == Some("brushCandidates"))
                    .and_then(|row| row.get("payload"))
                    .and_then(|p| p.get("candidates"))
                    .cloned()
            })
            .and_then(|c| c.as_array().cloned())
            .unwrap_or_default();
        let ids: Vec<String> = candidates.iter().filter_map(|x| x.as_str().map(str::to_string)).collect();
        assert!(
            !ids.iter().any(|id| id == CAPITAL_KIND),
            "door tambour left must not suggest Capital, got: {ids:?}"
        );
    }

    #[test]
    fn board_host_brush_slot_accepts_pointer_on_node_body_at_overview_lod() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        set_overview_lod(&mut h);
        h.set_active_tool("brush");
        h.set_brush_flush_distance(40.0);
        h.set_brush_node_size(40.0);
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [{ "handleKind": "child", "angle": 3.141592653589793 }]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_no_edge()).unwrap();
        let _ = h.drain_events_json();
        let inside = h.world_to_screen(Point::new(0.0, 0.0));
        h.pointer_move_screen(inside.x, inside.y, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushPreview"), "expected brushPreview when hovering node body at overview LOD, got: {ev}");
        assert!(ev.contains("brushCandidates"), "expected brushCandidates, got: {ev}");
    }

    #[test]
    fn board_host_brush_slot_accepts_pointer_on_indirect_ring_anchor() {
        let mut h = BoardHost::new();
        h.set_size(800, 600, 1.0);
        h.set_camera(0.0, 0.0, 1.0);
        h.set_active_tool("brush");
        h.set_brush_flush_distance(40.0);
        h.set_brush_node_size(40.0);
        h.set_handle_link_compat_from_json(r#"[{"source":"parent","target":"child"}]"#).unwrap();
        h.set_board_kind_catalogs_from_json(
            &serde_json::json!({
                "handleKinds": [
                    {"id": "parent", "name": "Parent", "color": "#888888"},
                    {"id": "child", "name": "Child", "color": "#888888"}
                ],
                "nodeKinds": [{
                    "id": "brush.kind",
                    "name": "Brush Kind",
                    "handles": [{ "handleKind": "child", "angle": 3.141592653589793 }]
                }]
            })
            .to_string(),
        )
        .unwrap();
        h.sync_descriptor(&link_test_scene_node_a_two_free_handles()).unwrap();
        let _ = h.drain_events_json();
        h.set_selection_ids(&["a".into()]);
        let ha0 = h.handles.get("a:h0").unwrap();
        let ring = h.indirect_handle_world_pos(ha0).unwrap();
        let s = h.world_to_screen(ring);
        h.pointer_move_screen(s.x, s.y, false, false);
        let ev = h.drain_events_json();
        assert!(ev.contains("brushPreview"), "expected brushPreview on indirect ring anchor, got: {ev}");
    }
}

#[cfg(test)]
mod force_graph_tests {
    use crate::graph::apply_edge_handle_snap_to_fixture_v1_json;
    use crate::{apply_force_graph_layout_to_fixture_v1_json, apply_normal_undirected_redraw_layout_to_fixture_v1_json};
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn force_graph_spreads_two_linked_circles_along_x() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 1.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "iterations": 200,
            "idealEdgeLength": 180.0,
            "repulsionStrength": 8000.0,
            "springStrength": 0.04,
            "gravity": 0.0,
            "randomSeed": 7
        });
        let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let bx = nodes[1]["x"].as_f64().unwrap();
        assert!((bx - ax).abs() > 80.0, "expected horizontal separation, got a={ax} b={bx}");
    }

    #[test]
    fn force_graph_pins_locked_node_positions() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 35.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 40.0,
                    "y": 0.0,
                    "radius": 35.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "iterations": 180,
            "idealEdgeLength": 160.0,
            "repulsionStrength": 7500.0,
            "springStrength": 0.045,
            "gravity": 0.0,
            "randomSeed": 101,
            "lockedNodeIds": ["a"]
        });
        let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let ay = nodes[0]["y"].as_f64().unwrap();
        assert!((ax - 0.0).abs() < 1e-9 && (ay - 0.0).abs() < 1e-9);
        let bx = nodes[1]["x"].as_f64().unwrap();
        assert!((bx - 40.0).abs() > 25.0, "expected free node to move, bx={bx}");
    }

    #[test]
    fn redraw_force_graph_top_level_locked_node_ids_pins() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 35.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 40.0,
                    "y": 0.0,
                    "radius": 35.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "mode": "force-graph",
            "lockedNodeIds": ["a"],
            "randomSeed": 101,
            "redrawHandlesAfter": false,
            "forceGraph": {
                "iterations": 180,
                "idealEdgeLength": 160.0,
                "repulsionStrength": 7500.0,
                "springStrength": 0.045,
                "gravity": 0.0
            }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        assert!((nodes[0]["x"].as_f64().unwrap() - 0.0).abs() < 1e-9);
        assert!((nodes[0]["y"].as_f64().unwrap() - 0.0).abs() < 1e-9);
    }

    #[test]
    fn redraw_force_graph_mindmap_schema_uses_undirected_layout() {
        let fixture = json!({
            "schema": "reasoning.mindmap.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "x": 0.0, "y": 0.0, "radius": 40.0 },
                { "id": "b", "x": 1.0, "y": 0.0, "radius": 40.0 }
            ],
            "edges": [{ "id": "e1", "source": "a", "target": "b" }]
        });
        let opts = json!({
            "mode": "force-graph",
            "randomSeed": 7,
            "forceGraph": {
                "iterations": 200,
                "idealEdgeLength": 180.0,
                "repulsionStrength": 0.0,
                "springStrength": 0.04,
                "gravity": 0.0
            }
        });
        let out = apply_normal_undirected_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let bx = nodes[1]["x"].as_f64().unwrap();
        assert!((bx - ax).abs() > 80.0, "expected mindmap undirected springs, got a={ax} b={bx}");
    }

    #[test]
    fn force_graph_normal_mode_node_id_edges_apply_spring_forces() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "x": 0.0, "y": 0.0, "radius": 40.0, "handles": [] },
                { "id": "b", "x": 1.0, "y": 0.0, "radius": 40.0, "handles": [] }
            ],
            "edges": [{ "id": "e1", "source": "a", "target": "b" }]
        });
        let opts = json!({
            "iterations": 200,
            "idealEdgeLength": 180.0,
            "repulsionStrength": 0.0,
            "springStrength": 0.04,
            "gravity": 0.0,
            "randomSeed": 7
        });
        let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let bx = nodes[1]["x"].as_f64().unwrap();
        assert!((bx - ax).abs() > 80.0, "expected node-id edge springs to spread nodes, got a={ax} b={bx}");
    }

    #[test]
    fn force_graph_rejects_bad_schema() {
        let err = apply_force_graph_layout_to_fixture_v1_json(r#"{"schema":"x","nodes":[],"edges":[]}"#, "{}").unwrap_err();
        assert!(err.contains("schema"));
    }

    #[test]
    fn force_graph_barnes_hut_many_bodies_yields_finite_coordinates() {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for k in 0..64 {
            let id = format!("n{k}");
            nodes.push(json!({
                "id": id,
                "x": (k % 8) as f64 * 12.0,
                "y": (k / 8) as f64 * 12.0,
                "radius": 8.0,
                "handles": [{ "id": format!("{id}:h0"), "angle": 0.0, "handleKind": "port" }]
            }));
            if k > 0 {
                let prev = format!("n{}", k - 1);
                edges.push(json!({
                    "id": format!("e{k}"),
                    "source": format!("{prev}:h0"),
                    "target": format!("{id}:h0")
                }));
            }
        }
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": nodes,
            "edges": edges
        });
        let opts = json!({
            "iterations": 180,
            "idealEdgeLength": 90.0,
            "repulsionStrength": 6000.0,
            "springStrength": 0.05,
            "gravity": 0.01,
            "randomSeed": 91,
            "barnesHutTheta": 0.72,
            "pairwiseRepulsionMaxBodies": 12
        });
        let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        for row in parsed["nodes"].as_array().unwrap() {
            let x = row["x"].as_f64().unwrap();
            let y = row["y"].as_f64().unwrap();
            assert!(x.is_finite() && y.is_finite());
        }
        let xs: Vec<f64> = parsed["nodes"].as_array().unwrap().iter().map(|r| r["x"].as_f64().unwrap()).collect();
        let ys: Vec<f64> = parsed["nodes"].as_array().unwrap().iter().map(|r| r["y"].as_f64().unwrap()).collect();
        let x_span = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max) - xs.iter().copied().fold(f64::INFINITY, f64::min);
        let y_span = ys.iter().copied().fold(f64::NEG_INFINITY, f64::max) - ys.iter().copied().fold(f64::INFINITY, f64::min);
        assert!(x_span > 40.0 && y_span > 35.0, "expected BH layout to spread graph, x_span={x_span} y_span={y_span}");
    }

    #[test]
    fn force_graph_bh_layout_is_deterministic_for_fixed_seed() {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for k in 0..36 {
            let id = format!("n{k}");
            nodes.push(json!({
                "id": id,
                "x": (k % 6) as f64 * 9.0,
                "y": (k / 6) as f64 * 9.0,
                "radius": 6.5,
                "handles": [{ "id": format!("{id}:h0"), "angle": 0.0, "handleKind": "port" }]
            }));
            if k > 0 {
                let prev = format!("n{}", k - 1);
                edges.push(json!({
                    "id": format!("e{k}"),
                    "source": format!("{prev}:h0"),
                    "target": format!("{id}:h0")
                }));
            }
        }
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": nodes,
            "edges": edges
        });
        let opts = json!({
            "iterations": 120,
            "idealEdgeLength": 88.0,
            "repulsionStrength": 5400.0,
            "springStrength": 0.047,
            "gravity": 0.013,
            "randomSeed": 4041,
            "barnesHutTheta": 0.55,
            "pairwiseRepulsionMaxBodies": 8
        });
        let s = fixture.to_string();
        let o = opts.to_string();
        let out_a = apply_force_graph_layout_to_fixture_v1_json(&s, &o).unwrap();
        let out_b = apply_force_graph_layout_to_fixture_v1_json(&s, &o).unwrap();
        assert_eq!(out_a, out_b, "BH path must be bitwise reproducible for identical inputs");
    }

    #[test]
    fn force_graph_pairwise_layout_is_deterministic_for_fixed_seed() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "x": 0.0, "y": 0.0, "radius": 30.0, "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }] },
                { "id": "b", "x": 3.0, "y": 1.0, "radius": 30.0, "handles": [{ "id": "b:h0", "angle": 3.14, "handleKind": "port" }] },
                { "id": "c", "x": -2.0, "y": 4.0, "radius": 28.0, "handles": [{ "id": "c:h0", "angle": 1.0, "handleKind": "port" }] }
            ],
            "edges": [
                { "id": "e1", "source": "a:h0", "target": "b:h0" },
                { "id": "e2", "source": "b:h0", "target": "c:h0" }
            ]
        });
        let opts = json!({
            "iterations": 90,
            "idealEdgeLength": 110.0,
            "repulsionStrength": 6200.0,
            "springStrength": 0.042,
            "gravity": 0.011,
            "randomSeed": 909,
            "pairwiseRepulsionMaxBodies": 80
        });
        let s = fixture.to_string();
        let o = opts.to_string();
        let out_a = apply_force_graph_layout_to_fixture_v1_json(&s, &o).unwrap();
        let out_b = apply_force_graph_layout_to_fixture_v1_json(&s, &o).unwrap();
        assert_eq!(out_a, out_b);
    }

    #[test]
    fn force_graph_clamped_barnes_hut_theta_runs_without_error() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "a", "x": 0.0, "y": 0.0, "radius": 20.0, "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }] },
                { "id": "b", "x": 5.0, "y": 0.0, "radius": 20.0, "handles": [{ "id": "b:h0", "angle": 3.14, "handleKind": "port" }] },
                { "id": "c", "x": 2.0, "y": 8.0, "radius": 18.0, "handles": [{ "id": "c:h0", "angle": 0.0, "handleKind": "port" }] }
            ],
            "edges": [
                { "id": "e1", "source": "a:h0", "target": "b:h0" },
                { "id": "e2", "source": "b:h0", "target": "c:h0" }
            ]
        });
        let opts = json!({
            "iterations": 40,
            "idealEdgeLength": 100.0,
            "repulsionStrength": 5000.0,
            "springStrength": 0.05,
            "gravity": 0.01,
            "randomSeed": 3,
            "barnesHutTheta": 500.0,
            "pairwiseRepulsionMaxBodies": 2
        });
        let out = apply_force_graph_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        for row in parsed["nodes"].as_array().unwrap() {
            assert!(row["x"].as_f64().unwrap().is_finite());
            assert!(row["y"].as_f64().unwrap().is_finite());
        }
    }

    #[test]
    fn redraw_force_graph_wraps_flat_options() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 1.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "mode": "force-graph",
            "randomSeed": 7,
            "forceGraph": {
                "iterations": 200,
                "idealEdgeLength": 180.0,
                "repulsionStrength": 8000.0,
                "springStrength": 0.04,
                "gravity": 0.0
            }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let bx = nodes[1]["x"].as_f64().unwrap();
        assert!((bx - ax).abs() > 80.0);
    }

    #[test]
    fn edge_handle_snap_sets_circle_handle_angles_on_center_line() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 1.57, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 200.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let out = crate::graph::apply_edge_handle_snap_to_fixture_v1_json(&fixture.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ang_a = nodes[0]["handles"][0]["angle"].as_f64().unwrap();
        let ang_b = nodes[1]["handles"][0]["angle"].as_f64().unwrap();
        assert!((ang_a - 0.0).abs() < 1e-6, "expected east on a, got {ang_a}");
        assert!((ang_b - std::f64::consts::PI).abs() < 1e-6, "expected west on b, got {ang_b}");
    }

    #[test]
    fn redraw_force_graph_with_snap_sets_handle_angles() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 1.57, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "x": 200.0,
                    "y": 0.0,
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "mode": "force-graph",
            "redrawHandlesAfter": true,
            "randomSeed": 7,
            "forceGraph": {
                "iterations": 200,
                "idealEdgeLength": 180.0,
                "repulsionStrength": 8000.0,
                "springStrength": 0.04,
                "gravity": 0.0
            }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let nodes = parsed["nodes"].as_array().unwrap();
        let ang_a = nodes[0]["handles"][0]["angle"].as_f64().unwrap();
        let ang_b = nodes[1]["handles"][0]["angle"].as_f64().unwrap();
        let ax = nodes[0]["x"].as_f64().unwrap();
        let bx = nodes[1]["x"].as_f64().unwrap();
        let ay = nodes[0]["y"].as_f64().unwrap();
        let by = nodes[1]["y"].as_f64().unwrap();
        let exp_a = f64::atan2(by - ay, bx - ax);
        let exp_b = f64::atan2(ay - by, ax - bx);
        let wrap_diff = |a: f64, b: f64| {
            let mut d = (a - b).rem_euclid(std::f64::consts::TAU);
            if d > std::f64::consts::PI {
                d -= std::f64::consts::TAU;
            }
            d.abs()
        };
        assert!(wrap_diff(ang_a, exp_a) < 0.03, "a angle {ang_a} vs exp {exp_a}");
        assert!(wrap_diff(ang_b, exp_b) < 0.03, "b angle {ang_b} vs exp {exp_b}");
    }

    #[test]
    fn force_graph_accepts_logical_nodes_without_xy() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "a",
                    "radius": 40.0,
                    "handles": [{ "id": "a:h0", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "b",
                    "radius": 40.0,
                    "handles": [{ "id": "b:h0", "angle": 3.14159, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "a:h0", "target": "b:h0" }]
        });
        let opts = json!({
            "mode": "force-graph",
            "centerX": 0.0,
            "centerY": 0.0,
            "randomSeed": 3,
            "forceGraph": { "iterations": 120, "idealEdgeLength": 160.0, "gravity": 0.0 }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        for n in parsed["nodes"].as_array().unwrap() {
            assert!(n["x"].as_f64().unwrap().is_finite());
            assert!(n["y"].as_f64().unwrap().is_finite());
        }
    }

    #[test]
    fn hierarchical_tree_normal_mode_node_id_edges_stacks_by_depth() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                { "id": "r", "root": true, "radius": 18.0, "handles": [] },
                { "id": "c1", "radius": 18.0, "handles": [] },
                { "id": "c2", "radius": 18.0, "handles": [] }
            ],
            "edges": [
                { "id": "e1", "source": "r", "target": "c1" },
                { "id": "e2", "source": "r", "target": "c2" }
            ]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "hierarchicalTree": { "direction": "downwards", "layerSpacing": 90.0, "siblingGap": 12.0 }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut ys: HashMap<String, f64> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            ys.insert(id, n["y"].as_f64().unwrap());
        }
        let ry = *ys.get("r").unwrap();
        let c1y = *ys.get("c1").unwrap();
        let c2y = *ys.get("c2").unwrap();
        assert!((c1y - ry).abs() > 40.0, "expected child below root");
        assert!((c2y - ry).abs() > 40.0);
        assert!((c1y - c2y).abs() < 1e-3, "siblings share row");
    }

    #[test]
    fn hierarchical_tree_stacks_by_depth() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c1",
                    "radius": 18.0,
                    "handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c2",
                    "radius": 18.0,
                    "handles": [{ "id": "c2:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [
                { "id": "e1", "source": "r:h", "target": "c1:h" },
                { "id": "e2", "source": "r:h", "target": "c2:h" }
            ]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "hierarchicalTree": { "direction": "downwards", "layerSpacing": 90.0, "siblingGap": 12.0 }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut ys: HashMap<String, f64> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            ys.insert(id, n["y"].as_f64().unwrap());
        }
        let ry = *ys.get("r").unwrap();
        let c1y = *ys.get("c1").unwrap();
        let c2y = *ys.get("c2").unwrap();
        assert!((c1y - ry).abs() > 40.0, "expected child below root");
        assert!((c2y - ry).abs() > 40.0);
        assert!((c1y - c2y).abs() < 1e-3, "siblings share row");
    }

    #[test]
    fn hierarchical_tree_pins_locked_root_coordinates() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "x": 120.0,
                    "y": -33.0,
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c1",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 18.0,
                    "handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c2",
                    "x": 5.0,
                    "y": 0.0,
                    "radius": 18.0,
                    "handles": [{ "id": "c2:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [
                { "id": "e1", "source": "r:h", "target": "c1:h" },
                { "id": "e2", "source": "r:h", "target": "c2:h" }
            ]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "lockedNodeIds": ["r"],
            "hierarchicalTree": { "direction": "downwards", "layerSpacing": 90.0, "siblingGap": 12.0 }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut by_id: HashMap<String, (f64, f64)> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            by_id.insert(id, (n["x"].as_f64().unwrap(), n["y"].as_f64().unwrap()));
        }
        let (rx, ry) = *by_id.get("r").unwrap();
        assert!((rx - 120.0).abs() < 1e-3 && (ry + 33.0).abs() < 1e-3, "locked root moved: {rx},{ry}");
        let (_c1x, c1y) = *by_id.get("c1").unwrap();
        let (_c2x, c2y) = *by_id.get("c2").unwrap();
        assert!((c1y - c2y).abs() < 1e-3, "siblings share row");
        assert!((c1y - ry).abs() > 40.0, "children laid relative to tree, root stayed pinned");
    }

    #[test]
    fn redraw_hierarchical_tree_nested_locked_node_ids_pins() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "x": 77.0,
                    "y": 12.0,
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c1",
                    "x": 0.0,
                    "y": 0.0,
                    "radius": 18.0,
                    "handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "r:h", "target": "c1:h" }]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "hierarchicalTree": {
                "direction": "downwards",
                "layerSpacing": 90.0,
                "siblingGap": 12.0,
                "lockedNodeIds": ["r"]
            }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut by_id: HashMap<String, (f64, f64)> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            by_id.insert(id, (n["x"].as_f64().unwrap(), n["y"].as_f64().unwrap()));
        }
        let (rx, ry) = *by_id.get("r").unwrap();
        assert!((rx - 77.0).abs() < 1e-3 && (ry - 12.0).abs() < 1e-3, "nested locked list ignored: {rx},{ry}");
    }

    #[test]
    fn hierarchical_tree_right_places_children_larger_x_than_root() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c1",
                    "radius": 18.0,
                    "handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "r:h", "target": "c1:h" }]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "hierarchicalTree": { "direction": "right", "layerSpacing": 90.0, "siblingGap": 12.0 }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut xs: HashMap<String, f64> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            xs.insert(id, n["x"].as_f64().unwrap());
        }
        let rx = *xs.get("r").unwrap();
        let c1x = *xs.get("c1").unwrap();
        assert!(c1x > rx + 40.0, "expected child to the right of root");
    }

    #[test]
    fn hierarchical_tree_upwards_places_children_smaller_y_than_root() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                },
                {
                    "id": "c1",
                    "radius": 18.0,
                    "handles": [{ "id": "c1:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": [{ "id": "e1", "source": "r:h", "target": "c1:h" }]
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "centerX": 0.0,
            "centerY": 0.0,
            "hierarchicalTree": { "direction": "upwards", "layerSpacing": 90.0, "siblingGap": 12.0 }
        });
        let out = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
        let mut ys: HashMap<String, f64> = HashMap::new();
        for n in parsed["nodes"].as_array().unwrap() {
            let id = n["id"].as_str().unwrap().to_string();
            ys.insert(id, n["y"].as_f64().unwrap());
        }
        let ry = *ys.get("r").unwrap();
        let c1y = *ys.get("c1").unwrap();
        assert!(c1y < ry - 40.0, "expected child above root (smaller y)");
    }

    #[test]
    fn hierarchical_tree_rejects_unknown_direction() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [
                {
                    "id": "r",
                    "root": true,
                    "radius": 18.0,
                    "handles": [{ "id": "r:h", "angle": 0.0, "handleKind": "port" }]
                }
            ],
            "edges": []
        });
        let opts = json!({
            "mode": "hierarchical-tree",
            "hierarchicalTree": { "direction": "sideways" }
        });
        let err = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), &opts.to_string()).unwrap_err();
        assert!(err.contains("unknown hierarchical tree direction"));
    }

    #[test]
    fn redraw_rejects_unknown_mode() {
        let fixture = json!({
            "schema": "puzzle.2d.fixture/v1",
            "camera": { "x": 0.0, "y": 0.0, "zoom": 1.0 },
            "nodes": [],
            "edges": []
        });
        let err = crate::graph::apply_redraw_layout_to_fixture_v1_json(&fixture.to_string(), r#"{"mode":"nope"}"#).unwrap_err();
        assert!(err.contains("unknown redraw mode"));
    }

    #[test]
    fn svg_icon_vello09_append_smoke() {
        let mut scene = crate::cavas::vello::Scene::new();
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><rect width="10" height="10" fill="#ffffff"/><path d="M0 0 L10 10" stroke="#000000" stroke-width="1"/></svg>"##;
        super::svg_icon_vello09::append_svg_str(&mut scene, svg).expect("parse svg");
        let fg = crate::cavas::vello::peniko::Color::from_rgba8(200, 10, 10, 255);
        let bg = crate::cavas::vello::peniko::Color::from_rgba8(10, 200, 10, 255);
        let mut scene2 = crate::cavas::vello::Scene::new();
        super::svg_icon_vello09::append_svg_str_themed(&mut scene2, svg, fg, bg).expect("parse themed");
    }

    #[test]
    fn board_icon_codec_resolves_typst_math_to_svg_plain() {
        let r = super::board_icon_codec::board_resolve_icon_kind("typst:$x^2$");
        match r {
            super::board_icon_codec::BoardResolvedIcon::SvgPlain(s) => {
                assert!(s.contains("<svg"), "{}", &s[..s.len().min(240)]);
            }
            other => panic!("unexpected resolution: {other:?}"),
        }
    }

    #[test]
    fn board_icon_codec_resolves_emoji_prefix_without_tofu() {
        let r = super::board_icon_codec::board_resolve_icon_kind("emoji:☺");
        match r {
            super::board_icon_codec::BoardResolvedIcon::SvgPlain(s) => {
                assert!(s.contains("<svg"), "{}", &s[..s.len().min(240)]);
                assert!(!s.contains('\u{fffd}'), "expected no U+FFFD replacement in emoji SVG, got {}", &s[..s.len().min(400)]);
            }
            other => panic!("unexpected resolution: {other:?}"),
        }
    }

    #[test]
    fn svg_icon_content_bounds_follows_nested_group_translate() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200"><g transform="translate(72 88)"><rect width="12" height="12" fill="rgb(8,8,8)"/></g></svg>"#;
        let tree = crate::usvg::Tree::from_str(svg, &crate::usvg::Options::default()).expect("parse");
        let (x, y, w, h) = super::svg_icon_vello09::svg_icon_content_bounds(&tree);
        assert!(x >= 70.0 && x <= 74.0, "expected translated art near x≈72, got {x}");
        assert!(y >= 86.0 && y <= 90.0, "expected translated art near y≈88, got {y}");
        assert!(w > 10.0 && w < 14.0 && h > 10.0 && h < 14.0, "expected ~12×12 bbox, got {w}×{h}");
    }

    #[test]
    fn svg_icon_content_bounds_includes_visible_image_abs_box() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><image href="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==" x="30" y="40" width="50" height="50"/></svg>"##;
        let tree = crate::usvg::Tree::from_str(svg, super::svg_icon_vello09::usvg_options_board_icons()).expect("parse");
        let (x, y, w, h) = super::svg_icon_vello09::svg_icon_content_bounds(&tree);
        assert!((x - 30.0).abs() < 2.0, "expected image bbox near x=30, got {x}");
        assert!((y - 40.0).abs() < 2.0, "expected image bbox near y=40, got {y}");
        assert!((w - 50.0).abs() < 2.0 && (h - 50.0).abs() < 2.0, "expected ~50×50 bbox, got {w}×{h}");
    }
}

// #endregion 🔖Tests
