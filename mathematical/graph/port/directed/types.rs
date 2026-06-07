//! 🧩 Directed port graph board types shared by normal and dag leaves.

use std::collections::{BTreeMap, BTreeSet};

use crate::cavas::vello::kurbo::{Point, Vec2};
use crate::cavas::vello::peniko::Color;
pub use crate::cavas::camera::Camera;
use crate::{HandleKindDef, NodeKindHandleTemplate};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeShape {
    Circle,
    Rectangle,
}

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

    /// @emoji 🎨 Merges a partial Vello theme JSON payload from the React host into this palette.
    pub fn merge_from_json(&mut self, json: &str) -> Result<(), String> {
        let v: serde_json::Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let mut next = *self;
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
        *self = next;
        Ok(())
    }
}

impl Default for VelloThemePalette {
    fn default() -> Self {
        Self {
            raster_clear: Color::from_rgba8(248, 248, 250, 255),
            grid_minor_stroke: Color::from_rgba8(160, 160, 170, 56),
            edge_stroke: Color::from_rgba8(120, 120, 130, 255),
            edge_stroke_hovered: Color::from_rgba8(40, 44, 52, 255),
            edge_stroke_selected: Color::from_rgba8(60, 120, 220, 255),
            edge_stroke_selection_exit: Color::from_rgba8(80, 140, 110, 255),
            edge_stroke_disabled: Color::from_rgba8(160, 160, 170, 56),
            node_fill: Color::from_rgba8(235, 238, 245, 255),
            node_stroke: Color::from_rgba8(40, 44, 52, 255),
            node_fill_hovered: Color::from_rgba8(235, 238, 245, 255),
            node_stroke_hovered: Color::from_rgba8(40, 44, 52, 255),
            node_fill_selected: Color::from_rgba8(60, 120, 220, 89),
            node_stroke_selected: Color::from_rgba8(60, 120, 220, 255),
            node_fill_selection_exit: Color::from_rgba8(196, 228, 213, 255),
            node_stroke_selection_exit: Color::from_rgba8(80, 140, 110, 255),
            node_fill_disabled: Color::from_rgba8(235, 238, 245, 255),
            node_stroke_disabled: Color::from_rgba8(160, 160, 170, 56),
            indirect_handle_fill: Color::from_rgba8(196, 228, 213, 255),
            indirect_handle_stroke: Color::from_rgba8(80, 140, 110, 255),
            handle_fill: Color::from_rgba8(248, 248, 250, 255),
            handle_stroke: Color::from_rgba8(40, 44, 52, 255),
            handle_fill_hovered: Color::from_rgba8(248, 248, 250, 255),
            handle_stroke_hovered: Color::from_rgba8(40, 44, 52, 255),
            handle_fill_selected: Color::from_rgba8(60, 120, 220, 89),
            handle_stroke_selected: Color::from_rgba8(60, 120, 220, 255),
            handle_fill_selection_exit: Color::from_rgba8(196, 228, 213, 255),
            handle_stroke_selection_exit: Color::from_rgba8(80, 140, 110, 255),
            handle_fill_disabled: Color::from_rgba8(248, 248, 250, 255),
            handle_stroke_disabled: Color::from_rgba8(160, 160, 170, 56),
            wire_stroke: Color::from_rgba8(120, 120, 130, 255),
            wire_stroke_hovered: Color::from_rgba8(40, 44, 52, 255),
            wire_stroke_selected: Color::from_rgba8(60, 120, 220, 255),
            wire_stroke_highlighted: Color::from_rgba8(80, 140, 110, 255),
            wire_stroke_disabled: Color::from_rgba8(160, 160, 170, 56),
            selection_preview_fill: Color::from_rgba8(60, 120, 220, 40),
            selection_preview_stroke: Color::from_rgba8(60, 120, 220, 180),
        }
    }
}
