//! 🖼️ The flat-surface scene payloads (canvas, paint, board, tiled map, ink canvas) with the canvas framing and snapshot lease.
//!
//! Carved out of the former `semio-framework-ui-scene` scenes module by ticket `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`.

use protocol::value::{DslValue, FromValue, ToValue, ValueError};
use semio_framework_ui_scene_doc::{
    deserialize_non_empty_interaction_domain_text, scene_lane_hash, scene_pack_wire, value_decode, value_decode_default, value_decode_option, value_push, value_push_if_nonempty, value_push_option, SceneDoc, SceneLanePayload, SceneLaneRef,
};
use semio_framework_ui_scene_pack::{from_bytes, to_bytes, PackError};
use serde::{Deserialize, Serialize};
#[path = "../../📷️framing/🦀️.rs"]
mod framing;
pub use framing::{Canvas2dFrameCamera, Canvas2dFraming};
#[path = "../../🖼️canvas2d-snapshot/🦀️.rs"]
mod canvas2d_snapshot;
pub use canvas2d_snapshot::{
    canvas2d_snapshot_abort_write, canvas2d_snapshot_abort_write_step, canvas2d_snapshot_admit_page, canvas2d_snapshot_begin, canvas2d_snapshot_begin_close, canvas2d_snapshot_close_step, canvas2d_snapshot_seal, canvas2d_snapshot_terminal_is_empty,
    canvas2d_snapshot_with_page, canvas2d_snapshot_write_terminal_is_empty, Canvas2dRejectedSnapshotPage, Canvas2dSnapshotDescriptor, Canvas2dSnapshotFault, Canvas2dSnapshotLease, Canvas2dSnapshotPage, Canvas2dSnapshotWriteToken,
    CANVAS2D_SNAPSHOT_CAPACITY, CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY, CANVAS2D_SNAPSHOT_PAGE_CAPACITY,
};

//#region 🔖️Canvas2dScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Canvas2dScene {
    pub camera_x: f64,
    pub camera_y: f64,
    pub zoom: f64,
    pub layers_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub framing: Option<Canvas2dFraming>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Canvas2dSnapshotLease>,
    /// ⏯️ The base64url `ToolRunTraceDelta` paged to this window — the 2d twin of
    /// [`World3dScene::tool_run_trace`], carried outside the doc as [`Canvas2dSceneLane::ToolRunTrace`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_run_trace: Option<String>,
    /// 🚚️ The spine's lane manifest — see [`World3dScene::lanes`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lanes: Vec<SceneLaneRef>,
}

impl Canvas2dScene {
    /** 🖼️ Builds a canvas-2d scene with every optional extension unset. */
    pub fn base(camera_x: f64, camera_y: f64, zoom: f64, layers_json: String) -> Self {
        Self { camera_x, camera_y, zoom, layers_json, framing: None, snapshot: None, tool_run_trace: None, lanes: Vec::new() }
    }
}

scene_pack_wire!(Canvas2dScenePack, Canvas2dScene {
    camera_x: f64,
    camera_y: f64,
    zoom: f64,
    layers_json: String,
    framing: Option<Canvas2dFraming>,
    snapshot: Option<Canvas2dSnapshotLease>,
    tool_run_trace: Option<String>,
    #[serde(default)]
    lanes: Vec<SceneLaneRef>,
});

impl SceneDoc for Canvas2dScene {
    const SCHEMA: &'static str = "canvas-2d@1";

    fn encode_pack(&self) -> Result<Vec<u8>, PackError> {
        to_bytes(&Canvas2dScenePack::from(self))
    }

    fn decode_pack(bytes: &[u8]) -> Result<Self, PackError> {
        from_bytes::<Canvas2dScenePack>(bytes).map(Into::into)
    }

    fn split_lanes(&self) -> (Self, Vec<SceneLanePayload>) {
        let mut spine = self.clone();
        let mut lanes = Vec::new();
        let mut refs = Vec::new();
        for lane in Canvas2dSceneLane::ALL {
            let Some(payload) = lane.take(&mut spine) else { continue };
            refs.push(SceneLaneRef { lane: lane.name().to_string(), bytes: payload.len() as u32, hash: scene_lane_hash(&payload) });
            lanes.push(SceneLanePayload { key: lane.body_key(), payload });
        }
        spine.lanes = refs;
        (spine, lanes)
    }

    fn merge_lane(&mut self, key: &str, payload: String) -> bool {
        let Some(lane) = Canvas2dSceneLane::from_body_key(key) else { return false };
        lane.put(self, payload);
        true
    }
}

impl ToValue for Canvas2dScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "cameraX", &self.camera_x);
        value_push(&mut entries, "cameraY", &self.camera_y);
        value_push(&mut entries, "zoom", &self.zoom);
        value_push(&mut entries, "layersJson", &self.layers_json);
        value_push_option(&mut entries, "framing", &self.framing);
        value_push_option(&mut entries, "snapshot", &self.snapshot);
        value_push_option(&mut entries, "toolRunTrace", &self.tool_run_trace);
        value_push_if_nonempty(&mut entries, "lanes", &self.lanes);
        DslValue::Object(entries)
    }
}

impl FromValue for Canvas2dScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            camera_x: value_decode(&entries, "cameraX")?,
            camera_y: value_decode(&entries, "cameraY")?,
            zoom: value_decode(&entries, "zoom")?,
            layers_json: value_decode(&entries, "layersJson")?,
            framing: value_decode_option(&entries, "framing")?,
            snapshot: value_decode_option(&entries, "snapshot")?,
            tool_run_trace: value_decode_option(&entries, "toolRunTrace")?,
            lanes: value_decode_default(&entries, "lanes", Vec::new)?,
        })
    }
}

/// 🚚️ The canvas-2d payload fields that ride OUTSIDE the fixed-capacity surface doc — the 2d twin of
/// [`World3dSceneLane`], pinned against `🧪️tests/🧫️fixtures/🚚️canvas2d-scene-lanes/🔣️.json` on both sides.
/// `layers_json` stays in the spine: it is a bounded per-frame descriptor, while a trace delta scales
/// with the run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Canvas2dSceneLane {
    ToolRunTrace,
}

/// 🚚️ Reserved carrier-key namespace of the canvas-2d lanes.
pub const CANVAS2D_SCENE_LANE_KEY_PREFIX: &str = "framework.scene.canvas2d.";

/// 🚚️ Wire name of each [`Canvas2dSceneLane`], in `Canvas2dSceneLane::ALL` order.
pub const CANVAS2D_SCENE_LANE_NAMES: [&str; 1] = ["toolRunTrace"];

/// 🚚️ [`Canvas2dScene`] field each lane carries, spelled as its serialized (camelCase) name.
pub const CANVAS2D_SCENE_LANE_FIELDS: [&str; 1] = ["toolRunTrace"];

/// 🚚️ Reserved carrier key of each lane.
pub const CANVAS2D_SCENE_LANE_BODY_KEYS: [&str; 1] = ["framework.scene.canvas2d.toolRunTrace"];

/// 🚚️ Whether each lane's [`Canvas2dScene`] field is an `Option<String>`.
pub const CANVAS2D_SCENE_LANE_OPTIONAL: [bool; 1] = [true];

impl Canvas2dSceneLane {
    pub const ALL: [Self; 1] = [Self::ToolRunTrace];

    /// 🏷️ See [`CANVAS2D_SCENE_LANE_NAMES`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn name(self) -> &'static str {
        CANVAS2D_SCENE_LANE_NAMES[self as usize]
    }

    /// 🏷️ See [`CANVAS2D_SCENE_LANE_FIELDS`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn field(self) -> &'static str {
        CANVAS2D_SCENE_LANE_FIELDS[self as usize]
    }

    /// 🪧️ See [`CANVAS2D_SCENE_LANE_BODY_KEYS`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn body_key(self) -> &'static str {
        CANVAS2D_SCENE_LANE_BODY_KEYS[self as usize]
    }

    /// 🏷️ See [`CANVAS2D_SCENE_LANE_OPTIONAL`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn optional(self) -> bool {
        CANVAS2D_SCENE_LANE_OPTIONAL[self as usize]
    }

    /// 🔎️ Resolves a carrier root key back to its lane.
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn from_body_key(body_key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.body_key() == body_key)
    }

    /// 🔎️ Resolves a lane wire name back to its lane.
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.name() == name)
    }

    /// 📤️ Removes this lane's payload from `scene`; `None` when the optional lane is unset.
    // 🚫️async: E6 sync payload construction — see this module's own header.
    pub fn take(self, scene: &mut Canvas2dScene) -> Option<String> {
        match self {
            Self::ToolRunTrace => scene.tool_run_trace.take(),
        }
    }

    /// 📥️ Writes this lane's payload back into `scene` — the inverse of [`Canvas2dSceneLane::take`].
    // 🚫️async: E6 sync payload construction — see this module's own header.
    pub fn put(self, scene: &mut Canvas2dScene, payload: String) {
        match self {
            Self::ToolRunTrace => scene.tool_run_trace = Some(payload),
        }
    }
}
//#endregion 🔖️Canvas2dScene

//#region 🔖️Paint2dScene
/** 🖼️ Paint-2d scene: WASM `RasterSession` sync channels for the composite/navigator windows. */
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paint2dScene {
    pub document_sync_json: String,
    pub assets_json: String,
    pub camera_json: String,
    pub selection_json: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixel_selection_json:Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hovered_id: Option<String>,
    pub active_utility: String,
    pub brush_size: f64,
    pub brush_opacity: f64,
    pub brush_color: String,
    pub brush_hardness: f64,
    pub paint_target:String,
    pub mask_value:u32,
    /// 🪣️ The session colour tolerance (0..255) every host's bucket and wand floods with.
    pub fill_tolerance:u32,
    pub view_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_viewport_json: Option<String>,
    /// 🚚️ The spine's lane manifest — see [`Paint2dSceneLane`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lanes: Vec<SceneLaneRef>,
}

impl SceneDoc for Paint2dScene {
    const SCHEMA: &'static str = "paint-2d@1";

    fn split_lanes(&self) -> (Self, Vec<SceneLanePayload>) {
        let mut spine = self.clone();
        let mut lanes = Vec::new();
        let mut refs = Vec::new();
        for lane in Paint2dSceneLane::ALL {
            let Some(payload) = lane.take(&mut spine) else { continue };
            refs.push(SceneLaneRef { lane: lane.name().to_string(), bytes: payload.len() as u32, hash: scene_lane_hash(&payload) });
            lanes.push(SceneLanePayload { key: lane.body_key(), payload });
        }
        spine.lanes = refs;
        (spine, lanes)
    }

    fn merge_lane(&mut self, key: &str, payload: String) -> bool {
        let Some(lane) = Paint2dSceneLane::from_body_key(key) else { return false };
        lane.put(self, payload);
        true
    }
}

/// 🚚️ The paint-2d payload fields that ride OUTSIDE the fixed-capacity surface doc — the raster twin
/// of [`Canvas2dSceneLane`]/[`Board2dSceneLane`], pinned against
/// `🧪️tests/🧫️fixtures/🚚️paint2d-scene-lanes/🔣️.json` on both sides.
///
/// Document, assets and completed pixel coverage can exceed the fixed scene header capacity.
/// Camera, layer selection and brush settings remain in the bounded header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Paint2dSceneLane {
    DocumentSync,
    Assets,
    PixelSelection,
}

/// 🚚️ Reserved carrier-key namespace of the paint-2d lanes.
pub const PAINT2D_SCENE_LANE_KEY_PREFIX: &str = "framework.scene.paint2d.";

/// 🚚️ Wire name of each [`Paint2dSceneLane`], in `Paint2dSceneLane::ALL` order.
pub const PAINT2D_SCENE_LANE_NAMES: [&str; 3] = ["documentSync", "assets", "pixelSelection"];

/// 🚚️ [`Paint2dScene`] field each lane carries, spelled as its serialized (camelCase) name.
pub const PAINT2D_SCENE_LANE_FIELDS: [&str; 3] = ["documentSyncJson", "assetsJson", "pixelSelectionJson"];

/// 🚚️ Reserved carrier key of each lane.
pub const PAINT2D_SCENE_LANE_BODY_KEYS: [&str; 3] = ["framework.scene.paint2d.documentSync", "framework.scene.paint2d.assets", "framework.scene.paint2d.pixelSelection"];

/// 🚚️ Whether each lane's [`Paint2dScene`] field is an `Option<String>`.
pub const PAINT2D_SCENE_LANE_OPTIONAL: [bool; 3] = [false, false, true];

impl Paint2dSceneLane {
    pub const ALL: [Self; 3] = [Self::DocumentSync, Self::Assets, Self::PixelSelection];

    /// 🏷️ See [`PAINT2D_SCENE_LANE_NAMES`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn name(self) -> &'static str {
        PAINT2D_SCENE_LANE_NAMES[self as usize]
    }

    /// 🏷️ See [`PAINT2D_SCENE_LANE_FIELDS`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn field(self) -> &'static str {
        PAINT2D_SCENE_LANE_FIELDS[self as usize]
    }

    /// 🪧️ See [`PAINT2D_SCENE_LANE_BODY_KEYS`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn body_key(self) -> &'static str {
        PAINT2D_SCENE_LANE_BODY_KEYS[self as usize]
    }

    /// 🏷️ See [`PAINT2D_SCENE_LANE_OPTIONAL`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn optional(self) -> bool {
        PAINT2D_SCENE_LANE_OPTIONAL[self as usize]
    }

    /// 🔎️ Resolves a carrier root key back to its lane.
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn from_body_key(body_key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.body_key() == body_key)
    }

    /// 🔎️ Resolves a lane wire name back to its lane.
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.name() == name)
    }

    /// 📤️ Removes this lane's payload from `scene`.
    // 🚫️async: E6 sync payload construction — see this module's own header.
    pub fn take(self, scene: &mut Paint2dScene) -> Option<String> {
        match self {
            Self::DocumentSync => Some(std::mem::take(&mut scene.document_sync_json)),
            Self::Assets => Some(std::mem::take(&mut scene.assets_json)),
            Self::PixelSelection=>scene.pixel_selection_json.take(),
        }
    }

    /// 📥️ Writes this lane's payload back into `scene` — the inverse of [`Paint2dSceneLane::take`].
    // 🚫️async: E6 sync payload construction — see this module's own header.
    pub fn put(self, scene: &mut Paint2dScene, payload: String) {
        match self {
            Self::DocumentSync => scene.document_sync_json = payload,
            Self::Assets => scene.assets_json = payload,
            Self::PixelSelection=>scene.pixel_selection_json=Some(payload),
        }
    }
}

impl ToValue for Paint2dScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "documentSyncJson", &self.document_sync_json);
        value_push(&mut entries, "assetsJson", &self.assets_json);
        value_push(&mut entries, "cameraJson", &self.camera_json);
        value_push(&mut entries, "selectionJson", &self.selection_json);
        value_push_option(&mut entries,"pixelSelectionJson",&self.pixel_selection_json);
        value_push_option(&mut entries, "hoveredId", &self.hovered_id);
        value_push(&mut entries, "activeUtility", &self.active_utility);
        value_push(&mut entries, "brushSize", &self.brush_size);
        value_push(&mut entries, "brushOpacity", &self.brush_opacity);
        value_push(&mut entries, "brushColor", &self.brush_color);
        value_push(&mut entries, "brushHardness", &self.brush_hardness);
        value_push(&mut entries,"paintTarget",&self.paint_target);
        value_push(&mut entries,"maskValue",&self.mask_value);
        value_push(&mut entries, "fillTolerance", &self.fill_tolerance);
        value_push(&mut entries, "viewMode", &self.view_mode);
        value_push_option(&mut entries, "compositeViewportJson", &self.composite_viewport_json);
        value_push_if_nonempty(&mut entries, "lanes", &self.lanes);
        DslValue::Object(entries)
    }
}

impl FromValue for Paint2dScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            document_sync_json: value_decode(&entries, "documentSyncJson")?,
            assets_json: value_decode(&entries, "assetsJson")?,
            camera_json: value_decode(&entries, "cameraJson")?,
            selection_json: value_decode(&entries, "selectionJson")?,
            pixel_selection_json:value_decode_option(&entries,"pixelSelectionJson")?,
            hovered_id: value_decode_option(&entries, "hoveredId")?,
            active_utility: value_decode(&entries, "activeUtility")?,
            brush_size: value_decode(&entries, "brushSize")?,
            brush_opacity: value_decode(&entries, "brushOpacity")?,
            brush_color: value_decode(&entries, "brushColor")?,
            brush_hardness: value_decode(&entries, "brushHardness")?,
            paint_target:value_decode(&entries,"paintTarget")?,
            mask_value:value_decode(&entries,"maskValue")?,
            fill_tolerance: value_decode(&entries, "fillTolerance")?,
            view_mode: value_decode(&entries, "viewMode")?,
            composite_viewport_json: value_decode_option(&entries, "compositeViewportJson")?,
            lanes: value_decode_default(&entries, "lanes", Vec::new)?,
        })
    }
}
//#endregion 🔖️Paint2dScene

//#region 🔖️TiledMapScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TiledMapScene {
    pub map_descriptor_json: String,
    pub camera_json: String,
    #[serde(default = "tiled_map_default_render_mode")]
    pub render_mode: String,
    #[serde(default = "tiled_map_default_vector_style")]
    pub vector_style: String,
    #[serde(default = "tiled_map_default_lod_mode")]
    pub lod_mode: String,
    #[serde(default = "tiled_map_default_tile_url_template")]
    pub tile_url_template: String,
    #[serde(default = "tiled_map_default_vector_tile_url_template")]
    pub vector_tile_url_template: String,
    #[serde(default = "tiled_map_default_layer_visibility_json")]
    pub layer_visibility_json: String,
    #[serde(default = "tiled_map_default_layer_stroke_scale_json")]
    pub layer_stroke_scale_json: String,
    #[serde(default = "tiled_map_default_selection_json")]
    pub selection_json: String,
    #[serde(default = "tiled_map_default_hover_json")]
    pub hover_json: String,
    #[serde(default = "tiled_map_default_selection_method")]
    pub selection_method: String,
    #[serde(default = "tiled_map_default_selection_mode")]
    pub selection_mode: String,
    /// 🚚️ The spine's lane manifest — see [`TiledMapSceneLane`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lanes: Vec<SceneLaneRef>,
}

impl SceneDoc for TiledMapScene {
    const SCHEMA: &'static str = "tiled-map@1";

    fn split_lanes(&self) -> (Self, Vec<SceneLanePayload>) {
        let mut spine = self.clone();
        let mut lanes = Vec::new();
        let mut refs = Vec::new();
        for lane in TiledMapSceneLane::ALL {
            let Some(payload) = lane.take(&mut spine) else { continue };
            refs.push(SceneLaneRef { lane: lane.name().to_string(), bytes: payload.len() as u32, hash: scene_lane_hash(&payload) });
            lanes.push(SceneLanePayload { key: lane.body_key(), payload });
        }
        spine.lanes = refs;
        (spine, lanes)
    }

    fn merge_lane(&mut self, key: &str, payload: String) -> bool {
        let Some(lane) = TiledMapSceneLane::from_body_key(key) else { return false };
        lane.put(self, payload);
        true
    }
}

/// 🚚️ The tiled-map payload field that rides OUTSIDE the fixed-capacity surface doc — the map twin of
/// [`Board2dSceneLane`]/[`Paint2dSceneLane`], pinned against
/// `🧪️tests/🧫️fixtures/🚚️tiledmap-scene-lanes/🔣️.json` on both sides.
///
/// `map_descriptor_json` is the whole map descriptor (`{ overlays, policy }`: the overlay features
/// per overlay layer id plus the app's map policy), so it scales with the DOCUMENT, not with the
/// frame: a map app's 300 overlay features can encode to about 60 000 bytes, nearly twice
/// `UI_FIXED_BYTES` (32 KiB). A surface doc cannot page, so `scene_surface.encode` would refuse the
/// whole surface and the window would never publish — `ui.fixed-capacity: fixed UI admission failed
/// at scene-surface.encode`.
/// Everything else (camera, render/vector/LOD mode, tile templates, layer visibility and stroke
/// scales, selection, hover, selection method/mode) is a bounded per-frame descriptor and stays in
/// the spine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TiledMapSceneLane {
    MapDescriptor,
}

/// 🚚️ Reserved carrier-key namespace of the tiled-map lanes.
pub const TILEDMAP_SCENE_LANE_KEY_PREFIX: &str = "framework.scene.tiledmap.";

/// 🚚️ Wire name of each [`TiledMapSceneLane`], in `TiledMapSceneLane::ALL` order.
pub const TILEDMAP_SCENE_LANE_NAMES: [&str; 1] = ["mapDescriptor"];

/// 🚚️ [`TiledMapScene`] field each lane carries, spelled as its serialized (camelCase) name.
pub const TILEDMAP_SCENE_LANE_FIELDS: [&str; 1] = ["mapDescriptorJson"];

/// 🚚️ Reserved carrier key of each lane.
pub const TILEDMAP_SCENE_LANE_BODY_KEYS: [&str; 1] = ["framework.scene.tiledmap.mapDescriptor"];

/// 🚚️ Whether each lane's [`TiledMapScene`] field is an `Option<String>`.
pub const TILEDMAP_SCENE_LANE_OPTIONAL: [bool; 1] = [false];

impl TiledMapSceneLane {
    pub const ALL: [Self; 1] = [Self::MapDescriptor];

    /// 🏷️ See [`TILEDMAP_SCENE_LANE_NAMES`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn name(self) -> &'static str {
        TILEDMAP_SCENE_LANE_NAMES[self as usize]
    }

    /// 🏷️ See [`TILEDMAP_SCENE_LANE_FIELDS`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn field(self) -> &'static str {
        TILEDMAP_SCENE_LANE_FIELDS[self as usize]
    }

    /// 🪧️ See [`TILEDMAP_SCENE_LANE_BODY_KEYS`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn body_key(self) -> &'static str {
        TILEDMAP_SCENE_LANE_BODY_KEYS[self as usize]
    }

    /// 🏷️ See [`TILEDMAP_SCENE_LANE_OPTIONAL`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn optional(self) -> bool {
        TILEDMAP_SCENE_LANE_OPTIONAL[self as usize]
    }

    /// 🔎️ Resolves a carrier root key back to its lane.
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn from_body_key(body_key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.body_key() == body_key)
    }

    /// 🔎️ Resolves a lane wire name back to its lane.
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.name() == name)
    }

    /// 📤️ Removes this lane's payload from `scene`.
    // 🚫️async: E6 sync payload construction — see this module's own header.
    pub fn take(self, scene: &mut TiledMapScene) -> Option<String> {
        match self {
            Self::MapDescriptor => Some(std::mem::take(&mut scene.map_descriptor_json)),
        }
    }

    /// 📥️ Writes this lane's payload back into `scene` — the inverse of [`TiledMapSceneLane::take`].
    // 🚫️async: E6 sync payload construction — see this module's own header.
    pub fn put(self, scene: &mut TiledMapScene, payload: String) {
        match self {
            Self::MapDescriptor => scene.map_descriptor_json = payload,
        }
    }
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_render_mode() -> String {
    "combined".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_vector_style() -> String {
    "colored".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_lod_mode() -> String {
    "automatic".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_tile_url_template() -> String {
    "/osm/{z}/{x}/{y}.png".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_vector_tile_url_template() -> String {
    "/vt/{z}/{x}/{y}.pbf".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_layer_visibility_json() -> String {
    "{}".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_layer_stroke_scale_json() -> String {
    "{}".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_selection_json() -> String {
    "{}".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_hover_json() -> String {
    "null".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_selection_method() -> String {
    "rectangle".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn tiled_map_default_selection_mode() -> String {
    "default".into()
}

impl TiledMapScene {
    /** 🗺️ Builds a tiled map scene with optional extensions unset. */
    pub fn base(map_descriptor_json: String, camera_json: String) -> Self {
        Self {
            map_descriptor_json,
            camera_json,
            render_mode: tiled_map_default_render_mode(),
            vector_style: tiled_map_default_vector_style(),
            lod_mode: tiled_map_default_lod_mode(),
            tile_url_template: tiled_map_default_tile_url_template(),
            vector_tile_url_template: tiled_map_default_vector_tile_url_template(),
            layer_visibility_json: tiled_map_default_layer_visibility_json(),
            layer_stroke_scale_json: tiled_map_default_layer_stroke_scale_json(),
            selection_json: tiled_map_default_selection_json(),
            hover_json: tiled_map_default_hover_json(),
            selection_method: tiled_map_default_selection_method(),
            selection_mode: tiled_map_default_selection_mode(),
            lanes: Vec::new(),
        }
    }
}

impl ToValue for TiledMapScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "mapDescriptorJson", &self.map_descriptor_json);
        value_push(&mut entries, "cameraJson", &self.camera_json);
        value_push(&mut entries, "renderMode", &self.render_mode);
        value_push(&mut entries, "vectorStyle", &self.vector_style);
        value_push(&mut entries, "lodMode", &self.lod_mode);
        value_push(&mut entries, "tileUrlTemplate", &self.tile_url_template);
        value_push(&mut entries, "vectorTileUrlTemplate", &self.vector_tile_url_template);
        value_push(&mut entries, "layerVisibilityJson", &self.layer_visibility_json);
        value_push(&mut entries, "layerStrokeScaleJson", &self.layer_stroke_scale_json);
        value_push(&mut entries, "selectionJson", &self.selection_json);
        value_push(&mut entries, "hoverJson", &self.hover_json);
        value_push(&mut entries, "selectionMethod", &self.selection_method);
        value_push(&mut entries, "selectionMode", &self.selection_mode);
        value_push_if_nonempty(&mut entries, "lanes", &self.lanes);
        DslValue::Object(entries)
    }
}

impl FromValue for TiledMapScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            map_descriptor_json: value_decode(&entries, "mapDescriptorJson")?,
            camera_json: value_decode(&entries, "cameraJson")?,
            render_mode: value_decode_default(&entries, "renderMode", tiled_map_default_render_mode)?,
            vector_style: value_decode_default(&entries, "vectorStyle", tiled_map_default_vector_style)?,
            lod_mode: value_decode_default(&entries, "lodMode", tiled_map_default_lod_mode)?,
            tile_url_template: value_decode_default(&entries, "tileUrlTemplate", tiled_map_default_tile_url_template)?,
            vector_tile_url_template: value_decode_default(&entries, "vectorTileUrlTemplate", tiled_map_default_vector_tile_url_template)?,
            layer_visibility_json: value_decode_default(&entries, "layerVisibilityJson", tiled_map_default_layer_visibility_json)?,
            layer_stroke_scale_json: value_decode_default(&entries, "layerStrokeScaleJson", tiled_map_default_layer_stroke_scale_json)?,
            selection_json: value_decode_default(&entries, "selectionJson", tiled_map_default_selection_json)?,
            hover_json: value_decode_default(&entries, "hoverJson", tiled_map_default_hover_json)?,
            selection_method: value_decode_default(&entries, "selectionMethod", tiled_map_default_selection_method)?,
            selection_mode: value_decode_default(&entries, "selectionMode", tiled_map_default_selection_mode)?,
            lanes: value_decode_default(&entries, "lanes", Vec::new)?,
        })
    }
}
//#endregion 🔖️TiledMapScene

//#region 🔖️Board2dScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Board2dScene {
    pub snapshot_json: String,
    pub camera_json: String,
    #[serde(default = "board2d_default_glyph_catalogs_json")]
    pub glyph_catalogs_json: String,
    #[serde(default = "board2d_default_selection_json")]
    pub selection_json: String,
    /// 🔗️ The ids the board paints highlighted, as a JSON id array — what the open time-travel draft references
    /// (`InteractionView::draft_references`); `[]` highlights nothing.
    #[serde(default = "board2d_default_highlighted_ids_json", skip_serializing_if = "board2d_highlights_nothing")]
    pub highlighted_ids_json: String,
    #[serde(default)]
    pub interactive: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hovered_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_utility: Option<String>,
    #[serde(default = "board2d_default_selection_method")]
    pub selection_method: String,
    /// 👁️ Whether the board host strokes its world grid at all — independent of snapping.
    #[serde(default = "board2d_default_true")]
    pub grid_visible: bool,
    #[serde(default)]
    pub grid_snap_enabled: bool,
    #[serde(default = "board2d_default_grid_factor")]
    pub grid_factor: f64,
    /// 🎯️ Which granularity a pick may reach — the app's own selectable-kind filter.
    #[serde(default = "board2d_default_true")]
    pub selectable_nodes: bool,
    #[serde(default = "board2d_default_true")]
    pub selectable_edges: bool,
    #[serde(default = "board2d_default_true")]
    pub selectable_handles: bool,
    #[serde(default)]
    pub suggestion_offset: f64,
    #[serde(default = "board2d_default_brush_weights_json")]
    pub brush_weights_json: String,
    #[serde(default = "board2d_default_placement_compatibility_json")]
    pub placement_compatibility_json: String,
    #[serde(default = "board2d_default_lod_mode")]
    pub lod_mode: String,
    /// 🕹️ Which selection-gumball handles this board's select utility composes, as
    /// `{"move":bool,"rotate":bool}` — `None` leaves the engine's own default (both on). Scale is
    /// deliberately absent, same law as the puzzle-3d gumball.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform_flags: Option<String>,
    /// 🖍️ World extent one area-brush CLICK paints, as `{"width":f64,"height":f64}` — `None` leaves the
    /// engine's own default. A click-drag states its own rectangle and ignores this.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_brush_size: Option<String>,
    /// 🎯️ The framework interaction domain this board's picks and hovers belong to — the owning app's
    /// declared `InteractionDefinition.id`. `None` means the app declares none and the host publishes
    /// no `interactionHover` at all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// 🏷️ The owning domain's brush placement id prefixes in the board's wire form,
    /// `{"brushNodePrefix":…,"brushEdgePrefix":…}`, so a host the domain cannot configure directly (the os wgpu surface)
    /// mints the same ids as the domain's own boards. `None` leaves the host's installed naming.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement_naming: Option<String>,
    /// 💡️ The one-shot handle-suggestions popup this window has open, as
    /// `{"open":true,"x":…,"y":…,"windowId":…,"handleId":…,"hoveredIndex":…,"candidates":[…]}` —
    /// `None` is the closed state. It is a bounded page (`BOARD2D_SUGGESTION_MENU_CANDIDATE_PAGE`
    /// rows), because the surface doc it rides is fixed-capacity and a richly catalogued handle
    /// resolves arbitrarily many candidates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggestion_menu_json: Option<String>,
    /// ⏯️ The base64url `ToolRunTraceDelta` paged to this board — the board twin of
    /// [`Canvas2dScene::tool_run_trace`], carried outside the doc as [`Board2dSceneLane::ToolRunTrace`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_run_trace: Option<String>,
    /// 🚚️ The spine's lane manifest — see [`World3dScene::lanes`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lanes: Vec<SceneLaneRef>,
}

scene_pack_wire!(Board2dScenePack, Board2dScene {
    snapshot_json: String,
    camera_json: String,
    glyph_catalogs_json: String,
    selection_json: String,
    interactive: bool,
    hovered_id: Option<String>,
    active_utility: Option<String>,
    selection_method: String,
    grid_visible: bool,
    grid_snap_enabled: bool,
    grid_factor: f64,
    selectable_nodes: bool,
    selectable_edges: bool,
    selectable_handles: bool,
    suggestion_offset: f64,
    brush_weights_json: String,
    placement_compatibility_json: String,
    lod_mode: String,
    transform_flags: Option<String>,
    area_brush_size: Option<String>,
    domain_id: Option<String>,
    placement_naming: Option<String>,
    suggestion_menu_json: Option<String>,
    tool_run_trace: Option<String>,
    highlighted_ids_json: String,
    #[serde(default)]
    lanes: Vec<SceneLaneRef>,
});

impl SceneDoc for Board2dScene {
    const SCHEMA: &'static str = "board-2d@1";

    fn encode_pack(&self) -> Result<Vec<u8>, PackError> {
        to_bytes(&Board2dScenePack::from(self))
    }

    fn decode_pack(bytes: &[u8]) -> Result<Self, PackError> {
        from_bytes::<Board2dScenePack>(bytes).map(Into::into)
    }

    fn split_lanes(&self) -> (Self, Vec<SceneLanePayload>) {
        let mut spine = self.clone();
        let mut lanes = Vec::new();
        let mut refs = Vec::new();
        for lane in Board2dSceneLane::ALL {
            let Some(payload) = lane.take(&mut spine) else { continue };
            refs.push(SceneLaneRef { lane: lane.name().to_string(), bytes: payload.len() as u32, hash: scene_lane_hash(&payload) });
            lanes.push(SceneLanePayload { key: lane.body_key(), payload });
        }
        spine.lanes = refs;
        (spine, lanes)
    }

    fn merge_lane(&mut self, key: &str, payload: String) -> bool {
        let Some(lane) = Board2dSceneLane::from_body_key(key) else { return false };
        lane.put(self, payload);
        true
    }
}

/// 🚚️ The board-2d payload fields that ride OUTSIDE the fixed-capacity surface doc — the board twin of
/// [`Canvas2dSceneLane`], pinned against `🧪️tests/🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json` on both sides.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Board2dSceneLane {
    /// 🧩️ The whole fixture graph — the one payload that outgrows the 32 KiB surface doc (Nakagin's
    /// 180-node puzzle 2d board is ~100 KiB of JSON), so it always rides as its own paged carrier.
    Snapshot,
    ToolRunTrace,
}

/// 🚚️ Reserved carrier-key namespace of the board-2d lanes.
pub const BOARD2D_SCENE_LANE_KEY_PREFIX: &str = "framework.scene.board2d.";

/// 🚚️ Wire name of each [`Board2dSceneLane`], in `Board2dSceneLane::ALL` order.
pub const BOARD2D_SCENE_LANE_NAMES: [&str; 2] = ["snapshot", "toolRunTrace"];

/// 🚚️ [`Board2dScene`] field each lane carries, spelled as its serialized (camelCase) name.
pub const BOARD2D_SCENE_LANE_FIELDS: [&str; 2] = ["snapshotJson", "toolRunTrace"];

/// 🚚️ Reserved carrier key of each lane.
pub const BOARD2D_SCENE_LANE_BODY_KEYS: [&str; 2] = ["framework.scene.board2d.snapshot", "framework.scene.board2d.toolRunTrace"];

/// 🚚️ Whether each lane's [`Board2dScene`] field is an `Option<String>`.
pub const BOARD2D_SCENE_LANE_OPTIONAL: [bool; 2] = [false, true];

impl Board2dSceneLane {
    pub const ALL: [Self; 2] = [Self::Snapshot, Self::ToolRunTrace];

    /// 🏷️ See [`BOARD2D_SCENE_LANE_NAMES`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn name(self) -> &'static str {
        BOARD2D_SCENE_LANE_NAMES[self as usize]
    }

    /// 🏷️ See [`BOARD2D_SCENE_LANE_FIELDS`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn field(self) -> &'static str {
        BOARD2D_SCENE_LANE_FIELDS[self as usize]
    }

    /// 🪧️ See [`BOARD2D_SCENE_LANE_BODY_KEYS`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn body_key(self) -> &'static str {
        BOARD2D_SCENE_LANE_BODY_KEYS[self as usize]
    }

    /// 🏷️ See [`BOARD2D_SCENE_LANE_OPTIONAL`].
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn optional(self) -> bool {
        BOARD2D_SCENE_LANE_OPTIONAL[self as usize]
    }

    /// 🔎️ Resolves a carrier root key back to its lane.
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn from_body_key(body_key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.body_key() == body_key)
    }

    /// 🔎️ Resolves a lane wire name back to its lane.
    // 🚫️async: E1 pure table lookup — see R9.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|lane| lane.name() == name)
    }

    /// 📤️ Removes this lane's payload from `scene`; `None` when the optional lane is unset.
    // 🚫️async: E6 sync payload construction — see this module's own header.
    pub fn take(self, scene: &mut Board2dScene) -> Option<String> {
        match self {
            Self::Snapshot => Some(std::mem::take(&mut scene.snapshot_json)),
            Self::ToolRunTrace => scene.tool_run_trace.take(),
        }
    }

    /// 📥️ Writes this lane's payload back into `scene` — the inverse of [`Board2dSceneLane::take`].
    // 🚫️async: E6 sync payload construction — see this module's own header.
    pub fn put(self, scene: &mut Board2dScene, payload: String) {
        match self {
            Self::Snapshot => scene.snapshot_json = payload,
            Self::ToolRunTrace => scene.tool_run_trace = Some(payload),
        }
    }
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_glyph_catalogs_json() -> String {
    "{}".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_selection_json() -> String {
    "[]".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_highlighted_ids_json() -> String {
    "[]".into()
}
// 🚫️async: E1 pure predicate consumed by external-trait impls (serde skip) — see R9
pub fn board2d_highlights_nothing(highlighted_ids_json: &str) -> bool {
    highlighted_ids_json == "[]"
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_selection_method() -> String {
    "rectangle".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_grid_factor() -> f64 {
    1.0
}
/// 👁️ A board scene an older producer wrote carries neither the grid-visibility nor the
/// selectable-kind flags; both default ON so the absent field reads as the pre-filter behaviour.
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_true() -> bool {
    true
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_brush_weights_json() -> String {
    "{}".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_placement_compatibility_json() -> String {
    "[]".into()
}
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn board2d_default_lod_mode() -> String {
    "automatic".into()
}

impl Board2dScene {
    /** 🧩️ Builds a 2D board scene with optional extensions unset. */
    pub fn base(snapshot_json: String, camera_json: String, interactive: bool) -> Self {
        Self {
            snapshot_json,
            camera_json,
            glyph_catalogs_json: board2d_default_glyph_catalogs_json(),
            selection_json: board2d_default_selection_json(),
            highlighted_ids_json: board2d_default_highlighted_ids_json(),
            interactive,
            hovered_id: None,
            active_utility: None,
            selection_method: board2d_default_selection_method(),
            grid_visible: board2d_default_true(),
            grid_snap_enabled: false,
            grid_factor: board2d_default_grid_factor(),
            selectable_nodes: board2d_default_true(),
            selectable_edges: board2d_default_true(),
            selectable_handles: board2d_default_true(),
            suggestion_offset: 0.0,
            brush_weights_json: board2d_default_brush_weights_json(),
            placement_compatibility_json: board2d_default_placement_compatibility_json(),
            lod_mode: board2d_default_lod_mode(),
            domain_id: None,
            placement_naming: None,
            transform_flags: None,
            area_brush_size: None,
            suggestion_menu_json: None,
            tool_run_trace: None,
            lanes: Vec::new(),
        }
    }
}

impl ToValue for Board2dScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "snapshotJson", &self.snapshot_json);
        value_push(&mut entries, "cameraJson", &self.camera_json);
        value_push(&mut entries, "glyphCatalogsJson", &self.glyph_catalogs_json);
        value_push(&mut entries, "selectionJson", &self.selection_json);
        if !board2d_highlights_nothing(&self.highlighted_ids_json) {
            value_push(&mut entries, "highlightedIdsJson", &self.highlighted_ids_json);
        }
        value_push(&mut entries, "interactive", &self.interactive);
        value_push_option(&mut entries, "hoveredId", &self.hovered_id);
        value_push_option(&mut entries, "activeUtility", &self.active_utility);
        value_push(&mut entries, "selectionMethod", &self.selection_method);
        value_push(&mut entries, "gridVisible", &self.grid_visible);
        value_push(&mut entries, "gridSnapEnabled", &self.grid_snap_enabled);
        value_push(&mut entries, "gridFactor", &self.grid_factor);
        value_push(&mut entries, "selectableNodes", &self.selectable_nodes);
        value_push(&mut entries, "selectableEdges", &self.selectable_edges);
        value_push(&mut entries, "selectableHandles", &self.selectable_handles);
        value_push(&mut entries, "suggestionOffset", &self.suggestion_offset);
        value_push(&mut entries, "brushWeightsJson", &self.brush_weights_json);
        value_push(&mut entries, "placementCompatibilityJson", &self.placement_compatibility_json);
        value_push(&mut entries, "lodMode", &self.lod_mode);
        value_push_option(&mut entries, "transformFlags", &self.transform_flags);
        value_push_option(&mut entries, "areaBrushSize", &self.area_brush_size);
        value_push_option(&mut entries, "domainId", &self.domain_id);
        value_push_option(&mut entries, "placementNaming", &self.placement_naming);
        value_push_option(&mut entries, "suggestionMenuJson", &self.suggestion_menu_json);
        value_push_option(&mut entries, "toolRunTrace", &self.tool_run_trace);
        value_push_if_nonempty(&mut entries, "lanes", &self.lanes);
        DslValue::Object(entries)
    }
}

impl FromValue for Board2dScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            snapshot_json: value_decode(&entries, "snapshotJson")?,
            camera_json: value_decode(&entries, "cameraJson")?,
            glyph_catalogs_json: value_decode_default(&entries, "glyphCatalogsJson", board2d_default_glyph_catalogs_json)?,
            selection_json: value_decode_default(&entries, "selectionJson", board2d_default_selection_json)?,
            highlighted_ids_json: value_decode_default(&entries, "highlightedIdsJson", board2d_default_highlighted_ids_json)?,
            interactive: value_decode_default(&entries, "interactive", Default::default)?,
            hovered_id: value_decode_option(&entries, "hoveredId")?,
            active_utility: value_decode_option(&entries, "activeUtility")?,
            selection_method: value_decode_default(&entries, "selectionMethod", board2d_default_selection_method)?,
            grid_visible: value_decode_default(&entries, "gridVisible", board2d_default_true)?,
            grid_snap_enabled: value_decode_default(&entries, "gridSnapEnabled", Default::default)?,
            grid_factor: value_decode_default(&entries, "gridFactor", board2d_default_grid_factor)?,
            selectable_nodes: value_decode_default(&entries, "selectableNodes", board2d_default_true)?,
            selectable_edges: value_decode_default(&entries, "selectableEdges", board2d_default_true)?,
            selectable_handles: value_decode_default(&entries, "selectableHandles", board2d_default_true)?,
            suggestion_offset: value_decode_default(&entries, "suggestionOffset", Default::default)?,
            brush_weights_json: value_decode_default(&entries, "brushWeightsJson", board2d_default_brush_weights_json)?,
            placement_compatibility_json: value_decode_default(&entries, "placementCompatibilityJson", board2d_default_placement_compatibility_json)?,
            lod_mode: value_decode_default(&entries, "lodMode", board2d_default_lod_mode)?,
            transform_flags: value_decode_option(&entries, "transformFlags")?,
            area_brush_size: value_decode_option(&entries, "areaBrushSize")?,
            domain_id: value_decode_option(&entries, "domainId")?,
            placement_naming: value_decode_option(&entries, "placementNaming")?,
            suggestion_menu_json: value_decode_option(&entries, "suggestionMenuJson")?,
            tool_run_trace: value_decode_option(&entries, "toolRunTrace")?,
            lanes: value_decode_default(&entries, "lanes", Vec::new)?,
        })
    }
}
//#endregion 🔖️Board2dScene

//#region 🔖️InkCanvasScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InkCanvasInteractionDomain {
    #[serde(deserialize_with = "deserialize_non_empty_interaction_domain_text")]
    pub id: String,
    #[serde(deserialize_with = "deserialize_non_empty_interaction_domain_text")]
    pub granularity_id: String,
}

impl ToValue for InkCanvasInteractionDomain {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "id", &self.id);
        value_push(&mut entries, "granularityId", &self.granularity_id);
        DslValue::Object(entries)
    }
}

impl FromValue for InkCanvasInteractionDomain {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        if let Some((field, _)) = entries.iter().find(|(field, _)| field != "id" && field != "granularityId") {
            return Err(ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, format!("unknown field `{field}`")));
        }
        for field in ["id", "granularityId"] {
            if entries.iter().filter(|(candidate, _)| candidate == field).count() > 1 {
                return Err(ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, format!("duplicate field `{field}`")));
            }
        }
        let domain = Self { id: value_decode(&entries, "id")?, granularity_id: value_decode(&entries, "granularityId")? };
        if domain.id.is_empty() || domain.granularity_id.is_empty() {
            return Err(ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, "ink canvas interaction domain fields must be non-empty"));
        }
        Ok(domain)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InkCanvasScene {
    pub document_json: String,
    #[serde(default = "ink_canvas_default_selection_json")]
    pub selection_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hovered_id: Option<String>,
    pub active_utility: String,
    pub view_mode: String,
    #[serde(default)]
    pub interactive: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction_domain: Option<InkCanvasInteractionDomain>,
}

impl SceneDoc for InkCanvasScene {
    const SCHEMA: &'static str = "ink-canvas@1";
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn ink_canvas_default_selection_json() -> String {
    "[]".into()
}

impl InkCanvasScene {
    /** 🖊️ Builds an ink canvas scene with the default empty selection. */
    pub fn base(document_json: String, active_utility: String, view_mode: String, interactive: bool) -> Self {
        Self { document_json, selection_json: ink_canvas_default_selection_json(), hovered_id: None, active_utility, view_mode, interactive, interaction_domain: None }
    }
}

impl ToValue for InkCanvasScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "documentJson", &self.document_json);
        value_push(&mut entries, "selectionJson", &self.selection_json);
        value_push_option(&mut entries, "hoveredId", &self.hovered_id);
        value_push(&mut entries, "activeUtility", &self.active_utility);
        value_push(&mut entries, "viewMode", &self.view_mode);
        value_push(&mut entries, "interactive", &self.interactive);
        value_push_option(&mut entries, "interactionDomain", &self.interaction_domain);
        DslValue::Object(entries)
    }
}

impl FromValue for InkCanvasScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            document_json: value_decode(&entries, "documentJson")?,
            selection_json: value_decode_default(&entries, "selectionJson", ink_canvas_default_selection_json)?,
            hovered_id: value_decode_option(&entries, "hoveredId")?,
            active_utility: value_decode(&entries, "activeUtility")?,
            view_mode: value_decode(&entries, "viewMode")?,
            interactive: value_decode_default(&entries, "interactive", Default::default)?,
            interaction_domain: value_decode_option(&entries, "interactionDomain")?,
        })
    }
}
//#endregion 🔖️InkCanvasScene

impl ToValue for Canvas2dFraming {
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![("revision".into(),self.revision.to_value()),("bounds".into(),self.bounds.to_vec().to_value()),("padding".into(),self.padding.to_value())])
    }
}

impl FromValue for Canvas2dFraming {
    fn from_value(value: DslValue) -> Result<Self,ValueError> {
        let entries = value.into_object()?;
        let bounds: Vec<f64> = value_decode(&entries,"bounds")?;
        Ok(Self { revision: value_decode(&entries,"revision")?,bounds: bounds.try_into().map_err(|_| ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, "canvas framing requires four bounds"))?,padding: value_decode(&entries,"padding")? })
    }
}

#[cfg(test)]
#[path = "../../🧪️tests/🔬️plane-unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "../../🧪️tests/🔬️plane-value-round-trip/🦀️.rs"]
mod value_round_trip_tests;
