// @emoji 🎨 Auto-generated from ui/styling/tokens.json — do not edit by hand.

pub mod strokes {
    pub const EDGE_BASE: f64 = 2.0;
    pub const EDGE_SELECTED_MULT: f64 = 1.35;
    pub const EDGE_HOVERED_MULT: f64 = 1.2;
    pub const EDGE_MINIMAP: f64 = 1.12;
    pub const EDGE_OVERVIEW: f64 = 2.75;
    pub const EDGE_DASH_LONG_MULT: f64 = 3.0;
    pub const EDGE_DASH_LONG_GAP_MULT: f64 = 2.0;
    pub const EDGE_DASH_DOT_MULT: f64 = 0.35;
    pub const EDGE_DASH_DOT_GAP_MULT: f64 = 1.65;
    pub const EDGE_TIP_MIN: f64 = 1.25;
    pub const EDGE_TIP_OUTLINE_MULT: f64 = 0.9;
    pub const EDGE_ARROW_HALF_MULT: f64 = 1.25;
    pub const NODE_BODY: f64 = 2.0;
    pub const WIRE_HIGHLIGHT: f64 = 2.85;
    pub const HANDLE: f64 = 2.0;
    pub const SELECTION_PREVIEW: f64 = 1.5;
    pub const SELECTION_PREVIEW_DASH: &[f64] = &[5.0, 4.0];
    pub const GRID_LARGE: f64 = 1.0;
    pub const GRID_MEDIUM: f64 = 0.72;
    pub const GRID_SMALL: f64 = 0.48;
    pub const GRID_MICRO: f64 = 0.32;
    pub const DAG_NODE: f64 = 1.5;
    pub const DAG_NODE_SELECTED: f64 = 2.25;
    pub const DAG_NODE_HOVERED: f64 = 2.0;
    pub const DAG_EDGE: f64 = 2.0;
    pub const DAG_EDGE_MINIMAP: f64 = 1.0;
    pub const DAG_CHROME: f64 = 1.25;
    pub const MAP_ROUTE_DEFAULT: f64 = 2.0;
    pub const MAP_REGION_MULT: f64 = 2.0;
    pub const MAP_POSITION_MULT: f64 = 2.0;
    pub const MAP_ROAD_MOTORWAY: f64 = 2.4;
    pub const MAP_ROAD_PATH: f64 = 0.55;
    pub const MAP_ROAD_CLAMP_MIN: f64 = 0.35;
    pub const MAP_ROAD_CLAMP_MAX: f64 = 2.8;
    pub const MAP_ROAD_MOTORWAY_MULT: f64 = 2.1;
    pub const MAP_ROAD_TRUNK_MULT: f64 = 1.8;
    pub const MAP_ROAD_PRIMARY_MULT: f64 = 1.45;
    pub const MAP_ROAD_SECONDARY_MULT: f64 = 1.2;
    pub const MAP_ROAD_TERTIARY_MULT: f64 = 0.95;
    pub const MAP_ROAD_RESIDENTIAL_MULT: f64 = 0.78;
    pub const MAP_ROAD_SERVICE_MULT: f64 = 0.9;
    pub const PUZZLE3D_OUTLINE: f64 = 4.0;
    pub const CAD_CONSTRUCTION: f64 = 1.0;
    pub const CAD_GUIDE: f64 = 1.5;
    pub const CAD_DIMENSION: f64 = 2.0;
    pub const CAD_PICK: f64 = 4.0;
    pub const CAD_HIGHLIGHT: f64 = 5.0;
    pub const CAD_EMPHASIS: f64 = 7.0;
    pub const CAD_EMPHASIS_STRONG: f64 = 8.0;
    pub const CAD_EMPHASIS_MAX: f64 = 9.0;
    pub const CAD_EMPHASIS_LINE: f64 = 12.0;
    pub const MAP_BOUNDARY_ADMIN2: f64 = 1.75;
    pub const MAP_BOUNDARY_DEFAULT: f64 = 0.65;
    pub const MAP_BOUNDARY_CLAMP_MIN: f64 = 0.45;
    pub const MAP_BOUNDARY_CLAMP_MAX: f64 = 2.2;
    pub const MAP_COASTLINE_MULT: f64 = 1.35;
    pub const MAP_COASTLINE_CLAMP_MIN: f64 = 0.85;
    pub const MAP_COASTLINE_CLAMP_MAX: f64 = 2.8;
    pub const MAP_LINE_SCALE_CAP: f64 = 1.38;
    pub const MAP_LINE_SCALE_RAW: f64 = 280.0;
    pub const MAP_ROAD_LOD_REGION: f64 = 0.4;
    pub const MAP_ROAD_LOD_CITY: f64 = 0.3;
}

pub mod radii {
    pub const NODE_DEFAULT: f64 = 24.0;
    pub const NODE_MIN: f64 = 28.0;
    pub const HANDLE_DEFAULT: f64 = 8.0;
    pub const HANDLE_MIN_HALF_EXTENT: f64 = 8.0;
    pub const NODE_RECT_DEFAULT: f64 = 40.0;
    pub const DAG_HANDLE_WORLD: f64 = 5.0;
    pub const MAP_POSITION_MARKER: f64 = 8.0;
    pub const MAP_LABEL_ANCHOR_OFFSET: f64 = 6.0;
}

pub mod opacities {
    pub const GRID_MINOR_ALPHA: f64 = 0.22;
    pub const DIM_STROKE_ALPHA: u8 = 110;
    pub const DIM_LABEL_ALPHA: u8 = 110;
    pub const DISABLED_FILL_ALPHA: u8 = 120;
    pub const DISABLED_STROKE_ALPHA: u8 = 120;
    pub const KIND_HINT_ALPHA: u8 = 140;
    pub const NODE_SELECTED_FILL_ALPHA: f64 = 0.35;
    pub const SELECTION_PREVIEW_FILL_ALPHA: f64 = 0.12;
    pub const MAP_LAND_STROKE_ALPHA: f64 = 0.42;
    pub const MAP_LABEL_HALO_ALPHA: f64 = 0.92;
    pub const MAP_REGION_FILL_ALPHA: f64 = 0.22;
    pub const MAP_REGION_STROKE_ALPHA: f64 = 0.9;
    pub const MAP_ROUTE_STROKE_ALPHA: f64 = 0.92;
    pub const MAP_BUILDING_FILL_WEIGHT: u8 = 220;
    pub const MAP_PARK_FILL_WEIGHT: u8 = 90;
    pub const DISABLED_MIX_ALPHA: u8 = 96;
    pub const DISABLED_PANEL_MIX_ALPHA: u8 = 128;
    pub const LABEL_HALO_ALPHA: u8 = 200;
    pub const SELECTION_PREVIEW_STROKE_ALPHA: u8 = 180;
    pub const HANDLE_FILL_TRANSPARENT: f64 = 0.0;
}

pub mod metrics {
    pub mod camera {
        pub const ZOOM_MIN: f64 = 0.05;
        pub const ZOOM_MAX: f64 = 32.0;
        pub const FLOW_ZOOM_MAX: f64 = 8.0;
        pub const WHEEL_ZOOM_IN_FACTOR: f64 = 1.1;
        pub const WHEEL_ZOOM_OUT_FACTOR: f64 = 0.9;
        pub const LOD_ZOOM_FLOOR: f64 = 0.05;
    }
    pub mod label {
        pub const MIN_PX: f64 = 4.0;
        pub const PAD_RATIO: f64 = 0.35;
        pub const CHAR_WIDTH_RATIO: f64 = 0.62;
        pub const HEIGHT_RATIO: f64 = 1.6;
        pub const WIDTH_MIN: f64 = 32.0;
        pub const WIDTH_MAX: f64 = 2048.0;
        pub const HEIGHT_MIN: f64 = 16.0;
        pub const HEIGHT_MAX: f64 = 256.0;
        pub const HALO_STROKE_RATIO: f64 = 0.12;
        pub const HALO_STROKE_MIN: f64 = 1.0;
        pub const SCALE_RATIO: f64 = 0.9;
        pub const SCALE_MAX: f64 = 2.5;
        pub const VERTICAL_OFFSET_RATIO: f64 = 0.85;
        pub const MAP_WIDTH_MIN: f64 = 24.0;
        pub const MAP_WIDTH_MAX: f64 = 420.0;
        pub const MAP_HEIGHT_MIN: f64 = 14.0;
        pub const MAP_HEIGHT_MAX: f64 = 96.0;
        pub const DECLUTTER_CELL_RATIO: f64 = 1.75;
        pub const DECLUTTER_CELL_MIN: f64 = 18.0;
        pub const DECLUTTER_CELL_MAX: f64 = 44.0;
        pub const DAG_COMPACT_PX: f64 = 10.0;
        pub const DAG_DEFAULT_PX: f64 = 11.0;
        pub const DAG_LABEL_SCALE_MULT: f64 = 1.05;
        pub const DAG_LABEL_GAP_RATIO: f64 = 0.35;
        pub const DAG_LABEL_GAP_COMPACT_RATIO: f64 = 0.2;
        pub const DAG_KIND_HINT_GAP_RATIO: f64 = 0.85;
        pub const HANDLE_OFFSET_PX: f64 = 10.0;
        pub const PUZZLE2D_DEFAULT_PX: f64 = 14.0;
    }
    pub mod icon {
        pub const FIT_INSET: f64 = 0.76;
        pub const CLIP_INSET: f64 = 0.88;
    }
    pub mod typst {
        pub const ICON_PAGE_PT: f64 = 96.0;
        pub const ICON_MARGIN_PT: f64 = 3.0;
        pub const EMOJI_PAGE_PT: f64 = 88.0;
        pub const EMOJI_MARGIN_PT: f64 = 2.0;
        pub const EMOJI_TEXT_PT: f64 = 44.0;
        pub const TEXT_ICON_PT: f64 = 28.0;
        pub const SVG_MARGIN_PT: f64 = 3.0;
    }
    pub mod board {
        pub const GRID_WORLD_LARGE: f64 = 10.0;
        pub const GRID_WORLD_MEDIUM: f64 = 2.5;
        pub const GRID_WORLD_SMALL: f64 = 0.5;
        pub const GRID_WORLD_MICRO: f64 = 0.1;
        pub const GRID_FACTOR_DEFAULT: f64 = 10.0;
        pub const WORLD_CLIP_TILE_WORLD: f64 = 256.0;
        pub const MAX_WORLD_CLIP_TILES: u32 = 768;
        pub const EDGE_HIT_TOLERANCE_PX: f64 = 8.0;
        pub const HANDLE_HIT_TOLERANCE_PX: f64 = 10.0;
        pub const INDIRECT_HANDLE_MARKER_SCALE: f64 = 0.8;
        pub const INDIRECT_HANDLE_RING_GAP_SCALE: f64 = 0.7;
        pub const LINK_DRAG_MIN_DISTANCE_PX: f64 = 5.0;
        pub const LINK_HANDLE_SNAP_EXTRA_PX: f64 = 22.0;
        pub const LINK_COMMIT_SNAP_TIGHT_PX: f64 = 2.0;
        pub const SUGGESTION_OFFSET: f64 = 80.0;
        pub const BRUSH_NODE_SIZE: f64 = 40.0;
        pub const SELECTION_LASSO_MIN_POINT_DISTANCE_PX: f64 = 3.0;
        pub const SELECTION_CLICK_MAX_DISTANCE_PX: f64 = 4.0;
        pub const SELECTION_MARQUEE_MAX_DISTANCE_PX: f64 = 4.0;
        pub const SELECTION_DRAG_DIRECTION_PX: f64 = 2.0;
        pub const BOUNDED_DRAG_HIT_PAD_PX: f64 = 8.0;
        pub const PROXIMITY_DISTANCE_WORLD: f64 = 48.0;
        pub const HANDLE_CAP_MARGIN_MULT: f64 = 2.5;
        pub const HANDLE_CAP_MARGIN_MIN: f64 = 4.0;
        pub const LAYOUT_LAYER_SPACING: f64 = 120.0;
        pub const LAYOUT_SIBLING_GAP: f64 = 40.0;
    }
    pub mod dag {
        pub const CHANNEL_ROW_HEIGHT: f64 = 14.0;
        pub const NODE_EDGE_INSET: f64 = 2.0;
        pub const NODE_COLUMN_GAP: f64 = 2.0;
        pub const IO_COLUMN_WIDTH: f64 = 20.0;
        pub const IO_WIDGET_HEIGHT: f64 = 28.0;
        pub const SLIDER_KNOB_SCREEN_PX: f64 = 8.0;
        pub const PREVIEW_PAD: f64 = 4.0;
        pub const PREVIEW_ROW_HEIGHT: f64 = 14.0;
        pub const PREVIEW_TREE_INDENT: f64 = 12.0;
        pub const PREVIEW_TOGGLE_WIDTH: f64 = 10.0;
        pub const PREVIEW_MAX_IMAGE: f64 = 200.0;
        pub const PREVIEW_MIN_SIZE: f64 = 20.0;
        pub const VARIADIC_PLUS_ZOOM_THRESHOLD: f64 = 1.5;
        pub const LOD_ZOOM_SHIFT: f64 = 0.25;
        pub const LOD_BAND_FLOOR_ZOOM: &[f64] = &[0.05, 0.15, 0.35, 0.55, 1.25, 2.5];
        pub const COMPONENT_WIDTH: f64 = 40.0;
    }
    pub mod map {
        pub const LABEL_PX_BANDS: &[f64] = &[26.0, 26.0, 20.0, 14.0, 12.5, 12.0, 10.5, 10.5];
        pub const LABEL_MAX_MIN: u32 = 48;
        pub const LABEL_MAX_MAX: u32 = 140;
        pub const LAYER_WEIGHT_MIN: f64 = 0.25;
        pub const LAYER_WEIGHT_MAX: f64 = 3.0;
        pub const LAYER_WEIGHT_DEFAULT: f64 = 1.0;
        pub const LAYER_WEIGHT_STEP: f64 = 0.05;
        pub const TILE_BLEED: f64 = 1.0;
        pub const LINE_SCALE_DAMP_MIN: f64 = 0.44;
        pub const LINE_SCALE_DAMP_MAX: f64 = 1.0;
        pub const WHEEL_ZOOM_FACTOR: f64 = 1.12;
        pub const WHEEL_DELTA_MULT: f64 = 2.5;
    }
    pub mod cad {
        pub const DIMENSION_FONT_WORLD: f64 = 0.22;
        pub const DRAG_PLANE_OFFSET: f64 = 0.06;
        pub const MESH_FACE_OPACITY: f64 = 0.72;
        pub const CAMERA_FIT_PADDING: f64 = 1.25;
        pub const CAMERA_FIT_MIN_DISTANCE: f64 = 2.0;
        pub const HATCH_SPACING: f64 = 0.4;
        pub const HATCH_LINE_WIDTH: f64 = 0.02;
        pub const HATCH_DIRECTION_DEG: f64 = 30.0;
        pub const CHUNK_SIZE: f64 = 256.0;
        pub const CHUNK_MAX_DISTANCE: f64 = 8000.0;
    }
    pub mod puzzle3d {
        pub const OUTLINE_THICKNESS: f64 = 4.0;
        pub const VORTEX_DRAG_THRESHOLD_PX: f64 = 6.0;
        pub const LINE_WIDTH_SELECTED: f64 = 3.0;
        pub const LINE_WIDTH_DEFAULT: f64 = 2.0;
    }
    pub mod dom {
        pub const ICON_TINY_UI_SPACING: f64 = 3.75;
        pub const ICON_SMALL_UI_SPACING: f64 = 6.25;
        pub const ICON_BASE_UI_SPACING: f64 = 7.5;
        pub const ICON_LARGE_UI_SPACING: f64 = 10.0;
        pub const TREE_ROW_UI_SPACING: f64 = 7.5;
        pub const TREE_TOGGLE_UI_SPACING: f64 = 4.375;
        pub const TREE_INDENT_PER_LEVEL_UI_SPACING: f64 = 3.125;
        pub const TREE_INDENT_LINE_EXTRA_UI_SPACING: f64 = 2.1875;
        pub const PROPERTY_LABEL_COLUMN_UI_SPACING: f64 = 30.0;
        pub const PROPERTY_INLINE_GAP_UI_SPACING: f64 = 2.5;
        pub const PROPERTY_STACKED_GAP_UI_SPACING: f64 = 1.25;
        pub const PROPERTY_STACKED_HYSTERESIS_UI_SPACING: f64 = 7.5;
        pub const CONTROL_VALUE_COLUMN_UI_SPACING: f64 = 50.0;
        pub const WINDOW_MEASURE_VALUE_COLUMN_UI_SPACING: f64 = 32.5;
        pub const RESIZABLE_CORNER_GRAB_UI_SPACING: f64 = 7.5;
        pub const LAYOUT_PANEL_RAIL_UI_SPACING: f64 = 70.0;
        pub const LAYOUT_PANEL_MIN_UI_SPACING: f64 = 46.875;
        pub const LAYOUT_PANEL_MAX_UI_SPACING: f64 = 150.0;
        pub const LAYOUT_ENGAGEMENT_MAX_UI_SPACING: f64 = 140.0;
        pub const CONTROL_TREE_ROW_MIN_UI_SPACING: f64 = 6.25;
        pub const WINDOW_MEASURE_ROW_MIN_UI_SPACING: f64 = 5.625;
    }
}

pub mod canvas_fonts {
    pub const MAP_LABEL_SANS_FALLBACK: &str = "sans-serif";
    pub const NOTO_COLOR_EMOJI: &str = "Noto Color Emoji";
}

pub struct BoardTheme {
    pub raster_clear: [f32; 4],
    pub grid_minor_stroke: [f32; 4],
    pub edge_stroke: [f32; 4],
    pub edge_stroke_hovered: [f32; 4],
    pub edge_stroke_selected: [f32; 4],
    pub edge_stroke_selection_exit: [f32; 4],
    pub edge_stroke_disabled: [f32; 4],
    pub node_fill: [f32; 4],
    pub node_stroke: [f32; 4],
    pub node_fill_hovered: [f32; 4],
    pub node_stroke_hovered: [f32; 4],
    pub node_fill_selected: [f32; 4],
    pub node_stroke_selected: [f32; 4],
    pub node_fill_selection_exit: [f32; 4],
    pub node_stroke_selection_exit: [f32; 4],
    pub node_fill_disabled: [f32; 4],
    pub node_stroke_disabled: [f32; 4],
    pub indirect_handle_fill: [f32; 4],
    pub indirect_handle_stroke: [f32; 4],
    pub handle_fill: [f32; 4],
    pub handle_stroke: [f32; 4],
    pub handle_fill_hovered: [f32; 4],
    pub handle_stroke_hovered: [f32; 4],
    pub handle_fill_selected: [f32; 4],
    pub handle_stroke_selected: [f32; 4],
    pub handle_fill_selection_exit: [f32; 4],
    pub handle_stroke_selection_exit: [f32; 4],
    pub handle_fill_disabled: [f32; 4],
    pub handle_stroke_disabled: [f32; 4],
    pub wire_stroke: [f32; 4],
    pub wire_stroke_hovered: [f32; 4],
    pub wire_stroke_selected: [f32; 4],
    pub wire_stroke_highlighted: [f32; 4],
    pub wire_stroke_disabled: [f32; 4],
    pub selection_preview_fill: [f32; 4],
    pub selection_preview_stroke: [f32; 4],
    pub label_fill: [f32; 4],
    pub label_fill_hovered: [f32; 4],
    pub label_halo: [f32; 4],
}

pub const BOARD_LIGHT: BoardTheme = BoardTheme {
    raster_clear: [0.87136712_f32, 0.83879901_f32, 0.72305513_f32, 1_f32],
    grid_minor_stroke: [0.19806932_f32, 0.22322796_f32, 0.20507874_f32, 0.21960784_f32],
    edge_stroke: [0.19806932_f32, 0.22322796_f32, 0.20507874_f32, 1_f32],
    edge_stroke_hovered: [0_f32, 0.00560539_f32, 0.00856813_f32, 1_f32],
    edge_stroke_selected: [1_f32, 0.03433981_f32, 0.07818742_f32, 1_f32],
    edge_stroke_selection_exit: [0.03433981_f32, 0.63759687_f32, 0.52099557_f32, 1_f32],
    edge_stroke_disabled: [0.02842604_f32, 0.03071344_f32, 0.02955683_f32, 0.38039216_f32],
    node_fill: [0.85499261_f32, 0.82278575_f32, 0.70837578_f32, 1_f32],
    node_stroke: [0.19806932_f32, 0.22322796_f32, 0.20507874_f32, 1_f32],
    node_fill_hovered: [0.19806932_f32, 0.22322796_f32, 0.20507874_f32, 1_f32],
    node_stroke_hovered: [0_f32, 0.00560539_f32, 0.00856813_f32, 1_f32],
    node_fill_selected: [1_f32, 0.03433981_f32, 0.07818742_f32, 1_f32],
    node_stroke_selected: [1_f32, 0.03433981_f32, 0.07818742_f32, 1_f32],
    node_fill_selection_exit: [0.37626212_f32, 0.59061884_f32, 0.50888132_f32, 1_f32],
    node_stroke_selection_exit: [0.03433981_f32, 0.63759687_f32, 0.52099557_f32, 1_f32],
    node_fill_disabled: [0.13013648_f32, 0.12743768_f32, 0.11443537_f32, 0.50196078_f32],
    node_stroke_disabled: [0.02842604_f32, 0.03071344_f32, 0.02955683_f32, 0.38039216_f32],
    indirect_handle_fill: [0.5520114_f32, 0.77582222_f32, 0.6653873_f32, 1_f32],
    indirect_handle_stroke: [0.03433981_f32, 0.63759687_f32, 0.52099557_f32, 1_f32],
    handle_fill: [0.93011086_f32, 0.89626935_f32, 0.76815115_f32, 0_f32],
    handle_stroke: [0.19806932_f32, 0.22322796_f32, 0.20507874_f32, 1_f32],
    handle_fill_hovered: [0.19806932_f32, 0.22322796_f32, 0.20507874_f32, 1_f32],
    handle_stroke_hovered: [0.19806932_f32, 0.22322796_f32, 0.20507874_f32, 1_f32],
    handle_fill_selected: [1_f32, 0.03433981_f32, 0.07818742_f32, 1_f32],
    handle_stroke_selected: [1_f32, 0.03433981_f32, 0.07818742_f32, 1_f32],
    handle_fill_selection_exit: [0.37626212_f32, 0.59061884_f32, 0.50888132_f32, 1_f32],
    handle_stroke_selection_exit: [0.03433981_f32, 0.63759687_f32, 0.52099557_f32, 1_f32],
    handle_fill_disabled: [0.13013648_f32, 0.12743768_f32, 0.11443537_f32, 0.50196078_f32],
    handle_stroke_disabled: [0.02842604_f32, 0.03071344_f32, 0.02955683_f32, 0.38039216_f32],
    wire_stroke: [0.19806932_f32, 0.22322796_f32, 0.20507874_f32, 1_f32],
    wire_stroke_hovered: [0_f32, 0.00560539_f32, 0.00856813_f32, 1_f32],
    wire_stroke_selected: [1_f32, 0.03433981_f32, 0.07818742_f32, 1_f32],
    wire_stroke_highlighted: [0.03433981_f32, 0.63759687_f32, 0.52099557_f32, 1_f32],
    wire_stroke_disabled: [0.02842604_f32, 0.03071344_f32, 0.02955683_f32, 0.38039216_f32],
    selection_preview_fill: [0.01370208_f32, 0.00182116_f32, 0.00273174_f32, 0.12156863_f32],
    selection_preview_stroke: [1_f32, 0.03433981_f32, 0.07818742_f32, 0.70196078_f32],
    label_fill: [0.19806932_f32, 0.22322796_f32, 0.20507874_f32, 1_f32],
    label_fill_hovered: [0_f32, 0.00560539_f32, 0.00856813_f32, 1_f32],
    label_halo: [0.87136712_f32, 0.83879901_f32, 0.72305513_f32, 0.78039216_f32],
};

pub const BOARD_DARK: BoardTheme = BoardTheme {
    raster_clear: [0.00367651_f32, 0.01161225_f32, 0.01520851_f32, 1_f32],
    grid_minor_stroke: [0.09084171_f32, 0.11443537_f32, 0.10946171_f32, 0.21960784_f32],
    edge_stroke: [0.36130678_f32, 0.37626212_f32, 0.33716362_f32, 1_f32],
    edge_stroke_hovered: [0.93011086_f32, 0.89626935_f32, 0.76815115_f32, 1_f32],
    edge_stroke_selected: [1_f32, 0.03433981_f32, 0.07818742_f32, 1_f32],
    edge_stroke_selection_exit: [0.03433981_f32, 0.63759687_f32, 0.52099557_f32, 1_f32],
    edge_stroke_disabled: [0.01444384_f32, 0.01764195_f32, 0.01680738_f32, 0.38039216_f32],
    node_fill: [0.02732089_f32, 0.0451862_f32, 0.04666509_f32, 1_f32],
    node_stroke: [0.36130678_f32, 0.37626212_f32, 0.33716362_f32, 1_f32],
    node_fill_hovered: [0.09084171_f32, 0.11443537_f32, 0.10946171_f32, 1_f32],
    node_stroke_hovered: [0.93011086_f32, 0.89626935_f32, 0.76815115_f32, 1_f32],
    node_fill_selected: [1_f32, 0.03433981_f32, 0.07818742_f32, 1_f32],
    node_stroke_selected: [1_f32, 0.03433981_f32, 0.07818742_f32, 1_f32],
    node_fill_selection_exit: [0.02842604_f32, 0.11697067_f32, 0.1070231_f32, 1_f32],
    node_stroke_selection_exit: [0.03433981_f32, 0.63759687_f32, 0.52099557_f32, 1_f32],
    node_fill_disabled: [0.00856813_f32, 0.01298303_f32, 0.01370208_f32, 0.50196078_f32],
    node_stroke_disabled: [0.01444384_f32, 0.01764195_f32, 0.01680738_f32, 0.38039216_f32],
    indirect_handle_fill: [0.02842604_f32, 0.11697067_f32, 0.1070231_f32, 1_f32],
    indirect_handle_stroke: [0.03433981_f32, 0.63759687_f32, 0.52099557_f32, 1_f32],
    handle_fill: [0.00367651_f32, 0.01161225_f32, 0.01520851_f32, 0_f32],
    handle_stroke: [0.36130678_f32, 0.37626212_f32, 0.33716362_f32, 1_f32],
    handle_fill_hovered: [0.09084171_f32, 0.11443537_f32, 0.10946171_f32, 1_f32],
    handle_stroke_hovered: [0.36130678_f32, 0.37626212_f32, 0.33716362_f32, 1_f32],
    handle_fill_selected: [1_f32, 0.03433981_f32, 0.07818742_f32, 1_f32],
    handle_stroke_selected: [1_f32, 0.03433981_f32, 0.07818742_f32, 1_f32],
    handle_fill_selection_exit: [0.02842604_f32, 0.11697067_f32, 0.1070231_f32, 1_f32],
    handle_stroke_selection_exit: [0.03433981_f32, 0.63759687_f32, 0.52099557_f32, 1_f32],
    handle_fill_disabled: [0.00856813_f32, 0.01298303_f32, 0.01370208_f32, 0.50196078_f32],
    handle_stroke_disabled: [0.01444384_f32, 0.01764195_f32, 0.01680738_f32, 0.38039216_f32],
    wire_stroke: [0.36130678_f32, 0.37626212_f32, 0.33716362_f32, 1_f32],
    wire_stroke_hovered: [0.93011086_f32, 0.89626935_f32, 0.76815115_f32, 1_f32],
    wire_stroke_selected: [1_f32, 0.03433981_f32, 0.07818742_f32, 1_f32],
    wire_stroke_highlighted: [0.03433981_f32, 0.63759687_f32, 0.52099557_f32, 1_f32],
    wire_stroke_disabled: [0.01444384_f32, 0.01764195_f32, 0.01680738_f32, 0.38039216_f32],
    selection_preview_fill: [0.01370208_f32, 0.00182116_f32, 0.00273174_f32, 0.12156863_f32],
    selection_preview_stroke: [1_f32, 0.03433981_f32, 0.07818742_f32, 0.70196078_f32],
    label_fill: [0.65140564_f32, 0.64447968_f32, 0.55834039_f32, 1_f32],
    label_fill_hovered: [0.93011086_f32, 0.89626935_f32, 0.76815115_f32, 1_f32],
    label_halo: [0.00367651_f32, 0.01161225_f32, 0.01520851_f32, 0.78039216_f32],
};

pub struct MapTheme {
    pub surface_clear: [f32; 4],
    pub land_fill: [f32; 4],
    pub land_stroke: [f32; 4],
    pub label_fill: [f32; 4],
    pub label_halo: [f32; 4],
    pub region_fill: [f32; 4],
    pub region_stroke: [f32; 4],
    pub route_stroke: [f32; 4],
    pub position_fill: [f32; 4],
    pub position_stroke: [f32; 4],
}

pub const MAP_LIGHT: MapTheme = MapTheme {
    surface_clear: [0.00367651_f32, 0.01161225_f32, 0.01520851_f32, 1_f32],
    land_fill: [0.02732089_f32, 0.0451862_f32, 0.04666509_f32, 1_f32],
    land_stroke: [0.03310477_f32, 0.05126946_f32, 0.05286065_f32, 0.41960784_f32],
    label_fill: [0.93011086_f32, 0.89626935_f32, 0.76815115_f32, 1_f32],
    label_halo: [0.00367651_f32, 0.01161225_f32, 0.01520851_f32, 0.92156863_f32],
    region_fill: [0.03433981_f32, 0.63759687_f32, 0.52099557_f32, 0.21960784_f32],
    region_stroke: [0.03433981_f32, 0.63759687_f32, 0.52099557_f32, 0.90196078_f32],
    route_stroke: [0.95597335_f32, 0.30054379_f32, 0_f32, 0.92156863_f32],
    position_fill: [1_f32, 0.03433981_f32, 0.07818742_f32, 1_f32],
    position_stroke: [0.93011086_f32, 0.89626935_f32, 0.76815115_f32, 1_f32],
};

pub const MAP_DARK: MapTheme = MapTheme {
    surface_clear: [0.00182116_f32, 0.00856813_f32, 0.01161225_f32, 1_f32],
    land_fill: [0.00367651_f32, 0.01161225_f32, 0.01520851_f32, 1_f32],
    land_stroke: [0.03310477_f32, 0.05126946_f32, 0.05286065_f32, 0.41960784_f32],
    label_fill: [0.93011086_f32, 0.89626935_f32, 0.76815115_f32, 1_f32],
    label_halo: [0.00182116_f32, 0.00856813_f32, 0.01161225_f32, 0.92156863_f32],
    region_fill: [0.03433981_f32, 0.63759687_f32, 0.52099557_f32, 0.21960784_f32],
    region_stroke: [0.03433981_f32, 0.63759687_f32, 0.52099557_f32, 0.90196078_f32],
    route_stroke: [0.95597335_f32, 0.30054379_f32, 0_f32, 0.92156863_f32],
    position_fill: [1_f32, 0.03433981_f32, 0.07818742_f32, 1_f32],
    position_stroke: [0.93011086_f32, 0.89626935_f32, 0.76815115_f32, 1_f32],
};

pub struct CanvasTheme {
    pub raster_clear: [f32; 4],
    pub icon_fg: [f32; 4],
    pub icon_bg: [f32; 4],
    pub label_fill: [f32; 4],
    pub label_halo: [f32; 4],
}

pub const CANVAS_LIGHT: CanvasTheme = CanvasTheme {
    raster_clear: [0.87136712_f32, 0.83879901_f32, 0.72305513_f32, 1_f32],
    icon_fg: [0_f32, 0_f32, 0_f32, 1_f32],
    icon_bg: [1_f32, 1_f32, 1_f32, 1_f32],
    label_fill: [0_f32, 0.00560539_f32, 0.00856813_f32, 1_f32],
    label_halo: [0.93011086_f32, 0.89626935_f32, 0.76815115_f32, 1_f32],
};

pub const CANVAS_DARK: CanvasTheme = CanvasTheme {
    raster_clear: [0.00367651_f32, 0.01161225_f32, 0.01520851_f32, 1_f32],
    icon_fg: [0.93011086_f32, 0.89626935_f32, 0.76815115_f32, 1_f32],
    icon_bg: [0_f32, 0.00560539_f32, 0.00856813_f32, 1_f32],
    label_fill: [0.93011086_f32, 0.89626935_f32, 0.76815115_f32, 1_f32],
    label_halo: [0.00367651_f32, 0.01161225_f32, 0.01520851_f32, 1_f32],
};
