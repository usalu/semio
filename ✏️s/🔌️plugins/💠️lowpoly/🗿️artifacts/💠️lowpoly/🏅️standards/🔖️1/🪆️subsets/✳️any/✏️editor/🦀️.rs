//! 🖌️ Lowpoly editor — the `ArtifactEditor` impl (dispatch-only), the aggregated command enum and the
//! manifest stitch.
//!
//! Everything substantive lives in a taxonomy node: command bodies in `🎮️commands/*`, window renders in
//! `🎭️modes/*/🪟️windows/*`, chrome measures/engagement shared by both windows in this file (they are
//! byte-identical between windows — see the master ticket's TEMPLATE.md §12.2 shared-options pattern,
//! extended here across mode boundaries since the Model window is reused by both `edit` and `paint`),
//! panel trees in `📌️panels/*`, labels in `🗣️terminology/🦀️.rs`, view state in
//! `🎚️config/🦀️.rs`, scratch (mid-gesture) state in `🖌️session/🦀️.rs`, shared
//! read-view/selection helpers in `🧭️view/🦀️.rs`.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

use semio_s_artifact_lowpoly_lowpoly_commands::{paint::LowpolyPaintTick, paint::lowpoly_paint_tick};
use semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::session::lowpoly_paint_drive;
use semio_s_artifact_lowpoly_lowpoly_mut_paint::{apply_paint_stroke::ApplyPaintStroke, edit_paint_layer::EditPaintLayer};
use semio_framework_tool_machine::GesturePhase;

use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceText;
pub use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, HistoryView};

use semio_s_artifact_lowpoly_lowpoly_commands::{add_primitive, camera, chrome, engagement, document, media, mesh_edit, object, paint, patch_object, selection, sun, transform, utility, uv};
use semio_s_artifact_lowpoly_lowpoly_config::{LowpolyConfig, LowpolyConfigMutation, SetActiveObjectEdit, SetPaintUtilityEdit, SetUtilityParamsEdit, SetPaintColorEdit, SetEngagementInputEdit, SetSunEdit};
use semio_s_artifact_lowpoly_lowpoly_modes::{self, editor::lowpoly::modes::paint as paint_mode};
use semio_s_artifact_lowpoly_lowpoly_panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel, layers as layers_panel};
use semio_s_artifact_lowpoly_lowpoly_view::{editor::lowpoly::session::LowpolyScratch, editor::lowpoly::session::LowpolyTransient, editor::lowpoly::session::LowpolyTransientMutation};
use semio_s_artifact_lowpoly_lowpoly_view::editor::lowpoly::terminology::LowpolyLabels;
use semio_s_artifact_lowpoly_lowpoly_view::{resolve_active_object_id, selection_from_interaction, selection_from_state, utility_param_f64, LowpolyView, MESH_GRANULARITY_OBJECT, MESH_INTERACTION_DOMAIN};
use semio_s_artifact_lowpoly_lowpoly_mutations::LowpolyMutation;
use semio_s_artifact_lowpoly_lowpoly_model::{snapshot::artifact_kind, snapshot::LowpolyObject, snapshot::LowpolySnapshot, snapshot::LOWPOLY_DOCUMENT_SCHEMA};
use protocol::Mutation;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_job::InteractiveJobCloseStep;
use semio_framework_plugin::app::{ArtifactOwnedToolJobContext, InteractionView};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactCommandWorkStep, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload};
use semio_framework_plugin::ActionArgDef;
use semio_framework_plugin::ActionArgOption;
use semio_framework_plugin::ActionDescriptor;
use semio_framework_plugin::ActionRef;
use semio_framework_plugin::AppOperationContext;
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin::ArtifactOwnedToolJobFactory;
use semio_framework_plugin::ArtifactOwnedToolJobRequest;
use semio_framework_plugin::ArtifactToolFactoryRegistry;
use semio_framework_plugin::DraftView;
use semio_framework_plugin::Editor;
use semio_framework_plugin::EditorApp;
use semio_framework_plugin::EphemeralEmit;
use semio_framework_plugin::GranularityDefinition;
use semio_framework_plugin::HierarchyProvider;
use semio_framework_plugin::HoverSpec;
use semio_framework_plugin::InteractionDefinition;
use semio_framework_ui_locale::LabelText;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::Media;
use semio_framework_plugin::MediaClass;
use semio_framework_plugin::MediaError;
use semio_framework_plugin::MediaForm;
use semio_framework_plugin::MediaPayload;
use semio_framework_plugin::MediaType;
use semio_framework_plugin::MergeMode;
use semio_framework_plugin::NoDraft;
use semio_framework_plugin::NoDraftMutation;
use semio_framework_plugin::SelectionMethod;
use semio_framework_plugin::SelectionMode;
use semio_framework_plugin::SelectionSpec;
use semio_framework_plugin::UtilityCategory;
use semio_framework_plugin::UtilityDefinition;
use semio_framework_plugin::WindowEngagement;
use semio_framework_plugin::WindowEngagementInput;
use semio_framework_plugin::WindowEngagementOption;
use semio_framework_plugin::WindowEngagementPossible;
use semio_framework_plugin::WindowEngagementStatus;
use semio_framework_plugin::WindowMeasure;
use std::collections::HashMap;
use pack::snapshot::ArtifactPack;
use semio_framework_2d_compute::EngineHandles;

//#region 🔖️Constants

//#endregion 🔖️Constants

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — mirrors the `ArtifactKindSpec` literal
/// `crate::artifact_kind()` declares for `"3d.lowpoly"`, plus the two workflow
/// ports: `mesh:in` (Many, unrequired — accepts upstream mesh producers, e.g. cad via a Brep→Mesh
/// conversion) and `mesh:out` (Many, unrequired). Relocated from the deleted
/// `🗿️artifacts/💠️lowpoly/…/⚙️engine/🦀️.rs` (ticket
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
        artifact_schema: LOWPOLY_DOCUMENT_SCHEMA.into(),
        artifact_media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh },
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
        "paintFill" as "paint-fill" => paint_fill::PaintFill,
        "fillBucket" as "fill-bucket" => fill_bucket::FillBucket,
        "importSnapshotJson" as "import-snapshot-json" => set_snapshot_json::ImportSnapshotJson,
        "replaceSnapshotJson" as "replace-snapshot-json" => replace_snapshot_json::ReplaceSnapshotJson,
        "exportMesh" as "export-mesh" => export_mesh::ExportMesh,
        "loadMeshRequest" as "load-mesh-request" => load_mesh_request::LoadMeshRequest,
        "importMeshFile" as "import-mesh-file" => import_mesh_file::ImportMeshFile,
        "deleteSelection" as "delete-selection" => delete_selection::DeleteSelection,
        "duplicateObject" as "duplicate-object" => duplicate_object::DuplicateObject,
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
        "paintSample" as "paint-sample" => paint_sample::PaintSample,
        "paintStroke" as "paint-stroke" => paint_stroke::PaintStroke,
        "paintAt" as "paint-at" => paint_at::PaintAt,
        "canvasPointerDown" as "canvas-pointer-down" => canvas_pointer_down::CanvasPointerDown,
        "canvasPointerMove" as "canvas-pointer-move" => canvas_pointer_move::CanvasPointerMove,
        "canvasPointerUp" as "canvas-pointer-up" => canvas_pointer_up::CanvasPointerUp,
    }
}

// 🧷️ `app_commands!` addresses each payload module by a single identifier, so every `🎮️commands/*`
// payload module is imported here under its own flat name. `mesh_edit`/`uv`/`transform`/`paint`/
// `selection`/`sun`/`utility`/`engagement`/`fixture` collide with their containing command-group
// modules and are flattened via glob-free explicit `use`.
use camera::set_camera;
use chrome::toggle_show_edges;
use engagement::{engagement_input, engagement_submit};
use document::{replace_snapshot_json, set_snapshot_json};
use media::{export_mesh, import_mesh_file, load_mesh_request};
use object::{delete_selection, duplicate_object};
use mesh_edit::{bevel, decimate, dissolve, extrude, flip_faces, inset, loop_cut, merge, mirror, snap, subdivide, toggle_smooth, triangulate};
use paint::{add_paint_layer, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, fill_bucket, paint_at, paint_fill, paint_sample, paint_stroke};
use selection::{set_active_object, set_active_paint_layer};
use sun::{set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use transform::{rotate_selection, scale_selection, translate_selection};
use utility::set_utility_param;
use uv::{clear_seam, mark_uv_seam, unwrap_active};
//#endregion 🔖️Commands

//#region 🔖️ActionBridge

/// 🎯️ Folds the host's `{action, args}` vocabulary (camelCase keys, JSON floats, merged control `value`s)
/// into the snake_case `FromValue` payloads of `🎮️commands/*`. Without it every shell-dispatched verb is
/// refused as "not a framework-reserved action".
#[path = "🌉️args-bridge/🦀️.rs"]
mod args_bridge;

//#endregion 🔖️ActionBridge

//#region 🧵️RetainedCommands

#[path = "🧵️retained-commands/🦀️.rs"]
mod retained_commands;
use retained_commands::*;

//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation

#[path = "📬️store-preparation/🦀️.rs"]
mod store_preparation;
use store_preparation::*;

//#endregion 📬️StorePreparation

//#region 🔖️LowpolyPlayApp

#[path = "🕹️play-app/🦀️.rs"]
mod play_app;
pub use play_app::*;

//#endregion 🔖️LowpolyPlayApp

//#region 🔖️Manifest

#[path = "📋️manifest/🦀️.rs"]
mod manifest;
pub use manifest::*;

//#endregion 🔖️Manifest

#[cfg(test)]
use semio_framework_plugin::InteractionRef;

//#region 🪢️TaxonomyMounts
#[path = "📚️examples/🎬️demo-session/🦀️.rs"]
pub mod demo_session;
#[cfg(test)]
#[path = "📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts

//#region 🖌️PaintTick
/// 🛠️ The retained route of every paint verb: builds the dispatch's tick on the committed `snapshot`, drives the
/// dispatching window's paint tool through its phase against the gesture the transient holds for that window, and
/// publishes the committed transaction as ONE edit stamped with its ref plus the window's next transient. A stream
/// tick of a window with no open gesture opens one only when `opens` (a press), so a stray canvas move leaves nothing.
#[expect(clippy::too_many_arguments, reason = "One retained paint dispatch: its phase, reason, target and points, whether it may open a gesture, plus the retained inputs it reads.")]
pub fn lowpoly_paint_step(
    phase: Option<&str>,
    reason: Option<&str>,
    object_id: Option<&str>,
    points: &[(f32, f32)],
    opens: bool,
    snapshot: &LowpolySnapshot,
    config: &LowpolyConfig,
    context: &ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>,
    operation: &AppOperationContext,
) -> Result<ArtifactCommandWorkStep<EditorApp<LowpolyPlayApp>>, Fault> {
    let phase = GesturePhase::parse(phase, reason).ok_or_else(|| Fault::from("lowpoly.paint.phase-unknown"))?;
    let window = context.view_state.as_ref().and_then(|view| view.window_id.clone()).unwrap_or_default();
    if phase == GesturePhase::Stream && !opens && context.transient.paint(&window).is_none() {
        return Ok(ArtifactCommandWorkStep::Complete(Emit::default()));
    }
    let gesture_object = context.transient.paint(&window).and_then(|gesture| match &gesture.leaf {
        LowpolyMutation::ApplyPaintStroke(stroke) => Some(stroke.object_id.clone()),
        LowpolyMutation::EditPaintLayer(fill) => Some(fill.object_id.clone()),
        _ => None,
    });
    let object_id = object_id.map(str::to_string).or(gesture_object).unwrap_or_else(|| resolve_active_object_id(snapshot, config));
    let tick = match lowpoly_paint_tick(snapshot, config, &object_id, points) {
        LowpolyPaintTick::Sample(mutation) => return Ok(ArtifactCommandWorkStep::Complete(Emit::config(vec![mutation]))),
        LowpolyPaintTick::Leaf(leaf) => Some(leaf),
        LowpolyPaintTick::Nothing => None,
    };
    let base_revision: String = operation.canonical_base_revision.iter().map(|byte| format!("{byte:02x}")).collect();
    let drive = lowpoly_paint_drive(&context.transient, &window, phase, tick, &operation.authoring_seed, &base_revision)?;
    let emit = match drive.committed {
        Some((reference, mutations)) if !operation.authoring_seed.is_empty() => Emit::commit_transaction(reference, mutations),
        Some((_, mutations)) => Emit::mutations(mutations),
        None => Emit::default(),
    };
    Ok(match drive.transient {
        Some(transient) => ArtifactCommandWorkStep::CompleteWithEphemeral { emit, ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![LowpolyTransientMutation::Snapshot { transient }], window_transient: Vec::new() } },
        None => ArtifactCommandWorkStep::Complete(emit),
    })
}
//#endregion 🖌️PaintTick
