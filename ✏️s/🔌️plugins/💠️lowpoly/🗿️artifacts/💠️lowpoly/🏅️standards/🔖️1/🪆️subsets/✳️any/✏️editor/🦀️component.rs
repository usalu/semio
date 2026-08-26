//! 🖌️ Lowpoly editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, chrome measures/engagement shared by both windows in this file (they are
//! byte-identical between windows — see the master ticket's TEMPLATE.md §12.2 shared-options pattern,
//! extended here across mode boundaries since the Model window is reused by both `edit` and `paint`),
//! panel trees in `📌️panels/*`, labels in `🗣️terminology/🦀️component.rs`, view state in
//! `🎚️config/🦀️component.rs`, scratch (mid-gesture) state in `🖌️session/🦀️component.rs`, shared
//! read-view/selection helpers in `🧭️view/🦀️component.rs`.

use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::{artifact_kind, LowpolySnapshot, LOWPOLY_DOCUMENT_SCHEMA};
use crate::editor::lowpoly::commands::{add_primitive, camera, chrome, engagement, fixture, mesh_edit, paint, patch_object, selection, sun, transform, utility, uv};
use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::modes::{edit, paint as paint_mode};
use crate::editor::lowpoly::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel, layers as layers_panel};
use crate::editor::lowpoly::session::{LowpolyScratch, LowpolyTransient, LowpolyTransientMutation};
use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::{resolve_active_object_id, selection_from_interaction, selection_from_state, utility_param_f64, LowpolyView, MESH_INTERACTION_DOMAIN};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::app::{ArtifactOwnedToolJobContext, InteractionView};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDescriptor, ActionRef, AppOperationContext, ArtifactEditor, ArtifactOutputChunks, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactView, ConfigView, DraftView,
    Editor, EditorApp, Emit, EphemeralEmit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, LabelText, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, MergeMode,
    NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, UiNode, UtilityCategory, UtilityDefinition, WindowEngagement, WindowEngagementInput, WindowEngagementOption, WindowEngagementPossible, WindowEngagementStatus,
    WindowMeasure,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use store::ArtifactPack;
use store::EngineHandles;

//#region 🔖️Constants
pub const LOWPOLY_PLAY_APP_ID: &str = "lowpoly-play";
const LOWPOLY_PLAY_CONTROLLER_ID: &str = "lowpoly-play";
pub use crate::editor::lowpoly::modes::edit::windows::model::LOWPOLY_PLAY_BODY_MAIN;
pub use crate::editor::lowpoly::modes::paint::windows::uv::LOWPOLY_PLAY_BODY_UV;
pub use crate::editor::lowpoly::panels::catalogue::LOWPOLY_PLAY_BODY_CATALOGUE;
pub use crate::editor::lowpoly::panels::document::LOWPOLY_PLAY_BODY_DOCUMENT;
pub use crate::editor::lowpoly::panels::inspection::LOWPOLY_PLAY_BODY_INSPECTION;
pub use crate::editor::lowpoly::panels::layers::LOWPOLY_PLAY_BODY_LAYERS;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`🛠️options/*`, `📌️panels/*`, window/engagement builders) builds its `on_change`/item actions with.
pub fn lowpoly_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(LOWPOLY_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🧱️ Admits one fixed UI text action value without JSON staging.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    semio_framework_plugin::UiText::try_from_str(value.as_ref()).map(semio_framework_plugin::UiValue::Text).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI text admission failed"))
}

/// 🔘️ Admits one boolean UI action value.
pub fn ui_value_bool(value: bool) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Bool(value)
}

/// 🔢️ Admits one numeric UI action value.
pub fn ui_value_number(value: impl Into<f64>) -> semio_framework_plugin::UiValue {
    semio_framework_plugin::UiValue::Number(value.into())
}

/// 📚️ Admits one fixed UI list action value without dynamic staging.
pub fn ui_value_list(values: impl IntoIterator<Item = semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI list item admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::List(builder.finish()))
}

/// 🗺️ Admits one ordered fixed UI map action value without JSON staging.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, semio_framework_plugin::UiValue)>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI map entry admission failed"))?;
    }
    Ok(semio_framework_plugin::UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled UI nodes into fixed child storage.
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        let node = value?;
        nodes.try_push(node).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "fixed UI node admission failed"))?;
    }
    Ok(nodes)
}

//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `crate::artifacts::lowpoly::artifact_kind()` declares for `"3d.lowpoly"`, plus the two workflow
/// ports: `mesh:in` (Many, unrequired — accepts upstream mesh producers, e.g. cad via a Brep→Mesh
/// conversion) and `mesh:out` (Many, unrequired). Relocated from the deleted
/// `🗿️artifacts/💠️lowpoly/…/⚙️engine/🦀️component.rs` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): behaviour describing this app's own IO
/// surface belongs on the app, not the artifact.
///
/// 🧱️ `mesh:out`'s `kind_id` was `Some("3d.mesh")`, pinned to the now-deleted duplicate interchange
/// kind (ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` — `3d.mesh` is being removed repo-wide,
/// mesh is canonically `s.stdio.semio@v1/mesh`, a subset of a composite artifact kind, never its own
/// standalone `ArtifactKindSpec`). Set to `None` here (matching `mesh:in`'s existing precedent of
/// accepting without a specific kind pin) rather than repointing at a stdio kind id, since choosing
/// the RIGHT replacement wiring for a cross-plugin media port is a design decision beyond this
/// migration's boundary — flagged under `sharedFileRequests` in this wave's report.
pub fn lowpoly_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: LOWPOLY_DOCUMENT_SCHEMA.into(),
        document_media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
        ports: vec![
            semio_framework_plugin::MediaPortSpec {
                id: "mesh:in".into(),
                label: "Mesh".into(),
                direction: semio_framework_plugin::MediaPortDirection::In,
                media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
                kind_id: None,
                required: false,
                multiplicity: semio_framework_plugin::PortMultiplicity::Many,
            },
            semio_framework_plugin::MediaPortSpec {
                id: "mesh:out".into(),
                label: "Mesh".into(),
                direction: semio_framework_plugin::MediaPortDirection::Out,
                media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
                kind_id: None,
                required: false,
                multiplicity: semio_framework_plugin::PortMultiplicity::Many,
            },
        ],
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "3d.lowpoly".into(), name: "3D Lowpoly".into(), dimension: "3d".into(), component_kind: "lowpoly".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️SharedMeasures
/// 🎛️ Collects every window-chrome measure from the app-level `🛠️options/*` shared by both windows
/// (Model + UV expose an identical set — see this file's top-level doc comment).
pub fn lowpoly_window_measures(config: &LowpolyConfig, labels: &LowpolyLabels) -> Vec<WindowMeasure> {
    use crate::editor::lowpoly::options;
    vec![
        options::show_edges::measure(config, labels),
        options::sun::measure(config, labels),
        options::snap::measure(config, labels),
        options::select::measure(config, labels),
        options::paint_params_brush::measure(config, labels),
        options::paint_params_eraser::measure(config, labels),
    ]
}

/// 🧮️ Shared leaf builder for one utility-param slider — used by the `🧲️snap` option and by
/// `paint_utility_params_group` below.
#[allow(clippy::too_many_arguments, reason = "one WindowMeasure::Slider literal per call site; a params struct would only move the same 8 fields around for this single builder")]
pub fn utility_param_slider(id: &str, label: LabelText, key: &str, params: &Value, default: f64, min: f64, max: f64, step: f64) -> WindowMeasure {
    WindowMeasure::Slider {
        id: format!("lowpoly-measure-{id}"),
        label: Some(label.into()),
        value: utility_param_f64(params, key, default),
        min,
        max,
        step: Some(step),
        ready: None,
        loading: None,
        disabled: None,
        reveal: None,
        on_change: lowpoly_action("setUtilityParam", Some(json!({ "key": key }))),
        waiting: None,
    }
}

/// 🖌️ Utility Options for a stamping paint utility (`brush`/`eraser`) — the live brush size/opacity/
/// hardness sliders, tagged `active_utility_id: Some(utility)` so `partition_window_measures` surfaces
/// them in the Utility Options rail only while that exact utility is active. Both utilities stamp
/// through the same `stamp_brush` path, so they share an identical param set.
pub fn paint_utility_params_group(utility: &str, params: &Value, labels: &LowpolyLabels) -> WindowMeasure {
    let slider = |suffix: &str, label: LabelText, key: &str, default: f64, min: f64, max: f64, step: f64| WindowMeasure::Slider {
        id: format!("lowpoly-measure-{utility}-{suffix}"),
        label: Some(label.into()),
        value: utility_param_f64(params, key, default),
        min,
        max,
        step: Some(step),
        ready: None,
        loading: None,
        disabled: None,
        reveal: None,
        on_change: lowpoly_action("setUtilityParam", Some(json!({ "key": key }))),
        waiting: None,
    };
    WindowMeasure::Group {
        id: format!("lowpoly-measure-paint-params-{utility}"),
        label: labels.brush_group.into(),
        default_open: Some(true),
        active_utility_id: Some(utility.into()),
        value: None,
        min: None,
        max: None,
        step: None,
        ready: None,
        loading: None,
        waiting: None,
        on_change: None,
        children: vec![
            slider("size", labels.brush_size, "brushSize", 16.0, 1.0, 128.0, 1.0),
            slider("opacity", labels.brush_opacity, "brushOpacity", 1.0, 0.0, 1.0, 0.05),
            slider("hardness", labels.brush_hardness, "brushHardness", 0.5, 0.0, 1.0, 0.05),
        ],
    }
}
//#endregion 🔖️SharedMeasures

//#region 🔖️SharedEngagement
/// 🎛️ The window engagement (options/status/input/possible-engagements) shared byte-identically by both
/// windows — see this file's top-level doc comment.
pub fn lowpoly_window_engagement(view: LowpolyView<'_>, active_utility: &str, labels: &LowpolyLabels) -> WindowEngagement {
    let config = view.config;
    WindowEngagement {
        session_active: Some(true),
        // 🧰️ The move/rotate/scale transform switcher lives in the framework utility bar (declared via
        // `.utility` + window-level `utilities`), so the engagement keeps only its non-utility options.
        options: Some(vec![
            WindowEngagementOption { id: "lowpoly.opt.snap".into(), label: Some(labels.snap.into()), icon_id: Some("magnet".into()), pressed: None, disabled: None, action: Some(lowpoly_action("snap", None)) },
            WindowEngagementOption { id: "lowpoly.opt.smooth".into(), label: Some(labels.smooth.into()), icon_id: Some("sun".into()), pressed: None, disabled: None, action: Some(lowpoly_action("toggleSmooth", None)) },
            WindowEngagementOption {
                id: "lowpoly.opt.show-edges".into(),
                label: Some(labels.show_edges.into()),
                icon_id: Some("grid-3x3".into()),
                pressed: Some(config.show_edges),
                disabled: None,
                action: Some(lowpoly_action("toggleShowEdges", None)),
            },
        ]),
        input: Some(WindowEngagementInput {
            id: Some("lowpoly-engagement".into()),
            value: Some(config.engagement_input.clone()),
            placeholder: Some("extrude, inset, mirror, decimate".into()),
            disabled: None,
            on_change: Some(lowpoly_action("engagementInput", None)),
            on_submit: Some(lowpoly_action("engagementSubmit", None)),
            on_repeat_last: None,
            on_abort: None,
        }),
        control: None,
        controls: None,
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the mesh domain's live selection
        // count used to read off `LowpolyConfig`; it is framework-owned `InteractionState` now, and
        // `ArtifactApp::window_engagements` (unlike `handle`/`copy_fragment`/`cut_operations`) is not
        // threaded an `InteractionView` this wave — the status line drops the selection summary rather
        // than reading stale app-local state. Peer/self selection is surfaced generically by the shell.
        status: Some(vec![WindowEngagementStatus { id: "lowpoly-status".into(), text: active_utility.to_string() }]),
        possible_engagements: Some(vec![
            WindowEngagementPossible { id: "lowpoly.eng.extrude".into(), label: labels.extrude.into(), detail: None, action: Some(lowpoly_action("extrude", None)) },
            WindowEngagementPossible { id: "lowpoly.eng.triangulate".into(), label: labels.triangulate.into(), detail: None, action: Some(lowpoly_action("triangulate", None)) },
        ]),
    }
}
//#endregion 🔖️SharedEngagement

//#region 🔖️Commands
semio_framework_plugin::app_commands! {
    /// 🎯️ `LowpolyPlayApp::Command` — the SOLE dispatch surface for lowpoly's own behavior, covering
    /// every declared action. Row order is the binary variant ordinal: appending is safe, reordering is
    /// a wire-format break.
    pub enum LowpolyCommand for LowpolySnapshot, LowpolyMutation, LowpolyConfig, LowpolyConfigMutation, ctx = LowpolyScratch {
        "addPrimitive" as "add-primitive" => add_primitive::AddPrimitive,
        "patchObject" as "patch-object" => patch_object::PatchObject,
        "extrude" as "extrude" => extrude::Extrude,
        "inset" as "inset" => inset::Inset,
        "bevel" as "bevel" => bevel::Bevel,
        "loopCut" as "loop-cut" => loop_cut::LoopCut,
        "subdivide" as "subdivide" => subdivide::Subdivide,
        "triangulate" as "triangulate" => triangulate::Triangulate,
        "mirror" as "mirror" => mirror::Mirror,
        "decimate" as "decimate" => decimate::Decimate,
        "flipFaces" as "flip-faces" => flip_faces::FlipFaces,
        "merge" as "merge" => merge::Merge,
        "dissolve" as "dissolve" => dissolve::Dissolve,
        "snap" as "snap" => snap::Snap,
        "toggleSmooth" as "toggle-smooth" => toggle_smooth::ToggleSmooth,
        "unwrapActive" as "unwrap-active" => unwrap_active::UnwrapActive,
        "markUvSeam" as "mark-uv-seam" => mark_uv_seam::MarkUvSeam,
        "clearSeam" as "clear-seam" => clear_seam::ClearSeam,
        "translateSelection" as "translate-selection" => translate_selection::TranslateSelection,
        "rotateSelection" as "rotate-selection" => rotate_selection::RotateSelection,
        "scaleSelection" as "scale-selection" => scale_selection::ScaleSelection,
        "addPaintLayer" as "add-paint-layer" => add_paint_layer::AddPaintLayer,
        "paintStrokeEnd" as "paint-stroke-end" => paint_stroke_end::PaintStrokeEnd,
        "paintFill" as "paint-fill" => paint_fill::PaintFill,
        "fillBucket" as "fill-bucket" => fill_bucket::FillBucket,
        "transformEnd" as "transform-end" => transform_end::TransformEnd,
        "importSnapshotJson" as "import-snapshot-json" => set_snapshot_json::ImportSnapshotJson,
        "setFixtureJson" as "set-fixture-json" => set_fixture_json::SetFixtureJson,
        "engagementSubmit" as "engagement-submit" => engagement_submit::EngagementSubmit,
        "setActiveObject" as "set-active-object" => set_active_object::SetActiveObject,
        "setActivePaintLayer" as "set-active-paint-layer" => set_active_paint_layer::SetActivePaintLayer,
        "setUtilityParam" as "set-utility-param" => set_utility_param::SetUtilityParam,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "toggleShowEdges" as "toggle-show-edges" => toggle_show_edges::ToggleShowEdges,
        "toggleSun" as "toggle-sun" => toggle_sun::ToggleSun,
        "setSunAzimuth" as "set-sun-azimuth" => set_sun_azimuth::SetSunAzimuth,
        "setSunElevation" as "set-sun-elevation" => set_sun_elevation::SetSunElevation,
        "setSunIntensity" as "set-sun-intensity" => set_sun_intensity::SetSunIntensity,
        "setCamera" as "set-camera" => set_camera::SetCamera,
        "paintStrokeBegin" as "paint-stroke-begin" => paint_stroke_begin::PaintStrokeBegin,
        "paintSample" as "paint-sample" => paint_sample::PaintSample,
        "paintStroke" as "paint-stroke" => paint_stroke::PaintStroke,
        "paintAt" as "paint-at" => paint_at::PaintAt,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "transformBegin" as "transform-begin" => transform_begin::TransformBegin,
        "setActiveUtility" as "set-active-utility" => set_active_utility::SetActiveUtility,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name. `mesh_edit`/`uv`/`transform`/`paint`/
// `selection`/`sun`/`utility`/`engagement`/`fixture` collide with their containing command-group
// modules and are flattened via glob-free explicit `use`.
use camera::set_camera;
use chrome::toggle_show_edges;
use engagement::{engagement_input, engagement_submit};
use fixture::{set_fixture_json, set_snapshot_json};
use mesh_edit::{bevel, decimate, dissolve, extrude, flip_faces, inset, loop_cut, merge, mirror, snap, subdivide, toggle_smooth, triangulate};
use paint::{add_paint_layer, canvas_pointer_down, canvas_pointer_move, fill_bucket, paint_at, paint_fill, paint_sample, paint_stroke, paint_stroke_begin, paint_stroke_end};
use selection::{set_active_object, set_active_paint_layer};
use sun::{set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use transform::{rotate_selection, scale_selection, transform_begin, transform_end, translate_selection};
use utility::{set_active_utility, set_utility_param};
use uv::{clear_seam, mark_uv_seam, unwrap_active};
//#endregion 🔖️Commands

//#region 🧵️RetainedCommands
const LOWPOLY_RETAINED_PAYLOAD_SCHEMA: &str = "lowpoly.command.v1";
const LOWPOLY_RETAINED_RAW_BYTES: usize = 1024 * 1024;
const LOWPOLY_RETAINED_WORK_ITEMS: usize = 1_024;
const LOWPOLY_SCAN_BYTES: usize = 4_096;
const LOWPOLY_RETAINED_OBJECTS: usize = 8;
const LOWPOLY_RETAINED_LAYERS_PER_OBJECT: usize = 8;
const LOWPOLY_RETAINED_MESH_BYTES: usize = 1024 * 1024;
const LOWPOLY_RETAINED_PAINT_BYTES: usize = crate::artifacts::lowpoly::LOWPOLY_PAINT_TEXTURE_SIZE * crate::artifacts::lowpoly::LOWPOLY_PAINT_TEXTURE_SIZE * 4;
const LOWPOLY_RETAINED_SELECTION_IDS: usize = 4_096;
const LOWPOLY_TOOL_IDS: &[&str] = &[
    "addPrimitive",
    "patchObject",
    "extrude",
    "inset",
    "bevel",
    "loopCut",
    "subdivide",
    "triangulate",
    "mirror",
    "decimate",
    "flipFaces",
    "merge",
    "dissolve",
    "snap",
    "toggleSmooth",
    "unwrapActive",
    "markUvSeam",
    "clearSeam",
    "translateSelection",
    "rotateSelection",
    "scaleSelection",
    "addPaintLayer",
    "paintStrokeEnd",
    "paintFill",
    "fillBucket",
    "transformEnd",
    "importSnapshotJson",
    "setFixtureJson",
    "engagementSubmit",
    "setActiveObject",
    "setActivePaintLayer",
    "setUtilityParam",
    "engagementInput",
    "toggleShowEdges",
    "toggleSun",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setCamera",
    "paintStrokeBegin",
    "paintSample",
    "paintStroke",
    "paintAt",
    "canvasPointerDown",
    "canvasPointerMove",
    "transformBegin",
    "setActiveUtility",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum LowpolyCommandDisposition {
    Sync = 1,
    Json = 2,
    Mesh = 3,
    Uv = 4,
    Paint = 5,
    Transform = 6,
}

fn lowpoly_command_disposition(tool_id: &str) -> Option<LowpolyCommandDisposition> {
    Some(match tool_id {
        "importSnapshotJson" | "setFixtureJson" => LowpolyCommandDisposition::Json,
        "addPrimitive" | "extrude" | "inset" | "bevel" | "loopCut" | "subdivide" | "triangulate" | "mirror" | "decimate" | "flipFaces" | "merge" | "dissolve" | "snap" | "toggleSmooth" | "engagementSubmit" => LowpolyCommandDisposition::Mesh,
        "unwrapActive" | "markUvSeam" | "clearSeam" => LowpolyCommandDisposition::Uv,
        "addPaintLayer" | "paintStrokeBegin" | "paintStroke" | "paintAt" | "canvasPointerDown" | "canvasPointerMove" | "paintStrokeEnd" | "paintFill" | "fillBucket" | "paintSample" => LowpolyCommandDisposition::Paint,
        "transformBegin" | "translateSelection" | "rotateSelection" | "scaleSelection" | "transformEnd" => LowpolyCommandDisposition::Transform,
        tool_id if LOWPOLY_TOOL_IDS.contains(&tool_id) => LowpolyCommandDisposition::Sync,
        _ => return None,
    })
}

fn lowpoly_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(LOWPOLY_RETAINED_RAW_BYTES, LOWPOLY_RETAINED_WORK_ITEMS, 1, 32 * 1024 * 1024, 7_500, 1, 1)
}

fn lowpoly_chunks(bytes: usize) -> usize {
    bytes.div_ceil(LOWPOLY_SCAN_BYTES).max(1)
}

fn lowpoly_snapshot_scan_units(snapshot: &LowpolySnapshot) -> Option<usize> {
    snapshot.objects.iter().try_fold(0_usize, |units, object| {
        let fields = lowpoly_chunks(object.id.len()).checked_add(lowpoly_chunks(object.name.len()))?;
        object.paint_layers.iter().try_fold(units.checked_add(fields)?, |units, layer| units.checked_add(lowpoly_chunks(layer.name.len()))?.checked_add(lowpoly_chunks(layer.pixels.len())))
    })
}

fn lowpoly_extent(snapshot: &LowpolySnapshot, interaction: &protocol::InteractionState, context: &ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>) -> Option<usize> {
    if snapshot.objects.len() > LOWPOLY_RETAINED_OBJECTS
        || snapshot.objects.iter().any(|object| {
            object.id.len() > LOWPOLY_SCAN_BYTES
                || object.name.len() > LOWPOLY_SCAN_BYTES
                || object.paint_layers.len() > LOWPOLY_RETAINED_LAYERS_PER_OBJECT
                || object.paint_layers.iter().any(|layer| layer.name.len() > LOWPOLY_SCAN_BYTES || layer.pixels.len() > LOWPOLY_RETAINED_PAINT_BYTES)
        })
        || interaction.selection.get(MESH_INTERACTION_DOMAIN).is_some_and(|selection| selection.ids.len() > LOWPOLY_RETAINED_SELECTION_IDS)
        || !context.transient.retained_shape_admitted(LOWPOLY_RETAINED_OBJECTS, LOWPOLY_RETAINED_MESH_BYTES, LOWPOLY_RETAINED_PAINT_BYTES)
    {
        return None;
    }
    context.transient.segmented_extent(LOWPOLY_SCAN_BYTES)?.checked_add(lowpoly_snapshot_scan_units(snapshot)?)?.checked_add(interaction.selection.get(MESH_INTERACTION_DOMAIN).map_or(0, |selection| selection.ids.len()))?.checked_add(1)
}

fn lowpoly_extent_admitted(extent: usize) -> bool {
    extent != 0 && extent <= LOWPOLY_RETAINED_WORK_ITEMS
}

fn lowpoly_snapshot_chunk(snapshot: &LowpolySnapshot, mut cursor: usize) -> Option<&[u8]> {
    for object in &snapshot.objects {
        for bytes in [object.id.as_bytes(), object.name.as_bytes()] {
            let units = lowpoly_chunks(bytes.len());
            if cursor < units {
                let start = cursor * LOWPOLY_SCAN_BYTES;
                return Some(&bytes[start.min(bytes.len())..start.saturating_add(LOWPOLY_SCAN_BYTES).min(bytes.len())]);
            }
            cursor -= units;
        }
        for layer in &object.paint_layers {
            for bytes in [layer.name.as_bytes(), layer.pixels.as_slice()] {
                let units = lowpoly_chunks(bytes.len());
                if cursor < units {
                    let start = cursor * LOWPOLY_SCAN_BYTES;
                    return Some(&bytes[start.min(bytes.len())..start.saturating_add(LOWPOLY_SCAN_BYTES).min(bytes.len())]);
                }
                cursor -= units;
            }
        }
    }
    None
}

fn lowpoly_retained_reduce(
    command: &LowpolyCommand,
    snapshot: &LowpolySnapshot,
    config: &LowpolyConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    context: &ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>,
    operation: &AppOperationContext,
) -> Result<ArtifactCommandWorkStep<EditorApp<LowpolyPlayApp>>, Fault> {
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config };
    let mut bounded = LowpolyScratch::default();
    let direct = match command {
        LowpolyCommand::PatchObject(payload) => Some(patch_object::handle(payload, &doc, &cfg, &mut bounded)),
        LowpolyCommand::ImportSnapshotJson(payload) => Some(set_snapshot_json::handle(payload, &doc, &cfg, &mut bounded)),
        LowpolyCommand::SetFixtureJson(payload) => Some(set_fixture_json::handle(payload, &doc, &cfg, &mut bounded)),
        LowpolyCommand::SetActiveObject(payload) => Some(set_active_object::handle(payload, &doc, &cfg, &mut bounded)),
        LowpolyCommand::SetActivePaintLayer(payload) => Some(set_active_paint_layer::handle(payload, &doc, &cfg, &mut bounded)),
        LowpolyCommand::SetUtilityParam(payload) => Some(set_utility_param::handle(payload, &doc, &cfg, &mut bounded)),
        LowpolyCommand::EngagementInput(payload) => Some(engagement_input::handle(payload, &doc, &cfg, &mut bounded)),
        LowpolyCommand::ToggleShowEdges(payload) => Some(toggle_show_edges::handle(payload, &doc, &cfg, &mut bounded)),
        LowpolyCommand::ToggleSun(payload) => Some(toggle_sun::handle(payload, &doc, &cfg, &mut bounded)),
        LowpolyCommand::SetSunAzimuth(payload) => Some(set_sun_azimuth::handle(payload, &doc, &cfg, &mut bounded)),
        LowpolyCommand::SetSunElevation(payload) => Some(set_sun_elevation::handle(payload, &doc, &cfg, &mut bounded)),
        LowpolyCommand::SetSunIntensity(payload) => Some(set_sun_intensity::handle(payload, &doc, &cfg, &mut bounded)),
        LowpolyCommand::SetCamera(payload) => Some(set_camera::handle(payload, &doc, &cfg, &mut bounded)),
        LowpolyCommand::AddPaintLayer(payload) => Some(add_paint_layer::handle(payload, &doc, &cfg, &mut bounded)),
        LowpolyCommand::PaintSample(payload) => Some(paint_sample::handle(payload, &doc, &cfg, &mut bounded)),
        _ => None,
    };
    if let Some(emit) = direct {
        return emit.map(ArtifactCommandWorkStep::Complete);
    }
    let retained = match command {
        LowpolyCommand::PaintStrokeBegin(_) => Some((Emit::default(), context.transient.begin_stroke_drag())),
        LowpolyCommand::TransformBegin(_) => Some((Emit::default(), context.transient.begin_transform_drag())),
        LowpolyCommand::SetActiveUtility(payload) => Some((set_active_utility::handle(payload, &doc, &cfg, &mut bounded)?, context.transient.reset_gestures())),
        _ => None,
    };
    if let Some((emit, transient)) = retained {
        return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![LowpolyTransientMutation::Snapshot { transient }] } });
    }
    let active = resolve_active_object_id(snapshot, config);
    let empty_selection = protocol::DomainSelection::default();
    let selection = selection_from_state(&active, interaction.selection.get(MESH_INTERACTION_DOMAIN).unwrap_or(&empty_selection));
    let mut scratch = LowpolyScratch::from_transient(&context.transient, selection).map_err(Fault::from)?;
    let emit = match command {
        LowpolyCommand::AddPrimitive(payload) => add_primitive::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::Extrude(payload) => extrude::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::Inset(payload) => inset::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::Bevel(payload) => bevel::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::LoopCut(payload) => loop_cut::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::Subdivide(payload) => subdivide::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::Triangulate(payload) => triangulate::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::Mirror(payload) => mirror::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::Decimate(payload) => decimate::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::FlipFaces(payload) => flip_faces::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::Merge(payload) => merge::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::Dissolve(payload) => dissolve::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::Snap(payload) => snap::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::ToggleSmooth(payload) => toggle_smooth::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::UnwrapActive(payload) => unwrap_active::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::MarkUvSeam(payload) => mark_uv_seam::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::ClearSeam(payload) => clear_seam::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::TranslateSelection(payload) => translate_selection::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::RotateSelection(payload) => rotate_selection::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::ScaleSelection(payload) => scale_selection::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::PaintStrokeEnd(payload) => paint_stroke_end::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::PaintFill(payload) => paint_fill::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::FillBucket(payload) => fill_bucket::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::TransformEnd(payload) => transform_end::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::EngagementSubmit(payload) => engagement_submit::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::PaintStroke(payload) => paint_stroke::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::PaintAt(payload) => paint_at::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::CanvasPointerDown(payload) => canvas_pointer_down::handle(payload, &doc, &cfg, &mut scratch),
        LowpolyCommand::CanvasPointerMove(payload) => canvas_pointer_move::handle(payload, &doc, &cfg, &mut scratch),
        _ => unreachable!("direct Lowpoly command returned before session-owned reduction"),
    }?;
    let transient = scratch.transient_snapshot().map_err(Fault::from)?;
    Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![LowpolyTransientMutation::Snapshot { transient }] } })
}

fn lowpoly_tool_identity(tool_id: &str) -> u64 {
    tool_id.bytes().fold(0xcbf2_9ce4_8422_2325, |digest, byte| (digest ^ u64::from(byte)).wrapping_mul(0x1000_0000_01b3))
}

struct LowpolyRetainedCommandWork {
    tool_id: &'static str,
    disposition: LowpolyCommandDisposition,
    extent: usize,
    cursor: usize,
    digest: u64,
    replay_target: Option<(usize, u64)>,
    workspace: ArtifactOutputChunks,
    complete: bool,
    closing: bool,
}

impl LowpolyRetainedCommandWork {
    fn new(tool_id: &'static str, disposition: LowpolyCommandDisposition, extent: usize) -> Self {
        Self { tool_id, disposition, extent, cursor: 0, digest: 0xcbf2_9ce4_8422_2325, replay_target: None, workspace: ArtifactOutputChunks::new(extent.saturating_mul(LOWPOLY_SCAN_BYTES)), complete: false, closing: false }
    }

    fn observe(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.digest = (self.digest ^ u64::from(*byte)).wrapping_mul(0x1000_0000_01b3);
        }
    }
}

impl ArtifactCommandWork<EditorApp<LowpolyPlayApp>> for LowpolyRetainedCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn workspace_identity(&self) -> u64 {
        lowpoly_tool_identity(self.tool_id) ^ (self.extent as u64).rotate_left(17) ^ (u64::from(self.disposition as u8) << 56)
    }

    fn extent(&self, _command: &LowpolyCommand, snapshot: &LowpolySnapshot, interaction: &protocol::InteractionState, context: Option<&ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>>) -> Option<usize> {
        lowpoly_extent(snapshot, interaction, context?)
    }

    fn step(
        &mut self,
        command: &LowpolyCommand,
        snapshot: &LowpolySnapshot,
        config: &LowpolyConfig,
        history: &semio_framework_plugin::HistoryView,
        interaction: &protocol::InteractionState,
        _hover: &semio_framework_plugin::app::InteractionHoverState,
        context: Option<&ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>>,
        operation: &AppOperationContext,
    ) -> Result<ArtifactCommandWorkStep<EditorApp<LowpolyPlayApp>>, Fault> {
        if self.complete {
            return Err(Fault::from("lowpoly-retained-work-repeated"));
        }
        let context = context.ok_or_else(|| Fault::from("lowpoly-retained-context-absent"))?;
        let extent = lowpoly_extent(snapshot, interaction, context).ok_or_else(|| Fault::from("lowpoly-retained-work-extent-overflow"))?;
        if !lowpoly_extent_admitted(extent) || extent != self.extent || self.cursor > extent {
            return Err(Fault::from("lowpoly-retained-work-extent-drift"));
        }
        if self.cursor + 1 < extent {
            let transient_units = context.transient.segmented_extent(LOWPOLY_SCAN_BYTES).ok_or_else(|| Fault::from("lowpoly-retained-transient-extent-overflow"))?;
            let snapshot_units = lowpoly_snapshot_scan_units(snapshot).ok_or_else(|| Fault::from("lowpoly-retained-work-extent-overflow"))?;
            let bytes = if self.cursor < transient_units {
                context.transient.segment_at(self.cursor, LOWPOLY_SCAN_BYTES).unwrap_or_default()
            } else if self.cursor - transient_units < snapshot_units {
                lowpoly_snapshot_chunk(snapshot, self.cursor - transient_units).unwrap_or_default()
            } else {
                let cursor = self.cursor - transient_units - snapshot_units;
                interaction.selection.get(MESH_INTERACTION_DOMAIN).and_then(|selection| selection.ids.get(cursor)).map_or(&[][..], String::as_bytes)
            };
            let segment = if bytes.is_empty() { vec![0] } else { bytes.to_vec() };
            self.workspace.push(segment)?;
            self.observe(bytes);
            self.cursor += 1;
            if let Some((target, expected_digest)) = self.replay_target {
                if self.cursor == target {
                    if self.digest != expected_digest {
                        return Err(Fault::from("lowpoly-retained-workspace-replay-drift"));
                    }
                    self.replay_target = None;
                }
                return Ok(ArtifactCommandWorkStep::Replay { stage: "lowpoly-command-workspace-replay", preview: b"{\"en\":\"Restoring Lowpoly workspace\",\"de\":\"Lowpoly-Arbeitsbereich wird wiederhergestellt\"}" });
            }
            return Ok(ArtifactCommandWorkStep::Progress { stage: "lowpoly-command-scan", preview: b"{\"en\":\"Preparing Lowpoly command\",\"de\":\"Lowpoly-Befehl wird vorbereitet\"}" });
        }
        self.complete = true;
        lowpoly_retained_reduce(command, snapshot, config, history, interaction, context, operation)
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 40 {
            return Err(Fault::from("lowpoly-retained-checkpoint-capacity"));
        }
        target[..40].fill(0);
        target[..4].copy_from_slice(b"LPC1");
        target[4] = self.disposition as u8;
        target[5] = u8::from(self.complete);
        target[8..16].copy_from_slice(&(self.cursor as u64).to_le_bytes());
        target[16..24].copy_from_slice(&self.digest.to_le_bytes());
        target[24..32].copy_from_slice(&lowpoly_tool_identity(self.tool_id).to_le_bytes());
        target[32..40].copy_from_slice(&(self.extent as u64).to_le_bytes());
        Ok(40)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 40 || &checkpoint[..4] != b"LPC1" || checkpoint[4] != self.disposition as u8 || checkpoint[5] > 1 || checkpoint[6] != 0 || checkpoint[7] != 0 {
            return Err(Fault::from("lowpoly-retained-checkpoint-invalid"));
        }
        let tool = u64::from_le_bytes(checkpoint[24..32].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-tool"))?);
        let extent = u64::from_le_bytes(checkpoint[32..40].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-extent"))?);
        if tool != lowpoly_tool_identity(self.tool_id) || extent != self.extent as u64 {
            return Err(Fault::from("lowpoly-retained-checkpoint-identity-mismatch"));
        }
        let cursor = u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-cursor"))?);
        if cursor > self.extent as u64 {
            return Err(Fault::from("lowpoly-retained-checkpoint-cursor"));
        }
        let digest = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-digest"))?);
        if self.workspace.chunks_remaining() != 0 {
            return Err(Fault::from("lowpoly-retained-workspace-not-empty-before-restore"));
        }
        self.cursor = 0;
        self.digest = 0xcbf2_9ce4_8422_2325;
        self.replay_target = (cursor != 0).then_some((cursor as usize, digest));
        self.complete = checkpoint[5] == 1;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if self.workspace.chunks_remaining() == 0 {
            return InteractiveJobCloseStep::Complete;
        }
        if _maximum_items == 0 || _maximum_bytes < LOWPOLY_SCAN_BYTES {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        let mut released_items = 0;
        let mut released_bytes = 0;
        while released_items < _maximum_items && released_bytes.saturating_add(LOWPOLY_SCAN_BYTES) <= _maximum_bytes {
            let Some(chunk) = self.workspace.close_take_chunk().ok().flatten() else { break };
            released_items += 1;
            released_bytes += chunk.len();
        }
        if self.workspace.chunks_remaining() == 0 {
            InteractiveJobCloseStep::Complete
        } else {
            InteractiveJobCloseStep::Pending { released_items, released_bytes }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.workspace.chunks_remaining() == 0
    }
}

struct LowpolyCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl LowpolyCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: LOWPOLY_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for LowpolyCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<LowpolyPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<LowpolyPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        LOWPOLY_RETAINED_PAYLOAD_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        lowpoly_contract()
    }

    fn create_job(&mut self, _operation: semio_framework_job::Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> {
        Ok(ArtifactRetainedCommandJob::new(payload))
    }

    fn create_job_from_wire_pages_with_payload(
        &mut self,
        _operation: semio_framework_job::Operation,
        payload: Self::Payload,
        input: semio_framework::action_bus::RetainedToolWireInput,
        checkpoint: Option<semio_framework::action_bus::RetainedToolWireInput>,
    ) -> Result<Self::Job, (ToolJobFactoryError, semio_framework::action_bus::RetainedToolWireInput, Option<semio_framework::action_bus::RetainedToolWireInput>)> {
        if input.declared_bytes() > LOWPOLY_RETAINED_RAW_BYTES || checkpoint.as_ref().is_some_and(|checkpoint| checkpoint.declared_bytes() > semio_framework_plugin::retained_command::ARTIFACT_COMMAND_CHECKPOINT_MAXIMUM_BYTES) {
            return Err((ToolJobFactoryError::new("Lowpoly retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(match checkpoint {
            Some(checkpoint) => ArtifactRetainedCommandJob::from_wire_with_checkpoint(payload, input, checkpoint),
            None => ArtifactRetainedCommandJob::from_wire(payload, input),
        })
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for LowpolyCommandJobFactory {
    type Owner = semio_framework_plugin::EditorApp<LowpolyPlayApp>;
    const TOOL_IDS: &'static [&'static str] = LOWPOLY_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = LOWPOLY_DOCUMENT_SCHEMA;
}
//#endregion 🧵️RetainedCommands

fn lowpoly_export_media(port: &str, doc: &ArtifactView<'_, LowpolySnapshot>, scratch: &LowpolyScratch) -> Result<Media, MediaError> {
    match port {
        "mesh:out" => {
            let document_json = serde_json::to_value(doc.snapshot).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
            let mesh = crate::editor::lowpoly::engine::lowpoly_mesh_from_document(&document_json, &scratch.mesh_workspace_map()).map_err(|error| MediaError::Payload(port.into(), error))?;
            let mesh_document = crate::artifacts::lowpoly::schema::mesh_document_from_mesh(&mesh).map_err(|error| MediaError::Payload(port.into(), error))?;
            let json = serde_json::to_string(&mesh_document).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
            Ok(Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: MediaPayload::Structured { schema: "mesh.document".into(), json } })
        }
        "document:out" => {
            let media_type = LowpolyPlayApp::io().map_or(MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, |io| io.document_media_type);
            let bytes = doc.snapshot.encode_pack();
            Ok(Media { media_type, payload: MediaPayload::Structured { schema: LOWPOLY_DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
        }
        _ => Err(MediaError::NotImplemented),
    }
}

fn lowpoly_render(body_key: &str, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, scratch: &mut LowpolyScratch) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let projection = doc.snapshot;
    let config = cfg.snapshot;
    let labels = crate::editor::lowpoly::terminology::lowpoly_play_labels(config);
    let active_utility = config.active_utility_id.as_str();
    if matches!(body_key, LOWPOLY_PLAY_BODY_MAIN | LOWPOLY_PLAY_BODY_UV) {
        scratch.refresh_texture_cache(projection);
    }
    let scratch_projection = scratch.transform_projection();
    let texture_cache = scratch.textures().clone();
    let render_projection = scratch_projection.as_ref().unwrap_or(projection);
    let view = LowpolyView { snapshot: render_projection, config };
    let loaded = matches!(body_key, LOWPOLY_PLAY_BODY_MAIN | LOWPOLY_PLAY_BODY_UV | LOWPOLY_PLAY_BODY_DOCUMENT).then(|| crate::editor::lowpoly::view::build_doc(projection, config, scratch)).flatten();
    match body_key {
        LOWPOLY_PLAY_BODY_MAIN => edit::windows::model::render(view, loaded.as_ref(), active_utility, &texture_cache),
        LOWPOLY_PLAY_BODY_UV => paint_mode::windows::uv::render(view, loaded.as_ref(), &texture_cache),
        LOWPOLY_PLAY_BODY_DOCUMENT => match &loaded {
            Some(loaded) => document_panel::render(view, loaded, labels),
            None => semio_framework_plugin::ui_text(semio_framework_plugin::Label::data("Failed to load lowpoly document")),
        },
        LOWPOLY_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels),
        LOWPOLY_PLAY_BODY_INSPECTION => inspection_panel::render(view, active_utility, labels),
        LOWPOLY_PLAY_BODY_LAYERS => layers_panel::render(view, labels),
        _ => semio_framework_plugin::ui_text(semio_framework_plugin::Label::data(format!("Unknown body: {body_key}"))),
    }
}

//#region 🔖️LowpolyPlayApp
/// @emoji 🖌️ B1: sheds `RefCell<LowpolyPlayRuntime>` entirely — every former runtime field now lives in
/// `LowpolyConfig`, written through `LowpolyConfigMutation`s emitted from `handle`. The one remaining
/// field is genuine mid-gesture scratch state (`LowpolyScratch`) — the "scratch + commit" pattern the
/// `ArtifactEditor` trait itself sanctions for `&self`-only `handle`/`render`.
#[derive(Default, Clone, Copy)]
pub struct LowpolyPlayApp;

impl ArtifactEditor for LowpolyPlayApp {
    type Snapshot = LowpolySnapshot;
    type Mutation = LowpolyMutation;
    type Config = LowpolyConfig;
    type ConfigMutation = LowpolyConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::lowpoly::presence::LowpolyPresence;
    type PresenceMutation = crate::editor::lowpoly::presence::LowpolyPresenceMutation;
    type Transient = LowpolyTransient;
    type TransientMutation = LowpolyTransientMutation;

    type Command = LowpolyCommand;

    const DIALECT: semio_framework_plugin::app::Dialect = crate::artifacts::lowpoly::LOWPOLY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = LOWPOLY_DOCUMENT_SCHEMA;

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<LowpolyPlayApp>,
        owner_file: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs",
        controller: "s.lowpoly.lowpoly@1/*#editor",
        document_schema: "lowpoly.document",
        factory: "LowpolyCommandJobFactory",
        tools: {
            "addPrimitive" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "patchObject" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "extrude" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "inset" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "bevel" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "loopCut" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "subdivide" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "triangulate" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "mirror" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "decimate" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "flipFaces" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "merge" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "dissolve" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "snap" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "toggleSmooth" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "unwrapActive" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "markUvSeam" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "clearSeam" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "translateSelection" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "rotateSelection" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "scaleSelection" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "addPaintLayer" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "paintStrokeEnd" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "paintFill" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "fillBucket" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "transformEnd" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "importSnapshotJson" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "setFixtureJson" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "engagementSubmit" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "setActiveObject" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "setActivePaintLayer" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "setUtilityParam" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "engagementInput" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "toggleShowEdges" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "toggleSun" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "setSunAzimuth" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "setSunElevation" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "setSunIntensity" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "setCamera" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "paintStrokeBegin" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "paintSample" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "paintStroke" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "paintAt" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "canvasPointerDown" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "canvasPointerMove" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "transformBegin" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
            "setActiveUtility" => semio_framework::ToolExecutionContract::resumable(1_048_576, 1_024, 1, 33_554_432, 7_500, 1, 1),
        }
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(LowpolyCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        let Some(disposition) = lowpoly_command_disposition(&request.tool_id) else {
            return Ok(None);
        };
        if request.command.command_id() != request.tool_id {
            return Err(Fault::from("lowpoly-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let extent = lowpoly_extent(&request.snapshot, &request.interaction_state, &request.context).ok_or_else(|| Fault::from("lowpoly-retained-work-extent-overflow"))?;
        if !lowpoly_extent_admitted(extent) {
            return Err(Fault::from("lowpoly-retained-work-extent-capacity"));
        }
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(LowpolyRetainedCommandWork::new(tool_id, disposition, extent));
        let operation_context = AppOperationContext {
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
        };
        let payload = ArtifactRetainedCommandPayload::try_new_with_context(
            *request.command,
            request.snapshot,
            request.config,
            request.history,
            request.interaction_state,
            request.interaction_hover,
            request.context,
            operation_context,
            request.completion,
            LowpolyCommand::command_id,
            LOWPOLY_RETAINED_RAW_BYTES,
            LOWPOLY_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::lowpoly::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> LowpolySnapshot {
        crate::artifacts::lowpoly::schema::default_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(lowpoly_io())
    }

    /// 🧬️ No `whole_document_operation` override — per `📓️taxonomy.md`, whole-document replace
    /// (the retired whole-document-replace variant) is banned outright with NO replacement mutation, so this falls back to the
    /// trait's own default (`None`); `import_media`'s `"mesh:in"`/`"document:in"` arms below build
    /// `reset_document_effect` (a `Effect::LoadDocument`, outside undo history) instead.
    ///
    /// 🎞️ `mesh:out` plus the inherited `document:out` default (the pack of `doc.snapshot`, replicated
    /// inline — overriding `export_media` shadows the trait's provided body for every port on this app,
    /// not just the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, LowpolySnapshot>) -> Result<Media, MediaError> {
        lowpoly_export_media(port, doc, &LowpolyScratch::default())
    }

    fn export_media_with_request_context(port: &str, doc: &ArtifactView<'_, LowpolySnapshot>, transient: &semio_framework_plugin::TransientView<'_, LowpolyTransient>) -> Result<Media, MediaError> {
        let scratch = LowpolyScratch::from_transient(transient.snapshot, crate::artifacts::lowpoly::LowpolySelection::default()).map_err(|error| MediaError::Payload(port.into(), error))?;
        lowpoly_export_media(port, doc, &scratch)
    }

    /// 🎞️ `mesh:in` round-trips a `mesh.document` payload into a `reset_document_effect`; `document:in`
    /// replicates the trait's default whole-pack import inline (overriding `import_media` shadows the
    /// default for every port on this app, not just the new one).
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, LowpolySnapshot>) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation, Self::DraftMutation>, MediaError> {
        match port {
            "mesh:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.into(), "mesh:in importer only accepts a Structured payload".into()));
                };
                let mesh_document: Value = serde_json::from_str(json).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
                let mesh = crate::artifacts::lowpoly::schema::mesh_from_mesh_document(&mesh_document).map_err(|error| MediaError::Payload(port.into(), error))?;
                let projection_json = crate::artifacts::lowpoly::schema::lowpoly_document_from_mesh(&mesh).map_err(|error| MediaError::Payload(port.into(), error))?;
                let snapshot: LowpolySnapshot = serde_json::from_value(projection_json).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
                Ok(Emit { effects: vec![reset_document_effect(&snapshot)], ..Default::default() })
            }
            "document:in" => {
                let MediaPayload::Structured { json, .. } = &media.payload else {
                    return Err(MediaError::Payload(port.into(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
                };
                let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
                let projection = <LowpolySnapshot as ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
                Ok(Emit { effects: vec![reset_document_effect(&projection)], ..Default::default() })
            }
            _ => Err(MediaError::NotImplemented),
        }
    }

    fn command_id(command: &LowpolyCommand) -> &'static str {
        command.command_id()
    }

    /// 🕹️ `interaction` is the mesh domain's current selection/hover/mode/granularity, resolved once per
    /// dispatch into `LowpolyScratch::current_selection` — the `app_commands!`-generated `dispatch` calls
    /// every leaf `🎮️commands/*::handle(payload, doc, cfg, ctx)` uniformly (no `interaction` parameter
    /// of its own), so this is the one seam by which those handlers (via `view::build_doc`/
    /// `session::mesh_edit`) see the framework-owned selection. See `🧭️view/🦀️component.rs`'s
    /// `🔖️MeshDomain` region for the id scheme.
    fn handle(
        command: &LowpolyCommand,
        doc: &ArtifactView<'_, LowpolySnapshot>,
        cfg: &ConfigView<'_, LowpolyConfig>,
        interaction: &InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation, Self::DraftMutation>, Fault> {
        let active = resolve_active_object_id(doc.snapshot, cfg.snapshot);
        let selection = selection_from_interaction(&active, interaction);
        let mut scratch = LowpolyScratch::from_transient(&LowpolyTransient::default(), selection).map_err(Fault::from)?;
        command.dispatch(doc, cfg, &mut scratch)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        lowpoly_render(body_key, doc, cfg, &mut LowpolyScratch::default())
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, LowpolySnapshot>,
        cfg: &ConfigView<'_, LowpolyConfig>,
        transient: &semio_framework_plugin::TransientView<'_, LowpolyTransient>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let mut scratch = LowpolyScratch::from_transient(transient.snapshot, crate::artifacts::lowpoly::LowpolySelection::default()).map_err(|error| semio_framework_plugin::PluginAssemblyError::new("lowpoly.transient", error))?;
        lowpoly_render(body_key, doc, cfg, &mut scratch)
    }

    fn window_engagements(doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>) -> HashMap<String, WindowEngagement> {
        let config = cfg.snapshot;
        let active_utility = config.active_utility_id.as_str();
        let labels = crate::editor::lowpoly::terminology::lowpoly_play_labels(config);
        let engagement = lowpoly_window_engagement(LowpolyView { snapshot: doc.snapshot, config }, active_utility, labels);
        HashMap::from([(edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN.into(), engagement.clone()), (paint_mode::windows::uv::LOWPOLY_PLAY_WINDOW_UV.into(), engagement)])
    }

    fn window_measures(_doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        let labels = crate::editor::lowpoly::terminology::lowpoly_play_labels(config);
        let measures = lowpoly_window_measures(config, labels);
        HashMap::from([(edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN.into(), measures.clone()), (paint_mode::windows::uv::LOWPOLY_PLAY_WINDOW_UV.into(), measures)])
    }
}
//#endregion 🔖️LowpolyPlayApp

//#region 🔖️ResetDocument
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `scene` OUTSIDE undo
/// history — the sanctioned non-mutation path for a whole-document replace (mesh import, file
/// open, dev fixture load). Per `📓️taxonomy.md`, whole-document replace is banned outright with NO
/// replacement mutation: whole-document replace is not expressible as an in-history `Mutation` at
/// all. Every former "replace the whole document" gesture in this package (`import_media`'s
/// `"mesh:in"`/`"document:in"` above, `commands::fixture::{set_snapshot_json,set_fixture_json}`)
/// builds this effect instead of an `Emit::mutations([...])`. The spr is a fresh, edit-free op-log
/// for `scene` — a genesis envelope with no history to encode.
pub fn reset_document_effect(scene: &LowpolySnapshot) -> semio_framework_plugin::Effect {
    let pack = <LowpolySnapshot as ArtifactPack>::encode_pack(scene);
    let envelope = store::create_document_envelope::<LowpolySnapshot, LowpolyMutation>(LOWPOLY_DOCUMENT_SCHEMA, "lowpoly", scene.clone(), None);
    let spr = store::print_document_spr(&envelope).expect("lowpoly document spr encode is infallible for a fresh, edit-free envelope");
    semio_framework_plugin::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️ResetDocument

//#region 🔖️Manifest
/// 🧰️ One transform/paint utility declaration (id/label/icon reused verbatim from the retired
/// `utilities()` impl).
fn lowpoly_utility(id: &str, label: impl Into<LocalizedLabel>, icon: &str, group: &str) -> UtilityDefinition {
    UtilityDefinition { group: Some(group.into()), category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new(id, label, icon) }
}

/// 🧱️ The manifest stitch: one call per taxonomy node, each sourced from that node's own `definition()`.
/// Only the leaf action/keybinding/utility declarations (which have no dedicated `_def` passthrough)
/// stay written out inline.
///
/// 🚧️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.4: `EditorBuilder` has no
/// `.example(...)`/`.workflow(...)` methods (`App { definition, examples }` split — `.editor::<E>(def)`
/// only takes the definition, examples always end up empty). The old
/// `.example("default", …, &default_example, "file")` / `.workflow("lowpoly", "Lowpoly", "mesh")` tail
/// calls this app used to make are DROPPED here, not ported — the subset's own `📚️examples/🎬️demo`
/// facet is the intended replacement mechanism per the pilot's report, not confirmed with the
/// coordinator by this packet.
pub fn create_lowpoly_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::lowpoly::LOWPOLY_DIALECT)
            .document(["semio", "lowpoly"])
            .artifact_kind(artifact_kind())
            .icon_id("shapes")
            .mode_def(edit::definition())
            .mode_def(paint_mode::definition())
            .default_mode_id(edit::LOWPOLY_PLAY_MODE_EDIT)
            .window_kind_def(edit::windows::model::definition())
            .window_kind_def(paint_mode::windows::uv::definition())
            .window_kind_action_refs(
                edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN,
                edit::windows::model::LOWPOLY_MAIN_ACTIONS.iter().map(|id| ActionRef::from(*id)).collect(),
            )
            .window_kind_action_refs(
                paint_mode::windows::uv::LOWPOLY_PLAY_WINDOW_UV,
                paint_mode::windows::uv::LOWPOLY_UV_ACTIONS.iter().map(|id| ActionRef::from(*id)).collect(),
            )
            .default_layout(edit::layout())
            .named_layout(paint_mode::layout())
            .panel_tab_def(document_panel::definition())
            .panel_tab_def(catalogue_panel::definition())
            .panel_tab_def(inspection_panel::definition())
            .panel_tab_def(layers_panel::definition())
            // 🔧️ Document-mutating operations — dispatched as VCS operations with true inverses.
            .mutation("addPrimitive", LocalizedLabel::native("Add Primitive", "Primitive hinzufügen"))
            .mutation("patchObject", LocalizedLabel::native("Patch Object", "Objekt aktualisieren"))
            .mutation("extrude", LocalizedLabel::native("Extrude", "Extrudieren"))
            .mutation("inset", LocalizedLabel::native("Inset", "Einziehen"))
            .mutation("bevel", LocalizedLabel::native("Bevel", "Fasen"))
            .mutation("loopCut", LocalizedLabel::native("Loop Cut", "Schleifenschnitt"))
            .mutation("subdivide", LocalizedLabel::native("Subdivide", "Unterteilen"))
            .mutation("triangulate", LocalizedLabel::native("Triangulate", "Triangulieren"))
            .mutation("mirror", LocalizedLabel::native("Mirror", "Spiegeln"))
            .mutation("decimate", LocalizedLabel::native("Decimate", "Dezimieren"))
            .mutation("flipFaces", LocalizedLabel::native("Flip Faces", "Flächen umkehren"))
            .mutation("merge", LocalizedLabel::native("Merge", "Zusammenführen"))
            .mutation("dissolve", LocalizedLabel::native("Dissolve", "Auflösen"))
            .mutation("snap", LocalizedLabel::native("Snap", "Einrasten"))
            .mutation("toggleSmooth", LocalizedLabel::native("Toggle Smooth", "Glättung umschalten"))
            .mutation("unwrapActive", LocalizedLabel::native("Unwrap", "Abwickeln"))
            .mutation("markUvSeam", LocalizedLabel::native("Mark Seam", "Naht markieren"))
            .mutation("clearSeam", LocalizedLabel::native("Clear Seam", "Naht entfernen"))
            .mutation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .mutation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .mutation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .mutation("transformEnd", LocalizedLabel::native("Transform End", "Transformation beenden"))
            .mutation("addPaintLayer", LocalizedLabel::native("Add Paint Layer", "Malebene hinzufügen"))
            .mutation("paintStrokeEnd", LocalizedLabel::native("Paint Stroke End", "Malstrich beenden"))
            .mutation("paintFill", LocalizedLabel::native("Paint Fill", "Füllen malen"))
            .mutation("fillBucket", LocalizedLabel::native("Fill Bucket", "Fülleimer"))
            .mutation("importSnapshotJson", LocalizedLabel::native("Import Snapshot Json", "Snapshot-JSON importieren"))
            .mutation("setFixtureJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"))
            .mutation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"))
            // 👁️ Ephemeral view state — selection, camera, hover, and the gesture drafts that emit no operations
            // mid-drag (paint ticks, gumball scratch, eyedropper sample).
            .view_action("setActiveObject", LocalizedLabel::native("Set Active Object", "Aktives Objekt festlegen"))
            .view_action("setActivePaintLayer", LocalizedLabel::native("Set Active Paint Layer", "Aktive Malebene festlegen"))
            .view_action("setUtilityParam", LocalizedLabel::native("Set Utility Param", "Werkzeugparameter festlegen"))
            .view_action("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"))
            .view_action("toggleShowEdges", LocalizedLabel::native("Toggle Show Edges", "Kantenanzeige umschalten"))
            .view_action("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"))
            .view_action("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"))
            .view_action("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"))
            .view_action("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"))
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .view_action("paintStrokeBegin", LocalizedLabel::native("Paint Stroke Begin", "Malstrich beginnen"))
            .view_action("paintStroke", LocalizedLabel::native("Paint Stroke", "Malstrich"))
            .view_action("paintAt", LocalizedLabel::native("Paint At", "Malen bei"))
            .view_action("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"))
            .view_action("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegt"))
            .view_action("paintSample", LocalizedLabel::native("Paint Sample", "Farbe aufnehmen"))
            .view_action("transformBegin", LocalizedLabel::native("Transform Begin", "Transformation beginnen"))
            // 📝️ Staged argument forms for the P1 actions — the panel form seeds from these defaults and
            // stages typed overrides read out of `args`; `config.utility_params_json` remains the live backing store.
            .action_args("extrude", vec![ActionArgDef::slider("extrudeDistance", LocalizedLabel::native("Extrude Distance", "Extrusionsabstand"), 0.01, 2.0).default_value(0.25)])
            .action_args("inset", vec![ActionArgDef::number("insetAmount", LocalizedLabel::native("Inset Amount", "Einzugsbetrag")).default_value(0.1)])
            .action_args("bevel", vec![
                ActionArgDef::number("bevelAmount", LocalizedLabel::native("Bevel Amount", "Fasenbetrag")).default_value(0.05),
                ActionArgDef::number("bevelSegments", LocalizedLabel::native("Bevel Segments", "Fasensegmente")).default_value(1),
            ])
            .action_args("loopCut", vec![ActionArgDef::number("loopCuts", LocalizedLabel::native("Loop Cuts", "Schleifenschnitte")).default_value(1)])
            .action_args("decimate", vec![ActionArgDef::slider("decimateRatio", LocalizedLabel::native("Decimate Ratio", "Dezimierungsverhältnis"), 0.05, 1.0).default_value(0.5)])
            .action_args("mirror", vec![ActionArgDef::select("axis", LocalizedLabel::native("Axis", "Achse"), vec![
                ActionArgOption::new("x", LocalizedLabel::native("X", "X")),
                ActionArgOption::new("y", LocalizedLabel::native("Y", "Y")),
                ActionArgOption::new("z", LocalizedLabel::native("Z", "Z")),
            ]).default_value("x")])
            .action_args("addPrimitive", vec![ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                ActionArgOption::new("box", LocalizedLabel::native("Cube", "Würfel")),
                ActionArgOption::new("plane", LocalizedLabel::native("Plane", "Ebene")),
                ActionArgOption::new("cylinder", LocalizedLabel::native("Cylinder", "Zylinder")),
                ActionArgOption::new("cone", LocalizedLabel::native("Cone", "Kegel")),
                ActionArgOption::new("ico_sphere", LocalizedLabel::native("Ico Sphere", "Ikokugel")),
            ]).default_value("box")])
            .action_args("markUvSeam", vec![ActionArgDef::toggle("seam", LocalizedLabel::native("Seam", "Naht")).default_value(true)])
            // 🧰️ Transform gumball + paint utilities — exclusive per-window active utility is host-owned (never a
            // document operation). Selection method/merge/kind live as an always-visible Select window-options group
            // (mirrors puzzle 3d); the transform group defaults to "move", paint bridges into `config.paint_utility`.
            .utility(lowpoly_utility("move", LocalizedLabel::native("Move", "Verschieben"), "move", "transform"))
            .utility(lowpoly_utility("rotate", LocalizedLabel::native("Rotate", "Drehen"), "rotate-cw", "transform"))
            .utility(lowpoly_utility("scale", LocalizedLabel::native("Scale", "Skalieren"), "maximize-2", "transform"))
            .utility(lowpoly_utility("brush", LocalizedLabel::native("Brush", "Pinsel"), "paintbrush", "paint"))
            .utility(lowpoly_utility("eraser", LocalizedLabel::native("Eraser", "Radierer"), "eraser", "paint"))
            .utility(lowpoly_utility("fill", LocalizedLabel::native("Fill", "Füllen"), "paint-bucket", "paint"))
            .utility(lowpoly_utility("eyedropper", LocalizedLabel::native("Eyedropper", "Pipette"), "pipette", "paint"))
            // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "mesh" interaction domain —
            // object/vertex/edge/face granularities (u32 component ids stringify at the `InteractionTarget`
            // boundary, see `🧭️view/🦀️component.rs`'s `🔖️MeshDomain` region), Flat hierarchy (a mesh has no
            // parent/child structure of its own to derive a topology from), all five merges + all three
            // pick methods per the migration's acceptance bar. Scoped to the Model window only — the UV
            // window paints textures, it never selects mesh components.
            .interaction(InteractionDefinition {
                id: MESH_INTERACTION_DOMAIN.into(),
                label: LocalizedLabel::native("Mesh", "Netz"),
                granularities: vec![
                    GranularityDefinition { id: "object".into(), label: LocalizedLabel::native("Object", "Objekt"), icon_id: "box".into() },
                    GranularityDefinition { id: "vertex".into(), label: LocalizedLabel::native("Vertex", "Eckpunkt"), icon_id: "circle".into() },
                    GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
                    GranularityDefinition { id: "face".into(), label: LocalizedLabel::native("Face", "Fläche"), icon_id: "square".into() },
                ],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec {
                    modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                    methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle, SelectionMethod::Lasso],
                    merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                    transitive: false,
                    broadcast: true,
                },
            })
            .keybinding("mod+z", "undo")
            .keybinding("mod+shift+z", "redo")
            .config(LowpolyPlayApp::config_spec())
            .io(lowpoly_io())
            .action_interactive_job("addPrimitive", InteractiveJobClassification::Migrated)
            .action_interactive_job("patchObject", InteractiveJobClassification::Migrated)
            .action_interactive_job("extrude", InteractiveJobClassification::Migrated)
            .action_interactive_job("inset", InteractiveJobClassification::Migrated)
            .action_interactive_job("bevel", InteractiveJobClassification::Migrated)
            .action_interactive_job("loopCut", InteractiveJobClassification::Migrated)
            .action_interactive_job("subdivide", InteractiveJobClassification::Migrated)
            .action_interactive_job("triangulate", InteractiveJobClassification::Migrated)
            .action_interactive_job("mirror", InteractiveJobClassification::Migrated)
            .action_interactive_job("decimate", InteractiveJobClassification::Migrated)
            .action_interactive_job("flipFaces", InteractiveJobClassification::Migrated)
            .action_interactive_job("merge", InteractiveJobClassification::Migrated)
            .action_interactive_job("dissolve", InteractiveJobClassification::Migrated)
            .action_interactive_job("snap", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSmooth", InteractiveJobClassification::Migrated)
            .action_interactive_job("unwrapActive", InteractiveJobClassification::Migrated)
            .action_interactive_job("markUvSeam", InteractiveJobClassification::Migrated)
            .action_interactive_job("clearSeam", InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("addPaintLayer", InteractiveJobClassification::Migrated)
            .action_interactive_job("paintStrokeEnd", InteractiveJobClassification::Migrated)
            .action_interactive_job("paintFill", InteractiveJobClassification::Migrated)
            .action_interactive_job("fillBucket", InteractiveJobClassification::Migrated)
            .action_interactive_job("transformEnd", InteractiveJobClassification::Migrated)
            .action_interactive_job("importSnapshotJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("setFixtureJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveObject", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActivePaintLayer", InteractiveJobClassification::Migrated)
            .action_interactive_job("setUtilityParam", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleShowEdges", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSun", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunAzimuth", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunElevation", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunIntensity", InteractiveJobClassification::Migrated)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("paintStrokeBegin", InteractiveJobClassification::Migrated)
            .action_interactive_job("paintSample", InteractiveJobClassification::Migrated)
            .action_interactive_job("paintStroke", InteractiveJobClassification::Migrated)
            .action_interactive_job("paintAt", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerMove", InteractiveJobClassification::Migrated)
            .action_interactive_job("transformBegin", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveUtility", InteractiveJobClassification::Migrated)
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️Testkit
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
pub(crate) mod testkit {
    use super::*;
    use semio_framework_plugin::testkit::{meta, new_app, new_app_with_registry};
    use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

    pub type LowpolyApp = VcsArtifactApp<EditorApp<LowpolyPlayApp>>;

    /// 🧪️ `new_app_with_registry`/`assert_declared_actions_bridge_to_commands` (framework testkit,
    /// unchanged for this ticket) still take `fn() -> App` — `create_lowpoly_app` now returns
    /// `AppDefinition` (contract §2.4). This tiny local wrapper is the documented bridge (pilot report
    /// `📓️w2-cad-report.md` recipe step 7), not a framework fix owed by this packet.
    fn lowpoly_manifest_for_testkit() -> App {
        App { definition: create_lowpoly_app(), examples: Vec::new() }
    }

    /// 🧪️ A bare app instance — no `AppActionRegistry`, so undeclared internal commands dispatch freely.
    pub fn app() -> LowpolyApp {
        new_app::<EditorApp<LowpolyPlayApp>>()
    }

    /// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline.
    pub fn app_with_registry() -> LowpolyApp {
        new_app_with_registry::<EditorApp<LowpolyPlayApp>>(lowpoly_manifest_for_testkit)
    }

    pub async fn dispatch(app: &mut LowpolyApp, command: LowpolyCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
    }

    pub async fn render(app: &mut LowpolyApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).await.expect("render")).expect("render json")
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking is now the framework's
    /// injected `interactionSelect` verb, dispatched against the "mesh" domain declared on this app —
    /// requires `app_with_registry()` (a bare `app()` has no declared interaction domains to select
    /// against). `object_id`/`face_id` address the same row id the Document panel tree renders (see
    /// `🧭️view/🦀️component.rs`'s `🔖️MeshDomain` region).
    pub async fn select_face(app: &mut LowpolyApp, object_id: &str, face_id: u32) {
        let target_id = crate::editor::lowpoly::view::document_target_row_id(object_id, 0, "face", face_id);
        let targets = serde_json::to_string(&serde_json::json!([{ "granularity": "face", "id": target_id }])).expect("targets json");
        app.handle_action("interactionSelect", Some(&serde_json::json!({ "domainId": crate::editor::lowpoly::view::MESH_INTERACTION_DOMAIN, "targets": targets, "merge": "replace" })), &meta("test")).await.expect("interactionSelect");
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::lowpoly::testkit::{app, app_with_registry, LowpolyApp};
    use semio_framework_plugin::{testkit, EditorApp, PluginApp};

    fn retained_operation() -> AppOperationContext {
        AppOperationContext { app_instance_id: 7, parent_document_id: "lowpoly-retained-test".into(), operation_id: 11, generation: 13, canonical_base_revision: [17; 32] }
    }

    fn retained_context(transient: LowpolyTransient, transient_generation: u64) -> std::sync::Arc<ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>> {
        std::sync::Arc::new(ArtifactOwnedToolJobContext::new(7, [17; 32], 0, transient_generation, std::sync::Arc::new(semio_framework_plugin::ChildContentView::EMPTY), std::sync::Arc::new(NoDraft::default()), std::sync::Arc::new(transient)))
    }

    fn completed_transient(step: ArtifactCommandWorkStep<EditorApp<LowpolyPlayApp>>) -> LowpolyTransient {
        let ArtifactCommandWorkStep::CompleteWithEphemeral { ephemeral, .. } = step else { panic!("Lowpoly retained reducer must publish its typed transient lane") };
        let [LowpolyTransientMutation::Snapshot { transient }] = ephemeral.transient.as_slice() else { panic!("one exact transient snapshot") };
        transient.clone()
    }

    #[test]
    fn retained_route_dispositions_are_exact_exhaustive_and_cancellable() {
        use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};

        let mut ids = LOWPOLY_TOOL_IDS.to_vec();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), 47);
        assert!(ids.iter().all(|tool_id| lowpoly_command_disposition(tool_id).is_some()));
        assert_eq!(<LowpolyPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 47);
        assert_eq!(lowpoly_contract().shape, ToolExecutionShape::Resumable);
        assert_eq!(lowpoly_contract().cancellation, ToolCancellationPolicy::PerOperation);
        assert_eq!((lowpoly_contract().checkpoint_every_steps, lowpoly_contract().progress_every_steps), (1, 1));
    }

    #[test]
    fn retained_extent_accepts_exact_maximum_and_rejects_max_plus_one() {
        let mut snapshot = crate::artifacts::lowpoly::schema::default_snapshot();
        snapshot.objects[0].paint_layers.push(crate::artifacts::lowpoly::LowpolyPaintLayer::new("Maximum A"));
        snapshot.objects[0].paint_layers.push(crate::artifacts::lowpoly::LowpolyPaintLayer::new("Maximum B"));
        let interaction = protocol::InteractionState::default();
        let fixed_context = retained_context(LowpolyTransient::with_test_workspace_bytes(0), 1);
        let fixed = lowpoly_extent(&snapshot, &interaction, &fixed_context).expect("fixed extent");
        let padding_units = LOWPOLY_RETAINED_WORK_ITEMS.checked_sub(fixed - 1).expect("fixture capacity");
        let maximum_context = retained_context(LowpolyTransient::with_test_workspace_bytes(padding_units * LOWPOLY_SCAN_BYTES), 1);
        let maximum_plus_one_context = retained_context(LowpolyTransient::with_test_workspace_bytes(padding_units * LOWPOLY_SCAN_BYTES + 1), 1);
        let maximum = lowpoly_extent(&snapshot, &interaction, &maximum_context).expect("maximum extent");
        let maximum_plus_one = maximum_plus_one_context
            .transient
            .segmented_extent(LOWPOLY_SCAN_BYTES)
            .expect("maximum plus one transient extent")
            .checked_add(lowpoly_snapshot_scan_units(&snapshot).expect("maximum snapshot extent"))
            .and_then(|extent| extent.checked_add(1))
            .expect("maximum plus one extent");
        assert_eq!(maximum, LOWPOLY_RETAINED_WORK_ITEMS);
        assert_eq!(maximum_plus_one, LOWPOLY_RETAINED_WORK_ITEMS + 1);
        assert!(lowpoly_extent_admitted(maximum));
        assert!(!lowpoly_extent_admitted(maximum_plus_one));
        assert_eq!(lowpoly_extent(&snapshot, &interaction, &maximum_plus_one_context), Some(maximum_plus_one));
        let mesh_maximum_plus_one = retained_context(LowpolyTransient::with_test_workspace_bytes(LOWPOLY_RETAINED_MESH_BYTES + 1), 1);
        assert!(lowpoly_extent(&crate::artifacts::lowpoly::schema::default_snapshot(), &interaction, &mesh_maximum_plus_one).is_none());
        let mut paint_maximum_plus_one = crate::artifacts::lowpoly::schema::default_snapshot();
        paint_maximum_plus_one.objects[0].paint_layers[0].pixels.push(0);
        assert!(lowpoly_extent(&paint_maximum_plus_one, &interaction, &retained_context(LowpolyTransient::default(), 1)).is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_context_interruption_replay_identity_and_close_are_exact() {
        let command = LowpolyCommand::PaintStrokeBegin(paint_stroke_begin::PaintStrokeBegin {});
        let snapshot = crate::artifacts::lowpoly::schema::default_snapshot();
        let config = LowpolyConfig::default();
        let interaction = protocol::InteractionState::default();
        let hover = semio_framework_plugin::app::InteractionHoverState::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let operation = retained_operation();
        let context = retained_context(LowpolyTransient::default(), 19);
        let drifted_context = retained_context(LowpolyTransient::default(), 20);
        assert_ne!(context.identity_digest(), drifted_context.identity_digest());
        let extent = lowpoly_extent(&snapshot, &interaction, &context).expect("retained extent");
        let mut uninterrupted = LowpolyRetainedCommandWork::new("paintStrokeBegin", LowpolyCommandDisposition::Paint, extent);
        assert!(matches!(uninterrupted.step(&command, &snapshot, &config, &history, &interaction, &hover, Some(&context), &operation).expect("checkpoint prefix"), ArtifactCommandWorkStep::Progress { .. }));
        let mut checkpoint = [0_u8; 40];
        uninterrupted.checkpoint(&mut checkpoint).expect("work checkpoint");
        let mut wrong_tool = LowpolyRetainedCommandWork::new("paintStroke", LowpolyCommandDisposition::Paint, extent);
        assert!(wrong_tool.restore(&checkpoint).is_err());
        let mut cancelled = LowpolyRetainedCommandWork::new("paintStrokeBegin", LowpolyCommandDisposition::Paint, extent);
        assert!(matches!(cancelled.step(&command, &snapshot, &config, &history, &interaction, &hover, Some(&context), &operation).expect("cancel prefix"), ArtifactCommandWorkStep::Progress { .. }));
        cancelled.begin_close();
        while !cancelled.terminal_is_empty() {
            let _ = cancelled.close_step(1, LOWPOLY_SCAN_BYTES);
        }
        assert!(cancelled.terminal_is_empty());
        let mut replayed = LowpolyRetainedCommandWork::new("paintStrokeBegin", LowpolyCommandDisposition::Paint, extent);
        replayed.restore(&checkpoint).expect("work restore");
        assert_eq!(replayed.replay_target, Some((uninterrupted.cursor, uninterrupted.digest)));
        let drive = |work: &mut LowpolyRetainedCommandWork| loop {
            match work.step(&command, &snapshot, &config, &history, &interaction, &hover, Some(&context), &operation).expect("retained step") {
                ArtifactCommandWorkStep::Replay { .. } | ArtifactCommandWorkStep::Progress { .. } => {}
                complete => break completed_transient(complete),
            }
        };
        let uninterrupted_transient = drive(&mut uninterrupted);
        let replayed_transient = drive(&mut replayed);
        assert_eq!(uninterrupted_transient, replayed_transient);
        let scratch = LowpolyScratch::from_transient(&replayed_transient, crate::artifacts::lowpoly::LowpolySelection::default()).expect("typed replay transient");
        assert!(scratch.stroke_drag_active());
        assert_eq!(replayed.close_step(0, 0), InteractiveJobCloseStep::Blocked);
        replayed.begin_close();
        assert!(matches!(replayed.close_step(0, 0), InteractiveJobCloseStep::Pending { .. }));
        while !replayed.terminal_is_empty() {
            let _ = replayed.close_step(1, LOWPOLY_SCAN_BYTES);
        }
        assert_eq!(replayed.close_step(1, LOWPOLY_SCAN_BYTES), InteractiveJobCloseStep::Complete);
        assert!(replayed.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_scan_step_stays_below_eight_milliseconds() {
        let command = LowpolyCommand::PaintStrokeBegin(paint_stroke_begin::PaintStrokeBegin {});
        let snapshot = crate::artifacts::lowpoly::schema::default_snapshot();
        let config = LowpolyConfig::default();
        let interaction = protocol::InteractionState::default();
        let hover = semio_framework_plugin::app::InteractionHoverState::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let operation = retained_operation();
        let context = retained_context(LowpolyTransient::with_test_workspace_bytes(LOWPOLY_SCAN_BYTES * 2), 23);
        let extent = lowpoly_extent(&snapshot, &interaction, &context).expect("retained extent");
        let mut work = LowpolyRetainedCommandWork::new("paintStrokeBegin", LowpolyCommandDisposition::Paint, extent);
        let started = std::time::Instant::now();
        assert!(matches!(work.step(&command, &snapshot, &config, &history, &interaction, &hover, Some(&context), &operation).expect("bounded scan"), ArtifactCommandWorkStep::Progress { .. }));
        assert!(started.elapsed() < std::time::Duration::from_millis(8));
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_every_command_turn_stays_below_eight_milliseconds() {
        let snapshot = crate::artifacts::lowpoly::schema::default_snapshot();
        let config = LowpolyConfig::default();
        let interaction = protocol::InteractionState::default();
        let hover = semio_framework_plugin::app::InteractionHoverState::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let operation = retained_operation();
        let context = retained_context(LowpolyTransient::default(), 29);
        for (command, tool_id) in every_command().into_iter().zip(LOWPOLY_TOOL_IDS.iter().copied()) {
            let disposition = lowpoly_command_disposition(tool_id).expect("exact Lowpoly disposition");
            let extent = lowpoly_extent(&snapshot, &interaction, &context).expect("retained extent");
            let mut work = LowpolyRetainedCommandWork::new(tool_id, disposition, extent);
            loop {
                let started = std::time::Instant::now();
                let step = work.step(&command, &snapshot, &config, &history, &interaction, &hover, Some(&context), &operation).expect("retained command turn");
                assert!(started.elapsed() < std::time::Duration::from_millis(8), "{tool_id} retained turn exceeded 8 ms");
                if matches!(step, ArtifactCommandWorkStep::Complete(_) | ArtifactCommandWorkStep::CompleteWithEphemeral { .. }) {
                    break;
                }
            }
            work.begin_close();
            while !work.terminal_is_empty() {
                let _ = work.close_step(1, LOWPOLY_SCAN_BYTES);
            }
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_mesh_uv_and_transform_maximum_fixture_turns_stay_below_eight_milliseconds() {
        let mut mesh_json = String::new();
        let mut rejected_next = false;
        for subdivisions in 0..=5 {
            let candidate = semio_framework_3d::mesh::HalfedgeMesh::ico_sphere_prim(1.0, subdivisions).expect("maximum mesh fixture").to_json().expect("maximum mesh json");
            if candidate.len() > LOWPOLY_RETAINED_MESH_BYTES {
                rejected_next = true;
                break;
            }
            mesh_json = candidate;
        }
        assert!(rejected_next && !mesh_json.is_empty(), "fixture family must cross the exact mesh byte cap");
        let snapshot = crate::artifacts::lowpoly::snapshot_from_mesh_json(&mesh_json, "obj-max", "Maximum");
        let config = LowpolyConfig::default();
        let interaction = protocol::InteractionState::default();
        let hover = semio_framework_plugin::app::InteractionHoverState::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let operation = retained_operation();
        let context = retained_context(LowpolyTransient::with_test_mesh_workspace("obj-max", mesh_json), 31);
        let commands = [
            ("triangulate", LowpolyCommand::Triangulate(triangulate::Triangulate {})),
            ("decimate", LowpolyCommand::Decimate(decimate::Decimate { decimate_ratio: Some(0.5) })),
            ("unwrapActive", LowpolyCommand::UnwrapActive(unwrap_active::UnwrapActive {})),
            ("translateSelection", LowpolyCommand::TranslateSelection(translate_selection::TranslateSelection { mode: Some("mesh".into()), ids: Some(Vec::new()), dx: 0.25, dy: 0.0, dz: 0.0 })),
        ];
        for (tool_id, command) in commands {
            let disposition = lowpoly_command_disposition(tool_id).expect("maximum fixture disposition");
            let extent = lowpoly_extent(&snapshot, &interaction, &context).expect("maximum fixture extent");
            let mut work = LowpolyRetainedCommandWork::new(tool_id, disposition, extent);
            loop {
                let started = std::time::Instant::now();
                let step = work.step(&command, &snapshot, &config, &history, &interaction, &hover, Some(&context), &operation).expect("maximum fixture turn");
                assert!(started.elapsed() < std::time::Duration::from_millis(8), "{tool_id} maximum fixture turn exceeded 8 ms");
                if matches!(step, ArtifactCommandWorkStep::Complete(_) | ArtifactCommandWorkStep::CompleteWithEphemeral { .. }) {
                    break;
                }
            }
            work.begin_close();
            while !work.terminal_is_empty() {
                let _ = work.close_step(1, LOWPOLY_SCAN_BYTES);
            }
        }
    }

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
    /// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[semio_framework_async_macros::async_test]
    async fn command_ids_are_unique() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 47, "every LowpolyCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword.
    #[semio_framework_async_macros::async_test]
    async fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
        for command in every_command() {
            let printed = protocol::OpText::print_op(&command);
            let first_token = printed.split(' ').next().unwrap_or_default();
            assert!(!first_token.is_empty(), "printed op line must start with a wire keyword: {printed:?}");
        }
    }

    /// 🧾️ One representative value per row, in declaration (= binary ordinal) order.
    pub(super) fn every_command() -> Vec<LowpolyCommand> {
        vec![
            LowpolyCommand::AddPrimitive(add_primitive::AddPrimitive { kind: Some("box".into()) }),
            LowpolyCommand::PatchObject(patch_object::PatchObject { object_id: "obj-1".into(), field: "name".into(), value_json: Some("\"Renamed\"".into()) }),
            LowpolyCommand::Extrude(extrude::Extrude { extrude_distance: Some(0.25) }),
            LowpolyCommand::Inset(inset::Inset { inset_amount: Some(0.1) }),
            LowpolyCommand::Bevel(bevel::Bevel { bevel_amount: Some(0.05), bevel_segments: Some(1) }),
            LowpolyCommand::LoopCut(loop_cut::LoopCut { loop_cuts: Some(1) }),
            LowpolyCommand::Subdivide(subdivide::Subdivide {}),
            LowpolyCommand::Triangulate(triangulate::Triangulate {}),
            LowpolyCommand::Mirror(mirror::Mirror { axis: Some("x".into()) }),
            LowpolyCommand::Decimate(decimate::Decimate { decimate_ratio: Some(0.5) }),
            LowpolyCommand::FlipFaces(flip_faces::FlipFaces { face_ids: vec![0] }),
            LowpolyCommand::Merge(merge::Merge {}),
            LowpolyCommand::Dissolve(dissolve::Dissolve {}),
            LowpolyCommand::Snap(snap::Snap {}),
            LowpolyCommand::ToggleSmooth(toggle_smooth::ToggleSmooth {}),
            LowpolyCommand::UnwrapActive(unwrap_active::UnwrapActive {}),
            LowpolyCommand::MarkUvSeam(mark_uv_seam::MarkUvSeam { seam: Some(true), edge_ids: Some(vec![0]) }),
            LowpolyCommand::ClearSeam(clear_seam::ClearSeam {}),
            LowpolyCommand::TranslateSelection(translate_selection::TranslateSelection { mode: Some("mesh".into()), ids: Some(vec![]), dx: 1.0, dy: 0.0, dz: 0.0 }),
            LowpolyCommand::RotateSelection(rotate_selection::RotateSelection { mode: Some("mesh".into()), ids: Some(vec![]), ax: 0.0, ay: 1.0, az: 0.0, angle: 45.0 }),
            LowpolyCommand::ScaleSelection(scale_selection::ScaleSelection { mode: Some("mesh".into()), ids: Some(vec![]), sx: 1.0, sy: 1.0, sz: 1.0 }),
            LowpolyCommand::AddPaintLayer(add_paint_layer::AddPaintLayer { object_id: None, name: Some("Detail".into()) }),
            LowpolyCommand::PaintStrokeEnd(paint_stroke_end::PaintStrokeEnd {}),
            LowpolyCommand::PaintFill(paint_fill::PaintFill { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
            LowpolyCommand::FillBucket(fill_bucket::FillBucket { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
            LowpolyCommand::TransformEnd(transform_end::TransformEnd {}),
            LowpolyCommand::ImportSnapshotJson(set_snapshot_json::ImportSnapshotJson { json: "{}".into() }),
            LowpolyCommand::SetFixtureJson(set_fixture_json::SetFixtureJson { json: "{}".into() }),
            LowpolyCommand::EngagementSubmit(engagement_submit::EngagementSubmit { value: Some("extrude".into()) }),
            LowpolyCommand::SetActiveObject(set_active_object::SetActiveObject { object_id: "obj-1".into() }),
            LowpolyCommand::SetActivePaintLayer(set_active_paint_layer::SetActivePaintLayer { layer_index: 0 }),
            LowpolyCommand::SetUtilityParam(set_utility_param::SetUtilityParam { key: "brushSize".into(), value_json: "20".into() }),
            LowpolyCommand::EngagementInput(engagement_input::EngagementInput { value: "ext".into() }),
            LowpolyCommand::ToggleShowEdges(toggle_show_edges::ToggleShowEdges {}),
            LowpolyCommand::ToggleSun(toggle_sun::ToggleSun {}),
            LowpolyCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 45.0 }),
            LowpolyCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 35.0 }),
            LowpolyCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 0.8 }),
            LowpolyCommand::SetCamera(set_camera::SetCamera { position: [1.0, 2.0, 3.0], target: [0.0, 0.0, 0.0], fov: 45.0 }),
            LowpolyCommand::PaintStrokeBegin(paint_stroke_begin::PaintStrokeBegin {}),
            LowpolyCommand::PaintSample(paint_sample::PaintSample { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
            LowpolyCommand::PaintStroke(paint_stroke::PaintStroke { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
            LowpolyCommand::PaintAt(paint_at::PaintAt { object_id: None, u: Some(0.5), v: Some(0.5), x: None, y: None }),
            LowpolyCommand::CanvasPointerDown(canvas_pointer_down::CanvasPointerDown { object_id: None, u: None, v: None, x: Some(0.0), y: Some(0.0) }),
            LowpolyCommand::CanvasPointerMove(canvas_pointer_move::CanvasPointerMove { object_id: None, u: None, v: None, x: Some(1.0), y: Some(1.0) }),
            LowpolyCommand::TransformBegin(transform_begin::TransformBegin {}),
            LowpolyCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "rotate".into() }),
        ]
    }
    //#endregion 🔖️CommandSurface

    //#region 🔖️ManifestSanity
    #[semio_framework_async_macros::async_test]
    async fn the_manifest_stitches_every_taxonomy_node() {
        let json = serde_json::to_string(&create_lowpoly_app()).expect("app definition json");
        for id in [edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN, paint_mode::windows::uv::LOWPOLY_PLAY_WINDOW_UV] {
            assert!(json.contains(id), "window kind {id} missing from the manifest: {json}");
        }
        for id in [edit::LOWPOLY_PLAY_MODE_EDIT, paint_mode::LOWPOLY_PLAY_MODE_PAINT, paint_mode::LOWPOLY_PLAY_LAYOUT_PAINT] {
            assert!(json.contains(id), "mode/layout {id} missing from the manifest");
        }
        for body in [LOWPOLY_PLAY_BODY_DOCUMENT, LOWPOLY_PLAY_BODY_CATALOGUE, LOWPOLY_PLAY_BODY_INSPECTION, LOWPOLY_PLAY_BODY_LAYERS] {
            assert!(json.contains(body), "panel body {body} missing from the manifest");
        }
        assert!(json.contains("3d.lowpoly"), "artifact kind missing from the manifest");
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the "mesh" domain is declared and
    /// scoped to the Model window, and the framework auto-injects its six interaction actions.
    #[semio_framework_async_macros::async_test]
    async fn the_mesh_interaction_domain_is_declared_and_scoped_to_the_model_window() {
        let definition = create_lowpoly_app();
        let mesh = definition.interactions.iter().find(|interaction| interaction.id == MESH_INTERACTION_DOMAIN).expect("mesh domain declared");
        assert_eq!(mesh.granularities.iter().map(|granularity| granularity.id.as_str()).collect::<Vec<_>>(), vec!["object", "vertex", "edge", "face"]);
        assert!(matches!(mesh.hierarchy, HierarchyProvider::Flat));
        let main_window = definition.window_kinds.iter().find(|window| window.id == edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN).expect("main window declared");
        assert_eq!(main_window.interactions, vec![InteractionRef::new(MESH_INTERACTION_DOMAIN)]);
        for injected in ["interactionSelect", "interactionHover", "clearSelection", "selectAll", "setSelectionMode", "setInteractionGranularity"] {
            assert!(main_window.actions.iter().any(|action| action.id == injected), "framework must auto-inject {injected}");
        }
        for deleted in ["setSelection", "toggleSelectionKind", "toggleSelectionTarget", "setSelectionMethod", "setSelectionModeDefault", "worldSelect", "worldHover", "setHover", "worldPick"] {
            assert!(!definition.window_kinds.iter().flat_map(|window| window.actions.iter()).any(|action| action.id == deleted), "{deleted} must no longer be app-declared");
        }
    }
    //#endregion 🔖️ManifestSanity

    //#region 🔖️CrossCutting
    #[semio_framework_async_macros::async_test]
    async fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<EditorApp<LowpolyPlayApp>, _>(
            "mem://lowpoly-convergence",
            LowpolyCommand::PatchObject(patch_object::PatchObject { object_id: "obj-1".into(), field: "name".into(), value_json: Some(serde_json::to_string("Renamed By A").unwrap()) }),
            LowpolyCommand::AddPrimitive(add_primitive::AddPrimitive { kind: Some("box".into()) }),
            |app| app.snapshot().expect("projection"),
        )
        .await;
    }

    #[semio_framework_async_macros::async_test]
    async fn ingest_operations_is_idempotent() {
        testkit::assert_ingest_idempotent::<EditorApp<LowpolyPlayApp>, _>(LowpolyCommand::PatchObject(patch_object::PatchObject { object_id: "obj-1".into(), field: "name".into(), value_json: Some(serde_json::to_string("Hero").unwrap()) }), |app| {
            app.snapshot().expect("projection")
        })
        .await;
    }

    #[semio_framework_async_macros::async_test]
    async fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::editor::lowpoly::testkit::render;
        let mut a = app();
        assert!(render(&mut a, "lowpoly.play.nope").await.contains("Unknown body"));
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️MediaPorts
    #[semio_framework_async_macros::async_test]
    async fn export_media_mesh_out_produces_mesh_document_payload() {
        let mut a: LowpolyApp = app();
        let media = semio_framework_plugin::resolve_ready(a.export_media("mesh:out")).expect("export mesh:out");
        assert_eq!(media.media_type, MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh });
        match media.payload {
            MediaPayload::Structured { schema, .. } => assert_eq!(schema, "mesh.document"),
            other => panic!("expected Structured payload, got {other:?}"),
        }
    }

    /// 🧬️ `"mesh:in"` replaces the whole document via `reset_document_effect` (a
    /// `Effect::LoadDocument`, outside undo history) — whole-document replace has no replacement
    /// mutation per `📓️taxonomy.md`, so this is an effect, not an `artifact_mutations` entry.
    #[semio_framework_async_macros::async_test]
    async fn import_media_mesh_in_round_trips_into_a_reset_document_effect() {
        let mesh = semio_framework_plugin::mesh_from_kind("box");
        let mesh_document = crate::artifacts::lowpoly::schema::mesh_document_from_mesh(&mesh).expect("mesh document");
        let json = serde_json::to_string(&mesh_document).expect("mesh document json");
        let media = Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: MediaPayload::Structured { schema: "mesh.document".into(), json } };
        let projection = crate::artifacts::lowpoly::schema::default_snapshot();
        let history = semio_framework_plugin::HistoryView::empty();
        let doc = ArtifactView::new(&projection, &history);
        let emit = LowpolyPlayApp::import_media("mesh:in", &media, &doc).expect("import mesh:in");
        assert!(emit.artifact_mutations.is_empty(), "whole-document replace is an effect, not a mutation");
        let semio_framework_plugin::Effect::LoadDocument { pack, .. } = emit.effects.first().expect("mesh:in must emit a LoadDocument effect") else {
            panic!("expected a LoadDocument effect");
        };
        let loaded = <LowpolySnapshot as ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert_eq!(loaded.objects.len(), 1);
    }
    //#endregion 🔖️MediaPorts

    //#region 🔖️ContextMenuRegistry
    #[semio_framework_async_macros::async_test]
    async fn registry_wired_app_dispatches_add_primitive() {
        let mut a = app_with_registry();
        crate::editor::lowpoly::testkit::dispatch(&mut a, LowpolyCommand::AddPrimitive(add_primitive::AddPrimitive { kind: Some("plane".into()) })).await;
        assert_eq!(a.snapshot().expect("projection").objects.len(), 2);
    }
    //#endregion 🔖️ContextMenuRegistry
}
//#endregion 🧪️Tests
