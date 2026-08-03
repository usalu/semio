//! 🎮️ Cad app — DocumentApp impl, render, manifest (constitutional: ui).

use cad_document::{
    cad_all_objects, cad_find_object_pane, cad_pane_from_model_definition_id, cad_pane_geometry,
    cad_pane_objects, CadCamera, CadGeometry, CadNode, CadObject, CadPaneId, CadReference,
    CadScene, CAD_DOCUMENT_SCHEMA,
};
use cad_document_engine::{
    interaction,
    interaction::{
        apply_event, can_commit, commit_object, keyed_transitions, list_interactions_for_model_definition,
        parse_repl_line, preview_display_items, resolve_interaction_key, start_session, CadEngagementSession,
    },
    transformation::{apply_from_building, apply_typology_fallback, run_derive_from_geometry, solid_for_object},
    CAD_EXAMPLE_FOREST_LEFT, CAD_MODEL_DEFINITION_BUILDING, CAD_MODEL_DEFINITION_ENERGY, CAD_MODEL_DEFINITION_SHAPE,
    CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC,
};
use cad_document_engine::{
    cad_brep_kernel, cad_camera_distance, cad_camera_projection_config,
    cad_camera_set_projection_config, cad_io, cad_sun_config_from_world, cad_sun_config_to_world,
    collect_mesh_urls, default_document, ensure_object_solid_handle,
    export_solids_as, forest_play_camera, forest_play_scene, import_cad_object_by_extension, next_cad_id, object_mesh_data,
    object_scale_json, primary_primitive_kind,
    resolve_object_mesh_url, scene_from_spatial_payload, unwrap_spatial_load_payload,
    CadComponentSelection, CadConfig, CadDislocateOptions, CadHoverTarget, CadSelectionTargets, CadSolidExport,
};
use cad_document_op::{CadConfigOperation, CadObjectPatch, CadOperation, CadReferencePatch};
use cad_document_protocol::CadCommand;
use semio_framework_plugin::{
    apply_world3d_projection_action, apply_world3d_sun_action, build_world_3d_scene,
    merge_world_selection_ids, SelectionSet, mesh_from_kind, ui_inspector_groups_to_tree,
    ui_inspector_mixed_text, ui_inspector_mixed_toggle, ui_inspector_readonly_field, ui_inspector_stepper_field,
    ui_inspector_vec3_group, ui_stack_vertical, ui_text, world3d_camera_projection_json, world3d_chunking_json,
    world3d_environment_json, world3d_mesh_id_from_url, world3d_projection_action_moves_pose,
    world3d_projection_measures, world3d_projection_pose, world3d_scene_extended, world3d_selection_json,
    world3d_sun_measures, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, AppLabelsOverlay,
    AppLabelsOverlayExt, ArtifactKindSpec, ConfigView, DocumentApp, DocumentView, Emit, Media, MediaClass, MediaError, MediaForm,
    MediaPayload, MediaType,
    OsMediaCapability, OsMediaFormat, PanelGroup, PanelTreeBuilder, IconName, SET_ACTIVE_UTILITY_ACTION_ID,
    UiFieldNode, UiGroupNode, UiInputNode, UiInspectorFieldGroup, UiNode, UiPresence, UiSelectItem, UiSelectNode,
    UiTreeItemAction, UiTreeItemNode, UtilityCategory, UtilityDefinition, WindowEngagement,
    AppActionRegistry, ContextMenuItemSpec, ContextMenuRequest, Menu,
    WindowEngagementInput, WindowEngagementPossible, WindowEngagementStatus, WindowLayout, WindowLayoutAxisNode,
    WindowLayoutChild, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, WindowMeasure,
    WorldSunConfig, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
    FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ID,
    FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, localized_label_map, tree_item,
};
use semio_framework_core::kernel::HostEffect;
use kernel_3d_engine::{BrepKernel, GeometryHandle};
use serde::{Deserialize, Serialize};
use base64::Engine as _;
use serde_json::{json, Value};
use std::collections::HashMap;
use ui_wgpu::SurfaceKind;

//#region 🔖️Constants
const CAD_PLAY_APP_ID: &str = "cad-play";

const CAD_PLAY_CONTROLLER_ID: &str = "cad-play";

const CAD_PLAY_BODY_SHAPE: &str = "cad.play.shape";

const CAD_PLAY_BODY_BUILDING: &str = "cad.play.building";

const CAD_PLAY_BODY_ENERGY: &str = "cad.play.energy";

const CAD_PLAY_BODY_STRUCTURE_CLASSIC: &str = "cad.play.structure-classic";

const CAD_PLAY_BODY_DOCUMENT: &str = "cad.play.document";

const CAD_PLAY_BODY_CATALOGUE: &str = "cad.play.catalogue";

const CAD_PLAY_BODY_PROPERTIES: &str = "cad.play.properties";

const CAD_PLAY_SURFACE_SHAPE: &str = "cad.play.scene3d/shape";

const CAD_PLAY_SURFACE_BUILDING: &str = "cad.play.scene3d/building";

const CAD_PLAY_SURFACE_ENERGY: &str = "cad.play.scene3d/energy";

const CAD_PLAY_SURFACE_STRUCTURE_CLASSIC: &str = "cad.play.scene3d/structure-classic";

const CAD_PLAY_WINDOW_SHAPE: &str = "cad-play-shape";

const CAD_PLAY_WINDOW_BUILDING: &str = "cad-play-building";

const CAD_PLAY_WINDOW_ENERGY: &str = "cad-play-energy";

const CAD_PLAY_WINDOW_STRUCTURE_CLASSIC: &str = "cad-play-structure-classic";

const CAD_DISLOCATE_UTILITY_ID: &str = "dislocate";

const CAD_FALLBACK_MESH_KIND: &str = "box";

struct CadTypologyEntry {
    typology: &'static str,
    label: &'static str,
    icon: &'static str,
    model_definition_id: &'static str,
}

const TYPOLOGY_CATALOG: &[CadTypologyEntry] = &[
    CadTypologyEntry {
        typology: "spatial.shape.primitive.box",
        label: "Box",
        icon: "box",
        model_definition_id: CAD_MODEL_DEFINITION_SHAPE,
    },
    CadTypologyEntry {
        typology: "building.building.slab",
        label: "Slab",
        icon: "square",
        model_definition_id: CAD_MODEL_DEFINITION_BUILDING,
    },
    CadTypologyEntry {
        typology: "building.building.column",
        label: "Column",
        icon: "columns",
        model_definition_id: CAD_MODEL_DEFINITION_BUILDING,
    },
    CadTypologyEntry {
        typology: "building.building.beam",
        label: "Beam",
        icon: "minus",
        model_definition_id: CAD_MODEL_DEFINITION_BUILDING,
    },
    CadTypologyEntry {
        typology: "building.building.wall",
        label: "Wall",
        icon: "panel-top",
        model_definition_id: CAD_MODEL_DEFINITION_BUILDING,
    },
    CadTypologyEntry {
        typology: "energy.energy.externalwall",
        label: "External Wall",
        icon: "panel-top",
        model_definition_id: CAD_MODEL_DEFINITION_ENERGY,
    },
    CadTypologyEntry {
        typology: "structure.structure.onewayreinforcedconcreteslab",
        label: "Slab",
        icon: "square",
        model_definition_id: CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC,
    },
    CadTypologyEntry {
        typology: "structure.structure.reinforcedconcretecolumn",
        label: "Column",
        icon: "columns",
        model_definition_id: CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC,
    },
];

struct CadTransformationSpec {
    id: &'static str,
    source_model_definition_id: &'static str,
    target_model_definition_id: &'static str,
    mode: TransformationMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TransformationMode {
    DeriveFromGeometry,
    FromBuilding,
    TypologyFallback,
}

const CAD_TRANSFORMATION_SPECS: &[CadTransformationSpec] = &[
    CadTransformationSpec {
        id: "from_geometry",
        source_model_definition_id: CAD_MODEL_DEFINITION_SHAPE,
        target_model_definition_id: CAD_MODEL_DEFINITION_ENERGY,
        mode: TransformationMode::DeriveFromGeometry,
    },
    CadTransformationSpec {
        id: "from_building",
        source_model_definition_id: CAD_MODEL_DEFINITION_BUILDING,
        target_model_definition_id: CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC,
        mode: TransformationMode::FromBuilding,
    },
    CadTransformationSpec {
        id: "classic",
        source_model_definition_id: CAD_MODEL_DEFINITION_BUILDING,
        target_model_definition_id: CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC,
        mode: TransformationMode::TypologyFallback,
    },
];

//#endregion 🔖️Constants

//#region 🔖️Document
/// @emoji 🎯️ `CadHoverTarget`/`CadSelectionTargets`/`CadComponentSelection`/`CadDislocateOptions` moved
/// to `cad_document_engine` (WORKFLOWS-END-TO-END-TYPED-PORTS config recipe): `CadConfig` embeds them
/// as `dsl::DslRecord` block fields, and `CadPlayRuntime` below reuses the exact same types (imported,
/// not redefined) so the two structs stay field-for-field convertible with no shape drift.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CadPlayRuntime {
    #[serde(default)]
    selected_object_ids: SelectionSet,
    #[serde(default)]
    selected_node_ids: Vec<String>,
    #[serde(default = "default_selection_method")]
    selection_method: String,
    #[serde(default)]
    hovered_object_id: Option<String>,
    #[serde(default)]
    hovered_target: Option<CadHoverTarget>,
    #[serde(default)]
    active_object_id: Option<String>,
    #[serde(default)]
    component_selection: CadComponentSelection,
    #[serde(default)]
    engagement_input: String,
    #[serde(default)]
    engagement_step: String,
    #[serde(default)]
    active_example_id: Option<String>,
    #[serde(default)]
    selected_reference_model_definition_id: Option<String>,
    #[serde(default)]
    selected_reference_id: Option<String>,
    #[serde(default)]
    selected_primitive_id: Option<String>,
    #[serde(default)]
    selected_primitive_kind: Option<String>,
    #[serde(default)]
    engagement_pane: Option<String>,
    #[serde(default)]
    engagement_session: Option<CadEngagementSession>,
    #[serde(default)]
    last_finalized_interaction_id: Option<String>,
    #[serde(default)]
    sun: WorldSunConfig,
    /// 🎥️ Per-pane camera pose — session-only view state (never a VCS-tracked document field): see
    /// `"setCamera"`/`"setProjection"`/`"setProjectionParam"` in `handle_action` below.
    #[serde(default)]
    camera: CadCamera,
    #[serde(default)]
    camera_building: CadCamera,
    #[serde(default)]
    camera_energy: CadCamera,
    #[serde(default)]
    camera_structure_classic: CadCamera,
    #[serde(default)]
    dislocate_options_by_window_id: HashMap<String, CadDislocateOptions>,
}

fn default_selection_method() -> String {
    "rectangle".into()
}

impl Default for CadPlayRuntime {
    fn default() -> Self {
        Self {
            selected_object_ids: SelectionSet::default(),
            selected_node_ids: Vec::new(),
            selection_method: default_selection_method(),
            hovered_object_id: None,
            hovered_target: None,
            active_object_id: None,
            component_selection: CadComponentSelection::default(),
            engagement_input: String::new(),
            engagement_step: "Idle".into(),
            active_example_id: None,
            selected_reference_model_definition_id: None,
            selected_reference_id: None,
            selected_primitive_id: None,
            selected_primitive_kind: None,
            engagement_pane: None,
            engagement_session: None,
            last_finalized_interaction_id: None,
            sun: WorldSunConfig::default(),
            camera: CadCamera::default(),
            camera_building: CadCamera::default(),
            camera_energy: CadCamera::default(),
            camera_structure_classic: CadCamera::default(),
            dislocate_options_by_window_id: HashMap::new(),
        }
    }
}

impl CadPlayRuntime {
    /// 🪟️ Reads the Dislocate handle configuration for one window instance without sharing it with siblings.
    fn dislocate_options(&self, window_id: &str) -> CadDislocateOptions {
        self.dislocate_options_by_window_id.get(window_id).copied().unwrap_or_default()
    }
}

/// @emoji 🔀️ WORKFLOWS-END-TO-END-TYPED-PORTS config recipe boundary (in): unpacks `cfg.projection`
/// (the persisted, VCS-tracked `CadConfig`) into the ergonomic `CadPlayRuntime` scratch shape every
/// helper function below already works with — a pure, allocation-only conversion, never itself an
/// operation. `dislocate_options_by_window_id` is seeded from the 4 fixed pane fields keyed by the 4
/// constant window-kind ids (`CAD_PLAY_WINDOW_*`) — see `CadDislocateOptions`'s doc comment in
/// `cad_document_engine` for why per-window-INSTANCE keying no longer applies.
fn cad_runtime_from_config(cfg: &CadConfig) -> CadPlayRuntime {
    CadPlayRuntime {
        selected_object_ids: SelectionSet::from(cfg.selected_object_ids.clone()),
        selected_node_ids: cfg.selected_node_ids.clone(),
        selection_method: cfg.selection_method.clone(),
        hovered_object_id: cfg.hovered_object_id.clone(),
        hovered_target: cfg.hovered_target.clone(),
        active_object_id: cfg.active_object_id.clone(),
        component_selection: cfg.component_selection.clone(),
        engagement_input: cfg.engagement_input.clone(),
        engagement_step: cfg.engagement_step.clone(),
        active_example_id: cfg.active_example_id.clone(),
        selected_reference_model_definition_id: cfg.selected_reference_model_definition_id.clone(),
        selected_reference_id: cfg.selected_reference_id.clone(),
        selected_primitive_id: cfg.selected_primitive_id.clone(),
        selected_primitive_kind: cfg.selected_primitive_kind.clone(),
        engagement_pane: cfg.engagement_pane.clone(),
        engagement_session: cfg.engagement_session_json.as_deref().and_then(|json| serde_json::from_str(json).ok()),
        last_finalized_interaction_id: cfg.last_finalized_interaction_id.clone(),
        sun: cad_sun_config_to_world(&cfg.sun),
        camera: cfg.camera.clone(),
        camera_building: cfg.camera_building.clone(),
        camera_energy: cfg.camera_energy.clone(),
        camera_structure_classic: cfg.camera_structure_classic.clone(),
        dislocate_options_by_window_id: HashMap::from([
            (CAD_PLAY_WINDOW_SHAPE.to_string(), cfg.dislocate_shape),
            (CAD_PLAY_WINDOW_BUILDING.to_string(), cfg.dislocate_building),
            (CAD_PLAY_WINDOW_ENERGY.to_string(), cfg.dislocate_energy),
            (CAD_PLAY_WINDOW_STRUCTURE_CLASSIC.to_string(), cfg.dislocate_structure_classic),
        ]),
    }
}

/// @emoji 🔀️ The `cad_runtime_from_config` boundary's outbound twin: repacks the (possibly mutated)
/// `CadPlayRuntime` scratch struct back into a real `CadConfig` snapshot for
/// `CadConfigOperation::Snapshot`. `active_utility_id`/`locale` aren't part of `CadPlayRuntime` (they
/// never had a runtime-side representation pre-B1 either — they were read straight off `ViewState`),
/// so callers that need to change them patch the returned `CadConfig` directly instead of threading
/// them through `CadPlayRuntime`.
fn cad_config_from_runtime(runtime: &CadPlayRuntime, base: &CadConfig) -> CadConfig {
    CadConfig {
        selected_object_ids: runtime.selected_object_ids.to_vec(),
        selected_node_ids: runtime.selected_node_ids.clone(),
        selection_method: runtime.selection_method.clone(),
        hovered_object_id: runtime.hovered_object_id.clone(),
        hovered_target: runtime.hovered_target.clone(),
        active_object_id: runtime.active_object_id.clone(),
        component_selection: runtime.component_selection.clone(),
        engagement_input: runtime.engagement_input.clone(),
        engagement_step: runtime.engagement_step.clone(),
        active_example_id: runtime.active_example_id.clone(),
        selected_reference_model_definition_id: runtime.selected_reference_model_definition_id.clone(),
        selected_reference_id: runtime.selected_reference_id.clone(),
        selected_primitive_id: runtime.selected_primitive_id.clone(),
        selected_primitive_kind: runtime.selected_primitive_kind.clone(),
        engagement_pane: runtime.engagement_pane.clone(),
        engagement_session_json: runtime.engagement_session.as_ref().map(|session| serde_json::to_string(session).unwrap_or_default()),
        last_finalized_interaction_id: runtime.last_finalized_interaction_id.clone(),
        sun: cad_sun_config_from_world(&runtime.sun),
        camera: runtime.camera.clone(),
        camera_building: runtime.camera_building.clone(),
        camera_energy: runtime.camera_energy.clone(),
        camera_structure_classic: runtime.camera_structure_classic.clone(),
        dislocate_shape: runtime.dislocate_options(CAD_PLAY_WINDOW_SHAPE),
        dislocate_building: runtime.dislocate_options(CAD_PLAY_WINDOW_BUILDING),
        dislocate_energy: runtime.dislocate_options(CAD_PLAY_WINDOW_ENERGY),
        dislocate_structure_classic: runtime.dislocate_options(CAD_PLAY_WINDOW_STRUCTURE_CLASSIC),
        active_utility_id: base.active_utility_id.clone(),
        locale: base.locale.clone(),
        terminology: base.terminology.clone(),
    }
}

/// 🗣️ B1: `cfg.locale`-driven counterpart of the deleted `ViewState`-driven `is_de_locale`.
fn cad_is_de_locale(cfg: &CadConfig) -> bool {
    cfg.locale.starts_with("de")
}

/// 🎥️ Reads the runtime-owned camera for `pane` — the session-only replacement for the old
/// document-backed `cad_pane_camera`.
fn cad_pane_camera_runtime(runtime: &CadPlayRuntime, pane: CadPaneId) -> &CadCamera {
    match pane {
        CadPaneId::Shape => &runtime.camera,
        CadPaneId::Building => &runtime.camera_building,
        CadPaneId::Energy => &runtime.camera_energy,
        CadPaneId::StructureClassic => &runtime.camera_structure_classic,
    }
}

/// 🎥️ Mutable counterpart of `cad_pane_camera_runtime`.
fn cad_pane_camera_runtime_mut(runtime: &mut CadPlayRuntime, pane: CadPaneId) -> &mut CadCamera {
    match pane {
        CadPaneId::Shape => &mut runtime.camera,
        CadPaneId::Building => &mut runtime.camera_building,
        CadPaneId::Energy => &mut runtime.camera_energy,
        CadPaneId::StructureClassic => &mut runtime.camera_structure_classic,
    }
}

/// @emoji 🎛️ Ephemeral read/render view assembled per call from the store's materialized
/// `CadScene` projection and the app's `CadPlayRuntime` view-state. Replaces the old persisted play
/// envelope: its embedded history/undo stacks are now owned by the wrapping `VcsDocumentApp`'s
/// `DocumentStore`, and its runtime view-state lives directly on the `CadPlayApp` struct.
struct CadPlayView {
    document: CadScene,
    runtime: CadPlayRuntime,
}

fn cad_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor {
        controller_id: CAD_PLAY_CONTROLLER_ID.into(),
        action: action.into(),
        args: semio_framework_plugin::optional_json_to_dsl(args),
    }
}

fn camera_json(camera: &CadCamera) -> String {
    world3d_camera_projection_json(camera.position, camera.target, None, camera.zoom, &cad_camera_projection_config(camera))
}


fn cad_pane_id_from_suffix(id_suffix: &str) -> CadPaneId {
    match id_suffix {
        "building" => CadPaneId::Building,
        "energy" => CadPaneId::Energy,
        "structure-classic" => CadPaneId::StructureClassic,
        _ => CadPaneId::Shape,
    }
}

fn cad_pane_id_from_surface_id(surface_id: &str) -> CadPaneId {
    let suffix = surface_id.split('/').last().unwrap_or(surface_id);
    cad_pane_id_from_suffix(suffix)
}

fn cad_pane_suffix(pane: CadPaneId) -> &'static str {
    match pane {
        CadPaneId::Shape => "shape",
        CadPaneId::Building => "building",
        CadPaneId::Energy => "energy",
        CadPaneId::StructureClassic => "structure-classic",
    }
}

/// @emoji 🔁️ Derives the target-pane objects for transformation `qid` and returns the operations
/// that both replace the target pane and refocus onto the target model definition — dispatched by
/// the caller through the store (no direct mutation).
fn apply_transformation_operations(document: &CadScene, qid: &str) -> Vec<CadOperation> {
    let Some((model_definition_id, transformation_id)) = qid.rsplit_once('.') else {
        return Vec::new();
    };
    let Some(spec) = CAD_TRANSFORMATION_SPECS.iter().find(|entry| {
        entry.source_model_definition_id == model_definition_id && entry.id == transformation_id
    }) else {
        return Vec::new();
    };
    let Some(source_pane) = cad_pane_from_model_definition_id(spec.source_model_definition_id) else {
        return Vec::new();
    };
    let Some(target_pane) = cad_pane_from_model_definition_id(spec.target_model_definition_id) else {
        return Vec::new();
    };
    let objects = {
        let source_objects: Vec<CadObject> = cad_pane_objects(document, source_pane)
            .iter()
            .cloned()
            .collect();
        let Ok(mut kernel) = cad_brep_kernel().lock() else {
            return Vec::new();
        };
        let mut prepared = source_objects;
        for object in &mut prepared {
            ensure_object_solid_handle(&mut **kernel, object);
        }
        match spec.mode {
            TransformationMode::DeriveFromGeometry => {
                run_derive_from_geometry(&mut **kernel, &prepared, "derived-energy")
            }
            TransformationMode::FromBuilding => apply_from_building(&prepared, "derived-structure"),
            TransformationMode::TypologyFallback => apply_typology_fallback(
                &prepared,
                &[
                    "building.building.slab",
                    "building.building.column",
                    "building.building.beam",
                    "building.building.wall",
                ],
                "derived-fallback",
            ),
        }
    };
    vec![
        CadOperation::SetPaneObjects {
            pane: target_pane,
            objects,
        },
        CadOperation::SetActiveModelDefinition {
            model_definition_id: spec.target_model_definition_id.into(),
        },
    ]
}

fn collect_pane_solids(kernel: &mut dyn BrepKernel, envelope: &CadPlayView, pane: CadPaneId) -> Vec<GeometryHandle> {
    cad_pane_objects(&envelope.document, pane)
        .iter()
        .filter_map(|object| {
            let mut next = object.clone();
            solid_for_object(kernel, &mut next)
        })
        .collect()
}

fn collect_modelspace_solids(kernel: &mut dyn BrepKernel, envelope: &CadPlayView) -> Vec<GeometryHandle> {
    CadPaneId::all()
        .into_iter()
        .flat_map(|pane| collect_pane_solids(kernel, envelope, pane))
        .collect()
}

fn export_solid_for_pane(envelope: &CadPlayView, pane: CadPaneId, format: OsMediaFormat) -> Option<CadSolidExport> {
    let Ok(mut kernel) = cad_brep_kernel().lock() else {
        return None;
    };
    let solids = collect_pane_solids(&mut **kernel, envelope, pane);
    if solids.is_empty() {
        return None;
    }
    let stem = format!("cad-{}", pane.model_definition_id().replace('.', "-"));
    export_solids_as(&mut **kernel, &solids, format, &stem)
}

fn export_solid_modelspace(envelope: &CadPlayView, format: OsMediaFormat) -> Option<CadSolidExport> {
    let Ok(mut kernel) = cad_brep_kernel().lock() else {
        return None;
    };
    let solids = collect_modelspace_solids(&mut **kernel, envelope);
    if solids.is_empty() {
        return None;
    }
    export_solids_as(&mut **kernel, &solids, format, "cad.modelspace")
}

/// @emoji ⬇️ Converts a staged native-geometry export into a download host effect emitted directly
/// to the shell (no document mutation, no pending-export runtime slot).
fn cad_solid_export_effect(export: CadSolidExport) -> HostEffect {
    let data = match export.data {
        Value::String(text) => text,
        other => serde_json::to_string(&other).unwrap_or_default(),
    };
    HostEffect::DownloadMediaExport {
        filename: export.filename,
        mime_type: export.mime_type,
        data,
        encoding: export.encoding,
    }
}

/// @emoji ⬇️ Wraps a spatial-JSON export document into a download host effect.
fn cad_spatial_export_effect(value: Value, filename: &str) -> HostEffect {
    HostEffect::DownloadMediaExport {
        filename: filename.into(),
        mime_type: "text/plain".into(),
        data: serde_json::to_string(&value).unwrap_or_default(),
        encoding: None,
    }
}

fn export_spatial_json(envelope: &CadPlayView, mode: &str) -> Value {
    let models: Vec<Value> = CadPaneId::all()
        .into_iter()
        .map(|pane| {
            json!({
                "id": pane.model_definition_id(),
                "model": {
                    "schema": "spatial.model",
                    "revision": 1,
                    "objects": cad_pane_objects(&envelope.document, pane),
                }
            })
        })
        .collect();
    match mode {
        "selected" => {
            let pane = cad_pane_from_model_definition_id(&envelope.document.active_model_definition_id)
                .unwrap_or(CadPaneId::Shape);
            let selected: Vec<&CadObject> = envelope
                .runtime
                .selected_object_ids
                .iter()
                .filter_map(|id| {
                    cad_all_objects(&envelope.document)
                        .find(|(object, _)| &object.id == id)
                        .map(|(object, _)| object)
                })
                .collect();
            let model = json!({
                "schema": "spatial.model",
                "revision": 1,
                "objects": selected,
            });
            let model_space = json!({
                "schema": "spatial.modelspace",
                "revision": 1,
                "models": [{
                    "id": pane.model_definition_id(),
                    "model": model,
                }],
            });
            json!({
                "model": model,
                "modelSpace": model_space,
                "activeModelDefinitionId": pane.model_definition_id(),
            })
        }
        "current" => {
            let pane = cad_pane_from_model_definition_id(&envelope.document.active_model_definition_id)
                .unwrap_or(CadPaneId::Shape);
            json!({
                "schema": "spatial.model",
                "revision": 1,
                "modelDefinitionId": pane.model_definition_id(),
                "objects": cad_pane_objects(&envelope.document, pane),
            })
        }
        _ => json!({
            "schema": "spatial.modelspace",
            "revision": 1,
            "activeModelDefinitionId": envelope.document.active_model_definition_id,
            "models": models,
        }),
    }
}

fn normalize_component_selection_mode(mode: &str) -> String {
    match mode {
        "vertex" | "edge" | "face" | "mesh" | "object" => {
            if mode == "object" {
                "mesh".into()
            } else {
                mode.into()
            }
        }
        _ => "mesh".into(),
    }
}

fn enable_component_selection_target(targets: &mut CadSelectionTargets, mode: &str) {
    match mode {
        "vertex" => targets.vertex = true,
        "edge" => targets.edge = true,
        "face" => targets.face = true,
        "mesh" | "object" => targets.mesh = true,
        _ => {}
    }
}

fn merge_component_selection_ids(existing: &[u32], incoming: &[u32], merge: &str) -> Vec<u32> {
    match merge {
        "add" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if !merged.contains(id) {
                    merged.push(*id);
                }
            }
            merged
        }
        "toggle" | "invertive" => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if let Some(index) = merged.iter().position(|entry| entry == id) {
                    merged.remove(index);
                } else {
                    merged.push(*id);
                }
            }
            merged
        }
        "remove" | "subtractive" => existing.iter().copied().filter(|id| !incoming.contains(id)).collect(),
        _ => incoming.to_vec(),
    }
}

fn clear_component_selection(runtime: &mut CadPlayRuntime) {
    runtime.component_selection.mode = "mesh".into();
    runtime.component_selection.ids.clear();
}

fn apply_component_selection(runtime: &mut CadPlayRuntime, mode: &str, incoming: &[u32], merge: &str, object_id: Option<&str>) {
    let normalized = normalize_component_selection_mode(mode);
    enable_component_selection_target(&mut runtime.component_selection.targets, &normalized);
    runtime.component_selection.mode = normalized.clone();
    if normalized == "mesh" {
        runtime.component_selection.ids.clear();
        return;
    }
    runtime.component_selection.ids =
        merge_component_selection_ids(&runtime.component_selection.ids, incoming, merge);
    if let Some(object_id) = object_id {
        runtime.active_object_id = Some(object_id.into());
        if merge == "replace" || runtime.selected_object_ids.is_empty() {
            runtime.selected_object_ids = SelectionSet::from(vec![object_id.into()]);
        } else if !runtime.selected_object_ids.contains(object_id) {
            runtime.selected_object_ids.push_unique(object_id.into());
        }
    }
}

fn resolve_active_object_id(runtime: &CadPlayRuntime) -> Option<String> {
    runtime
        .active_object_id
        .clone()
        .or_else(|| runtime.selected_object_ids.first().map(str::to_string))
}

fn instance_is_component_hovered(runtime: &CadPlayRuntime, object_id: &str) -> bool {
    runtime
        .hovered_target
        .as_ref()
        .map(|target| {
            target.mode.as_deref() == Some("mesh") && target.object_id.as_deref() == Some(object_id)
        })
        .unwrap_or_else(|| runtime.hovered_object_id.as_deref() == Some(object_id))
}

/// @emoji 🕹️ Whether this window's active Dislocate utility has a visible handle for the selection.
fn gumball_active(runtime: &CadPlayRuntime, active_utility: Option<&str>, options: CadDislocateOptions) -> bool {
    active_utility == Some(CAD_DISLOCATE_UTILITY_ID)
        && (options.move_enabled || options.rotate_enabled)
        && (!runtime.selected_object_ids.is_empty() || !runtime.component_selection.ids.is_empty())
}

/// @emoji 🎯️ World-space pivot for the gumball: centroid of selected objects across all panes.
fn gumball_target_for(document: &CadScene, selected_ids: &[String]) -> Option<[f64; 3]> {
    let mut sum = [0.0; 3];
    let mut count = 0usize;
    for (object, _) in cad_all_objects(document) {
        if selected_ids.contains(&object.id) {
            sum[0] += object.origin[0];
            sum[1] += object.origin[1];
            sum[2] += object.origin[2];
            count += 1;
        }
    }
    if count == 0 {
        return None;
    }
    let n = count as f64;
    Some([sum[0] / n, sum[1] / n, sum[2] / n])
}

fn world_instances_json(objects: &[CadObject], runtime: &CadPlayRuntime) -> String {
    let instances: Vec<Value> = objects
        .iter()
        .filter(|object| object.visible)
        .map(|object| {
            let mesh_id = resolve_object_mesh_url(object)
                .map(|url| world3d_mesh_id_from_url(&url))
                .unwrap_or_else(|| object.id.clone());
            let selected = runtime.selected_object_ids.contains(&object.id);
            let hovered = instance_is_component_hovered(runtime, &object.id);
            json!({
                "id": object.id,
                "meshId": mesh_id,
                "position": [
                    object.origin.first().copied().unwrap_or(0.0),
                    object.origin.get(1).copied().unwrap_or(0.0),
                    object.origin.get(2).copied().unwrap_or(0.0),
                ],
                "rotation": object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                "scale": object_scale_json(object),
                "label": object.label,
                "color": if selected { "#3b82f6" } else { "#64748b" },
                "selected": selected,
                "hovered": hovered,
            })
        })
        .collect();
    serde_json::to_string(&instances).unwrap_or_else(|_| "[]".into())
}

fn world_meshes_json(objects: &[CadObject], geometry: Option<&CadGeometry>) -> String {
    let urls = collect_mesh_urls(objects);
    if !urls.is_empty() {
        return semio_framework_plugin::world3d_meshes_json_from_urls(&urls);
    }
    let meshes: Vec<Value> = objects
        .iter()
        .filter(|object| object.visible)
        .map(|object| {
            let data = object_mesh_data(object, geometry);
            json!({ "id": object.id, "data": data })
        })
        .collect();
    if meshes.is_empty() {
        let data = mesh_from_kind(CAD_FALLBACK_MESH_KIND);
        return serde_json::to_string(&[json!({ "id": CAD_FALLBACK_MESH_KIND, "data": data })])
            .unwrap_or_else(|_| "[]".into());
    }
    serde_json::to_string(&meshes).unwrap_or_else(|_| "[]".into())
}

fn world_selection_json(
    document: &CadScene,
    runtime: &CadPlayRuntime,
    active_utility: Option<&str>,
    options: CadDislocateOptions,
) -> String {
    let mut value: Value = serde_json::from_str(&world3d_selection_json(
        &runtime.selection_method,
        runtime.selected_object_ids.as_slice(),
        runtime.hovered_object_id.as_deref(),
    ))
    .unwrap_or_else(|_| json!({}));
    if let Some(object) = value.as_object_mut() {
        let active = gumball_active(runtime, active_utility, options);
        if active_utility == Some(CAD_DISLOCATE_UTILITY_ID) {
            object.insert("transformMode".into(), json!("transform"));
            object.insert(
                "gumballConfig".into(),
                json!({
                    "moveAxes": options.move_enabled,
                    "movePlanes": options.move_enabled,
                    "rotate": options.rotate_enabled,
                    "scaleAxes": false,
                    "scalePlanes": false,
                    "scaleUniform": false,
                }),
            );
        }
        object.insert("gumballActive".into(), json!(active));
        object.insert(
            "engagementSessionActive".into(),
            json!(runtime.engagement_session.is_some()),
        );
        object.insert("showEdges".into(), json!(true));
        object.insert("selectionMode".into(), json!(runtime.component_selection.mode));
        object.insert("granularity".into(), json!(runtime.component_selection.mode));
        object.insert("targets".into(), json!(runtime.component_selection.targets));
        object.insert("componentIds".into(), json!(runtime.component_selection.ids));
        if let Some(active) = resolve_active_object_id(runtime) {
            object.insert("activeObjectId".into(), json!(active));
        }
        if let Some(target) = runtime.hovered_target.as_ref() {
            object.insert("hoveredComponent".into(), json!(target));
        }
        if let Some(reference_id) = runtime.selected_reference_id.as_deref() {
            object.insert("referenceSelectedId".into(), json!(reference_id));
        }
        if active {
            if let Some(target) = gumball_target_for(document, runtime.selected_object_ids.as_slice()) {
            object.insert("gumballTarget".into(), json!(target));
            }
        }
    }
    value.to_string()
}

fn world_references_json(document: &CadScene, pane: CadPaneId) -> Option<String> {
    let references = document
        .references_by_model_definition_id
        .get(pane.model_definition_id())?;
    if references.is_empty() {
        return None;
    }
    let records: Vec<Value> = references
        .iter()
        .filter(|reference| !reference.hidden)
        .map(|reference| {
            json!({
                "id": reference.id,
                "url": reference.source_url,
                "origin": reference.origin,
                "widthWorld": if reference.width_world > 0.0 { reference.width_world } else { 1.0 },
                "locked": reference.locked,
                "hidden": reference.hidden,
                "opacity": reference.opacity.unwrap_or(1.0),
            })
        })
        .collect();
    Some(serde_json::to_string(&records).unwrap_or_else(|_| "[]".into()))
}

fn build_world_scene_for_pane(
    envelope: &CadPlayView,
    pane: CadPaneId,
    surface_id: &str,
    active_utility: Option<&str>,
    options: CadDislocateOptions,
) -> UiNode {
    let objects = cad_pane_objects(&envelope.document, pane);
    let preview = envelope
        .runtime
        .engagement_session
        .as_ref()
        .filter(|session| session.pane == pane)
        .map(preview_display_items)
        .filter(|items| !items.is_empty())
        .map(|items| serde_json::to_string(&items).unwrap_or_else(|_| "[]".into()));
    build_world_3d_scene(
        surface_id,
        CAD_PLAY_APP_ID,
        world3d_scene_extended(
            camera_json(cad_pane_camera_runtime(&envelope.runtime, pane)),
            world_meshes_json(objects, cad_pane_geometry(&envelope.document, pane)),
            world_instances_json(objects, &envelope.runtime),
            world_selection_json(&envelope.document, &envelope.runtime, active_utility, options),
            None,
            None,
            None,
            world_references_json(&envelope.document, pane),
            None,
            None,
            preview,
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

/// 🗣️ Complete UI label set for the CAD app; one field per label makes every terminology×locale combination compile-checked.
//#endregion 🔖️Document

//#region 🔖️Terminology
struct CadLabels {
    // entity nouns — remapped under the "reuse" terminology
    object: &'static str,
    objects: &'static str,
    primitive: &'static str,
    // model-definition pane / document-tree section names
    pane_shape: &'static str,
    pane_building: &'static str,
    pane_energy: &'static str,
    pane_structure_classic: &'static str,
    references: &'static str,
    nodes: &'static str,
    // catalogue
    typologies: &'static str,
    typology_box: &'static str,
    typology_slab: &'static str,
    typology_column: &'static str,
    typology_beam: &'static str,
    typology_wall: &'static str,
    typology_external_wall: &'static str,
    // inspector group titles
    reference: &'static str,
    node: &'static str,
    // tree item actions
    hide: &'static str,
    show: &'static str,
    lock: &'static str,
    unlock: &'static str,
    duplicate: &'static str,
    delete: &'static str,
    // inspector field chrome
    label: &'static str,
    typology: &'static str,
    hidden: &'static str,
    locked: &'static str,
    position: &'static str,
    scale: &'static str,
    rotation: &'static str,
    slot: &'static str,
    kind: &'static str,
    id: &'static str,
    source: &'static str,
    width_world: &'static str,
    // catalogue / tree chrome
    none_placeholder: &'static str,
    // properties fallback + engagement chrome
    schema: &'static str,
    utility: &'static str,
    action_placeholder: &'static str,
    ok: &'static str,
    selected: &'static str,
    step: &'static str,
}

const CAD_LABELS_NATIVE_EN: CadLabels = CadLabels {
    object: "Object",
    objects: "Objects",
    primitive: "Primitive",
    pane_shape: "Shape",
    pane_building: "Building",
    pane_energy: "Energy",
    pane_structure_classic: "Structure Classic",
    references: "References",
    nodes: "Nodes",
    typologies: "Typologies",
    typology_box: "Box",
    typology_slab: "Slab",
    typology_column: "Column",
    typology_beam: "Beam",
    typology_wall: "Wall",
    typology_external_wall: "External Wall",
    reference: "Reference",
    node: "Node",
    hide: "Hide",
    show: "Show",
    lock: "Lock",
    unlock: "Unlock",
    duplicate: "Duplicate",
    delete: "Delete",
    label: "Label",
    typology: "Typology",
    hidden: "Hidden",
    locked: "Locked",
    position: "Position",
    scale: "Scale",
    rotation: "Rotation",
    slot: "Slot",
    kind: "Kind",
    id: "Id",
    source: "Source",
    width_world: "Width (world)",
    none_placeholder: "(none)",
    schema: "Schema",
    utility: "Utility",
    action_placeholder: "Action",
    ok: "OK",
    selected: "selected",
    step: "Step",
};

const CAD_LABELS_NATIVE_DE: CadLabels = CadLabels {
    object: "Objekt",
    objects: "Objekte",
    primitive: "Grundkörper",
    pane_shape: "Form",
    pane_building: "Gebäude",
    pane_energy: "Energie",
    pane_structure_classic: "Tragwerk Klassisch",
    references: "Referenzen",
    nodes: "Knoten",
    typologies: "Typologien",
    typology_box: "Quader",
    typology_slab: "Platte",
    typology_column: "Stütze",
    typology_beam: "Träger",
    typology_wall: "Wand",
    typology_external_wall: "Außenwand",
    reference: "Referenz",
    node: "Knoten",
    hide: "Ausblenden",
    show: "Anzeigen",
    lock: "Sperren",
    unlock: "Entsperren",
    duplicate: "Duplizieren",
    delete: "Löschen",
    label: "Bezeichnung",
    typology: "Typologie",
    hidden: "Ausgeblendet",
    locked: "Gesperrt",
    position: "Position",
    scale: "Skalierung",
    rotation: "Drehung",
    slot: "Platz",
    kind: "Art",
    id: "Id",
    source: "Quelle",
    width_world: "Breite (Weltkoordinaten)",
    none_placeholder: "(keine)",
    schema: "Schema",
    utility: "Werkzeug",
    action_placeholder: "Aktion",
    ok: "OK",
    selected: "ausgewählt",
    step: "Schritt",
};

const CAD_LABELS_REUSE_EN: CadLabels = CadLabels {
    object: "Building component",
    objects: "Building components",
    primitive: "Component part",
    ..CAD_LABELS_NATIVE_EN
};

const CAD_LABELS_REUSE_DE: CadLabels = CadLabels {
    object: "Baukomponente",
    objects: "Baukomponenten",
    primitive: "Bauteil",
    ..CAD_LABELS_NATIVE_DE
};

/// 🗣️ Resolves the active label set from the config-carried locale/terminology (was shell-provided
/// `ViewState`, deleted by B1); unknown terminology ids fall back to native. Two-dimensional (locale ×
/// terminology) label selection doesn't fit the SDK's `LocaleLabels`/`app_labels!` (locale-only, one
/// struct type per resolution), so this stays hand-rolled — reusing `cad_is_de_locale` for the locale
/// half instead of duplicating its `starts_with("de")` check.
fn cad_labels(cfg: &CadConfig) -> &'static CadLabels {
    let terminology = if cfg.terminology.is_empty() { "native" } else { cfg.terminology.as_str() };
    let is_de = cad_is_de_locale(cfg);
    match (terminology, is_de) {
        ("reuse", true) => &CAD_LABELS_REUSE_DE,
        ("reuse", false) => &CAD_LABELS_REUSE_EN,
        (_, true) => &CAD_LABELS_NATIVE_DE,
        (_, false) => &CAD_LABELS_NATIVE_EN,
    }
}

/// 🗣️ Resolves a typology catalog entry's display label from its stable id; unknown ids fall back to the catalog's native English text or the raw id.
fn typology_label<'a>(typology: &'a str, labels: &CadLabels) -> &'a str {
    match typology {
        "spatial.shape.primitive.box" => labels.typology_box,
        "building.building.slab" | "structure.structure.onewayreinforcedconcreteslab" => labels.typology_slab,
        "building.building.column" | "structure.structure.reinforcedconcretecolumn" => labels.typology_column,
        "building.building.beam" => labels.typology_beam,
        "building.building.wall" => labels.typology_wall,
        "energy.energy.externalwall" => labels.typology_external_wall,
        other => TYPOLOGY_CATALOG.iter().find(|entry| entry.typology == other).map(|entry| entry.label).unwrap_or(other),
    }
}

/// 🗣️ (action id) -> localized label for every operation/view-action/shell-action declared in `create_cad_app`'s
/// static manifest — the manifest itself has no `view_state`/locale parameter, so this overlay is how the command
/// palette and Actions rail get a translated label without threading locale through the whole builder chain.
fn cad_action_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(is_de, &[
        ("addObject", "Add Object", "Objekt hinzufügen"),
        ("patchObject", "Patch Object", "Objekt aktualisieren"),
        ("patchSelection", "Patch Selection", "Auswahl aktualisieren"),
        ("deleteObject", "Delete Object", "Objekt löschen"),
        ("duplicateObject", "Duplicate Object", "Objekt duplizieren"),
        ("addNode", "Add Node", "Knoten hinzufügen"),
        ("renameNode", "Rename Node", "Knoten umbenennen"),
        ("translateSelection", "Translate Selection", "Auswahl verschieben"),
        ("rotateSelection", "Rotate Selection", "Auswahl drehen"),
        ("scaleSelection", "Scale Selection", "Auswahl skalieren"),
        ("applyTransformation", "Apply Transformation", "Transformation anwenden"),
        ("importCadFile", "Import CAD File", "CAD-Datei importieren"),
        ("patchCadPlayReference", "Patch Reference", "Referenz aktualisieren"),
        ("engagementSubmit", "Engagement Submit", "Eingabe bestätigen"),
        ("setCamera", "Set Camera", "Kamera festlegen"),
        ("setProjection", "Set Projection", "Projektion festlegen"),
        ("setProjectionParam", "Set Projection Parameter", "Projektionsparameter festlegen"),
        ("focusModelDefinition", "Focus Model Definition", "Modelldefinition fokussieren"),
        ("setActiveExample", "Set Active Example", "Aktives Beispiel festlegen"),
        ("setSelection", "Set Selection", "Auswahl festlegen"),
        ("setNodeSelection", "Set Node Selection", "Knotenauswahl festlegen"),
        ("worldSelect", "World Select", "Welt auswählen"),
        ("worldHover", "World Hover", "Überfahren (Welt)"),
        ("setHover", "Set Hover", "Überfahren festlegen"),
        ("worldPick", "World Pick", "Punkt in der Welt wählen"),
        ("setSelectionMethod", "Set Selection Method", "Auswahlmethode festlegen"),
        ("setReferenceSelection", "Set Reference Selection", "Referenzauswahl festlegen"),
        ("referenceHover", "Reference Hover", "Überfahren (Referenz)"),
        ("engagementInput", "Engagement Input", "Eingabe"),
        ("engagementPossibleSelect", "Engagement Possible Select", "Eingabeoption auswählen"),
        ("engagementRepeatLast", "Engagement Repeat Last", "Letzte Eingabe wiederholen"),
        ("engagementAbort", "Engagement Abort", "Eingabe abbrechen"),
        ("worldPointerDown", "World Pointer Down", "Welt-Zeiger gedrückt"),
        ("worldPointerMove", "World Pointer Move", "Welt-Zeiger bewegt"),
        ("engagementPointerDown", "Engagement Pointer Down", "Eingabe-Zeiger gedrückt"),
        ("setPrimitiveSelection", "Set Primitive Selection", "Grundkörperauswahl festlegen"),
        ("toggleSun", "Toggle Sun", "Sonne umschalten"),
        ("setSunAzimuth", "Set Sun Azimuth", "Sonnenazimut festlegen"),
        ("setSunElevation", "Set Sun Elevation", "Sonnenhöhe festlegen"),
        ("setSunIntensity", "Set Sun Intensity", "Sonnenintensität festlegen"),
        ("setDislocateOption", "Set Dislocate Option", "Versetzen-Option festlegen"),
        ("saveSelected", "Save Selected", "Auswahl speichern"),
        ("saveInPlay", "Save In Play", "Im Play speichern"),
        ("saveCurrent", "Save Current", "Aktuelles speichern"),
        ("loadRawRequest", "Load Raw Request", "Rohdaten laden"),
    ])
}

/// 🗣️ (utility id) -> localized utility bar button label, for every `.utility(...)` declared in `create_cad_app`.
fn cad_utility_labels(is_de: bool) -> HashMap<String, String> {
    localized_label_map(is_de, &[
        (CAD_DISLOCATE_UTILITY_ID, "Dislocate", "Versetzen"),
    ])
}

//#endregion 🔖️Terminology

//#region 🔖️Panels
/// 🎛️ Move and Rotate handle groups shown only while this window owns the Dislocate utility.
fn cad_dislocate_utility_options(options: CadDislocateOptions, is_de: bool) -> WindowMeasure {
    WindowMeasure::Group {
        id: "cad-play-utility-options-dislocate".into(),
        label: String::new(),
        default_open: Some(true),
        active_utility_id: Some(CAD_DISLOCATE_UTILITY_ID.into()),
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
                id: "cad-dislocate-move".into(),
                icon_id: "move-3d".into(),
                label: Some(if is_de { "Verschieben" } else { "Move" }.into()),
                pressed: options.move_enabled,
                text: None,
                on_change: cad_action("setDislocateOption", Some(json!({ "option": "move" }))),
            },
            WindowMeasure::Toggle {
                id: "cad-dislocate-rotate".into(),
                icon_id: "rotate-cw".into(),
                label: Some(if is_de { "Drehen" } else { "Rotate" }.into()),
                pressed: options.rotate_enabled,
                text: None,
                on_change: cad_action("setDislocateOption", Some(json!({ "option": "rotate" }))),
            },
        ],
    }
}

fn object_tree_item(id_suffix: &str, object: &CadObject, labels: &CadLabels) -> UiTreeItemNode {
    let primitive_items: Vec<UiTreeItemNode> = object
        .primitives
        .iter()
        .map(|primitive| {
            let mut item = cad_tree_item(
                format!("cad-primitive:{id_suffix}:{}:{}", object.id, primitive.primitive_id),
                format!("{}: {}", primitive.slot, primitive.primitive_id),
                Some("hexagon"),
                cad_action(
                    "setPrimitiveSelection",
                    Some(json!({
                        "objectId": object.id,
                        "primitiveId": primitive.primitive_id,
                        "kind": primitive.kind,
                    })),
                ),
            );
            item.hover_action = Some(cad_action("worldHover", Some(json!({ "id": object.id }))));
            item.unhover_action = Some(cad_action("worldHover", None));
            item
        })
        .collect();
    let mut item = cad_tree_item(
        format!("cad-object:{id_suffix}:{}", object.id),
        object.label.clone(),
        Some("box"),
        cad_action("setSelection", Some(json!({ "objectIds": [object.id] }))),
    );
    if !object.typology.is_empty() {
        item.description = Some(typology_label(&object.typology, labels).to_string());
    }
    item.hover_action = Some(cad_action("worldHover", Some(json!({ "id": object.id }))));
    item.unhover_action = Some(cad_action("worldHover", None));
    item.dimmed = Some(!object.visible);
    item.draggable = Some(!object.locked);
    item.actions = Some(vec![
        UiTreeItemAction {
            icon_id: if object.visible { "eye-off" } else { "eye" }.into(),
            label: Some(if object.visible { labels.hide } else { labels.show }.into()),
            action: cad_action(
                "patchObject",
                Some(json!({ "objectId": object.id, "field": "hidden", "value": object.visible })),
            ),
            reveal_on_hover: Some(true),
        },
        UiTreeItemAction {
            icon_id: if object.locked { "unlock" } else { "lock" }.into(),
            label: Some(if object.locked { labels.unlock } else { labels.lock }.into()),
            action: cad_action(
                "patchObject",
                Some(json!({ "objectId": object.id, "field": "locked", "value": !object.locked })),
            ),
            reveal_on_hover: Some(true),
        },
        UiTreeItemAction {
            icon_id: "copy".into(),
            label: Some(labels.duplicate.into()),
            action: cad_action("duplicateObject", Some(json!({ "objectId": object.id }))),
            reveal_on_hover: Some(true),
        },
        UiTreeItemAction {
            icon_id: "trash-2".into(),
            label: Some(labels.delete.into()),
            action: cad_action("deleteObject", Some(json!({ "objectId": object.id }))),
            reveal_on_hover: Some(true),
        },
    ]);
    if !primitive_items.is_empty() {
        item.items = Some(primitive_items);
        item.default_open = Some(false);
    }
    item
}

fn reference_tree_item(model_definition_id: &str, reference: &CadReference, labels: &CadLabels) -> UiTreeItemNode {
    let mut item = cad_tree_item(
        format!("cad-reference:{model_definition_id}:{}", reference.id),
        reference.id.clone(),
        Some("image"),
        cad_action(
            "setReferenceSelection",
            Some(json!({ "modelDefinitionId": model_definition_id, "referenceId": reference.id })),
        ),
    );
    item.description = Some(reference.source_url.clone());
    item.hover_action = Some(cad_action(
        "referenceHover",
        Some(json!({ "modelDefinitionId": model_definition_id, "referenceId": reference.id })),
    ));
    item.unhover_action = Some(cad_action("referenceHover", None));
    item.dimmed = Some(reference.hidden);
    item.actions = Some(vec![
        UiTreeItemAction {
            icon_id: if reference.hidden { "eye" } else { "eye-off" }.into(),
            label: Some(if reference.hidden { labels.show } else { labels.hide }.into()),
            action: cad_action(
                "patchCadPlayReference",
                Some(json!({
                    "modelDefinitionId": model_definition_id,
                    "referenceId": reference.id,
                    "field": "hidden",
                    "value": !reference.hidden,
                })),
            ),
            reveal_on_hover: Some(true),
        },
        UiTreeItemAction {
            icon_id: if reference.locked { "unlock" } else { "lock" }.into(),
            label: Some(if reference.locked { labels.unlock } else { labels.lock }.into()),
            action: cad_action(
                "patchCadPlayReference",
                Some(json!({
                    "modelDefinitionId": model_definition_id,
                    "referenceId": reference.id,
                    "field": "locked",
                    "value": !reference.locked,
                })),
            ),
            reveal_on_hover: Some(true),
        },
    ]);
    item
}

/// 🌳️ Cad's tree items carry an icon rather than the SDK `tree_item_with_action`'s description slot, so
/// this stays a thin app-specific wrapper — built on the SDK's bare `tree_item` rather than hand-rolling
/// the full `UiTreeItemNode` struct literal.
fn cad_tree_item(id: impl Into<String>, label: impl Into<String>, icon_id: Option<&str>, action: ActionDescriptor) -> UiTreeItemNode {
    let mut item = tree_item(id, label);
    item.icon_id = icon_id.and_then(IconName::from_str);
    item.action = Some(action);
    item
}

/// 🗂️ The `document.references_by_model_definition_id` lookup repeated once per pane in `build_document_tree`.
fn references_for<'a>(document: &'a CadScene, model_definition_id: &str) -> &'a [CadReference] {
    document
        .references_by_model_definition_id
        .get(model_definition_id)
        .map(|rows| rows.as_slice())
        .unwrap_or(&[])
}

fn document_tree_selected_ids(document: &CadScene, runtime: &CadPlayRuntime) -> Option<Vec<String>> {
    if let (Some(model_definition_id), Some(reference_id)) = (
        runtime.selected_reference_model_definition_id.as_deref(),
        runtime.selected_reference_id.as_deref(),
    ) {
        return Some(vec![format!("cad-reference:{model_definition_id}:{reference_id}")]);
    }
    if let (Some(object_id), Some(primitive_id)) = (
        runtime.selected_object_ids.first(),
        runtime.selected_primitive_id.as_deref(),
    ) {
        if let Some(pane) = cad_find_object_pane(document, object_id) {
            return Some(vec![format!(
                "cad-primitive:{}:{object_id}:{primitive_id}",
                cad_pane_suffix(pane)
            )]);
        }
    }
    let selected: Vec<String> = runtime
        .selected_object_ids
        .iter()
        .filter_map(|object_id| {
            cad_find_object_pane(document, object_id)
                .map(|pane| format!("cad-object:{}:{object_id}", cad_pane_suffix(pane)))
        })
        .collect();
    if selected.is_empty() {
        None
    } else {
        Some(selected)
    }
}

fn document_tree_highlighted_ids(document: &CadScene, runtime: &CadPlayRuntime) -> Option<Vec<String>> {
    let hovered = runtime.hovered_object_id.as_deref()?;
    if let Some(reference_id) = hovered.strip_prefix("reference:") {
        for pane in CadPaneId::all() {
            let model_definition_id = pane.model_definition_id();
            if document
                .references_by_model_definition_id
                .get(model_definition_id)
                .is_some_and(|rows| rows.iter().any(|row| row.id == reference_id))
            {
                return Some(vec![format!("cad-reference:{model_definition_id}:{reference_id}")]);
            }
        }
        return None;
    }
    cad_find_object_pane(document, hovered).map(|pane| {
        vec![format!("cad-object:{}:{hovered}", cad_pane_suffix(pane))]
    })
}

/// 🌳️ One pane's object section: namespaced by `id_suffix`, always expanded.
fn document_pane_section(label: &str, id_suffix: &str, objects: &[CadObject], labels: &CadLabels) -> (String, Option<String>, bool, Vec<UiTreeItemNode>) {
    (
        format!("cad-play-document.{id_suffix}"),
        Some(label.into()),
        true,
        objects.iter().map(|object| object_tree_item(id_suffix, object, labels)).collect(),
    )
}

/// 🌳️ One pane's references section: collapsed by default, "(none)"-placeholder when empty.
fn document_references_section(document: &CadScene, model_definition_id: &str, labels: &CadLabels) -> (String, Option<String>, bool, Vec<UiTreeItemNode>) {
    (
        format!("cad-play-document.references.{model_definition_id}"),
        Some(labels.references.into()),
        false,
        references_for(document, model_definition_id)
            .iter()
            .map(|reference| reference_tree_item(model_definition_id, reference, labels))
            .collect(),
    )
}

fn build_document_tree(envelope: &CadPlayView, labels: &CadLabels) -> UiNode {
    let node_items: Vec<UiTreeItemNode> = envelope
        .document
        .nodes
        .iter()
        .map(|node| {
            cad_tree_item(
                format!("cad-node:{}", node.id),
                node.label.clone(),
                Some("git-branch"),
                cad_action("setNodeSelection", Some(json!({ "nodeIds": [node.id] }))),
            )
        })
        .collect();

    let (shape_id, shape_label, shape_open, shape_items) = document_pane_section(labels.pane_shape, "shape", &envelope.document.objects, labels);
    let (shape_refs_id, shape_refs_label, shape_refs_open, shape_refs_items) = document_references_section(&envelope.document, CAD_MODEL_DEFINITION_SHAPE, labels);
    let (building_id, building_label, building_open, building_items) = document_pane_section(labels.pane_building, "building", &envelope.document.building_objects, labels);
    let (building_refs_id, building_refs_label, building_refs_open, building_refs_items) = document_references_section(&envelope.document, CAD_MODEL_DEFINITION_BUILDING, labels);
    let (energy_id, energy_label, energy_open, energy_items) = document_pane_section(labels.pane_energy, "energy", &envelope.document.energy_objects, labels);
    let (energy_refs_id, energy_refs_label, energy_refs_open, energy_refs_items) = document_references_section(&envelope.document, CAD_MODEL_DEFINITION_ENERGY, labels);
    let (structure_id, structure_label, structure_open, structure_items) = document_pane_section(
        labels.pane_structure_classic,
        "structure-classic",
        &envelope.document.structure_classic_objects,
        labels,
    );
    let (structure_refs_id, structure_refs_label, structure_refs_open, structure_refs_items) = document_references_section(&envelope.document, CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC, labels);

    let mut builder = PanelTreeBuilder::new("cad-play-document")
        .section(shape_id, shape_label, shape_open, shape_items)
        .section_or_placeholder(shape_refs_id, shape_refs_label, shape_refs_open, shape_refs_items, labels.none_placeholder)
        .section(building_id, building_label, building_open, building_items)
        .section_or_placeholder(building_refs_id, building_refs_label, building_refs_open, building_refs_items, labels.none_placeholder)
        .section(energy_id, energy_label, energy_open, energy_items)
        .section_or_placeholder(energy_refs_id, energy_refs_label, energy_refs_open, energy_refs_items, labels.none_placeholder)
        .section(structure_id, structure_label, structure_open, structure_items)
        .section_or_placeholder(structure_refs_id, structure_refs_label, structure_refs_open, structure_refs_items, labels.none_placeholder)
        .section("cad-play-document.nodes", Some(labels.nodes.into()), true, node_items);
    if let Some(ids) = document_tree_selected_ids(&envelope.document, &envelope.runtime) {
        builder = builder.selected(ids);
    }
    if let Some(ids) = document_tree_highlighted_ids(&envelope.document, &envelope.runtime) {
        builder = builder.highlighted(ids);
    }
    builder.build()
}

fn build_catalogue_tree(labels: &CadLabels) -> UiNode {
    let items: Vec<UiTreeItemNode> = TYPOLOGY_CATALOG
        .iter()
        .map(|entry| {
            cad_tree_item(
                format!("cad-play-catalogue.{}", entry.typology),
                typology_label(entry.typology, labels),
                Some(entry.icon),
                cad_action("addObject", Some(json!({ "typology": entry.typology, "modelDefinitionId": entry.model_definition_id }))),
            )
        })
        .collect();
    PanelTreeBuilder::new("cad-play-catalogue")
        .section("cad-play-catalogue.typologies", Some(labels.typologies.into()), true, items)
        .build()
}

fn build_properties_panel(envelope: &CadPlayView, labels: &CadLabels, active_utility: Option<&str>) -> UiNode {
    if let (Some(object_id), Some(primitive_id)) = (
        envelope.runtime.selected_object_ids.first(),
        envelope.runtime.selected_primitive_id.as_deref(),
    ) {
        if let Some((object, _)) = cad_all_objects(&envelope.document).find(|(object, _)| object.id == *object_id) {
            let kind = envelope
                .runtime
                .selected_primitive_kind
                .as_deref()
                .or_else(|| {
                    object
                        .primitives
                        .iter()
                        .find(|primitive| primitive.primitive_id == primitive_id)
                        .map(|primitive| primitive.kind.as_str())
                })
                .unwrap_or("primitive");
            return ui_inspector_groups_to_tree(&[primitive_inspector_group(
                object,
                labels,
                primitive_id,
                kind,
            )]);
        }
    }
    if !envelope.runtime.selected_object_ids.is_empty() {
        let selected: Vec<&CadObject> = envelope
            .runtime
            .selected_object_ids
            .iter()
            .filter_map(|id| {
                cad_all_objects(&envelope.document)
                    .find(|(object, _)| &object.id == id)
                    .map(|(object, _)| object)
            })
            .collect();
        if !selected.is_empty() {
            return ui_inspector_groups_to_tree(&[object_inspector_group(&selected, labels)]);
        }
    }
    if let (Some(model_definition_id), Some(reference_id)) = (
        envelope.runtime.selected_reference_model_definition_id.as_deref(),
        envelope.runtime.selected_reference_id.as_deref(),
    ) {
        if let Some(reference) = envelope
            .document
            .references_by_model_definition_id
            .get(model_definition_id)
            .and_then(|rows| rows.iter().find(|row| row.id == reference_id))
        {
            return ui_inspector_groups_to_tree(&[reference_inspector_group(
                model_definition_id,
                reference,
                labels,
            )]);
        }
    }
    if let Some(node_id) = envelope.runtime.selected_node_ids.first() {
        if let Some(node) = envelope.document.nodes.iter().find(|entry| &entry.id == node_id) {
            return ui_inspector_groups_to_tree(&[node_inspector_group(node, labels)]);
        }
    }
    ui_stack_vertical(vec![
        ui_text(format!("{}: {}", labels.schema, envelope.document.schema)),
        ui_text(format!("{}: {}", labels.utility, active_utility.unwrap_or(labels.none_placeholder))),
        ui_text(format!("{}: {}", labels.objects, envelope.document.objects.len())),
    ])
}


/// @emoji 🌀️ Builds an editable 4-component quaternion group (`X`/`Y`/`Z`/`W` steppers) — orientation
/// fields have no shared helper (quaternions aren't `ui_inspector_vec3_group`'s 3-wide shape), so
/// this mirrors that helper's structure one component wider. The patch handler renormalizes after
/// any component edit so the result stays a valid unit quaternion.
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

fn object_inspector_group(objects: &[&CadObject], term_labels: &CadLabels) -> UiInspectorFieldGroup {
    let object_ids: Vec<String> = objects.iter().map(|object| object.id.clone()).collect();
    let labels: Vec<String> = objects.iter().map(|object| object.label.clone()).collect();
    let typologies: Vec<String> = objects.iter().map(|object| object.typology.clone()).collect();
    let hidden: Vec<bool> = objects.iter().map(|object| !object.visible).collect();
    let locked: Vec<bool> = objects.iter().map(|object| object.locked).collect();
    let origins: Vec<[f64; 3]> = objects.iter().map(|object| object.origin).collect();
    let scales: Vec<[f64; 3]> = objects
        .iter()
        .map(|object| object.scale.unwrap_or([1.0, 1.0, 1.0]))
        .collect();
    let orientations: Vec<[f64; 4]> = objects
        .iter()
        .map(|object| object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]))
        .collect();
    let label_mixed = ui_inspector_mixed_text(&labels);
    let typology_mixed = ui_inspector_mixed_text(&typologies);
    let hidden_mixed = ui_inspector_mixed_toggle(&hidden);
    let locked_mixed = ui_inspector_mixed_toggle(&locked);
    UiInspectorFieldGroup {
        id: "cad-play-inspector.object".into(),
        label: if objects.len() == 1 {
            term_labels.object.into()
        } else {
            format!("{} {}", objects.len(), term_labels.objects)
        },
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.label".into(),
                label: term_labels.label.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    id: "cad-play-inspector.object.label.input".into(),
                    input_kind: "text".into(),
                    value: label_mixed.value.clone(),
                    placeholder: label_mixed.placeholder.clone(),
                    commit: None,
                    on_change: cad_action(
                        "patchSelection",
                        Some(json!({ "objectIds": object_ids, "field": "label" })),
                    ),
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
            }),
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.typology".into(),
                label: term_labels.typology.into(),
                child: Box::new(UiNode::Select(UiSelectNode {
                    id: "cad-play-inspector.object.typology.select".into(),
                    value: typology_mixed.value.clone(),
                    items: TYPOLOGY_CATALOG
                        .iter()
                        .map(|entry| UiSelectItem {
                            value: entry.typology.into(),
                            label: typology_label(entry.typology, term_labels).into(),
                        })
                        .collect(),
                    placeholder: typology_mixed.placeholder.clone(),
                    on_change: cad_action(
                        "patchSelection",
                        Some(json!({ "objectIds": object_ids, "field": "typology" })),
                    ),
                    presence: UiPresence::default(),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                presence: UiPresence::default(),
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.hidden".into(),
                label: term_labels.hidden.into(),
                child: Box::new(UiNode::Toggle(semio_framework_plugin::UiToggleNode {
                    id: "cad-play-inspector.object.hidden.toggle".into(),
                    icon_id: "eye-off".into(),
                    text: None,
                    on_change: cad_action(
                        "patchSelection",
                        Some(json!({ "objectIds": object_ids, "field": "hidden" })),
                    ),
                    presence: UiPresence::selected(hidden_mixed.pressed),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                presence: UiPresence::default(),
                menu: None,
            }),
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.object.locked".into(),
                label: term_labels.locked.into(),
                child: Box::new(UiNode::Toggle(semio_framework_plugin::UiToggleNode {
                    id: "cad-play-inspector.object.locked.toggle".into(),
                    icon_id: "lock".into(),
                    text: None,
                    on_change: cad_action(
                        "patchSelection",
                        Some(json!({ "objectIds": object_ids, "field": "locked" })),
                    ),
                    presence: UiPresence::selected(locked_mixed.pressed),
                    menu: None,
                })),
                description: None,
                required: None,
                error: None,
                presence: UiPresence::default(),
                menu: None,
            }),
            {
                let object_ids = object_ids.clone();
                ui_inspector_vec3_group("cad-play-inspector.object.origin", term_labels.position, &origins, 0.1, move |axis| {
                    cad_action("patchSelection", Some(json!({ "objectIds": object_ids, "field": format!("origin.{axis}") })))
                })
            },
            {
                let object_ids = object_ids.clone();
                ui_inspector_vec3_group("cad-play-inspector.object.scale", term_labels.scale, &scales, 0.1, move |axis| {
                    cad_action("patchSelection", Some(json!({ "objectIds": object_ids, "field": format!("scale.{axis}") })))
                })
            },
            inspector_quat_group("cad-play-inspector.object.orientation", term_labels.rotation, &orientations, 0.01, |axis| {
                cad_action("patchSelection", Some(json!({ "objectIds": object_ids, "field": format!("orientation.{axis}") })))
            }),
        ],
    }
}

fn primitive_inspector_group(object: &CadObject, labels: &CadLabels, primitive_id: &str, kind: &str) -> UiInspectorFieldGroup {
    let slot = object
        .primitives
        .iter()
        .find(|primitive| primitive.primitive_id == primitive_id)
        .map(|primitive| primitive.slot.as_str())
        .unwrap_or("primitive");
    UiInspectorFieldGroup {
        id: "cad-play-inspector.primitive".into(),
        label: labels.primitive.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            ui_inspector_readonly_field("cad-play-inspector.primitive.object", labels.object, &object.label),
            ui_inspector_readonly_field("cad-play-inspector.primitive.slot", labels.slot, slot),
            ui_inspector_readonly_field("cad-play-inspector.primitive.kind", labels.kind, kind),
            ui_inspector_readonly_field("cad-play-inspector.primitive.id", labels.id, primitive_id),
        ],
    }
}

fn reference_inspector_group(model_definition_id: &str, reference: &CadReference, labels: &CadLabels) -> UiInspectorFieldGroup {
    UiInspectorFieldGroup {
        id: "cad-play-inspector.reference".into(),
        label: labels.reference.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            ui_inspector_readonly_field("cad-play-inspector.reference.id", labels.id, &reference.id),
            ui_inspector_readonly_field(
                "cad-play-inspector.reference.source",
                labels.source,
                &reference.source_url,
            ),
            {
                let patch_cmd = |field: &str| {
                    cad_action(
                        "patchCadPlayReference",
                        Some(json!({ "modelDefinitionId": model_definition_id, "referenceId": reference.id, "field": field })),
                    )
                };
                ui_inspector_stepper_field("cad-play-inspector.reference.widthWorld", labels.width_world, &[reference.width_world], 0.1, patch_cmd("widthWorld"))
            },
            {
                let patch_cmd = move |axis: &str| {
                    cad_action(
                        "patchCadPlayReference",
                        Some(json!({ "modelDefinitionId": model_definition_id, "referenceId": reference.id, "field": format!("origin.{axis}") })),
                    )
                };
                ui_inspector_vec3_group("cad-play-inspector.reference.origin", labels.position, &[reference.origin], 0.1, patch_cmd)
            },
        ],
    }
}

fn node_inspector_group(node: &CadNode, labels: &CadLabels) -> UiInspectorFieldGroup {
    UiInspectorFieldGroup {
        id: "cad-play-inspector.node".into(),
        label: labels.node.into(),
        default_open: None,
        presence: UiPresence::default(),
        fields: vec![
            UiNode::Field(UiFieldNode {
                id: "cad-play-inspector.node.label".into(),
                label: labels.label.into(),
                child: Box::new(UiNode::Input(UiInputNode {
                    id: "cad-play-inspector.node.label.input".into(),
                    input_kind: "text".into(),
                    value: node.label.clone(),
                    placeholder: None,
                    commit: None,
                    on_change: cad_action(
                        "renameNode",
                        Some(json!({ "nodeId": node.id })),
                    ),
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
            }),
            ui_inspector_readonly_field("cad-play-inspector.node.kind", labels.kind, &node.kind),
        ],
    }
}

fn cad_window_engagement(envelope: &CadPlayView, pane: CadPaneId, labels: &CadLabels) -> WindowEngagement {
    let selected_count = envelope.runtime.selected_object_ids.len();
    let model_definition_id = pane.model_definition_id();
    let session_active = envelope.runtime.engagement_session.is_some();
    let possible_engagements: Vec<WindowEngagementPossible> =
        if let Some(session) = envelope.runtime.engagement_session.as_ref() {
            keyed_transitions(session)
                .into_iter()
                .map(|transition| WindowEngagementPossible {
                    id: transition.event_kind.clone(),
                    label: transition.label,
                    detail: Some(transition.key),
                    action: Some(cad_action(
                        "engagementPossibleSelect",
                        Some(json!({
                            "pane": cad_pane_suffix(pane),
                            "possibleId": transition.event_kind,
                        })),
                    )),
                })
                .collect()
        } else {
            list_interactions_for_model_definition(model_definition_id)
                .into_iter()
                .map(|entry| WindowEngagementPossible {
                    id: entry.id.clone(),
                    label: entry.label.clone(),
                    detail: Some(entry.key.clone()),
                    action: Some(cad_action(
                        "engagementPossibleSelect",
                        Some(json!({ "pane": cad_pane_suffix(pane), "possibleId": entry.id.clone() })),
                    )),
                })
                .collect()
        };
    let step_text = envelope
        .runtime
        .engagement_session
        .as_ref()
        .map(|session| session.state.clone())
        .unwrap_or_else(|| envelope.runtime.engagement_step.clone());
    WindowEngagement {
        session_active: Some(session_active),
        // 🧰️ The move/rotate/scale transform switcher now lives in the framework utility bar (derived
        // from `UtilityDefinition`s + `ViewState::active_utility_id`); the engagement HUD no longer
        // duplicates it — utilities must have exactly one surface.
        options: None,
        input: Some(WindowEngagementInput {
            id: Some("engagement-input".into()),
            value: Some(envelope.runtime.engagement_input.clone()),
            placeholder: Some(labels.action_placeholder.into()),
            disabled: None,
            on_change: Some(cad_action(
                "engagementInput",
                Some(json!({ "pane": cad_pane_suffix(pane) })),
            )),
            on_submit: Some(cad_action(
                "engagementSubmit",
                Some(json!({ "pane": cad_pane_suffix(pane) })),
            )),
            on_repeat_last: Some(cad_action(
                "engagementRepeatLast",
                Some(json!({ "pane": cad_pane_suffix(pane) })),
            )),
            on_abort: Some(cad_action(
                "engagementAbort",
                Some(json!({ "pane": cad_pane_suffix(pane) })),
            )),
        }),
        control: None,
        controls: None,
        status: Some(vec![
            WindowEngagementStatus {
                id: "cad-status".into(),
                text: format!("{selected_count} {}", labels.selected),
            },
            WindowEngagementStatus {
                id: "cad-step".into(),
                text: format!("{}: {step_text}", labels.step),
            },
            WindowEngagementStatus {
                id: "cad-response".into(),
                text: envelope
                    .runtime
                    .engagement_session
                    .as_ref()
                    .and_then(|session| session.last_response.clone())
                    .unwrap_or_else(|| labels.ok.into()),
            },
        ]),
        possible_engagements: Some(possible_engagements),
    }
}

//#endregion 🔖️Panels

//#region 🔖️ActionHelpers
fn object_patch_from_field(field: &str, value: Option<&Value>) -> Option<CadObjectPatch> {
    match field {
        "label" | "name" => value
            .and_then(|entry| entry.as_str())
            .map(|label| CadObjectPatch {
                label: Some(label.into()),
                ..Default::default()
            }),
        "typology" => value
            .and_then(|entry| entry.as_str())
            .map(|typology| CadObjectPatch {
                typology: Some(typology.into()),
                ..Default::default()
            }),
        "hidden" => value
            .and_then(|entry| entry.as_bool())
            .map(|hidden| CadObjectPatch {
                visible: Some(!hidden),
                ..Default::default()
            }),
        "locked" => value.and_then(|entry| entry.as_bool()).map(|locked| CadObjectPatch {
            locked: Some(locked),
            ..Default::default()
        }),
        _ => None,
    }
}

fn resolve_number_edit(current: f64, value: Option<&Value>, delta: Option<&Value>) -> Option<f64> {
    if let Some(absolute) = value.and_then(Value::as_f64) {
        return Some(absolute);
    }
    delta.and_then(Value::as_f64).map(|delta| current + delta)
}

fn axis3_index(field: &str, base: &str) -> Option<usize> {
    match field.strip_prefix(base)?.strip_prefix('.')? {
        "x" => Some(0),
        "y" => Some(1),
        "z" => Some(2),
        _ => None,
    }
}

fn axis4_index(field: &str, base: &str) -> Option<usize> {
    match field.strip_prefix(base)?.strip_prefix('.')? {
        "x" => Some(0),
        "y" => Some(1),
        "z" => Some(2),
        "w" => Some(3),
        _ => None,
    }
}

fn quat_normalize(q: [f64; 4]) -> [f64; 4] {
    let len = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    if len < 1e-9 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    [q[0] / len, q[1] / len, q[2] / len, q[3] / len]
}

/// @emoji 🩹️ Builds the `PatchObject` operations that apply `field`'s edit across `object_ids`: whole-value
/// fields (label/typology/hidden/locked) use the same patch for every object; `origin.<axis>`/`scale.<axis>`/
/// `orientation.<axis>` read each object's own current component so `value` (absolute) or `delta` (relative)
/// applies per-object, preserving each object's other axes and any offset across a multi-select.
fn patch_objects_operations(
    document: &CadScene,
    object_ids: &[String],
    field: &str,
    value: Option<&Value>,
    delta: Option<&Value>,
) -> Vec<CadOperation> {
    if let Some(patch) = object_patch_from_field(field, value) {
        return object_ids
            .iter()
            .filter_map(|object_id| cad_find_object_pane(document, object_id).map(|pane| CadOperation::PatchObject { pane, object_id: object_id.clone(), patch: patch.clone() }))
            .collect();
    }
    let mut operations = Vec::new();
    for object_id in object_ids {
        let Some((object, pane)) = cad_all_objects(document).find(|(object, _)| &object.id == object_id) else {
            continue;
        };
        let patch = if let Some(axis) = axis3_index(field, "origin") {
            let mut origin = object.origin;
            let Some(updated) = resolve_number_edit(origin[axis], value, delta) else { continue };
            origin[axis] = updated;
            CadObjectPatch { origin: Some(origin), ..Default::default() }
        } else if let Some(axis) = axis3_index(field, "scale") {
            let mut scale = object.scale.unwrap_or([1.0, 1.0, 1.0]);
            let Some(updated) = resolve_number_edit(scale[axis], value, delta) else { continue };
            scale[axis] = updated;
            CadObjectPatch { scale: Some(scale), ..Default::default() }
        } else if let Some(axis) = axis4_index(field, "orientation") {
            let mut orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
            let Some(updated) = resolve_number_edit(orientation[axis], value, delta) else { continue };
            orientation[axis] = updated;
            CadObjectPatch { orientation: Some(quat_normalize(orientation)), ..Default::default() }
        } else {
            continue;
        };
        operations.push(CadOperation::PatchObject { pane, object_id: object_id.clone(), patch });
    }
    operations
}

fn make_object_for_typology(typology: &str, label_count: usize, pane: CadPaneId) -> CadObject {
    let label = TYPOLOGY_CATALOG
        .iter()
        .find(|entry| entry.typology == typology)
        .map(|entry| entry.label)
        .unwrap_or("Object");
    let extent = match typology {
        t if t.contains("column") => Some([0.5, 0.5, 3.0]),
        t if t.contains("slab") => Some([4.0, 4.0, 0.25]),
        t if t.contains("wall") => Some([4.0, 0.2, 3.0]),
        _ => Some([1.0, 1.0, 1.0]),
    };
    let mut object = CadObject {
        id: next_cad_id("object"),
        label: format!("{label} {}", label_count + 1),
        typology: typology.into(),
        visible: true,
        locked: false,
        origin: [0.0, 0.0, 0.0],
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: None,
        mesh_url: None,
        extent,
        solid_handle: None,
        primitives: Vec::new(),
    };
    if let Ok(mut kernel) = cad_brep_kernel().lock() {
        ensure_object_solid_handle(&mut **kernel, &mut object);
    }
    let _ = pane;
    object
}

/// Commits `session` if it satisfies `can_commit`, returning the `AddObject` operation and clearing
/// the session runtime state. Returns the operations (empty when no commit happened) — used by both the
/// direct-event and keyed-transition REPL paths in `engagement_submit_operations` (a state reached via
/// either path can be commit-ready, e.g. box's explicit `confirm` step reachable via a keyed
/// transition).
fn try_commit_session_operations(document: &CadScene, runtime: &mut CadPlayRuntime, pane: CadPaneId, session: &CadEngagementSession) -> Vec<CadOperation> {
    if !can_commit(session) {
        return Vec::new();
    }
    let label_count = cad_pane_objects(document, pane).len();
    let Ok(mut kernel) = cad_brep_kernel().lock() else {
        return Vec::new();
    };
    let Some(object) = commit_object(&mut **kernel, session, label_count, |prefix| next_cad_id(prefix)) else {
        return Vec::new();
    };
    drop(kernel);
    let id = object.id.clone();
    let interaction_id = session.interaction_id.clone();
    runtime.selected_object_ids = SelectionSet::from(vec![id]);
    runtime.engagement_input.clear();
    runtime.last_finalized_interaction_id = Some(interaction_id);
    runtime.engagement_session = None;
    runtime.engagement_step = "Idle".into();
    vec![CadOperation::AddObject { pane, object }]
}

/// @emoji ⌨️ Advances the engagement REPL for the current `engagement_input`, mutating runtime
/// session state and returning any commit operations produced.
fn engagement_submit_operations(document: &CadScene, runtime: &mut CadPlayRuntime, pane: CadPaneId) -> Vec<CadOperation> {
    let input = runtime.engagement_input.trim().to_string();
    if input.is_empty() {
        runtime.engagement_step = "Idle".into();
        return Vec::new();
    }
    let model_definition_id = pane.model_definition_id();
    let current_state = runtime.engagement_session.as_ref().map(|session| session.state.clone());
    if let Some((event_kind, payload)) = parse_repl_line(&input, current_state.as_deref()) {
        // An active session's own events/keyed-transitions always take priority over starting an
        // unrelated interaction by key — otherwise a mid-flow keypress that happens to collide
        // with another interaction's top-level key (e.g. box's "d" for diagonal mode vs. length's
        // top-level key "d") would silently abandon the current session.
        if let Some(session) = runtime.engagement_session.as_mut() {
            if apply_event(session, &event_kind, payload.as_ref()) {
                runtime.engagement_step = session.state.clone();
                let session_snapshot = session.clone();
                return try_commit_session_operations(document, runtime, pane, &session_snapshot);
            }
            for transition in keyed_transitions(session) {
                if transition.key.eq_ignore_ascii_case(&input) || transition.event_kind.eq_ignore_ascii_case(&input) {
                    if apply_event(session, &transition.event_kind, None) {
                        runtime.engagement_step = session.state.clone();
                        runtime.engagement_input.clear();
                        let session_snapshot = session.clone();
                        return try_commit_session_operations(document, runtime, pane, &session_snapshot);
                    }
                }
            }
        } else if let Some(entry) = resolve_interaction_key(&event_kind, model_definition_id) {
            runtime.engagement_session = start_session(&entry.id, pane);
            if let Some(session) = runtime.engagement_session.as_mut() {
                let _ = apply_event(session, "start", None);
            }
            runtime.engagement_step = runtime
                .engagement_session
                .as_ref()
                .map(|session| session.state.clone())
                .unwrap_or_else(|| "Idle".into());
            runtime.engagement_input.clear();
            return Vec::new();
        }
    }
    runtime.engagement_step = format!("Unknown: {input}");
    Vec::new()
}

/// Starts a fresh engagement session for `interaction_id` in `pane` (used by
/// `engagementPossibleSelect`'s start-by-id path and `engagementRepeatLast`).
fn start_interaction_session(runtime: &mut CadPlayRuntime, pane: CadPaneId, interaction_id: &str) -> bool {
    let Some(entry) = interaction::interaction_by_id(interaction_id) else {
        return false;
    };
    runtime.engagement_session = start_session(&entry.id, pane);
    if let Some(session) = runtime.engagement_session.as_mut() {
        let _ = apply_event(session, "start", None);
    }
    runtime.engagement_step = runtime
        .engagement_session
        .as_ref()
        .map(|session| session.state.clone())
        .unwrap_or_else(|| "Idle".into());
    true
}

/// @emoji 🔀️ WORKFLOWS-END-TO-END-TYPED-PORTS: the typed-command counterpart of the pre-B1
/// `mesh_selection_ids` (JSON-args) helper — falls back to the current selection when the command
/// carries no explicit ids.
fn ids_or_selection(ids: &[String], fallback: &[String]) -> Vec<String> {
    if ids.is_empty() { fallback.to_vec() } else { ids.to_vec() }
}

/// @emoji 🪟️ Maps a pane to the window-KIND id whose Dislocate options it owns — the typed-command
/// counterpart of the pre-B1 `view_state.window_id` resolution (see `CadConfig::dislocate_shape`'s
/// doc comment in `cad_document_engine`).
fn cad_window_id_for_pane(pane: CadPaneId) -> &'static str {
    match pane {
        CadPaneId::Shape => CAD_PLAY_WINDOW_SHAPE,
        CadPaneId::Building => CAD_PLAY_WINDOW_BUILDING,
        CadPaneId::Energy => CAD_PLAY_WINDOW_ENERGY,
        CadPaneId::StructureClassic => CAD_PLAY_WINDOW_STRUCTURE_CLASSIC,
    }
}

/// @emoji 🩹️ Typed-command counterpart of a raw JSON patch value: `CadCommand::PatchObject`/
/// `PatchSelection`/`PatchCadPlayReference` all carry `value: Option<String>` (the typed channel has no
/// single Rust type spanning "maybe a string, maybe a number, maybe a bool") — this recovers the
/// `serde_json::Value` shape `object_patch_from_field`/`resolve_number_edit` already expect, dispatching
/// on the same field-name vocabulary those helpers use (bool fields by name, everything else tried as a
/// number first, falling back to a string).
fn command_value_json(field: &str, value: &str) -> Value {
    match field {
        "hidden" | "locked" => value.parse::<bool>().map(Value::Bool).unwrap_or(Value::Null),
        _ => value.parse::<f64>().map(|number| json!(number)).unwrap_or_else(|_| Value::String(value.into())),
    }
}

/// @emoji 📐️ The CAD play app. Document content lives in the wrapping `VcsDocumentApp`'s
/// `DocumentStore<CadScene, CadOperation>`; only ephemeral view-state (selection, hover, engagement
/// session, transform utility, sun) lives here on `runtime`. History (undo/redo/checkpoint) is
/// intercepted and dispatched by the wrapper — no manual arms or keybindings.
//#endregion 🔖️ActionHelpers

//#region 🔖️CadPlayApp
/// @emoji 📐️ B1/WORKFLOWS-END-TO-END-TYPED-PORTS: unit-struct-shaped pure `DocumentApp` — every former
/// `CadPlayRuntime`/`self.runtime` field now lives in `cad_document_engine::CadConfig`, written through
/// `cad_document_op::CadConfigOperation`s (real `backwards`, no ad hoc `InverseAction`). `preview_seq`
/// is the sole surviving interior-mutable field — it backs `gesture_preview`'s never-VCS'd, never-config'd
/// live rubber-band tick counter (see that method's doc comment), not app state.
#[derive(Default)]
pub struct CadPlayApp {
    /// 👻️ Per-`key` monotone counter for `gesture_preview` — see `//#region 🔖️GesturePreview`.
    preview_seq: std::cell::RefCell<u64>,
}

impl CadPlayApp {
    /// 👻️ CW7 db+protocol+vcs-slimming campaign, "preview law for gesture apps": the live rubber-band
    /// engagement session — `worldPointerMove`'s own doc already calls this out: "applies pointer.move
    /// ... without ever committing an object or touching VCS history" — shaped as the exact payload
    /// `framework_sync::SyncSession::publish_preview(key, seq, payload)` expects, ready to hand off the
    /// instant a transport exists. `None` outside an active engagement session; reads
    /// `CadEngagementSession` only, never `CadScene`/`CadOperation` — a preview can never become
    /// persistent state.
    ///
    /// 🚧️ Deliberately unwired beyond this accessor — same gap as `draw-plugin`'s
    /// `draw_gesture_preview_payload` (see that doc for the full explanation): `framework/sync::
    /// SyncSession::publish_preview` is host-only and unreachable from this WASI-P2 sandboxed plugin
    /// crate, and `store::BackboneMessage` has no preview-shaped variant to relay one through. See
    /// `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/cw7-preview-law.txt`.
    /// `#[allow(dead_code)]`: exercised by `🧪️Tests` only until a host bridge exists.
    #[allow(dead_code)]
    fn gesture_preview(&self, config: &CadConfig) -> Option<(&'static str, u64, Vec<u8>)> {
        let runtime = cad_runtime_from_config(config);
        let session = runtime.engagement_session?;
        Some(("gesture:engagement", *self.preview_seq.borrow(), serde_json::to_vec(&session).ok()?))
    }
}

impl DocumentApp for CadPlayApp {
    type Projection = CadScene;
    type Operation = CadOperation;
    type Config = CadConfig;
    type ConfigOperation = CadConfigOperation;
    type Command = CadCommand;

    fn app_id(&self) -> &str {
        CAD_PLAY_APP_ID
    }

    fn document_schema(&self) -> &str {
        CAD_DOCUMENT_SCHEMA
    }

    fn initial_projection(&self) -> CadScene {
        forest_play_scene()
    }

    fn io(&self) -> Option<semio_framework_plugin::AppIo> {
        Some(cad_io())
    }

    fn whole_document_operation(&self, projection: CadScene) -> Option<CadOperation> {
        Some(CadOperation::SetScene { scene: Box::new(projection) })
    }

    /// 🎞️ `geometry:in` (WORKFLOWS-END-TO-END-TYPED-PORTS port recipe): accepts incoming mesh/brep
    /// geometry from any upstream 3D producer and inserts it as a new `CadObject` in the Shape pane —
    /// reuses `cad_document_engine::geometry_import::cad_object_from_mesh`/`cad_object_from_solid_handle`
    /// (the same helpers `import_cad_object_by_extension` already builds on for file-drop imports),
    /// through the same brep kernel every other import path shares. Falls through to the default
    /// `document:in` importer for any other port.
    fn import_media(&self, port: &str, media: &Media, _doc: &DocumentView<'_, CadScene>) -> Result<Emit<CadOperation, CadConfigOperation>, MediaError> {
        if port != "geometry:in" {
            if port != "document:in" {
                return Err(MediaError::NotImplemented);
            }
            let MediaPayload::Structured { json, .. } = &media.payload else {
                return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
            };
            let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            let projection = <CadScene as store::DocumentPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            return match self.whole_document_operation(projection) {
                Some(operation) => Ok(Emit::operations(vec![operation])),
                None => Err(MediaError::NotImplemented),
            };
        }
        let name = match &media.media_type.form {
            MediaForm::Brep => "import.step",
            _ => "import.obj",
        };
        let payload = match &media.payload {
            MediaPayload::Structured { json, .. } => Value::String(json.clone()),
            MediaPayload::Binary { .. } => return Err(MediaError::Payload(port.to_string(), "geometry:in only accepts a Structured payload today".into())),
        };
        match import_cad_object_by_extension(name, &payload) {
            Some(object) => Ok(Emit::operations(vec![CadOperation::AddObject { pane: CadPaneId::Shape, object }])),
            None => Err(MediaError::Payload(port.to_string(), "unrecognized geometry payload".into())),
        }
    }

    /// 🎞️ `brep:out` (WORKFLOWS-END-TO-END-TYPED-PORTS port recipe): exports the cad document's current
    /// brep geometry (every pane's solids fused into one modelspace, same as `saveInPlay`'s STEP export)
    /// wrapped as `Media`. Falls through to the default whole-document `document:out` for any other port.
    fn export_media(&self, port: &str, doc: &DocumentView<'_, CadScene>) -> Result<Media, MediaError> {
        if port != "brep:out" {
            if port != "document:out" {
                return Err(MediaError::NotImplemented);
            }
            let media_type = self.io().map(|io| io.document_media_type).unwrap_or(MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep });
            let bytes = <CadScene as store::DocumentPack>::encode_pack(doc.projection);
            return Ok(Media { media_type, payload: MediaPayload::Structured { schema: self.document_schema().to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } });
        }
        let view = CadPlayView { document: doc.projection.clone(), runtime: CadPlayRuntime::default() };
        let Ok(mut kernel) = cad_brep_kernel().lock() else {
            return Err(MediaError::Payload(port.to_string(), "brep kernel unavailable".into()));
        };
        let solids = collect_modelspace_solids(&mut **kernel, &view);
        if solids.is_empty() {
            return Err(MediaError::Payload(port.to_string(), "no solids to export".into()));
        }
        let Some(export) = export_solids_as(&mut **kernel, &solids, OsMediaFormat::Step, "cad.modelspace") else {
            return Err(MediaError::Payload(port.to_string(), "brep export failed".into()));
        };
        let text = match export.data {
            Value::String(text) => text,
            other => other.to_string(),
        };
        Ok(Media {
            media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
            payload: MediaPayload::Structured { schema: "3d.cad".into(), json: base64::engine::general_purpose::STANDARD.encode(text.as_bytes()) },
        })
    }

    fn command_id(&self, command: &CadCommand) -> &str {
        match command {
            CadCommand::AddObject { .. } => "addObject",
            CadCommand::PatchObject { .. } => "patchObject",
            CadCommand::PatchSelection { .. } => "patchSelection",
            CadCommand::DeleteObject { .. } => "deleteObject",
            CadCommand::DuplicateObject { .. } => "duplicateObject",
            CadCommand::AddNode { .. } => "addNode",
            CadCommand::RenameNode { .. } => "renameNode",
            CadCommand::TranslateSelection { .. } => "translateSelection",
            CadCommand::RotateSelection { .. } => "rotateSelection",
            CadCommand::ScaleSelection { .. } => "scaleSelection",
            CadCommand::ApplyTransformation { .. } => "applyTransformation",
            CadCommand::ImportCadFile { .. } => "importCadFile",
            CadCommand::PatchCadPlayReference { .. } => "patchCadPlayReference",
            CadCommand::EngagementSubmit { .. } => "engagementSubmit",
            CadCommand::FocusModelDefinition { .. } => "focusModelDefinition",
            CadCommand::SetActiveExample { .. } => "setActiveExample",
            CadCommand::WorldPointerDown { .. } => "worldPointerDown",
            CadCommand::SetCamera { .. } => "setCamera",
            CadCommand::SetProjection { .. } => "setProjection",
            CadCommand::SetProjectionParam { .. } => "setProjectionParam",
            CadCommand::SetDislocateOption { .. } => "setDislocateOption",
            CadCommand::SetSelection { .. } => "setSelection",
            CadCommand::SetNodeSelection { .. } => "setNodeSelection",
            CadCommand::WorldSelect { .. } => "worldSelect",
            CadCommand::WorldHover { .. } => "worldHover",
            CadCommand::SetHover { .. } => "setHover",
            CadCommand::WorldPick { .. } => "worldPick",
            CadCommand::SetSelectionMethod { .. } => "setSelectionMethod",
            CadCommand::SetReferenceSelection { .. } => "setReferenceSelection",
            CadCommand::ReferenceHover { .. } => "referenceHover",
            CadCommand::EngagementInput { .. } => "engagementInput",
            CadCommand::EngagementPossibleSelect { .. } => "engagementPossibleSelect",
            CadCommand::EngagementRepeatLast { .. } => "engagementRepeatLast",
            CadCommand::EngagementAbort => "engagementAbort",
            CadCommand::WorldPointerMove { .. } => "worldPointerMove",
            CadCommand::SetPrimitiveSelection { .. } => "setPrimitiveSelection",
            CadCommand::ToggleSun => "toggleSun",
            CadCommand::SetSunAzimuth { .. } => "setSunAzimuth",
            CadCommand::SetSunElevation { .. } => "setSunElevation",
            CadCommand::SetSunIntensity { .. } => "setSunIntensity",
            CadCommand::SetActiveUtility { .. } => SET_ACTIVE_UTILITY_ACTION_ID,
            CadCommand::SetLocale { .. } => "setLocale",
            CadCommand::SetTerminology { .. } => "setTerminology",
            CadCommand::SaveSelected => "saveSelected",
            CadCommand::SaveInPlay => "saveInPlay",
            CadCommand::SaveCurrent { .. } => "saveCurrent",
            CadCommand::LoadRawRequest => "loadRawRequest",
        }
    }

    fn handle(
        &self,
        command: &CadCommand,
        doc: &DocumentView<'_, CadScene>,
        cfg: &ConfigView<'_, CadConfig>,
    ) -> Emit<CadOperation, CadConfigOperation> {
        let document = doc.projection;
        let base_config = cfg.projection;
        let mut runtime = cad_runtime_from_config(base_config);
        let snapshot_of = |runtime: &CadPlayRuntime| CadConfigOperation::Snapshot { config: cad_config_from_runtime(runtime, base_config) };
        match command {
            CadCommand::SetActiveExample { example_id } => {
                let (scene, example_runtime) = if example_id.is_empty() {
                    (default_document(), CadPlayRuntime::default())
                } else if example_id == CAD_EXAMPLE_FOREST_LEFT || example_id == "forest-left" {
                    let forest_camera = forest_play_camera();
                    (
                        forest_play_scene(),
                        CadPlayRuntime {
                            active_example_id: Some(CAD_EXAMPLE_FOREST_LEFT.into()),
                            camera: forest_camera.clone(),
                            camera_building: forest_camera.clone(),
                            camera_energy: forest_camera.clone(),
                            camera_structure_classic: forest_camera,
                            ..CadPlayRuntime::default()
                        },
                    )
                } else {
                    return Emit::default();
                };
                runtime = example_runtime;
                let mut emit = Emit::operations(vec![CadOperation::SetScene { scene: Box::new(scene) }]);
                emit.config_operations = vec![snapshot_of(&runtime)];
                emit
            }
            CadCommand::SetActiveUtility { utility_id } => {
                // 🧰️ Switching the active utility is config-only: it never mutates the document. Clear
                // any in-progress engagement session / rubber-band scratch so a stale preview cannot
                // leak across a utility switch.
                runtime.engagement_input.clear();
                runtime.engagement_session = None;
                runtime.engagement_step = "Idle".into();
                runtime.hovered_object_id = None;
                runtime.hovered_target = None;
                let mut config = cad_config_from_runtime(&runtime, base_config);
                config.active_utility_id = utility_id.clone();
                Emit::config(vec![CadConfigOperation::Snapshot { config }])
            }
            CadCommand::SetLocale { value } => {
                let mut config = cad_config_from_runtime(&runtime, base_config);
                config.locale = value.clone();
                Emit::config(vec![CadConfigOperation::Snapshot { config }])
            }
            CadCommand::SetTerminology { value } => {
                let mut config = cad_config_from_runtime(&runtime, base_config);
                config.terminology = value.clone();
                Emit::config(vec![CadConfigOperation::Snapshot { config }])
            }
            CadCommand::SetDislocateOption { pane, option, pressed } => {
                let pane = pane.as_deref().map(cad_pane_id_from_suffix).unwrap_or(CadPaneId::Shape);
                let window_id = cad_window_id_for_pane(pane);
                let options = runtime.dislocate_options_by_window_id.entry(window_id.into()).or_default();
                match option.as_str() {
                    "move" => options.move_enabled = pressed.unwrap_or(!options.move_enabled),
                    "rotate" => options.rotate_enabled = pressed.unwrap_or(!options.rotate_enabled),
                    _ => {}
                }
                Emit::config(vec![snapshot_of(&runtime)])
            }
            CadCommand::SetSelection { object_ids } => {
                runtime.selected_object_ids = SelectionSet::from(object_ids.clone());
                runtime.selected_node_ids.clear();
                runtime.selected_primitive_id = None;
                runtime.selected_primitive_kind = None;
                runtime.selected_reference_model_definition_id = None;
                runtime.selected_reference_id = None;
                runtime.active_object_id = runtime.selected_object_ids.first().map(str::to_string);
                clear_component_selection(&mut runtime);
                Emit::config(vec![snapshot_of(&runtime)])
            }
            CadCommand::SetNodeSelection { node_ids } => {
                runtime.selected_node_ids = node_ids.clone();
                runtime.selected_object_ids.clear();
                Emit::config(vec![snapshot_of(&runtime)])
            }
            CadCommand::SetCamera { pane, camera } => {
                // 🎥️ `pane` carries the FULL `surfaceId` (`"cad.play.scene3d/building"`), not a bare
                // pane suffix — mirrors the pre-B1 `args.get("surfaceId")` resolution exactly.
                let pane = pane.as_deref().map(cad_pane_id_from_surface_id).unwrap_or(CadPaneId::Shape);
                *cad_pane_camera_runtime_mut(&mut runtime, pane) = camera.clone();
                Emit::amend_config(vec![snapshot_of(&runtime)], format!("camera:{}", cad_pane_suffix(pane)))
            }
            CadCommand::SetProjection { pane, field, value_str, value_num, param } => {
                // 🎥️ `pane` carries the full `surfaceId` — see `SetCamera`'s doc comment above.
                let pane_id = pane.as_deref().map(cad_pane_id_from_surface_id).unwrap_or(CadPaneId::Shape);
                let mut camera = cad_pane_camera_runtime(&runtime, pane_id).clone();
                let mut projection_config = cad_camera_projection_config(&camera);
                let args_value = json!({ "field": field, "value": value_str.clone().map(Value::String).or_else(|| value_num.map(|number| json!(number))), "param": param });
                let args = Some(&args_value);
                let moves_pose = world3d_projection_action_moves_pose("setProjection", args);
                apply_world3d_projection_action(&mut projection_config, "setProjection", args);
                if moves_pose {
                    let (position, _up) = world3d_projection_pose(&projection_config, camera.target, cad_camera_distance(&camera));
                    camera.position = position;
                }
                cad_camera_set_projection_config(&mut camera, &projection_config);
                *cad_pane_camera_runtime_mut(&mut runtime, pane_id) = camera;
                Emit::amend_config(vec![snapshot_of(&runtime)], format!("projection:{}", cad_pane_suffix(pane_id)))
            }
            CadCommand::SetProjectionParam { pane, field, value_str, value_num, param } => {
                // 🎥️ `pane` carries the full `surfaceId` — see `SetCamera`'s doc comment above.
                let pane_id = pane.as_deref().map(cad_pane_id_from_surface_id).unwrap_or(CadPaneId::Shape);
                let mut camera = cad_pane_camera_runtime(&runtime, pane_id).clone();
                let mut projection_config = cad_camera_projection_config(&camera);
                let args_value = json!({ "field": field, "value": value_str.clone().map(Value::String).or_else(|| value_num.map(|number| json!(number))), "param": param });
                let args = Some(&args_value);
                let moves_pose = world3d_projection_action_moves_pose("setProjectionParam", args);
                apply_world3d_projection_action(&mut projection_config, "setProjectionParam", args);
                if moves_pose {
                    let (position, _up) = world3d_projection_pose(&projection_config, camera.target, cad_camera_distance(&camera));
                    camera.position = position;
                }
                cad_camera_set_projection_config(&mut camera, &projection_config);
                *cad_pane_camera_runtime_mut(&mut runtime, pane_id) = camera;
                Emit::amend_config(vec![snapshot_of(&runtime)], format!("projection:{}", cad_pane_suffix(pane_id)))
            }
            CadCommand::TranslateSelection { object_ids, dx, dy, dz } => {
                let ids = ids_or_selection(object_ids, runtime.selected_object_ids.as_slice());
                if ids.is_empty() {
                    return Emit::default();
                }
                Emit::amend(vec![CadOperation::TranslateObjects { object_ids: ids, dx: *dx, dy: *dy, dz: *dz }], "gumball.translate")
            }
            CadCommand::RotateSelection { object_ids, ax, ay, az, angle } => {
                let ids = ids_or_selection(object_ids, runtime.selected_object_ids.as_slice());
                if ids.is_empty() {
                    return Emit::default();
                }
                Emit::amend(vec![CadOperation::RotateObjects { object_ids: ids, ax: *ax, ay: *ay, az: *az, angle: *angle }], "gumball.rotate")
            }
            CadCommand::ScaleSelection { object_ids, sx, sy, sz } => {
                let ids = ids_or_selection(object_ids, runtime.selected_object_ids.as_slice());
                if ids.is_empty() {
                    return Emit::default();
                }
                Emit::amend(vec![CadOperation::ScaleObjects { object_ids: ids, sx: *sx, sy: *sy, sz: *sz }], "gumball.scale")
            }
            CadCommand::AddObject { typology } => {
                let typology = typology.as_deref().unwrap_or("spatial.shape.primitive.box");
                let pane = cad_pane_from_model_definition_id(&document.active_model_definition_id)
                    .unwrap_or(CadPaneId::Shape);
                let object = make_object_for_typology(typology, cad_pane_objects(document, pane).len(), pane);
                runtime.selected_object_ids = SelectionSet::from(vec![object.id.clone()]);
                let mut emit = Emit::operations(vec![CadOperation::AddObject { pane, object }]);
                emit.config_operations = vec![snapshot_of(&runtime)];
                emit
            }
            CadCommand::PatchObject { object_id, field, value, delta } => {
                let value_json = value.as_deref().map(|entry| command_value_json(field, entry));
                let delta_json = delta.map(|entry| json!(entry));
                Emit::operations(patch_objects_operations(document, std::slice::from_ref(object_id), field, value_json.as_ref(), delta_json.as_ref()))
            }
            CadCommand::PatchSelection { object_ids, field, value, delta } => {
                let ids = ids_or_selection(object_ids, runtime.selected_object_ids.as_slice());
                let value_json = value.as_deref().map(|entry| command_value_json(field, entry));
                let delta_json = delta.map(|entry| json!(entry));
                Emit::operations(patch_objects_operations(document, &ids, field, value_json.as_ref(), delta_json.as_ref()))
            }
            CadCommand::DeleteObject { object_id } => {
                if let Some(pane) = cad_find_object_pane(document, object_id) {
                    runtime.selected_object_ids.remove_id(object_id);
                    let mut emit = Emit::operations(vec![CadOperation::RemoveObject { pane, object_id: object_id.clone() }]);
                    emit.config_operations = vec![snapshot_of(&runtime)];
                    return emit;
                }
                Emit::default()
            }
            CadCommand::DuplicateObject { object_id } => {
                let duplicate_target = cad_all_objects(document)
                    .find(|(object, _)| &object.id == object_id)
                    .map(|(object, pane)| (object.clone(), pane));
                if let Some((mut duplicate, pane)) = duplicate_target {
                    duplicate.id = next_cad_id("object");
                    duplicate.label = format!("{} copy", duplicate.label);
                    runtime.selected_object_ids = SelectionSet::from(vec![duplicate.id.clone()]);
                    let mut emit = Emit::operations(vec![CadOperation::AddObject { pane, object: duplicate }]);
                    emit.config_operations = vec![snapshot_of(&runtime)];
                    return emit;
                }
                Emit::default()
            }
            CadCommand::AddNode { kind } => {
                let id = next_cad_id("node");
                let label = format!("Node {}", document.nodes.len() + 1);
                let node = CadNode { id: id.clone(), label, kind: kind.clone() };
                runtime.selected_node_ids = vec![id];
                let mut emit = Emit::operations(vec![CadOperation::AddNode { node }]);
                emit.config_operations = vec![snapshot_of(&runtime)];
                emit
            }
            CadCommand::RenameNode { node_id, value } => {
                if node_id.is_empty() || value.is_empty() {
                    return Emit::default();
                }
                Emit::operations(vec![CadOperation::RenameNode { node_id: node_id.clone(), label: value.clone() }])
            }
            CadCommand::WorldSelect { ids, merge } => {
                runtime.selected_object_ids = merge_world_selection_ids(&runtime.selected_object_ids, ids, merge);
                runtime.selected_node_ids.clear();
                runtime.selected_primitive_id = None;
                runtime.selected_primitive_kind = None;
                runtime.selected_reference_model_definition_id = None;
                runtime.selected_reference_id = None;
                runtime.active_object_id = runtime.selected_object_ids.first().map(str::to_string);
                clear_component_selection(&mut runtime);
                Emit::config(vec![snapshot_of(&runtime)])
            }
            CadCommand::WorldHover { object_id } => {
                runtime.hovered_object_id = object_id.clone();
                runtime.hovered_target = runtime.hovered_object_id.as_ref().map(|object_id| CadHoverTarget {
                    object_id: Some(object_id.clone()),
                    mode: Some("mesh".into()),
                    id: Some(0),
                });
                Emit::config(vec![snapshot_of(&runtime)])
            }
            CadCommand::SetHover { object_id, mode, id } => {
                if object_id.is_none() {
                    runtime.hovered_target = None;
                    runtime.hovered_object_id = None;
                } else {
                    let mut mode = mode.clone();
                    // 🧵️ Curve-primitive objects (structure beams/columns/walls) are whole instances.
                    if mode.as_deref() == Some("edge") {
                        if let Some(object_id) = object_id.as_deref() {
                            if cad_all_objects(document)
                                .find(|(object, _)| object.id == object_id)
                                .is_some_and(|(object, _)| primary_primitive_kind(object) == "curve")
                            {
                                mode = Some("mesh".into());
                            }
                        }
                    }
                    runtime.hovered_object_id = object_id.clone();
                    runtime.hovered_target = Some(CadHoverTarget { object_id: object_id.clone(), mode, id: *id });
                }
                Emit::config(vec![snapshot_of(&runtime)])
            }
            CadCommand::WorldPick { id, merge, granularity, object_id, surface_id, pane } => {
                if id.is_none() {
                    if merge == "replace" {
                        runtime.selected_object_ids.clear();
                        runtime.selected_primitive_id = None;
                        runtime.selected_primitive_kind = None;
                        runtime.active_object_id = None;
                        clear_component_selection(&mut runtime);
                    }
                    return Emit::config(vec![snapshot_of(&runtime)]);
                }
                if matches!(granularity.as_str(), "edge" | "face" | "vertex") {
                    let resolved_object_id = object_id
                        .clone()
                        .or_else(|| runtime.hovered_target.as_ref().and_then(|target| target.object_id.clone()))
                        .or_else(|| runtime.hovered_object_id.clone())
                        .or_else(|| resolve_active_object_id(&runtime));
                    // 🧵️ Curve centerlines are the model-definition objects — select the instance, not an edge component.
                    let curve_object_id = resolved_object_id.as_deref().and_then(|object_id| {
                        cad_all_objects(document)
                            .find(|(object, _)| object.id == object_id)
                            .map(|(object, _)| object)
                            .filter(|object| primary_primitive_kind(object) == "curve")
                            .map(|object| object.id.clone())
                    });
                    if let Some(curve_id) = curve_object_id {
                        runtime.selected_object_ids = merge_world_selection_ids(&runtime.selected_object_ids, &[curve_id.clone()], merge);
                        runtime.active_object_id = Some(curve_id);
                        runtime.selected_node_ids.clear();
                        runtime.selected_primitive_id = None;
                        runtime.selected_primitive_kind = None;
                        runtime.selected_reference_model_definition_id = None;
                        runtime.selected_reference_id = None;
                        clear_component_selection(&mut runtime);
                        return Emit::config(vec![snapshot_of(&runtime)]);
                    }
                    let component_id = id.unwrap_or(0) as u32;
                    apply_component_selection(&mut runtime, granularity, &[component_id], merge, resolved_object_id.as_deref());
                    runtime.selected_node_ids.clear();
                    runtime.selected_primitive_id = None;
                    runtime.selected_primitive_kind = None;
                    runtime.selected_reference_model_definition_id = None;
                    runtime.selected_reference_id = None;
                    return Emit::config(vec![snapshot_of(&runtime)]);
                }
                let index = id.unwrap_or(0) as usize;
                let pane_id = surface_id
                    .as_deref()
                    .map(cad_pane_id_from_surface_id)
                    .or_else(|| pane.as_deref().map(cad_pane_id_from_suffix))
                    .unwrap_or(CadPaneId::Shape);
                if let Some(object) = cad_pane_objects(document, pane_id).iter().filter(|object| object.visible).nth(index) {
                    let picked_id = object.id.clone();
                    runtime.selected_object_ids = merge_world_selection_ids(&runtime.selected_object_ids, &[picked_id.clone()], merge);
                    runtime.active_object_id = Some(picked_id);
                    runtime.selected_node_ids.clear();
                    runtime.selected_primitive_id = None;
                    runtime.selected_primitive_kind = None;
                    runtime.selected_reference_model_definition_id = None;
                    runtime.selected_reference_id = None;
                    clear_component_selection(&mut runtime);
                }
                Emit::config(vec![snapshot_of(&runtime)])
            }
            CadCommand::SetSelectionMethod { method } => {
                runtime.selection_method = method.clone();
                Emit::config(vec![snapshot_of(&runtime)])
            }
            CadCommand::FocusModelDefinition { model_definition_id } => {
                Emit::operations(vec![CadOperation::SetActiveModelDefinition { model_definition_id: model_definition_id.clone() }])
            }
            CadCommand::ApplyTransformation { qid } => Emit::operations(apply_transformation_operations(document, qid)),
            CadCommand::SaveSelected => {
                let view = CadPlayView { document: document.clone(), runtime: runtime.clone() };
                Emit::effect(cad_spatial_export_effect(export_spatial_json(&view, "selected"), "cad.selected.spatial.dsl"))
            }
            CadCommand::SaveInPlay => {
                let view = CadPlayView { document: document.clone(), runtime: runtime.clone() };
                let effect = match export_solid_modelspace(&view, OsMediaFormat::Step) {
                    Some(export) => cad_solid_export_effect(export),
                    None => cad_spatial_export_effect(export_spatial_json(&view, "modelspace"), "cad.modelspace.spatial.dsl"),
                };
                Emit::effect(effect)
            }
            CadCommand::SaveCurrent { format } => {
                let format = match format.as_deref() {
                    Some("obj") => OsMediaFormat::Obj,
                    Some("stl") => OsMediaFormat::Stl,
                    _ => OsMediaFormat::Step,
                };
                let pane = cad_pane_from_model_definition_id(&document.active_model_definition_id).unwrap_or(CadPaneId::Shape);
                let view = CadPlayView { document: document.clone(), runtime: runtime.clone() };
                let effect = match export_solid_for_pane(&view, pane, format) {
                    Some(export) => cad_solid_export_effect(export),
                    None => cad_spatial_export_effect(export_spatial_json(&view, "current"), "cad.current.spatial.dsl"),
                };
                Emit::effect(effect)
            }
            CadCommand::LoadRawRequest => Emit::effect(HostEffect::RequestFileOpen {
                accept: ".dsl,.spatial.dsl,.spk,.ops,.stp,.step,.obj,.stl,.glb,application/octet-stream,text/plain".into(),
                read_as: Some("dataUrl".into()),
                import_action: "importCadFile".into(),
                multiple: false,
            }),
            CadCommand::ImportCadFile { name, payload } => {
                let name_lower = name.to_ascii_lowercase();
                let payload_value: Value = serde_json::from_str(payload).unwrap_or_else(|_| Value::String(payload.clone()));
                if let Some(object) = import_cad_object_by_extension(&name_lower, &payload_value) {
                    runtime.selected_object_ids = SelectionSet::from(vec![object.id.clone()]);
                    let mut emit = Emit::operations(vec![CadOperation::AddObject { pane: CadPaneId::Shape, object }]);
                    emit.config_operations = vec![snapshot_of(&runtime)];
                    return emit;
                }
                let unwrapped = unwrap_spatial_load_payload(&payload_value).unwrap_or(payload_value);
                let scene = scene_from_spatial_payload(&unwrapped).or_else(|| serde_json::from_value::<CadScene>(unwrapped).ok());
                if let Some(scene) = scene {
                    runtime.selected_object_ids.clear();
                    runtime.engagement_session = None;
                    let mut emit = Emit::operations(vec![CadOperation::SetScene { scene: Box::new(scene) }]);
                    emit.config_operations = vec![snapshot_of(&runtime)];
                    return emit;
                }
                Emit::default()
            }
            CadCommand::SetReferenceSelection { pane, model_definition_id, reference_id } => {
                let pane_id = pane
                    .as_deref()
                    .map(cad_pane_id_from_suffix)
                    .or_else(|| model_definition_id.as_deref().and_then(cad_pane_from_model_definition_id))
                    .unwrap_or(CadPaneId::Shape);
                runtime.selected_reference_model_definition_id = Some(pane_id.model_definition_id().into());
                runtime.selected_reference_id = reference_id.clone();
                runtime.selected_object_ids.clear();
                runtime.selected_node_ids.clear();
                runtime.selected_primitive_id = None;
                runtime.selected_primitive_kind = None;
                runtime.active_object_id = None;
                clear_component_selection(&mut runtime);
                Emit::config(vec![snapshot_of(&runtime)])
            }
            CadCommand::ReferenceHover { reference_id } => {
                runtime.hovered_object_id = reference_id.as_deref().map(|id| format!("reference:{id}"));
                Emit::config(vec![snapshot_of(&runtime)])
            }
            CadCommand::PatchCadPlayReference { model_definition_id, reference_id, field, value, delta } => {
                let value_json = value.as_deref().map(|entry| command_value_json(field, entry));
                let delta_json = delta.map(|entry| json!(entry));
                let patch = match field.as_str() {
                    "hidden" => value_json.as_ref().and_then(Value::as_bool).map(|hidden| CadReferencePatch { hidden: Some(hidden), ..Default::default() }),
                    "locked" => value_json.as_ref().and_then(Value::as_bool).map(|locked| CadReferencePatch { locked: Some(locked), ..Default::default() }),
                    "widthWorld" => {
                        let current = document
                            .references_by_model_definition_id
                            .get(model_definition_id)
                            .and_then(|refs| refs.iter().find(|reference| &reference.id == reference_id))
                            .map(|reference| reference.width_world)
                            .unwrap_or(0.0);
                        resolve_number_edit(current, value_json.as_ref(), delta_json.as_ref()).map(|width_world| CadReferencePatch { width_world: Some(width_world), ..Default::default() })
                    }
                    _ => axis3_index(field, "origin").and_then(|axis| {
                        let mut origin = document
                            .references_by_model_definition_id
                            .get(model_definition_id)
                            .and_then(|refs| refs.iter().find(|reference| &reference.id == reference_id))
                            .map(|reference| reference.origin)
                            .unwrap_or([0.0, 0.0, 0.0]);
                        let updated = resolve_number_edit(origin[axis], value_json.as_ref(), delta_json.as_ref())?;
                        origin[axis] = updated;
                        Some(CadReferencePatch { origin: Some(origin), ..Default::default() })
                    }),
                };
                match patch {
                    Some(patch) => Emit::operations(vec![CadOperation::PatchReference { model_definition_id: model_definition_id.clone(), reference_id: reference_id.clone(), patch }]),
                    None => Emit::default(),
                }
            }
            CadCommand::EngagementInput { value, pane } => {
                runtime.engagement_input = value.clone();
                runtime.engagement_pane = pane.clone();
                Emit::config(vec![snapshot_of(&runtime)])
            }
            CadCommand::EngagementSubmit { pane } => {
                let pane_id = pane.as_deref().map(cad_pane_id_from_suffix).unwrap_or(CadPaneId::Shape);
                let ops = engagement_submit_operations(document, &mut runtime, pane_id);
                let mut emit = Emit::operations(ops);
                emit.config_operations = vec![snapshot_of(&runtime)];
                emit
            }
            CadCommand::EngagementPossibleSelect { pane, possible_id } => {
                let pane_id = pane.as_deref().map(cad_pane_id_from_suffix).unwrap_or(CadPaneId::Shape);
                let step = runtime.engagement_session.as_mut().and_then(|session| apply_event(session, possible_id, None).then(|| session.state.clone()));
                if let Some(step) = step {
                    runtime.engagement_step = step;
                } else if !start_interaction_session(&mut runtime, pane_id, possible_id) {
                    runtime.engagement_input = possible_id.clone();
                }
                Emit::config(vec![snapshot_of(&runtime)])
            }
            CadCommand::EngagementRepeatLast { pane } => {
                let pane_id = pane.as_deref().map(cad_pane_id_from_suffix).unwrap_or(CadPaneId::Shape);
                if runtime.engagement_session.is_none() {
                    if let Some(interaction_id) = runtime.last_finalized_interaction_id.clone() {
                        start_interaction_session(&mut runtime, pane_id, &interaction_id);
                        return Emit::config(vec![snapshot_of(&runtime)]);
                    }
                }
                runtime.engagement_step = "Idle".into();
                Emit::config(vec![snapshot_of(&runtime)])
            }
            CadCommand::EngagementAbort => {
                runtime.engagement_input.clear();
                runtime.engagement_session = None;
                runtime.engagement_step = "Idle".into();
                Emit::config(vec![snapshot_of(&runtime)])
            }
            CadCommand::WorldPointerDown { pane, surface_id, x, y, z } => {
                let pane_id = pane
                    .as_deref()
                    .map(cad_pane_id_from_suffix)
                    .or_else(|| surface_id.as_deref().and_then(|surface_id| surface_id.rsplit('/').next()).map(cad_pane_id_from_suffix))
                    .unwrap_or(CadPaneId::Shape);
                // 📍️ `apply_event`'s payload for a pointer event is the raw position value itself
                // (mirrors the pre-B1 `args.get("position")` extraction — NOT re-wrapped in another
                // `{"position": ...}` object).
                let point_value = (x.is_some() || y.is_some() || z.is_some()).then(|| json!([x.unwrap_or(0.0), y.unwrap_or(0.0), z.unwrap_or(0.0)]));
                let commit = runtime.engagement_session.as_mut().and_then(|session| apply_event(session, "pointer.down", point_value.as_ref()).then(|| (session.state.clone(), session.clone())));
                if let Some((step, snapshot)) = commit {
                    runtime.engagement_step = step;
                    let ops = try_commit_session_operations(document, &mut runtime, pane_id, &snapshot);
                    let mut emit = Emit::operations(ops);
                    emit.config_operations = vec![snapshot_of(&runtime)];
                    return emit;
                }
                Emit::default()
            }
            CadCommand::WorldPointerMove { x, y, z } => {
                // Live rubber-band preview during an active engagement session: applies `pointer.move`
                // (updating the session's cursor/preview context) without ever committing an object or
                // touching VCS history — coalesced (`amend_config`) so a whole drag is one undo step.
                // 📍️ `apply_event`'s payload for a pointer event is the raw position value itself
                // (mirrors the pre-B1 `args.get("position")` extraction — NOT re-wrapped in another
                // `{"position": ...}` object).
                let point_value = (x.is_some() || y.is_some() || z.is_some()).then(|| json!([x.unwrap_or(0.0), y.unwrap_or(0.0), z.unwrap_or(0.0)]));
                if let Some(session) = runtime.engagement_session.as_mut() {
                    apply_event(session, "pointer.move", point_value.as_ref());
                    let mut seq = self.preview_seq.borrow_mut();
                    *seq = seq.wrapping_add(1);
                    Emit::amend_config(vec![snapshot_of(&runtime)], "engagement.pointer-move")
                } else {
                    Emit::default()
                }
            }
            CadCommand::SetPrimitiveSelection { object_id, primitive_id, kind } => {
                runtime.selected_object_ids = SelectionSet::from(vec![object_id.clone()]);
                runtime.selected_node_ids.clear();
                runtime.selected_primitive_id = primitive_id.clone();
                runtime.selected_primitive_kind = kind.clone();
                runtime.selected_reference_model_definition_id = None;
                runtime.selected_reference_id = None;
                Emit::config(vec![snapshot_of(&runtime)])
            }
            CadCommand::ToggleSun => {
                apply_world3d_sun_action(&mut runtime.sun, "toggleSun", None);
                Emit::amend_config(vec![snapshot_of(&runtime)], "sun")
            }
            CadCommand::SetSunAzimuth { value } => {
                let args_value = json!({ "value": value });
                apply_world3d_sun_action(&mut runtime.sun, "setSunAzimuth", Some(&args_value));
                Emit::amend_config(vec![snapshot_of(&runtime)], "sun")
            }
            CadCommand::SetSunElevation { value } => {
                let args_value = json!({ "value": value });
                apply_world3d_sun_action(&mut runtime.sun, "setSunElevation", Some(&args_value));
                Emit::amend_config(vec![snapshot_of(&runtime)], "sun")
            }
            CadCommand::SetSunIntensity { value } => {
                let args_value = json!({ "value": value });
                apply_world3d_sun_action(&mut runtime.sun, "setSunIntensity", Some(&args_value));
                Emit::amend_config(vec![snapshot_of(&runtime)], "sun")
            }
        }
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, CadScene>, cfg: &ConfigView<'_, CadConfig>) -> UiNode {
        let view = CadPlayView { document: doc.projection.clone(), runtime: cad_runtime_from_config(cfg.projection) };
        let labels = cad_labels(cfg.projection);
        let window_kind_id = match body_key {
            CAD_PLAY_BODY_SHAPE => CAD_PLAY_WINDOW_SHAPE,
            CAD_PLAY_BODY_BUILDING => CAD_PLAY_WINDOW_BUILDING,
            CAD_PLAY_BODY_ENERGY => CAD_PLAY_WINDOW_ENERGY,
            CAD_PLAY_BODY_STRUCTURE_CLASSIC => CAD_PLAY_WINDOW_STRUCTURE_CLASSIC,
            _ => CAD_PLAY_WINDOW_SHAPE,
        };
        let active_utility = Some(cfg.projection.active_utility_id.as_str());
        let options = view.runtime.dislocate_options(window_kind_id);
        match body_key {
            CAD_PLAY_BODY_SHAPE => build_world_scene_for_pane(&view, CadPaneId::Shape, CAD_PLAY_SURFACE_SHAPE, active_utility, options),
            CAD_PLAY_BODY_BUILDING => {
                build_world_scene_for_pane(&view, CadPaneId::Building, CAD_PLAY_SURFACE_BUILDING, active_utility, options)
            }
            CAD_PLAY_BODY_ENERGY => build_world_scene_for_pane(&view, CadPaneId::Energy, CAD_PLAY_SURFACE_ENERGY, active_utility, options),
            CAD_PLAY_BODY_STRUCTURE_CLASSIC => build_world_scene_for_pane(
                &view,
                CadPaneId::StructureClassic,
                CAD_PLAY_SURFACE_STRUCTURE_CLASSIC,
                active_utility,
                options,
            ),
            CAD_PLAY_BODY_DOCUMENT => build_document_tree(&view, labels),
            CAD_PLAY_BODY_CATALOGUE => build_catalogue_tree(labels),
            CAD_PLAY_BODY_PROPERTIES => build_properties_panel(&view, labels, active_utility),
            _ => ui_text(format!("Unknown body: {body_key}")),
        }
    }

    fn window_engagements(&self, doc: &DocumentView<'_, CadScene>, cfg: &ConfigView<'_, CadConfig>) -> HashMap<String, WindowEngagement> {
        let view = CadPlayView { document: doc.projection.clone(), runtime: cad_runtime_from_config(cfg.projection) };
        let labels = cad_labels(cfg.projection);
        HashMap::from([
            (
                CAD_PLAY_WINDOW_SHAPE.to_string(),
                cad_window_engagement(&view, CadPaneId::Shape, labels),
            ),
            (
                CAD_PLAY_WINDOW_BUILDING.to_string(),
                cad_window_engagement(&view, CadPaneId::Building, labels),
            ),
            (
                CAD_PLAY_WINDOW_ENERGY.to_string(),
                cad_window_engagement(&view, CadPaneId::Energy, labels),
            ),
            (
                CAD_PLAY_WINDOW_STRUCTURE_CLASSIC.to_string(),
                cad_window_engagement(&view, CadPaneId::StructureClassic, labels),
            ),
        ])
    }

    /// 🪟️ Keyed by the 4 fixed window-KIND ids (was keyed by dynamic window-INSTANCE id, resolved off
    /// `ViewState.window_instances` — deleted by B1; `window_measures` has no per-instance parameter
    /// anymore, see `CadDislocateOptions`'s doc comment in `cad_document_engine`).
    fn window_measures(&self, _doc: &DocumentView<'_, CadScene>, cfg: &ConfigView<'_, CadConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let runtime = cad_runtime_from_config(cfg.projection);
        let is_de = cad_is_de_locale(cfg.projection);
        let pane_measures = |pane: CadPaneId, window_id: &str| {
            vec![
                world3d_projection_measures(&format!("cad-{}", pane.model_definition_id()), &cad_camera_projection_config(cad_pane_camera_runtime(&runtime, pane)), cad_action),
                world3d_sun_measures("cad", &runtime.sun, cad_action),
                cad_dislocate_utility_options(runtime.dislocate_options(window_id), is_de),
            ]
        };
        [
            (CAD_PLAY_WINDOW_SHAPE, CadPaneId::Shape),
            (CAD_PLAY_WINDOW_BUILDING, CadPaneId::Building),
            (CAD_PLAY_WINDOW_ENERGY, CadPaneId::Energy),
            (CAD_PLAY_WINDOW_STRUCTURE_CLASSIC, CadPaneId::StructureClassic),
        ]
        .into_iter()
        .map(|(window_kind_id, pane)| (window_kind_id.to_string(), pane_measures(pane, window_kind_id)))
        .collect()
    }

    fn app_labels(&self, cfg: &ConfigView<'_, CadConfig>) -> AppLabelsOverlay {
        let labels = cad_labels(cfg.projection);
        let is_de = cad_is_de_locale(cfg.projection);
        AppLabelsOverlay::default()
            .window_kind_label(CAD_PLAY_WINDOW_SHAPE, labels.pane_shape)
            .window_kind_label(CAD_PLAY_WINDOW_BUILDING, labels.pane_building)
            .window_kind_label(CAD_PLAY_WINDOW_ENERGY, labels.pane_energy)
            .window_kind_label(CAD_PLAY_WINDOW_STRUCTURE_CLASSIC, labels.pane_structure_classic)
            .mode_label("edit", if is_de { "Bearbeiten" } else { "Edit" })
            .action_labels(cad_action_labels(is_de))
            .utility_labels(cad_utility_labels(is_de))
            .example_labels(HashMap::from([
                (CAD_EXAMPLE_FOREST_LEFT.to_string(), (if is_de { "Sechseckig geschnittener Betonwald links" } else { "Hexagonal Cut Concrete Forest Left" }).to_string()),
            ]))
    }

    /// 🖱️ Selection-gated menu: transform/duplicate/delete only once something is selected — a bare
    /// right-click on empty World3d background (nothing selected) falls through to the shell's
    /// window-level menu (undo/redo/view actions) instead of showing an empty CAD-specific section.
    fn context_menu(
        &self,
        _request: &ContextMenuRequest,
        _doc: &DocumentView<'_, CadScene>,
        cfg: &ConfigView<'_, CadConfig>,
        registry: &AppActionRegistry,
    ) -> Vec<ContextMenuItemSpec> {
        if cfg.projection.selected_object_ids.is_empty() {
            return Vec::new();
        }
        Menu::of(registry)
            .action("translateSelection")
            .action("rotateSelection")
            .action("scaleSelection")
            .separator()
            .action("duplicateObject")
            .destructive("deleteObject")
            .build()
    }
}

/// @emoji 🪟️ One quadrant of the quad layout: a stack holding a single window kind.
//#endregion 🔖️CadPlayApp

//#region 🔖️Manifest
fn cad_window_stack(window_kind_id: &str, title: &str, size: Option<f64>) -> WindowLayoutChild {
    WindowLayoutChild::Stack(WindowLayoutStackNode {
        kind: "stack".into(),
        size,
        active_window_kind_id: None,
        children: vec![WindowLayoutWindowNode {
            kind: "window".into(),
            window_kind_id: window_kind_id.into(),
            title: Some(title.into()),
            instance_id: None,
            template_id: None,
        }],
    })
}

/// @emoji 🪟️ Quad play layout: shape/building left column, energy/structure classic right column.
fn cad_quad_layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Axis(WindowLayoutAxisNode {
            kind: "row".into(),
            size: None,
            children: vec![
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "column".into(),
                    size: Some(0.5),
                    children: vec![
                        cad_window_stack(CAD_PLAY_WINDOW_SHAPE, "Shape", Some(0.5)),
                        cad_window_stack(CAD_PLAY_WINDOW_BUILDING, "Building", Some(0.5)),
                    ],
                }),
                WindowLayoutChild::Axis(WindowLayoutAxisNode {
                    kind: "column".into(),
                    size: Some(0.5),
                    children: vec![
                        cad_window_stack(CAD_PLAY_WINDOW_ENERGY, "Energy", Some(0.5)),
                        cad_window_stack(CAD_PLAY_WINDOW_STRUCTURE_CLASSIC, "Structure Classic", Some(0.5)),
                    ],
                }),
            ],
        }),
    }
}

/// @emoji 🧰️ The window-scoped CAD Dislocate utility, whose Move and Rotate handles are utility options.
fn cad_dislocate_utility() -> UtilityDefinition {
    UtilityDefinition {
        category: Some(UtilityCategory::Utilities),
        ..UtilityDefinition::new(CAD_DISLOCATE_UTILITY_ID, "Dislocate", "move-3d")
    }
}

/// @emoji 🧰️ The single Dislocate utility ref exposed independently by each world-3d window.
fn cad_dislocate_utility_refs() -> Vec<semio_framework_plugin::UtilityRef> {
    vec![CAD_DISLOCATE_UTILITY_ID.into()]
}

pub fn create_cad_app() -> App {
    App::from_builder(
        App::builder(CAD_PLAY_APP_ID, "CAD").document(["semio", "cad"])
            .artifact_kind(ArtifactKindSpec {
                id: "3d.cad".into(),
                name: "3D CAD".into(),
                source_format: "cad.scene".into(),
                component_kind: "cad".into(),
                dimension: "3d".into(),
                media_capability: OsMediaCapability::Brep,
                media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
                schema: "cad.scene".into(),
                export_formats: vec![OsMediaFormat::Step, OsMediaFormat::Obj, OsMediaFormat::Stl, OsMediaFormat::Glb],
                import_formats: vec![OsMediaFormat::Step, OsMediaFormat::Obj, OsMediaFormat::Stl],
            })
            .icon_id("box")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "cad"])
            .mode("edit", "Edit", "square-pen")
            .default_mode_id("edit")
            .window_kind(CAD_PLAY_WINDOW_SHAPE, "Shape", CAD_PLAY_BODY_SHAPE, SurfaceKind::World3d, "cad-shape")
            .window_kind(CAD_PLAY_WINDOW_BUILDING, "Building", CAD_PLAY_BODY_BUILDING, SurfaceKind::World3d, "landmark")
            .window_kind(CAD_PLAY_WINDOW_ENERGY, "Energy", CAD_PLAY_BODY_ENERGY, SurfaceKind::World3d, "sun")
            .window_kind(CAD_PLAY_WINDOW_STRUCTURE_CLASSIC, "Structure Classic", CAD_PLAY_BODY_STRUCTURE_CLASSIC, SurfaceKind::World3d, "component")
            .default_layout(cad_quad_layout())
            .operation("addObject", "Add Object")
            .operation("patchObject", "Patch Object")
            .operation("patchSelection", "Patch Selection")
            .operation("deleteObject", "Delete Object")
            .operation("duplicateObject", "Duplicate Object")
            .operation("addNode", "Add Node")
            .operation("renameNode", "Rename Node")
            .operation("translateSelection", "Translate Selection")
            .operation("rotateSelection", "Rotate Selection")
            .operation("scaleSelection", "Scale Selection")
            .operation("applyTransformation", "Apply Transformation")
            .operation("importCadFile", "Import CAD File")
            .action_with(ActionDefinition::new_catalog("patchCadPlayReference", "Patch Reference", ActionKind::Operation).in_palette(false))
            .action_with(ActionDefinition::new_catalog("engagementSubmit", "Engagement Submit", ActionKind::Operation).in_palette(false))
            .view_action("setCamera", "Set Camera")
            .view_action("setProjection", "Set Projection")
            .view_action("setProjectionParam", "Set Projection Parameter")
            .operation("focusModelDefinition", "Focus Model Definition")
            .operation("setActiveExample", "Set Active Example")
            .action_with(ActionDefinition::new_catalog("setSelection", "Set Selection", ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("setNodeSelection", "Set Node Selection", ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("worldSelect", "World Select", ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("worldHover", "World Hover", ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("setHover", "Set Hover", ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("worldPick", "World Pick", ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("setSelectionMethod", "Set Selection Method", ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("setReferenceSelection", "Set Reference Selection", ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("referenceHover", "Reference Hover", ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("engagementInput", "Engagement Input", ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("engagementPossibleSelect", "Engagement Possible Select", ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("engagementRepeatLast", "Engagement Repeat Last", ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("engagementAbort", "Engagement Abort", ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("worldPointerDown", "World Pointer Down", ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("worldPointerMove", "World Pointer Move", ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("engagementPointerDown", "Engagement Pointer Down", ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("setPrimitiveSelection", "Set Primitive Selection", ActionKind::View).in_palette(false))
            .view_action("toggleSun", "Toggle Sun")
            .view_action("setSunAzimuth", "Set Sun Azimuth")
            .view_action("setSunElevation", "Set Sun Elevation")
            .view_action("setSunIntensity", "Set Sun Intensity")
            .action_with(ActionDefinition::new_catalog("setDislocateOption", "Set Dislocate Option", ActionKind::View).in_palette(false))
            .shell_action("saveSelected", "Save Selected")
            .shell_action("saveInPlay", "Save In Play")
            .shell_action("saveCurrent", "Save Current")
            .shell_action("loadRawRequest", "Load Raw Request")
            .action_args("saveCurrent", vec![ActionArgDef::select("format", "Format", vec![
                ActionArgOption::new("step", "STEP"),
                ActionArgOption::new("obj", "OBJ"),
                ActionArgOption::new("stl", "STL"),
            ]).default_value("step")])
            .action_args("focusModelDefinition", vec![ActionArgDef::select("modelDefinitionId", "Model Definition", vec![
                ActionArgOption::new(CAD_MODEL_DEFINITION_SHAPE, "Shape"),
                ActionArgOption::new(CAD_MODEL_DEFINITION_BUILDING, "Building"),
                ActionArgOption::new(CAD_MODEL_DEFINITION_ENERGY, "Energy"),
                ActionArgOption::new(CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC, "Structure Classic"),
            ]).required()])
            .action_args("setActiveExample", vec![ActionArgDef::select("exampleId", "Example", vec![
                ActionArgOption::new(CAD_EXAMPLE_FOREST_LEFT, "Hexagonal Cut Concrete Forest Left"),
            ]).required()])
            .utility(cad_dislocate_utility())
            .window_kind_utilities(CAD_PLAY_WINDOW_SHAPE, cad_dislocate_utility_refs())
            .window_kind_utilities(CAD_PLAY_WINDOW_BUILDING, cad_dislocate_utility_refs())
            .window_kind_utilities(CAD_PLAY_WINDOW_ENERGY, cad_dislocate_utility_refs())
            .window_kind_utilities(CAD_PLAY_WINDOW_STRUCTURE_CLASSIC, cad_dislocate_utility_refs())
            .panel_tab(
                FRAMEWORK_PANEL_TAB_DOCUMENT_ID,
                FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
                PanelGroup::Workbench,
                CAD_PLAY_BODY_DOCUMENT,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
                FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
                PanelGroup::Workbench,
                CAD_PLAY_BODY_CATALOGUE,
            )
            .panel_tab(
                FRAMEWORK_PANEL_TAB_INSPECTION_ID,
                FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
                PanelGroup::Details,
                CAD_PLAY_BODY_PROPERTIES,
            )
            // 🎯️ Typed channel + port surface (WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2) — `cad_io()` is
            // this same `3d.cad`/Brep information's single source of truth, reused here rather than
            // duplicated; `config_spec()` stays empty (cad has no sticky-default settings analogous to
            // shooting's format defaults — every `CadConfig` field is session view-state, not a setting).
            .config(CadPlayApp::default().config_spec())
            .io(cad_io()),
    )
    .example(
        CAD_EXAMPLE_FOREST_LEFT,
        "Hexagonal Cut Concrete Forest Left",
        &serde_json::to_string(&forest_play_scene()).unwrap(),
        "trees",
    )
    .workflow("cad", "CAD", "model")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use cad_document::{empty_cad_projection, CAD_PLAY_DOCUMENT_SCHEMA};
    use cad_document_engine::{
        align_mesh_to_fixture_centroid, cad_document_from_dwg, CAD_DEFAULT_TYPOLOGY_EXTENT,
        CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX, CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX, CAD_FOREST_REFERENCE_WIDTH_WORLD,
        CAD_FOREST_REFERENCE_Y_OFFSET_RATIO,
    };
    use semio_framework_plugin::{ActionMeta, AppActionRegistry, HistoryView, PluginApp, UiMenuRef, VcsDocumentApp, ViewState};
    use protocol::{Operation, OperationDiff};
    use store::{Backbone, BackboneMessage, MemoryBackbone};

    //#region 🔖️Harness
    fn meta(actor: &str) -> ActionMeta {
        semio_framework_plugin::testkit::meta(actor)
    }

    fn new_app() -> VcsDocumentApp<CadPlayApp> {
        semio_framework_plugin::testkit::new_app::<CadPlayApp>()
    }

    fn empty_history() -> HistoryView {
        HistoryView::empty()
    }

    /// @emoji 🔀️ WORKFLOWS-END-TO-END-TYPED-PORTS test-only bridge: recovers a typed `CadCommand` from
    /// the pre-B1 `(action id, JSON args)` shape every test in this module was already written against
    /// — the same information `AppDefinition`'s declared `ActionArgDef`s carry, reconstructed by hand
    /// here rather than threading a real host-side action→command bridge (out of scope for this ticket;
    /// Wave 3 wires the shell). Panics on an unrecognized action id — every id used below is covered.
    fn command_from_action(action: &str, args: Option<&Value>) -> CadCommand {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        let f64_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_f64);
        let u64_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_u64);
        let bool_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_bool);
        let str_vec_field = |key: &str| -> Vec<String> {
            args.and_then(|value| value.get(key)).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default()
        };
        let value_string = || -> Option<String> {
            args.and_then(|value| value.get("value")).and_then(|value| match value {
                Value::String(text) => Some(text.clone()),
                Value::Bool(flag) => Some(flag.to_string()),
                Value::Number(number) => Some(number.to_string()),
                _ => None,
            })
        };
        let position_axis = |index: usize| args.and_then(|value| value.get("position")).and_then(|value| value.get(index)).and_then(Value::as_f64);
        match action {
            "setActiveExample" => CadCommand::SetActiveExample { example_id: str_field("exampleId").unwrap_or_default() },
            SET_ACTIVE_UTILITY_ACTION_ID => CadCommand::SetActiveUtility { utility_id: str_field("utilityId").unwrap_or_default() },
            "setLocale" => CadCommand::SetLocale { value: str_field("value").unwrap_or_default() },
            "setTerminology" => CadCommand::SetTerminology { value: str_field("value").unwrap_or_default() },
            "setDislocateOption" => CadCommand::SetDislocateOption { pane: str_field("pane"), option: str_field("option").unwrap_or_default(), pressed: bool_field("pressed") },
            "setSelection" => CadCommand::SetSelection { object_ids: str_vec_field("objectIds") },
            "setNodeSelection" => CadCommand::SetNodeSelection { node_ids: str_vec_field("nodeIds") },
            "setCamera" => CadCommand::SetCamera {
                pane: str_field("surfaceId"),
                camera: args.and_then(|value| value.get("camera")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default(),
            },
            "setProjection" => CadCommand::SetProjection {
                pane: str_field("surfaceId"),
                field: str_field("field"),
                value_str: args.and_then(|value| value.get("value")).and_then(Value::as_str).map(String::from),
                value_num: args.and_then(|value| value.get("value")).and_then(Value::as_f64),
                param: str_field("param"),
            },
            "setProjectionParam" => CadCommand::SetProjectionParam {
                pane: str_field("surfaceId"),
                field: str_field("field"),
                value_str: args.and_then(|value| value.get("value")).and_then(Value::as_str).map(String::from),
                value_num: args.and_then(|value| value.get("value")).and_then(Value::as_f64),
                param: str_field("param"),
            },
            "translateSelection" => CadCommand::TranslateSelection { object_ids: str_vec_field("objectIds"), dx: f64_field("dx").unwrap_or(0.0), dy: f64_field("dy").unwrap_or(0.0), dz: f64_field("dz").unwrap_or(0.0) },
            "rotateSelection" => CadCommand::RotateSelection { object_ids: str_vec_field("objectIds"), ax: f64_field("ax").unwrap_or(0.0), ay: f64_field("ay").unwrap_or(0.0), az: f64_field("az").unwrap_or(0.0), angle: f64_field("angle").unwrap_or(0.0) },
            "scaleSelection" => CadCommand::ScaleSelection { object_ids: str_vec_field("objectIds"), sx: f64_field("sx").unwrap_or(1.0), sy: f64_field("sy").unwrap_or(1.0), sz: f64_field("sz").unwrap_or(1.0) },
            "addObject" => CadCommand::AddObject { typology: str_field("typology") },
            "patchObject" => CadCommand::PatchObject { object_id: str_field("objectId").unwrap_or_default(), field: str_field("field").unwrap_or_default(), value: value_string(), delta: f64_field("delta") },
            "patchSelection" => CadCommand::PatchSelection { object_ids: str_vec_field("objectIds"), field: str_field("field").unwrap_or_default(), value: value_string(), delta: f64_field("delta") },
            "deleteObject" => CadCommand::DeleteObject { object_id: str_field("objectId").unwrap_or_default() },
            "duplicateObject" => CadCommand::DuplicateObject { object_id: str_field("objectId").unwrap_or_default() },
            "addNode" => CadCommand::AddNode { kind: str_field("kind").unwrap_or_else(|| "solid".into()) },
            "renameNode" => CadCommand::RenameNode { node_id: str_field("nodeId").unwrap_or_default(), value: str_field("value").unwrap_or_default() },
            "worldSelect" => CadCommand::WorldSelect { ids: str_vec_field("ids"), merge: str_field("merge").unwrap_or_else(|| "replace".into()) },
            "worldHover" => CadCommand::WorldHover { object_id: str_field("id") },
            "setHover" => CadCommand::SetHover { object_id: str_field("objectId"), mode: str_field("mode"), id: u64_field("id").map(|value| value as u32) },
            "worldPick" => CadCommand::WorldPick { id: u64_field("id"), merge: str_field("merge").unwrap_or_else(|| "replace".into()), granularity: str_field("granularity").unwrap_or_else(|| "mesh".into()), object_id: str_field("objectId"), surface_id: str_field("surfaceId"), pane: str_field("pane") },
            "setSelectionMethod" => CadCommand::SetSelectionMethod { method: str_field("method").unwrap_or_else(|| "rectangle".into()) },
            "focusModelDefinition" => CadCommand::FocusModelDefinition { model_definition_id: str_field("modelDefinitionId").unwrap_or_default() },
            "applyTransformation" => CadCommand::ApplyTransformation { qid: str_field("qid").unwrap_or_default() },
            "saveSelected" => CadCommand::SaveSelected,
            "saveInPlay" => CadCommand::SaveInPlay,
            "saveCurrent" => CadCommand::SaveCurrent { format: str_field("format") },
            "loadRawRequest" => CadCommand::LoadRawRequest,
            "importCadFile" => {
                let payload = args.and_then(|value| value.get("payload").or_else(|| value.get("modelSpace"))).cloned().or_else(|| args.cloned());
                let payload = match payload {
                    Some(Value::String(text)) => text,
                    Some(other) => other.to_string(),
                    None => String::new(),
                };
                CadCommand::ImportCadFile { name: str_field("name").unwrap_or_default(), payload }
            }
            "setReferenceSelection" => CadCommand::SetReferenceSelection { pane: str_field("pane"), model_definition_id: str_field("modelDefinitionId"), reference_id: str_field("referenceId") },
            "referenceHover" => CadCommand::ReferenceHover { reference_id: str_field("referenceId") },
            "patchCadPlayReference" => CadCommand::PatchCadPlayReference {
                model_definition_id: str_field("modelDefinitionId").unwrap_or_default(),
                reference_id: str_field("referenceId").unwrap_or_default(),
                field: str_field("field").unwrap_or_default(),
                value: value_string(),
                delta: f64_field("delta"),
            },
            "engagementInput" => CadCommand::EngagementInput { value: str_field("value").unwrap_or_default(), pane: str_field("pane") },
            "engagementSubmit" => CadCommand::EngagementSubmit { pane: str_field("pane") },
            "engagementPossibleSelect" => CadCommand::EngagementPossibleSelect { pane: str_field("pane"), possible_id: str_field("possibleId").unwrap_or_default() },
            "engagementRepeatLast" => CadCommand::EngagementRepeatLast { pane: str_field("pane") },
            "engagementAbort" => CadCommand::EngagementAbort,
            "worldPointerDown" | "engagementPointerDown" => CadCommand::WorldPointerDown { pane: str_field("pane"), surface_id: str_field("surfaceId"), x: position_axis(0), y: position_axis(1), z: position_axis(2) },
            "worldPointerMove" => CadCommand::WorldPointerMove { x: position_axis(0), y: position_axis(1), z: position_axis(2) },
            "setPrimitiveSelection" => CadCommand::SetPrimitiveSelection { object_id: str_field("objectId").unwrap_or_default(), primitive_id: str_field("primitiveId"), kind: str_field("kind") },
            "toggleSun" => CadCommand::ToggleSun,
            "setSunAzimuth" => CadCommand::SetSunAzimuth { value: f64_field("value").unwrap_or(0.0) },
            "setSunElevation" => CadCommand::SetSunElevation { value: f64_field("value").unwrap_or(0.0) },
            "setSunIntensity" => CadCommand::SetSunIntensity { value: f64_field("value").unwrap_or(0.0) },
            other => panic!("command_from_action: unhandled test action {other}"),
        }
    }

    /// 🕹️ Drives one action against a bare `CadPlayApp` (unwrapped, config defaulted) so tests can
    /// inspect the emitted document/config operations directly.
    fn drive(app: &CadPlayApp, scene: &CadScene, action: &str, args: Option<Value>) -> Emit<CadOperation, CadConfigOperation> {
        drive_with_config(app, scene, action, args, &CadConfig::default())
    }

    fn drive_with_config(app: &CadPlayApp, scene: &CadScene, action: &str, args: Option<Value>, config: &CadConfig) -> Emit<CadOperation, CadConfigOperation> {
        let history = empty_history();
        let doc = DocumentView { projection: scene, history: &history };
        let cfg = ConfigView { projection: config };
        let command = command_from_action(action, args.as_ref());
        app.handle(&command, &doc, &cfg)
    }

    fn render_direct(app: &CadPlayApp, body_key: &str, doc: &DocumentView<'_, CadScene>, config: &CadConfig) -> UiNode {
        let cfg = ConfigView { projection: config };
        app.render(body_key, doc, &cfg)
    }

    fn window_measures_direct(app: &CadPlayApp, doc: &DocumentView<'_, CadScene>, config: &CadConfig) -> HashMap<String, Vec<WindowMeasure>> {
        let cfg = ConfigView { projection: config };
        app.window_measures(doc, &cfg)
    }

    fn context_menu_direct(app: &CadPlayApp, doc: &DocumentView<'_, CadScene>, config: &CadConfig, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        let cfg = ConfigView { projection: config };
        let request = ContextMenuRequest { menu: UiMenuRef { id: "world3d".into(), args: None }, surface: None, window_instance_id: None, point: None };
        app.context_menu(&request, doc, &cfg, registry)
    }

    /// 🧮️ Folds a list of `CadOperation`s onto a scene via the core `Operation`/`OperationDiff` impls —
    /// mirrors what the wrapping `VcsDocumentApp` store does when it dispatches the emitted operations.
    fn apply_operations(scene: &CadScene, operations: &[CadOperation]) -> CadScene {
        let mut next = scene.clone();
        for operation in operations {
            next = operation.diff(&next).apply(&next);
        }
        next
    }

    /// 🧮️ `apply_operations`'s config-targeted twin — folds an `Emit`'s `config_operations` onto a base
    /// `CadConfig` (mirrors what `VcsDocumentApp`'s config store does when it dispatches them).
    fn config_after(emit: &Emit<CadOperation, CadConfigOperation>, base: &CadConfig) -> CadConfig {
        let mut next = base.clone();
        for operation in &emit.config_operations {
            next = operation.diff(&next);
        }
        next
    }

    /// 🧮️ `config_after` plus the `CadConfig -> CadPlayRuntime` boundary conversion — the direct
    /// replacement for the pre-B1 `app.runtime.borrow()` most tests below inspected after `drive(..)`.
    fn runtime_after(emit: &Emit<CadOperation, CadConfigOperation>, base: &CadConfig) -> CadPlayRuntime {
        cad_runtime_from_config(&config_after(emit, base))
    }

    fn view(scene: CadScene, runtime: CadPlayRuntime) -> CadPlayView {
        CadPlayView { document: scene, runtime }
    }
    //#endregion 🔖️Harness

    //#region 🔖️Fixtures
    #[test]
    fn forest_example_uses_per_object_brep_meshes() {
        let scene = forest_play_scene();
        let runtime = CadPlayRuntime::default();
        let json = world_instances_json(&scene.building_objects, &runtime);
        assert!(json.contains("object-hexagonal-cut-concrete-forest-left-bim-10"));
        let meshes = world_meshes_json(&scene.building_objects, scene.building_geometry.as_ref());
        assert!(meshes.contains("object-hexagonal-cut-concrete-forest-left-bim-10"));
        assert!(!meshes.contains("🧊️hexagonal-cut-concrete-forest-left.glb"));
        assert!(scene.building_objects.len() > 5);
        assert!(scene.building_objects.iter().all(|object| object.solid_handle.is_some()));
    }

    #[test]
    fn cad_document_from_dwg_creates_one_object_per_layer_with_geometry() {
        let mut drawing = semio_framework_core::DwgDrawing::default();
        let outline = drawing.ensure_layer("outline");
        let empty_layer = drawing.ensure_layer("empty");
        let _ = empty_layer;
        drawing.entities.push(semio_framework_core::DwgEntity {
            layer: outline,
            color: semio_framework_core::DwgColor::ByLayer,
            geometry: semio_framework_core::DwgGeometry::PolyfaceMesh {
                vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]],
                faces: vec![[1, 2, 3, 4]],
            },
        });
        let value = cad_document_from_dwg(&drawing).expect("cad document from dwg");
        let scene: CadScene = serde_json::from_value(value).expect("valid cad scene");
        assert_eq!(scene.objects.len(), 1);
        assert_eq!(scene.objects[0].label, "outline");
    }

    #[test]
    fn cad_document_from_empty_dwg_falls_back_to_default_document() {
        let drawing = semio_framework_core::DwgDrawing::default();
        let value = cad_document_from_dwg(&drawing).expect("cad document from empty dwg");
        let scene: CadScene = serde_json::from_value(value).expect("valid cad scene");
        assert!(!scene.objects.is_empty());
    }

    #[test]
    fn quad_panes_each_populate_distinct_objects() {
        let scene = forest_play_scene();
        assert!(!scene.objects.is_empty(), "shape pane");
        assert!(!scene.building_objects.is_empty(), "building pane");
        assert!(!scene.energy_objects.is_empty(), "energy pane");
        assert!(!scene.structure_classic_objects.is_empty(), "structure classic pane");
    }

    #[test]
    fn initial_projection_is_cut_concrete_forest_not_placeholder_box() {
        let app = CadPlayApp::default();
        let scene = app.initial_projection();
        assert_eq!(scene.id, CAD_EXAMPLE_FOREST_LEFT);
        assert_ne!(scene.objects.first().map(|object| object.id.as_str()), Some("object-box-1"));
        assert!(!scene.building_objects.is_empty(), "building pane must not be the empty default placeholder");
        assert!(!scene.energy_objects.is_empty(), "energy pane must not be the empty default placeholder");
        assert!(!scene.structure_classic_objects.is_empty(), "structure pane must not be the empty default placeholder");
        assert!(scene.objects.iter().all(|object| object.solid_handle.is_some()));
    }

    #[test]
    fn forest_energy_world_mesh_survives_scene_roundtrip() {
        let scene = forest_play_scene();
        let roundtrip: CadScene = serde_json::from_str(&serde_json::to_string(&scene).expect("serialize")).expect("deserialize");
        let object = roundtrip.energy_objects.first().expect("energy object");
        let mesh = object_mesh_data(object, roundtrip.energy_geometry.as_ref());
        let min_z = mesh
            .positions
            .chunks_exact(3)
            .map(|vertex| vertex[2])
            .fold(f32::INFINITY, f32::min);
        assert!(min_z > 2.5, "energy world mesh min z {min_z}");
        let slab = roundtrip
            .structure_classic_objects
            .iter()
            .find(|object| object.primitives.iter().any(|primitive| primitive.kind == "surface"))
            .expect("structure surface");
        let slab_mesh = object_mesh_data(slab, roundtrip.structure_classic_geometry.as_ref());
        let slab_min_z = slab_mesh
            .positions
            .chunks_exact(3)
            .map(|vertex| vertex[2])
            .fold(f32::INFINITY, f32::min);
        assert!(slab_min_z > 2.5, "structure world mesh min z {slab_min_z}");
    }

    #[test]
    fn forest_references_use_xy_ground_plane_and_z_up() {
        let scene = forest_play_scene();
        let reference = scene
            .references_by_model_definition_id
            .get(CAD_MODEL_DEFINITION_ENERGY)
            .and_then(|references| references.first())
            .expect("energy reference");
        assert!(
            reference.origin[2] > 2.5,
            "reference z {} should match slab elevation",
            reference.origin[2]
        );
        assert!(
            (reference.origin[0] - (-9.7)).abs() < 1e-9,
            "reference x {} should be base + 50% width (right)",
            reference.origin[0]
        );
        let expected_y = -18.0
            + CAD_FOREST_REFERENCE_WIDTH_WORLD * CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX / CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX
                * (0.5 + CAD_FOREST_REFERENCE_Y_OFFSET_RATIO);
        assert!(
            (reference.origin[1] - expected_y).abs() < 1e-9,
            "reference CAD y {} should be centered then moved +20% forward on the world plane",
            reference.origin[1]
        );
        let centered_y = -18.0
            + CAD_FOREST_REFERENCE_WIDTH_WORLD * CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX / CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX
                * 0.5;
        assert!(
            ((reference.origin[1] - centered_y)
                - CAD_FOREST_REFERENCE_WIDTH_WORLD * CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX
                    / CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX
                    * 0.2)
                .abs()
                < 1e-9,
            "the requested offset must affect CAD y only"
        );
        assert_eq!(CAD_FOREST_REFERENCE_Y_OFFSET_RATIO, 0.2);
        assert!(reference.locked, "example references default locked like puzzle 3d");
        assert_eq!(reference.width_world, 28.6);
    }

    #[test]
    fn align_mesh_to_fixture_centroid_corrects_drifted_surface() {
        let scene = forest_play_scene();
        let geometry = scene.energy_geometry.as_ref().expect("energy geometry");
        let object = scene.energy_objects.first().expect("energy object");
        let mut mesh = object_mesh_data(object, Some(geometry));
        for vertex in mesh.positions.chunks_exact_mut(3) {
            vertex[2] = 0.0;
        }
        align_mesh_to_fixture_centroid(&mut mesh, geometry, &object.primitives);
        let min_z = mesh
            .positions
            .chunks_exact(3)
            .map(|vertex| vertex[2])
            .fold(f32::INFINITY, f32::min);
        assert!(min_z > 2.5, "aligned mesh min z {min_z}");
    }

    #[test]
    fn forest_surface_meshes_use_authored_height_without_pane_geometry() {
        let scene = forest_play_scene();
        let energy = scene.energy_objects.first().expect("energy object");
        let energy_mesh = object_mesh_data(energy, None);
        let energy_min_z = energy_mesh
            .positions
            .chunks_exact(3)
            .map(|vertex| vertex[2])
            .fold(f32::INFINITY, f32::min);
        assert!(
            energy_min_z > 2.5,
            "energy mesh must stay at authored z without pane geometry, got min_z={energy_min_z}"
        );
        let slab = scene
            .structure_classic_objects
            .iter()
            .find(|object| object.primitives.iter().any(|primitive| primitive.kind == "surface"))
            .expect("structure surface");
        let slab_mesh = object_mesh_data(slab, None);
        let slab_min_z = slab_mesh
            .positions
            .chunks_exact(3)
            .map(|vertex| vertex[2])
            .fold(f32::INFINITY, f32::min);
        assert!(
            slab_min_z > 2.5,
            "structure slab must stay at authored z without pane geometry, got min_z={slab_min_z}"
        );
    }

    #[test]
    fn cad_document_schema_matches_domain() {
        let scene = empty_cad_projection();
        assert_eq!(scene.schema, CAD_PLAY_DOCUMENT_SCHEMA);
    }

    #[test]
    fn default_example_and_forest_scene_parse_as_projections() {
        let default_json = serde_json::to_string(&default_document()).unwrap();
        let default_scene: CadScene = serde_json::from_str(&default_json).unwrap();
        assert!(!default_scene.objects.is_empty());
        let forest_json = serde_json::to_string(&forest_play_scene()).unwrap();
        let forest_scene: CadScene = serde_json::from_str(&forest_json).unwrap();
        assert!(!forest_scene.building_objects.is_empty());
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️Render
    #[test]
    fn renders_world_scene_for_each_pane() {
        let app = CadPlayApp::default();
        let scene = forest_play_scene();
        let history = empty_history();
        let doc = DocumentView { projection: &scene, history: &history };
        for body_key in [
            CAD_PLAY_BODY_SHAPE,
            CAD_PLAY_BODY_BUILDING,
            CAD_PLAY_BODY_ENERGY,
            CAD_PLAY_BODY_STRUCTURE_CLASSIC,
        ] {
            let node = render_direct(&app, body_key, &doc, &CadConfig::default());
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("world-3d"), "body {body_key} should render a world-3d scene");
        }
    }

    #[test]
    fn document_lists_objects_and_nodes() {
        let mut app = new_app();
        let node = app.render(CAD_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("cad-object:"));
        assert!(json.contains("cad-node:"));
    }

    #[test]
    fn document_tree_shows_name_with_kind_as_secondary_label() {
        let app = CadPlayApp::default();
        let mut scene = default_document();
        scene.objects[0].label = "U2".into();
        scene.objects[0].typology = "building.building.beam".into();
        let history = empty_history();
        let doc = DocumentView { projection: &scene, history: &history };
        let node = render_direct(&app, CAD_PLAY_BODY_DOCUMENT, &doc, &CadConfig::default());
        let UiNode::Tree(tree) = node else {
            panic!("document body should render a tree");
        };
        let object_item = tree
            .sections
            .iter()
            .flat_map(|section| section.items.iter())
            .find(|item| item.id.contains("cad-object:") && item.label == "U2")
            .expect("named object tree item");
        assert_eq!(object_item.description.as_deref(), Some("Beam"));

        let de_node = render_direct(
            &app,
            CAD_PLAY_BODY_DOCUMENT,
            &doc,
            &CadConfig { locale: "de".into(), ..CadConfig::default() },
        );
        let UiNode::Tree(de_tree) = de_node else {
            panic!("document body should render a tree");
        };
        let de_object_item = de_tree
            .sections
            .iter()
            .flat_map(|section| section.items.iter())
            .find(|item| item.id.contains("cad-object:") && item.label == "U2")
            .expect("named object tree item in German");
        assert_eq!(de_object_item.description.as_deref(), Some("Träger"));
    }

    #[test]
    fn document_tree_includes_primitive_children() {
        let mut app = new_app();
        let node = app.render(CAD_PLAY_BODY_DOCUMENT, None, &ViewState::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("cad-primitive:"));
        assert!(json.contains("hoverAction"));
    }

    #[test]
    fn app_definition_declares_one_window_scoped_dislocate_utility() {
        let definition = create_cad_app().definition;
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, vec![CAD_DISLOCATE_UTILITY_ID]);
        // 🧰️ The framework auto-injects `setActiveUtility` as a View action once utilities are declared —
        // cad must NOT also declare it as an Operation.
        let set_active_utility = definition.actions.iter().find(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID).expect("setActiveUtility auto-injected");
        assert_eq!(set_active_utility.kind, ActionKind::View);
        // 🚦️ Transform utilities gate the action panel while active (the default) — cad declares no
        // passive `allows_actions_while_active` view utilities.
        assert!(definition.utilities.iter().all(|utility| !utility.allows_actions_while_active));
        // 🧭️ Every world-3d pane owns its own Dislocate utility activation.
        for window in &definition.window_kinds {
            let refs: Vec<&str> = window.utilities.iter().map(|utility_ref| utility_ref.as_str()).collect();
            assert_eq!(refs, vec![CAD_DISLOCATE_UTILITY_ID], "window {} utilities", window.id);
        }
    }

    #[test]
    fn internal_and_plumbing_actions_excluded_from_palette() {
        let definition = create_cad_app().definition;
        let hidden_actions = [
            "patchCadPlayReference",
            "engagementSubmit",
            "setSelection",
            "setNodeSelection",
            "worldSelect",
            "worldHover",
            "setHover",
            "worldPick",
            "setSelectionMethod",
            "setReferenceSelection",
            "referenceHover",
            "engagementInput",
            "engagementPossibleSelect",
            "engagementRepeatLast",
            "engagementAbort",
            "worldPointerDown",
            "worldPointerMove",
            "engagementPointerDown",
            "setPrimitiveSelection",
            "setDislocateOption",
        ];
        for action_id in hidden_actions {
            let action = definition
                .actions
                .iter()
                .find(|entry| entry.id == action_id)
                .unwrap_or_else(|| panic!("action {action_id} missing from manifest"));
            assert!(!action.in_palette, "internal action {action_id} must have in_palette: false");
        }

        let palette_user_actions = [
            "addObject",
            "deleteObject",
            "duplicateObject",
            "translateSelection",
            "rotateSelection",
            "scaleSelection",
        ];
        for action_id in palette_user_actions {
            let action = definition
                .actions
                .iter()
                .find(|entry| entry.id == action_id)
                .unwrap_or_else(|| panic!("user action {action_id} missing from manifest"));
            assert!(action.in_palette, "user action {action_id} must have in_palette: true");
        }
    }

    #[test]
    fn engagement_input_and_possible_engagements_present() {
        let mut app = new_app();
        let engagements = app.window_engagements();
        let shape = engagements.get(CAD_PLAY_WINDOW_SHAPE).expect("shape engagement");
        assert!(shape.input.is_some());
        assert!(shape.possible_engagements.as_ref().is_some_and(|rows| !rows.is_empty()));
    }

    #[test]
    fn window_engagements_registered_for_all_four_panes() {
        let mut app = new_app();
        let engagements = app.window_engagements();
        for window_kind in [
            CAD_PLAY_WINDOW_SHAPE,
            CAD_PLAY_WINDOW_BUILDING,
            CAD_PLAY_WINDOW_ENERGY,
            CAD_PLAY_WINDOW_STRUCTURE_CLASSIC,
        ] {
            assert!(engagements.contains_key(window_kind), "missing engagement for {window_kind}");
        }
    }

    #[test]
    fn forest_example_includes_reference_overlay() {
        let scene = forest_play_scene();
        let references = world_references_json(&scene, CadPaneId::Shape).expect("references");
        assert!(references.contains("ref-concrete-forest"));
    }

    #[test]
    fn typology_extent_derives_from_authored_geometry() {
        let scene = forest_play_scene();
        let column = scene
            .building_objects
            .iter()
            .find(|object| object.typology == "building.building.column")
            .expect("column object");
        let extent = column.extent.expect("column extent derived from geometry");
        assert!(extent[2] > 0.05, "authored column height should be measurable");
        assert_ne!(extent, CAD_DEFAULT_TYPOLOGY_EXTENT, "should differ from the universal fallback");
    }
    //#endregion 🔖️Render

    //#region 🔖️ViewState
    #[test]
    fn gumball_fields_present_when_selection_active() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let emit = drive(&app, &scene, "setSelection", Some(json!({ "objectIds": ["object-box-1"] })));
        let runtime = runtime_after(&emit, &CadConfig::default());
        let selection = world_selection_json(
            &scene,
            &runtime,
            Some(CAD_DISLOCATE_UTILITY_ID),
            CadDislocateOptions::default(),
        );
        assert!(selection.contains("\"transformMode\":\"transform\""));
        assert!(selection.contains("\"moveAxes\":true"));
        assert!(selection.contains("\"rotate\":true"));
        assert!(selection.contains("\"scaleAxes\":false"));
        assert!(selection.contains("\"gumballActive\":true"));
        assert!(selection.contains("\"gumballTarget\""));
    }

    /// 🎥️ `setCamera`/`setProjection`/`setProjectionParam` are `ActionKind::View` (see the `.view_action`
    /// registrations below) — they must never emit a `CadOperation` (no VCS edit, no undo entry) and
    /// instead write a coalesced `CadConfigOperation`, isolated per pane.
    #[test]
    fn set_camera_writes_config_not_operations() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let emit = drive(
            &app,
            &scene,
            "setCamera",
            Some(json!({ "surfaceId": "cad.play.scene3d/building", "camera": { "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0], "zoom": 2.0, "fov": 60.0 } })),
        );
        assert!(emit.document_operations.is_empty(), "setCamera must not emit a VCS operation");
        assert!(!emit.config_operations.is_empty(), "setCamera must write a config operation");
        let runtime = runtime_after(&emit, &CadConfig::default());
        assert_eq!(cad_pane_camera_runtime(&runtime, CadPaneId::Building).zoom, 2.0);
        assert_eq!(cad_pane_camera_runtime(&runtime, CadPaneId::Shape).zoom, 1.0, "panes stay isolated");
    }

    #[test]
    fn gumball_inactive_without_selection() {
        let selection = world_selection_json(
            &default_document(),
            &CadPlayRuntime::default(),
            Some(CAD_DISLOCATE_UTILITY_ID),
            CadDislocateOptions::default(),
        );
        assert!(selection.contains("\"gumballActive\":false"));
        assert!(!selection.contains("\"gumballTarget\""));
    }

    #[test]
    fn active_utility_flows_from_config_into_scene() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let history = empty_history();
        let doc = DocumentView { projection: &scene, history: &history };
        let config = CadConfig { active_utility_id: CAD_DISLOCATE_UTILITY_ID.into(), ..CadConfig::default() };
        let node = render_direct(&app, CAD_PLAY_BODY_SHAPE, &doc, &config);
        let json = serde_json::to_string(&node).unwrap();
        // The world selection blob is embedded as an escaped JSON string inside the scene node.
        assert!(json.contains(r#"transformMode\":\"transform"#), "render sources Dislocate from CadConfig::active_utility_id");
    }

    /// @emoji 🎯️ WORKFLOWS-END-TO-END-TYPED-PORTS: `active_utility_id` is now a single, global
    /// `CadConfig` field (the pre-B1 per-window-instance `ViewState.active_utility_by_window_id` has no
    /// replacement — `render`/`window_measures` have no per-instance parameter anymore, see
    /// `CadDislocateOptions`'s doc comment in `cad_document_engine`) — so the gumball is active in
    /// EVERY pane with an active selection once the Dislocate utility is on, not isolated per window.
    #[test]
    fn dislocate_gumball_is_visible_in_every_pane_once_the_utility_is_active() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let base_config = CadConfig { active_utility_id: CAD_DISLOCATE_UTILITY_ID.into(), ..CadConfig::default() };
        let emit = drive_with_config(&app, &scene, "setSelection", Some(json!({ "objectIds": ["object-box-1"] })), &base_config);
        let config = config_after(&emit, &base_config);
        let history = empty_history();
        let doc = DocumentView { projection: &scene, history: &history };
        let shape = render_direct(&app, CAD_PLAY_BODY_SHAPE, &doc, &config);
        let building = render_direct(&app, CAD_PLAY_BODY_BUILDING, &doc, &config);
        let shape_json = serde_json::to_string(&shape).unwrap();
        let building_json = serde_json::to_string(&building).unwrap();
        assert!(shape_json.contains(r#"gumballActive\":true"#));
        assert!(shape_json.contains(r#"transformMode\":\"transform"#));
        assert!(building_json.contains(r#"gumballActive\":true"#));
        assert!(building_json.contains(r#"transformMode\":\"transform"#));
    }

    #[test]
    fn context_menu_is_selection_gated_and_resolves_labels_from_the_registry() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let history = empty_history();
        let doc = DocumentView { projection: &scene, history: &history };
        let registry = AppActionRegistry::from_definition(&create_cad_app().definition);
        let empty_config = CadConfig::default();

        assert!(context_menu_direct(&app, &doc, &empty_config, &registry).is_empty(), "no selection must fall through to the shell's window-level menu");

        let emit = drive(&app, &scene, "setSelection", Some(json!({ "objectIds": ["object-box-1"] })));
        let config = config_after(&emit, &empty_config);
        let items = context_menu_direct(&app, &doc, &config, &registry);
        assert!(items.iter().any(|item| item.id == "translateSelection" && item.label.is_some()), "labels must resolve from the registry: {items:?}");
        assert!(
            items.iter().any(|item| item.id == "deleteObject" && item.destructive == Some(true)),
            "deleteObject must be marked destructive: {items:?}"
        );
    }

    /// @emoji 🎛️ Dislocate move/rotate options are now keyed by PANE (`CadConfig::dislocate_shape`/
    /// `dislocate_building`/…), not by an arbitrary host-pushed window-instance id — the direct
    /// replacement for the pre-B1 per-window-instance isolation test.
    #[test]
    fn dislocate_move_and_rotate_options_are_per_pane() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let emit = drive(&app, &scene, "setDislocateOption", Some(json!({ "pane": "building", "option": "rotate", "pressed": false })));
        let config = config_after(&emit, &CadConfig::default());
        let history = empty_history();
        let doc = DocumentView { projection: &scene, history: &history };
        let measures = window_measures_direct(&app, &doc, &config);
        let rotate_pressed = |window_id: &str| {
            measures
                .get(window_id)
                .and_then(|items| items.iter().find_map(|measure| match measure {
                    WindowMeasure::Group { id, children, .. } if id == "cad-play-utility-options-dislocate" => {
                        children.iter().find_map(|child| match child {
                            WindowMeasure::Toggle { id, pressed, .. } if id == "cad-dislocate-rotate" => Some(*pressed),
                            _ => None,
                        })
                    }
                    _ => None,
                }))
        };
        assert_eq!(rotate_pressed(CAD_PLAY_WINDOW_SHAPE), Some(true));
        assert_eq!(rotate_pressed(CAD_PLAY_WINDOW_BUILDING), Some(false));
    }

    #[test]
    fn engagement_hud_no_longer_carries_utility_switcher_options() {
        let mut app = new_app();
        let engagements = app.window_engagements();
        for engagement in engagements.values() {
            assert!(engagement.options.is_none(), "utility switching now lives in the framework utility bar, not the engagement HUD");
        }
    }

    #[test]
    fn switching_utility_emits_no_operations_and_no_history_entry() {
        // 🧰️ The key regression guard: switching the host-owned active utility must be a pure View
        // action — zero operations, no projection mutation, and (proven below) no intervening
        // history entry. If the switch recorded an edit, the single undo would revert the switch
        // instead of the preceding addObject.
        let mut app = new_app();
        let before = app.projection().expect("projection").objects.len();
        app.dispatch_typed(CadCommand::AddObject { typology: Some("spatial.shape.primitive.box".into()) }, &meta("local"))
            .expect("add object");
        let projection_after_add = serde_json::to_string(&app.projection().expect("projection")).unwrap();
        let result = app
            .dispatch_typed(CadCommand::SetActiveUtility { utility_id: CAD_DISLOCATE_UTILITY_ID.into() }, &meta("local"))
            .expect("set active utility");
        assert!(result.operations.is_empty(), "utility switch must emit zero operations");
        let projection_after_switch = serde_json::to_string(&app.projection().expect("projection")).unwrap();
        assert_eq!(projection_after_add, projection_after_switch, "utility switch must not mutate the projection");
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert_eq!(
            app.projection().expect("projection").objects.len(),
            before,
            "a single undo reverts the addObject — proving the utility switch created no history entry"
        );
    }

    #[test]
    fn sun_measures_registered_for_all_four_panes_and_default_off() {
        let app = CadPlayApp::default();
        let base_config = CadConfig::default();
        assert!(!base_config.sun.enabled, "sun must be off by default");
        let scene = default_document();
        let history = empty_history();
        let doc = DocumentView { projection: &scene, history: &history };
        let measures = window_measures_direct(&app, &doc, &base_config);
        for window_kind in [
            CAD_PLAY_WINDOW_SHAPE,
            CAD_PLAY_WINDOW_BUILDING,
            CAD_PLAY_WINDOW_ENERGY,
            CAD_PLAY_WINDOW_STRUCTURE_CLASSIC,
        ] {
            assert!(measures.contains_key(window_kind), "missing sun measures for {window_kind}");
        }
        let emit = drive(&app, &scene, "toggleSun", None);
        let runtime = runtime_after(&emit, &base_config);
        assert!(runtime.sun.enabled);
    }

    #[test]
    fn world_pick_selects_visible_object_by_index() {
        // The Shape pane's fixture object is a single hexagonal-cut solid (one object), so this
        // exercises worldPick-by-index against the Building pane, which has multiple objects.
        let app = CadPlayApp::default();
        let scene = forest_play_scene();
        let building_visible: Vec<_> = scene.building_objects.iter().filter(|object| object.visible).collect();
        assert!(building_visible.len() > 1);
        let expected_id = building_visible[1].id.clone();
        let emit = drive(
            &app,
            &scene,
            "worldPick",
            Some(json!({ "surfaceId": "cad.play.scene3d/building", "id": 1, "merge": "replace" })),
        );
        let runtime = runtime_after(&emit, &CadConfig::default());
        assert_eq!(runtime.selected_object_ids.to_vec(), vec![expected_id]);
        assert_eq!(runtime.component_selection.mode, "mesh");
    }

    #[test]
    fn set_hover_edge_round_trips_hovered_component() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let object_id = scene.objects.iter().find(|object| object.visible).expect("visible").id.clone();
        let emit = drive(
            &app,
            &scene,
            "setHover",
            Some(json!({ "objectId": object_id, "mode": "edge", "id": 3 })),
        );
        let runtime = runtime_after(&emit, &CadConfig::default());
        let selection = world_selection_json(&scene, &runtime, None, CadDislocateOptions::default());
        assert!(selection.contains("\"hoveredComponent\""));
        assert!(selection.contains("\"mode\":\"edge\""));
        assert!(selection.contains("\"id\":3"));
        assert!(selection.contains("\"edge\":true"), "edge targets must stay enabled: {selection}");
        let instances = world_instances_json(&scene.objects, &runtime);
        assert!(
            instances.contains("\"hovered\":false"),
            "edge hover must not tint the whole mesh surface: {instances}"
        );
    }

    #[test]
    fn world_pick_edge_selects_component_and_emits_selection_mode() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let object_id = scene.objects.iter().find(|object| object.visible).expect("visible").id.clone();
        let emit = drive(
            &app,
            &scene,
            "worldPick",
            Some(json!({
                "granularity": "edge",
                "id": 7,
                "objectId": object_id,
                "merge": "replace"
            })),
        );
        let runtime = runtime_after(&emit, &CadConfig::default());
        assert_eq!(runtime.component_selection.mode, "edge");
        assert_eq!(runtime.component_selection.ids, vec![7]);
        assert_eq!(runtime.active_object_id.as_deref(), Some(object_id.as_str()));
        assert!(runtime.selected_object_ids.contains(&object_id));
        let selection = world_selection_json(&scene, &runtime, None, CadDislocateOptions::default());
        assert!(selection.contains("\"selectionMode\":\"edge\""));
        assert!(selection.contains("\"componentIds\":[7]"));
        assert!(selection.contains(&format!("\"activeObjectId\":\"{object_id}\"")));
    }

    #[test]
    fn world_pick_curve_centerline_selects_whole_object() {
        let app = CadPlayApp::default();
        let scene = forest_play_scene();
        let curve = scene
            .structure_classic_objects
            .iter()
            .find(|object| object.visible && primary_primitive_kind(object) == "curve")
            .expect("structure classic curve object");
        let object_id = curve.id.clone();
        let emit = drive(
            &app,
            &scene,
            "worldPick",
            Some(json!({
                "granularity": "edge",
                "id": 0,
                "objectId": object_id,
                "merge": "replace"
            })),
        );
        let config_after_pick = config_after(&emit, &CadConfig::default());
        let runtime = cad_runtime_from_config(&config_after_pick);
        assert_eq!(runtime.selected_object_ids.to_vec(), vec![object_id.clone()]);
        assert_eq!(runtime.active_object_id.as_deref(), Some(object_id.as_str()));
        assert_eq!(runtime.component_selection.mode, "mesh");
        assert!(runtime.component_selection.ids.is_empty());
        let emit = drive_with_config(
            &app,
            &scene,
            "setHover",
            Some(json!({ "objectId": object_id, "mode": "edge", "id": 0 })),
            &config_after_pick,
        );
        let runtime = runtime_after(&emit, &config_after_pick);
        assert_eq!(
            runtime.hovered_target.as_ref().and_then(|target| target.mode.as_deref()),
            Some("mesh"),
            "curve hover must promote to instance mesh hover"
        );
        let instances = world_instances_json(&scene.structure_classic_objects, &runtime);
        assert!(
            instances.contains(&format!("\"id\":\"{object_id}\"")) && instances.contains("\"hovered\":true"),
            "curve instance must show hovered: {instances}"
        );
    }

    #[test]
    fn document_tree_reflects_viewport_selection() {
        let scene = forest_play_scene();
        let object_id = scene.objects.iter().find(|object| object.visible).expect("visible shape object").id.clone();
        let runtime = CadPlayRuntime {
            selected_object_ids: SelectionSet::from(vec![object_id.clone()]),
            hovered_object_id: Some(object_id.clone()),
            ..CadPlayRuntime::default()
        };
        let selected = document_tree_selected_ids(&scene, &runtime).expect("selected");
        assert!(selected.iter().any(|id| id.contains(&object_id) && id.starts_with("cad-object:shape:")));
        let highlighted = document_tree_highlighted_ids(&scene, &runtime).expect("highlighted");
        assert!(highlighted.iter().any(|id| id.contains(&object_id) && id.starts_with("cad-object:shape:")));
    }
    //#endregion 🔖️ViewState

    //#region 🔖️Terminology
    #[test]
    fn multi_selection_inspector_shows_mixed_values() {
        let mut scene = default_document();
        let second = make_object_for_typology("spatial.shape.primitive.box", 1, CadPaneId::Shape);
        let second_id = second.id.clone();
        scene.objects.push(second);
        scene.objects[0].label = "Alpha".into();
        scene.objects[1].label = "Beta".into();
        scene.objects[0].orientation = Some([0.0, 0.0, 0.0, 1.0]);
        scene.objects[1].orientation = Some([0.0, 0.707, 0.0, 0.707]);
        let runtime = CadPlayRuntime {
            selected_object_ids: SelectionSet::from(vec!["object-box-1".into(), second_id]),
            ..CadPlayRuntime::default()
        };
        let panel = build_properties_panel(&view(scene, runtime), cad_labels(&CadConfig::default()), None);
        let json = serde_json::to_string(&panel).unwrap();
        assert!(json.contains("Mixed"));
        assert!(json.contains("cad-play-inspector.object.orientation"));
    }

    fn selected_box_panel(config: &CadConfig) -> String {
        let runtime = CadPlayRuntime {
            selected_object_ids: SelectionSet::from(vec!["object-box-1".into()]),
            ..CadPlayRuntime::default()
        };
        let panel = build_properties_panel(&view(default_document(), runtime), cad_labels(config), None);
        serde_json::to_string(&panel).unwrap()
    }

    #[test]
    fn cad_labels_resolve_native_by_default() {
        let json = selected_box_panel(&CadConfig::default());
        assert!(json.contains("\"Object\""));
        assert!(!json.contains("Building component"));
    }

    #[test]
    fn cad_labels_resolve_reuse_terminology_in_english() {
        let config = CadConfig { terminology: "reuse".into(), locale: "en".into(), ..CadConfig::default() };
        let json = selected_box_panel(&config);
        assert!(json.contains("Building component"));
        assert!(!json.contains("\"Object\""));
    }

    #[test]
    fn cad_labels_resolve_reuse_terminology_in_german() {
        let config = CadConfig { terminology: "reuse".into(), locale: "de".into(), ..CadConfig::default() };
        assert!(selected_box_panel(&config).contains("Baukomponente"));
    }

    #[test]
    fn cad_labels_resolve_native_terminology_in_german() {
        let config = CadConfig { terminology: "native".into(), locale: "de".into(), ..CadConfig::default() };
        assert!(selected_box_panel(&config).contains("\"Objekt\""));
    }

    #[test]
    fn cad_labels_resolve_reuse_terminology_for_primitive() {
        let runtime = CadPlayRuntime {
            selected_object_ids: SelectionSet::from(vec!["object-box-1".into()]),
            selected_primitive_id: Some("box-solid".into()),
            ..CadPlayRuntime::default()
        };
        let config = CadConfig { terminology: "reuse".into(), locale: "de".into(), ..CadConfig::default() };
        let panel = build_properties_panel(&view(default_document(), runtime), cad_labels(&config), None);
        assert!(serde_json::to_string(&panel).unwrap().contains("Bauteil"));
    }

    #[test]
    fn cad_labels_translate_document_tree_panes_in_german() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let history = empty_history();
        let doc = DocumentView { projection: &scene, history: &history };
        let config = CadConfig { locale: "de".into(), ..CadConfig::default() };
        let node = render_direct(&app, CAD_PLAY_BODY_DOCUMENT, &doc, &config);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("\"Form\""));
        assert!(json.contains("Gebäude"));
        assert!(json.contains("Energie"));
        assert!(json.contains("Tragwerk Klassisch"));
        assert!(json.contains("Referenzen"));
        assert!(json.contains("\"Knoten\""));
        assert!(!json.contains("\"Shape\""));
        assert!(!json.contains("Struktur Klassisch"));
    }

    #[test]
    fn cad_labels_translate_catalogue_typologies_in_german() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let history = empty_history();
        let doc = DocumentView { projection: &scene, history: &history };
        let config = CadConfig { locale: "de".into(), ..CadConfig::default() };
        let node = render_direct(&app, CAD_PLAY_BODY_CATALOGUE, &doc, &config);
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("Typologien"));
        assert!(json.contains("Quader"));
        assert!(json.contains("Platte"));
        assert!(json.contains("Stütze"));
        assert!(json.contains("Träger"));
        assert!(json.contains("Wand"));
        assert!(json.contains("Außenwand"));
        assert!(!json.contains("\"Slab\""));
        assert!(!json.contains("\"Balken\""));
    }
    //#endregion 🔖️Terminology

    //#region 🔖️Operations
    #[test]
    fn add_object_action_appends_object_and_selects_it() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let emit = drive(&app, &scene, "addObject", Some(json!({ "typology": "building.building.column" })));
        assert_eq!(emit.document_operations.len(), 1);
        let next = apply_operations(&scene, &emit.document_operations);
        assert!(
            next.objects.iter().any(|object| object.typology == "building.building.column")
                || next.building_objects.iter().any(|object| object.typology == "building.building.column")
        );
        let runtime = runtime_after(&emit, &CadConfig::default());
        assert_eq!(runtime.selected_object_ids.len(), 1);
    }

    #[test]
    fn add_object_through_wrapper_grows_projection() {
        let mut app = new_app();
        let before = app.projection().expect("projection").objects.len();
        app.dispatch_typed(CadCommand::AddObject { typology: Some("spatial.shape.primitive.box".into()) }, &meta("local"))
            .expect("add object");
        assert_eq!(app.projection().expect("projection").objects.len(), before + 1);
    }

    #[test]
    fn focus_model_definition_emits_document_operation() {
        let mut app = new_app();
        app.dispatch_typed(CadCommand::FocusModelDefinition { model_definition_id: "aec.building".into() }, &meta("local"))
            .expect("focus model definition");
        assert_eq!(app.projection().expect("projection").active_model_definition_id, "aec.building");
    }

    #[test]
    fn derive_transformation_populates_energy_pane() {
        let app = CadPlayApp::default();
        let mut scene = default_document();
        scene.objects = vec![make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape)];
        let emit = drive(&app, &scene, "applyTransformation", Some(json!({ "qid": "spatial.shape.from_geometry" })));
        assert!(!emit.document_operations.is_empty());
        let next = apply_operations(&scene, &emit.document_operations);
        assert!(!next.energy_objects.is_empty());
        assert!(next.energy_objects.iter().any(|object| object.typology.starts_with("energy.energy.")));
        assert_eq!(next.active_model_definition_id, "aec.building.energy");
    }

    #[test]
    fn forest_transformation_uses_live_shape_pane() {
        let app = CadPlayApp::default();
        let mut scene = forest_play_scene();
        let fixture_energy_ids: Vec<String> = scene.energy_objects.iter().map(|object| object.id.clone()).collect();
        assert!(!fixture_energy_ids.is_empty(), "forest fixture should have energy objects");
        scene.energy_objects.clear();
        scene.objects.truncate(1);
        scene.objects[0].typology = "spatial.shape.primitive.box".into();
        scene.objects[0].label = "live-shape-only".into();
        let emit = drive(&app, &scene, "applyTransformation", Some(json!({ "qid": "spatial.shape.from_geometry" })));
        let next = apply_operations(&scene, &emit.document_operations);
        assert!(!next.energy_objects.is_empty());
        assert!(
            next.energy_objects.iter().all(|object| !fixture_energy_ids.contains(&object.id)),
            "live single-box derive should not repopulate the static forest energy fixture's original objects"
        );
    }

    #[test]
    fn save_selected_emits_download_effect() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let config = CadConfig { selected_object_ids: vec!["object-box-1".into()], ..CadConfig::default() };
        let emit = drive_with_config(&app, &scene, "saveSelected", None, &config);
        assert!(emit.document_operations.is_empty(), "export must not mutate the document");
        assert_eq!(emit.effects.len(), 1);
        match &emit.effects[0] {
            HostEffect::DownloadMediaExport { filename, data, .. } => {
                assert_eq!(filename, "cad.selected.spatial.dsl");
                assert!(data.contains("activeModelDefinitionId"));
            }
            other => panic!("expected DownloadMediaExport, got {other:?}"),
        }
    }

    #[test]
    fn load_raw_request_emits_file_open_effect() {
        let app = CadPlayApp::default();
        let emit = drive(&app, &default_document(), "loadRawRequest", None);
        match &emit.effects[0] {
            HostEffect::RequestFileOpen { import_action, read_as, .. } => {
                assert_eq!(import_action, "importCadFile");
                assert_eq!(read_as.as_deref(), Some("dataUrl"));
            }
            other => panic!("expected RequestFileOpen, got {other:?}"),
        }
    }
    //#endregion 🔖️Operations

    //#region 🔖️Engagement
    #[test]
    fn engagement_starts_box_interaction_session() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
        let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
        let runtime = runtime_after(&emit, &config);
        assert!(runtime.engagement_session.is_some());
    }

    #[test]
    fn world_pointer_move_updates_live_preview_without_committing_or_emitting_operations() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
        let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
        let config = config_after(&emit, &config);

        let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [3.0, 4.0, 0.0] })), &config);
        assert!(emit.document_operations.is_empty(), "a pointer move must not emit any document operation");
        let runtime = runtime_after(&emit, &config);
        let session = runtime.engagement_session.as_ref().expect("session still active");
        assert_eq!(session.state, "first_corner", "pointer.move must not change state");
        assert_eq!(session.context.get("cursor"), Some(&json!([3.0, 4.0, 0.0])));
    }

    //#region 🔖️GesturePreview
    /// 🔬️ CW7 preview-law seam: `CadPlayApp::gesture_preview` reads `CadEngagementSession` only, never
    /// `CadScene`/`CadOperation` — driven through the real `worldPointerMove` handler (the natural
    /// per-tick gesture handler) via the existing `drive` helper, config threaded explicitly across
    /// calls (the pure `CadPlayApp` no longer holds any of this state itself).
    #[test]
    fn gesture_preview_is_none_without_a_live_engagement_session() {
        let app = CadPlayApp::default();
        assert!(app.gesture_preview(&CadConfig::default()).is_none(), "no live engagement session, nothing to preview");
    }

    #[test]
    fn gesture_preview_reflects_the_live_rubber_band_preview_and_clears_on_abort() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
        let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
        let config = config_after(&emit, &config);

        let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [3.0, 4.0, 0.0] })), &config);
        let config = config_after(&emit, &config);
        let (key, seq_after_first, payload) = app.gesture_preview(&config).expect("a live engagement session is previewable");
        assert_eq!(key, "gesture:engagement");
        let value: Value = serde_json::from_slice(&payload).expect("payload is valid json");
        assert_eq!(value["context"]["cursor"], json!([3.0, 4.0, 0.0]));

        let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [5.0, 6.0, 0.0] })), &config);
        let config = config_after(&emit, &config);
        let (_, seq_after_second, payload_after_second) = app.gesture_preview(&config).expect("still live mid-gesture");
        assert!(seq_after_second > seq_after_first, "seq is monotone per tick, for staleness detection on the receiving end");
        let value_after_second: Value = serde_json::from_slice(&payload_after_second).expect("payload is valid json");
        assert_eq!(value_after_second["context"]["cursor"], json!([5.0, 6.0, 0.0]), "preview tracks the live cursor, not the gesture start");

        let emit = drive_with_config(&app, &scene, "engagementAbort", None, &config);
        let config = config_after(&emit, &config);
        assert!(app.gesture_preview(&config).is_none(), "the engagement session was aborted: nothing left to preview");
    }

    #[test]
    fn gesture_preview_is_a_pure_read_never_mutating_the_engagement_session() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
        let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
        let config = config_after(&emit, &config);
        let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [1.0, 2.0, 0.0] })), &config);
        let config = config_after(&emit, &config);
        let session_before = config.engagement_session_json.clone();
        let _ = app.gesture_preview(&config);
        let _ = app.gesture_preview(&config);
        assert_eq!(config.engagement_session_json, session_before, "gesture_preview must never mutate the live engagement session it reads");
    }
    //#endregion 🔖️GesturePreview

    #[test]
    fn engagement_repeat_last_restarts_the_last_finalized_interaction() {
        let app = CadPlayApp::default();
        let mut scene = default_document();
        let mut config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
        let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
        config = config_after(&emit, &config);
        assert!(cad_runtime_from_config(&config).engagement_session.is_some());

        // 🔣️box.json's default boxMode is "point" (length/width prompt); select diagonal mode (key
        // "d") to reach the classic two-corner-click flow.
        config.engagement_input = "d".into();
        let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
        config = config_after(&emit, &config);

        for position in [json!([0.0, 0.0, 0.0]), json!([2.0, 3.0, 0.0])] {
            let emit = drive_with_config(&app, &scene, "worldPointerDown", Some(json!({ "pane": "shape", "position": position })), &config);
            scene = apply_operations(&scene, &emit.document_operations);
            config = config_after(&emit, &config);
        }

        config.engagement_input = "SetHeight2.5".into();
        let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
        config = config_after(&emit, &config);

        // 🔣️box.json's `set.height` only records the height (state stays first_corner_height); an
        // explicit `confirm` (Enter) is needed to reach `ready`, box's commit.fromStates.
        config.engagement_input = "Confirm".into();
        let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
        scene = apply_operations(&scene, &emit.document_operations);
        config = config_after(&emit, &config);
        let runtime = cad_runtime_from_config(&config);
        assert!(runtime.engagement_session.is_none(), "box should have committed");
        assert_eq!(runtime.last_finalized_interaction_id.as_deref(), Some("primitive.box"));

        let emit = drive_with_config(&app, &scene, "engagementRepeatLast", Some(json!({ "pane": "shape" })), &config);
        let runtime = runtime_after(&emit, &config);
        let session = runtime.engagement_session.as_ref().expect("repeat-last should start a session");
        assert_eq!(session.interaction_id, "primitive.box");
    }
    //#endregion 🔖️Engagement

    //#region 🔖️Import
    #[test]
    fn import_spatial_modelspace_round_trips() {
        let payload = json!({
            "schema": "spatial.modelspace",
            "revision": 1,
            "activeModelDefinitionId": "spatial.shape",
            "models": [{
                "id": "spatial.shape",
                "model": {
                    "schema": "spatial.model",
                    "revision": 1,
                    "objects": [{
                        "id": "object-imported",
                        "label": "Imported",
                        "typology": "spatial.shape.primitive.box",
                        "visible": true,
                        "locked": false,
                        "origin": [1.0, 2.0, 3.0],
                        "primitives": []
                    }]
                }
            }]
        });
        let scene = scene_from_spatial_payload(&payload).expect("scene");
        assert_eq!(scene.objects.len(), 1);
        assert_eq!(scene.objects[0].id, "object-imported");
    }

    #[test]
    fn import_cad_file_action_accepts_spatial_json_text_string_payload() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let file_text = json!({
            "schema": "spatial.model",
            "revision": 1,
            "modelDefinitionId": "spatial.shape",
            "objects": [{
                "id": "object-loaded",
                "label": "Loaded",
                "typology": "spatial.shape.primitive.box",
                "visible": true,
                "locked": false,
                "origin": [1.0, 2.0, 3.0],
                "primitives": []
            }]
        })
        .to_string();
        let emit = drive(
            &app,
            &scene,
            "importCadFile",
            Some(json!({ "payload": file_text, "name": "cad.spatial.json" })),
        );
        assert!(!emit.document_operations.is_empty(), "importCadFile must emit a SetScene operation for a spatial JSON string payload");
        let next = apply_operations(&scene, &emit.document_operations);
        assert_eq!(next.objects.len(), 1);
        assert_eq!(next.objects[0].id, "object-loaded");
    }

    #[test]
    fn import_cad_file_action_imports_obj_by_extension() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let obj_text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";
        let obj_data_url = format!(
            "data:model/obj;base64,{}",
            base64::engine::general_purpose::STANDARD.encode(obj_text)
        );
        let emit = drive(
            &app,
            &scene,
            "importCadFile",
            Some(json!({ "payload": obj_data_url, "name": "triangle.obj" })),
        );
        assert!(!emit.document_operations.is_empty(), "importCadFile must emit an AddObject operation for an OBJ payload");
        let next = apply_operations(&scene, &emit.document_operations);
        assert_eq!(next.objects.len(), scene.objects.len() + 1);
        assert!(next.objects.last().unwrap().solid_handle.is_some());
    }
    //#endregion 🔖️Import

    //#region 🔖️History
    #[test]
    fn undo_redo_round_trips_added_object_through_wrapper() {
        let mut app = new_app();
        let before = app.projection().expect("projection").objects.len();
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(
            &mut app,
            CadCommand::AddObject { typology: Some("spatial.shape.primitive.box".into()) },
            |app| app.projection().expect("projection").objects.len(),
            before,
            before + 1,
        );
    }

    #[test]
    fn undo_redo_round_trips_added_node_through_wrapper() {
        let mut app = new_app();
        let before = app.projection().expect("projection").nodes.len();
        app.dispatch_typed(CadCommand::AddNode { kind: "solid".into() }, &meta("local")).expect("add node");
        assert_eq!(app.projection().expect("projection").nodes.len(), before + 1);
        let undo = app.handle_action("undo", None, &meta("local")).expect("undo");
        assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(app.projection().expect("projection").nodes.len(), before);
        app.handle_action("redo", None, &meta("local")).expect("redo");
        assert_eq!(app.projection().expect("projection").nodes.len(), before + 1);
    }

    #[test]
    fn coalesced_translate_drag_is_a_single_undo_step() {
        let mut app = new_app();
        app.dispatch_typed(CadCommand::AddObject { typology: Some("spatial.shape.primitive.box".into()) }, &meta("local"))
            .expect("add object");
        let object_id = app.projection().expect("projection").objects.last().unwrap().id.clone();
        let origin_before = app
            .projection()
            .expect("projection")
            .objects
            .iter()
            .find(|object| object.id == object_id)
            .unwrap()
            .origin;
        for _ in 0..3 {
            app.dispatch_typed(
                CadCommand::TranslateSelection { object_ids: vec![object_id.clone()], dx: 1.0, dy: 0.0, dz: 0.0 },
                &meta("local"),
            )
            .expect("translate tick");
        }
        let dragged = app
            .projection()
            .expect("projection")
            .objects
            .iter()
            .find(|object| object.id == object_id)
            .unwrap()
            .origin;
        assert_eq!(dragged[0], origin_before[0] + 3.0, "three coalesced ticks accumulate");
        // One undo reverts the whole coalesced drag back to the pre-drag origin (not one tick).
        app.handle_action("undo", None, &meta("local")).expect("undo drag");
        let after_undo = app
            .projection()
            .expect("projection")
            .objects
            .iter()
            .find(|object| object.id == object_id)
            .unwrap()
            .origin;
        assert_eq!(after_undo, origin_before, "the coalesced drag undoes as one edit");
    }
    //#endregion 🔖️History

    //#region 🔖️Convergence
    /// 🧪️ The definitional merge proof: two instances start from the SAME base projection, apply
    /// DISJOINT edits (A translates object A, B patches object B's label), and after exchanging operations
    /// over a `MemoryBackbone` both converge to contain BOTH edits — impossible under whole-document
    /// `setDocument` snapshots.
    #[test]
    fn two_instances_converge_disjoint_edits_via_backbone() {
        // A shared two-object base scene loaded identically into both instances.
        let mut base = default_document();
        base.objects = vec![
            make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape),
            make_object_for_typology("spatial.shape.primitive.box", 1, CadPaneId::Shape),
        ];
        let object_a = base.objects[0].id.clone();
        let object_b = base.objects[1].id.clone();
        let base_envelope = store::create_document_envelope::<CadScene, CadOperation>(CAD_DOCUMENT_SCHEMA, "cad-play", base, None);
        let base_files = store::print_document_pack(&base_envelope).expect("print document pack");

        let mut instance_a = new_app();
        let mut instance_b = new_app();
        instance_a.load_document_pack(&base_files).expect("load a");
        instance_b.load_document_pack(&base_files).expect("load b");
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://cad-convergence", "mem://cad-convergence");
        instance_a.attach_backbone(Box::new(backbone_a)).expect("attach a");
        instance_b.attach_backbone(Box::new(backbone_b)).expect("attach b");

        // A renames object A.
        instance_a
            .dispatch_typed(
                CadCommand::PatchObject { object_id: object_a.clone(), field: "label".into(), value: Some("Renamed By A".into()), delta: None },
                &meta("actor-a"),
            )
            .expect("a renames object a");

        // B renames object B — a disjoint edit that must survive alongside A's.
        instance_b
            .dispatch_typed(
                CadCommand::PatchObject { object_id: object_b.clone(), field: "label".into(), value: Some("Renamed By B".into()), delta: None },
                &meta("actor-b"),
            )
            .expect("b renames object b");

        // A neutral history command always pumps inbound operations before doing its own work.
        instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).expect("pump b");

        let scene_a = instance_a.projection().expect("projection a");
        let scene_b = instance_b.projection().expect("projection b");

        let label_a_in_a = scene_a.objects.iter().find(|object| object.id == object_a).unwrap().label.clone();
        let label_a_in_b = scene_b.objects.iter().find(|object| object.id == object_a).unwrap().label.clone();
        let label_b_in_a = scene_a.objects.iter().find(|object| object.id == object_b).unwrap().label.clone();
        let label_b_in_b = scene_b.objects.iter().find(|object| object.id == object_b).unwrap().label.clone();

        assert_eq!(label_a_in_a, "Renamed By A", "instance A keeps its own edit");
        assert_eq!(label_a_in_b, "Renamed By A", "instance B converges on A's edit");
        assert_eq!(label_b_in_a, "Renamed By B", "instance A converges on B's edit");
        assert_eq!(label_b_in_b, "Renamed By B", "instance B keeps its own edit");
    }

    #[test]
    fn ingest_operations_is_idempotent_for_cad() {
        let mut sender = new_app();
        let (near, mut far) = MemoryBackbone::pair("mem://cad-doc", "mem://cad-doc");
        sender.attach_backbone(Box::new(near)).expect("attach");
        sender
            .dispatch_typed(CadCommand::AddNode { kind: "solid".into() }, &meta("local"))
            .expect("add node");

        let mut envelopes = Vec::new();
        for message in far.receive().expect("receive") {
            if let BackboneMessage::Operations { envelopes: operations } = message {
                envelopes.extend(operations);
            }
        }
        assert!(!envelopes.is_empty(), "expected the applied operation to flow onto the channel");
        let operations = protocol::encode_envelopes(&envelopes);

        let mut receiver = new_app();
        let nodes_before = receiver.projection().expect("projection").nodes.len();
        receiver.ingest_operations(&operations).expect("ingest once");
        receiver.ingest_operations(&operations).expect("ingest twice");
        assert_eq!(
            receiver.projection().expect("projection").nodes.len(),
            nodes_before + 1,
            "feeding the same operation twice must not double-apply"
        );
    }
    //#endregion 🔖️Convergence
}
//#endregion 🧪️Tests
