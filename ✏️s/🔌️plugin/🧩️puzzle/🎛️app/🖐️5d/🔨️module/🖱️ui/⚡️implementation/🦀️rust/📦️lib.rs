//! 👯️ Puzzle 5d app — DocumentApp impl, render, manifest (constitutional: ui).

use puzzle_5d::Puzzle5dProjection;
use puzzle_5d_engine::{import_compose_design_json, BrushPlacePayload, Puzzle5dPrecomputeSession};
use puzzle_5d_op::{puzzle5d_document_delta_operations, Puzzle5dOperation, Puzzle5dPlayProjection};
use semio_framework_plugin::{
    apply_world3d_sun_action, build_board2d_scene, build_world_3d_scene, create_default_layout,
    ActionArgDef, ActionArgOption, ActionDefinition, ActionKind, ConfigView, DocumentApp, DocumentView, Emit, MeasureSelectItem, WindowEngagementStatus,
    merge_world_selection_ids, ui_declarative_sections_to_tree, ui_inspector_groups_to_tree, ui_inspector_readonly_field, ui_inspector_stepper_field, ui_inspector_vec3_group, ui_stack_vertical, ui_text, world3d_chunking_json, world3d_environment_json, world3d_mesh_id_from_url, world3d_meshes_json_from_urls, world3d_scene_extended, world3d_selection_json, world3d_sun_measures, App,
    ActionDescriptor, AppIo, Media, MediaClass, MediaError, MediaForm, MediaPortDirection, MediaPortSpec, MediaType, OsMediaCapability, PanelGroup, PortMultiplicity, ArtifactKindSpec, ArtifactPresentation, Board2dScene, SurfaceKind, UtilityCategory, UtilityDefinition, UiFieldNode, UiInspectorFieldGroup, UiNode, UiPresence, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, WindowEngagement, ui_tree_stamp_presence, IconName,
    WindowEngagementInput, WindowMeasure, WorldSunConfig, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, SET_ACTIVE_UTILITY_ACTION_ID, SelectionSet, AppLabels, Label, Locale, Terminology, LocalizedLabel, LabelText};
use semio_framework_plugin::kernel::{ClipboardError, ClipboardFragment, HostEffect, PasteAnchor, PastePlacement};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;

//#region 🔖️Constants
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
/// 🧰️ Host-owned active utility (`view_state.active_utility_id`) when the host hasn't set one yet — the first declared utility.
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
/// 🌉️ This app's own scratch fixture stays a local structural-twin mirror (`Puzzle5dDocument`) of
/// `puzzle_5d::Puzzle5dProjection` — see `puzzle_5d`'s `🔖️ValueBridge` region — so the DSL-text
/// example fixtures are parsed once into the typed `puzzle_5d::Puzzle5dProjection` and
/// re-serialized to the JSON string this module's `document_from_json`/`.example(...)` call sites expect.
static CONCRETE_FOREST_EXAMPLE_JSON: LazyLock<String> =
    LazyLock::new(|| serde_json::to_string(&<Puzzle5dProjection as store::DocumentDsl>::parse_dsl(CONCRETE_FOREST_EXAMPLE_DSL).expect("concrete-forest example fixture parses as dsl")).expect("serialize concrete-forest example fixture"));
static NAKAGIN_EXAMPLE_JSON: LazyLock<String> =
    LazyLock::new(|| serde_json::to_string(&<Puzzle5dProjection as store::DocumentDsl>::parse_dsl(NAKAGIN_EXAMPLE_DSL).expect("nakagin example fixture parses as dsl")).expect("serialize nakagin example fixture"));

static PUZZLE5D_ID_COUNTER: AtomicU32 = AtomicU32::new(0);
//#endregion 🔖️Constants

//#region 🔖️Terminology
// 🗣️ Complete UI label set for the 5D app — every field declares all four locale×terminology cells
// explicitly via `app_labels!`, so a missing translation fails to compile rather than silently
// falling back. Reuse-terminology fields that read identically to native just repeat the native text.
semio_framework_plugin::app_labels! {
    struct Puzzle5dLabels {
        parts: native_en "Parts", native_de "Teile", reuse_en "Building components", reuse_de "Baukomponenten";
        fasteners: native_en "Fasteners", native_de "Verbinder", reuse_en "Component connections", reuse_de "Baukomponentenverbindungen";
        grips: native_en "Grips", native_de "Griffe", reuse_en "Connection points", reuse_de "Verbindungspunkte";
        ropes: native_en "Ropes", native_de "Seile", reuse_en "Ropes", reuse_de "Seile";
        part: native_en "Part", native_de "Teil", reuse_en "Building component", reuse_de "Baukomponente";
        grip: native_en "Grip", native_de "Griff", reuse_en "Connection point", reuse_de "Verbindungspunkt";
        select: native_en "Select", native_de "Auswählen", reuse_en "Select", reuse_de "Auswählen";
        brush: native_en "Brush", native_de "Pinsel", reuse_en "Brush", reuse_de "Pinsel";
        fill: native_en "Fill", native_de "Füllen", reuse_en "Fill", reuse_de "Füllen";
        count: native_en "Count", native_de "Anzahl", reuse_en "Count", reuse_de "Anzahl";
        placement: native_en "Placement", native_de "Platzierung", reuse_en "Placement", reuse_de "Platzierung";
        duplicate: native_en "Duplicate", native_de "Duplizieren", reuse_en "Duplicate", reuse_de "Duplizieren";
        select_same_kind: native_en "Select all of same kind", native_de "Alle gleicher Art auswählen", reuse_en "Select all of same kind", reuse_de "Alle gleicher Art auswählen";
        zoom_to_selection: native_en "Zoom to selection", native_de "Auf Auswahl zoomen", reuse_en "Zoom to selection", reuse_de "Auf Auswahl zoomen";
        delete: native_en "Delete", native_de "Löschen", reuse_en "Delete", reuse_de "Löschen";
        hide: native_en "Hide", native_de "Ausblenden", reuse_en "Hide", reuse_de "Ausblenden";
        show: native_en "Show", native_de "Anzeigen", reuse_en "Show", reuse_de "Anzeigen";
        lock: native_en "Lock", native_de "Sperren", reuse_en "Lock", reuse_de "Sperren";
        unlock: native_en "Unlock", native_de "Entsperren", reuse_en "Unlock", reuse_de "Entsperren";
        lod: native_en "LOD", native_de "LOD", reuse_en "LOD", reuse_de "LOD";
        automatic: native_en "Automatic", native_de "Automatisch", reuse_en "Automatic", reuse_de "Automatisch";
        suggestion: native_en "Suggestion", native_de "Vorschlag", reuse_en "Suggestion", reuse_de "Vorschlag";
        offset: native_en "Offset", native_de "Versatz", reuse_en "Offset", reuse_de "Versatz";
        part_weights: native_en "Part Weights", native_de "Teilgewichte", reuse_en "Part Weights", reuse_de "Teilgewichte";
        grip_weights: native_en "Grip Weights", native_de "Griffgewichte", reuse_en "Grip Weights", reuse_de "Griffgewichte";
        overlap: native_en "Overlap", native_de "Überlappung", reuse_en "Overlap", reuse_de "Überlappung";
        window_2d: native_en "Puzzle 2D", native_de "Puzzle 2D", reuse_en "Puzzle 2D", reuse_de "Puzzle 2D";
        window_3d: native_en "Puzzle 3D", native_de "Puzzle 3D", reuse_en "Puzzle 3D", reuse_de "Puzzle 3D";
        // inspector field labels
        id: native_en "Id", native_de "Id", reuse_en "Id", reuse_de "Id";
        kind: native_en "Kind", native_de "Art", reuse_en "Kind", reuse_de "Art";
        label: native_en "Label", native_de "Bezeichnung", reuse_en "Label", reuse_de "Bezeichnung";
        flat_text: native_en "Flat text", native_de "Flachtext", reuse_en "Flat text", reuse_de "Flachtext";
        flat_x: native_en "Flat x", native_de "Flach-X", reuse_en "Flat x", reuse_de "Flach-X";
        flat_y: native_en "Flat y", native_de "Flach-Y", reuse_en "Flat y", reuse_de "Flach-Y";
        volume_origin: native_en "Volume origin", native_de "Volumenursprung", reuse_en "Volume origin", reuse_de "Volumenursprung";
        flat_angle: native_en "Flat angle", native_de "Flachwinkel", reuse_en "Flat angle", reuse_de "Flachwinkel";
        radius: native_en "Radius", native_de "Radius", reuse_en "Radius", reuse_de "Radius";
        position: native_en "Position", native_de "Position", reuse_en "Position", reuse_de "Position";
        direction: native_en "Direction", native_de "Richtung", reuse_en "Direction", reuse_de "Richtung";
        source: native_en "Source", native_de "Quelle", reuse_en "Source", reuse_de "Quelle";
        target: native_en "Target", native_de "Ziel", reuse_en "Target", reuse_de "Ziel";
        schema: native_en "Schema", native_de "Schema", reuse_en "Schema", reuse_de "Schema";
        utility: native_en "Utility", native_de "Werkzeug", reuse_en "Utility", reuse_de "Werkzeug";
        none: native_en "(none)", native_de "(keine)", reuse_en "(none)", reuse_de "(keine)";
        example_concrete_forest: native_en "Concrete Forest", native_de "Betonwald", reuse_en "Abbau Aufbau", reuse_de "Abbau Aufbau";
        gap: native_en "Gap", native_de "Abstand", reuse_en "Gap", reuse_de "Abstand";
        shift: native_en "Shift", native_de "Verschiebung", reuse_en "Shift", reuse_de "Verschiebung";
        rise: native_en "Rise", native_de "Anstieg", reuse_en "Rise", reuse_de "Anstieg";
        rotation: native_en "Rotation", native_de "Rotation", reuse_en "Rotation", reuse_de "Rotation";
        turn: native_en "Turn", native_de "Drehung", reuse_en "Turn", reuse_de "Drehung";
        tilt: native_en "Tilt", native_de "Neigung", reuse_en "Tilt", reuse_de "Neigung";
        mixed: native_en "Mixed", native_de "Gemischt", reuse_en "Mixed", reuse_de "Gemischt";
    }
}

/// 🗣️ Resolves the active label set from this document's persisted locale/terminology config
/// (see `Puzzle5dRuntime.locale`/`.terminology` — this app VCS's its own axes rather than reading
/// `ViewState`, so `resolve_labels::<Puzzle5dLabels>(view_state)` doesn't apply here).
fn puzzle5d_labels(config: &Puzzle5dConfig) -> &'static Puzzle5dLabels {
    let locale = if puzzle5d_is_de_locale(config) { Locale::De } else { Locale::En };
    let terminology = if config.terminology == "reuse" { Terminology::Reuse } else { Terminology::Native };
    Puzzle5dLabels::labels(locale, terminology)
}

/// 🗣️ Lifts a `Puzzle5dLabels` field accessor into a full manifest-level `LocalizedLabel` matrix —
/// for `.operation`/`.utility`/arg-option declarations that should track this app's own native/reuse
/// naming instead of a fixed, terminology-invariant string.
fn puzzle5d_localized(field: fn(&Puzzle5dLabels) -> LabelText) -> LocalizedLabel {
    LocalizedLabel::from_fn(|terminology, locale| field(Puzzle5dLabels::labels(locale, terminology)).as_str().to_string())
}
//#endregion 🔖️Terminology

//#region 🔖️Document
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
    part_ids: SelectionSet,
    #[serde(default)]
    grip_ids: SelectionSet,
    #[serde(default)]
    fastener_ids: SelectionSet,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Puzzle5dRuntime {
    /// 📷️ Camera pose — session-only view state (`ActionKind::View`), never a VCS document field:
    /// see `setCamera`/`setCamera2d`/`setCamera3d` in `Puzzle5dPlayApp::handle_action_impl`.
    #[serde(default)]
    camera2d: Puzzle5dCamera2d,
    #[serde(default)]
    camera3d: Puzzle5dCamera3d,
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
    /// 🧰️ B1: per-window (kind-keyed — puzzle5d's two window KINDS are each single-instance, see
    /// `window_instance_ids`) active utility — was host-pushed `view_state.active_utility_by_window_id`,
    /// now real VCS'd config (see `SET_ACTIVE_UTILITY_ACTION_ID` in `handle_action_impl`).
    #[serde(default)]
    active_utility_by_window_id: BTreeMap<String, String>,
    /// 🗣️ B1: terminology overlay (native/reuse) — was host-pushed `view_state.terminology`.
    #[serde(default = "default_terminology")]
    terminology: String,
    /// 🗣️ B1: BCP-47 locale tag — was host-pushed `view_state.locale` (read via the deleted
    /// `semio_framework_plugin::is_de_locale(&ViewState)`; see the local `puzzle5d_is_de_locale` below).
    #[serde(default = "default_locale")]
    locale: String,
}

fn default_terminology() -> String {
    "native".into()
}

fn default_locale() -> String {
    "en-US".into()
}

/// 🗣️ B1: local replacement for the deleted `semio_framework_plugin::is_de_locale(&ViewState)`.
fn puzzle5d_is_de_locale(config: &Puzzle5dRuntime) -> bool {
    config.locale.starts_with("de")
}

/// ⚠️ Explicit impl (not `#[derive(Default)]`) so Rust construction matches the serde field defaults above.
impl Default for Puzzle5dRuntime {
    fn default() -> Self {
        Self {
            camera2d: Puzzle5dCamera2d { x: 0.0, y: 0.0, zoom: 1.0 },
            camera3d: Puzzle5dCamera3d { position: [8.0, -8.0, 8.0], target: [0.0, 0.0, 0.0], zoom: 1.0 },
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
            active_utility_by_window_id: BTreeMap::new(),
            terminology: default_terminology(),
            locale: default_locale(),
        }
    }
}

//#region 🔖️Config
/// 🧮️ B1: puzzle5d's real `DocumentApp::Config` — `Puzzle5dRuntime` itself doubles as the config
/// record (an alias, not a new type), mirroring `puzzle_3d_ui::Puzzle3dConfig`'s identical recipe, so
/// every existing helper taking `&Puzzle5dRuntime`/`&mut Puzzle5dRuntime` throughout this file keeps
/// working unchanged; only `Puzzle5dPlayApp`'s own ambient `RefCell<Puzzle5dRuntime>` field is gone
/// (see `struct Puzzle5dPlayApp` below) — every read now comes from `cfg.projection`, every write
/// flows out as a `Puzzle5dConfigOperation` in the returned `Emit` instead of a silent `self` mutation.
pub type Puzzle5dConfig = Puzzle5dRuntime;

impl store::DocumentDsl for Puzzle5dRuntime {
    const EXTENSION: &'static str = "puzzle5dcfg";

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
}

impl store::DocumentPack for Puzzle5dRuntime {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        dsl::to_dsl_value(self).map_err(store::PackError::Schema)?.encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = dsl::DslValue::decode_pack_with(bytes, options)?;
        dsl::from_dsl_value(value).map_err(store::PackError::Schema)
    }
}

impl store::ConfigRecord for Puzzle5dRuntime {}

/// @emoji 🧮️ Whole-record diff — every `Puzzle5dConfigOperation` is a full-config `Snapshot` (see
/// `Puzzle5dConfigOperation` below), so `apply` ignores `base` entirely, matching
/// `puzzle_3d_ui::Puzzle3dRuntime`'s identical pattern.
impl protocol::OperationDiff<Puzzle5dRuntime> for Puzzle5dRuntime {
    fn apply(&self, _base: &Puzzle5dRuntime) -> Puzzle5dRuntime {
        self.clone()
    }
    fn absorb(&mut self, other: Self) {
        *self = other;
    }
}

/// @emoji 🧮️ B1: `Puzzle5dConfig`'s operation enum — lives here (in `ui`, not a lower `op`/`engine`
/// crate) because `Puzzle5dConfig` itself is a type alias for `Puzzle5dRuntime`, which is (and stays)
/// a ui-crate-local type. Mirrors `puzzle_3d_ui::Puzzle3dConfigOperation`'s single-generic-`Snapshot`-
/// variant pattern exactly: every real config edit is captured as "the whole config after this edit";
/// `backwards()` is the same one-liner regardless of what changed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Puzzle5dConfigOperation {
    Snapshot { config: Puzzle5dConfig },
}

impl protocol::Operation<Puzzle5dConfig> for Puzzle5dConfigOperation {
    type Diff = Puzzle5dConfig;

    fn diff(&self, _base: &Puzzle5dConfig) -> Puzzle5dConfig {
        match self {
            Puzzle5dConfigOperation::Snapshot { config } => config.clone(),
        }
    }

    fn backwards(&self, base: &Puzzle5dConfig) -> Vec<Self> {
        vec![Puzzle5dConfigOperation::Snapshot { config: base.clone() }]
    }
}

impl protocol::OpBinary for Puzzle5dConfigOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}

impl protocol::OpText for Puzzle5dConfigOperation {
    fn print_op(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        serde_json::from_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️Config

/// 🧾️ Transient render/mutation bundle pairing the persisted projection (the bare `Puzzle5dDocument`
/// json) with the app's ephemeral view state. Never persisted — the {@link VcsDocumentApp} store owns
/// the document and {@link Puzzle5dPlayApp} owns the runtime — but rebuilt per call so the existing
/// board/world/engagement helpers keep their `&scene` signatures.
#[derive(Clone)]
struct Puzzle5dScene {
    document: Puzzle5dDocument,
    runtime: Puzzle5dRuntime,
    /// 🧰️ The active utility for this window (was host-pushed `view_state.active_utility_id`) — transient, never persisted.
    active_utility: String,
}

/// 🧰️ B1: the active utility for `window_id`, from `Puzzle5dConfig::active_utility_by_window_id` (was
/// host-pushed `view_state.active_utility_by_window_id`/`view_state.active_utility_id`) — falls back to
/// [`PUZZLE5D_DEFAULT_UTILITY`] when the window has never had a utility switch recorded yet.
fn puzzle5d_scene_active_utility(config: &Puzzle5dConfig, window_id: Option<&str>) -> String {
    if let Some(wid) = window_id {
        if let Some(utility) = config.active_utility_by_window_id.get(wid) {
            return utility.clone();
        }
    }
    PUZZLE5D_DEFAULT_UTILITY.to_string()
}

/// 🧭️ The select/brush/fill interaction mode the world engine reads, derived from the flat active utility
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

/// 🧭️ Whether the active utility is a transform gumball mode.
fn puzzle5d_transform_utility_active(active_utility: &str) -> bool {
    puzzle5d_transform_handle(active_utility).is_some()
}

/// 🕹️ Whether the world gumball should render for the current selection and utility.
fn puzzle5d_gumball_active(runtime: &Puzzle5dRuntime, active_utility: &str) -> bool {
    !runtime.selection.part_ids.is_empty() && puzzle5d_transform_utility_active(active_utility)
}

/// 🧹️ Clears every selection bag.
fn puzzle5d_clear_selection(selection: &mut Puzzle5dSelection) {
    *selection = Puzzle5dSelection::default();
}

/// 🧹️ Clears every selection bag except part ids.
fn puzzle5d_clear_non_part_selection(selection: &mut Puzzle5dSelection) {
    selection.grip_ids.clear();
    selection.fastener_ids.clear();
}

/// 🧹️ Clears every selection bag except grip ids.
fn puzzle5d_clear_non_grip_selection(selection: &mut Puzzle5dSelection) {
    selection.part_ids.clear();
    selection.fastener_ids.clear();
}

/// 🧭️ Whether the engagement HUD should mark an active session for the given utility.
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

/// 🧾️ Materializes the transient scene from the persisted projection (bare document json) and the
/// app's current view state; an unparseable projection degrades to an empty document.
fn scene_from_projection(projection: &Value, runtime: Puzzle5dRuntime, active_utility: &str) -> Puzzle5dScene {
    let document = serde_json::from_value::<Puzzle5dDocument>(projection.clone()).unwrap_or_else(|_| empty_document());
    Puzzle5dScene { document, runtime, active_utility: active_utility.to_string() }
}

/// 🧮️ Document ops for a document mutation — normalizes `before` through the same program typed
/// round-trip as `after` so View-kind actions that only touch runtime never trip the
/// "must not emit operations" guard when the live store still holds a `puzzle_5d`-shaped
/// projection from a prior op apply.
fn puzzle5d_operations_from_document_change(before: &Value, after_document: &Puzzle5dDocument) -> Vec<Puzzle5dOperation> {
    let before_normalized = serde_json::to_value(serde_json::from_value::<Puzzle5dDocument>(before.clone()).unwrap_or_else(|_| empty_document())).unwrap_or_else(|_| before.clone());
    let after = serde_json::to_value(after_document).unwrap_or_else(|_| before_normalized.clone());
    puzzle5d_document_delta_operations(&before_normalized, &after)
}

/// 🪟️ B1: puzzle5d has exactly two window KINDS (2D and 3D), each single-instance — unlike puzzle3d's
/// split top/perspective panes (two INSTANCES of one kind), puzzle5d's own dispatch never distinguishes
/// a window instance id from its kind id (every action matches the literal kind id via
/// `PUZZLE5D_PLAY_WINDOWS.contains(&window)`), so this needs none of `Puzzle3dConfig`'s self-maintained
/// `window_ids`/`load_window`/`save_window` machinery — each kind's sole instance id is the kind id
/// itself. Kept as a named helper (rather than inlining `vec![kind_id.to_string()]`) purely so
/// `window_engagements`/`window_measures` read the same "one entry per live window instance" shape
/// `DocumentApp`'s doc comment describes, and so a future genuine multi-instance need has one seam to extend.
fn window_instance_ids(kind_id: &str) -> Vec<String> {
    vec![kind_id.to_string()]
}

fn puzzle5d_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: PUZZLE5D_PLAY_CONTROLLER_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }
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

//#region 🔖️CopyPaste
/// 🧩️ The part id a `"part_id:grip_id"` full grip reference belongs to.
fn owning_part_id_local(grip_ref: &str) -> &str {
    grip_ref.split(':').next().unwrap_or(grip_ref)
}

fn rewrite_grip_ref_local(grip_ref: &str, id_map: &HashMap<String, String>) -> String {
    match grip_ref.split_once(':') {
        Some((part_id, grip_id)) => match id_map.get(part_id) {
            Some(fresh_part_id) => format!("{fresh_part_id}:{grip_id}"),
            None => grip_ref.to_string(),
        },
        None => grip_ref.to_string(),
    }
}

/// 🧮️ Closure-selects a copy fragment: expands the part set to include every selected fastener's
/// endpoint parts, then expands the fastener set to include every fastener whose BOTH endpoints are
/// now in the part set — mirrors semio_compose_rs's `copyDesign` closure rule
/// (`semio_compose_rs/dev/algorithm/js/index.ts:483`) and `puzzle_5d_engine::copy_selection`'s typed twin.
fn copy_selection_local(document: &Puzzle5dDocument, part_ids: &[String], fastener_ids: &[String]) -> (Vec<Puzzle5dPart>, Vec<Puzzle5dFastener>) {
    let mut part_set: HashSet<String> = part_ids.iter().cloned().collect();
    for fastener in &document.fasteners {
        if fastener_ids.contains(&fastener.id) {
            part_set.insert(owning_part_id_local(&fastener.source).to_string());
            part_set.insert(owning_part_id_local(&fastener.target).to_string());
        }
    }
    let mut fastener_set: HashSet<String> = fastener_ids.iter().cloned().collect();
    if !part_set.is_empty() {
        for fastener in &document.fasteners {
            let source_part = owning_part_id_local(&fastener.source);
            let target_part = owning_part_id_local(&fastener.target);
            if part_set.contains(source_part) && part_set.contains(target_part) {
                fastener_set.insert(fastener.id.clone());
            }
        }
    }
    let parts = document.parts.iter().filter(|part| part_set.contains(&part.id)).cloned().collect();
    let fasteners = document.fasteners.iter().filter(|fastener| fastener_set.contains(&fastener.id)).cloned().collect();
    (parts, fasteners)
}

fn centroid_2d_local(parts: &[Puzzle5dPart]) -> Option<(f64, f64)> {
    if parts.is_empty() {
        return None;
    }
    let (mut sum_x, mut sum_y) = (0.0, 0.0);
    for part in parts {
        sum_x += part.part_2d.x;
        sum_y += part.part_2d.y;
    }
    let count = parts.len() as f64;
    Some((sum_x / count, sum_y / count))
}

/// 🧮️ Resolves the 2D paste offset from `placement`: `Original` uses the (optional) position
/// override verbatim; every other anchor uses the target-minus-source centroid delta plus the
/// (optional) position override — mirrors semio_compose_rs's `__pasteCoordinateOffset`
/// (`semio_compose_rs/dev/algorithm/js/index.ts:358`; semio_compose_rs itself only differentiates `original` vs
/// every other anchor, all of which resolve to the centroid offset).
fn paste_delta_2d(fragment_parts: &[Puzzle5dPart], target_parts: &[Puzzle5dPart], placement: &PastePlacement) -> (f64, f64) {
    let (offset_x, offset_y) = placement.position.map(|position| (position[0], position[1])).unwrap_or((0.0, 0.0));
    if matches!(placement.anchor, PasteAnchor::Original) {
        return (offset_x, offset_y);
    }
    match (centroid_2d_local(fragment_parts), centroid_2d_local(target_parts)) {
        (Some(source), Some(target)) => (target.0 - source.0 + offset_x, target.1 - source.1 + offset_y),
        _ => (offset_x, offset_y),
    }
}

/// 🧮️ Materializes a copied fragment at 2D delta `delta` (applied verbatim to the 3D origin's x/y
/// too) — fresh ids (via `next_part_id`/`next_fastener_id`) dodge collisions with the live document,
/// and fastener endpoints are remapped onto the fresh part ids. Mirrors semio_compose_rs's `pasteDesign`
/// (`semio_compose_rs/dev/algorithm/js/index.ts:515`).
fn paste_selection_local(fragment_parts: &[Puzzle5dPart], fragment_fasteners: &[Puzzle5dFastener], delta: (f64, f64)) -> (Vec<Puzzle5dPart>, Vec<Puzzle5dFastener>) {
    let mut id_map: HashMap<String, String> = HashMap::new();
    let mut fresh_parts = Vec::with_capacity(fragment_parts.len());
    for part in fragment_parts {
        let fresh_id = next_part_id();
        id_map.insert(part.id.clone(), fresh_id.clone());
        let mut next = part.clone();
        next.id = fresh_id;
        next.part_2d.x += delta.0;
        next.part_2d.y += delta.1;
        next.part_3d.origin[0] += delta.0;
        next.part_3d.origin[1] += delta.1;
        fresh_parts.push(next);
    }
    let mut fresh_fasteners = Vec::with_capacity(fragment_fasteners.len());
    for fastener in fragment_fasteners {
        let mut next = fastener.clone();
        next.id = next_fastener_id();
        next.source = rewrite_grip_ref_local(&fastener.source, &id_map);
        next.target = rewrite_grip_ref_local(&fastener.target, &id_map);
        fresh_fasteners.push(next);
    }
    (fresh_parts, fresh_fasteners)
}
//#endregion 🔖️CopyPaste

/** @emoji 📐️ Resolves one numeric-field edit: an absolute `value` (typed entry) wins when
 * present, otherwise a `delta` (stepper nudge) is added to `current`. `None` when neither parses. */
fn puzzle5d_resolve_number_edit(current: f64, value: Option<&Value>, delta: Option<&Value>) -> Option<f64> {
    if let Some(absolute) = value.and_then(Value::as_f64) {
        return Some(absolute);
    }
    delta.and_then(Value::as_f64).map(|delta| current + delta)
}

/** @emoji 📐️ Parses a nested stepper-group field id as `"<base>.<axis>"` (`x`/`y`/`z`), returning
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
            selection.part_ids.push_unique(id.to_string());
        } else if fastener_ids.contains(id) {
            selection.fastener_ids.push_unique(id.to_string());
        } else if grip_ids.contains(id) {
            selection.grip_ids.push_unique(id.to_string());
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
//#endregion 🔖️Document

//#region 🔖️Board
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

fn board_fixture_value(document: &Puzzle5dDocument, camera2d: &Puzzle5dCamera2d) -> Value {
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
        "camera": board_camera_value(camera2d),
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
        fixture_json: board_fixture_value(&envelope.document, &envelope.runtime.camera2d).to_string(),
        camera_json: board_camera_value(&envelope.runtime.camera2d).to_string(),
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

/// 🎨️ Palette drop: creates a free paired part at the flat drop point, deriving the volume origin from the nearest peer part's offset.
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
    envelope.runtime.selection = Puzzle5dSelection { part_ids: SelectionSet::from_ids(vec![id]), grip_ids: SelectionSet::default(), fastener_ids: SelectionSet::default() };
}
//#endregion 🔖️Board

//#region 🔖️Engine
/// 🧠️ Maps the unified 5d kind bundle to the puzzle 3d engine naming (`objects` with `vortices` templates, `vortices`, `cables`).
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

/// 🔄️ Adopts an engine fixture while preserving flat aspects: existing parts keep `2d`, new parts get a synthesized flat aspect.
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
    let existing_transforms: HashMap<String, (f64, f64, f64, f64, f64, f64)> =
        envelope.document.fasteners.iter().map(|fastener| (fastener.id.clone(), (fastener.gap, fastener.shift, fastener.rise, fastener.rotation, fastener.turn, fastener.tilt))).collect();
    next.document.fasteners = parsed
        .get("attractions")
        .and_then(|value| value.as_array())
        .into_iter()
        .flatten()
        .filter_map(|attraction| {
            let id = attraction.get("id").and_then(|value| value.as_str()).unwrap_or("fastener").to_string();
            let (gap, shift, rise, rotation, turn, tilt) = existing_transforms.get(&id).copied().unwrap_or_default();
            Some(Puzzle5dFastener {
                fastener_kind: existing_kinds.get(&id).cloned().flatten().or_else(|| attraction.get("attractionKind").and_then(|value| value.as_str()).map(str::to_string)),
                source: attraction.get("attracting")?.as_str()?.to_string(),
                target: attraction.get("attracted")?.as_str()?.to_string(),
                id,
                gap,
                shift,
                rise,
                rotation,
                turn,
                tilt,
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
//#endregion 🔖️Engine

//#region 🔖️World
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

/// 🎯️ Base selection JSON augmented with the mesh granularity, transform tool, and gumball fields the world-3d host reads.
fn world_selection_json_ex(envelope: &Puzzle5dScene) -> String {
    let runtime = &envelope.runtime;
    let mut value: Value = serde_json::from_str(&world3d_selection_json(&runtime.selection_method, runtime.selection.part_ids.as_slice(), runtime.hovered_part_id.as_deref())).unwrap_or_else(|_| json!({}));
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
        "hoveredVortexFullId": runtime.selection.grip_ids.first().map(str::to_string),
    })
    .to_string()
}

/// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: `duplicateSelection`/`selectSameKindSelection`/
/// `zoomToSelection` stay top-level verbs; the hide/lock toggles (bespoke rows — their label/icon flip
/// on selection state, so they can't resolve from a single static `ActionDefinition`) fold into a
/// `settings` group; `deleteSelection` (bespoke label carrying the selection-count phrase) stays the
/// trailing destructive row. `organize_context_menu`, run automatically at the `VcsDocumentApp::context_menu`
/// funnel, handles taxonomy ordering/separator placement — this function only needs to emit the rows.
fn puzzle5d_context_menu_items(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels, is_de: bool, registry: &semio_framework_plugin::AppActionRegistry) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
    use semio_framework_plugin::{selection_count_phrase, ContextMenuItemSpec, Menu};
    if envelope.runtime.selection.part_ids.is_empty() {
        return Vec::new();
    }
    let selected: Vec<&Puzzle5dPart> = envelope.document.parts.iter().filter(|part| envelope.runtime.selection.part_ids.contains(&part.id)).collect();
    let all_hidden = !selected.is_empty() && selected.iter().all(|part| part.part_2d.hidden.unwrap_or(false));
    let all_locked = !selected.is_empty() && selected.iter().all(|part| part.part_2d.locked.unwrap_or(false));
    let phrase = selection_count_phrase(is_de, &[(envelope.runtime.selection.part_ids.len(), if is_de { "Teil" } else { "part" }, if is_de { "Teile" } else { "parts" })]);
    let bespoke = |id: &str, label: String, icon: &str, action: &str, args: Option<serde_json::Value>, destructive: bool| ContextMenuItemSpec {
        id: id.into(),
        label: Some(label),
        icon: Some(icon.into()),
        action: Some(action.into()),
        args: semio_framework_plugin::optional_json_to_dsl(args),
        destructive: destructive.then_some(true),
        ..Default::default()
    };
    Menu::of(registry)
        .action("duplicateSelection")
        .action("selectSameKindSelection")
        .action("zoomToSelection")
        .group("settings", |m| {
            m.item(bespoke(
                "hide-show",
                if all_hidden { labels.show.into() } else { labels.hide.into() },
                if all_hidden { "eye" } else { "eye-off" },
                "setSelectionFlag",
                Some(json!({ "flag": "hidden", "value": !all_hidden })),
                false,
            ))
            .item(bespoke(
                "lock-unlock",
                if all_locked { labels.unlock.into() } else { labels.lock.into() },
                if all_locked { "lock-open" } else { "lock" },
                "setSelectionFlag",
                Some(json!({ "flag": "locked", "value": !all_locked })),
                false,
            ))
        })
        .item(bespoke("delete", format!("{} ({phrase})", labels.delete.as_str()), "trash", "deleteSelection", None, true))
        .build()
}


fn camera3d_json(camera: &Puzzle5dCamera3d) -> String {
    json!({ "position": camera.position, "target": camera.target, "zoom": camera.zoom, "fov": 45.0 }).to_string()
}
//#endregion 🔖️World

//#region 🔖️Brush
fn puzzle5d_brush_target_grip(envelope: &Puzzle5dScene) -> Option<String> {
    envelope.runtime.selection.grip_ids.first().map(str::to_string).or_else(|| {
        let part_id = envelope.runtime.hovered_part_id.as_deref().or_else(|| envelope.runtime.selection.part_ids.first())?;
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
//#endregion 🔖️Brush

//#region 🔖️Engagement
/// 🧰️ The select/brush/fill switcher lives in the framework utility bar (declared via `.utility` +
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
        status: Some(vec![WindowEngagementStatus { id: format!("puzzle5d-status-{window}"), text: format!("{part_count} {} · {fastener_count} {} · {} {active_utility}", labels.parts.as_str(), labels.fasteners.as_str(), labels.utility.as_str()) }]),
        options: None,
        possible_engagements: None,
    }
}
//#endregion 🔖️Engagement

//#region 🔖️Measures
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
            label: format!("{} ({:.0}%)", labels.part_weights.as_str(), puzzle5d_kind_weight_sum(&envelope.runtime.object_kind_weights, &part_ids) * 100.0).into(),
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
            label: format!("{} ({:.0}%)", labels.grip_weights.as_str(), puzzle5d_kind_weight_sum(&envelope.runtime.vortex_kind_weights, &grip_ids) * 100.0).into(),
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

/// 🪣️ Fill-count slider measure — the fill-utility's core parameter, mirrors the retired
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

/// 🪣️ Utility Options group for the Fill utility — the fill-count slider, tagged `Some("fill")` so
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
//#endregion 🔖️Measures

//#region 🔖️Panels
/// 📊️ `label` is always genuine runtime document content here (a part/grip/fastener/catalog name),
/// never `app_labels!` chrome text — wrapped via `Label::data` accordingly.
fn tree_item_with_action(id: impl Into<String>, label: impl Into<String>, icon_id: Option<&str>, action: ActionDescriptor) -> UiTreeItemNode {
    let mut item = UiTreeItemNode::base(id, Label::data(label));
    item.icon_id = icon_id.map(IconName::from);
    item.action = Some(action);
    item
}

/// 📊️ See `tree_item_with_action`'s doc comment — same `Label::data` rationale.
fn tree_info_item(id: impl Into<String>, label: impl Into<String>, description: Option<String>) -> UiTreeItemNode {
    let mut item = UiTreeItemNode::base(id, Label::data(label));
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
        selected_ids: None,
        highlighted_ids: None,
        selection_change: Some(puzzle5d_action("setSelection", None)),
        drop_action: None,
        menu: None,
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

fn kind_catalog_section(section_id: &str, label: LabelText, entries: &[Value], add_action: Option<&str>, none_label: LabelText) -> UiTreeSectionNode {
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
        selected_ids: None,
        highlighted_ids: None,
        selection_change: None,
        drop_action: None,
        menu: None,
    })
}

fn inspector_text_field(id: &str, label: LabelText, value: String, action: ActionDescriptor) -> UiNode {
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
            menu: None,
        })),
        presence: UiPresence::default(),
        menu: None,
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

/// 🔧️ Editable fastener inspector: the six pose-solver offsets (gap/shift/rise/rotation/turn/tilt) as
/// steppers bound to `patchFastener`, plus a "Mixed" summary when more than one fastener is selected
/// (steppers edit the first selected fastener only; a real multi-edit broadcast is a follow-up).
fn build_fastener_inspector(fastener: &Puzzle5dFastener, selected_count: usize, labels: &Puzzle5dLabels) -> UiNode {
    let patch_cmd = |field: &str| puzzle5d_action("patchFastener", Some(json!({ "fastenerId": fastener.id, "field": field })));
    let mut fields = vec![
        ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.id", labels.id, &fastener.id),
        ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.source", labels.source, &fastener.source),
        ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.target", labels.target, &fastener.target),
        inspector_text_field("puzzle5d-play-inspector.fastener.kind", labels.kind, fastener.fastener_kind.clone().unwrap_or_default(), patch_cmd("fastenerKind")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.gap", labels.gap, &[fastener.gap], 0.05, patch_cmd("gap")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.shift", labels.shift, &[fastener.shift], 0.05, patch_cmd("shift")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.rise", labels.rise, &[fastener.rise], 0.05, patch_cmd("rise")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.rotation", labels.rotation, &[fastener.rotation], 1.0, patch_cmd("rotation")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.turn", labels.turn, &[fastener.turn], 1.0, patch_cmd("turn")),
        ui_inspector_stepper_field("puzzle5d-play-inspector.fastener.tilt", labels.tilt, &[fastener.tilt], 1.0, patch_cmd("tilt")),
    ];
    if selected_count > 1 {
        fields.push(ui_inspector_readonly_field("puzzle5d-play-inspector.fastener.mixed", labels.mixed, format!("{selected_count}")));
    }
    ui_stack_vertical(fields)
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
            return build_fastener_inspector(fastener, envelope.runtime.selection.fastener_ids.len(), labels);
        }
    }
    ui_declarative_sections_to_tree(&[semio_framework_plugin::UiSectionNode {
        id: "puzzle5d-play-inspector.empty".into(),
        label: Some(Label::data(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)),
        default_open: Some(true),
        children: vec![
            ui_text(Label::data(format!("{}: {}", labels.schema.as_str(), envelope.document.schema))),
            ui_text(Label::data(format!("{}: {}", labels.parts.as_str(), envelope.document.parts.len()))),
            ui_text(Label::data(format!("{}: {}", labels.fasteners.as_str(), envelope.document.fasteners.len()))),
            ui_text(Label::data(format!("{}: {}", labels.utility.as_str(), envelope.active_utility))),
        ],
        presence: UiPresence::default(),
        menu: None,
    }])
}
//#endregion 🔖️Panels

//#region 🔖️Puzzle5dPlayApp
/// 🧩️ B1: Puzzle-5d play app. Owns the precompute engine and the registered-mesh cache — both
/// per-call scratch, never VCS-tracked; the persisted document (bare `Puzzle5dDocument` json) lives in
/// the wrapping `VcsDocumentApp`'s document store, and the former ambient `RefCell<Puzzle5dRuntime>`
/// ephemeral view state now lives in the wrapping store's real, VCS-tracked `Puzzle5dConfig` artifact
/// (see `//#region 🔖️Config`) — every read comes from `cfg.projection`, every write flows out as a
/// `Puzzle5dConfigOperation` in the returned `Emit`. Each action mutates a transient
/// {@link Puzzle5dScene}, then emits the granular operation delta. Undo/redo/checkpoints are handled
/// by the wrapper.
pub struct Puzzle5dPlayApp {
    precompute: RefCell<Puzzle5dPrecomputeSession>,
    registered_mesh_urls: RefCell<HashSet<String>>,
}

impl Default for Puzzle5dPlayApp {
    fn default() -> Self {
        Self { precompute: RefCell::new(Puzzle5dPrecomputeSession::new()), registered_mesh_urls: RefCell::new(HashSet::new()) }
    }
}

impl Puzzle5dPlayApp {
    fn drive_precompute(&self, envelope: &Puzzle5dScene) {
        let _ = self.precompute.borrow_mut().set_scene(&scene_config_json(envelope));
        // 🧊️ Guarded by `has_mesh` (mirrors the puzzle3d path): `register_mesh` now invalidates the
        // precompute cache, so re-registering the same fallback body on every drive would wipe
        // suggestion/fill progress every call and defeat `set_scene`'s idempotence above.
        if !self.precompute.borrow_mut().has_mesh(PUZZLE5D_FALLBACK_MESH_KIND) {
            let fallback = semio_framework_plugin::mesh_from_kind(PUZZLE5D_FALLBACK_MESH_KIND);
            self.precompute.borrow_mut().register_mesh(PUZZLE5D_FALLBACK_MESH_KIND, &fallback.positions, &fallback.indices);
        }
        for url in collect_mesh_urls(&envelope.document) {
            if !self.registered_mesh_urls.borrow_mut().contains(&url) && !self.precompute.borrow_mut().has_mesh(&url) {
                let fallback = semio_framework_plugin::mesh_from_kind(PUZZLE5D_FALLBACK_MESH_KIND);
                self.precompute.borrow_mut().register_mesh(&url, &fallback.positions, &fallback.indices);
            }
        }
        let _ = self.precompute.borrow_mut().precompute_step(8);
    }

    fn apply_engine_brush_placement(&self, envelope: &Puzzle5dScene, payload: &Value) -> Option<Puzzle5dScene> {
        let brush_payload = serde_json::from_value::<BrushPlacePayload>(payload.clone()).ok()?;
        let fixture_json = self.precompute.borrow_mut().apply_brush_placement_rust(&serde_json::to_string(&brush_payload).ok()?).ok()?;
        merge_engine_fixture(envelope, &fixture_json)
    }

    /// 🖌️ Paired placement for a board `brushPlace` event: the engine picks the volume pose for the flat payload's kind, both aspects land in one part.
    fn apply_board_brush_place(&self, envelope: &mut Puzzle5dScene, payload: &Value) {
        self.drive_precompute(envelope);
        let node_kind = payload.get("nodeKind").and_then(|value| value.as_str()).unwrap_or("Part").to_string();
        let source_grip = payload.get("sourceHandleId").and_then(|value| value.as_str()).map(str::to_string).or_else(|| puzzle5d_brush_target_grip(envelope));
        if let Some(source_grip) = source_grip.as_ref() {
            let candidates = parse_brush_candidates_free(&self.precompute.borrow().brush_candidates(source_grip));
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
                    next.runtime.selection = Puzzle5dSelection { part_ids: SelectionSet::from_ids(vec![new_id]), grip_ids: SelectionSet::default(), fastener_ids: SelectionSet::default() };
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
                envelope.document.fasteners.push(Puzzle5dFastener { id: payload.get("edgeId").and_then(|value| value.as_str()).map(str::to_string).unwrap_or_else(next_fastener_id), source, target, fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 });
            }
        }
        envelope.runtime.selection = Puzzle5dSelection { part_ids: SelectionSet::from_ids(vec![id]), grip_ids: SelectionSet::default(), fastener_ids: SelectionSet::default() };
    }

    fn apply_board_events_from_json(&self, events_json: &str, envelope: &mut Puzzle5dScene) {
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
                        envelope.runtime.camera2d = camera;
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
                            gap: 0.0,
                            shift: 0.0,
                            rise: 0.0,
                            rotation: 0.0,
                            turn: 0.0,
                            tilt: 0.0,
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
//#endregion 🔖️Puzzle5dPlayApp

//#region 🔖️Puzzle5dCommand
/// @emoji 🎯️ B1: `Puzzle5dPlayApp::Command` — the SOLE dispatch surface, one variant per declared
/// action (mirrors every `.operation(...)`/`.view_action(...)` id `create_puzzle5d_app` registers,
/// plus the framework-injected `SET_ACTIVE_UTILITY_ACTION_ID`). Mirrors `puzzle_3d_ui::Puzzle3dCommand`
/// exactly: each variant carries `window_id` (was host-pushed `view_state.window_id`) plus `args` (the
/// action's original `{...}` JSON payload, unchanged) — `handle` reconstructs the exact
/// `(action, args, window_id)` triple `handle_action_impl` (the preserved pre-B1 business logic, see
/// its doc comment) already expects, so every arm's internal `args.get("field")` extraction stays
/// byte-for-byte identical to the pre-B1 implementation. `OpBinary` is a plain JSON-bytes bridge (not
/// `#[derive(dsl::DslOps)]`) — same established "local JSON bridge" idiom `Puzzle5dPlayProjection`
/// (`puzzle_5d_op`) already uses; no DSL text form is required by `DocumentApp::Command` (only
/// `OpBinary`), and a generic `args: Value` field is not representable in the DSL grammar the
/// `#[derive(dsl::DslOps)]` macro targets.
macro_rules! puzzle5d_command_variants {
    ($($Variant:ident = $id:tt),* $(,)?) => {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Puzzle5dCommand {
            $($Variant { window_id: Option<String>, args: Option<Value> }),*
        }

        impl Puzzle5dCommand {
            /// 🏷️ The action id this variant was declared under — used both for `command_id()`
            /// (command-log labeling / registry kind-discipline) and to reconstruct the exact
            /// `action: &str` `handle_action_impl` dispatches on.
            fn action_id(&self) -> &'static str {
                match self {
                    $(Puzzle5dCommand::$Variant { .. } => $id),*
                }
            }

            fn window_id(&self) -> Option<&str> {
                match self {
                    $(Puzzle5dCommand::$Variant { window_id, .. } => window_id.as_deref()),*
                }
            }

            fn args(&self) -> Option<&Value> {
                match self {
                    $(Puzzle5dCommand::$Variant { args, .. } => args.as_ref()),*
                }
            }

            /// 🧪️ Test-only reverse of `action_id()` — builds the variant for a given action id, for
            /// the existing test module's `dispatch_action(...)` helper (see `//#region 🧪️Tests`).
            /// Panics on an unknown action id (a test bug, not a runtime path).
            #[cfg(test)]
            fn from_action(action: &str, args: Option<Value>, window_id: Option<String>) -> Self {
                match action {
                    $($id => Puzzle5dCommand::$Variant { window_id, args }),*,
                    other => panic!("unknown puzzle5d action id in test: {other}"),
                }
            }
        }
    };
}

puzzle5d_command_variants! {
    SetFixtureJson = "setFixtureJson",
    SetActiveExample = "setActiveExample",
    AddNode = "addNode",
    AddPartKind = "addPartKind",
    AddBrushPart = "addBrushPart",
    AddBrushObject = "addBrushObject",
    DeleteSelection = "deleteSelection",
    DuplicateSelection = "duplicateSelection",
    SetSelectionFlag = "setSelectionFlag",
    ZoomToSelection = "zoomToSelection",
    FocusSelection = "focusSelection",
    EngagementSubmit = "engagementSubmit",
    SetFillCount = "setFillCount",
    PatchPart = "patchPart",
    PatchGrip = "patchGrip",
    PatchFastener = "patchFastener",
    ImportComposeKit = "importComposeKit",
    TranslateSelection = "translateSelection",
    RotateSelection = "rotateSelection",
    ScaleSelection = "scaleSelection",
    WorldRelocate = "worldRelocate",
    ApplyBoardEvents = "applyBoardEvents",
    SetCamera = "setCamera",
    SetCamera2d = "setCamera2d",
    SetCamera3d = "setCamera3d",
    SetSelection = "setSelection",
    DocumentSelect = "documentSelect",
    ClearSelection = "clearSelection",
    SelectAll = "selectAll",
    SelectSameKindSelection = "selectSameKindSelection",
    SelectSameKind = "selectSameKind",
    ToggleSun = "toggleSun",
    SetSunAzimuth = "setSunAzimuth",
    SetSunElevation = "setSunElevation",
    SetSunIntensity = "setSunIntensity",
    EngagementInput = "engagementInput",
    EngagementAbort = "engagementAbort",
    EngagementControlSelect = "engagementControlSelect",
    CycleBrushCandidate = "cycleBrushCandidate",
    RegisterBrushMesh = "registerBrushMesh",
    SetBrushPlacementOverlapBudget = "setBrushPlacementOverlapBudget",
    SetObjectKindWeight = "setObjectKindWeight",
    SetVortexKindWeight = "setVortexKindWeight",
    WorldSelect = "worldSelect",
    WorldPick = "worldPick",
    WorldHover = "worldHover",
    SetHover = "setHover",
    WorldVortexHover = "worldVortexHover",
    WorldVortexSelect = "worldVortexSelect",
    SetSelectionMethod = "setSelectionMethod",
    SetLodMode = "setLodMode",
    SetSuggestionOffset = "setSuggestionOffset",
    SetGridSnapEnabled = "setGridSnapEnabled",
    SetGridFactor = "setGridFactor",
    WorldPointerDown = "worldPointerDown",
    CanvasPointerDown = "canvasPointerDown",
    SetActiveUtility = SET_ACTIVE_UTILITY_ACTION_ID,
}

impl protocol::OpBinary for Puzzle5dCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        serde_json::to_vec(self).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        serde_json::from_slice(bytes).map_err(|error| protocol::ProtocolError::Pack(store::PackError::Schema(error.to_string())))
    }
}
//#endregion 🔖️Puzzle5dCommand

//#region 🔖️Puzzle5dPlayAppTrait
impl DocumentApp for Puzzle5dPlayApp {
    type Projection = Puzzle5dPlayProjection;
    type Operation = Puzzle5dOperation;
    type Config = Puzzle5dConfig;
    type ConfigOperation = Puzzle5dConfigOperation;
    type Command = Puzzle5dCommand;

    fn app_id(&self) -> &str {
        PUZZLE5D_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        PUZZLE5D_SCHEMA
    }

    fn initial_projection(&self) -> Puzzle5dPlayProjection {
        Puzzle5dPlayProjection(serde_json::to_value(default_document()).unwrap_or(Value::Null))
    }

    fn clipboard_media_type(&self) -> Option<MediaType> {
        Some(MediaType { class: MediaClass::Kit, form: MediaForm::Design })
    }

    fn copy_fragment(&self, doc: &DocumentView<'_, Puzzle5dPlayProjection>, cfg: &ConfigView<'_, Puzzle5dConfig>) -> Result<ClipboardFragment, ClipboardError> {
        let document: Puzzle5dDocument = serde_json::from_value(doc.projection.0.clone()).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let selection = &cfg.projection.selection;
        let (parts, fasteners) = copy_selection_local(&document, selection.part_ids.as_slice(), selection.fastener_ids.as_slice());
        if parts.is_empty() {
            return Err(ClipboardError::EmptySelection);
        }
        let fragment_value = json!({ "schema": PUZZLE5D_SCHEMA, "parts": parts, "fasteners": fasteners });
        Ok(ClipboardFragment {
            schema: PUZZLE5D_SCHEMA.to_string(),
            media_type: self.clipboard_media_type().expect("declared above"),
            dsl_text: serde_json::to_string_pretty(&fragment_value).unwrap_or_default(),
            pack_bytes: None,
            source_app: PUZZLE5D_PLAY_APP_ID.to_string(),
            label: format!("{} part(s)", parts.len()),
        })
    }

    /// @emoji ✂️ B1: `DocumentApp::cut_operations`'s signature carries no config output channel (it
    /// returns a bare `Vec<Self::Operation>`, not an `Emit`), so — unlike the pre-B1 version, which
    /// also reset `self.runtime.borrow_mut().selection` after cutting — this can only emit the document
    /// removal; clearing the selection is left to the framework's own post-cut selection reconciliation
    /// (the cut parts/fasteners are gone from the document either way, so a stale selection referencing
    /// them is inert until the next real selection action overwrites it).
    fn cut_operations(&self, doc: &DocumentView<'_, Puzzle5dPlayProjection>, cfg: &ConfigView<'_, Puzzle5dConfig>) -> Vec<Puzzle5dOperation> {
        let before = doc.projection.0.clone();
        let Ok(document) = serde_json::from_value::<Puzzle5dDocument>(before.clone()) else {
            return Vec::new();
        };
        let selection = &cfg.projection.selection;
        let (parts, fasteners) = copy_selection_local(&document, selection.part_ids.as_slice(), selection.fastener_ids.as_slice());
        if parts.is_empty() {
            return Vec::new();
        }
        let remove_part_ids: HashSet<&str> = parts.iter().map(|part| part.id.as_str()).collect();
        let remove_fastener_ids: HashSet<&str> = fasteners.iter().map(|fastener| fastener.id.as_str()).collect();
        let mut after = document;
        after.parts.retain(|part| !remove_part_ids.contains(part.id.as_str()));
        after.fasteners.retain(|fastener| !remove_fastener_ids.contains(fastener.id.as_str()));
        puzzle5d_operations_from_document_change(&before, &after)
    }

    /// @emoji 📋️ B1: `DocumentApp::paste_operations` carries no `ConfigView` at all (only `doc`/
    /// `fragment`/`placement`), so — unlike the pre-B1 version, which also selected the freshly pasted
    /// parts via `self.runtime.borrow_mut().selection` — the new selection can't be threaded through
    /// this call; a following `setSelection` command (which the host already issues after a paste in
    /// practice) is what actually selects the pasted parts now.
    fn paste_operations(&self, doc: &DocumentView<'_, Puzzle5dPlayProjection>, fragment: &ClipboardFragment, placement: &PastePlacement) -> Result<Vec<Puzzle5dOperation>, ClipboardError> {
        let expected = self.clipboard_media_type().unwrap_or(MediaType { class: MediaClass::Kit, form: MediaForm::Design });
        if fragment.media_type != expected {
            return Err(ClipboardError::IncompatibleMediaType(fragment.media_type));
        }
        let fragment_value: Value = serde_json::from_str(&fragment.dsl_text).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let fragment_parts: Vec<Puzzle5dPart> = serde_json::from_value(fragment_value.get("parts").cloned().unwrap_or_else(|| json!([]))).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let fragment_fasteners: Vec<Puzzle5dFastener> = serde_json::from_value(fragment_value.get("fasteners").cloned().unwrap_or_else(|| json!([]))).unwrap_or_default();
        let before = doc.projection.0.clone();
        let document: Puzzle5dDocument = serde_json::from_value(before.clone()).map_err(|error| ClipboardError::ParseFailed(error.to_string()))?;
        let delta = paste_delta_2d(&fragment_parts, &document.parts, placement);
        let (fresh_parts, fresh_fasteners) = paste_selection_local(&fragment_parts, &fragment_fasteners, delta);
        let mut after = document;
        after.parts.extend(fresh_parts);
        after.fasteners.extend(fresh_fasteners);
        Ok(puzzle5d_operations_from_document_change(&before, &after))
    }

    /// 🏷️ Maps each `Puzzle5dCommand` variant back to the action id it was declared under.
    fn command_id(&self, command: &Puzzle5dCommand) -> &str {
        command.action_id()
    }

    /// @emoji 🧩️ B1: thin typed-command adapter — reconstructs the exact `(action, args, window_id)`
    /// triple the preserved pre-B1 `handle_action_impl` (see its doc comment, in the `impl
    /// Puzzle5dPlayApp` block right below) already expects, from the typed `Puzzle5dCommand`.
    fn handle(&self, command: &Puzzle5dCommand, doc: &DocumentView<'_, Puzzle5dPlayProjection>, cfg: &ConfigView<'_, Puzzle5dConfig>) -> Emit<Puzzle5dOperation, Puzzle5dConfigOperation> {
        self.handle_action_impl(command.action_id(), command.args(), command.window_id(), doc, cfg.projection)
    }

    /// 🔌️ Declares puzzle5d's typed media I/O surface: the implicit document ports (from
    /// `.document([...])`/`.artifact_kind(...)` in `create_puzzle5d_app`) plus `kit:in` (accepting a
    /// `kit.catalog` fragment shaped like block3d's `puzzle3d_catalog_fragment`, fanning IN from
    /// potentially many producers) and `design:out` (this app's own `5d.puzzle` design artifact, fanning
    /// OUT to potentially many consumers).
    fn io(&self) -> Option<AppIo> {
        Some(
            AppIo::from_document(
                "puzzle.5d",
                MediaType { class: MediaClass::Kit, form: MediaForm::Design },
                ArtifactPresentation { id: "5d.puzzle".into(), name: "5D Puzzle".into(), dimension: "5d".into(), component_kind: "puzzle5d".into() },
            )
            .with_ports(vec![
                MediaPortSpec {
                    id: "kit:in".into(),
                    label: "Kit Catalog".into(),
                    direction: MediaPortDirection::In,
                    media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
                    kind_id: Some("kit.catalog".into()),
                    required: false,
                    multiplicity: PortMultiplicity::Many,
                },
                MediaPortSpec {
                    id: "design:out".into(),
                    label: "5D Puzzle Design".into(),
                    direction: MediaPortDirection::Out,
                    // 🔁️ Reuses the exact `id`/`media_type` already declared on `create_puzzle5d_app`'s
                    // `ArtifactKindSpec { id: "5d.puzzle", .. }` — the same design artifact this app's
                    // document already publishes, just exposed as an explicit workflow output port.
                    media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Design },
                    kind_id: Some("5d.puzzle".into()),
                    required: false,
                    multiplicity: PortMultiplicity::Many,
                },
            ]),
        )
    }

    /// 🎞️ `kit:in` seam: normalizes an incoming `kit.catalog` fragment (block3d's
    /// `puzzle3d_catalog_fragment` shape — `objectKinds`/`vortexKinds`/`cableKinds`/`attractionKinds`/
    /// `kindCompatibility`) into puzzle5d's own typed `kindCatalogs`/`kindCompatibility` vocabulary and
    /// upserts it (keyed by row `id`, deterministic/order-independent — safe for `multiplicity: Many`
    /// fan-in from several producers), then bridges the before/after document through
    /// `puzzle5d_operations_from_document_change` exactly like every other document-mutating action —
    /// this never mutates anything directly, only real, undoable operations. See
    /// `puzzle5d_upsert_catalog_parts`/`puzzle5d_upsert_catalog_grips`/`puzzle5d_upsert_kind_compatibility`.
    fn import_media(&self, port: &str, media: &Media, doc: &DocumentView<'_, Puzzle5dPlayProjection>) -> Result<Emit<Puzzle5dOperation, Puzzle5dConfigOperation>, MediaError> {
        if port != "kit:in" {
            return Err(MediaError::NotImplemented);
        }
        let semio_framework_plugin::MediaPayload::Structured { json, .. } = &media.payload else {
            return Err(MediaError::Payload(port.to_string(), "kit:in only accepts a Structured (JSON) payload".into()));
        };
        let fragment: Value = serde_json::from_str(json.as_str()).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
        let mut document: Puzzle5dDocument = serde_json::from_value(doc.projection.0.clone()).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;

        let mut catalogs: puzzle_5d::Puzzle5dKindCatalogs = document.kind_catalogs.as_ref().and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();

        if let Some(incoming_parts) = fragment.get("objectKinds").and_then(Value::as_array) {
            let parsed: Vec<puzzle_5d::Puzzle5dCatalogPart> = incoming_parts
                .iter()
                .filter_map(|row| {
                    let parsed_row: Puzzle5dKitInObjectKindFragment = serde_json::from_value(row.clone()).ok()?;
                    Some(puzzle_5d::Puzzle5dCatalogPart {
                        id: parsed_row.id,
                        name: parsed_row.name,
                        label: parsed_row.label,
                        mesh_url: parsed_row.mesh_url,
                        grips: parsed_row
                            .vortices
                            .into_iter()
                            .map(|vortex| puzzle_5d::Puzzle5dCatalogGripTemplate {
                                grip_kind: vortex.vortex_kind,
                                grip_2d: None,
                                grip_3d: Some(puzzle_5d::Puzzle5dCatalogGripTemplate3d { position: vortex.position, direction: vortex.direction, radius: vortex.radius }),
                            })
                            .collect(),
                    })
                })
                .collect();
            puzzle5d_upsert_catalog_parts(&mut catalogs.parts, parsed);
        }
        if let Some(incoming_grips) = fragment.get("vortexKinds").and_then(Value::as_array) {
            let parsed: Vec<puzzle_5d::Puzzle5dCatalogGrip> = incoming_grips
                .iter()
                .filter_map(|row| {
                    let parsed_row: Puzzle5dKitInVortexKindFragment = serde_json::from_value(row.clone()).ok()?;
                    Some(puzzle_5d::Puzzle5dCatalogGrip { id: parsed_row.id, name: parsed_row.name, label: parsed_row.label, color: parsed_row.color, default_rope_kind: parsed_row.default_cable_kind })
                })
                .collect();
            puzzle5d_upsert_catalog_grips(&mut catalogs.grips, parsed);
        }
        // 🚫️ `cableKinds`/`attractionKinds` are deliberately left unmapped: puzzle5d's `Puzzle5dKindCatalogs`
        // has no genuine 1:1 counterpart for either in THIS fragment shape — `ropes`/`fasteners` exist as
        // catalog sections, but `cableKinds`/`attractionKinds` describe 3D cable-tension/attraction-constraint
        // kinds, a different domain concept than puzzle5d's fastener/rope kinds. Forcing a mapping here would
        // be a fabricated guess, not a verified equivalence — left for a follow-up ticket with real fixtures.
        document.kind_catalogs = Some(serde_json::to_value(&catalogs).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?);

        if let Some(incoming_compat) = fragment.get("kindCompatibility").and_then(Value::as_array) {
            let mut compatibility: Vec<puzzle_5d::Puzzle5dKindCompatibility> = document.kind_compatibility.as_ref().and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
            let parsed: Vec<puzzle_5d::Puzzle5dKindCompatibility> = incoming_compat.iter().filter_map(|row| serde_json::from_value(row.clone()).ok()).collect();
            puzzle5d_upsert_kind_compatibility(&mut compatibility, parsed);
            document.kind_compatibility = Some(serde_json::to_value(&compatibility).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?);
        }

        let operations = puzzle5d_operations_from_document_change(&doc.projection.0, &document);
        Ok(Emit::operations(operations))
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, Puzzle5dPlayProjection>, cfg: &ConfigView<'_, Puzzle5dConfig>) -> UiNode {
        let config = cfg.projection;
        let window_for_body = match body_key {
            PUZZLE5D_PLAY_BODY_2D => PUZZLE5D_PLAY_WINDOW_2D,
            _ => PUZZLE5D_PLAY_WINDOW_3D,
        };
        let active_utility = puzzle5d_scene_active_utility(config, Some(window_for_body));
        let envelope = scene_from_projection(&doc.projection.0, config.clone(), &active_utility);
        let labels = puzzle5d_labels(config);
        match body_key {
            PUZZLE5D_PLAY_BODY_2D => build_board2d_scene(PUZZLE5D_PLAY_SURFACE_2D, PUZZLE5D_PLAY_CONTROLLER_ID, puzzle5d_board_scene(&envelope)),
            PUZZLE5D_PLAY_BODY_3D => {
                let brush_preview = world_brush_preview_json(&self.precompute.borrow(), &envelope);
                build_world_3d_scene(
                    PUZZLE5D_PLAY_SURFACE_3D,
                    PUZZLE5D_PLAY_CONTROLLER_ID,
                    world3d_scene_extended(
                        camera3d_json(&envelope.runtime.camera3d),
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
                        Some(world3d_environment_json(&envelope.runtime.sun)),
                        None,
                        None,
                        None,
                        None,
                        None,
                    ),
                )
            }
            PUZZLE5D_PLAY_BODY_DOCUMENT => build_document_tree(&envelope, labels),
            PUZZLE5D_PLAY_BODY_KINDS => build_kinds_tree(&envelope, labels),
            PUZZLE5D_PLAY_BODY_INSPECTOR => build_inspector_tree(&envelope, labels),
            _ => ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_engagements(&self, doc: &DocumentView<'_, Puzzle5dPlayProjection>, cfg: &ConfigView<'_, Puzzle5dConfig>) -> HashMap<String, WindowEngagement> {
        let config = cfg.projection;
        let labels = puzzle5d_labels(config);
        // 🪟️ One entry per live window INSTANCE of each of the 2D/3D window kinds — see
        // `window_instance_ids`'s doc comment for why puzzle5d needs none of puzzle3d's genuine
        // multi-instance-per-kind machinery here (each kind is always its own sole instance).
        PUZZLE5D_PLAY_WINDOWS
            .iter()
            .flat_map(|window| {
                window_instance_ids(window).into_iter().map(|wid| {
                    let active_utility = puzzle5d_scene_active_utility(config, Some(&wid));
                    let envelope = scene_from_projection(&doc.projection.0, config.clone(), &active_utility);
                    (wid, puzzle5d_engagement(&envelope, window, labels))
                })
            })
            .collect()
    }

    fn window_measures(&self, doc: &DocumentView<'_, Puzzle5dPlayProjection>, cfg: &ConfigView<'_, Puzzle5dConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.projection;
        let labels = puzzle5d_labels(config);
        PUZZLE5D_PLAY_WINDOWS
            .iter()
            .flat_map(|window| {
                window_instance_ids(window).into_iter().map(|wid| {
                    let active_utility = puzzle5d_scene_active_utility(config, Some(&wid));
                    let envelope = scene_from_projection(&doc.projection.0, config.clone(), &active_utility);
                    (wid, puzzle5d_window_measures(window, &envelope, &self.precompute.borrow(), labels))
                })
            })
            .collect()
    }

    fn context_menu(
        &self,
        request: &semio_framework_plugin::ContextMenuRequest,
        doc: &DocumentView<'_, Puzzle5dPlayProjection>,
        cfg: &ConfigView<'_, Puzzle5dConfig>,
        registry: &semio_framework_plugin::AppActionRegistry,
    ) -> Vec<semio_framework_plugin::ContextMenuItemSpec> {
        let config = cfg.projection;
        let labels = puzzle5d_labels(config);
        let is_de = puzzle5d_is_de_locale(config);
        let active_utility = puzzle5d_scene_active_utility(config, Some(PUZZLE5D_PLAY_WINDOW_3D));
        let mut envelope = scene_from_projection(&doc.projection.0, config.clone(), &active_utility);
        if let Some(surface) = request.surface.as_ref() {
            let part_ids: Vec<String> = surface.selection.iter().filter(|g| g.domain == "object" || g.domain == "node" || g.domain == "part").flat_map(|g| g.ids.iter().cloned()).collect();
            if !part_ids.is_empty() {
                envelope.runtime.selection.part_ids = part_ids.into();
            }
        }
        puzzle5d_context_menu_items(&envelope, labels, is_de, registry)
    }
}

/// 🎞️ `kit:in` fragment row shapes (block3d's `puzzle3d_catalog_fragment`, camelCase) — local
/// deserialize-only mirrors of `objectKinds[]`/`objectKinds[].vortices[]`/`vortexKinds[]` entries, kept
/// separate from `puzzle_5d::Puzzle5dCatalogPart`/`Puzzle5dCatalogGripTemplate`/`Puzzle5dCatalogGrip`
/// (whose field names/shape differ) so `import_media` can parse the fragment once and then build the
/// real typed catalog rows explicitly, field by field.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dKitInObjectKindFragment {
    id: String,
    name: String,
    label: String,
    #[serde(default)]
    mesh_url: Option<String>,
    #[serde(default)]
    vortices: Vec<Puzzle5dKitInVortexFragment>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dKitInVortexFragment {
    #[serde(default, rename = "id")]
    #[allow(dead_code)]
    _id: String,
    vortex_kind: String,
    position: [f64; 3],
    direction: [f64; 3],
    radius: f64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Puzzle5dKitInVortexKindFragment {
    id: String,
    name: String,
    label: String,
    color: String,
    #[serde(default)]
    default_cable_kind: String,
}

/// 🔌️ `kit:in` seam helper: keyed UPSERT of catalog PART rows (by `id`) — replaces any existing row
/// with the same id, else appends. Deterministic/order-independent in the resulting SET of ids (a
/// `multiplicity: Many` port may fan in from several producers across several `import_media` calls);
/// when two producers disagree on one id's content, the most-recently-applied wins. Mirrors
/// `puzzle_3d_ui::puzzle3d_upsert_catalog_rows` but against the typed
/// `Vec<puzzle_5d::Puzzle5dCatalogPart>` puzzle5d's own kind catalogs use, instead of a raw `Value` array.
fn puzzle5d_upsert_catalog_parts(existing: &mut Vec<puzzle_5d::Puzzle5dCatalogPart>, incoming: Vec<puzzle_5d::Puzzle5dCatalogPart>) {
    for row in incoming {
        match existing.iter().position(|entry| entry.id == row.id) {
            Some(index) => existing[index] = row,
            None => existing.push(row),
        }
    }
}

/// 🔌️ `kit:in` seam helper: keyed UPSERT of catalog GRIP-KIND rows (by `id`) — see
/// `puzzle5d_upsert_catalog_parts`'s doc for the upsert/idempotency contract.
fn puzzle5d_upsert_catalog_grips(existing: &mut Vec<puzzle_5d::Puzzle5dCatalogGrip>, incoming: Vec<puzzle_5d::Puzzle5dCatalogGrip>) {
    for row in incoming {
        match existing.iter().position(|entry| entry.id == row.id) {
            Some(index) => existing[index] = row,
            None => existing.push(row),
        }
    }
}

/// 🔌️ `kit:in` seam helper: keyed UPSERT of kind-compatibility rows by the `(source, target)` pair —
/// mirrors `puzzle_3d_ui`'s compatibility-upsert loop in `Puzzle3dPlayApp::import_media`.
fn puzzle5d_upsert_kind_compatibility(existing: &mut Vec<puzzle_5d::Puzzle5dKindCompatibility>, incoming: Vec<puzzle_5d::Puzzle5dKindCompatibility>) {
    for row in incoming {
        match existing.iter().position(|entry| entry.source == row.source && entry.target == row.target) {
            Some(index) => existing[index] = row,
            None => existing.push(row),
        }
    }
}

impl Puzzle5dPlayApp {
    /// @emoji 🧩️ B1: the pure per-action core, dispatched into by `DocumentApp::handle` above with
    /// `action`/`args`/`window_id` reconstructed 1:1 from the typed `Puzzle5dCommand` — everything past
    /// this adapter boundary is the ORIGINAL pre-B1 business logic, unchanged, now reading/writing a
    /// passed-in `Puzzle5dConfig` snapshot instead of an ambient `self.runtime` `RefCell` and returning
    /// a real `Emit` (document + config operations) instead of mutating `self` and returning a bare
    /// document-only `ActionEmit`.
    fn handle_action_impl(&self, action: &str, args: Option<&Value>, window_id: Option<&str>, doc: &DocumentView<'_, Puzzle5dPlayProjection>, config: &Puzzle5dConfig) -> Emit<Puzzle5dOperation, Puzzle5dConfigOperation> {
        let before = doc.projection.0.clone();
        let active_utility_initial = puzzle5d_scene_active_utility(config, window_id);
        let wid = window_id.map(str::to_string).unwrap_or_else(|| PUZZLE5D_PLAY_WINDOW_3D.to_string());
        let mut envelope = scene_from_projection(&before, config.clone(), &active_utility_initial);
        match action {
            "setFixtureJson" => {
                if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
                    if let Ok(document) = serde_json::from_str::<Puzzle5dDocument>(json_text) {
                        envelope.document = document;
                    }
                }
            }
            "importComposeKit" => {
                // 🌉️ Merges a semio_compose_rs Design document's pieces/connections into the live document
                // (replacing `parts`/`fasteners`; camera/catalogs untouched) — see
                // `puzzle_5d_engine::import_compose_design_json`'s doc comment for scope (one already-
                // exported design, not a full multi-file kit bundle).
                if let Some(design_json) = args.and_then(|value| value.get("design")) {
                    let imported = import_compose_design_json(design_json);
                    let imported_value = serde_json::to_value(&imported).unwrap_or(Value::Null);
                    if let Some(parts) = imported_value.get("parts").cloned().and_then(|value| serde_json::from_value::<Vec<Puzzle5dPart>>(value).ok()) {
                        envelope.document.parts = parts;
                    }
                    if let Some(fasteners) = imported_value.get("fasteners").cloned().and_then(|value| serde_json::from_value::<Vec<Puzzle5dFastener>>(value).ok()) {
                        envelope.document.fasteners = fasteners;
                    }
                    if let Some(label) = imported.label {
                        envelope.document.label = Some(label);
                    }
                    envelope.runtime.selection = Puzzle5dSelection::default();
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
                    envelope.runtime.selection = Puzzle5dSelection { part_ids: SelectionSet::from_ids(read("partIds").unwrap_or_default()), grip_ids: SelectionSet::from_ids(read("gripIds").unwrap_or_default()), fastener_ids: SelectionSet::from_ids(read("fastenerIds").unwrap_or_default()) };
                }
            }
            "clearSelection" => {
                envelope.runtime.selection = Puzzle5dSelection::default();
            }
            "selectAll" => {
                envelope.runtime.selection = Puzzle5dSelection { part_ids: envelope.document.parts.iter().map(|part| part.id.clone()).collect(), grip_ids: SelectionSet::default(), fastener_ids: SelectionSet::default() };
            }
            "deleteSelection" => {
                let selection = envelope.runtime.selection.clone();
                remove_parts(&mut envelope.document, selection.part_ids.as_slice());
                remove_grips(&mut envelope.document, selection.grip_ids.as_slice());
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
                    return Emit::default();
                }
                let new_ids: Vec<String> = clones.iter().map(|part| part.id.clone()).collect();
                envelope.document.parts.extend(clones);
                envelope.runtime.selection = Puzzle5dSelection { part_ids: SelectionSet::from_ids(new_ids), grip_ids: SelectionSet::default(), fastener_ids: SelectionSet::default() };
            }
            "selectSameKindSelection" | "selectSameKind" => {
                let Some(kind) = envelope.runtime.selection.part_ids.first().and_then(|id| envelope.document.parts.iter().find(|part| &part.id == id)).map(|part| part.part_kind.clone()) else {
                    return Emit::default();
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
                    return Emit::default();
                };
                let camera = &mut envelope.runtime.camera3d;
                let offset = [camera.position[0] - camera.target[0], camera.position[1] - camera.target[1], camera.position[2] - camera.target[2]];
                camera.target = target;
                camera.position = [target[0] + offset[0], target[1] + offset[1], target[2] + offset[2]];
                let selected_2d: Vec<(f64, f64)> = envelope.document.parts.iter().filter(|part| envelope.runtime.selection.part_ids.contains(&part.id)).map(|part| (part.part_2d.x, part.part_2d.y)).collect();
                if !selected_2d.is_empty() {
                    envelope.runtime.camera2d.x = selected_2d.iter().map(|(x, _)| x).sum::<f64>() / selected_2d.len() as f64;
                    envelope.runtime.camera2d.y = selected_2d.iter().map(|(_, y)| y).sum::<f64>() / selected_2d.len() as f64;
                }
            }
            SET_ACTIVE_UTILITY_ACTION_ID => {
                // 🧰️ B1: this Command IS the utility switch now (was host-applied ambient
                // `view_state.active_utility_id`/`active_utility_by_window_id` — the host no longer owns
                // that state, `Puzzle5dConfig` does), so this arm must itself write the new value before
                // clearing in-progress engagement scratch and refreshing the placement engine.
                if let Some(utility_id) = args.and_then(|value| value.get("utilityId")).and_then(|value| value.as_str()) {
                    envelope.runtime.active_utility_by_window_id.insert(wid.clone(), utility_id.to_string());
                    envelope.active_utility = utility_id.to_string();
                }
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
                    if let Ok(fixture_json) = self.precompute.borrow_mut().apply_fill_count_rust(count) {
                        if let Some(next) = merge_engine_fixture(&envelope, &fixture_json) {
                            envelope = next;
                        }
                    }
                }
            }
            "cycleBrushCandidate" => {
                self.drive_precompute(&envelope);
                if let Some(grip_full_id) = puzzle5d_brush_target_grip(&envelope) {
                    let free = parse_brush_candidates_free(&self.precompute.borrow().brush_candidates(&grip_full_id)).len();
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
                    self.precompute.borrow_mut().register_mesh(url, &positions, &indices);
                    self.registered_mesh_urls.borrow_mut().insert(url.to_string());
                }
                return Emit::default();
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
            "patchFastener" => {
                let fastener_id = args.and_then(|value| value.get("fastenerId")).and_then(|value| value.as_str()).unwrap_or("").to_string();
                let field = args.and_then(|value| value.get("field")).and_then(|value| value.as_str()).unwrap_or("");
                let value = args.and_then(|value| value.get("value"));
                let delta = args.and_then(|value| value.get("delta"));
                let text = value.and_then(Value::as_str).map(str::to_string);
                for fastener in &mut envelope.document.fasteners {
                    if fastener.id != fastener_id {
                        continue;
                    }
                    match field {
                        "fastenerKind" => fastener.fastener_kind = text.clone().filter(|text| !text.is_empty()),
                        "gap" => {
                            if let Some(updated) = puzzle5d_resolve_number_edit(fastener.gap, value, delta) {
                                fastener.gap = updated;
                            }
                        }
                        "shift" => {
                            if let Some(updated) = puzzle5d_resolve_number_edit(fastener.shift, value, delta) {
                                fastener.shift = updated;
                            }
                        }
                        "rise" => {
                            if let Some(updated) = puzzle5d_resolve_number_edit(fastener.rise, value, delta) {
                                fastener.rise = updated;
                            }
                        }
                        "rotation" => {
                            if let Some(updated) = puzzle5d_resolve_number_edit(fastener.rotation, value, delta) {
                                fastener.rotation = updated;
                            }
                        }
                        "turn" => {
                            if let Some(updated) = puzzle5d_resolve_number_edit(fastener.turn, value, delta) {
                                fastener.turn = updated;
                            }
                        }
                        "tilt" => {
                            if let Some(updated) = puzzle5d_resolve_number_edit(fastener.tilt, value, delta) {
                                fastener.tilt = updated;
                            }
                        }
                        _ => {}
                    }
                }
            }
            // 📷️ Camera pose is session-only view state (`ActionKind::View`) — writes runtime directly
            // and never touches the document, so it never emits a VCS operation.
            "setCamera" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    let surface_id = args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()).unwrap_or("");
                    if surface_id == PUZZLE5D_PLAY_SURFACE_2D || camera.get("position").is_none() {
                        if let Ok(parsed) = serde_json::from_value::<Puzzle5dCamera2d>(camera.clone()) {
                            envelope.runtime.camera2d = parsed;
                        }
                    } else if let Ok(parsed) = serde_json::from_value::<Puzzle5dCamera3d>(camera.clone()) {
                        envelope.runtime.camera3d = parsed;
                    }
                }
            }
            "setCamera2d" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        envelope.runtime.camera2d = parsed;
                    }
                }
            }
            "setCamera3d" => {
                if let Some(camera) = args.and_then(|value| value.get("camera")) {
                    if let Ok(parsed) = serde_json::from_value(camera.clone()) {
                        envelope.runtime.camera3d = parsed;
                    }
                }
            }
            "translateSelection" => {
                let ids = mesh_selection_ids(args, envelope.runtime.selection.part_ids.as_slice());
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
                let ids = mesh_selection_ids(args, envelope.runtime.selection.part_ids.as_slice());
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
                let ids = mesh_selection_ids(args, envelope.runtime.selection.part_ids.as_slice());
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
                                    merged.push_unique(id);
                                    merged
                                }
                                "toggle" => {
                                    let mut merged = envelope.runtime.selection.part_ids.clone();
                                    if merged.contains(&id) {
                                        merged.remove_id(&id);
                                    } else {
                                        merged.push_unique(id);
                                    }
                                    merged
                                }
                                _ => {
                                    puzzle5d_clear_non_part_selection(&mut envelope.runtime.selection);
                                    SelectionSet::from_ids(vec![id])
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
                envelope.runtime.selection.grip_ids = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()).map(|full_id| SelectionSet::from_ids(vec![full_id.to_string()])).unwrap_or_default();
                if envelope.active_utility == "brush" && !envelope.runtime.selection.grip_ids.is_empty() {
                    self.drive_precompute(&envelope);
                }
            }
            "worldVortexSelect" => {
                if let Some(full_id) = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()) {
                    puzzle5d_clear_non_grip_selection(&mut envelope.runtime.selection);
                    envelope.runtime.selection.grip_ids = SelectionSet::from_ids(vec![full_id.to_string()]);
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
                                    envelope.document.fasteners.push(Puzzle5dFastener { id: next_fastener_id(), source: source_id.clone(), target: target_id, fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0 });
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
            "worldPointerDown" | "canvasPointerDown" => return Emit::default(),
            _ => {}
        }
        let next_active_utility = envelope.active_utility.clone();
        let operations = puzzle5d_operations_from_document_change(&before, &envelope.document);
        // 🌀️ Coalesce each gumball drag tick into one undoable edit (compact per-part records, not full meshes).
        let coalesce_key = match action {
            "translateSelection" => Some("gumball-translate".to_string()),
            "rotateSelection" => Some("gumball-rotate".to_string()),
            "scaleSelection" => Some("gumball-scale".to_string()),
            _ => None,
        };
        // 🧰️ B1: a DIRECT `SetActiveUtility` command already told the host what it needs to know — never
        // re-emit the same switch as a `HostEffect` (the pre-B1 code only had to guard this for the
        // INDIRECT paths below, since the host itself pushed the direct switch before dispatching; now
        // the command IS the direct switch, so this arm must self-exclude). Programmatic utility
        // switches (engagement submit/abort, fill) still push the active utility back into the host
        // session for both windows.
        let is_direct_utility_switch = action == SET_ACTIVE_UTILITY_ACTION_ID;
        let effects = if !is_direct_utility_switch && next_active_utility != active_utility_initial {
            PUZZLE5D_PLAY_WINDOWS.iter().map(|window| HostEffect::SetActiveUtility { window_id: (*window).into(), utility_id: next_active_utility.clone() }).collect()
        } else {
            Vec::new()
        };
        // 🧮️ B1: only a REAL config change becomes a `Puzzle5dConfigOperation` — `PartialEq` (derived)
        // makes this cheap, and keeps a pure read-only action from creating a no-op undo entry.
        let config_operations = if &envelope.runtime != config { vec![Puzzle5dConfigOperation::Snapshot { config: envelope.runtime }] } else { Vec::new() };
        Emit { document_operations: operations, config_operations, coalesce_key, effects, ..Default::default() }
    }
}

//#endregion 🔖️Puzzle5dPlayAppTrait

//#region 🔖️Manifest
pub fn create_puzzle5d_app() -> App {
    let envelope = Puzzle5dScene { document: default_document(), runtime: Puzzle5dRuntime::default(), active_utility: PUZZLE5D_DEFAULT_UTILITY.into() };
    let precompute = Puzzle5dPrecomputeSession::new();
    let manifest_labels = puzzle5d_labels(&Puzzle5dConfig::default());
    let mut app = App::from_builder(
        App::builder(PUZZLE5D_PLAY_APP_ID, LocalizedLabel::native("Puzzle 5D", "Puzzle 5D"))
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
            .mode("edit", LocalizedLabel::native("Edit", "Bearbeiten"), "pencil")
            .default_mode_id("edit")
            .window_kind_with_engagement(PUZZLE5D_PLAY_WINDOW_2D, puzzle5d_localized(|l| l.window_2d), PUZZLE5D_PLAY_BODY_2D, SurfaceKind::Board2d, puzzle5d_engagement(&envelope, PUZZLE5D_PLAY_WINDOW_2D, manifest_labels), "layout-grid")
            .window_kind_with_engagement(PUZZLE5D_PLAY_WINDOW_3D, puzzle5d_localized(|l| l.window_3d), PUZZLE5D_PLAY_BODY_3D, SurfaceKind::World3d, puzzle5d_engagement(&envelope, PUZZLE5D_PLAY_WINDOW_3D, manifest_labels), "puzzle5d-3d")
            // 🏗️ 3D-first 60/40 split — mirrors semio_compose_rs's design app (scene 60% / diagram 40%,
            // `semio_compose_rs/client/lib/sketchpad/js/index.ts:15367-15378`), the assembly-editing use case
            // this app replaces.
            .default_layout(create_default_layout(&[PUZZLE5D_PLAY_WINDOW_3D.into(), PUZZLE5D_PLAY_WINDOW_2D.into()], "row", Some(&[60.0, 40.0]), Some(&["Puzzle 3D".into(), "Puzzle 2D".into()])))
            .panel_tab(FRAMEWORK_PANEL_TAB_DOCUMENT_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"), PanelGroup::Workbench, PUZZLE5D_PLAY_BODY_DOCUMENT)
            .panel_tab(FRAMEWORK_PANEL_TAB_CATALOGUE_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"), PanelGroup::Workbench, PUZZLE5D_PLAY_BODY_KINDS)
            .panel_tab(FRAMEWORK_PANEL_TAB_INSPECTION_ID, LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"), PanelGroup::Details, PUZZLE5D_PLAY_BODY_INSPECTOR)
            // 🔧️ Document-mutating operations (emit VCS operations through the before/after document delta).
            .action_with(ActionDefinition { in_palette: false, ..ActionDefinition::new_catalog("setFixtureJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"), ActionKind::Operation) })
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .operation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .operation("addPartKind", LocalizedLabel::native("Add Part", "Teil hinzufügen"))
            .operation("addBrushPart", LocalizedLabel::native("Add Brush Part", "Pinselteil hinzufügen"))
            .operation("addBrushObject", LocalizedLabel::native("Add Brush Object", "Pinselobjekt hinzufügen"))
            .action_with(ActionDefinition::new_catalog("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"), ActionKind::Operation).with_category("selection"))
            .action_with(ActionDefinition::new_catalog("duplicateSelection", LocalizedLabel::native("Duplicate Selection", "Auswahl duplizieren"), ActionKind::Operation).with_category("create"))
            .action_with(ActionDefinition::new_catalog("setSelectionFlag", LocalizedLabel::native("Set Selection Flag", "Auswahlmarkierung festlegen"), ActionKind::Operation).with_category("settings"))
            .action_with(ActionDefinition::new_catalog("zoomToSelection", LocalizedLabel::native("Zoom To Selection", "Auf Auswahl zoomen"), ActionKind::Operation).with_category("view"))
            .operation("focusSelection", LocalizedLabel::native("Focus Selection", "Auswahl fokussieren"))
            .operation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"))
            .operation("setFillCount", LocalizedLabel::native("Set Fill Count", "Füllanzahl festlegen"))
            .operation("patchPart", LocalizedLabel::native("Patch Part", "Teil aktualisieren"))
            .operation("patchGrip", LocalizedLabel::native("Patch Grip", "Griff aktualisieren"))
            .operation("patchFastener", LocalizedLabel::native("Patch Fastener", "Verbinder aktualisieren"))
            .operation("importComposeKit", LocalizedLabel::native("Import Compose Kit", "Compose-Kit importieren"))
            .operation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .operation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .operation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .operation("worldRelocate", LocalizedLabel::native("Relocate Part", "Teil verlagern"))
            .operation("applyBoardEvents", LocalizedLabel::native("Apply Board Events", "Board-Ereignisse anwenden"))
            // 👁️ Ephemeral view state — selection, hover, utility parameters, brush cycling, camera pose.
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .view_action("setCamera2d", LocalizedLabel::native("Set Camera 2D", "Kamera 2D festlegen"))
            .view_action("setCamera3d", LocalizedLabel::native("Set Camera 3D", "Kamera 3D festlegen"))
            .view_action("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"))
            .view_action("documentSelect", LocalizedLabel::native("Document Select", "Dokument auswählen"))
            .view_action("clearSelection", LocalizedLabel::native("Clear Selection", "Auswahl aufheben"))
            .view_action("selectAll", LocalizedLabel::native("Select All", "Alles auswählen"))
            .action_with(ActionDefinition::new_catalog("selectSameKindSelection", LocalizedLabel::native("Select Same Kind", "Gleiche Art auswählen"), ActionKind::View).with_category("selection"))
            .view_action("selectSameKind", LocalizedLabel::native("Select Same Kind (alias)", "Gleiche Art auswählen (Alias)"))
            .view_action("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"))
            .view_action("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"))
            .view_action("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"))
            .view_action("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"))
            .view_action("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"))
            .view_action("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"))
            .view_action("engagementControlSelect", LocalizedLabel::native("Engagement Control Select", "Eingabesteuerung auswählen"))
            .view_action("cycleBrushCandidate", LocalizedLabel::native("Cycle Brush Candidate", "Pinselkandidat wechseln"))
            .view_action("registerBrushMesh", LocalizedLabel::native("Register Brush Mesh", "Pinsel-Mesh registrieren"))
            .view_action("setBrushPlacementOverlapBudget", LocalizedLabel::native("Set Brush Placement Overlap Budget", "Pinsel-Überlappungsbudget festlegen"))
            .view_action("setObjectKindWeight", LocalizedLabel::native("Set Object Kind Weight", "Objektart-Gewicht festlegen"))
            .view_action("setVortexKindWeight", LocalizedLabel::native("Set Vortex Kind Weight", "Vortexart-Gewicht festlegen"))
            .view_action("worldSelect", LocalizedLabel::native("World Select", "Welt auswählen"))
            .view_action("worldPick", LocalizedLabel::native("World Pick", "Welt-Auswahl (Pick)"))
            .view_action("worldHover", LocalizedLabel::native("World Hover", "Überfahren (Welt)"))
            .view_action("setHover", LocalizedLabel::native("Set Hover", "Überfahren festlegen"))
            .view_action("worldVortexHover", LocalizedLabel::native("World Vortex Hover", "Welt-Vortex-Hover"))
            .view_action("worldVortexSelect", LocalizedLabel::native("World Vortex Select", "Welt-Vortex auswählen"))
            .view_action("setSelectionMethod", LocalizedLabel::native("Set Selection Method", "Auswahlmethode festlegen"))
            .view_action("setLodMode", LocalizedLabel::native("Set Lod Mode", "LOD-Modus festlegen"))
            .view_action("setSuggestionOffset", LocalizedLabel::native("Set Suggestion Offset", "Vorschlagsversatz festlegen"))
            .view_action("setGridSnapEnabled", LocalizedLabel::native("Set Grid Snap Enabled", "Rasterfang aktivieren"))
            .view_action("setGridFactor", LocalizedLabel::native("Set Grid Factor", "Rasterfaktor festlegen"))
            .view_action("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"))
            .view_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"))
            // 📝️ Staged argument forms for the brush create actions (P1).
            .action_args("addPartKind", vec![
                ActionArgDef::select("partKind", puzzle5d_localized(|l| l.kind), vec![ActionArgOption::new("Part", puzzle5d_localized(|l| l.part))]).default_value("Part"),
            ])
            .action_args("addBrushPart", vec![
                ActionArgDef::select("partKind", puzzle5d_localized(|l| l.kind), vec![ActionArgOption::new("Part", puzzle5d_localized(|l| l.part))]).default_value("Part"),
            ])
            .action_args("addBrushObject", vec![
                ActionArgDef::select("partKind", puzzle5d_localized(|l| l.kind), vec![ActionArgOption::new("Part", puzzle5d_localized(|l| l.part))]).default_value("Part"),
            ])
            // 🧰️ Flat per-window set of utilities (host-owned `view_state.active_utility_id`); `select` is the default.
            .utility(UtilityDefinition { category: Some(UtilityCategory::Selection), ..UtilityDefinition::new("select", puzzle5d_localized(|l| l.select), "mouse-pointer") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("move", LocalizedLabel::native("Move", "Verschieben"), "move") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("rotate", LocalizedLabel::native("Rotate", "Drehen"), "rotate-cw") })
            .utility(UtilityDefinition { group: Some("transform".into()), ..UtilityDefinition::new("scale", LocalizedLabel::native("Scale", "Skalieren"), "maximize-2") })
            .utility(UtilityDefinition::new("brush", puzzle5d_localized(|l| l.brush), "paintbrush"))
            .utility(UtilityDefinition::new("fill", puzzle5d_localized(|l| l.fill), "paint-bucket"))
            .utility(UtilityDefinition::new("worldRelocate", LocalizedLabel::native("Relocate", "Verlagern"), "relocate-3d"))
            .window_kind_utilities(PUZZLE5D_PLAY_WINDOW_2D, vec!["select".into(), "brush".into(), "fill".into()])
            .window_kind_utilities(PUZZLE5D_PLAY_WINDOW_3D, vec!["move".into(), "rotate".into(), "scale".into(), "brush".into(), "fill".into(), "worldRelocate".into()])
            // 📇️ Per-window action scoping — the 3D window (World3d) owns the transform-gumball operations
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
    app.example(PUZZLE5D_EXAMPLE_CONCRETE_FOREST, puzzle5d_localized(|l| l.example_concrete_forest), CONCRETE_FOREST_EXAMPLE_JSON.clone(), "list-tree")
        .example(PUZZLE5D_EXAMPLE_NAKAGIN, LocalizedLabel::native("Nakagin Capsule Tower", "Nakagin-Kapselturm"), NAKAGIN_EXAMPLE_JSON.clone(), "building")
        .workflow("puzzle5d", "Puzzle 5D", "model")
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
/// 📥️ Tier C DWG mesh import — always returns the empty puzzle-5d document; never errors on a structurally valid mesh.
fn puzzle5d_document_from_mesh(_mesh: &semio_framework_plugin::MeshData) -> Result<Value, String> {
    serde_json::to_value(empty_document()).map_err(|error| error.to_string())
}

pub fn register_puzzle5d_exports() {
    // 🗂️ Registers `Puzzle5dPlayProjection`'s pack<->dsl codec under its real `document_schema()`
    // string so `framework/sync`'s `FolderEndpoint::Pack` can print/parse puzzle-5d play documents
    // without depending on this crate's concrete `Projection`/`Operation` types.
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<Puzzle5dPlayApp>(PUZZLE5D_SCHEMA);
    #[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
    {
        semio_framework_os::register_mesh_exporter("5d.puzzle", "puzzle5d", |_| Ok(semio_framework_plugin::mesh_from_kind("box")), Box::new(semio_framework_plugin::ObjExporter));
        semio_framework_os::register_mesh_exporter("5d.puzzle", "puzzle5d", |_| Ok(semio_framework_plugin::mesh_from_kind("box")), Box::new(semio_framework_plugin::GlbExporter));
        semio_framework_os::register_mesh_exporter("5d.puzzle", "puzzle5d", |_| Ok(semio_framework_plugin::mesh_from_kind("box")), Box::new(semio_framework_plugin::StlExporter));
        semio_framework_os::register_mesh_importer("5d.puzzle", puzzle5d_document_from_mesh, Box::new(semio_framework_plugin::ObjImporter));
        semio_framework_os::register_mesh_importer("5d.puzzle", puzzle5d_document_from_mesh, Box::new(semio_framework_plugin::GlbImporter));
        semio_framework_os::register_mesh_importer("5d.puzzle", puzzle5d_document_from_mesh, Box::new(semio_framework_plugin::StlImporter));
        semio_framework_os::register_mesh_dwg_export_handler("5d.puzzle", "puzzle5d", |_| Ok(semio_framework_plugin::mesh_from_kind("box")));
        semio_framework_os::register_mesh_dwg_import_handler("5d.puzzle", puzzle5d_document_from_mesh);
    }
}
//#endregion 🔖️Manifest

//#region 🔖️WasmBridge
#[cfg(target_arch = "wasm32")]
mod wasm_bridge {
    
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
//#endregion 🔖️WasmBridge

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::OperationDiff;
    use semio_framework_plugin::{testkit, ContextMenuRequest, ContextMenuSelectionGroup, ContextMenuSurfaceTarget, PluginApp, UiMenuRef, VcsDocumentApp, ViewState};

    fn new_app_with_registry() -> VcsDocumentApp<Puzzle5dPlayApp> {
        testkit::new_app_with_registry::<Puzzle5dPlayApp>(create_puzzle5d_app)
    }

    fn part_count(app: &VcsDocumentApp<Puzzle5dPlayApp>) -> usize {
        app.projection().expect("projection").0.get("parts").and_then(|value| value.as_array()).map(Vec::len).unwrap_or(0)
    }

    /// 🧪️ B1: test-only replacement for the deleted `VcsDocumentApp::handle_action` app-dispatch path
    /// (that method is FRAMEWORK-reserved now — see its doc comment in `semio_framework_plugin`; an
    /// app's own actions go exclusively through the typed `Self::Command` channel). Reconstructs the
    /// `Puzzle5dCommand` from the same `(action, args, window_id)` triple every pre-B1 test already
    /// passed and dispatches it via `VcsDocumentApp::dispatch_typed`.
    fn dispatch_action(app: &mut VcsDocumentApp<Puzzle5dPlayApp>, action: &str, args: Option<&Value>, window_id: Option<&str>, meta: &semio_framework_plugin::ActionMeta) -> Result<semio_framework_plugin::InvocationResult, String> {
        // 🕰️ Framework-reserved verbs (undo/redo/checkpoint/…) stay on `handle_action` — B1 keeps that
        // path FRAMEWORK-only, an app's own actions go through the typed `Self::Command` channel below.
        if matches!(action, "undo" | "redo" | "checkpoint" | "alternative" | "revertToCommand" | "historyFilter" | "noteShellCommand" | "copy" | "cut" | "paste") {
            return app.handle_action(action, args, meta);
        }
        app.dispatch_typed(Puzzle5dCommand::from_action(action, args.cloned(), window_id.map(str::to_string)), meta)
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
        let app = testkit::new_app::<Puzzle5dPlayApp>();
        assert_eq!(app.projection().expect("projection").0.get("schema").and_then(|value| value.as_str()), Some(PUZZLE5D_SCHEMA));
        assert!(part_count(&app) > 0, "the concrete-forest default document ships with parts");
    }

    /// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: the selection context menu stays a shallow,
    /// disclosed list (top-level verbs + a handful of taxonomy groups) rather than a flat wall of rows,
    /// and the known destructive `deleteSelection` action stays the trailing group's last item.
    #[test]
    fn context_menu_is_grouped_and_keeps_delete_selection_last() {
        let mut app = new_app_with_registry();
        let part_id = app.projection().expect("projection").0["parts"][0]["id"].as_str().expect("seeded part").to_string();
        dispatch_action(&mut app, "setSelection", Some(&json!({ "partIds": [part_id.clone()] })), None, &testkit::meta("local")).expect("select part");
        let request = ContextMenuRequest {
            menu: UiMenuRef { id: "world3d".into(), args: None },
            surface: Some(ContextMenuSurfaceTarget {
                surface_id: PUZZLE5D_PLAY_WINDOW_3D.into(),
                kind: "world3d".into(),
                hits: vec![],
                selection: vec![ContextMenuSelectionGroup { domain: "part".into(), ids: vec![part_id] }],
                text: None,
            }),
            window_instance_id: None,
            point: None,
        };
        let menu = app.context_menu(&request);
        assert!(menu.len() <= 9, "top-level context menu should stay progressively disclosed: {menu:?}");
        let last = menu.last().expect("selection context menu should not be empty");
        let last_is_destructive_leaf = last.action.as_deref() == Some("deleteSelection") && last.destructive == Some(true);
        let last_is_group_ending_in_destructive = last.children.as_ref().and_then(|children| children.last()).map(|child| child.action.as_deref() == Some("deleteSelection") && child.destructive == Some(true)).unwrap_or(false);
        assert!(last_is_destructive_leaf || last_is_group_ending_in_destructive, "known destructive deleteSelection must stay last: {menu:?}");
    }

    /// 📦️ `Puzzle5dPlayProjection`'s pack encoding round-trips through the same `(RecordSpec,
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
        dispatch_action(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None, &testkit::meta("local")).expect("empty");
        assert_eq!(part_count(&app), 0, "empty example clears the parts");
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert_eq!(part_count(&app), loaded, "undo restores the concrete-forest parts");
        app.handle_action("redo", None, &testkit::meta("local")).expect("redo");
        assert_eq!(part_count(&app), 0);
    }

    #[test]
    fn patch_fastener_updates_transform_offsets_and_undoes() {
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        dispatch_action(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None, &testkit::meta("local")).expect("load nakagin (has fasteners)");
        let projection = app.projection().expect("projection");
        let fastener_id = projection.0["fasteners"][0]["id"].as_str().expect("seeded fastener").to_string();
        dispatch_action(&mut app, "patchFastener", Some(&json!({ "fastenerId": fastener_id, "field": "gap", "value": 2.5 })), None, &testkit::meta("local")).expect("patch gap");
        let after = app.projection().expect("projection");
        let fastener = after.0["fasteners"].as_array().unwrap().iter().find(|entry| entry["id"] == fastener_id).expect("fastener");
        assert_eq!(fastener["gap"], 2.5);
        assert_eq!(fastener["shift"], 0.0);
        dispatch_action(&mut app, "patchFastener", Some(&json!({ "fastenerId": fastener_id, "field": "rotation", "value": 30.0 })), None, &testkit::meta("local")).expect("patch rotation");
        let after2 = app.projection().expect("projection");
        let fastener2 = after2.0["fasteners"].as_array().unwrap().iter().find(|entry| entry["id"] == fastener_id).expect("fastener");
        assert_eq!(fastener2["gap"], 2.5, "earlier gap edit must survive a later rotation edit");
        assert_eq!(fastener2["rotation"], 30.0);
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        let undone = app.projection().expect("projection");
        let fastener3 = undone.0["fasteners"].as_array().unwrap().iter().find(|entry| entry["id"] == fastener_id).expect("fastener");
        assert_eq!(fastener3["rotation"], 0.0, "undo restores the pre-rotation-edit value");
        assert_eq!(fastener3["gap"], 2.5, "undo of rotation edit must not also revert the earlier gap edit");
    }

    #[test]
    fn copy_emits_clipboard_fragment_for_the_closed_selection() {
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        dispatch_action(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None, &testkit::meta("local")).expect("load nakagin");
        let projection = app.projection().expect("projection");
        let first_part_id = projection.0["parts"][0]["id"].as_str().expect("seeded part").to_string();
        dispatch_action(&mut app, "setSelection", Some(&json!({ "partIds": [first_part_id] })), None, &testkit::meta("local")).expect("select");
        let result = app.handle_action("copy", None, &testkit::meta("local")).expect("copy");
        assert!(result.operations.is_empty(), "copy must not record an undo entry");
        assert_eq!(result.requested_effects.len(), 1);
        let HostEffect::ClipboardWrite { fragment } = &result.requested_effects[0] else { panic!("expected ClipboardWrite effect") };
        assert_eq!(fragment.source_app, PUZZLE5D_PLAY_APP_ID);
        let fragment_value: Value = serde_json::from_str(&fragment.dsl_text).expect("fragment dsl_text is JSON");
        assert_eq!(fragment_value["parts"].as_array().expect("parts").len(), 1);
    }

    #[test]
    fn copy_with_no_selection_is_a_benign_no_operation() {
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        let result = app.handle_action("copy", None, &testkit::meta("local")).expect("copy");
        assert!(result.operations.is_empty());
        assert!(result.requested_effects.is_empty());
    }

    #[test]
    fn cut_removes_selected_part_and_undo_restores_it() {
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        dispatch_action(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None, &testkit::meta("local")).expect("load nakagin");
        let before_count = part_count(&app);
        let projection = app.projection().expect("projection");
        let first_part_id = projection.0["parts"][0]["id"].as_str().expect("seeded part").to_string();
        dispatch_action(&mut app, "setSelection", Some(&json!({ "partIds": [first_part_id.clone()] })), None, &testkit::meta("local")).expect("select");
        let result = app.handle_action("cut", None, &testkit::meta("local")).expect("cut");
        assert_eq!(result.requested_effects.len(), 1, "cut must also copy to the clipboard");
        assert_eq!(part_count(&app), before_count - 1);
        let after = app.projection().expect("projection");
        assert!(!after.0["parts"].as_array().unwrap().iter().any(|part| part["id"] == first_part_id));
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert_eq!(part_count(&app), before_count, "one undo restores the cut part as a single edit");
    }

    #[test]
    fn paste_materializes_fragment_parts_at_original_anchor_with_fresh_ids() {
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        dispatch_action(&mut app, "setActiveExample", Some(&json!({ "exampleId": PUZZLE5D_EXAMPLE_NAKAGIN })), None, &testkit::meta("local")).expect("load nakagin");
        let projection = app.projection().expect("projection");
        let first_part_id = projection.0["parts"][0]["id"].as_str().expect("seeded part").to_string();
        dispatch_action(&mut app, "setSelection", Some(&json!({ "partIds": [first_part_id.clone()] })), None, &testkit::meta("local")).expect("select");
        let copy_result = app.handle_action("copy", None, &testkit::meta("local")).expect("copy");
        let HostEffect::ClipboardWrite { fragment } = &copy_result.requested_effects[0] else { panic!("expected ClipboardWrite effect") };
        let before_count = part_count(&app);
        let before_ids: HashSet<String> = projection.0["parts"].as_array().unwrap().iter().map(|part| part["id"].as_str().unwrap_or_default().to_string()).collect();
        let paste_args = json!({ "fragment": fragment, "anchor": "original", "position": [10.0, 0.0, 0.0] });
        app.handle_action("paste", Some(&paste_args), &testkit::meta("local")).expect("paste");
        assert_eq!(part_count(&app), before_count + 1);
        let after = app.projection().expect("projection");
        let pasted_parts: Vec<&Value> = after.0["parts"].as_array().unwrap().iter().filter(|part| !before_ids.contains(part["id"].as_str().unwrap_or_default())).collect();
        assert_eq!(pasted_parts.len(), 1);
        // "original" anchor uses the raw position override verbatim as the 2D delta.
        let original_x = projection.0["parts"][0]["2d"]["x"].as_f64().unwrap_or(0.0);
        assert_eq!(pasted_parts[0]["2d"]["x"].as_f64().unwrap(), original_x + 10.0);
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert_eq!(part_count(&app), before_count, "one undo removes the whole pasted fragment");
    }

    #[test]
    fn paste_with_no_fragment_arg_is_a_benign_no_operation() {
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        let before_count = part_count(&app);
        let result = app.handle_action("paste", None, &testkit::meta("local")).expect("paste");
        assert!(result.operations.is_empty());
        assert_eq!(part_count(&app), before_count);
    }

    #[test]
    fn import_compose_kit_replaces_parts_and_fasteners_and_undoes_as_one_edit() {
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        let before_count = part_count(&app);
        let compose_design = json!({
            "id": "design-1",
            "name": "Imported Tower",
            "pieces": { "items": [
                { "id": "piece-a", "type": { "id": "type-x" }, "pose": { "center": { "u": 1.0, "v": 2.0 }, "plane": { "origin": { "x": 0.0, "y": 0.0, "z": 0.0 }, "xAxis": { "x": 1.0, "y": 0.0, "z": 0.0 }, "yAxis": { "x": 0.0, "y": 1.0, "z": 0.0 } } } },
                { "id": "piece-b", "type": { "id": "type-x" }, "pose": { "center": { "u": 3.0, "v": 4.0 }, "plane": { "origin": { "x": 1.0, "y": 1.0, "z": 1.0 }, "xAxis": { "x": 1.0, "y": 0.0, "z": 0.0 }, "yAxis": { "x": 0.0, "y": 1.0, "z": 0.0 } } } },
            ] },
            "connections": { "items": [
                { "id": "conn-1", "parent": { "piece": { "id": "piece-a" }, "connector": { "id": "c1" } }, "child": { "piece": { "id": "piece-b" }, "connector": { "id": "c2" } }, "gap": 0.5, "shift": 0.0, "rise": 0.0, "rotation": 0.0, "turn": 0.0, "tilt": 0.0 },
            ] },
        });
        dispatch_action(&mut app, "importComposeKit", Some(&json!({ "design": compose_design })), None, &testkit::meta("local")).expect("import");
        assert_eq!(part_count(&app), 2);
        let projection = app.projection().expect("projection");
        assert_eq!(projection.0["label"], "Imported Tower");
        assert_eq!(projection.0["fasteners"].as_array().unwrap().len(), 1);
        assert_eq!(projection.0["fasteners"][0]["gap"], 0.5);
        assert_eq!(projection.0["fasteners"][0]["source"], "piece-a:c1");
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert_eq!(part_count(&app), before_count, "one undo restores the pre-import document");
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
        let engagements = app.window_engagements();
        assert!(engagements.contains_key(PUZZLE5D_PLAY_WINDOW_2D));
        assert!(engagements.contains_key(PUZZLE5D_PLAY_WINDOW_3D));
    }

    //#region 🧰️ Window Actions & Utilities contract
    #[test]
    fn add_part_kind_materializes_the_declared_kind_default() {
        // 📝️ P1 arg form: addPartKind with no args materializes the declared `partKind` default and adds a part.
        let mut app = new_app_with_registry();
        dispatch_action(&mut app, "setActiveExample", Some(&json!({ "exampleId": "" })), None, &testkit::meta("local")).expect("empty");
        let before = part_count(&app);
        let result = dispatch_action(&mut app, "addPartKind", None, None, &testkit::meta("local")).expect("addPartKind");
        assert!(!result.operations.is_empty(), "addPartKind is an Operation that emits operations");
        assert_eq!(part_count(&app), before + 1, "the materialized default kind adds exactly one part");
        let projection = app.projection().expect("projection").0;
        let kind = projection.get("parts").and_then(Value::as_array).and_then(|parts| parts.last()).and_then(|part| part.get("partKind")).and_then(Value::as_str);
        assert_eq!(kind, Some("Part"), "the declared partKind default was materialized host-side");
    }

    #[test]
    fn set_active_utility_emits_no_ops_and_no_history_entry() {
        // 🧰️ Switching utilities is the framework View action: no document operations, no undo entry, no re-emitted effect.
        let mut app = new_app_with_registry();
        let before = app.projection().expect("projection").0;
        let result = dispatch_action(&mut app, SET_ACTIVE_UTILITY_ACTION_ID, Some(&json!({ "utilityId": "brush" })), None, &testkit::meta("local")).expect("switch utility");
        assert!(result.operations.is_empty(), "utility switching never emits document operations");
        assert!(result.requested_effects.is_empty(), "a user utility switch does not re-emit SetActiveUtility");
        assert_eq!(app.projection().expect("projection").0, before, "utility switching does not mutate the document");
    }

    #[test]
    fn set_camera_actions_write_runtime_and_emit_no_operations() {
        // 📷️ Camera pose is session-only view state (`ActionKind::View`): `setCamera2d`/`setCamera3d`
        // must mutate the app's runtime (visible via the rendered scene) without ever touching the
        // VCS-tracked document or emitting an operation.
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        let before = app.projection().expect("projection").0;
        let camera2d_result = dispatch_action(&mut app, "setCamera2d", Some(&json!({ "camera": { "x": 12.5, "y": -6.5, "zoom": 3.5 } })), None, &testkit::meta("local")).expect("setCamera2d");
        assert!(camera2d_result.operations.is_empty(), "setCamera2d is a View action and must never emit a document operation");
        assert_eq!(app.projection().expect("projection").0, before, "setCamera2d must not mutate the document");
        let board = serde_json::to_string(&app.render(PUZZLE5D_PLAY_BODY_2D, None, &ViewState::default()).expect("render 2d")).unwrap();
        assert!(board.contains("12.5") && board.contains("-6.5"), "the new 2D camera pose must be reflected in the rendered runtime state");
        let camera3d_result = dispatch_action(&mut app, "setCamera3d", Some(&json!({ "camera": { "position": [42.5, 7.5, 3.5], "target": [1.5, 2.5, 3.5], "zoom": 5.5 } })), None, &testkit::meta("local")).expect("setCamera3d");
        assert!(camera3d_result.operations.is_empty(), "setCamera3d is a View action and must never emit a document operation");
        assert_eq!(app.projection().expect("projection").0, before, "setCamera3d must not mutate the document");
        let world = serde_json::to_string(&app.render(PUZZLE5D_PLAY_BODY_3D, None, &ViewState::default()).expect("render 3d")).unwrap();
        assert!(world.contains("42.5") && world.contains("7.5") && world.contains("1.5"), "the new 3D camera pose must be reflected in the rendered runtime state");
    }

    #[test]
    fn engagements_expose_no_utility_switch_options_for_either_window() {
        // 🧰️ select/brush/fill switching lives only on the framework utility bar; neither the 2D nor the 3D
        // engagement HUD may duplicate it as options.
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        let engagements = app.window_engagements();
        for window in [PUZZLE5D_PLAY_WINDOW_2D, PUZZLE5D_PLAY_WINDOW_3D] {
            assert!(engagements.get(window).expect("engagement").options.is_none(), "the {window} engagement must not re-expose utility switching as options");
        }
    }

    /// 🎯️ D-3 follow-up: the fill-count slider and brush placement picker are tagged `WindowMeasure::Group`s
    /// in [`puzzle5d_window_measures`] (surfaced by `partition_window_measures` only for their active utility),
    /// never `WindowEngagementControl`s on the HUD — for both the 2D and 3D windows.
    #[test]
    fn fill_and_brush_params_are_tagged_utility_options_not_engagement_controls() {
        let labels = puzzle5d_labels(&Puzzle5dConfig::default());
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
        // 🪣️ Fill utility: the fill-count slider now lives in a "fill"-tagged Utility Options group (per window),
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
        // 🧰️ Reconciled dual entry point: the engagement token drives the same host-owned utility switch, once per window.
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        let result = dispatch_action(&mut app, "engagementSubmit", Some(&json!({ "window": PUZZLE5D_PLAY_WINDOW_3D, "value": "brush" })), None, &testkit::meta("local")).expect("submit");
        let windows: Vec<&str> = result.requested_effects.iter().filter_map(|effect| match effect { HostEffect::SetActiveUtility { window_id, utility_id } if utility_id == "brush" => Some(window_id.as_str()), _ => None }).collect();
        assert!(windows.contains(&PUZZLE5D_PLAY_WINDOW_2D) && windows.contains(&PUZZLE5D_PLAY_WINDOW_3D), "brush switch is pushed to both windows, got {windows:?}");
    }

    #[test]
    fn gumball_translate_drag_coalesces_into_one_edit() {
        // 🌀️ Coalescing regression: three translate ticks with the same key are ONE undoable edit.
        let mut app = testkit::new_app::<Puzzle5dPlayApp>();
        let part_id = app.projection().expect("projection").0.get("parts").and_then(Value::as_array).and_then(|parts| parts.first()).and_then(|part| part.get("id")).and_then(Value::as_str).expect("part id").to_string();
        let origin_x = |app: &VcsDocumentApp<Puzzle5dPlayApp>| -> f64 {
            app.projection().expect("projection").0.get("parts").and_then(Value::as_array).and_then(|parts| parts.iter().find(|part| part.get("id").and_then(Value::as_str) == Some(part_id.as_str()))).and_then(|part| part.pointer("/3d/origin/0")).and_then(Value::as_f64).unwrap_or(0.0)
        };
        let start = origin_x(&app);
        for dx in [1.0, 2.0, 3.0] {
            dispatch_action(&mut app, "translateSelection", Some(&json!({ "ids": [part_id], "dx": dx, "dy": 0.0, "dz": 0.0 })), None, &testkit::meta("local")).expect("drag tick");
        }
        assert!((origin_x(&app) - start - 6.0).abs() < 1e-9, "three ticks accumulate 1+2+3 on x");
        app.handle_action("undo", None, &testkit::meta("local")).expect("undo");
        assert!((origin_x(&app) - start).abs() < 1e-9, "one undo restores the whole coalesced gumball drag");
    }
    //#endregion 🧰️ Window Actions & Utilities contract

    //#region 🔖️KitInPort
    /// 🔌️ The flagship `kit:in` seam: feeding a `kit.catalog` fragment shaped exactly like
    /// block3d's `puzzle3d_catalog_fragment` (`objectKinds`/`vortexKinds`, camelCase) through
    /// `Puzzle5dPlayApp::import_media` must normalize `objectKinds` into the typed
    /// `kindCatalogs.parts` (with each per-object `vortices[]` entry becoming a grip template) and
    /// `vortexKinds` into `kindCatalogs.grips`, and land both after applying the returned operations.
    #[test]
    fn kit_in_import_media_upserts_part_and_grip_kinds_into_kind_catalogs() {
        let app = Puzzle5dPlayApp::default();
        let projection = app.initial_projection();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = DocumentView { projection: &projection, history: &history };

        let fragment = json!({
            "schema": "manifest",
            "objectKinds": [{
                "id": "capsule",
                "name": "capsule",
                "label": "Capsule",
                "meshUrl": "/mesh/capsule.glb",
                "vortices": [{ "id": "v0", "vortexKind": "door", "position": [0.0, 0.0, 0.0], "direction": [0.0, 1.0, 0.0], "radius": 0.3 }],
            }],
            "vortexKinds": [{ "id": "door", "name": "door", "label": "Door", "color": "#ff0000", "defaultCableKind": "" }],
            "cableKinds": [],
            "attractionKinds": [],
            "kindCompatibility": [{ "source": "door", "target": "door", "bidirectional": true }],
        });
        let media = Media {
            media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
            payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() },
        };

        let emit = app.import_media("kit:in", &media, &doc).expect("kit:in import_media succeeds");
        assert!(!emit.document_operations.is_empty(), "importing a non-empty fragment must emit real operations");

        let mut next_projection = projection.0.clone();
        for operation in &emit.document_operations {
            next_projection = protocol::Operation::<Value>::diff(operation, &next_projection).apply(&next_projection);
        }

        let parts = next_projection.pointer("/kindCatalogs/parts").and_then(Value::as_array).expect("parts catalog present");
        let capsule = parts.iter().find(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")).expect("the imported part kind must appear in kindCatalogs.parts");
        assert_eq!(capsule.get("meshUrl").and_then(Value::as_str), Some("/mesh/capsule.glb"));
        assert_eq!(capsule.pointer("/grips/0/gripKind").and_then(Value::as_str), Some("door"), "the per-part grip template keeps its gripKind after normalization");
        assert_eq!(capsule.pointer("/grips/0/3d/position").and_then(Value::as_array), Some(&vec![json!(0.0), json!(0.0), json!(0.0)]));
        assert_eq!(capsule.pointer("/grips/0/3d/direction").and_then(Value::as_array), Some(&vec![json!(0.0), json!(1.0), json!(0.0)]));
        assert_eq!(capsule.pointer("/grips/0/3d/radius").and_then(Value::as_f64), Some(0.3));

        let grips = next_projection.pointer("/kindCatalogs/grips").and_then(Value::as_array).expect("grips catalog present");
        let door = grips.iter().find(|entry| entry.get("id").and_then(Value::as_str) == Some("door")).expect("the imported grip kind must appear in kindCatalogs.grips");
        assert_eq!(door.get("defaultRopeKind").and_then(Value::as_str), Some(""), "defaultCableKind maps onto defaultRopeKind (a naming judgment call — see import_media's doc comment)");

        let compatibility = next_projection.pointer("/kindCompatibility").and_then(Value::as_array).expect("kind compatibility present");
        assert!(compatibility.iter().any(|entry| entry.get("source").and_then(Value::as_str) == Some("door") && entry.get("target").and_then(Value::as_str) == Some("door")));
    }

    /// 🔁️ Re-importing the SAME fragment (simulating a second producer edge, or a redelivered
    /// message on a `multiplicity: Many` port) must upsert idempotently — no duplicate rows.
    #[test]
    fn kit_in_import_media_is_idempotent_on_repeated_delivery() {
        let app = Puzzle5dPlayApp::default();
        let projection = app.initial_projection();
        let history = semio_framework_plugin::HistoryView::empty();
        let mut current = projection.0.clone();

        let fragment = json!({
            "objectKinds": [{ "id": "capsule", "name": "capsule", "label": "Capsule", "meshUrl": "/mesh/capsule.glb", "vortices": [] }],
            "vortexKinds": [],
            "cableKinds": [],
            "attractionKinds": [],
            "kindCompatibility": [],
        });
        let media = Media {
            media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
            payload: semio_framework_plugin::MediaPayload::Structured { schema: "kit.catalog".into(), json: fragment.to_string() },
        };

        for _ in 0..2 {
            let doc_projection = Puzzle5dPlayProjection(current.clone());
            let doc = DocumentView { projection: &doc_projection, history: &history };
            let emit = app.import_media("kit:in", &media, &doc).expect("kit:in import_media succeeds");
            for operation in &emit.document_operations {
                current = protocol::Operation::<Value>::diff(operation, &current).apply(&current);
            }
        }

        let parts = current.pointer("/kindCatalogs/parts").and_then(Value::as_array).expect("parts catalog present");
        assert_eq!(parts.iter().filter(|entry| entry.get("id").and_then(Value::as_str) == Some("capsule")).count(), 1, "repeated delivery of the same fragment must upsert, never duplicate");
    }

    #[test]
    fn kit_in_port_is_declared_on_the_app_io() {
        let app = Puzzle5dPlayApp::default();
        let io = app.io().expect("puzzle5d declares an AppIo");
        let kit_in = io.ports.iter().find(|port| port.id == "kit:in").expect("kit:in port declared");
        assert_eq!(kit_in.kind_id.as_deref(), Some("kit.catalog"));
        assert_eq!(kit_in.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Type });
        assert!(matches!(kit_in.multiplicity, PortMultiplicity::Many));
        let design_out = io.ports.iter().find(|port| port.id == "design:out").expect("design:out port declared");
        assert_eq!(design_out.kind_id.as_deref(), Some("5d.puzzle"));
        assert_eq!(design_out.media_type, MediaType { class: MediaClass::Kit, form: MediaForm::Design });
        assert!(matches!(design_out.multiplicity, PortMultiplicity::Many));
    }
    //#endregion 🔖️KitInPort
}
//#endregion 🧪️Tests
