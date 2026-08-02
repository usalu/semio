//! 🧊️ Puzzle 3d app — DocumentApp impl, render, manifest (constitutional: ui).

use puzzle_3d::Puzzle3dProjection;
use puzzle_3d_engine::{BrushPlacePayload, Puzzle3dEngineCommand, Puzzle3dEngineOutcome, PrecomputeLane, Puzzle3dPrecomputeSession};
use puzzle_3d_op::{puzzle3d_document_delta_operations, Puzzle3dOperation, Puzzle3dPlayProjection};
use semio_framework_plugin::{
    apply_world3d_projection_action, apply_world3d_sun_action, build_world_3d_scene, create_window_layout, ActionArgDef, ActionArgOption, ActionDefinition, ActionEmit, ActionKind, DocumentApp, DocumentView, MeasureSelectItem, merge_world_selection_ids, mesh_from_kind, strip_engagement_prefix, SelectionSet, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_inspector_stepper_field, ui_inspector_toggle_field, ui_inspector_vec3_group, ui_text, ui_declarative_sections_to_tree, world3d_camera_projection_json, world3d_chunking_json, world3d_environment_json, world3d_mesh_id_from_url, world3d_meshes_json_from_kinds_and_urls, world3d_projection_action_moves_pose, world3d_projection_measures, world3d_projection_pose, world3d_scene_extended, world3d_selection_json, world3d_sun_measures, App, ActionDescriptor, MediaClass, MediaForm, MediaType, OsMediaCapability, OsMediaFormat, PanelGroup, ArtifactKindSpec,
    SurfaceKind, ToolRef, UtilityDefinition, UiFieldNode, UiGroupNode, UiInspectorFieldGroup, UiNode, UiPresence, UiTreeItemAction, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, WindowEngagement, WindowEngagementInput, WindowLayout, WindowLayoutAxisNode, WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowMeasure, WorldProjectionConfig, WorldSunConfig, is_de_locale, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, SET_ACTIVE_TOOL_ACTION_ID, SET_ACTIVE_UTILITY_ACTION_ID,
    IntroductionDefinition, IntroductionInteraction, IntroductionPlacement, IntroductionStepDefinition,
    window_element_id, panel_tab_element_id, panel_tab_first_draggable_element_id,
    ActionRef, AppLabelsOverlayExt, DialogDefinition, IconName,
};
use semio_framework_plugin::kernel::HostEffect;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{LazyLock, Mutex};

//#region 🔖️Constants
const PUZZLE3D_PLAY_APP_ID: &str = "puzzle3d-play";
const PUZZLE3D_PLAY_CONTROLLER_ID: &str = "puzzle3d-play";
const PUZZLE3D_PLAY_SURFACE_VIEWPORT: &str = "puzzle.3d.play.viewport";
const PUZZLE3D_PLAY_BODY_COMPOSITE: &str = "puzzle3d.play.composite";
const PUZZLE3D_PLAY_BODY_DOCUMENT: &str = "puzzle.3d.play.document";
const PUZZLE3D_PLAY_BODY_KINDS: &str = "puzzle.3d.play.kinds";
const PUZZLE3D_PLAY_BODY_INSPECTOR: &str = "puzzle.3d.play.inspector";
const PUZZLE3D_PLAY_BODY_SETTINGS: &str = "puzzle.3d.play.settings";
const PUZZLE3D_PLAY_WINDOW_MAIN: &str = "puzzle3d-main";
const PUZZLE3D_PLAY_WINDOW_TOP: &str = "puzzle3d-main-top";
const PUZZLE3D_PLAY_WINDOW_PERSPECTIVE: &str = "puzzle3d-main-perspective";
/// 🪟️ Display-template id for an orthographic top pane — mirrors `encodeWorldProjectionTemplateId({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } })`.
const PUZZLE3D_TEMPLATE_TOP: &str = r#"world-projection:{"mode":{"kind":"orthographic"},"orientation":{"type":"cardinal","view":"top"}}"#;
/// 🪟️ Display-template id for a three-point perspective pane — mirrors `encodeWorldProjectionTemplateId({ mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } })`.
const PUZZLE3D_TEMPLATE_PERSPECTIVE: &str = r#"world-projection:{"mode":{"kind":"threePoint","fov":50},"orientation":{"type":"free"}}"#;
const PUZZLE3D_FIXTURE_SCHEMA: &str = "puzzle.3d.fixture";
const PUZZLE3D_EXAMPLE_CONCRETE_FOREST: &str = "concrete-forest";
const PUZZLE3D_EXAMPLE_NAKAGIN: &str = "nakagin-capsule-tower";
const PUZZLE3D_FALLBACK_MESH_KIND: &str = "box";
/// 🧰️ Host-owned active utility (`view_state.active_utility_id`) when the host hasn't set one yet — none.
/// Transform gumball utility (`transform`) must be pressed explicitly; an unset/cleared utility must not fall back to `transform` or the gumball appears without an active transform tool.
const PUZZLE3D_DEFAULT_UTILITY: &str = "";
const PUZZLE3D_FILL_COUNT_MAX: u32 = 1000;
/// 🌀️ Window option: emit every object's vortices into the 3D scene.
const PUZZLE3D_VORTEX_SHOW_ALWAYS: &str = "always";
/// 🌀️ Window option: emit vortices only for hovered/selected objects (and vortex-only hover/selection).
const PUZZLE3D_VORTEX_SHOW_SELECTED: &str = "selected";
/// 🧭️ Window option: arrow tip points away from the vortex point along `direction`.
const PUZZLE3D_VORTEX_DIRECTION_OUTWARDS: &str = "outwards";
/// 🧭️ Window option: arrow tip ends on the vortex point; shaft starts at `point - direction * length`.
const PUZZLE3D_VORTEX_DIRECTION_INWARDS: &str = "inwards";

const CONCRETE_FOREST_EXAMPLE_DSL: &str = puzzle_3d_dsl::PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT;
const NAKAGIN_EXAMPLE_DSL: &str = puzzle_3d_dsl::PUZZLE3D_NAKAGIN_EXAMPLE_TEXT;
/// 🌉️ This app's own `Puzzle3dScene.fixture: Puzzle3dFixture` (and `DocumentApp::Projection =
/// Value`) stays a local structural-twin mirror of `puzzle_3d::Puzzle3dProjection` — see
/// `puzzle_3d`'s `🔖️ValueBridge` region — so the DSL-text example fixtures are parsed once into
/// the typed `puzzle_3d::Puzzle3dProjection` and re-serialized to the JSON string this module's
/// `serde_json::from_str::<Puzzle3dFixture>`/`.example(...)` call sites expect.
static CONCRETE_FOREST_EXAMPLE_JSON: LazyLock<String> =
    LazyLock::new(|| serde_json::to_string(&<Puzzle3dProjection as store::DocumentDsl>::parse_dsl(CONCRETE_FOREST_EXAMPLE_DSL).expect("concrete-forest example fixture parses as dsl")).expect("serialize concrete-forest example fixture"));
static NAKAGIN_EXAMPLE_JSON: LazyLock<String> =
    LazyLock::new(|| serde_json::to_string(&<Puzzle3dProjection as store::DocumentDsl>::parse_dsl(NAKAGIN_EXAMPLE_DSL).expect("nakagin example fixture parses as dsl")).expect("serialize nakagin example fixture"));
//#endregion 🔖️Constants

//#region 🔖️Document
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dCamera {
    #[serde(default)]
    position: [f64; 3],
    #[serde(default)]
    target: [f64; 3],
    #[serde(default = "one_f64")]
    zoom: f64,
    #[serde(default)]
    up: Option<[f64; 3]>,
    #[serde(default)]
    projection: WorldProjectionConfig,
}

/// 📐️ Distance from `camera.position` to `camera.target`, defaulting to the historic 30-unit orbit radius when degenerate.
fn puzzle3d_camera_distance(camera: &Puzzle3dCamera) -> f64 {
    let [dx, dy, dz] = [camera.position[0] - camera.target[0], camera.position[1] - camera.target[1], camera.position[2] - camera.target[2]];
    let distance = (dx * dx + dy * dy + dz * dz).sqrt();
    if distance > 1e-3 {
        distance
    } else {
        30.0
    }
}

fn one_f64() -> f64 {
    1.0
}

fn default_selection_method() -> String {
    "rectangle".into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dVortex {
    id: String,
    #[serde(default, rename = "vortexKind")]
    vortex_kind: Option<String>,
    #[serde(default)]
    position: [f64; 3],
    #[serde(default)]
    direction: Option<[f64; 3]>,
    #[serde(default)]
    radius: Option<f64>,
    #[serde(default)]
    hidden: bool,
    #[serde(default)]
    locked: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dReferenceSource {
    #[serde(default)]
    url: String,
    #[serde(default, rename = "mediaKind")]
    media_kind: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dReference {
    id: String,
    #[serde(default)]
    source: Puzzle3dReferenceSource,
    #[serde(default)]
    origin: [f64; 3],
    #[serde(default, rename = "widthWorld")]
    width_world: f64,
    #[serde(default)]
    locked: bool,
    #[serde(default)]
    hidden: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dObject {
    id: String,
    #[serde(default)]
    label: Option<String>,
    #[serde(default, rename = "objectKind")]
    object_kind: Option<String>,
    #[serde(default)]
    origin: [f64; 3],
    #[serde(default)]
    orientation: Option<[f64; 4]>,
    #[serde(default)]
    scale: Option<Value>,
    #[serde(default, rename = "meshUrl")]
    mesh_url: Option<String>,
    #[serde(default)]
    vortices: Vec<Puzzle3dVortex>,
    #[serde(default)]
    hidden: bool,
    #[serde(default)]
    locked: bool,
    /// 🪣️ Live-viewport-only tag from `compose_fill_display` — this object's 0-based position in the
    /// fill plan's sequence, never persisted to the committed document. See `world_instances_json`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reveal_index: Option<usize>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dFixtureMeta {
    #[serde(default, rename = "kindCatalogs")]
    kind_catalogs: Option<Value>,
    #[serde(default, rename = "kindCompatibility")]
    kind_compatibility: Option<Value>,
}

/// 🧊️ Persisted oriented box constraining fill placement. Volume Brush creates axis-aligned voxel-sized
/// instances; Transform gumball edits arbitrary oriented boxes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dTargetVolume {
    id: String,
    #[serde(default)]
    origin: [f64; 3],
    #[serde(default)]
    orientation: Option<[f64; 4]>,
    #[serde(default)]
    scale: Option<Value>,
    #[serde(default)]
    hidden: bool,
    #[serde(default)]
    locked: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dAttraction {
    #[serde(default)]
    id: String,
    attracting: String,
    attracted: String,
    #[serde(default)]
    gap: f64,
    #[serde(default)]
    shift: f64,
    #[serde(default)]
    rise: f64,
    #[serde(default)]
    rotation: f64,
    #[serde(default)]
    turn: f64,
    #[serde(default)]
    tilt: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dFixture {
    schema: String,
    #[serde(default)]
    domain: String,
    #[serde(default)]
    meta: Puzzle3dFixtureMeta,
    #[serde(default)]
    objects: Vec<Puzzle3dObject>,
    #[serde(default)]
    attractions: Vec<Puzzle3dAttraction>,
    #[serde(default, rename = "targetVolumes")]
    target_volumes: Vec<Puzzle3dTargetVolume>,
    #[serde(default)]
    references: Vec<Puzzle3dReference>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dSelection {
    #[serde(default)]
    object_ids: SelectionSet,
    #[serde(default)]
    vortex_ids: SelectionSet,
    #[serde(default)]
    attraction_ids: SelectionSet,
    #[serde(default, rename = "targetVolumeIds")]
    target_volume_ids: SelectionSet,
    #[serde(default)]
    reference_ids: SelectionSet,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dSelectableKinds {
    #[serde(default = "default_true")]
    objects: bool,
    #[serde(default = "default_true")]
    vortices: bool,
    #[serde(default = "default_true")]
    attractions: bool,
}

impl Default for Puzzle3dSelectableKinds {
    fn default() -> Self {
        Self { objects: true, vortices: true, attractions: true }
    }
}

fn default_true() -> bool {
    true
}

fn default_vortex_show() -> String {
    PUZZLE3D_VORTEX_SHOW_SELECTED.into()
}

fn default_vortex_direction() -> String {
    PUZZLE3D_VORTEX_DIRECTION_OUTWARDS.into()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dSuggestionMenu {
    x: f64,
    y: f64,
    #[serde(default)]
    window_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dRuntime {
    #[serde(default)]
    selection: Puzzle3dSelection,
    #[serde(default = "default_selection_method")]
    selection_method: String,
    #[serde(default)]
    hovered_object_id: Option<String>,
    #[serde(default)]
    hovered_vortex_full_id: Option<String>,
    /// 🎯️ Open per-vortex brush-candidate suggestion popup (context menu / Alt+right-click), or `None` when closed.
    #[serde(default)]
    suggestion_menu: Option<Puzzle3dSuggestionMenu>,
    #[serde(default = "default_overlap_budget")]
    overlap_budget: f64,
    #[serde(default)]
    fill_count: u32,
    #[serde(default)]
    brush_candidate_index: usize,
    #[serde(default)]
    object_kind_weights: HashMap<String, f64>,
    #[serde(default)]
    vortex_kind_weights: HashMap<String, f64>,
    #[serde(default = "default_true")]
    lod_automatic: bool,
    #[serde(default)]
    lod_depth_variable: bool,
    #[serde(default = "default_true")]
    grid_visible: bool,
    #[serde(default = "default_manual_lod")]
    lod_manual: f64,
    #[serde(default)]
    grid_snap_enabled: bool,
    #[serde(default = "default_grid_spacing")]
    grid_spacing: f64,
    #[serde(default)]
    selectable_kinds: Puzzle3dSelectableKinds,
    #[serde(default)]
    hovered_kind_id: Option<String>,
    #[serde(default)]
    engagement_input: String,
    #[serde(default = "default_selection_mode")]
    selection_mode_default: String,
    #[serde(default = "default_proximity_radius")]
    proximity_radius: f64,
    #[serde(default = "default_chunk_size")]
    chunk_size: f64,
    #[serde(default = "default_voxel_dims")]
    voxel_dims: [u32; 3],
    /// 🎛️ Whether the transform gumball exposes translate (move axes + move planes).
    #[serde(default = "default_true")]
    transform_move: bool,
    /// 🎛️ Whether the transform gumball exposes rotate handles.
    #[serde(default = "default_true")]
    transform_rotate: bool,
    /// 🌀️ When to emit vortex markers: [`PUZZLE3D_VORTEX_SHOW_ALWAYS`] or [`PUZZLE3D_VORTEX_SHOW_SELECTED`].
    #[serde(default = "default_vortex_show")]
    vortex_show: String,
    /// 🧭️ How vortex direction arrows are drawn: [`PUZZLE3D_VORTEX_DIRECTION_OUTWARDS`] or [`PUZZLE3D_VORTEX_DIRECTION_INWARDS`].
    #[serde(default = "default_vortex_direction")]
    vortex_direction: String,
    #[serde(default)]
    sun: WorldSunConfig,
    /// 🎥️ Session-only viewport camera for the window instance currently materialized onto this
    /// runtime (via `load_window`/`save_window`) — never persisted to the document; see
    /// [`Puzzle3dWindowOptions::camera`].
    #[serde(default)]
    camera: Puzzle3dCamera,
    /// 🪟️ Per-window-instance snapshot of view-local chrome options, keyed by window INSTANCE id (never
    /// by window kind) — see [`Puzzle3dWindowOptions`]. The flat fields that mirror those options are a
    /// scratch, currently-materialized-window working copy: `load_window`/`save_window` swap them in/out
    /// around every `render`/`window_measures`/`window_engagements`/`handle_action` call so two window
    /// instances of the same kind (e.g. split top/perspective panes) never share a grid/LOD/selection
    /// preference. Fill count, distribution weights, and overlap budget stay on the flat runtime —
    /// they mutate the shared document/precompute plan and must agree across every pane.
    #[serde(default)]
    window_options: BTreeMap<String, Puzzle3dWindowOptions>,
}

impl Default for Puzzle3dRuntime {
    /// 🎛️ Mirrors every `#[serde(default = "...")]` above — `#[derive(Default)]` would silently ignore
    /// them and zero out fields like `overlap_budget`/`selection_method`/`lod_automatic` in Rust-constructed runtimes.
    fn default() -> Self {
        Self {
            selection: Puzzle3dSelection::default(),
            selection_method: default_selection_method(),
            hovered_object_id: None,
            hovered_vortex_full_id: None,
            suggestion_menu: None,
            overlap_budget: default_overlap_budget(),
            fill_count: 0,
            brush_candidate_index: 0,
            object_kind_weights: HashMap::new(),
            vortex_kind_weights: HashMap::new(),
            lod_automatic: default_true(),
            lod_depth_variable: false,
            grid_visible: default_true(),
            lod_manual: default_manual_lod(),
            grid_snap_enabled: false,
            grid_spacing: default_grid_spacing(),
            selectable_kinds: Puzzle3dSelectableKinds::default(),
            hovered_kind_id: None,
            engagement_input: String::new(),
            selection_mode_default: default_selection_mode(),
            proximity_radius: default_proximity_radius(),
            chunk_size: default_chunk_size(),
            voxel_dims: default_voxel_dims(),
            transform_move: default_true(),
            transform_rotate: default_true(),
            vortex_show: default_vortex_show(),
            vortex_direction: default_vortex_direction(),
            sun: WorldSunConfig::default(),
            camera: Puzzle3dCamera::default(),
            window_options: BTreeMap::new(),
        }
    }
}

fn default_overlap_budget() -> f64 {
    0.02
}

fn default_manual_lod() -> f64 {
    100.0
}

fn default_grid_spacing() -> f64 {
    10.0
}

fn default_selection_mode() -> String {
    "default".into()
}

fn default_proximity_radius() -> f64 {
    0.75
}

fn default_chunk_size() -> f64 {
    256.0
}

fn default_voxel_dims() -> [u32; 3] {
    [1, 1, 1]
}

/// 🪟️ View-local chrome options a puzzle3d window exposes (grid, LOD, selection method/mode, vortex
/// display, sun, voxel steppers, camera) — stored per window INSTANCE in
/// [`Puzzle3dRuntime::window_options`]. Fill count, distribution weights, and overlap budget are
/// intentionally absent: they drive the shared document/precompute plan and live only on the flat
/// [`Puzzle3dRuntime`] fields so split panes can never disagree about which fill objects are shown.
/// See `load_window`/`save_window`.
///
/// 🎥️ `camera` is session-only per-window state — orbiting/panning/zooming one window instance (via
/// `setCamera`/`setProjection`/`setProjectionParam`/`focusSelection`) must never move any sibling
/// instance's camera and must never become a VCS-tracked document edit (see those actions'
/// `ActionKind::View`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle3dWindowOptions {
    selection_method: String,
    lod_automatic: bool,
    lod_depth_variable: bool,
    grid_visible: bool,
    lod_manual: f64,
    grid_snap_enabled: bool,
    grid_spacing: f64,
    selectable_kinds: Puzzle3dSelectableKinds,
    engagement_input: String,
    selection_mode_default: String,
    proximity_radius: f64,
    chunk_size: f64,
    voxel_dims: [u32; 3],
    transform_move: bool,
    transform_rotate: bool,
    vortex_show: String,
    vortex_direction: String,
    sun: WorldSunConfig,
    camera: Puzzle3dCamera,
}

impl Default for Puzzle3dWindowOptions {
    fn default() -> Self {
        Self {
            selection_method: default_selection_method(),
            lod_automatic: default_true(),
            lod_depth_variable: false,
            grid_visible: default_true(),
            lod_manual: default_manual_lod(),
            grid_snap_enabled: false,
            grid_spacing: default_grid_spacing(),
            selectable_kinds: Puzzle3dSelectableKinds::default(),
            engagement_input: String::new(),
            selection_mode_default: default_selection_mode(),
            proximity_radius: default_proximity_radius(),
            chunk_size: default_chunk_size(),
            voxel_dims: default_voxel_dims(),
            transform_move: default_true(),
            transform_rotate: default_true(),
            vortex_show: default_vortex_show(),
            vortex_direction: default_vortex_direction(),
            sun: WorldSunConfig::default(),
            camera: Puzzle3dCamera::default(),
        }
    }
}

impl Puzzle3dRuntime {
    /// 🪟️ Snapshots this runtime's currently-materialized flat window-option fields into a
    /// [`Puzzle3dWindowOptions`] — the counterpart to `apply_window_options`. Does not snapshot fill
    /// count / distribution / overlap: those stay app-global on the flat runtime.
    fn snapshot_window_options(&self) -> Puzzle3dWindowOptions {
        Puzzle3dWindowOptions {
            selection_method: self.selection_method.clone(),
            lod_automatic: self.lod_automatic,
            lod_depth_variable: self.lod_depth_variable,
            grid_visible: self.grid_visible,
            lod_manual: self.lod_manual,
            grid_snap_enabled: self.grid_snap_enabled,
            grid_spacing: self.grid_spacing,
            selectable_kinds: self.selectable_kinds.clone(),
            engagement_input: self.engagement_input.clone(),
            selection_mode_default: self.selection_mode_default.clone(),
            proximity_radius: self.proximity_radius,
            chunk_size: self.chunk_size,
            voxel_dims: self.voxel_dims,
            transform_move: self.transform_move,
            transform_rotate: self.transform_rotate,
            vortex_show: self.vortex_show.clone(),
            vortex_direction: self.vortex_direction.clone(),
            sun: self.sun.clone(),
            camera: self.camera.clone(),
        }
    }

    /// 🪟️ Materializes `options` onto this runtime's flat window-option fields — the counterpart to
    /// `snapshot_window_options`. Leaves fill count / distribution / overlap untouched so a pane
    /// switch cannot rewrite the shared fill scene.
    fn apply_window_options(&mut self, options: &Puzzle3dWindowOptions) {
        self.selection_method = options.selection_method.clone();
        self.lod_automatic = options.lod_automatic;
        self.lod_depth_variable = options.lod_depth_variable;
        self.grid_visible = options.grid_visible;
        self.lod_manual = options.lod_manual;
        self.grid_snap_enabled = options.grid_snap_enabled;
        self.grid_spacing = options.grid_spacing;
        self.selectable_kinds = options.selectable_kinds.clone();
        self.engagement_input = options.engagement_input.clone();
        self.selection_mode_default = options.selection_mode_default.clone();
        self.proximity_radius = options.proximity_radius;
        self.chunk_size = options.chunk_size;
        self.voxel_dims = options.voxel_dims;
        self.transform_move = options.transform_move;
        self.transform_rotate = options.transform_rotate;
        self.vortex_show = options.vortex_show.clone();
        self.vortex_direction = options.vortex_direction.clone();
        self.sun = options.sun.clone();
        self.camera = options.camera.clone();
    }

    /// 🪟️ Materializes `window_id`'s stored options (the type default, for a window never touched yet)
    /// onto this runtime's flat fields — call before building a `Puzzle3dScene` for that window, in
    /// every read (`render`/`window_engagements`/`window_measures`) and write (`handle_action`) path.
    fn load_window(&mut self, window_id: &str) {
        let options = self.window_options.get(window_id).cloned().unwrap_or_default();
        self.apply_window_options(&options);
    }

    /// 🪟️ Snapshots this runtime's current flat view-local option fields (as left by whatever action
    /// just ran) back into `window_id`'s stored entry. Other windows' entries in `window_options` are
    /// untouched, so a `setGridVisible` in one window instance never affects another's. Shared fill
    /// fields are not part of the snapshot.
    fn save_window(&mut self, window_id: &str) {
        let options = self.snapshot_window_options();
        self.window_options.insert(window_id.to_string(), options);
    }
}


/// 🧾️ Transient render/mutation bundle pairing the persisted projection (the bare `Puzzle3dFixture`
/// json) with the app's ephemeral view state. Never persisted — the {@link VcsDocumentApp} store owns
/// the fixture and {@link Puzzle3dPlayApp} owns the runtime — but rebuilt per call so the existing
/// panel/world/engagement helpers keep their `&scene` signatures.
#[derive(Clone)]
struct Puzzle3dScene {
    fixture: Puzzle3dFixture,
    runtime: Puzzle3dRuntime,
    /// 🧰️ Host-owned active utility mirrored from `view_state.active_utility_id` — transient, never persisted.
    active_utility: String,
}

/// 🧭️ The select/brush/fill interaction mode the world engine reads, derived from the flat active utility
/// (the transform gumball utilities `move`/`rotate`/`scale` and `worldRelocate` all present as `select`).
fn puzzle3d_scene_mode(active_utility: &str) -> &str {
    match active_utility {
        "brush" => "brush",
        "fill" => "fill",
        "volumeBrush" => "volumeBrush",
        _ => "select",
    }
}

/// 🎚️ The gumball handle the world engine draws when a transform utility is active.
fn puzzle3d_transform_handle(active_utility: &str) -> Option<&'static str> {
    if active_utility == "transform" {
        Some("transform")
    } else {
        None
    }
}

/// 🧭️ Whether the active utility is a transform gumball mode.
fn puzzle3d_transform_utility_active(active_utility: &str) -> bool {
    puzzle3d_transform_handle(active_utility).is_some()
}

/// 🕹️ Whether the world gumball should render for the current selection and utility.
fn puzzle3d_gumball_active(runtime: &Puzzle3dRuntime, active_utility: &str) -> bool {
    !runtime.selection.object_ids.is_empty() && puzzle3d_transform_utility_active(active_utility)
}

/// 🧹️ Clears every selection bag.
fn puzzle3d_clear_selection(selection: &mut Puzzle3dSelection) {
    *selection = Puzzle3dSelection::default();
}

/// 🧹️ Clears every selection bag except object ids.
fn puzzle3d_clear_non_object_selection(selection: &mut Puzzle3dSelection) {
    selection.vortex_ids.clear();
    selection.attraction_ids.clear();
    selection.target_volume_ids.clear();
    selection.reference_ids.clear();
}

/// 🧹️ Clears every selection bag except vortex ids.
fn puzzle3d_clear_non_vortex_selection(selection: &mut Puzzle3dSelection) {
    selection.object_ids.clear();
    selection.attraction_ids.clear();
    selection.target_volume_ids.clear();
    selection.reference_ids.clear();
}

/// 🧭️ Whether the engagement HUD should mark an active session for the given utility.
fn puzzle3d_engagement_session_active(active_utility: &str) -> bool {
    matches!(active_utility, "brush" | "fill" | "worldRelocate")
}

/// 🪣️ Whether the host-owned, mode-level Fill tool currently authorizes fill planning and interaction.
fn puzzle3d_fill_tool_active(view_state: &ViewState) -> bool {
    view_state.active_tool_id.as_deref() == Some("fill")
}

/// 🛠️ The effective interaction id threaded through `Puzzle3dScene.active_utility`: the host-owned
/// window utility (`active_utility_by_window_id` for `window_id`, else `active_utility_id`), UNLESS the
/// mode-level fill tool is active (`active_tool_id`), in which case fill wins. Fill keeps its viewport
/// interaction (world engine `activeUtility` JSON, engagement session, scene mode) even though it is
/// declared as a windowless tool, not a `WindowKindDefinition` utility — see `ToolDefinition`/`mode_tools`.
fn puzzle3d_scene_active_utility(view_state: &ViewState, window_id: Option<&str>) -> String {
    if puzzle3d_fill_tool_active(view_state) {
        return "fill".to_string();
    }
    if let Some(wid) = window_id {
        if let Some(utility) = view_state.active_utility_by_window_id.get(wid) {
            return utility.clone();
        }
    }
    view_state.active_utility_id.as_deref().unwrap_or(PUZZLE3D_DEFAULT_UTILITY).to_string()
}

static PUZZLE3D_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

fn empty_fixture() -> Puzzle3dFixture {
    Puzzle3dFixture {
        schema: PUZZLE3D_FIXTURE_SCHEMA.into(),
        domain: "architecture".into(),
        meta: Puzzle3dFixtureMeta::default(),
        objects: Vec::new(),
        attractions: Vec::new(),
        target_volumes: Vec::new(),
        references: Vec::new(),
    }
}

fn default_fixture() -> Puzzle3dFixture {
    serde_json::from_str::<Puzzle3dFixture>(CONCRETE_FOREST_EXAMPLE_JSON.as_str()).unwrap_or_else(|_| empty_fixture())
}

fn nakagin_fixture() -> Puzzle3dFixture {
    serde_json::from_str::<Puzzle3dFixture>(NAKAGIN_EXAMPLE_JSON.as_str()).unwrap_or_else(|_| empty_fixture())
}

/// 🧾️ Materializes the transient scene from the persisted projection (bare fixture json) and the
/// app's current view state; an unparseable projection degrades to an empty board.
fn scene_from_projection(projection: &Value, runtime: Puzzle3dRuntime, active_utility: &str) -> Puzzle3dScene {
    let fixture = serde_json::from_value::<Puzzle3dFixture>(projection.clone()).unwrap_or_else(|_| empty_fixture());
    Puzzle3dScene { fixture, runtime, active_utility: active_utility.to_string() }
}

/// 🧮️ Document ops for a fixture mutation — normalizes `before` through the same program typed
/// round-trip as `after` so View-kind actions that only touch runtime never trip the
/// "must not emit operations" guard when the live store still holds a `puzzle_3d`-shaped
/// projection (`skip_serializing_if`-elided optional fields) from a prior op apply.
fn puzzle3d_operations_from_fixture_change(before: &Value, after_fixture: &Puzzle3dFixture) -> Vec<Puzzle3dOperation> {
    let before_normalized = serde_json::to_value(serde_json::from_value::<Puzzle3dFixture>(before.clone()).unwrap_or_else(|_| empty_fixture())).unwrap_or_else(|_| before.clone());
    let after = serde_json::to_value(after_fixture).unwrap_or_else(|_| before_normalized.clone());
    puzzle3d_document_delta_operations(&before_normalized, &after)
}

/// 🪟️ Live window-instance ids of `kind_id` from `view_state.window_instances`, falling back to
/// `vec![kind_id]` when the list is empty — a headless/test call that never threads instances still
/// gets exactly the one entry today's single-window callers expect.
fn window_instance_ids(view_state: &ViewState, kind_id: &str) -> Vec<String> {
    let ids: Vec<String> = view_state.window_instances.iter().filter(|instance| instance.window_kind_id == kind_id).map(|instance| instance.id.clone()).collect();
    if ids.is_empty() { vec![kind_id.to_string()] } else { ids }
}

fn puzzle3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: PUZZLE3D_PLAY_CONTROLLER_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
}

fn camera_json(camera: &Puzzle3dCamera) -> String {
    world3d_camera_projection_json(camera.position, camera.target, camera.up, camera.zoom, &camera.projection)
}

fn mesh_selection_ids(args: Option<&Value>, fallback: &SelectionSet) -> Vec<String> {
    args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).filter(|ids: &Vec<String>| !ids.is_empty()).unwrap_or_else(|| fallback.as_slice().to_vec())
}

/** @emoji 🧭️ Whether `handle_action` may emit VCS operations from a fixture before/after delta — view-only actions skip the document snapshot entirely. */
fn puzzle3d_action_document_intent(action: &str) -> bool {
    matches!(
        action,
        "setFixtureJson"
            | "setActiveExample"
            | "addObjectKind"
            | "deleteSelection"
            | "duplicateSelection"
            | "translateSelection"
            | "rotateSelection"
            | "scaleSelection"
            | "transformEnd"
            | "worldRelocate"
            | "setSelectionFlag"
            | "patchInspector"
            | "engagementSubmit"
            | "engagementRepeatLast"
            | "createAttraction"
            | "deleteAttraction"
            | "addTargetVolume"
            | "deleteTargetVolume"
            | "setTargetVolumeFlag"
            | "addBrushObject"
            | "setFillCount"
            | "acceptSuggestion"
    )
}

/// 🎥️ Named orbit-camera rigs — top/front/right use an orthographic projection with a non-Z `up` to avoid gimbal lock when looking straight down/along the Z-up axis.
fn quat_mul(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
    [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]]
}

fn quat_from_axis_angle(ax: f64, ay: f64, az: f64, angle: f64) -> [f64; 4] {
    let len = (ax * ax + ay * ay + az * az).sqrt();
    if len < 1e-8 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let half = angle * 0.5;
    let s = half.sin();
    [ax / len * s, ay / len * s, az / len * s, half.cos()]
}

fn scale_value_mul(scale: &Option<Value>, sx: f64, sy: f64, sz: f64) -> Value {
    match scale {
        Some(Value::Array(values)) if values.len() >= 3 => json!([values[0].as_f64().unwrap_or(1.0) * sx, values[1].as_f64().unwrap_or(1.0) * sy, values[2].as_f64().unwrap_or(1.0) * sz,]),
        Some(Value::Number(value)) => {
            let factor = value.as_f64().unwrap_or(1.0);
            json!([factor * sx, factor * sy, factor * sz])
        }
        _ => json!([sx, sy, sz]),
    }
}

fn resolve_object_mesh_url(object: &Puzzle3dObject, meta: &Puzzle3dFixtureMeta) -> Option<String> {
    if let Some(url) = object.mesh_url.as_ref().filter(|url| !url.is_empty()) {
        return Some(url.clone());
    }
    let kind_id = object.object_kind.as_deref()?;
    let catalogs = meta.kind_catalogs.as_ref()?;
    let objects = catalogs.get("objects")?.as_array()?;
    for entry in objects {
        if entry.get("id").and_then(|v| v.as_str()) == Some(kind_id) {
            return entry.get("meshUrl").and_then(|v| v.as_str()).map(str::to_string);
        }
    }
    None
}

fn collect_mesh_urls(fixture: &Puzzle3dFixture) -> Vec<String> {
    let mut urls = HashSet::new();
    for object in &fixture.objects {
        if let Some(url) = resolve_object_mesh_url(object, &fixture.meta) {
            urls.insert(url);
        }
    }
    if let Some(catalogs) = fixture.meta.kind_catalogs.as_ref() {
        if let Some(objects) = catalogs.get("objects").and_then(|v| v.as_array()) {
            for entry in objects {
                if let Some(url) = entry.get("meshUrl").and_then(|v| v.as_str()) {
                    urls.insert(url.to_string());
                }
            }
        }
    }
    urls.into_iter().collect()
}

fn object_scale_json(object: &Puzzle3dObject) -> [f64; 3] {
    match &object.scale {
        Some(Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        _ => [1.0, 1.0, 1.0],
    }
}

/// 🙈️ Hidden objects stay in the emitted array — `worldPick`'s `id` arg is the array index into it — but render at zero scale so they're effectively invisible without shifting any other object's index.
/// `revealIndex` is omitted entirely for untagged objects rather than emitted as `null`: the host's reveal cutoff (`framework/renderer/react`'s `applyRevealCutoff`) only skips instances with no reveal index, and a JSON `null` would coerce to `0` and hide every ordinary object behind the boot cutoff.
/// Selection/hover paint is driven by `selectionJson` on the host — never baked here so instance geometry stays stable across picks.
fn world_instances_geometry_json(fixture: &Puzzle3dFixture) -> String {
    let instances: Vec<Value> = fixture
        .objects
        .iter()
        .map(|object| {
            let mesh_id = resolve_object_mesh_url(object, &fixture.meta).map(|url| world3d_mesh_id_from_url(&url)).unwrap_or_else(|| PUZZLE3D_FALLBACK_MESH_KIND.into());
            let scale = if object.hidden { json!([0.0, 0.0, 0.0]) } else { json!(object_scale_json(object)) };
            let mut instance = json!({
                "id": object.id,
                "meshId": mesh_id,
                "position": [
                    object.origin.first().copied().unwrap_or(0.0),
                    object.origin.get(1).copied().unwrap_or(0.0),
                    object.origin.get(2).copied().unwrap_or(0.0),
                ],
                "rotation": object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": scale,
                "label": object.label.clone().or_else(|| object.object_kind.clone()).unwrap_or_else(|| object.id.clone()),
                "disabled": object.locked,
            });
            if let Some(kind) = &object.object_kind {
                instance["objectKind"] = json!(kind);
            }
            if let Some(reveal_index) = object.reveal_index {
                instance["revealIndex"] = json!(reveal_index);
            }
            instance
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn fixture_geometry_fingerprint(fixture: &Puzzle3dFixture) -> u64 {
    let payload = serde_json::to_string(&( &fixture.objects, &fixture.references, &fixture.target_volumes, &fixture.meta)).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    payload.hash(&mut hasher);
    hasher.finish()
}

fn world_meshes_json(fixture: &Puzzle3dFixture) -> String {
    let urls = collect_mesh_urls(fixture);
    let kinds = vec![PUZZLE3D_FALLBACK_MESH_KIND.into(), "vortex-marker".into()];
    if urls.is_empty() {
        return world3d_meshes_json_from_kinds_and_urls(&kinds, &[]);
    }
    let mut meshes_json = world3d_meshes_json_from_kinds_and_urls(&kinds, &urls);
    if !meshes_json.contains(PUZZLE3D_FALLBACK_MESH_KIND) {
        let fallback = world3d_meshes_json_from_kinds_and_urls(&[PUZZLE3D_FALLBACK_MESH_KIND.into()], &[]);
        let mut merged: Vec<Value> = serde_json::from_str(&meshes_json).unwrap_or_default();
        let fallback_meshes: Vec<Value> = serde_json::from_str(&fallback).unwrap_or_default();
        merged.extend(fallback_meshes);
        meshes_json = serde_json::to_string(&merged).unwrap_or(meshes_json);
    }
    meshes_json
}

fn quat_rotate_vector(quat: [f64; 4], vector: [f64; 3]) -> [f64; 3] {
    let [x, y, z, w] = quat;
    let vx = vector[0];
    let vy = vector[1];
    let vz = vector[2];
    let ix = w * vx + y * vz - z * vy;
    let iy = w * vy + z * vx - x * vz;
    let iz = w * vz + x * vy - y * vx;
    let iw = -x * vx - y * vy - z * vz;
    [ix * w + iw * -x + iy * -z - iz * -y, iy * w + iw * -y + iz * -x - ix * -z, iz * w + iw * -z + ix * -y - iy * -x]
}

fn world_vortex_position(object: &Puzzle3dObject, vortex: &Puzzle3dVortex) -> [f64; 3] {
    let orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let rotated = quat_rotate_vector(orientation, vortex.position);
    [object.origin.first().copied().unwrap_or(0.0) + rotated[0], object.origin.get(1).copied().unwrap_or(0.0) + rotated[1], object.origin.get(2).copied().unwrap_or(0.0) + rotated[2]]
}

fn world_vortex_direction(object: &Puzzle3dObject, vortex: &Puzzle3dVortex) -> [f64; 3] {
    let direction = vortex.direction.unwrap_or([0.0, 0.0, -1.0]);
    quat_rotate_vector(object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), direction)
}

fn vortex_color(meta: &Puzzle3dFixtureMeta, vortex_kind: Option<&str>) -> String {
    let Some(kind_id) = vortex_kind else {
        return "#38bdf8".into();
    };
    let Some(catalogs) = meta.kind_catalogs.as_ref() else {
        return "#38bdf8".into();
    };
    let Some(entries) = catalogs.get("vortices").and_then(|value| value.as_array()) else {
        return "#38bdf8".into();
    };
    for entry in entries {
        if entry.get("id").and_then(|value| value.as_str()) == Some(kind_id) {
            return entry.get("color").and_then(|value| value.as_str()).unwrap_or("#38bdf8").to_string();
        }
    }
    "#38bdf8".into()
}

fn object_kind_color(meta: &Puzzle3dFixtureMeta, object_kind: Option<&str>) -> String {
    let Some(kind_id) = object_kind else {
        return "#38bdf8".into();
    };
    let Some(catalogs) = meta.kind_catalogs.as_ref() else {
        return "#38bdf8".into();
    };
    let Some(entries) = catalogs.get("objects").and_then(|value| value.as_array()) else {
        return "#38bdf8".into();
    };
    for entry in entries {
        if entry.get("id").and_then(|value| value.as_str()) == Some(kind_id) {
            return entry.get("color").and_then(|value| value.as_str()).unwrap_or("#38bdf8").to_string();
        }
    }
    "#38bdf8".into()
}

fn object_kind_icon(meta: &Puzzle3dFixtureMeta, object_kind: Option<&str>) -> String {
    let Some(kind_id) = object_kind else {
        return "box".into();
    };
    let Some(catalogs) = meta.kind_catalogs.as_ref() else {
        return "box".into();
    };
    let Some(entries) = catalogs.get("objects").and_then(|value| value.as_array()) else {
        return "box".into();
    };
    for entry in entries {
        if entry.get("id").and_then(|value| value.as_str()) == Some(kind_id) {
            return entry.get("icon").or_else(|| entry.get("iconId")).and_then(|value| value.as_str()).unwrap_or("box").to_string();
        }
    }
    "box".into()
}

fn puzzle3d_vortex_full_id(object_id: &str, vortex_id: &str) -> String {
    if vortex_id.contains(':') {
        vortex_id.to_string()
    } else {
        format!("{object_id}:{vortex_id}")
    }
}

fn resolve_vortex_world_position(fixture: &Puzzle3dFixture, full_id: &str) -> Option<[f64; 3]> {
    for object in &fixture.objects {
        for vortex in &object.vortices {
            if puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id {
                return Some(world_vortex_position(object, vortex));
            }
        }
    }
    None
}

fn resolve_vortex_kind(fixture: &Puzzle3dFixture, full_id: &str) -> Option<String> {
    fixture.objects.iter().find_map(|object| object.vortices.iter().find(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id).and_then(|vortex| vortex.vortex_kind.clone()))
}

/// 🧲️ Permissive when the fixture declares no `kindCompatibility` rules at all — otherwise requires an explicit (or bidirectional) entry.
fn puzzle3d_kinds_compatible(fixture: &Puzzle3dFixture, source_kind: &str, target_kind: &str) -> bool {
    let Some(entries) = fixture.meta.kind_compatibility.as_ref().and_then(|value| value.as_array()) else {
        return true;
    };
    if entries.is_empty() {
        return true;
    }
    entries.iter().any(|entry| {
        let source = entry.get("source").and_then(|value| value.as_str()).unwrap_or("");
        let target = entry.get("target").and_then(|value| value.as_str()).unwrap_or("");
        let bidirectional = entry.get("bidirectional").and_then(|value| value.as_bool()).unwrap_or(false);
        (source == source_kind && target == target_kind) || (bidirectional && source == target_kind && target == source_kind)
    })
}

/// 👁️ True when this object's vortices should render — always when `vortex_show` is Always; otherwise only when the parent object is hovered/selected, or any of its vortices are hovered/selected (vortex-only selection still needs markers).
fn puzzle3d_object_vortices_visible(object: &Puzzle3dObject, runtime: &Puzzle3dRuntime) -> bool {
    if runtime.vortex_show == PUZZLE3D_VORTEX_SHOW_ALWAYS {
        return true;
    }
    if runtime.selection.object_ids.contains(&object.id) {
        return true;
    }
    if runtime.hovered_object_id.as_deref() == Some(object.id.as_str()) {
        return true;
    }
    object.vortices.iter().any(|vortex| {
        let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
        runtime.selection.vortex_ids.contains(&full_id) || runtime.hovered_vortex_full_id.as_deref() == Some(full_id.as_str())
    })
}

fn world_vortices_json(fixture: &Puzzle3dFixture, runtime: &Puzzle3dRuntime) -> String {
    let mut records = Vec::new();
    for object in &fixture.objects {
        if !puzzle3d_object_vortices_visible(object, runtime) {
            continue;
        }
        for vortex in &object.vortices {
            let position = world_vortex_position(object, vortex);
            let direction = world_vortex_direction(object, vortex);
            let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
            records.push(json!({
                "fullId": full_id,
                "objectId": object.id,
                "vortexKind": vortex.vortex_kind,
                "position": position,
                "direction": direction,
                "radius": vortex.radius.unwrap_or(0.36),
                "color": vortex_color(&fixture.meta, vortex.vortex_kind.as_deref()),
                "displayDirection": runtime.vortex_direction,
            }));
        }
    }
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

fn world_attractions_json(fixture: &Puzzle3dFixture) -> String {
    let records: Vec<Value> = fixture
        .attractions
        .iter()
        .filter_map(|attraction| {
            let from = resolve_vortex_world_position(fixture, &attraction.attracting)?;
            let to = resolve_vortex_world_position(fixture, &attraction.attracted)?;
            Some(json!({
                "id": attraction.id,
                "from": from,
                "to": to,
                "color": "#60a5fa",
            }))
        })
        .collect();
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

fn target_volume_scale_json(volume: &Puzzle3dTargetVolume) -> [f64; 3] {
    match &volume.scale {
        Some(Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        _ => [1.0, 1.0, 1.0],
    }
}

fn world_target_volumes_json(fixture: &Puzzle3dFixture) -> String {
    let records: Vec<Value> = fixture
        .target_volumes
        .iter()
        .map(|volume| {
            json!({
                "id": volume.id,
                "origin": volume.origin,
                "orientation": volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": target_volume_scale_json(volume),
                "color": "#f472b6",
                "hidden": volume.hidden,
                "locked": volume.locked,
            })
        })
        .collect();
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

fn world_references_json(fixture: &Puzzle3dFixture) -> String {
    let records: Vec<Value> = fixture
        .references
        .iter()
        .map(|reference| {
            json!({
                "id": reference.id,
                "url": reference.source.url,
                "origin": reference.origin,
                "widthWorld": if reference.width_world > 0.0 { reference.width_world } else { 1.0 },
                "locked": reference.locked,
                "hidden": reference.hidden,
            })
        })
        .collect();
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

fn world_interaction_json(envelope: &Puzzle3dScene, session: &Puzzle3dPrecomputeSession) -> String {
    let runtime = &envelope.runtime;
    let suggestion_menu = runtime.suggestion_menu.as_ref().map(|menu| {
        let (pending, candidates) = puzzle3d_brush_target_vortex(envelope)
            .map(|target| {
                let result = session.brush_candidates(&target);
                let candidates: Vec<Value> = result
                    .free
                    .iter()
                    .enumerate()
                    .map(|(index, candidate)| {
                        let object_kind = Some(candidate.object_kind_id.as_str());
                        let object_label = candidate.object_kind_id.as_str();
                        let source_vortex_index = candidate.source_vortex_index;
                        let color = object_kind_color(&envelope.fixture.meta, object_kind);
                        let icon = object_kind_icon(&envelope.fixture.meta, object_kind);
                        json!({
                            "index": index,
                            "objectLabel": object_label,
                            "vortexLabel": format!("vortex {source_vortex_index}"),
                            "icon": icon,
                            "color": color,
                        })
                    })
                    .collect();
                (result.unknown_pending, candidates)
            })
            .unwrap_or((false, Vec::new()));
        json!({
            "open": true,
            "x": menu.x,
            "y": menu.y,
            "windowId": menu.window_id,
            "vortexFullId": puzzle3d_brush_target_vortex(envelope),
            "pending": pending,
            "candidates": candidates,
        })
    });
    let fill_build = session.fill_progress_summary();
    let fill_build = json!({
        "count": fill_build.count,
        "appliedCount": fill_build.applied_count,
        "maxCount": fill_build.max_count,
        "done": fill_build.done,
    });
    // 🪣️ Committed fill count as a viewport reveal cutoff — instances tagged `revealIndex` (see
    // `world_instances_json`) below this value are shown, the rest (already planned, not yet
    // committed) stay hidden until the host commits a higher value or the live drag store overrides
    // it locally. Keyed so future reveal-driven measures/tools can share the same channel.
    json!({
        "activeUtility": puzzle3d_scene_mode(&envelope.active_utility),
        "brushCandidateIndex": runtime.brush_candidate_index,
        "hoveredVortexFullId": runtime.hovered_vortex_full_id.clone(),
        "voxelDims": runtime.voxel_dims,
        "gridFactor": runtime.grid_spacing,
        "suggestionMenu": suggestion_menu,
        "fillBuild": fill_build,
        "revealCutoffs": { "puzzle3d-fill": runtime.fill_count },
    })
    .to_string()
}

fn world3d_lod_json(runtime: &Puzzle3dRuntime) -> String {
    json!({
        "gridFactor": runtime.grid_spacing,
        "gridSnapEnabled": runtime.grid_snap_enabled,
        "showLodGrid": runtime.grid_visible,
        "automaticLod": runtime.lod_automatic,
        "depthVariableLod": runtime.lod_depth_variable,
        "manualLod": runtime.lod_manual,
    })
    .to_string()
}

/// 👻️ Ghost placement for the brush utility, or for a one-shot context-menu / Alt+right-click
/// suggestion popup (`suggestion_menu`) that must not switch the host-owned active utility into brush.
fn world_brush_preview_json(session: &Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) -> Option<String> {
    if envelope.active_utility != "brush" && envelope.runtime.suggestion_menu.is_none() {
        return None;
    }
    let vortex_id = puzzle3d_brush_target_vortex(envelope)?;
    let preview = session.brush_preview(&vortex_id, envelope.runtime.brush_candidate_index)?;
    let color = object_kind_color(&envelope.fixture.meta, Some(preview.object_kind_id.as_str()));
    let mut value = serde_json::to_value(&preview).ok()?;
    if let Some(obj) = value.as_object_mut() {
        obj.insert("color".into(), json!(color));
    }
    serde_json::to_string(&value).ok()
}

/// ⏱️ Bounded to one small chunk per call (matches puzzle5d's drive path and the premigration idle
/// worker's chunk budget) — `handle_action` runs synchronously on the UI thread, and the host redrives
/// this via 120ms `suggestionsTick`/`fillBuildTick` ticks, so a large per-call budget here is exactly
/// what froze the UI for minutes: 128×32 Monte-Carlo collision task units, blocking, every single tick.
fn drive_precompute(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
    sync_precompute_session(session, envelope);
    session.precompute_step_lane(PrecomputeLane::Brush, 8);
}

fn puzzle3d_viewport_scope() -> semio_framework_core::kernel::UiDirtyScope {
    semio_framework_core::kernel::UiDirtyScope::Partial {
        window_bodies: vec![PUZZLE3D_PLAY_BODY_COMPOSITE.to_string()],
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: false,
        measures: false,
        labels: false,
    }
}

fn puzzle3d_chrome_scope() -> semio_framework_core::kernel::UiDirtyScope {
    semio_framework_core::kernel::UiDirtyScope::Partial {
        window_bodies: Vec::new(),
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: false,
        measures: false,
        labels: false,
    }
}

fn puzzle3d_selection_scope() -> semio_framework_core::kernel::UiDirtyScope {
    semio_framework_core::kernel::UiDirtyScope::Partial {
        window_bodies: Vec::new(),
        panel_bodies: vec![PUZZLE3D_PLAY_BODY_INSPECTOR.to_string()],
        utilities: false,
        tools: false,
        engagements: false,
        measures: false,
        labels: false,
    }
}

/// 🐢️ Background fill planning only mutates the main world body's `fillBuild` interaction JSON and the
/// fill-count slider range in the fill tool's measures — never panels, engagements, window measures, or
/// labels. Emitting `Full` on every 120ms tick was the other half of the fill-utility stall (alongside
/// unbounded tick queueing on the host): each tick re-fetched the entire shell UI.
fn puzzle3d_fill_build_scope() -> semio_framework_core::kernel::UiDirtyScope {
    semio_framework_core::kernel::UiDirtyScope::Partial {
        window_bodies: vec![PUZZLE3D_PLAY_BODY_COMPOSITE.to_string()],
        panel_bodies: Vec::new(),
        utilities: false,
        tools: true,
        engagements: false,
        measures: false,
        labels: false,
    }
}

/// 🐢️ Fill/distribution slider gestures refresh the world body, fill-tool measures, and utility-option
/// window measures — never the full shell chrome.
fn puzzle3d_fill_options_scope() -> semio_framework_core::kernel::UiDirtyScope {
    semio_framework_core::kernel::UiDirtyScope::Partial {
        window_bodies: vec![PUZZLE3D_PLAY_BODY_COMPOSITE.to_string()],
        panel_bodies: Vec::new(),
        utilities: false,
        tools: true,
        engagements: false,
        measures: true,
        labels: false,
    }
}

/// 🐢️ Suggestion collision ticking only refreshes the world body's suggestion-menu interaction JSON.
fn puzzle3d_suggestions_tick_scope() -> semio_framework_core::kernel::UiDirtyScope {
    semio_framework_core::kernel::UiDirtyScope::Partial {
        window_bodies: vec![PUZZLE3D_PLAY_BODY_COMPOSITE.to_string()],
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: false,
        measures: false,
        labels: false,
    }
}

/// 🐢️ Mid-drag gumball scratch only refreshes the world composite body — never the full shell.
fn puzzle3d_transform_drag_scope() -> semio_framework_core::kernel::UiDirtyScope {
    semio_framework_core::kernel::UiDirtyScope::Partial {
        window_bodies: vec![PUZZLE3D_PLAY_BODY_COMPOSITE.to_string()],
        panel_bodies: Vec::new(),
        utilities: false,
        tools: false,
        engagements: false,
        measures: false,
        labels: false,
    }
}

/// 🧲️ Applies one absolute gumball translate (total delta from drag-start) onto a fixture snapshot.
fn puzzle3d_apply_translate(fixture: &mut Puzzle3dFixture, object_ids: &[String], volume_ids: &[String], dx: f64, dy: f64, dz: f64) {
    for object in &mut fixture.objects {
        if object_ids.contains(&object.id) {
            object.origin[0] += dx;
            object.origin[1] += dy;
            object.origin[2] += dz;
        }
    }
    for volume in fixture.target_volumes.iter_mut().filter(|volume| volume_ids.contains(&volume.id) && !volume.locked) {
        volume.origin[0] += dx;
        volume.origin[1] += dy;
        volume.origin[2] += dz;
    }
}

/// 🧲️ Applies one absolute gumball rotate (total axis-angle from drag-start) onto a fixture snapshot.
fn puzzle3d_apply_rotate(fixture: &mut Puzzle3dFixture, object_ids: &[String], volume_ids: &[String], ax: f64, ay: f64, az: f64, angle: f64) {
    let delta = quat_from_axis_angle(ax, ay, az, angle);
    for object in &mut fixture.objects {
        if object_ids.contains(&object.id) {
            let current = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
            object.orientation = Some(quat_mul(delta, current));
        }
    }
    for volume in fixture.target_volumes.iter_mut().filter(|volume| volume_ids.contains(&volume.id) && !volume.locked) {
        let current = volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
        volume.orientation = Some(quat_mul(delta, current));
    }
}

/// 🧲️ Applies one absolute gumball scale (total factors from drag-start) onto a fixture snapshot.
fn puzzle3d_apply_scale(fixture: &mut Puzzle3dFixture, object_ids: &[String], volume_ids: &[String], sx: f64, sy: f64, sz: f64) {
    for object in &mut fixture.objects {
        if object_ids.contains(&object.id) {
            object.scale = Some(scale_value_mul(&object.scale, sx, sy, sz));
        }
    }
    for volume in fixture.target_volumes.iter_mut().filter(|volume| volume_ids.contains(&volume.id) && !volume.locked) {
        volume.scale = Some(scale_value_mul(&volume.scale, sx, sy, sz));
    }
}

fn scene_config_json(envelope: &Puzzle3dScene) -> String {
    json!({
        "fixture": {
            "objects": envelope.fixture.objects,
            "attractions": envelope.fixture.attractions,
            "targetVolumes": envelope.fixture.target_volumes,
        },
        "kindCatalogs": envelope.fixture.meta.kind_catalogs,
        "kindCompatibility": envelope.fixture.meta.kind_compatibility.clone().unwrap_or(json!([])),
        "overlapBudget": envelope.runtime.overlap_budget,
        "seed": 1,
        "weights": {
            "objectWeights": envelope.runtime.object_kind_weights,
            "vortexWeights": envelope.runtime.vortex_kind_weights,
        }
    })
    .to_string()
}

/// 🧊️ Scales the unit box fallback (`mesh_from_kind` extent 1.0) past `BRUSH_COLLISION_MESH_MIN_EXTENT` (2.0) in `puzzle_3d`'s collision engine, otherwise its registration is a silent no-operation and brush candidates never populate before a real GLB arrives.
const PUZZLE3D_FALLBACK_MESH_SCALE: f32 = 4.0;

fn scaled_mesh_positions(positions: &[f32], scale: f32) -> Vec<f32> {
    positions.iter().map(|value| value * scale).collect()
}

/// 🧊️ Only seeds the box fallback for URLs with no mesh yet, so a real GLB registered earlier via `registerBrushMesh` survives every subsequent resync.
/// 🎯️ `scene_config_json` bridges two independently-evolved Rust schemas (this app's own document
/// model here vs. `puzzle_3d_engine::SceneConfig`) through JSON — that's schema translation, not a
/// wasm-bindgen boundary, so it stays. What changed for the headless-engine-law fix is the next step:
/// the parsed, TYPED `SceneConfig` goes into `Puzzle3dEngineCommand::SetScene` and through
/// `dispatch`, rather than a raw JSON string crossing into the engine's own JSON-string API.
fn sync_precompute_session(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
    if let Ok(scene) = serde_json::from_str::<puzzle_3d_engine::SceneConfig>(&scene_config_json(envelope)) {
        let _ = session.dispatch(Puzzle3dEngineCommand::SetScene { scene });
    }
    let fallback = mesh_from_kind(PUZZLE3D_FALLBACK_MESH_KIND);
    let fallback_positions = scaled_mesh_positions(&fallback.positions, PUZZLE3D_FALLBACK_MESH_SCALE);
    if !session.has_mesh(PUZZLE3D_FALLBACK_MESH_KIND) {
        session.register_mesh_fallback(PUZZLE3D_FALLBACK_MESH_KIND, &fallback_positions, &fallback.indices);
    }
    for url in collect_mesh_urls(&envelope.fixture) {
        if !session.has_mesh(&url) {
            session.register_mesh_fallback(&url, &fallback_positions, &fallback.indices);
        }
    }
}

fn sync_precompute_weights(session: &mut Puzzle3dPrecomputeSession, envelope: &Puzzle3dScene) {
    let object_weights = envelope.runtime.object_kind_weights.iter().map(|(k, v)| (k.clone(), *v)).collect();
    let vortex_weights = envelope.runtime.vortex_kind_weights.iter().map(|(k, v)| (k.clone(), *v)).collect();
    let _ = session.dispatch(Puzzle3dEngineCommand::UpdateKindWeights { object_weights, vortex_weights });
}

fn world_selection_json(envelope: &Puzzle3dScene) -> String {
    let runtime = &envelope.runtime;
    let mut value: Value = serde_json::from_str(&world3d_selection_json(&runtime.selection_method, runtime.selection.object_ids.as_slice(), runtime.hovered_object_id.as_deref())).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("selectionMergeMode".into(), json!(runtime.selection_mode_default));
        object.insert("granularity".into(), json!("mesh"));
        object.insert("selectionMode".into(), json!("mesh"));
        object.insert(
            "targets".into(),
            json!({
                "mesh": true,
                "vertex": false,
                "edge": false,
                "face": false,
            }),
        );
        object.insert("targetVolumeIds".into(), json!(runtime.selection.target_volume_ids));
        object.insert("vortexIds".into(), json!(runtime.selection.vortex_ids));
        if let Some(vortex_hover) = runtime.hovered_vortex_full_id.as_deref() {
            object.insert("hoveredVortexFullId".into(), json!(vortex_hover));
        }
        if let Some(transform_mode) = puzzle3d_transform_handle(&envelope.active_utility) {
            object.insert("transformMode".into(), json!(transform_mode));
            object.insert(
                "gumballConfig".into(),
                json!({
                    "moveAxes": runtime.transform_move,
                    "movePlanes": runtime.transform_move,
                    "rotate": runtime.transform_rotate,
                    "scaleAxes": false,
                    "scalePlanes": false,
                    "scaleUniform": false,
                }),
            );
        }
        if let Some(active_id) = runtime.selection.object_ids.first() {
            object.insert("activeObjectId".into(), json!(active_id));
        }
        if let Some(kind_id) = runtime.hovered_kind_id.as_deref() {
            object.insert("hoveredKindId".into(), json!(kind_id));
        }
        let gumball_active = puzzle3d_gumball_active(runtime, &envelope.active_utility);
        object.insert("gumballActive".into(), json!(gumball_active));
        if gumball_active {
            if let Some(target) = gumball_target_world(envelope) {
                object.insert("gumballTarget".into(), json!(target));
            }
        }
    }
    value.to_string()
}

fn gumball_target_world(envelope: &Puzzle3dScene) -> Option<[f64; 3]> {
    let selected: Vec<&Puzzle3dObject> = envelope.fixture.objects.iter().filter(|object| envelope.runtime.selection.object_ids.contains(&object.id)).collect();
    if selected.is_empty() {
        return None;
    }
    let mut sum = [0.0, 0.0, 0.0];
    for object in &selected {
        sum[0] += object.origin.first().copied().unwrap_or(0.0);
        sum[1] += object.origin.get(1).copied().unwrap_or(0.0);
        sum[2] += object.origin.get(2).copied().unwrap_or(0.0);
    }
    let count = selected.len() as f64;
    Some([sum[0] / count, sum[1] / count, sum[2] / count])
}

/// 🎯️ `dispatch`'s `Fixture` outcome is `puzzle_3d_engine`'s own typed fixture shape, distinct from
/// this app's `Puzzle3dFixture` document model — bridged through one JSON round trip (schema
/// translation between two independently-evolved Rust types, not a wasm-bindgen boundary) exactly
/// like `scene_config_json` bridges the same two schemas in the other direction.
fn fixture_from_engine_fixture(envelope: &Puzzle3dScene, fixture: &puzzle_3d_engine::Fixture) -> Option<Puzzle3dScene> {
    let parsed = serde_json::to_value(fixture).ok()?;
    let mut next = envelope.clone();
    next.fixture.objects = serde_json::from_value(parsed.get("objects")?.clone()).ok()?;
    next.fixture.attractions = parsed.get("attractions").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
    next.fixture.target_volumes = parsed.get("targetVolumes").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
    Some(next)
}

#[derive(Deserialize, Clone)]
struct Puzzle3dFillDisplayPayload {
    #[serde(default)]
    objects: Vec<Puzzle3dObject>,
    #[serde(default)]
    attractions: Vec<Puzzle3dAttraction>,
}

#[derive(Clone)]
struct FillDisplayMemo {
    plan_count: u32,
    available_count: u32,
    applied_count: u32,
    payload: Puzzle3dFillDisplayPayload,
}

/// 🎯️ Bridges `puzzle_3d_engine::Fixture` (typed `compose_fill_display` outcome) into this app's own
/// `Puzzle3dObject`/`Puzzle3dAttraction` document types via one JSON round trip — same schema
/// translation `fixture_from_engine_fixture` does, factored out since both fill-display helpers below
/// need it.
fn fill_display_payload_from_fixture(fixture: &puzzle_3d_engine::Fixture) -> Option<Puzzle3dFillDisplayPayload> {
    serde_json::to_value(fixture).ok().and_then(|value| serde_json::from_value(value).ok())
}

fn append_fill_display_tail(fixture: &mut Puzzle3dFixture, payload: &Puzzle3dFillDisplayPayload, applied_count: u32, available_count: u32) {
    let reveal_count = (available_count - applied_count) as usize;
    let objects_tail_start = payload.objects.len().saturating_sub(reveal_count);
    fixture.objects.extend(payload.objects.iter().skip(objects_tail_start).cloned());
    let attractions_tail_start = payload.attractions.len().saturating_sub(reveal_count);
    fixture.attractions.extend(payload.attractions.iter().skip(attractions_tail_start).cloned());
}

fn puzzle3d_fixture_with_fill_display_memo(
    mut fixture: Puzzle3dFixture,
    precompute: &Puzzle3dPrecomputeSession,
    applied_count: u32,
    available_count: u32,
    memo: &Mutex<Option<FillDisplayMemo>>,
) -> Puzzle3dFixture {
    if available_count <= applied_count {
        return fixture;
    }
    let plan_count: u32 = precompute.fill_progress_summary().count as u32;
    let cached = memo.lock().ok().and_then(|guard| {
        guard.as_ref().filter(|entry| entry.plan_count == plan_count && entry.available_count == available_count && entry.applied_count == applied_count).cloned()
    });
    let payload = if let Some(entry) = cached {
        entry.payload
    } else {
        let payload = precompute.compose_fill_display(available_count).and_then(|engine_fixture| fill_display_payload_from_fixture(&engine_fixture));
        if let Some(payload) = payload {
            if let Ok(mut guard) = memo.lock() {
                *guard = Some(FillDisplayMemo { plan_count, available_count, applied_count, payload: payload.clone() });
            }
            payload
        } else {
            return fixture;
        }
    };
    append_fill_display_tail(&mut fixture, &payload, applied_count, available_count);
    fixture
}

/// 🔒️ Clamps to what the engine actually has planned so far — the slider primitive already clamps to
/// `ready` client-side, this is the root-level backstop so the committed value and the document can
/// never disagree with what `compose_fill_display`/`apply_fill_count` actually applied.
fn apply_puzzle3d_fill_count(precompute: &mut Puzzle3dPrecomputeSession, mut envelope: Puzzle3dScene, count: u32) -> Puzzle3dScene {
    if count > 0 {
        envelope.active_utility = "fill".into();
    }
    envelope.runtime.fill_count = count.min(precompute.fill_available_count());
    if let Ok(Puzzle3dEngineOutcome::Fixture(fixture)) = precompute.dispatch(Puzzle3dEngineCommand::ApplyFillCount { count }) {
        if let Some(next) = fixture_from_engine_fixture(&envelope, &fixture) {
            envelope = next;
        }
    }
    envelope
}

/// 🎯️ Mirrors the host's client-side `handleZoomToSelection` framing math so a keybinding/engagement-token
/// driven focus (which bypasses that host interception) still produces a sensible camera. Camera-only:
/// writes `envelope.runtime.camera` (session-only per-window state), never the shared `fixture`.
fn apply_puzzle3d_focus_selection(envelope: &mut Puzzle3dScene) {
    let selected_origins: Vec<[f64; 3]> = envelope.fixture.objects.iter().filter(|object| envelope.runtime.selection.object_ids.contains(&object.id)).map(|object| object.origin).collect();
    if selected_origins.is_empty() {
        return;
    }
    let count = selected_origins.len() as f64;
    let mut center = [0.0, 0.0, 0.0];
    for origin in &selected_origins {
        center[0] += origin[0];
        center[1] += origin[1];
        center[2] += origin[2];
    }
    center = [center[0] / count, center[1] / count, center[2] / count];
    let max_distance = selected_origins
        .iter()
        .map(|origin| {
            let dx = origin[0] - center[0];
            let dy = origin[1] - center[1];
            let dz = origin[2] - center[2];
            (dx * dx + dy * dy + dz * dz).sqrt()
        })
        .fold(1.0_f64, f64::max);
    let distance = max_distance * 3.0 + 2.0;
    envelope.runtime.camera.position = [center[0] + distance * 0.6, center[1] - distance * 0.6, center[2] + distance * 0.5];
    envelope.runtime.camera.target = center;
}

fn next_object_id() -> String {
    let next = PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("object-{next}")
}

/// 🧊️ Seeds real vortices for a freshly placed object from its kind catalog's `vortices` templates, so it is immediately brushable instead of connector-less.
fn puzzle3d_vortices_from_kind_template(catalog_entry: &Value) -> Vec<Puzzle3dVortex> {
    catalog_entry
        .get("vortices")
        .and_then(|value| value.as_array())
        .map(|templates| {
            templates
                .iter()
                .enumerate()
                .map(|(index, template)| {
                    let position = template.get("position").and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0]);
                    let direction = template.get("direction").and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok());
                    let radius = template.get("radius").and_then(|value| value.as_f64());
                    Puzzle3dVortex { id: format!("v{index}"), vortex_kind: template.get("vortexKind").and_then(|value| value.as_str()).map(str::to_string), position, direction, radius, hidden: false, locked: false }
                })
                .collect()
        })
        .unwrap_or_default()
}

/// 🙈️ Applies `hidden`/`locked` to the given ids of one entity kind — `"vortex"` ids are full ids (`objectId:vortexId`).
fn apply_puzzle3d_selection_flag(fixture: &mut Puzzle3dFixture, entity: &str, ids: &[String], flag: &str, value: bool) {
    if ids.is_empty() {
        return;
    }
    let ids: HashSet<&str> = ids.iter().map(String::as_str).collect();
    match entity {
        "object" => {
            for object in fixture.objects.iter_mut().filter(|object| ids.contains(object.id.as_str())) {
                if flag == "locked" { object.locked = value } else { object.hidden = value }
            }
        }
        "vortex" => {
            for object in fixture.objects.iter_mut() {
                for vortex in object.vortices.iter_mut() {
                    if ids.contains(puzzle3d_vortex_full_id(&object.id, &vortex.id).as_str()) {
                        if flag == "locked" { vortex.locked = value } else { vortex.hidden = value }
                    }
                }
            }
        }
        "reference" => {
            for reference in fixture.references.iter_mut().filter(|reference| ids.contains(reference.id.as_str())) {
                if flag == "locked" { reference.locked = value } else { reference.hidden = value }
            }
        }
        "targetVolume" => {
            for volume in fixture.target_volumes.iter_mut().filter(|volume| ids.contains(volume.id.as_str())) {
                if flag == "locked" { volume.locked = value } else { volume.hidden = value }
            }
        }
        _ => {}
    }
}

fn value_as_vec3(value: &Value) -> Option<[f64; 3]> {
    let array = value.as_array()?;
    Some([array.first()?.as_f64()?, array.get(1)?.as_f64()?, array.get(2)?.as_f64()?])
}

/** @emoji 📐️ Resolves one numeric-field edit: an absolute `value` (typed entry) wins when
 * present, otherwise a `delta` (stepper nudge) is added to `current` — offset-preserving across
 * a multi-select where `current` differs per entity. `None` when neither parses. */
fn puzzle3d_resolve_number_edit(current: f64, value: Option<&Value>, delta: Option<&Value>) -> Option<f64> {
    if let Some(absolute) = value.and_then(Value::as_f64) {
        return Some(absolute);
    }
    delta.and_then(Value::as_f64).map(|delta| current + delta)
}

/** @emoji 📐️ Settings-panel counterpart to `puzzle3d_resolve_number_edit`: reads `value`/`delta`
 * directly out of an action's `args`, for single global settings (not per-entity multi-select)
 * whose stepper dispatches straight to their own dedicated action. */
fn puzzle3d_absolute_or_delta(args: Option<&Value>, current: f64) -> Option<f64> {
    puzzle3d_resolve_number_edit(current, args.and_then(|value| value.get("value")), args.and_then(|value| value.get("delta")))
}

/** @emoji 📐️ Parses a nested stepper-group field id as `"<base>.<axis>"` (`x`/`y`/`z`/`w`),
 * returning the axis index when `field` names a component of `base` — the dot-path convention
 * `ui_inspector_vec3_group`/`inspector_quat_group` use for their per-axis actions. */
fn puzzle3d_axis_index(field: &str, base: &str) -> Option<usize> {
    match field.strip_prefix(base)?.strip_prefix('.')? {
        "x" => Some(0),
        "y" => Some(1),
        "z" => Some(2),
        "w" => Some(3),
        _ => None,
    }
}

/// 🔎️ Generic inspector edit dispatcher — `entity`/`field` select the target, `ids` scope it (full ids for vortices, `objectId:vortexId`).
/// `hidden`/`locked` delegate to `apply_puzzle3d_selection_flag` (shared with the non-inspector toggle path); every other field
/// resolves via `value` (absolute) or `delta` (relative, added to each entity's own current component).
fn apply_puzzle3d_inspector_patch(fixture: &mut Puzzle3dFixture, entity: &str, ids: &[String], field: &str, value: Option<&Value>, delta: Option<&Value>) {
    if ids.is_empty() {
        return;
    }
    if field == "hidden" || field == "locked" {
        if let Some(pressed) = value.and_then(Value::as_bool) {
            apply_puzzle3d_selection_flag(fixture, entity, ids, field, pressed);
        }
        return;
    }
    let id_set: HashSet<&str> = ids.iter().map(String::as_str).collect();
    match entity {
        "object" => {
            for object in fixture.objects.iter_mut().filter(|object| id_set.contains(object.id.as_str())) {
                match field {
                    "label" => object.label = value.and_then(Value::as_str).map(str::to_string),
                    "objectKind" => object.object_kind = value.and_then(Value::as_str).map(str::to_string),
                    "meshUrl" => object.mesh_url = value.and_then(Value::as_str).map(str::to_string),
                    "origin" => {
                        if let Some(origin) = value.and_then(value_as_vec3) {
                            object.origin = origin;
                        }
                    }
                    _ => {
                        if let Some(axis) = puzzle3d_axis_index(field, "origin") {
                            if let Some(updated) = puzzle3d_resolve_number_edit(object.origin[axis], value, delta) {
                                object.origin[axis] = updated;
                            }
                        } else if let Some(axis) = puzzle3d_axis_index(field, "scale") {
                            let mut scale = object_scale_json(object);
                            if let Some(updated) = puzzle3d_resolve_number_edit(scale[axis], value, delta) {
                                scale[axis] = updated;
                                object.scale = Some(json!(scale));
                            }
                        } else if let Some(axis) = puzzle3d_axis_index(field, "orientation") {
                            let mut quat = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                            if let Some(updated) = puzzle3d_resolve_number_edit(quat[axis], value, delta) {
                                quat[axis] = updated;
                                object.orientation = Some(quat_normalize(quat));
                            }
                        }
                    }
                }
            }
        }
        "vortex" => {
            for object in fixture.objects.iter_mut() {
                for vortex in object.vortices.iter_mut() {
                    if !id_set.contains(puzzle3d_vortex_full_id(&object.id, &vortex.id).as_str()) {
                        continue;
                    }
                    match field {
                        "vortexKind" => vortex.vortex_kind = value.and_then(Value::as_str).map(str::to_string),
                        "position" => {
                            if let Some(position) = value.and_then(value_as_vec3) {
                                vortex.position = position;
                            }
                        }
                        "direction" => {
                            if let Some(direction) = value.and_then(value_as_vec3) {
                                vortex.direction = Some(direction);
                            }
                        }
                        "radius" => {
                            if let Some(updated) = puzzle3d_resolve_number_edit(vortex.radius.unwrap_or(0.35), value, delta) {
                                vortex.radius = Some(updated);
                            }
                        }
                        _ => {
                            if let Some(axis) = puzzle3d_axis_index(field, "position") {
                                if let Some(updated) = puzzle3d_resolve_number_edit(vortex.position[axis], value, delta) {
                                    vortex.position[axis] = updated;
                                }
                            } else if let Some(axis) = puzzle3d_axis_index(field, "direction") {
                                let mut direction = vortex.direction.unwrap_or([0.0, 0.0, 1.0]);
                                if let Some(updated) = puzzle3d_resolve_number_edit(direction[axis], value, delta) {
                                    direction[axis] = updated;
                                    vortex.direction = Some(direction);
                                }
                            }
                        }
                    }
                }
            }
        }
        "attraction" => {
            for attraction in fixture.attractions.iter_mut().filter(|attraction| id_set.contains(attraction.id.as_str())) {
                match field {
                    "attracting" => {
                        if let Some(text) = value.and_then(Value::as_str) {
                            attraction.attracting = text.into();
                        }
                    }
                    "attracted" => {
                        if let Some(text) = value.and_then(Value::as_str) {
                            attraction.attracted = text.into();
                        }
                    }
                    "gap" => {
                        if let Some(v) = puzzle3d_resolve_number_edit(attraction.gap, value, delta) {
                            attraction.gap = v;
                        }
                    }
                    "shift" => {
                        if let Some(v) = puzzle3d_resolve_number_edit(attraction.shift, value, delta) {
                            attraction.shift = v;
                        }
                    }
                    "rise" => {
                        if let Some(v) = puzzle3d_resolve_number_edit(attraction.rise, value, delta) {
                            attraction.rise = v;
                        }
                    }
                    "rotation" => {
                        if let Some(v) = puzzle3d_resolve_number_edit(attraction.rotation, value, delta) {
                            attraction.rotation = v;
                        }
                    }
                    "turn" => {
                        if let Some(v) = puzzle3d_resolve_number_edit(attraction.turn, value, delta) {
                            attraction.turn = v;
                        }
                    }
                    "tilt" => {
                        if let Some(v) = puzzle3d_resolve_number_edit(attraction.tilt, value, delta) {
                            attraction.tilt = v;
                        }
                    }
                    _ => {}
                }
            }
        }
        "reference" => {
            for reference in fixture.references.iter_mut().filter(|reference| id_set.contains(reference.id.as_str())) {
                match field {
                    "sourceUrl" => {
                        if let Some(text) = value.and_then(Value::as_str) {
                            reference.source.url = text.into();
                        }
                    }
                    "mediaKind" => reference.source.media_kind = value.and_then(Value::as_str).map(str::to_string),
                    "origin" => {
                        if let Some(origin) = value.and_then(value_as_vec3) {
                            reference.origin = origin;
                        }
                    }
                    "widthWorld" => {
                        if let Some(width) = puzzle3d_resolve_number_edit(reference.width_world, value, delta) {
                            reference.width_world = width;
                        }
                    }
                    _ => {
                        if let Some(axis) = puzzle3d_axis_index(field, "origin") {
                            if let Some(updated) = puzzle3d_resolve_number_edit(reference.origin[axis], value, delta) {
                                reference.origin[axis] = updated;
                            }
                        }
                    }
                }
            }
        }
        "targetVolume" => {
            for volume in fixture.target_volumes.iter_mut().filter(|volume| id_set.contains(volume.id.as_str())) {
                match field {
                    "origin" => {
                        if let Some(origin) = value.and_then(value_as_vec3) {
                            volume.origin = origin;
                        }
                    }
                    _ => {
                        if let Some(axis) = puzzle3d_axis_index(field, "origin") {
                            if let Some(updated) = puzzle3d_resolve_number_edit(volume.origin[axis], value, delta) {
                                volume.origin[axis] = updated;
                            }
                        } else if let Some(axis) = puzzle3d_axis_index(field, "scale") {
                            let mut scale = target_volume_scale_json(volume);
                            if let Some(updated) = puzzle3d_resolve_number_edit(scale[axis], value, delta) {
                                scale[axis] = updated;
                                volume.scale = Some(json!(scale));
                            }
                        } else if let Some(axis) = puzzle3d_axis_index(field, "orientation") {
                            let mut quat = volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                            if let Some(updated) = puzzle3d_resolve_number_edit(quat[axis], value, delta) {
                                quat[axis] = updated;
                                volume.orientation = Some(quat_normalize(quat));
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
//#endregion 🔖️Document

//#region 🔖️AttractionResolve
/// 📐️ Attraction placement math — a quaternion-only port of semio_compose_rs's `compute_child_plane`
/// (semio_compose_rs/client/lib/rs/lib.rs:1328) so it composes directly with `Puzzle3dObject.orientation`. Every attraction
/// is directed (`attracting` → `attracted`); an attracted object's world pose is derived from the attracting
/// vortex's world pose plus the 6 connection-style parameters (gap/shift/rise/rotation/turn/tilt, angles in
/// degrees, same semantics as semio_compose_rs connections).
const PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE: f64 = 0.01;

fn vec3_sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn vec3_add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

fn vec3_scale(a: [f64; 3], s: f64) -> [f64; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

fn vec3_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn vec3_dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn vec3_len(a: [f64; 3]) -> f64 {
    vec3_dot(a, a).sqrt()
}

fn vec3_normalize(a: [f64; 3]) -> [f64; 3] {
    let len = vec3_len(a);
    if len < 1e-12 {
        a
    } else {
        vec3_scale(a, 1.0 / len)
    }
}

fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}

fn quat_conjugate(q: [f64; 4]) -> [f64; 4] {
    [-q[0], -q[1], -q[2], q[3]]
}

fn quat_normalize(q: [f64; 4]) -> [f64; 4] {
    let len = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    if len < 1e-12 {
        [0.0, 0.0, 0.0, 1.0]
    } else {
        [q[0] / len, q[1] / len, q[2] / len, q[3] / len]
    }
}

/// 🧭️ Ports semio_compose_rs's `quaternion_from_unit_vectors` (semio_compose_rs/client/lib/rs/lib.rs:1276) — the quaternion rotating
/// unit vector `from` onto unit vector `to`.
fn puzzle3d_quaternion_from_unit_vectors(from: [f64; 3], to: [f64; 3]) -> [f64; 4] {
    let r = vec3_dot(from, to) + 1.0;
    let quat = if r < 0.000_001 {
        if from[0].abs() > from[2].abs() {
            [-from[1], from[0], 0.0, 0.0]
        } else {
            [0.0, -from[2], from[1], 0.0]
        }
    } else {
        let c = vec3_cross(from, to);
        [c[0], c[1], c[2], r]
    };
    quat_normalize(quat)
}

/// 🧲️ Ports semio_compose_rs's align-quaternion special-case branch (semio_compose_rs/client/lib/rs/lib.rs:1345-1356) for when the
/// attracted vortex is already (anti)parallel to the attracting vortex. Falls back to an alternate cross axis when
/// the attracting direction is exactly ±Z — a double-degenerate corner semio_compose_rs's own branch doesn't otherwise guard.
fn puzzle3d_attraction_align_quat(parent_dir: [f64; 3], child_dir: [f64; 3]) -> [f64; 4] {
    let reverse_child = vec3_scale(child_dir, -1.0);
    let cross_vec = vec3_cross(parent_dir, reverse_child);
    if vec3_len(cross_vec) < PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE {
        if parent_dir[2].abs() < PUZZLE3D_ATTRACTION_ALIGN_TOLERANCE {
            puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], [0.0, 0.0, -1.0])
        } else {
            let mut axis = vec3_cross([0.0, 0.0, 1.0], parent_dir);
            if vec3_len(axis) < 1e-9 {
                axis = vec3_cross([1.0, 0.0, 0.0], parent_dir);
            }
            let axis = vec3_normalize(axis);
            let half = std::f64::consts::FRAC_PI_2;
            quat_normalize([axis[0] * half.sin(), axis[1] * half.sin(), axis[2] * half.sin(), half.cos()])
        }
    } else {
        puzzle3d_quaternion_from_unit_vectors(reverse_child, parent_dir)
    }
}

/// 📌️ Resolves an attraction endpoint (`objectId:vortexId`) to its owning object id and its vortex's LOCAL
/// (object-frame) position/direction — the frame semio_compose_rs's connector math expects, before the object's own world
/// transform is applied.
fn puzzle3d_local_vortex_geom(fixture: &Puzzle3dFixture, full_id: &str) -> Option<(String, [f64; 3], [f64; 3])> {
    for object in &fixture.objects {
        for vortex in &object.vortices {
            if puzzle3d_vortex_full_id(&object.id, &vortex.id) == full_id {
                return Some((object.id.clone(), vortex.position, vortex.direction.unwrap_or([0.0, 0.0, -1.0])));
            }
        }
    }
    None
}

/// 🔗️ Resolves an attraction's `attracting`/`attracted` vortex full-ids to their owning object ids. Returns `None`
/// for dangling references or same-object attractions (legal today but not a resolvable directed edge).
fn puzzle3d_attraction_object_ids(fixture: &Puzzle3dFixture, attraction: &Puzzle3dAttraction) -> Option<(String, String)> {
    let attracting_object = puzzle3d_local_vortex_geom(fixture, &attraction.attracting)?.0;
    let attracted_object = puzzle3d_local_vortex_geom(fixture, &attraction.attracted)?.0;
    if attracting_object == attracted_object {
        return None;
    }
    Some((attracting_object, attracted_object))
}

/// 📐️ Forward attraction placement — given the attracting object's world pose (`t_a`/`q_a`), both vortices' LOCAL
/// position/direction, and the 6 connection-style parameters (angles in degrees), returns the attracted object's
/// world pose. Exact quaternion port of semio_compose_rs's `compute_child_plane`.
#[allow(clippy::too_many_arguments)]
fn puzzle3d_attraction_child_pose(t_a: [f64; 3], q_a: [f64; 4], p_a: [f64; 3], d_a: [f64; 3], p_b: [f64; 3], d_b: [f64; 3], gap: f64, shift: f64, rise: f64, rotation_deg: f64, turn_deg: f64, tilt_deg: f64) -> ([f64; 3], [f64; 4]) {
    let parent_dir = vec3_normalize(d_a);
    let child_dir = vec3_normalize(d_b);
    let align_q = puzzle3d_attraction_align_quat(parent_dir, child_dir);

    let pq = puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], parent_dir);
    let gap_dir = quat_rotate_vector(pq, [0.0, 1.0, 0.0]);
    let shift_dir = quat_rotate_vector(pq, [1.0, 0.0, 0.0]);
    let raise_dir = quat_rotate_vector(pq, [0.0, 0.0, 1.0]);

    let rotate_q = quat_from_axis_angle(parent_dir[0], parent_dir[1], parent_dir[2], -deg_to_rad(rotation_deg));
    let turn_axis = quat_rotate_vector(rotate_q, raise_dir);
    let tilt_axis = quat_rotate_vector(rotate_q, shift_dir);
    let turn_q = quat_from_axis_angle(turn_axis[0], turn_axis[1], turn_axis[2], deg_to_rad(turn_deg));
    let tilt_q = quat_from_axis_angle(tilt_axis[0], tilt_axis[1], tilt_axis[2], deg_to_rad(tilt_deg));

    let mut orientation_local = quat_conjugate(align_q);
    orientation_local = quat_mul(orientation_local, quat_conjugate(rotate_q));
    orientation_local = quat_mul(orientation_local, quat_conjugate(turn_q));
    orientation_local = quat_mul(orientation_local, quat_conjugate(tilt_q));
    let orientation_local = quat_normalize(orientation_local);

    let offset = vec3_add(vec3_add(t_a, p_a), vec3_add(vec3_add(vec3_scale(gap_dir, gap), vec3_scale(shift_dir, shift)), vec3_scale(raise_dir, rise)));
    let t_b = vec3_sub(quat_rotate_vector(orientation_local, offset), p_b);
    let q_b = quat_normalize(quat_mul(orientation_local, q_a));
    (t_b, q_b)
}

/// 🔁️ Inverse of `puzzle3d_attraction_child_pose` — given the attracted object's CURRENT world pose, derives the 6
/// parameters that reproduce it exactly, so moving/rotating an attracted object never causes a resolve-triggered
/// snap-back and creating an attraction never moves either endpoint.
#[allow(clippy::too_many_arguments)]
fn derive_attraction_params(t_a: [f64; 3], q_a: [f64; 4], p_a: [f64; 3], d_a: [f64; 3], p_b: [f64; 3], d_b: [f64; 3], t_b: [f64; 3], q_b: [f64; 4]) -> (f64, f64, f64, f64, f64, f64) {
    let parent_dir = vec3_normalize(d_a);
    let child_dir = vec3_normalize(d_b);
    let align_q = puzzle3d_attraction_align_quat(parent_dir, child_dir);
    let pq = puzzle3d_quaternion_from_unit_vectors([0.0, 1.0, 0.0], parent_dir);
    let gap_dir = quat_rotate_vector(pq, [0.0, 1.0, 0.0]);
    let shift_dir = quat_rotate_vector(pq, [1.0, 0.0, 0.0]);
    let raise_dir = quat_rotate_vector(pq, [0.0, 0.0, 1.0]);

    let orientation_local = quat_normalize(quat_mul(q_b, quat_conjugate(q_a)));

    let offset = quat_rotate_vector(quat_conjugate(orientation_local), vec3_add(t_b, p_b));
    let diff = vec3_sub(vec3_sub(offset, t_a), p_a);
    let gap = vec3_dot(diff, gap_dir);
    let shift = vec3_dot(diff, shift_dir);
    let rise = vec3_dot(diff, raise_dir);

    let residual = quat_mul(align_q, orientation_local);
    let m = quat_mul(quat_mul(quat_conjugate(pq), residual), pq);
    let col_x = quat_rotate_vector(m, [1.0, 0.0, 0.0]);
    let col_y = quat_rotate_vector(m, [0.0, 1.0, 0.0]);

    let clamp = |v: f64| v.clamp(-1.0, 1.0);
    let tilt_rad = -(clamp(col_y[2])).asin();
    let (rotation_rad, turn_rad) = if (col_y[2].abs() - 1.0).abs() < 1e-6 {
        (col_x[1].atan2(col_x[0]), 0.0)
    } else {
        let col_z = quat_rotate_vector(m, [0.0, 0.0, 1.0]);
        ((-col_x[2]).atan2(col_z[2]), col_y[0].atan2(col_y[1]))
    };

    (gap, shift, rise, rad_to_deg(rotation_rad), rad_to_deg(turn_rad), rad_to_deg(tilt_rad))
}

/// 🌲️ Resolves every attracted object's world pose from its attracting root, over a directed BFS per
/// weakly-connected component. Roots are in-degree-zero objects; a component that is a pure cycle (the "donut"
/// case) picks the lexicographically smallest object id in that component as a deterministic root. Multiple
/// incoming attractions to the same object are resolved first-visit-wins (mirrors semio_compose_rs's
/// `flatten_design_positions` cycle handling). Idempotent: re-running against already-resolved poses reproduces
/// them exactly. Returns, for every non-root object touched, the attraction index that positioned it — callers
/// (e.g. `translateSelection`) use this to rederive params before a direct move so resolving never snaps it back.
fn resolve_puzzle3d_attractions(fixture: &mut Puzzle3dFixture) -> HashMap<String, usize> {
    let mut edges: HashMap<String, Vec<(String, usize)>> = HashMap::new();
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    let mut all_object_ids: Vec<String> = fixture.objects.iter().map(|object| object.id.clone()).collect();
    all_object_ids.sort();
    for id in &all_object_ids {
        in_degree.entry(id.clone()).or_insert(0);
    }
    for (index, attraction) in fixture.attractions.iter().enumerate() {
        if let Some((attracting_id, attracted_id)) = puzzle3d_attraction_object_ids(fixture, attraction) {
            edges.entry(attracting_id).or_default().push((attracted_id.clone(), index));
            *in_degree.entry(attracted_id).or_insert(0) += 1;
        }
    }

    fn find(parent_of: &mut HashMap<String, String>, id: &str) -> String {
        let mut current = id.to_string();
        while parent_of[&current] != current {
            let grandparent = parent_of[&parent_of[&current]].clone();
            parent_of.insert(current.clone(), grandparent.clone());
            current = grandparent;
        }
        current
    }
    fn union(parent_of: &mut HashMap<String, String>, a: &str, b: &str) {
        let root_a = find(parent_of, a);
        let root_b = find(parent_of, b);
        if root_a != root_b {
            parent_of.insert(root_a, root_b);
        }
    }
    let mut parent_of: HashMap<String, String> = all_object_ids.iter().map(|id| (id.clone(), id.clone())).collect();
    for (attracting_id, targets) in &edges {
        for (attracted_id, _) in targets {
            union(&mut parent_of, attracting_id, attracted_id);
        }
    }

    let mut components: HashMap<String, Vec<String>> = HashMap::new();
    for id in &all_object_ids {
        let root = find(&mut parent_of, id);
        components.entry(root).or_default().push(id.clone());
    }
    let mut component_keys: Vec<String> = components.keys().cloned().collect();
    component_keys.sort();

    let mut incoming: HashMap<String, usize> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();

    for component_key in component_keys {
        let mut members = components.remove(&component_key).unwrap_or_default();
        members.sort();
        let roots: Vec<String> = members.iter().filter(|id| in_degree.get(id.as_str()).copied().unwrap_or(0) == 0).cloned().collect();
        let seed_roots: Vec<String> = if roots.is_empty() { vec![members[0].clone()] } else { roots };

        let mut queue: std::collections::VecDeque<String> = std::collections::VecDeque::new();
        for root in &seed_roots {
            if visited.insert(root.clone()) {
                queue.push_back(root.clone());
            }
        }
        while let Some(current_id) = queue.pop_front() {
            let Some(targets) = edges.get(&current_id) else { continue };
            for (attracted_id, attraction_index) in targets.clone() {
                if visited.contains(&attracted_id) {
                    continue;
                }
                let attraction = fixture.attractions[attraction_index].clone();
                let (Some((_, p_a, d_a)), Some((_, p_b, d_b))) = (puzzle3d_local_vortex_geom(fixture, &attraction.attracting), puzzle3d_local_vortex_geom(fixture, &attraction.attracted)) else { continue };
                let Some(attracting_object) = fixture.objects.iter().find(|object| object.id == current_id) else { continue };
                let t_a = attracting_object.origin;
                let q_a = attracting_object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                let (t_b, q_b) = puzzle3d_attraction_child_pose(t_a, q_a, p_a, d_a, p_b, d_b, attraction.gap, attraction.shift, attraction.rise, attraction.rotation, attraction.turn, attraction.tilt);
                if let Some(attracted_object) = fixture.objects.iter_mut().find(|object| object.id == attracted_id) {
                    attracted_object.origin = t_b;
                    attracted_object.orientation = Some(q_b);
                }
                incoming.insert(attracted_id.clone(), attraction_index);
                visited.insert(attracted_id.clone());
                queue.push_back(attracted_id);
            }
        }
    }
    incoming
}

/// 🧰️ Rederives every attraction's 6 params from its endpoints' CURRENT poses. Used after merging externally
/// computed poses (brush/fill placement via the collision-aware `puzzle_3d` engine, which knows nothing about
/// gap/shift/rise/rotation/turn/tilt) so the follow-up resolve reproduces those poses exactly instead of
/// re-deriving a bare port-to-port docking that could visibly jump the just-placed object.
fn puzzle3d_rederive_all_attractions(fixture: &mut Puzzle3dFixture) {
    let ids: Vec<String> = fixture.attractions.iter().map(|attraction| attraction.id.clone()).collect();
    for id in ids {
        let Some(attraction) = fixture.attractions.iter().find(|attraction| attraction.id == id).cloned() else { continue };
        let (Some((attracting_id, p_a, d_a)), Some((attracted_id, p_b, d_b))) = (puzzle3d_local_vortex_geom(fixture, &attraction.attracting), puzzle3d_local_vortex_geom(fixture, &attraction.attracted)) else { continue };
        let pose = |object_id: &str| fixture.objects.iter().find(|object| object.id == object_id).map(|object| (object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])));
        let (Some((t_a, q_a)), Some((t_b, q_b))) = (pose(&attracting_id), pose(&attracted_id)) else { continue };
        let (gap, shift, rise, rotation, turn, tilt) = derive_attraction_params(t_a, q_a, p_a, d_a, p_b, d_b, t_b, q_b);
        if let Some(attraction) = fixture.attractions.iter_mut().find(|attraction| attraction.id == id) {
            attraction.gap = gap;
            attraction.shift = shift;
            attraction.rise = rise;
            attraction.rotation = rotation;
            attraction.turn = turn;
            attraction.tilt = tilt;
        }
    }
}

/// ✋️ After a direct move/rotate on selected objects, rederives the 6 params of every moved object's incoming
/// attraction (per the `incoming` map from a prior `resolve_puzzle3d_attractions` call) from its NEW pose, so the
/// follow-up resolve reproduces that pose exactly instead of snapping the object back to its old one. Harmless for
/// objects whose attracting object was moved by the same delta (relative pose is unchanged, so derived params come
/// out unchanged too).
fn puzzle3d_rederive_moved_attractions(fixture: &mut Puzzle3dFixture, moved_ids: &[String], incoming: &HashMap<String, usize>) {
    for object_id in moved_ids {
        let Some(&attraction_index) = incoming.get(object_id) else { continue };
        let Some(attraction) = fixture.attractions.get(attraction_index).cloned() else { continue };
        let (Some((attracting_id, p_a, d_a)), Some((_, p_b, d_b))) = (puzzle3d_local_vortex_geom(fixture, &attraction.attracting), puzzle3d_local_vortex_geom(fixture, &attraction.attracted)) else { continue };
        let Some(t_a_q_a) = fixture.objects.iter().find(|object| object.id == attracting_id).map(|object| (object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]))) else { continue };
        let Some(t_b_q_b) = fixture.objects.iter().find(|object| &object.id == object_id).map(|object| (object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]))) else { continue };
        let (t_a, q_a) = t_a_q_a;
        let (t_b, q_b) = t_b_q_b;
        let (gap, shift, rise, rotation, turn, tilt) = derive_attraction_params(t_a, q_a, p_a, d_a, p_b, d_b, t_b, q_b);
        if let Some(attraction) = fixture.attractions.get_mut(attraction_index) {
            attraction.gap = gap;
            attraction.shift = shift;
            attraction.rise = rise;
            attraction.rotation = rotation;
            attraction.turn = turn;
            attraction.tilt = tilt;
        }
    }
}
//#endregion 🔖️AttractionResolve

//#region 🔖️Terminology
/// 🗣️ Complete UI label set for the 3d app; one field per label makes every terminology×locale combination compile-checked.
struct Puzzle3dLabels {
    objects: &'static str,
    object: &'static str,
    vortices: &'static str,
    vortex: &'static str,
    attractions: &'static str,
    attraction: &'static str,
    cables: &'static str,
    references: &'static str,
    reference: &'static str,
    target_volumes: &'static str,
    target_volume: &'static str,
    window_main: &'static str,
    example_concrete_forest: &'static str,
    fill: &'static str,
    count: &'static str,
    brush: &'static str,
    move_flag: &'static str,
    rotate_flag: &'static str,
    volume_brush: &'static str,
    voxel: &'static str,
    width: &'static str,
    depth: &'static str,
    height: &'static str,
    placement: &'static str,
    show: &'static str,
    hide: &'static str,
    lock: &'static str,
    unlock: &'static str,
    always: &'static str,
    selected: &'static str,
    selected_count: &'static str,
    vortex_show: &'static str,
    outwards: &'static str,
    inwards: &'static str,
    vortex_direction: &'static str,
    distribution: &'static str,
    suggest_objects: &'static str,
    duplicate: &'static str,
    select_same_kind: &'static str,
    zoom_to_selection: &'static str,
    delete: &'static str,
    select: &'static str,
    rectangle: &'static str,
    lasso: &'static str,
    selective: &'static str,
    additive: &'static str,
    subtractive: &'static str,
    invertive: &'static str,
    lod: &'static str,
    auto_zoom: &'static str,
    depth_variable: &'static str,
    grid: &'static str,
    visible: &'static str,
    snap: &'static str,
    spacing: &'static str,
    overlap_budget: &'static str,
    id: &'static str,
    label: &'static str,
    kind: &'static str,
    origin: &'static str,
    orientation: &'static str,
    scale: &'static str,
    mesh_url: &'static str,
    hidden: &'static str,
    locked: &'static str,
    full_id: &'static str,
    vortex_kind: &'static str,
    position: &'static str,
    direction: &'static str,
    radius: &'static str,
    attracting: &'static str,
    attracted: &'static str,
    gap: &'static str,
    shift: &'static str,
    rise: &'static str,
    rotation_deg: &'static str,
    turn_deg: &'static str,
    tilt_deg: &'static str,
    source_url: &'static str,
    media_kind: &'static str,
    settings: &'static str,
    selection_mode: &'static str,
    proximity_radius: &'static str,
    chunk_size: &'static str,
    schema: &'static str,
    domain: &'static str,
}

const PUZZLE3D_LABELS_NATIVE_EN: Puzzle3dLabels = Puzzle3dLabels {
    objects: "Objects",
    object: "Object",
    vortices: "Vortices",
    vortex: "Vortex",
    attractions: "Attractions",
    attraction: "Attraction",
    cables: "Cables",
    references: "References",
    reference: "Reference",
    target_volumes: "Target Volumes",
    target_volume: "Target Volume",
    window_main: "Puzzle 3D",
    example_concrete_forest: "Concrete Forest",
    fill: "Fill",
    count: "Count",
    brush: "Brush",
    move_flag: "Move",
    rotate_flag: "Rotate",
    volume_brush: "Volume Brush",
    voxel: "Voxel",
    width: "Width",
    depth: "Depth",
    height: "Height",
    placement: "Placement",
    show: "Show",
    hide: "Hide",
    lock: "Lock",
    unlock: "Unlock",
    always: "Always",
    selected: "Selected",
    selected_count: "selected",
    vortex_show: "Vortex Show",
    outwards: "Outwards",
    inwards: "Inwards",
    vortex_direction: "Vortex Direction",
    distribution: "Distribution",
    suggest_objects: "Suggest objects",
    duplicate: "Duplicate",
    select_same_kind: "Select all of same kind",
    zoom_to_selection: "Zoom to selection",
    delete: "Delete",
    select: "Select",
    rectangle: "Rectangle",
    lasso: "Lasso",
    selective: "Selective",
    additive: "Additive",
    subtractive: "Subtractive",
    invertive: "Invertive",
    lod: "LOD",
    auto_zoom: "Auto zoom",
    depth_variable: "Depth-variable",
    grid: "Grid",
    visible: "Visible",
    snap: "Snap",
    spacing: "Spacing",
    overlap_budget: "Overlap budget (m³)",
    id: "Id",
    label: "Label",
    kind: "Kind",
    origin: "Origin",
    orientation: "Orientation",
    scale: "Scale",
    mesh_url: "Mesh Url",
    hidden: "Hidden",
    locked: "Locked",
    full_id: "Full Id",
    vortex_kind: "Vortex Kind",
    position: "Position",
    direction: "Direction",
    radius: "Radius",
    attracting: "Attracting",
    attracted: "Attracted",
    gap: "Gap",
    shift: "Shift",
    rise: "Rise",
    rotation_deg: "Rotation (°)",
    turn_deg: "Turn (°)",
    tilt_deg: "Tilt (°)",
    source_url: "Source Url",
    media_kind: "Media Kind",
    settings: "Settings",
    selection_mode: "Selection Mode",
    proximity_radius: "Proximity Radius",
    chunk_size: "Chunk Size",
    schema: "Schema",
    domain: "Domain",
};
const PUZZLE3D_LABELS_NATIVE_DE: Puzzle3dLabels = Puzzle3dLabels {
    objects: "Objekte",
    object: "Objekt",
    vortices: "Vortices",
    vortex: "Vortex",
    attractions: "Anziehungen",
    attraction: "Anziehung",
    cables: "Kabel",
    references: "Referenzen",
    reference: "Referenz",
    target_volumes: "Zielvolumina",
    target_volume: "Zielvolumen",
    window_main: "Puzzle 3D",
    example_concrete_forest: "Betonwald",
    fill: "Füllen",
    count: "Anzahl",
    brush: "Pinsel",
    move_flag: "Verschieben",
    rotate_flag: "Drehen",
    volume_brush: "Volumenpinsel",
    voxel: "Voxel",
    width: "Breite",
    depth: "Tiefe",
    height: "Höhe",
    placement: "Platzierung",
    show: "Anzeigen",
    hide: "Ausblenden",
    lock: "Sperren",
    unlock: "Entsperren",
    always: "Immer",
    selected: "Auswahl",
    selected_count: "ausgewählt",
    vortex_show: "Vortex-Anzeige",
    outwards: "Auswärts",
    inwards: "Einwärts",
    vortex_direction: "Vortex-Richtung",
    distribution: "Verteilung",
    suggest_objects: "Objekte vorschlagen",
    duplicate: "Duplizieren",
    select_same_kind: "Alle gleicher Art auswählen",
    zoom_to_selection: "Zur Auswahl zoomen",
    delete: "Löschen",
    select: "Auswählen",
    rectangle: "Rechteck",
    lasso: "Lasso",
    selective: "Selektiv",
    additive: "Additiv",
    subtractive: "Subtraktiv",
    invertive: "Invertierend",
    lod: "Detailstufe",
    auto_zoom: "Automatischer Zoom",
    depth_variable: "Tiefenvariabel",
    grid: "Raster",
    visible: "Sichtbar",
    snap: "Fang",
    spacing: "Abstand",
    overlap_budget: "Überlappungsbudget (m³)",
    id: "Id",
    label: "Bezeichnung",
    kind: "Art",
    origin: "Ursprung",
    orientation: "Orientierung",
    scale: "Skalierung",
    mesh_url: "Mesh-URL",
    hidden: "Ausgeblendet",
    locked: "Gesperrt",
    full_id: "Vollständige Id",
    vortex_kind: "Vortex-Art",
    position: "Position",
    direction: "Richtung",
    radius: "Radius",
    attracting: "Anziehend",
    attracted: "Angezogen",
    gap: "Spalt",
    shift: "Verschiebung",
    rise: "Anstieg",
    rotation_deg: "Drehung (°)",
    turn_deg: "Drehung um Achse (°)",
    tilt_deg: "Neigung (°)",
    source_url: "Quell-URL",
    media_kind: "Medienart",
    settings: "Einstellungen",
    selection_mode: "Auswahlmodus",
    proximity_radius: "Näheradius",
    chunk_size: "Blockgröße",
    schema: "Schema",
    domain: "Domäne",
};
const PUZZLE3D_LABELS_REUSE_EN: Puzzle3dLabels = Puzzle3dLabels {
    objects: "Building components",
    object: "Building component",
    vortices: "Connection points",
    vortex: "Connection point",
    attractions: "Connections",
    attraction: "Connection",
    cables: "Cables",
    window_main: "Aggregator",
    example_concrete_forest: "Abbau Aufbau",
    vortex_show: "Show connection points",
    vortex_direction: "Connection point direction",
    vortex_kind: "Connection point kind",
    suggest_objects: "Suggest building components",
    attracting: "Host connection point",
    attracted: "Guest connection point",
    ..PUZZLE3D_LABELS_NATIVE_EN
};
const PUZZLE3D_LABELS_REUSE_DE: Puzzle3dLabels = Puzzle3dLabels {
    objects: "Baukomponenten",
    object: "Baukomponente",
    vortices: "Verbindungspunkte",
    vortex: "Verbindungspunkt",
    attractions: "Verbindungen",
    attraction: "Verbindung",
    cables: "Kabel",
    window_main: "Aggregator",
    example_concrete_forest: "Abbau Aufbau",
    vortex_show: "Verbindungspunkte anzeigen",
    vortex_direction: "Richtung der Verbindungspunkte",
    vortex_kind: "Verbindungspunkt-Art",
    suggest_objects: "Baukomponenten vorschlagen",
    attracting: "Wirts-Verbindungspunkt",
    attracted: "Gast-Verbindungspunkt",
    ..PUZZLE3D_LABELS_NATIVE_DE
};

/// 🗣️ Resolves the active label set from the shell-provided locale/terminology; unknown terminology ids fall back to native.
/// ⚠️ Not routed through the SDK's `LocaleLabels`/`app_labels!`/`resolve_labels` — see `puzzle2d_labels`'s
/// doc comment for why (an extra terminology axis the SDK's `Terminology` region does not model).
fn puzzle3d_labels(view_state: &ViewState) -> &'static Puzzle3dLabels {
    let terminology = view_state.terminology.as_deref().unwrap_or("native");
    let is_de = is_de_locale(view_state);
    match (terminology, is_de) {
        ("reuse", true) => &PUZZLE3D_LABELS_REUSE_DE,
        ("reuse", false) => &PUZZLE3D_LABELS_REUSE_EN,
        (_, true) => &PUZZLE3D_LABELS_NATIVE_DE,
        (_, false) => &PUZZLE3D_LABELS_NATIVE_EN,
    }
}
//#endregion 🔖️Terminology

//#region 🔖️Panels
fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, icon_id: Option<&str>, action: ActionDescriptor) -> UiTreeItemNode {
    UiTreeItemNode {
        presence: UiPresence::default(),
        id: id.into(),
        label: label.into(),
        description: None,
        icon_id: icon_id.map(IconName::from),
        default_open: None,
        action: Some(action),
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        dimmed: None,
        menu: None,
    }
}

fn puzzle3d_hide_lock_actions(hidden: bool, locked: bool, labels: &Puzzle3dLabels, flag_args: impl Fn(&str) -> Value) -> Vec<UiTreeItemAction> {
    vec![
        UiTreeItemAction { icon_id: if hidden { "eye-off".into() } else { "eye".into() }, label: Some(if hidden { labels.show.into() } else { labels.hide.into() }), action: puzzle3d_action("setSelectionFlag", Some(flag_args("hidden"))), reveal_on_hover: Some(true),
        },
        UiTreeItemAction { icon_id: if locked { "lock".into() } else { "lock-open".into() }, label: Some(if locked { labels.unlock.into() } else { labels.lock.into() }), action: puzzle3d_action("setSelectionFlag", Some(flag_args("locked"))), reveal_on_hover: Some(true),
        },
    ]
}

fn puzzle3d_document_selected_ids(selection: &Puzzle3dSelection) -> Vec<String> {
    let mut ids = Vec::new();
    for id in selection.object_ids.iter() {
        ids.push(format!("puzzle3d-object:{id}"));
    }
    for id in selection.reference_ids.iter() {
        ids.push(format!("puzzle3d-reference:{id}"));
    }
    for id in selection.target_volume_ids.iter() {
        ids.push(format!("puzzle3d-target-volume:{id}"));
    }
    for id in selection.vortex_ids.iter() {
        ids.push(format!("puzzle3d-vortex:{id}"));
    }
    for id in selection.attraction_ids.iter() {
        ids.push(format!("puzzle3d-attraction:{id}"));
    }
    ids
}

fn puzzle3d_chrome_action(action: &str) -> bool {
    matches!(
        action,
        "setHover" | "worldHover" | "worldPick" | "worldSelect" | "setSelection" | "clearSelection" | "selectAll" | "worldVortexHover" | "worldVortexSelect"
    )
}

fn puzzle3d_patch_chrome_effect(envelope: &Puzzle3dScene) -> HostEffect {
    HostEffect::PatchWorld3dChrome {
        selection_json: world_selection_json(envelope),
        vortices_json: Some(world_vortices_json(&envelope.fixture, &envelope.runtime)),
        document_selected_ids: puzzle3d_document_selected_ids(&envelope.runtime.selection),
        document_highlighted_ids: None,
    }
}

fn document_tree_sections(fixture: &Puzzle3dFixture, labels: &Puzzle3dLabels) -> Vec<UiTreeSectionNode> {
    let object_items: Vec<UiTreeItemNode> = fixture
        .objects
        .iter()
        .map(|object| {
            let vortex_items: Vec<UiTreeItemNode> = object
                .vortices
                .iter()
                .map(|vortex| {
                    let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                    tree_item_with_action(
                        format!("puzzle3d-vortex:{full_id}"),
                        vortex.vortex_kind.clone().unwrap_or_else(|| vortex.id.clone()),
                        Some("circle-dot"),
                        puzzle3d_action("setSelection", Some(json!({ "selection": { "objectIds": [], "vortexIds": [full_id], "attractionIds": [] } }))),
                    )
                })
                .collect();
            let flag_args = {
                let id = object.id.clone();
                move |flag: &str| json!({ "flag": flag, "value": true, "entity": "object", "ids": [id.clone()] })
            };
            UiTreeItemNode {
                presence: UiPresence::default(),
                id: format!("puzzle3d-object:{}", object.id),
                label: object.object_kind.clone().unwrap_or_else(|| object.id.clone()),
                description: None,
                icon_id: Some("box".into()),
                default_open: Some(false),
                action: Some(puzzle3d_action("setSelection", Some(json!({ "selection": { "objectIds": [object.id], "vortexIds": [], "attractionIds": [] } })))),
                hover_action: Some(puzzle3d_action("setHover", Some(json!({ "objectId": object.id })))),
                unhover_action: Some(puzzle3d_action("setHover", None)),
                actions: Some(puzzle3d_hide_lock_actions(object.hidden, object.locked, labels, flag_args)),
                draggable: None,
                drag_data: None,
                items: if vortex_items.is_empty() { None } else { Some(vortex_items) },
                control: None,
                dimmed: Some(object.hidden),
                menu: None,
            }
        })
        .collect();
    let reference_items: Vec<UiTreeItemNode> = fixture
        .references
        .iter()
        .map(|reference| {
            let flag_args = {
                let id = reference.id.clone();
                move |flag: &str| json!({ "flag": flag, "value": true, "entity": "reference", "ids": [id.clone()] })
            };
            UiTreeItemNode {
                presence: UiPresence::default(),
                id: format!("puzzle3d-reference:{}", reference.id),
                label: reference.id.clone(),
                description: Some(reference.source.url.clone()),
                icon_id: Some("globe".into()),
                default_open: None,
                action: Some(puzzle3d_action("setSelection", Some(json!({ "selection": { "objectIds": [], "vortexIds": [], "attractionIds": [], "referenceIds": [reference.id] } })))),
                hover_action: None,
                unhover_action: None,
                actions: Some(puzzle3d_hide_lock_actions(reference.hidden, reference.locked, labels, flag_args)),
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                dimmed: Some(reference.hidden),
                menu: None,
            }
        })
        .collect();
    let target_volume_items: Vec<UiTreeItemNode> = fixture
        .target_volumes
        .iter()
        .map(|volume| {
            let flag_args = {
                let id = volume.id.clone();
                move |flag: &str| json!({ "flag": flag, "value": true, "entity": "targetVolume", "ids": [id.clone()] })
            };
            UiTreeItemNode {
                presence: UiPresence::default(),
                id: format!("puzzle3d-target-volume:{}", volume.id),
                label: volume.id.clone(),
                description: None,
                icon_id: Some("cylinder".into()),
                default_open: None,
                action: Some(puzzle3d_action("setSelection", Some(json!({ "selection": { "objectIds": [], "vortexIds": [], "attractionIds": [], "targetVolumeIds": [volume.id] } })))),
                hover_action: None,
                unhover_action: None,
                actions: Some(puzzle3d_hide_lock_actions(volume.hidden, volume.locked, labels, flag_args)),
                draggable: None,
                drag_data: None,
                items: None,
                control: None,
                dimmed: Some(volume.hidden),
                menu: None,
            }
        })
        .collect();
    let attraction_items: Vec<UiTreeItemNode> = fixture
        .attractions
        .iter()
        .map(|attraction| {
            tree_item_with_action(
                format!("puzzle3d-attraction:{}", attraction.id),
                format!("{} → {}", attraction.attracting, attraction.attracted),
                Some("link"),
                puzzle3d_action("setSelection", Some(json!({ "selection": { "objectIds": [], "vortexIds": [], "attractionIds": [attraction.id] } }))),
            )
        })
        .collect();
    vec![
        UiTreeSectionNode { id: "puzzle3d-play-document.objects".into(), label: Some(labels.objects.into()), default_open: Some(true), presence: UiPresence::default(), items: object_items,
        },
        UiTreeSectionNode { id: "puzzle3d-play-document.references".into(), label: Some(labels.references.into()), default_open: Some(false), presence: UiPresence::default(), items: reference_items,
        },
        UiTreeSectionNode { id: "puzzle3d-play-document.target-volumes".into(), label: Some(labels.target_volumes.into()), default_open: Some(false), presence: UiPresence::default(), items: target_volume_items,
        },
        UiTreeSectionNode { id: "puzzle3d-play-document.attractions".into(), label: Some(labels.attractions.into()), default_open: Some(false), presence: UiPresence::default(), items: attraction_items,
        },
    ]
}

/// 🖱️ MIME key `DeclarativeTreePanel` (framework/renderer/react/ui-interpreter.tsx) reads to auto-wire catalogue drag sources.
const PUZZLE3D_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";

fn puzzle3d_catalog_entries<'a>(fixture: &'a Puzzle3dFixture, section: &str) -> &'a [Value] {
    fixture.meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get(section)).and_then(|entries| entries.as_array()).map(Vec::as_slice).unwrap_or(&[])
}

fn puzzle3d_catalog_entry_label(entry: &Value) -> String {
    entry.get("label").and_then(|value| value.as_str()).or_else(|| entry.get("name").and_then(|value| value.as_str())).or_else(|| entry.get("id").and_then(|value| value.as_str())).unwrap_or("kind").into()
}

fn puzzle3d_object_kind_vortex_items(entry: &Value) -> Vec<UiTreeItemNode> {
    entry
        .get("vortices")
        .and_then(|value| value.as_array())
        .map(|templates| {
            templates
                .iter()
                .enumerate()
                .map(|(index, template)| {
                    let vortex_kind = template.get("vortexKind").and_then(|value| value.as_str()).unwrap_or("vortex");
                    let position = template.get("position").cloned().unwrap_or(json!([0.0, 0.0, 0.0]));
                    UiTreeItemNode {
                        presence: UiPresence::default(),
                        id: format!("puzzle3d-kind-vortex.{index}.{vortex_kind}"),
                        label: vortex_kind.into(),
                        description: Some(position.to_string()),
                        icon_id: Some("circle-dot".into()),
                        default_open: None,
                        action: None,
                        hover_action: None,
                        unhover_action: None,
                        actions: None,
                        draggable: None,
                        drag_data: None,
                        items: None,
                        control: None,
                        dimmed: None,
                        menu: None,
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

fn puzzle3d_object_kind_item(entry: &Value) -> UiTreeItemNode {
    let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind").to_string();
    let mesh_url = entry.get("meshUrl").and_then(|value| value.as_str()).filter(|url| !url.is_empty()).map(str::to_string);
    let draggable = mesh_url.is_some();
    UiTreeItemNode {
        presence: UiPresence::default(),
        id: format!("puzzle3d-kind:{kind_id}"),
        label: puzzle3d_catalog_entry_label(entry),
        description: Some(kind_id.clone()),
        icon_id: Some("box".into()),
        default_open: Some(false),
        action: Some(puzzle3d_action("addObjectKind", Some(json!({ "objectKind": kind_id.clone() })))),
        hover_action: Some(puzzle3d_action("setKindHover", Some(json!({ "kindId": kind_id.clone() })))),
        unhover_action: Some(puzzle3d_action("setKindHover", Some(json!({ "kindId": Value::Null })))),
        actions: None,
        draggable: draggable.then_some(true),
        drag_data: draggable.then(|| {
            let mut payload = json!({ "objectKind": kind_id });
            if let Some(url) = mesh_url {
                payload["meshUrl"] = json!(url);
            }
            HashMap::from([(PUZZLE3D_CATALOGUE_DRAG_MIME.to_string(), payload.to_string())])
        }),
        items: Some(puzzle3d_object_kind_vortex_items(entry)),
        control: None,
        dimmed: None,
        menu: None,
    }
}

fn puzzle3d_catalog_kind_item(entry: &Value, icon_id: &str) -> UiTreeItemNode {
    let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind").to_string();
    UiTreeItemNode {
        presence: UiPresence::default(),
        id: format!("puzzle3d-kind-entry:{kind_id}"),
        label: puzzle3d_catalog_entry_label(entry),
        description: Some(kind_id),
        icon_id: Some(icon_id.into()),
        default_open: None,
        action: None,
        hover_action: None,
        unhover_action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        dimmed: None,
        menu: None,
    }
}

fn build_kinds_tree(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> UiNode {
    let object_entries = puzzle3d_catalog_entries(&envelope.fixture, "objects");
    let vortex_entries = puzzle3d_catalog_entries(&envelope.fixture, "vortices");
    let cable_entries = puzzle3d_catalog_entries(&envelope.fixture, "cables");
    let attraction_entries = puzzle3d_catalog_entries(&envelope.fixture, "attractions");
    UiNode::Tree(UiTreeNode {
        presence: UiPresence::default(),
        selected_ids: None,
        highlighted_ids: None,
        sections: vec![
            UiTreeSectionNode { id: "puzzle3d-play-kinds.objects".into(), label: Some(labels.objects.into()), default_open: Some(false), presence: UiPresence::default(), items: object_entries.iter().map(puzzle3d_object_kind_item).collect(),
        },
            UiTreeSectionNode { id: "puzzle3d-play-kinds.vortices".into(), label: Some(labels.vortices.into()), default_open: Some(false), presence: UiPresence::default(), items: vortex_entries.iter().map(|entry| puzzle3d_catalog_kind_item(entry, "circle-dot")).collect(),
        },
            UiTreeSectionNode { id: "puzzle3d-play-kinds.cables".into(), label: Some(labels.cables.into()), default_open: Some(false), presence: UiPresence::default(), items: cable_entries.iter().map(|entry| puzzle3d_catalog_kind_item(entry, "plug")).collect(),
        },
            UiTreeSectionNode { id: "puzzle3d-play-kinds.attractions".into(), label: Some(labels.attractions.into()), default_open: Some(false), presence: UiPresence::default(), items: attraction_entries.iter().map(|entry| puzzle3d_catalog_kind_item(entry, "link")).collect(),
        },
        ],
        selection_change: None,
        drop_action: None,
        menu: None,
    })
}

fn inspector_text_field(id: impl Into<String>, label: impl Into<String>, mixed_text: semio_framework_plugin::UiInspectorMixedText, action: ActionDescriptor) -> UiNode {
    let id = id.into();
    UiNode::Field(UiFieldNode {
        id: id.clone(),
        label: label.into(),
        child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
            id: format!("{id}.input"),
            input_kind: "text".into(),
            value: mixed_text.value,
            placeholder: mixed_text.placeholder,
            commit: None,
            on_change: action,
            min: None,
            max: None,
            step: None,
            accept: None,
            presence: UiPresence::default(),
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        menu: None,
    })
}

/// @emoji 🌀️ Builds an editable 4-component quaternion group (`W`/`X`/`Y`/`Z` steppers) — puzzle3d's
/// `orientation: Option<[f64; 4]>` fields have no shared helper (quaternions aren't `ui_inspector_vec3_group`'s
/// 3-wide shape), so this mirrors that helper's structure one component wider. `axis_action(component)`
/// builds the per-component action; the patch handler renormalizes after any component edit so the
/// result stays a valid unit quaternion.
fn inspector_quat_group(id: &str, label: &str, values: &[[f64; 4]], step: f64, axis_action: impl Fn(&str) -> ActionDescriptor) -> UiNode {
    let component = |index: usize, name: &str, label: &str| {
        let values: Vec<f64> = values.iter().map(|q| q[index]).collect();
        ui_inspector_stepper_field(format!("{id}.{name}"), label, &values, step, axis_action(name))
    };
    UiNode::Group(UiGroupNode {
        id: id.into(),
        label: label.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        children: vec![component(0, "x", "X"), component(1, "y", "Y"), component(2, "z", "Z"), component(3, "w", "W")],
        menu: None,
    })
}

fn inspector_header_and_delete(count: usize, noun: &str, labels: &Puzzle3dLabels) -> Vec<UiNode> {
    vec![
        ui_text(format!("{count} {noun} {}", labels.selected_count)),
        UiNode::Button(semio_framework_plugin::UiButtonNode { id: Some("puzzle3d-play-inspector.delete".into()), icon_id: "trash-2".into(), label: labels.delete.into(), action: puzzle3d_action("deleteSelection", None), style: None, presence: UiPresence::default(),
            menu: None,
        }),
    ]
}

fn puzzle3d_inspector_target_ids(entity: &str, selection: &Puzzle3dSelection) -> Vec<String> {
    match entity {
        "object" => selection.object_ids.to_vec(),
        "vortex" => selection.vortex_ids.to_vec(),
        "attraction" => selection.attraction_ids.to_vec(),
        "reference" => selection.reference_ids.to_vec(),
        "targetVolume" => selection.target_volume_ids.to_vec(),
        _ => Vec::new(),
    }
}

fn build_inspector_tree(envelope: &Puzzle3dScene, term_labels: &Puzzle3dLabels) -> UiNode {
    let selection = &envelope.runtime.selection;
    if !selection.object_ids.is_empty() {
        let objects: Vec<&Puzzle3dObject> = envelope.fixture.objects.iter().filter(|object| selection.object_ids.contains(&object.id)).collect();
        if !objects.is_empty() {
            let patch_cmd = |field: &str| puzzle3d_action("patchInspector", Some(json!({ "entity": "object", "field": field })));
            let mut fields = inspector_header_and_delete(objects.len(), term_labels.object, term_labels);
            if let [object] = objects.as_slice() {
                fields.push(ui_inspector_readonly_field("puzzle3d-play-inspector.object.id", term_labels.id, &object.id));
            }
            let labels: Vec<String> = objects.iter().map(|object| object.label.clone().unwrap_or_default()).collect();
            let kinds: Vec<String> = objects.iter().map(|object| object.object_kind.clone().unwrap_or_default()).collect();
            let origins: Vec<[f64; 3]> = objects.iter().map(|object| object.origin).collect();
            let orientations: Vec<[f64; 4]> = objects.iter().map(|object| object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])).collect();
            let scales: Vec<[f64; 3]> = objects.iter().map(|object| object_scale_json(object)).collect();
            let mesh_urls: Vec<String> = objects.iter().map(|object| object.mesh_url.clone().unwrap_or_default()).collect();
            let hidden: Vec<bool> = objects.iter().map(|object| object.hidden).collect();
            let locked: Vec<bool> = objects.iter().map(|object| object.locked).collect();
            fields.push(inspector_text_field("puzzle3d-play-inspector.object.label", term_labels.label, semio_framework_plugin::ui_inspector_mixed_text(&labels), patch_cmd("label")));
            fields.push(inspector_text_field("puzzle3d-play-inspector.object.kind", term_labels.kind, semio_framework_plugin::ui_inspector_mixed_text(&kinds), patch_cmd("objectKind")));
            fields.push(ui_inspector_vec3_group("puzzle3d-play-inspector.object.origin", term_labels.origin, &origins, 0.1, |axis| patch_cmd(&format!("origin.{axis}"))));
            fields.push(inspector_quat_group("puzzle3d-play-inspector.object.orientation", term_labels.orientation, &orientations, 0.01, |axis| patch_cmd(&format!("orientation.{axis}"))));
            fields.push(ui_inspector_vec3_group("puzzle3d-play-inspector.object.scale", term_labels.scale, &scales, 0.1, |axis| patch_cmd(&format!("scale.{axis}"))));
            fields.push(inspector_text_field("puzzle3d-play-inspector.object.mesh-url", term_labels.mesh_url, semio_framework_plugin::ui_inspector_mixed_text(&mesh_urls), patch_cmd("meshUrl")));
            fields.push(ui_inspector_toggle_field("puzzle3d-play-inspector.object.hidden", term_labels.hidden, "eye-off", &hidden, patch_cmd("hidden")));
            fields.push(ui_inspector_toggle_field("puzzle3d-play-inspector.object.locked", term_labels.locked, "lock", &locked, patch_cmd("locked")));
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.object".into(), label: term_labels.object.into(), default_open: None, presence: UiPresence::default(), fields }]);
        }
    }
    if !selection.vortex_ids.is_empty() {
        let vortices: Vec<(&Puzzle3dObject, &Puzzle3dVortex)> = envelope
            .fixture
            .objects
            .iter()
            .flat_map(|object| object.vortices.iter().map(move |vortex| (object, vortex)))
            .filter(|(object, vortex)| selection.vortex_ids.contains(&puzzle3d_vortex_full_id(&object.id, &vortex.id)))
            .collect();
        if !vortices.is_empty() {
            let full_ids: Vec<String> = vortices.iter().map(|(object, vortex)| puzzle3d_vortex_full_id(&object.id, &vortex.id)).collect();
            let patch_cmd = |field: &str| puzzle3d_action("patchInspector", Some(json!({ "entity": "vortex", "field": field })));
            let mut fields = inspector_header_and_delete(vortices.len(), term_labels.vortex, term_labels);
            if let [(_, vortex)] = vortices.as_slice() {
                fields.push(ui_inspector_readonly_field("puzzle3d-play-inspector.vortex.id", term_labels.full_id, &full_ids[0]));
                let _ = vortex;
            }
            let kinds: Vec<String> = vortices.iter().map(|(_, vortex)| vortex.vortex_kind.clone().unwrap_or_default()).collect();
            let positions: Vec<[f64; 3]> = vortices.iter().map(|(_, vortex)| vortex.position).collect();
            let directions: Vec<[f64; 3]> = vortices.iter().map(|(_, vortex)| vortex.direction.unwrap_or([0.0, 0.0, 1.0])).collect();
            let radii: Vec<f64> = vortices.iter().map(|(_, vortex)| vortex.radius.unwrap_or(0.35)).collect();
            let hidden: Vec<bool> = vortices.iter().map(|(_, vortex)| vortex.hidden).collect();
            let locked: Vec<bool> = vortices.iter().map(|(_, vortex)| vortex.locked).collect();
            fields.push(inspector_text_field("puzzle3d-play-inspector.vortex.kind", term_labels.vortex_kind, semio_framework_plugin::ui_inspector_mixed_text(&kinds), patch_cmd("vortexKind")));
            fields.push(ui_inspector_vec3_group("puzzle3d-play-inspector.vortex.position", term_labels.position, &positions, 0.1, |axis| patch_cmd(&format!("position.{axis}"))));
            fields.push(ui_inspector_vec3_group("puzzle3d-play-inspector.vortex.direction", term_labels.direction, &directions, 0.1, |axis| patch_cmd(&format!("direction.{axis}"))));
            fields.push(ui_inspector_stepper_field("puzzle3d-play-inspector.vortex.radius", term_labels.radius, &radii, 0.05, patch_cmd("radius")));
            fields.push(ui_inspector_toggle_field("puzzle3d-play-inspector.vortex.hidden", term_labels.hidden, "eye-off", &hidden, patch_cmd("hidden")));
            fields.push(ui_inspector_toggle_field("puzzle3d-play-inspector.vortex.locked", term_labels.locked, "lock", &locked, patch_cmd("locked")));
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.vortex".into(), label: term_labels.vortex.into(), default_open: None, presence: UiPresence::default(), fields }]);
        }
    }
    if !selection.attraction_ids.is_empty() {
        let attractions: Vec<&Puzzle3dAttraction> = envelope.fixture.attractions.iter().filter(|attraction| selection.attraction_ids.contains(&attraction.id)).collect();
        if !attractions.is_empty() {
            let patch_cmd = |field: &str| puzzle3d_action("patchInspector", Some(json!({ "entity": "attraction", "field": field })));
            let mut fields = inspector_header_and_delete(attractions.len(), term_labels.attraction, term_labels);
            let attracting: Vec<String> = attractions.iter().map(|attraction| attraction.attracting.clone()).collect();
            let attracted: Vec<String> = attractions.iter().map(|attraction| attraction.attracted.clone()).collect();
            fields.push(inspector_text_field("puzzle3d-play-inspector.attraction.attracting", term_labels.attracting, semio_framework_plugin::ui_inspector_mixed_text(&attracting), patch_cmd("attracting")));
            fields.push(inspector_text_field("puzzle3d-play-inspector.attraction.attracted", term_labels.attracted, semio_framework_plugin::ui_inspector_mixed_text(&attracted), patch_cmd("attracted")));
            let gaps: Vec<f64> = attractions.iter().map(|attraction| attraction.gap).collect();
            let shifts: Vec<f64> = attractions.iter().map(|attraction| attraction.shift).collect();
            let rises: Vec<f64> = attractions.iter().map(|attraction| attraction.rise).collect();
            let rotations: Vec<f64> = attractions.iter().map(|attraction| attraction.rotation).collect();
            let turns: Vec<f64> = attractions.iter().map(|attraction| attraction.turn).collect();
            let tilts: Vec<f64> = attractions.iter().map(|attraction| attraction.tilt).collect();
            fields.push(ui_inspector_stepper_field("puzzle3d-play-inspector.attraction.gap", term_labels.gap, &gaps, 0.1, patch_cmd("gap")));
            fields.push(ui_inspector_stepper_field("puzzle3d-play-inspector.attraction.shift", term_labels.shift, &shifts, 0.1, patch_cmd("shift")));
            fields.push(ui_inspector_stepper_field("puzzle3d-play-inspector.attraction.rise", term_labels.rise, &rises, 0.1, patch_cmd("rise")));
            fields.push(ui_inspector_stepper_field("puzzle3d-play-inspector.attraction.rotation", term_labels.rotation_deg, &rotations, 1.0, patch_cmd("rotation")));
            fields.push(ui_inspector_stepper_field("puzzle3d-play-inspector.attraction.turn", term_labels.turn_deg, &turns, 1.0, patch_cmd("turn")));
            fields.push(ui_inspector_stepper_field("puzzle3d-play-inspector.attraction.tilt", term_labels.tilt_deg, &tilts, 1.0, patch_cmd("tilt")));
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.attraction".into(), label: term_labels.attraction.into(), default_open: None, presence: UiPresence::default(), fields }]);
        }
    }
    if !selection.reference_ids.is_empty() {
        let references: Vec<&Puzzle3dReference> = envelope.fixture.references.iter().filter(|reference| selection.reference_ids.contains(&reference.id)).collect();
        if !references.is_empty() {
            let patch_cmd = |field: &str| puzzle3d_action("patchInspector", Some(json!({ "entity": "reference", "field": field })));
            let mut fields = inspector_header_and_delete(references.len(), term_labels.reference, term_labels);
            let urls: Vec<String> = references.iter().map(|reference| reference.source.url.clone()).collect();
            let media_kinds: Vec<String> = references.iter().map(|reference| reference.source.media_kind.clone().unwrap_or_default()).collect();
            let origins: Vec<[f64; 3]> = references.iter().map(|reference| reference.origin).collect();
            let widths: Vec<f64> = references.iter().map(|reference| reference.width_world).collect();
            let hidden: Vec<bool> = references.iter().map(|reference| reference.hidden).collect();
            let locked: Vec<bool> = references.iter().map(|reference| reference.locked).collect();
            fields.push(inspector_text_field("puzzle3d-play-inspector.reference.url", term_labels.source_url, semio_framework_plugin::ui_inspector_mixed_text(&urls), patch_cmd("sourceUrl")));
            fields.push(inspector_text_field("puzzle3d-play-inspector.reference.media-kind", term_labels.media_kind, semio_framework_plugin::ui_inspector_mixed_text(&media_kinds), patch_cmd("mediaKind")));
            fields.push(ui_inspector_vec3_group("puzzle3d-play-inspector.reference.origin", term_labels.origin, &origins, 0.1, |axis| patch_cmd(&format!("origin.{axis}"))));
            fields.push(ui_inspector_stepper_field("puzzle3d-play-inspector.reference.width", term_labels.width, &widths, 0.1, patch_cmd("widthWorld")));
            fields.push(ui_inspector_toggle_field("puzzle3d-play-inspector.reference.hidden", term_labels.hidden, "eye-off", &hidden, patch_cmd("hidden")));
            fields.push(ui_inspector_toggle_field("puzzle3d-play-inspector.reference.locked", term_labels.locked, "lock", &locked, patch_cmd("locked")));
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.reference".into(), label: term_labels.reference.into(), default_open: None, presence: UiPresence::default(), fields }]);
        }
    }
    if !selection.target_volume_ids.is_empty() {
        let volumes: Vec<&Puzzle3dTargetVolume> = envelope.fixture.target_volumes.iter().filter(|volume| selection.target_volume_ids.contains(&volume.id)).collect();
        if !volumes.is_empty() {
            let patch_cmd = |field: &str| puzzle3d_action("patchInspector", Some(json!({ "entity": "targetVolume", "field": field })));
            let mut fields = inspector_header_and_delete(volumes.len(), term_labels.target_volume, term_labels);
            let origins: Vec<[f64; 3]> = volumes.iter().map(|volume| volume.origin).collect();
            let orientations: Vec<[f64; 4]> = volumes.iter().map(|volume| volume.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])).collect();
            let scales: Vec<[f64; 3]> = volumes.iter().map(|volume| target_volume_scale_json(volume)).collect();
            let hidden: Vec<bool> = volumes.iter().map(|volume| volume.hidden).collect();
            let locked: Vec<bool> = volumes.iter().map(|volume| volume.locked).collect();
            fields.push(ui_inspector_vec3_group("puzzle3d-play-inspector.target-volume.origin", term_labels.origin, &origins, 0.1, |axis| patch_cmd(&format!("origin.{axis}"))));
            fields.push(inspector_quat_group("puzzle3d-play-inspector.target-volume.orientation", term_labels.orientation, &orientations, 0.01, |axis| patch_cmd(&format!("orientation.{axis}"))));
            fields.push(ui_inspector_vec3_group("puzzle3d-play-inspector.target-volume.scale", term_labels.scale, &scales, 0.1, |axis| patch_cmd(&format!("scale.{axis}"))));
            fields.push(ui_inspector_toggle_field("puzzle3d-play-inspector.target-volume.hidden", term_labels.hidden, "eye-off", &hidden, patch_cmd("hidden")));
            fields.push(ui_inspector_toggle_field("puzzle3d-play-inspector.target-volume.locked", term_labels.locked, "lock", &locked, patch_cmd("locked")));
            return ui_inspector_groups_to_tree(&[UiInspectorFieldGroup { id: "puzzle3d-play-inspector.target-volume".into(), label: term_labels.target_volume.into(), default_open: None, presence: UiPresence::default(), fields }]);
        }
    }
    ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
            id: "puzzle3d-play-inspector.empty".into(),
            label: Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()),
            default_open: Some(true),
            children: vec![
                ui_text(format!("{}: {}", term_labels.schema, envelope.fixture.schema)),
        ui_text(format!("{}: {}", term_labels.domain, envelope.fixture.domain)),
        ui_text(format!("{}: {}", term_labels.objects, envelope.fixture.objects.len())),
            ],
            presence: UiPresence::default(),
            menu: None,
        }])
}

fn build_settings_body(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> UiNode {
    let runtime = &envelope.runtime;
    let selection_mode_field = UiNode::Field(UiFieldNode {
        id: "puzzle3d-play-settings.selection-mode".into(),
        label: labels.selection_mode.into(),
        child: Box::new(UiNode::Select(semio_framework_plugin::UiSelectNode {
            id: "puzzle3d-play-settings.selection-mode.input".into(),
            value: runtime.selection_mode_default.clone(),
            items: vec![
                semio_framework_plugin::UiSelectItem { value: "default".into(), label: labels.selective.into(),
        },
                semio_framework_plugin::UiSelectItem { value: "additive".into(), label: labels.additive.into(),
        },
                semio_framework_plugin::UiSelectItem { value: "subtractive".into(), label: labels.subtractive.into(),
        },
                semio_framework_plugin::UiSelectItem { value: "invertive".into(), label: labels.invertive.into(),
        },
            ],
            placeholder: None,
            on_change: puzzle3d_action("setSelectionModeDefault", None),
            presence: UiPresence::default(),
            menu: None,
        })),
        description: None,
        required: None,
        error: None,
        presence: UiPresence::default(),
        menu: None,
    });
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "puzzle3d-play-settings".into(),
        label: labels.settings.into(),
        default_open: Some(true),
        presence: UiPresence::default(),
        fields: vec![
            selection_mode_field,
            ui_inspector_stepper_field("puzzle3d-play-settings.overlap-budget", labels.overlap_budget, &[runtime.overlap_budget], 0.05, puzzle3d_action("setBrushPlacementOverlapBudget", None)),
            ui_inspector_stepper_field("puzzle3d-play-settings.proximity-radius", labels.proximity_radius, &[runtime.proximity_radius], 0.1, puzzle3d_action("setProximityRadius", None)),
            ui_inspector_stepper_field("puzzle3d-play-settings.chunk-size", labels.chunk_size, &[runtime.chunk_size], 1.0, puzzle3d_action("setChunkSize", None)),
            ui_inspector_stepper_field("puzzle3d-play-settings.grid-spacing", labels.spacing, &[runtime.grid_spacing], 0.5, puzzle3d_action("setGridSpacing", None)),
        ],
    }])
}
//#endregion 🔖️Panels


//#region 🔖️Engagement
fn puzzle3d_brush_target_vortex(envelope: &Puzzle3dScene) -> Option<String> {
    envelope
        .runtime
        .selection
        .vortex_ids
        .first()
        .map(str::to_string)
        .or_else(|| envelope.runtime.hovered_vortex_full_id.clone())
        .or_else(|| {
            let object_id = envelope.runtime.hovered_object_id.as_deref()?;
            let object = envelope.fixture.objects.iter().find(|entry| entry.id == object_id)?;
            let vortex = object.vortices.first()?;
            Some(puzzle3d_vortex_full_id(&object.id, &vortex.id))
        })
}

/// 🧰️ The select/brush/fill switcher lives in the framework utility bar (declared via `.utility` +
/// `.window_kind_utilities`); the fill-count slider, voxel edit-mode picker, voxel-dimension steppers and
/// brush placement picker now live as tagged [`WindowMeasure::Group`]s in [`puzzle3d_window_measures`]
/// (surfaced by [`partition_window_measures`] in the dedicated "Utility Options" rail only while their
/// utility is active), so the engagement HUD is a bare command input plus a status line.
fn puzzle3d_engagement(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> WindowEngagement {
    let object_count = envelope.fixture.objects.len();
    let attraction_count = envelope.fixture.attractions.len();
    let active_utility = envelope.active_utility.as_str();
    let objects_label = labels.objects;
    let attractions_label = labels.attractions;
    WindowEngagement {
        session_active: Some(puzzle3d_engagement_session_active(active_utility)),
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("puzzle3d-engagement".into()),
            value: Some(envelope.runtime.engagement_input.clone()),
            placeholder: Some("brush, fill <n>, zoom, clear, rectangle, lasso".into()),
            disabled: None,
            on_change: Some(puzzle3d_action("engagementInput", None)),
            on_submit: Some(puzzle3d_action("engagementSubmit", None)),
            on_repeat_last: Some(puzzle3d_action("engagementRepeatLast", None)),
            on_abort: Some(puzzle3d_action("engagementAbort", None)),
        }),
        control: None,
        controls: None,
        status: Some(vec![semio_framework_plugin::WindowEngagementStatus { id: "puzzle3d-world-status".into(), text: format!("{object_count} {objects_label} · {attraction_count} {attractions_label}") }]),
        possible_engagements: None,
    }
}

fn puzzle3d_context_menu_json(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> Option<String> {
    let selection = &envelope.runtime.selection;
    if !selection.object_ids.is_empty() {
        let all_hidden = envelope.fixture.objects.iter().filter(|object| selection.object_ids.contains(&object.id)).all(|object| object.hidden);
        let all_locked = envelope.fixture.objects.iter().filter(|object| selection.object_ids.contains(&object.id)).all(|object| object.locked);
        let items = vec![
            json!({ "id": "duplicate", "label": labels.duplicate, "icon": "copy", "action": "duplicateSelection" }),
            json!({ "id": "select-same-kind", "label": labels.select_same_kind, "icon": "layers", "action": "selectSameKindSelection" }),
            json!({ "id": "sep-flags", "separator": true }),
            json!({
                "id": "hide-show",
                "label": if all_hidden { labels.show } else { labels.hide },
                "icon": if all_hidden { "eye" } else { "eye-off" },
                "action": "setSelectionFlag",
                "args": { "flag": "hidden", "value": !all_hidden },
            }),
            json!({
                "id": "lock-unlock",
                "label": if all_locked { labels.unlock } else { labels.lock },
                "icon": if all_locked { "lock-open" } else { "lock" },
                "action": "setSelectionFlag",
                "args": { "flag": "locked", "value": !all_locked },
            }),
            json!({ "id": "sep-zoom", "separator": true }),
            json!({ "id": "zoom", "label": labels.zoom_to_selection, "icon": "crosshair", "action": "zoomToSelection" }),
            json!({ "id": "sep-delete", "separator": true }),
            json!({ "id": "delete", "label": labels.delete, "icon": "trash", "action": "deleteSelection", "destructive": true }),
        ];
        return serde_json::to_string(&items).ok();
    }
    if !selection.vortex_ids.is_empty() {
        let mut items = Vec::new();
        if let [only] = selection.vortex_ids.as_slice() {
            items.push(json!({
                "id": "suggest",
                "label": labels.suggest_objects,
                "icon": "sparkles",
                "action": "openVortexSuggestions",
                "args": { "fullId": only },
            }));
            items.push(json!({ "id": "sep-suggest", "separator": true }));
        }
        items.push(json!({ "id": "zoom", "label": labels.zoom_to_selection, "icon": "crosshair", "action": "zoomToSelection" }));
        items.push(json!({ "id": "sep-delete", "separator": true }));
        items.push(json!({ "id": "delete", "label": labels.delete, "icon": "trash", "action": "deleteSelection", "destructive": true }));
        return serde_json::to_string(&items).ok();
    }
    if let Some(id) = selection.attraction_ids.first() {
        let items = vec![json!({
            "id": "delete",
            "label": labels.delete,
            "icon": "trash",
            "action": "deleteAttraction",
            "args": { "id": id },
            "destructive": true,
        })];
        return serde_json::to_string(&items).ok();
    }
    if let Some(id) = selection.target_volume_ids.first() {
        let target_volume = envelope.fixture.target_volumes.iter().find(|volume| &volume.id == id);
        let hidden = target_volume.map(|volume| volume.hidden).unwrap_or(false);
        let locked = target_volume.map(|volume| volume.locked).unwrap_or(false);
        let items = vec![
            json!({
                "id": "hide-show",
                "label": if hidden { labels.show } else { labels.hide },
                "icon": if hidden { "eye" } else { "eye-off" },
                "action": "setTargetVolumeFlag",
                "args": { "id": id, "flag": "hidden", "value": !hidden },
            }),
            json!({
                "id": "lock-unlock",
                "label": if locked { labels.unlock } else { labels.lock },
                "icon": if locked { "lock-open" } else { "lock" },
                "action": "setTargetVolumeFlag",
                "args": { "id": id, "flag": "locked", "value": !locked },
            }),
            json!({ "id": "sep-delete", "separator": true }),
            json!({
                "id": "delete",
                "label": labels.delete,
                "icon": "trash",
                "action": "deleteTargetVolume",
                "args": { "id": id },
                "destructive": true,
            }),
        ];
        return serde_json::to_string(&items).ok();
    }
    if let Some(_id) = selection.reference_ids.first() {
        let items = vec![
            json!({ "id": "zoom", "label": labels.zoom_to_selection, "icon": "crosshair", "action": "zoomToSelection" }),
            json!({ "id": "sep-delete", "separator": true }),
            json!({
                "id": "delete",
                "label": labels.delete,
                "icon": "trash",
                "action": "deleteSelection",
                "destructive": true,
            }),
        ];
        return serde_json::to_string(&items).ok();
    }
    None
}
//#endregion 🔖️Engagement

//#region 🔖️Measures
const PUZZLE3D_LOD_SLIDER_MIN: f64 = 0.0;
const PUZZLE3D_LOD_SLIDER_MAX: f64 = 1000.0;

fn puzzle3d_kind_ids(fixture: &Puzzle3dFixture, section: &str) -> Vec<String> {
    fixture
        .meta
        .kind_catalogs
        .as_ref()
        .and_then(|catalogs| catalogs.get(section))
        .and_then(|entries| entries.as_array())
        .map(|entries| entries.iter().filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect())
        .unwrap_or_default()
}

fn puzzle3d_uniform_kind_weights(ids: &[String]) -> HashMap<String, f64> {
    if ids.is_empty() {
        return HashMap::new();
    }
    let weight = 1.0 / ids.len() as f64;
    ids.iter().map(|id| (id.clone(), weight)).collect()
}

fn puzzle3d_normalize_kind_weight_group(weights: &HashMap<String, f64>, kind_ids: &[String], changed_id: &str, new_value: f64) -> HashMap<String, f64> {
    if kind_ids.is_empty() {
        return HashMap::new();
    }
    if kind_ids.len() == 1 {
        return HashMap::from([(kind_ids[0].clone(), 1.0)]);
    }
    let new_value = new_value.clamp(0.0, 1.0);
    let others: Vec<&String> = kind_ids.iter().filter(|id| id.as_str() != changed_id).collect();
    let remainder = (1.0 - new_value).max(0.0);
    let other_sum: f64 = others.iter().map(|id| weights.get(*id).copied().unwrap_or(0.0)).sum();
    let mut next = HashMap::new();
    next.insert(changed_id.to_string(), new_value);
    if remainder <= f64::EPSILON {
        for id in others {
            next.insert((*id).clone(), 0.0);
        }
        return next;
    }
    if other_sum <= f64::EPSILON {
        let each = remainder / others.len() as f64;
        for id in others {
            next.insert((*id).clone(), each);
        }
    } else {
        for id in others {
            let old = weights.get(id).copied().unwrap_or(0.0);
            next.insert((*id).clone(), old / other_sum * remainder);
        }
    }
    next
}

fn puzzle3d_ensure_catalog_kind_weights(weights: &mut HashMap<String, f64>, kind_ids: &[String]) {
    if kind_ids.is_empty() {
        return;
    }
    if weights.is_empty() || kind_ids.iter().any(|id| !weights.contains_key(id)) {
        *weights = puzzle3d_uniform_kind_weights(kind_ids);
        return;
    }
    let sum: f64 = kind_ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum();
    if (sum - 1.0).abs() > 0.001 {
        for id in kind_ids {
            if let Some(weight) = weights.get_mut(id) {
                *weight /= sum;
            }
        }
    }
}

fn puzzle3d_lod_measures_group(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod"),
        label: labels.lod.into(),
        default_open: Some(true),
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-auto"), icon_id: "zoom-in".into(), label: Some(labels.auto_zoom.into()), pressed: runtime.lod_automatic, text: None, on_change: puzzle3d_action("setLodAutomatic", None) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-depth-variable"), icon_id: "lod-depth".into(), label: Some(labels.depth_variable.into()), pressed: runtime.lod_depth_variable, text: None, on_change: puzzle3d_action("setLodDepthVariable", None) },
            WindowMeasure::Slider { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-lod-value"), label: Some(format!("{} {:.0}", labels.lod, runtime.lod_manual)), value: runtime.lod_manual, min: PUZZLE3D_LOD_SLIDER_MIN, max: PUZZLE3D_LOD_SLIDER_MAX, step: Some(1.0), ready: None, loading: None, waiting: None, disabled: None, reveal: None, on_change: puzzle3d_action("setLodManual", None) },
        ],
    }
}

fn puzzle3d_grid_measures_group(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid"),
        label: labels.grid.into(),
        default_open: Some(true),
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible"), icon_id: "layout-grid".into(), label: Some(labels.visible.into()), pressed: runtime.grid_visible, text: None, on_change: puzzle3d_action("setGridVisible", None) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-snap"), icon_id: "magnet".into(), label: Some(labels.snap.into()), pressed: runtime.grid_snap_enabled, text: None, on_change: puzzle3d_action("setGridSnapEnabled", None) },
            WindowMeasure::Slider { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-spacing"), label: Some(format!("{} {:.1}", labels.spacing, runtime.grid_spacing)), value: runtime.grid_spacing, min: 0.5, max: 50.0, step: Some(0.5), ready: None, loading: None, waiting: None, disabled: None, reveal: None, on_change: puzzle3d_action("setGridSpacing", None) },
        ],
    }
}

fn puzzle3d_select_measures_group(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select"),
        label: labels.select.into(),
        default_open: Some(true),
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-rectangle"), icon_id: "rectangle-tool".into(), label: Some(labels.rectangle.into()), pressed: runtime.selection_method == "rectangle", text: None, on_change: puzzle3d_action("setSelectionMethod", Some(json!({ "method": "rectangle" }))) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-lasso"), icon_id: "lasso".into(), label: Some(labels.lasso.into()), pressed: runtime.selection_method == "lasso", text: None, on_change: puzzle3d_action("setSelectionMethod", Some(json!({ "method": "lasso" }))) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-mode-default"), icon_id: "mouse-pointer".into(), label: Some(labels.selective.into()), pressed: runtime.selection_mode_default == "default", text: None, on_change: puzzle3d_action("setSelectionModeDefault", Some(json!({ "mode": "default" }))) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-mode-additive"), icon_id: "plus".into(), label: Some(labels.additive.into()), pressed: runtime.selection_mode_default == "additive", text: None, on_change: puzzle3d_action("setSelectionModeDefault", Some(json!({ "mode": "additive" }))) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-mode-subtractive"), icon_id: "minus".into(), label: Some(labels.subtractive.into()), pressed: runtime.selection_mode_default == "subtractive", text: None, on_change: puzzle3d_action("setSelectionModeDefault", Some(json!({ "mode": "subtractive" }))) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-mode-invertive"), icon_id: "rotate-ccw".into(), label: Some(labels.invertive.into()), pressed: runtime.selection_mode_default == "invertive", text: None, on_change: puzzle3d_action("setSelectionModeDefault", Some(json!({ "mode": "invertive" }))) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-objects"), icon_id: "box".into(), label: Some(labels.objects.into()), pressed: runtime.selectable_kinds.objects, text: None, on_change: puzzle3d_action("setSelectableKind", Some(json!({ "kind": "objects" }))) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-vortices"), icon_id: "circle-dot".into(), label: Some(labels.vortices.into()), pressed: runtime.selectable_kinds.vortices, text: None, on_change: puzzle3d_action("setSelectableKind", Some(json!({ "kind": "vortices" }))) },
            WindowMeasure::Toggle { id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-select-attractions"), icon_id: "link".into(), label: Some(labels.attractions.into()), pressed: runtime.selectable_kinds.attractions, text: None, on_change: puzzle3d_action("setSelectableKind", Some(json!({ "kind": "attractions" }))) },
        ],
    }
}

fn puzzle3d_object_kind_catalog_entry<'a>(fixture: &'a Puzzle3dFixture, object_kind_id: &str) -> Option<&'a Value> {
    fixture
        .meta
        .kind_catalogs
        .as_ref()
        .and_then(|catalogs| catalogs.get("objects"))
        .and_then(|entries| entries.as_array())
        .and_then(|entries| entries.iter().find(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(object_kind_id)))
}

fn puzzle3d_object_kind_label(fixture: &Puzzle3dFixture, object_kind_id: &str) -> String {
    puzzle3d_object_kind_catalog_entry(fixture, object_kind_id)
        .and_then(|entry| entry.get("label").and_then(|value| value.as_str()).or_else(|| entry.get("name").and_then(|value| value.as_str())))
        .unwrap_or(object_kind_id)
        .to_string()
}

fn puzzle3d_joint_vortex_weight(object_weight: f64, vortex_weight: f64) -> f64 {
    object_weight * vortex_weight
}

/// 🎲️ Vortex-kind sliders under an object row — displayed value is the **final** joint percentage
/// `P(object) × P(vortex)`. Every **global** vortex kind is listed under each object so the sum of
/// all nested joint percentages across the tree is 1 (not a local simplex per object). Editing
/// converts back to relative `P(vortex)` on the shared vortex simplex. Disabled when the parent
/// object weight is 0. Step tracks ~1% of the object weight for a smooth `[0, P(object)]` range.
fn puzzle3d_joint_vortex_measures(object_kind_id: &str, object_weight: f64, vortex_kind_ids: &[String], vortex_weights: &HashMap<String, f64>) -> Vec<WindowMeasure> {
    let object_kind_zero = object_weight <= f64::EPSILON;
    let joint_max = if object_kind_zero { 1.0 } else { object_weight };
    let joint_step = if object_kind_zero { 0.01 } else { (object_weight * 0.01).max(0.0001) };
    let fallback = if vortex_kind_ids.is_empty() { 0.0 } else { 1.0 / vortex_kind_ids.len() as f64 };
    vortex_kind_ids
        .iter()
        .map(|vortex_kind_id| {
            let vortex_weight = vortex_weights.get(vortex_kind_id).copied().unwrap_or(fallback);
            let joint = puzzle3d_joint_vortex_weight(object_weight, vortex_weight);
            WindowMeasure::Slider {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-joint-vortex-{object_kind_id}-{vortex_kind_id}"),
                label: Some(vortex_kind_id.clone()),
                value: joint,
                min: 0.0,
                max: joint_max,
                step: Some(joint_step),
                ready: None,
                loading: None,
                waiting: None,
                disabled: if object_kind_zero { Some(true) } else { None },
                                    reveal: None,
on_change: puzzle3d_action("setVortexKindWeight", Some(json!({ "kindId": vortex_kind_id, "objectKindId": object_kind_id }))),
            }
        })
        .collect()
}

/// 🎲️ Nested object/vortex distribution — one group per object kind (header slider = P(object)),
/// vortex children are the **global** vortex catalog shown as joint P(object)×P(vortex). Moving an
/// object header scales its children; the sum of every nested joint across all objects is 1.
/// Shared by fill tool and brush utility options.
fn puzzle3d_distribution_children(envelope: &Puzzle3dScene, _labels: &Puzzle3dLabels, default_open: Option<bool>) -> Vec<WindowMeasure> {
    let object_ids = puzzle3d_kind_ids(&envelope.fixture, "objects");
    let vortex_kind_ids = puzzle3d_kind_ids(&envelope.fixture, "vortices");
    object_ids
        .iter()
        .map(|object_kind_id| {
            let object_weight = envelope.runtime.object_kind_weights.get(object_kind_id).copied().unwrap_or_else(|| if object_ids.is_empty() { 0.0 } else { 1.0 / object_ids.len() as f64 });
            let label = puzzle3d_object_kind_label(&envelope.fixture, object_kind_id);
            WindowMeasure::Group {
                id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution-object-{object_kind_id}"),
                label,
                default_open,
                active_utility_id: None,
                value: Some(object_weight),
                min: Some(0.0),
                max: Some(1.0),
                step: Some(0.01),
                ready: None,
                loading: None,
                waiting: None,
                on_change: Some(puzzle3d_action("setObjectKindWeight", Some(json!({ "kindId": object_kind_id })))),
                children: puzzle3d_joint_vortex_measures(object_kind_id, object_weight, &vortex_kind_ids, &envelope.runtime.vortex_kind_weights),
            }
        })
        .collect()
}

fn puzzle3d_distribution_group(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels, default_open: Option<bool>) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-distribution"),
        label: labels.distribution.into(),
        default_open,
        active_utility_id: None,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: puzzle3d_distribution_children(envelope, labels, Some(false)),
    }
}

/// 🌀️ Window option for when vortex markers are emitted — Always (every object) or Selected (hovered/selected only).
fn puzzle3d_vortex_show_measure(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Select {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-show"),
        label: Some(labels.vortex_show.into()),
        value: runtime.vortex_show.clone(),
        items: vec![
            MeasureSelectItem { id: PUZZLE3D_VORTEX_SHOW_ALWAYS.into(), value: PUZZLE3D_VORTEX_SHOW_ALWAYS.into(), label: labels.always.into() },
            MeasureSelectItem { id: PUZZLE3D_VORTEX_SHOW_SELECTED.into(), value: PUZZLE3D_VORTEX_SHOW_SELECTED.into(), label: labels.selected.into() },
        ],
        on_change: puzzle3d_action("setVortexShow", None),
    }
}

/// 🧭️ Window option for how vortex direction arrows are drawn — Outwards (tip away from point) or Inwards (tip on point).
fn puzzle3d_vortex_direction_measure(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Select {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-vortex-direction"),
        label: Some(labels.vortex_direction.into()),
        value: runtime.vortex_direction.clone(),
        items: vec![
            MeasureSelectItem { id: PUZZLE3D_VORTEX_DIRECTION_OUTWARDS.into(), value: PUZZLE3D_VORTEX_DIRECTION_OUTWARDS.into(), label: labels.outwards.into() },
            MeasureSelectItem { id: PUZZLE3D_VORTEX_DIRECTION_INWARDS.into(), value: PUZZLE3D_VORTEX_DIRECTION_INWARDS.into(), label: labels.inwards.into() },
        ],
        on_change: puzzle3d_action("setVortexDirection", None),
    }
}

/// 🪣️ Fill-count slider measure — the fill-utility's core parameter, mirrors the retired
/// `puzzle3d_fill_count_control` (`setFillCount` reads `count`-or-`value`, so a slider's `{value}` payload
/// preserves the action semantics). The label stays fixed; preload progress is the ready extent + loading ring.
/// The slider range stays fixed at [`PUZZLE3D_FILL_COUNT_MAX`]; `ready` tracks how far planning has preloaded.
fn puzzle3d_fill_count_measure(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels) -> WindowMeasure {
    let progress = precompute.fill_progress_summary();
    let done = progress.done;
    let available_count = progress.count;
    WindowMeasure::Slider {
        id: "puzzle3d-fill-count".into(),
        label: Some(labels.count.into()),
        value: envelope.runtime.fill_count.min(PUZZLE3D_FILL_COUNT_MAX) as f64,
        min: 0.0,
        max: PUZZLE3D_FILL_COUNT_MAX as f64,
        step: Some(1.0),
        ready: Some(available_count as f64),
        loading: if done { None } else { Some(true) }, waiting: None,
        disabled: None,
        // 🪣️ Live drag reveals/hides already-planned pieces client-side (see `WorldInstancesLayer`'s
        // reveal cutoff store); only the committed value on gesture release round-trips through here.
        reveal: Some("puzzle3d-fill".into()),
        on_change: puzzle3d_action("setFillCount", None),
    }
}

/// 🧊️ Voxel width/depth/height measures for the Volume Brush utility.
fn puzzle3d_voxel_dim_measures(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> Vec<WindowMeasure> {
    let [w, d, h] = runtime.voxel_dims;
    let axis_slider = |axis: &str, label: &str, value: u32| WindowMeasure::Slider {
        id: format!("puzzle3d-voxel-{axis}"),
        label: Some(format!("{} {label} {value}", labels.voxel)),
        value: value as f64,
        min: 1.0,
        max: 64.0,
        step: Some(1.0),
        ready: None,
        loading: None, waiting: None,
        disabled: None,
        reveal: None,
        on_change: puzzle3d_action("setVoxelDims", Some(json!({ "axis": axis }))),
    };
    vec![axis_slider("w", labels.width, w), axis_slider("d", labels.depth, d), axis_slider("h", labels.height, h)]
}

/// 🛠️ Fill tool measures — count slider and nested distribution tree.
fn puzzle3d_fill_tool_measures(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels) -> Vec<WindowMeasure> {
    vec![puzzle3d_fill_count_measure(envelope, precompute, labels), puzzle3d_distribution_group(envelope, labels, Some(true))]
}

/// 🧊️ Utility Options for the Volume Brush utility — voxel width/depth/height sliders for Alt+click painting.
fn puzzle3d_volume_brush_utility_options(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-volume-brush"),
        label: labels.volume_brush.into(),
        default_open: Some(true),
        active_utility_id: Some("volumeBrush".into()),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: puzzle3d_voxel_dim_measures(runtime, labels),
    }
}

/// 🖌️ Utility Options for the Brush utility — overlap budget, distribution trees, and (when
/// candidates exist) the placement picker. Tagged `Some("brush")` as a routing envelope only;
/// `partition_window_measures` unwraps the children so the utility bar shows the option tree directly
/// (no nested "Brush"/"Pinsel" header — the utility toggle already owns that row).
fn puzzle3d_brush_utility_options(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels) -> WindowMeasure {
    let mut children = vec![
        WindowMeasure::Slider {
            id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-brush-overlap-budget"),
            label: Some(labels.overlap_budget.into()),
            value: envelope.runtime.overlap_budget,
            min: 0.0,
            max: 1.0,
            step: Some(0.01),
            ready: None,
            loading: None,
            waiting: None,
            disabled: None,
            reveal: None,
            on_change: puzzle3d_action("setBrushPlacementOverlapBudget", None),
        },
        puzzle3d_distribution_group(envelope, labels, Some(false)),
    ];
    if envelope.active_utility == "brush" {
        if let Some(target) = puzzle3d_brush_target_vortex(envelope) {
            let candidates = precompute.brush_candidates(&target).free;
            if !candidates.is_empty() {
            let items: Vec<MeasureSelectItem> = candidates
                .iter()
                .enumerate()
                .map(|(index, candidate)| {
                    let label = candidate.object_kind_id.as_str();
                    let id = format!("puzzle3d.brush.candidate.{index}");
                    MeasureSelectItem { id: id.clone(), value: id, label: label.into() }
                })
                .collect();
            let selected_index = envelope.runtime.brush_candidate_index.min(items.len().saturating_sub(1));
            children.push(WindowMeasure::Select {
                id: "puzzle3d-brush-placement".into(),
                label: Some(labels.placement.into()),
                value: format!("puzzle3d.brush.candidate.{selected_index}"),
                items,
                on_change: puzzle3d_action("engagementControlSelect", None),
            });
            }
        }
    }
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-brush"),
        label: labels.brush.into(),
        default_open: Some(true),
        active_utility_id: Some("brush".into()),
        children,
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
    }
}

/// 🎛️ Utility Options for the Transform utility — Move and Rotate flags that semio_compose_rs the gumball.
/// Tagged `Some("transform")` as a routing envelope only; children render flat under the Transform toggle.
fn puzzle3d_transform_utility_options(runtime: &Puzzle3dRuntime, labels: &Puzzle3dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-utility-options-transform"),
        label: String::new(),
        default_open: Some(true),
        active_utility_id: Some("transform".into()),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            WindowMeasure::Toggle {
                id: "puzzle3d-transform-move".into(),
                icon_id: "move-3d".into(),
                label: Some(labels.move_flag.into()),
                pressed: runtime.transform_move,
                text: None,
                on_change: puzzle3d_action("setTransformGumballFlag", Some(json!({ "flag": "move" }))),
            },
            WindowMeasure::Toggle {
                id: "puzzle3d-transform-rotate".into(),
                icon_id: "rotate-cw".into(),
                label: Some(labels.rotate_flag.into()),
                pressed: runtime.transform_rotate,
                text: None,
                on_change: puzzle3d_action("setTransformGumballFlag", Some(json!({ "flag": "rotate" }))),
            },
        ],
    }
}

fn puzzle3d_window_measures(envelope: &Puzzle3dScene, precompute: &Puzzle3dPrecomputeSession, labels: &Puzzle3dLabels) -> Vec<WindowMeasure> {
    vec![
        world3d_projection_measures("puzzle3d", &envelope.runtime.camera.projection, puzzle3d_action),
        puzzle3d_vortex_show_measure(&envelope.runtime, labels),
        puzzle3d_vortex_direction_measure(&envelope.runtime, labels),
        puzzle3d_lod_measures_group(&envelope.runtime, labels),
        puzzle3d_grid_measures_group(&envelope.runtime, labels),
        puzzle3d_select_measures_group(&envelope.runtime, labels),
        world3d_sun_measures("puzzle3d", &envelope.runtime.sun, puzzle3d_action),
        puzzle3d_transform_utility_options(&envelope.runtime, labels),
        puzzle3d_brush_utility_options(envelope, precompute, labels),
        puzzle3d_volume_brush_utility_options(&envelope.runtime, labels),
    ]
}
//#endregion 🔖️Measures

//#region 🔖️Puzzle3dPlayApp
/// 🧩️ Puzzle-3d play app. Owns the precompute engine and ephemeral view `runtime`; the persisted
/// document (bare `Puzzle3dFixture` json) lives in the wrapping `VcsDocumentApp`'s operation store. Each
/// action rehydrates the engine from the projection, mutates a transient {@link Puzzle3dScene},
/// then emits the granular operation delta. Undo/redo/checkpoints are handled by the wrapper — the former
/// manual `undo_stack`/`redo_stack` machinery is gone.
///
/// 🧲️ Gumball drags use a scratch-commit session (`transform_drag_active` + `transform_base` /
/// `transform_scratch`): mid-drag ticks accumulate incremental deltas onto the scratch and emit no
/// operations; `transformEnd` commits the base→scratch fixture delta once.
pub struct Puzzle3dPlayApp {
    precompute: std::cell::RefCell<Puzzle3dPrecomputeSession>,
    runtime: std::cell::RefCell<Puzzle3dRuntime>,
    transform_drag_active: std::cell::RefCell<bool>,
    transform_base: std::cell::RefCell<Option<Puzzle3dFixture>>,
    transform_scratch: std::cell::RefCell<Option<Puzzle3dFixture>>,
    /// 👻️ Per-`key` monotone counter for `gesture_preview` — see `//#region 🔖️GesturePreview`.
    preview_seq: std::cell::RefCell<u64>,
    fill_display_memo: Mutex<Option<FillDisplayMemo>>,
    geometry_cache: Mutex<Option<(u64, String, String)>>,
    document_sections_cache: Mutex<Option<(u64, Vec<UiTreeSectionNode>)>>,
}

impl Default for Puzzle3dPlayApp {
    fn default() -> Self {
        Self {
            precompute: std::cell::RefCell::new(Puzzle3dPrecomputeSession::new()),
            runtime: std::cell::RefCell::new(Puzzle3dRuntime::default()),
            transform_drag_active: std::cell::RefCell::new(false),
            transform_base: std::cell::RefCell::new(None),
            transform_scratch: std::cell::RefCell::new(None),
            preview_seq: std::cell::RefCell::new(0),
            fill_display_memo: Mutex::new(None),
            geometry_cache: Mutex::new(None),
            document_sections_cache: Mutex::new(None),
        }
    }
}

impl Puzzle3dPlayApp {
    fn geometry_jsons(&self, fixture: &Puzzle3dFixture) -> (String, String) {
        let fingerprint = fixture_geometry_fingerprint(fixture);
        let mut cache = self.geometry_cache.lock().expect("geometry cache");
        if cache.as_ref().is_none_or(|(fp, _, _)| *fp != fingerprint) {
            *cache = Some((fingerprint, world_instances_geometry_json(fixture), world_meshes_json(fixture)));
            *self.document_sections_cache.lock().expect("document cache") = None;
        }
        let (_, instances, meshes) = cache.as_ref().expect("geometry cache populated");
        (instances.clone(), meshes.clone())
    }

    fn document_sections_cached(&self, fixture: &Puzzle3dFixture, labels: &Puzzle3dLabels) -> Vec<UiTreeSectionNode> {
        let fingerprint = fixture_geometry_fingerprint(fixture);
        let mut cache = self.document_sections_cache.lock().expect("document cache");
        if cache.as_ref().is_none_or(|(fp, _)| *fp != fingerprint) {
            *cache = Some((fingerprint, document_tree_sections(fixture, labels)));
        }
        cache.as_ref().expect("document cache populated").1.clone()
    }
}

impl Puzzle3dPlayApp {
    /// 🎬️ Snapshots the live fixture as the gumball drag base and clears any prior scratch.
    fn begin_transform_session(&self, projection: &Value) {
        let fixture = serde_json::from_value::<Puzzle3dFixture>(projection.clone()).unwrap_or_else(|_| empty_fixture());
        *self.transform_drag_active.borrow_mut() = true;
        *self.transform_base.borrow_mut() = Some(fixture);
        *self.transform_scratch.borrow_mut() = None;
    }

    /// 🧹️ Drops an in-progress gumball scratch without committing.
    fn clear_transform_session(&self) {
        *self.transform_drag_active.borrow_mut() = false;
        *self.transform_base.borrow_mut() = None;
        *self.transform_scratch.borrow_mut() = None;
    }

    /// 🧲️ One mid-drag gumball tick: accumulates an incremental delta onto `transform_scratch`
    /// (seeded from the drag-start base) and emits zero operations (scratch-commit pattern b).
    fn transform_drag_tick(&self, action: &str, args: Option<&Value>, projection: &Value) -> ActionEmit<Puzzle3dOperation> {
        if self.transform_base.borrow().is_none() {
            self.begin_transform_session(projection);
        }
        let object_ids = mesh_selection_ids(args, &self.runtime.borrow().selection.object_ids);
        let volume_ids = self.runtime.borrow().selection.target_volume_ids.to_vec();
        let mut scratch = self
            .transform_scratch
            .borrow()
            .clone()
            .or_else(|| self.transform_base.borrow().clone())
            .unwrap_or_else(empty_fixture);
        match action {
            "translateSelection" => {
                let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                puzzle3d_apply_translate(&mut scratch, &object_ids, &volume_ids, dx, dy, dz);
            }
            "rotateSelection" => {
                let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                puzzle3d_apply_rotate(&mut scratch, &object_ids, &volume_ids, ax, ay, az, angle);
            }
            "scaleSelection" => {
                let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                puzzle3d_apply_scale(&mut scratch, &object_ids, &volume_ids, sx, sy, sz);
            }
            _ => {}
        }
        *self.transform_scratch.borrow_mut() = Some(scratch);
        {
            let next = self.preview_seq.borrow().wrapping_add(1);
            *self.preview_seq.borrow_mut() = next;
        }
        ActionEmit { ui_scope: puzzle3d_transform_drag_scope(), ..Default::default() }
    }

    /// 📌️ Commits the whole gumball drag as ONE fixture delta (base → scratch), resolving attractions once.
    fn commit_transform(&self, projection: &Value) -> ActionEmit<Puzzle3dOperation> {
        *self.transform_drag_active.borrow_mut() = false;
        let Some(mut scratch) = self.transform_scratch.borrow_mut().take() else {
            *self.transform_base.borrow_mut() = None;
            return ActionEmit::default();
        };
        *self.transform_base.borrow_mut() = None;
        let object_ids = self.runtime.borrow().selection.object_ids.to_vec();
        let incoming = resolve_puzzle3d_attractions(&mut scratch);
        puzzle3d_rederive_moved_attractions(&mut scratch, &object_ids, &incoming);
        resolve_puzzle3d_attractions(&mut scratch);
        let operations = puzzle3d_operations_from_fixture_change(projection, &scratch);
        if operations.is_empty() {
            ActionEmit { ui_scope: puzzle3d_transform_drag_scope(), ..Default::default() }
        } else {
            ActionEmit::commit(operations, "Transform selection")
        }
    }

    /// 🖼️ Fixture used for world render — live scratch while a gumball drag is in progress.
    fn render_fixture<'a>(&'a self, projection: &'a Value) -> Puzzle3dFixture {
        if let Some(scratch) = self.transform_scratch.borrow().as_ref() {
            return scratch.clone();
        }
        serde_json::from_value::<Puzzle3dFixture>(projection.clone()).unwrap_or_else(|_| empty_fixture())
    }

    //#region 🔖️GesturePreview
    /// 👻️ CW7 db+protocol+vcs-slimming campaign, "preview law for gesture apps": the live gumball
    /// drag's current fixture state, expressed as the same document-delta operations
    /// `commit_transform` would eventually emit for real — anchored to the drag-start snapshot
    /// (`transform_base`), never to the previous preview tick, so a preview built from this stays
    /// correct even when the lossy, uncredited preview lane drops every message but the latest (a
    /// receiver only ever needs `transform_base` — the last-synced canonical fixture, which it
    /// already has — plus this one delta, never a chain of prior preview messages). `None` outside
    /// an active drag; this reads `transform_base`/`transform_scratch` only, never emits or mutates
    /// a `Puzzle3dOperation`.
    ///
    /// 🚧️ Deliberately unwired beyond this accessor — same gap as `draw-plugin`'s
    /// `draw_gesture_preview_payload`: `framework/sync::SyncSession::publish_preview` is host-only
    /// and unreachable from this WASI-P2 sandboxed plugin crate, and `store::BackboneMessage` has no
    /// preview-shaped variant to relay one through. See `.🦑️repo/🎫️tickets/26/07/27/
    /// INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/cw7-preview-law.txt`.
    /// `#[allow(dead_code)]`: exercised by `🧪️Tests` only until a host bridge exists.
    #[allow(dead_code)]
    fn gesture_preview(&self) -> Option<(&'static str, u64, Vec<u8>)> {
        let base_binding = self.transform_base.borrow();
        let base = base_binding.as_ref()?;
        let scratch_binding = self.transform_scratch.borrow();
        let scratch = scratch_binding.as_ref()?;
        let before = serde_json::to_value(base).ok()?;
        let operations = puzzle3d_operations_from_fixture_change(&before, scratch);
        let payload = json!({ "operations": operations });
        Some(("gesture:transform", *self.preview_seq.borrow(), serde_json::to_vec(&payload).ok()?))
    }
    //#endregion 🔖️GesturePreview
}

impl DocumentApp for Puzzle3dPlayApp {
    type Projection = Puzzle3dPlayProjection;
    type Operation = Puzzle3dOperation;
        type Config = semio_framework_plugin::NoConfig;
        type ConfigOperation = semio_framework_plugin::NoConfigOperation;

    fn app_id(&self) -> &str {
        PUZZLE3D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        PUZZLE3D_FIXTURE_SCHEMA
    }

    fn initial_projection(&self) -> Puzzle3dPlayProjection {
        Puzzle3dPlayProjection(serde_json::to_value(default_fixture()).unwrap_or_else(|_| serde_json::to_value(empty_fixture()).unwrap_or(Value::Null)))
    }

    fn handle_action(
        &self,
        action: &str,
        args: Option<&Value>,
        doc: &DocumentView<'_, Puzzle3dPlayProjection>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        view_state: &ViewState,
    ) -> ActionEmit<Puzzle3dOperation> {
        // 🗨️ Shell-only effect (no document interaction, hence no `envelope`/`before`/`after` scaffolding
        // below): opens the declared "addObject" dialog over a glass veil.
        if action == "openAddObjectDialog" {
            return ActionEmit::effect(HostEffect::OpenDialog { dialog_id: "addObject".into(), args: None });
        }
        if action == "transformBegin" {
            self.begin_transform_session(&doc.projection.0);
            return ActionEmit::default();
        }
        if action == "transformEnd" {
            return self.commit_transform(&doc.projection.0);
        }
        if *self.transform_drag_active.borrow() && matches!(action, "translateSelection" | "rotateSelection" | "scaleSelection") {
            return self.transform_drag_tick(action, args, &doc.projection.0);
        }
        let document_action = puzzle3d_action_document_intent(action);
        let before = document_action.then(|| doc.projection.0.clone());
        let active_utility_initial = puzzle3d_scene_active_utility(view_state, view_state.window_id.as_deref());
        // 🪟️ This action targets exactly one window instance — materialize ITS view-local options onto
        // the scene runtime before handling, and snapshot them back out (via `save_window`, at every
        // exit below) so a grid/LOD/selection/vortex/sun mutation never leaks into another window's
        // options. Fill count / distribution / overlap stay on the flat runtime and are shared.
        let wid = view_state.window_id.clone().unwrap_or_else(|| PUZZLE3D_PLAY_WINDOW_MAIN.into());
        let mut runtime_for_window = self.runtime.borrow().clone();
        runtime_for_window.load_window(&wid);
        let mut envelope = scene_from_projection(&doc.projection.0, runtime_for_window, &active_utility_initial);
        let mut ui_scope = semio_framework_core::kernel::UiDirtyScope::Full;
        let mut effects = Vec::new();
        let preserve_fill_plan = matches!(action, "setFillCount" | "fillBuildTick");
        let skip_precompute_sync = matches!(action, "worldPick" | "worldSelect" | "setSelection" | "clearSelection" | "selectAll");
        if !preserve_fill_plan && !skip_precompute_sync {
            sync_precompute_session(&mut *self.precompute.borrow_mut(), &envelope);
        }
        match action {
            "setFixtureJson" => {
                if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                    if let Ok(fixture) = serde_json::from_str::<Puzzle3dFixture>(json_text) {
                        envelope.fixture = fixture;
                        resolve_puzzle3d_attractions(&mut envelope.fixture);
                    }
                }
            }
            "setActiveExample" => {
                let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                let next = if example_id.is_empty() {
                    Some(empty_fixture())
                } else if example_id == PUZZLE3D_EXAMPLE_CONCRETE_FOREST || example_id == "concrete" {
                    Some(default_fixture())
                } else if example_id == PUZZLE3D_EXAMPLE_NAKAGIN || example_id == "nakagin" {
                    Some(nakagin_fixture())
                } else {
                    None
                };
                if let Some(fixture) = next {
                    envelope.fixture = fixture;
                    envelope.runtime = Puzzle3dRuntime::default();
                }
                resolve_puzzle3d_attractions(&mut envelope.fixture);
                drive_precompute(&mut *self.precompute.borrow_mut(), &envelope);
            }
            "setSelection" => {
                if let Some(selection) = args.and_then(|value| value.get("selection")) {
                    if let Ok(parsed) = serde_json::from_value(selection.clone()) {
                        envelope.runtime.selection = parsed;
                    }
                }
            }
            SET_ACTIVE_UTILITY_ACTION_ID | SET_ACTIVE_TOOL_ACTION_ID => {
                // 🧰️🛠️ Host already applied `view_state.active_utility_id`/`active_tool_id`; clear
                // in-progress scratch and refresh the placement engine for the new utility/tool. Emits
                // no operations (View-kind) and no utility/tool-switch effect (the host already applied it).
                self.clear_transform_session();
                envelope.runtime.hovered_object_id = None;
                envelope.runtime.hovered_vortex_full_id = None;
                envelope.runtime.suggestion_menu = None;
                envelope.runtime.engagement_input = String::new();
                envelope.runtime.brush_candidate_index = 0;
                if envelope.active_utility == "brush" || envelope.active_utility == "fill" {
                    drive_precompute(&mut *self.precompute.borrow_mut(), &envelope);
                }
            }
            "addObjectKind" => {
                let object_kind = args.and_then(|value| value.get("objectKind")).and_then(|value| value.as_str()).unwrap_or("Object");
                let id = next_object_id();
                let catalog_entry = envelope.fixture.meta.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get("objects")?.as_array()?.iter().find(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(object_kind)).cloned());
                let mesh_url = catalog_entry.as_ref().and_then(|entry| entry.get("meshUrl").and_then(|v| v.as_str()).map(str::to_string));
                let vortices = catalog_entry.as_ref().map(|entry| puzzle3d_vortices_from_kind_template(entry)).unwrap_or_default();
                let origin = args
                    .and_then(|value| value.get("origin"))
                    .and_then(|value| value.as_array())
                    .map(|values| [values.first().and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(2).and_then(|v| v.as_f64()).unwrap_or(0.0)])
                    .unwrap_or([0.0, 0.0, 0.0]);
                envelope.fixture.objects.push(Puzzle3dObject {
                    id: id.clone(),
                    label: Some(object_kind.into()),
                    object_kind: Some(object_kind.into()),
                    origin,
                    orientation: Some([0.0, 0.0, 0.0, 1.0]),
                    scale: None,
                    mesh_url,
                    vortices,
                    hidden: false,
                    locked: false,
                    reveal_index: None,
                });
                envelope.runtime.selection.object_ids = SelectionSet::from(vec![id]);
                resolve_puzzle3d_attractions(&mut envelope.fixture);
            }
            "deleteSelection" => {
                let object_ids: Vec<String> = envelope.runtime.selection.object_ids.to_vec();
                let vortex_ids: HashSet<String> = envelope.runtime.selection.vortex_ids.iter().cloned().collect();
                let attraction_ids: Vec<String> = envelope.runtime.selection.attraction_ids.to_vec();
                let target_volume_ids: Vec<String> = envelope.runtime.selection.target_volume_ids.to_vec();
                envelope.fixture.objects.retain(|object| !object_ids.contains(&object.id));
                if !vortex_ids.is_empty() {
                    for object in envelope.fixture.objects.iter_mut() {
                        object.vortices.retain(|vortex| !vortex_ids.contains(&puzzle3d_vortex_full_id(&object.id, &vortex.id)));
                    }
                }
                envelope.fixture.attractions.retain(|attraction| !attraction_ids.contains(&attraction.id) && !object_ids.iter().any(|id| attraction.attracting.starts_with(&format!("{id}:")) || attraction.attracted.starts_with(&format!("{id}:"))));
                envelope.fixture.target_volumes.retain(|volume| !target_volume_ids.contains(&volume.id));
                let reference_ids: Vec<String> = envelope.runtime.selection.reference_ids.to_vec();
                envelope.fixture.references.retain(|reference| !reference_ids.contains(&reference.id));
                envelope.runtime.selection = Puzzle3dSelection::default();
            }
            "duplicateSelection" => {
                let ids = &envelope.runtime.selection.object_ids;
                let clones: Vec<Puzzle3dObject> = envelope
                    .fixture
                    .objects
                    .iter()
                    .filter(|object| ids.contains(&object.id))
                    .map(|object| {
                        let mut clone = object.clone();
                        clone.id = next_object_id();
                        clone.origin[0] += 0.5;
                        clone.origin[1] += 0.5;
                        clone
                    })
                    .collect();
                let new_ids: Vec<String> = clones.iter().map(|object| object.id.clone()).collect();
                envelope.fixture.objects.extend(clones);
                envelope.runtime.selection.object_ids = SelectionSet::from(new_ids);
                resolve_puzzle3d_attractions(&mut envelope.fixture);
            }
            "selectSameKindSelection" => {
                let Some(first_id) = envelope.runtime.selection.object_ids.first() else {
                    envelope.runtime.save_window(&wid);
                    *self.runtime.borrow_mut() = envelope.runtime;
                    return ActionEmit::default();
                };
                let Some(kind) = envelope.fixture.objects.iter().find(|object| object.id == *first_id).and_then(|object| object.object_kind.clone()).filter(|kind| !kind.is_empty()) else {
                    envelope.runtime.save_window(&wid);
                    *self.runtime.borrow_mut() = envelope.runtime;
                    return ActionEmit::default();
                };
                envelope.runtime.selection.object_ids = envelope
                    .fixture
                    .objects
                    .iter()
                    .filter(|object| object.object_kind.as_deref() == Some(kind.as_str()))
                    .map(|object| object.id.clone())
                    .collect::<SelectionSet>();
            }
            "setCamera" => {
                // 🎥️ Session-only per-window state (View-kind, see the app builder below) — writes the
                // materialized-window camera on `runtime`, never the shared `fixture`, so this never
                // creates a VCS edit and never moves any sibling window instance's camera.
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        envelope.runtime.camera = parsed;
                    }
                }
            }
            "setProjection" | "setProjectionParam" => {
                // 🎥️ Session-only per-window state (View-kind, see the app builder below) — same as `setCamera`.
                let moves_pose = world3d_projection_action_moves_pose(action, args);
                apply_world3d_projection_action(&mut envelope.runtime.camera.projection, action, args);
                if moves_pose {
                    let distance = puzzle3d_camera_distance(&envelope.runtime.camera);
                    let (position, up) = world3d_projection_pose(&envelope.runtime.camera.projection, envelope.runtime.camera.target, distance);
                    envelope.runtime.camera.position = position;
                    envelope.runtime.camera.up = Some(up);
                }
            }
            "setVortexShow" => {
                if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    if mode == PUZZLE3D_VORTEX_SHOW_ALWAYS || mode == PUZZLE3D_VORTEX_SHOW_SELECTED {
                        envelope.runtime.vortex_show = mode.into();
                    }
                }
            }
            "setVortexDirection" => {
                if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
                    if mode == PUZZLE3D_VORTEX_DIRECTION_OUTWARDS || mode == PUZZLE3D_VORTEX_DIRECTION_INWARDS {
                        envelope.runtime.vortex_direction = mode.into();
                    }
                }
            }
            "translateSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selection.object_ids);
                let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let incoming = resolve_puzzle3d_attractions(&mut envelope.fixture);
                puzzle3d_apply_translate(&mut envelope.fixture, &ids, envelope.runtime.selection.target_volume_ids.as_slice(), dx, dy, dz);
                puzzle3d_rederive_moved_attractions(&mut envelope.fixture, &ids, &incoming);
                resolve_puzzle3d_attractions(&mut envelope.fixture);
            }
            "rotateSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selection.object_ids);
                let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let incoming = resolve_puzzle3d_attractions(&mut envelope.fixture);
                puzzle3d_apply_rotate(&mut envelope.fixture, &ids, envelope.runtime.selection.target_volume_ids.as_slice(), ax, ay, az, angle);
                puzzle3d_rederive_moved_attractions(&mut envelope.fixture, &ids, &incoming);
                resolve_puzzle3d_attractions(&mut envelope.fixture);
            }
            "scaleSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selection.object_ids);
                let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                puzzle3d_apply_scale(&mut envelope.fixture, &ids, envelope.runtime.selection.target_volume_ids.as_slice(), sx, sy, sz);
            }
            "relocateTargetVolume" => {
                let volume_id = args.and_then(|value| value.get("volumeId")).and_then(|value| value.as_str()).unwrap_or("");
                let after = args.and_then(|value| value.get("after"));
                if let Some(volume) = envelope.fixture.target_volumes.iter_mut().find(|volume| volume.id == volume_id && !volume.locked) {
                    if let Some(after) = after {
                        if let Some(origin) = after.get("position").and_then(value_as_vec3) {
                            volume.origin = origin;
                        }
                        if let Some(values) = after.get("quaternion").and_then(|value| value.as_array()).filter(|values| values.len() >= 4) {
                            volume.orientation = Some([
                                values[0].as_f64().unwrap_or(0.0),
                                values[1].as_f64().unwrap_or(0.0),
                                values[2].as_f64().unwrap_or(0.0),
                                values[3].as_f64().unwrap_or(1.0),
                            ]);
                        }
                        if let Some(scale) = after.get("scale").and_then(|value| value.as_array()).filter(|values| values.len() >= 3) {
                            volume.scale = Some(json!([
                                scale[0].as_f64().unwrap_or(1.0),
                                scale[1].as_f64().unwrap_or(1.0),
                                scale[2].as_f64().unwrap_or(1.0),
                            ]));
                        }
                    }
                }
            }
            "worldSelect" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                envelope.runtime.selection.object_ids = merge_world_selection_ids(&envelope.runtime.selection.object_ids, &ids, merge);
            }
            "worldHover" => {
                envelope.runtime.hovered_object_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
            }
            "setHover" => {
                if args.is_none() || args.and_then(|value| value.get("objectId")).is_none() {
                    envelope.runtime.hovered_object_id = None;
                } else {
                    envelope.runtime.hovered_object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).map(str::to_string);
                }
            }
            "worldPick" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                if args.and_then(|value| value.get("id")).map_or(true, |value| value.is_null()) {
                    if merge == "replace" {
                        puzzle3d_clear_selection(&mut envelope.runtime.selection);
                    }
                } else if envelope.runtime.selectable_kinds.objects {
                    let index = args.and_then(|value| value.get("id")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                    // 🔓️ Locked/hidden picks are equivalent to background: clear on replace instead of
                    // no-opping while the mesh still absorbs the click ahead of `onPointerMissed`.
                    match envelope.fixture.objects.get(index).filter(|object| !object.locked && !object.hidden) {
                        Some(object) => {
                            let id = object.id.clone();
                            if merge == "replace" {
                                puzzle3d_clear_non_object_selection(&mut envelope.runtime.selection);
                            }
                            envelope.runtime.selection.object_ids =
                                merge_world_selection_ids(&envelope.runtime.selection.object_ids, &[id], merge);
                        }
                        None if merge == "replace" => {
                            puzzle3d_clear_selection(&mut envelope.runtime.selection);
                        }
                        None => {}
                    }
                }
            }
            "worldVortexHover" => {
                envelope.runtime.hovered_vortex_full_id = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()).map(str::to_string);
                if envelope.active_utility == "brush" && envelope.runtime.hovered_vortex_full_id.is_some() {
                    drive_precompute(&mut *self.precompute.borrow_mut(), &envelope);
                }
            }
            "worldVortexSelect" => {
                if envelope.runtime.selectable_kinds.vortices {
                    if let Some(full_id) = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()) {
                        let merge = args
                            .and_then(|value| value.get("merge"))
                            .and_then(|value| value.as_str())
                            .unwrap_or(&envelope.runtime.selection_mode_default);
                        let merge_mode = match merge {
                            "additive" => "add",
                            "subtractive" => "remove",
                            "invertive" => "toggle",
                            "default" => "replace",
                            other => other,
                        };
                        if merge_mode == "replace" {
                            puzzle3d_clear_non_vortex_selection(&mut envelope.runtime.selection);
                        }
                        envelope.runtime.selection.vortex_ids = merge_world_selection_ids(&envelope.runtime.selection.vortex_ids, &[full_id.to_string()], merge_mode);
                        drive_precompute(&mut *self.precompute.borrow_mut(), &envelope);
                    }
                }
            }
            "worldRelocate" => {
                let object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).unwrap_or("");
                let position = args
                    .and_then(|value| value.get("position"))
                    .and_then(|value| value.as_array())
                    .map(|values| [values.first().and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(1).and_then(|v| v.as_f64()).unwrap_or(0.0), values.get(2).and_then(|v| v.as_f64()).unwrap_or(0.0)]);
                let proximity_radius = envelope.runtime.proximity_radius;
                if let (Some(object), Some(position)) = (envelope.fixture.objects.iter_mut().find(|object| object.id == object_id && !object.locked && !object.hidden), position) {
                    object.origin = position;
                    let object_orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                    let mut source_vortex: Option<(String, [f64; 3], [f64; 3], [f64; 3])> = None;
                    for vortex in &object.vortices {
                        let full_id = puzzle3d_vortex_full_id(&object.id, &vortex.id);
                        source_vortex = Some((full_id, world_vortex_position(object, vortex), vortex.position, vortex.direction.unwrap_or([0.0, 0.0, -1.0])));
                        break;
                    }
                    // 🌲️ New attractions attach the MOVED object as `attracted`: the pre-existing, stationary
                    // structure it snapped onto stays the resolution root. Params are derived from the current
                    // (already-relocated) poses so nothing jumps when the resolver next runs.
                    if let Some((source_id, source_pos, source_local_pos, source_local_dir)) = source_vortex {
                        for other in &envelope.fixture.objects {
                            if other.id == object_id {
                                continue;
                            }
                            for vortex in &other.vortices {
                                let target_id = puzzle3d_vortex_full_id(&other.id, &vortex.id);
                                if target_id == source_id {
                                    continue;
                                }
                                let target_pos = world_vortex_position(other, vortex);
                                let dx = source_pos[0] - target_pos[0];
                                let dy = source_pos[1] - target_pos[1];
                                let dz = source_pos[2] - target_pos[2];
                                let distance = (dx * dx + dy * dy + dz * dz).sqrt();
                                if distance <= proximity_radius {
                                    let already_connected = envelope.fixture.attractions.iter().any(|entry| entry.attracting == source_id && entry.attracted == target_id || entry.attracting == target_id && entry.attracted == source_id);
                                    if !already_connected {
                                        let attraction_id = format!("attraction-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
                                        let (gap, shift, rise, rotation, turn, tilt) = derive_attraction_params(other.origin, other.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), vortex.position, vortex.direction.unwrap_or([0.0, 0.0, -1.0]), source_local_pos, source_local_dir, position, object_orientation);
                                        envelope.fixture.attractions.push(Puzzle3dAttraction { id: attraction_id, attracting: target_id, attracted: source_id.clone(), gap, shift, rise, rotation, turn, tilt });
                                    }
                                }
                            }
                        }
                    }
                    resolve_puzzle3d_attractions(&mut envelope.fixture);
                    sync_precompute_session(&mut *self.precompute.borrow_mut(), &envelope);
                }
            }
            "setSelectionMethod" => {
                let method = args.and_then(|value| value.get("method")).and_then(|value| value.as_str()).unwrap_or("rectangle");
                envelope.runtime.selection_method = method.into();
            }
            "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => {
                apply_world3d_sun_action(&mut envelope.runtime.sun, action, args);
            }
            "setLodAutomatic" => {
                envelope.runtime.lod_automatic = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!envelope.runtime.lod_automatic);
            }
            "setLodDepthVariable" => {
                envelope.runtime.lod_depth_variable = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!envelope.runtime.lod_depth_variable);
            }
            "setGridVisible" => {
                envelope.runtime.grid_visible = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!envelope.runtime.grid_visible);
            }
            "setLodManual" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                    envelope.runtime.lod_manual = value.clamp(PUZZLE3D_LOD_SLIDER_MIN, PUZZLE3D_LOD_SLIDER_MAX);
                }
            }
            "setGridSnapEnabled" => {
                envelope.runtime.grid_snap_enabled = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!envelope.runtime.grid_snap_enabled);
            }
            "setGridSpacing" => {
                if let Some(value) = puzzle3d_absolute_or_delta(args, envelope.runtime.grid_spacing) {
                    envelope.runtime.grid_spacing = value.max(0.1);
                }
            }
            "setSelectionModeDefault" => {
                if let Some(mode) = args
                    .and_then(|value| value.get("mode").or_else(|| value.get("value")))
                    .and_then(|value| value.as_str())
                {
                    envelope.runtime.selection_mode_default = mode.into();
                }
            }
            "setProximityRadius" => {
                if let Some(value) = puzzle3d_absolute_or_delta(args, envelope.runtime.proximity_radius) {
                    envelope.runtime.proximity_radius = value.max(0.0);
                }
            }
            "setChunkSize" => {
                if let Some(value) = puzzle3d_absolute_or_delta(args, envelope.runtime.chunk_size) {
                    envelope.runtime.chunk_size = value.max(1.0);
                }
            }
            "setSelectableKind" => {
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("");
                let pressed = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool());
                match kind {
                    "objects" => envelope.runtime.selectable_kinds.objects = pressed.unwrap_or(!envelope.runtime.selectable_kinds.objects),
                    "vortices" => envelope.runtime.selectable_kinds.vortices = pressed.unwrap_or(!envelope.runtime.selectable_kinds.vortices),
                    "attractions" => envelope.runtime.selectable_kinds.attractions = pressed.unwrap_or(!envelope.runtime.selectable_kinds.attractions),
                    _ => {}
                }
            }
            "setKindHover" => {
                envelope.runtime.hovered_kind_id = args.and_then(|value| value.get("kindId")).and_then(|value| value.as_str()).map(str::to_string);
            }
            "setSelectionFlag" => {
                let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
                let entity = args.and_then(|value| value.get("entity")).and_then(|value| value.as_str());
                let explicit_ids: Option<Vec<String>> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok());
                match (entity, explicit_ids) {
                    (Some(entity), Some(ids)) => apply_puzzle3d_selection_flag(&mut envelope.fixture, entity, &ids, flag, value),
                    _ => {
                        apply_puzzle3d_selection_flag(&mut envelope.fixture, "object", envelope.runtime.selection.object_ids.as_slice(), flag, value);
                        apply_puzzle3d_selection_flag(&mut envelope.fixture, "vortex", envelope.runtime.selection.vortex_ids.as_slice(), flag, value);
                        apply_puzzle3d_selection_flag(&mut envelope.fixture, "targetVolume", envelope.runtime.selection.target_volume_ids.as_slice(), flag, value);
                    }
                }
            }
            "patchInspector" => {
                let entity = args.and_then(|value| value.get("entity")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let ids = args
                    .and_then(|value| value.get("ids"))
                    .and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok())
                    .filter(|ids| !ids.is_empty())
                    .unwrap_or_else(|| puzzle3d_inspector_target_ids(entity, &envelope.runtime.selection));
                let value = args.and_then(|value| value.get("value"));
                let delta = args.and_then(|value| value.get("delta"));
                apply_puzzle3d_inspector_patch(&mut envelope.fixture, entity, &ids, field, value, delta);
                resolve_puzzle3d_attractions(&mut envelope.fixture);
            }
            "selectAll" => {
                envelope.runtime.selection.object_ids = if envelope.runtime.selectable_kinds.objects {
                    envelope
                        .fixture
                        .objects
                        .iter()
                        .filter(|object| !object.hidden && !object.locked)
                        .map(|object| object.id.clone())
                        .collect::<SelectionSet>()
                } else {
                    SelectionSet::default()
                };
                envelope.runtime.selection.vortex_ids.clear();
                envelope.runtime.selection.attraction_ids.clear();
                envelope.runtime.selection.target_volume_ids.clear();
                envelope.runtime.selection.reference_ids.clear();
            }
            "clearSelection" => {
                envelope.runtime.selection = Puzzle3dSelection::default();
            }
            "contextMenuAt" => {
                // 🖱️ Right-click on an unselected entity selects it and opens its menu in one round trip,
                // instead of requiring a separate pick action before the menu items become available.
                let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("");
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).unwrap_or("");
                envelope.runtime.selection = Puzzle3dSelection::default();
                match kind {
                    "object" => envelope.runtime.selection.object_ids = SelectionSet::from(vec![id.to_string()]),
                    "vortex" => envelope.runtime.selection.vortex_ids = SelectionSet::from(vec![id.to_string()]),
                    "attraction" => envelope.runtime.selection.attraction_ids = SelectionSet::from(vec![id.to_string()]),
                    "targetVolume" => envelope.runtime.selection.target_volume_ids = SelectionSet::from(vec![id.to_string()]),
                    "reference" => envelope.runtime.selection.reference_ids = SelectionSet::from(vec![id.to_string()]),
                    _ => {}
                }
            }
            "focusSelection" => {
                apply_puzzle3d_focus_selection(&mut envelope);
            }
            "engagementInput" => {
                envelope.runtime.engagement_input = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("").to_string();
            }
            "engagementSubmit" => {
                let raw = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("").trim().to_string();
                if let Some(rest) = strip_engagement_prefix(&raw, "fill") {
                    envelope.active_utility = "fill".into();
                    drive_precompute(&mut *self.precompute.borrow_mut(), &envelope);
                    let count = rest.parse::<u32>().ok().unwrap_or(envelope.runtime.fill_count).min(PUZZLE3D_FILL_COUNT_MAX);
                    envelope = apply_puzzle3d_fill_count(&mut *self.precompute.borrow_mut(), envelope, count);
                } else {
                    match raw.to_lowercase().as_str() {
                        "brush" => {
                            envelope.active_utility = "brush".into();
                            drive_precompute(&mut *self.precompute.borrow_mut(), &envelope);
                        }
                        "zoom" => apply_puzzle3d_focus_selection(&mut envelope),
                        "clear" => puzzle3d_clear_selection(&mut envelope.runtime.selection),
                        "rectangle" => envelope.runtime.selection_method = "rectangle".into(),
                        "lasso" => envelope.runtime.selection_method = "lasso".into(),
                        _ => {}
                    }
                }
                envelope.runtime.engagement_input = String::new();
            }
            "engagementRepeatLast" => {
                if envelope.active_utility == "fill" {
                    let count = (envelope.runtime.fill_count + 1).min(PUZZLE3D_FILL_COUNT_MAX);
                    envelope = apply_puzzle3d_fill_count(&mut *self.precompute.borrow_mut(), envelope, count);
                }
            }
            "engagementAbort" => {
                envelope.runtime.engagement_input = String::new();
                envelope.runtime.brush_candidate_index = 0;
                envelope.active_utility = PUZZLE3D_DEFAULT_UTILITY.into();
            }
            "createAttraction" => {
                let attracting = args.and_then(|value| value.get("attracting")).and_then(|value| value.as_str()).unwrap_or("");
                let attracted = args.and_then(|value| value.get("attracted")).and_then(|value| value.as_str()).unwrap_or("");
                if !attracting.is_empty() && !attracted.is_empty() && attracting != attracted {
                    let already_connected = envelope.fixture.attractions.iter().any(|attraction| (attraction.attracting == attracting && attraction.attracted == attracted) || (attraction.attracting == attracted && attraction.attracted == attracting));
                    let compatible = match (resolve_vortex_kind(&envelope.fixture, attracting), resolve_vortex_kind(&envelope.fixture, attracted)) {
                        (Some(source_kind), Some(target_kind)) => puzzle3d_kinds_compatible(&envelope.fixture, &source_kind, &target_kind),
                        _ => false,
                    };
                    if !already_connected && compatible {
                        let id = format!("attraction-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
                        // 🌲️ Keep the drag gesture's direction (source = attracting) but derive params from the
                        // CURRENT poses of both objects, so creating an attraction never moves either endpoint.
                        let (gap, shift, rise, rotation, turn, tilt) = match (puzzle3d_local_vortex_geom(&envelope.fixture, attracting), puzzle3d_local_vortex_geom(&envelope.fixture, attracted)) {
                            (Some((attracting_object_id, p_a, d_a)), Some((attracted_object_id, p_b, d_b))) => {
                                let pose = |object_id: &str| envelope.fixture.objects.iter().find(|object| object.id == object_id).map(|object| (object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0])));
                                match (pose(&attracting_object_id), pose(&attracted_object_id)) {
                                    (Some((t_a, q_a)), Some((t_b, q_b))) => derive_attraction_params(t_a, q_a, p_a, d_a, p_b, d_b, t_b, q_b),
                                    _ => (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
                                }
                            }
                            _ => (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
                        };
                        envelope.fixture.attractions.push(Puzzle3dAttraction { id, attracting: attracting.into(), attracted: attracted.into(), gap, shift, rise, rotation, turn, tilt });
                        resolve_puzzle3d_attractions(&mut envelope.fixture);
                    }
                }
            }
            "deleteAttraction" => {
                if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                    envelope.fixture.attractions.retain(|attraction| attraction.id != id);
                }
            }
            "setTransformGumballFlag" => {
                let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("");
                let pressed = args.and_then(|value| value.get("pressed")).and_then(Value::as_bool);
                match flag {
                    "move" => envelope.runtime.transform_move = pressed.unwrap_or(!envelope.runtime.transform_move),
                    "rotate" => envelope.runtime.transform_rotate = pressed.unwrap_or(!envelope.runtime.transform_rotate),
                    _ => {}
                }
            }
            "setVoxelDims" => {
                let axis = args.and_then(|value| value.get("axis")).and_then(|value| value.as_str()).unwrap_or("");
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                    let dimension = value.max(1.0).round() as u32;
                    match axis {
                        "w" => envelope.runtime.voxel_dims[0] = dimension,
                        "d" => envelope.runtime.voxel_dims[1] = dimension,
                        "h" => envelope.runtime.voxel_dims[2] = dimension,
                        _ => {}
                    }
                }
            }
            "addTargetVolume" => {
                if let Some(origin) = args.and_then(|value| value.get("origin")).and_then(value_as_vec3) {
                    let grid_spacing = envelope.runtime.grid_spacing.max(0.1);
                    let snapped = [(origin[0] / grid_spacing).round() * grid_spacing, (origin[1] / grid_spacing).round() * grid_spacing, (origin[2] / grid_spacing).round() * grid_spacing];
                    let [w, d, h] = envelope.runtime.voxel_dims;
                    let scale = json!([w as f64 * grid_spacing, d as f64 * grid_spacing, h as f64 * grid_spacing]);
                    let id = format!("target-volume-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
                    envelope.fixture.target_volumes.push(Puzzle3dTargetVolume { id, origin: snapped, orientation: None, scale: Some(scale), hidden: false, locked: false });
                }
            }
            "deleteTargetVolume" => {
                if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
                    envelope.fixture.target_volumes.retain(|volume| volume.id != id);
                }
            }
            "setTargetVolumeFlag" => {
                let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).unwrap_or("");
                let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(false);
                if let Some(volume) = envelope.fixture.target_volumes.iter_mut().find(|volume| volume.id == id) {
                    match flag {
                        "hidden" => volume.hidden = value,
                        "locked" => volume.locked = value,
                        _ => {}
                    }
                }
            }
            "engagementControlSelect" => {
                let candidate_id = args.and_then(|value| value.get("id").or_else(|| value.get("value"))).and_then(|value| value.as_str()).unwrap_or("");
                if let Some(index) = candidate_id.strip_prefix("puzzle3d.brush.candidate.").and_then(|rest| rest.parse::<usize>().ok()) {
                    envelope.runtime.brush_candidate_index = index;
                }
            }
            "addBrushObject" => {
                drive_precompute(&mut *self.precompute.borrow_mut(), &envelope);
                if let Some(payload_value) = args {
                    if let Ok(payload) = serde_json::from_value::<BrushPlacePayload>(payload_value.clone()) {
                        if let Ok(Puzzle3dEngineOutcome::Fixture(fixture)) = self.precompute.borrow_mut().dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload }) {
                            if let Some(next) = fixture_from_engine_fixture(&envelope, &fixture) {
                                envelope = next;
                                puzzle3d_rederive_all_attractions(&mut envelope.fixture);
                                resolve_puzzle3d_attractions(&mut envelope.fixture);
                            }
                        }
                    }
                }
            }
            "setFillCount" => {
                let count = args
                    .and_then(|value| value.get("count").or_else(|| value.get("value")))
                    .and_then(|value| value.as_f64())
                    .map(|value| value.round().max(0.0) as u32)
                    .unwrap_or(0)
                    .min(PUZZLE3D_FILL_COUNT_MAX);
                envelope = apply_puzzle3d_fill_count(&mut *self.precompute.borrow_mut(), envelope, count);
                ui_scope = puzzle3d_fill_build_scope();
            }
            "setBrushPlacementOverlapBudget" => {
                if let Some(value) = puzzle3d_absolute_or_delta(args, envelope.runtime.overlap_budget) {
                    envelope.runtime.overlap_budget = value.clamp(0.0, 1.0);
                    sync_precompute_session(&mut *self.precompute.borrow_mut(), &envelope);
                }
            }
            "setObjectKindWeight" | "setVortexKindWeight" => {
                let kind_id = args.and_then(|v| v.get("kindId")).and_then(|v| v.as_str()).unwrap_or("");
                let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()).unwrap_or(1.0).clamp(0.0, 1.0);
                let object_ids = puzzle3d_kind_ids(&envelope.fixture, "objects");
                let vortex_ids = puzzle3d_kind_ids(&envelope.fixture, "vortices");
                puzzle3d_ensure_catalog_kind_weights(&mut envelope.runtime.object_kind_weights, &object_ids);
                puzzle3d_ensure_catalog_kind_weights(&mut envelope.runtime.vortex_kind_weights, &vortex_ids);
                if action == "setObjectKindWeight" {
                    envelope.runtime.object_kind_weights = puzzle3d_normalize_kind_weight_group(&envelope.runtime.object_kind_weights, &object_ids, kind_id, value);
                } else if let Some(object_kind_id) = args.and_then(|v| v.get("objectKindId")).and_then(|v| v.as_str()) {
                    let object_weight = envelope.runtime.object_kind_weights.get(object_kind_id).copied().unwrap_or(0.0);
                    if object_weight > f64::EPSILON {
                        // 🎚️ Nested slider value is joint P(object)×P(vortex); convert to relative P(vortex).
                        let relative = (value / object_weight).clamp(0.0, 1.0);
                        envelope.runtime.vortex_kind_weights = puzzle3d_normalize_kind_weight_group(&envelope.runtime.vortex_kind_weights, &vortex_ids, kind_id, relative);
                    }
                    // 🚫️ Parent object weight is 0 — joint contribution is always 0; ignore vortex edits.
                } else {
                    envelope.runtime.vortex_kind_weights = puzzle3d_normalize_kind_weight_group(&envelope.runtime.vortex_kind_weights, &vortex_ids, kind_id, value);
                }
                sync_precompute_weights(&mut *self.precompute.borrow_mut(), &envelope);
                ui_scope = puzzle3d_fill_options_scope();
            }
            "cycleBrushCandidate" | "cycleBrushCandidateBack" => {
                drive_precompute(&mut *self.precompute.borrow_mut(), &envelope);
                let default_delta = if action == "cycleBrushCandidateBack" { -1 } else { 1 };
                let delta = args.and_then(|value| value.get("delta")).and_then(|value| value.as_i64()).unwrap_or(default_delta);
                if let Some(vortex_id) = puzzle3d_brush_target_vortex(&envelope) {
                    let free_count = self.precompute.borrow_mut().brush_candidates(&vortex_id).free.len();
                    if free_count > 0 {
                        let current = envelope.runtime.brush_candidate_index as i64;
                        let next = (current + delta).rem_euclid(free_count as i64);
                        envelope.runtime.brush_candidate_index = next as usize;
                    }
                } else {
                    envelope.runtime.brush_candidate_index = envelope.runtime.brush_candidate_index.saturating_add_signed(delta as isize);
                }
            }
            "openVortexSuggestions" => {
                // 💡️ One-shot suggestion popup: select the vortex and open the picker without
                // switching the host-owned utility/tool into brush mode.
                if let Some(full_id) = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()) {
                    envelope.runtime.selection.vortex_ids = SelectionSet::from(vec![full_id.to_string()]);
                    envelope.runtime.selection.object_ids.clear();
                    envelope.runtime.brush_candidate_index = 0;
                    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                    let window_id = args
                        .and_then(|value| value.get("windowId"))
                        .and_then(|value| value.as_str())
                        .filter(|id| !id.is_empty())
                        .unwrap_or(wid.as_str())
                        .to_string();
                    envelope.runtime.suggestion_menu = Some(Puzzle3dSuggestionMenu { x, y, window_id });
                    // 🧊️ Drop any stale empty/pending cache for this vortex, then refresh so the popup
                    // does not open on a previous "No placement" result while meshes/candidates are ready.
                    self.precompute.borrow_mut().invalidate_brush_target(full_id);
                    sync_precompute_session(&mut *self.precompute.borrow_mut(), &envelope);
                    self.precompute.borrow_mut().refresh_brush_candidates(full_id);
                    drive_precompute(&mut *self.precompute.borrow_mut(), &envelope);
                }
            }
            "closeVortexSuggestions" => {
                envelope.runtime.suggestion_menu = None;
                envelope.runtime.hovered_vortex_full_id = None;
            }
            "hoverSuggestion" => {
                if let Some(index) = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()) {
                    envelope.runtime.brush_candidate_index = index as usize;
                }
            }
            "acceptSuggestion" => {
                // 🧹️ Always dismiss the one-shot picker first — a failed preview/place must not leave
                // `suggestionMenu.open` gating every split pane's regular context menu.
                drive_precompute(&mut *self.precompute.borrow_mut(), &envelope);
                let index = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()).unwrap_or(envelope.runtime.brush_candidate_index as u64) as usize;
                let vortex_id = args
                    .and_then(|value| value.get("fullId"))
                    .and_then(|value| value.as_str())
                    .map(str::to_string)
                    .or_else(|| puzzle3d_brush_target_vortex(&envelope));
                envelope.runtime.suggestion_menu = None;
                envelope.runtime.hovered_vortex_full_id = None;
                if let Some(vortex_id) = vortex_id {
                    envelope.runtime.selection.vortex_ids = SelectionSet::from(vec![vortex_id.clone()]);
                    envelope.runtime.selection.object_ids.clear();
                    self.precompute.borrow_mut().refresh_brush_candidates(&vortex_id);
                    if let Some(preview) = self.precompute.borrow_mut().brush_preview(&vortex_id, index) {
                        if let Ok(Puzzle3dEngineOutcome::Fixture(fixture)) = self.precompute.borrow_mut().dispatch(Puzzle3dEngineCommand::ApplyBrushPlacement { payload: BrushPlacePayload::from(preview) }) {
                            if let Some(next) = fixture_from_engine_fixture(&envelope, &fixture) {
                                envelope = next;
                                puzzle3d_rederive_all_attractions(&mut envelope.fixture);
                                resolve_puzzle3d_attractions(&mut envelope.fixture);
                                // ✅️ One-shot place finished — leave the scene idle (no sticky vortex/hover/menu).
                                puzzle3d_clear_selection(&mut envelope.runtime.selection);
                                envelope.runtime.suggestion_menu = None;
                                envelope.runtime.hovered_vortex_full_id = None;
                            }
                        }
                    }
                }
            }
            "suggestionsTick" => {
                drive_precompute(&mut *self.precompute.borrow_mut(), &envelope);
                ui_scope = puzzle3d_suggestions_tick_scope();
            }
            "fillBuildTick" => {
                // 🪣️ No catch-up `setFillCount` dispatch here: `apply_puzzle3d_fill_count` always
                // clamps the committed count to what's available at commit time, so `fill_count` can
                // never run ahead of `applied_count` — a slider can only request what `render`'s
                // reveal-tagged instances already show. Ticks purely advance background planning.
                if puzzle3d_fill_tool_active(view_state) {
                    let available_before = self.precompute.borrow_mut().fill_available_count();
                    let done_before = self.precompute.borrow_mut().fill_is_done();
                    self.precompute.borrow_mut().precompute_step_lane(PrecomputeLane::Fill, 8);
                    let available_after = self.precompute.borrow_mut().fill_available_count();
                    let done_after = self.precompute.borrow_mut().fill_is_done();
                    ui_scope = if available_after != available_before || done_after != done_before {
                        puzzle3d_fill_build_scope()
                    } else {
                        semio_framework_core::kernel::UiDirtyScope::None
                    };
                } else {
                    ui_scope = semio_framework_core::kernel::UiDirtyScope::None;
                }
            }
            "registerBrushMesh" => {
                if let (Some(url), Some(positions), Some(indices)) =
                    (args.and_then(|v| v.get("url")).and_then(|v| v.as_str()), args.and_then(|v| v.get("positions")).and_then(|v| v.as_array()), args.and_then(|v| v.get("indices")).and_then(|v| v.as_array()))
                {
                    let positions: Vec<f32> = positions.iter().filter_map(|v| v.as_f64().map(|n| n as f32)).collect();
                    let indices: Vec<u32> = indices.iter().filter_map(|v| v.as_u64().map(|n| n as u32)).collect();
                    self.precompute.borrow_mut().register_mesh(url, &positions, &indices);
                    if let Ok(mut registry) = PUZZLE3D_MESH_REGISTRY.lock() {
                        registry.insert(url.to_string(), (positions, indices));
                    }
                }
            }
            "worldPointerDown" => {}
            _ => {}
        }
        ui_scope = match action {
            "setHover" | "worldHover" => puzzle3d_chrome_scope(),
            "setCamera" | "setProjection" | "setProjectionParam" | "focusSelection" => puzzle3d_viewport_scope(),
            "worldPick" | "worldSelect" | "setSelection" | "clearSelection" | "selectAll" | "worldVortexHover" | "worldVortexSelect" => puzzle3d_selection_scope(),
            _ => ui_scope,
        };
        if puzzle3d_chrome_action(action) {
            effects.push(puzzle3d_patch_chrome_effect(&envelope));
        }
        let next_active_utility = envelope.active_utility.clone();
        envelope.runtime.save_window(&wid);
        *self.runtime.borrow_mut() = envelope.runtime;
        let operations = if let Some(before) = before.as_ref() {
            puzzle3d_operations_from_fixture_change(before, &envelope.fixture)
        } else {
            debug_assert!(!puzzle3d_action_document_intent(action));
            Vec::new()
        };
        let coalesce_key = match action {
            "translateSelection" => Some("gumball-translate".to_string()),
            "rotateSelection" => Some("gumball-rotate".to_string()),
            "scaleSelection" => Some("gumball-scale".to_string()),
            "setFillCount" => Some("fill-count".to_string()),
            _ => None,
        };
        // 🧰️🛠️ Programmatic utility/tool switches (engagement submit/abort, suggestions, fill) push the
        // active utility/tool back into the host session; `setActiveUtility`/`setActiveTool` themselves
        // never re-emit (the host already applied them). Fill transitions go through `SetActiveTool`
        // exclusively — the window's real utility is untouched by entering/leaving the fill tool; a
        // genuine utility transition (that does not involve fill on either side) still emits
        // `SetActiveUtility` exactly as before.
        let initial_is_fill_tool = active_utility_initial == "fill";
        let next_is_fill_tool = next_active_utility == "fill";
        if next_is_fill_tool != initial_is_fill_tool {
            effects.push(HostEffect::SetActiveTool { tool_id: if next_is_fill_tool { "fill".into() } else { String::new() } });
        }
        if !next_is_fill_tool && !initial_is_fill_tool && next_active_utility != active_utility_initial {
            effects.push(HostEffect::SetActiveUtility { window_id: wid, utility_id: next_active_utility });
        }
        ActionEmit { operations, coalesce_key, effects, ui_scope, ..Default::default() }
    }

    fn render(
        &self,
        body_key: &str,
        doc: &DocumentView<'_, Puzzle3dPlayProjection>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        view_state: &ViewState,
    ) -> UiNode {
        let wid = view_state.window_id.as_deref().unwrap_or(PUZZLE3D_PLAY_WINDOW_MAIN);
        let active_utility = puzzle3d_scene_active_utility(view_state, Some(wid));
        let wid = view_state.window_id.as_deref().unwrap_or(PUZZLE3D_PLAY_WINDOW_MAIN);
        let mut runtime_for_window = self.runtime.borrow().clone();
        runtime_for_window.load_window(wid);
        // 🪣️ Additive-only: appends just the not-yet-committed fill-plan tail onto the live fixture
        // (see `puzzle3d_fixture_with_fill_display`) — safe even during a live gumball scratch drag,
        // since it never touches/replaces any already-present object (the dragged one included).
        let fill_available = self.precompute.borrow().fill_available_count();
        let fixture = puzzle3d_fixture_with_fill_display_memo(
            self.render_fixture(&doc.projection.0),
            &*self.precompute.borrow(),
            runtime_for_window.fill_count,
            fill_available,
            &self.fill_display_memo,
        );
        let envelope = Puzzle3dScene { fixture, runtime: runtime_for_window, active_utility: active_utility.clone() };
        let labels = puzzle3d_labels(view_state);
        let (instances_json, meshes_json) = self.geometry_jsons(&envelope.fixture);
        match body_key {
            PUZZLE3D_PLAY_BODY_COMPOSITE => {
                let brush_preview = world_brush_preview_json(&*self.precompute.borrow(), &envelope);
                build_world_3d_scene(
                    PUZZLE3D_PLAY_SURFACE_VIEWPORT,
                    PUZZLE3D_PLAY_APP_ID,
                    world3d_scene_extended(
                        camera_json(&envelope.runtime.camera),
                        meshes_json,
                        instances_json,
                        world_selection_json(&envelope),
                        Some(world_vortices_json(&envelope.fixture, &envelope.runtime)),
                        Some(world_attractions_json(&envelope.fixture)),
                        Some(world_target_volumes_json(&envelope.fixture)),
                        Some(world_references_json(&envelope.fixture)),
                        brush_preview,
                        Some(world_interaction_json(&envelope, &*self.precompute.borrow())),
                        None,
                        Some(world3d_lod_json(&envelope.runtime)),
                        Some(world3d_chunking_json(envelope.runtime.chunk_size, 8000.0)),
                        puzzle3d_context_menu_json(&envelope, labels),
                        Some(world3d_environment_json(&envelope.runtime.sun)),
                    ),
                )
            }
            PUZZLE3D_PLAY_BODY_DOCUMENT => UiNode::Tree(UiTreeNode {
                sections: self.document_sections_cached(&envelope.fixture, &labels),
                presence: UiPresence::default(),
                selected_ids: Some(puzzle3d_document_selected_ids(&envelope.runtime.selection)),
                highlighted_ids: None,
                selection_change: Some(puzzle3d_action("setSelection", None)),
                drop_action: None,
                menu: None,
            }),
            PUZZLE3D_PLAY_BODY_KINDS => build_kinds_tree(&envelope, labels),
            PUZZLE3D_PLAY_BODY_INSPECTOR => build_inspector_tree(&envelope, labels),
            PUZZLE3D_PLAY_BODY_SETTINGS => build_settings_body(&envelope, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_engagements(
        &self,
        doc: &DocumentView<'_, Puzzle3dPlayProjection>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        view_state: &ViewState,
    ) -> HashMap<String, WindowEngagement> {
        let labels = puzzle3d_labels(view_state);
        // 🪟️ One entry per live window INSTANCE (split top/perspective panes are two instances of the
        // same kind) — each built from ITS OWN materialized options, never the shared kind entry.
        window_instance_ids(view_state, PUZZLE3D_PLAY_WINDOW_MAIN)
            .into_iter()
            .map(|wid| {
                let active_utility = puzzle3d_scene_active_utility(view_state, Some(&wid));
                let mut runtime_for_window = self.runtime.borrow().clone();
                runtime_for_window.load_window(&wid);
                let envelope = scene_from_projection(&doc.projection.0, runtime_for_window, &active_utility);
                (wid, puzzle3d_engagement(&envelope, labels))
            })
            .collect()
    }

    fn window_measures(
        &self,
        doc: &DocumentView<'_, Puzzle3dPlayProjection>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        view_state: &ViewState,
    ) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = puzzle3d_labels(view_state);
        window_instance_ids(view_state, PUZZLE3D_PLAY_WINDOW_MAIN)
            .into_iter()
            .map(|wid| {
                let active_utility = puzzle3d_scene_active_utility(view_state, Some(&wid));
                let mut runtime_for_window = self.runtime.borrow().clone();
                runtime_for_window.load_window(&wid);
                let envelope = scene_from_projection(&doc.projection.0, runtime_for_window, &active_utility);
                (wid, puzzle3d_window_measures(&envelope, &*self.precompute.borrow(), labels))
            })
            .collect()
    }

    fn tool_measures(
        &self,
        doc: &DocumentView<'_, Puzzle3dPlayProjection>,
        _cfg: &semio_framework_plugin::ConfigView<'_, semio_framework_plugin::NoConfig>,
        view_state: &ViewState,
    ) -> HashMap<String, Vec<WindowMeasure>> {
        let wid = view_state.window_id.as_deref().unwrap_or(PUZZLE3D_PLAY_WINDOW_MAIN);
        let active_utility = puzzle3d_scene_active_utility(view_state, Some(wid));
        let labels = puzzle3d_labels(view_state);
        let wid = view_state.window_id.as_deref().unwrap_or(PUZZLE3D_PLAY_WINDOW_MAIN);
        let mut runtime_for_window = self.runtime.borrow().clone();
        runtime_for_window.load_window(wid);
        let envelope = scene_from_projection(&doc.projection.0, runtime_for_window, &active_utility);
        HashMap::from([("fill".to_string(), puzzle3d_fill_tool_measures(&envelope, &*self.precompute.borrow(), labels))])
    }

    fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
        puzzle3d_app_labels_overlay(view_state)
    }
}
//#endregion 🔖️Puzzle3dPlayApp

//#region 🔖️CommandLabels
/// 🗣️ Full chrome overlay for puzzle3d — locale × terminology (native/reuse). Empty maps used to leak English
/// manifest labels ("Edit", "Add Object", "Context Menu At") into locale-locked brand shells.
fn puzzle3d_app_labels_overlay(view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
    let labels = puzzle3d_labels(view_state);
    let is_de = is_de_locale(view_state);
    let object = labels.object;
    let objects = labels.objects;
    semio_framework_plugin::AppLabelsOverlay::default()
        .window_kind_label(PUZZLE3D_PLAY_WINDOW_MAIN, labels.window_main)
        .panel_tab_label("puzzle3d.panel.settings", if is_de { "Einstellungen" } else { "Settings" })
        .mode_label("edit", if is_de { "Bearbeiten" } else { "Edit" })
        .action_labels(puzzle3d_action_labels(view_state))
        .utility_labels(puzzle3d_utility_labels(&labels, is_de))
        .example_labels(HashMap::from([
            (PUZZLE3D_EXAMPLE_CONCRETE_FOREST.to_string(), labels.example_concrete_forest.to_string()),
            (PUZZLE3D_EXAMPLE_NAKAGIN.to_string(), "Nakagin Capsule Tower".to_string()),
        ]))
        .action_arg_label("addObjectKind.objectKind", if is_de { "Art" } else { "Kind" })
        .action_arg_label("addObjectKind.objectKind.option.Object", object)
        .action_arg_label("addObject.objectKind", if is_de { "Art" } else { "Kind" })
        .action_arg_label("addObject.objectKind.option.Object", object)
        .dialog_labels(HashMap::from([
            (
                "addObject.title".to_string(),
                if is_de { format!("{object} hinzufügen") } else { format!("Add {object}") },
            ),
            (
                "addObject.body".to_string(),
                if is_de {
                    "Wählen Sie die Art zum Hinzufügen.".into()
                } else {
                    format!("Choose the kind of {object} to add to the scene.")
                },
            ),
            ("addObject.submit".to_string(), if is_de { "Hinzufügen".to_string() } else { "Add".to_string() }),
        ]))
        .introduction_labels(HashMap::from([
            (
                "intro.title".to_string(),
                if is_de {
                    format!("Willkommen bei {}", labels.window_main)
                } else {
                    format!("Welcome to {}", labels.window_main)
                },
            ),
            (
                "intro.step.welcome.title".to_string(),
                if is_de {
                    format!("Willkommen bei {}", labels.window_main)
                } else {
                    format!("Welcome to {}", labels.window_main)
                },
            ),
            (
                "intro.step.welcome.body".to_string(),
                if is_de {
                    "Eine kurze Tour durch Ansicht, Hilfsmittel und Paneele, bevor Sie mit dem Zusammenfügen beginnen.".into()
                } else {
                    "A quick tour of the viewport, utilities, and panels before you start composing.".into()
                },
            ),
            (
                "intro.step.viewport.title".to_string(),
                if is_de { "Die 3D-Ansicht".into() } else { "The Viewport".into() },
            ),
            (
                "intro.step.viewport.body".to_string(),
                if is_de {
                    "Das ist Ihre 3D-Szene — orbitieren, verschieben und zoomen Sie, um sich umzusehen.".into()
                } else {
                    "This is your 3D scene — orbit, pan, and zoom to look around.".into()
                },
            ),
            (
                "intro.step.catalogue.title".to_string(),
                if is_de { "Der Katalog".into() } else { "The Catalogue".into() },
            ),
            (
                "intro.step.catalogue.body".to_string(),
                if is_de {
                    format!("Durchstöbern Sie hier die verfügbaren {objects}.")
                } else {
                    format!("Browse the {objects} available to place from here.")
                },
            ),
            (
                "intro.step.add-object.title".to_string(),
                if is_de { format!("{object} hinzufügen") } else { format!("Add a {object}") },
            ),
            (
                "intro.step.add-object.body".to_string(),
                if is_de {
                    "Ziehen Sie den ersten Eintrag per Drag-and-Drop aus dem Katalog in die 3D-Ansicht.".into()
                } else {
                    format!("Drag the first {object} from the catalogue into the viewport.")
                },
            ),
            (
                "intro.step.transform-utility.title".to_string(),
                if is_de {
                    format!("{objects} transformieren")
                } else {
                    format!("Transform {objects}")
                },
            ),
            (
                "intro.step.transform-utility.body".to_string(),
                if is_de {
                    format!("Aktivieren Sie das Transformieren-Hilfsmittel, um {objects} zu verschieben und zu drehen.")
                } else {
                    format!("Activate the Transform utility to move and rotate {objects} in the scene.")
                },
            ),
        ]))
        .group_label("edit", if is_de { "Bearbeiten" } else { "Edit" })
}

/// 🗣️ (action id) → localized, terminology-aware label for every operation/view/shell action in `create_puzzle3d_app`.
fn puzzle3d_action_labels(view_state: &ViewState) -> HashMap<String, String> {
    let labels = puzzle3d_labels(view_state);
    let is_de = is_de_locale(view_state);
    let object = labels.object;
    let vortex = labels.vortex;
    let pick = |en: &str, de: &str| (if is_de { de } else { en }).to_string();
    let mut map = HashMap::from([
        ("setFixtureJson".to_string(), pick("Set Fixture Json", "Rohdaten festlegen")),
        ("setActiveExample".to_string(), pick("Set Active Example", "Aktives Beispiel festlegen")),
        ("deleteSelection".to_string(), pick("Delete Selection", "Auswahl löschen")),
        ("duplicateSelection".to_string(), pick("Duplicate Selection", "Auswahl duplizieren")),
        ("setCamera".to_string(), pick("Set Camera", "Kamera festlegen")),
        ("setProjection".to_string(), pick("Set Projection", "Projektion festlegen")),
        ("setProjectionParam".to_string(), pick("Set Projection Parameter", "Projektionsparameter festlegen")),
        ("translateSelection".to_string(), pick("Translate Selection", "Auswahl verschieben")),
        ("rotateSelection".to_string(), pick("Rotate Selection", "Auswahl drehen")),
        ("scaleSelection".to_string(), pick("Scale Selection", "Auswahl skalieren")),
        ("setSelectionFlag".to_string(), pick("Set Selection Flag", "Auswahlmarkierung festlegen")),
        ("patchInspector".to_string(), pick("Patch Inspector", "Inspektor aktualisieren")),
        ("focusSelection".to_string(), pick("Focus Selection", "Auswahl fokussieren")),
        ("engagementSubmit".to_string(), pick("Engagement Submit", "Eingabe bestätigen")),
        ("engagementRepeatLast".to_string(), pick("Engagement Repeat Last", "Letzte Eingabe wiederholen")),
        ("deleteTargetVolume".to_string(), pick("Delete Target Volume", "Zielvolumen löschen")),
        ("relocateTargetVolume".to_string(), pick("Relocate Target Volume", "Zielvolumen verlagern")),
        ("setTargetVolumeFlag".to_string(), pick("Set Target Volume Flag", "Zielvolumenmarkierung festlegen")),
        ("setFillCount".to_string(), pick("Set Fill Count", "Füllanzahl festlegen")),
        ("acceptSuggestion".to_string(), pick("Accept Suggestion", "Vorschlag annehmen")),
        ("fillBuildTick".to_string(), pick("Fill Build Tick", "Füllaufbau-Takt")),
        ("setSelection".to_string(), pick("Set Selection", "Auswahl festlegen")),
        ("selectSameKindSelection".to_string(), pick("Select Same Kind", "Gleiche Art auswählen")),
        ("setJackQuery".to_string(), pick("Set Jack Query", "Abfrage festlegen")),
        ("worldSelect".to_string(), pick("World Select", "In der Welt auswählen")),
        ("worldHover".to_string(), pick("World Hover", "Überfahren (Welt)")),
        ("setHover".to_string(), pick("Set Hover", "Überfahren festlegen")),
        ("worldPick".to_string(), pick("World Pick", "Punkt in der Welt wählen")),
        ("setSelectionMethod".to_string(), pick("Set Selection Method", "Auswahlmethode festlegen")),
        ("toggleSun".to_string(), pick("Toggle Sun", "Sonne umschalten")),
        ("setSunAzimuth".to_string(), pick("Set Sun Azimuth", "Sonnenazimut festlegen")),
        ("setSunElevation".to_string(), pick("Set Sun Elevation", "Sonnenhöhe festlegen")),
        ("setSunIntensity".to_string(), pick("Set Sun Intensity", "Sonnenintensität festlegen")),
        ("setLodAutomatic".to_string(), pick("Set Lod Automatic", "Detailstufe automatisch")),
        ("setLodDepthVariable".to_string(), pick("Set Lod Depth Variable", "Detailstufen-Tiefe festlegen")),
        ("setGridVisible".to_string(), pick("Set Grid Visible", "Raster anzeigen")),
        ("setLodManual".to_string(), pick("Set Lod Manual", "Detailstufe manuell")),
        ("setGridSnapEnabled".to_string(), pick("Set Grid Snap Enabled", "Rasterfang aktivieren")),
        ("setGridSpacing".to_string(), pick("Set Grid Spacing", "Rasterabstand festlegen")),
        ("setSelectionModeDefault".to_string(), pick("Set Selection Mode Default", "Standardauswahlmodus festlegen")),
        ("setProximityRadius".to_string(), pick("Set Proximity Radius", "Näheradius festlegen")),
        ("setChunkSize".to_string(), pick("Set Chunk Size", "Blockgröße festlegen")),
        ("setSelectableKind".to_string(), pick("Set Selectable Kind", "Auswählbare Art festlegen")),
        ("setKindHover".to_string(), pick("Set Kind Hover", "Überfahren (Art) festlegen")),
        ("selectAll".to_string(), pick("Select All", "Alles auswählen")),
        ("clearSelection".to_string(), pick("Clear Selection", "Auswahl aufheben")),
        ("contextMenuAt".to_string(), pick("Open Actions Menu", "Aktionsmenü öffnen")),
        ("engagementInput".to_string(), pick("Engagement Input", "Eingabe")),
        ("engagementAbort".to_string(), pick("Engagement Abort", "Eingabe abbrechen")),
        ("engagementControlSelect".to_string(), pick("Engagement Control Select", "Eingabesteuerung auswählen")),
        ("setTransformGumballFlag".to_string(), pick("Set Transform Gumball Flag", "Transformieren-Griff festlegen")),
        ("setVoxelDims".to_string(), pick("Set Voxel Dims", "Voxel-Abmessungen festlegen")),
        ("setBrushPlacementOverlapBudget".to_string(), pick("Set Brush Placement Overlap Budget", "Pinsel-Überlappungsbudget festlegen")),
        ("cycleBrushCandidate".to_string(), pick("Cycle Brush Candidate", "Pinselkandidat wechseln")),
        ("cycleBrushCandidateBack".to_string(), pick("Cycle Brush Candidate Back", "Pinselkandidat rückwärts wechseln")),
        ("hoverSuggestion".to_string(), pick("Hover Suggestion", "Vorschlag überfahren")),
        ("suggestionsTick".to_string(), pick("Suggestions Tick", "Vorschläge-Takt")),
        ("registerBrushMesh".to_string(), pick("Register Brush Mesh", "Pinsel-Mesh registrieren")),
        ("worldPointerDown".to_string(), pick("World Pointer Down", "Welt-Zeiger gedrückt")),
        ("transformEnd".to_string(), pick("Transform End", "Transformieren beenden")),
        ("transformBegin".to_string(), pick("Transform Begin", "Transformieren beginnen")),
    ]);
    map.insert(
        "addObjectKind".into(),
        if is_de { format!("{object} hinzufügen") } else { format!("Add {object}") },
    );
    map.insert(
        "openAddObjectDialog".into(),
        if is_de { format!("{object} hinzufügen…") } else { format!("Add {object}…") },
    );
    map.insert(
        "worldRelocate".into(),
        if is_de { format!("{object} verlagern") } else { format!("Relocate {object}") },
    );
    map.insert(
        "addBrushObject".into(),
        if is_de { format!("Pinsel-{object} hinzufügen") } else { format!("Add Brush {object}") },
    );
    map.insert(
        "createAttraction".into(),
        if is_de { format!("{} erstellen", labels.attraction) } else { format!("Create {}", labels.attraction) },
    );
    map.insert(
        "deleteAttraction".into(),
        if is_de { format!("{} löschen", labels.attraction) } else { format!("Delete {}", labels.attraction) },
    );
    map.insert(
        "addTargetVolume".into(),
        if is_de { format!("{} hinzufügen", labels.target_volume) } else { format!("Add {}", labels.target_volume) },
    );
    map.insert(
        "worldVortexHover".into(),
        if is_de { format!("Überfahren ({vortex})") } else { format!("World {vortex} Hover") },
    );
    map.insert(
        "worldVortexSelect".into(),
        if is_de { format!("{vortex} in der Welt auswählen") } else { format!("World {vortex} Select") },
    );
    map.insert(
        "setVortexShow".into(),
        if is_de { format!("{} festlegen", labels.vortex_show) } else { format!("Set {}", labels.vortex_show) },
    );
    map.insert(
        "setVortexDirection".into(),
        if is_de { format!("{} festlegen", labels.vortex_direction) } else { format!("Set {}", labels.vortex_direction) },
    );
    map.insert(
        "setObjectKindWeight".into(),
        if is_de { format!("{object}-Art-Gewicht festlegen") } else { format!("Set {object} Kind Weight") },
    );
    map.insert(
        "setVortexKindWeight".into(),
        if is_de { format!("{vortex}-Art-Gewicht festlegen") } else { format!("Set {vortex} Kind Weight") },
    );
    map.insert(
        "openVortexSuggestions".into(),
        if is_de { format!("{vortex}-Vorschläge öffnen") } else { format!("Open {vortex} Suggestions") },
    );
    map.insert(
        "closeVortexSuggestions".into(),
        if is_de { format!("{vortex}-Vorschläge schließen") } else { format!("Close {vortex} Suggestions") },
    );
    map
}

/// 🗣️ (utility id) → localized utility bar button label for every `.utility(...)` / `.tool_simple(...)` in `create_puzzle3d_app`.
fn puzzle3d_utility_labels(labels: &Puzzle3dLabels, is_de: bool) -> HashMap<String, String> {
    HashMap::from([
        ("select".to_string(), labels.select.to_string()),
        ("transform".to_string(), (if is_de { "Transformieren" } else { "Transform" }).to_string()),
        ("brush".to_string(), labels.brush.to_string()),
        ("volumeBrush".to_string(), labels.volume_brush.to_string()),
        ("fill".to_string(), labels.fill.to_string()),
        ("worldRelocate".to_string(), (if is_de { "Verlagern" } else { "Relocate" }).to_string()),
    ])
}
//#endregion 🔖️CommandLabels

//#region 🔖️DefaultLayout
/// 🪟️ Top (left ⅓) + Perspective (right ⅔) — the default dual-pane workbench for Puzzle 3D and the Aggregator.
fn puzzle3d_default_layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![
                WindowLayoutChild::Stack(WindowLayoutStackNode {
                    kind: "stack".into(),
                    size: Some(100.0 / 3.0),
                    active_window_kind_id: None,
                    children: vec![create_window_layout(
                        PUZZLE3D_PLAY_WINDOW_MAIN,
                        Some("Top".into()),
                        Some(PUZZLE3D_PLAY_WINDOW_TOP.into()),
                        Some(PUZZLE3D_TEMPLATE_TOP.into()),
                    )],
                }),
                WindowLayoutChild::Stack(WindowLayoutStackNode {
                    kind: "stack".into(),
                    size: Some(200.0 / 3.0),
                    active_window_kind_id: None,
                    children: vec![create_window_layout(
                        PUZZLE3D_PLAY_WINDOW_MAIN,
                        Some("Perspective".into()),
                        Some(PUZZLE3D_PLAY_WINDOW_PERSPECTIVE.into()),
                        Some(PUZZLE3D_TEMPLATE_PERSPECTIVE.into()),
                    )],
                }),
            ],
        }),
    }
}
//#endregion 🔖️DefaultLayout

//#region 🔖️Manifest
pub fn create_puzzle3d_app() -> App {
    let envelope = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
    App::from_builder(
        App::builder(PUZZLE3D_PLAY_APP_ID, "Puzzle 3D")
            .document(["semio", "puzzle", "3d"])
            .artifact_kind(ArtifactKindSpec {
                id: "3d.puzzle".into(),
                name: "3D Puzzle".into(),
                source_format: "puzzle.3d".into(),
                component_kind: "puzzle3d".into(),
                dimension: "3d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Design },
                schema: "puzzle.3d".into(),
                export_formats: vec![OsMediaFormat::Glb, OsMediaFormat::Obj, OsMediaFormat::Stl],
                import_formats: vec![OsMediaFormat::Glb, OsMediaFormat::Obj],
            })
            .icon_id("puzzle")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "Aggregator"])
            .mode("edit", "Edit", "square-pen")
            .default_mode_id("edit")
            .window_kind_with_engagement(PUZZLE3D_PLAY_WINDOW_MAIN, "Puzzle 3D", PUZZLE3D_PLAY_BODY_COMPOSITE, SurfaceKind::World3d, puzzle3d_engagement(&envelope, &PUZZLE3D_LABELS_NATIVE_EN), "puzzle")
            .default_layout(puzzle3d_default_layout())
            .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, PanelGroup::Workbench, PUZZLE3D_PLAY_BODY_DOCUMENT)
            .panel_tab(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, PanelGroup::Workbench, PUZZLE3D_PLAY_BODY_KINDS)
            .panel_tab(FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, PanelGroup::Details, PUZZLE3D_PLAY_BODY_INSPECTOR)
            .panel_tab("puzzle3d.panel.settings", "Settings", PanelGroup::Settings, PUZZLE3D_PLAY_BODY_SETTINGS)
            .keybinding("mod+a", "selectAll")
            .keybinding("escape", "engagementAbort")
            .keybinding("delete", "deleteSelection")
            .keybinding("backspace", "deleteSelection")
            .keybinding("mod+d", "duplicateSelection")
            .keybinding("tab", "cycleBrushCandidate")
            .keybinding("shift+tab", "cycleBrushCandidateBack")
            .keybinding("f", "focusSelection")
            // 🔧️ Document-mutating operations (emit VCS operations through the before/after fixture delta).
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setFixtureJson", "Set Fixture Json", ActionKind::Operation) })
            .operation("setActiveExample", "Set Active Example")
            .operation("addObjectKind", "Add Object")
            .operation("deleteSelection", "Delete Selection")
            .operation("duplicateSelection", "Duplicate Selection")
            .operation("translateSelection", "Translate Selection")
            .operation("rotateSelection", "Rotate Selection")
            .operation("scaleSelection", "Scale Selection")
            .operation("transformEnd", "Transform End")
            .operation("worldRelocate", "Relocate Object")
            .operation("setSelectionFlag", "Set Selection Flag")
            .operation("patchInspector", "Patch Inspector")
            .operation("engagementSubmit", "Engagement Submit")
            .operation("engagementRepeatLast", "Engagement Repeat Last")
            .operation("createAttraction", "Create Attraction")
            .operation("deleteAttraction", "Delete Attraction")
            .operation("addTargetVolume", "Add Target Volume")
            .operation("deleteTargetVolume", "Delete Target Volume")
            .operation("setTargetVolumeFlag", "Set Target Volume Flag")
            .operation("addBrushObject", "Add Brush Object")
            .operation("setFillCount", "Set Fill Count")
            .operation("acceptSuggestion", "Accept Suggestion")
            // 🗨️ Shell-only effect (no document mutation): opens the "addObject" dialog.
            .shell_action("openAddObjectDialog", "Add Object…")
            // 👁️ Ephemeral view state — selection, hover, camera scratch, utility-parameter runtime.
            .view_action("setCamera", "Set Camera")
            .view_action("setProjection", "Set Projection")
            .view_action("setProjectionParam", "Set Projection Parameter")
            .view_action("focusSelection", "Focus Selection")
            .view_action("setSelection", "Set Selection")
            .view_action("selectSameKindSelection", "Select Same Kind")
            .view_action("setJackQuery", "Set Jack Query")
            .view_action("worldSelect", "World Select")
            .view_action("worldHover", "World Hover")
            .view_action("setHover", "Set Hover")
            .view_action("worldPick", "World Pick")
            .view_action("worldVortexHover", "World Vortex Hover")
            .view_action("worldVortexSelect", "World Vortex Select")
            .view_action("setSelectionMethod", "Set Selection Method")
            .view_action("setVortexShow", "Set Vortex Show")
            .view_action("setVortexDirection", "Set Vortex Direction")
            .view_action("toggleSun", "Toggle Sun")
            .view_action("setSunAzimuth", "Set Sun Azimuth")
            .view_action("setSunElevation", "Set Sun Elevation")
            .view_action("setSunIntensity", "Set Sun Intensity")
            .view_action("setLodAutomatic", "Set Lod Automatic")
            .view_action("setLodDepthVariable", "Set Lod Depth Variable")
            .view_action("setGridVisible", "Set Grid Visible")
            .view_action("setLodManual", "Set Lod Manual")
            .view_action("setGridSnapEnabled", "Set Grid Snap Enabled")
            .view_action("setGridSpacing", "Set Grid Spacing")
            .view_action("setSelectionModeDefault", "Set Selection Mode Default")
            .view_action("setProximityRadius", "Set Proximity Radius")
            .view_action("setChunkSize", "Set Chunk Size")
            .view_action("setSelectableKind", "Set Selectable Kind")
            .view_action("setKindHover", "Set Kind Hover")
            .view_action("selectAll", "Select All")
            .view_action("clearSelection", "Clear Selection")
            .view_action("contextMenuAt", "Open Actions Menu")
            .view_action("engagementInput", "Engagement Input")
            .view_action("engagementAbort", "Engagement Abort")
            .view_action("engagementControlSelect", "Engagement Control Select")
            .view_action("setTransformGumballFlag", "Set Transform Gumball Flag")
            .view_action("transformBegin", "Transform Begin")
            .view_action("setVoxelDims", "Set Voxel Dims")
            .view_action("relocateTargetVolume", "Relocate Target Volume")
            .view_action("setBrushPlacementOverlapBudget", "Set Brush Placement Overlap Budget")
            .view_action("setObjectKindWeight", "Set Object Kind Weight")
            .view_action("setVortexKindWeight", "Set Vortex Kind Weight")
            .view_action("cycleBrushCandidate", "Cycle Brush Candidate")
            .view_action("cycleBrushCandidateBack", "Cycle Brush Candidate Back")
            .view_action("openVortexSuggestions", "Open Vortex Suggestions")
            .view_action("closeVortexSuggestions", "Close Vortex Suggestions")
            .view_action("hoverSuggestion", "Hover Suggestion")
            .view_action("suggestionsTick", "Suggestions Tick")
            .view_action("fillBuildTick", "Fill Build Tick")
            .view_action("registerBrushMesh", "Register Brush Mesh")
            .view_action("worldPointerDown", "World Pointer Down")
            // 📝️ Staged argument forms for the panel-visible create/query actions (P1).
            .action_args("addObjectKind", vec![
                ActionArgDef::select("objectKind", "Kind", vec![ActionArgOption::new("Object", "Object")]).default_value("Object"),
            ])
            // 🧰️ Flat per-window set of utilities (host-owned `view_state.active_utility_id`); no utility is active until the host presses one — the transform gumball exposes translate and rotate together via Move/Rotate flags.
            .utility(UtilityDefinition::new("transform", "Transform", "transform-3d"))
            .utility(UtilityDefinition::new("brush", "Brush", "paintbrush"))
            .utility(UtilityDefinition::new("volumeBrush", "Volume Brush", "volume-brush"))
            .utility(UtilityDefinition::new("worldRelocate", "Relocate", "relocate-3d"))
            .window_kind_utilities(PUZZLE3D_PLAY_WINDOW_MAIN, vec!["transform".into(), "brush".into(), "volumeBrush".into(), "worldRelocate".into()])
            // 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility — it keeps
            // its viewport interaction via `ViewState.active_tool_id` (see `puzzle3d_scene_active_utility`).
            .tool_simple("fill", "Fill", "paint-bucket")
            .mode_tools("edit", vec![ToolRef::new("fill")])
            // 🎓️ Reference introduction (proof of the framework's Introduction mechanism, see
            // `IntroductionDefinition` in `framework/core/rs/lib.rs`): a short first-run walkthrough
            // of the viewport, the catalogue panel, adding an object, and the Move utility.
            .introduction(IntroductionDefinition {
                title: "Welcome to Puzzle 3D".into(),
                steps: vec![
                    IntroductionStepDefinition::new(
                        "welcome",
                        "Welcome to Puzzle 3D",
                        "A quick tour of the viewport, utilities, and panels before you start composing.",
                    ),
                    IntroductionStepDefinition::new("viewport", "The Viewport", "This is your 3D scene — orbit, pan, and zoom to look around.")
                        .introduce(window_element_id(PUZZLE3D_PLAY_WINDOW_MAIN))
                        .interact(vec![
                            IntroductionInteraction::zoom(PUZZLE3D_PLAY_WINDOW_MAIN, "Zoom"),
                            IntroductionInteraction::pan(PUZZLE3D_PLAY_WINDOW_MAIN, "Pan"),
                            IntroductionInteraction::orbit(PUZZLE3D_PLAY_WINDOW_MAIN, "Orbit"),
                        ]),
                    IntroductionStepDefinition::new("catalogue", "The Catalogue", "Browse the object kinds available to place from here.")
                        .introduce(panel_tab_element_id(FRAMEWORK_PANEL_TAB_CATALOGUE_ID))
                        .placement(IntroductionPlacement::Right),
                    IntroductionStepDefinition::new("add-object", "Add an Object", "Drag the first object kind from the catalogue into the viewport.")
                        .introduce(panel_tab_first_draggable_element_id(FRAMEWORK_PANEL_TAB_CATALOGUE_ID))
                        .show(vec![panel_tab_element_id(FRAMEWORK_PANEL_TAB_CATALOGUE_ID), window_element_id(PUZZLE3D_PLAY_WINDOW_MAIN)])
                        .placement(IntroductionPlacement::Right)
                        .interact(vec![IntroductionInteraction::action("addObjectKind", "Add an object")]),
                    IntroductionStepDefinition::new("transform-utility", "Transform Objects", "Activate the Transform utility to move and rotate objects in the scene.")
                        .introduce("transform")
                        .show(vec![window_element_id(PUZZLE3D_PLAY_WINDOW_MAIN)])
                        .interact(vec![IntroductionInteraction::utility("transform", "Activate Transform")]),
                ],
            })
            // 🗨️ Reference dialog (proof of the framework's Dialog mechanism, see `DialogDefinition`
            // in `framework/core/rs/lib.rs`): opened by `openAddObjectDialog`, drives the existing
            // `addObjectKind` operation's `objectKind` select arg.
            .dialog(
                DialogDefinition::new("addObject", "Add Object", ActionRef::new("addObjectKind"))
                    .body("Choose the kind of object to add to the scene.")
                    .args(vec![
                        ActionArgDef::select("objectKind", "Kind", vec![ActionArgOption::new("Object", "Object")]).default_value("Object").required(),
                    ])
                    .submit_label("Add"),
            ),
    )
    .example(PUZZLE3D_EXAMPLE_CONCRETE_FOREST, "Concrete Forest", CONCRETE_FOREST_EXAMPLE_JSON.clone(), "trees")
    .example(PUZZLE3D_EXAMPLE_NAKAGIN, "Nakagin Capsule Tower", NAKAGIN_EXAMPLE_JSON.clone(), "building-2")
    .workflow("puzzle3d", "Puzzle 3D", "model")
}

/// 🗃️ Real GLB geometry the browser round-tripped via `registerBrushMesh` this session, keyed by mesh url; falls back to a box for anything not yet loaded. `fn` pointers can't capture state, so this backs the export handler's plain-function-pointer signature.
static PUZZLE3D_MESH_REGISTRY: LazyLock<Mutex<HashMap<String, (Vec<f32>, Vec<u32>)>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
/// 🌀️ Undoes glTF's Y-up convention to land in this world's Z-up frame — mirrors `GLB_MESH_FRAME_ROTATION_X` (a fixed +90° turn about X) from `@semio-tech/infinite-world-r3f`, which the viewer applies visually but which raw `registerBrushMesh` vertices never carry.
fn glb_frame_correct(position: [f32; 3]) -> [f32; 3] {
    [position[0], -position[2], position[1]]
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn quat_rotate_point(point: [f32; 3], quat: [f64; 4]) -> [f32; 3] {
    let [qx, qy, qz, qw] = quat;
    let (x, y, z) = (point[0] as f64, point[1] as f64, point[2] as f64);
    let (cx, cy, cz) = (qy * z - qz * y, qz * x - qx * z, qx * y - qy * x);
    let (tx, ty, tz) = (2.0 * cx, 2.0 * cy, 2.0 * cz);
    let (ux, uy, uz) = (qy * tz - qz * ty, qz * tx - qx * tz, qx * ty - qy * tx);
    [(x + qw * tx + ux) as f32, (y + qw * ty + uy) as f32, (z + qw * tz + uz) as f32]
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
/// 💾️ Bakes each object's world transform (GLB frame correction, then scale/orientation/origin) into a single merged mesh for OBJ/GLB export; objects whose GLB hasn't round-tripped through `registerBrushMesh` this session fall back to a box.
fn puzzle3d_mesh_from_document(doc: &Value) -> Result<semio_framework_plugin::MeshData, String> {
    let fixture: Puzzle3dFixture = serde_json::from_value(doc.clone()).map_err(|error| error.to_string())?;
    let registry = PUZZLE3D_MESH_REGISTRY.lock().map_err(|_| "puzzle3d mesh registry poisoned".to_string())?;
    let fallback = mesh_from_kind(PUZZLE3D_FALLBACK_MESH_KIND);
    let mut merged = semio_framework_plugin::MeshData::default();
    for object in fixture.objects.iter().filter(|object| !object.hidden) {
        let mesh_url = resolve_object_mesh_url(object, &fixture.meta);
        let (positions, indices): (&[f32], &[u32]) = match mesh_url.as_deref().and_then(|url| registry.get(url)) {
            Some((positions, indices)) => (positions, indices),
            None => (&fallback.positions, &fallback.indices),
        };
        let orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let scale = object_scale_json(object);
        let index_offset = (merged.positions.len() / 3) as u32;
        for chunk in positions.chunks_exact(3) {
            let corrected = glb_frame_correct([chunk[0], chunk[1], chunk[2]]);
            let scaled = [corrected[0] * scale[0] as f32, corrected[1] * scale[1] as f32, corrected[2] * scale[2] as f32];
            let rotated = quat_rotate_point(scaled, orientation);
            merged.positions.push(rotated[0] + object.origin[0] as f32);
            merged.positions.push(rotated[1] + object.origin[1] as f32);
            merged.positions.push(rotated[2] + object.origin[2] as f32);
        }
        merged.indices.extend(indices.iter().map(|index| index + index_offset));
    }
    if merged.positions.is_empty() {
        return Ok(fallback);
    }
    merged.compute_normals();
    Ok(merged)
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
/// 📥️ Tier C DWG mesh import — always returns the empty puzzle-3d fixture; never errors on a structurally valid mesh.
fn puzzle3d_document_from_mesh(_mesh: &semio_framework_plugin::MeshData) -> Result<Value, String> {
    serde_json::to_value(empty_fixture()).map_err(|error| error.to_string())
}

pub fn register_puzzle3d_exports() {
    // 🗂️ Registers `Puzzle3dPlayProjection`'s pack<->dsl codec under its real `document_schema()`
    // string so `framework/sync`'s `FolderEndpoint::Pack` can print/parse puzzle-3d play documents
    // without depending on this crate's concrete `Projection`/`Operation` types.
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Puzzle3dPlayApp>(PUZZLE3D_FIXTURE_SCHEMA);
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    {
        semio_framework_os::register_mesh_exporter("3d.puzzle", "puzzle", puzzle3d_mesh_from_document, Box::new(semio_framework_plugin::ObjExporter));
        semio_framework_os::register_mesh_exporter("3d.puzzle", "puzzle", puzzle3d_mesh_from_document, Box::new(semio_framework_plugin::GlbExporter));
        semio_framework_os::register_mesh_exporter("3d.puzzle", "puzzle", puzzle3d_mesh_from_document, Box::new(semio_framework_plugin::StlExporter));
        semio_framework_os::register_mesh_importer("3d.puzzle", puzzle3d_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
        semio_framework_os::register_mesh_importer("3d.puzzle", puzzle3d_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
        semio_framework_os::register_mesh_importer("3d.puzzle", puzzle3d_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
        semio_framework_os::register_mesh_dwg_export_handler("3d.puzzle", "puzzle", puzzle3d_mesh_from_document);
        semio_framework_os::register_mesh_dwg_import_handler("3d.puzzle", puzzle3d_document_from_mesh);
    }
}
//#endregion 🔖️Manifest

//#region 🔖️WasmBridge
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
mod wasm_bridge {
    use super::*;
    use puzzle_3d::PUZZLE_3D_SCHEMA;
    use puzzle_3d_engine::empty_puzzle3d_projection;
    use puzzle_3d_op::{Puzzle3dEnvelope, Puzzle3dStore};
    use std::cell::RefCell;
    use store::create_document_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Puzzle3dDocumentVcs {
        store: RefCell<Puzzle3dStore>,
    }

    #[wasm_bindgen]
    impl Puzzle3dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Puzzle3dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Puzzle3dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Puzzle3dStore::new(envelope)
                }
                None => Puzzle3dStore::new(create_document_envelope(PUZZLE_3D_SCHEMA, "puzzle3d", empty_puzzle3d_projection(), None)),
            };
            Ok(Self { store: RefCell::new(store) })
        }

        #[wasm_bindgen(js_name = dispatchText)]
        pub fn dispatch_text(&self, command_text: &str) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_text(command_text).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = dispatchBinary)]
        pub fn dispatch_binary(&self, command_bytes: &[u8]) -> Result<(), JsValue> {
            self.store.borrow_mut().dispatch_binary(command_bytes).map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = projectionJson)]
        pub fn projection_json(&self) -> Result<String, JsValue> {
            self.store.borrow().projection_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = envelopeJson)]
        pub fn envelope_json(&self) -> Result<String, JsValue> {
            self.store.borrow().envelope_json().map_err(|e| JsValue::from_str(&e.to_string()))
        }

        #[wasm_bindgen(js_name = generation)]
        pub fn generation(&self) -> u32 {
            self.store.borrow().generation() as u32
        }
    }

    /// 🔤️ Parses `.puzzle3d` DSL text (`Puzzle3dProjection`'s `dsl::DslDocument` grammar) into the same
    /// camelCase JSON shape callers previously got from a hand-authored `*.3d.json` fixture — lets
    /// non-Rust consumers (e.g. Storybook stories) load the real example fixtures without duplicating
    /// the DSL grammar. Moved here from `puzzle_3d_engine` (`HEADLESS-ENGINE-LAW-AND-OFFENDER-FIXES`):
    /// the engine (constitutional: engine) slot must not depend on `wasm-bindgen`, and this UI slot is
    /// where every other wasm-bindgen-exported puzzle-3d surface already lives.
    #[wasm_bindgen(js_name = puzzle3dParseDslJson)]
    pub fn puzzle3d_parse_dsl_json(dsl_text: &str) -> Result<String, JsValue> {
        use store::DocumentDsl;
        let projection = Puzzle3dProjection::parse_dsl(dsl_text).map_err(|error| JsValue::from_str(&error.to_string()))?;
        serde_json::to_string(&projection).map_err(|error| JsValue::from_str(&error.to_string()))
    }
}
//#endregion 🔖️WasmBridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp, FRAMEWORK_HISTORY_BODY_KEY};

    fn new_app_with_registry() -> VcsDocumentApp<Puzzle3dPlayApp> {
        testkit::new_app_with_registry::<Puzzle3dPlayApp>(create_puzzle3d_app)
    }

    fn object_count(app: &VcsDocumentApp<Puzzle3dPlayApp>) -> usize {
        app.projection().expect("projection").0.get("objects").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0)
    }

    #[test]
    fn renders_world_scene() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let node = app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render");
        assert!(serde_json::to_string(&node).unwrap().contains("world-3d"));
    }

    #[test]
    fn initial_projection_is_the_concrete_forest_fixture() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        assert_eq!(app.projection().expect("projection").0.get("schema").and_then(|value| value.as_str()), Some(PUZZLE3D_FIXTURE_SCHEMA));
        assert!(object_count(&app) > 0, "the concrete-forest default fixture ships with objects");
    }

    /// 📦️ `Puzzle3dPlayProjection`'s pack encoding round-trips through the same `(RecordSpec,
    /// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
    /// `serde_json::Value` bridge impls), reusing the default concrete-forest fixture the test
    /// above already loads.
    #[test]
    fn puzzle3d_play_projection_pack_round_trips() {
        let app = testkit::new_app::<Puzzle3dPlayApp>();
        store::test_support::assert_dsl_pack_equivalence(&app.projection().expect("projection"));
    }

    #[test]
    fn open_add_object_dialog_emits_the_open_dialog_effect_with_no_document_change() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let before = object_count(&app);
        let result = app.handle_action("openAddObjectDialog", None, &ViewState::default(), &testkit::meta("local")).expect("openAddObjectDialog");
        assert!(
            matches!(result.requested_effects.as_slice(), [HostEffect::OpenDialog { dialog_id, args }] if dialog_id == "addObject" && args.is_none()),
            "expected a single OpenDialog effect for the addObject dialog, got {:?}",
            result.requested_effects,
        );
        assert_eq!(object_count(&app), before, "opening the dialog does not mutate the document");
    }

    #[test]
    fn set_active_example_swaps_the_document_and_undo_restores_it() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let loaded = object_count(&app);
        assert!(loaded > 0);
        app.handle_action("setActiveExample", Some(&json!({ "exampleId": "" })), &ViewState::default(), &testkit::meta("local")).expect("empty");
        assert_eq!(object_count(&app), 0, "empty example clears the objects");
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("local")).expect("undo");
        assert_eq!(object_count(&app), loaded, "undo restores the concrete-forest objects");
        app.handle_action("redo", None, &ViewState::default(), &testkit::meta("local")).expect("redo");
        assert_eq!(object_count(&app), 0);
    }

    #[test]
    fn nakagin_example_loads_via_operations() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        app.handle_action("setActiveExample", Some(&json!({ "exampleId": PUZZLE3D_EXAMPLE_NAKAGIN })), &ViewState::default(), &testkit::meta("local")).expect("nakagin");
        let projection = app.projection().expect("projection").0;
        assert_eq!(projection.get("schema").and_then(|value| value.as_str()), Some(PUZZLE3D_FIXTURE_SCHEMA));
        assert!(projection.get("objects").and_then(|value| value.as_array()).is_some_and(|objects| !objects.is_empty()));
    }

    #[test]
    fn document_and_inspector_panels_render() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        for body in [PUZZLE3D_PLAY_BODY_DOCUMENT, PUZZLE3D_PLAY_BODY_KINDS, PUZZLE3D_PLAY_BODY_INSPECTOR] {
            let node = app.render(body, None, &ViewState::default()).expect("render");
            assert!(!serde_json::to_string(&node).unwrap().is_empty());
        }
    }

    #[test]
    fn selected_object_inspector_nests_origin_into_x_y_z_steppers() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let object_id = app.projection().expect("projection").0.get("objects").and_then(|value| value.as_array()).and_then(|objects| objects.first()).and_then(|object| object.get("id")).and_then(|value| value.as_str()).expect("first object id").to_string();
        app.handle_action("worldSelect", Some(&json!({ "ids": [object_id], "merge": "replace" })), &ViewState::default(), &testkit::meta("local")).expect("worldSelect");
        let node = app.render(PUZZLE3D_PLAY_BODY_INSPECTOR, None, &ViewState::default()).expect("render");
        let json = serde_json::to_value(&node).unwrap();
        let origin_item = json
            .get("sections")
            .and_then(|value| value.as_array())
            .and_then(|sections| sections.first())
            .and_then(|section| section.get("items"))
            .and_then(|value| value.as_array())
            .and_then(|items| items.iter().find(|item| item.get("id").and_then(|value| value.as_str()) == Some("puzzle3d-play-inspector.object.origin")))
            .expect("Origin tree item");
        let axis_ids: Vec<String> = origin_item
            .get("items")
            .and_then(|value| value.as_array())
            .expect("Origin has nested axis items")
            .iter()
            .map(|item| item.get("control").and_then(|control| control.get("id")).and_then(|value| value.as_str()).unwrap_or_default().to_string())
            .collect();
        assert_eq!(axis_ids, vec!["puzzle3d-play-inspector.object.origin.x", "puzzle3d-play-inspector.object.origin.y", "puzzle3d-play-inspector.object.origin.z"]);
        for item in origin_item.get("items").and_then(|value| value.as_array()).unwrap() {
            assert_eq!(item.get("control").and_then(|control| control.get("type")).and_then(|value| value.as_str()), Some("numberStepper"));
        }
    }

    #[test]
    fn patch_inspector_origin_axis_sets_absolute_value_and_preserves_other_axes() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let object_id = app.projection().expect("projection").0.get("objects").and_then(|value| value.as_array()).and_then(|objects| objects.first()).and_then(|object| object.get("id")).and_then(|value| value.as_str()).expect("first object id").to_string();
        let before_y = app.projection().expect("projection").0.get("objects").and_then(|value| value.as_array()).and_then(|objects| objects.first()).and_then(|object| object.get("origin")).and_then(|value| value.as_array()).and_then(|origin| origin.get(1)).and_then(|value| value.as_f64()).expect("origin.y");
        app.handle_action(
            "patchInspector",
            Some(&json!({ "entity": "object", "ids": [object_id.clone()], "field": "origin.x", "value": 42.5 })),
            &ViewState::default(),
            &testkit::meta("local"),
        )
        .expect("patchInspector");
        let projection = app.projection().expect("projection").0;
        let objects = projection.get("objects").and_then(|value| value.as_array()).expect("objects");
        let object = objects.iter().find(|object| object.get("id").and_then(|value| value.as_str()) == Some(object_id.as_str())).expect("patched object");
        let origin = object.get("origin").and_then(|value| value.as_array()).expect("origin");
        assert_eq!(origin[0].as_f64(), Some(42.5), "origin.x should be set to the absolute value");
        assert_eq!(origin[1].as_f64(), Some(before_y), "origin.y should be untouched by an origin.x edit");
    }

    #[test]
    fn patch_inspector_origin_axis_delta_offsets_each_selected_object_from_its_own_current_value() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let id_a = app.projection().expect("projection").0.get("objects").and_then(|value| value.as_array()).and_then(|objects| objects.first()).and_then(|object| object.get("id")).and_then(|value| value.as_str()).expect("first object id").to_string();
        app.handle_action("addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [10.0, 0.0, 0.0] })), &ViewState::default(), &testkit::meta("local")).expect("addObjectKind");
        let id_b = app.projection().expect("projection").0.get("objects").and_then(|value| value.as_array()).and_then(|objects| objects.last()).and_then(|object| object.get("id")).and_then(|value| value.as_str()).expect("added object id").to_string();
        assert_ne!(id_a, id_b, "the added object must be distinct from the first fixture object");
        let objects = app.projection().expect("projection").0.get("objects").and_then(|value| value.as_array()).cloned().unwrap_or_default();
        let x_a_before = objects.iter().find(|object| object.get("id").and_then(|value| value.as_str()) == Some(id_a.as_str())).and_then(|object| object.get("origin")).and_then(|value| value.as_array()).and_then(|origin| origin.first()).and_then(|value| value.as_f64()).unwrap();
        let x_b_before = objects.iter().find(|object| object.get("id").and_then(|value| value.as_str()) == Some(id_b.as_str())).and_then(|object| object.get("origin")).and_then(|value| value.as_array()).and_then(|origin| origin.first()).and_then(|value| value.as_f64()).unwrap();
        assert_ne!(x_a_before, x_b_before, "the two objects must start at different x values for this test to prove per-object offset preservation");
        app.handle_action(
            "patchInspector",
            Some(&json!({ "entity": "object", "ids": [id_a.clone(), id_b.clone()], "field": "origin.x", "delta": 3.0 })),
            &ViewState::default(),
            &testkit::meta("local"),
        )
        .expect("patchInspector");
        let projection = app.projection().expect("projection").0;
        let objects = projection.get("objects").and_then(|value| value.as_array()).expect("objects");
        let x_a_after = objects.iter().find(|object| object.get("id").and_then(|value| value.as_str()) == Some(id_a.as_str())).and_then(|object| object.get("origin")).and_then(|value| value.as_array()).and_then(|origin| origin.first()).and_then(|value| value.as_f64()).unwrap();
        let x_b_after = objects.iter().find(|object| object.get("id").and_then(|value| value.as_str()) == Some(id_b.as_str())).and_then(|object| object.get("origin")).and_then(|value| value.as_array()).and_then(|origin| origin.first()).and_then(|value| value.as_f64()).unwrap();
        assert_eq!(x_a_after, x_a_before + 3.0, "a delta edit adds to each object's own current x");
        assert_eq!(x_b_after, x_b_before + 3.0, "a delta edit preserves each object's own starting offset");
    }

    #[test]
    fn app_definition_has_the_main_world_window() {
        let app = create_puzzle3d_app();
        assert!(app.definition.window_kinds.iter().any(|window| window.id == PUZZLE3D_PLAY_WINDOW_MAIN));
    }

    #[test]
    fn default_layout_is_top_left_third_and_perspective_right_two_thirds() {
        let app = create_puzzle3d_app();
        let layout = app.definition.default_layout.as_ref().expect("default layout");
        let WindowLayoutRoot::Axis(root) = &layout.root else {
            panic!("default layout root must be a row axis");
        };
        assert_eq!(root.kind, "row");
        assert_eq!(root.children.len(), 2);
        let WindowLayoutChild::Stack(top) = &root.children[0] else {
            panic!("left pane must be a stack");
        };
        let WindowLayoutChild::Stack(perspective) = &root.children[1] else {
            panic!("right pane must be a stack");
        };
        assert!((top.size.unwrap() - 100.0 / 3.0).abs() < 1e-9);
        assert!((perspective.size.unwrap() - 200.0 / 3.0).abs() < 1e-9);
        let top_window = &top.children[0];
        let perspective_window = &perspective.children[0];
        assert_eq!(top_window.window_kind_id, PUZZLE3D_PLAY_WINDOW_MAIN);
        assert_eq!(perspective_window.window_kind_id, PUZZLE3D_PLAY_WINDOW_MAIN);
        assert_eq!(top_window.instance_id.as_deref(), Some(PUZZLE3D_PLAY_WINDOW_TOP));
        assert_eq!(perspective_window.instance_id.as_deref(), Some(PUZZLE3D_PLAY_WINDOW_PERSPECTIVE));
        assert_eq!(top_window.title.as_deref(), Some("Top"));
        assert_eq!(perspective_window.title.as_deref(), Some("Perspective"));
        assert_eq!(top_window.template_id.as_deref(), Some(PUZZLE3D_TEMPLATE_TOP));
        assert_eq!(perspective_window.template_id.as_deref(), Some(PUZZLE3D_TEMPLATE_PERSPECTIVE));
    }

    #[test]
    fn app_definition_declares_the_add_object_dialog() {
        let app = create_puzzle3d_app();
        let dialog = app.definition.dialogs.iter().find(|entry| entry.id == "addObject").expect("addObject dialog declared");
        assert_eq!(dialog.submit_action.as_str(), "addObjectKind");
        assert_eq!(dialog.args.len(), 1);
    }

    #[test]
    fn app_labels_overlay_is_german_reuse_branded_for_aggregator() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let view = ViewState {
            locale: Some("de".into()),
            terminology: Some("reuse".into()),
            ..ViewState::default()
        };
        let overlay = app.app_labels(&view);
        assert_eq!(overlay.mode_labels.get("edit").map(String::as_str), Some("Bearbeiten"));
        assert_eq!(overlay.window_kind_labels.get(PUZZLE3D_PLAY_WINDOW_MAIN).map(String::as_str), Some("Aggregator"));
        assert_eq!(overlay.dialog_labels.get("addObject.title").map(String::as_str), Some("Baukomponente hinzufügen"));
        assert_eq!(overlay.dialog_labels.get("addObject.submit").map(String::as_str), Some("Hinzufügen"));
        assert_eq!(overlay.action_arg_labels.get("addObject.objectKind.option.Object").map(String::as_str), Some("Baukomponente"));
        assert_eq!(overlay.action_labels.get("addObjectKind").map(String::as_str), Some("Baukomponente hinzufügen"));
        assert_eq!(overlay.action_labels.get("contextMenuAt").map(String::as_str), Some("Aktionsmenü öffnen"));
        assert_eq!(overlay.action_labels.get("worldPick").map(String::as_str), Some("Punkt in der Welt wählen"));
        assert_eq!(overlay.action_labels.get("openVortexSuggestions").map(String::as_str), Some("Verbindungspunkt-Vorschläge öffnen"));
        assert_eq!(overlay.action_labels.get("createAttraction").map(String::as_str), Some("Verbindung erstellen"));
        assert_eq!(overlay.utility_labels.get("transform").map(String::as_str), Some("Transformieren"));
        assert_eq!(overlay.example_labels.get(PUZZLE3D_EXAMPLE_CONCRETE_FOREST).map(String::as_str), Some("Abbau Aufbau"));
        assert!(!overlay.action_labels.get("contextMenuAt").is_some_and(|label| label.contains("Kontextmenü") || label.contains("Context Menu")));
        assert!(!overlay.action_labels.values().any(|label| label.contains("Hover") || label.contains("Pick") || label.contains("hovern")));
    }

    #[test]
    fn document_and_kinds_trees_use_german_reuse_section_labels() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let view = ViewState {
            locale: Some("de".into()),
            terminology: Some("reuse".into()),
            ..ViewState::default()
        };
        let document = serde_json::to_string(&app.render(PUZZLE3D_PLAY_BODY_DOCUMENT, None, &view).expect("document")).unwrap();
        let kinds = serde_json::to_string(&app.render(PUZZLE3D_PLAY_BODY_KINDS, None, &view).expect("kinds")).unwrap();
        let measures_json = serde_json::to_string(&app.window_measures(&view)).unwrap();
        assert!(document.contains("Baukomponenten"), "document tree objects section");
        assert!(document.contains("Verbindungen"), "document tree attractions section");
        assert!(document.contains("Referenzen"), "document tree references section");
        assert!(document.contains("Zielvolumina"), "document tree target volumes section");
        assert!(kinds.contains("Kabel"), "catalogue cables section");
        assert!(kinds.contains("Verbindungen"), "catalogue attractions section");
        assert!(!document.contains("\"Attractions\"") && !kinds.contains("\"Attractions\""), "English Attractions must not appear");
        assert!(!kinds.contains("\"Cables\""), "English Cables must not appear");
        assert!(measures_json.contains("Verbindungen"), "select measures attractions toggle");
        assert!(!measures_json.contains("\"Attractions\""), "select measures must not hardcode Attractions");
    }

    #[test]
    fn app_labels_overlay_stays_english_native_without_brand_locks() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let overlay = app.app_labels(&ViewState::default());
        assert_eq!(overlay.mode_labels.get("edit").map(String::as_str), Some("Edit"));
        assert_eq!(overlay.window_kind_labels.get(PUZZLE3D_PLAY_WINDOW_MAIN).map(String::as_str), Some("Puzzle 3D"));
        assert_eq!(overlay.dialog_labels.get("addObject.title").map(String::as_str), Some("Add Object"));
        assert_eq!(overlay.action_labels.get("contextMenuAt").map(String::as_str), Some("Open Actions Menu"));
        assert_eq!(overlay.action_labels.get("addObjectKind").map(String::as_str), Some("Add Object"));
    }

    //#region 🧭️ Suggestions, select-then-open context menu, fill build progress (Round 2)
    fn vortex_full_ids(app: &VcsDocumentApp<Puzzle3dPlayApp>) -> Vec<String> {
        let projection = app.projection().expect("projection").0;
        let mut ids = Vec::new();
        for object in projection.get("objects").and_then(Value::as_array).into_iter().flatten() {
            let object_id = object.get("id").and_then(Value::as_str).unwrap_or_default();
            for vortex in object.get("vortices").and_then(Value::as_array).into_iter().flatten() {
                if let Some(vortex_id) = vortex.get("id").and_then(Value::as_str) {
                    ids.push(puzzle3d_vortex_full_id(object_id, vortex_id));
                }
            }
        }
        ids
    }

    fn first_vortex_full_id(app: &VcsDocumentApp<Puzzle3dPlayApp>) -> String {
        vortex_full_ids(app).into_iter().next().expect("seed vortex")
    }

    fn render_composite(app: &mut VcsDocumentApp<Puzzle3dPlayApp>) -> Value {
        let node = app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render");
        serde_json::to_value(&node).unwrap()
    }

    fn instance_count(node: &Value) -> usize {
        node.pointer("/world3d/instancesJson")
            .and_then(Value::as_str)
            .and_then(|raw| serde_json::from_str::<Vec<Value>>(raw).ok())
            .map(|instances| instances.len())
            .unwrap_or(0)
    }

    fn interaction_of(node: &Value) -> Value {
        node.pointer("/world3d/interactionJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or(Value::Null)
    }

    fn selection_of(node: &Value) -> Value {
        node.pointer("/world3d/selectionJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or(Value::Null)
    }

    fn lod_of(node: &Value) -> Value {
        node.pointer("/world3d/lodJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or(Value::Null)
    }

    fn camera_of(node: &Value) -> Value {
        node.pointer("/world3d/cameraJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or(Value::Null)
    }

    /// 🔍️ Depth-first search for a [`WindowMeasure::Slider`]'s value by id, descending into groups (the
    /// fill-count slider now nests inside the fill Utility Options group rather than sitting on the engagement).
    fn find_measure_slider(measures: &[WindowMeasure], slider_id: &str) -> Option<f64> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Slider { id, value, .. } if id == slider_id => Some(*value),
            WindowMeasure::Group { children, .. } => find_measure_slider(children, slider_id),
            _ => None,
        })
    }

    fn find_measure_slider_max(measures: &[WindowMeasure], slider_id: &str) -> Option<f64> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Slider { id, max, .. } if id == slider_id => Some(*max),
            WindowMeasure::Group { children, .. } => find_measure_slider_max(children, slider_id),
            _ => None,
        })
    }

    fn find_measure_slider_ready(measures: &[WindowMeasure], slider_id: &str) -> Option<f64> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Slider { id, ready, .. } if id == slider_id => *ready,
            WindowMeasure::Group { children, .. } => find_measure_slider_ready(children, slider_id),
            _ => None,
        })
    }

    fn find_measure_select(measures: &[WindowMeasure], select_id: &str) -> Option<String> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Select { id, value, .. } if id == select_id => Some(value.clone()),
            WindowMeasure::Group { children, .. } => find_measure_select(children, select_id),
            _ => None,
        })
    }

    fn find_measure_toggle(measures: &[WindowMeasure], toggle_id: &str) -> Option<bool> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Toggle { id, pressed, .. } if id == toggle_id => Some(*pressed),
            WindowMeasure::Group { children, .. } => find_measure_toggle(children, toggle_id),
            _ => None,
        })
    }

    /// 🎯️ Top-level utility tag of a [`WindowMeasure::Group`] by id, or `None` when the group is absent.
    fn measure_group_tag(measures: &[WindowMeasure], group_id: &str) -> Option<Option<String>> {
        measures.iter().find_map(|measure| match measure {
            WindowMeasure::Group { id, active_utility_id, .. } if id == group_id => Some(active_utility_id.clone()),
            _ => None,
        })
    }

    fn context_menu_of(node: &Value) -> Value {
        node.pointer("/world3d/contextMenuJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or(Value::Null)
    }

    fn brush_preview_of(node: &Value) -> Value {
        node.pointer("/world3d/brushPreviewJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or(Value::Null)
    }

    #[test]
    fn context_menu_at_selects_vortex_and_prepends_suggest_objects() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let vortex = first_vortex_full_id(&app);
        app.handle_action("contextMenuAt", Some(&json!({ "kind": "vortex", "id": vortex })), &ViewState::default(), &testkit::meta("local")).expect("contextMenuAt");
        let menu = context_menu_of(&render_composite(&mut app));
        let menu_json = serde_json::to_string(&menu).unwrap();
        assert!(menu_json.contains("Suggest objects"), "menu should be {menu_json}");
        assert!(menu_json.contains("openVortexSuggestions"));
        assert!(menu_json.contains("sparkles"), "menu should include suggest icon: {menu_json}");
        assert!(menu_json.contains("Zoom to selection"), "menu should include zoom: {menu_json}");
        assert!(menu_json.contains("deleteSelection"), "menu should include delete: {menu_json}");
    }

    #[test]
    fn context_menu_at_selects_target_volume_and_set_target_volume_flag_toggles_hidden() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        app.handle_action("addTargetVolume", Some(&json!({ "origin": [1.0, 2.0, 3.0] })), &ViewState::default(), &testkit::meta("local")).expect("addTargetVolume");
        let projection = app.projection().expect("projection").0;
        let volume_id = projection.get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("id")).and_then(Value::as_str).expect("volume id").to_string();
        app.handle_action("contextMenuAt", Some(&json!({ "kind": "targetVolume", "id": volume_id })), &ViewState::default(), &testkit::meta("local")).expect("contextMenuAt");
        let menu_json = serde_json::to_string(&context_menu_of(&render_composite(&mut app))).unwrap();
        assert!(menu_json.contains("setTargetVolumeFlag"), "menu should be {menu_json}");
        app.handle_action("setTargetVolumeFlag", Some(&json!({ "id": volume_id, "flag": "hidden", "value": true })), &ViewState::default(), &testkit::meta("local")).expect("setTargetVolumeFlag");
        let projection = app.projection().expect("projection").0;
        let hidden = projection.get("targetVolumes").and_then(Value::as_array).and_then(|volumes| volumes.first()).and_then(|volume| volume.get("hidden")).and_then(Value::as_bool);
        assert_eq!(hidden, Some(true));
    }

    #[test]
    fn open_vortex_suggestions_opens_the_suggestion_popup() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let vortex = first_vortex_full_id(&app);
        let result = app.handle_action("openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 12.0, "y": 34.0 })), &ViewState::default(), &testkit::meta("local")).expect("openVortexSuggestions");
        assert!(
            result.requested_effects.iter().all(|effect| !matches!(effect, HostEffect::SetActiveUtility { .. } | HostEffect::SetActiveTool { .. })),
            "opening a one-shot suggestion must not switch the host-owned utility or tool: {:?}",
            result.requested_effects,
        );
        let node = app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render");
        let interaction = interaction_of(&serde_json::to_value(&node).unwrap());
        assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "context-menu suggestion stays in the current selection mode");
        let menu = interaction.get("suggestionMenu").expect("suggestionMenu present");
        assert_eq!(menu.get("open").and_then(Value::as_bool), Some(true));
        assert_eq!(menu.get("x").and_then(Value::as_f64), Some(12.0));
        assert_eq!(menu.get("y").and_then(Value::as_f64), Some(34.0));
        assert_eq!(menu.get("vortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
        assert!(menu.get("windowId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "suggestion menu is scoped to the opening window: {menu}");
    }

    #[test]
    fn open_vortex_suggestions_records_explicit_window_id() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let vortex = first_vortex_full_id(&app);
        let mut view = ViewState::default();
        view.window_id = Some("puzzle3d-main-perspective".into());
        app.handle_action(
            "openVortexSuggestions",
            Some(&json!({ "fullId": vortex, "x": 8.0, "y": 16.0, "windowId": "puzzle3d-main-top" })),
            &view,
            &testkit::meta("local"),
        )
        .expect("openVortexSuggestions");
        let interaction = interaction_of(&render_composite(&mut app));
        let menu = interaction.get("suggestionMenu").expect("suggestionMenu present");
        assert_eq!(menu.get("windowId").and_then(Value::as_str), Some("puzzle3d-main-top"));
        assert_eq!(menu.get("vortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
    }

    #[test]
    fn accept_suggestion_with_full_id_places_even_if_selection_was_cleared() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let vortex = first_vortex_full_id(&app);
        app.handle_action("openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 0.0, "y": 0.0 })), &ViewState::default(), &testkit::meta("local")).expect("openVortexSuggestions");
        let before_count = app.projection().expect("projection").0.get("objects").and_then(Value::as_array).map(|objects| objects.len()).unwrap_or(0);
        // 🧹️ Simulate the split-pane outside-dismiss race clearing vortex selection before accept.
        app.handle_action("setSelection", Some(&json!({ "selection": { "objectIds": [], "vortexIds": [], "attractionIds": [], "targetVolumeIds": [], "referenceIds": [] } })), &ViewState::default(), &testkit::meta("local")).expect("setSelection");
        let result = app
            .handle_action("acceptSuggestion", Some(&json!({ "index": 0, "fullId": vortex })), &ViewState::default(), &testkit::meta("local"))
            .expect("acceptSuggestion");
        assert!(
            result.requested_effects.iter().all(|effect| !matches!(effect, HostEffect::SetActiveUtility { .. } | HostEffect::SetActiveTool { .. })),
            "accept must not switch utility/tool: {:?}",
            result.requested_effects,
        );
        let after_count = app.projection().expect("projection").0.get("objects").and_then(Value::as_array).map(|objects| objects.len()).unwrap_or(0);
        assert!(after_count > before_count, "accept with fullId must place even after selection clear ({before_count} -> {after_count})");
        let interaction = interaction_of(&render_composite(&mut app));
        assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
    }

    #[test]
    fn close_vortex_suggestions_clears_the_menu() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let vortex = first_vortex_full_id(&app);
        app.handle_action("openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 0.0, "y": 0.0 })), &ViewState::default(), &testkit::meta("local")).expect("openVortexSuggestions");
        app.handle_action("closeVortexSuggestions", None, &ViewState::default(), &testkit::meta("local")).expect("closeVortexSuggestions");
        let interaction = interaction_of(&render_composite(&mut app));
        assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
    }

    /// 🖱️ Hovering a row in the suggestion popup must live-update the 3D brush preview (rendered by
    /// `world_brush_preview_json`, which reads `runtime.brush_candidate_index`) to the hovered
    /// candidate, so the UI can highlight it in 3D before the user clicks to accept — without
    /// switching the host-owned active utility into brush mode.
    #[test]
    fn hover_suggestion_updates_the_brush_candidate_index_and_live_preview() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let vortex = first_vortex_full_id(&app);
        app.handle_action("openVortexSuggestions", Some(&json!({ "fullId": vortex.clone(), "x": 0.0, "y": 0.0 })), &ViewState::default(), &testkit::meta("local")).expect("openVortexSuggestions");
        let node = app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render");
        let composite = serde_json::to_value(&node).unwrap();
        let interaction = interaction_of(&composite);
        assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "suggestion hover must not enter brush mode");
        assert_eq!(interaction.get("brushCandidateIndex").and_then(Value::as_u64), Some(0), "opening suggestions starts hover at the first candidate");
        let candidates = interaction.pointer("/suggestionMenu/candidates").and_then(Value::as_array).cloned().unwrap_or_default();
        assert!(!candidates.is_empty(), "suggestion candidates should be present");
        assert!(candidates[0].get("color").and_then(Value::as_str).is_some_and(|color| color.starts_with('#')), "candidates carry object-kind color: {candidates:?}");
        assert!(candidates[0].get("icon").and_then(Value::as_str).is_some_and(|icon| !icon.is_empty()), "candidates carry icon: {candidates:?}");
        let preview = brush_preview_of(&composite);
        assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "the live preview must target the vortex the suggestion menu was opened on");
        assert!(preview.get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "the live preview must resolve to a real candidate object kind");
        assert!(preview.get("color").and_then(Value::as_str).is_some_and(|color| color.starts_with('#')), "brush preview carries object-kind color: {preview}");

        app.handle_action("hoverSuggestion", Some(&json!({ "index": 1 })), &ViewState::default(), &testkit::meta("local")).expect("hoverSuggestion");
        let node = app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render");
        let composite = serde_json::to_value(&node).unwrap();
        let interaction = interaction_of(&composite);
        assert_eq!(interaction.get("brushCandidateIndex").and_then(Value::as_u64), Some(1), "hovering a different row must move the tracked candidate index");
        let preview = brush_preview_of(&composite);
        assert_eq!(preview.get("targetVortexFullId").and_then(Value::as_str), Some(vortex.as_str()), "the preview must keep targeting the same vortex while only the hovered candidate changes");
        assert!(preview.get("color").and_then(Value::as_str).is_some_and(|color| color.starts_with('#')), "hovered brush preview still carries color: {preview}");
    }

    #[test]
    fn accept_suggestion_appends_an_object_and_closes_the_menu() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let object_count_before = object_count(&app);
        let vortex = first_vortex_full_id(&app);
        app.handle_action("openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 0.0, "y": 0.0 })), &ViewState::default(), &testkit::meta("local")).expect("openVortexSuggestions");
        let result = app.handle_action("acceptSuggestion", None, &ViewState::default(), &testkit::meta("local")).expect("acceptSuggestion");
        assert_eq!(object_count(&app), object_count_before + 1);
        assert!(
            result.requested_effects.iter().all(|effect| !matches!(effect, HostEffect::SetActiveUtility { .. } | HostEffect::SetActiveTool { .. })),
            "accepting a one-shot suggestion must leave the host-owned utility/tool unchanged: {:?}",
            result.requested_effects,
        );
        let interaction = interaction_of(&render_composite(&mut app));
        assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
        assert_eq!(interaction.get("activeUtility").and_then(Value::as_str), Some("select"));
        assert!(interaction.get("hoveredVortexFullId").is_none_or(|value| value.is_null()), "accept must clear sticky vortex hover");
        let selected_vortices = render_composite(&mut app)
            .pointer("/world3d/vorticesJson")
            .and_then(Value::as_str)
            .and_then(|raw| serde_json::from_str::<Vec<Value>>(raw).ok())
            .unwrap_or_default()
            .iter()
            .filter(|entry| entry.get("selected").and_then(Value::as_bool) == Some(true))
            .count();
        assert_eq!(selected_vortices, 0, "one-shot accept must leave no sticky vortex selection");
    }

    /// 🧹️ A failed place (unknown vortex) must still close the suggestion menu — otherwise
    /// `suggestionMenu.open` stays true and every split pane's regular context menu is gated shut.
    #[test]
    fn accept_suggestion_closes_menu_even_when_placement_fails() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let vortex = first_vortex_full_id(&app);
        app.handle_action("worldVortexHover", Some(&json!({ "fullId": vortex.clone() })), &ViewState::default(), &testkit::meta("local")).expect("worldVortexHover");
        app.handle_action(
            "openVortexSuggestions",
            Some(&json!({ "fullId": vortex.clone(), "x": 10.0, "y": 20.0, "windowId": "puzzle3d-main-top" })),
            &ViewState::default(),
            &testkit::meta("local"),
        )
        .expect("openVortexSuggestions");
        let before = interaction_of(&render_composite(&mut app));
        assert_eq!(before.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true));
        assert_eq!(before.get("hoveredVortexFullId").and_then(Value::as_str), Some(vortex.as_str()));
        let object_count_before = object_count(&app);
        app.handle_action(
            "acceptSuggestion",
            Some(&json!({ "index": 0, "fullId": "missing-object::missing-vortex" })),
            &ViewState::default(),
            &testkit::meta("local"),
        )
        .expect("acceptSuggestion");
        assert_eq!(object_count(&app), object_count_before, "unknown-vortex accept must not place");
        let interaction = interaction_of(&render_composite(&mut app));
        assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()), "failed accept must still dismiss the suggestion menu");
        assert!(interaction.get("hoveredVortexFullId").is_none_or(|value| value.is_null()), "failed accept must clear sticky vortex hover");
    }

    #[test]
    fn close_vortex_suggestions_clears_sticky_hover() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let vortex = first_vortex_full_id(&app);
        app.handle_action("worldVortexHover", Some(&json!({ "fullId": vortex.clone() })), &ViewState::default(), &testkit::meta("local")).expect("worldVortexHover");
        app.handle_action("openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 0.0, "y": 0.0 })), &ViewState::default(), &testkit::meta("local")).expect("openVortexSuggestions");
        app.handle_action("closeVortexSuggestions", None, &ViewState::default(), &testkit::meta("local")).expect("closeVortexSuggestions");
        let interaction = interaction_of(&render_composite(&mut app));
        assert!(interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
        assert!(interaction.get("hoveredVortexFullId").is_none_or(|value| value.is_null()));
    }

    #[test]
    fn grid_window_options_control_one_visible_grid_spacing() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        app.handle_action("setGridVisible", Some(&json!({ "pressed": false })), &ViewState::default(), &testkit::meta("local")).expect("setGridVisible");
        app.handle_action("setGridSpacing", Some(&json!({ "value": 7.5 })), &ViewState::default(), &testkit::meta("local")).expect("setGridSpacing");
        let lod = lod_of(&render_composite(&mut app));
        assert_eq!(lod.get("showLodGrid").and_then(Value::as_bool), Some(false));
        assert_eq!(lod.get("gridFactor").and_then(Value::as_f64), Some(7.5));
        let measures = app.window_measures(&ViewState::default());
        let window_measures = measures.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("main window measures");
        assert_eq!(measure_group_tag(window_measures, "puzzle3d-play-grid"), Some(None));
        assert_eq!(find_measure_slider(window_measures, "puzzle3d-play-grid-spacing"), Some(7.5));
    }

    /// 🪟️ The regression this whole ticket exists for: two window instances of the same kind (e.g. a
    /// split top/perspective pane pair) must never share window options — toggling grid visibility in
    /// one instance must leave every other instance's grid untouched, both in its measures chrome and
    /// in its own rendered scene.
    #[test]
    fn window_options_are_local_to_the_window_instance_not_shared_across_split_panes() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let second_window = "puzzle3d-main-2";
        let instances = vec![
            ViewWindowInstance { id: PUZZLE3D_PLAY_WINDOW_MAIN.to_string(), window_kind_id: PUZZLE3D_PLAY_WINDOW_MAIN.to_string() },
            ViewWindowInstance { id: second_window.to_string(), window_kind_id: PUZZLE3D_PLAY_WINDOW_MAIN.to_string() },
        ];
        let second_window_view = ViewState { window_id: Some(second_window.to_string()), window_instances: instances.clone(), ..ViewState::default() };
        let toggle_id = format!("{PUZZLE3D_PLAY_CONTROLLER_ID}-grid-visible");

        // Both instances start visible (the type default).
        let initial_measures = app.window_measures(&ViewState { window_instances: instances.clone(), ..ViewState::default() });
        assert_eq!(find_measure_toggle(initial_measures.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("base measures"), &toggle_id), Some(true));
        assert_eq!(find_measure_toggle(initial_measures.get(second_window).expect("second measures"), &toggle_id), Some(true));

        // Hide the grid, but ONLY on the second window instance.
        app.handle_action("setGridVisible", Some(&json!({ "pressed": false })), &second_window_view, &testkit::meta("local")).expect("setGridVisible on second window");

        let measures_after = app.window_measures(&ViewState { window_instances: instances.clone(), ..ViewState::default() });
        assert_eq!(
            find_measure_toggle(measures_after.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("base measures"), &toggle_id),
            Some(true),
            "the base window instance's grid must stay visible",
        );
        assert_eq!(
            find_measure_toggle(measures_after.get(second_window).expect("second measures"), &toggle_id),
            Some(false),
            "only the targeted window instance's grid toggles off",
        );

        // The rendered scenes agree: the base window still draws its LOD grid, the second does not.
        let base_composite = app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &ViewState { window_id: Some(PUZZLE3D_PLAY_WINDOW_MAIN.into()), ..ViewState::default() }).expect("render base window");
        let base_lod = lod_of(&serde_json::to_value(&base_composite).unwrap());
        assert_eq!(base_lod.get("showLodGrid").and_then(Value::as_bool), Some(true));

        let second_composite = app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &second_window_view).expect("render second window");
        let second_lod = lod_of(&serde_json::to_value(&second_composite).unwrap());
        assert_eq!(second_lod.get("showLodGrid").and_then(Value::as_bool), Some(false));
    }

    /// 🎥️ `setCamera`/`setProjection`/`setProjectionParam`/`focusSelection` moved off the document —
    /// they are View-kind and must never emit VCS operations, no matter what they mutate.
    #[test]
    fn camera_actions_are_view_actions_that_emit_no_document_operations() {
        let app = create_puzzle3d_app();
        for action_id in ["setCamera", "setProjection", "setProjectionParam", "focusSelection"] {
            let def = app.definition.actions.iter().find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("{action_id} declared"));
            assert_eq!(def.kind, ActionKind::View, "{action_id} must be a View action — camera is session-only, never a VCS edit");
        }
        let mut live = new_app_with_registry();
        let before = live.projection().expect("projection").0.clone();
        let result = live.handle_action("setCamera", Some(&json!({ "camera": { "position": [1.0, 2.0, 3.0], "target": [4.0, 5.0, 6.0], "zoom": 2.5 } })), &ViewState::default(), &testkit::meta("local")).expect("setCamera");
        assert!(result.operations.is_empty(), "setCamera must not emit document operations");
        assert_eq!(live.projection().expect("projection").0, before, "setCamera must not mutate the document");
    }

    /// 🪟️📷️ The hard dependency for the React 3D viewport's per-window `setCamera` dispatch: orbiting one
    /// window instance's camera must never move any sibling instance's camera, and must never touch the
    /// shared document (dispatching `setCamera` for window A leaves window B's rendered camera and the
    /// document itself untouched).
    #[test]
    fn set_camera_is_per_window_and_leaves_sibling_windows_and_the_document_untouched() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let window_a = "puzzle3d-main-a";
        let window_b = "puzzle3d-main-b";
        let instances = vec![
            ViewWindowInstance { id: window_a.to_string(), window_kind_id: PUZZLE3D_PLAY_WINDOW_MAIN.to_string() },
            ViewWindowInstance { id: window_b.to_string(), window_kind_id: PUZZLE3D_PLAY_WINDOW_MAIN.to_string() },
        ];
        let window_a_view = ViewState { window_id: Some(window_a.to_string()), window_instances: instances.clone(), ..ViewState::default() };
        let window_b_view = ViewState { window_id: Some(window_b.to_string()), window_instances: instances.clone(), ..ViewState::default() };

        let before_document = app.projection().expect("projection").0.clone();
        let camera_b_before = camera_of(&serde_json::to_value(&app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &window_b_view).expect("render window B before")).unwrap());

        let result = app
            .handle_action("setCamera", Some(&json!({ "camera": { "position": [11.0, 22.0, 33.0], "target": [1.0, 2.0, 3.0], "zoom": 4.0 } })), &window_a_view, &testkit::meta("local"))
            .expect("setCamera on window A");
        assert!(result.operations.is_empty(), "setCamera must not emit document operations");
        assert_eq!(app.projection().expect("projection").0, before_document, "setCamera must never mutate the shared document");

        let camera_a_after = camera_of(&serde_json::to_value(&app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &window_a_view).expect("render window A after")).unwrap());
        assert_eq!(camera_a_after.get("position").and_then(|value| value.as_array()).cloned(), Some(vec![json!(11.0), json!(22.0), json!(33.0)]), "window A's own rendered camera picks up the new pose");

        let camera_b_after = camera_of(&serde_json::to_value(&app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &window_b_view).expect("render window B after")).unwrap());
        assert_eq!(camera_b_after, camera_b_before, "window B's rendered camera must be unaffected by window A's setCamera");
    }

    /// 🪣️ Fill count drives the shared document + reveal cutoff — split top/perspective panes must never
    /// disagree about which planned objects are visible after a slider commit on either pane.
    #[test]
    fn fill_count_is_shared_across_split_panes_reveal_cutoffs_and_instances() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let top = PUZZLE3D_PLAY_WINDOW_TOP;
        let perspective = PUZZLE3D_PLAY_WINDOW_PERSPECTIVE;
        let instances = vec![
            ViewWindowInstance { id: top.to_string(), window_kind_id: PUZZLE3D_PLAY_WINDOW_MAIN.to_string() },
            ViewWindowInstance { id: perspective.to_string(), window_kind_id: PUZZLE3D_PLAY_WINDOW_MAIN.to_string() },
        ];
        let top_view = ViewState {
            window_id: Some(top.to_string()),
            window_instances: instances.clone(),
            active_tool_id: Some("fill".into()),
            ..ViewState::default()
        };
        let perspective_view = ViewState {
            window_id: Some(perspective.to_string()),
            window_instances: instances.clone(),
            active_tool_id: Some("fill".into()),
            ..ViewState::default()
        };

        app.handle_action(SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": "fill" })), &top_view, &testkit::meta("local")).expect("select fill tool");
        for _ in 0..64 {
            app.handle_action("fillBuildTick", None, &top_view, &testkit::meta("local")).expect("fillBuildTick");
            let ready = app
                .tool_measures(&top_view)
                .get("fill")
                .and_then(|tool_measures| find_measure_slider_ready(tool_measures, "puzzle3d-fill-count"))
                .unwrap_or(0.0);
            if ready >= 3.0 {
                break;
            }
        }
        let ready = app
            .tool_measures(&top_view)
            .get("fill")
            .and_then(|tool_measures| find_measure_slider_ready(tool_measures, "puzzle3d-fill-count"))
            .unwrap_or(0.0) as u32;
        assert!(ready >= 3, "need a planned fill prefix to assert cross-pane sync");

        // Commit from the top pane only — the perspective pane must still track the same cutoff.
        let committed = ready.min(3);
        app.handle_action("setFillCount", Some(&json!({ "value": committed })), &top_view, &testkit::meta("local")).expect("setFillCount on top");

        let top_render = serde_json::to_value(app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &top_view).expect("render top")).unwrap();
        let perspective_render = serde_json::to_value(app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &perspective_view).expect("render perspective")).unwrap();
        assert_eq!(
            interaction_of(&top_render).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64),
            Some(committed as u64),
            "top pane reveal cutoff must track the committed fill count",
        );
        assert_eq!(
            interaction_of(&perspective_render).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64),
            Some(committed as u64),
            "perspective pane must share the same reveal cutoff — fill is document-global, not per-window",
        );
        assert_eq!(instance_count(&top_render), instance_count(&perspective_render), "both panes must emit the same instance list for the shared fill plan");

        let top_ids: Vec<String> = top_render
            .pointer("/world3d/instancesJson")
            .and_then(Value::as_str)
            .and_then(|raw| serde_json::from_str::<Vec<Value>>(raw).ok())
            .into_iter()
            .flatten()
            .filter_map(|instance| instance.get("id").and_then(Value::as_str).map(str::to_string))
            .collect();
        let perspective_ids: Vec<String> = perspective_render
            .pointer("/world3d/instancesJson")
            .and_then(Value::as_str)
            .and_then(|raw| serde_json::from_str::<Vec<Value>>(raw).ok())
            .into_iter()
            .flatten()
            .filter_map(|instance| instance.get("id").and_then(Value::as_str).map(str::to_string))
            .collect();
        assert_eq!(top_ids, perspective_ids, "top and perspective must show the exact same object ids after a fill slider commit");

        // Sliding from the other pane must keep both panes in lockstep.
        let reduced = committed.saturating_sub(1);
        app.handle_action("setFillCount", Some(&json!({ "value": reduced })), &perspective_view, &testkit::meta("local")).expect("setFillCount on perspective");
        let top_after = serde_json::to_value(app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &top_view).expect("render top after reduce")).unwrap();
        let perspective_after = serde_json::to_value(app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &perspective_view).expect("render perspective after reduce")).unwrap();
        assert_eq!(interaction_of(&top_after).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(reduced as u64));
        assert_eq!(interaction_of(&perspective_after).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(reduced as u64));
        assert_eq!(instance_count(&top_after), instance_count(&perspective_after));
    }

    #[test]
    fn vortex_show_window_option_defaults_to_selected_and_switches_to_always() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let all_vortex_ids = vortex_full_ids(&app);
        assert!(!all_vortex_ids.is_empty(), "fixture must expose vortices");
        let measures = app.window_measures(&ViewState::default());
        let window_measures = measures.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("main window measures");
        assert_eq!(find_measure_select(window_measures, "puzzle3d-play-vortex-show").as_deref(), Some(PUZZLE3D_VORTEX_SHOW_SELECTED));

        let idle_selected = render_composite(&mut app).pointer("/world3d/vorticesJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str::<Vec<Value>>(raw).ok()).unwrap_or_default();
        assert!(idle_selected.is_empty(), "Selected mode must hide vortices while idle");

        app.handle_action("setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), &ViewState::default(), &testkit::meta("local")).expect("setVortexShow always");
        let measures_always = app.window_measures(&ViewState::default());
        let window_measures_always = measures_always.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("main window measures");
        assert_eq!(find_measure_select(window_measures_always, "puzzle3d-play-vortex-show").as_deref(), Some(PUZZLE3D_VORTEX_SHOW_ALWAYS));
        let idle_always = render_composite(&mut app).pointer("/world3d/vorticesJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str::<Vec<Value>>(raw).ok()).unwrap_or_default();
        assert_eq!(idle_always.len(), all_vortex_ids.len(), "Always mode must emit every vortex while idle");

        app.handle_action("setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_SELECTED })), &ViewState::default(), &testkit::meta("local")).expect("setVortexShow selected");
        let idle_again = render_composite(&mut app).pointer("/world3d/vorticesJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str::<Vec<Value>>(raw).ok()).unwrap_or_default();
        assert!(idle_again.is_empty(), "switching back to Selected must hide idle vortices");
    }

    #[test]
    fn vortex_direction_window_option_defaults_to_outwards_and_switches_to_inwards() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let measures = app.window_measures(&ViewState::default());
        let window_measures = measures.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("main window measures");
        assert_eq!(find_measure_select(window_measures, "puzzle3d-play-vortex-direction").as_deref(), Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS));

        app.handle_action("setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), &ViewState::default(), &testkit::meta("local")).expect("setVortexShow always");
        let outwards_vortices = render_composite(&mut app).pointer("/world3d/vorticesJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str::<Vec<Value>>(raw).ok()).unwrap_or_default();
        assert!(!outwards_vortices.is_empty(), "fixture must expose vortices");
        assert!(outwards_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS)));

        app.handle_action("setVortexDirection", Some(&json!({ "value": PUZZLE3D_VORTEX_DIRECTION_INWARDS })), &ViewState::default(), &testkit::meta("local")).expect("setVortexDirection inwards");
        let measures_inwards = app.window_measures(&ViewState::default());
        let window_measures_inwards = measures_inwards.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("main window measures");
        assert_eq!(find_measure_select(window_measures_inwards, "puzzle3d-play-vortex-direction").as_deref(), Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS));
        let inwards_vortices = render_composite(&mut app).pointer("/world3d/vorticesJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str::<Vec<Value>>(raw).ok()).unwrap_or_default();
        assert!(inwards_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS)));
    }

    #[test]
    fn vortex_direction_option_is_local_to_the_window_instance() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let second_window = "puzzle3d-main-2";
        let instances = vec![
            ViewWindowInstance { id: PUZZLE3D_PLAY_WINDOW_MAIN.to_string(), window_kind_id: PUZZLE3D_PLAY_WINDOW_MAIN.to_string() },
            ViewWindowInstance { id: second_window.to_string(), window_kind_id: PUZZLE3D_PLAY_WINDOW_MAIN.to_string() },
        ];
        let second_window_view = ViewState { window_id: Some(second_window.to_string()), window_instances: instances.clone(), ..ViewState::default() };

        app.handle_action("setVortexShow", Some(&json!({ "value": PUZZLE3D_VORTEX_SHOW_ALWAYS })), &ViewState { window_instances: instances.clone(), ..ViewState::default() }, &testkit::meta("local")).expect("setVortexShow always");
        app.handle_action("setVortexDirection", Some(&json!({ "value": PUZZLE3D_VORTEX_DIRECTION_INWARDS })), &second_window_view, &testkit::meta("local")).expect("setVortexDirection inwards on second window");

        let base_composite = serde_json::to_value(app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &ViewState { window_id: Some(PUZZLE3D_PLAY_WINDOW_MAIN.into()), window_instances: instances.clone(), ..ViewState::default() }).expect("render base window")).unwrap();
        let base_vortices = base_composite.pointer("/world3d/vorticesJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str::<Vec<Value>>(raw).ok()).unwrap_or_default();
        assert!(base_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_OUTWARDS)));

        let second_composite = serde_json::to_value(app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &second_window_view).expect("render second window")).unwrap();
        let second_vortices = second_composite.pointer("/world3d/vorticesJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str::<Vec<Value>>(raw).ok()).unwrap_or_default();
        assert!(second_vortices.iter().all(|record| record.get("displayDirection").and_then(Value::as_str) == Some(PUZZLE3D_VORTEX_DIRECTION_INWARDS)));
    }

    #[test]
    fn fill_build_tick_is_ignored_when_fill_tool_is_inactive() {
        use semio_framework_core::kernel::UiDirtyScope;
        let mut app = Puzzle3dPlayApp::default();
        let projection = app.initial_projection();
        let history = semio_framework_plugin::HistoryView::empty();
        let document = DocumentView { projection: &projection, history: &history };
        let fill_view = ViewState { active_tool_id: Some("fill".into()), ..ViewState::default() };
        app.handle_action(SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": "fill" })), &document, &fill_view);

        let inactive_view = ViewState::default();
        app.handle_action(SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": null })), &document, &inactive_view);
        let before = app.precompute.fill_progress_summary();
        for _ in 0..64 {
            let result = app.handle_action("fillBuildTick", None, &document, &inactive_view);
            assert!(matches!(result.ui_scope, UiDirtyScope::None), "an inactive fill tick must not request any UI refresh");
        }
        let after = app.precompute.fill_progress_summary();
        assert_eq!(after, before, "stale or queued fill ticks must not advance planning after the Fill tool is deactivated");
    }

    #[test]
    fn fill_build_tick_only_plans_available_slider_range() {
        // 🐢️ `drive_precompute` is now bounded to a small per-call budget (the fix for the UI-freeze
        // bug: a single action must never grind the whole precompute queue synchronously), so the
        // build converges over several ticks — exactly like the real 120ms `fillBuildTick` loop in
        // `world-3d-host.tsx` — rather than in one call.
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let object_count_before = object_count(&app);
        let fill_view = ViewState { active_tool_id: Some("fill".into()), ..ViewState::default() };
        app.handle_action(SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": "fill" })), &fill_view, &testkit::meta("local")).expect("select fill tool");
        for _ in 0..64 {
            app.handle_action("fillBuildTick", None, &fill_view, &testkit::meta("local")).expect("fillBuildTick");
            let ready = app
                .tool_measures(&fill_view)
                .get("fill")
                .and_then(|tool_measures| find_measure_slider_ready(tool_measures, "puzzle3d-fill-count"))
                .unwrap_or(0.0);
            if ready >= 4.0 {
                break;
            }
        }
        let measures = app.tool_measures(&fill_view);
        let tool_measures = measures.get("fill").expect("fill tool measures");
        match find_measure_slider(tool_measures, "puzzle3d-fill-count") {
            Some(value) => assert_eq!(value, 0.0, "background planning must not change the selected fill count"),
            None => panic!("expected a fill-count slider in the fill tool measures"),
        }
        assert_eq!(object_count(&app), object_count_before, "background planning must not append generated objects below the slider count");
        assert_eq!(find_measure_slider_max(tool_measures, "puzzle3d-fill-count"), Some(PUZZLE3D_FILL_COUNT_MAX as f64), "fill slider range stays fixed at the fill count max");
        let available_count = find_measure_slider_ready(tool_measures, "puzzle3d-fill-count").expect("expected a fill-count slider ready extent") as usize;
        assert!(available_count > 0, "the fill slider ready extent must expose collision-free compatible placements");
        app.handle_action("setFillCount", Some(&json!({ "value": available_count })), &fill_view, &testkit::meta("local")).expect("setFillCount");
        assert_eq!(object_count(&app), object_count_before + available_count, "the fill slider must materialize exactly its available placement count");
        let rendered_after_fill = render_composite(&mut app);
        assert_eq!(instance_count(&rendered_after_fill), object_count_before + available_count, "the viewport must show every materialized fill object immediately");
        let initial_fill_ids: HashSet<String> = app
            .projection()
            .expect("projection").0
            .get("objects")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .skip(object_count_before)
            .filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string))
            .collect();
        // 🪪️ Incidental actions re-sync the applied document into the precompute session. That used to
        // rebuild `fill.base` around the materialized objects, after which the slider could neither
        // remove them nor replan — reproduce with a hover sync before clearing.
        let hovered_id = app.projection().expect("projection").0.get("objects").and_then(Value::as_array).and_then(|objects| objects.first()).and_then(|object| object.get("id")).and_then(Value::as_str).unwrap_or("").to_string();
        app.handle_action("setHover", Some(&json!({ "objectId": hovered_id })), &fill_view, &testkit::meta("local")).expect("setHover after fill");
        let reduced = (available_count / 2).max(0);
        app.handle_action("setFillCount", Some(&json!({ "value": reduced })), &fill_view, &testkit::meta("local")).expect("reduce fill count after sync");
        assert_eq!(object_count(&app), object_count_before + reduced as usize, "sliding down after an incidental sync must still remove fill objects from the document");
        let reduced_render = render_composite(&mut app);
        // 🪣️ The viewport keeps showing the FULL available plan (tagged revealIndex) even after
        // reducing — hiding is a client-side reveal-cutoff concern now, not a server-side instance
        // count concern; only the document (checked above) and the committed cutoff actually shrink.
        assert_eq!(instance_count(&reduced_render), object_count_before + available_count, "the viewport still exposes the full plan for instant re-reveal — nothing was discarded");
        assert_eq!(
            interaction_of(&reduced_render).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64),
            Some(reduced as u64),
            "the committed reveal cutoff tracks the reduced count"
        );
        // 🔽️🔼️ Prefix-stable plan: moving back up to a count that was already planned before must be
        // INSTANT — no replanning, no `fillBuildTick` catch-up dispatch — because the downward move
        // never discarded `sequence`/`appended_objects`/`appended_attractions`/`placed`.
        app.handle_action("setFillCount", Some(&json!({ "value": available_count })), &fill_view, &testkit::meta("local")).expect("move back up to the previously-planned count");
        assert_eq!(object_count(&app), object_count_before + available_count, "moving back up within the preserved plan is instant, not gated on another fillBuildTick");
        let target_measures = app.tool_measures(&fill_view);
        let target_tool_measures = target_measures.get("fill").expect("fill tool measures");
        assert_eq!(find_measure_slider(target_tool_measures, "puzzle3d-fill-count"), Some(available_count as f64));
        let restored_fill_ids: HashSet<String> = app
            .projection()
            .expect("projection").0
            .get("objects")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .skip(object_count_before)
            .filter_map(|object| object.get("id").and_then(Value::as_str).map(str::to_string))
            .collect();
        assert_eq!(restored_fill_ids, initial_fill_ids, "up-down-up restores the exact same planned objects — the plan is prefix-stable, never discarded and re-rolled");
        app.handle_action("setFillCount", Some(&json!({ "value": 0 })), &fill_view, &testkit::meta("local")).expect("clear fill count");
        assert_eq!(object_count(&app), object_count_before, "moving the fill slider to zero must remove every generated object");
    }

    #[test]
    fn set_fill_count_clamps_to_available_and_no_longer_dispatches_catch_up() {
        // 🔒️ Requesting more than is currently planned must clamp (never leave `runtime.fill_count`
        // and the applied document disagreeing), and `fillBuildTick` must never self-dispatch another
        // `setFillCount` — the viewport already shows every planned piece (tagged `revealIndex`) via
        // `compose_fill_display(available_count)`, so there is nothing left for a catch-up round trip
        // to accomplish, and it used to be the mechanism that turned one drag into a long chain of
        // expensive document amends.
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let object_count_before = object_count(&app);
        let fill_view = ViewState { active_tool_id: Some("fill".into()), ..ViewState::default() };
        app.handle_action(SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": "fill" })), &fill_view, &testkit::meta("local")).expect("select fill tool");
        app.handle_action("fillBuildTick", None, &fill_view, &testkit::meta("local")).expect("one fillBuildTick");
        let available_count = app
            .tool_measures(&fill_view)
            .get("fill")
            .and_then(|tool_measures| find_measure_slider_ready(tool_measures, "puzzle3d-fill-count"))
            .unwrap_or(0.0) as u32;
        // Request far beyond what a single tick could have planned.
        app.handle_action("setFillCount", Some(&json!({ "value": PUZZLE3D_FILL_COUNT_MAX })), &fill_view, &testkit::meta("local")).expect("setFillCount beyond available");
        let measures = app.tool_measures(&fill_view);
        let tool_measures = measures.get("fill").expect("fill tool measures");
        let clamped = find_measure_slider(tool_measures, "puzzle3d-fill-count").expect("fill-count slider value");
        assert!(clamped <= available_count as f64, "runtime.fill_count must clamp to what's actually planned, not the raw request");
        assert_eq!(clamped as usize, object_count(&app) - object_count_before, "the clamped measure value must match what the document actually materialized");
        let tick = app.handle_action("fillBuildTick", None, &fill_view, &testkit::meta("local")).expect("fillBuildTick after an above-ready request");
        assert!(
            !tick.requested_effects.iter().any(|effect| matches!(effect, HostEffect::DispatchAction { action, .. } if action == "setFillCount")),
            "fillBuildTick must never self-dispatch setFillCount — the clamp at commit time means fill_count can never run ahead of what's planned"
        );
    }

    #[test]
    fn fill_render_reveals_the_full_available_plan_tagged_with_reveal_index() {
        // 🪣️ `render()` now composes EVERY currently-planned piece (not just the committed
        // `fill_count`), each tagged `revealIndex` — the viewport applies its own live, main-thread
        // cutoff to show/hide them per drag value with zero WASM round trips. The committed cutoff is
        // separately exposed as `interactionJson.revealCutoffs["puzzle3d-fill"]`, which only advances
        // on `setFillCount` (the document itself stays untouched until then).
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let object_count_before = object_count(&app);
        let fill_view = ViewState { active_tool_id: Some("fill".into()), ..ViewState::default() };
        app.handle_action(SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": "fill" })), &fill_view, &testkit::meta("local")).expect("select fill tool");
        for _ in 0..64 {
            app.handle_action("fillBuildTick", None, &fill_view, &testkit::meta("local")).expect("fillBuildTick");
            let ready = app
                .tool_measures(&fill_view)
                .get("fill")
                .and_then(|tool_measures| find_measure_slider_ready(tool_measures, "puzzle3d-fill-count"))
                .unwrap_or(0.0);
            if ready >= 3.0 {
                break;
            }
        }
        let ready = app
            .tool_measures(&fill_view)
            .get("fill")
            .and_then(|tool_measures| find_measure_slider_ready(tool_measures, "puzzle3d-fill-count"))
            .unwrap_or(0.0) as usize;
        assert!(ready >= 3, "fill planning must expose at least three ready placements");
        assert_eq!(object_count(&app), object_count_before, "background planning must not mutate the document before setFillCount");

        let rendered = render_composite(&mut app);
        assert_eq!(instance_count(&rendered), object_count_before + ready, "render must already expose every planned piece, tagged for client-side reveal");
        let instances: Vec<Value> = rendered.pointer("/world3d/instancesJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or_default();
        let reveal_indices: Vec<u64> = instances.iter().skip(object_count_before).filter_map(|instance| instance.get("revealIndex").and_then(Value::as_u64)).collect();
        assert_eq!(reveal_indices.len(), ready, "every planned (not-yet-committed) instance must carry revealIndex");
        let mut sorted_indices = reveal_indices.clone();
        sorted_indices.sort_unstable();
        assert_eq!(sorted_indices, (0..ready as u64).collect::<Vec<_>>(), "revealIndex is a dense 0-based sequence matching plan order");
        // 🪣️ Untagged objects omit the `revealIndex` key entirely — a `null` would compare as `0`
        // against the host's boot cutoff and hide every ordinary object (see `world_instances_json`).
        let base_reveal_keys = instances.iter().take(object_count_before).filter(|instance| instance.get("revealIndex").is_some()).count();
        assert_eq!(base_reveal_keys, 0, "base (non-plan) objects never carry a revealIndex key, not even a null one");
        let interaction = interaction_of(&rendered);
        assert_eq!(interaction.pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(0), "nothing committed yet — the reveal cutoff mirrors runtime.fill_count (0)");
        assert_eq!(interaction.pointer("/fillBuild/appliedCount").and_then(Value::as_u64), Some(0));

        app.handle_action("setFillCount", Some(&json!({ "value": ready })), &fill_view, &testkit::meta("local")).expect("setFillCount");
        let after_commit = render_composite(&mut app);
        assert_eq!(instance_count(&after_commit), object_count_before + ready, "instance count is unchanged by commit — only the cutoff (and document) advanced");
        let committed_interaction = interaction_of(&after_commit);
        assert_eq!(committed_interaction.pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64), Some(ready as u64));
        assert_eq!(committed_interaction.pointer("/fillBuild/appliedCount").and_then(Value::as_u64), Some(ready as u64));
    }

    #[test]
    fn seeded_objects_omit_reveal_index_so_the_boot_cutoff_cannot_hide_them() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let rendered = render_composite(&mut app);
        let instances: Vec<Value> = rendered.pointer("/world3d/instancesJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str(raw).ok()).unwrap_or_default();
        assert!(!instances.is_empty(), "the default fixture seeds at least one object");
        for instance in &instances {
            assert!(instance.get("revealIndex").is_none(), "seeded object {} must omit revealIndex — a null coerces to 0 and the boot cutoff would hide its mesh", instance.get("id").and_then(Value::as_str).unwrap_or("?"));
        }
        assert_eq!(
            interaction_of(&rendered).pointer("/revealCutoffs/puzzle3d-fill").and_then(Value::as_u64),
            Some(0),
            "the boot cutoff really is 0 — this is the value that hid every mesh while revealIndex serialized as null"
        );
    }

    #[test]
    fn fill_build_tick_is_a_view_action_with_narrow_ui_scope() {
        use semio_framework_core::kernel::UiDirtyScope;
        let app = create_puzzle3d_app();
        let def = app.definition.actions.iter().find(|entry| entry.id == "fillBuildTick").expect("fillBuildTick declared");
        assert_eq!(def.kind, ActionKind::View, "fillBuildTick must stay a View action — it only advances background planning");
        let mut live = testkit::new_app::<Puzzle3dPlayApp>();
        let fill_view = ViewState { active_tool_id: Some("fill".into()), ..ViewState::default() };
        live.handle_action(SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": "fill" })), &fill_view, &testkit::meta("local")).expect("select fill tool");
        let result = live.handle_action("fillBuildTick", None, &fill_view, &testkit::meta("local")).expect("fillBuildTick");
        match result.ui_scope {
            UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
                assert_eq!(window_bodies, vec![PUZZLE3D_PLAY_BODY_COMPOSITE.to_string()]);
                assert_eq!(panel_bodies, vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()]);
                assert!(tools, "fill planning must refresh the fill-count slider range in the fill tool's measures");
                assert!(!measures);
                assert!(!engagements);
                assert!(!utilities);
                assert!(!labels);
            }
            other => panic!("expected a Partial ui_scope for fillBuildTick, got {other:?}"),
        }
    }

    #[test]
    fn set_fill_count_declares_narrow_ui_scope() {
        use semio_framework_core::kernel::UiDirtyScope;
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let fill_view = ViewState { active_tool_id: Some("fill".into()), ..ViewState::default() };
        app.handle_action(SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": "fill" })), &fill_view, &testkit::meta("local")).expect("select fill tool");
        let result = app.handle_action("setFillCount", Some(&json!({ "value": 1 })), &fill_view, &testkit::meta("local")).expect("setFillCount");
        match result.ui_scope {
            UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
                assert_eq!(window_bodies, vec![PUZZLE3D_PLAY_BODY_COMPOSITE.to_string()]);
                assert_eq!(panel_bodies, vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()]);
                assert!(tools);
                assert!(!measures);
                assert!(!engagements);
                assert!(!utilities);
                assert!(!labels);
            }
            other => panic!("expected a Partial ui_scope for setFillCount, got {other:?}"),
        }
    }

    #[test]
    fn set_object_kind_weight_declares_fill_options_ui_scope() {
        use semio_framework_core::kernel::UiDirtyScope;
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let fill_view = ViewState { active_tool_id: Some("fill".into()), ..ViewState::default() };
        app.handle_action(SET_ACTIVE_TOOL_ACTION_ID, Some(&json!({ "toolId": "fill" })), &fill_view, &testkit::meta("local")).expect("select fill tool");
        let object_ids = puzzle3d_kind_ids(&nakagin_fixture(), "objects");
        let kind_id = object_ids.first().expect("object kind");
        let result = app
            .handle_action("setObjectKindWeight", Some(&json!({ "kindId": kind_id, "value": 0.75 })), &fill_view, &testkit::meta("local"))
            .expect("setObjectKindWeight");
        match result.ui_scope {
            UiDirtyScope::Partial { window_bodies, panel_bodies, engagements, measures, utilities, tools, labels } => {
                assert_eq!(window_bodies, vec![PUZZLE3D_PLAY_BODY_COMPOSITE.to_string()]);
                assert_eq!(panel_bodies, vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()]);
                assert!(tools);
                assert!(measures, "distribution sliders live in tool + window measures");
                assert!(!engagements);
                assert!(!utilities);
                assert!(!labels);
            }
            other => panic!("expected a Partial ui_scope for setObjectKindWeight, got {other:?}"),
        }
    }

    #[test]
    fn fill_count_measure_shows_planning_progress_while_precompute_incomplete() {
        let mut session = Puzzle3dPrecomputeSession::new();
        let scene = Puzzle3dScene { fixture: nakagin_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: "fill".into() };
        sync_precompute_session(&mut session, &scene);
        session.precompute_step(1);
        match puzzle3d_fill_count_measure(&scene, &session, &PUZZLE3D_LABELS_NATIVE_EN) {
            WindowMeasure::Slider { label: Some(label), max, ready, loading, .. } => {
                assert_eq!(label, PUZZLE3D_LABELS_NATIVE_EN.count, "fill count label stays fixed as Count while planning");
                assert_eq!(max, PUZZLE3D_FILL_COUNT_MAX as f64, "fill slider max stays fixed while planning");
                let ready = ready.expect("planning must expose a ready extent");
                assert!(ready >= 0.0 && ready <= max, "ready extent must lie on the fixed range");
                assert_eq!(loading, Some(true), "planning must mark the measure tree leaf as loading");
            }
            other => panic!("expected a slider measure, got {other:?}"),
        }
    }

    #[test]
    fn puzzle3d_normalize_kind_weight_group_redistributes_siblings_proportionally() {
        let ids = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let mut weights = HashMap::from([("a".to_string(), 0.2), ("b".to_string(), 0.3), ("c".to_string(), 0.5)]);
        weights = puzzle3d_normalize_kind_weight_group(&weights, &ids, "a", 0.5);
        let sum: f64 = ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum();
        assert!((sum - 1.0).abs() < 1e-9, "simplex must stay at 1, got {sum}");
        assert!((weights.get("a").copied().unwrap_or(0.0) - 0.5).abs() < 1e-9);
        // b:c were 0.3:0.5 — remainder 0.5 splits 0.3/0.8 and 0.5/0.8
        assert!((weights.get("b").copied().unwrap_or(0.0) - 0.5 * 0.3 / 0.8).abs() < 1e-9);
        assert!((weights.get("c").copied().unwrap_or(0.0) - 0.5 * 0.5 / 0.8).abs() < 1e-9);
    }

    #[test]
    fn puzzle3d_vortex_measure_exposes_joint_weight_scaled_by_object() {
        let object_ids = vec!["Object".to_string(), "Placed".to_string()];
        let vortex_ids = vec!["c-b".to_string(), "b-s".to_string()];
        let object_weights = puzzle3d_uniform_kind_weights(&object_ids);
        let vortex_weights = HashMap::from([("c-b".to_string(), 0.75), ("b-s".to_string(), 0.25)]);
        let object_weight = *object_weights.get("Object").unwrap();
        let measures = puzzle3d_joint_vortex_measures("Object", object_weight, &vortex_ids, &vortex_weights);
        match &measures[0] {
            WindowMeasure::Slider { value, max, step, disabled, .. } => {
                let expected_joint = puzzle3d_joint_vortex_weight(object_weight, 0.75);
                assert!((*value - expected_joint).abs() < 1e-9, "slider must show P(object)×P(vortex), got {value}");
                assert!((*max - object_weight).abs() < 1e-9, "joint range max is P(object)");
                assert_eq!(*step, Some(object_weight * 0.01), "step tracks 1% of P(object)");
                assert_eq!(*disabled, None);
            }
            other => panic!("expected vortex slider, got {other:?}"),
        }
        let raised = puzzle3d_normalize_kind_weight_group(&object_weights, &object_ids, "Object", 0.8);
        let raised_weight = *raised.get("Object").unwrap();
        let raised_measures = puzzle3d_joint_vortex_measures("Object", raised_weight, &vortex_ids, &vortex_weights);
        match (&measures[0], &raised_measures[0]) {
            (WindowMeasure::Slider { value: before, .. }, WindowMeasure::Slider { value: after, .. }) => {
                assert!(*after > *before, "raising P(object) must raise joint vortex percentages");
                assert!((*after - raised_weight * 0.75).abs() < 1e-9);
            }
            _ => panic!("expected vortex sliders"),
        }
    }

    #[test]
    fn puzzle3d_distribution_lists_global_vortices_and_joints_sum_to_one() {
        let labels = puzzle3d_labels(&ViewState::default());
        let fixture = default_fixture();
        let object_ids = puzzle3d_kind_ids(&fixture, "objects");
        let vortex_ids = puzzle3d_kind_ids(&fixture, "vortices");
        assert!(object_ids.len() >= 2, "default fixture needs multiple object kinds");
        assert!(vortex_ids.len() >= 2, "default fixture needs multiple vortex kinds");
        let object_kind_weights = puzzle3d_uniform_kind_weights(&object_ids);
        let vortex_kind_weights = puzzle3d_uniform_kind_weights(&vortex_ids);
        let scene = Puzzle3dScene {
            fixture,
            runtime: Puzzle3dRuntime { object_kind_weights, vortex_kind_weights, ..Puzzle3dRuntime::default() },
            active_utility: "fill".into(),
        };
        let distribution_children = puzzle3d_distribution_children(&scene, labels, Some(true));
        assert_eq!(distribution_children.len(), object_ids.len());
        let mut joint_sum = 0.0;
        for measure in &distribution_children {
            let WindowMeasure::Group { children, value: Some(object_weight), .. } = measure else {
                panic!("expected object-kind group");
            };
            assert_eq!(children.len(), vortex_ids.len(), "each object must list the full global vortex catalog");
            let local_sum: f64 = children
                .iter()
                .map(|child| match child {
                    WindowMeasure::Slider { value, .. } => *value,
                    _ => panic!("expected vortex slider"),
                })
                .sum();
            assert!((local_sum - object_weight).abs() < 1e-6, "under one object joints sum to P(object), not 1");
            joint_sum += local_sum;
        }
        assert!((joint_sum - 1.0).abs() < 1e-6, "all nested joint percentages across objects must sum to 1, got {joint_sum}");
    }

    #[test]
    fn puzzle3d_object_weight_change_scales_joint_sampling_product() {
        let object_ids = vec!["Object".to_string(), "Placed".to_string()];
        let vortex_ids = vec!["c-b".to_string(), "b-s".to_string()];
        let mut object_weights = puzzle3d_uniform_kind_weights(&object_ids);
        let vortex_weights = puzzle3d_uniform_kind_weights(&vortex_ids);
        object_weights = puzzle3d_normalize_kind_weight_group(&object_weights, &object_ids, "Object", 0.6);
        let object_weight = *object_weights.get("Object").unwrap();
        let vortex_weight = *vortex_weights.get("c-b").unwrap();
        let joint_before = puzzle3d_joint_vortex_weight(0.5, vortex_weight);
        let joint_after = puzzle3d_joint_vortex_weight(object_weight, vortex_weight);
        assert!(joint_after > joint_before);
    }

    /// 🚫️ Zero object-kind weight disables every vortex slider under that kind — anything × 0 is 0.
    #[test]
    fn zero_object_kind_weight_disables_joint_vortex_sliders() {
        let labels = puzzle3d_labels(&ViewState::default());
        let session = Puzzle3dPrecomputeSession::new();
        let fixture = default_fixture();
        let object_ids = puzzle3d_kind_ids(&fixture, "objects");
        assert!(!object_ids.is_empty(), "default fixture must expose object kinds");
        let zeroed_id = object_ids[0].clone();
        let mut object_kind_weights = puzzle3d_uniform_kind_weights(&object_ids);
        object_kind_weights = puzzle3d_normalize_kind_weight_group(&object_kind_weights, &object_ids, &zeroed_id, 0.0);
        assert!(object_kind_weights.get(&zeroed_id).copied().unwrap_or(1.0) <= f64::EPSILON);
        let scene = Puzzle3dScene {
            fixture,
            runtime: Puzzle3dRuntime { object_kind_weights, ..Puzzle3dRuntime::default() },
            active_utility: "fill".into(),
        };
        let fill_tool_measures = puzzle3d_fill_tool_measures(&scene, &session, labels);
        let distribution_children = fill_tool_measures
            .iter()
            .find_map(|measure| match measure {
                WindowMeasure::Group { id, children, .. } if id == "puzzle3d-play-distribution" => Some(children.as_slice()),
                _ => None,
            })
            .expect("fill must expose a Distribution group");
        let zeroed_group = distribution_children
            .iter()
            .find(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == &format!("puzzle3d-play-distribution-object-{zeroed_id}")))
            .expect("zeroed object kind must appear in distribution");
        match zeroed_group {
            WindowMeasure::Group { value: Some(value), children, .. } => {
                assert!(*value <= f64::EPSILON, "object-kind header must read 0%");
                assert!(!children.is_empty(), "object kind must still list vortex sliders");
                assert!(
                    children.iter().all(|child| matches!(child, WindowMeasure::Slider { disabled: Some(true), value, .. } if *value <= f64::EPSILON)),
                    "every joint vortex slider under a 0% object kind must be disabled at 0%"
                );
            }
            other => panic!("expected object-kind group, got {other:?}"),
        }
        let live_group = distribution_children.iter().find(|measure| match measure {
            WindowMeasure::Group { id, value: Some(value), .. } if id != &format!("puzzle3d-play-distribution-object-{zeroed_id}") => *value > f64::EPSILON,
            _ => false,
        });
        if let Some(WindowMeasure::Group { children, .. }) = live_group {
            assert!(
                children.iter().all(|child| matches!(child, WindowMeasure::Slider { disabled: None | Some(false), .. })),
                "joint vortex sliders under a non-zero object kind must stay enabled"
            );
        }
    }

    /// 🎯️ Fill tool measures expose count + nested distribution tree under the Fill toggle.
    /// Volume Brush voxel dims live in a utility-options group in [`puzzle3d_window_measures`].
    #[test]
    fn fill_and_brush_params_are_tagged_utility_options_not_engagement_controls() {
        let labels = puzzle3d_labels(&ViewState::default());
        let session = Puzzle3dPrecomputeSession::new();
        let fill_scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: "fill".into() };
        let fill_tool_measures = puzzle3d_fill_tool_measures(&fill_scene, &session, labels);
        assert!(
            !fill_tool_measures.iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id == "puzzle3d-play-tool-options-fill")),
            "fill must not wrap its options in a nested Fill group — the tool toggle already owns that row"
        );
        assert_eq!(measure_group_tag(&fill_tool_measures, "puzzle3d-play-distribution"), Some(None));
        let distribution_children = fill_tool_measures
            .iter()
            .find_map(|measure| match measure {
                WindowMeasure::Group { id, children, .. } if id == "puzzle3d-play-distribution" => Some(children.as_slice()),
                _ => None,
            })
            .expect("fill must expose a Distribution group");
        assert!(!distribution_children.is_empty(), "distribution must list object-kind groups");
        assert!(
            distribution_children.iter().all(|measure| matches!(measure, WindowMeasure::Group { value: Some(_), on_change: Some(_), .. })),
            "each object-kind group must carry a header weight slider"
        );
        assert!(
            distribution_children.iter().all(|measure| match measure {
                WindowMeasure::Group { label, value: Some(_), on_change: Some(_), .. } => !label.contains('%'),
                _ => false,
            }),
            "object-kind group labels must not embed percentages — the header slider owns the value readout"
        );
        assert!(
            distribution_children.iter().any(|measure| match measure {
                WindowMeasure::Group { children, .. } => children.iter().any(|child| matches!(child, WindowMeasure::Slider { label: Some(label), .. } if !label.contains('%'))),
                _ => false,
            }),
            "vortex joint sliders must label kinds without embedding percentages"
        );
        assert!(find_measure_toggle(&fill_tool_measures, "puzzle3d-edit-volumes").is_none(), "fill must not carry edit-volumes toggle");
        assert_eq!(measure_group_tag(&fill_tool_measures, "puzzle3d-play-tool-options-voxel"), None, "fill must not carry voxel-dimension sliders");
        assert!(find_measure_slider(&fill_tool_measures, "puzzle3d-fill-count").is_some(), "fill-count slider always lives in the fill tool measures");
        assert!(
            !puzzle3d_window_measures(&fill_scene, &session, labels).iter().any(|measure| matches!(measure, WindowMeasure::Group { id, .. } if id.contains("fill"))),
            "fill must no longer surface in window_measures — it is a mode-level tool, not a window utility"
        );
        let volume_brush_scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: "volumeBrush".into() };
        assert_eq!(
            measure_group_tag(&puzzle3d_window_measures(&volume_brush_scene, &session, labels), "puzzle3d-play-utility-options-volume-brush"),
            Some(Some("volumeBrush".into()))
        );
        assert!(find_measure_slider(&puzzle3d_window_measures(&volume_brush_scene, &session, labels), "puzzle3d-voxel-w").is_some(), "volume brush utility exposes voxel width slider");
        let fill_engagement = puzzle3d_engagement(&fill_scene, &PUZZLE3D_LABELS_NATIVE_EN);
        assert!(fill_engagement.control.is_none() && fill_engagement.controls.is_none(), "fill engagement HUD must no longer carry the relocated controls");
        let brush_scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: "brush".into() };
        assert_eq!(measure_group_tag(&puzzle3d_window_measures(&brush_scene, &session, labels), "puzzle3d-play-utility-options-brush"), Some(Some("brush".into())));
        let brush_engagement = puzzle3d_engagement(&brush_scene, &PUZZLE3D_LABELS_NATIVE_EN);
        assert!(brush_engagement.control.is_none() && brush_engagement.controls.is_none(), "brush engagement HUD must no longer carry the relocated control");
        // 🖌️ Positive case: while already in the brush utility, opening a vortex's suggestions
        // selects it and drives precompute so real candidates exist — the brush Utility Options
        // group must then surface, tagged for "brush". One-shot suggestions outside brush mode
        // must not switch into brush just to show this group.
        let brush_view = ViewState { active_utility_id: Some("brush".into()), ..ViewState::default() };
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let vortex = first_vortex_full_id(&app);
        app.handle_action("openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 0.0, "y": 0.0 })), &brush_view, &testkit::meta("local")).expect("openVortexSuggestions");
        let brush_app_measures = app.window_measures(&brush_view);
        let window_measures = brush_app_measures.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("main window measures");
        assert_eq!(measure_group_tag(window_measures, "puzzle3d-play-utility-options-brush"), Some(Some("brush".into())), "the brush Utility Options group surfaces once there are candidates to place");
    }

    /// 🧰️ Context-menu / Alt+right-click suggestions are a one-shot placement: opening and accepting
    /// must leave whatever host-owned utility was already active (e.g. transform) untouched.
    #[test]
    fn open_and_accept_vortex_suggestions_preserve_active_utility() {
        let transform_view = ViewState { active_utility_id: Some("transform".into()), ..ViewState::default() };
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let vortex = first_vortex_full_id(&app);
        let open = app.handle_action("openVortexSuggestions", Some(&json!({ "fullId": vortex, "x": 0.0, "y": 0.0 })), &transform_view, &testkit::meta("local")).expect("openVortexSuggestions");
        assert!(
            open.requested_effects.iter().all(|effect| !matches!(effect, HostEffect::SetActiveUtility { .. } | HostEffect::SetActiveTool { .. })),
            "opening suggestions must not emit utility/tool switches: {:?}",
            open.requested_effects,
        );
        let open_node = app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &transform_view).expect("render");
        let open_interaction = interaction_of(&serde_json::to_value(&open_node).unwrap());
        assert_eq!(open_interaction.get("activeUtility").and_then(Value::as_str), Some("select"), "transform remains non-brush scene mode during suggestions");
        assert_eq!(open_interaction.pointer("/suggestionMenu/open").and_then(Value::as_bool), Some(true));
        assert!(brush_preview_of(&serde_json::to_value(&open_node).unwrap()).get("objectKindId").and_then(Value::as_str).is_some_and(|id| !id.is_empty()), "one-shot suggestions still emit a placement preview without entering brush mode");
        let accept = app.handle_action("acceptSuggestion", None, &transform_view, &testkit::meta("local")).expect("acceptSuggestion");
        assert!(
            accept.requested_effects.iter().all(|effect| !matches!(effect, HostEffect::SetActiveUtility { .. } | HostEffect::SetActiveTool { .. })),
            "accepting suggestions must not emit utility/tool switches: {:?}",
            accept.requested_effects,
        );
        let accept_interaction = interaction_of(&app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &transform_view).map(|node| serde_json::to_value(&node).unwrap()).expect("render"));
        assert!(accept_interaction.get("suggestionMenu").is_none_or(|menu| menu.is_null()));
        assert_eq!(accept_interaction.get("activeUtility").and_then(Value::as_str), Some("select"));
    }
    //#endregion 🧭️ Suggestions, select-then-open context menu, fill build progress (Round 2)

    //#region 🧰️ Window Actions & Utilities contract
    #[test]
    fn kinds_tree_object_drag_data_carries_object_kind_and_mesh_url() {
        let envelope = Puzzle3dScene { fixture: nakagin_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
        let labels = puzzle3d_labels(&ViewState::default());
        let node = build_kinds_tree(&envelope, &labels);
        let tree = match node {
            UiNode::Tree(tree) => tree,
            _ => panic!("expected kinds tree"),
        };
        let objects = tree.sections.iter().find(|section| section.id == "puzzle3d-play-kinds.objects").expect("objects section");
        let draggable = objects.items.iter().find(|item| item.draggable == Some(true)).expect("draggable object kind");
        let drag_data = draggable.drag_data.as_ref().expect("drag data");
        let encoded = drag_data.get(PUZZLE3D_CATALOGUE_DRAG_MIME).expect("catalogue mime");
        let payload: Value = serde_json::from_str(encoded).expect("drag payload json");
        assert!(payload.get("objectKind").and_then(Value::as_str).is_some(), "drag payload must carry objectKind");
        assert!(payload.get("meshUrl").and_then(Value::as_str).filter(|url| !url.is_empty()).is_some(), "drag payload must carry meshUrl for preview");
    }

    #[test]
    fn add_object_kind_honors_drop_origin() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let before = object_count(&app);
        app.handle_action("addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [2.5, 3.5, 0.0] })), &ViewState::default(), &testkit::meta("local")).expect("addObjectKind");
        assert_eq!(object_count(&app), before + 1);
        let projection = app.projection().expect("projection").0;
        let object = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).expect("added object");
        let origin = object.get("origin").and_then(Value::as_array).expect("origin array");
        assert_eq!(origin.first().and_then(Value::as_f64), Some(2.5));
        assert_eq!(origin.get(1).and_then(Value::as_f64), Some(3.5));
        assert_eq!(origin.get(2).and_then(Value::as_f64), Some(0.0));
    }

    #[test]
    fn add_object_kind_materializes_the_declared_kind_default() {
        // 📝️ P1 arg form: firing addObjectKind with no args must materialize the declared `objectKind`
        // default and emit the object-add operation under registry enforcement.
        let mut app = new_app_with_registry();
        app.handle_action("setActiveExample", Some(&json!({ "exampleId": "" })), &ViewState::default(), &testkit::meta("local")).expect("empty");
        let before = object_count(&app);
        let result = app.handle_action("addObjectKind", None, &ViewState::default(), &testkit::meta("local")).expect("addObjectKind");
        assert!(!result.operations.is_empty(), "addObjectKind is an Operation that emits operations");
        assert_eq!(object_count(&app), before + 1, "the materialized default kind adds exactly one object");
        let projection = app.projection().expect("projection").0;
        let kind = projection.get("objects").and_then(Value::as_array).and_then(|objects| objects.last()).and_then(|object| object.get("objectKind")).and_then(Value::as_str);
        assert_eq!(kind, Some("Object"), "the declared objectKind default was materialized host-side");
    }

    #[test]
    fn set_active_utility_emits_no_ops_and_no_history_entry() {
        // 🧰️ Switching utilities is the framework-injected View action: no document operations, no undo entry, no
        // re-emitted utility-switch effect (the host already applied `view_state.active_utility_id`).
        let mut app = new_app_with_registry();
        let before = app.projection().expect("projection").0;
        let brush_view = ViewState { active_utility_id: Some("brush".into()), ..ViewState::default() };
        let result = app.handle_action(SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "brush" })), &brush_view, &testkit::meta("local")).expect("switch utility");
        assert!(result.operations.is_empty(), "utility switching never emits document operations");
        assert!(result.requested_effects.is_empty(), "a user utility switch does not re-emit SetActiveUtility");
        assert_eq!(app.projection().expect("projection").0, before, "utility switching does not mutate the document");
    }

    #[test]
    fn set_hover_is_a_view_action_with_no_ops_after_document_mutation() {
        // 🖱️ After a real document edit the live store holds a `puzzle_3d`-shaped projection
        // (skip_serializing_if-elided optional fields). Hover must still round-trip as View-kind
        // with zero operations — not fall into a spurious SetDocument from serde shape noise.
        let mut app = new_app_with_registry();
        app.handle_action("addObjectKind", Some(&json!({ "objectKind": "Object", "origin": [1.0, 2.0, 3.0] })), &ViewState::default(), &testkit::meta("local")).expect("addObjectKind");
        let object_id = app
            .projection()
            .expect("projection")
            .0
            .get("objects")
            .and_then(Value::as_array)
            .and_then(|objects| objects.last())
            .and_then(|object| object.get("id"))
            .and_then(Value::as_str)
            .expect("added object id")
            .to_string();
        let before = app.projection().expect("projection").0;
        let result = app.handle_action("setHover", Some(&json!({ "objectId": object_id })), &ViewState::default(), &testkit::meta("local")).expect("setHover");
        assert!(result.operations.is_empty(), "setHover must not emit document operations");
        assert_eq!(app.projection().expect("projection").0, before, "setHover must not mutate the document");
        use semio_framework_core::kernel::{HostEffect, UiDirtyScope};
        match result.ui_scope {
            UiDirtyScope::Partial { window_bodies, panel_bodies, measures, utilities, tools, engagements, labels } => {
                assert!(window_bodies.is_empty());
                assert_eq!(panel_bodies, vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()]);
                assert!(!measures && !utilities && !tools && !engagements && !labels);
            }
            other => panic!("setHover must use chrome-only dirty scope, got {other:?}"),
        }
        assert!(result.requested_effects.iter().any(|effect| matches!(effect, HostEffect::PatchWorld3dChrome { .. })));
        let clear = app.handle_action("setHover", None, &ViewState::default(), &testkit::meta("local")).expect("clear hover");
        assert!(clear.operations.is_empty(), "clearing hover must not emit document operations");
    }

    #[test]
    fn engagement_exposes_no_utility_switch_options() {
        // 🧰️ select/brush/fill switching lives only on the framework utility bar (declared via `.utility` +
        // `.window_kind_utilities`); the engagement HUD must not duplicate it as options.
        let scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
        let engagement = puzzle3d_engagement(&scene, &PUZZLE3D_LABELS_NATIVE_EN);
        assert!(engagement.options.is_none(), "the puzzle3d engagement must not re-expose utility switching as options");
    }

    #[test]
    fn main_window_utilities_lead_with_transform_without_select_tool_and_no_default_utility() {
        let definition = create_puzzle3d_app().definition;
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert!(!utility_ids.contains(&"select"), "puzzle 3d must not declare a select utility");
        assert!(!utility_ids.contains(&"scale"), "puzzle 3d must not declare a scale utility");
        assert!(!utility_ids.contains(&"fill"), "fill is a mode-level tool, not a window utility");
        let main = definition.window_kinds.iter().find(|window| window.id == PUZZLE3D_PLAY_WINDOW_MAIN).expect("main window");
        let main_utilities: Vec<&str> = main.utilities.iter().map(|utility| utility.as_str()).collect();
        assert_eq!(main_utilities.first().copied(), Some("transform"));
        assert!(!main_utilities.contains(&"select"));
        assert!(!main_utilities.contains(&"fill"), "fill must not be bound to the main window as a utility");
        assert_eq!(PUZZLE3D_DEFAULT_UTILITY, "", "unset/cleared host utility must not impersonate transform");
    }

    /// 🛠️ Fill is a mode-level tool (a whole-document generator), not a window utility.
    #[test]
    fn tool_registry_declares_fill_tool() {
        use semio_framework_plugin::{ToolRef, SET_ACTIVE_TOOL_ACTION_ID};
        let definition = create_puzzle3d_app().definition;
        let tool_ids: Vec<&str> = definition.tools.iter().map(|tool| tool.id.as_str()).collect();
        assert_eq!(tool_ids, vec!["fill"]);
        assert_eq!(definition.modes[0].tools, vec![ToolRef::new("fill")]);
        assert!(definition.actions.iter().any(|action| action.id == SET_ACTIVE_TOOL_ACTION_ID), "declaring tools must inject the setActiveTool action");
    }

    #[test]
    fn world_select_emits_no_document_operations() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let before = app.projection().expect("projection").0.clone();
        let object_id = before
            .get("objects")
            .and_then(|value| value.as_array())
            .and_then(|objects| objects.first())
            .and_then(|object| object.get("id"))
            .and_then(|value| value.as_str())
            .expect("first object id")
            .to_string();
        let result = app
            .handle_action("worldSelect", Some(&json!({ "ids": [object_id], "merge": "replace" })), &ViewState::default(), &testkit::meta("local"))
            .expect("worldSelect");
        assert!(result.operations.is_empty(), "worldSelect is view-only and must not diff the document");
        assert_eq!(app.projection().expect("projection").0, before);
    }

    #[test]
    fn inspector_field_actions_resolve_selection_without_embedding_ids() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let object_id = app
            .projection()
            .expect("projection")
            .0
            .get("objects")
            .and_then(|value| value.as_array())
            .and_then(|objects| objects.first())
            .and_then(|object| object.get("id"))
            .and_then(|value| value.as_str())
            .expect("first object id")
            .to_string();
        app.handle_action("worldSelect", Some(&json!({ "ids": [object_id], "merge": "replace" })), &ViewState::default(), &testkit::meta("local")).expect("worldSelect");
        let node = app.render(PUZZLE3D_PLAY_BODY_INSPECTOR, None, &ViewState::default()).expect("render");
        let json = serde_json::to_value(&node).unwrap();
        let stepper = json
            .pointer("/sections/0/items")
            .and_then(|value| value.as_array())
            .and_then(|items| {
                items.iter().find_map(|item| {
                    item.get("items")
                        .and_then(|nested| nested.as_array())
                        .and_then(|axes| axes.first())
                        .and_then(|axis| axis.get("control"))
                        .filter(|control| control.get("type").and_then(Value::as_str) == Some("numberStepper"))
                })
            })
            .expect("nested stepper");
        let patch_args = stepper.get("onAbsolute").and_then(|value| value.get("args")).expect("patch args");
        assert!(patch_args.get("ids").is_none(), "inspector chrome must not embed selection ids in every action");
        assert_eq!(patch_args.get("entity").and_then(Value::as_str), Some("object"));
    }

    #[test]
    fn world_pick_declares_selection_ui_scope() {
        use semio_framework_core::kernel::{HostEffect, UiDirtyScope};
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let result = app.handle_action("worldPick", Some(&json!({ "id": 0, "merge": "replace" })), &ViewState::default(), &testkit::meta("local")).expect("worldPick");
        assert!(result.requested_effects.iter().any(|effect| matches!(effect, HostEffect::PatchWorld3dChrome { .. })));
        match result.ui_scope {
            UiDirtyScope::Partial { window_bodies, panel_bodies, measures, utilities, tools, engagements, labels } => {
                assert!(window_bodies.is_empty());
                assert!(panel_bodies.contains(&PUZZLE3D_PLAY_BODY_INSPECTOR.to_string()));
                assert!(!panel_bodies.contains(&PUZZLE3D_PLAY_BODY_DOCUMENT.to_string()));
                assert!(!measures && !utilities && !tools && !engagements && !labels);
            }
            other => panic!("worldPick must narrow dirty scope to selection surfaces, got {other:?}"),
        }
    }

    #[test]
    fn world_pick_keeps_instances_geometry_json_stable() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let before = serde_json::to_value(app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render")).unwrap();
        let instances_before = before.pointer("/world3d/instancesJson").and_then(Value::as_str).expect("instances").to_string();
        app.handle_action("worldPick", Some(&json!({ "id": 0, "merge": "replace" })), &ViewState::default(), &testkit::meta("local")).expect("worldPick");
        let after = serde_json::to_value(app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render")).unwrap();
        let instances_after = after.pointer("/world3d/instancesJson").and_then(Value::as_str).expect("instances");
        assert_eq!(instances_before, instances_after);
        let selection_after = after.pointer("/world3d/selectionJson").and_then(Value::as_str).expect("selection");
        assert!(selection_after.contains("\"ids\""));
    }

    #[test]
    fn world_pick_null_clears_without_reselecting_first_object() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        app.handle_action("worldPick", Some(&json!({ "id": 0, "merge": "replace" })), &ViewState::default(), &testkit::meta("local")).expect("pick");
        let selected_before_clear = selection_of(&render_composite(&mut app));
        assert!(selected_before_clear.get("ids").and_then(Value::as_array).is_some_and(|ids| !ids.is_empty()));
        app.handle_action("worldPick", Some(&json!({ "id": null, "merge": "replace" })), &ViewState::default(), &testkit::meta("local")).expect("clear");
        let selected_after_clear = selection_of(&render_composite(&mut app));
        assert_eq!(selected_after_clear.get("ids").and_then(Value::as_array).map(Vec::len), Some(0));
    }

    #[test]
    fn world_pick_locked_object_clears_like_background() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        app.handle_action("worldPick", Some(&json!({ "id": 0, "merge": "replace" })), &ViewState::default(), &testkit::meta("local")).expect("pick");
        let selected_id = selection_of(&render_composite(&mut app))
            .get("ids")
            .and_then(Value::as_array)
            .and_then(|ids| ids.first())
            .and_then(Value::as_str)
            .expect("selected id")
            .to_string();
        app.handle_action(
            "setSelectionFlag",
            Some(&json!({ "entity": "object", "ids": [selected_id], "flag": "locked", "value": true })),
            &ViewState::default(),
            &testkit::meta("local"),
        )
        .expect("lock");
        let instances = render_composite(&mut app)
            .pointer("/world3d/instancesJson")
            .and_then(Value::as_str)
            .and_then(|raw| serde_json::from_str::<Vec<Value>>(raw).ok())
            .unwrap_or_default();
        assert_eq!(instances.first().and_then(|entry| entry.get("disabled")).and_then(Value::as_bool), Some(true));
        app.handle_action("worldPick", Some(&json!({ "id": 0, "merge": "replace" })), &ViewState::default(), &testkit::meta("local")).expect("pick locked");
        let selected_after_locked_pick = selection_of(&render_composite(&mut app));
        assert_eq!(selected_after_locked_pick.get("ids").and_then(Value::as_array).map(Vec::len), Some(0));
    }

    #[test]
    fn world_vortices_only_emit_for_hovered_or_selected_objects() {
        // 🌀️ Default vortex show mode is Selected — idle hides markers; hover/selection reveals them.
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let all_vortex_ids = vortex_full_ids(&app);
        assert!(!all_vortex_ids.is_empty(), "fixture must expose vortices");
        let first_object_id = all_vortex_ids[0].split(':').next().expect("object id").to_string();
        let idle = render_composite(&mut app);
        let idle_vortices = idle.pointer("/world3d/vorticesJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str::<Vec<Value>>(raw).ok()).unwrap_or_default();
        assert!(idle_vortices.is_empty(), "idle scene must hide every vortex marker");

        app.handle_action("worldHover", Some(&json!({ "id": first_object_id })), &ViewState::default(), &testkit::meta("local")).expect("hover object");
        let hovered = render_composite(&mut app);
        let hovered_vortices = hovered.pointer("/world3d/vorticesJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str::<Vec<Value>>(raw).ok()).unwrap_or_default();
        assert!(!hovered_vortices.is_empty(), "hovered object must reveal its vortices");
        assert!(hovered_vortices.iter().all(|entry| entry.get("objectId").and_then(Value::as_str) == Some(first_object_id.as_str())));

        app.handle_action("worldHover", Some(&json!({ "id": null })), &ViewState::default(), &testkit::meta("local")).expect("clear hover");
        app.handle_action("worldPick", Some(&json!({ "id": 0, "merge": "replace" })), &ViewState::default(), &testkit::meta("local")).expect("select object");
        let selected = render_composite(&mut app);
        let selected_vortices = selected.pointer("/world3d/vorticesJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str::<Vec<Value>>(raw).ok()).unwrap_or_default();
        assert!(!selected_vortices.is_empty(), "selected object must reveal its vortices");

        app.handle_action("worldPick", Some(&json!({ "id": null, "merge": "replace" })), &ViewState::default(), &testkit::meta("local")).expect("clear selection");
        let cleared = render_composite(&mut app);
        let cleared_vortices = cleared.pointer("/world3d/vorticesJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str::<Vec<Value>>(raw).ok()).unwrap_or_default();
        assert!(cleared_vortices.is_empty(), "clearing selection must hide vortex markers again");
    }

    #[test]
    fn world_pick_object_replaces_vortex_selection() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let vortex = first_vortex_full_id(&app);
        app.handle_action("worldVortexSelect", Some(&json!({ "fullId": vortex, "merge": "default" })), &ViewState::default(), &testkit::meta("local")).expect("select vortex");
        app.handle_action("worldPick", Some(&json!({ "id": 0, "merge": "replace" })), &ViewState::default(), &testkit::meta("local")).expect("pick object");
        let node = render_composite(&mut app);
        let selection = selection_of(&node);
        assert!(selection.get("ids").and_then(Value::as_array).is_some_and(|ids| !ids.is_empty()));
        let vortices = node.pointer("/world3d/vorticesJson").and_then(Value::as_str).and_then(|raw| serde_json::from_str::<Vec<Value>>(raw).ok()).unwrap_or_default();
        assert!(!vortices.iter().any(|entry| entry.get("selected").and_then(Value::as_bool) == Some(true)));
    }

    #[test]
    fn world_vortex_select_clears_object_selection() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        app.handle_action("worldPick", Some(&json!({ "id": 0, "merge": "replace" })), &ViewState::default(), &testkit::meta("local")).expect("pick object");
        let vortex = first_vortex_full_id(&app);
        app.handle_action("worldVortexSelect", Some(&json!({ "fullId": vortex, "merge": "default" })), &ViewState::default(), &testkit::meta("local")).expect("select vortex");
        let selection = selection_of(&render_composite(&mut app));
        assert_eq!(selection.get("ids").and_then(Value::as_array).map(Vec::len), Some(0));
        assert!(selection.get("vortexIds").and_then(Value::as_array).is_some_and(|ids| ids.iter().any(|id| id.as_str() == Some(vortex.as_str()))));
    }

    #[test]
    fn world_vortex_click_replaces_until_invertive_mode_is_selected() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let vortices = vortex_full_ids(&app);
        assert!(vortices.len() >= 2, "fixture must expose two vortices");
        app.handle_action("worldVortexSelect", Some(&json!({ "fullId": vortices[0] })), &ViewState::default(), &testkit::meta("local")).expect("select first vortex");
        app.handle_action("worldVortexSelect", Some(&json!({ "fullId": vortices[1] })), &ViewState::default(), &testkit::meta("local")).expect("replace with second vortex");
        let selective = render_composite(&mut app);
        let selected: Vec<String> = selection_of(&selective)
            .get("vortexIds")
            .and_then(Value::as_array)
            .map(|ids| ids.iter().filter_map(Value::as_str).map(str::to_string).collect())
            .unwrap_or_default();
        assert_eq!(selected, vec![vortices[1].clone()]);
        assert_eq!(selection_of(&selective).get("selectionMergeMode").and_then(Value::as_str), Some("default"));

        app.handle_action("setSelectionModeDefault", Some(&json!({ "mode": "invertive" })), &ViewState::default(), &testkit::meta("local")).expect("enable invertive mode");
        app.handle_action("worldVortexSelect", Some(&json!({ "fullId": vortices[0] })), &ViewState::default(), &testkit::meta("local")).expect("toggle first vortex into selection");
        let invertive = render_composite(&mut app);
        let selected_count = selection_of(&invertive)
            .get("vortexIds")
            .and_then(Value::as_array)
            .map(|ids| ids.len())
            .unwrap_or(0);
        assert_eq!(selected_count, 2);
        assert_eq!(selection_of(&invertive).get("selectionMergeMode").and_then(Value::as_str), Some("invertive"));
    }

    #[test]
    fn gumball_active_only_for_transform_utilities_with_object_selection() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        app.handle_action("worldPick", Some(&json!({ "id": 0, "merge": "replace" })), &ViewState::default(), &testkit::meta("local")).expect("pick");
        let idle_selection = selection_of(&serde_json::to_value(app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &ViewState::default()).expect("render")).unwrap());
        assert_eq!(idle_selection.get("gumballActive").and_then(Value::as_bool), Some(false), "selection alone must not show the gumball");
        assert!(idle_selection.get("transformMode").is_none(), "non-transform utility must not emit transformMode");
        let transform_view = ViewState { active_utility_id: Some("transform".into()), ..ViewState::default() };
        let transform_selection = selection_of(&serde_json::to_value(app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &transform_view).expect("render")).unwrap());
        assert_eq!(transform_selection.get("gumballActive").and_then(Value::as_bool), Some(true));
        assert_eq!(transform_selection.get("transformMode").and_then(Value::as_str), Some("transform"));
        assert_eq!(transform_selection.pointer("/gumballConfig/moveAxes").and_then(Value::as_bool), Some(true));
        assert_eq!(transform_selection.pointer("/gumballConfig/rotate").and_then(Value::as_bool), Some(true));
        let brush_view = ViewState { active_utility_id: Some("brush".into()), ..ViewState::default() };
        let brush_selection = selection_of(&serde_json::to_value(app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &brush_view).expect("render")).unwrap());
        assert_eq!(brush_selection.get("gumballActive").and_then(Value::as_bool), Some(false));
        assert!(brush_selection.get("transformMode").is_none());
    }

    #[test]
    fn transform_utility_is_local_to_the_window_instance_not_shared_across_split_panes() {
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        app.handle_action("worldPick", Some(&json!({ "id": 0, "merge": "replace" })), &ViewState::default(), &testkit::meta("local")).expect("pick");
        let top = PUZZLE3D_PLAY_WINDOW_TOP;
        let perspective = PUZZLE3D_PLAY_WINDOW_PERSPECTIVE;
        let instances = vec![
            ViewWindowInstance { id: top.to_string(), window_kind_id: PUZZLE3D_PLAY_WINDOW_MAIN.to_string() },
            ViewWindowInstance { id: perspective.to_string(), window_kind_id: PUZZLE3D_PLAY_WINDOW_MAIN.to_string() },
        ];
        let mut active_utility_by_window_id = std::collections::HashMap::new();
        active_utility_by_window_id.insert(top.to_string(), "transform".to_string());
        let shared = ViewState { window_instances: instances, active_utility_by_window_id, ..ViewState::default() };
        let top_view = ViewState { window_id: Some(top.into()), ..shared.clone() };
        let perspective_view = ViewState { window_id: Some(perspective.into()), ..shared };
        let top_selection = selection_of(&serde_json::to_value(app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &top_view).expect("render top")).unwrap());
        assert_eq!(top_selection.get("gumballActive").and_then(Value::as_bool), Some(true), "transform on top pane must show the gumball");
        let perspective_selection = selection_of(&serde_json::to_value(app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &perspective_view).expect("render perspective")).unwrap());
        assert_eq!(perspective_selection.get("gumballActive").and_then(Value::as_bool), Some(false), "perspective pane must not inherit top pane's transform utility");
        assert!(perspective_selection.get("transformMode").is_none());
    }

    #[test]
    fn transform_utility_options_expose_move_and_rotate_flags() {
        let labels = puzzle3d_labels(&ViewState::default());
        let session = Puzzle3dPrecomputeSession::new();
        let scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: "transform".into() };
        let measures = puzzle3d_window_measures(&scene, &session, labels);
        assert_eq!(measure_group_tag(&measures, "puzzle3d-play-utility-options-transform"), Some(Some("transform".into())));
        assert_eq!(find_measure_toggle(&measures, "puzzle3d-transform-move"), Some(true));
        assert_eq!(find_measure_toggle(&measures, "puzzle3d-transform-rotate"), Some(true));
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        let transform_view = ViewState { active_utility_id: Some("transform".into()), ..ViewState::default() };
        app.handle_action("worldPick", Some(&json!({ "id": 0, "merge": "replace" })), &transform_view, &testkit::meta("local")).expect("pick");
        app.handle_action("setTransformGumballFlag", Some(&json!({ "flag": "rotate", "pressed": false })), &transform_view, &testkit::meta("local")).expect("disable rotate");
        let selection = selection_of(&serde_json::to_value(app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, &transform_view).expect("render")).unwrap());
        assert_eq!(selection.pointer("/gumballConfig/moveAxes").and_then(Value::as_bool), Some(true));
        assert_eq!(selection.pointer("/gumballConfig/rotate").and_then(Value::as_bool), Some(false));
        let app_measures = app.window_measures(&transform_view);
        let window_measures = app_measures.get(PUZZLE3D_PLAY_WINDOW_MAIN).expect("main window measures");
        assert_eq!(find_measure_toggle(window_measures, "puzzle3d-transform-rotate"), Some(false));
    }

    #[test]
    fn transform_engagement_does_not_block_background_deselect() {
        let scene = Puzzle3dScene { fixture: default_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: "transform".into() };
        let engagement = puzzle3d_engagement(&scene, &PUZZLE3D_LABELS_NATIVE_EN);
        assert_eq!(engagement.session_active, Some(false));
    }

    #[test]
    fn gumball_translate_drag_coalesces_into_one_edit() {
        // 🌀️ Unbracketed translate ticks still coalesce via AmendLast (compat path without transformBegin).
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        app.handle_action("setActiveExample", Some(&json!({ "exampleId": "" })), &ViewState::default(), &testkit::meta("local")).expect("empty");
        app.handle_action("addObjectKind", Some(&json!({ "objectKind": "Object" })), &ViewState::default(), &testkit::meta("local")).expect("add object");
        let object_id = app.projection().expect("projection").0.get("objects").and_then(Value::as_array).and_then(|objects| objects.first()).and_then(|object| object.get("id")).and_then(Value::as_str).expect("object id").to_string();
        let origin_before = |app: &VcsDocumentApp<Puzzle3dPlayApp>| -> Vec<f64> {
            app.projection().expect("projection").0.get("objects").and_then(Value::as_array).and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id.as_str()))).and_then(|object| object.get("origin")).and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()).unwrap_or_default()
        };
        let start = origin_before(&app);
        for dx in [1.0, 2.0, 3.0] {
            app.handle_action("translateSelection", Some(&json!({ "ids": [object_id], "dx": dx, "dy": 0.0, "dz": 0.0 })), &ViewState { active_utility_id: Some("transform".into()), ..ViewState::default() }, &testkit::meta("local")).expect("drag tick");
        }
        let dragged = origin_before(&app);
        assert!((dragged[0] - start[0] - 6.0).abs() < 1e-9, "three ticks accumulate 1+2+3 on x");
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("local")).expect("undo");
        assert_eq!(origin_before(&app), start, "one undo restores the whole coalesced gumball drag");
    }

    #[test]
    fn gumball_transform_session_commits_once_on_end() {
        // 🧲️ Scratch-commit: mid-drag ticks emit ZERO operations; transformEnd commits ONE edit from base→scratch.
        // Incremental host deltas accumulate on scratch — 1 then 5 → final +6.
        let mut app = testkit::new_app::<Puzzle3dPlayApp>();
        app.handle_action("setActiveExample", Some(&json!({ "exampleId": "" })), &ViewState::default(), &testkit::meta("local")).expect("empty");
        app.handle_action("addObjectKind", Some(&json!({ "objectKind": "Object" })), &ViewState::default(), &testkit::meta("local")).expect("add object");
        let object_id = app.projection().expect("projection").0.get("objects").and_then(Value::as_array).and_then(|objects| objects.first()).and_then(|object| object.get("id")).and_then(Value::as_str).expect("object id").to_string();
        let origin_of = |app: &VcsDocumentApp<Puzzle3dPlayApp>| -> Vec<f64> {
            app.projection().expect("projection").0.get("objects").and_then(Value::as_array).and_then(|objects| objects.iter().find(|object| object.get("id").and_then(Value::as_str) == Some(object_id.as_str()))).and_then(|object| object.get("origin")).and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()).unwrap_or_default()
        };
        let scratch_origin_of = |app: &mut VcsDocumentApp<Puzzle3dPlayApp>, view: &ViewState| -> Vec<f64> {
            let rendered = serde_json::to_value(app.render(PUZZLE3D_PLAY_BODY_COMPOSITE, None, view).expect("render")).expect("json");
            let instances: Vec<Value> = rendered.pointer("/world3d/instancesJson").and_then(Value::as_str).and_then(|json| serde_json::from_str(json).ok()).unwrap_or_default();
            instances.iter().find(|instance| instance.get("id").and_then(Value::as_str) == Some(object_id.as_str())).and_then(|instance| instance.get("position")).and_then(Value::as_array).map(|values| values.iter().filter_map(Value::as_f64).collect()).unwrap_or_default()
        };
        let transform_view = ViewState { active_utility_id: Some("transform".into()), ..ViewState::default() };
        app.handle_action("worldPick", Some(&json!({ "id": 0, "merge": "replace" })), &transform_view, &testkit::meta("local")).expect("pick");
        let start = origin_of(&app);
        app.handle_action("transformBegin", None, &transform_view, &testkit::meta("local")).expect("begin");
        let tick_a = app.handle_action("translateSelection", Some(&json!({ "ids": [object_id], "dx": 1.0, "dy": 0.0, "dz": 0.0 })), &transform_view, &testkit::meta("local")).expect("tick a");
        let tick_b = app.handle_action("translateSelection", Some(&json!({ "ids": [object_id], "dx": 5.0, "dy": 0.0, "dz": 0.0 })), &transform_view, &testkit::meta("local")).expect("tick b");
        assert!(tick_a.operations.is_empty() && tick_b.operations.is_empty(), "mid-drag transform ticks emit no operations");
        assert_eq!(origin_of(&app), start, "document stays at the drag-start pose mid-drag");
        let preview = scratch_origin_of(&mut app, &transform_view);
        assert!((preview[0] - start[0] - 6.0).abs() < 1e-9, "scratch render accumulates incremental ticks");
        let end = app.handle_action("transformEnd", None, &transform_view, &testkit::meta("local")).expect("end");
        assert_eq!(end.operations.len(), 1, "the whole drag commits as exactly one operation");
        let dragged = origin_of(&app);
        assert!((dragged[0] - start[0] - 6.0).abs() < 1e-9, "transformEnd lands on the accumulated total");
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("local")).expect("undo");
        assert_eq!(origin_of(&app), start, "one undo restores the whole scratch-committed gumball drag");
        app.handle_action("transformBegin", None, &transform_view, &testkit::meta("local")).expect("begin again");
        app.handle_action("translateSelection", Some(&json!({ "ids": [object_id], "dx": 2.0, "dy": 0.0, "dz": 0.0 })), &transform_view, &testkit::meta("local")).expect("second drag tick");
        app.handle_action("transformEnd", None, &transform_view, &testkit::meta("local")).expect("second end");
        let second = origin_of(&app);
        assert!((second[0] - start[0] - 2.0).abs() < 1e-9, "a second gumball drag session works from the restored base");
    }

    //#region 🔖️GesturePreview
    /// 🔬️ CW7 preview-law seam: `Puzzle3dPlayApp::gesture_preview` reads `transform_base`/
    /// `transform_scratch` only, never a `Puzzle3dOperation` — exercised directly against
    /// `Puzzle3dPlayApp` (bypassing the `VcsDocumentApp` wrapper, which has no accessor into the
    /// inner app) since `transform_drag_tick` is the natural per-tick gesture handler.
    #[test]
    fn gesture_preview_is_none_without_an_active_transform_drag() {
        let app = Puzzle3dPlayApp::default();
        assert!(app.gesture_preview().is_none(), "no live gumball drag, nothing to preview");
    }

    #[test]
    fn gesture_preview_reflects_the_live_gumball_drag_and_clears_on_commit() {
        let mut app = Puzzle3dPlayApp::default();
        let fixture = default_fixture();
        let object_id = fixture.objects[0].id.clone();
        let projection = serde_json::to_value(&fixture).unwrap();
        *app.transform_drag_active.borrow_mut() = true;

        let tick_a = app.transform_drag_tick("translateSelection", Some(&json!({ "ids": [object_id.clone()], "dx": 1.0, "dy": 0.0, "dz": 0.0 })), &projection);
        assert!(tick_a.operations.is_empty(), "mid-drag ticks emit zero operations (scratch-commit pattern)");
        let (key, seq_after_a, payload_a) = app.gesture_preview().expect("a live gumball drag is previewable");
        assert_eq!(key, "gesture:transform");
        let value_a: Value = serde_json::from_slice(&payload_a).expect("payload is valid json");
        assert!(!value_a["operations"].as_array().expect("operations array").is_empty(), "the delta anchored to the drag-start snapshot must reflect the first tick");

        let tick_b = app.transform_drag_tick("translateSelection", Some(&json!({ "ids": [object_id], "dx": 5.0, "dy": 0.0, "dz": 0.0 })), &projection);
        assert!(tick_b.operations.is_empty());
        let (_, seq_after_b, payload_b) = app.gesture_preview().expect("still live mid-drag");
        assert!(seq_after_b > seq_after_a, "seq is monotone per tick, for staleness detection on the receiving end");
        assert_ne!(payload_a, payload_b, "the base-anchored delta accumulates both ticks, not just the latest one");

        let end = app.commit_transform(&projection);
        assert_eq!(end.operations.len(), 1, "the whole drag commits as exactly one real operation");
        assert!(app.gesture_preview().is_none(), "the drag ended: nothing left to preview, and the commit above already carried the real operation");
    }

    #[test]
    fn gesture_preview_is_a_pure_read_never_mutating_the_transform_scratch() {
        let mut app = Puzzle3dPlayApp::default();
        let fixture = default_fixture();
        let object_id = fixture.objects[0].id.clone();
        let projection = serde_json::to_value(&fixture).unwrap();
        *app.transform_drag_active.borrow_mut() = true;
        app.transform_drag_tick("translateSelection", Some(&json!({ "ids": [object_id], "dx": 1.0, "dy": 0.0, "dz": 0.0 })), &projection);
        let scratch_before = app.transform_scratch.clone();
        let _ = app.gesture_preview();
        let _ = app.gesture_preview();
        assert_eq!(app.transform_scratch, scratch_before, "gesture_preview must never mutate the live transform scratch it reads");
    }
    //#endregion 🔖️GesturePreview
    //#endregion 🧰️ Window Actions & Utilities contract
}
//#endregion 🧪️Tests
