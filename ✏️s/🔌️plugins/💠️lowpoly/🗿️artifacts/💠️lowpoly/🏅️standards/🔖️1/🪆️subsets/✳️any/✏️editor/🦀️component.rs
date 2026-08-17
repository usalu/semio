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

use crate::editor::lowpoly::commands::{add_primitive, camera, chrome, engagement, fixture, mesh_edit, patch_object, paint, selection, sun, transform, utility, uv};
use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation};
use crate::editor::lowpoly::modes::{edit, paint as paint_mode};
use crate::editor::lowpoly::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel, layers as layers_panel};
use crate::editor::lowpoly::session::LowpolyScratch;
use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::{resolve_active_object_id, selection_from_interaction, utility_param_f64, LowpolyView, MESH_INTERACTION_DOMAIN};
use crate::artifacts::lowpoly::op::LowpolyMutation;
use crate::artifacts::lowpoly::{artifact_kind, LowpolySnapshot, LOWPOLY_DOCUMENT_SCHEMA};
use semio_framework_plugin::{NoDraft, NoDraftMutation, DraftView,
    ActionArgDef, ActionArgOption, ActionDescriptor, ActionRef, ArtifactEditor, ConfigView, ArtifactView, Editor, Emit, Fault, LabelText, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, UiNode, UtilityCategory,
    UtilityDefinition, WindowEngagement, WindowEngagementInput, WindowEngagementOption, WindowEngagementPossible, WindowEngagementStatus, WindowMeasure,
    GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, MergeMode, SelectionMethod, SelectionMode, SelectionSpec,
};
use semio_framework_plugin::app::InteractionView;
use store::EngineHandles;
use serde_json::{json, Value};
use std::collections::HashMap;
use store::ArtifactPack;

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
pub fn lowpoly_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(LOWPOLY_PLAY_CONTROLLER_ID).action(action, args)
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

//#region 🔖️ScratchSlot
thread_local! {
    /// 🖌️ Mid-gesture scratch survives across `ArtifactApp::handle` calls (associated fn has no `&mut self`).
    /// Host-owned session scratch lands with CHANNEL_VERSION 5; until then one TLS slot per thread.
    static LOWPOLY_SCRATCH: std::cell::RefCell<LowpolyScratch> = std::cell::RefCell::new(LowpolyScratch::default());
}
//#endregion 🔖️ScratchSlot


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
    WindowMeasure::Slider { id: format!("lowpoly-measure-{id}"), label: Some(label.into()), value: utility_param_f64(params, key, default), min, max, step: Some(step), ready: None, loading: None, disabled: None, reveal: None, on_change: lowpoly_action("setUtilityParam", Some(json!({ "key": key }))), waiting: None }
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
use mesh_edit::{bevel, decimate, dissolve, extrude, flip_faces, inset, loop_cut, merge, mirror, snap, subdivide, toggle_smooth, triangulate};
use uv::{clear_seam, mark_uv_seam, unwrap_active};
use transform::{rotate_selection, scale_selection, transform_begin, transform_end, translate_selection};
use paint::{add_paint_layer, canvas_pointer_down, canvas_pointer_move, fill_bucket, paint_at, paint_fill, paint_sample, paint_stroke, paint_stroke_begin, paint_stroke_end};
use selection::{set_active_object, set_active_paint_layer};
use camera::set_camera;
use sun::{set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use utility::{set_active_utility, set_utility_param};
use engagement::{engagement_input, engagement_submit};
use fixture::{set_fixture_json, set_snapshot_json};
use chrome::toggle_show_edges;
//#endregion 🔖️Commands

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
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;

    type Command = LowpolyCommand;

    const DIALECT: semio_framework_plugin::app::Dialect = crate::artifacts::lowpoly::LOWPOLY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = LOWPOLY_DOCUMENT_SCHEMA;

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
        match port {
            "mesh:out" => {
                let mesh_workspace = LOWPOLY_SCRATCH.with(|scratch| scratch.borrow().mesh_workspace_map());
                let document_json = serde_json::to_value(doc.snapshot).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
                let mesh = crate::editor::lowpoly::engine::lowpoly_mesh_from_document(&document_json, &mesh_workspace).map_err(|error| MediaError::Payload(port.into(), error))?;
                let mesh_document = crate::artifacts::lowpoly::schema::mesh_document_from_mesh(&mesh).map_err(|error| MediaError::Payload(port.into(), error))?;
                let json = serde_json::to_string(&mesh_document).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
                Ok(Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: MediaPayload::Structured { schema: "mesh.document".into(), json } })
            }
            "document:out" => {
                let media_type = Self::io().map_or(MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, |io| io.document_media_type);
                let bytes = doc.snapshot.encode_pack();
                Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
            }
            _ => Err(MediaError::NotImplemented),
        }
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
    fn handle(command: &LowpolyCommand, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, interaction: &InteractionView<'_>, _draft: &DraftView<'_, Self::Draft>, _engines: &EngineHandles) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation, Self::DraftMutation>, Fault> {
        let active = resolve_active_object_id(doc.snapshot, cfg.snapshot);
        let selection = selection_from_interaction(&active, interaction);
        LOWPOLY_SCRATCH.with(|scratch| {
            scratch.borrow_mut().set_current_selection(selection);
            command.dispatch(doc, cfg, &mut scratch.borrow_mut())
        })
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>) -> UiNode {
        let projection = doc.snapshot;
        let config = cfg.snapshot;
        let labels = crate::editor::lowpoly::terminology::lowpoly_play_labels(config);
        let active_utility = config.active_utility_id.as_str();
        let (scratch_projection, texture_cache) = LOWPOLY_SCRATCH.with(|scratch| {
            let mut scratch = scratch.borrow_mut();
            if matches!(body_key, LOWPOLY_PLAY_BODY_MAIN | LOWPOLY_PLAY_BODY_UV) {
                scratch.refresh_texture_cache(projection);
            }
            (scratch.transform_projection(), scratch.textures().clone())
        });
        let render_projection = scratch_projection.as_ref().unwrap_or(projection);
        let view = LowpolyView { snapshot: render_projection, config };
        let loaded = matches!(body_key, LOWPOLY_PLAY_BODY_MAIN | LOWPOLY_PLAY_BODY_UV | LOWPOLY_PLAY_BODY_DOCUMENT)
            .then(|| LOWPOLY_SCRATCH.with(|scratch| crate::editor::lowpoly::view::build_doc(projection, config, &scratch.borrow())))
            .flatten();
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
            .mutation("setSnapshotJson", LocalizedLabel::native("Set Projection Json", "Projektions-JSON festlegen"))
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

    pub fn dispatch(app: &mut LowpolyApp, command: LowpolyCommand) -> InvocationResult {
        app.dispatch_typed(command, &meta("local")).expect("dispatch")
    }

    pub fn render(app: &mut LowpolyApp, body_key: &str) -> String {
        serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).expect("render")).expect("render json")
    }

    /// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: picking is now the framework's
    /// injected `interactionSelect` verb, dispatched against the "mesh" domain declared on this app —
    /// requires `app_with_registry()` (a bare `app()` has no declared interaction domains to select
    /// against). `object_id`/`face_id` address the same row id the Document panel tree renders (see
    /// `🧭️view/🦀️component.rs`'s `🔖️MeshDomain` region).
    pub fn select_face(app: &mut LowpolyApp, object_id: &str, face_id: u32) {
        let target_id = crate::editor::lowpoly::view::document_target_row_id(object_id, 0, "face", face_id);
        let targets = serde_json::to_string(&serde_json::json!([{ "granularity": "face", "id": target_id }])).expect("targets json");
        app.handle_action("interactionSelect", Some(&serde_json::json!({ "domainId": crate::editor::lowpoly::view::MESH_INTERACTION_DOMAIN, "targets": targets, "merge": "replace" })), &meta("test")).expect("interactionSelect");
    }
}
//#endregion 🧪️Testkit

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::lowpoly::testkit::{app, app_with_registry, LowpolyApp};
    use semio_framework_plugin::{testkit, EditorApp, PluginApp};

    //#region 🔖️CommandSurface
    /// 🏷️ Every declared manifest action id must be reachable as exactly one command row, and every row's
    /// wire keyword must be distinct — the cross-cutting invariant `app_commands!` is there to hold.
    #[test]
    fn command_ids_are_unique() {
        let commands = every_command();
        let ids: Vec<&str> = commands.iter().map(|command| command.command_id()).collect();
        let mut sorted = ids.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
        assert_eq!(ids.len(), 47, "every LowpolyCommand row must be covered by every_command()");
    }

    /// ⚖️ LAW: text and binary are two projections of the same command, for every single row.
    #[test]
    fn every_command_round_trips_through_text_and_binary() {
        for command in every_command() {
            semio_framework_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
        }
    }

    /// ⚖️ LAW: the leading token of every printed op line is the row's `dsl` wire keyword.
    #[test]
    fn every_printed_op_line_starts_with_the_rows_wire_keyword() {
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
    #[test]
    fn the_manifest_stitches_every_taxonomy_node() {
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
    #[test]
    fn the_mesh_interaction_domain_is_declared_and_scoped_to_the_model_window() {
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
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        testkit::assert_two_instances_converge::<EditorApp<LowpolyPlayApp>, _>(
            "mem://lowpoly-convergence",
            LowpolyCommand::PatchObject(patch_object::PatchObject { object_id: "obj-1".into(), field: "name".into(), value_json: Some(serde_json::to_string("Renamed By A").unwrap()) }),
            LowpolyCommand::AddPrimitive(add_primitive::AddPrimitive { kind: Some("box".into()) }),
            |app| app.snapshot().expect("projection"),
        );
    }

    #[test]
    fn ingest_operations_is_idempotent() {
        testkit::assert_ingest_idempotent::<EditorApp<LowpolyPlayApp>, _>(LowpolyCommand::PatchObject(patch_object::PatchObject { object_id: "obj-1".into(), field: "name".into(), value_json: Some(serde_json::to_string("Hero").unwrap()) }), |app| app.snapshot().expect("projection"));
    }

    #[test]
    fn an_unknown_body_key_renders_a_diagnostic_instead_of_panicking() {
        use crate::editor::lowpoly::testkit::render;
        let mut a = app();
        assert!(render(&mut a, "lowpoly.play.nope").contains("Unknown body"));
    }
    //#endregion 🔖️CrossCutting

    //#region 🔖️MediaPorts
    #[test]
    fn export_media_mesh_out_produces_mesh_document_payload() {
        let mut a: LowpolyApp = app();
        let media = a.export_media("mesh:out").expect("export mesh:out");
        assert_eq!(media.media_type, MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh });
        match media.payload {
            MediaPayload::Structured { schema, .. } => assert_eq!(schema, "mesh.document"),
            other => panic!("expected Structured payload, got {other:?}"),
        }
    }

    /// 🧬️ `"mesh:in"` replaces the whole document via `reset_document_effect` (a
    /// `Effect::LoadDocument`, outside undo history) — whole-document replace has no replacement
    /// mutation per `📓️taxonomy.md`, so this is an effect, not an `artifact_mutations` entry.
    #[test]
    fn import_media_mesh_in_round_trips_into_a_reset_document_effect() {
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
    #[test]
    fn registry_wired_app_dispatches_add_primitive() {
        let mut a = app_with_registry();
        crate::editor::lowpoly::testkit::dispatch(&mut a, LowpolyCommand::AddPrimitive(add_primitive::AddPrimitive { kind: Some("plane".into()) }));
        assert_eq!(a.snapshot().expect("projection").objects.len(), 2);
    }
    //#endregion 🔖️ContextMenuRegistry
}
//#endregion 🧪️Tests
