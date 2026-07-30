//! 👯 Puzzle 5d app — DocumentApp impl, render, manifest (constitutional: ui).

use puzzle_5d::Puzzle5dProjection;
use puzzle_5d_engine::{BrushPlacePayload, Puzzle5dPrecomputeSession};
use puzzle_5d_op::{puzzle5d_document_delta_operations, Puzzle5dOperation, Puzzle5dPlayProjection};
use semio_framework_os::{register_mesh_exporter, register_mesh_importer};
use semio_framework_plugin::{
    apply_world3d_sun_action, build_board2d_scene, build_world_3d_scene, create_default_layout,
    ActionArgDef, ActionArgOption, ActionDefinition, ActionEmit, ActionKind, DocumentApp, DocumentView, MeasureSelectItem, WindowEngagementStatus,
    merge_world_selection_ids, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_inspector_stepper_field, ui_inspector_vec3_group, ui_stack_vertical, ui_text, world3d_chunking_json, world3d_environment_json, world3d_mesh_id_from_url, world3d_meshes_json_from_urls, world3d_scene_extended, world3d_selection_json, world3d_sun_measures, App,
    ActionDescriptor, MediaClass, MediaForm, MediaType, OsMediaCapability, PanelGroup, ArtifactKindSpec, Board2dScene, SurfaceKind, UtilityCategory, UtilityDefinition, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, ViewState, WindowEngagement, ui_tree_stamp_presence, IconName,
    WindowEngagementInput, WindowMeasure, WorldSunConfig, is_de_locale, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, SET_ACTIVE_UTILITY_ACTION_ID,
};
use semio_framework_plugin::kernel::HostEffect;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;

//#region 🔖Constants
const PUZZLE5D_PLAY_APP_ID: &str = "puzzle5d-play";
const PUZZLE5D_PLAY_CONTROLLER_ID: &str = "puzzle5d-play";
const PUZZLE5D_PLAY_SURFACE_2D: &str = "puzzle.5d.play.2d";
const PUZZLE5D_PLAY_SURFACE_3D: &str = "puzzle.5d.play.3d";
const PUZZLE5D_PLAY_BODY_2D: &str = "puzzle.5d.play.2d";
const PUZZLE5D_PLAY_BODY_3D: &str = "puzzle.5d.play.3d";
const PUZZLE5D_PLAY_BODY_DOCUMENT: &str = "puzzle.5d.play.document";
const PUZZLE5D_PLAY_BODY_KINDS: &str = "puzzle.5d.play.kinds";
const PUZZLE5D_PLAY_BODY_INSPECTOR: &str = "puzzle.5d.play.inspector";
const PUZZLE5D_PLAY_WINDOW_2D: &str = "puzzle5d-2d";
const PUZZLE5D_PLAY_WINDOW_3D: &str = "puzzle5d-3d";
const PUZZLE5D_PLAY_WINDOWS: [&str; 2] = [PUZZLE5D_PLAY_WINDOW_2D, PUZZLE5D_PLAY_WINDOW_3D];
const PUZZLE5D_SCHEMA: &str = "puzzle.5d";
const PUZZLE5D_BOARD_FIXTURE_SCHEMA: &str = "puzzle.2d.fixture";
const PUZZLE5D_EXAMPLE_CONCRETE_FOREST: &str = "concrete-forest";
const PUZZLE5D_EXAMPLE_NAKAGIN: &str = "nakagin-capsule-tower";

const PUZZLE5D_FALLBACK_MESH_KIND: &str = "box";
/// 🧰 Host-owned active utility (`view_state.active_utility_id`) when the host hasn't set one yet — the first declared utility.
const PUZZLE5D_DEFAULT_UTILITY: &str = "select";
const PUZZLE5D_FILL_COUNT_MAX: u32 = 1000;
const PUZZLE5D_LOD_MODE_AUTOMATIC: &str = "automatic";
const PUZZLE5D_SUGGESTION_OFFSET_MIN: f64 = 0.0;
const PUZZLE5D_SUGGESTION_OFFSET_MAX: f64 = 160.0;
const PUZZLE5D_SUGGESTION_OFFSET_STEP: f64 = 4.0;
const PUZZLE5D_DEFAULT_SUGGESTION_OFFSET: f64 = 80.0;
const PUZZLE5D_DEFAULT_PART_RADIUS: f64 = 20.0;
const PUZZLE5D_BOARD_PLACEMENT_GAP: f64 = 16.0;
const PUZZLE5D_PROXIMITY_RADIUS: f64 = 0.75;

const CONCRETE_FOREST_EXAMPLE_DSL: &str = puzzle_5d_dsl::PUZZLE5D_CONCRETE_FOREST_EXAMPLE_TEXT;
const NAKAGIN_EXAMPLE_DSL: &str = puzzle_5d_dsl::PUZZLE5D_NAKAGIN_EXAMPLE_TEXT;
/// 🌉 This app's own scratch fixture stays a local structural-twin mirror (`Puzzle5dDocument`) of
/// `puzzle_5d::Puzzle5dProjection` — see `puzzle_5d`'s `🔖ValueBridge` region — so the DSL-text
/// example fixtures are parsed once into the typed `puzzle_5d::Puzzle5dProjection` and
/// re-serialized to the JSON string this module's `document_from_json`/`.example(...)` call sites expect.
static CONCRETE_FOREST_EXAMPLE_JSON: LazyLock<String> =
    LazyLock::new(|| serde_json::to_string(&<Puzzle5dProjection as store::DocumentDsl>::parse_dsl(CONCRETE_FOREST_EXAMPLE_DSL).expect("concrete-forest example fixture parses as dsl")).expect("serialize concrete-forest example fixture"));
static NAKAGIN_EXAMPLE_JSON: LazyLock<String> =
    LazyLock::new(|| serde_json::to_string(&<Puzzle5dProjection as store::DocumentDsl>::parse_dsl(NAKAGIN_EXAMPLE_DSL).expect("nakagin example fixture parses as dsl")).expect("serialize nakagin example fixture"));

static PUZZLE5D_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖Constants

//#region 🔖Terminology
/// 🗣️ Complete UI label set for the 5D app; one field per label makes every locale combination compile-checked.
struct Puzzle5dLabels {
    parts: &'static str,
    fasteners: &'static str,
    grips: &'static str,
    ropes: &'static str,
    part: &'static str,
    grip: &'static str,
    select: &'static str,
    brush: &'static str,
    fill: &'static str,
    count: &'static str,
    placement: &'static str,
    duplicate: &'static str,
    select_same_kind: &'static str,
    zoom_to_selection: &'static str,
    delete: &'static str,
    hide: &'static str,
    show: &'static str,
    lock: &'static str,
    unlock: &'static str,
    lod: &'static str,
    automatic: &'static str,
    suggestion: &'static str,
    offset: &'static str,
    part_weights: &'static str,
    grip_weights: &'static str,
    overlap: &'static str,
    window_2d: &'static str,
    window_3d: &'static str,
    // inspector field labels
    id: &'static str,
    kind: &'static str,
    label: &'static str,
    flat_text: &'static str,
    flat_x: &'static str,
    flat_y: &'static str,
    volume_origin: &'static str,
    flat_angle: &'static str,
    radius: &'static str,
    position: &'static str,
    direction: &'static str,
    source: &'static str,
    target: &'static str,
    schema: &'static str,
    utility: &'static str,
    none: &'static str,
    example_concrete_forest: &'static str,
}

const PUZZLE5D_LABELS_NATIVE_EN: Puzzle5dLabels = Puzzle5dLabels {
    parts: "Parts",
    fasteners: "Fasteners",
    grips: "Grips",
    ropes: "Ropes",
    part: "Part",
    grip: "Grip",
    select: "Select",
    brush: "Brush",
    fill: "Fill",
    count: "Count",
    placement: "Placement",
    duplicate: "Duplicate",
    select_same_kind: "Select all of same kind",
    zoom_to_selection: "Zoom to selection",
    delete: "Delete",
    hide: "Hide",
    show: "Show",
    lock: "Lock",
    unlock: "Unlock",
    lod: "LOD",
    automatic: "Automatic",
    suggestion: "Suggestion",
    offset: "Offset",
    part_weights: "Part Weights",
    grip_weights: "Grip Weights",
    overlap: "Overlap",
    window_2d: "Puzzle 2D",
    window_3d: "Puzzle 3D",
    id: "Id",
    kind: "Kind",
    label: "Label",
    flat_text: "Flat text",
    flat_x: "Flat x",
    flat_y: "Flat y",
    volume_origin: "Volume origin",
    flat_angle: "Flat angle",
    radius: "Radius",
    position: "Position",
    direction: "Direction",
    source: "Source",
    target: "Target",
    schema: "Schema",
    utility: "Utility",
    none: "(none)",
    example_concrete_forest: "Concrete Forest",
};

const PUZZLE5D_LABELS_NATIVE_DE: Puzzle5dLabels = Puzzle5dLabels {
    parts: "Teile",
    fasteners: "Verbinder",
    grips: "Griffe",
    ropes: "Seile",
    part: "Teil",
    grip: "Griff",
    select: "Auswählen",
    brush: "Pinsel",
    fill: "Füllen",
    count: "Anzahl",
    placement: "Platzierung",
    duplicate: "Duplizieren",
    select_same_kind: "Alle gleicher Art auswählen",
    zoom_to_selection: "Auf Auswahl zoomen",
    delete: "Löschen",
    show: "Anzeigen",
    hide: "Ausblenden",
    lock: "Sperren",
    unlock: "Entsperren",
    lod: "LOD",
    automatic: "Automatisch",
    suggestion: "Vorschlag",
    offset: "Versatz",
    part_weights: "Teilgewichte",
    grip_weights: "Griffgewichte",
    overlap: "Überlappung",
    window_2d: "Puzzle 2D",
    window_3d: "Puzzle 3D",
    id: "Id",
    kind: "Art",
    label: "Bezeichnung",
    flat_text: "Flachtext",
    flat_x: "Flach-X",
    flat_y: "Flach-Y",
    volume_origin: "Volumenursprung",
    flat_angle: "Flachwinkel",
    radius: "Radius",
    position: "Position",
    direction: "Richtung",
    source: "Quelle",
    target: "Ziel",
    schema: "Schema",
    utility: "Werkzeug",
    none: "(keine)",
    example_concrete_forest: "Betonwald",
};

const PUZZLE5D_LABELS_REUSE_EN: Puzzle5dLabels = Puzzle5dLabels {
    parts: "Building components",
    part: "Building component",
    grips: "Connection points",
    grip: "Connection point",
    fasteners: "Component connections",
    example_concrete_forest: "Abbau Aufbau",
    ..PUZZLE5D_LABELS_NATIVE_EN
};
const PUZZLE5D_LABELS_REUSE_DE: Puzzle5dLabels = Puzzle5dLabels {
    parts: "Baukomponenten",
    part: "Baukomponente",
    grips: "Verbindungspunkte",
    grip: "Verbindungspunkt",
    fasteners: "Baukomponentenverbindungen",
    example_concrete_forest: "Abbau Aufbau",
    ..PUZZLE5D_LABELS_NATIVE_DE
};

/// 🗣️ Resolves the active label set from the shell-provided locale/terminology; unknown terminology ids fall back to native — mirrors `puzzle3d_labels`.
/// ⚠️ Not routed through the SDK's `LocaleLabels`/`app_labels!`/`resolve_labels` — see `puzzle2d_labels`'s
/// doc comment for why (an extra terminology axis the SDK's `Terminology` region does not model).
fn puzzle5d_labels(view_state: &ViewState) -> &'static Puzzle5dLabels {
    let terminology = view_state.terminology.as_deref().unwrap_or("native");
    let is_de = is_de_locale(view_state);
    match (terminology, is_de) {
        ("reuse", true) => &PUZZLE5D_LABELS_REUSE_DE,
        ("reuse", false) => &PUZZLE5D_LABELS_REUSE_EN,
        (_, true) => &PUZZLE5D_LABELS_NATIVE_DE,
        (_, false) => &PUZZLE5D_LABELS_NATIVE_EN,
    }
}
//#endregion 🔖Terminology

//#region 🔖Document
fn one_f64() -> f64 {
    1.0
}

fn default_selection_method() -> String {
    "rectangle".into()
}

fn default_overlap_budget() -> f64 {
    0.02
}

fn default_lod_mode() -> String {
    PUZZLE5D_LOD_MODE_AUTOMATIC.into()
}

fn default_suggestion_offset() -> f64 {
    PUZZLE5D_DEFAULT_SUGGESTION_OFFSET
}

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dCamera2d {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default = "one_f64")]
    zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dCamera3d {
    #[serde(default)]
    position: [f64; 3],
    #[serde(default)]
    target: [f64; 3],
    #[serde(default = "one_f64")]
    zoom: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dGrip2d {
    #[serde(default)]
    angle: f64,
    #[serde(default, rename = "gripKind")]
    grip_kind: String,
    #[serde(default)]
    radius: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dGrip3d {
    #[serde(default)]
    position: [f64; 3],
    #[serde(default)]
    direction: Option<[f64; 3]>,
    #[serde(default)]
    radius: f64,
    #[serde(default)]
    label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dGrip {
    id: String,
    #[serde(default, rename = "gripKind")]
    grip_kind: String,
    #[serde(default, rename = "2d")]
    grip_2d: Puzzle5dGrip2d,
    #[serde(default, rename = "3d")]
    grip_3d: Puzzle5dGrip3d,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dFastener {
    id: String,
    source: String,
    target: String,
    #[serde(default, rename = "fastenerKind", skip_serializing_if = "Option::is_none")]
    fastener_kind: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dPart2d {
    #[serde(default)]
    x: f64,
    #[serde(default)]
    y: f64,
    #[serde(default)]
    shape: String,
    #[serde(default)]
    radius: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    width: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    height: Option<f64>,
    #[serde(default)]
    text: String,
    #[serde(default, rename = "iconKind", skip_serializing_if = "Option::is_none")]
    icon_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    locked: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dPart3d {
    #[serde(default)]
    origin: [f64; 3],
    #[serde(default, rename = "meshUrl")]
    mesh_url: Option<String>,
    #[serde(default)]
    orientation: Option<[f64; 4]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    scale: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dPart {
    id: String,
    #[serde(rename = "partKind")]
    part_kind: String,
    #[serde(default, rename = "2d")]
    part_2d: Puzzle5dPart2d,
    #[serde(default, rename = "3d")]
    part_3d: Puzzle5dPart3d,
    #[serde(default)]
    grips: Vec<Puzzle5dGrip>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dDocument {
    schema: String,
    #[serde(default)]
    domain: String,
    #[serde(default)]
    camera2d: Puzzle5dCamera2d,
    #[serde(default)]
    camera3d: Puzzle5dCamera3d,
    #[serde(default)]
    parts: Vec<Puzzle5dPart>,
    #[serde(default)]
    fasteners: Vec<Puzzle5dFastener>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    meta: Option<Value>,
    #[serde(default, rename = "kindCatalogs")]
    kind_catalogs: Option<Value>,
    #[serde(default, rename = "kindCompatibility")]
    kind_compatibility: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    label: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dSelection {
    #[serde(default)]
    part_ids: Vec<String>,
    #[serde(default)]
    grip_ids: Vec<String>,
    #[serde(default)]
    fastener_ids: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dRuntime {
    #[serde(default)]
    selection: Puzzle5dSelection,
    #[serde(default = "default_selection_method")]
    selection_method: String,
    #[serde(default)]
    hovered_part_id: Option<String>,
    #[serde(default)]
    fill_count: u32,
    #[serde(default)]
    brush_candidate_index: usize,
    #[serde(default = "default_overlap_budget")]
    overlap_budget: f64,
    #[serde(default = "default_lod_mode")]
    lod_mode: String,
    #[serde(default = "default_suggestion_offset")]
    suggestion_offset: f64,
    #[serde(default = "default_true")]
    grid_snap_enabled: bool,
    #[serde(default = "one_f64")]
    grid_factor: f64,
    #[serde(default)]
    engagement_input_by_window: BTreeMap<String, String>,
    #[serde(default)]
    object_kind_weights: HashMap<String, f64>,
    #[serde(default)]
    vortex_kind_weights: HashMap<String, f64>,
    #[serde(default)]
    sun: WorldSunConfig,
}

/// ⚠️ Explicit impl (not `#[derive(Default)]`) so Rust construction matches the serde field defaults above.
impl Default for Puzzle5dRuntime {
    fn default() -> Self {
        Self {
            selection: Puzzle5dSelection::default(),
            selection_method: default_selection_method(),
            hovered_part_id: None,
            fill_count: 0,
            brush_candidate_index: 0,
            overlap_budget: default_overlap_budget(),
            lod_mode: default_lod_mode(),
            suggestion_offset: default_suggestion_offset(),
            grid_snap_enabled: true,
            grid_factor: 1.0,
            engagement_input_by_window: BTreeMap::new(),
            object_kind_weights: HashMap::new(),
            vortex_kind_weights: HashMap::new(),
            sun: WorldSunConfig::default(),
        }
    }
}

/// 🧾 Transient render/mutation bundle pairing the persisted projection (the bare `Puzzle5dDocument`
/// json) with the app's ephemeral view state. Never persisted — the {@link VcsDocumentApp} store owns
/// the document and {@link Puzzle5dPlayApp} owns the runtime — but rebuilt per call so the existing
/// board/world/engagement helpers keep their `&scene` signatures.
#[derive(Clone)]
struct Puzzle5dScene {
    document: Puzzle5dDocument,
    runtime: Puzzle5dRuntime,
    /// 🧰 Host-owned active utility mirrored from `view_state.active_utility_id` — transient, never persisted.
    active_utility: String,
}

/// 🧰 The host-owned active utility for this view — per window instance via
/// `active_utility_by_window_id`, then the per-call `active_utility_id` overlay, then `select`.
fn puzzle5d_scene_active_utility(view_state: &ViewState, window_id: Option<&str>) -> String {
    if let Some(wid) = window_id {
        if let Some(utility) = view_state.active_utility_by_window_id.get(wid) {
            return utility.clone();
        }
    }
    view_state.active_utility_id.as_deref().unwrap_or(PUZZLE5D_DEFAULT_UTILITY).to_string()
}

/// 🧭 The select/brush/fill interaction mode the world engine reads, derived from the flat active utility
/// (the transform gumball utilities `move`/`rotate`/`scale` and `worldRelocate` all present as `select`).
fn puzzle5d_scene_mode(active_utility: &str) -> &str {
    match active_utility {
        "brush" => "brush",
        "fill" => "fill",
        _ => "select",
    }
}

/// 🎚️ The gumball handle the world engine draws when a transform utility is active.
fn puzzle5d_transform_handle(active_utility: &str) -> Option<&'static str> {
    match active_utility {
        "move" => Some("move"),
        "rotate" => Some("rotate"),
        "scale" => Some("scale"),
        _ => None,
    }
}

/// 🧭 Whether the active utility is a transform gumball mode.
fn puzzle5d_transform_utility_active(active_utility: &str) -> bool {
    puzzle5d_transform_handle(active_utility).is_some()
}

/// 🕹️ Whether the world gumball should render for the current selection and utility.
fn puzzle5d_gumball_active(runtime: &Puzzle5dRuntime, active_utility: &str) -> bool {
    !runtime.selection.part_ids.is_empty() && puzzle5d_transform_utility_active(active_utility)
}

/// 🧹 Clears every selection bag.
fn puzzle5d_clear_selection(selection: &mut Puzzle5dSelection) {
    *selection = Puzzle5dSelection::default();
}

/// 🧹 Clears every selection bag except part ids.
fn puzzle5d_clear_non_part_selection(selection: &mut Puzzle5dSelection) {
    selection.grip_ids.clear();
    selection.fastener_ids.clear();
}

/// 🧹 Clears every selection bag except grip ids.
fn puzzle5d_clear_non_grip_selection(selection: &mut Puzzle5dSelection) {
    selection.part_ids.clear();
    selection.fastener_ids.clear();
}

/// 🧭 Whether the engagement HUD should mark an active session for the given utility.
fn puzzle5d_engagement_session_active(window: &str, active_utility: &str) -> bool {
    match window {
        PUZZLE5D_PLAY_WINDOW_3D => matches!(active_utility, "brush" | "fill" | "worldRelocate"),
        _ => active_utility != "select",
    }
}

fn empty_document() -> Puzzle5dDocument {
    Puzzle5dDocument {
        schema: PUZZLE5D_SCHEMA.into(),
        domain: "architecture".into(),
        camera2d: Puzzle5dCamera2d { x: 0.0, y: 0.0, zoom: 1.0 },
        camera3d: Puzzle5dCamera3d { position: [8.0, -8.0, 8.0], target: [0.0, 0.0, 0.0], zoom: 1.0 },
        parts: Vec::new(),
        fasteners: Vec::new(),
        meta: None,
        kind_catalogs: None,
        kind_compatibility: None,
        label: None,
    }
}

fn document_from_json(json_text: &str) -> Puzzle5dDocument {
    serde_json::from_str::<Puzzle5dDocument>(json_text).unwrap_or_else(|_| empty_document())
}

fn default_document() -> Puzzle5dDocument {
    document_from_json(CONCRETE_FOREST_EXAMPLE_JSON.as_str())
}

/// 🧾 Materializes the transient scene from the persisted projection (bare document json) and the
/// app's current view state; an unparseable projection degrades to an empty document.
fn scene_from_projection(projection: &Value, runtime: Puzzle5dRuntime, active_utility: &str) -> Puzzle5dScene {
    let document = serde_json::from_value::<Puzzle5dDocument>(projection.clone()).unwrap_or_else(|_| empty_document());
    Puzzle5dScene { document, runtime, active_utility: active_utility.to_string() }
}

/// 🧮 Document ops for a document mutation — normalizes `before` through the same program typed
/// round-trip as `after` so View-kind actions that only touch runtime never trip the
/// "must not emit operations" guard when the live store still holds a `puzzle_5d`-shaped
/// projection from a prior op apply.
fn puzzle5d_operations_from_document_change(before: &Value, after_document: &Puzzle5dDocument) -> Vec<Puzzle5dOperation> {
    let before_normalized = serde_json::to_value(serde_json::from_value::<Puzzle5dDocument>(before.clone()).unwrap_or_else(|_| empty_document())).unwrap_or_else(|_| before.clone());
    let after = serde_json::to_value(after_document).unwrap_or_else(|_| before_normalized.clone());
    puzzle5d_document_delta_operations(&before_normalized, &after)
}

/// 🪟 Live window-instance ids of `kind_id` from `view_state.window_instances`, falling back to
/// `vec![kind_id]` when the list is empty — a headless/test call that never threads instances still
/// gets exactly the one entry today's single-instance-per-window callers expect.
fn window_instance_ids(view_state: &ViewState, kind_id: &str) -> Vec<String> {
    let ids: Vec<String> = view_state.window_instances.iter().filter(|instance| instance.window_kind_id == kind_id).map(|instance| instance.id.clone()).collect();
    if ids.is_empty() { vec![kind_id.to_string()] } else { ids }
}

fn puzzle5d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: PUZZLE5D_PLAY_CONTROLLER_ID.into(), action: action.into(), args }
}

fn puzzle5d_grip_full_id(part_id: &str, grip_id: &str) -> String {
    if grip_id.contains(':') {
        grip_id.to_string()
    } else {
        format!("{part_id}:{grip_id}")
    }
}

fn next_part_id() -> String {
    let next = PUZZLE5D_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("part-{next}")
}

fn next_fastener_id() -> String {
    let next = PUZZLE5D_ID_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    format!("fastener-{next}")
}

/** @emoji 📐 Resolves one numeric-field edit: an absolute `value` (typed entry) wins when
 * present, otherwise a `delta` (stepper nudge) is added to `current`. `None` when neither parses. */
fn puzzle5d_resolve_number_edit(current: f64, value: Option<&Value>, delta: Option<&Value>) -> Option<f64> {
    if let Some(absolute) = value.and_then(Value::as_f64) {
        return Some(absolute);
    }
    delta.and_then(Value::as_f64).map(|delta| current + delta)
}

/** @emoji 📐 Parses a nested stepper-group field id as `"<base>.<axis>"` (`x`/`y`/`z`), returning
 * the axis index when `field` names a component of `base` — the dot-path convention
 * `ui_inspector_vec3_group` uses for its per-axis actions. */
fn puzzle5d_axis_index(field: &str, base: &str) -> Option<usize> {
    match field.strip_prefix(base)?.strip_prefix('.')? {
        "x" => Some(0),
        "y" => Some(1),
        "z" => Some(2),
        _ => None,
    }
}

fn resolve_part_mesh_url(part: &Puzzle5dPart, kind_catalogs: Option<&Value>) -> Option<String> {
    if let Some(url) = part.part_3d.mesh_url.as_ref().filter(|url| !url.is_empty()) {
        return Some(url.clone());
    }
    resolve_part_kind_mesh_url(&part.part_kind, kind_catalogs)
}

fn resolve_part_kind_mesh_url(part_kind: &str, kind_catalogs: Option<&Value>) -> Option<String> {
    let parts = kind_catalogs?.get("parts")?.as_array()?;
    parts.iter().find(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(part_kind)).and_then(|entry| entry.get("meshUrl").and_then(|v| v.as_str()).map(str::to_string))
}

fn collect_mesh_urls(document: &Puzzle5dDocument) -> Vec<String> {
    let mut urls = HashSet::new();
    for part in &document.parts {
        if let Some(url) = resolve_part_mesh_url(part, document.kind_catalogs.as_ref()) {
            urls.insert(url);
        }
    }
    if let Some(parts) = document.kind_catalogs.as_ref().and_then(|catalogs| catalogs.get("parts")).and_then(|v| v.as_array()) {
        for entry in parts {
            if let Some(url) = entry.get("meshUrl").and_then(|v| v.as_str()) {
                urls.insert(url.to_string());
            }
        }
    }
    urls.into_iter().collect()
}

fn part_kind_grip_templates(document: &Puzzle5dDocument, part_kind: &str) -> Vec<Value> {
    document
        .kind_catalogs
        .as_ref()
        .and_then(|catalogs| catalogs.get("parts"))
        .and_then(|parts| parts.as_array())
        .and_then(|parts| parts.iter().find(|entry| entry.get("id").and_then(|v| v.as_str()) == Some(part_kind)))
        .and_then(|entry| entry.get("grips"))
        .and_then(|grips| grips.as_array())
        .cloned()
        .unwrap_or_default()
}

fn grips_from_templates(document: &Puzzle5dDocument, part_kind: &str) -> Vec<Puzzle5dGrip> {
    part_kind_grip_templates(document, part_kind)
        .iter()
        .enumerate()
        .map(|(index, template)| {
            let grip_kind = template.get("gripKind").and_then(|v| v.as_str()).unwrap_or("grip").to_string();
            let grip_2d: Puzzle5dGrip2d = template.get("2d").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
            let grip_3d: Puzzle5dGrip3d = template.get("3d").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
            Puzzle5dGrip { id: format!("v{index}"), grip_kind, grip_2d, grip_3d }
        })
        .collect()
}

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

fn world_grip_position(part: &Puzzle5dPart, grip: &Puzzle5dGrip) -> [f64; 3] {
    let orientation = part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let rotated = quat_rotate_vector(orientation, grip.grip_3d.position);
    [part.part_3d.origin[0] + rotated[0], part.part_3d.origin[1] + rotated[1], part.part_3d.origin[2] + rotated[2]]
}

fn world_grip_direction(part: &Puzzle5dPart, grip: &Puzzle5dGrip) -> [f64; 3] {
    let direction = grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]);
    quat_rotate_vector(part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), direction)
}

fn resolve_grip_world_position(document: &Puzzle5dDocument, full_id: &str) -> Option<[f64; 3]> {
    for part in &document.parts {
        for grip in &part.grips {
            if puzzle5d_grip_full_id(&part.id, &grip.id) == full_id {
                return Some(world_grip_position(part, grip));
            }
        }
    }
    None
}

fn find_part_by_grip_full_id<'a>(document: &'a Puzzle5dDocument, full_id: &str) -> Option<(&'a Puzzle5dPart, &'a Puzzle5dGrip)> {
    for part in &document.parts {
        for grip in &part.grips {
            if puzzle5d_grip_full_id(&part.id, &grip.id) == full_id {
                return Some((part, grip));
            }
        }
    }
    None
}

fn strip_tree_prefix(id: &str) -> &str {
    for prefix in ["puzzle5d-play-document.part.", "puzzle5d-play-document.grip.", "puzzle5d-play-document.fastener."] {
        if let Some(rest) = id.strip_prefix(prefix) {
            return rest;
        }
    }
    id
}

fn classify_selection(document: &Puzzle5dDocument, ids: &[String]) -> Puzzle5dSelection {
    let part_ids: HashSet<&str> = document.parts.iter().map(|part| part.id.as_str()).collect();
    let fastener_ids: HashSet<&str> = document.fasteners.iter().map(|fastener| fastener.id.as_str()).collect();
    let grip_ids: HashSet<String> = document.parts.iter().flat_map(|part| part.grips.iter().map(|grip| puzzle5d_grip_full_id(&part.id, &grip.id))).collect();
    let mut selection = Puzzle5dSelection::default();
    for raw in ids {
        let id = strip_tree_prefix(raw);
        if part_ids.contains(id) {
            selection.part_ids.push(id.to_string());
        } else if fastener_ids.contains(id) {
            selection.fastener_ids.push(id.to_string());
        } else if grip_ids.contains(id) {
            selection.grip_ids.push(id.to_string());
        }
    }
    selection
}

fn selection_flat_ids(selection: &Puzzle5dSelection) -> Vec<String> {
    selection.part_ids.iter().chain(selection.grip_ids.iter()).chain(selection.fastener_ids.iter()).cloned().collect()
}

fn mesh_selection_ids(args: Option<&Value>, fallback: &[String]) -> Vec<String> {
    args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()).filter(|ids| !ids.is_empty()).unwrap_or_else(|| fallback.to_vec())
}

fn remove_parts(document: &mut Puzzle5dDocument, part_ids: &[String]) {
    let removed_grips: Vec<String> = document.parts.iter().filter(|part| part_ids.contains(&part.id)).flat_map(|part| part.grips.iter().map(|grip| puzzle5d_grip_full_id(&part.id, &grip.id))).collect();
    document.parts.retain(|part| !part_ids.contains(&part.id));
    document.fasteners.retain(|fastener| !removed_grips.contains(&fastener.source) && !removed_grips.contains(&fastener.target));
}

fn remove_grips(document: &mut Puzzle5dDocument, grip_full_ids: &[String]) {
    if grip_full_ids.is_empty() {
        return;
    }
    for part in &mut document.parts {
        let part_id = part.id.clone();
        part.grips.retain(|grip| !grip_full_ids.contains(&puzzle5d_grip_full_id(&part_id, &grip.id)));
    }
    document.fasteners.retain(|fastener| !grip_full_ids.contains(&fastener.source) && !grip_full_ids.contains(&fastener.target));
}
//#endregion 🔖Document

//#region 🔖Board
fn board_camera_value(camera: &Puzzle5dCamera2d) -> Value {
    json!({ "x": camera.x, "y": camera.y, "zoom": camera.zoom })
}

fn board_node_value(part: &Puzzle5dPart) -> Value {
    let shape = if part.part_2d.shape.is_empty() { "circle" } else { part.part_2d.shape.as_str() };
    let handles: Vec<Value> = part
        .grips
        .iter()
        .map(|grip| {
            json!({
                "id": puzzle5d_grip_full_id(&part.id, &grip.id),
                "handleKind": if grip.grip_kind.is_empty() { grip.grip_2d.grip_kind.clone() } else { grip.grip_kind.clone() },
                "angle": grip.grip_2d.angle,
                "radius": if grip.grip_2d.radius > 0.0 { grip.grip_2d.radius } else { 3.0 },
            })
        })
        .collect();
    let mut node = json!({
        "id": part.id,
        "nodeKind": part.part_kind,
        "shape": shape,
        "x": part.part_2d.x,
        "y": part.part_2d.y,
        "text": part.part_2d.text,
        "handles": handles,
    });
    if shape == "rectangle" {
        node["width"] = json!(part.part_2d.width.unwrap_or(48.0));
        node["height"] = json!(part.part_2d.height.unwrap_or(48.0));
    } else {
        node["radius"] = json!(if part.part_2d.radius > 0.0 { part.part_2d.radius } else { PUZZLE5D_DEFAULT_PART_RADIUS });
    }
    if let Some(icon) = part.part_2d.icon_kind.as_ref() {
        node["iconKind"] = json!(icon);
    }
    if let Some(hidden) = part.part_2d.hidden {
        node["hidden"] = json!(hidden);
    }
    if let Some(locked) = part.part_2d.locked {
        node["locked"] = json!(locked);
    }
    node
}

/// 🗂️ Projects the unified 5d kind bundle (`parts/grips/fasteners/ropes`) to the board's `nodes/handles/edges/wires` naming.
fn board_kind_catalogs_value(document: &Puzzle5dDocument) -> Value {
    let catalogs = document.kind_catalogs.clone().unwrap_or(json!({}));
    json!({
        "nodes": catalogs.get("parts").cloned().unwrap_or(json!([])),
        "handles": catalogs.get("grips").cloned().unwrap_or(json!([])),
        "edges": catalogs.get("fasteners").cloned().unwrap_or(json!([])),
        "wires": catalogs.get("ropes").cloned().unwrap_or(json!([])),
    })
}

fn board_fixture_value(document: &Puzzle5dDocument) -> Value {
    let nodes: Vec<Value> = document.parts.iter().map(board_node_value).collect();
    let edges: Vec<Value> = document
        .fasteners
        .iter()
        .map(|fastener| {
            json!({
                "id": fastener.id,
                "edgeKind": fastener.fastener_kind.clone().unwrap_or_else(|| "link".into()),
                "source": fastener.source,
                "target": fastener.target,
            })
        })
        .collect();
    json!({
        "schema": PUZZLE5D_BOARD_FIXTURE_SCHEMA,
        "camera": board_camera_value(&document.camera2d),
        "nodes": nodes,
        "edges": edges,
        "wires": [],
        "meta": {
            "kindCatalogs": board_kind_catalogs_value(document),
            "kindCompatibility": document.kind_compatibility.clone().unwrap_or(json!([])),
        },
    })
}

fn board_brush_weights_json(runtime: &Puzzle5dRuntime) -> String {
    json!({ "nodeWeights": runtime.object_kind_weights, "handleWeights": runtime.vortex_kind_weights }).to_string()
}

fn puzzle5d_board_scene(envelope: &Puzzle5dScene) -> Board2dScene {
    Board2dScene {
        fixture_json: board_fixture_value(&envelope.document).to_string(),
        camera_json: board_camera_value(&envelope.document.camera2d).to_string(),
        glyph_catalogs_json: board_kind_catalogs_value(&envelope.document).to_string(),
        selection_json: serde_json::to_string(&selection_flat_ids(&envelope.runtime.selection)).unwrap_or_else(|_| "[]".into()),
        interactive: true,
        hovered_id: envelope.runtime.hovered_part_id.clone(),
        active_utility: Some(puzzle5d_scene_mode(&envelope.active_utility).to_string()),
        selection_method: envelope.runtime.selection_method.clone(),
        grid_snap_enabled: envelope.runtime.grid_snap_enabled,
        grid_factor: envelope.runtime.grid_factor,
        suggestion_offset: envelope.runtime.suggestion_offset,
        brush_weights_json: board_brush_weights_json(&envelope.runtime),
        placement_compatibility_json: envelope.document.kind_compatibility.clone().unwrap_or(json!([])).to_string(),
        lod_mode: envelope.runtime.lod_mode.clone(),
    }
}

fn set_part_2d_position(document: &mut Puzzle5dDocument, part_id: &str, x: Option<f64>, y: Option<f64>) {
    if let Some(part) = document.parts.iter_mut().find(|part| part.id == part_id) {
        if let Some(x) = x {
            part.part_2d.x = x;
        }
        if let Some(y) = y {
            part.part_2d.y = y;
        }
    }
}

/// 🎨 Palette drop: creates a free paired part at the flat drop point, deriving the volume origin from the nearest peer part's offset.
fn add_palette_part(envelope: &mut Puzzle5dScene, part_kind: &str, x: f64, y: f64) {
    let flat_to_world = 1.0 / 48.0;
    let origin = envelope
        .document
        .parts
        .first()
        .map(|peer| [peer.part_3d.origin[0] + (x - peer.part_2d.x) * flat_to_world, peer.part_3d.origin[1] - (y - peer.part_2d.y) * flat_to_world, peer.part_3d.origin[2]])
        .unwrap_or([x * flat_to_world, -y * flat_to_world, 0.0]);
    let id = next_part_id();
    let mesh_url = resolve_part_kind_mesh_url(part_kind, envelope.document.kind_catalogs.as_ref());
    let grips = grips_from_templates(&envelope.document, part_kind);
    envelope.document.parts.push(Puzzle5dPart {
        id: id.clone(),
        part_kind: part_kind.into(),
        part_2d: Puzzle5dPart2d { x, y, shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: part_kind.into(), icon_kind: None, hidden: None, locked: None },
        part_3d: Puzzle5dPart3d { origin, mesh_url, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
        grips,
    });
    envelope.runtime.selection = Puzzle5dSelection { part_ids: vec![id], grip_ids: Vec::new(), fastener_ids: Vec::new() };
}
//#endregion 🔖Board

//#region 🔖Engine
/// 🧠 Maps the unified 5d kind bundle to the puzzle 3d engine naming (`objects` with `vortices` templates, `vortices`, `cables`).
fn engine_kind_catalogs_value(document: &Puzzle5dDocument) -> Option<Value> {
    let catalogs = document.kind_catalogs.as_ref()?;
    let objects: Vec<Value> = catalogs
        .get("parts")
        .and_then(|parts| parts.as_array())
        .into_iter()
        .flatten()
        .map(|entry| {
            let mut object = entry.clone();
            let vortices: Vec<Value> = entry
                .get("grips")
                .and_then(|grips| grips.as_array())
                .into_iter()
                .flatten()
                .map(|template| {
                    let volume = template.get("3d").cloned().unwrap_or(json!({}));
                    json!({
                        "vortexKind": template.get("gripKind").cloned().unwrap_or(json!("grip")),
                        "position": volume.get("position").cloned().unwrap_or(json!([0.0, 0.0, 0.0])),
                        "direction": volume.get("direction").cloned().unwrap_or(json!([0.0, 0.0, -1.0])),
                        "radius": volume.get("radius").cloned().unwrap_or(json!(0.36)),
                    })
                })
                .collect();
            if let Some(object) = object.as_object_mut() {
                object.remove("grips");
                object.insert("vortices".into(), json!(vortices));
            }
            object
        })
        .collect();
    Some(json!({
        "objects": objects,
        "vortices": catalogs.get("grips").cloned().unwrap_or(json!([])),
        "cables": catalogs.get("ropes").cloned().unwrap_or(json!([])),
    }))
}

fn scene_config_json(envelope: &Puzzle5dScene) -> String {
    let objects: Vec<Value> = envelope
        .document
        .parts
        .iter()
        .map(|part| {
            json!({
                "id": part.id,
                "objectKind": part.part_kind,
                "meshUrl": resolve_part_mesh_url(part, envelope.document.kind_catalogs.as_ref()),
                "origin": part.part_3d.origin,
                "orientation": part.part_3d.orientation,
                "scale": part.part_3d.scale,
                "vortices": part.grips.iter().map(|grip| json!({
                    "id": grip.id,
                    "vortexKind": if grip.grip_kind.is_empty() { grip.grip_2d.grip_kind.clone() } else { grip.grip_kind.clone() },
                    "position": grip.grip_3d.position,
                    "direction": grip.grip_3d.direction,
                })).collect::<Vec<_>>(),
            })
        })
        .collect();
    let attractions: Vec<Value> = envelope.document.fasteners.iter().map(|fastener| json!({ "id": fastener.id, "attracting": fastener.source, "attracted": fastener.target })).collect();
    json!({
        "fixture": {
            "objects": objects,
            "attractions": attractions,
            "targetVolumes": [],
        },
        "kindCatalogs": engine_kind_catalogs_value(&envelope.document),
        "kindCompatibility": envelope.document.kind_compatibility.clone().unwrap_or(json!([])),
        "overlapBudget": envelope.runtime.overlap_budget,
        "seed": 1,
        "hostRules": {},
        "weights": {
            "objectWeights": envelope.runtime.object_kind_weights,
            "vortexWeights": envelope.runtime.vortex_kind_weights,
        },
    })
    .to_string()
}

/// 🔄 Adopts an engine fixture while preserving flat aspects: existing parts keep `2d`, new parts get a synthesized flat aspect.
fn merge_engine_fixture(envelope: &Puzzle5dScene, fixture_json: &str) -> Option<Puzzle5dScene> {
    let parsed: Value = serde_json::from_str(fixture_json).ok()?;
    let objects = parsed.get("objects")?.as_array()?;
    let mut next = envelope.clone();
    let existing: HashMap<String, Puzzle5dPart> = envelope.document.parts.iter().map(|part| (part.id.clone(), part.clone())).collect();
    let mut new_ids: Vec<String> = Vec::new();
    next.document.parts = objects
        .iter()
        .filter_map(|object| {
            let id = object.get("id")?.as_str()?.to_string();
            let part_kind = object.get("objectKind").and_then(|value| value.as_str()).unwrap_or("Part").to_string();
            let origin: [f64; 3] = object.get("origin").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0]);
            let orientation: Option<[f64; 4]> = object.get("orientation").and_then(|value| serde_json::from_value(value.clone()).ok());
            let mesh_url = object.get("meshUrl").and_then(|value| value.as_str()).map(str::to_string);
            let scale = object.get("scale").cloned().filter(|value| !value.is_null());
            if let Some(previous) = existing.get(&id) {
                let mut part = previous.clone();
                part.part_kind = part_kind;
                part.part_3d.origin = origin;
                part.part_3d.orientation = orientation.or(part.part_3d.orientation);
                part.part_3d.mesh_url = mesh_url.or(part.part_3d.mesh_url.clone());
                if scale.is_some() {
                    part.part_3d.scale = scale;
                }
                return Some(part);
            }
            let templates = grips_from_templates(&envelope.document, &part_kind);
            let grips: Vec<Puzzle5dGrip> = object
                .get("vortices")
                .and_then(|value| value.as_array())
                .into_iter()
                .flatten()
                .enumerate()
                .map(|(index, vortex)| {
                    let template = templates.get(index);
                    Puzzle5dGrip {
                        id: vortex.get("id").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(|| format!("v{index}")),
                        grip_kind: vortex.get("vortexKind").and_then(|value| value.as_str()).map(str::to_string).or_else(|| template.map(|t| t.grip_kind.clone())).unwrap_or_else(|| "grip".into()),
                        grip_2d: template.map(|t| t.grip_2d.clone()).unwrap_or_default(),
                        grip_3d: Puzzle5dGrip3d {
                            position: vortex.get("position").and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or([0.0, 0.0, 0.0]),
                            direction: vortex.get("direction").and_then(|value| serde_json::from_value(value.clone()).ok()),
                            radius: vortex.get("radius").and_then(|value| value.as_f64()).unwrap_or(0.36),
                            label: vortex.get("label").and_then(|value| value.as_str()).map(str::to_string),
                        },
                    }
                })
                .collect();
            let grips = if grips.is_empty() { templates } else { grips };
            new_ids.push(id.clone());
            Some(Puzzle5dPart {
                id,
                part_kind: part_kind.clone(),
                part_2d: Puzzle5dPart2d { x: 0.0, y: 0.0, shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: part_kind, icon_kind: None, hidden: None, locked: None },
                part_3d: Puzzle5dPart3d { origin, mesh_url, orientation: orientation.or(Some([0.0, 0.0, 0.0, 1.0])), scale, label: None },
                grips,
            })
        })
        .collect();
    let existing_kinds: HashMap<String, Option<String>> = envelope.document.fasteners.iter().map(|fastener| (fastener.id.clone(), fastener.fastener_kind.clone())).collect();
    next.document.fasteners = parsed
        .get("attractions")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|attraction| {
            let id = attraction.get("id").and_then(|value| value.as_str()).unwrap_or("fastener").to_string();
            Some(Puzzle5dFastener {
                fastener_kind: existing_kinds.get(&id).cloned().flatten().or_else(|| attraction.get("attractionKind").and_then(|value| value.as_str()).map(str::to_string)),
                source: attraction.get("attracting")?.as_str()?.to_string(),
                target: attraction.get("attracted")?.as_str()?.to_string(),
                id,
            })
        })
        .collect();
    synthesize_flat_for_new_parts(&mut next.document, &new_ids);
    Some(next)
}

/// 🌤️ Places flat centers for freshly-adopted parts next to their fastened neighbor, walking chains until every new part is placed.
fn synthesize_flat_for_new_parts(document: &mut Puzzle5dDocument, new_ids: &[String]) {
    let mut pending: HashSet<String> = new_ids.iter().cloned().collect();
    for _ in 0..=new_ids.len() {
        if pending.is_empty() {
            break;
        }
        let mut placed: Vec<(String, f64, f64)> = Vec::new();
        for fastener in &document.fasteners {
            for (own, other) in [(&fastener.source, &fastener.target), (&fastener.target, &fastener.source)] {
                let Some((own_part, _)) = find_part_by_grip_full_id(document, own) else {
                    continue;
                };
                if !pending.contains(&own_part.id) {
                    continue;
                }
                let Some((other_part, other_grip)) = find_part_by_grip_full_id(document, other) else {
                    continue;
                };
                if pending.contains(&other_part.id) {
                    continue;
                }
                let angle = other_grip.grip_2d.angle;
                let own_radius = if own_part.part_2d.radius > 0.0 { own_part.part_2d.radius } else { PUZZLE5D_DEFAULT_PART_RADIUS };
                let other_radius = if other_part.part_2d.radius > 0.0 { other_part.part_2d.radius } else { PUZZLE5D_DEFAULT_PART_RADIUS };
                let distance = own_radius + other_radius + PUZZLE5D_BOARD_PLACEMENT_GAP;
                placed.push((own_part.id.clone(), other_part.part_2d.x + angle.cos() * distance, other_part.part_2d.y + angle.sin() * distance));
            }
        }
        if placed.is_empty() {
            break;
        }
        for (id, x, y) in placed {
            set_part_2d_position(document, &id, Some(x), Some(y));
            pending.remove(&id);
        }
    }
    let mut column = 0usize;
    for id in pending {
        set_part_2d_position(document, &id, Some(120.0 + column as f64 * 56.0), Some(120.0));
        column += 1;
    }
}
//#endregion 🔖Engine

//#region 🔖World
fn world_instances_json(document: &Puzzle5dDocument, runtime: &Puzzle5dRuntime) -> String {
    let instances: Vec<Value> = document
        .parts
        .iter()
        .map(|part| {
            let selected = runtime.selection.part_ids.contains(&part.id);
            let hovered = runtime.hovered_part_id.as_deref() == Some(part.id.as_str());
            let mesh_id = resolve_part_mesh_url(part, document.kind_catalogs.as_ref()).map(|url| world3d_mesh_id_from_url(&url)).unwrap_or_else(|| PUZZLE5D_FALLBACK_MESH_KIND.into());
            json!({
                "id": part.id,
                "meshId": mesh_id,
                "position": part.part_3d.origin,
                "rotation": part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": part_scale_json(part),
                "label": part.part_3d.label.clone().unwrap_or_else(|| part.part_kind.clone()),
                "selected": selected,
                "hovered": hovered,
                "disabled": part.part_2d.locked.unwrap_or(false),
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn part_scale_json(part: &Puzzle5dPart) -> [f64; 3] {
    match &part.part_3d.scale {
        Some(Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)],
        Some(Value::Number(value)) => {
            let factor = value.as_f64().unwrap_or(1.0);
            [factor, factor, factor]
        }
        _ => [1.0, 1.0, 1.0],
    }
}

fn world_meshes_json(document: &Puzzle5dDocument) -> String {
    world3d_meshes_json_from_urls(&collect_mesh_urls(document))
}

fn grip_color(kind_catalogs: Option<&Value>, grip_kind: &str) -> String {
    kind_catalogs
        .and_then(|catalogs| catalogs.get("grips"))
        .and_then(|value| value.as_array())
        .and_then(|entries| entries.iter().find(|entry| entry.get("id").and_then(|value| value.as_str()) == Some(grip_kind)))
        .and_then(|entry| entry.get("color").and_then(|value| value.as_str()).map(str::to_string))
        .unwrap_or_else(|| "#38bdf8".into())
}

fn world_grips_json(document: &Puzzle5dDocument) -> String {
    let mut records = Vec::new();
    for part in &document.parts {
        for grip in &part.grips {
            records.push(json!({
                "fullId": puzzle5d_grip_full_id(&part.id, &grip.id),
                "objectId": part.id,
                "vortexKind": grip.grip_kind,
                "position": world_grip_position(part, grip),
                "direction": world_grip_direction(part, grip),
                "radius": grip.grip_3d.radius.max(0.36),
                "color": grip_color(document.kind_catalogs.as_ref(), &grip.grip_kind),
            }));
        }
    }
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

fn world_fasteners_json(document: &Puzzle5dDocument) -> String {
    let records: Vec<Value> = document
        .fasteners
        .iter()
        .filter_map(|fastener| {
            let from = resolve_grip_world_position(document, &fastener.source)?;
            let to = resolve_grip_world_position(document, &fastener.target)?;
            Some(json!({ "id": fastener.id, "from": from, "to": to, "color": "#60a5fa" }))
        })
        .collect();
    serde_json::to_string(&records).unwrap_or_else(|_| "[]".into())
}

fn gumball_target_world(envelope: &Puzzle5dScene) -> Option<[f64; 3]> {
    let selected: Vec<&Puzzle5dPart> = envelope.document.parts.iter().filter(|part| envelope.runtime.selection.part_ids.contains(&part.id)).collect();
    if selected.is_empty() {
        return None;
    }
    let mut sum = [0.0, 0.0, 0.0];
    for part in &selected {
        sum[0] += part.part_3d.origin[0];
        sum[1] += part.part_3d.origin[1];
        sum[2] += part.part_3d.origin[2];
    }
    let count = selected.len() as f64;
    Some([sum[0] / count, sum[1] / count, sum[2] / count])
}

/// 🎯 Base selection JSON augmented with the mesh granularity, transform tool, and gumball fields the world-3d host reads.
fn world_selection_json_ex(envelope: &Puzzle5dScene) -> String {
    let runtime = &envelope.runtime;
    let mut value: Value = serde_json::from_str(&world3d_selection_json(&runtime.selection_method, &runtime.selection.part_ids, runtime.hovered_part_id.as_deref())).unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        object.insert("granularity".into(), json!("mesh"));
        object.insert("selectionMode".into(), json!("mesh"));
        object.insert("targets".into(), json!({ "mesh": true, "vertex": false, "edge": false, "face": false }));
        if let Some(transform_mode) = puzzle5d_transform_handle(&envelope.active_utility) {
            object.insert("transformMode".into(), json!(transform_mode));
        }
        if let Some(active_id) = runtime.selection.part_ids.first() {
            object.insert("activeObjectId".into(), json!(active_id));
        }
        let gumball_active = puzzle5d_gumball_active(runtime, &envelope.active_utility);
        object.insert("gumballActive".into(), json!(gumball_active));
        if gumball_active {
            if let Some(target) = gumball_target_world(envelope) {
                object.insert("gumballTarget".into(), json!(target));
            }
        }
    }
    value.to_string()
}

fn world_interaction_json(runtime: &Puzzle5dRuntime, active_utility: &str) -> String {
    json!({
        "activeUtility": puzzle5d_scene_mode(active_utility),
        "brushCandidateIndex": runtime.brush_candidate_index,
        "fillCount": runtime.fill_count,
        "hoveredVortexFullId": runtime.selection.grip_ids.first().cloned(),
    })
    .to_string()
}

fn puzzle5d_context_menu_json(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> Option<String> {
    if envelope.runtime.selection.part_ids.is_empty() {
        return None;
    }
    let selected: Vec<&Puzzle5dPart> = envelope.document.parts.iter().filter(|part| envelope.runtime.selection.part_ids.contains(&part.id)).collect();
    let all_hidden = !selected.is_empty() && selected.iter().all(|part| part.part_2d.hidden.unwrap_or(false));
    let all_locked = !selected.is_empty() && selected.iter().all(|part| part.part_2d.locked.unwrap_or(false));
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
    serde_json::to_string(&items).ok()
}

fn camera3d_json(camera: &Puzzle5dCamera3d) -> String {
    json!({ "position": camera.position, "target": camera.target, "zoom": camera.zoom, "fov": 45.0 }).to_string()
}
//#endregion 🔖World

//#region 🔖Brush
fn puzzle5d_brush_target_grip(envelope: &Puzzle5dScene) -> Option<String> {
    envelope.runtime.selection.grip_ids.first().cloned().or_else(|| {
        let part_id = envelope.runtime.hovered_part_id.as_deref().or_else(|| envelope.runtime.selection.part_ids.first().map(String::as_str))?;
        let part = envelope.document.parts.iter().find(|part| part.id == part_id)?;
        let grip = part.grips.first()?;
        Some(puzzle5d_grip_full_id(&part.id, &grip.id))
    })
}

fn parse_brush_candidates_free(raw: &str) -> Vec<Value> {
    let parsed: Value = serde_json::from_str(raw).unwrap_or(Value::Null);
    parsed.get("free").and_then(|value| value.as_array()).cloned().unwrap_or_default()
}

fn world_brush_preview_json(session: &Puzzle5dPrecomputeSession, envelope: &Puzzle5dScene) -> Option<String> {
    if envelope.active_utility != "brush" {
        return None;
    }
    let full_id = puzzle5d_brush_target_grip(envelope)?;
    session.brush_preview_json(&full_id, envelope.runtime.brush_candidate_index)
}
//#endregion 🔖Brush

//#region 🔖Engagement
/// 🧰 The select/brush/fill switcher lives in the framework utility bar (declared via `.utility` +
/// `.window_kind_utilities`); the fill-count slider and brush placement picker now live as tagged
/// [`WindowMeasure::Group`]s in [`puzzle5d_window_measures`] (surfaced by [`partition_window_measures`]
/// in the dedicated "Utility Options" rail only while their utility is active), so the engagement HUD is a
/// bare command input plus a status line.
fn puzzle5d_engagement(envelope: &Puzzle5dScene, window: &str, labels: &Puzzle5dLabels) -> WindowEngagement {
    let part_count = envelope.document.parts.len();
    let fastener_count = envelope.document.fasteners.len();
    let active_utility = envelope.active_utility.as_str();
    let input_value = envelope.runtime.engagement_input_by_window.get(window).cloned().unwrap_or_default();
    let placeholder = match active_utility {
        "fill" => "Fill",
        "brush" => "Brush",
        _ => "select, brush, fill, clear",
    };
    WindowEngagement {
        session_active: Some(puzzle5d_engagement_session_active(window, active_utility)),
        input: Some(WindowEngagementInput {
            id: Some(format!("puzzle5d-engagement-{window}")),
            value: Some(input_value),
            placeholder: Some(placeholder.into()),
            disabled: None,
            on_change: Some(puzzle5d_action("engagementInput", Some(json!({ "window": window })))),
            on_submit: Some(puzzle5d_action("engagementSubmit", Some(json!({ "window": window })))),
            on_repeat_last: None,
            on_abort: Some(puzzle5d_action("engagementAbort", Some(json!({ "window": window })))),
        }),
        control: None,
        controls: None,
        status: Some(vec![WindowEngagementStatus { id: format!("puzzle5d-status-{window}"), text: format!("{part_count} {} · {fastener_count} {} · {} {active_utility}", labels.parts, labels.fasteners, labels.utility) }]),
        options: None,
        possible_engagements: None,
    }
}
//#endregion 🔖Engagement

//#region 🔖Measures
fn puzzle5d_lod_tier_ids() -> Vec<String> {
    serde_json::from_str::<Vec<Value>>(&infinite_board_port_directed_normal::puzzle_2d_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|row| row.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
}

fn puzzle5d_kind_ids(document: &Puzzle5dDocument, slice: &str) -> Vec<String> {
    let mut ids: Vec<String> = document
        .kind_catalogs
        .as_ref()
        .and_then(|catalogs| catalogs.get(slice))
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.get("id").and_then(|value| value.as_str()).map(str::to_string))
        .collect();
    if ids.is_empty() {
        let mut inferred: Vec<String> = match slice {
            "parts" => document.parts.iter().map(|part| part.part_kind.clone()).collect(),
            "grips" => document.parts.iter().flat_map(|part| part.grips.iter().map(|grip| grip.grip_kind.clone())).collect(),
            _ => Vec::new(),
        };
        inferred.sort();
        inferred.dedup();
        ids = inferred;
    }
    ids
}

fn puzzle5d_uniform_kind_weights(ids: &[String]) -> HashMap<String, f64> {
    if ids.is_empty() {
        return HashMap::new();
    }
    let weight = 1.0 / ids.len() as f64;
    ids.iter().map(|id| (id.clone(), weight)).collect()
}

fn puzzle5d_normalize_kind_weight_group(weights: &HashMap<String, f64>, kind_ids: &[String], changed_id: &str, new_value: f64) -> HashMap<String, f64> {
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

fn puzzle5d_ensure_catalog_kind_weights(weights: &mut HashMap<String, f64>, kind_ids: &[String]) {
    if kind_ids.is_empty() {
        return;
    }
    if weights.is_empty() || kind_ids.iter().any(|id| !weights.contains_key(id)) {
        *weights = puzzle5d_uniform_kind_weights(kind_ids);
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

fn puzzle5d_kind_weight_sum(weights: &HashMap<String, f64>, kind_ids: &[String]) -> f64 {
    kind_ids.iter().map(|id| weights.get(id).copied().unwrap_or(0.0)).sum()
}

fn puzzle5d_lod_measure(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels) -> WindowMeasure {
    let mut items = vec![MeasureSelectItem { id: PUZZLE5D_LOD_MODE_AUTOMATIC.into(), value: PUZZLE5D_LOD_MODE_AUTOMATIC.into(), label: labels.automatic.into() }];
    items.extend(puzzle5d_lod_tier_ids().into_iter().map(|tier| MeasureSelectItem { id: tier.clone(), value: tier.clone(), label: tier }));
    WindowMeasure::Select { id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-lod"), label: Some(labels.lod.into()), value: runtime.lod_mode.clone(), items, on_change: puzzle5d_action("setLodMode", None) }
}

fn puzzle5d_kind_weight_measures(prefix: &str, action: &str, ids: &[String], weights: &HashMap<String, f64>) -> Vec<WindowMeasure> {
    ids.iter()
        .map(|kind_id| {
            let weight = weights.get(kind_id).copied().unwrap_or_else(|| if ids.is_empty() { 0.0 } else { 1.0 / ids.len() as f64 });
            WindowMeasure::Slider {
                id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-{prefix}-{kind_id}"),
                label: Some(format!("{kind_id} {:.0}%", weight * 100.0)),
                value: weight,
                min: 0.0,
                max: 1.0,
                step: Some(0.01),
                ready: None,
                loading: None, waiting: None,
                disabled: None,
                reveal: None,
                on_change: puzzle5d_action(action, Some(json!({ "kindId": kind_id }))),
            }
        })
        .collect()
}

fn puzzle5d_brush_distribution_children(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> Vec<WindowMeasure> {
    let part_ids = puzzle5d_kind_ids(&envelope.document, "parts");
    let grip_ids = puzzle5d_kind_ids(&envelope.document, "grips");
    vec![
        WindowMeasure::Group {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-suggestion-parts"),
            label: format!("{} ({:.0}%)", labels.part_weights, puzzle5d_kind_weight_sum(&envelope.runtime.object_kind_weights, &part_ids) * 100.0).into(),
            default_open: Some(false),
            active_utility_id: None,
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: puzzle5d_kind_weight_measures("part-kind", "setObjectKindWeight", &part_ids, &envelope.runtime.object_kind_weights),
        },
        WindowMeasure::Group {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-suggestion-grips"),
            label: format!("{} ({:.0}%)", labels.grip_weights, puzzle5d_kind_weight_sum(&envelope.runtime.vortex_kind_weights, &grip_ids) * 100.0).into(),
            default_open: Some(false),
            active_utility_id: None,
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: puzzle5d_kind_weight_measures("grip-kind", "setVortexKindWeight", &grip_ids, &envelope.runtime.vortex_kind_weights),
        },
    ]
}

/// 🪣 Fill-count slider measure — the fill-utility's core parameter, mirrors the retired
/// `puzzle5d_fill_count_control` (`setFillCount` reads `count`-or-`value`, so the slider's `{value}`
/// payload preserves the action semantics).
fn puzzle5d_fill_count_measure(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> WindowMeasure {
    WindowMeasure::Slider {
        id: "puzzle5d-fill-count".into(),
        label: Some(labels.count.into()),
        value: envelope.runtime.fill_count as f64,
        min: 0.0,
        max: PUZZLE5D_FILL_COUNT_MAX as f64,
        step: Some(1.0),
        ready: None,
        loading: None, waiting: None,
        disabled: None,
        reveal: None,
        on_change: puzzle5d_action("setFillCount", None),
    }
}

/// 🪣 Utility Options group for the Fill utility — the fill-count slider, tagged `Some("fill")` so
/// [`partition_window_measures`] surfaces it in the Utility Options rail only while the Fill utility is active.
fn puzzle5d_fill_utility_options(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> WindowMeasure {
    WindowMeasure::Group {
        id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-utility-options-fill"),
        label: labels.fill.into(),
        default_open: Some(true),
        active_utility_id: Some("fill".into()),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![puzzle5d_fill_count_measure(envelope, labels)],
    }
}

/// 🖌️ Utility Options group for the Brush utility — suggestion offset, overlap budget, distribution
/// trees, and (when candidates exist) the placement picker. Tagged `Some("brush")`.
fn puzzle5d_brush_utility_options(envelope: &Puzzle5dScene, precompute: &Puzzle5dPrecomputeSession, labels: &Puzzle5dLabels) -> WindowMeasure {
    let mut children = vec![
        WindowMeasure::Slider {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-suggestion-offset"),
            label: Some(labels.offset.into()),
            value: envelope.runtime.suggestion_offset,
            min: PUZZLE5D_SUGGESTION_OFFSET_MIN,
            max: PUZZLE5D_SUGGESTION_OFFSET_MAX,
            step: Some(PUZZLE5D_SUGGESTION_OFFSET_STEP),
            ready: None,
            loading: None, waiting: None,
            disabled: None,
            reveal: None,
            on_change: puzzle5d_action("setSuggestionOffset", None),
        },
        WindowMeasure::Slider {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-brush-overlap"),
            label: Some(labels.overlap.into()),
            value: envelope.runtime.overlap_budget,
            min: 0.0,
            max: 0.2,
            step: Some(0.005),
            ready: None,
            loading: None, waiting: None,
            disabled: None,
            reveal: None,
            on_change: puzzle5d_action("setBrushPlacementOverlapBudget", None),
        },
        WindowMeasure::Group {
            id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-brush-distribution"),
            label: labels.suggestion.into(),
            default_open: Some(false),
            active_utility_id: None,
            value: None,
            min: None,
            max: None,
            step: None,
            ready: None,
            loading: None,
            waiting: None,
            on_change: None,
            children: puzzle5d_brush_distribution_children(envelope, labels),
        },
    ];
    if let Some(target) = puzzle5d_brush_target_grip(envelope) {
        let candidates = parse_brush_candidates_free(&precompute.brush_candidates(&target));
        if !candidates.is_empty() {
            let items: Vec<MeasureSelectItem> = candidates
                .iter()
                .enumerate()
                .map(|(index, candidate)| {
                    let label = candidate.get("objectKind").and_then(|value| value.as_str()).or_else(|| candidate.get("objectKindId").and_then(|value| value.as_str())).unwrap_or("kind");
                    let id = format!("puzzle5d.brush.candidate.{index}");
                    MeasureSelectItem { id: id.clone(), value: id, label: label.into() }
                })
                .collect();
            let selected_index = envelope.runtime.brush_candidate_index.min(items.len().saturating_sub(1));
            children.push(WindowMeasure::Select {
                id: "puzzle5d-brush-placement".into(),
                label: Some(labels.placement.into()),
                value: format!("puzzle5d.brush.candidate.{selected_index}"),
                items,
                on_change: puzzle5d_action("engagementControlSelect", None),
            });
        }
    }
    WindowMeasure::Group {
        id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-utility-options-brush"),
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

fn puzzle5d_window_measures(window: &str, envelope: &Puzzle5dScene, precompute: &Puzzle5dPrecomputeSession, labels: &Puzzle5dLabels) -> Vec<WindowMeasure> {
    let mut measures = if window == PUZZLE5D_PLAY_WINDOW_2D {
        vec![puzzle5d_lod_measure(&envelope.runtime, labels)]
    } else {
        vec![world3d_sun_measures("puzzle5d", &envelope.runtime.sun, puzzle5d_action)]
    };
    measures.push(puzzle5d_fill_utility_options(envelope, labels));
    measures.push(puzzle5d_brush_utility_options(envelope, precompute, labels));
    measures
}
//#endregion 🔖Measures

//#region 🔖Panels
fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, icon_id: Option<&str>, action: ActionDescriptor) -> UiTreeItemNode {
    let mut item = UiTreeItemNode::base(id, label);
    item.icon_id = icon_id.map(IconName::from);
    item.action = Some(action);
    item
}

fn tree_info_item(id: impl Into<String>, label: impl Into<String>, description: Option<String>) -> UiTreeItemNode {
    let mut item = UiTreeItemNode::base(id, label);
    item.description = description;
    item
}

fn part_label(part: &Puzzle5dPart) -> String {
    if !part.part_2d.text.is_empty() {
        return part.part_2d.text.clone();
    }
    part.part_3d.label.clone().unwrap_or_else(|| part.part_kind.clone())
}

fn fastener_label(document: &Puzzle5dDocument, fastener: &Puzzle5dFastener) -> String {
    let side = |full_id: &str| find_part_by_grip_full_id(document, full_id).map(|(part, _)| part_label(part)).unwrap_or_else(|| full_id.to_string());
    format!("{} → {}", side(&fastener.source), side(&fastener.target))
}

fn document_tree_selected_ids(envelope: &Puzzle5dScene) -> Vec<String> {
    let selection = &envelope.runtime.selection;
    selection
        .part_ids
        .iter()
        .map(|id| format!("puzzle5d-play-document.part.{id}"))
        .chain(selection.grip_ids.iter().map(|id| format!("puzzle5d-play-document.grip.{id}")))
        .chain(selection.fastener_ids.iter().map(|id| format!("puzzle5d-play-document.fastener.{id}")))
        .collect()
}

fn build_document_tree(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> UiNode {
    let part_items: Vec<UiTreeItemNode> = envelope
        .document
        .parts
        .iter()
        .map(|part| {
            let grip_items: Vec<UiTreeItemNode> = part
                .grips
                .iter()
                .map(|grip| {
                    let full_id = puzzle5d_grip_full_id(&part.id, &grip.id);
                    tree_item_with_action(format!("puzzle5d-play-document.grip.{full_id}"), format!("{} ({})", grip.id, grip.grip_kind), Some("circle-dot"), puzzle5d_action("setSelection", Some(json!({ "gripIds": [full_id] }))))
                })
                .collect();
            let mut item = tree_item_with_action(format!("puzzle5d-play-document.part.{}", part.id), part_label(part), Some("box"), puzzle5d_action("setSelection", Some(json!({ "partIds": [part.id] }))));
            item.description = Some(part.part_kind.clone());
            if !grip_items.is_empty() {
                item.items = Some(grip_items);
            }
            item
        })
        .collect();
    let fastener_items: Vec<UiTreeItemNode> = envelope
        .document
        .fasteners
        .iter()
        .map(|fastener| tree_item_with_action(format!("puzzle5d-play-document.fastener.{}", fastener.id), fastener_label(&envelope.document, fastener), Some("link"), puzzle5d_action("setSelection", Some(json!({ "fastenerIds": [fastener.id] })))))
        .collect();
    let mut sections = vec![
        UiTreeSectionNode {
            presence: UiPresence::default(),
            id: "puzzle5d-play-document.parts".into(),
            label: Some(labels.parts.into()),
            default_open: Some(true),
            items: if part_items.is_empty() { vec![tree_info_item("puzzle5d-play-document.parts.empty", labels.none, None)] } else { part_items },
        },
        UiTreeSectionNode {
            presence: UiPresence::default(),
            id: "puzzle5d-play-document.fasteners".into(),
            label: Some(labels.fasteners.into()),
            default_open: Some(false),
            items: if fastener_items.is_empty() { vec![tree_info_item("puzzle5d-play-document.fasteners.empty", labels.none, None)] } else { fastener_items },
        },
    ];
    let selected: HashSet<String> = document_tree_selected_ids(envelope).into_iter().collect();
    ui_tree_stamp_presence(&mut sections, &selected, &HashSet::new());
    UiNode::Tree(UiTreeNode {
        presence: UiPresence::default(),
        sections,
        selection_change: Some(puzzle5d_action("setSelection", None)),
        drop_action: None,
    })
}

fn catalog_kind_label(entry: &Value) -> String {
    entry
        .get("label")
        .and_then(|value| value.as_str())
        .filter(|value| !value.is_empty())
        .or_else(|| entry.get("name").and_then(|value| value.as_str()).filter(|value| !value.is_empty()))
        .or_else(|| entry.get("id").and_then(|value| value.as_str()))
        .unwrap_or("kind")
        .into()
}

/// 🖱️ MIME key `DeclarativeTreePanel` (framework/renderer/react/ui-interpreter.tsx) reads to auto-wire catalogue drag sources.
const PUZZLE5D_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";

fn puzzle5d_catalog_item_drag_data(kind_id: &str, entry: &Value) -> HashMap<String, String> {
    let mut payload = json!({ "kindId": kind_id, "catalogSlice": "nodes" });
    if let Some(object) = payload.as_object_mut() {
        for key in ["shape", "radius", "width", "height", "iconKind"] {
            if let Some(value) = entry.get(key) {
                object.insert(key.into(), value.clone());
            }
        }
    }
    HashMap::from([(PUZZLE5D_CATALOGUE_DRAG_MIME.to_string(), payload.to_string())])
}

fn kind_catalog_section(section_id: &str, label: &str, entries: &[Value], add_action: Option<&str>, none_label: &str) -> UiTreeSectionNode {
    let items: Vec<UiTreeItemNode> = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind");
            match add_action {
                Some(action) => {
                    let mut item = tree_item_with_action(format!("{section_id}.{index}.{kind_id}"), catalog_kind_label(entry), Some("box"), puzzle5d_action(action, Some(json!({ "partKind": kind_id }))));
                    item.description = Some(kind_id.into());
                    item.draggable = Some(true);
                    item.drag_data = Some(puzzle5d_catalog_item_drag_data(kind_id, entry));
                    item
                }
                None => tree_info_item(format!("{section_id}.{index}.{kind_id}"), catalog_kind_label(entry), Some(kind_id.into())),
            }
        })
        .collect();
    UiTreeSectionNode {
        presence: UiPresence::default(),
        id: section_id.into(),
        label: Some(label.into()),
        default_open: Some(!items.is_empty()),
        items: if items.is_empty() { vec![tree_info_item(format!("{section_id}.empty"), none_label, None)] } else { items },
    }
}

fn build_kinds_tree(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> UiNode {
    let catalogs = envelope.document.kind_catalogs.clone().unwrap_or(json!({}));
    let slice = |key: &str| catalogs.get(key).and_then(|value| value.as_array()).cloned().unwrap_or_default();
    let mut part_entries = slice("parts");
    if part_entries.is_empty() {
        let mut ids: Vec<String> = envelope.document.parts.iter().map(|part| part.part_kind.clone()).collect();
        ids.sort();
        ids.dedup();
        part_entries = ids.into_iter().map(|id| json!({ "id": id, "name": id })).collect();
    }
    UiNode::Tree(UiTreeNode {
        presence: UiPresence::default(),
        sections: vec![
            kind_catalog_section("puzzle5d-play-kinds.parts", labels.parts, &part_entries, Some("addPartKind"), labels.none),
            kind_catalog_section("puzzle5d-play-kinds.grips", labels.grips, &slice("grips"), None, labels.none),
            kind_catalog_section("puzzle5d-play-kinds.fasteners", labels.fasteners, &slice("fasteners"), None, labels.none),
            kind_catalog_section("puzzle5d-play-kinds.ropes", labels.ropes, &slice("ropes"), None, labels.none),
        ],
        selection_change: None,
        drop_action: None,
    })
}

fn inspector_text_field(id: &str, label: &str, value: String, action: ActionDescriptor) -> UiNode {
    UiNode::Field(UiFieldNode {
        id: id.into(),
        label: label.into(),
        description: None,
        required: None,
        error: None,
        child: Box::new(UiNode::Input(semio_framework_plugin::UiInputNode {
            id: format!("{id}.input"),
            input_kind: "text".into(),
            value,
            placeholder: None,
            commit: None,
            min: None,
            max: None,
            step: None,
            accept: None,
            on_change: action,
            presence: UiPresence::default(),
        })),
        presence: UiPresence::default(),
    })
}

fn build_part_inspector(part: &Puzzle5dPart, labels: &Puzzle5dLabels) -> UiNode {
    let origin = part.part_3d.origin;
    let patch_cmd = |field: &str| puzzle5d_action("patchPart", Some(json!({ "partId": part.id, "field": field })));
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "puzzle5d-play-inspector.part".into(),
        label: labels.part.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            ui_inspector_readonly_field("puzzle5d-play-inspector.part.id", labels.id, &part.id),
            inspector_text_field("puzzle5d-play-inspector.part.kind", labels.kind, part.part_kind.clone(), patch_cmd("partKind")),
            inspector_text_field("puzzle5d-play-inspector.part.label", labels.label, part.part_3d.label.clone().unwrap_or_default(), patch_cmd("label")),
            inspector_text_field("puzzle5d-play-inspector.part.text", labels.flat_text, part.part_2d.text.clone(), patch_cmd("text")),
            ui_inspector_stepper_field("puzzle5d-play-inspector.part.x", labels.flat_x, &[part.part_2d.x], 0.1, patch_cmd("x")),
            ui_inspector_stepper_field("puzzle5d-play-inspector.part.y", labels.flat_y, &[part.part_2d.y], 0.1, patch_cmd("y")),
            ui_inspector_vec3_group("puzzle5d-play-inspector.part.origin", labels.volume_origin, &[origin], 0.1, |axis| patch_cmd(&format!("origin.{axis}"))),
        ],
    }])
}

fn build_grip_inspector(part: &Puzzle5dPart, grip: &Puzzle5dGrip, labels: &Puzzle5dLabels) -> UiNode {
    let full_id = puzzle5d_grip_full_id(&part.id, &grip.id);
    let position = grip.grip_3d.position;
    let direction = grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]);
    let patch_cmd = |field: &str| puzzle5d_action("patchGrip", Some(json!({ "gripFullId": full_id, "field": field })));
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        id: "puzzle5d-play-inspector.grip".into(),
        label: labels.grip.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            ui_inspector_readonly_field("puzzle5d-play-inspector.grip.id", labels.id, &full_id),
            inspector_text_field("puzzle5d-play-inspector.grip.kind", labels.kind, grip.grip_kind.clone(), patch_cmd("gripKind")),
            ui_inspector_stepper_field("puzzle5d-play-inspector.grip.angle", labels.flat_angle, &[grip.grip_2d.angle], 1.0, patch_cmd("angle")),
            ui_inspector_stepper_field("puzzle5d-play-inspector.grip.radius", labels.radius, &[grip.grip_3d.radius], 0.05, patch_cmd("radius")),
            ui_inspector_vec3_group("puzzle5d-play-inspector.grip.position", labels.position, &[position], 0.1, |axis| patch_cmd(&format!("position.{axis}"))),
            ui_inspector_vec3_group("puzzle5d-play-inspector.grip.direction", labels.direction, &[direction], 0.1, |axis| patch_cmd(&format!("direction.{axis}"))),
        ],
    }])
}

fn build_inspector_tree(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> UiNode {
    if let Some(grip_full_id) = envelope.runtime.selection.grip_ids.first() {
        if let Some((part, grip)) = find_part_by_grip_full_id(&envelope.document, grip_full_id) {
            return build_grip_inspector(part, grip, labels);
        }
    }
    if let Some(part_id) = envelope.runtime.selection.part_ids.first() {
        if let Some(part) = envelope.document.parts.iter().find(|entry| &entry.id == part_id) {
            return build_part_inspector(part, labels);
        }
    }
    if let Some(fastener_id) = envelope.runtime.selection.fastener_ids.first() {
        if let Some(fastener) = envelope.document.fasteners.iter().find(|entry| &entry.id == fastener_id) {
            return ui_stack_vertical(vec![
                ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.id", labels.id, &fastener.id),
                ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.source", labels.source, &fastener.source),
                ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.target", labels.target, &fastener.target),
                ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.kind", labels.kind, fastener.fastener_kind.as_deref().unwrap_or("link")),
            ]);
        }
    }
    ui_stack_vertical(vec![
        ui_text(format!("{}: {}", labels.schema, envelope.document.schema)),
        ui_text(format!("{}: {}", labels.parts, envelope.document.parts.len())),
        ui_text(format!("{}: {}", labels.fasteners, envelope.document.fasteners.len())),
        ui_text(format!("{}: {}", labels.utility, envelope.active_utility)),
    ])
}
//#endregion 🔖Panels

//#region 🔖Puzzle5dPlayApp
/// 🧩 Puzzle-5d play app. Owns the precompute engine, the registered-mesh cache, and the ephemeral
/// view `runtime`; the persisted document (bare `Puzzle5dDocument` json) lives in the wrapping
/// `VcsDocumentApp`. Each action mutates a transient {@link Puzzle5dScene}, then emits the granular
/// operation delta. Undo/redo/checkpoints are handled by the wrapper.
pub struct Puzzle5dPlayApp {
    precompute: Puzzle5dPrecomputeSession,
    registered_mesh_urls: HashSet<String>,
    runtime: Puzzle5dRuntime,
}

impl Default for Puzzle5dPlayApp {
    fn default() -> Self {
        Self { precompute: Puzzle5dPrecomputeSession::new(), registered_mesh_urls: HashSet::new(), runtime: Puzzle5dRuntime::default() }
    }
}

impl Puzzle5dPlayApp {
    fn drive_precompute(&mut self, envelope: &Puzzle5dScene) {
        let _ = self.precompute.set_scene(&scene_config_json(envelope));
        // 🧊 Guarded by `has_mesh` (mirrors the puzzle3d path): `register_mesh` now invalidates the
        // precompute cache, so re-registering the same fallback body on every drive would wipe
        // suggestion/fill progress every call and defeat `set_scene`'s idempotence above.
        if !self.precompute.has_mesh(PUZZLE5D_FALLBACK_MESH_KIND) {
            let fallback = semio_framework_plugin::mesh_from_kind(PUZZLE5D_FALLBACK_MESH_KIND);
            self.precompute.register_mesh(PUZZLE5D_FALLBACK_MESH_KIND, &fallback.positions, &fallback.indices);
        }
        for url in collect_mesh_urls(&envelope.document) {
            if !self.registered_mesh_urls.contains(&url) && !self.precompute.has_mesh(&url) {
                let fallback = semio_framework_plugin::mesh_from_kind(PUZZLE5D_FALLBACK_MESH_KIND);
                self.precompute.register_mesh(&url, &fallback.positions, &fallback.indices);
            }
        }
        let _ = self.precompute.precompute_step(8);
    }

    fn apply_engine_brush_placement(&mut self, envelope: &Puzzle5dScene, payload: &Value) -> Option<Puzzle5dScene> {
        let brush_payload = serde_json::from_value::<BrushPlacePayload>(payload.clone()).ok()?;
        let fixture_json = self.precompute.apply_brush_placement_rust(&serde_json::to_string(&brush_payload).ok()?).ok()?;
        merge_engine_fixture(envelope, &fixture_json)
    }

    /// 🖌️ Paired placement for a board `brushPlace` event: the engine picks the volume pose for the flat payload's kind, both aspects land in one part.
    fn apply_board_brush_place(&mut self, envelope: &mut Puzzle5dScene, payload: &Value) {
        self.drive_precompute(envelope);
        let node_kind = payload.get("nodeKind").and_then(|value| value.as_str()).unwrap_or("Part").to_string();
        let source_grip = payload.get("sourceHandleId").and_then(|value| value.as_str()).map(str::to_string).or_else(|| puzzle5d_brush_target_grip(envelope));
        if let Some(source_grip) = source_grip.as_ref() {
            let candidates = parse_brush_candidates_free(&self.precompute.brush_candidates(source_grip));
            let candidate_index = candidates
                .iter()
                .position(|candidate| candidate.get("objectKindId").or_else(|| candidate.get("objectKind")).and_then(|value| value.as_str()) == Some(node_kind.as_str()))
                .unwrap_or(envelope.runtime.brush_candidate_index);
            let engine_payload = json!({ "objectKindId": node_kind, "targetVortexFullId": source_grip, "candidateIndex": candidate_index });
            if let Some(mut next) = self.apply_engine_brush_placement(envelope, &engine_payload) {
                let previous_ids: HashSet<String> = envelope.document.parts.iter().map(|part| part.id.clone()).collect();
                let new_id = next.document.parts.iter().map(|part| part.id.clone()).find(|id| !previous_ids.contains(id));
                if let Some(new_id) = new_id {
                    let x = payload.get("x").and_then(|value| value.as_f64());
                    let y = payload.get("y").and_then(|value| value.as_f64());
                    set_part_2d_position(&mut next.document, &new_id, x, y);
                    next.runtime.selection = Puzzle5dSelection { part_ids: vec![new_id], grip_ids: Vec::new(), fastener_ids: Vec::new() };
                }
                *envelope = next;
                return;
            }
        }
        let id = payload.get("nodeId").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(next_part_id);
        let x = payload.get("x").and_then(|value| value.as_f64()).unwrap_or(120.0);
        let y = payload.get("y").and_then(|value| value.as_f64()).unwrap_or(120.0);
        let mesh_url = resolve_part_kind_mesh_url(&node_kind, envelope.document.kind_catalogs.as_ref());
        let grips = grips_from_templates(&envelope.document, &node_kind);
        let source_world = source_grip.as_ref().and_then(|full_id| find_part_by_grip_full_id(&envelope.document, full_id).map(|(part, grip)| (world_grip_position(part, grip), world_grip_direction(part, grip))));
        let origin = source_world.map(|(position, direction)| [position[0] + direction[0], position[1] + direction[1], position[2] + direction[2]]).unwrap_or([0.0, 0.0, 0.0]);
        envelope.document.parts.push(Puzzle5dPart {
            id: id.clone(),
            part_kind: node_kind.clone(),
            part_2d: Puzzle5dPart2d { x, y, shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: node_kind, icon_kind: None, hidden: None, locked: None },
            part_3d: Puzzle5dPart3d { origin, mesh_url, orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None, label: None },
            grips,
        });
        if let (Some(source), Some(part)) = (source_grip, envelope.document.parts.last()) {
            if let Some(grip) = part.grips.first() {
                let target = puzzle5d_grip_full_id(&part.id, &grip.id);
                envelope.document.fasteners.push(Puzzle5dFastener { id: payload.get("edgeId").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(next_fastener_id), source, target, fastener_kind: None });
            }
        }
        envelope.runtime.selection = Puzzle5dSelection { part_ids: vec![id], grip_ids: Vec::new(), fastener_ids: Vec::new() };
    }

    fn apply_board_events_from_json(&mut self, events_json: &str, envelope: &mut Puzzle5dScene) {
        let Ok(events) = serde_json::from_str::<Vec<Value>>(events_json) else {
            return;
        };
        for event in events {
            let Some(name) = event.get("name").and_then(|value| value.as_str()) else {
                continue;
            };
            let payload = event.get("payload").cloned().unwrap_or(Value::Null);
            match name {
                "camera" => {
                    if let Ok(camera) = serde_json::from_value::<Puzzle5dCamera2d>(payload) {
                        envelope.document.camera2d = camera;
                    }
                }
                "select" => {
                    if let Some(ids) = payload.get("ids").and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
                        envelope.runtime.selection = classify_selection(&envelope.document, &ids);
                    }
                }
                "nodeDragEnd" => {
                    for entry in payload.get("moves").and_then(|value| value.as_array()).into_iter().flatten() {
                        if let Some(id) = entry.get("id").and_then(|value| value.as_str()) {
                            set_part_2d_position(&mut envelope.document, id, entry.get("x").and_then(|value| value.as_f64()), entry.get("y").and_then(|value| value.as_f64()));
                        }
                    }
                }
                "nodeMove" => {
                    if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                        set_part_2d_position(&mut envelope.document, id, payload.get("x").and_then(|value| value.as_f64()), payload.get("y").and_then(|value| value.as_f64()));
                    }
                }
                "brushPlace" => {
                    self.apply_board_brush_place(envelope, &payload);
                }
                "edgeCreate" => {
                    let source = payload.get("source").and_then(|value| value.as_str()).unwrap_or("").to_string();
                    let target = payload.get("target").and_then(|value| value.as_str()).unwrap_or("").to_string();
                    if !source.is_empty() && !target.is_empty() && !envelope.document.fasteners.iter().any(|entry| entry.source == source && entry.target == target || entry.source == target && entry.target == source) {
                        envelope.document.fasteners.push(Puzzle5dFastener {
                            id: payload.get("id").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(next_fastener_id),
                            source,
                            target,
                            fastener_kind: payload.get("edgeKind").and_then(|value| value.as_str()).map(str::to_string),
                        });
                    }
                }
                "nodeDelete" => {
                    if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                        remove_parts(&mut envelope.document, &[id.to_string()]);
                        envelope.runtime.selection = Puzzle5dSelection::default();
                    }
                }
                "edgeDelete" => {
                    if let Some(id) = payload.get("id").and_then(|value| value.as_str()) {
                        envelope.document.fasteners.retain(|fastener| fastener.id != id);
                    }
                }
                _ => {}
            }
        }
    }
}

impl DocumentApp for Puzzle5dPlayApp {
    type Projection = Puzzle5dPlayProjection;
    type Operation = Puzzle5dOperation;

    fn app_id(&self) -> &str {
        PUZZLE5D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        PUZZLE5D_SCHEMA
    }

    fn initial_projection(&self) -> Puzzle5dPlayProjection {
        Puzzle5dPlayProjection(serde_json::to_value(default_document()).unwrap_or(Value::Null))
    }

    fn handle_action(&mut self, action: &str, args: Option<&Value>, doc: &DocumentView<'_, Puzzle5dPlayProjection>, view_state: &ViewState) -> ActionEmit<Puzzle5dOperation> {
        let before = doc.projection.0.clone();
        let active_utility_initial = puzzle5d_scene_active_utility(view_state, view_state.window_id.as_deref());
        let mut envelope = scene_from_projection(&before, self.runtime.clone(), &active_utility_initial);
        match action {
            "setFixtureJson" => {
                if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                    if let Ok(document) = serde_json::from_str::<Puzzle5dDocument>(json_text) {
                        envelope.document = document;
                    }
                }
            }
            "toggleSun" | "setSunAzimuth" | "setSunElevation" | "setSunIntensity" => {
                apply_world3d_sun_action(&mut envelope.runtime.sun, action, args);
            }
            "setActiveExample" => {
                let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
                let next = if example_id.is_empty() {
                    Some(empty_document())
                } else if example_id == PUZZLE5D_EXAMPLE_CONCRETE_FOREST || example_id == "concrete" {
                    Some(default_document())
                } else if example_id == PUZZLE5D_EXAMPLE_NAKAGIN || example_id == "nakagin" {
                    Some(document_from_json(NAKAGIN_EXAMPLE_JSON.as_str()))
                } else {
                    None
                };
                if let Some(document) = next {
                    envelope.document = document;
                    envelope.runtime = Puzzle5dRuntime::default();
                }
                self.drive_precompute(&envelope);
            }
            "setSelection" | "documentSelect" => {
                if let Some(ids) = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
                    envelope.runtime.selection = classify_selection(&envelope.document, &ids);
                } else {
                    let read = |key: &str| args.and_then(|value| value.get(key)).and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok());
                    envelope.runtime.selection = Puzzle5dSelection { part_ids: read("partIds").unwrap_or_default(), grip_ids: read("gripIds").unwrap_or_default(), fastener_ids: read("fastenerIds").unwrap_or_default() };
                }
            }
            "clearSelection" => {
                envelope.runtime.selection = Puzzle5dSelection::default();
            }
            "selectAll" => {
                envelope.runtime.selection = Puzzle5dSelection { part_ids: envelope.document.parts.iter().map(|part| part.id.clone()).collect(), grip_ids: Vec::new(), fastener_ids: Vec::new() };
            }
            "deleteSelection" => {
                let selection = envelope.runtime.selection.clone();
                remove_parts(&mut envelope.document, &selection.part_ids);
                remove_grips(&mut envelope.document, &selection.grip_ids);
                envelope.document.fasteners.retain(|fastener| !selection.fastener_ids.contains(&fastener.id));
                envelope.runtime.selection = Puzzle5dSelection::default();
            }
            "duplicateSelection" => {
                let ids = envelope.runtime.selection.part_ids.clone();
                let clones: Vec<Puzzle5dPart> = envelope
                    .document
                    .parts
                    .iter()
                    .filter(|part| ids.contains(&part.id))
                    .map(|part| {
                        let mut clone = part.clone();
                        clone.id = next_part_id();
                        clone.part_3d.origin[0] += 0.5;
                        clone.part_3d.origin[1] += 0.5;
                        clone.part_2d.x += 48.0;
                        clone.part_2d.y += 24.0;
                        clone
                    })
                    .collect();
                if clones.is_empty() {
                    return ActionEmit::default();
                }
                let new_ids: Vec<String> = clones.iter().map(|part| part.id.clone()).collect();
                envelope.document.parts.extend(clones);
                envelope.runtime.selection = Puzzle5dSelection { part_ids: new_ids, grip_ids: Vec::new(), fastener_ids: Vec::new() };
            }
            "selectSameKindSelection" | "selectSameKind" => {
                let Some(kind) = envelope.runtime.selection.part_ids.first().and_then(|id| envelope.document.parts.iter().find(|part| &part.id == id)).map(|part| part.part_kind.clone()) else {
                    return ActionEmit::default();
                };
                envelope.runtime.selection.part_ids = envelope.document.parts.iter().filter(|part| part.part_kind == kind).map(|part| part.id.clone()).collect();
            }
            "addNode" => {
                let part_kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("Part").to_string();
                let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
                add_palette_part(&mut envelope, &part_kind, x, y);
            }
            "setSelectionFlag" => {
                let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(false);
                let part_ids = envelope.runtime.selection.part_ids.clone();
                for part in &mut envelope.document.parts {
                    if !part_ids.contains(&part.id) {
                        continue;
                    }
                    match flag {
                        "hidden" => part.part_2d.hidden = Some(value),
                        "locked" => part.part_2d.locked = Some(value),
                        _ => {}
                    }
                }
                if !part_ids.is_empty() && (flag == "hidden" || flag == "locked") {
                }
            }
            "zoomToSelection" | "focusSelection" => {
                let Some(target) = gumball_target_world(&envelope) else {
                    return ActionEmit::default();
                };
                let camera = &mut envelope.document.camera3d;
                let offset = [camera.position[0] - camera.target[0], camera.position[1] - camera.target[1], camera.position[2] - camera.target[2]];
                camera.target = target;
                camera.position = [target[0] + offset[0], target[1] + offset[1], target[2] + offset[2]];
                let selected_2d: Vec<(f64, f64)> = envelope.document.parts.iter().filter(|part| envelope.runtime.selection.part_ids.contains(&part.id)).map(|part| (part.part_2d.x, part.part_2d.y)).collect();
                if !selected_2d.is_empty() {
                    envelope.document.camera2d.x = selected_2d.iter().map(|(x, _)| x).sum::<f64>() / selected_2d.len() as f64;
                    envelope.document.camera2d.y = selected_2d.iter().map(|(_, y)| y).sum::<f64>() / selected_2d.len() as f64;
                }
            }
            SET_ACTIVE_UTILITY_ACTION_ID => {
                // 🧰 Host already applied `view_state.active_utility_id`; clear per-window engagement scratch and
                // refresh the placement engine for the new utility. Emits no operations and no utility-switch effect.
                for window in PUZZLE5D_PLAY_WINDOWS {
                    envelope.runtime.engagement_input_by_window.insert(window.to_string(), String::new());
                }
                envelope.runtime.brush_candidate_index = 0;
                if envelope.active_utility == "brush" || envelope.active_utility == "fill" {
                    self.drive_precompute(&envelope);
                }
            }
            "engagementInput" => {
                let window = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()).unwrap_or(PUZZLE5D_PLAY_WINDOW_2D);
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
                if PUZZLE5D_PLAY_WINDOWS.contains(&window) {
                    envelope.runtime.engagement_input_by_window.insert(window.to_string(), value.to_string());
                }
            }
            "engagementSubmit" => {
                let window = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()).unwrap_or(PUZZLE5D_PLAY_WINDOW_2D).to_string();
                let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).map(str::trim).unwrap_or("").to_lowercase();
                match value.as_str() {
                    "select" if window == PUZZLE5D_PLAY_WINDOW_3D => envelope.active_utility = "move".into(),
                    "select" | "brush" | "fill" => {
                        envelope.active_utility = if value == "select" { "select".into() } else { value };
                        if envelope.active_utility != "select" {
                            self.drive_precompute(&envelope);
                        }
                    }
                    "clear" => puzzle5d_clear_selection(&mut envelope.runtime.selection),
                    "rectangle" | "lasso" => envelope.runtime.selection_method = value,
                    _ => {}
                }
                if PUZZLE5D_PLAY_WINDOWS.contains(&window.as_str()) {
                    envelope.runtime.engagement_input_by_window.insert(window, String::new());
                }
            }
            "engagementAbort" => {
                if let Some(window) = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()) {
                    if PUZZLE5D_PLAY_WINDOWS.contains(&window) {
                        envelope.runtime.engagement_input_by_window.insert(window.to_string(), String::new());
                    }
                }
                let window = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()).unwrap_or(PUZZLE5D_PLAY_WINDOW_2D);
                envelope.active_utility = if window == PUZZLE5D_PLAY_WINDOW_3D { "move".into() } else { "select".into() };
            }
            "engagementControlSelect" => {
                let candidate_id = args.and_then(|value| value.get("id").or_else(|| value.get("value"))).and_then(|value| value.as_str()).unwrap_or("");
                if let Some(index) = candidate_id.strip_prefix("puzzle5d.brush.candidate.").and_then(|rest| rest.parse::<usize>().ok()) {
                    envelope.runtime.brush_candidate_index = index;
                }
            }
            "addBrushPart" | "addBrushObject" => {
                self.drive_precompute(&envelope);
                if let Some(payload_value) = args {
                    let mut payload = payload_value.clone();
                    if let Some(object) = payload.as_object_mut() {
                        if let Some(part_kind) = object.remove("partKind") {
                            object.insert("objectKindId".to_string(), part_kind);
                        }
                        if object.get("targetVortexFullId").is_none() {
                            if let Some(grip_id) = puzzle5d_brush_target_grip(&envelope) {
                                object.insert("targetVortexFullId".to_string(), json!(grip_id));
                            }
                        }
                    }
                    if let Some(next) = self.apply_engine_brush_placement(&envelope, &payload) {
                        envelope = next;
                    }
                }
                let part_kind = args.and_then(|value| value.get("partKind").or_else(|| value.get("objectKindId"))).and_then(|value| value.as_str()).unwrap_or("Part").to_string();
                let payload = json!({ "nodeKind": part_kind, "x": args.and_then(|value| value.get("x")).cloned().unwrap_or(json!(120.0)), "y": args.and_then(|value| value.get("y")).cloned().unwrap_or(json!(120.0)) });
                self.apply_board_brush_place(&mut envelope, &payload);
            }
            "setFillCount" => {
                self.drive_precompute(&envelope);
                let count = args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(|value| value.as_f64()).map(|value| value.round().max(0.0) as u32).unwrap_or(0).min(PUZZLE5D_FILL_COUNT_MAX);
                envelope.runtime.fill_count = count;
                if count > 0 {
                    envelope.active_utility = "fill".into();
                    if let Ok(fixture_json) = self.precompute.apply_fill_count_rust(count) {
                        if let Some(next) = merge_engine_fixture(&envelope, &fixture_json) {
                            envelope = next;
                        }
                    }
                }
            }
            "cycleBrushCandidate" => {
                self.drive_precompute(&envelope);
                if let Some(grip_full_id) = puzzle5d_brush_target_grip(&envelope) {
                    let free = parse_brush_candidates_free(&self.precompute.brush_candidates(&grip_full_id)).len();
                    if free > 0 {
                        envelope.runtime.brush_candidate_index = (envelope.runtime.brush_candidate_index + 1) % free;
                    }
                } else {
                    envelope.runtime.brush_candidate_index = envelope.runtime.brush_candidate_index.saturating_add(1);
                }
            }
            "registerBrushMesh" => {
                if let (Some(url), Some(positions), Some(indices)) =
                    (args.and_then(|v| v.get("url")).and_then(|v| v.as_str()), args.and_then(|v| v.get("positions")).and_then(|v| v.as_array()), args.and_then(|v| v.get("indices")).and_then(|v| v.as_array()))
                {
                    let positions: Vec<f32> = positions.iter().filter_map(|v| v.as_f64().map(|n| n as f32)).collect();
                    let indices: Vec<u32> = indices.iter().filter_map(|v| v.as_u64().map(|n| n as u32)).collect();
                    self.precompute.register_mesh(url, &positions, &indices);
                    self.registered_mesh_urls.insert(url.to_string());
                }
                return ActionEmit::default();
            }
            "setBrushPlacementOverlapBudget" => {
                if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()) {
                    envelope.runtime.overlap_budget = value.clamp(0.0, 1.0);
                    self.drive_precompute(&envelope);
                }
            }
            "setObjectKindWeight" | "setVortexKindWeight" => {
                let kind_id = args.and_then(|v| v.get("kindId")).and_then(|v| v.as_str()).unwrap_or("");
                let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()).unwrap_or(1.0).clamp(0.0, 1.0);
                let part_ids = puzzle5d_kind_ids(&envelope.document, "parts");
                let grip_ids = puzzle5d_kind_ids(&envelope.document, "grips");
                puzzle5d_ensure_catalog_kind_weights(&mut envelope.runtime.object_kind_weights, &part_ids);
                puzzle5d_ensure_catalog_kind_weights(&mut envelope.runtime.vortex_kind_weights, &grip_ids);
                if action == "setObjectKindWeight" {
                    envelope.runtime.object_kind_weights = puzzle5d_normalize_kind_weight_group(&envelope.runtime.object_kind_weights, &part_ids, kind_id, value);
                } else {
                    envelope.runtime.vortex_kind_weights = puzzle5d_normalize_kind_weight_group(&envelope.runtime.vortex_kind_weights, &grip_ids, kind_id, value);
                }
                self.drive_precompute(&envelope);
            }
            "addPartKind" => {
                let part_kind = args.and_then(|value| value.get("partKind")).and_then(|value| value.as_str()).unwrap_or("Part").to_string();
                let payload = json!({ "nodeKind": part_kind, "x": 120.0, "y": 120.0 });
                self.apply_board_brush_place(&mut envelope, &payload);
            }
            "patchPart" => {
                let part_id = args.and_then(|value| value.get("partId")).and_then(|value| value.as_str()).unwrap_or("");
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value"));
                let delta = args.and_then(|value| value.get("delta"));
                let text = value.and_then(Value::as_str).map(str::to_string);
                for part in &mut envelope.document.parts {
                    if part.id != part_id {
                        continue;
                    }
                    match field {
                        "partKind" => {
                            if let Some(text) = &text {
                                part.part_kind = text.clone();
                            }
                        }
                        "text" => {
                            if let Some(text) = &text {
                                part.part_2d.text = text.clone();
                            }
                        }
                        "label" => part.part_3d.label = text.clone().filter(|text| !text.is_empty()),
                        "x" => {
                            if let Some(updated) = puzzle5d_resolve_number_edit(part.part_2d.x, value, delta) {
                                part.part_2d.x = updated;
                            }
                        }
                        "y" => {
                            if let Some(updated) = puzzle5d_resolve_number_edit(part.part_2d.y, value, delta) {
                                part.part_2d.y = updated;
                            }
                        }
                        _ => {
                            if let Some(axis) = puzzle5d_axis_index(field, "origin") {
                                if let Some(updated) = puzzle5d_resolve_number_edit(part.part_3d.origin[axis], value, delta) {
                                    part.part_3d.origin[axis] = updated;
                                }
                            }
                        }
                    }
                }
            }
            "patchGrip" => {
                let grip_full_id = args.and_then(|value| value.get("gripFullId")).and_then(|value| value.as_str()).unwrap_or("").to_string();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value"));
                let delta = args.and_then(|value| value.get("delta"));
                let text = value.and_then(Value::as_str).map(str::to_string);
                for part in &mut envelope.document.parts {
                    let part_id = part.id.clone();
                    for grip in &mut part.grips {
                        if puzzle5d_grip_full_id(&part_id, &grip.id) != grip_full_id {
                            continue;
                        }
                        match field {
                            "gripKind" => {
                                if let Some(text) = &text {
                                    grip.grip_kind = text.clone();
                                    grip.grip_2d.grip_kind = text.clone();
                                }
                            }
                            "angle" => {
                                if let Some(updated) = puzzle5d_resolve_number_edit(grip.grip_2d.angle, value, delta) {
                                    grip.grip_2d.angle = updated;
                                }
                            }
                            "radius" => {
                                if let Some(updated) = puzzle5d_resolve_number_edit(grip.grip_3d.radius, value, delta) {
                                    grip.grip_2d.radius = updated;
                                    grip.grip_3d.radius = updated;
                                }
                            }
                            "label" => grip.grip_3d.label = text.clone().filter(|text| !text.is_empty()),
                            _ => {
                                if let Some(axis) = puzzle5d_axis_index(field, "position") {
                                    if let Some(updated) = puzzle5d_resolve_number_edit(grip.grip_3d.position[axis], value, delta) {
                                        grip.grip_3d.position[axis] = updated;
                                    }
                                } else if let Some(axis) = puzzle5d_axis_index(field, "direction") {
                                    let mut direction = grip.grip_3d.direction.unwrap_or([0.0, 0.0, -1.0]);
                                    if let Some(updated) = puzzle5d_resolve_number_edit(direction[axis], value, delta) {
                                        direction[axis] = updated;
                                        grip.grip_3d.direction = Some(direction);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    let surface_id = args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()).unwrap_or("");
                    if surface_id == PUZZLE5D_PLAY_SURFACE_2D || camera.get("position").is_none() {
                        if let Ok(parsed) = serde_json::from_value::<Puzzle5dCamera2d>(camera.clone()) {
                            envelope.document.camera2d = parsed;
                        }
                    } else if let Ok(parsed) = serde_json::from_value::<Puzzle5dCamera3d>(camera.clone()) {
                        envelope.document.camera3d = parsed;
                    }
                }
            }
            "setCamera2d" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        envelope.document.camera2d = parsed;
                    }
                }
            }
            "setCamera3d" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        envelope.document.camera3d = parsed;
                    }
                }
            }
            "translateSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selection.part_ids);
                let dx = args.and_then(|value| value.get("dx")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dy = args.and_then(|value| value.get("dy")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let dz = args.and_then(|value| value.get("dz")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                for part in &mut envelope.document.parts {
                    if ids.contains(&part.id) {
                        part.part_3d.origin[0] += dx;
                        part.part_3d.origin[1] += dy;
                        part.part_3d.origin[2] += dz;
                    }
                }
                if !ids.is_empty() {
                }
            }
            "rotateSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selection.part_ids);
                let ax = args.and_then(|value| value.get("ax")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let ay = args.and_then(|value| value.get("ay")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let az = args.and_then(|value| value.get("az")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let angle = args.and_then(|value| value.get("angle")).and_then(|value| value.as_f64()).unwrap_or(0.0);
                let delta = quat_from_axis_angle(ax, ay, az, angle);
                for part in &mut envelope.document.parts {
                    if ids.contains(&part.id) {
                        let current = part.part_3d.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
                        part.part_3d.orientation = Some(quat_mul(delta, current));
                    }
                }
                if !ids.is_empty() {
                }
            }
            "scaleSelection" => {
                let ids = mesh_selection_ids(args, &envelope.runtime.selection.part_ids);
                let sx = args.and_then(|value| value.get("sx")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sy = args.and_then(|value| value.get("sy")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                let sz = args.and_then(|value| value.get("sz")).and_then(|value| value.as_f64()).unwrap_or(1.0);
                for part in &mut envelope.document.parts {
                    if ids.contains(&part.id) {
                        let current = part_scale_json(part);
                        part.part_3d.scale = Some(json!([current[0] * sx, current[1] * sy, current[2] * sz]));
                    }
                }
                if !ids.is_empty() {
                }
            }
            "worldSelect" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
                envelope.runtime.selection.part_ids = merge_world_selection_ids(&envelope.runtime.selection.part_ids, &ids, merge);
            }
            "worldPick" => {
                let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
                if args.and_then(|value| value.get("id")).is_none_or(|value| value.is_null()) {
                    if merge == "replace" {
                        puzzle5d_clear_selection(&mut envelope.runtime.selection);
                    }
                } else {
                    let index = args.and_then(|value| value.get("id")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
                    match envelope.document.parts.get(index).filter(|part| part.part_2d.locked != Some(true)) {
                        Some(part) => {
                            let id = part.id.clone();
                            envelope.runtime.selection.part_ids = match merge {
                                "add" => {
                                    let mut merged = envelope.runtime.selection.part_ids.clone();
                                    if !merged.contains(&id) {
                                        merged.push(id);
                                    }
                                    merged
                                }
                                "toggle" => {
                                    let mut merged = envelope.runtime.selection.part_ids.clone();
                                    if let Some(position) = merged.iter().position(|entry| entry == &id) {
                                        merged.remove(position);
                                    } else {
                                        merged.push(id);
                                    }
                                    merged
                                }
                                _ => {
                                    puzzle5d_clear_non_part_selection(&mut envelope.runtime.selection);
                                    vec![id]
                                }
                            };
                        }
                        None if merge == "replace" => {
                            puzzle5d_clear_selection(&mut envelope.runtime.selection);
                        }
                        None => {}
                    }
                }
            }
            "worldHover" => {
                envelope.runtime.hovered_part_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
            }
            "setHover" => {
                envelope.runtime.hovered_part_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).map(str::to_string);
            }
            "worldVortexHover" => {
                envelope.runtime.selection.grip_ids = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()).map(|full_id| vec![full_id.to_string()]).unwrap_or_default();
                if envelope.active_utility == "brush" && !envelope.runtime.selection.grip_ids.is_empty() {
                    self.drive_precompute(&envelope);
                }
            }
            "worldVortexSelect" => {
                if let Some(full_id) = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()) {
                    puzzle5d_clear_non_grip_selection(&mut envelope.runtime.selection);
                    envelope.runtime.selection.grip_ids = vec![full_id.to_string()];
                    self.drive_precompute(&envelope);
                }
            }
            "worldRelocate" => {
                let object_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).unwrap_or("");
                let position = args.and_then(|value| value.get("position")).and_then(|value| serde_json::from_value::<[f64; 3]>(value.clone()).ok());
                if let (Some(part), Some(position)) = (envelope.document.parts.iter_mut().find(|part| part.id == object_id), position) {
                    part.part_3d.origin = position;
                    let source_grip = part.grips.first().map(|grip| (puzzle5d_grip_full_id(&part.id, &grip.id), world_grip_position(part, grip)));
                    if let Some((source_id, source_position)) = source_grip {
                        for other in &envelope.document.parts {
                            if other.id == object_id {
                                continue;
                            }
                            for grip in &other.grips {
                                let target_id = puzzle5d_grip_full_id(&other.id, &grip.id);
                                if target_id == source_id {
                                    continue;
                                }
                                let target_position = world_grip_position(other, grip);
                                let dx = source_position[0] - target_position[0];
                                let dy = source_position[1] - target_position[1];
                                let dz = source_position[2] - target_position[2];
                                if (dx * dx + dy * dy + dz * dz).sqrt() <= PUZZLE5D_PROXIMITY_RADIUS
                                    && !envelope.document.fasteners.iter().any(|entry| entry.source == source_id && entry.target == target_id || entry.source == target_id && entry.target == source_id)
                                {
                                    envelope.document.fasteners.push(Puzzle5dFastener { id: next_fastener_id(), source: source_id.clone(), target: target_id, fastener_kind: None });
                                }
                            }
                        }
                    }
                    self.drive_precompute(&envelope);
                }
            }
            "setSelectionMethod" => {
                let method = args.and_then(|value| value.get("method")).and_then(|value| value.as_str()).unwrap_or("rectangle");
                envelope.runtime.selection_method = method.into();
            }
            "setLodMode" => {
                if let Some(mode) = args.and_then(|value| value.get("value").or_else(|| value.get("mode"))).and_then(|value| value.as_str()) {
                    envelope.runtime.lod_mode = mode.into();
                }
            }
            "setSuggestionOffset" => {
                if let Some(distance) = args.and_then(|value| value.get("distance").or_else(|| value.get("value"))).and_then(|value| value.as_f64()) {
                    envelope.runtime.suggestion_offset = distance.clamp(PUZZLE5D_SUGGESTION_OFFSET_MIN, PUZZLE5D_SUGGESTION_OFFSET_MAX);
                }
            }
            "setGridSnapEnabled" => {
                envelope.runtime.grid_snap_enabled = args.and_then(|value| value.get("enabled")).and_then(|value| value.as_bool()).unwrap_or(false);
            }
            "setGridFactor" => {
                if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
                    envelope.runtime.grid_factor = value;
                }
            }
            "applyBoardEvents" => {
                if let Some(events_json) = args.and_then(|value| value.get("eventsJson")).and_then(|value| value.as_str()) {
                    self.apply_board_events_from_json(events_json, &mut envelope);
                }
            }
            "worldPointerDown" | "canvasPointerDown" => return ActionEmit::default(),
            _ => {}
        }
        let next_active_utility = envelope.active_utility.clone();
        self.runtime = envelope.runtime;
        let operations = puzzle5d_operations_from_document_change(&before, &envelope.document);
        // 🌀 Coalesce each gumball drag tick into one undoable edit (compact per-part records, not full meshes).
        let coalesce_key = match action {
            "translateSelection" => Some("gumball-translate".to_string()),
            "rotateSelection" => Some("gumball-rotate".to_string()),
            "scaleSelection" => Some("gumball-scale".to_string()),
            _ => None,
        };
        // 🧰 Programmatic utility switches (engagement submit/abort, fill) push the active utility back into the
        // host session for both windows; `setActiveUtility` itself never re-emits (the host already applied it).
        let effects = if next_active_utility != active_utility_initial {
            PUZZLE5D_PLAY_WINDOWS.iter().map(|window| HostEffect::SetActiveUtility { window_id: (*window).into(), utility_id: next_active_utility.clone() }).collect()
        } else {
            Vec::new()
        };
        ActionEmit { operations, coalesce_key, effects, ..Default::default() }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Puzzle5dPlayProjection>, view_state: &ViewState) -> UiNode {
        let active_utility = puzzle5d_scene_active_utility(view_state, view_state.window_id.as_deref());
        let envelope = scene_from_projection(&doc.projection.0, self.runtime.clone(), &active_utility);
        let labels = puzzle5d_labels(view_state);
        match body_key {
            PUZZLE5D_PLAY_BODY_2D => build_board2d_scene(PUZZLE5D_PLAY_SURFACE_2D, PUZZLE5D_PLAY_CONTROLLER_ID, puzzle5d_board_scene(&envelope)),
            PUZZLE5D_PLAY_BODY_3D => {
                let brush_preview = world_brush_preview_json(&self.precompute, &envelope);
                build_world_3d_scene(
                    PUZZLE5D_PLAY_SURFACE_3D,
                    PUZZLE5D_PLAY_CONTROLLER_ID,
                    world3d_scene_extended(
                        camera3d_json(&envelope.document.camera3d),
                        world_meshes_json(&envelope.document),
                        world_instances_json(&envelope.document, &envelope.runtime),
                        world_selection_json_ex(&envelope),
                        Some(world_grips_json(&envelope.document)),
                        Some(world_fasteners_json(&envelope.document)),
                        None,
                        None,
                        brush_preview,
                        Some(world_interaction_json(&envelope.runtime, &envelope.active_utility)),
                        None,
                        None,
                        Some(world3d_chunking_json(256.0, 8000.0)),
                        puzzle5d_context_menu_json(&envelope, labels),
                        Some(world3d_environment_json(&envelope.runtime.sun)),
                    ),
                )
            }
            PUZZLE5D_PLAY_BODY_DOCUMENT => build_document_tree(&envelope, labels),
            PUZZLE5D_PLAY_BODY_KINDS => build_kinds_tree(&envelope, labels),
            PUZZLE5D_PLAY_BODY_INSPECTOR => build_inspector_tree(&envelope, labels),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_engagements(&self, doc: &DocumentView<'_, Puzzle5dPlayProjection>, view_state: &ViewState) -> HashMap<String, WindowEngagement> {
        let labels = puzzle5d_labels(view_state);
        // 🪟 One entry per live window INSTANCE of each of the 2D/3D window kinds — a split/extra
        // instance gets its own entry instead of being silently absent.
        PUZZLE5D_PLAY_WINDOWS
            .iter()
            .flat_map(|window| {
                window_instance_ids(view_state, window).into_iter().map(|wid| {
                    let active_utility = puzzle5d_scene_active_utility(view_state, Some(&wid));
                    let envelope = scene_from_projection(&doc.projection.0, self.runtime.clone(), &active_utility);
                    (wid, puzzle5d_engagement(&envelope, window, labels))
                })
            })
            .collect()
    }

    fn window_measures(&self, doc: &DocumentView<'_, Puzzle5dPlayProjection>, view_state: &ViewState) -> HashMap<String, Vec<WindowMeasure>> {
        let labels = puzzle5d_labels(view_state);
        PUZZLE5D_PLAY_WINDOWS
            .iter()
            .flat_map(|window| {
                window_instance_ids(view_state, window).into_iter().map(|wid| {
                    let active_utility = puzzle5d_scene_active_utility(view_state, Some(&wid));
                    let envelope = scene_from_projection(&doc.projection.0, self.runtime.clone(), &active_utility);
                    (wid, puzzle5d_window_measures(window, &envelope, &self.precompute, labels))
                })
            })
            .collect()
    }

    fn app_labels(&self, view_state: &ViewState) -> semio_framework_plugin::AppLabelsOverlay {
        let labels = puzzle5d_labels(view_state);
        semio_framework_plugin::AppLabelsOverlay {
            window_kind_labels: std::collections::HashMap::from([
                (PUZZLE5D_PLAY_WINDOW_2D.to_string(), labels.window_2d.to_string()),
                (PUZZLE5D_PLAY_WINDOW_3D.to_string(), labels.window_3d.to_string()),
            ]),
            panel_tab_labels: std::collections::HashMap::new(),
            mode_labels: std::collections::HashMap::new(),
            action_labels: puzzle5d_action_labels(view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"))),
            utility_labels: puzzle5d_utility_labels(view_state.locale.as_deref().is_some_and(|locale| locale.starts_with("de"))),
            example_labels: std::collections::HashMap::from([(PUZZLE5D_EXAMPLE_CONCRETE_FOREST.to_string(), labels.example_concrete_forest.to_string())]),
            action_arg_labels: HashMap::new(),
            dialog_labels: HashMap::new(),
            introduction_labels: HashMap::new(),
            tutorial_labels: HashMap::new(),
            group_labels: HashMap::new(),
        }
    }
}
//#endregion 🔖Puzzle5dPlayApp

//#region 🔖CommandLabels
/// 🗣️ (action id) -> localized label for every operation/view-action declared in `create_puzzle5d_app`'s
/// static manifest — mirrors `puzzle3d_action_labels`.
fn puzzle5d_action_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("setFixtureJson", "Set Fixture Json", "Fixture-JSON festlegen"),
        ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
        ("addNode", "Add Node", "Knoten hinzufügen"),
        ("addPartKind", "Add Part", "Teil hinzufügen"),
        ("addBrushPart", "Add Brush Part", "Pinselteil hinzufügen"),
        ("addBrushObject", "Add Brush Object", "Pinselobjekt hinzufügen"),
        ("deleteSelection", "Delete Selection", "Auswahl löschen"),
        ("duplicateSelection", "Duplicate Selection", "Auswahl duplizieren"),
        ("setSelectionFlag", "Set Selection Flag", "Auswahlmarkierung festlegen"),
        ("zoomToSelection", "Zoom To Selection", "Auf Auswahl zoomen"),
        ("focusSelection", "Focus Selection", "Auswahl fokussieren"),
        ("engagementSubmit", "Engagement Submit", "Eingabe bestätigen"),
        ("setFillCount", "Set Fill Count", "Füllanzahl festlegen"),
        ("patchPart", "Patch Part", "Teil aktualisieren"),
        ("patchGrip", "Patch Grip", "Griff aktualisieren"),
        ("setCamera", "Set Camera", "Kamera festlegen"),
        ("setCamera2d", "Set Camera 2D", "Kamera 2D festlegen"),
        ("setCamera3d", "Set Camera 3D", "Kamera 3D festlegen"),
        ("translateSelection", "Translate Selection", "Auswahl verschieben"),
        ("rotateSelection", "Rotate Selection", "Auswahl drehen"),
        ("scaleSelection", "Scale Selection", "Auswahl skalieren"),
        ("worldRelocate", "Relocate Part", "Teil verlagern"),
        ("applyBoardEvents", "Apply Board Events", "Board-Ereignisse anwenden"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("documentSelect", "Document Select", "Dokument auswählen"),
        ("clearSelection", "Clear Selection", "Auswahl aufheben"),
        ("selectAll", "Select All", "Alles auswählen"),
        ("selectSameKindSelection", "Select Same Kind", "Gleiche Art auswählen"),
        ("selectSameKind", "Select Same Kind (alias)", "Gleiche Art auswählen (Alias)"),
        ("toggleSun", "Toggle Sun", "Sonne umschalten"),
        ("setSunAzimuth", "Set Sun Azimuth", "Sonnenazimut festlegen"),
        ("setSunElevation", "Set Sun Elevation", "Sonnenhöhe festlegen"),
        ("setSunIntensity", "Set Sun Intensity", "Sonnenintensität festlegen"),
        ("engagementInput", "Engagement Input", "Eingabe"),
        ("engagementAbort", "Engagement Abort", "Eingabe abbrechen"),
        ("engagementControlSelect", "Engagement Control Select", "Eingabesteuerung auswählen"),
        ("cycleBrushCandidate", "Cycle Brush Candidate", "Pinselkandidat wechseln"),
        ("registerBrushMesh", "Register Brush Mesh", "Pinsel-Mesh registrieren"),
        ("setBrushPlacementOverlapBudget", "Set Brush Placement Overlap Budget", "Pinsel-Überlappungsbudget festlegen"),
        ("setObjectKindWeight", "Set Object Kind Weight", "Objektart-Gewicht festlegen"),
        ("setVortexKindWeight", "Set Vortex Kind Weight", "Vortexart-Gewicht festlegen"),
        ("worldSelect", "World Select", "Welt auswählen"),
        ("worldPick", "World Pick", "Welt-Auswahl (Pick)"),
        ("worldHover", "World Hover", "Überfahren (Welt)"),
        ("setHover", "Set Hover", "Überfahren festlegen"),
        ("worldVortexHover", "World Vortex Hover", "Welt-Vortex-Hover"),
        ("worldVortexSelect", "World Vortex Select", "Welt-Vortex auswählen"),
        ("setSelectionMethod", "Set Selection Method", "Auswahlmethode festlegen"),
        ("setLodMode", "Set Lod Mode", "LOD-Modus festlegen"),
        ("setSuggestionOffset", "Set Suggestion Offset", "Vorschlagsversatz festlegen"),
        ("setGridSnapEnabled", "Set Grid Snap Enabled", "Rasterfang aktivieren"),
        ("setGridFactor", "Set Grid Factor", "Rasterfaktor festlegen"),
        ("worldPointerDown", "World Pointer Down", "Welt-Zeiger gedrückt"),
        ("canvasPointerDown", "Canvas Pointer Down", "Leinwand-Zeiger gedrückt"),
    ];
    ENTRIES.iter().map(|(id, en, de)| ((*id).to_string(), (if is_de { *de } else { *en }).to_string())).collect()
}

/// 🗣️ (utility id) -> localized utility bar button label, for every `.utility(...)` declared in `create_puzzle5d_app`.
fn puzzle5d_utility_labels(is_de: bool) -> std::collections::HashMap<String, String> {
    const ENTRIES: &[(&str, &str, &str)] = &[
        ("select", "Select", "Auswählen"),
        ("move", "Move", "Verschieben"),
        ("rotate", "Rotate", "Drehen"),
        ("scale", "Scale", "Skalieren"),
        ("brush", "Brush", "Pinsel"),
        ("fill", "Fill", "Füllen"),
        ("worldRelocate", "Relocate", "Verlagern"),
    ];
    ENTRIES.iter().map(|(id, en, de)| ((*id).to_string(), (if is_de { *de } else { *en }).to_string())).collect()
}
//#endregion 🔖CommandLabels

//#region 🔖Manifest
pub fn create_puzzle5d_app() -> App {
    let envelope = Puzzle5dScene { document: default_document(), runtime: Puzzle5dRuntime::default(), active_utility: PUZZLE5D_DEFAULT_UTILITY.into() };
    let precompute = Puzzle5dPrecomputeSession::new();
    let manifest_labels = puzzle5d_labels(&ViewState::default());
    let mut app = App::from_builder(
        App::builder(PUZZLE5D_PLAY_APP_ID, "Puzzle 5D")
            .document(["semio", "puzzle", "5d"])
            .artifact_kind(ArtifactKindSpec {
                id: "5d.puzzle".into(),
                name: "5D Puzzle".into(),
                source_format: "puzzle.5d".into(),
                component_kind: "puzzle5d".into(),
                dimension: "5d".into(),
                media_capability: OsMediaCapability::MeshOnly,
                media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Design },
                schema: "puzzle.5d".into(),
                export_formats: vec![],
                import_formats: vec![],
            })
            .icon_id("puzzle")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "puzzle", "5d"])
            .mode("edit", "Edit")
            .default_mode_id("edit")
            .window_kind_with_engagement(PUZZLE5D_PLAY_WINDOW_2D, "Puzzle 2D", PUZZLE5D_PLAY_BODY_2D, SurfaceKind::Board2d, puzzle5d_engagement(&envelope, PUZZLE5D_PLAY_WINDOW_2D, manifest_labels), "layout-grid")
            .window_kind_with_engagement(PUZZLE5D_PLAY_WINDOW_3D, "Puzzle 3D", PUZZLE5D_PLAY_BODY_3D, SurfaceKind::World3d, puzzle5d_engagement(&envelope, PUZZLE5D_PLAY_WINDOW_3D, manifest_labels), "puzzle5d-3d")
            .default_layout(create_default_layout(&[PUZZLE5D_PLAY_WINDOW_2D.into(), PUZZLE5D_PLAY_WINDOW_3D.into()], "row", Some(&[50.0, 50.0]), Some(&["Puzzle 2D".into(), "Puzzle 3D".into()])))
            .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, PanelGroup::Workbench, PUZZLE5D_PLAY_BODY_DOCUMENT)
            .panel_tab(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, PanelGroup::Workbench, PUZZLE5D_PLAY_BODY_KINDS)
            .panel_tab(FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, PanelGroup::Details, PUZZLE5D_PLAY_BODY_INSPECTOR)
            // 🔧 Document-mutating operations (emit VCS operations through the before/after document delta).
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new("setFixtureJson", "Set Fixture Json", ActionKind::Operation) })
            .operation("setActiveExample", "Set Active Example")
            .operation("addNode", "Add Node")
            .operation("addPartKind", "Add Part")
            .operation("addBrushPart", "Add Brush Part")
            .operation("addBrushObject", "Add Brush Object")
            .operation("deleteSelection", "Delete Selection")
            .operation("duplicateSelection", "Duplicate Selection")
            .operation("setSelectionFlag", "Set Selection Flag")
            .operation("zoomToSelection", "Zoom To Selection")
            .operation("focusSelection", "Focus Selection")
            .operation("engagementSubmit", "Engagement Submit")
            .operation("setFillCount", "Set Fill Count")
            .operation("patchPart", "Patch Part")
            .operation("patchGrip", "Patch Grip")
            .operation("setCamera", "Set Camera")
            .operation("setCamera2d", "Set Camera 2D")
            .operation("setCamera3d", "Set Camera 3D")
            .operation("translateSelection", "Translate Selection")
            .operation("rotateSelection", "Rotate Selection")
            .operation("scaleSelection", "Scale Selection")
            .operation("worldRelocate", "Relocate Part")
            .operation("applyBoardEvents", "Apply Board Events")
            // 👁️ Ephemeral view state — selection, hover, utility parameters, brush cycling.
            .view_action("setSelection", "Set Selection")
            .view_action("documentSelect", "Document Select")
            .view_action("clearSelection", "Clear Selection")
            .view_action("selectAll", "Select All")
            .view_action("selectSameKindSelection", "Select Same Kind")
            .view_action("selectSameKind", "Select Same Kind (alias)")
            .view_action("toggleSun", "Toggle Sun")
            .view_action("setSunAzimuth", "Set Sun Azimuth")
            .view_action("setSunElevation", "Set Sun Elevation")
            .view_action("setSunIntensity", "Set Sun Intensity")
            .view_action("engagementInput", "Engagement Input")
            .view_action("engagementAbort", "Engagement Abort")
            .view_action("engagementControlSelect", "Engagement Control Select")
            .view_action("cycleBrushCandidate", "Cycle Brush Candidate")
            .view_action("registerBrushMesh", "Register Brush Mesh")
            .view_action("setBrushPlacementOverlapBudget", "Set Brush Placement Overlap Budget")
            .view_action("setObjectKindWeight", "Set Object Kind Weight")
            .view_action("setVortexKindWeight", "Set Vortex Kind Weight")
            .view_action("worldSelect", "World Select")
            .view_action("worldPick", "World Pick")
            .view_action("worldHover", "World Hover")
            .view_action("setHover", "Set Hover")
            .view_action("worldVortexHover", "World Vortex Hover")
            .view_action("worldVortexSelect", "World Vortex Select")
            .view_action("setSelectionMethod", "Set Selection Method")
            .view_action("setLodMode", "Set Lod Mode")
            .view_action("setSuggestionOffset", "Set Suggestion Offset")
            .view_action("setGridSnapEnabled", "Set Grid Snap Enabled")
            .view_action("setGridFactor", "Set Grid Factor")
            .view_action("worldPointerDown", "World Pointer Down")
            .view_action("canvasPointerDown", "Canvas Pointer Down")
            // 📝 Staged argument forms for the brush create actions (P1).
            .action_args("addPartKind", vec![
                ActionArgDef::select("partKind", "Kind", vec![ActionArgOption::new("Part", "Part")]).default_value("Part"),
            ])
            .action_args("addBrushPart", vec![
                ActionArgDef::select("partKind", "Kind", vec![ActionArgOption::new("Part", "Part")]).default_value("Part"),
            ])
            .action_args("addBrushObject", vec![
                ActionArgDef::select("partKind", "Kind", vec![ActionArgOption::new("Part", "Part")]).default_value("Part"),
            ])
            // 🧰 Flat per-window set of utilities (host-owned `view_state.active_utility_id`); `select` is the default.
            .utility(UtilityDefinition { category: Some(UtilityCategory::Selection), ..UtilityDefinition::new("select", "Select", "mouse-pointer") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("move", "Move", "move") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("rotate", "Rotate", "rotate-cw") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("scale", "Scale", "maximize-2") })
            .utility(UtilityDefinition::new("brush", "Brush", "paintbrush"))
            .utility(UtilityDefinition::new("fill", "Fill", "paint-bucket"))
            .utility(UtilityDefinition::new("worldRelocate", "Relocate", "relocate-3d"))
            .window_kind_utilities(PUZZLE5D_PLAY_WINDOW_2D, vec!["select".into(), "brush".into(), "fill".into()])
            .window_kind_utilities(PUZZLE5D_PLAY_WINDOW_3D, vec!["move".into(), "rotate".into(), "scale".into(), "brush".into(), "fill".into(), "worldRelocate".into()])
            // 📇 Per-window action scoping — the 3D window (World3d) owns the transform-gumball operations
            // (move/rotate/scale/relocate utilities are 3D-only) plus its own camera; the 2D window
            // (Board2d) owns board-event dispatch and its own camera. Select/brush/fill create
            // operations, deletion, engagement, and global example/json actions apply to both surfaces and
            // stay unscoped orphans, appearing on both windows.
            .window_kind_actions(PUZZLE5D_PLAY_WINDOW_3D, vec![
                "translateSelection".into(), "rotateSelection".into(), "scaleSelection".into(),
                "worldRelocate".into(), "setCamera3d".into(),
            ])
            .window_kind_actions(PUZZLE5D_PLAY_WINDOW_2D, vec![
                "applyBoardEvents".into(), "setCamera2d".into(),
            ]),
    );
    for window in PUZZLE5D_PLAY_WINDOWS {
        if let Some(window_kind) = app.definition.window_kinds.iter_mut().find(|window_kind| window_kind.id == window) {
            window_kind.options.measures = puzzle5d_window_measures(window, &envelope, &precompute, manifest_labels);
        }
    }
    app.example(PUZZLE5D_EXAMPLE_CONCRETE_FOREST, "Concrete Forest", CONCRETE_FOREST_EXAMPLE_JSON.clone())
        .example(PUZZLE5D_EXAMPLE_NAKAGIN, "Nakagin Capsule Tower", NAKAGIN_EXAMPLE_JSON.clone())
        .workflow("puzzle5d", "Puzzle 5D", "model")
}

/// 📥 Tier C DWG mesh import — always returns the empty puzzle-5d document; never errors on a structurally valid mesh.
fn puzzle5d_document_from_mesh(_mesh: &semio_framework_plugin::MeshData) -> Result<Value, String> {
    serde_json::to_value(empty_document()).map_err(|error| error.to_string())
}

pub fn register_puzzle5d_exports() {
    // 🗂️ Registers `Puzzle5dPlayProjection`'s pack<->dsl codec under its real `document_schema()`
    // string so `framework/sync`'s `FolderEndpoint::Pack` can print/parse puzzle-5d play documents
    // without depending on this crate's concrete `Projection`/`Operation` types.
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Puzzle5dPlayApp>(PUZZLE5D_SCHEMA);
    register_mesh_exporter("5d.puzzle", "puzzle5d", |_| Ok(semio_framework_plugin::mesh_from_kind("box")), Box::new(semio_framework_plugin::ObjExporter));
    register_mesh_exporter("5d.puzzle", "puzzle5d", |_| Ok(semio_framework_plugin::mesh_from_kind("box")), Box::new(semio_framework_plugin::GlbExporter));
    register_mesh_exporter("5d.puzzle", "puzzle5d", |_| Ok(semio_framework_plugin::mesh_from_kind("box")), Box::new(semio_framework_plugin::StlExporter));
    register_mesh_importer("5d.puzzle", puzzle5d_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
    register_mesh_importer("5d.puzzle", puzzle5d_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
    register_mesh_importer("5d.puzzle", puzzle5d_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
    semio_framework_os::register_mesh_dwg_export_handler("5d.puzzle", "puzzle5d", |_| Ok(semio_framework_plugin::mesh_from_kind("box")));
    semio_framework_os::register_mesh_dwg_import_handler("5d.puzzle", puzzle5d_document_from_mesh);
}
//#endregion 🔖Manifest

//#region 🔖WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    use super::*;
    use puzzle_5d::PUZZLE_5D_SCHEMA;
    use puzzle_5d_engine::empty_puzzle5d_projection;
    use puzzle_5d_op::{Puzzle5dEnvelope, Puzzle5dStore};
    use std::cell::RefCell;
    use store::create_document_envelope;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub struct Puzzle5dDocumentVcs {
        store: RefCell<Puzzle5dStore>,
    }

    #[wasm_bindgen]
    impl Puzzle5dDocumentVcs {
        #[wasm_bindgen(constructor)]
        pub fn new(envelope_json: Option<String>) -> Result<Puzzle5dDocumentVcs, JsValue> {
            let store = match envelope_json {
                Some(json) => {
                    let envelope: Puzzle5dEnvelope = serde_json::from_str(&json).map_err(|e| JsValue::from_str(&e.to_string()))?;
                    Puzzle5dStore::new(envelope)
                }
                None => Puzzle5dStore::new(create_document_envelope(PUZZLE_5D_SCHEMA, "puzzle5d", empty_puzzle5d_projection(), None)),
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
}
//#endregion 🔖WasmBridge

//#region 🧪Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_plugin::{testkit, PluginApp, VcsDocumentApp};

    fn new_app_with_registry() -> VcsDocumentApp<Puzzle5dPlayApp> {
        testkit::new_app_with_registry::<Puzzle5dPlayApp>(create_puzzle5d_app)
    }

    fn part_count(app: &VcsDocumentApp<Puzzle5dPlayApp>) -> usize {
        app.projection().expect("projection").0.get("parts").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0)
    }

    #[test]
    fn renders_paired_board_and_world_scenes() {
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        let board = serde_json::to_string(&app.render(PUZZLE5D_PLAY_BODY_2D, None, &ViewState::default()).expect("render 2d")).unwrap();
        assert!(board.contains("board-2d"));
        let world = serde_json::to_string(&app.render(PUZZLE5D_PLAY_BODY_3D, None, &ViewState::default()).expect("render 3d")).unwrap();
        assert!(world.contains("world-3d"));
    }

    #[test]
    fn initial_projection_is_the_concrete_forest_document() {
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        assert_eq!(app.projection().expect("projection").0.get("schema").and_then(|value| value.as_str()), Some(PUZZLE5D_SCHEMA));
        assert!(part_count(&app) > 0, "the concrete-forest default document ships with parts");
    }

    /// 📦 `Puzzle5dPlayProjection`'s pack encoding round-trips through the same `(RecordSpec,
    /// RecordValue)` pair its `parse_dsl`/`print_dsl` do (both delegate to the underlying
    /// `serde_json::Value` bridge impls), reusing the default concrete-forest fixture the test
    /// above already loads.
    #[test]
    fn puzzle5d_play_projection_pack_round_trips() {
        let app = testkit::new_app::<Puzzle5dPlayApp>();
        store::test_support::assert_dsl_pack_equivalence(&app.projection().expect("projection"));
    }

    #[test]
    fn set_active_example_swaps_the_document_and_undo_restores_it() {
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        let loaded = part_count(&app);
        assert!(loaded > 0);
        app.handle_action("setActiveExample", Some(&json!({ "exampleId": "" })), &ViewState::default(), &testkit::meta("local")).expect("empty");
        assert_eq!(part_count(&app), 0, "empty example clears the parts");
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("local")).expect("undo");
        assert_eq!(part_count(&app), loaded, "undo restores the concrete-forest parts");
        app.handle_action("redo", None, &ViewState::default(), &testkit::meta("local")).expect("redo");
        assert_eq!(part_count(&app), 0);
    }

    #[test]
    fn document_panel_renders() {
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        let node = app.render(PUZZLE5D_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        assert!(!serde_json::to_string(&node).unwrap().is_empty());
    }

    #[test]
    fn app_definition_has_the_paired_windows() {
        let app = create_puzzle5d_app();
        let ids: Vec<&str> = app.definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
        assert!(ids.contains(&PUZZLE5D_PLAY_WINDOW_2D) && ids.contains(&PUZZLE5D_PLAY_WINDOW_3D));
    }

    #[test]
    fn window_kind_actions_scope_transform_to_3d_only() {
        let definition = create_puzzle5d_app().definition;
        let resolve = |window_id: &str| -> Vec<String> {
            let window = definition.window_kinds.iter().find(|window| window.id == window_id).unwrap();
            semio_framework_plugin::resolve_window_actions(&definition, window)
                .into_iter()
                .map(|action| action.id.clone())
                .collect()
        };
        let board = resolve(PUZZLE5D_PLAY_WINDOW_2D);
        let world = resolve(PUZZLE5D_PLAY_WINDOW_3D);
        for transform_operation in ["translateSelection", "rotateSelection", "scaleSelection", "worldRelocate", "setCamera3d"] {
            assert!(world.contains(&transform_operation.to_string()), "3D must expose {transform_operation}");
            assert!(!board.contains(&transform_operation.to_string()), "2D must NOT expose {transform_operation}");
        }
        assert!(board.contains(&"applyBoardEvents".to_string()), "2D must expose applyBoardEvents");
        assert!(!world.contains(&"applyBoardEvents".to_string()), "3D must NOT expose applyBoardEvents");
        for shared in ["addBrushPart", "deleteSelection"] {
            assert!(board.contains(&shared.to_string()) && world.contains(&shared.to_string()), "{shared} stays on both windows");
        }
    }

    #[test]
    fn window_engagements_cover_both_windows() {
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        let engagements = app.window_engagements(&ViewState::default());
        assert!(engagements.contains_key(PUZZLE5D_PLAY_WINDOW_2D));
        assert!(engagements.contains_key(PUZZLE5D_PLAY_WINDOW_3D));
    }

    //#region 🧰 Window Actions & Utilities contract
    #[test]
    fn add_part_kind_materializes_the_declared_kind_default() {
        // 📝 P1 arg form: addPartKind with no args materializes the declared `partKind` default and adds a part.
        let mut app = new_app_with_registry();
        app.handle_action("setActiveExample", Some(&json!({ "exampleId": "" })), &ViewState::default(), &testkit::meta("local")).expect("empty");
        let before = part_count(&app);
        let result = app.handle_action("addPartKind", None, &ViewState::default(), &testkit::meta("local")).expect("addPartKind");
        assert!(!result.operations.is_empty(), "addPartKind is an Operation that emits operations");
        assert_eq!(part_count(&app), before + 1, "the materialized default kind adds exactly one part");
        let projection = app.projection().expect("projection").0;
        let kind = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.last()).and_then(|part| part.get("partKind")).and_then(Value::as_str);
        assert_eq!(kind, Some("Part"), "the declared partKind default was materialized host-side");
    }

    #[test]
    fn set_active_utility_emits_no_ops_and_no_history_entry() {
        // 🧰 Switching utilities is the framework View action: no document operations, no undo entry, no re-emitted effect.
        let mut app = new_app_with_registry();
        let before = app.projection().expect("projection").0;
        let brush_view = ViewState { active_utility_id: Some("brush".into()), ..ViewState::default() };
        let result = app.handle_action(SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "brush" })), &brush_view, &testkit::meta("local")).expect("switch utility");
        assert!(result.operations.is_empty(), "utility switching never emits document operations");
        assert!(result.requested_effects.is_empty(), "a user utility switch does not re-emit SetActiveUtility");
        assert_eq!(app.projection().expect("projection").0, before, "utility switching does not mutate the document");
    }

    #[test]
    fn engagements_expose_no_utility_switch_options_for_either_window() {
        // 🧰 select/brush/fill switching lives only on the framework utility bar; neither the 2D nor the 3D
        // engagement HUD may duplicate it as options.
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        let engagements = app.window_engagements(&ViewState::default());
        for window in [PUZZLE5D_PLAY_WINDOW_2D, PUZZLE5D_PLAY_WINDOW_3D] {
            assert!(engagements.get(window).expect("engagement").options.is_none(), "the {window} engagement must not re-expose utility switching as options");
        }
    }

    /// 🎯 D-3 follow-up: the fill-count slider and brush placement picker are tagged `WindowMeasure::Group`s
    /// in [`puzzle5d_window_measures`] (surfaced by `partition_window_measures` only for their active utility),
    /// never `WindowEngagementControl`s on the HUD — for both the 2D and 3D windows.
    #[test]
    fn fill_and_brush_params_are_tagged_utility_options_not_engagement_controls() {
        let labels = puzzle5d_labels(&ViewState::default());
        let session = Puzzle5dPrecomputeSession::new();
        let group_tag = |measures: &[WindowMeasure], id: &str| {
            measures.iter().find_map(|measure| match measure {
                WindowMeasure::Group { id: gid, active_utility_id, .. } if gid == id => Some(active_utility_id.clone()),
                _ => None,
            })
        };
        let fill_slider_in_group = |measures: &[WindowMeasure], group_id: &str| {
            measures.iter().any(|measure| matches!(measure, WindowMeasure::Group { id, children, .. } if id == group_id && children.iter().any(|child| matches!(child, WindowMeasure::Slider { id, .. } if id == "puzzle5d-fill-count"))))
        };
        // 🪣 Fill utility: the fill-count slider now lives in a "fill"-tagged Utility Options group (per window),
        // NOT the engagement HUD.
        let mut fill_runtime = Puzzle5dRuntime::default();
        fill_runtime.fill_count = 3;
        let fill_scene = Puzzle5dScene { document: default_document(), runtime: fill_runtime, active_utility: "fill".into() };
        for window in [PUZZLE5D_PLAY_WINDOW_2D, PUZZLE5D_PLAY_WINDOW_3D] {
            let measures = puzzle5d_window_measures(window, &fill_scene, &session, labels);
            assert_eq!(group_tag(&measures, "puzzle5d-play-utility-options-fill"), Some(Some("fill".into())), "{window} fill Utility Options must be tagged for the fill utility");
            assert!(fill_slider_in_group(&measures, "puzzle5d-play-utility-options-fill"), "{window} fill Utility Options must carry the fill-count slider");
            let fill_hud = puzzle5d_engagement(&fill_scene, window, labels);
            assert!(fill_hud.control.is_none() && fill_hud.controls.is_none(), "{window} fill engagement HUD must no longer carry the relocated control");
        }
        // 🖌️ Brush utility: with no candidates to place, the "brush"-tagged group is absent (matching the old
        // gate), and the engagement HUD is likewise bare.
        let brush_scene = Puzzle5dScene { document: default_document(), runtime: Puzzle5dRuntime::default(), active_utility: "brush".into() };
        for window in [PUZZLE5D_PLAY_WINDOW_2D, PUZZLE5D_PLAY_WINDOW_3D] {
            assert_eq!(group_tag(&puzzle5d_window_measures(window, &brush_scene, &session, labels), "puzzle5d-play-utility-options-brush"), Some(Some("brush".into())), "{window} brush Utility Options surfaces even without candidates");
            let brush_hud = puzzle5d_engagement(&brush_scene, window, labels);
            assert!(brush_hud.control.is_none() && brush_hud.controls.is_none(), "{window} brush engagement HUD must no longer carry the relocated control");
        }
        // 🖌️ The positive brush-candidate surfacing (group present ⇒ tagged "brush") is proven by
        // construction: `puzzle5d_brush_utility_options` returns the same tagged `Select` group shape as the
        // d3 helper, whose end-to-end positive path is covered by the sibling d3 test.
    }

    #[test]
    fn engagement_submit_switches_utility_via_host_effect_for_both_windows() {
        // 🧰 Reconciled dual entry point: the engagement token drives the same host-owned utility switch, once per window.
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        let result = app.handle_action("engagementSubmit", Some(&json!({ "window": PUZZLE5D_PLAY_WINDOW_3D, "value": "brush" })), &ViewState::default(), &testkit::meta("local")).expect("submit");
        let windows: Vec<&str> = result.requested_effects.iter().filter_map(|effect| match effect { HostEffect::SetActiveUtility { window_id, utility_id } if utility_id == "brush" => Some(window_id.as_str()), _ => None }).collect();
        assert!(windows.contains(&PUZZLE5D_PLAY_WINDOW_2D) && windows.contains(&PUZZLE5D_PLAY_WINDOW_3D), "brush switch is pushed to both windows, got {windows:?}");
    }

    #[test]
    fn gumball_translate_drag_coalesces_into_one_edit() {
        // 🌀 Coalescing regression: three translate ticks with the same key are ONE undoable edit.
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        let part_id = app.projection().expect("projection").0.get("parts").and_then(Value::as_array).and_then(|parts| parts.first()).and_then(|part| part.get("id")).and_then(Value::as_str).expect("part id").to_string();
        let origin_x = |app: &VcsDocumentApp<Puzzle5dPlayApp>| -> f64 {
            app.projection().expect("projection").0.get("parts").and_then(Value::as_array).and_then(|parts| parts.iter().find(|part| part.get("id").and_then(Value::as_str) == Some(part_id.as_str()))).and_then(|part| part.pointer("/3d/origin/0")).and_then(Value::as_f64).unwrap_or(0.0)
        };
        let start = origin_x(&app);
        let move_view = ViewState { active_utility_id: Some("move".into()), ..ViewState::default() };
        for dx in [1.0, 2.0, 3.0] {
            app.handle_action("translateSelection", Some(&json!({ "ids": [part_id], "dx": dx, "dy": 0.0, "dz": 0.0 })), &move_view, &testkit::meta("local")).expect("drag tick");
        }
        assert!((origin_x(&app) - start - 6.0).abs() < 1e-9, "three ticks accumulate 1+2+3 on x");
        app.handle_action("undo", None, &ViewState::default(), &testkit::meta("local")).expect("undo");
        assert!((origin_x(&app) - start).abs() < 1e-9, "one undo restores the whole coalesced gumball drag");
    }
    //#endregion 🧰 Window Actions & Utilities contract
}
//#endregion 🧪Tests
