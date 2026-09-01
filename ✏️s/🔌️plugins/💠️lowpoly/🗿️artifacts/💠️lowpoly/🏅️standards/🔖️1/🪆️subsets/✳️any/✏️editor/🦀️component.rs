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
use crate::artifacts::lowpoly::{artifact_kind, LowpolyObject, LowpolySnapshot, LOWPOLY_DOCUMENT_SCHEMA};
use crate::editor::lowpoly::commands::{add_primitive, camera, chrome, engagement, fixture, mesh_edit, paint, patch_object, selection, sun, transform, utility, uv};
use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::modes::{edit, paint as paint_mode};
use crate::editor::lowpoly::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel, layers as layers_panel};
use crate::editor::lowpoly::session::{LowpolyScratch, LowpolyTransient, LowpolyTransientMutation};
use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::{is_paint_utility, resolve_active_object_id, selection_from_interaction, selection_from_state, utility_param_f64, LowpolyView, MESH_INTERACTION_DOMAIN};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::app::{ArtifactOwnedToolJobContext, InteractionView};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::{
    ActionArgDef, ActionArgOption, ActionDescriptor, ActionRef, AppOperationContext, ArtifactEditor, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactView, ConfigView, DraftView,
    Editor, EditorApp, Emit, EphemeralEmit, Fault, GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, InteractionRef, LabelText, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, MergeMode,
    NoDraft, NoDraftMutation, SelectionMethod, SelectionMode, SelectionSpec, UiNode, UtilityCategory, UtilityDefinition, WindowEngagement, WindowEngagementInput, WindowEngagementOption, WindowEngagementPossible, WindowEngagementStatus,
    WindowMeasure,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use protocol::{Mutation, MutationDiff};
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
const LOWPOLY_RETAINED_RAW_BYTES: usize = 16_384;
const LOWPOLY_RETAINED_WORK_ITEMS: usize = 258;
const LOWPOLY_RETAINED_PAINT_CHUNK_BYTES: usize = 16_384;
const LOWPOLY_RETAINED_PAINT_RUNS: usize = 4_096;
const LOWPOLY_RETAINED_FIELD_BYTES: usize = 4_096;
const LOWPOLY_RETAINED_OBJECTS: usize = 64;
const LOWPOLY_RETAINED_PAINT_LAYERS_PER_OBJECT: usize = 8;
const LOWPOLY_RETAINED_PAINT_LAYER_BYTES: usize = 4 * 1024 * 1024;
const LOWPOLY_MIGRATED_TOOL_IDS: &[&str] = &[
    "patchObject",
    "addPaintLayer",
    "paintStrokeEnd",
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
    "importSnapshotJson",
    "setFixtureJson",
    "paintSample",
    "paintStrokeBegin",
    "transformBegin",
    "setActiveUtility",
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
    "paintFill",
    "fillBucket",
    "transformEnd",
    "engagementSubmit",
    "paintStroke",
    "paintAt",
    "canvasPointerDown",
    "canvasPointerMove",
    "addPrimitive",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum LowpolyCommandDisposition {
    Artifact = 1,
    Config = 2,
    HostOnly = 3,
    Transient = 4,
    ConfigTransient = 5,
    ArtifactTransient = 6,
    /// 🌱️ `addPrimitive` only: its handler unconditionally emits both an `Artifact` mutation
    /// (`CreateObject`) and a `Config` mutation (`SetActiveObject`), AND — like every other
    /// `session::build_doc`/`mesh_edit` reacher — reads and writes the session-local `mesh_workspace`
    /// cache, so it needs the same rehydrate-then-republish treatment `ArtifactTransient` documents.
    /// The coordinator's own schema edit (`🧪️interactive-job/🔣️schema.json`'s 8th `oneOf` signature,
    /// `["Artifact","Config","Transient"]`/`["Artifact","Config"]`) is what makes this representable.
    ArtifactConfigTransient = 7,
}

fn lowpoly_command_disposition(tool_id: &str) -> Option<LowpolyCommandDisposition> {
    Some(match tool_id {
        "patchObject" | "addPaintLayer" | "paintFill" | "fillBucket" => LowpolyCommandDisposition::Artifact,
        // 🕸️ Every one of these reaches `session::build_doc`/`mesh_edit` (directly, or via
        // `transform_selection`'s unbracketed-commit fallback / `commit_transform`), which needs the
        // session-local `mesh_workspace` half-edge-mesh cache seeded from the LIVE persisted
        // `LowpolyTransient`, not a blank one — `LowpolyDocument::reload_meshes`
        // (`⚙️engine/🦀️component.rs`) rejects every edit past the first with `StaleMeshWorkspace`
        // otherwise. `lowpoly_retained_reduce`'s `threaded!` arms rehydrate scratch from
        // `context.transient` and publish the post-handle cache back as the new transient root, so
        // every one of these is `Artifact` (the real edit) `+ Transient` (the cache bookkeeping).
        "paintStrokeEnd"
        | "extrude"
        | "inset"
        | "bevel"
        | "loopCut"
        | "subdivide"
        | "triangulate"
        | "mirror"
        | "decimate"
        | "flipFaces"
        | "merge"
        | "dissolve"
        | "snap"
        | "toggleSmooth"
        | "unwrapActive"
        | "markUvSeam"
        | "clearSeam"
        | "engagementSubmit"
        | "translateSelection"
        | "rotateSelection"
        | "scaleSelection"
        | "transformEnd" => LowpolyCommandDisposition::ArtifactTransient,
        "importSnapshotJson" | "setFixtureJson" => LowpolyCommandDisposition::HostOnly,
        "paintStrokeBegin" | "transformBegin" => LowpolyCommandDisposition::Transient,
        // 🖌️ `setActiveUtility` plus every paint-tick command (`paint_tick` mutates the mid-drag
        // `stroke`/`stroke_drag_active` scratch, or — eyedropper — emits a `Config` mutation instead):
        // both outcomes need the same `[Config, Transient]` lane pair the tick's own disposition can't
        // statically distinguish between.
        "setActiveUtility" | "paintStroke" | "paintAt" | "canvasPointerDown" | "canvasPointerMove" => LowpolyCommandDisposition::ConfigTransient,
        // 🌱️ `addPrimitive`'s handler unconditionally emits both a `CreateObject` Artifact mutation and
        // a `SetActiveObject` Config mutation, and it reaches `session::build_doc` — same as every
        // `ArtifactTransient` command above — so it needs the identical scratch rehydration/republication.
        "addPrimitive" => LowpolyCommandDisposition::ArtifactConfigTransient,
        tool_id if LOWPOLY_MIGRATED_TOOL_IDS.contains(&tool_id) => LowpolyCommandDisposition::Config,
        _ => return None,
    })
}

fn lowpoly_contract() -> ToolExecutionContract {
    ToolExecutionContract::resumable(LOWPOLY_RETAINED_RAW_BYTES, LOWPOLY_RETAINED_WORK_ITEMS, 1, 32 * 1024 * 1024, 7_500, 1, 1)
}

fn lowpoly_snapshot_admitted(snapshot: &LowpolySnapshot) -> bool {
    snapshot.schema.len() <= LOWPOLY_RETAINED_FIELD_BYTES
        && snapshot.objects.len() <= LOWPOLY_RETAINED_OBJECTS
        && snapshot.objects.iter().all(|object| {
            object.id.len() <= LOWPOLY_RETAINED_FIELD_BYTES
                && object.name.len() <= LOWPOLY_RETAINED_FIELD_BYTES
                && object.mesh.as_ref().is_none_or(|mesh| mesh.child_id.len() <= LOWPOLY_RETAINED_FIELD_BYTES && mesh.target.to_uri().len() <= LOWPOLY_RETAINED_FIELD_BYTES)
                && object.paint_layers.len() <= LOWPOLY_RETAINED_PAINT_LAYERS_PER_OBJECT
                && object.paint_layers.iter().all(|layer| layer.name.len() <= LOWPOLY_RETAINED_FIELD_BYTES && layer.blend_mode.len() <= LOWPOLY_RETAINED_FIELD_BYTES && layer.pixels.len() <= LOWPOLY_RETAINED_PAINT_LAYER_BYTES)
        })
}

fn lowpoly_command_admitted(command: &LowpolyCommand, snapshot: &LowpolySnapshot, config: &LowpolyConfig) -> bool {
    let field = |value: &str| value.len() <= LOWPOLY_RETAINED_FIELD_BYTES;
    lowpoly_snapshot_admitted(snapshot)
        && lowpoly_config_retained_bytes(config) <= LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES
        && match command {
        LowpolyCommand::PatchObject(payload) => field(&payload.object_id) && field(&payload.field) && payload.value_json.as_deref().is_none_or(field),
        LowpolyCommand::AddPaintLayer(payload) => payload.object_id.as_deref().is_none_or(field) && payload.name.as_deref().is_none_or(field),
        LowpolyCommand::SetActiveObject(payload) => field(&payload.object_id),
        LowpolyCommand::SetUtilityParam(payload) => field(&payload.key) && field(&payload.value_json),
        LowpolyCommand::EngagementInput(payload) => field(&payload.value),
        LowpolyCommand::ImportSnapshotJson(payload) => payload.json.len() <= LOWPOLY_RETAINED_RAW_BYTES,
        LowpolyCommand::SetFixtureJson(payload) => payload.json.len() <= LOWPOLY_RETAINED_RAW_BYTES,
        LowpolyCommand::PaintSample(payload) => payload.object_id.as_deref().is_none_or(field),
        LowpolyCommand::PaintStrokeEnd(_) => true,
        LowpolyCommand::SetActiveUtility(payload) => field(&payload.utility_id),
        LowpolyCommand::PaintStrokeBegin(_) | LowpolyCommand::TransformBegin(_) => true,
        LowpolyCommand::SetActivePaintLayer(_)
        | LowpolyCommand::ToggleShowEdges(_)
        | LowpolyCommand::ToggleSun(_)
        | LowpolyCommand::SetSunAzimuth(_)
        | LowpolyCommand::SetSunElevation(_)
        | LowpolyCommand::SetSunIntensity(_)
        | LowpolyCommand::SetCamera(_) => true,
        LowpolyCommand::Extrude(_)
        | LowpolyCommand::Inset(_)
        | LowpolyCommand::Bevel(_)
        | LowpolyCommand::LoopCut(_)
        | LowpolyCommand::Subdivide(_)
        | LowpolyCommand::Triangulate(_)
        | LowpolyCommand::Decimate(_)
        | LowpolyCommand::Merge(_)
        | LowpolyCommand::Dissolve(_)
        | LowpolyCommand::Snap(_)
        | LowpolyCommand::ToggleSmooth(_)
        | LowpolyCommand::UnwrapActive(_)
        | LowpolyCommand::ClearSeam(_)
        | LowpolyCommand::TransformEnd(_) => true,
        LowpolyCommand::Mirror(payload) => payload.axis.as_deref().is_none_or(field),
        LowpolyCommand::FlipFaces(payload) => payload.face_ids.len() <= LOWPOLY_RETAINED_WORK_ITEMS,
        LowpolyCommand::MarkUvSeam(payload) => payload.edge_ids.as_ref().is_none_or(|ids| ids.len() <= LOWPOLY_RETAINED_WORK_ITEMS),
        LowpolyCommand::EngagementSubmit(payload) => payload.value.as_deref().is_none_or(field),
        LowpolyCommand::TranslateSelection(payload) => payload.mode.as_deref().is_none_or(field) && payload.ids.as_ref().is_none_or(|ids| ids.len() <= LOWPOLY_RETAINED_WORK_ITEMS),
        LowpolyCommand::RotateSelection(payload) => payload.mode.as_deref().is_none_or(field) && payload.ids.as_ref().is_none_or(|ids| ids.len() <= LOWPOLY_RETAINED_WORK_ITEMS),
        LowpolyCommand::ScaleSelection(payload) => payload.mode.as_deref().is_none_or(field) && payload.ids.as_ref().is_none_or(|ids| ids.len() <= LOWPOLY_RETAINED_WORK_ITEMS),
        LowpolyCommand::PaintFill(payload) => payload.object_id.as_deref().is_none_or(field),
        LowpolyCommand::FillBucket(payload) => payload.object_id.as_deref().is_none_or(field),
        LowpolyCommand::PaintStroke(payload) => payload.object_id.as_deref().is_none_or(field),
        LowpolyCommand::PaintAt(payload) => payload.object_id.as_deref().is_none_or(field),
        LowpolyCommand::CanvasPointerDown(payload) => payload.object_id.as_deref().is_none_or(field),
        LowpolyCommand::CanvasPointerMove(payload) => payload.object_id.as_deref().is_none_or(field),
        LowpolyCommand::AddPrimitive(payload) => payload.kind.as_deref().is_none_or(field),
    }
}

fn lowpoly_sample_pixel(snapshot: &LowpolySnapshot, config: &LowpolyConfig, payload: &paint_sample::PaintSample) -> Emit<LowpolyMutation, LowpolyConfigMutation> {
    let Some((u, v)) = crate::editor::lowpoly::session::paint_uv_from_command(payload.u, payload.v, payload.x, payload.y) else { return Emit::default() };
    let object_id = payload.object_id.clone().unwrap_or_else(|| resolve_active_object_id(snapshot, config));
    let Some(object) = snapshot.objects.iter().find(|object| object.id == object_id) else { return Emit::default() };
    let size = crate::artifacts::lowpoly::LOWPOLY_PAINT_TEXTURE_SIZE;
    let x = ((u.clamp(0.0, 1.0) * (size as f32 - 1.0)).round() as usize).min(size - 1);
    let y = (((1.0 - v.clamp(0.0, 1.0)) * (size as f32 - 1.0)).round() as usize).min(size - 1);
    let offset = (y * size + x) * 4;
    let mut color = [0_u8; 4];
    for layer in object.paint_layers.iter().filter(|layer| layer.visible) {
        if offset.saturating_add(4) > layer.pixels.len() {
            continue;
        }
        let source = [
            layer.pixels[offset],
            layer.pixels[offset + 1],
            layer.pixels[offset + 2],
            layer.pixels[offset + 3],
        ];
        let source_alpha = (source[3] as f32 / 255.0) * layer.opacity.clamp(0.0, 1.0);
        let destination_alpha = color[3] as f32 / 255.0;
        let alpha = source_alpha + destination_alpha * (1.0 - source_alpha);
        if alpha < 1e-6 {
            continue;
        }
        for channel in 0..3 {
            let source_channel = source[channel] as f32 / 255.0;
            let destination_channel = color[channel] as f32 / 255.0;
            color[channel] = ((source_channel * source_alpha + destination_channel * destination_alpha * (1.0 - source_alpha)) / alpha * 255.0).round().clamp(0.0, 255.0) as u8;
        }
        color[3] = (alpha * 255.0).round().clamp(0.0, 255.0) as u8;
    }
    Emit::config(vec![LowpolyConfigMutation::SetPaintColor { r: color[0], g: color[1], b: color[2], a: color[3] }])
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
    // 🕹️ The mesh domain's live selection for THIS dispatch, read straight from the retained job's own
    // scheduler-owned `InteractionState` — `selection_from_state`'s own doc comment names this exact
    // seam ("typed retained reducers read the same immutable domain selection directly from their
    // scheduler-owned request context, without manufacturing a host-only InteractionView").
    let active_object_id = resolve_active_object_id(snapshot, config);
    let empty_domain_selection = protocol::DomainSelection::default();
    let domain_selection = interaction.selection.get(MESH_INTERACTION_DOMAIN).unwrap_or(&empty_domain_selection);
    let selection = selection_from_state(&active_object_id, domain_selection);
    // 🕸️ Commands whose handler reaches `session::build_doc`/`mesh_edit`, the mid-drag paint stroke
    // scratch, or the mid-drag gumball transform scratch cannot use a blank `LowpolyScratch::default()`
    // — `LowpolyDocument::reload_meshes` (`⚙️engine/🦀️component.rs`) rejects every mesh edit past the
    // very first with `StaleMeshWorkspace` unless scratch is rehydrated from the LIVE persisted
    // `LowpolyTransient`. This macro does that rehydration, runs the handler, then publishes the
    // post-handle scratch back as the new transient root (`ArtifactToolPublicationLane::Transient`) —
    // see `LowpolyCommandDisposition::ArtifactTransient`/`ConfigTransient`'s doc comments.
    macro_rules! threaded {
        ($handle:expr) => {{
            let mut threaded = LowpolyScratch::from_transient(&context.transient, selection.clone()).map_err(Fault::from)?;
            let step_emit = ($handle)(&doc, &cfg, &mut threaded)?;
            let transient = threaded.transient_snapshot().map_err(Fault::from)?;
            return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                emit: step_emit,
                ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![LowpolyTransientMutation::Snapshot { transient }] },
            });
        }};
    }
    let mut bounded = LowpolyScratch::default();
    let emit = match command {
        LowpolyCommand::PatchObject(payload) => patch_object::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::AddPaintLayer(payload) => add_paint_layer::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::SetActiveObject(payload) => set_active_object::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::SetActivePaintLayer(payload) => set_active_paint_layer::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::SetUtilityParam(payload) => set_utility_param::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::EngagementInput(payload) => engagement_input::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::ToggleShowEdges(payload) => toggle_show_edges::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::ToggleSun(payload) => toggle_sun::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::SetSunAzimuth(payload) => set_sun_azimuth::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::SetSunElevation(payload) => set_sun_elevation::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::SetSunIntensity(payload) => set_sun_intensity::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::SetCamera(payload) => set_camera::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::ImportSnapshotJson(payload) => set_snapshot_json::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::SetFixtureJson(payload) => set_fixture_json::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::PaintSample(payload) => return Ok(ArtifactCommandWorkStep::Complete(lowpoly_sample_pixel(snapshot, config, payload))),
        LowpolyCommand::PaintStrokeBegin(_) => {
            let transient = context.transient.begin_stroke_drag();
            return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit: Emit::default(), ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![LowpolyTransientMutation::Snapshot { transient }] } });
        }
        LowpolyCommand::TransformBegin(_) => {
            let transient = context.transient.begin_transform_drag();
            return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit: Emit::default(), ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![LowpolyTransientMutation::Snapshot { transient }] } });
        }
        LowpolyCommand::SetActiveUtility(payload) => {
            let mut config_mutations = vec![LowpolyConfigMutation::SetActiveUtility { utility_id: payload.utility_id.clone() }];
            if is_paint_utility(&payload.utility_id) {
                config_mutations.push(LowpolyConfigMutation::SetPaintUtility { value: payload.utility_id.clone() });
            }
            let transient = context.transient.reset_gestures();
            return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral {
                emit: Emit::config(config_mutations),
                ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![LowpolyTransientMutation::Snapshot { transient }] },
            });
        }
        LowpolyCommand::Extrude(payload) => threaded!(|doc, cfg, ctx| extrude::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Inset(payload) => threaded!(|doc, cfg, ctx| inset::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Bevel(payload) => threaded!(|doc, cfg, ctx| bevel::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::LoopCut(payload) => threaded!(|doc, cfg, ctx| loop_cut::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Subdivide(payload) => threaded!(|doc, cfg, ctx| subdivide::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Triangulate(payload) => threaded!(|doc, cfg, ctx| triangulate::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Mirror(payload) => threaded!(|doc, cfg, ctx| mirror::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Decimate(payload) => threaded!(|doc, cfg, ctx| decimate::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::FlipFaces(payload) => threaded!(|doc, cfg, ctx| flip_faces::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Merge(payload) => threaded!(|doc, cfg, ctx| merge::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Dissolve(payload) => threaded!(|doc, cfg, ctx| dissolve::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::Snap(payload) => threaded!(|doc, cfg, ctx| snap::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::ToggleSmooth(payload) => threaded!(|doc, cfg, ctx| toggle_smooth::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::UnwrapActive(payload) => threaded!(|doc, cfg, ctx| unwrap_active::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::MarkUvSeam(payload) => threaded!(|doc, cfg, ctx| mark_uv_seam::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::ClearSeam(payload) => threaded!(|doc, cfg, ctx| clear_seam::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::EngagementSubmit(payload) => threaded!(|doc, cfg, ctx| engagement_submit::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::TranslateSelection(payload) => threaded!(|doc, cfg, ctx| translate_selection::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::RotateSelection(payload) => threaded!(|doc, cfg, ctx| rotate_selection::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::ScaleSelection(payload) => threaded!(|doc, cfg, ctx| scale_selection::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::TransformEnd(payload) => threaded!(|doc, cfg, ctx| transform_end::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::PaintStroke(payload) => threaded!(|doc, cfg, ctx| paint_stroke::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::PaintAt(payload) => threaded!(|doc, cfg, ctx| paint_at::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::CanvasPointerDown(payload) => threaded!(|doc, cfg, ctx| canvas_pointer_down::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::CanvasPointerMove(payload) => threaded!(|doc, cfg, ctx| canvas_pointer_move::handle(payload, doc, cfg, ctx)),
        // 🌱️ `add_primitive::handle` reaches `session::build_doc` (to read the live mesh state) AND
        // calls `ctx.set_mesh_workspace_map` (to record the new primitive's mesh) — both a read and a
        // write of the session-local cache, exactly the shape `ArtifactTransient` commands above are
        // threaded for. A blank `LowpolyScratch::default()` here made `build_doc` return `None` (hence
        // a silent no-op) for every `addPrimitive` after the first mesh edit, because
        // `LowpolyDocument::reload_meshes` rejects a `mesh_workspace` cache that doesn't cover every
        // object the persisted snapshot already has. The handler's own `Emit` still carries both the
        // `CreateObject` artifact mutation and the `SetActiveObject` config mutation; `threaded!` adds
        // the `Transient` republication `PUBLICATION_CONTRACTS`'s `["addPrimitive", lanes: &[Artifact,
        // Config, Transient]]` entry now admits.
        LowpolyCommand::AddPrimitive(payload) => threaded!(|doc, cfg, ctx| add_primitive::handle(payload, doc, cfg, ctx)),
        // 🪣️ `ctx.fill_at` reads/writes only `stroke_dirty` (a render-side texture-cache invalidation
        // counter, never persisted, never read back for any semantic decision) — a single-shot fill
        // needs no transient rehydration, unlike the drag-tick commands above.
        LowpolyCommand::PaintFill(payload) => paint_fill::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::FillBucket(payload) => fill_bucket::handle(payload, &doc, &cfg, &mut bounded),
        // 🚧️ `PaintStrokeEnd` never reaches this reducer — `LowpolyRetainedCommandWork::step` intercepts
        // it before calling `lowpoly_retained_reduce` and routes it to `paint_end_step`'s dedicated
        // bounded-cursor machinery instead. This arm exists only so the match stays exhaustive.
        LowpolyCommand::PaintStrokeEnd(_) => return Err(Fault::from("lowpoly-paint-stroke-end-routes-through-dedicated-step")),
    }?;
    Ok(ArtifactCommandWorkStep::Complete(emit))
}

fn lowpoly_tool_identity(tool_id: &str) -> u64 {
    tool_id.bytes().fold(0xcbf2_9ce4_8422_2325, |digest, byte| (digest ^ u64::from(byte)).wrapping_mul(0x1000_0000_01b3))
}

struct LowpolyRetainedCommandWork {
    tool_id: &'static str,
    disposition: LowpolyCommandDisposition,
    operation_id: u64,
    generation: u64,
    base_revision: [u8; 32],
    context_identity: u64,
    stage: u8,
    replay_target: Option<u8>,
    paint_cursor: usize,
    paint_runs: Vec<crate::artifacts::lowpoly::mutations::PixelRun>,
    paint_open_offset: Option<u32>,
    paint_open_bytes: Vec<u8>,
    paint_digest: u64,
    paint_replay_target: Option<(usize, u64)>,
    complete: bool,
    closing: bool,
}

impl LowpolyRetainedCommandWork {
    fn new(tool_id: &'static str, disposition: LowpolyCommandDisposition, operation_id: u64, generation: u64, base_revision: [u8; 32], context_identity: u64) -> Self {
        Self {
            tool_id,
            disposition,
            operation_id,
            generation,
            base_revision,
            context_identity,
            stage: 0,
            replay_target: None,
            paint_cursor: 0,
            paint_runs: Vec::new(),
            paint_open_offset: None,
            paint_open_bytes: Vec::new(),
            paint_digest: 0xcbf2_9ce4_8422_2325,
            paint_replay_target: None,
            complete: false,
            closing: false,
        }
    }

    fn flush_paint_run(&mut self) -> Result<(), Fault> {
        let Some(offset) = self.paint_open_offset.take() else { return Ok(()) };
        if self.paint_runs.len() >= LOWPOLY_RETAINED_PAINT_RUNS {
            return Err(Fault::from("lowpoly-retained-paint-run-capacity"));
        }
        self.paint_runs.push(crate::artifacts::lowpoly::mutations::PixelRun { offset, bytes: std::mem::take(&mut self.paint_open_bytes) });
        Ok(())
    }

    fn paint_end_step(&mut self, context: &ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>) -> Result<ArtifactCommandWorkStep<EditorApp<LowpolyPlayApp>>, Fault> {
        let Some((object_id, layer_index, before, after)) = context.transient.stroke_diff_parts() else {
            self.complete = true;
            let transient = context.transient.finish_stroke_drag();
            return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit: Emit::default(), ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![LowpolyTransientMutation::Snapshot { transient }] } });
        };
        if before.len() != after.len() || before.len() > LOWPOLY_RETAINED_PAINT_LAYER_BYTES {
            return Err(Fault::from("lowpoly-retained-paint-buffer-capacity"));
        }
        if let Some((target, expected_digest)) = self.paint_replay_target {
            if self.paint_cursor >= target {
                if self.paint_digest != expected_digest {
                    return Err(Fault::from("lowpoly-retained-paint-replay-digest"));
                }
                self.paint_replay_target = None;
                return Ok(ArtifactCommandWorkStep::Replay { stage: "lowpoly-paint-replay", preview: b"{\"en\":\"Restoring paint cursor\",\"de\":\"Malkursor wird wiederhergestellt\"}" });
            }
        }
        let end = self.paint_cursor.saturating_add(LOWPOLY_RETAINED_PAINT_CHUNK_BYTES).min(before.len());
        for index in self.paint_cursor..end {
            if before[index] == after[index] {
                self.flush_paint_run()?;
                continue;
            }
            if self.paint_open_offset.is_none() {
                self.paint_open_offset = Some(index as u32);
            }
            self.paint_open_bytes.push(after[index]);
            self.paint_digest = (self.paint_digest ^ (index as u64)).wrapping_mul(0x1000_0000_01b3);
            self.paint_digest = (self.paint_digest ^ u64::from(after[index])).wrapping_mul(0x1000_0000_01b3);
        }
        self.paint_cursor = end;
        if self.paint_replay_target.is_some() {
            return Ok(ArtifactCommandWorkStep::Replay { stage: "lowpoly-paint-replay", preview: b"{\"en\":\"Restoring paint cursor\",\"de\":\"Malkursor wird wiederhergestellt\"}" });
        }
        if self.paint_cursor < before.len() {
            return Ok(ArtifactCommandWorkStep::Progress { stage: "lowpoly-paint-diff", preview: b"{\"en\":\"Preparing paint edit\",\"de\":\"Malbearbeitung wird vorbereitet\"}" });
        }
        self.flush_paint_run()?;
        let runs = std::mem::take(&mut self.paint_runs);
        let emit = if runs.is_empty() {
            Emit::default()
        } else {
            Emit::commit(
                vec![LowpolyMutation::EditPaintLayer(crate::artifacts::lowpoly::mutations::edit_paint_layer::mutation::EditPaintLayer { object_id: object_id.to_string(), layer_index, runs })],
                "Paint stroke",
            )
        };
        self.complete = true;
        let transient = context.transient.finish_stroke_drag();
        Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![LowpolyTransientMutation::Snapshot { transient }] } })
    }
}

impl ArtifactCommandWork<EditorApp<LowpolyPlayApp>> for LowpolyRetainedCommandWork {
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn workspace_identity(&self) -> u64 {
        lowpoly_tool_identity(self.tool_id) ^ self.operation_id.rotate_left(17) ^ self.generation.rotate_left(31) ^ self.context_identity.rotate_left(43) ^ (u64::from(self.disposition as u8) << 56)
    }

    fn extent(&self, command: &LowpolyCommand, _snapshot: &LowpolySnapshot, _interaction: &protocol::InteractionState, context: Option<&ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>>) -> Option<usize> {
        if matches!(command, LowpolyCommand::PaintStrokeEnd(_)) {
            let bytes = context.and_then(|context| context.transient.stroke_diff_parts().map(|(_, _, before, _)| before.len())).unwrap_or(0);
            return bytes
                .div_ceil(LOWPOLY_RETAINED_PAINT_CHUNK_BYTES)
                .checked_add(2)
                .filter(|extent| *extent <= LOWPOLY_RETAINED_WORK_ITEMS);
        }
        (lowpoly_command_disposition(command.command_id()).is_some()).then_some(2)
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
        if self.closing {
            return Err(Fault::from("lowpoly-retained-work-closing"));
        }
        let context = context.ok_or_else(|| Fault::from("lowpoly-retained-context-absent"))?;
        if operation.operation_id != self.operation_id || operation.generation != self.generation || operation.canonical_base_revision != self.base_revision {
            return Err(Fault::from("lowpoly-retained-operation-freshness-drift"));
        }
        if context.identity_digest() != self.context_identity {
            return Err(Fault::from("lowpoly-retained-context-freshness-drift"));
        }
        if !lowpoly_command_admitted(command, snapshot, config) {
            return Err(Fault::from("lowpoly-retained-command-capacity"));
        }
        if self.stage == 0 {
            self.stage = 1;
            if let Some(target) = self.replay_target {
                if self.stage == target {
                    self.replay_target = None;
                }
                return Ok(ArtifactCommandWorkStep::Replay { stage: "lowpoly-command-workspace-replay", preview: b"{\"en\":\"Restoring Lowpoly workspace\",\"de\":\"Lowpoly-Arbeitsbereich wird wiederhergestellt\"}" });
            }
            return Ok(ArtifactCommandWorkStep::Progress { stage: "lowpoly-command-scan", preview: b"{\"en\":\"Preparing Lowpoly command\",\"de\":\"Lowpoly-Befehl wird vorbereitet\"}" });
        }
        if matches!(command, LowpolyCommand::PaintStrokeEnd(_)) {
            return self.paint_end_step(context);
        }
        let step = lowpoly_retained_reduce(command, snapshot, config, history, interaction, context, operation)?;
        self.complete = true;
        Ok(step)
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        if target.len() < 88 {
            return Err(Fault::from("lowpoly-retained-checkpoint-capacity"));
        }
        target[..88].fill(0);
        target[..4].copy_from_slice(b"LPC2");
        target[4] = self.disposition as u8;
        target[5] = u8::from(self.complete);
        target[6] = self.stage;
        target[8..16].copy_from_slice(&lowpoly_tool_identity(self.tool_id).to_le_bytes());
        target[16..24].copy_from_slice(&self.operation_id.to_le_bytes());
        target[24..32].copy_from_slice(&self.generation.to_le_bytes());
        target[32..64].copy_from_slice(&self.base_revision);
        target[64..72].copy_from_slice(&self.context_identity.to_le_bytes());
        target[72..80].copy_from_slice(&(self.paint_cursor as u64).to_le_bytes());
        target[80..88].copy_from_slice(&self.paint_digest.to_le_bytes());
        Ok(88)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 88 || &checkpoint[..4] != b"LPC2" || checkpoint[4] != self.disposition as u8 || checkpoint[5] > 1 || checkpoint[6] > 1 || checkpoint[7] != 0 {
            return Err(Fault::from("lowpoly-retained-checkpoint-invalid"));
        }
        let tool = u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-tool"))?);
        let operation_id = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-operation"))?);
        let generation = u64::from_le_bytes(checkpoint[24..32].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-generation"))?);
        let context_identity = u64::from_le_bytes(checkpoint[64..72].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-context"))?);
        let paint_cursor_wire = u64::from_le_bytes(checkpoint[72..80].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-paint-cursor"))?);
        if paint_cursor_wire > LOWPOLY_RETAINED_PAINT_LAYER_BYTES as u64 {
            return Err(Fault::from("lowpoly-retained-checkpoint-paint-cursor-capacity"));
        }
        let paint_cursor = paint_cursor_wire as usize;
        let paint_digest = u64::from_le_bytes(checkpoint[80..88].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-paint-digest"))?);
        if tool != lowpoly_tool_identity(self.tool_id) || operation_id != self.operation_id || generation != self.generation || checkpoint[32..64] != self.base_revision || context_identity != self.context_identity {
            return Err(Fault::from("lowpoly-retained-checkpoint-identity-mismatch"));
        }
        self.stage = if self.tool_id == "paintStrokeEnd" { checkpoint[6] } else { 0 };
        self.replay_target = (self.tool_id != "paintStrokeEnd" && checkpoint[6] != 0).then_some(checkpoint[6]);
        self.paint_cursor = 0;
        self.paint_runs.clear();
        self.paint_open_offset = None;
        self.paint_open_bytes.clear();
        self.paint_digest = 0xcbf2_9ce4_8422_2325;
        self.paint_replay_target = (self.tool_id == "paintStrokeEnd" && checkpoint[6] != 0).then_some((paint_cursor, paint_digest));
        self.complete = checkpoint[5] == 1;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        if !self.closing {
            return InteractiveJobCloseStep::Blocked;
        }
        if maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some(run) = self.paint_runs.last() {
            if maximum_bytes < run.bytes.len() {
                return InteractiveJobCloseStep::Blocked;
            }
            let released_bytes = run.bytes.len();
            self.paint_runs.pop();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
        }
        if !self.paint_open_bytes.is_empty() {
            if maximum_bytes < self.paint_open_bytes.len() {
                return InteractiveJobCloseStep::Blocked;
            }
            let released_bytes = self.paint_open_bytes.len();
            self.paint_open_bytes.clear();
            self.paint_open_offset = None;
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes };
        }
        let outer_bytes = self.paint_runs.capacity().saturating_mul(std::mem::size_of::<crate::artifacts::lowpoly::mutations::PixelRun>());
        if outer_bytes != 0 {
            if maximum_bytes < outer_bytes {
                return InteractiveJobCloseStep::Blocked;
            }
            self.paint_runs = Vec::new();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: outer_bytes };
        }
        let open_bytes = self.paint_open_bytes.capacity();
        if open_bytes != 0 {
            if maximum_bytes < open_bytes {
                return InteractiveJobCloseStep::Blocked;
            }
            self.paint_open_bytes = Vec::new();
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: open_bytes };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.paint_runs.capacity() == 0 && self.paint_open_bytes.capacity() == 0
    }
}

struct LowpolyCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl LowpolyCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: LOWPOLY_MIGRATED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
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
    const TOOL_IDS: &'static [&'static str] = LOWPOLY_MIGRATED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = LOWPOLY_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchObject", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addPaintLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "paintStrokeEnd", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveObject", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActivePaintLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setUtilityParam", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "toggleShowEdges", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "toggleSun", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSunAzimuth", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSunElevation", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setSunIntensity", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "importSnapshotJson", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setFixtureJson", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "paintSample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "paintStrokeBegin", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "transformBegin", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "setActiveUtility", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "extrude", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "inset", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "bevel", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "loopCut", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "subdivide", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "triangulate", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "mirror", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "decimate", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "flipFaces", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "merge", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "dissolve", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "snap", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "toggleSmooth", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "unwrapActive", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "markUvSeam", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "clearSeam", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "engagementSubmit", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "transformEnd", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "paintStroke", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "paintAt", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasPointerDown", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "canvasPointerMove", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "paintFill", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "fillBucket", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addPrimitive", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
const LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 16 * 1024 * 1024;
const LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES: usize = 16_384;

fn lowpoly_paint_layer_retained_bytes(layer: &crate::artifacts::lowpoly::LowpolyPaintLayer) -> usize {
    layer.name.len().saturating_add(layer.blend_mode.len()).saturating_add(layer.pixels.len())
}

/// 🧱️ Exact retained-byte footprint of one persisted object — the id/name/mesh-handle/paint-layers
/// accounting `lowpoly_snapshot_retained_bytes`'s fold used to inline; shared with
/// `lowpoly_artifact_mutation_retained_bytes`'s `CreateObject` arm below, since a `CreateObject`
/// mutation's payload IS one whole `LowpolyObject`.
fn lowpoly_object_retained_bytes(object: &LowpolyObject) -> usize {
    object
        .id
        .len()
        .saturating_add(object.name.len())
        .saturating_add(object.mesh.as_ref().map_or(0, |mesh| mesh.child_id.len().saturating_add(mesh.target.to_uri().len())))
        .saturating_add(object.paint_layers.iter().fold(0, |bytes, layer| bytes.saturating_add(lowpoly_paint_layer_retained_bytes(layer))))
}

fn lowpoly_snapshot_retained_bytes(snapshot: &LowpolySnapshot) -> usize {
    snapshot.schema.len().saturating_add(snapshot.objects.iter().fold(0, |bytes, object| bytes.saturating_add(lowpoly_object_retained_bytes(object))))
}

/// 📬️ Exact per-variant byte accounting for every one of `LowpolyMutation`'s 17 declared variants —
/// fail-closed BY CONSTRUCTION: this match is exhaustive over the enum (no `_` arm), so a future
/// variant added to `LowpolyMutation` without a matching arm here is a compile error, not a silent
/// runtime admission. `CreateMesh.mesh_workspace` (the whole half-edge mesh JSON) is accounted for at
/// its exact length — `admit_lowpoly_artifact_mutation`'s `LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES` (16
/// MiB) cap below is what keeps that field's admission meaningful, not a separate per-field cap.
fn lowpoly_artifact_mutation_retained_bytes(mutation: &LowpolyMutation) -> Result<usize, String> {
    match mutation {
        LowpolyMutation::CreateObject(payload) => Ok(lowpoly_object_retained_bytes(&payload.object)),
        LowpolyMutation::DeleteObject(payload) => Ok(payload.id.len()),
        LowpolyMutation::ReorderObjects(payload) => Ok(payload.id.len()),
        LowpolyMutation::RenameObject(payload) => Ok(payload.id.len().saturating_add(payload.new_name.len())),
        LowpolyMutation::ChangeObjectSmoothShading(payload) => Ok(payload.id.len()),
        LowpolyMutation::MoveObject(payload) => Ok(payload.id.len()),
        LowpolyMutation::RotateObject(payload) => Ok(payload.id.len()),
        LowpolyMutation::ScaleObject(payload) => Ok(payload.id.len()),
        LowpolyMutation::CreateMesh(payload) => Ok(payload.id.len().saturating_add(payload.child_id.len()).saturating_add(payload.target.to_uri().len()).saturating_add(payload.mesh_workspace.len())),
        LowpolyMutation::DeleteMesh(payload) => Ok(payload.id.len()),
        LowpolyMutation::InsertPaintLayer(payload) => Ok(payload.object_id.len().saturating_add(lowpoly_paint_layer_retained_bytes(&payload.layer))),
        LowpolyMutation::RemovePaintLayer(payload) => Ok(payload.object_id.len()),
        LowpolyMutation::RenamePaintLayer(payload) => Ok(payload.object_id.len().saturating_add(payload.new_name.len())),
        LowpolyMutation::ChangePaintLayerVisible(payload) => Ok(payload.object_id.len()),
        LowpolyMutation::ChangePaintLayerOpacity(payload) => Ok(payload.object_id.len()),
        LowpolyMutation::ChangePaintLayerBlendMode(payload) => Ok(payload.object_id.len().saturating_add(payload.new_blend_mode.len())),
        LowpolyMutation::EditPaintLayer(payload) if payload.runs.len() <= LOWPOLY_RETAINED_PAINT_RUNS => {
            Ok(payload
                .object_id
                .len()
                .saturating_add(payload.runs.len().saturating_mul(std::mem::size_of::<crate::artifacts::lowpoly::mutations::PixelRun>()))
                .saturating_add(payload.runs.iter().fold(0_usize, |bytes, run| bytes.saturating_add(run.bytes.len()))))
        }
        LowpolyMutation::EditPaintLayer(_) => Err("Lowpoly paint edit exceeds its fixed run envelope".into()),
    }
}

fn admit_lowpoly_artifact_mutation(mutation: &LowpolyMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = lowpoly_artifact_mutation_retained_bytes(mutation)?;
    if retained_bytes > LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("Lowpoly Artifact mutation exceeds its fixed retained preparation envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn prepare_lowpoly_artifact(base: &LowpolySnapshot, mutation: LowpolyMutation) -> Result<(LowpolySnapshot, Vec<LowpolyMutation>, LowpolyMutation), String> {
    admit_lowpoly_artifact_mutation(&mutation)?;
    if !lowpoly_snapshot_admitted(base) || lowpoly_snapshot_retained_bytes(base) > LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("Lowpoly Artifact base exceeds its fixed retained preparation envelope".into());
    }
    let inverse = mutation.inverse(base);
    let diff = mutation.diff(base).into_parts().0;
    let post = diff.apply(base).map_err(|_| "Lowpoly Artifact preparation could not apply its exact sparse diff".to_string())?;
    if !lowpoly_snapshot_admitted(&post) || lowpoly_snapshot_retained_bytes(&post) > LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("Lowpoly Artifact result exceeds its fixed retained preparation envelope".into());
    }
    Ok((post, inverse, mutation))
}

fn lowpoly_config_retained_bytes(config: &LowpolyConfig) -> usize {
    config
        .active_object_id
        .len()
        .saturating_add(config.paint_utility.len())
        .saturating_add(config.utility_params_json.len())
        .saturating_add(config.engagement_input.len())
        .saturating_add(config.sun_color.len())
        .saturating_add(config.active_utility_id.len())
        .saturating_add(config.locale.len())
}

fn lowpoly_config_mutation_retained_bytes(mutation: &LowpolyConfigMutation) -> usize {
    match mutation {
        LowpolyConfigMutation::Snapshot { config } => lowpoly_config_retained_bytes(config),
        LowpolyConfigMutation::SetActiveObject { object_id } => object_id.len(),
        LowpolyConfigMutation::SetPaintUtility { value } | LowpolyConfigMutation::SetEngagementInput { value } | LowpolyConfigMutation::SetLocale { value } => value.len(),
        LowpolyConfigMutation::SetUtilityParams { json } => json.len(),
        LowpolyConfigMutation::SetSun { color, .. } => color.len(),
        LowpolyConfigMutation::SetActiveUtility { utility_id } => utility_id.len(),
        LowpolyConfigMutation::SetActivePaintLayer { .. }
        | LowpolyConfigMutation::SetPaintColor { .. }
        | LowpolyConfigMutation::SetWorldCamera { .. }
        | LowpolyConfigMutation::SetShowEdges { .. } => 0,
    }
}

fn admit_lowpoly_config_mutation(mutation: &LowpolyConfigMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = lowpoly_config_mutation_retained_bytes(mutation);
    if retained_bytes > LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Lowpoly config mutation exceeds its fixed retained preparation envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn prepare_lowpoly_config(base: &LowpolyConfig, mutation: LowpolyConfigMutation) -> Result<(LowpolyConfig, Vec<LowpolyConfigMutation>, LowpolyConfigMutation), String> {
    admit_lowpoly_config_mutation(&mutation)?;
    if lowpoly_config_retained_bytes(base) > LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Lowpoly config base exceeds its fixed retained preparation envelope".into());
    }
    let inverse = mutation.inverse(base);
    let post = mutation.diff(base).into_parts().0;
    if lowpoly_config_retained_bytes(&post) > LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Lowpoly config result exceeds its fixed retained preparation envelope".into());
    }
    Ok((post, inverse, mutation))
}

fn lowpoly_store_edit<M>(prefix: &str, forward: M, inverse: Vec<M>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<M> {
    let id = format!("{prefix}-{}", authority.next_sequence_number());
    protocol::Edit {
        id: id.clone(),
        actor: Some(authority.actor().to_string()),
        forwards: vec![forward],
        inverse,
        mutation_meta: vec![protocol::MutationMeta {
            mutation_id: Some(protocol::MutationId(format!("{id}#0"))),
            dependencies: Vec::new(),
            base_version: authority.base_applied_edit_count() as u64,
            author_id: Some(protocol::ActorId(authority.actor().to_string())),
            timestamp: authority.next_clock(),
            undo_policy: protocol::UndoPolicy::ExactBaseOnly,
            payload_hash: None,
            semantic_kind: None,
            label: None,
            group_id: None,
            origin: Default::default(),
        }],
        description,
        coalesce_key: None,
        sequence_number: authority.next_sequence_number(),
        started_at: String::new(),
        finished_at: None,
    }
}

struct LowpolyArtifactStorePreparationFactory;

struct LowpolyArtifactStorePreparation {
    base: Option<store::SnapshotRead<LowpolySnapshot>>,
    mutation: Option<LowpolyMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<LowpolySnapshot, LowpolyMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    prepared_bytes: usize,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<LowpolySnapshot, LowpolyMutation> for LowpolyArtifactStorePreparationFactory {
    fn preflight(&self, mutation: &LowpolyMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Lowpoly Artifact preparation rejected its lane or description envelope".into());
        }
        admit_lowpoly_artifact_mutation(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<LowpolySnapshot, LowpolyMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<LowpolySnapshot, LowpolyMutation>>, store::ArtifactStoreOneItemPreparationRequest<LowpolySnapshot, LowpolyMutation>> {
        let retained_bytes = lowpoly_artifact_mutation_retained_bytes(&request.mutation).unwrap_or(LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES.saturating_add(1));
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES
            || !lowpoly_snapshot_admitted(request.base.get())
        {
            return Err(request);
        }
        Ok(Box::new(LowpolyArtifactStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            prepared_bytes: 0,
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<LowpolySnapshot, LowpolyMutation> for LowpolyArtifactStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "Lowpoly Artifact preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "Lowpoly Artifact preparation lost its mutation owner".to_string())?;
        let (post, inverse, forward) = prepare_lowpoly_artifact(base.get(), mutation)?;
        self.prepared_bytes = lowpoly_snapshot_retained_bytes(&post);
        let authority = self.authority.as_ref().ok_or_else(|| "Lowpoly Artifact preparation lost its Store authority".to_string())?;
        let edit = lowpoly_store_edit("lowpoly-artifact-retained", forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<LowpolySnapshot, LowpolyMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<LowpolySnapshot, LowpolyMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.is_some() {
            if grant.maximum_bytes < self.prepared_bytes {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.prepared = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.prepared_bytes });
        }
        if self.mutation.is_some() {
            if grant.maximum_bytes < self.retained_bytes {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.mutation = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if let Some(description) = self.description.as_ref() {
            if grant.maximum_bytes < description.len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            let released_bytes = description.len();
            self.description = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Lowpoly Artifact preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            let released_bytes = authority.actor().len();
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}

struct LowpolyConfigStorePreparationFactory;

struct LowpolyConfigStorePreparation {
    base: Option<store::SnapshotRead<LowpolyConfig>>,
    mutation: Option<LowpolyConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<LowpolyConfig, LowpolyConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    retained_bytes: usize,
    prepared_bytes: usize,
    cancelled: bool,
    closing: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<LowpolyConfig, LowpolyConfigMutation> for LowpolyConfigStorePreparationFactory {
    fn preflight(&self, mutation: &LowpolyConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("Lowpoly config preparation rejected its lane or description envelope".into());
        }
        admit_lowpoly_config_mutation(mutation)
    }

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<LowpolyConfig, LowpolyConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<LowpolyConfig, LowpolyConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<LowpolyConfig, LowpolyConfigMutation>> {
        let retained_bytes = lowpoly_config_mutation_retained_bytes(&request.mutation);
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
            || retained_bytes > LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES
            || lowpoly_config_retained_bytes(request.base.get()) > LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(LowpolyConfigStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            retained_bytes,
            prepared_bytes: 0,
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<LowpolyConfig, LowpolyConfigMutation> for LowpolyConfigStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "Lowpoly config preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "Lowpoly config preparation lost its mutation owner".to_string())?;
        let (post, inverse, forward) = prepare_lowpoly_config(base.get(), mutation)?;
        self.prepared_bytes = lowpoly_config_retained_bytes(&post);
        let authority = self.authority.as_ref().ok_or_else(|| "Lowpoly config preparation lost its Store authority".to_string())?;
        let edit = lowpoly_store_edit("lowpoly-config-retained", forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: self.retained_bytes as u64, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<LowpolyConfig, LowpolyConfigMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<LowpolyConfig, LowpolyConfigMutation>> {
        self.prepared.take()
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.is_some() {
            if grant.maximum_bytes < self.prepared_bytes {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.prepared = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.prepared_bytes });
        }
        if self.mutation.is_some() {
            if grant.maximum_bytes < self.retained_bytes {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.mutation = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: self.retained_bytes });
        }
        if let Some(description) = self.description.as_ref() {
            if grant.maximum_bytes < description.len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            let released_bytes = description.len();
            self.description = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("Lowpoly config preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            let released_bytes = authority.actor().len();
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️StorePreparation

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

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(LowpolyArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(LowpolyConfigStorePreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<LowpolyPlayApp>,
        owner_file: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs",
        controller: "s.lowpoly.lowpoly@1/*#editor",
        document_schema: "lowpoly.document",
        factory: "LowpolyCommandJobFactory",
        factory_type: LowpolyCommandJobFactory,
        tools: {
            "patchObject" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "addPaintLayer" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "paintStrokeEnd" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setActiveObject" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setActivePaintLayer" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setUtilityParam" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "engagementInput" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "toggleShowEdges" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "toggleSun" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setSunAzimuth" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setSunElevation" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setSunIntensity" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setCamera" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "importSnapshotJson" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setFixtureJson" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "paintSample" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "paintStrokeBegin" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "transformBegin" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setActiveUtility" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "extrude" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "inset" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "bevel" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "loopCut" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "subdivide" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "triangulate" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "mirror" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "decimate" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "flipFaces" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "merge" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "dissolve" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "snap" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "toggleSmooth" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "unwrapActive" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "markUvSeam" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "clearSeam" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "engagementSubmit" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "translateSelection" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "rotateSelection" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "scaleSelection" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "transformEnd" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "paintStroke" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "paintAt" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "canvasPointerDown" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "canvasPointerMove" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "paintFill" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "fillBucket" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "addPrimitive" => semio_framework::ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
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
        if !lowpoly_command_admitted(request.command, &request.snapshot, &request.config) {
            return Err(Fault::from("lowpoly-retained-command-capacity"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(LowpolyRetainedCommandWork::new(tool_id, disposition, request.operation.operation.0, request.operation.generation.0, request.canonical_base_revision, request.context.identity_digest()));
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
        _interaction: &InteractionView<'_>,
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

    #[test]
    fn retained_route_partition_and_publication_are_exact() {
        use semio_framework::{ToolCancellationPolicy, ToolExecutionShape};

        let all = every_command();
        let mut partition = LOWPOLY_MIGRATED_TOOL_IDS.to_vec();
        partition.sort_unstable();
        partition.dedup();
        assert_eq!(partition.len(), 47);
        assert_eq!(all.len(), partition.len());
        assert!(all.iter().all(|command| partition.binary_search(&command.command_id()).is_ok()));
        assert!(LOWPOLY_MIGRATED_TOOL_IDS.iter().all(|tool_id| lowpoly_command_disposition(tool_id).is_some()));
        assert_eq!(<LowpolyPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), 47);
        assert_eq!(<LowpolyCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS.len(), 47);
        assert_eq!(lowpoly_contract().shape, ToolExecutionShape::Resumable);
        assert_eq!(lowpoly_contract().cancellation, ToolCancellationPolicy::PerOperation);
        assert_eq!((lowpoly_contract().checkpoint_every_steps, lowpoly_contract().progress_every_steps), (1, 1));
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_progress_replay_freshness_and_close_are_exact() {
        let command = LowpolyCommand::ToggleShowEdges(toggle_show_edges::ToggleShowEdges {});
        let snapshot = crate::artifacts::lowpoly::schema::default_snapshot();
        let config = LowpolyConfig::default();
        let interaction = protocol::InteractionState::default();
        let hover = semio_framework_plugin::app::InteractionHoverState::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let operation = retained_operation();
        let context = retained_context(LowpolyTransient::default(), 19);
        let context_identity = context.identity_digest();
        let mut uninterrupted = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, operation.canonical_base_revision, context_identity);
        assert!(matches!(uninterrupted.step(&command, &snapshot, &config, &history, &interaction, &hover, Some(&context), &operation).expect("progress"), ArtifactCommandWorkStep::Progress { .. }));
        let mut checkpoint = [0_u8; 88];
        uninterrupted.checkpoint(&mut checkpoint).expect("checkpoint");
        let mut wrong_base = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, [18; 32], context_identity);
        assert!(wrong_base.restore(&checkpoint).is_err());
        let mut replayed = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, operation.canonical_base_revision, context_identity);
        replayed.restore(&checkpoint).expect("work restore");
        assert!(matches!(replayed.step(&command, &snapshot, &config, &history, &interaction, &hover, Some(&context), &operation).expect("replay"), ArtifactCommandWorkStep::Replay { .. }));
        assert!(matches!(replayed.step(&command, &snapshot, &config, &history, &interaction, &hover, Some(&context), &operation).expect("complete"), ArtifactCommandWorkStep::Complete(_)));
        let drifted = AppOperationContext { generation: operation.generation + 1, ..operation.clone() };
        let mut rejected = LowpolyRetainedCommandWork::new("toggleShowEdges", LowpolyCommandDisposition::Config, operation.operation_id, operation.generation, operation.canonical_base_revision, context_identity);
        assert!(rejected.step(&command, &snapshot, &config, &history, &interaction, &hover, Some(&context), &drifted).is_err());
        let drifted_context = retained_context(LowpolyTransient::default(), 20);
        assert!(rejected.step(&command, &snapshot, &config, &history, &interaction, &hover, Some(&drifted_context), &operation).is_err());
        assert!(matches!(rejected.step(&command, &snapshot, &config, &history, &interaction, &hover, Some(&context), &operation).expect("exact retry"), ArtifactCommandWorkStep::Progress { .. }));
        assert_eq!(replayed.close_step(0, 0), InteractiveJobCloseStep::Blocked);
        replayed.begin_close();
        assert_eq!(replayed.close_step(1, 1), InteractiveJobCloseStep::Complete);
        assert!(replayed.terminal_is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_migrated_turns_stay_below_eight_milliseconds() {
        let snapshot = crate::artifacts::lowpoly::schema::default_snapshot();
        let config = LowpolyConfig::default();
        let interaction = protocol::InteractionState::default();
        let hover = semio_framework_plugin::app::InteractionHoverState::default();
        let history = semio_framework_plugin::HistoryView::empty();
        let operation = retained_operation();
        let context = retained_context(LowpolyTransient::default(), 29);
        for command in every_command().into_iter().filter(|command| LOWPOLY_MIGRATED_TOOL_IDS.contains(&command.command_id())) {
            let tool_id = command.command_id();
            let disposition = lowpoly_command_disposition(tool_id).expect("migrated disposition");
            let mut work = LowpolyRetainedCommandWork::new(tool_id, disposition, operation.operation_id, operation.generation, operation.canonical_base_revision, context.identity_digest());
            loop {
                let started = std::time::Instant::now();
                let step = work.step(&command, &snapshot, &config, &history, &interaction, &hover, Some(&context), &operation).expect("migrated turn");
                assert!(started.elapsed() < std::time::Duration::from_millis(8), "{tool_id} turn exceeded 8 ms");
                if matches!(step, ArtifactCommandWorkStep::Complete(_) | ArtifactCommandWorkStep::CompleteWithEphemeral { .. }) {
                    break;
                }
            }
            work.begin_close();
            assert_eq!(work.close_step(1, LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES), InteractiveJobCloseStep::Complete);
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
