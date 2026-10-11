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

use semio_framework_artifact_reference::io::text::artifact_reference::ArtifactReferenceText;
pub use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, HistoryView};

use crate::editor::lowpoly::commands::{add_primitive, camera, chrome, engagement, document, media, mesh_edit, object, paint, patch_object, selection, sun, transform, utility, uv};
use crate::editor::lowpoly::config::{LowpolyConfig, LowpolyConfigMutation, SetActiveObjectEdit, SetPaintUtilityEdit, SetUtilityParamsEdit, SetPaintColorEdit, SetEngagementInputEdit, SetSunEdit};
use crate::editor::lowpoly::modes::{edit, paint as paint_mode};
use crate::editor::lowpoly::panels::{catalogue as catalogue_panel, document as document_panel, inspection as inspection_panel, layers as layers_panel};
use crate::editor::lowpoly::session::{LowpolyScratch, LowpolyTransient, LowpolyTransientMutation};
use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::{resolve_active_object_id, selection_from_interaction, selection_from_state, utility_param_f64, LowpolyView, MESH_GRANULARITY_OBJECT, MESH_INTERACTION_DOMAIN};
use crate::standards::v1::subsets::any::schema::mutations::LowpolyMutation;
use crate::{artifact_kind, LowpolyObject, LowpolySnapshot, LOWPOLY_DOCUMENT_SCHEMA};
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
use store::ArtifactPack;
use semio_framework_2d::compute::EngineHandles;

//#region 🔖️Constants
pub const LOWPOLY_PLAY_APP_ID: &str = "lowpoly-play";
pub(crate) const LOWPOLY_PLAY_CONTROLLER_ID: &str = "lowpoly-play";
pub use crate::editor::lowpoly::modes::edit::windows::model::LOWPOLY_PLAY_BODY_MAIN;
pub use crate::editor::lowpoly::modes::paint::windows::uv::LOWPOLY_PLAY_BODY_UV;
pub use crate::editor::lowpoly::panels::catalogue::LOWPOLY_PLAY_BODY_CATALOGUE;
pub use crate::editor::lowpoly::panels::document::LOWPOLY_PLAY_BODY_ARTIFACT;
pub use crate::editor::lowpoly::panels::inspection::LOWPOLY_PLAY_BODY_INSPECTION;
pub use crate::editor::lowpoly::panels::layers::LOWPOLY_PLAY_BODY_LAYERS;

/// 🎯️ An `ActionDescriptor` addressed at this app — the single factory every taxonomy node's chrome
/// (`🛠️options/*`, `📌️panels/*`, window/engagement builders) builds its `on_change`/item actions with.
pub fn lowpoly_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(LOWPOLY_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🪟️ Bridges window chrome, which still carries the retained WGPU action descriptor.
pub fn lowpoly_window_action(action: &str, args: Option<semio_framework::DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: LOWPOLY_PLAY_CONTROLLER_ID.into(), action: action.into(), args }
}

/// 🏷️ Admits resolved Lowpoly text into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "lowpoly UI label admission failed"))
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

//#region 🔖️SharedMeasures
/// 🎛️ Collects every window-chrome measure from the app-level `🛠️options/*` shared by both windows
/// (Model + UV expose an identical set — see this file's top-level doc comment).
pub fn lowpoly_window_measures(config: &LowpolyConfig, labels: &LowpolyLabels, select: &crate::editor::lowpoly::options::select::SelectState) -> Vec<WindowMeasure> {
    use crate::editor::lowpoly::options;
    vec![
        options::show_edges::measure(config, labels),
        options::sun::measure(config, labels),
        options::snap::measure(config, labels),
        options::gumball::measure(config, labels),
        options::select::measure(config, labels, select),
        options::paint_params_brush::measure(config, labels),
        options::paint_params_eraser::measure(config, labels),
    ]
}

/// 🧮️ Shared leaf builder for one utility-param slider — used by the `🧲️snap` option and by
/// `paint_utility_params_group` below.
#[allow(clippy::too_many_arguments, reason = "one WindowMeasure::Slider literal per call site; a params struct would only move the same 8 fields around for this single builder")]
pub fn utility_param_slider(id: &str, label: LabelText, key: &str, params: &serde_json::Value, default: f64, min: f64, max: f64, step: f64) -> WindowMeasure {
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
        on_change: lowpoly_window_action("setUtilityParam", Some(semio_framework_value::DslValue::object([("key".to_string(), semio_framework_value::DslValue::String(key.to_string()))]))),
        waiting: None,
    }
}

/// 🖌️ Utility Options for a stamping paint utility (`brush`/`eraser`) — the live brush size/opacity/
/// hardness sliders, tagged `active_utility_id: Some(utility)` so `partition_window_measures` surfaces
/// them in the Utility Options rail only while that exact utility is active. Both utilities stamp
/// through the same `stamp_brush` path, so they share an identical param set.
pub fn paint_utility_params_group(utility: &str, params: &serde_json::Value, labels: &LowpolyLabels) -> WindowMeasure {
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
        on_change: lowpoly_window_action("setUtilityParam", Some(semio_framework_value::DslValue::object([("key".to_string(), semio_framework_value::DslValue::String(key.to_string()))]))),
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
    lowpoly_window_engagement_with_selection(view, active_utility, labels, &crate::editor::lowpoly::options::select::SelectState::default(), 0)
}

/// 🎯️ The engagement with the mesh domain's LIVE state: which granularity the next pick addresses, how
/// many components are selected, and which gumball handle groups are on. These switches live in the
/// engagement's quick-action rail (the always-visible strip that yields to chrome panels) rather than
/// only in Window Options, which the framework lets a right-anchored panel cover by design.
pub fn lowpoly_window_engagement_with_selection(view: LowpolyView<'_>, active_utility: &str, labels: &LowpolyLabels, select: &crate::editor::lowpoly::options::select::SelectState, selected_components: usize) -> WindowEngagement {
    let config = view.config;
    let handles = crate::editor::lowpoly::options::gumball::GumballHandles::from_config(config);
    let granularity = |id: &str, icon: &str, label: LabelText, granularity_id: &str| WindowEngagementOption {
        id: format!("lowpoly.opt.select-{id}"),
        label: Some(label.into()),
        icon_id: Some(icon.into()),
        pressed: Some(select.granularity == granularity_id),
        disabled: None,
        action: Some(lowpoly_window_action(
            "setInteractionGranularity",
            Some(semio_framework_value::DslValue::object([("domainId".to_string(), semio_framework_value::DslValue::String(MESH_INTERACTION_DOMAIN.to_string())), ("granularityId".to_string(), semio_framework_value::DslValue::String(granularity_id.to_string()))])),
        )),
    };
    let gumball = |id: &str, icon: &str, label: LabelText, key: &str, pressed: bool| WindowEngagementOption {
        id: format!("lowpoly.opt.gumball-{id}"),
        label: Some(label.into()),
        icon_id: Some(icon.into()),
        pressed: Some(pressed),
        disabled: None,
        // 🎛️ An engagement option carries no toggle value, so the press flips the flag it reads.
        action: Some(lowpoly_window_action("setUtilityParam", Some(semio_framework_value::DslValue::object([("key".to_string(), semio_framework_value::DslValue::String(key.to_string())), ("value".to_string(), semio_framework_value::DslValue::Bool(!pressed))])))),
    };
    let status = if selected_components > 0 { format!("{active_utility} · {selected_components} {} {}", select.granularity, labels.selected.as_str()) } else { active_utility.to_string() };
    WindowEngagement {
        session_active: Some(true),
        // 🧰️ The move/rotate/scale transform switcher lives in the framework utility bar (declared via
        // `.utility` + window-level `utilities`); the rail carries what a mesh edit needs at hand: the
        // pick granularity, the gumball handle groups, and the three chrome switches.
        options: Some(vec![
            granularity("mesh", "box", labels.mesh, MESH_GRANULARITY_OBJECT),
            granularity("vertex", "circle", labels.vertex, "vertex"),
            granularity("edge", "minus", labels.edge, "edge"),
            granularity("face", "square", labels.face, "face"),
            gumball("move", "move", labels.gumball_move, crate::editor::lowpoly::view::GUMBALL_MOVE_PARAM, handles.r#move),
            gumball("rotate", "rotate-cw", labels.gumball_rotate, crate::editor::lowpoly::view::GUMBALL_ROTATE_PARAM, handles.rotate),
            gumball("scale", "scaling", labels.gumball_scale, crate::editor::lowpoly::view::GUMBALL_SCALE_PARAM, handles.scale),
            WindowEngagementOption { id: "lowpoly.opt.snap".into(), label: Some(labels.snap.into()), icon_id: Some("magnet".into()), pressed: None, disabled: None, action: Some(lowpoly_window_action("snap", None)) },
            WindowEngagementOption { id: "lowpoly.opt.smooth".into(), label: Some(labels.smooth.into()), icon_id: Some("sun".into()), pressed: None, disabled: None, action: Some(lowpoly_window_action("toggleSmooth", None)) },
            WindowEngagementOption {
                id: "lowpoly.opt.show-edges".into(),
                label: Some(labels.show_edges.into()),
                icon_id: Some("grid-3x3".into()),
                pressed: Some(config.show_edges),
                disabled: None,
                action: Some(lowpoly_window_action("toggleShowEdges", None)),
            },
        ]),
        input: Some(WindowEngagementInput {
            id: Some("lowpoly-engagement".into()),
            value: Some(config.engagement_input.clone()),
            placeholder: Some("extrude, inset, mirror, decimate".into()),
            disabled: None,
            on_change: Some(lowpoly_window_action("engagementInput", None)),
            on_submit: Some(lowpoly_window_action("engagementSubmit", None)),
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
        status: Some(vec![WindowEngagementStatus { id: "lowpoly-status".into(), text: status }]),
        possible_engagements: Some(vec![
            WindowEngagementPossible { id: "lowpoly.eng.extrude".into(), label: labels.extrude.into(), detail: None, action: Some(lowpoly_window_action("extrude", None)) },
            WindowEngagementPossible { id: "lowpoly.eng.triangulate".into(), label: labels.triangulate.into(), detail: None, action: Some(lowpoly_window_action("triangulate", None)) },
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
mod args_bridge {
    use super::*;
    use semio_framework_value::DslValue;
    use semio_framework_plugin::{FaultCode, FaultOrigin};

    fn snake(key: &str) -> String {
        let mut out = String::with_capacity(key.len() + 4);
        for ch in key.chars() {
            if ch.is_ascii_uppercase() {
                out.push('_');
                out.push(ch.to_ascii_lowercase());
            } else {
                out.push(ch);
            }
        }
        out
    }

    fn put(entries: &mut Vec<(String, DslValue)>, key: &str, value: DslValue) {
        entries.retain(|(existing, _)| existing != key);
        entries.push((key.to_string(), value));
    }

    /// 🔢️ The host's JSON round trip turns every integer into `Number::Float`; integer codecs decode exact
    /// integers only, so whole finite floats get their integer variant back.
    fn integral(value: DslValue) -> DslValue {
        match value {
            semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(float)) if float.is_finite() && float.fract() == 0.0 && float.abs() < 9.007_199_254_740_992e15 => {
                if float >= 0.0 { semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(float as u64)) } else { semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(float as i64)) }
            }
            semio_framework_value::DslValue::Array(items) => semio_framework_value::DslValue::Array(items.into_iter().map(integral).collect()),
            semio_framework_value::DslValue::Object(entries) => semio_framework_value::DslValue::Object(entries.into_iter().map(|(key, value)| (key, integral(value))).collect()),
            other => other,
        }
    }

    /// 🔁️ Snake-cases every key, applies `aliases` (source → destination, never overwriting a present
    /// destination) and seeds `defaults` for keys still absent.
    fn fold(args: Option<&DslValue>, aliases: &[(&str, &str)], defaults: &[(&str, DslValue)]) -> Vec<(String, DslValue)> {
        let mut entries: Vec<(String, DslValue)> = Vec::new();
        if let Some(semio_framework_value::DslValue::Object(object)) = args {
            for (key, value) in object {
                put(&mut entries, &snake(key), integral(value.clone()));
            }
        }
        for (from, into) in aliases {
            if entries.iter().any(|(key, _)| key == into) {
                continue;
            }
            if let Some((_, value)) = entries.iter().find(|(key, _)| key == from).cloned() {
                put(&mut entries, into, value);
            }
        }
        for (key, value) in defaults {
            if !entries.iter().any(|(existing, _)| existing == key) {
                entries.push(((*key).to_string(), value.clone()));
            }
        }
        entries
    }

    /// 📝️ Moves the merged control `value` into the JSON-text `value_json` field the reducers re-parse.
    fn value_json(mut entries: Vec<(String, DslValue)>) -> Vec<(String, DslValue)> {
        if !entries.iter().any(|(key, _)| key == "value_json") {
            // 🎛️ A window toggle measure dispatches its new state as `pressed` (`WindowMeasureToggle`),
            // a slider/number as `value`; both are the param's value.
            if let Some((_, value)) = entries.iter().find(|(key, _)| key == "value" || key == "pressed").cloned() {
                put(&mut entries, "value_json", semio_framework_value::DslValue::String(semio_framework_pack_json::to_json_string(&value)));
            }
        }
        entries.retain(|(key, _)| key != "value" && key != "pressed");
        entries
    }

    /// 📝️ Prints a control value into a `String` field (`"0.5"`, `"true"`, or the text itself).
    fn text_value(mut entries: Vec<(String, DslValue)>) -> Vec<(String, DslValue)> {
        if let Some(slot) = entries.iter_mut().find(|(key, _)| key == "value") {
            slot.1 = match slot.1.clone() {
                semio_framework_value::DslValue::String(text) => semio_framework_value::DslValue::String(text),
                semio_framework_value::DslValue::Null => semio_framework_value::DslValue::String(String::new()),
                other => semio_framework_value::DslValue::String(semio_framework_pack_json::to_json_string(&other)),
            };
        }
        entries
    }

    /// 📷️ Accepts `{position, target, fov}` or the World3d host's `{windowId, camera: {position, target, zoom, up}}`.
    fn camera(mut entries: Vec<(String, DslValue)>) -> Vec<(String, DslValue)> {
        if let Some((_, semio_framework_value::DslValue::Object(pose))) = entries.iter().find(|(key, _)| key == "camera").cloned() {
            for (key, value) in pose {
                if matches!(key.as_str(), "position" | "target" | "fov") && !entries.iter().any(|(existing, _)| existing == &key) {
                    entries.push((key, value));
                }
            }
        }
        entries.retain(|(key, _)| matches!(key.as_str(), "position" | "target" | "fov"));
        let float = |value: &DslValue| match value {
            semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(v)) => semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(*v)),
            semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(v)) => semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(*v as f64)),
            semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(v)) => semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(*v as f64)),
            other => other.clone(),
        };
        for key in ["position", "target"] {
            if let Some(slot) = entries.iter_mut().find(|(existing, _)| existing == key) {
                if let semio_framework_value::DslValue::Array(items) = &slot.1 {
                    slot.1 = semio_framework_value::DslValue::Array(items.iter().map(float).collect());
                }
            }
        }
        if let Some(slot) = entries.iter_mut().find(|(existing, _)| existing == "fov") {
            slot.1 = float(&slot.1);
        } else {
            entries.push(("fov".into(), semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(50.0))));
        }
        entries
    }

    /// 🧲️ The World3d gumball sends `{mode, ids}` naming the host's own selection (string target ids);
    /// the reducers read the live mesh-domain selection instead, so both are dropped.
    fn gumball(mut entries: Vec<(String, DslValue)>) -> Vec<(String, DslValue)> {
        entries.retain(|(key, _)| key != "ids" && key != "mode");
        entries
    }

    fn decode<T: semio_framework_value::FromValue>(action: &str, entries: Vec<(String, DslValue)>) -> Result<T, Fault> {
        T::from_value(semio_framework_value::DslValue::Object(entries)).map_err(|error| Fault::new(FaultOrigin::App, FaultCode::new("app.command.invalid-args"), format!("lowpoly action '{action}' arguments do not decode: {error}")))
    }

    pub fn command_from_action(action: &str, args: Option<&DslValue>) -> Result<LowpolyCommand, Fault> {
        const OBJECT: &[(&str, &str)] = &[("id", "object_id"), ("value", "object_id")];
        let plain = || fold(args, &[], &[]);
        let none = || Vec::new();
        let zero = || semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(0.0));
        Ok(match action {
            "addPrimitive" => LowpolyCommand::AddPrimitive(decode(action, fold(args, &[("value", "kind")], &[]))?),
            "patchObject" => LowpolyCommand::PatchObject(decode(action, value_json(fold(args, &[("id", "object_id")], &[])))?),
            "extrude" => LowpolyCommand::Extrude(decode(action, fold(args, &[("value", "extrude_distance"), ("distance", "extrude_distance")], &[]))?),
            "inset" => LowpolyCommand::Inset(decode(action, fold(args, &[("value", "inset_amount"), ("amount", "inset_amount")], &[]))?),
            "bevel" => LowpolyCommand::Bevel(decode(action, fold(args, &[("value", "bevel_amount"), ("amount", "bevel_amount"), ("segments", "bevel_segments")], &[]))?),
            "loopCut" => LowpolyCommand::LoopCut(decode(action, fold(args, &[("value", "loop_cuts"), ("cuts", "loop_cuts")], &[]))?),
            "subdivide" => LowpolyCommand::Subdivide(decode(action, none())?),
            "triangulate" => LowpolyCommand::Triangulate(decode(action, none())?),
            "mirror" => LowpolyCommand::Mirror(decode(action, fold(args, &[("value", "axis")], &[]))?),
            "decimate" => LowpolyCommand::Decimate(decode(action, fold(args, &[("value", "decimate_ratio"), ("ratio", "decimate_ratio")], &[]))?),
            "flipFaces" => LowpolyCommand::FlipFaces(decode(action, fold(args, &[("ids", "face_ids")], &[("face_ids", semio_framework_value::DslValue::Array(Vec::new()))]))?),
            "merge" => LowpolyCommand::Merge(decode(action, none())?),
            "dissolve" => LowpolyCommand::Dissolve(decode(action, none())?),
            "snap" => LowpolyCommand::Snap(decode(action, none())?),
            "toggleSmooth" => LowpolyCommand::ToggleSmooth(decode(action, none())?),
            "unwrapActive" => LowpolyCommand::UnwrapActive(decode(action, none())?),
            "markUvSeam" => LowpolyCommand::MarkUvSeam(decode(action, fold(args, &[("value", "seam"), ("ids", "edge_ids")], &[]))?),
            "clearSeam" => LowpolyCommand::ClearSeam(decode(action, none())?),
            "translateSelection" => LowpolyCommand::TranslateSelection(decode(action, gumball(fold(args, &[("x", "dx"), ("y", "dy"), ("z", "dz")], &[("dx", zero()), ("dy", zero()), ("dz", zero())])))?),
            "rotateSelection" => LowpolyCommand::RotateSelection(decode(action, gumball(fold(args, &[], &[("ax", zero()), ("ay", semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(1.0))), ("az", zero()), ("angle", zero())])))?),
            "scaleSelection" => LowpolyCommand::ScaleSelection(decode(action, gumball(fold(args, &[("x", "sx"), ("y", "sy"), ("z", "sz")], &[("sx", semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(1.0))), ("sy", semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(1.0))), ("sz", semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(1.0)))])))?),
            "addPaintLayer" => LowpolyCommand::AddPaintLayer(decode(action, plain())?),
            "paintFill" => LowpolyCommand::PaintFill(decode(action, plain())?),
            "fillBucket" => LowpolyCommand::FillBucket(decode(action, plain())?),
            "importSnapshotJson" => LowpolyCommand::ImportSnapshotJson(decode(action, fold(args, &[("value", "json")], &[]))?),
            "replaceSnapshotJson" => LowpolyCommand::ReplaceSnapshotJson(decode(action, fold(args, &[("value", "json")], &[]))?),
            "exportMesh" => LowpolyCommand::ExportMesh(decode(action, fold(args, &[("value", "format")], &[("format", semio_framework_value::DslValue::String("obj".into()))]))?),
            "loadMeshRequest" => LowpolyCommand::LoadMeshRequest(decode(action, none())?),
            "importMeshFile" => LowpolyCommand::ImportMeshFile(decode(action, fold(args, &[("filename", "name"), ("contents", "payload")], &[]))?),
            "deleteSelection" => LowpolyCommand::DeleteSelection(decode(action, none())?),
            "duplicateObject" => LowpolyCommand::DuplicateObject(decode(action, fold(args, OBJECT, &[]))?),
            "engagementSubmit" => LowpolyCommand::EngagementSubmit(decode(action, text_value(fold(args, &[("text", "value"), ("input", "value")], &[])))?),
            "setActiveObject" => LowpolyCommand::SetActiveObject(decode(action, fold(args, OBJECT, &[]))?),
            "setActivePaintLayer" => LowpolyCommand::SetActivePaintLayer(decode(action, fold(args, &[("index", "layer_index"), ("value", "layer_index")], &[]))?),
            "setUtilityParam" => LowpolyCommand::SetUtilityParam(decode(action, value_json(plain()))?),
            "engagementInput" => LowpolyCommand::EngagementInput(decode(action, text_value(fold(args, &[("text", "value"), ("input", "value")], &[("value", semio_framework_value::DslValue::String(String::new()))])))?),
            "toggleShowEdges" => LowpolyCommand::ToggleShowEdges(decode(action, none())?),
            "toggleSun" => LowpolyCommand::ToggleSun(decode(action, none())?),
            "setSunAzimuth" => LowpolyCommand::SetSunAzimuth(decode(action, fold(args, &[], &[("value", zero())]))?),
            "setSunElevation" => LowpolyCommand::SetSunElevation(decode(action, fold(args, &[], &[("value", zero())]))?),
            "setSunIntensity" => LowpolyCommand::SetSunIntensity(decode(action, fold(args, &[], &[("value", zero())]))?),
            "setCamera" => LowpolyCommand::SetCamera(decode(action, camera(plain()))?),
            "paintSample" => LowpolyCommand::PaintSample(decode(action, plain())?),
            "paintStroke" => LowpolyCommand::PaintStroke(decode(action, plain())?),
            "paintAt" => LowpolyCommand::PaintAt(decode(action, plain())?),
            "canvasPointerDown" => LowpolyCommand::CanvasPointerDown(decode(action, plain())?),
            "canvasPointerMove" => LowpolyCommand::CanvasPointerMove(decode(action, plain())?),
            "canvasPointerUp" => LowpolyCommand::CanvasPointerUp(decode(action, plain())?),
            _ => return Err(Fault::new(FaultOrigin::App, FaultCode::new("app.command.unsupported"), format!("the lowpoly editor has no command for action '{action}'"))),
        })
    }
}
//#endregion 🔖️ActionBridge

//#region 🧵️RetainedCommands
const LOWPOLY_RETAINED_PAYLOAD_SCHEMA: &str = "lowpoly.command.v1";
const LOWPOLY_RETAINED_RAW_BYTES: usize = 16_384;
const LOWPOLY_RETAINED_WORK_ITEMS: usize = 258;
const LOWPOLY_RETAINED_PAINT_RUNS: usize = 4_096;
const LOWPOLY_RETAINED_FIELD_BYTES: usize = 4_096;
const LOWPOLY_RETAINED_OBJECTS: usize = 64;
const LOWPOLY_RETAINED_PAINT_LAYERS_PER_OBJECT: usize = 8;
const LOWPOLY_RETAINED_PAINT_LAYER_BYTES: usize = 4 * 1024 * 1024;
const LOWPOLY_MIGRATED_TOOL_IDS: &[&str] = &[
    "patchObject",
    "addPaintLayer",
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
    "replaceSnapshotJson",
    "exportMesh",
    "loadMeshRequest",
    "importMeshFile",
    "deleteSelection",
    "duplicateObject",
    "paintSample",
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
    "engagementSubmit",
    "paintStroke",
    "paintAt",
    "canvasPointerDown",
    "canvasPointerMove",
    "canvasPointerUp",
    "addPrimitive",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum LowpolyCommandDisposition {
    Artifact = 1,
    Config = 2,
    HostOnly = 3,
    ArtifactTransient = 6,
    /// 🌱️ `addPrimitive` only: its handler unconditionally emits both an `Artifact` mutation
    /// (`CreateObject`) and a `Config` mutation (`SetActiveObject`), AND — like every other
    /// `session::build_doc`/`mesh_edit` reacher — reads and writes the session-local `mesh_workspace`
    /// cache, so it needs the same rehydrate-then-republish treatment `ArtifactTransient` documents.
    /// The coordinator's own schema edit (`schema://s.lowpoly/LowpolyInteractiveJobPartition`'s 8th
    /// `oneOf` arm, `["Artifact","Config","Transient"]`/`["Artifact","Config"]`) is what makes this
    /// representable.
    ArtifactConfigTransient = 7,
}

fn lowpoly_command_disposition(tool_id: &str) -> Option<LowpolyCommandDisposition> {
    Some(match tool_id {
        // 🧲️ A fill commits one one-shot transaction; the gumball reads the live mesh cache to resolve its selection but
        // writes only its relative leaves, so its rehydrated cache is never republished.
        "patchObject" | "addPaintLayer" | "paintFill" | "fillBucket" | "translateSelection" | "rotateSelection" | "scaleSelection" => LowpolyCommandDisposition::Artifact,
        // 🕸️ Every one of these reaches `session::build_doc`/`mesh_edit`, which needs the session-local `mesh_workspace`
        // cache seeded from the LIVE persisted `LowpolyTransient` — `LowpolyDocument::reload_meshes` rejects a stale one
        // with `StaleMeshWorkspace` — so `lowpoly_retained_reduce`'s `threaded!` arms rehydrate scratch from
        // `context.transient` and publish the post-handle cache back: `Artifact` (the edit) `+ Transient` (the cache).
        "extrude" | "inset" | "bevel" | "loopCut" | "subdivide" | "triangulate" | "mirror" | "decimate" | "flipFaces" | "merge" | "dissolve" | "snap" | "toggleSmooth" | "unwrapActive" | "markUvSeam" | "clearSeam" | "engagementSubmit" | "deleteSelection" => LowpolyCommandDisposition::ArtifactTransient,
        "importSnapshotJson" | "replaceSnapshotJson" | "exportMesh" | "loadMeshRequest" | "importMeshFile" => LowpolyCommandDisposition::HostOnly,
        // 🖌️ Every paint verb drives the window's paint gesture in the transient, commits its transaction on release (or as a
        // one-shot) and, under the eyedropper, samples into config — the tick cannot tell statically which.
        "paintStroke" | "paintAt" | "canvasPointerDown" | "canvasPointerMove" | "canvasPointerUp" => LowpolyCommandDisposition::ArtifactConfigTransient,
        // 🌱️ `addPrimitive`'s handler unconditionally emits both a `CreateObject` Artifact mutation and
        // a `SetActiveObject` Config mutation, and it reaches `session::build_doc` — same as every
        // `ArtifactTransient` command above — so it needs the identical scratch rehydration/republication.
        "addPrimitive" | "duplicateObject" => LowpolyCommandDisposition::ArtifactConfigTransient,
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
            LowpolyCommand::ReplaceSnapshotJson(payload) => payload.json.len() <= LOWPOLY_RETAINED_RAW_BYTES,
            LowpolyCommand::ExportMesh(payload) => field(&payload.format),
            LowpolyCommand::LoadMeshRequest(_) => true,
            LowpolyCommand::ImportMeshFile(payload) => field(&payload.name) && payload.payload.len() <= media::LOWPOLY_MESH_FILE_BYTES,
            LowpolyCommand::DeleteSelection(_) => true,
            LowpolyCommand::DuplicateObject(payload) => payload.object_id.as_deref().is_none_or(field),
            LowpolyCommand::PaintSample(payload) => payload.object_id.as_deref().is_none_or(field),
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
            | LowpolyCommand::TranslateSelection(_)
            | LowpolyCommand::RotateSelection(_)
            | LowpolyCommand::ScaleSelection(_)
            | LowpolyCommand::CanvasPointerUp(_) => true,
            LowpolyCommand::Mirror(payload) => payload.axis.as_deref().is_none_or(field),
            LowpolyCommand::FlipFaces(payload) => payload.face_ids.len() <= LOWPOLY_RETAINED_WORK_ITEMS,
            LowpolyCommand::MarkUvSeam(payload) => payload.edge_ids.as_ref().is_none_or(|ids| ids.len() <= LOWPOLY_RETAINED_WORK_ITEMS),
            LowpolyCommand::EngagementSubmit(payload) => payload.value.as_deref().is_none_or(field),
            LowpolyCommand::PaintFill(payload) => payload.object_id.as_deref().is_none_or(field),
            LowpolyCommand::FillBucket(payload) => payload.object_id.as_deref().is_none_or(field),
            LowpolyCommand::PaintStroke(payload) => payload.object_id.as_deref().is_none_or(field) && payload.phase.as_deref().is_none_or(field) && payload.reason.as_deref().is_none_or(field),
            LowpolyCommand::PaintAt(payload) => payload.object_id.as_deref().is_none_or(field) && payload.phase.as_deref().is_none_or(field) && payload.reason.as_deref().is_none_or(field),
            LowpolyCommand::CanvasPointerDown(payload) => payload.object_id.as_deref().is_none_or(field),
            LowpolyCommand::CanvasPointerMove(payload) => payload.object_id.as_deref().is_none_or(field) && payload.samples.as_ref().is_none_or(|samples| samples.len() <= LOWPOLY_RETAINED_WORK_ITEMS),
            LowpolyCommand::AddPrimitive(payload) => payload.kind.as_deref().is_none_or(field),
        }
}

fn lowpoly_sample_pixel(snapshot: &LowpolySnapshot, config: &LowpolyConfig, payload: &paint_sample::PaintSample) -> Emit<LowpolyMutation, LowpolyConfigMutation> {
    let Some((u, v)) = crate::editor::lowpoly::session::paint_uv_from_command(payload.u, payload.v, payload.x, payload.y) else { return Emit::default() };
    let object_id = payload.object_id.clone().unwrap_or_else(|| resolve_active_object_id(snapshot, config));
    let Some(object) = snapshot.objects.iter().find(|object| object.id == object_id) else { return Emit::default() };
    let size = crate::LOWPOLY_PAINT_TEXTURE_SIZE;
    let x = ((u.clamp(0.0, 1.0) * (size as f32 - 1.0)).round() as usize).min(size - 1);
    let y = (((1.0 - v.clamp(0.0, 1.0)) * (size as f32 - 1.0)).round() as usize).min(size - 1);
    let offset = (y * size + x) * 4;
    let mut color = [0_u8; 4];
    for layer in object.paint_layers.iter().filter(|layer| layer.visible) {
        // 🎨️ A sparse (never painted) layer IS opaque white everywhere; only a buffer too short for the
        // sample point is skipped.
        let source = if layer.pixels.is_empty() {
            [255_u8; 4]
        } else if offset.saturating_add(4) > layer.pixels.len() {
            continue;
        } else {
            [layer.pixels[offset], layer.pixels[offset + 1], layer.pixels[offset + 2], layer.pixels[offset + 3]]
        };
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
    Emit::config(vec![LowpolyConfigMutation::SetPaintColor(SetPaintColorEdit { r: color[0], g: color[1], b: color[2], a: color[3] })])
}

fn lowpoly_retained_reduce(
    command: &LowpolyCommand,
    snapshot: &LowpolySnapshot,
    config: &LowpolyConfig,
    history: &HistoryView,
    interaction: &protocol::InteractionState,
    context: &ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>,
    operation: &AppOperationContext,
) -> Result<ArtifactCommandWorkStep<EditorApp<LowpolyPlayApp>>, Fault> {
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window: None };
    // 🕹️ The mesh domain's live selection for THIS dispatch, read straight from the retained job's own
    // scheduler-owned `InteractionState` — `selection_from_state`'s own doc comment names this exact
    // seam ("typed retained reducers read the same immutable domain selection directly from their
    // scheduler-owned request context, without manufacturing a host-only InteractionView").
    let empty_domain_selection = protocol::DomainSelection::default();
    let domain_selection = interaction.selection.get(MESH_INTERACTION_DOMAIN).unwrap_or(&empty_domain_selection);
    let active_object_id = crate::editor::lowpoly::view::active_object_for_selection(snapshot, config, domain_selection);
    let selection = selection_from_state(&active_object_id, domain_selection);
    let selection_object_id = crate::editor::lowpoly::view::selection_object_id(snapshot, domain_selection);
    // 🗿️ The whole objects the selection names (object-granularity rows), for delete/duplicate.
    let selected_object_ids: Vec<String> = domain_selection.ids.iter().filter_map(|raw| crate::editor::lowpoly::view::parse_mesh_target_id(raw)).filter(|(_, component)| component.is_none()).map(|(object_id, _)| object_id).collect();
    // 🕸️ Commands whose handler reaches `session::build_doc`/`mesh_edit`, the mid-drag paint stroke
    // scratch, or the mid-drag gumball transform scratch cannot use a blank `LowpolyScratch::default()`
    // — `LowpolyDocument::reload_meshes` (`⚙️engine/🦀️.rs`) rejects every mesh edit past the
    // very first with `StaleMeshWorkspace` unless scratch is rehydrated from the LIVE persisted
    // `LowpolyTransient`. This macro does that rehydration, runs the handler, then publishes the
    // post-handle scratch back as the new transient root (`ArtifactToolPublicationLane::Transient`) —
    // see `LowpolyCommandDisposition::ArtifactTransient`/`ConfigTransient`'s doc comments.
    macro_rules! threaded {
        ($handle:expr) => {{
            let mut threaded = LowpolyScratch::from_transient(&context.transient, selection.clone());
            threaded.set_selection_object_id(selection_object_id.clone());
            threaded.set_selected_object_ids(selected_object_ids.clone());
            let step_emit = ($handle)(&doc, &cfg, &mut threaded)?;
            let transient = threaded.transient_snapshot();
            return Ok(ArtifactCommandWorkStep::CompleteWithEphemeral { emit: step_emit, ephemeral: EphemeralEmit { presence: Vec::new(), transient: vec![LowpolyTransientMutation::Snapshot { transient }], window_transient: Vec::new() } });
        }};
    }
    // 🧲️ The gumball reads the live mesh cache to resolve its selection and publishes nothing but its transaction.
    macro_rules! read_threaded {
        ($handle:expr) => {{
            let mut threaded = LowpolyScratch::from_transient(&context.transient, selection.clone());
            threaded.set_selection_object_id(selection_object_id.clone());
            threaded.set_selected_object_ids(selected_object_ids.clone());
            return Ok(ArtifactCommandWorkStep::Complete(($handle)(&doc, &cfg, &mut threaded)?));
        }};
    }
    let mut bounded = LowpolyScratch::default();
    bounded.set_selection_object_id(selection_object_id.clone());
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
        LowpolyCommand::ReplaceSnapshotJson(payload) => replace_snapshot_json::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::ExportMesh(payload) => export_mesh::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::LoadMeshRequest(payload) => load_mesh_request::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::ImportMeshFile(payload) => import_mesh_file::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::PaintSample(payload) => return Ok(ArtifactCommandWorkStep::Complete(lowpoly_sample_pixel(snapshot, config, payload))),
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
        LowpolyCommand::TranslateSelection(payload) => read_threaded!(|doc, cfg, ctx| translate_selection::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::RotateSelection(payload) => read_threaded!(|doc, cfg, ctx| rotate_selection::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::ScaleSelection(payload) => read_threaded!(|doc, cfg, ctx| scale_selection::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::PaintStroke(payload) => return paint::lowpoly_paint_step(payload.phase.as_deref(), payload.reason.as_deref(), payload.object_id.as_deref(), &payload.points(), true, snapshot, config, context, operation),
        LowpolyCommand::PaintAt(payload) => return paint::lowpoly_paint_step(payload.phase.as_deref(), payload.reason.as_deref(), payload.object_id.as_deref(), &payload.points(), true, snapshot, config, context, operation),
        LowpolyCommand::CanvasPointerDown(payload) => return paint::lowpoly_paint_step(Some("stream"), None, payload.object_id.as_deref(), &payload.points(), true, snapshot, config, context, operation),
        LowpolyCommand::CanvasPointerMove(payload) => return paint::lowpoly_paint_step(Some("stream"), None, payload.object_id.as_deref(), &payload.points(), false, snapshot, config, context, operation),
        LowpolyCommand::CanvasPointerUp(payload) => {
            let (phase, reason) = payload.phase();
            return paint::lowpoly_paint_step(Some(phase), reason, None, &[], false, snapshot, config, context, operation);
        }
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
        LowpolyCommand::DeleteSelection(payload) => threaded!(|doc, cfg, ctx| delete_selection::handle(payload, doc, cfg, ctx)),
        LowpolyCommand::DuplicateObject(payload) => threaded!(|doc, cfg, ctx| duplicate_object::handle(payload, doc, cfg, ctx)),
        // 🪣️ A one-shot fill reads only the committed layer, so it needs no transient rehydration.
        LowpolyCommand::PaintFill(payload) => paint_fill::handle(payload, &doc, &cfg, &mut bounded),
        LowpolyCommand::FillBucket(payload) => fill_bucket::handle(payload, &doc, &cfg, &mut bounded),
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
    complete: bool,
    closing: bool,
}

impl LowpolyRetainedCommandWork {
    fn new(tool_id: &'static str, disposition: LowpolyCommandDisposition, operation_id: u64, generation: u64, base_revision: [u8; 32], context_identity: u64) -> Self {
        Self { tool_id, disposition, operation_id, generation, base_revision, context_identity, stage: 0, replay_target: None, complete: false, closing: false }
    }
}

impl ArtifactCommandWork<EditorApp<LowpolyPlayApp>> for LowpolyRetainedCommandWork {
    fn work_demands(&self, _input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<LowpolyPlayApp>>, _maximum_copy_bytes: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::RetirementDemand { copy_bytes: std::mem::size_of::<Self>(), depth: 1, ..Default::default() })
    }
    fn tool_id(&self) -> &'static str {
        self.tool_id
    }

    fn workspace_identity(&self) -> u64 {
        lowpoly_tool_identity(self.tool_id) ^ self.operation_id.rotate_left(17) ^ self.generation.rotate_left(31) ^ self.context_identity.rotate_left(43) ^ (u64::from(self.disposition as u8) << 56)
    }

    fn extent(&self, command: &LowpolyCommand, _snapshot: &LowpolySnapshot, _interaction: &protocol::InteractionState, _context: Option<&ArtifactOwnedToolJobContext<EditorApp<LowpolyPlayApp>>>) -> Option<usize> {
        (lowpoly_command_disposition(command.command_id()).is_some()).then_some(2)
    }

    fn step(&mut self, input: &semio_framework_plugin::retained_command::ArtifactCommandInputs<'_, EditorApp<LowpolyPlayApp>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<LowpolyPlayApp>>, Fault> {
        let semio_framework_plugin::retained_command::ArtifactCommandInputs { snapshot_owner: _, command, snapshot, config, history, interaction, hover: _hover, context, operation } = *input;
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
        let step = lowpoly_retained_reduce(command, snapshot, config, history, interaction, context, operation)?;
        self.complete = true;
        Ok(step)
    }

    fn checkpoint_byte(&self, index: usize) -> Option<u8> {
        match index {
            0..=3 => Some(b"LPC3"[index]),
            4 => Some(self.disposition as u8),
            5 => Some(u8::from(self.complete)),
            6 => Some(self.stage),
            7 => Some(0),
            8..=15 => Some(lowpoly_tool_identity(self.tool_id).to_le_bytes()[index - 8]),
            16..=23 => Some(self.operation_id.to_le_bytes()[index - 16]),
            24..=31 => Some(self.generation.to_le_bytes()[index - 24]),
            32..=63 => Some(self.base_revision[index - 32]),
            64..=71 => Some(self.context_identity.to_le_bytes()[index - 64]),
            _ => None,
        }
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        if checkpoint.len() != 72 || &checkpoint[..4] != b"LPC3" || checkpoint[4] != self.disposition as u8 || checkpoint[5] > 1 || checkpoint[6] > 1 || checkpoint[7] != 0 {
            return Err(Fault::from("lowpoly-retained-checkpoint-invalid"));
        }
        let tool = u64::from_le_bytes(checkpoint[8..16].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-tool"))?);
        let operation_id = u64::from_le_bytes(checkpoint[16..24].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-operation"))?);
        let generation = u64::from_le_bytes(checkpoint[24..32].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-generation"))?);
        let context_identity = u64::from_le_bytes(checkpoint[64..72].try_into().map_err(|_| Fault::from("lowpoly-retained-checkpoint-context"))?);
        if tool != lowpoly_tool_identity(self.tool_id) || operation_id != self.operation_id || generation != self.generation || checkpoint[32..64] != self.base_revision || context_identity != self.context_identity {
            return Err(Fault::from("lowpoly-retained-checkpoint-identity-mismatch"));
        }
        self.stage = 0;
        self.replay_target = (checkpoint[6] != 0).then_some(checkpoint[6]);
        self.complete = checkpoint[5] == 1;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, _grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> InteractiveJobCloseStep {
        if self.closing { InteractiveJobCloseStep::Complete { progress: Default::default() } } else { InteractiveJobCloseStep::Blocked }
    }

    fn terminal_frame_release_bytes(&self) -> Option<usize> {
        self.closing.then_some(size_of::<Self>())
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
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

impl ToolJobFactory for LowpolyCommandJobFactory {
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

impl ArtifactOwnedToolJobFactory for LowpolyCommandJobFactory {
    type Owner = EditorApp<LowpolyPlayApp>;
    const TOOL_IDS: &'static [&'static str] = LOWPOLY_MIGRATED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = LOWPOLY_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [semio_framework_plugin::ArtifactToolPublicationContract] = &[
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "patchObject", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "addPaintLayer", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
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
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "replaceSnapshotJson", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "exportMesh", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "loadMeshRequest", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "importMeshFile", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::HostOnly] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "deleteSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Transient] },
        semio_framework_plugin::ArtifactToolPublicationContract {
            tool_id: "duplicateObject",
            lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient],
        },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "paintSample", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Config] },
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
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "translateSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "rotateSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "scaleSelection", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract {
            tool_id: "paintStroke",
            lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient],
        },
        semio_framework_plugin::ArtifactToolPublicationContract {
            tool_id: "paintAt",
            lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient],
        },
        semio_framework_plugin::ArtifactToolPublicationContract {
            tool_id: "canvasPointerDown",
            lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient],
        },
        semio_framework_plugin::ArtifactToolPublicationContract {
            tool_id: "canvasPointerMove",
            lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient],
        },
        semio_framework_plugin::ArtifactToolPublicationContract {
            tool_id: "canvasPointerUp",
            lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient],
        },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "paintFill", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract { tool_id: "fillBucket", lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact] },
        semio_framework_plugin::ArtifactToolPublicationContract {
            tool_id: "addPrimitive",
            lanes: &[semio_framework_plugin::ArtifactToolPublicationLane::Artifact, semio_framework_plugin::ArtifactToolPublicationLane::Config, semio_framework_plugin::ArtifactToolPublicationLane::Transient],
        },
    ];
}
//#endregion 🧵️RetainedCommands

//#region 📬️StorePreparation
const LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 16 * 1024 * 1024;
const LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES: usize = 16_384;

fn lowpoly_paint_layer_retained_bytes(layer: &crate::LowpolyPaintLayer) -> usize {
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
        .saturating_add(object.mesh_content.len())
        .saturating_add(object.paint_layers.iter().fold(0, |bytes, layer| bytes.saturating_add(lowpoly_paint_layer_retained_bytes(layer))))
}

fn lowpoly_snapshot_retained_bytes(snapshot: &LowpolySnapshot) -> usize {
    snapshot.schema.len().saturating_add(snapshot.objects.iter().fold(0, |bytes, object| bytes.saturating_add(lowpoly_object_retained_bytes(object))))
}

/// 📬️ Exact per-variant byte accounting for every one of `LowpolyMutation`'s 21 declared variants —
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
        LowpolyMutation::SetVertexPositions(payload) => Ok(payload.object_id.len().saturating_add(payload.positions.len().saturating_mul(size_of::<crate::diff::LowpolyVertexPosition>())).saturating_add(payload.channels.iter().map(|channel| channel.name.len().saturating_add(channel.values.len().saturating_mul(size_of::<semio_framework_value::DslValue>())).saturating_add(channel.indices.as_ref().map_or(0, |indices| indices.len().saturating_mul(4)))).fold(0usize, usize::saturating_add))),
        LowpolyMutation::InsertPaintLayer(payload) => Ok(payload.object_id.len().saturating_add(lowpoly_paint_layer_retained_bytes(&payload.layer))),
        LowpolyMutation::RemovePaintLayer(payload) => Ok(payload.object_id.len()),
        LowpolyMutation::RenamePaintLayer(payload) => Ok(payload.object_id.len().saturating_add(payload.new_name.len())),
        LowpolyMutation::ChangePaintLayerVisible(payload) => Ok(payload.object_id.len()),
        LowpolyMutation::ChangePaintLayerOpacity(payload) => Ok(payload.object_id.len()),
        LowpolyMutation::ChangePaintLayerBlendMode(payload) => Ok(payload.object_id.len().saturating_add(payload.new_blend_mode.len())),
        LowpolyMutation::EditPaintLayer(payload) if payload.runs.len() <= LOWPOLY_RETAINED_PAINT_RUNS => {
            Ok(payload.object_id.len().saturating_add(payload.runs.len().saturating_mul(size_of::<crate::schema::PixelRun>())).saturating_add(payload.runs.iter().fold(0_usize, |bytes, run| bytes.saturating_add(run.bytes.len()))))
        }
        LowpolyMutation::EditPaintLayer(_) => Err("Lowpoly paint edit exceeds its fixed run envelope".into()),
        LowpolyMutation::ApplyPaintStroke(payload) => Ok(payload.object_id.len().saturating_add(payload.points.len().saturating_mul(size_of::<[f32; 2]>())).saturating_add(16)),
        LowpolyMutation::MoveSelection(payload) => Ok(payload.object_id.len().saturating_add(payload.vertex_ids.len().saturating_mul(4)).saturating_add(12)),
        LowpolyMutation::RotateSelection(payload) => Ok(payload.object_id.len().saturating_add(payload.vertex_ids.len().saturating_mul(4)).saturating_add(28)),
        LowpolyMutation::ScaleSelection(payload) => Ok(payload.object_id.len().saturating_add(payload.vertex_ids.len().saturating_mul(4)).saturating_add(24)),
    }
}

/// 🧺️ One forward row plus the leaf's derived inverse rows. A relative leaf's inverse carries what the base held — a
/// stroke's overwritten pixels, a selection motion's prior mesh content — which preflight never sees, so it declares the
/// lane's whole one-item byte envelope; every other inverse is bounded by its forward twin.
fn admit_lowpoly_artifact_mutation(mutation: &LowpolyMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = lowpoly_artifact_mutation_retained_bytes(mutation)?;
    if retained_bytes > LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("Lowpoly Artifact mutation exceeds its fixed retained preparation envelope".into());
    }
    let inverse_bytes = match mutation {
        LowpolyMutation::ApplyPaintStroke(_) | LowpolyMutation::MoveSelection(_) | LowpolyMutation::RotateSelection(_) | LowpolyMutation::ScaleSelection(_) => store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES.saturating_sub(retained_bytes),
        _ => retained_bytes,
    };
    Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes.saturating_add(inverse_bytes)))
}

fn lowpoly_refusal(message: impl Into<String>) -> semio_framework_value::ValueError {
    semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message.into())
}

fn prepare_lowpoly_artifact(base: &LowpolySnapshot, mutation: &LowpolyMutation) -> Result<(LowpolySnapshot, Vec<LowpolyMutation>), semio_framework_value::ValueError> {
    admit_lowpoly_artifact_mutation(mutation).map_err(lowpoly_refusal)?;
    if !lowpoly_snapshot_admitted(base) || lowpoly_snapshot_retained_bytes(base) > LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err(lowpoly_refusal("Lowpoly Artifact base exceeds its fixed retained preparation envelope"));
    }
    let inverse = mutation.inverse(base)?;
    let diff = mutation.diff(base).into_parts().0;
    let post = protocol::apply_diff(&diff, base).map_err(|_| lowpoly_refusal("Lowpoly Artifact preparation could not apply its exact sparse diff"))?;
    if !lowpoly_snapshot_admitted(&post) || lowpoly_snapshot_retained_bytes(&post) > LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err(lowpoly_refusal("Lowpoly Artifact result exceeds its fixed retained preparation envelope"));
    }
    Ok((post, inverse))
}

fn lowpoly_config_retained_bytes(config: &LowpolyConfig) -> usize {
    config.active_object_id.len().saturating_add(config.paint_utility.len()).saturating_add(config.utility_params_json.len()).saturating_add(config.engagement_input.len()).saturating_add(config.sun_color.len())
}

fn lowpoly_config_mutation_retained_bytes(mutation: &LowpolyConfigMutation) -> usize {
    match mutation {
        LowpolyConfigMutation::SetActiveObject(SetActiveObjectEdit { object_id }) => object_id.len(),
        LowpolyConfigMutation::SetPaintUtility(SetPaintUtilityEdit { value }) | LowpolyConfigMutation::SetEngagementInput(SetEngagementInputEdit { value }) => value.len(),
        LowpolyConfigMutation::SetUtilityParams(SetUtilityParamsEdit { json }) => json.len(),
        LowpolyConfigMutation::SetSun(SetSunEdit { color, .. }) => color.len(),
        LowpolyConfigMutation::SetActivePaintLayer(_) | LowpolyConfigMutation::SetPaintColor(_) | LowpolyConfigMutation::SetWorldCamera(_) | LowpolyConfigMutation::SetShowEdges(_) => 0,
    }
}

/// 🧺️ One forward row plus its point inverse, whose bytes (for `create-mesh` the prior content too) the forward twin bounds.
fn admit_lowpoly_config_mutation(mutation: &LowpolyConfigMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = lowpoly_config_mutation_retained_bytes(mutation);
    if retained_bytes > LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("Lowpoly config mutation exceeds its fixed retained preparation envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint::for_leaf(mutation, retained_bytes.saturating_mul(2)))
}

fn prepare_lowpoly_config(base: &LowpolyConfig, mutation: &LowpolyConfigMutation) -> Result<(LowpolyConfig, Vec<LowpolyConfigMutation>), semio_framework_value::ValueError> {
    admit_lowpoly_config_mutation(mutation).map_err(lowpoly_refusal)?;
    if lowpoly_config_retained_bytes(base) > LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES {
        return Err(lowpoly_refusal("Lowpoly config base exceeds its fixed retained preparation envelope"));
    }
    let inverse = mutation.inverse(base)?;
    let post = protocol::apply_diff(&<LowpolyConfigMutation as protocol::Mutation<LowpolyConfig>>::diff(mutation, base).into_parts().0, base).map_err(|error| lowpoly_refusal(error.to_string()))?;
    if lowpoly_config_retained_bytes(&post) > LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES {
        return Err(lowpoly_refusal("Lowpoly config result exceeds its fixed retained preparation envelope"));
    }
    Ok((post, inverse))
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct LowpolyArtifactStorePreparationFactory;

struct LowpolyArtifactStorePreparation {
    owners: store::OneItemOwners<LowpolySnapshot, LowpolyMutation>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<LowpolySnapshot, LowpolyMutation> for LowpolyArtifactStorePreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<LowpolyMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<LowpolyMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &LowpolyMutation, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document {
            return Err("Lowpoly Artifact preparation rejected its lane".into());
        }
        admit_lowpoly_artifact_mutation(mutation)
    }

    fn begin_demand(&self, _mutation: &LowpolyMutation, _lane: store::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: std::mem::size_of::<LowpolyArtifactStorePreparation>(), depth: 1 })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<LowpolySnapshot, LowpolyMutation, LowpolyMutation>, grant: store::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::ArtifactStoreOneItemPreparation<LowpolySnapshot, LowpolyMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::ArtifactStoreOneItemPreparationRequest<LowpolySnapshot, LowpolyMutation, LowpolyMutation>)> {
        if request.lane != store::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES || lowpoly_artifact_mutation_retained_bytes(&request.mutation).map_or(true, |bytes| bytes > LOWPOLY_ARTIFACT_STORE_MAXIMUM_BYTES) || !lowpoly_snapshot_admitted(request.base.get()) {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Lowpoly Artifact preparation rejected original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        Ok((Box::new(LowpolyArtifactStorePreparation {
            owners: store::OneItemOwners::from_request(request),
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
        }), progress))
    }
}

impl store::ArtifactStoreOneItemPreparation<LowpolySnapshot, LowpolyMutation> for LowpolyArtifactStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || self.cancelled { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.owners.refused.is_some() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal")); }
        if self.owners.prepared.is_some() { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default())); }
        let base = self.owners.base.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Lowpoly Artifact preparation lost its exact base root"))?;
        let mutation = self.owners.mutation.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Lowpoly Artifact preparation lost its mutation owner"))?;
        let retained_bytes = lowpoly_artifact_mutation_retained_bytes(mutation).unwrap_or(0);
        let (post, inverse) = prepare_lowpoly_artifact(base.get(), mutation)?;
        let authority = self.owners.authority.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Lowpoly Artifact preparation lost its Store authority"))?;
        let forward = self.owners.mutation.take().expect("observed original mutation owner");
        let edit = authority.next_edit(forward, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => { *self.owners.refused = Some((edit, post)); return Err(error); }
        };
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: retained_bytes as u64, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<LowpolySnapshot, LowpolyMutation>> { self.owners.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<LowpolySnapshot, LowpolyMutation>> { self.owners.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.owners.begin_close(); }
    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> { self.owners.close_step(grant.retained_grant()) }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.owners.terminal_is_empty() }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct LowpolyConfigStorePreparationFactory;

struct LowpolyConfigStorePreparation {
    owners: store::OneItemOwners<LowpolyConfig, LowpolyConfigMutation>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
}

impl store::ArtifactStoreOneItemPreparationFactory<LowpolyConfig, LowpolyConfigMutation> for LowpolyConfigStorePreparationFactory {
    fn begin_batch_digest(&self, edit: &mut Option<Box<protocol::Edit<LowpolyConfigMutation>>>, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<Option<(Box<dyn store::ArtifactStoreBatchDigest<LowpolyConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress)>, semio_framework_value::ValueError> {
        store::admit_artifact_batch_digest(edit, grant)
    }

    fn preflight(&self, mutation: &LowpolyConfigMutation, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document {
            return Err("Lowpoly config preparation rejected its lane".into());
        }
        admit_lowpoly_config_mutation(mutation)
    }

    fn begin_demand(&self, _mutation: &LowpolyConfigMutation, _lane: store::HistoryLane) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
        Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: std::mem::size_of::<LowpolyConfigStorePreparation>(), depth: 1 })
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<LowpolyConfig, LowpolyConfigMutation, LowpolyConfigMutation>, grant: store::ArtifactStoreOneItemGrant) -> Result<(Box<dyn store::ArtifactStoreOneItemPreparation<LowpolyConfig, LowpolyConfigMutation>>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, store::ArtifactStoreOneItemPreparationRequest<LowpolyConfig, LowpolyConfigMutation, LowpolyConfigMutation>)> {
        if request.lane != store::HistoryLane::Document || request.operation != request.authority.operation() || request.generation != request.authority.generation() || request.base_revision != request.authority.base_revision() || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES || lowpoly_config_mutation_retained_bytes(&request.mutation) > LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES || lowpoly_config_retained_bytes(request.base.get()) > LOWPOLY_CONFIG_STORE_MAXIMUM_BYTES {
            return Err((semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Lowpoly config preparation rejected original publication authority"), request));
        }
        let demand = match self.begin_demand(&request.mutation, request.lane) { Ok(demand) => demand, Err(error) => return Err((error, request)) };
        let progress = match demand.admit(grant.retained_grant()) { Ok(progress) => progress, Err(error) => return Err((error, request)) };
        Ok((Box::new(LowpolyConfigStorePreparation {
            owners: store::OneItemOwners::from_request(request),
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
        }), progress))
    }
}

impl store::ArtifactStoreOneItemPreparation<LowpolyConfig, LowpolyConfigMutation> for LowpolyConfigStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError> {
        if !grant.permits_one() || self.cancelled { return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked); }
        if self.owners.refused.is_some() { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "preparation retains its original semantic refusal")); }
        if self.owners.prepared.is_some() { return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default())); }
        let base = self.owners.base.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Lowpoly config preparation lost its exact base root"))?;
        let mutation = self.owners.mutation.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Lowpoly config preparation lost its mutation owner"))?;
        let retained_bytes = lowpoly_config_mutation_retained_bytes(mutation);
        let (post, inverse) = prepare_lowpoly_config(base.get(), mutation)?;
        let authority = self.owners.authority.as_ref().ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "Lowpoly config preparation lost its Store authority"))?;
        let forward = self.owners.mutation.take().expect("observed original mutation owner");
        let edit = authority.next_edit(forward, inverse);
        let prepared = match authority.prepare_one_item(edit, std::sync::Arc::new(post)) {
            Ok(prepared) => prepared,
            Err((error, edit, post)) => { *self.owners.refused = Some((edit, post)); return Err(error); }
        };
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: retained_bytes as u64, digest: prepared.edit_digest() };
        *self.owners.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint, Default::default()))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }
    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<LowpolyConfig, LowpolyConfigMutation>> { self.owners.prepared.as_ref() }
    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<LowpolyConfig, LowpolyConfigMutation>> { self.owners.prepared.take() }
    fn cancel(&mut self) { self.cancelled = true; }
    fn begin_close(&mut self) { self.owners.begin_close(); }
    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> { self.owners.close_step(grant.retained_grant()) }
    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.copy_bytes) }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(body)?.capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.owners.close_demands(0)?.depth) }
    fn terminal_is_empty(&self) -> bool { self.owners.terminal_is_empty() }
}

//#endregion 📬️StorePreparation

fn lowpoly_export_media(port: &str, doc: &ArtifactView<'_, LowpolySnapshot>, scratch: &LowpolyScratch) -> Result<Media, MediaError> {
    match port {
        "mesh:out" => {
            let mesh = crate::editor::lowpoly::engine::lowpoly_mesh_from_document(doc.snapshot, &scratch.mesh_workspace_map()).map_err(|error| MediaError::Payload(port.into(), error))?;
            let mesh_document = crate::schema::mesh_document_value(&mesh);
            let json = semio_framework_pack_json::to_json_string(&mesh_document);
            Ok(Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, payload: MediaPayload::Structured { schema: "mesh.document".into(), json } })
        }
        "artifact:out" => {
            let media_type = LowpolyPlayApp::io().map_or(MediaType { class: MediaClass::ThreeD, form: MediaForm::Mesh }, |io| io.artifact_media_type);
            let bytes = doc.snapshot.encode_pack();
            Ok(Media { media_type, payload: MediaPayload::Structured { schema: LOWPOLY_DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } })
        }
        _ => Err(MediaError::NotImplemented),
    }
}

fn lowpoly_render(
    body_key: &str,
    doc: &ArtifactView<'_, LowpolySnapshot>,
    cfg: &ConfigView<'_, LowpolyConfig>,
    view_state: &semio_framework_plugin::ViewModel,
    scratch: &mut LowpolyScratch,
    interaction: Option<&InteractionView<'_>>,
    preview: Option<&LowpolySnapshot>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
    let projection = preview.unwrap_or(doc.snapshot);
    let config = cfg.snapshot;
    let empty_domain_selection = protocol::DomainSelection::default();
    let domain_selection = interaction.map_or(&empty_domain_selection, |interaction| interaction.selection(MESH_INTERACTION_DOMAIN));
    let world_selection = crate::editor::lowpoly::view::world_selection_with_hover(projection, config, domain_selection, interaction.and_then(|interaction| interaction.active_granularity(MESH_INTERACTION_DOMAIN)), interaction.map(|interaction| interaction.hover(MESH_INTERACTION_DOMAIN, "pointer")));
    scratch.set_selection_object_id(crate::editor::lowpoly::view::selection_object_id(projection, domain_selection));
    let labels = crate::editor::lowpoly::terminology::lowpoly_play_labels(view_state);
    let active_utility = view_state.active_utility_id.as_deref().filter(|utility| !utility.is_empty()).unwrap_or("move");
    if matches!(body_key, LOWPOLY_PLAY_BODY_MAIN | LOWPOLY_PLAY_BODY_UV) {
        scratch.refresh_texture_cache(projection);
    }
    let texture_cache = scratch.textures().clone();
    let view = LowpolyView { snapshot: projection, config };
    let loaded = matches!(body_key, LOWPOLY_PLAY_BODY_MAIN | LOWPOLY_PLAY_BODY_UV | LOWPOLY_PLAY_BODY_ARTIFACT).then(|| crate::editor::lowpoly::view::build_doc(projection, config, scratch)).flatten();
    let node = match body_key {
        LOWPOLY_PLAY_BODY_MAIN => edit::windows::model::render(view, loaded.as_ref(), active_utility, &texture_cache, &world_selection),
        LOWPOLY_PLAY_BODY_UV => paint_mode::windows::uv::render(view, loaded.as_ref(), &texture_cache),
        LOWPOLY_PLAY_BODY_ARTIFACT => match &loaded {
            Some(loaded) => document_panel::render(view, loaded, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, LOWPOLY_PLAY_BODY_ARTIFACT)),
            None => semio_framework_plugin::built_text_node(semio_framework_ui_locale::Label::data("Failed to load lowpoly document"))
                .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "lowpoly document failed-load text admission failed")),
        },
        LOWPOLY_PLAY_BODY_CATALOGUE => catalogue_panel::render(labels, &semio_framework_plugin::TreeWindows::for_body(view_state, LOWPOLY_PLAY_BODY_CATALOGUE)),
        LOWPOLY_PLAY_BODY_INSPECTION => inspection_panel::render(view, active_utility, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, LOWPOLY_PLAY_BODY_INSPECTION)),
        LOWPOLY_PLAY_BODY_LAYERS => layers_panel::render(view, labels, &semio_framework_plugin::TreeWindows::for_body(view_state, LOWPOLY_PLAY_BODY_LAYERS)),
        _ => semio_framework_plugin::built_text_node(semio_framework_ui_locale::Label::data(format!("Unknown body: {body_key}")))
            .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.fixed-capacity", "lowpoly unknown-body text admission failed")),
    }?;
    Ok(semio_framework_plugin::built_to_component_tree(node))
}

//#region 🔖️LowpolyPlayApp
/// 🖌️ B1: sheds `RefCell<LowpolyPlayRuntime>` entirely — every former runtime field now lives in
/// `LowpolyConfig`, written through `LowpolyConfigMutation`s emitted from `handle`. The one remaining
/// field is genuine mid-gesture scratch state (`LowpolyScratch`) — the "scratch + commit" pattern the
/// `ArtifactEditor` trait itself sanctions for `&self`-only `handle`/`render`.
#[derive(Default, Clone, Copy)]
pub struct LowpolyPlayApp;

impl ArtifactEditor for LowpolyPlayApp {
    /// 📚️ Artifact catalogue stamped by `PluginBuilder::editor` onto the navbar dropdown.
    fn examples() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![crate::examples::hexagonal_cut_concrete_forest_left::source()]
    }
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

    const DIALECT: semio_framework_plugin::app::Dialect = crate::LOWPOLY_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = LOWPOLY_DOCUMENT_SCHEMA;

    /// 🖱️ "Use selection" over the mesh domain: a row `lowpoly-document.<object>` names that object for an `object`
    /// reference, and a row `lowpoly-document.<object>.<granularity>.<n>` names the component number `n` for a reference of
    /// that granularity (`vertex`, `edge`, `face`); any other row names nothing.
    fn selection_reference_id(kinds: &[String], row: &str) -> Option<String> {
        let (object_id, component) = crate::editor::lowpoly::view::parse_mesh_target_id(row)?;
        match component {
            None => kinds.iter().any(|kind| kind == "object").then_some(object_id),
            Some((granularity, number)) => kinds.iter().any(|kind| *kind == granularity).then(|| number.to_string()),
        }
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(LowpolyArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(LowpolyConfigStorePreparationFactory))
    }

    // 🧹️ Registered-app store ownership: without owners + disposers for every lane the mounted instance
    // faults `interactive-job.close-owned-disposer-missing` and threaded commands cannot republish scratch.
    /// 🎯️ What a mesh-domain interaction verb repaints. Hover: the Model scene only (the guest echoes
    /// `hoveredComponent` there; the rail and panels never show a hover). A moved selection: the scene,
    /// the Artifact tree and Inspection panels, the engagement rail (`N faces selected`) and the
    /// measures. A granularity or mode switch: the scene (the pick targets change), the engagement
    /// rail and the measures — the framework default refreshes measures but never engagements, which
    /// left the rail's pressed granularity stale until the next pick (2026-09-18).
    fn interaction_scope(verb: semio_framework_plugin::InteractionVerb, domains: &[&str]) -> Option<semio_framework::kernel::UiDirtyScope> {
        use semio_framework::kernel::UiDirtyScope;
        use semio_framework_plugin::InteractionVerb;
        if domains.is_empty() || domains.iter().any(|domain| *domain != MESH_INTERACTION_DOMAIN) {
            return None;
        }
        let windows = vec![LOWPOLY_PLAY_BODY_MAIN.to_string(), LOWPOLY_PLAY_BODY_UV.to_string()];
        Some(match verb {
            InteractionVerb::Hover => UiDirtyScope::Partial { window_bodies: vec![LOWPOLY_PLAY_BODY_MAIN.to_string()], panel_bodies: Vec::new(), utilities: false, tools: false, engagements: false, measures: false, labels: false },
            InteractionVerb::Select | InteractionVerb::ClearSelection | InteractionVerb::SelectAll => UiDirtyScope::Partial {
                window_bodies: windows,
                panel_bodies: vec![LOWPOLY_PLAY_BODY_ARTIFACT.to_string(), LOWPOLY_PLAY_BODY_INSPECTION.to_string()],
                utilities: false,
                tools: false,
                engagements: true,
                measures: true,
                labels: false,
            },
            InteractionVerb::SetSelectionMode | InteractionVerb::SetGranularity => UiDirtyScope::Partial { window_bodies: windows, panel_bodies: Vec::new(), utilities: false, tools: false, engagements: true, measures: true, labels: false },
        })
    }

    fn build_transient_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(semio_framework_plugin::bounded_transient_preparation_factory::<Self::Transient, Self::TransientMutation>())
    }

    fn build_transient_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(semio_framework_plugin::bounded_transient_root_retirement_factory::<Self::Transient>())
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<LowpolyPlayApp>,
        owner_file: "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.lowpoly.lowpoly@1/*#editor",
        artifact_schema: "lowpoly.document",
        factory: "LowpolyCommandJobFactory",
        factory_type: LowpolyCommandJobFactory,
        tools: {
            "patchObject" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "addPaintLayer" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setActiveObject" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setActivePaintLayer" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setUtilityParam" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "engagementInput" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "toggleShowEdges" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "toggleSun" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setSunAzimuth" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setSunElevation" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setSunIntensity" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "setCamera" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "importSnapshotJson" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "replaceSnapshotJson" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "exportMesh" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "loadMeshRequest" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            // 📥️ Every row must equal the ONE contract `LowpolyCommandJobFactory` registers
            // (`registration.contract == row.contract`, else the boot-time proof join refuses the tool);
            // the mesh file rides inside that shared wire budget.
            "importMeshFile" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "deleteSelection" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "duplicateObject" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "paintSample" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "extrude" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "inset" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "bevel" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "loopCut" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "subdivide" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "triangulate" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "mirror" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "decimate" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "flipFaces" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "merge" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "dissolve" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "snap" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "toggleSmooth" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "unwrapActive" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "markUvSeam" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "clearSeam" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "engagementSubmit" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "translateSelection" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "rotateSelection" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "scaleSelection" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "paintStroke" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "paintAt" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "canvasPointerDown" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "canvasPointerMove" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "canvasPointerUp" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "paintFill" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "fillBucket" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
            "addPrimitive" => ToolExecutionContract::resumable(16_384, 258, 1, 33_554_432, 7_500, 1, 1),
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
            return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("app.command.tool-mismatch"), "lowpoly retained command does not match its exact registered tool"));
        }
        if !lowpoly_command_admitted(&request.command, &request.snapshot, &request.config) {
            return Err(Fault::from("lowpoly-retained-command-capacity"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> =
            Box::new(LowpolyRetainedCommandWork::new(tool_id, disposition, request.operation.operation.0, request.operation.generation.0, request.canonical_base_revision, request.context.identity_digest()));
        let operation_context = AppOperationContext { retained: request.retained,
            app_instance_id: request.app_instance_id,
            parent_document_id: request.parent_document_id.clone(),
            operation_id: request.operation.operation.0,
            generation: request.operation.generation.0,
            canonical_base_revision: request.canonical_base_revision,
            authoring_seed: request.authoring_seed.clone(),
        };
        let payload = ArtifactRetainedCommandPayload::new(
            semio_framework_plugin::retained_command::ArtifactRetainedCommandInputs {
                command: *request.command,
                snapshot: request.snapshot,
                config: request.config,
                history: request.history,
                interaction_state: request.interaction_state,
                interaction_hover: request.interaction_hover,
                context: Some(request.context),
                operation: operation_context,
                completion: request.completion,
            },
            LowpolyCommand::command_id,
            LOWPOLY_RETAINED_RAW_BYTES,
            LOWPOLY_RETAINED_WORK_ITEMS,
            work,
        );
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::semio_framework_schema_registry::AppSchemaDescriptor> {
        Some(crate::editor::lowpoly::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> LowpolySnapshot {
        crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(lowpoly_io())
    }

    /// 🧬️ `import_media`'s `"mesh:in"`/`"artifact:in"` arms build `reset_document_effect` (a `Effect::LoadDocument`, outside
    /// undo history); whole-document replace has no mutation.
    ///
    /// 🎞️ `mesh:out` plus the inherited `document:out` default (the pack of `doc.snapshot`, replicated
    /// inline — overriding `export_media` shadows the trait's provided body for every port on this app,
    /// not just the new one).
    fn export_media(port: &str, doc: &ArtifactView<'_, LowpolySnapshot>) -> Result<Media, MediaError> {
        lowpoly_export_media(port, doc, &LowpolyScratch::default())
    }

    fn export_media_with_request_context(_owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle, port: &str, doc: &ArtifactView<'_, LowpolySnapshot>, transient: &semio_framework_plugin::TransientView<'_, LowpolyTransient>) -> Result<Media, MediaError> {
        let scratch = LowpolyScratch::from_transient(transient.snapshot, crate::LowpolySelection::default());
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
                let mesh_document = semio_framework_pack_json::from_json_str::<semio_framework_value::DslValue>(json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| MediaError::Payload(port.into(), error.to_string()))?;
                let mesh = crate::schema::mesh_from_document_value(&mesh_document).map_err(|error| MediaError::Payload(port.into(), error))?;
                let snapshot = crate::schema::lowpoly_snapshot_from_mesh(&mesh).map_err(|error| MediaError::Payload(port.into(), error))?;
                Ok(Emit { effects: vec![reset_document_effect(&snapshot)], ..Default::default() })
            }
            "artifact:in" => {
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

    fn command_from_action(action: &str, args: Option<&semio_framework_value::DslValue>) -> Result<LowpolyCommand, Fault> {
        args_bridge::command_from_action(action, args)
    }

    /// 🕹️ `interaction` is the mesh domain's current selection/hover/mode/granularity, resolved once per
    /// dispatch into `LowpolyScratch::current_selection` — the `app_commands!`-generated `dispatch` calls
    /// every leaf `🎮️commands/*::handle(payload, doc, cfg, ctx)` uniformly (no `interaction` parameter
    /// of its own), so this is the one seam by which those handlers (via `view::build_doc`/
    /// `session::mesh_edit`) see the framework-owned selection. See `🧭️view/🦀️.rs`'s
    /// `🔖️MeshDomain` region for the id scheme.
    fn handle(
        command: &LowpolyCommand,
        doc: &ArtifactView<'_, LowpolySnapshot>,
        cfg: &ConfigView<'_, LowpolyConfig>,
        interaction: &InteractionView<'_>,
        _view_state: Option<&semio_framework_plugin::ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<LowpolyMutation, LowpolyConfigMutation, Self::DraftMutation>, Fault> {
        let domain_selection = interaction.selection(MESH_INTERACTION_DOMAIN);
        let active = crate::editor::lowpoly::view::active_object_for_selection(doc.snapshot, cfg.snapshot, domain_selection);
        let selection = selection_from_interaction(&active, interaction);
        let mut scratch = LowpolyScratch::from_transient(&LowpolyTransient::default(), selection);
        scratch.set_selection_object_id(crate::editor::lowpoly::view::selection_object_id(doc.snapshot, domain_selection));
        command.dispatch(doc, cfg, &mut scratch)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        lowpoly_render(body_key, doc, cfg, view_state, &mut LowpolyScratch::default(), None, None)
    }

    fn render_with_request_context(
        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,
        body_key: &str,
        doc: &ArtifactView<'_, LowpolySnapshot>,
        cfg: &ConfigView<'_, LowpolyConfig>,
        view_state: &semio_framework_plugin::ViewModel,
        transient: &semio_framework_plugin::TransientView<'_, LowpolyTransient>,
        interaction: &InteractionView<'_>,
    ) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        let mut scratch = LowpolyScratch::from_transient(transient.snapshot, crate::LowpolySelection::default());
        let preview = transient.snapshot.paint_preview(doc.snapshot);
        lowpoly_render(body_key, doc, cfg, view_state, &mut scratch, Some(interaction), preview.as_ref())
    }

    fn window_engagements(doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, WindowEngagement> {
        let config = cfg.snapshot;
        let active_utility = view_state.active_utility_id.as_deref().filter(|utility| !utility.is_empty()).unwrap_or("move");
        let labels = crate::editor::lowpoly::terminology::lowpoly_play_labels(view_state);
        let engagement = lowpoly_window_engagement(LowpolyView { snapshot: doc.snapshot, config }, active_utility, labels);
        HashMap::from([(edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN.into(), engagement.clone()), (paint_mode::windows::uv::LOWPOLY_PLAY_WINDOW_UV.into(), engagement)])
    }

    /// 🎯️ The live engagement: granularity switches pressed to the mesh domain's state, the selected
    /// component count in the status line.
    fn window_engagements_with_request_context(doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, view_state: &semio_framework_plugin::ViewModel, _transient: &semio_framework_plugin::TransientView<'_, LowpolyTransient>, interaction: &InteractionView<'_>) -> HashMap<String, WindowEngagement> {
        let config = cfg.snapshot;
        let active_utility = view_state.active_utility_id.as_deref().filter(|utility| !utility.is_empty()).unwrap_or("move");
        let labels = crate::editor::lowpoly::terminology::lowpoly_play_labels(view_state);
        let select = crate::editor::lowpoly::options::select::SelectState::from_interaction(interaction);
        let world = crate::editor::lowpoly::view::world_selection_from_state(doc.snapshot, config, interaction.selection(MESH_INTERACTION_DOMAIN), interaction.active_granularity(MESH_INTERACTION_DOMAIN));
        let selected = if world.granularity == MESH_GRANULARITY_OBJECT { world.object_ids.len() } else { world.component_ids.len() };
        let engagement = lowpoly_window_engagement_with_selection(LowpolyView { snapshot: doc.snapshot, config }, active_utility, labels, &select, selected);
        HashMap::from([(edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN.into(), engagement.clone()), (paint_mode::windows::uv::LOWPOLY_PLAY_WINDOW_UV.into(), engagement)])
    }

    fn window_measures(_doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, view_state: &semio_framework_plugin::ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        let labels = crate::editor::lowpoly::terminology::lowpoly_play_labels(view_state);
        let measures = lowpoly_window_measures(config, labels, &crate::editor::lowpoly::options::select::SelectState::default());
        HashMap::from([(edit::windows::model::LOWPOLY_PLAY_WINDOW_MAIN.into(), measures.clone()), (paint_mode::windows::uv::LOWPOLY_PLAY_WINDOW_UV.into(), measures)])
    }

    /// 🎛️ The live measures: the granularity/selection-mode toggles read the mesh domain's current
    /// state, so the pressed toggle IS the one the next pick uses (they used to be always-off).
    fn window_measures_with_request_context(_doc: &ArtifactView<'_, LowpolySnapshot>, cfg: &ConfigView<'_, LowpolyConfig>, view_state: &semio_framework_plugin::ViewModel, interaction: &InteractionView<'_>) -> HashMap<String, Vec<WindowMeasure>> {
        let config = cfg.snapshot;
        let labels = crate::editor::lowpoly::terminology::lowpoly_play_labels(view_state);
        let select = crate::editor::lowpoly::options::select::SelectState::from_interaction(interaction);
        let measures = lowpoly_window_measures(config, labels, &select);
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
/// `"mesh:in"`/`"artifact:in"` above, `commands::document::{set_snapshot_json,replace_snapshot_json}`)
/// builds this effect instead of an `Emit::mutations([...])`. The spr is a fresh, edit-free op-log
/// (`store::empty_document_spr`, the `🏗️fem`/process3d shape) — never a live `ArtifactEnvelope`
/// minted just to print it: such an envelope owns a bounded retirement authority, and dropping it at
/// the end of this function trapped the guest (`artifact envelope terminal shell reached Drop before
/// its app-owned bounded retirement authority detached every nested owner`) on every mesh import and
/// `replaceSnapshotJson` (ticket 26/08/29/LOWPOLY-END-TO-END-COMMANDS-IO-AND-MUTATIONS, 2026-09-17).
pub fn reset_document_effect(scene: &LowpolySnapshot) -> semio_framework_plugin::Effect {
    let pack = <LowpolySnapshot as ArtifactPack>::encode_pack(scene);
    let spr = ::semio_framework_async::poll::resolve_ready(store::empty_document_spr("lowpoly", LOWPOLY_DOCUMENT_SCHEMA));
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
/// calls this app used to make are DROPPED here, not ported — the subset's own `📚️examples/🌲️hexagonal-cut-concrete-forest-left`
/// facet is the intended replacement mechanism per the pilot's report, not confirmed with the
/// coordinator by this packet.
pub fn create_lowpoly_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::LOWPOLY_DIALECT)
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
            .action_destructive("clearSeam")
            .mutation("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"))
            .mutation("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"))
            .mutation("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"))
            .mutation("addPaintLayer", LocalizedLabel::native("Add Paint Layer", "Malebene hinzufügen"))
            .mutation("paintFill", LocalizedLabel::native("Paint Fill", "Füllen malen"))
            .mutation("fillBucket", LocalizedLabel::native("Fill Bucket", "Fülleimer"))
            .mutation("importSnapshotJson", LocalizedLabel::native("Import Snapshot Json", "Snapshot-JSON importieren"))
            .mutation("replaceSnapshotJson", LocalizedLabel::native("Set Fixture Json", "Fixture-JSON festlegen"))
            // 📤️ Shell effects: a mesh download, a file-open request, and the import the shell answers it with.
            .mutation("deleteSelection", LocalizedLabel::native("Delete Selection", "Auswahl löschen"))
            .action_destructive("deleteSelection")
            .mutation("duplicateObject", LocalizedLabel::native("Duplicate Object", "Objekt duplizieren"))
            .shell_action("exportMesh", LocalizedLabel::native("Export Mesh", "Mesh exportieren"))
            .shell_action("loadMeshRequest", LocalizedLabel::native("Load Mesh…", "Mesh laden…"))
            .action_with(semio_framework_plugin::ActionDefinition { in_palette: false, ..semio_framework_plugin::ActionDefinition::bounded_catalog("importMeshFile", LocalizedLabel::native("Import Mesh File", "Mesh-Datei importieren"), semio_framework_plugin::ActionKind::Mutation) })
            .mutation("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"))
            .action_audience("engagementSubmit", semio_framework_plugin::CapabilityAudience::Input)
            // 👁️ Ephemeral view state — selection, camera, hover and the eyedropper sample.
            .view_action("setActiveObject", LocalizedLabel::native("Set Active Object", "Aktives Objekt festlegen"))
            .view_action("setActivePaintLayer", LocalizedLabel::native("Set Active Paint Layer", "Aktive Malebene festlegen"))
            .view_action("setUtilityParam", LocalizedLabel::native("Set Utility Param", "Werkzeugparameter festlegen"))
            .action_with(semio_framework_plugin::ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), semio_framework_plugin::ActionKind::View, "hand"))
            .action_audience("engagementInput", semio_framework_plugin::CapabilityAudience::Input)
            .view_action("toggleShowEdges", LocalizedLabel::native("Toggle Show Edges", "Kantenanzeige umschalten"))
            .action_with(semio_framework_plugin::ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), semio_framework_plugin::ActionKind::View, "sun"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), semio_framework_plugin::ActionKind::View, "sun"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), semio_framework_plugin::ActionKind::View, "sun"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), semio_framework_plugin::ActionKind::View, "sun"))
            .action_with(semio_framework_plugin::ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), semio_framework_plugin::ActionKind::View, "camera"))
            .action_with(semio_framework_plugin::ActionDefinition::new("paintStroke", LocalizedLabel::native("Paint Stroke", "Malstrich"), semio_framework_plugin::ActionKind::Mutation, "paintbrush"))
            .action_with(semio_framework_plugin::ActionDefinition::new("paintAt", LocalizedLabel::native("Paint At", "Malen bei"), semio_framework_plugin::ActionKind::Mutation, "paintbrush"))
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerDown", LocalizedLabel::native("Canvas Pointer Down", "Leinwand-Zeiger gedrückt"), semio_framework_plugin::ActionKind::Mutation, "mouse-pointer"))
            .action_audience("canvasPointerDown", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerMove", LocalizedLabel::native("Canvas Pointer Move", "Leinwand-Zeiger bewegt"), semio_framework_plugin::ActionKind::Mutation, "mouse-pointer"))
            .action_audience("canvasPointerMove", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(semio_framework_plugin::ActionDefinition::new("canvasPointerUp", LocalizedLabel::native("Canvas Pointer Up", "Leinwand-Zeiger losgelassen"), semio_framework_plugin::ActionKind::Mutation, "mouse-pointer"))
            .action_audience("canvasPointerUp", semio_framework_plugin::CapabilityAudience::Input)
            .action_with(semio_framework_plugin::ActionDefinition::new("paintSample", LocalizedLabel::native("Paint Sample", "Farbe aufnehmen"), semio_framework_plugin::ActionKind::View, "paintbrush"))
            // 📝️ Staged argument forms for the P1 actions — the panel form seeds from these defaults and
            // stages typed overrides read out of `args`; `config.utility_params_json` remains the live backing store.
            .action_args("extrude", vec![ActionArgDef::slider("extrudeDistance", LocalizedLabel::native("Extrude Distance", "Extrusionsabstand"), 0.01, 2.0).default_value(&0.25)])
            .action_args("inset", vec![ActionArgDef::number("insetAmount", LocalizedLabel::native("Inset Amount", "Einzugsbetrag")).default_value(&0.1)])
            // 🧲️ Numeric transform entry from the Actions pane — the gumball drag fills the same fields.
            .action_args("translateSelection", vec![
                ActionArgDef::number("dx", LocalizedLabel::native("Move X", "Verschieben X")).default_value(&0.0),
                ActionArgDef::number("dy", LocalizedLabel::native("Move Y", "Verschieben Y")).default_value(&0.0),
                ActionArgDef::number("dz", LocalizedLabel::native("Move Z", "Verschieben Z")).default_value(&0.0),
            ])
            .action_args("rotateSelection", vec![
                ActionArgDef::number("angle", LocalizedLabel::native("Angle", "Winkel")).default_value(&0.0),
                ActionArgDef::number("ax", LocalizedLabel::native("Axis X", "Achse X")).default_value(&0.0),
                ActionArgDef::number("ay", LocalizedLabel::native("Axis Y", "Achse Y")).default_value(&1.0),
                ActionArgDef::number("az", LocalizedLabel::native("Axis Z", "Achse Z")).default_value(&0.0),
            ])
            .action_args("scaleSelection", vec![
                ActionArgDef::number("sx", LocalizedLabel::native("Scale X", "Skalieren X")).default_value(&1.0),
                ActionArgDef::number("sy", LocalizedLabel::native("Scale Y", "Skalieren Y")).default_value(&1.0),
                ActionArgDef::number("sz", LocalizedLabel::native("Scale Z", "Skalieren Z")).default_value(&1.0),
            ])
            .action_args("exportMesh", vec![ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), vec![
                ActionArgOption::new("obj", LocalizedLabel::native("OBJ", "OBJ")),
                ActionArgOption::new("ply", LocalizedLabel::native("PLY", "PLY")),
                ActionArgOption::new("stl", LocalizedLabel::native("STL", "STL")),
            ]).required().default_value(&"obj")])
            .action_args("bevel", vec![
                ActionArgDef::number("bevelAmount", LocalizedLabel::native("Bevel Amount", "Fasenbetrag")).default_value(&0.05),
                ActionArgDef::number("bevelSegments", LocalizedLabel::native("Bevel Segments", "Fasensegmente")).default_value(&1),
            ])
            .action_args("loopCut", vec![ActionArgDef::number("loopCuts", LocalizedLabel::native("Loop Cuts", "Schleifenschnitte")).default_value(&1)])
            .action_args("decimate", vec![ActionArgDef::slider("decimateRatio", LocalizedLabel::native("Decimate Ratio", "Dezimierungsverhältnis"), 0.05, 1.0).default_value(&0.5)])
            .action_args("mirror", vec![ActionArgDef::select("axis", LocalizedLabel::native("Axis", "Achse"), vec![
                ActionArgOption::new("x", LocalizedLabel::native("X", "X")),
                ActionArgOption::new("y", LocalizedLabel::native("Y", "Y")),
                ActionArgOption::new("z", LocalizedLabel::native("Z", "Z")),
            ]).default_value(&"x")])
            .action_args("addPrimitive", vec![ActionArgDef::select("kind", LocalizedLabel::native("Kind", "Art"), vec![
                ActionArgOption::new("box", LocalizedLabel::native("Cube", "Würfel")),
                ActionArgOption::new("plane", LocalizedLabel::native("Plane", "Ebene")),
                ActionArgOption::new("cylinder", LocalizedLabel::native("Cylinder", "Zylinder")),
                ActionArgOption::new("cone", LocalizedLabel::native("Cone", "Kegel")),
                ActionArgOption::new("ico_sphere", LocalizedLabel::native("Ico Sphere", "Ikokugel")),
            ]).default_value(&"box")])
            .action_args("markUvSeam", vec![ActionArgDef::toggle("seam", LocalizedLabel::native("Seam", "Naht")).default_value(&true)])
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
            // boundary, see `🧭️view/🦀️.rs`'s `🔖️MeshDomain` region), Flat hierarchy (a mesh has no
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
            // ⌨️ The modeller's hands: delete what is selected, duplicate the active object, extrude/inset it.
            .keybinding("delete", "deleteSelection")
            .keybinding("backspace", "deleteSelection")
            .keybinding("mod+d", "duplicateObject")
            .keybinding("e", "extrude")
            .keybinding("i", "inset")
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
            .action_interactive_job("paintFill", InteractiveJobClassification::Migrated)
            .action_interactive_job("fillBucket", InteractiveJobClassification::Migrated)
            .action_interactive_job("importSnapshotJson", InteractiveJobClassification::Migrated)
            .action_interactive_job("replaceSnapshotJson", InteractiveJobClassification::Migrated)
            .action_destructive("replaceSnapshotJson")
            .action_interactive_job("exportMesh", InteractiveJobClassification::Migrated)
            .action_destructive("exportMesh")
            .action_interactive_job("loadMeshRequest", InteractiveJobClassification::Migrated)
            .action_interactive_job("importMeshFile", InteractiveJobClassification::Migrated)
            .action_interactive_job("deleteSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("duplicateObject", InteractiveJobClassification::Migrated)
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
            .action_interactive_job("paintSample", InteractiveJobClassification::Migrated)
            .action_interactive_job("paintStroke", InteractiveJobClassification::Migrated)
            .action_interactive_job("paintAt", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerDown", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerMove", InteractiveJobClassification::Migrated)
            .action_interactive_job("canvasPointerUp", InteractiveJobClassification::Migrated)
            .action_describe("addPrimitive", LocalizedLabel::native("Adds a new primitive mesh object of the given kind (a box, sphere, cylinder, cone or plane) to the scene and makes it the active object.", "Fügt der Szene ein neues Grundkörper-Netzobjekt der angegebenen Art (Quader, Kugel, Zylinder, Kegel oder Ebene) hinzu und macht es zum aktiven Objekt."))
            .action_describe("patchObject", LocalizedLabel::native("Sets one scalar field of one object by id, its name or whether it is shaded smooth, from a JSON value.", "Setzt ein skalares Feld eines Objekts anhand seiner Id, Name oder ob es glatt schattiert wird, aus einem JSON-Wert."))
            .action_describe("extrude", LocalizedLabel::native("Extrudes the selected faces of the active object outwards by the given distance, creating new side faces.", "Extrudiert die ausgewählten Flächen des aktiven Objekts um den angegebenen Abstand nach außen und erzeugt neue Seitenflächen."))
            .action_describe("inset", LocalizedLabel::native("Insets the selected faces of the active object by the given amount, creating a smaller face inside each with a ring of faces around it.", "Zieht die ausgewählten Flächen des aktiven Objekts um den angegebenen Betrag ein und erzeugt in jeder eine kleinere Fläche mit einem Flächenring darum."))
            .action_describe("bevel", LocalizedLabel::native("Bevels the selected edges of the active object by the given amount with the given number of segments.", "Fast die ausgewählten Kanten des aktiven Objekts um den angegebenen Betrag mit der angegebenen Segmentanzahl."))
            .action_describe("loopCut", LocalizedLabel::native("Cuts the given number of edge loops across the selected faces of the active object.", "Schneidet die angegebene Anzahl von Kantenschleifen quer durch die ausgewählten Flächen des aktiven Objekts."))
            .action_describe("subdivide", LocalizedLabel::native("Subdivides the selected faces of the active object into smaller faces.", "Unterteilt die ausgewählten Flächen des aktiven Objekts in kleinere Flächen."))
            .action_describe("triangulate", LocalizedLabel::native("Splits the selected faces of the active object into triangles.", "Zerlegt die ausgewählten Flächen des aktiven Objekts in Dreiecke."))
            .action_describe("mirror", LocalizedLabel::native("Mirrors the active object's selected geometry across the given axis (x, y or z).", "Spiegelt die ausgewählte Geometrie des aktiven Objekts an der angegebenen Achse (x, y oder z)."))
            .action_describe("decimate", LocalizedLabel::native("Reduces the face count of the active object's selection to the given ratio, simplifying its shape.", "Reduziert die Flächenzahl der Auswahl des aktiven Objekts auf das angegebene Verhältnis und vereinfacht damit ihre Form."))
            .action_describe("flipFaces", LocalizedLabel::native("Flips the normals of the selected faces of the active object so they face the other way.", "Kehrt die Normalen der ausgewählten Flächen des aktiven Objekts um, sodass sie in die andere Richtung zeigen."))
            .action_describe("merge", LocalizedLabel::native("Merges the selected vertices of the active object into one.", "Verschmilzt die ausgewählten Punkte des aktiven Objekts zu einem."))
            .action_describe("dissolve", LocalizedLabel::native("Dissolves the selected elements of the active object, removing them while keeping the surrounding surface closed.", "Löst die ausgewählten Elemente des aktiven Objekts auf, entfernt sie also und hält die umgebende Fläche geschlossen."))
            .action_describe("snap", LocalizedLabel::native("Snaps the selected vertices of the active object to a grid with the snap tool's spacing.", "Rastet die ausgewählten Punkte des aktiven Objekts an einem Raster mit dem Abstand des Einrast-Werkzeugs ein."))
            .action_describe("toggleSmooth", LocalizedLabel::native("Switches the active object between smooth and flat shading.", "Schaltet das aktive Objekt zwischen glatter und flacher Schattierung um."))
            .action_describe("unwrapActive", LocalizedLabel::native("Computes a new UV unwrap of the active object along its marked seams, replacing its previous texture coordinates.", "Berechnet eine neue UV-Abwicklung des aktiven Objekts entlang seiner markierten Nähte und ersetzt die bisherigen Texturkoordinaten."))
            .action_describe("markUvSeam", LocalizedLabel::native("Marks (or with seam false unmarks) the given or selected edges of the active object as UV seams for the next unwrap.", "Markiert die angegebenen oder ausgewählten Kanten des aktiven Objekts als UV-Nähte für die nächste Abwicklung (mit seam false wird die Markierung entfernt)."))
            .action_describe("clearSeam", LocalizedLabel::native("Removes every UV seam mark from the active object.", "Entfernt alle UV-Nahtmarkierungen vom aktiven Objekt."))
            .action_describe("translateSelection", LocalizedLabel::native("Moves the mesh selection — the selected vertices, edges or faces of the active object, else every selected object — by dx, dy and dz as one undoable edit whose offset stays editable in history.", "Verschiebt die Netzauswahl — die ausgewählten Punkte, Kanten oder Flächen des aktiven Objekts, sonst jedes ausgewählte Objekt — um dx, dy und dz als eine rückgängig machbare Änderung, deren Versatz im Verlauf bearbeitbar bleibt."))
            .action_describe("rotateSelection", LocalizedLabel::native("Rotates the mesh selection by an angle in radians around the axis ax, ay, az through its centre as one undoable edit whose angle and axis stay editable in history.", "Dreht die Netzauswahl um einen Winkel im Bogenmaß um die Achse ax, ay, az durch ihre Mitte als eine rückgängig machbare Änderung, deren Winkel und Achse im Verlauf bearbeitbar bleiben."))
            .action_describe("scaleSelection", LocalizedLabel::native("Scales the mesh selection by sx, sy and sz about its centre as one undoable edit whose factors stay editable in history.", "Skaliert die Netzauswahl um sx, sy und sz um ihre Mitte als eine rückgängig machbare Änderung, deren Faktoren im Verlauf bearbeitbar bleiben."))
            .action_describe("addPaintLayer", LocalizedLabel::native("Adds a new, named paint layer to the given or active object's texture.", "Fügt der Textur des angegebenen oder aktiven Objekts eine neue, benannte Malebene hinzu."))
            .action_describe("paintAt", LocalizedLabel::native("Paints one dab of the active brush at the texture point u, v of the given or active object; a host drag streams its dabs into one stroke that the release writes as one undoable edit whose brush and dabs stay editable in history.", "Malt einen Tupfer des aktiven Pinsels am Texturpunkt u, v des angegebenen oder aktiven Objekts; ein Ziehen des Hosts sammelt seine Tupfer zu einem Strich, den das Loslassen als eine rückgängig machbare Änderung schreibt, deren Pinsel und Tupfer im Verlauf bearbeitbar bleiben."))
            .action_describe("paintFill", LocalizedLabel::native("Flood-fills the region of the active paint layer around the texture point u, v (the layer's centre when none is given) with the brush colour.", "Füllt den Bereich der aktiven Malebene um den Texturpunkt u, v (ohne Angabe die Ebenenmitte) mit der Pinselfarbe."))
            .action_describe("fillBucket", LocalizedLabel::native("Bucket-fills the connected area of the active paint layer at u, v (its centre when omitted) with the brush colour, exactly as Paint Fill does.", "Füllt die zusammenhängende Fläche der aktiven Malebene bei u, v (ohne Angabe ihre Mitte) mit der Pinselfarbe, genau wie Füllen malen."))
            .action_describe("exportMesh", LocalizedLabel::native("Writes the scene's meshes in the chosen format to a downloaded file on the user's machine.", "Schreibt die Netze der Szene im gewählten Format in eine heruntergeladene Datei auf dem Rechner des Nutzers."))
            .action_describe("loadMeshRequest", LocalizedLabel::native("Opens the host's file picker for a mesh file; the chosen file then replaces the whole document.", "Öffnet die Dateiauswahl des Hosts für eine Netzdatei; die gewählte Datei ersetzt dann das gesamte Dokument."))
            .action_describe("importMeshFile", LocalizedLabel::native("Replaces the whole document with the meshes read from an imported mesh file; the previous scene is discarded.", "Ersetzt das gesamte Dokument durch die Netze aus einer importierten Netzdatei; die bisherige Szene wird verworfen."))
            .action_describe("deleteSelection", LocalizedLabel::native("Deletes what the mesh selection holds: whole objects when objects are selected, otherwise the selected faces of the active object.", "Löscht, was die Netzauswahl enthält: ganze Objekte, wenn Objekte ausgewählt sind, sonst die ausgewählten Flächen des aktiven Objekts."))
            .action_describe("duplicateObject", LocalizedLabel::native("Copies the given or active object and adds the copy to the scene.", "Kopiert das angegebene oder aktive Objekt und fügt die Kopie der Szene hinzu."))
            .action_describe("importSnapshotJson", LocalizedLabel::native("Replaces the whole lowpoly document with one parsed from the given snapshot JSON; invalid JSON changes nothing.", "Ersetzt das gesamte Lowpoly-Dokument durch eines aus dem angegebenen Snapshot-JSON; ungültiges JSON ändert nichts."))
            .action_describe("replaceSnapshotJson", LocalizedLabel::native("Loads a test fixture given as JSON as the whole lowpoly document, replacing the current scene; invalid JSON changes nothing.", "Lädt eine als JSON übergebene Test-Fixture als gesamtes Lowpoly-Dokument und ersetzt die aktuelle Szene; ungültiges JSON ändert nichts."))
            .action_describe("setActiveObject", LocalizedLabel::native("Makes the object with the given id the active one that mesh edits apply to; only the editor's view state changes.", "Macht das Objekt mit der angegebenen Id zum aktiven, auf das Netzänderungen wirken; nur der Ansichtszustand des Editors ändert sich."))
            .action_describe("setActivePaintLayer", LocalizedLabel::native("Makes the given paint layer the one painting writes to; only the editor's view state changes.", "Macht die angegebene Malebene zu der, auf die gemalt wird; nur der Ansichtszustand des Editors ändert sich."))
            .action_describe("setUtilityParam", LocalizedLabel::native("Sets one parameter of the armed tool (such as extrude distance or brush size) from a JSON value; only the tool setting changes.", "Setzt einen Parameter des gewählten Werkzeugs (etwa Extrusionsabstand oder Pinselgröße) aus einem JSON-Wert; nur die Werkzeugeinstellung ändert sich."))
            .action_describe("toggleShowEdges", LocalizedLabel::native("Shows or hides the mesh edges in the model window; only the view changes.", "Blendet die Netzkanten im Modellfenster ein oder aus; nur die Ansicht ändert sich."))
            .action_describe("toggleSun", LocalizedLabel::native("Switches the model window's sun light on or off; only the view changes.", "Schaltet das Sonnenlicht des Modellfensters ein oder aus; nur die Ansicht ändert sich."))
            .action_describe("setSunAzimuth", LocalizedLabel::native("Sets the compass direction the model window's sun shines from; only the view changes.", "Legt die Himmelsrichtung fest, aus der die Sonne des Modellfensters scheint; nur die Ansicht ändert sich."))
            .action_describe("setSunElevation", LocalizedLabel::native("Sets how high the model window's sun stands above the horizon; only the view changes.", "Legt fest, wie hoch die Sonne des Modellfensters über dem Horizont steht; nur die Ansicht ändert sich."))
            .action_describe("setSunIntensity", LocalizedLabel::native("Sets the brightness of the model window's sun; only the view changes.", "Legt die Helligkeit der Sonne des Modellfensters fest; nur die Ansicht ändert sich."))
            .action_audience("setCamera", semio_framework_plugin::CapabilityAudience::Chrome)
            .action_audience("paintStroke", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("paintAt", semio_framework_plugin::CapabilityAudience::Input)
            .action_audience("paintSample", semio_framework_plugin::CapabilityAudience::Input)
            .action_destructive("importSnapshotJson")
            .action_destructive("importMeshFile")
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🧪️UnitTests
/// 🧪️ Shared test scaffolding for every taxonomy node's own `🧪️Tests` region — a component file must be
/// able to drive the whole app without re-deriving the harness.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod unit_tests;

#[cfg(test)]
#[path = "🧪️tests/🔌️mounted/🦀️.rs"]
mod mounted_tests;
//#endregion 🧪️UnitTests


#[cfg(test)]
use semio_framework_plugin::InteractionRef;

//#region 🪢️TaxonomyMounts
#[path = "📚️examples/🎬️demo-session/🦀️.rs"]
pub mod demo_session;
#[cfg(test)]
#[path = "📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod example;
//#endregion 🪢️TaxonomyMounts
