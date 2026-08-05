//! 📐️ CAD play app — the plugin's play app: its `DocumentApp` impl (dispatch-only), the
//! `CadPlayRuntime` scratch mirror of `CadConfig`, the shared view/export helpers its command,
//! panel and window nodes build on, and the manifest that stitches those nodes together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️component.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/<window>`. This file dispatches and stitches.

use crate::apps::cad::commands::camera::{set_camera, set_projection, set_projection_param};
use crate::apps::cad::commands::engagement::{engagement_abort, engagement_input, engagement_possible_select, engagement_repeat_last, engagement_submit, world_pointer_down, world_pointer_move};
use crate::apps::cad::commands::io::{import_cad_file, load_raw_request, save_current, save_in_play, save_selected};
use crate::apps::cad::commands::locale::{set_locale, set_terminology};
use crate::apps::cad::commands::model_definition::{focus_model_definition, set_active_example};
use crate::apps::cad::commands::node::{add_node, rename_node};
use crate::apps::cad::commands::object::{add_object, delete_object, duplicate_object, patch_object, patch_selection};
use crate::apps::cad::commands::reference::{patch_cad_play_reference, reference_hover, set_reference_selection};
use crate::apps::cad::commands::selection::{set_hover, set_node_selection, set_primitive_selection, set_selection, set_selection_method, world_hover, world_pick, world_select};
use crate::apps::cad::commands::sun::{set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use crate::apps::cad::commands::transform::{apply_transformation, rotate_selection, scale_selection, translate_selection};
use crate::apps::cad::commands::utility::{set_active_utility, set_dislocate_option};
use crate::apps::cad::config::{cad_sun_config_from_world, cad_sun_config_to_world, CadComponentSelection, CadConfig, CadConfigOperation, CadDislocateOptions, CadHoverTarget, CadSelectionTargets};
use crate::apps::cad::modes::edit;
use crate::apps::cad::modes::edit::windows::{building, energy, shape, structure_classic};
use crate::apps::cad::panels::{catalogue, document, inspection};
use crate::apps::cad::terminology::{cad_is_de_locale, cad_labels};
use crate::artifacts::cad::engine::interaction::{apply_event, can_commit, commit_object, keyed_transitions, parse_repl_line, resolve_interaction_key, start_session, CadEngagementSession};
use crate::artifacts::cad::engine::transformation::{apply_from_building, apply_typology_fallback, run_derive_from_geometry, solid_for_object};
use crate::artifacts::cad::engine::{
    cad_brep_kernel, cad_camera_projection_config, ensure_object_solid_handle, export_solids_as, forest_play_scene, interaction, next_cad_id, CadSolidExport, CAD_EXAMPLE_FOREST_LEFT, CAD_MODEL_DEFINITION_BUILDING, CAD_MODEL_DEFINITION_ENERGY,
    CAD_MODEL_DEFINITION_SHAPE, CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC,
};
use crate::artifacts::cad::op::{CadObjectPatch, CadOperation};
use crate::artifacts::cad::{artifact_kind, cad_all_objects, cad_find_object_pane, cad_pane_from_model_definition_id, cad_pane_objects, CadCamera, CadObject, CadPaneId, CadScene, CAD_DOCUMENT_SCHEMA};
use base64::Engine as _;
use kernel_3d_engine::{BrepKernel, GeometryHandle};
use semio_framework_core::kernel::HostEffect;
use semio_framework_plugin::{
    tree_item, world3d_camera_projection_json, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, App, AppActionRegistry, ConfigView, ContextMenuItemSpec, ContextMenuRequest, DocumentApp, DocumentView,
    Emit, Fault, IconName, Label, WorldSunConfig, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, Menu, OsMediaFormat, SelectionSet, UiNode, UtilityCategory, UtilityDefinition, WindowEngagement, WindowMeasure,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖️Constants
pub const CAD_PLAY_APP_ID: &str = "cad-play";

pub const CAD_PLAY_CONTROLLER_ID: &str = "cad-play";
pub const CAD_DISLOCATE_UTILITY_ID: &str = "dislocate";

pub const CAD_FALLBACK_MESH_KIND: &str = "box";

pub struct CadTypologyEntry {
    pub typology: &'static str,
    pub label: &'static str,
    pub icon: &'static str,
    pub model_definition_id: &'static str,
}

pub const TYPOLOGY_CATALOG: &[CadTypologyEntry] = &[
    CadTypologyEntry { typology: "spatial.shape.primitive.box", label: "Box", icon: "box", model_definition_id: CAD_MODEL_DEFINITION_SHAPE },
    CadTypologyEntry { typology: "building.building.slab", label: "Slab", icon: "square", model_definition_id: CAD_MODEL_DEFINITION_BUILDING },
    CadTypologyEntry { typology: "building.building.column", label: "Column", icon: "columns", model_definition_id: CAD_MODEL_DEFINITION_BUILDING },
    CadTypologyEntry { typology: "building.building.beam", label: "Beam", icon: "minus", model_definition_id: CAD_MODEL_DEFINITION_BUILDING },
    CadTypologyEntry { typology: "building.building.wall", label: "Wall", icon: "panel-top", model_definition_id: CAD_MODEL_DEFINITION_BUILDING },
    CadTypologyEntry { typology: "energy.energy.externalwall", label: "External Wall", icon: "panel-top", model_definition_id: CAD_MODEL_DEFINITION_ENERGY },
    CadTypologyEntry { typology: "structure.structure.onewayreinforcedconcreteslab", label: "Slab", icon: "square", model_definition_id: CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC },
    CadTypologyEntry { typology: "structure.structure.reinforcedconcretecolumn", label: "Column", icon: "columns", model_definition_id: CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC },
];

pub struct CadTransformationSpec {
    pub id: &'static str,
    pub source_model_definition_id: &'static str,
    pub target_model_definition_id: &'static str,
    pub mode: TransformationMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransformationMode {
    DeriveFromGeometry,
    FromBuilding,
    TypologyFallback,
}

pub const CAD_TRANSFORMATION_SPECS: &[CadTransformationSpec] = &[
    CadTransformationSpec { id: "from_geometry", source_model_definition_id: CAD_MODEL_DEFINITION_SHAPE, target_model_definition_id: CAD_MODEL_DEFINITION_ENERGY, mode: TransformationMode::DeriveFromGeometry },
    CadTransformationSpec { id: "from_building", source_model_definition_id: CAD_MODEL_DEFINITION_BUILDING, target_model_definition_id: CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC, mode: TransformationMode::FromBuilding },
    CadTransformationSpec { id: "classic", source_model_definition_id: CAD_MODEL_DEFINITION_BUILDING, target_model_definition_id: CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC, mode: TransformationMode::TypologyFallback },
];
//#endregion 🔖️Constants

//#region 🔖️Runtime
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadPlayRuntime {
    #[serde(default)]
    pub selected_object_ids: SelectionSet,
    #[serde(default)]
    pub selected_node_ids: Vec<String>,
    #[serde(default = "default_selection_method")]
    pub selection_method: String,
    #[serde(default)]
    pub hovered_object_id: Option<String>,
    #[serde(default)]
    pub hovered_target: Option<CadHoverTarget>,
    #[serde(default)]
    pub active_object_id: Option<String>,
    #[serde(default)]
    pub component_selection: CadComponentSelection,
    #[serde(default)]
    pub engagement_input: String,
    #[serde(default)]
    pub engagement_step: String,
    #[serde(default)]
    pub active_example_id: Option<String>,
    #[serde(default)]
    pub selected_reference_model_definition_id: Option<String>,
    #[serde(default)]
    pub selected_reference_id: Option<String>,
    #[serde(default)]
    pub selected_primitive_id: Option<String>,
    #[serde(default)]
    pub selected_primitive_kind: Option<String>,
    #[serde(default)]
    pub engagement_pane: Option<String>,
    #[serde(default)]
    pub engagement_session: Option<CadEngagementSession>,
    #[serde(default)]
    pub last_finalized_interaction_id: Option<String>,
    #[serde(default)]
    pub sun: WorldSunConfig,
    /// 🎥️ Per-pane camera pose — session-only view state (never a VCS-tracked document field): see
    /// `"setCamera"`/`"setProjection"`/`"setProjectionParam"` in `handle_action` below.
    #[serde(default)]
    pub camera: CadCamera,
    #[serde(default)]
    pub camera_building: CadCamera,
    #[serde(default)]
    pub camera_energy: CadCamera,
    #[serde(default)]
    pub camera_structure_classic: CadCamera,
    #[serde(default)]
    pub dislocate_options_by_window_id: HashMap<String, CadDislocateOptions>,
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
    pub fn dislocate_options(&self, window_id: &str) -> CadDislocateOptions {
        self.dislocate_options_by_window_id.get(window_id).copied().unwrap_or_default()
    }
}

/// @emoji 🔀️ WORKFLOWS-END-TO-END-TYPED-PORTS config recipe boundary (in): unpacks `cfg.projection`
/// (the persisted, VCS-tracked `CadConfig`) into the ergonomic `CadPlayRuntime` scratch shape every
/// helper function below already works with — a pure, allocation-only conversion, never itself an
/// operation. `dislocate_options_by_window_id` is seeded from the 4 fixed pane fields keyed by the 4
/// constant window-kind ids (`CAD_PLAY_WINDOW_*`) — see `CadDislocateOptions`'s doc comment in
/// `cad_document_engine` for why per-window-INSTANCE keying no longer applies.
pub fn cad_runtime_from_config(cfg: &CadConfig) -> CadPlayRuntime {
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
            (shape::WINDOW_KIND_ID.to_string(), cfg.dislocate_shape),
            (building::WINDOW_KIND_ID.to_string(), cfg.dislocate_building),
            (energy::WINDOW_KIND_ID.to_string(), cfg.dislocate_energy),
            (structure_classic::WINDOW_KIND_ID.to_string(), cfg.dislocate_structure_classic),
        ]),
    }
}

/// @emoji 🔀️ The `cad_runtime_from_config` boundary's outbound twin: repacks the (possibly mutated)
/// `CadPlayRuntime` scratch struct back into a real `CadConfig` snapshot for
/// `CadConfigOperation::Snapshot`. `active_utility_id`/`locale` aren't part of `CadPlayRuntime` (they
/// never had a runtime-side representation pre-B1 either — they were read straight off `ViewState`),
/// so callers that need to change them patch the returned `CadConfig` directly instead of threading
/// them through `CadPlayRuntime`.
pub fn cad_config_from_runtime(runtime: &CadPlayRuntime, base: &CadConfig) -> CadConfig {
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
        dislocate_shape: runtime.dislocate_options(shape::WINDOW_KIND_ID),
        dislocate_building: runtime.dislocate_options(building::WINDOW_KIND_ID),
        dislocate_energy: runtime.dislocate_options(energy::WINDOW_KIND_ID),
        dislocate_structure_classic: runtime.dislocate_options(structure_classic::WINDOW_KIND_ID),
        active_utility_id: base.active_utility_id.clone(),
        locale: base.locale.clone(),
        terminology: base.terminology.clone(),
    }
}

/// 🎥️ Reads the runtime-owned camera for `pane` — the session-only replacement for the old
/// document-backed `cad_pane_camera`.
pub fn cad_pane_camera_runtime(runtime: &CadPlayRuntime, pane: CadPaneId) -> &CadCamera {
    match pane {
        CadPaneId::Shape => &runtime.camera,
        CadPaneId::Building => &runtime.camera_building,
        CadPaneId::Energy => &runtime.camera_energy,
        CadPaneId::StructureClassic => &runtime.camera_structure_classic,
    }
}

/// 🎥️ Mutable counterpart of `cad_pane_camera_runtime`.
pub fn cad_pane_camera_runtime_mut(runtime: &mut CadPlayRuntime, pane: CadPaneId) -> &mut CadCamera {
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
pub struct CadPlayView {
    pub document: CadScene,
    pub runtime: CadPlayRuntime,
}

pub fn cad_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    semio_framework_plugin::ActionFactory::new(CAD_PLAY_CONTROLLER_ID).action(action, args)
}

pub fn camera_json(camera: &CadCamera) -> String {
    world3d_camera_projection_json(camera.position, camera.target, None, camera.zoom, &cad_camera_projection_config(camera))
}

pub fn cad_pane_id_from_suffix(id_suffix: &str) -> CadPaneId {
    match id_suffix {
        "building" => CadPaneId::Building,
        "energy" => CadPaneId::Energy,
        "structure-classic" => CadPaneId::StructureClassic,
        _ => CadPaneId::Shape,
    }
}

pub fn cad_pane_id_from_surface_id(surface_id: &str) -> CadPaneId {
    let suffix = surface_id.split('/').next_back().unwrap_or(surface_id);
    cad_pane_id_from_suffix(suffix)
}

pub fn cad_pane_suffix(pane: CadPaneId) -> &'static str {
    match pane {
        CadPaneId::Shape => "shape",
        CadPaneId::Building => "building",
        CadPaneId::Energy => "energy",
        CadPaneId::StructureClassic => "structure-classic",
    }
}

/// 🌳️ Cad's tree items carry an icon rather than the SDK `tree_item_with_action`'s description slot, so
/// this stays a thin app-specific wrapper — built on the SDK's bare `tree_item` rather than hand-rolling
/// the full `UiTreeItemNode` struct literal.
pub fn cad_tree_item(id: impl Into<String>, label: impl Into<Label>, icon_id: Option<&str>, action: ActionDescriptor) -> semio_framework_plugin::UiTreeItemNode {
    let mut item = tree_item(id, label);
    item.icon_id = icon_id.and_then(IconName::from_str);
    item.action = Some(action);
    item
}

/// 🪟️ Maps a pane to the window-KIND id whose Dislocate options it owns — the typed-command
/// counterpart of the pre-B1 `view_state.window_id` resolution.
pub fn cad_window_id_for_pane(pane: CadPaneId) -> &'static str {
    match pane {
        CadPaneId::Shape => shape::WINDOW_KIND_ID,
        CadPaneId::Building => building::WINDOW_KIND_ID,
        CadPaneId::Energy => energy::WINDOW_KIND_ID,
        CadPaneId::StructureClassic => structure_classic::WINDOW_KIND_ID,
    }
}

/// 🔀️ The `CadConfig -> CadPlayRuntime` boundary every command handler opens with.
pub fn runtime_of(cfg: &ConfigView<'_, CadConfig>) -> CadPlayRuntime {
    cad_runtime_from_config(cfg.projection)
}

/// 🔀️ The outbound twin of [`runtime_of`]: the whole-record config snapshot a handler emits.
pub fn snapshot_of(runtime: &CadPlayRuntime, base: &CadConfig) -> CadConfigOperation {
    CadConfigOperation::Snapshot { config: cad_config_from_runtime(runtime, base) }
}
//#endregion 🔖️Runtime

//#region 🔖️Helpers
/// @emoji 🔁️ Derives the target-pane objects for transformation `qid` and returns the operations
/// that both replace the target pane and refocus onto the target model definition — dispatched by
/// the caller through the store (no direct mutation).
pub fn apply_transformation_operations(document: &CadScene, qid: &str) -> Vec<CadOperation> {
    let Some((model_definition_id, transformation_id)) = qid.rsplit_once('.') else {
        return Vec::new();
    };
    let Some(spec) = CAD_TRANSFORMATION_SPECS.iter().find(|entry| entry.source_model_definition_id == model_definition_id && entry.id == transformation_id) else {
        return Vec::new();
    };
    let Some(source_pane) = cad_pane_from_model_definition_id(spec.source_model_definition_id) else {
        return Vec::new();
    };
    let Some(target_pane) = cad_pane_from_model_definition_id(spec.target_model_definition_id) else {
        return Vec::new();
    };
    let objects = {
        let source_objects: Vec<CadObject> = cad_pane_objects(document, source_pane).to_vec();
        let Ok(mut kernel) = cad_brep_kernel().lock() else {
            return Vec::new();
        };
        let mut prepared = source_objects;
        for object in &mut prepared {
            ensure_object_solid_handle(&mut **kernel, object);
        }
        match spec.mode {
            TransformationMode::DeriveFromGeometry => run_derive_from_geometry(&mut **kernel, &prepared, "derived-energy"),
            TransformationMode::FromBuilding => apply_from_building(&prepared, "derived-structure"),
            TransformationMode::TypologyFallback => apply_typology_fallback(&prepared, &["building.building.slab", "building.building.column", "building.building.beam", "building.building.wall"], "derived-fallback"),
        }
    };
    vec![CadOperation::SetPaneObjects { pane: target_pane, objects }, CadOperation::SetActiveModelDefinition { model_definition_id: spec.target_model_definition_id.into() }]
}

pub fn collect_pane_solids(kernel: &mut dyn BrepKernel, envelope: &CadPlayView, pane: CadPaneId) -> Vec<GeometryHandle> {
    cad_pane_objects(&envelope.document, pane)
        .iter()
        .filter_map(|object| {
            let next = object.clone();
            solid_for_object(kernel, &next)
        })
        .collect()
}

pub fn collect_modelspace_solids(kernel: &mut dyn BrepKernel, envelope: &CadPlayView) -> Vec<GeometryHandle> {
    CadPaneId::all().into_iter().flat_map(|pane| collect_pane_solids(kernel, envelope, pane)).collect()
}

pub fn export_solid_for_pane(envelope: &CadPlayView, pane: CadPaneId, format: OsMediaFormat) -> Option<CadSolidExport> {
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

pub fn export_solid_modelspace(envelope: &CadPlayView, format: OsMediaFormat) -> Option<CadSolidExport> {
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
pub fn cad_solid_export_effect(export: CadSolidExport) -> HostEffect {
    let data = match export.data {
        Value::String(text) => text,
        other => serde_json::to_string(&other).unwrap_or_default(),
    };
    HostEffect::DownloadMediaExport { filename: export.filename, mime_type: export.mime_type, data, encoding: export.encoding }
}

/// @emoji ⬇️ Wraps a spatial-JSON export document into a download host effect.
pub fn cad_spatial_export_effect(value: &Value, filename: &str) -> HostEffect {
    HostEffect::DownloadMediaExport { filename: filename.into(), mime_type: "text/plain".into(), data: serde_json::to_string(value).unwrap_or_default(), encoding: None }
}

pub fn export_spatial_json(envelope: &CadPlayView, mode: &str) -> Value {
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
            let pane = cad_pane_from_model_definition_id(&envelope.document.active_model_definition_id).unwrap_or(CadPaneId::Shape);
            let selected: Vec<&CadObject> = envelope.runtime.selected_object_ids.iter().filter_map(|id| cad_all_objects(&envelope.document).find(|(object, _)| &object.id == id).map(|(object, _)| object)).collect();
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
            let pane = cad_pane_from_model_definition_id(&envelope.document.active_model_definition_id).unwrap_or(CadPaneId::Shape);
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

pub fn normalize_component_selection_mode(mode: &str) -> String {
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

pub fn enable_component_selection_target(targets: &mut CadSelectionTargets, mode: &str) {
    match mode {
        "vertex" => targets.vertex = true,
        "edge" => targets.edge = true,
        "face" => targets.face = true,
        "mesh" | "object" => targets.mesh = true,
        _ => {}
    }
}

pub fn merge_component_selection_ids(existing: &[u32], incoming: &[u32], merge: &str) -> Vec<u32> {
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

pub fn clear_component_selection(runtime: &mut CadPlayRuntime) {
    runtime.component_selection.mode = "mesh".into();
    runtime.component_selection.ids.clear();
}

pub fn apply_component_selection(runtime: &mut CadPlayRuntime, mode: &str, incoming: &[u32], merge: &str, object_id: Option<&str>) {
    let normalized = normalize_component_selection_mode(mode);
    enable_component_selection_target(&mut runtime.component_selection.targets, &normalized);
    runtime.component_selection.mode = normalized.clone();
    if normalized == "mesh" {
        runtime.component_selection.ids.clear();
        return;
    }
    runtime.component_selection.ids = merge_component_selection_ids(&runtime.component_selection.ids, incoming, merge);
    if let Some(object_id) = object_id {
        runtime.active_object_id = Some(object_id.into());
        if merge == "replace" || runtime.selected_object_ids.is_empty() {
            runtime.selected_object_ids = SelectionSet::from(vec![object_id.into()]);
        } else if !runtime.selected_object_ids.contains(object_id) {
            runtime.selected_object_ids.push_unique(object_id.into());
        }
    }
}

pub fn resolve_active_object_id(runtime: &CadPlayRuntime) -> Option<String> {
    runtime.active_object_id.clone().or_else(|| runtime.selected_object_ids.first().map(str::to_string))
}
pub fn object_patch_from_field(field: &str, value: Option<&Value>) -> Option<CadObjectPatch> {
    match field {
        "label" | "name" => value.and_then(|entry| entry.as_str()).map(|label| CadObjectPatch { label: Some(label.into()), ..Default::default() }),
        "typology" => value.and_then(|entry| entry.as_str()).map(|typology| CadObjectPatch { typology: Some(typology.into()), ..Default::default() }),
        "hidden" => value.and_then(|entry| entry.as_bool()).map(|hidden| CadObjectPatch { visible: Some(!hidden), ..Default::default() }),
        "locked" => value.and_then(|entry| entry.as_bool()).map(|locked| CadObjectPatch { locked: Some(locked), ..Default::default() }),
        _ => None,
    }
}

pub fn resolve_number_edit(current: f64, value: Option<&Value>, delta: Option<&Value>) -> Option<f64> {
    if let Some(absolute) = value.and_then(Value::as_f64) {
        return Some(absolute);
    }
    delta.and_then(Value::as_f64).map(|delta| current + delta)
}

pub fn axis3_index(field: &str, base: &str) -> Option<usize> {
    match field.strip_prefix(base)?.strip_prefix('.')? {
        "x" => Some(0),
        "y" => Some(1),
        "z" => Some(2),
        _ => None,
    }
}

pub fn axis4_index(field: &str, base: &str) -> Option<usize> {
    match field.strip_prefix(base)?.strip_prefix('.')? {
        "x" => Some(0),
        "y" => Some(1),
        "z" => Some(2),
        "w" => Some(3),
        _ => None,
    }
}

pub fn quat_normalize(q: [f64; 4]) -> [f64; 4] {
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
pub fn patch_objects_operations(document: &CadScene, object_ids: &[String], field: &str, value: Option<&Value>, delta: Option<&Value>) -> Vec<CadOperation> {
    if let Some(patch) = object_patch_from_field(field, value) {
        return object_ids.iter().filter_map(|object_id| cad_find_object_pane(document, object_id).map(|pane| CadOperation::PatchObject { pane, object_id: object_id.clone(), patch: patch.clone() })).collect();
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

pub fn make_object_for_typology(typology: &str, label_count: usize, pane: CadPaneId) -> CadObject {
    let label = TYPOLOGY_CATALOG.iter().find(|entry| entry.typology == typology).map_or("Object", |entry| entry.label);
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
pub fn try_commit_session_operations(document: &CadScene, runtime: &mut CadPlayRuntime, pane: CadPaneId, session: &CadEngagementSession) -> Vec<CadOperation> {
    if !can_commit(session) {
        return Vec::new();
    }
    let label_count = cad_pane_objects(document, pane).len();
    let Ok(mut kernel) = cad_brep_kernel().lock() else {
        return Vec::new();
    };
    let Some(object) = commit_object(&mut **kernel, session, label_count, next_cad_id) else {
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
pub fn engagement_submit_operations(document: &CadScene, runtime: &mut CadPlayRuntime, pane: CadPaneId) -> Vec<CadOperation> {
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
                if (transition.key.eq_ignore_ascii_case(&input) || transition.event_kind.eq_ignore_ascii_case(&input))
                    && apply_event(session, &transition.event_kind, None) {
                        runtime.engagement_step = session.state.clone();
                        runtime.engagement_input.clear();
                        let session_snapshot = session.clone();
                        return try_commit_session_operations(document, runtime, pane, &session_snapshot);
                    }
            }
        } else if let Some(entry) = resolve_interaction_key(&event_kind, model_definition_id) {
            runtime.engagement_session = start_session(&entry.id, pane);
            if let Some(session) = runtime.engagement_session.as_mut() {
                let _ = apply_event(session, "start", None);
            }
            runtime.engagement_step = runtime.engagement_session.as_ref().map_or_else(|| "Idle".into(), |session| session.state.clone());
            runtime.engagement_input.clear();
            return Vec::new();
        }
    }
    runtime.engagement_step = format!("Unknown: {input}");
    Vec::new()
}

/// Starts a fresh engagement session for `interaction_id` in `pane` (used by
/// `engagementPossibleSelect`'s start-by-id path and `engagementRepeatLast`).
pub fn start_interaction_session(runtime: &mut CadPlayRuntime, pane: CadPaneId, interaction_id: &str) -> bool {
    let Some(entry) = interaction::interaction_by_id(interaction_id) else {
        return false;
    };
    runtime.engagement_session = start_session(&entry.id, pane);
    if let Some(session) = runtime.engagement_session.as_mut() {
        let _ = apply_event(session, "start", None);
    }
    runtime.engagement_step = runtime.engagement_session.as_ref().map_or_else(|| "Idle".into(), |session| session.state.clone());
    true
}

/// @emoji 🔀️ WORKFLOWS-END-TO-END-TYPED-PORTS: the typed-command counterpart of the pre-B1
/// `mesh_selection_ids` (JSON-args) helper — falls back to the current selection when the command
/// carries no explicit ids.
pub fn ids_or_selection(ids: &[String], fallback: &[String]) -> Vec<String> {
    if ids.is_empty() {
        fallback.to_vec()
    } else {
        ids.to_vec()
    }
}

/// @emoji 🩹️ Typed-command counterpart of a raw JSON patch value: `CadCommand::PatchObject`/
/// `PatchSelection`/`PatchCadPlayReference` all carry `value: Option<String>` (the typed channel has no
/// single Rust type spanning "maybe a string, maybe a number, maybe a bool") — this recovers the
/// `serde_json::Value` shape `object_patch_from_field`/`resolve_number_edit` already expect, dispatching
/// on the same field-name vocabulary those helpers use (bool fields by name, everything else tried as a
/// number first, falling back to a string).
pub fn command_value_json(field: &str, value: &str) -> Value {
    match field {
        "hidden" | "locked" => value.parse::<bool>().map_or(Value::Null, Value::Bool),
        _ => value.parse::<f64>().map_or_else(|_| Value::String(value.into()), |number| json!(number)),
    }
}
//#endregion 🔖️Helpers

//#region 🔖️Io
/// 🔌️ This app's typed media I/O surface (`AppDefinition.io`) — the implicit document ports (`3d.cad`,
/// `ThreeD×Brep`) plus the two workflow ports the port recipe adds: `geometry:in` (accepts geometry
/// from any upstream 3D producer — `MediaForm::Any` only ever legal on the accepting side) and
/// `brep:out` (this app's own `3d.cad` kind, `Many` multiplicity so several downstream consumers can
/// each pull an independent export).
pub fn cad_io() -> semio_framework_plugin::AppIo {
    semio_framework_plugin::AppIo {
        document_schema: "cad.scene".into(),
        document_media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
        ports: vec![
            semio_framework_plugin::MediaPortSpec {
                id: "geometry:in".into(),
                label: "Geometry".into(),
                direction: semio_framework_plugin::MediaPortDirection::In,
                media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Any },
                kind_id: None,
                required: false,
                multiplicity: semio_framework_core::PortMultiplicity::Many,
            },
            semio_framework_plugin::MediaPortSpec {
                id: "brep:out".into(),
                label: "Brep".into(),
                direction: semio_framework_plugin::MediaPortDirection::Out,
                media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
                kind_id: Some("3d.cad".into()),
                required: false,
                multiplicity: semio_framework_core::PortMultiplicity::Many,
            },
        ],
        export_formats: vec![OsMediaFormat::Step, OsMediaFormat::Obj, OsMediaFormat::Stl, OsMediaFormat::Glb],
        import_formats: vec![OsMediaFormat::Step, OsMediaFormat::Obj, OsMediaFormat::Stl],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "3d.cad".into(), name: "3D CAD".into(), dimension: "3d".into(), component_kind: "cad".into() },
    }
}
//#endregion 🔖️Io


//#region 🔖️Commands
/// 🧵️ Per-dispatch app-struct state that is neither document nor config — cad has exactly one such
/// field, `gesture_preview`'s monotone tick counter (see [`CadPlayApp::gesture_preview`]).
pub struct CadDispatchCtx<'a> {
    pub preview_seq: &'a std::cell::RefCell<u64>,
}

semio_framework_plugin::app_commands! {
    /// 🎯️ `CadPlayApp::Command` — the SOLE dispatch surface for cad's own behavior, decomposed into
    /// one `🎮️commands/<group>/<command>` payload module per row. Row order IS the binary variant
    /// ordinal and the two literals are two different vocabularies (camelCase manifest action id,
    /// kebab wire keyword) — both are copied verbatim from the pre-consolidation `CadCommand` enum.
    pub enum CadCommand for CadScene, CadOperation, CadConfig, CadConfigOperation, ctx = CadDispatchCtx<'_> {
        // 🔧️ Document-mutating — dispatched as VCS operations with a true inverse.
        "addObject" as "add-object" => add_object::AddObject,
        "patchObject" as "patch-object" => patch_object::PatchObject,
        "patchSelection" as "patch-selection" => patch_selection::PatchSelection,
        "deleteObject" as "delete-object" => delete_object::DeleteObject,
        "duplicateObject" as "duplicate-object" => duplicate_object::DuplicateObject,
        "addNode" as "add-node" => add_node::AddNode,
        "renameNode" as "rename-node" => rename_node::RenameNode,
        "translateSelection" as "translate-selection" => translate_selection::TranslateSelection,
        "rotateSelection" as "rotate-selection" => rotate_selection::RotateSelection,
        "scaleSelection" as "scale-selection" => scale_selection::ScaleSelection,
        "applyTransformation" as "apply-transformation" => apply_transformation::ApplyTransformation,
        "importCadFile" as "import-cad-file" => import_cad_file::ImportCadFile,
        "patchCadPlayReference" as "patch-cad-play-reference" => patch_cad_play_reference::PatchCadPlayReference,
        "engagementSubmit" as "engagement-submit" => engagement_submit::EngagementSubmit,
        "focusModelDefinition" as "focus-model-definition" => focus_model_definition::FocusModelDefinition,
        "setActiveExample" as "set-active-example" => set_active_example::SetActiveExample,
        "worldPointerDown" as "world-pointer-down" => world_pointer_down::WorldPointerDown,

        // 👁️ Config-only — emit `config_operations`, never document operations.
        "setCamera" as "camera" => set_camera::SetCamera,
        "setProjection" as "projection" => set_projection::SetProjection,
        "setProjectionParam" as "projection-param" => set_projection_param::SetProjectionParam,
        "setDislocateOption" as "dislocate-option" => set_dislocate_option::SetDislocateOption,
        "setSelection" as "set-selection" => set_selection::SetSelection,
        "setNodeSelection" as "set-node-selection" => set_node_selection::SetNodeSelection,
        "worldSelect" as "world-select" => world_select::WorldSelect,
        "worldHover" as "world-hover" => world_hover::WorldHover,
        "setHover" as "set-hover" => set_hover::SetHover,
        "worldPick" as "world-pick" => world_pick::WorldPick,
        "setSelectionMethod" as "selection-method" => set_selection_method::SetSelectionMethod,
        "setReferenceSelection" as "reference-selection" => set_reference_selection::SetReferenceSelection,
        "referenceHover" as "reference-hover" => reference_hover::ReferenceHover,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "engagementPossibleSelect" as "engagement-possible-select" => engagement_possible_select::EngagementPossibleSelect,
        "engagementRepeatLast" as "engagement-repeat-last" => engagement_repeat_last::EngagementRepeatLast,
        "engagementAbort" as "engagement-abort" => engagement_abort::EngagementAbort,
        "worldPointerMove" as "world-pointer-move" => world_pointer_move::WorldPointerMove,
        "setPrimitiveSelection" as "set-primitive-selection" => set_primitive_selection::SetPrimitiveSelection,
        "toggleSun" as "toggle-sun" => toggle_sun::ToggleSun,
        "setSunAzimuth" as "sun-azimuth" => set_sun_azimuth::SetSunAzimuth,
        "setSunElevation" as "sun-elevation" => set_sun_elevation::SetSunElevation,
        "setSunIntensity" as "sun-intensity" => set_sun_intensity::SetSunIntensity,
        "setActiveUtility" as "active-utility" => set_active_utility::SetActiveUtility,
        "setLocale" as "locale" => set_locale::SetLocale,
        "setTerminology" as "terminology" => set_terminology::SetTerminology,

        // 🐚️ Shell effects — export/import round-trips through the host, no operations either way.
        "saveSelected" as "save-selected" => save_selected::SaveSelected,
        "saveInPlay" as "save-in-play" => save_in_play::SaveInPlay,
        "saveCurrent" as "save-current" => save_current::SaveCurrent,
        "loadRawRequest" as "load-raw-request" => load_raw_request::LoadRawRequest,
    }
}
//#endregion 🔖️Commands

//#region 🔖️PlayApp
/// 📐️ B1/WORKFLOWS-END-TO-END-TYPED-PORTS: unit-struct-shaped pure `DocumentApp` — every former
/// `CadPlayRuntime`/`self.runtime` field now lives in `CadConfig`, written through
/// `CadConfigOperation`s (real `backwards`, no ad hoc `InverseAction`). `preview_seq` is the sole
/// surviving interior-mutable field — it backs `gesture_preview`'s never-VCS'd, never-config'd live
/// rubber-band tick counter, not app state.
#[derive(Default)]
pub struct CadPlayApp {
    /// 👻️ Per-`key` monotone counter for `gesture_preview`.
    preview_seq: std::cell::RefCell<u64>,
}

impl CadPlayApp {
    /// 👻️ CW7 db+protocol+vcs-slimming campaign, "preview law for gesture apps": the live rubber-band
    /// engagement session, shaped as the exact payload
    /// `framework_sync::SyncSession::publish_preview(key, seq, payload)` expects, ready to hand off the
    /// instant a transport exists. `None` outside an active engagement session; reads
    /// `CadEngagementSession` only, never `CadScene`/`CadOperation` — a preview can never become
    /// persistent state.
    ///
    /// 🚧️ Deliberately unwired beyond this accessor — `framework/sync::SyncSession::publish_preview`
    /// is host-only and unreachable from this WASI-P2 sandboxed plugin crate, and
    /// `store::BackboneMessage` has no preview-shaped variant to relay one through. See
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
    /// geometry from any upstream 3D producer and inserts it as a new `CadObject` in the Shape pane,
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
        match crate::artifacts::cad::engine::import_cad_object_by_extension(name, &payload) {
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
            let media_type = self.io().map_or(MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep }, |io| io.document_media_type);
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
        Ok(Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep }, payload: MediaPayload::Structured { schema: "3d.cad".into(), json: base64::engine::general_purpose::STANDARD.encode(text.as_bytes()) } })
    }

    fn command_id(&self, command: &CadCommand) -> &str {
        command.command_id()
    }

    fn handle(&self, command: &CadCommand, doc: &DocumentView<'_, CadScene>, cfg: &ConfigView<'_, CadConfig>) -> Result<Emit<CadOperation, CadConfigOperation>, Fault> {
        let mut ctx = CadDispatchCtx { preview_seq: &self.preview_seq };
        command.dispatch(doc, cfg, &mut ctx)
    }

    fn render(&self, body_key: &str, doc: &DocumentView<'_, CadScene>, cfg: &ConfigView<'_, CadConfig>) -> UiNode {
        let view = CadPlayView { document: doc.projection.clone(), runtime: cad_runtime_from_config(cfg.projection) };
        let labels = cad_labels(cfg.projection);
        let window_kind_id = match body_key {
            shape::BODY_KEY => shape::WINDOW_KIND_ID,
            building::BODY_KEY => building::WINDOW_KIND_ID,
            energy::BODY_KEY => energy::WINDOW_KIND_ID,
            structure_classic::BODY_KEY => structure_classic::WINDOW_KIND_ID,
            _ => shape::WINDOW_KIND_ID,
        };
        let active_utility = Some(cfg.projection.active_utility_id.as_str());
        let options = view.runtime.dislocate_options(window_kind_id);
        match body_key {
            shape::BODY_KEY => shape::render(&view, active_utility, options),
            building::BODY_KEY => building::render(&view, active_utility, options),
            energy::BODY_KEY => energy::render(&view, active_utility, options),
            structure_classic::BODY_KEY => structure_classic::render(&view, active_utility, options),
            document::CAD_PLAY_BODY_DOCUMENT => document::build_document_tree(&view, labels),
            catalogue::CAD_PLAY_BODY_CATALOGUE => catalogue::build_catalogue_tree(labels),
            inspection::CAD_PLAY_BODY_PROPERTIES => inspection::build_properties_panel(&view, labels, active_utility),
            _ => semio_framework_plugin::ui_text(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_engagements(&self, doc: &DocumentView<'_, CadScene>, cfg: &ConfigView<'_, CadConfig>) -> HashMap<String, WindowEngagement> {
        let view = CadPlayView { document: doc.projection.clone(), runtime: cad_runtime_from_config(cfg.projection) };
        let labels = cad_labels(cfg.projection);
        HashMap::from([
            (shape::WINDOW_KIND_ID.to_string(), shape::engagement(&view, labels)),
            (building::WINDOW_KIND_ID.to_string(), building::engagement(&view, labels)),
            (energy::WINDOW_KIND_ID.to_string(), energy::engagement(&view, labels)),
            (structure_classic::WINDOW_KIND_ID.to_string(), structure_classic::engagement(&view, labels)),
        ])
    }

    /// 🪟️ Keyed by the 4 fixed window-KIND ids; each window collects its own measures from the edit
    /// mode's `🎚️options/*` components.
    fn window_measures(&self, _doc: &DocumentView<'_, CadScene>, cfg: &ConfigView<'_, CadConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let runtime = cad_runtime_from_config(cfg.projection);
        let is_de = cad_is_de_locale(cfg.projection);
        HashMap::from([
            (shape::WINDOW_KIND_ID.to_string(), shape::window_measures(&runtime, is_de)),
            (building::WINDOW_KIND_ID.to_string(), building::window_measures(&runtime, is_de)),
            (energy::WINDOW_KIND_ID.to_string(), energy::window_measures(&runtime, is_de)),
            (structure_classic::WINDOW_KIND_ID.to_string(), structure_classic::window_measures(&runtime, is_de)),
        ])
    }

    /// 🖱️ Selection-gated menu: transform/duplicate/delete only once something is selected — a bare
    /// right-click on empty World3d background (nothing selected) falls through to the shell's
    /// window-level menu (undo/redo/view actions) instead of showing an empty CAD-specific section.
    fn context_menu(&self, _request: &ContextMenuRequest, _doc: &DocumentView<'_, CadScene>, cfg: &ConfigView<'_, CadConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        if cfg.projection.selected_object_ids.is_empty() {
            return Vec::new();
        }
        Menu::of(registry).action("translateSelection").action("rotateSelection").action("scaleSelection").group("create", |m| m.action("duplicateObject")).destructive("deleteObject").build()
    }
}
//#endregion 🔖️PlayApp

//#region 🔖️Manifest
/// @emoji 🧰️ The window-scoped CAD Dislocate utility, whose Move and Rotate handles are utility options.
pub fn cad_dislocate_utility() -> UtilityDefinition {
    UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new(CAD_DISLOCATE_UTILITY_ID, LocalizedLabel::native("Dislocate", "Versetzen"), "move-3d") }
}

/// @emoji 🧰️ The single Dislocate utility ref exposed independently by each world-3d window.
pub fn cad_dislocate_utility_refs() -> Vec<semio_framework_plugin::UtilityRef> {
    vec![CAD_DISLOCATE_UTILITY_ID.into()]
}

pub fn create_cad_app() -> App {
    App::from_builder(
        App::builder(CAD_PLAY_APP_ID, LocalizedLabel::native("CAD", "CAD")).document(["semio", "cad"])
            .artifact_kind(artifact_kind())
            .icon_id("box")
            .terminology("reuse")
            .terminology_document("reuse", ["Entwerfen mit Bestand", "cad"])
            .mode_def(edit::definition())
            .default_mode_id(edit::CAD_PLAY_MODE_EDIT)
            .window_kind_def(shape::definition())
            .window_kind_def(building::definition())
            .window_kind_def(energy::definition())
            .window_kind_def(structure_classic::definition())
            .default_layout(edit::layout())
            .operation("addObject", LocalizedLabel::native("Add Object", "Objekt hinzufügen"))
            .operation("patchObject", LocalizedLabel::native("Patch Object", "Objekt aktualisieren"))
            .operation("patchSelection", LocalizedLabel::native("Patch Selection", "Auswahl aktualisieren"))
            .action_with(ActionDefinition::new_catalog("deleteObject", LocalizedLabel::native("Delete Object", "Objekt löschen"), ActionKind::Operation).category("actions"))
            .action_with(ActionDefinition::new_catalog("duplicateObject", LocalizedLabel::native("Duplicate Object", "Objekt duplizieren"), ActionKind::Operation).category("create"))
            .operation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .operation("renameNode", LocalizedLabel::native("Rename Node", "Knoten umbenennen"))
            .action_with(ActionDefinition::new_catalog("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"), ActionKind::Operation).category("transform"))
            .action_with(ActionDefinition::new_catalog("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"), ActionKind::Operation).category("transform"))
            .action_with(ActionDefinition::new_catalog("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"), ActionKind::Operation).category("transform"))
            .operation("applyTransformation", LocalizedLabel::native("Apply Transformation", "Transformation anwenden"))
            .operation("importCadFile", LocalizedLabel::native("Import CAD File", "CAD-Datei importieren"))
            .action_with(ActionDefinition::new_catalog("patchCadPlayReference", LocalizedLabel::native("Patch Reference", "Referenz aktualisieren"), ActionKind::Operation).in_palette(false))
            .action_with(ActionDefinition::new_catalog("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Operation).in_palette(false))
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .view_action("setProjection", LocalizedLabel::native("Set Projection", "Projektion festlegen"))
            .view_action("setProjectionParam", LocalizedLabel::native("Set Projection Parameter", "Projektionsparameter festlegen"))
            .operation("focusModelDefinition", LocalizedLabel::native("Focus Model Definition", "Modelldefinition fokussieren"))
            .operation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .action_with(ActionDefinition::new_catalog("setSelection", LocalizedLabel::native("Set Selection", "Auswahl festlegen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("setNodeSelection", LocalizedLabel::native("Set Node Selection", "Knotenauswahl festlegen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("worldSelect", LocalizedLabel::native("World Select", "Welt auswählen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("worldHover", LocalizedLabel::native("World Hover", "Überfahren (Welt)"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("setHover", LocalizedLabel::native("Set Hover", "Überfahren festlegen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("worldPick", LocalizedLabel::native("World Pick", "Punkt in der Welt wählen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("setSelectionMethod", LocalizedLabel::native("Set Selection Method", "Auswahlmethode festlegen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("setReferenceSelection", LocalizedLabel::native("Set Reference Selection", "Referenzauswahl festlegen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("referenceHover", LocalizedLabel::native("Reference Hover", "Überfahren (Referenz)"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("engagementPossibleSelect", LocalizedLabel::native("Engagement Possible Select", "Eingabeoption auswählen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("engagementRepeatLast", LocalizedLabel::native("Engagement Repeat Last", "Letzte Eingabe wiederholen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("worldPointerMove", LocalizedLabel::native("World Pointer Move", "Welt-Zeiger bewegt"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("engagementPointerDown", LocalizedLabel::native("Engagement Pointer Down", "Eingabe-Zeiger gedrückt"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new_catalog("setPrimitiveSelection", LocalizedLabel::native("Set Primitive Selection", "Grundkörperauswahl festlegen"), ActionKind::View).in_palette(false))
            .view_action("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"))
            .view_action("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"))
            .view_action("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"))
            .view_action("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"))
            .action_with(ActionDefinition::new_catalog("setDislocateOption", LocalizedLabel::native("Set Dislocate Option", "Versetzen-Option festlegen"), ActionKind::View).in_palette(false))
            .shell_action("saveSelected", LocalizedLabel::native("Save Selected", "Auswahl speichern"))
            .shell_action("saveInPlay", LocalizedLabel::native("Save In Play", "Im Play speichern"))
            .shell_action("saveCurrent", LocalizedLabel::native("Save Current", "Aktuelles speichern"))
            .shell_action("loadRawRequest", LocalizedLabel::native("Load Raw Request", "Rohdaten laden"))
            .action_args("saveCurrent", vec![ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), vec![
                ActionArgOption::new("step", LocalizedLabel::native("STEP", "STEP")),
                ActionArgOption::new("obj", LocalizedLabel::native("OBJ", "OBJ")),
                ActionArgOption::new("stl", LocalizedLabel::native("STL", "STL")),
            ]).default_value("step")])
            .action_args("focusModelDefinition", vec![ActionArgDef::select("modelDefinitionId", LocalizedLabel::native("Model Definition", "Modelldefinition"), vec![
                ActionArgOption::new(CAD_MODEL_DEFINITION_SHAPE, LocalizedLabel::native("Shape", "Form")),
                ActionArgOption::new(CAD_MODEL_DEFINITION_BUILDING, LocalizedLabel::native("Building", "Gebäude")),
                ActionArgOption::new(CAD_MODEL_DEFINITION_ENERGY, LocalizedLabel::native("Energy", "Energie")),
                ActionArgOption::new(CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC, LocalizedLabel::native("Structure Classic", "Tragwerk Klassisch")),
            ]).required()])
            .action_args("setActiveExample", vec![ActionArgDef::select("exampleId", LocalizedLabel::native("Example", "Beispiel"), vec![
                ActionArgOption::new(CAD_EXAMPLE_FOREST_LEFT, LocalizedLabel::native("Hexagonal Cut Concrete Forest Left", "Sechseckig geschnittener Betonwald links")),
            ]).required()])
            .utility(cad_dislocate_utility())
            .window_kind_utilities(shape::WINDOW_KIND_ID, cad_dislocate_utility_refs())
            .window_kind_utilities(building::WINDOW_KIND_ID, cad_dislocate_utility_refs())
            .window_kind_utilities(energy::WINDOW_KIND_ID, cad_dislocate_utility_refs())
            .window_kind_utilities(structure_classic::WINDOW_KIND_ID, cad_dislocate_utility_refs())
            .panel_tab_def(document::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            // 🎯️ Typed channel + port surface (WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2) — `cad_io()` is
            // this same `3d.cad`/Brep information's single source of truth, reused here rather than
            // duplicated; `config_spec()` stays empty (cad has no sticky-default settings analogous to
            // shooting's format defaults — every `CadConfig` field is session view-state, not a setting).
            .config(CadPlayApp::default().config_spec())
            .io(cad_io()),
    )
    .example(CAD_EXAMPLE_FOREST_LEFT, LocalizedLabel::native("Hexagonal Cut Concrete Forest Left", "Sechseckig geschnittener Betonwald links"), serde_json::to_string(&forest_play_scene()).unwrap(), "list-tree")
    .workflow("cad", "CAD", "model")
}
//#endregion 🔖️Manifest

//#region 🧪️Tests
#[cfg(test)]
pub(crate) mod testkit {
    //! 🧪️ The one cad-app test harness — every other taxonomy node's `🧪️Tests` region builds on it
    //! instead of re-deriving a store/dispatch/render scaffold of its own.
    use super::*;
    use protocol::{Operation, OperationDiff};
    use semio_framework_plugin::{ActionMeta, HistoryView, UiMenuRef, VcsDocumentApp, SET_ACTIVE_UTILITY_ACTION_ID};


    pub fn meta(actor: &str) -> ActionMeta {
        semio_framework_plugin::testkit::meta(actor)
    }

    pub fn new_app() -> VcsDocumentApp<CadPlayApp> {
        semio_framework_plugin::testkit::new_app::<CadPlayApp>()
    }

    pub fn empty_history() -> HistoryView {
        HistoryView::empty()
    }

    /// @emoji 🔀️ WORKFLOWS-END-TO-END-TYPED-PORTS test-only bridge: recovers a typed `CadCommand` from
    /// the pre-B1 `(action id, JSON args)` shape every test in this module was already written against
    /// — the same information `AppDefinition`'s declared `ActionArgDef`s carry, reconstructed by hand
    /// here rather than threading a real host-side action→command bridge (out of scope for this ticket;
    /// Wave 3 wires the shell). Panics on an unrecognized action id — every id used below is covered.
    pub fn command_from_action(action: &str, args: Option<&Value>) -> CadCommand {
        let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
        let f64_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_f64);
        let u64_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_u64);
        let bool_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_bool);
        let str_vec_field = |key: &str| -> Vec<String> { args.and_then(|value| value.get(key)).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default() };
        let u32_vec_field = |key: &str| -> Vec<u32> {
            args.and_then(|value| value.get(key))
                .and_then(|value| if let Some(array) = value.as_array() { Some(array.iter().filter_map(|entry| entry.as_u64().map(|number| number as u32)).collect()) } else { serde_json::from_value(value.clone()).ok() })
                .unwrap_or_default()
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
            "setActiveExample" => CadCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_field("exampleId").unwrap_or_default() }),
            SET_ACTIVE_UTILITY_ACTION_ID => CadCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: str_field("utilityId").unwrap_or_default() }),
            "setLocale" => CadCommand::SetLocale(set_locale::SetLocale { value: str_field("value").unwrap_or_default() }),
            "setTerminology" => CadCommand::SetTerminology(set_terminology::SetTerminology { value: str_field("value").unwrap_or_default() }),
            "setDislocateOption" => CadCommand::SetDislocateOption(set_dislocate_option::SetDislocateOption { pane: str_field("pane"), option: str_field("option").unwrap_or_default(), pressed: bool_field("pressed") }),
            "setSelection" => CadCommand::SetSelection(set_selection::SetSelection { mode: str_field("mode").unwrap_or_else(|| "mesh".into()), ids: u32_vec_field("ids"), object_id: str_field("objectId"), merge: str_field("merge").unwrap_or_else(|| "replace".into()) }),
            "setNodeSelection" => CadCommand::SetNodeSelection(set_node_selection::SetNodeSelection { node_ids: str_vec_field("nodeIds") }),
            "setCamera" => CadCommand::SetCamera(set_camera::SetCamera { pane: str_field("surfaceId"), camera: args.and_then(|value| value.get("camera")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default() }),
            "setProjection" => CadCommand::SetProjection(set_projection::SetProjection {
                pane: str_field("surfaceId"),
                field: str_field("field"),
                value_str: args.and_then(|value| value.get("value")).and_then(Value::as_str).map(String::from),
                value_num: args.and_then(|value| value.get("value")).and_then(Value::as_f64),
                param: str_field("param"),
            }),
            "setProjectionParam" => CadCommand::SetProjectionParam(set_projection_param::SetProjectionParam {
                pane: str_field("surfaceId"),
                field: str_field("field"),
                value_str: args.and_then(|value| value.get("value")).and_then(Value::as_str).map(String::from),
                value_num: args.and_then(|value| value.get("value")).and_then(Value::as_f64),
                param: str_field("param"),
            }),
            "translateSelection" => CadCommand::TranslateSelection(translate_selection::TranslateSelection { object_ids: str_vec_field("objectIds"), dx: f64_field("dx").unwrap_or(0.0), dy: f64_field("dy").unwrap_or(0.0), dz: f64_field("dz").unwrap_or(0.0) }),
            "rotateSelection" => {
                CadCommand::RotateSelection(rotate_selection::RotateSelection { object_ids: str_vec_field("objectIds"), ax: f64_field("ax").unwrap_or(0.0), ay: f64_field("ay").unwrap_or(0.0), az: f64_field("az").unwrap_or(0.0), angle: f64_field("angle").unwrap_or(0.0) })
            }
            "scaleSelection" => CadCommand::ScaleSelection(scale_selection::ScaleSelection { object_ids: str_vec_field("objectIds"), sx: f64_field("sx").unwrap_or(1.0), sy: f64_field("sy").unwrap_or(1.0), sz: f64_field("sz").unwrap_or(1.0) }),
            "addObject" => CadCommand::AddObject(add_object::AddObject { typology: str_field("typology") }),
            "patchObject" => CadCommand::PatchObject(patch_object::PatchObject { object_id: str_field("objectId").unwrap_or_default(), field: str_field("field").unwrap_or_default(), value: value_string(), delta: f64_field("delta") }),
            "patchSelection" => CadCommand::PatchSelection(patch_selection::PatchSelection { object_ids: str_vec_field("objectIds"), field: str_field("field").unwrap_or_default(), value: value_string(), delta: f64_field("delta") }),
            "deleteObject" => CadCommand::DeleteObject(delete_object::DeleteObject { object_id: str_field("objectId").unwrap_or_default() }),
            "duplicateObject" => CadCommand::DuplicateObject(duplicate_object::DuplicateObject { object_id: str_field("objectId").unwrap_or_default() }),
            "addNode" => CadCommand::AddNode(add_node::AddNode { kind: str_field("kind").unwrap_or_else(|| "solid".into()) }),
            "renameNode" => CadCommand::RenameNode(rename_node::RenameNode { node_id: str_field("nodeId").unwrap_or_default(), value: str_field("value").unwrap_or_default() }),
            "worldSelect" => CadCommand::WorldSelect(world_select::WorldSelect { ids: str_vec_field("ids"), merge: str_field("merge").unwrap_or_else(|| "replace".into()) }),
            "worldHover" => CadCommand::WorldHover(world_hover::WorldHover { object_id: str_field("id") }),
            "setHover" => CadCommand::SetHover(set_hover::SetHover { object_id: str_field("objectId"), mode: str_field("mode"), id: u64_field("id").map(|value| value as u32) }),
            "worldPick" => CadCommand::WorldPick(world_pick::WorldPick {
                id: u64_field("id"),
                merge: str_field("merge").unwrap_or_else(|| "replace".into()),
                granularity: str_field("granularity").unwrap_or_else(|| "mesh".into()),
                object_id: str_field("objectId"),
                surface_id: str_field("surfaceId"),
                pane: str_field("pane"),
            }),
            "setSelectionMethod" => CadCommand::SetSelectionMethod(set_selection_method::SetSelectionMethod { method: str_field("method").unwrap_or_else(|| "rectangle".into()) }),
            "focusModelDefinition" => CadCommand::FocusModelDefinition(focus_model_definition::FocusModelDefinition { model_definition_id: str_field("modelDefinitionId").unwrap_or_default() }),
            "applyTransformation" => CadCommand::ApplyTransformation(apply_transformation::ApplyTransformation { qid: str_field("qid").unwrap_or_default() }),
            "saveSelected" => CadCommand::SaveSelected(save_selected::SaveSelected {}),
            "saveInPlay" => CadCommand::SaveInPlay(save_in_play::SaveInPlay {}),
            "saveCurrent" => CadCommand::SaveCurrent(save_current::SaveCurrent { format: str_field("format") }),
            "loadRawRequest" => CadCommand::LoadRawRequest(load_raw_request::LoadRawRequest {}),
            "importCadFile" => {
                let payload = args.and_then(|value| value.get("payload").or_else(|| value.get("modelSpace"))).cloned().or_else(|| args.cloned());
                let payload = match payload {
                    Some(Value::String(text)) => text,
                    Some(other) => other.to_string(),
                    None => String::new(),
                };
                CadCommand::ImportCadFile(import_cad_file::ImportCadFile { name: str_field("name").unwrap_or_default(), payload })
            }
            "setReferenceSelection" => CadCommand::SetReferenceSelection(set_reference_selection::SetReferenceSelection { pane: str_field("pane"), model_definition_id: str_field("modelDefinitionId"), reference_id: str_field("referenceId") }),
            "referenceHover" => CadCommand::ReferenceHover(reference_hover::ReferenceHover { reference_id: str_field("referenceId") }),
            "patchCadPlayReference" => CadCommand::PatchCadPlayReference(patch_cad_play_reference::PatchCadPlayReference {
                model_definition_id: str_field("modelDefinitionId").unwrap_or_default(),
                reference_id: str_field("referenceId").unwrap_or_default(),
                field: str_field("field").unwrap_or_default(),
                value: value_string(),
                delta: f64_field("delta"),
            }),
            "engagementInput" => CadCommand::EngagementInput(engagement_input::EngagementInput { value: str_field("value").unwrap_or_default(), pane: str_field("pane") }),
            "engagementSubmit" => CadCommand::EngagementSubmit(engagement_submit::EngagementSubmit { pane: str_field("pane") }),
            "engagementPossibleSelect" => CadCommand::EngagementPossibleSelect(engagement_possible_select::EngagementPossibleSelect { pane: str_field("pane"), possible_id: str_field("possibleId").unwrap_or_default() }),
            "engagementRepeatLast" => CadCommand::EngagementRepeatLast(engagement_repeat_last::EngagementRepeatLast { pane: str_field("pane") }),
            "engagementAbort" => CadCommand::EngagementAbort(engagement_abort::EngagementAbort {}),
            "worldPointerDown" | "engagementPointerDown" => CadCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { pane: str_field("pane"), surface_id: str_field("surfaceId"), x: position_axis(0), y: position_axis(1), z: position_axis(2) }),
            "worldPointerMove" => CadCommand::WorldPointerMove(world_pointer_move::WorldPointerMove { x: position_axis(0), y: position_axis(1), z: position_axis(2) }),
            "setPrimitiveSelection" => CadCommand::SetPrimitiveSelection(set_primitive_selection::SetPrimitiveSelection { object_id: str_field("objectId").unwrap_or_default(), primitive_id: str_field("primitiveId"), kind: str_field("kind") }),
            "toggleSun" => CadCommand::ToggleSun(toggle_sun::ToggleSun {}),
            "setSunAzimuth" => CadCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: f64_field("value").unwrap_or(0.0) }),
            "setSunElevation" => CadCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: f64_field("value").unwrap_or(0.0) }),
            "setSunIntensity" => CadCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: f64_field("value").unwrap_or(0.0) }),
            other => panic!("command_from_action: unhandled test action {other}"),
        }
    }

    /// 🕹️ Drives one action against a bare `CadPlayApp` (unwrapped, config defaulted) so tests can
    /// inspect the emitted document/config operations directly.
    pub fn drive(app: &CadPlayApp, scene: &CadScene, action: &str, args: Option<Value>) -> Emit<CadOperation, CadConfigOperation> {
        drive_with_config(app, scene, action, args, &CadConfig::default())
    }

    /// 🧪️ `args` stays owned so every ported test keeps the pre-migration `(action id, json!(..))`
    /// call shape verbatim; `command_from_action` only ever reads it.
    #[allow(clippy::needless_pass_by_value)]
    pub fn drive_with_config(app: &CadPlayApp, scene: &CadScene, action: &str, args: Option<Value>, config: &CadConfig) -> Emit<CadOperation, CadConfigOperation> {
        let history = empty_history();
        let doc = DocumentView { projection: scene, history: &history };
        let cfg = ConfigView { projection: config };
        let command = command_from_action(action, args.as_ref());
        app.handle(&command, &doc, &cfg).expect("cad command handled")
    }

    pub fn render_direct(app: &CadPlayApp, body_key: &str, doc: &DocumentView<'_, CadScene>, config: &CadConfig) -> UiNode {
        let cfg = ConfigView { projection: config };
        app.render(body_key, doc, &cfg)
    }

    pub fn window_measures_direct(app: &CadPlayApp, doc: &DocumentView<'_, CadScene>, config: &CadConfig) -> HashMap<String, Vec<WindowMeasure>> {
        let cfg = ConfigView { projection: config };
        app.window_measures(doc, &cfg)
    }

    pub fn context_menu_direct(app: &CadPlayApp, doc: &DocumentView<'_, CadScene>, config: &CadConfig, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        let cfg = ConfigView { projection: config };
        let request = ContextMenuRequest { menu: UiMenuRef { id: "world3d".into(), args: None }, surface: None, window_instance_id: None, point: None };
        app.context_menu(&request, doc, &cfg, registry)
    }

    /// 🧮️ Folds a list of `CadOperation`s onto a scene via the core `Operation`/`OperationDiff` impls —
    /// mirrors what the wrapping `VcsDocumentApp` store does when it dispatches the emitted operations.
    pub fn apply_operations(scene: &CadScene, operations: &[CadOperation]) -> CadScene {
        let mut next = scene.clone();
        for operation in operations {
            next = operation.diff(&next).apply(&next);
        }
        next
    }

    /// 🧮️ `apply_operations`'s config-targeted twin — folds an `Emit`'s `config_operations` onto a base
    /// `CadConfig` (mirrors what `VcsDocumentApp`'s config store does when it dispatches them).
    pub fn config_after(emit: &Emit<CadOperation, CadConfigOperation>, base: &CadConfig) -> CadConfig {
        let mut next = base.clone();
        for operation in &emit.config_operations {
            next = operation.diff(&next);
        }
        next
    }

    /// 🧮️ `config_after` plus the `CadConfig -> CadPlayRuntime` boundary conversion — the direct
    /// replacement for the pre-B1 `app.runtime.borrow()` most tests below inspected after `drive(..)`.
    pub fn runtime_after(emit: &Emit<CadOperation, CadConfigOperation>, base: &CadConfig) -> CadPlayRuntime {
        cad_runtime_from_config(&config_after(emit, base))
    }

    pub fn view(scene: CadScene, runtime: CadPlayRuntime) -> CadPlayView {
        CadPlayView { document: scene, runtime }
    }

}

#[cfg(test)]
mod tests {
    use super::testkit::*;
    use super::*;
    use crate::artifacts::cad::engine::{align_mesh_to_fixture_centroid, cad_document_from_dwg, default_document, object_mesh_data, primary_primitive_kind, scene_from_spatial_payload, CAD_DEFAULT_TYPOLOGY_EXTENT, CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX, CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX, CAD_FOREST_REFERENCE_PLANE_Z, CAD_FOREST_REFERENCE_WIDTH_WORLD, CAD_FOREST_REFERENCE_Y_OFFSET_RATIO};
    use crate::artifacts::cad::{empty_cad_projection, CAD_PLAY_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{ActionKind, AppActionRegistry, PluginApp, SET_ACTIVE_UTILITY_ACTION_ID};
    use store::{Backbone, BackboneMessage, MemoryBackbone};


    //#region 🔖️Fixtures
    /// ⚖️ One value per `app_commands!` row (plus a `None`-everywhere twin for every row with
    /// `Option` fields) — the closed set the wire laws below iterate. Captured from the
    /// pre-consolidation `CadCommand` enum, ticket
    /// `26/08/05/CAD-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION`.
    pub(crate) fn every_command() -> Vec<CadCommand> {
        vec![
            CadCommand::AddObject(add_object::AddObject { typology: Some("spatial.shape.primitive.box".into()) }),
            CadCommand::AddObject(add_object::AddObject { typology: None }),
            CadCommand::PatchObject(patch_object::PatchObject { object_id: "object-1".into(), field: "origin.x".into(), value: Some("1.5".into()), delta: Some(2.5) }),
            CadCommand::PatchObject(patch_object::PatchObject { object_id: "object-1".into(), field: "origin.x".into(), value: None, delta: None }),
            CadCommand::PatchSelection(patch_selection::PatchSelection { object_ids: vec!["object-1".into(), "object-2".into()], field: "label".into(), value: Some("Renamed".into()), delta: Some(0.25) }),
            CadCommand::PatchSelection(patch_selection::PatchSelection { object_ids: Vec::new(), field: "label".into(), value: None, delta: None }),
            CadCommand::DeleteObject(delete_object::DeleteObject { object_id: "object-1".into() }),
            CadCommand::DuplicateObject(duplicate_object::DuplicateObject { object_id: "object-1".into() }),
            CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }),
            CadCommand::RenameNode(rename_node::RenameNode { node_id: "node-1".into(), value: "Renamed".into() }),
            CadCommand::TranslateSelection(translate_selection::TranslateSelection { object_ids: vec!["object-1".into()], dx: 1.0, dy: -2.0, dz: 3.5 }),
            CadCommand::RotateSelection(rotate_selection::RotateSelection { object_ids: vec!["object-1".into()], ax: 0.0, ay: 0.0, az: 1.0, angle: 1.57 }),
            CadCommand::ScaleSelection(scale_selection::ScaleSelection { object_ids: vec!["object-1".into()], sx: 2.0, sy: 2.0, sz: 2.0 }),
            CadCommand::ApplyTransformation(apply_transformation::ApplyTransformation { qid: "spatial.shape.from_geometry".into() }),
            CadCommand::ImportCadFile(import_cad_file::ImportCadFile { name: "triangle.obj".into(), payload: "data:model/obj;base64,AAAA".into() }),
            CadCommand::PatchCadPlayReference(patch_cad_play_reference::PatchCadPlayReference { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), field: "widthWorld".into(), value: Some("8".into()), delta: Some(0.5) }),
            CadCommand::PatchCadPlayReference(patch_cad_play_reference::PatchCadPlayReference { model_definition_id: "spatial.shape".into(), reference_id: "ref-1".into(), field: "hidden".into(), value: None, delta: None }),
            CadCommand::EngagementSubmit(engagement_submit::EngagementSubmit { pane: Some("shape".into()) }),
            CadCommand::EngagementSubmit(engagement_submit::EngagementSubmit { pane: None }),
            CadCommand::FocusModelDefinition(focus_model_definition::FocusModelDefinition { model_definition_id: "aec.building".into() }),
            CadCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: "hexagonal-cut-concrete-forest-left".into() }),
            CadCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { pane: Some("shape".into()), surface_id: Some("cad.play.scene3d/shape".into()), x: Some(1.0), y: Some(2.0), z: Some(3.0) }),
            CadCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { pane: None, surface_id: None, x: None, y: None, z: None }),
            CadCommand::SetCamera(set_camera::SetCamera { pane: Some("cad.play.scene3d/building".into()), camera: CadCamera::default() }),
            CadCommand::SetCamera(set_camera::SetCamera { pane: None, camera: CadCamera { position: [1.0, 2.0, 3.0], target: [4.0, 5.0, 6.0], zoom: 2.0, fov: 60.0, ..CadCamera::default() } }),
            CadCommand::SetProjection(set_projection::SetProjection { pane: Some("cad.play.scene3d/shape".into()), field: Some("orthographicView".into()), value_str: Some("top".into()), value_num: Some(12.5), param: Some("fov".into()) }),
            CadCommand::SetProjection(set_projection::SetProjection { pane: None, field: None, value_str: None, value_num: None, param: None }),
            CadCommand::SetProjectionParam(set_projection_param::SetProjectionParam { pane: Some("cad.play.scene3d/shape".into()), field: Some("fov".into()), value_str: Some("x".into()), value_num: Some(45.0), param: Some("fov".into()) }),
            CadCommand::SetProjectionParam(set_projection_param::SetProjectionParam { pane: None, field: None, value_str: None, value_num: None, param: None }),
            CadCommand::SetDislocateOption(set_dislocate_option::SetDislocateOption { pane: Some("building".into()), option: "rotate".into(), pressed: Some(false) }),
            CadCommand::SetDislocateOption(set_dislocate_option::SetDislocateOption { pane: None, option: "move".into(), pressed: None }),
            CadCommand::SetSelection(set_selection::SetSelection { mode: "edge".into(), ids: vec![3, 9], object_id: Some("object-1".into()), merge: "replace".into() }),
            CadCommand::SetSelection(set_selection::SetSelection { mode: "mesh".into(), ids: Vec::new(), object_id: None, merge: "add".into() }),
            CadCommand::SetNodeSelection(set_node_selection::SetNodeSelection { node_ids: vec!["node-1".into(), "node-2".into()] }),
            CadCommand::WorldSelect(world_select::WorldSelect { ids: vec!["object-1".into(), "object-2".into()], merge: "replace".into() }),
            CadCommand::WorldHover(world_hover::WorldHover { object_id: Some("object-1".into()) }),
            CadCommand::WorldHover(world_hover::WorldHover { object_id: None }),
            CadCommand::SetHover(set_hover::SetHover { object_id: Some("object-1".into()), mode: Some("edge".into()), id: Some(3) }),
            CadCommand::SetHover(set_hover::SetHover { object_id: None, mode: None, id: None }),
            CadCommand::WorldPick(world_pick::WorldPick { id: Some(7), merge: "replace".into(), granularity: "edge".into(), object_id: Some("object-1".into()), surface_id: Some("cad.play.scene3d/building".into()), pane: Some("building".into()) }),
            CadCommand::WorldPick(world_pick::WorldPick { id: None, merge: "replace".into(), granularity: "mesh".into(), object_id: None, surface_id: None, pane: None }),
            CadCommand::SetSelectionMethod(set_selection_method::SetSelectionMethod { method: "lasso".into() }),
            CadCommand::SetReferenceSelection(set_reference_selection::SetReferenceSelection { pane: Some("shape".into()), model_definition_id: Some("spatial.shape".into()), reference_id: Some("ref-1".into()) }),
            CadCommand::SetReferenceSelection(set_reference_selection::SetReferenceSelection { pane: None, model_definition_id: None, reference_id: None }),
            CadCommand::ReferenceHover(reference_hover::ReferenceHover { reference_id: Some("ref-1".into()) }),
            CadCommand::ReferenceHover(reference_hover::ReferenceHover { reference_id: None }),
            CadCommand::EngagementInput(engagement_input::EngagementInput { value: "SetHeight2.5".into(), pane: Some("shape".into()) }),
            CadCommand::EngagementInput(engagement_input::EngagementInput { value: String::new(), pane: None }),
            CadCommand::EngagementPossibleSelect(engagement_possible_select::EngagementPossibleSelect { pane: Some("shape".into()), possible_id: "primitive.box".into() }),
            CadCommand::EngagementPossibleSelect(engagement_possible_select::EngagementPossibleSelect { pane: None, possible_id: "primitive.box".into() }),
            CadCommand::EngagementRepeatLast(engagement_repeat_last::EngagementRepeatLast { pane: Some("shape".into()) }),
            CadCommand::EngagementRepeatLast(engagement_repeat_last::EngagementRepeatLast { pane: None }),
            CadCommand::EngagementAbort(engagement_abort::EngagementAbort {}),
            CadCommand::WorldPointerMove(world_pointer_move::WorldPointerMove { x: Some(3.0), y: Some(4.0), z: Some(0.0) }),
            CadCommand::WorldPointerMove(world_pointer_move::WorldPointerMove { x: None, y: None, z: None }),
            CadCommand::SetPrimitiveSelection(set_primitive_selection::SetPrimitiveSelection { object_id: "object-1".into(), primitive_id: Some("solid-1".into()), kind: Some("solid".into()) }),
            CadCommand::SetPrimitiveSelection(set_primitive_selection::SetPrimitiveSelection { object_id: "object-1".into(), primitive_id: None, kind: None }),
            CadCommand::ToggleSun(toggle_sun::ToggleSun {}),
            CadCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 45.0 }),
            CadCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 35.0 }),
            CadCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 0.85 }),
            CadCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "dislocate".into() }),
            CadCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            CadCommand::SetTerminology(set_terminology::SetTerminology { value: "reuse".into() }),
            CadCommand::SaveSelected(save_selected::SaveSelected {}),
            CadCommand::SaveInPlay(save_in_play::SaveInPlay {}),
            CadCommand::SaveCurrent(save_current::SaveCurrent { format: Some("step".into()) }),
            CadCommand::SaveCurrent(save_current::SaveCurrent { format: None }),
            CadCommand::LoadRawRequest(load_raw_request::LoadRawRequest {}),
        ]
    }

    /// ⚖️ Text and binary are two projections of the same command, and every printed line starts with
    /// that row's wire keyword — the guard that a command decomposition cannot silently rename a row.
    #[test]
    fn every_command_round_trips_text_and_binary_under_its_own_wire_keyword() {
        for command in every_command() {
            store::test_support::assert_op_text_binary_equivalence(&command);
            let printed = protocol::OpText::print_op(&command);
            let keyword = printed.split_whitespace().next().unwrap_or_default().to_string();
            assert!(!keyword.is_empty(), "a command must print a leading wire keyword: {printed:?}");
            let decoded = <CadCommand as protocol::OpText>::parse_op(&printed).expect("re-parse");
            assert_eq!(decoded, command, "printed line must re-parse to the same command: {printed:?}");
        }
    }

    /// 🔒️ Wire-format pin: the exact bytes of rows whose `Option` fields make `None`/`Some` distinct
    /// wire cases, copied out of the pre-consolidation baseline dump.
    #[test]
    fn optional_field_rows_keep_their_pre_migration_bytes() {
        let hex = |command: &CadCommand| -> String { protocol::OpBinary::encode_op(command).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect() };
        assert_eq!(hex(&CadCommand::AddObject(add_object::AddObject { typology: Some("spatial.shape.primitive.box".into()) })), "0100011b7370617469616c2e73686170652e7072696d69746976652e626f7801000600");
        assert_eq!(hex(&CadCommand::AddObject(add_object::AddObject { typology: None })), "01000000");
        assert_eq!(
            hex(&CadCommand::PatchObject(patch_object::PatchObject { object_id: "object-1".into(), field: "origin.x".into(), value: Some("1.5".into()), delta: Some(2.5) })),
            "01010303312e35086f626a6563742d31086f726967696e2e780400060101060202060003050000000000000440"
        );
        assert_eq!(hex(&CadCommand::PatchObject(patch_object::PatchObject { object_id: "object-1".into(), field: "origin.x".into(), value: None, delta: None })), "010102086f626a6563742d31086f726967696e2e7802000600010601");
        assert_eq!(
            hex(&CadCommand::SetHover(set_hover::SetHover { object_id: Some("object-1".into()), mode: Some("edge".into()), id: Some(3) })),
            "0119020465646765086f626a6563742d3103000601010600020403"
        );
        assert_eq!(hex(&CadCommand::SetHover(set_hover::SetHover { object_id: None, mode: None, id: None })), "01190000");
        assert_eq!(
            hex(&CadCommand::WorldPick(world_pick::WorldPick { id: Some(7), merge: "replace".into(), granularity: "edge".into(), object_id: Some("object-1".into()), surface_id: Some("cad.play.scene3d/building".into()), pane: Some("building".into()) })),
            "011a05086275696c64696e67196361642e706c61792e7363656e6533642f6275696c64696e670465646765086f626a6563742d31077265706c61636506000407010604020602030603040601050600"
        );
        assert_eq!(hex(&CadCommand::WorldPick(world_pick::WorldPick { id: None, merge: "replace".into(), granularity: "mesh".into(), object_id: None, surface_id: None, pane: None })), "011a02046d657368077265706c61636502010601020600");
        assert_eq!(hex(&CadCommand::EngagementAbort(engagement_abort::EngagementAbort {})), "01210000");
        assert_eq!(hex(&CadCommand::ToggleSun(toggle_sun::ToggleSun {})), "01240000");
        assert_eq!(hex(&CadCommand::SaveSelected(save_selected::SaveSelected {})), "012b0000");
        assert_eq!(hex(&CadCommand::LoadRawRequest(load_raw_request::LoadRawRequest {})), "012e0000");
    }

    #[test]
    fn forest_example_uses_per_object_brep_meshes() {
        let scene = forest_play_scene();
        let runtime = CadPlayRuntime::default();
        let json = edit::world_instances_json(&scene.building_objects, &runtime);
        assert!(json.contains("object-hexagonal-cut-concrete-forest-left-bim-10"));
        let meshes = edit::world_meshes_json(&scene.building_objects, scene.building_geometry.as_ref());
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
            geometry: semio_framework_core::DwgGeometry::PolyfaceMesh { vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]], faces: vec![[1, 2, 3, 4]] },
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
        let min_z = mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
        assert!(min_z > 2.5, "energy world mesh min z {min_z}");
        let slab = roundtrip.structure_classic_objects.iter().find(|object| object.primitives.iter().any(|primitive| primitive.kind == "surface")).expect("structure surface");
        let slab_mesh = object_mesh_data(slab, roundtrip.structure_classic_geometry.as_ref());
        let slab_min_z = slab_mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
        assert!(slab_min_z > 2.5, "structure world mesh min z {slab_min_z}");
    }

    #[test]
    fn forest_references_use_xy_ground_plane_and_z_up() {
        let scene = forest_play_scene();
        let reference = scene.references_by_model_definition_id.get(CAD_MODEL_DEFINITION_ENERGY).and_then(|references| references.first()).expect("energy reference");
        assert_eq!(reference.origin[2], CAD_FOREST_REFERENCE_PLANE_Z, "reference must stay on the CAD ground datum");
        assert!((reference.origin[0] - (-9.7)).abs() < 1e-9, "reference x {} should be base + 50% width (right)", reference.origin[0]);
        let expected_y = -18.0 + CAD_FOREST_REFERENCE_WIDTH_WORLD * CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX / CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX * (0.5 + CAD_FOREST_REFERENCE_Y_OFFSET_RATIO);
        assert!((reference.origin[1] - expected_y).abs() < 1e-9, "reference CAD y {} should be centered then moved +20% forward on the world plane", reference.origin[1]);
        let centered_y = -18.0 + CAD_FOREST_REFERENCE_WIDTH_WORLD * CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX / CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX * 0.5;
        assert!(((reference.origin[1] - centered_y) - CAD_FOREST_REFERENCE_WIDTH_WORLD * CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX / CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX * 0.2).abs() < 1e-9, "the requested offset must affect CAD y only");
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
        for vertex in mesh.positions.as_chunks_mut::<3>().0 {
            vertex[2] = 0.0;
        }
        align_mesh_to_fixture_centroid(&mut mesh, geometry, &object.primitives);
        let min_z = mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
        assert!(min_z > 2.5, "aligned mesh min z {min_z}");
    }

    #[test]
    fn forest_surface_meshes_use_authored_height_without_pane_geometry() {
        let scene = forest_play_scene();
        let energy = scene.energy_objects.first().expect("energy object");
        let energy_mesh = object_mesh_data(energy, None);
        let energy_min_z = energy_mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
        assert!(energy_min_z > 2.5, "energy mesh must stay at authored z without pane geometry, got min_z={energy_min_z}");
        let slab = scene.structure_classic_objects.iter().find(|object| object.primitives.iter().any(|primitive| primitive.kind == "surface")).expect("structure surface");
        let slab_mesh = object_mesh_data(slab, None);
        let slab_min_z = slab_mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
        assert!(slab_min_z > 2.5, "structure slab must stay at authored z without pane geometry, got min_z={slab_min_z}");
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
        for body_key in [shape::BODY_KEY, building::BODY_KEY, energy::BODY_KEY, structure_classic::BODY_KEY] {
            let node = render_direct(&app, body_key, &doc, &CadConfig::default());
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("world-3d"), "body {body_key} should render a world-3d scene");
        }
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

    /// 🧱️ The manifest stitch: every window kind / panel tab the taxonomy nodes export lands in the
    /// built `AppDefinition` with the same id, body key, surface kind and (empty) manifest measures the
    /// pre-consolidation scalar `.window_kind(..)`/`.panel_tab(..)` calls produced — measures stay
    /// config-derived per frame via `DocumentApp::window_measures`, never frozen into the manifest.
    #[test]
    fn manifest_stitches_every_taxonomy_node_with_its_pre_migration_shape() {
        let definition = create_cad_app().definition;
        let windows: Vec<(&str, &str)> = definition.window_kinds.iter().map(|window| (window.id.as_str(), window.body_key.as_str())).collect();
        assert_eq!(
            windows,
            vec![
                (shape::WINDOW_KIND_ID, shape::BODY_KEY),
                (building::WINDOW_KIND_ID, building::BODY_KEY),
                (energy::WINDOW_KIND_ID, energy::BODY_KEY),
                (structure_classic::WINDOW_KIND_ID, structure_classic::BODY_KEY),
            ]
        );
        for window in definition.window_kinds.iter() {
            assert_eq!(window.surface_kind, ui_wgpu::SurfaceKind::World3d, "window {} surface kind", window.id);
            assert!(window.options.measures.is_empty(), "window {} must not freeze measures into the manifest", window.id);
        }
        let modes: Vec<&str> = definition.modes.iter().map(|mode| mode.id.as_str()).collect();
        assert_eq!(modes, vec![edit::CAD_PLAY_MODE_EDIT]);
        assert_eq!(definition.default_mode_id, edit::CAD_PLAY_MODE_EDIT);
        // 🕰️ The framework appends its own history tab after the app-declared ones.
        let panels: Vec<(&str, Option<&str>)> = definition.panel_tabs.iter().map(|tab| (tab.id(), tab.body_key.as_deref())).take(3).collect();
        assert_eq!(
            panels,
            vec![
                (semio_framework_plugin::FRAMEWORK_PANEL_TAB_DOCUMENT_ID, Some(document::CAD_PLAY_BODY_DOCUMENT)),
                (semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID, Some(catalogue::CAD_PLAY_BODY_CATALOGUE)),
                (semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID, Some(inspection::CAD_PLAY_BODY_PROPERTIES)),
            ]
        );
        let layout_json = serde_json::to_string(&edit::layout()).expect("layout json");
        for window_kind_id in [shape::WINDOW_KIND_ID, building::WINDOW_KIND_ID, energy::WINDOW_KIND_ID, structure_classic::WINDOW_KIND_ID] {
            assert!(layout_json.contains(window_kind_id), "default quad layout must place {window_kind_id}: {layout_json}");
        }
        assert_eq!(definition.artifact_kinds.iter().map(|kind| kind.id.as_str()).collect::<Vec<_>>(), vec!["3d.cad"]);
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
            let action = definition.actions.iter().find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("action {action_id} missing from manifest"));
            assert!(!action.in_palette, "internal action {action_id} must have in_palette: false");
        }

        let palette_user_actions = ["addObject", "deleteObject", "duplicateObject", "translateSelection", "rotateSelection", "scaleSelection"];
        for action_id in palette_user_actions {
            let action = definition.actions.iter().find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("user action {action_id} missing from manifest"));
            assert!(action.in_palette, "user action {action_id} must have in_palette: true");
        }
    }

    #[test]
    fn engagement_input_and_possible_engagements_present() {
        let mut app = new_app();
        let engagements = app.window_engagements();
        let shape = engagements.get(shape::WINDOW_KIND_ID).expect("shape engagement");
        assert!(shape.input.is_some());
        assert!(shape.possible_engagements.as_ref().is_some_and(|rows| !rows.is_empty()));
    }

    #[test]
    fn window_engagements_registered_for_all_four_panes() {
        let mut app = new_app();
        let engagements = app.window_engagements();
        for window_kind in [shape::WINDOW_KIND_ID, building::WINDOW_KIND_ID, energy::WINDOW_KIND_ID, structure_classic::WINDOW_KIND_ID] {
            assert!(engagements.contains_key(window_kind), "missing engagement for {window_kind}");
        }
    }

    #[test]
    fn forest_example_includes_reference_overlay() {
        let scene = forest_play_scene();
        let references = edit::world_references_json(&scene, CadPaneId::Shape).expect("references");
        assert!(references.contains("ref-concrete-forest"));
    }

    #[test]
    fn typology_extent_derives_from_authored_geometry() {
        let scene = forest_play_scene();
        let column = scene.building_objects.iter().find(|object| object.typology == "building.building.column").expect("column object");
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
        let emit = drive(&app, &scene, "worldSelect", Some(json!({ "ids": ["object-box-1"], "merge": "replace" })));
        let runtime = runtime_after(&emit, &CadConfig::default());
        let selection = edit::world_selection_json(&scene, &runtime, Some(CAD_DISLOCATE_UTILITY_ID), CadDislocateOptions::default());
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
        let emit = drive(&app, &scene, "setCamera", Some(json!({ "surfaceId": "cad.play.scene3d/building", "camera": { "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0], "zoom": 2.0, "fov": 60.0 } })));
        assert!(emit.document_operations.is_empty(), "setCamera must not emit a VCS operation");
        assert!(!emit.config_operations.is_empty(), "setCamera must write a config operation");
        let runtime = runtime_after(&emit, &CadConfig::default());
        assert_eq!(cad_pane_camera_runtime(&runtime, CadPaneId::Building).zoom, 2.0);
        assert_eq!(cad_pane_camera_runtime(&runtime, CadPaneId::Shape).zoom, 1.0, "panes stay isolated");
    }

    #[test]
    fn gumball_inactive_without_selection() {
        let selection = edit::world_selection_json(&default_document(), &CadPlayRuntime::default(), Some(CAD_DISLOCATE_UTILITY_ID), CadDislocateOptions::default());
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
        let node = render_direct(&app, shape::BODY_KEY, &doc, &config);
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
        let emit = drive_with_config(&app, &scene, "worldSelect", Some(json!({ "ids": ["object-box-1"], "merge": "replace" })), &base_config);
        let config = config_after(&emit, &base_config);
        let history = empty_history();
        let doc = DocumentView { projection: &scene, history: &history };
        let shape = render_direct(&app, shape::BODY_KEY, &doc, &config);
        let building = render_direct(&app, building::BODY_KEY, &doc, &config);
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

        let emit = drive(&app, &scene, "worldSelect", Some(json!({ "ids": ["object-box-1"], "merge": "replace" })));
        let config = config_after(&emit, &empty_config);
        let items = context_menu_direct(&app, &doc, &config, &registry);
        assert!(items.iter().any(|item| item.id == "translateSelection" && item.label.is_some()), "labels must resolve from the registry: {items:?}");
        assert!(items.iter().any(|item| item.id == "deleteObject" && item.destructive == Some(true)), "deleteObject must be marked destructive: {items:?}");
    }

    /// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: the selection context menu stays a shallow,
    /// disclosed list (top-level verbs + a handful of taxonomy groups) rather than a flat wall of rows,
    /// and the destructive `deleteObject` action stays the trailing item.
    #[test]
    fn context_menu_is_grouped_and_keeps_delete_object_last() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let history = empty_history();
        let doc = DocumentView { projection: &scene, history: &history };
        let registry = AppActionRegistry::from_definition(&create_cad_app().definition);
        let empty_config = CadConfig::default();

        let emit = drive(&app, &scene, "worldSelect", Some(json!({ "ids": ["object-box-1"], "merge": "replace" })));
        let config = config_after(&emit, &empty_config);
        let items = context_menu_direct(&app, &doc, &config, &registry);

        assert!(items.len() <= 9, "top-level context menu should stay progressively disclosed: {items:?}");
        assert_eq!(items.last().map(|item| item.id.as_str()), Some("deleteObject"), "deleteObject must stay the trailing item: {items:?}");
        assert_eq!(items.last().and_then(|item| item.destructive), Some(true), "trailing deleteObject must be marked destructive: {items:?}");
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
            measures.get(window_id).and_then(|items| {
                items.iter().find_map(|measure| match measure {
                    WindowMeasure::Group { id, children, .. } if id == "cad-play-utility-options-dislocate" => children.iter().find_map(|child| match child {
                        WindowMeasure::Toggle { id, pressed, .. } if id == "cad-dislocate-rotate" => Some(*pressed),
                        _ => None,
                    }),
                    _ => None,
                })
            })
        };
        assert_eq!(rotate_pressed(shape::WINDOW_KIND_ID), Some(true));
        assert_eq!(rotate_pressed(building::WINDOW_KIND_ID), Some(false));
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
        app.dispatch_typed(CadCommand::AddObject(add_object::AddObject { typology: Some("spatial.shape.primitive.box".into()) }), &meta("local")).expect("add object");
        let projection_after_add = serde_json::to_string(&app.projection().expect("projection")).unwrap();
        let result = app.dispatch_typed(CadCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: CAD_DISLOCATE_UTILITY_ID.into() }), &meta("local")).expect("set active utility");
        assert!(result.operations.is_empty(), "utility switch must emit zero operations");
        let projection_after_switch = serde_json::to_string(&app.projection().expect("projection")).unwrap();
        assert_eq!(projection_after_add, projection_after_switch, "utility switch must not mutate the projection");
        app.handle_action("undo", None, &meta("local")).expect("undo");
        assert_eq!(app.projection().expect("projection").objects.len(), before, "a single undo reverts the addObject — proving the utility switch created no history entry");
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
        for window_kind in [shape::WINDOW_KIND_ID, building::WINDOW_KIND_ID, energy::WINDOW_KIND_ID, structure_classic::WINDOW_KIND_ID] {
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
        let emit = drive(&app, &scene, "worldPick", Some(json!({ "surfaceId": "cad.play.scene3d/building", "id": 1, "merge": "replace" })));
        let runtime = runtime_after(&emit, &CadConfig::default());
        assert_eq!(runtime.selected_object_ids.to_vec(), vec![expected_id]);
        assert_eq!(runtime.component_selection.mode, "mesh");
    }

    #[test]
    fn set_hover_edge_round_trips_hovered_component() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let object_id = scene.objects.iter().find(|object| object.visible).expect("visible").id.clone();
        let emit = drive(&app, &scene, "setHover", Some(json!({ "objectId": object_id, "mode": "edge", "id": 3 })));
        let runtime = runtime_after(&emit, &CadConfig::default());
        let selection = edit::world_selection_json(&scene, &runtime, None, CadDislocateOptions::default());
        assert!(selection.contains("\"hoveredComponent\""));
        assert!(selection.contains("\"mode\":\"edge\""));
        assert!(selection.contains("\"id\":3"));
        assert!(selection.contains("\"edge\":true"), "edge targets must stay enabled: {selection}");
        let instances = edit::world_instances_json(&scene.objects, &runtime);
        assert!(instances.contains("\"hovered\":false"), "edge hover must not tint the whole mesh surface: {instances}");
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
        let selection = edit::world_selection_json(&scene, &runtime, None, CadDislocateOptions::default());
        assert!(selection.contains("\"selectionMode\":\"edge\""));
        assert!(selection.contains("\"componentIds\":[7]"));
        assert!(selection.contains(&format!("\"activeObjectId\":\"{object_id}\"")));
    }

    #[test]
    fn marquee_set_selection_commits_component_ids() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let object_id = scene.objects.iter().find(|object| object.visible).expect("visible").id.clone();
        let emit = drive(
            &app,
            &scene,
            "setSelection",
            Some(json!({
                "mode": "edge",
                "ids": [3, 9],
                "objectId": object_id,
                "merge": "replace"
            })),
        );
        let runtime = runtime_after(&emit, &CadConfig::default());
        assert_eq!(runtime.component_selection.mode, "edge");
        assert_eq!(runtime.component_selection.ids, vec![3, 9]);
        assert_eq!(runtime.active_object_id.as_deref(), Some(object_id.as_str()));
        let selection = edit::world_selection_json(&scene, &runtime, None, CadDislocateOptions::default());
        assert!(selection.contains("\"componentIds\":[3,9]"));
    }

    #[test]
    fn world_pick_curve_centerline_selects_whole_object() {
        let app = CadPlayApp::default();
        let scene = forest_play_scene();
        let curve = scene.structure_classic_objects.iter().find(|object| object.visible && primary_primitive_kind(object) == "curve").expect("structure classic curve object");
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
        let emit = drive_with_config(&app, &scene, "setHover", Some(json!({ "objectId": object_id, "mode": "edge", "id": 0 })), &config_after_pick);
        let runtime = runtime_after(&emit, &config_after_pick);
        assert_eq!(runtime.hovered_target.as_ref().and_then(|target| target.mode.as_deref()), Some("mesh"), "curve hover must promote to instance mesh hover");
        let instances = edit::world_instances_json(&scene.structure_classic_objects, &runtime);
        assert!(instances.contains(&format!("\"id\":\"{object_id}\"")) && instances.contains("\"hovered\":true"), "curve instance must show hovered: {instances}");
    }

    //#endregion 🔖️ViewState
    //#region 🔖️Operations
    #[test]
    fn add_object_action_appends_object_and_selects_it() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let emit = drive(&app, &scene, "addObject", Some(json!({ "typology": "building.building.column" })));
        assert_eq!(emit.document_operations.len(), 1);
        let next = apply_operations(&scene, &emit.document_operations);
        assert!(next.objects.iter().any(|object| object.typology == "building.building.column") || next.building_objects.iter().any(|object| object.typology == "building.building.column"));
        let runtime = runtime_after(&emit, &CadConfig::default());
        assert_eq!(runtime.selected_object_ids.len(), 1);
    }

    #[test]
    fn add_object_through_wrapper_grows_projection() {
        let mut app = new_app();
        let before = app.projection().expect("projection").objects.len();
        app.dispatch_typed(CadCommand::AddObject(add_object::AddObject { typology: Some("spatial.shape.primitive.box".into()) }), &meta("local")).expect("add object");
        assert_eq!(app.projection().expect("projection").objects.len(), before + 1);
    }

    #[test]
    fn focus_model_definition_emits_document_operation() {
        let mut app = new_app();
        app.dispatch_typed(CadCommand::FocusModelDefinition(focus_model_definition::FocusModelDefinition { model_definition_id: "aec.building".into() }), &meta("local")).expect("focus model definition");
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
        assert!(next.energy_objects.iter().all(|object| !fixture_energy_ids.contains(&object.id)), "live single-box derive should not repopulate the static forest energy fixture's original objects");
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
        let emit = drive(&app, &scene, "importCadFile", Some(json!({ "payload": file_text, "name": "cad.spatial.json" })));
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
        let obj_data_url = format!("data:model/obj;base64,{}", base64::engine::general_purpose::STANDARD.encode(obj_text));
        let emit = drive(&app, &scene, "importCadFile", Some(json!({ "payload": obj_data_url, "name": "triangle.obj" })));
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
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(&mut app, CadCommand::AddObject(add_object::AddObject { typology: Some("spatial.shape.primitive.box".into()) }), |app| app.projection().expect("projection").objects.len(), before, before + 1);
    }

    #[test]
    fn undo_redo_round_trips_added_node_through_wrapper() {
        let mut app = new_app();
        let before = app.projection().expect("projection").nodes.len();
        app.dispatch_typed(CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }), &meta("local")).expect("add node");
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
        app.dispatch_typed(CadCommand::AddObject(add_object::AddObject { typology: Some("spatial.shape.primitive.box".into()) }), &meta("local")).expect("add object");
        let object_id = app.projection().expect("projection").objects.last().unwrap().id.clone();
        let origin_before = app.projection().expect("projection").objects.iter().find(|object| object.id == object_id).unwrap().origin;
        for _ in 0..3 {
            app.dispatch_typed(CadCommand::TranslateSelection(translate_selection::TranslateSelection { object_ids: vec![object_id.clone()], dx: 1.0, dy: 0.0, dz: 0.0 }), &meta("local")).expect("translate tick");
        }
        let dragged = app.projection().expect("projection").objects.iter().find(|object| object.id == object_id).unwrap().origin;
        assert_eq!(dragged[0], origin_before[0] + 3.0, "three coalesced ticks accumulate");
        // One undo reverts the whole coalesced drag back to the pre-drag origin (not one tick).
        app.handle_action("undo", None, &meta("local")).expect("undo drag");
        let after_undo = app.projection().expect("projection").objects.iter().find(|object| object.id == object_id).unwrap().origin;
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
        base.objects = vec![make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape), make_object_for_typology("spatial.shape.primitive.box", 1, CadPaneId::Shape)];
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
        instance_a.dispatch_typed(CadCommand::PatchObject(patch_object::PatchObject { object_id: object_a.clone(), field: "label".into(), value: Some("Renamed By A".into()), delta: None }), &meta("actor-a")).expect("a renames object a");

        // B renames object B — a disjoint edit that must survive alongside A's.
        instance_b.dispatch_typed(CadCommand::PatchObject(patch_object::PatchObject { object_id: object_b.clone(), field: "label".into(), value: Some("Renamed By B".into()), delta: None }), &meta("actor-b")).expect("b renames object b");

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
        sender.dispatch_typed(CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }), &meta("local")).expect("add node");

        let mut envelopes = Vec::new();
        for message in far.receive().expect("receive") {
            if let BackboneMessage::Operations { envelopes: operations } = message {
                envelopes.extend(operations);
            }
        }
        assert!(!envelopes.is_empty(), "expected the applied operation to flow onto the channel");
        let operations = envelopes;

        let mut receiver = new_app();
        let nodes_before = receiver.projection().expect("projection").nodes.len();
        receiver.ingest_operations(&operations).expect("ingest once");
        receiver.ingest_operations(&operations).expect("ingest twice");
        assert_eq!(receiver.projection().expect("projection").nodes.len(), nodes_before + 1, "feeding the same operation twice must not double-apply");
    }
    //#endregion 🔖️Convergence
}
//#endregion 🧪️Tests
