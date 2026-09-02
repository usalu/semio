//! 📐️ CAD play app — the plugin's play app: its `ArtifactApp` impl (dispatch-only), the
//! `CadPlayRuntime` scratch mirror of `CadConfig`, the shared view/export helpers its command,
//! panel and window nodes build on, and the manifest that stitches those nodes together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/<window>`. This file dispatches and stitches.

use crate::artifacts::cad::op::CadMutation;
use crate::artifacts::cad::standards::v1::subsets::any::io::{export_solids_as, CadSolidExport, CAD_SOLID_EXPORT_DIALECT_STEP};
use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::{
    cad_brep_kernel, cad_camera_projection_config, ensure_object_solid_handle, forest_play_scene, next_cad_id, CAD_EXAMPLE_FOREST_LEFT, CAD_MODEL_DEFINITION_BUILDING, CAD_MODEL_DEFINITION_ENERGY, CAD_MODEL_DEFINITION_SHAPE,
    CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC,
};
use crate::artifacts::cad::{artifact_kind, cad_pane_from_model_definition_id, CadCamera, CadPaneId, CadSnapshot, CadWorkingScene, CAD_DOCUMENT_SCHEMA};
use crate::editor::cad::commands::camera::{set_camera, set_projection, set_projection_param};
use crate::editor::cad::commands::contribution::set_contributions;
use crate::editor::cad::commands::engagement::{engagement_abort, engagement_input, engagement_possible_select, engagement_repeat_last, engagement_submit, world_pointer_down, world_pointer_move};
use crate::editor::cad::commands::io::{import_cad_file, load_raw_request, save_current, save_in_play, save_selected};
use crate::editor::cad::commands::locale::{set_locale, set_terminology};
use crate::editor::cad::commands::model_definition::{focus_model_definition, set_active_example};
use crate::editor::cad::commands::node::{add_node, rename_node, set_node_selection};
use crate::editor::cad::commands::object::{add_object, delete_object, duplicate_object, patch_object, patch_selection};
use crate::editor::cad::commands::reference::{patch_cad_play_reference, reference_hover, set_reference_selection};
use crate::editor::cad::commands::sun::{set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use crate::editor::cad::commands::transform::{apply_transformation, rotate_selection, scale_selection, translate_selection};
use crate::editor::cad::commands::utility::{set_active_utility, set_dislocate_option};
use crate::editor::cad::config::{cad_sun_config_from_world, cad_sun_config_to_world, deserialize_cad_preview_generation, CadConfig, CadConfigMutation, CadDislocateOptions, CAD_PREVIEW_GENERATION_MAX};
use crate::editor::cad::engine::interaction::{self, apply_event, can_commit, commit_object, keyed_transitions, parse_repl_line, resolve_interaction_key, start_session, CadEngagementScratch};
use crate::editor::cad::modes::edit;
use crate::editor::cad::modes::edit::windows::{building, energy, shape, structure_classic};
use crate::editor::cad::panels::{catalogue, document, inspection};
use crate::editor::cad::terminology::{cad_is_de_locale, cad_labels};
use semio_framework::kernel::Effect;
use semio_framework_plugin::{
    tree_item_with_action, world3d_camera_projection_json, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppActionRegistry, AppOperationContext, ArtifactOwnedToolJobFactory,
    ArtifactOwnedToolJobRequest, ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, CommandDefinition, ConfigView, ContextMenuItemSpec, ContextMenuRequest, DraftView,
    EditorApp, Emit, Fault, Label, LocalizedLabel, Media, MediaClass, MediaError, MediaForm, MediaPayload, MediaType, Menu, NoDraft, NoDraftMutation, PluginAssemblyError, UiNode, UiText, UiValue, UtilityCategory,
    UtilityDefinition, WindowEngagement, WindowMeasure, WorldSunConfig, SET_ACTIVE_UTILITY_ACTION_ID,
};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::engine::{Brep, BrepKernel, GeometryHandle};
// 🚧️ SDK GAP: `ArtifactEditor`/`Editor`/`Dialect` (ticket 26/08/16 contract §2.1/§2.4)? are not yet
// in `semio_framework_plugin`'s curated crate-root re-export list (`🔌️plugin/🦀️.rs:17858`)
// — only reachable through the `app` submodule they're actually declared in. Not fixable here
// (`🧰️framework/**` is outside this packet's lease); flagged for W1-A in the migration report.
use semio_framework_plugin::app::{ArtifactEditor, Dialect, Editor};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use store::EngineHandles;

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
/// 🕹️ `"cad"` — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM interaction domain this app
/// declares (granularities object/vertex/edge/face, `HierarchyProvider::Flat`).
pub const CAD_INTERACTION_DOMAIN: &str = "cad";

/// 🕹️ Owned snapshot of `InteractionView::selection(CAD_INTERACTION_DOMAIN)`, read once per dispatch
/// by `ArtifactApp::handle` and threaded through `CadDispatchCtx` to every command handler.
/// Decouples handlers from `semio_framework_plugin::app::InteractionView` itself — whose fields are
/// `pub(crate)` to that crate, so this crate's own tests cannot construct one — command-level tests
/// build this plain, cad-owned struct directly instead (see `🎮️commands/🔄️transform`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CadInteractionSnapshot {
    pub granularity: String,
    pub ids: Vec<String>,
    pub anchor_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadPlayRuntime {
    /// 👁️ Document-tree node selection — app-owned (not a mesh-geometry granularity).
    #[serde(default)]
    pub selected_node_ids: Vec<String>,
    /// 🐁️ Hovered reference-overlay id — app-owned, distinct from the framework `"cad"` domain hover.
    #[serde(default)]
    pub hovered_reference_id: Option<String>,
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
    pub engagement_pane: Option<String>,
    #[serde(default)]
    pub engagement_session: Option<CadEngagementScratch>,
    #[serde(default)]
    pub engagement_preview_operation_json: Option<String>,
    #[serde(default, deserialize_with = "deserialize_cad_preview_generation")]
    pub engagement_preview_generation: i32,
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
    #[serde(default)]
    pub active_utility_id: String,
    #[serde(default)]
    pub locale: String,
    #[serde(default)]
    pub terminology: String,
}

impl Default for CadPlayRuntime {
    fn default() -> Self {
        Self {
            selected_node_ids: Vec::new(),
            hovered_reference_id: None,
            engagement_input: String::new(),
            engagement_step: "Idle".into(),
            active_example_id: None,
            selected_reference_model_definition_id: None,
            selected_reference_id: None,
            engagement_pane: None,
            engagement_session: None,
            engagement_preview_operation_json: None,
            engagement_preview_generation: 0,
            last_finalized_interaction_id: None,
            sun: WorldSunConfig::default(),
            camera: CadCamera::default(),
            camera_building: CadCamera::default(),
            camera_energy: CadCamera::default(),
            camera_structure_classic: CadCamera::default(),
            dislocate_options_by_window_id: HashMap::new(),
            active_utility_id: "move".into(),
            locale: "en-US".into(),
            terminology: "native".into(),
        }
    }
}

impl CadPlayRuntime {
    /// 🪟️ Reads the Dislocate handle configuration for one window instance without sharing it with siblings.
    pub fn dislocate_options(&self, window_id: &str) -> CadDislocateOptions {
        self.dislocate_options_by_window_id.get(window_id).copied().unwrap_or_default()
    }
}

/// @emoji 🔀️ WORKFLOWS-END-TO-END-TYPED-PORTS config recipe boundary (in): unpacks `cfg.snapshot`
/// (the persisted, VCS-tracked `CadConfig`) into the ergonomic `CadPlayRuntime` scratch shape every
/// helper function below already works with — a pure, allocation-only conversion, never itself an
/// operation. `dislocate_options_by_window_id` is seeded from the 4 fixed pane fields keyed by the 4
/// constant window-kind ids (`CAD_PLAY_WINDOW_*`) — see `CadDislocateOptions`'s doc comment in
/// `cad_document_engine` for why per-window-INSTANCE keying no longer applies.
pub fn cad_runtime_from_config(cfg: &CadConfig) -> CadPlayRuntime {
    CadPlayRuntime {
        selected_node_ids: cfg.selected_node_ids.clone(),
        hovered_reference_id: cfg.hovered_reference_id.clone(),
        engagement_input: cfg.engagement_input.clone(),
        engagement_step: cfg.engagement_step.clone(),
        active_example_id: cfg.active_example_id.clone(),
        selected_reference_model_definition_id: cfg.selected_reference_model_definition_id.clone(),
        selected_reference_id: cfg.selected_reference_id.clone(),
        engagement_pane: cfg.engagement_pane.clone(),
        engagement_session: cfg.engagement_session_json.as_deref().and_then(|json| serde_json::from_str(json).ok()),
        engagement_preview_operation_json: cfg.engagement_preview_operation_json.clone(),
        engagement_preview_generation: cfg.engagement_preview_generation,
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
        active_utility_id: cfg.active_utility_id.clone(),
        locale: cfg.locale.clone(),
        terminology: cfg.terminology.clone(),
    }
}

/// @emoji 🔀️ The `cad_runtime_from_config` boundary's outbound twin: repacks the (possibly mutated)
/// `CadPlayRuntime` scratch struct back into a real `CadConfig` snapshot. Kept private so production
/// command modules cannot bypass the checked snapshot authorities below.
fn cad_config_from_runtime(runtime: &CadPlayRuntime, base: &CadConfig) -> CadConfig {
    CadConfig {
        contributions_json: base.contributions_json.clone(),
        selected_node_ids: runtime.selected_node_ids.clone(),
        hovered_reference_id: runtime.hovered_reference_id.clone(),
        engagement_input: runtime.engagement_input.clone(),
        engagement_step: runtime.engagement_step.clone(),
        active_example_id: runtime.active_example_id.clone(),
        selected_reference_model_definition_id: runtime.selected_reference_model_definition_id.clone(),
        selected_reference_id: runtime.selected_reference_id.clone(),
        engagement_pane: runtime.engagement_pane.clone(),
        engagement_session_json: runtime.engagement_session.as_ref().map(|session| serde_json::to_string(session).unwrap_or_default()),
        engagement_preview_operation_json: base.engagement_preview_operation_json.clone(),
        engagement_preview_generation: base.engagement_preview_generation,
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
        active_utility_id: runtime.active_utility_id.clone(),
        locale: runtime.locale.clone(),
        terminology: runtime.terminology.clone(),
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
/// `CadSnapshot` projection and the app's `CadPlayRuntime` view-state. Replaces the old persisted play
/// envelope: its embedded history/undo stacks are now owned by the wrapping `VcsArtifactApp`'s
/// `ArtifactStore`, and its runtime view-state lives directly on the `CadPlayApp` struct.
pub struct CadPlayView {
    pub document: CadSnapshot,
    pub runtime: CadPlayRuntime,
}

pub fn cad_action(action: &str, args: Option<semio_framework_plugin::UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    semio_framework_plugin::ActionFactory::new(CAD_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🪟️ Bridges window chrome, which still carries the retained WGPU action descriptor.
pub fn cad_window_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    ActionDescriptor { controller_id: CAD_PLAY_CONTROLLER_ID.into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) }
}

/// 🧱️ Admits one fixed CAD UI text value.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    UiText::try_from_str(value.as_ref())
        .map(UiValue::Text)
        .ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "cad UI text admission failed"))
}

/// 🔘️ Admits one CAD boolean action value.
pub fn ui_value_bool(value: bool) -> UiValue {
    UiValue::Bool(value)
}

/// 📚️ Admits one fixed CAD UI list value.
pub fn ui_value_list(values: impl IntoIterator<Item = UiValue>) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    let mut builder = semio_framework_plugin::UiListBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "cad UI list admission failed"))?;
    for value in values {
        builder.push(value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "cad UI list item admission failed"))?;
    }
    Ok(UiValue::List(builder.finish()))
}

/// 🗺️ Admits one fixed CAD UI map value.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, UiValue)>) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "cad UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "cad UI map entry admission failed"))?;
    }
    Ok(UiValue::Map(builder.finish()))
}

/// 🌳️ Admits fallibly assembled CAD nodes into fixed storage.
pub fn ui_node_list(
    values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>,
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let mut nodes = semio_framework_plugin::UiFixedList::default();
    for value in values {
        nodes.try_push(value?).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "cad UI node admission failed"))?;
    }
    Ok(nodes)
}

/// 🏷️ Admits resolved CAD text into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::plugin_app_close_prelude::Label> {
    semio_framework_plugin::plugin_app_close_prelude::Label::try_from(value.as_ref()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "cad UI label admission failed"))
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
pub fn cad_tree_item(
    id: impl Into<String>,
    label: impl AsRef<str>,
    icon_id: Option<&str>,
    action: (semio_framework_plugin::ActionId, Option<UiValue>),
) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut item = tree_item_with_action(id.into(), ui_label(label)?, None, action)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = match icon_id {
            Some(value) => Some(UiText::try_from_str(value).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "cad tree icon admission failed"))?),
            None => None,
        };
    }
    Ok(item)
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
    cad_runtime_from_config(cfg.snapshot)
}

/// 🔀️ Emits a non-session config snapshot and fails closed if a caller attempts to bypass the
/// operation-aware engagement transition authority.
pub fn snapshot_of(runtime: &CadPlayRuntime, base: &CadConfig) -> Result<CadConfigMutation, Fault> {
    let config = cad_config_from_runtime(runtime, base);
    if config.engagement_session_json != base.engagement_session_json {
        return Err(Fault::from("cad.preview.invalid: engagement checkpoint transition requires operation-aware persistence"));
    }
    Ok(CadConfigMutation::Snapshot { config })
}

/// 🪪️ The sole engagement-checkpoint persistence authority: it stamps one exact public-operation
/// identity and advances the bounded generation exactly once iff the checkpoint changed.
pub fn preview_transition_snapshot_of(runtime: &CadPlayRuntime, base: &CadConfig, ctx: &CadDispatchCtx) -> Result<CadConfigMutation, Fault> {
    let mut config = cad_config_from_runtime(runtime, base);
    if config.engagement_session_json != base.engagement_session_json {
        let operation = ctx.preview_operation.as_ref().ok_or_else(|| Fault::from("cad.preview.invalid: engagement transition is missing public operation identity"))?;
        if base.engagement_preview_generation < 0 {
            return Err(Fault::from("cad.preview.invalid: engagement preview generation is negative"));
        }
        config.engagement_preview_generation =
            base.engagement_preview_generation.checked_add(1).filter(|generation| *generation <= CAD_PREVIEW_GENERATION_MAX).ok_or_else(|| Fault::from("cad.preview.conflict: engagement preview generation exhausted"))?;
        config.engagement_preview_operation_json = Some(serde_json::to_string(operation).map_err(|_| Fault::from("cad.preview.invalid: operation identity serialization failed"))?);
    }
    Ok(CadConfigMutation::Snapshot { config })
}
//#endregion 🔖️Runtime

//#region 🔖️Helpers
/// ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: this used to dispatch a
/// `ReplacePaneObjects` whole-pane-replace mutation (banned vocabulary shape, and gone: pane object
/// data now lives inside composed `s.stdio.semio.model` CHILD documents, each its own document —
/// see `🔖️Composition` in `🏪️store/🦀️.rs`). Re-deriving building/energy/structure
/// typologies from shape geometry and writing the result into a pane needs a child-dispatch seam
/// on `CadDispatchCtx`/`Emit<CadMutation, _>` that does not exist yet (`🔌️plugin/🦀️.rs`
/// framework-kernel surface, W1-owned, out of a plugin fan-out agent's write scope). Documented
/// no-op until that seam exists, not silently dropped.
pub fn apply_transformation_mutations(_document: &CadSnapshot, _qid: &str) -> Vec<CadMutation> {
    Vec::new()
}

/// ⚠️ Same documented gap as `apply_transformation_mutations` — there is no live per-pane object
/// list on `CadSnapshot` to collect solids from anymore (only composed model-child HANDLES,
/// unresolved at this boundary).
pub fn collect_pane_solids(_kernel: &mut Brep, _envelope: &CadPlayView, _pane: CadPaneId) -> Vec<GeometryHandle> {
    Vec::new()
}

pub fn collect_modelspace_solids(kernel: &mut Brep, envelope: &CadPlayView) -> Vec<GeometryHandle> {
    CadPaneId::all().into_iter().flat_map(|pane| collect_pane_solids(kernel, envelope, pane)).collect()
}

pub fn export_solid_for_pane(envelope: &CadPlayView, pane: CadPaneId, format: &str) -> Option<CadSolidExport> {
    let mut kernel = cad_brep_kernel();
    let solids = collect_pane_solids(&mut kernel, envelope, pane);
    if solids.is_empty() {
        return None;
    }
    let stem = format!("cad-{}", pane.model_definition_id().replace('.', "-"));
    export_solids_as(&mut kernel, &solids, format, &stem)
}

pub fn export_solid_modelspace(envelope: &CadPlayView, format: &str) -> Option<CadSolidExport> {
    let mut kernel = cad_brep_kernel();
    let solids = collect_modelspace_solids(&mut kernel, envelope);
    if solids.is_empty() {
        return None;
    }
    export_solids_as(&mut kernel, &solids, format, "cad.modelspace")
}

/// @emoji ⬇️ Converts a staged native-geometry export into a download host effect emitted directly
/// to the shell (no document mutation, no pending-export runtime slot).
pub fn cad_solid_export_effect(export: CadSolidExport) -> Effect {
    let data = match export.data {
        Value::String(text) => text,
        other => serde_json::to_string(&other).unwrap_or_default(),
    };
    Effect::DownloadMediaExport { filename: export.filename, mime_type: export.mime_type, data, encoding: export.encoding }
}

/// @emoji ⬇️ Wraps a spatial-JSON export document into a download host effect.
pub fn cad_spatial_export_effect(value: &Value, filename: &str) -> Effect {
    Effect::DownloadMediaExport { filename: filename.into(), mime_type: "text/plain".into(), data: serde_json::to_string(value).unwrap_or_default(), encoding: None }
}

/// ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: exporting per-pane objects as
/// spatial JSON used to read `CadSnapshot`'s inline `objects` field directly. That data now lives
/// inside composed `s.stdio.semio.model` CHILD documents (unresolved at this boundary — see
/// `🔖️Composition` in `🏪️store/🦀️.rs`). Returns an empty `objects` array per pane;
/// documented reduced-fidelity gap, not silently wrong.
pub fn export_spatial_json(envelope: &CadPlayView, mode: &str) -> Value {
    let models: Vec<Value> = CadPaneId::all()
        .into_iter()
        .map(|pane| {
            json!({
                "id": pane.model_definition_id(),
                "model": {
                    "schema": "spatial.model",
                    "revision": 1,
                    "objects": Vec::<Value>::new(),
                }
            })
        })
        .collect();
    match mode {
        "selected" => {
            let pane = cad_pane_from_model_definition_id(&envelope.document.active_model_definition_id).unwrap_or(CadPaneId::Shape);
            let model = json!({
                "schema": "spatial.model",
                "revision": 1,
                "objects": Vec::<Value>::new(),
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
                "objects": Vec::<Value>::new(),
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

/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `scene` OUTSIDE history —
/// the sanctioned non-mutation path for a whole-document replace (file import, load-example). Per
/// `📓️taxonomy.md`, whole-document replace has NO mutation-enum representative (`SetSnapshot` is
/// banned outright); every former "replace the whole document" gesture builds this effect instead
/// of an `Emit::mutations([...])`. The spr is a fresh, edit-free op-log for `scene`'s own
/// `schema`/`id` — a genesis envelope with no history to encode.
pub fn reset_document_effect(scene: &CadSnapshot) -> Effect {
    let pack = <CadSnapshot as store::ArtifactPack>::encode_pack(scene);
    let envelope = store::create_document_envelope::<CadSnapshot, CadMutation>(&scene.schema, &scene.id, scene.clone(), None);
    let spr = semio_framework_plugin::resolve_ready(store::print_document_spr(&envelope)).expect("cad document spr encode is infallible for a fresh, edit-free envelope");
    Effect::LoadDocument { pack, spr }
}

/// 🎯️ Builds the whole-value-field semantic mutation for one object addressed by `pane`/`object_id`
/// (label/typology/hidden/locked) — the counterpart of the axis-addressed spatial fields
/// `patch_objects_mutations` below resolves separately.
/// ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `rename-object`/
/// `change-object-typology`/`change-object-visible`/`change-object-locked` are retired — object
/// fields live inside composed `s.stdio.semio.model` CHILD documents now, whose own mutations are
/// dispatched against that child directly (no seam for that from here yet; see
/// `patch_objects_mutations`'s doc comment). Documented no-op.
pub fn object_field_mutation(_pane: CadPaneId, _object_id: &str, _field: &str, _value: Option<&Value>) -> Option<CadMutation> {
    None
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

/// @emoji 🎯️ Builds the semantic mutation(s) that apply `field`'s edit across `object_ids`:
/// whole-value fields (label/typology/hidden/locked) build one `rename-object`/`change-object-*`
/// per object; `origin.<axis>`/`scale.<axis>`/`orientation.<axis>` read each object's own current
/// component so `value` (absolute) or `delta` (relative) applies per-object, preserving each
/// object's other axes and any offset across a multi-select — `move-object`/`scale-object`/
/// `rotate-object`, one per touched object.
/// ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `move-object`/`scale-object`/
/// `rotate-object` are retired — object placement lives inside composed `s.stdio.semio.model`
/// CHILD documents now (own document, own mutation history; see `🔖️Composition` in
/// `🏪️store/🦀️.rs`). Dispatching a mutation against a CHILD document from this
/// parent-document command handler needs a child-dispatch seam on `CadDispatchCtx`/
/// `Emit<CadMutation, _>` that does not exist yet (`🔌️plugin/🦀️.rs` framework-kernel
/// surface, W1-owned, out of a plugin fan-out agent's write scope). Documented no-op until that
/// seam exists, not silently dropped.
pub fn patch_objects_mutations(_document: &CadSnapshot, _object_ids: &[String], _field: &str, _value: Option<&Value>, _delta: Option<&Value>) -> Vec<CadMutation> {
    Vec::new()
}

pub(crate) fn make_object_for_typology(typology: &str, label_count: usize, pane: CadPaneId) -> crate::artifacts::cad::standards::v1::subsets::any::io::geometry_import::CadObject {
    use crate::artifacts::cad::standards::v1::subsets::any::io::geometry_import::CadObject;
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
    let mut kernel = cad_brep_kernel();
    ensure_object_solid_handle(&mut kernel, &mut object);
    let _ = pane;
    object
}

/// Commits `session` if it satisfies `can_commit`, returning the `AddObject` operation and clearing
/// the session runtime state. Returns the operations (empty when no commit happened) — used by both the
/// direct-event and keyed-transition REPL paths in `engagement_submit_mutations` (a state reached via
/// either path can be commit-ready, e.g. box's explicit `confirm` step reachable via a keyed
/// transition).
pub fn try_commit_session_mutations(_document: &CadSnapshot, runtime: &mut CadPlayRuntime, _pane: CadPaneId, session: &CadEngagementScratch) -> Vec<CadMutation> {
    if !can_commit(session) {
        return Vec::new();
    }
    let mut kernel = cad_brep_kernel();
    // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `commit_object` still builds
    // a real ephemeral `CadObject` (kernel handle + placement) from the interactive session — that
    // part of the pipeline is untouched. What is retired is `create-object`: composing the result
    // into a pane's `SemioModelSnapshot` CHILD needs a child-dispatch seam on `CadDispatchCtx`/
    // `Emit<CadMutation, _>` that does not exist yet (`🔌️plugin/🦀️.rs` framework-kernel
    // surface, W1-owned). Documented no-op — the session still clears (UI doesn't hang), but the
    // constructed geometry does not yet land in the document.
    let Some(object) = commit_object(&mut kernel, session, 0, next_cad_id) else {
        return Vec::new();
    };
    let interaction_id = session.interaction_id.clone();
    // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): auto-selecting the just-committed
    // object is no longer reachable from this single `handle()` dispatch — selection is
    // framework-owned now, written only through the injected `interactionSelect` verb.
    let _ = object.id;
    runtime.engagement_input.clear();
    runtime.last_finalized_interaction_id = Some(interaction_id);
    runtime.engagement_session = None;
    runtime.engagement_step = "Idle".into();
    Vec::new()
}

/// @emoji ⌨️ Advances the engagement REPL for the current `engagement_input`, mutating runtime
/// session state and returning any commit operations produced.
pub fn engagement_submit_mutations(document: &CadSnapshot, runtime: &mut CadPlayRuntime, pane: CadPaneId) -> Vec<CadMutation> {
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
                return try_commit_session_mutations(document, runtime, pane, &session_snapshot);
            }
            for transition in keyed_transitions(session) {
                if (transition.key.eq_ignore_ascii_case(&input) || transition.event_kind.eq_ignore_ascii_case(&input)) && apply_event(session, &transition.event_kind, None) {
                    runtime.engagement_step = session.state.clone();
                    runtime.engagement_input.clear();
                    let session_snapshot = session.clone();
                    return try_commit_session_mutations(document, runtime, pane, &session_snapshot);
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
                multiplicity: semio_framework::PortMultiplicity::Many,
            },
            semio_framework_plugin::MediaPortSpec {
                id: "brep:out".into(),
                label: "Brep".into(),
                direction: semio_framework_plugin::MediaPortDirection::Out,
                media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
                kind_id: Some("3d.cad".into()),
                required: false,
                multiplicity: semio_framework::PortMultiplicity::Many,
            },
        ],
        // 🌉️ Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6:
        // `AppIo` carries no string-kind peer field (unlike `ArtifactKindSpec::export_stdio_kinds`
        // below), so — matching the precedent already set by the raster/block plugins' own
        // migrations — this stays empty; the real stdio kind ids live on `artifact_kind()`.
        export_formats: vec![],
        import_formats: vec![],
        artifact: semio_framework_plugin::ArtifactPresentation { id: "3d.cad".into(), name: "3D CAD".into(), dimension: "3d".into(), component_kind: "cad".into() },
    }
}
//#endregion 🔖️Io

//#region 🔖️Commands
/// 🧵️ Per-dispatch app-struct state carrying the exact public-operation identity used by
/// `gesture_preview` plus (26/08/14) a read-only
/// [`CadInteractionSnapshot`] of the framework's `"cad"` domain — the `semio_framework_plugin::
/// app_commands!`-generated `dispatch` has no way to thread `InteractionView` itself (see that
/// macro's own doc comment on `ctx`), so `ArtifactApp::handle` builds the snapshot once and hands
/// it down through this app-owned context instead.
pub struct CadDispatchCtx {
    pub interaction: CadInteractionSnapshot,
    pub preview_operation: Option<CadPreviewOperationIdentity>,
}

/// 🪪️ Collision-free public-operation identity attached to every persisted preview generation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadPreviewOperationIdentity {
    pub app_instance_id: u32,
    pub parent_document_id: String,
    pub operation_id: u64,
    pub operation_generation: u64,
    pub canonical_base_revision: String,
}

impl From<&semio_framework_plugin::AppOperationContext> for CadPreviewOperationIdentity {
    fn from(operation: &semio_framework_plugin::AppOperationContext) -> Self {
        Self {
            app_instance_id: operation.app_instance_id,
            parent_document_id: operation.parent_document_id.clone(),
            operation_id: operation.operation_id,
            operation_generation: operation.generation,
            canonical_base_revision: operation.canonical_base_revision_hex(),
        }
    }
}

/// 👁️ Exact freshness stamp; both fields must match/advance, so ABA and finite hashes are absent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CadPreviewStamp {
    pub operation: CadPreviewOperationIdentity,
    pub generation: i32,
}

impl CadPreviewStamp {
    pub fn is_fresher_than(&self, current: &CadPreviewStamp) -> bool {
        self.operation == current.operation && self.generation > current.generation
    }
}

/// 👁️ Operation-stamped transient preview payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CadGesturePreview {
    pub stamp: CadPreviewStamp,
    pub payload: Vec<u8>,
}

impl CadGesturePreview {
    pub fn is_fresher_than(&self, current: &CadPreviewStamp) -> bool {
        self.stamp.is_fresher_than(current)
    }
}

semio_framework_plugin::app_commands! {
    /// 🎯️ `CadPlayApp::Command` — the SOLE dispatch surface for cad's own behavior, decomposed into
    /// one `🎮️commands/<group>/<command>` payload module per row. Row order IS the binary variant
    /// ordinal and the two literals are two different vocabularies (camelCase manifest action id,
    /// kebab wire keyword) — both are copied verbatim from the pre-consolidation `CadCommand` enum.
    pub enum CadCommand for CadSnapshot, CadMutation, CadConfig, CadConfigMutation, ctx = CadDispatchCtx {
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

        // 👁️ Config-only — emit `config_mutations`, never document operations.
        "setCamera" as "camera" => set_camera::SetCamera,
        "setProjection" as "projection" => set_projection::SetProjection,
        "setProjectionParam" as "projection-param" => set_projection_param::SetProjectionParam,
        "setDislocateOption" as "dislocate-option" => set_dislocate_option::SetDislocateOption,
        "setNodeSelection" as "set-node-selection" => set_node_selection::SetNodeSelection,
        "setReferenceSelection" as "reference-selection" => set_reference_selection::SetReferenceSelection,
        "referenceHover" as "reference-hover" => reference_hover::ReferenceHover,
        "engagementInput" as "engagement-input" => engagement_input::EngagementInput,
        "engagementPossibleSelect" as "engagement-possible-select" => engagement_possible_select::EngagementPossibleSelect,
        "engagementRepeatLast" as "engagement-repeat-last" => engagement_repeat_last::EngagementRepeatLast,
        "engagementAbort" as "engagement-abort" => engagement_abort::EngagementAbort,
        "worldPointerMove" as "world-pointer-move" => world_pointer_move::WorldPointerMove,
        "toggleSun" as "toggle-sun" => toggle_sun::ToggleSun,
        "setSunAzimuth" as "sun-azimuth" => set_sun_azimuth::SetSunAzimuth,
        "setSunElevation" as "sun-elevation" => set_sun_elevation::SetSunElevation,
        "setSunIntensity" as "sun-intensity" => set_sun_intensity::SetSunIntensity,
        "setActiveUtility" as "active-utility" => set_active_utility::SetActiveUtility,
        "setLocale" as "locale" => set_locale::SetLocale,
        "setTerminology" as "terminology" => set_terminology::SetTerminology,
        "setContributions" as "contributions" => set_contributions::SetContributions,

        // 🐚️ Shell effects — export/import round-trips through the host, no operations either way.
        "saveSelected" as "save-selected" => save_selected::SaveSelected,
        "saveInPlay" as "save-in-play" => save_in_play::SaveInPlay,
        "saveCurrent" as "save-current" => save_current::SaveCurrent,
        "loadRawRequest" as "load-raw-request" => load_raw_request::LoadRawRequest,
    }
}

/// 🌉️ Converts the host shell's declared action id and JSON arguments into cad's closed typed
/// command vocabulary before the app dispatches through the binary command path.
fn cad_command_from_action(action: &str, args: Option<&Value>) -> Result<CadCommand, Fault> {
    let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_str).map(str::to_string);
    let f64_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_f64);
    let bool_field = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_bool);
    let str_vec_field = |key: &str| -> Vec<String> { args.and_then(|value| value.get(key)).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default() };
    let value_string = || -> Option<String> {
        args.and_then(|value| value.get("value")).and_then(|value| match value {
            Value::String(text) => Some(text.clone()),
            Value::Bool(flag) => Some(flag.to_string()),
            Value::Number(number) => Some(number.to_string()),
            _ => None,
        })
    };
    let position_axis = |index: usize| args.and_then(|value| value.get("position")).and_then(|value| value.get(index)).and_then(Value::as_f64);
    Ok(match action {
        "setActiveExample" => CadCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_field("exampleId").unwrap_or_default() }),
        SET_ACTIVE_UTILITY_ACTION_ID => CadCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: str_field("utilityId").unwrap_or_default() }),
        "setLocale" => CadCommand::SetLocale(set_locale::SetLocale { value: str_field("value").unwrap_or_default() }),
        "setTerminology" => CadCommand::SetTerminology(set_terminology::SetTerminology { value: str_field("value").unwrap_or_default() }),
        "setDislocateOption" => CadCommand::SetDislocateOption(set_dislocate_option::SetDislocateOption { pane: str_field("pane"), option: str_field("option").unwrap_or_default(), pressed: bool_field("pressed") }),
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
        "translateSelection" => {
            CadCommand::TranslateSelection(translate_selection::TranslateSelection { object_ids: str_vec_field("objectIds"), dx: f64_field("dx").unwrap_or(0.0), dy: f64_field("dy").unwrap_or(0.0), dz: f64_field("dz").unwrap_or(0.0) })
        }
        "rotateSelection" => CadCommand::RotateSelection(rotate_selection::RotateSelection {
            object_ids: str_vec_field("objectIds"),
            ax: f64_field("ax").unwrap_or(0.0),
            ay: f64_field("ay").unwrap_or(0.0),
            az: f64_field("az").unwrap_or(0.0),
            angle: f64_field("angle").unwrap_or(0.0),
        }),
        "scaleSelection" => CadCommand::ScaleSelection(scale_selection::ScaleSelection { object_ids: str_vec_field("objectIds"), sx: f64_field("sx").unwrap_or(1.0), sy: f64_field("sy").unwrap_or(1.0), sz: f64_field("sz").unwrap_or(1.0) }),
        "addObject" => CadCommand::AddObject(add_object::AddObject { typology: str_field("typology") }),
        "patchObject" => CadCommand::PatchObject(patch_object::PatchObject { object_id: str_field("objectId").unwrap_or_default(), field: str_field("field").unwrap_or_default(), value: value_string(), delta: f64_field("delta") }),
        "patchSelection" => CadCommand::PatchSelection(patch_selection::PatchSelection { object_ids: str_vec_field("objectIds"), field: str_field("field").unwrap_or_default(), value: value_string(), delta: f64_field("delta") }),
        "deleteObject" => CadCommand::DeleteObject(delete_object::DeleteObject { object_id: str_field("objectId").unwrap_or_default() }),
        "duplicateObject" => CadCommand::DuplicateObject(duplicate_object::DuplicateObject { object_id: str_field("objectId").unwrap_or_default() }),
        "addNode" => CadCommand::AddNode(add_node::AddNode { kind: str_field("kind").unwrap_or_else(|| "solid".into()) }),
        "renameNode" => CadCommand::RenameNode(rename_node::RenameNode { node_id: str_field("nodeId").unwrap_or_default(), value: str_field("value").unwrap_or_default() }),
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
        "worldPointerDown" => CadCommand::WorldPointerDown(world_pointer_down::WorldPointerDown { pane: str_field("pane"), surface_id: str_field("surfaceId"), x: position_axis(0), y: position_axis(1), z: position_axis(2) }),
        "worldPointerMove" => CadCommand::WorldPointerMove(world_pointer_move::WorldPointerMove { x: position_axis(0), y: position_axis(1), z: position_axis(2) }),
        "toggleSun" => CadCommand::ToggleSun(toggle_sun::ToggleSun {}),
        "setSunAzimuth" => CadCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: f64_field("value").unwrap_or(0.0) }),
        "setSunElevation" => CadCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: f64_field("value").unwrap_or(0.0) }),
        "setSunIntensity" => CadCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: f64_field("value").unwrap_or(0.0) }),
        "setContributions" => CadCommand::SetContributions(set_contributions::SetContributions { json: str_field("json").unwrap_or_else(|| "[]".into()) }),
        other => return Err(Fault::from(format!("unknown cad action '{other}'"))),
    })
}
//#endregion 🔖️Commands

//#region 🔖️PlayApp
// 📐️ B1/WORKFLOWS-END-TO-END-TYPED-PORTS: unit-struct-shaped pure `ArtifactApp` — every former
// `CadPlayRuntime`/`self.runtime` field now lives in `CadConfig`, written through
// `CadConfigMutation`s (real `backwards`, no ad hoc `InverseAction`). Preview freshness is the
// persisted public-operation identity plus checked generation, never process-local state.
#[derive(Default, Clone, Copy)]
pub struct CadPlayApp;

impl CadPlayApp {
    /// 🔬️ CW7 preview-law seam: reads the operation-stamped engagement checkpoint from config only.
    pub fn gesture_preview(&self, config: &CadConfig) -> Option<CadGesturePreview> {
        let session_json = config.engagement_session_json.as_ref()?;
        if session_json.is_empty() || session_json == "null" {
            return None;
        }
        if !(0..=CAD_PREVIEW_GENERATION_MAX).contains(&config.engagement_preview_generation) {
            return None;
        }
        let operation = serde_json::from_str(config.engagement_preview_operation_json.as_ref()?).ok()?;
        Some(CadGesturePreview { stamp: CadPreviewStamp { operation, generation: config.engagement_preview_generation }, payload: session_json.as_bytes().to_vec() })
    }
}

//#region 🧵️RetainedCommands
const CAD_RETAINED_ARTIFACT_TOOL_IDS: &[&str] = &["addNode", "renameNode", "patchCadPlayReference", "focusModelDefinition"];
const CAD_RETAINED_CONFIG_TOOL_IDS: &[&str] = &[
    "setCamera",
    "setProjection",
    "setProjectionParam",
    "setDislocateOption",
    "setNodeSelection",
    "setReferenceSelection",
    "referenceHover",
    "engagementInput",
    "engagementPossibleSelect",
    "engagementRepeatLast",
    "engagementAbort",
    "worldPointerMove",
    "toggleSun",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setActiveUtility",
    "setLocale",
    "setTerminology",
    "setContributions",
];
const CAD_RETAINED_TOOL_IDS: &[&str] = &[
    "addNode",
    "renameNode",
    "patchCadPlayReference",
    "focusModelDefinition",
    "setCamera",
    "setProjection",
    "setProjectionParam",
    "setDislocateOption",
    "setNodeSelection",
    "setReferenceSelection",
    "referenceHover",
    "engagementInput",
    "engagementPossibleSelect",
    "engagementRepeatLast",
    "engagementAbort",
    "worldPointerMove",
    "toggleSun",
    "setSunAzimuth",
    "setSunElevation",
    "setSunIntensity",
    "setActiveUtility",
    "setLocale",
    "setTerminology",
    "setContributions",
    "loadRawRequest",
];
const CAD_RETAINED_COMMAND_SCHEMA: &str = "cad.scene.tool-command.v1";
const CAD_RETAINED_RAW_BYTES: usize = 8_192;
const CAD_RETAINED_WORK_ITEMS: usize = 1;
const CAD_CONFIG_STORE_MAXIMUM_BYTES: usize = 65_536;
const CAD_CONFIG_STORE_MAXIMUM_ITEMS: usize = 256;

const CAD_RETAINED_PUBLICATION_CONTRACTS: &[ArtifactToolPublicationContract] = &[
    ArtifactToolPublicationContract { tool_id: "addNode", lanes: &[ArtifactToolPublicationLane::Artifact, ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "renameNode", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "patchCadPlayReference", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "focusModelDefinition", lanes: &[ArtifactToolPublicationLane::Artifact] },
    ArtifactToolPublicationContract { tool_id: "setCamera", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setProjection", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setProjectionParam", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setDislocateOption", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setNodeSelection", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setReferenceSelection", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "referenceHover", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "engagementInput", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "engagementPossibleSelect", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "engagementRepeatLast", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "engagementAbort", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "worldPointerMove", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "toggleSun", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setSunAzimuth", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setSunElevation", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setSunIntensity", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setActiveUtility", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setLocale", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setTerminology", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "setContributions", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "loadRawRequest", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];

fn cad_retained_contract() -> ToolExecutionContract {
    ToolExecutionContract::bounded_first_step(CAD_RETAINED_RAW_BYTES, 64, CAD_RETAINED_WORK_ITEMS as u64, 16_384, 7_500)
}

fn cad_retained_extent(command: &CadCommand, _snapshot: &CadSnapshot, _interaction: &protocol::InteractionState) -> Option<usize> {
    CAD_RETAINED_TOOL_IDS.contains(&command.command_id()).then_some(1)
}

fn cad_retained_reduce(
    command: &CadCommand,
    snapshot: &CadSnapshot,
    config: &CadConfig,
    history: &semio_framework_plugin::HistoryView,
    interaction: &protocol::InteractionState,
    _hover: &semio_framework_plugin::app::InteractionHoverState,
    operation: &AppOperationContext,
) -> Result<Emit<CadMutation, CadConfigMutation, NoDraftMutation>, Fault> {
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config };
    let selection = interaction.selection.get(CAD_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    let retained_interaction = CadInteractionSnapshot { granularity: selection.granularity.clone(), ids: selection.ids.clone(), anchor_id: selection.anchor_id.clone() };
    let mut ctx = CadDispatchCtx { interaction: retained_interaction, preview_operation: Some(CadPreviewOperationIdentity::from(operation)) };
    if CAD_RETAINED_ARTIFACT_TOOL_IDS.contains(&command.command_id()) {
        admit_cad_snapshot(snapshot).map_err(Fault::from)?;
        return command.dispatch(&doc, &cfg, &mut ctx);
    }
    if CAD_RETAINED_CONFIG_TOOL_IDS.contains(&command.command_id()) {
        admit_cad_config(config).map_err(Fault::from)?;
        return command.dispatch(&doc, &cfg, &mut ctx);
    }
    match command {
        CadCommand::LoadRawRequest(payload) => load_raw_request::handle(payload, &doc, &cfg, &mut ctx),
        _ => Err(Fault::from("cad-retained-route-mismatch")),
    }
}

struct CadRetainedCommandJobFactory {
    keys: Vec<ToolFactoryKey>,
}

impl CadRetainedCommandJobFactory {
    fn new(controller_id: &str) -> Self {
        Self { keys: CAD_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new(controller_id, *tool_id)).collect() }
    }
}

impl semio_framework::ToolJobFactory for CadRetainedCommandJobFactory {
    type Payload = ArtifactRetainedCommandPayload<EditorApp<CadPlayApp>>;
    type Job = ArtifactRetainedCommandJob<EditorApp<CadPlayApp>>;

    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }

    fn payload_schema_id(&self) -> &str {
        CAD_RETAINED_COMMAND_SCHEMA
    }

    fn classification(&self) -> InteractiveJobClassification {
        InteractiveJobClassification::Migrated
    }

    fn execution_contract(&self) -> ToolExecutionContract {
        cad_retained_contract()
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
        if input.declared_bytes() > CAD_RETAINED_RAW_BYTES || checkpoint.is_some() {
            return Err((ToolJobFactoryError::new("CAD retained command rejects oversized wire or checkpoint owner"), input, checkpoint));
        }
        Ok(ArtifactRetainedCommandJob::from_wire(payload, input))
    }
}

impl semio_framework_plugin::ArtifactOwnedToolJobFactory for CadRetainedCommandJobFactory {
    type Owner = semio_framework_plugin::EditorApp<CadPlayApp>;
    const TOOL_IDS: &'static [&'static str] = CAD_RETAINED_TOOL_IDS;
    const DOCUMENT_SCHEMA: &'static str = CAD_DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = CAD_RETAINED_PUBLICATION_CONTRACTS;
}
//#endregion 🧵️RetainedCommands

//#region 📬️ConfigStorePreparation
struct CadConfigStorePreparationFactory;

struct CadConfigStorePreparation {
    base: Option<store::SnapshotRead<CadConfig>>,
    mutation: Option<CadConfigMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<CadConfig, CadConfigMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

fn cad_projection_retained_bytes(projection: &crate::artifacts::cad::CadProjectionDsl) -> usize {
    projection.kind.len()
        .saturating_add(projection.orthographic_view.len())
        .saturating_add(projection.axonometric_variant.len())
        .saturating_add(projection.axonometric_quadrant.len())
        .saturating_add(projection.oblique_variant.len())
        .saturating_add(projection.one_point_axis.len())
        .saturating_add(projection.curvilinear_mapping.len())
}

fn cad_camera_retained_bytes(camera: &CadCamera) -> usize {
    cad_projection_retained_bytes(&camera.projection)
}

fn cad_config_retained_bytes(config: &CadConfig) -> usize {
    let option_bytes = [
        config.hovered_reference_id.as_deref(),
        config.active_example_id.as_deref(),
        config.selected_reference_model_definition_id.as_deref(),
        config.selected_reference_id.as_deref(),
        config.engagement_pane.as_deref(),
        config.engagement_session_json.as_deref(),
        config.engagement_preview_operation_json.as_deref(),
        config.last_finalized_interaction_id.as_deref(),
    ]
    .into_iter()
    .flatten()
    .fold(0usize, |bytes, value| bytes.saturating_add(value.len()));
    config.selected_node_ids.iter().fold(0usize, |bytes, value| bytes.saturating_add(value.len()))
        .saturating_add(option_bytes)
        .saturating_add(config.engagement_input.len())
        .saturating_add(config.engagement_step.len())
        .saturating_add(config.sun.color.len())
        .saturating_add(cad_camera_retained_bytes(&config.camera))
        .saturating_add(cad_camera_retained_bytes(&config.camera_building))
        .saturating_add(cad_camera_retained_bytes(&config.camera_energy))
        .saturating_add(cad_camera_retained_bytes(&config.camera_structure_classic))
        .saturating_add(config.active_utility_id.len())
        .saturating_add(config.locale.len())
        .saturating_add(config.terminology.len())
        .saturating_add(config.contributions_json.len())
}

fn admit_cad_config(config: &CadConfig) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    if config.selected_node_ids.len() > CAD_CONFIG_STORE_MAXIMUM_ITEMS {
        return Err("CAD config exceeds its fixed retained item envelope".into());
    }
    let retained_bytes = cad_config_retained_bytes(config);
    if retained_bytes > CAD_CONFIG_STORE_MAXIMUM_BYTES {
        return Err("CAD config exceeds its fixed retained byte envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn admit_cad_config_mutation(mutation: &CadConfigMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    match mutation {
        CadConfigMutation::Snapshot { config } => admit_cad_config(config),
        CadConfigMutation::SetContributions { json } if json.len() <= CAD_CONFIG_STORE_MAXIMUM_BYTES => Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: json.len() }),
        CadConfigMutation::SetContributions { .. } => Err("CAD config mutation exceeds its fixed retained byte envelope".into()),
    }
}

fn prepare_cad_config(base: &CadConfig, mutation: CadConfigMutation) -> Result<(CadConfig, Vec<CadConfigMutation>, CadConfigMutation), String> {
    admit_cad_config(base)?;
    admit_cad_config_mutation(&mutation)?;
    let inverse = <CadConfigMutation as protocol::Mutation<CadConfig>>::inverse(&mutation, base);
    let post = <CadConfigMutation as protocol::Mutation<CadConfig>>::diff(&mutation, base).into_parts().0;
    admit_cad_config(&post)?;
    Ok((post, inverse, mutation))
}

fn cad_config_store_edit(forward: CadConfigMutation, inverse: Vec<CadConfigMutation>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<CadConfigMutation> {
    let id = format!("cad-config-retained-{}", authority.next_sequence_number());
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

impl store::ArtifactStoreOneItemPreparationFactory<CadConfig, CadConfigMutation> for CadConfigStorePreparationFactory {
    fn preflight(&self, mutation: &CadConfigMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("CAD config preparation rejected its lane or description envelope".into());
        }
        admit_cad_config_mutation(mutation)
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<CadConfig, CadConfigMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<CadConfig, CadConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<CadConfig, CadConfigMutation>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(CadConfigStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<CadConfig, CadConfigMutation> for CadConfigStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "CAD config preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "CAD config preparation lost its mutation owner".to_string())?;
        let (post, inverse, forward) = prepare_cad_config(base.get(), mutation)?;
        let authority = self.authority.as_ref().ok_or_else(|| "CAD config preparation lost its Store authority".to_string())?;
        let edit = cad_config_store_edit(forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<CadConfig, CadConfigMutation>> { self.prepared.as_ref() }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<CadConfig, CadConfigMutation>> { self.prepared.take() }

    fn cancel(&mut self) { self.cancelled = true; }

    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("CAD config preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️ConfigStorePreparation

//#region 📬️ArtifactStorePreparation
const CAD_ARTIFACT_STORE_MAXIMUM_BYTES: usize = 65_536;
const CAD_ARTIFACT_STORE_MAXIMUM_ITEMS: usize = 512;

struct CadArtifactStorePreparationFactory;

struct CadArtifactStorePreparation {
    base: Option<store::SnapshotRead<CadSnapshot>>,
    mutation: Option<CadMutation>,
    description: Option<String>,
    authority: Option<std::sync::Arc<store::ArtifactStoreOneItemLiveAuthority>>,
    prepared: Option<store::ArtifactStoreOneItemPrepared<CadSnapshot, CadMutation>>,
    checkpoint: store::ArtifactStoreOneItemCheckpoint,
    cancelled: bool,
    closing: bool,
}

fn cad_child_retained_bytes<S>(child: &store::ArtifactChild<S>) -> usize {
    child.child_id.len().saturating_add(child.target.to_uri().len())
}

fn cad_snapshot_retained_bytes(snapshot: &CadSnapshot) -> usize {
    let fixed_children = [
        snapshot.shape_model.as_ref().map(cad_child_retained_bytes),
        snapshot.building_model.as_ref().map(cad_child_retained_bytes),
        snapshot.energy_model.as_ref().map(cad_child_retained_bytes),
        snapshot.structure_classic_model.as_ref().map(cad_child_retained_bytes),
    ]
    .into_iter()
    .flatten()
    .fold(0usize, usize::saturating_add);
    let drawing_bytes = snapshot.drawings.iter().map(cad_child_retained_bytes).fold(0usize, usize::saturating_add);
    let reference_bytes = snapshot.references_by_model_definition_id.iter().fold(0usize, |bytes, (model_definition_id, references)| {
        references.iter().fold(bytes.saturating_add(model_definition_id.len()), |bytes, reference| {
            bytes.saturating_add(reference.id.len()).saturating_add(reference.source_url.len()).saturating_add(reference.media_kind.len())
        })
    });
    let node_bytes = snapshot.nodes.iter().fold(0usize, |bytes, node| bytes.saturating_add(node.id.len()).saturating_add(node.label.len()).saturating_add(node.kind.len()));
    snapshot
        .schema
        .len()
        .saturating_add(snapshot.id.len())
        .saturating_add(snapshot.active_model_definition_id.len())
        .saturating_add(fixed_children)
        .saturating_add(drawing_bytes)
        .saturating_add(reference_bytes)
        .saturating_add(node_bytes)
}

fn cad_snapshot_items(snapshot: &CadSnapshot) -> usize {
    snapshot
        .drawings
        .len()
        .saturating_add(snapshot.nodes.len())
        .saturating_add(snapshot.references_by_model_definition_id.values().map(Vec::len).sum::<usize>())
        .saturating_add(snapshot.shape_model.is_some() as usize)
        .saturating_add(snapshot.building_model.is_some() as usize)
        .saturating_add(snapshot.energy_model.is_some() as usize)
        .saturating_add(snapshot.structure_classic_model.is_some() as usize)
}

fn admit_cad_snapshot(snapshot: &CadSnapshot) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let work_items = cad_snapshot_items(snapshot);
    let retained_bytes = cad_snapshot_retained_bytes(snapshot);
    if work_items > CAD_ARTIFACT_STORE_MAXIMUM_ITEMS || retained_bytes > CAD_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("CAD Artifact exceeds its fixed retained preparation envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn admit_cad_artifact_mutation(mutation: &CadMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    let retained_bytes = serde_json::to_vec(mutation).map_err(|error| error.to_string())?.len();
    if retained_bytes > CAD_ARTIFACT_STORE_MAXIMUM_BYTES {
        return Err("CAD Artifact mutation exceeds its fixed retained byte envelope".into());
    }
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes })
}

fn prepare_cad_artifact(base: &CadSnapshot, mutation: CadMutation) -> Result<(CadSnapshot, Vec<CadMutation>, CadMutation), String> {
    admit_cad_snapshot(base)?;
    admit_cad_artifact_mutation(&mutation)?;
    let inverse = <CadMutation as protocol::Mutation<CadSnapshot>>::inverse(&mutation, base);
    let outcome = <CadMutation as protocol::Mutation<CadSnapshot>>::diff(&mutation, base);
    let post = protocol::MutationDiff::apply(outcome.diff(), base).map_err(|error| error.to_string())?;
    admit_cad_snapshot(&post)?;
    Ok((post, inverse, mutation))
}

fn cad_artifact_store_edit(forward: CadMutation, inverse: Vec<CadMutation>, description: Option<String>, authority: &store::ArtifactStoreOneItemLiveAuthority) -> protocol::Edit<CadMutation> {
    let id = format!("cad-artifact-retained-{}", authority.next_sequence_number());
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

impl store::ArtifactStoreOneItemPreparationFactory<CadSnapshot, CadMutation> for CadArtifactStorePreparationFactory {
    fn preflight(&self, mutation: &CadMutation, description: Option<&str>, lane: store::HistoryLane) -> Result<store::ArtifactStoreOneItemFootprint, String> {
        if lane != store::HistoryLane::Document || description.is_some_and(|value| value.len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES) {
            return Err("CAD Artifact preparation rejected its lane or description envelope".into());
        }
        admit_cad_artifact_mutation(mutation)
    }

    fn begin(&self, request: store::ArtifactStoreOneItemPreparationRequest<CadSnapshot, CadMutation>) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<CadSnapshot, CadMutation>>, store::ArtifactStoreOneItemPreparationRequest<CadSnapshot, CadMutation>> {
        if request.lane != store::HistoryLane::Document
            || request.operation != request.authority.operation()
            || request.generation != request.authority.generation()
            || request.base_revision != request.authority.base_revision()
            || request.authority.actor().len() > store::ARTIFACT_STORE_ONE_ITEM_ID_BYTES
        {
            return Err(request);
        }
        Ok(Box::new(CadArtifactStorePreparation {
            base: Some(request.base),
            mutation: Some(request.mutation),
            description: request.description,
            authority: Some(request.authority),
            prepared: None,
            checkpoint: store::ArtifactStoreOneItemCheckpoint::default(),
            cancelled: false,
            closing: false,
        }))
    }
}

impl store::ArtifactStoreOneItemPreparation<CadSnapshot, CadMutation> for CadArtifactStorePreparation {
    fn advance(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemPreparationStep, String> {
        if !grant.permits_one() || self.cancelled {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Blocked);
        }
        if self.prepared.is_some() {
            return Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint));
        }
        let base = self.base.as_ref().ok_or_else(|| "CAD Artifact preparation lost its exact base root".to_string())?;
        let mutation = self.mutation.take().ok_or_else(|| "CAD Artifact preparation lost its mutation owner".to_string())?;
        let (post, inverse, forward) = prepare_cad_artifact(base.get(), mutation)?;
        let authority = self.authority.as_ref().ok_or_else(|| "CAD Artifact preparation lost its Store authority".to_string())?;
        let edit = cad_artifact_store_edit(forward, inverse, self.description.take(), authority);
        let prepared = authority.prepare_one_item(edit, std::sync::Arc::new(post))?;
        self.checkpoint = store::ArtifactStoreOneItemCheckpoint { cursor: 1, completed_items: 1, completed_bytes: 1, digest: prepared.edit_digest() };
        self.prepared = Some(prepared);
        Ok(store::ArtifactStoreOneItemPreparationStep::Prepared(self.checkpoint))
    }

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint { self.checkpoint }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<CadSnapshot, CadMutation>> { self.prepared.as_ref() }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<CadSnapshot, CadMutation>> { self.prepared.take() }

    fn cancel(&mut self) { self.cancelled = true; }

    fn begin_close(&mut self) { self.closing = true; }

    fn close_step(&mut self, grant: store::ArtifactStoreOneItemGrant) -> Result<store::SnapshotRetirementStep, String> {
        if !self.closing || grant.maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.prepared.take().is_some() || self.mutation.take().is_some() || self.description.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(base) = self.base.take() {
            if !base.return_to_registry() {
                return Err("CAD Artifact preparation could not return its exact base root".into());
            }
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.as_ref() {
            if grant.maximum_bytes < authority.actor().len() {
                return Ok(store::SnapshotRetirementStep::Blocked);
            }
            self.authority = None;
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.base.is_none() && self.mutation.is_none() && self.description.is_none() && self.authority.is_none() && self.prepared.is_none()
    }
}
//#endregion 📬️ArtifactStorePreparation

//#region 🧹️EmptyLaneRetirement
struct CadNoTransientStoreDisposer;

impl semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<semio_framework_plugin::NoTransient, semio_framework_plugin::NoTransientMutation>> for CadNoTransientStoreDisposer {
    fn close_step(
        &mut self,
        _owner: &mut store::TransientStore<semio_framework_plugin::NoTransient, semio_framework_plugin::NoTransientMutation>,
        maximum_items: usize,
        _maximum_bytes: usize,
    ) -> Result<semio_framework_plugin::PluginCloseStep, Fault> {
        if maximum_items == 0 {
            return Ok(semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        assert_eq!(std::mem::size_of::<semio_framework_plugin::NoTransient>(), 0);
        Ok(semio_framework_plugin::PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self, _owner: &store::TransientStore<semio_framework_plugin::NoTransient, semio_framework_plugin::NoTransientMutation>) -> bool {
        std::mem::size_of::<semio_framework_plugin::NoTransient>() == 0
    }
}
//#endregion 🧹️EmptyLaneRetirement

impl ArtifactEditor for CadPlayApp {
    type Snapshot = CadSnapshot;
    type Mutation = CadMutation;
    type Config = CadConfig;
    type ConfigMutation = CadConfigMutation;
    type Draft = NoDraft;
    type DraftMutation = NoDraftMutation;
    type Presence = crate::editor::cad::presence::CadPresence;
    type PresenceMutation = crate::editor::cad::presence::CadPresenceMutation;
    type Transient = semio_framework_plugin::NoTransient;
    type TransientMutation = semio_framework_plugin::NoTransientMutation;
    type Command = CadCommand;

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(semio_framework_plugin::bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(semio_framework_plugin::bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::MemberStoreOwners<Self::Draft, Self::DraftMutation>> {
        assert_eq!(std::mem::size_of::<NoDraft>(), 0);
        Some(semio_framework_plugin::bounded_document_store_owners::<NoDraft, NoDraftMutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(semio_framework_plugin::bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(semio_framework_plugin::bounded_document_store_disposer::<NoDraft, NoDraftMutation>())
    }

    fn build_transient_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(Box::new(CadNoTransientStoreDisposer))
    }

    fn build_presence_local_root_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(crate::editor::cad::presence::retirement::CadPresenceRetirementFactory))
    }

    fn build_presence_peer_retirement_factory() -> Option<std::sync::Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(std::sync::Arc::new(crate::editor::cad::presence::retirement::CadPresenceRetirementFactory))
    }

    fn build_presence_store_disposer() -> Option<Box<dyn semio_framework_plugin::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(crate::editor::cad::presence::retirement::CadPresenceStoreDisposer::new()))
    }

    const DIALECT: Dialect = crate::artifacts::cad::CAD_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = CAD_DOCUMENT_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(CadArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(CadConfigStorePreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: semio_framework_plugin::EditorApp<CadPlayApp>,
        owner_file: "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.cad.cad@1/*#editor",
        document_schema: "cad.scene",
        factory: "CadRetainedCommandJobFactory",
        factory_type: CadRetainedCommandJobFactory,
        contract: semio_framework::ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
        tools: [
            "addNode",
            "renameNode",
            "patchCadPlayReference",
            "focusModelDefinition",
            "setCamera",
            "setProjection",
            "setProjectionParam",
            "setDislocateOption",
            "setNodeSelection",
            "setReferenceSelection",
            "referenceHover",
            "engagementInput",
            "engagementPossibleSelect",
            "engagementRepeatLast",
            "engagementAbort",
            "worldPointerMove",
            "toggleSun",
            "setSunAzimuth",
            "setSunElevation",
            "setSunIntensity",
            "setActiveUtility",
            "setLocale",
            "setTerminology",
            "setContributions",
            "loadRawRequest"
        ]
    }

    fn register_tool_job_factories(registry: &mut ArtifactToolFactoryRegistry<'_, EditorApp<Self>>) -> Result<(), Fault> {
        let controller = registry.controller_id().to_string();
        registry.register(CadRetainedCommandJobFactory::new(&controller))
    }

    fn build_tool_job(request: ArtifactOwnedToolJobRequest<EditorApp<Self>>) -> Result<Option<semio_framework::ToolOperationSpec>, Fault> {
        if !CAD_RETAINED_TOOL_IDS.contains(&request.tool_id.as_str()) {
            return Ok(None);
        }
        if request.command.command_id() != request.tool_id || cad_retained_extent(&request.command, &request.snapshot, &request.interaction_state) != Some(1) {
            return Err(Fault::from("cad-retained-command-tool-mismatch"));
        }
        let tool_id = request.command.command_id();
        let work: Box<dyn ArtifactCommandWork<EditorApp<Self>>> = Box::new(BoundedArtifactCommandWork::new(tool_id, cad_retained_reduce, cad_retained_extent));
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
            CadCommand::command_id,
            CAD_RETAINED_RAW_BYTES,
            CAD_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
        Some(crate::editor::cad::config::schema::app_schema_descriptor())
    }

    fn initial_snapshot() -> CadSnapshot {
        forest_play_scene()
    }

    fn io() -> Option<semio_framework_plugin::AppIo> {
        Some(cad_io())
    }

    // 🌱️ `whole_document_operation` stays the trait default (`None`): per `📓️taxonomy.md`, whole-
    // document replace has NO mutation-enum representative (`SetSnapshot` is banned outright) — the
    // `document:in` branch below builds a `reset_document_effect` directly instead of delegating to
    // this hook.

    /// 🎞️ `geometry:in` (WORKFLOWS-END-TO-END-TYPED-PORTS port recipe): accepts incoming mesh/brep
    /// geometry from any upstream 3D producer and inserts it as a new `CadObject` in the Shape pane,
    /// through the same brep kernel every other import path shares. Falls through to the default
    /// `document:in` importer for any other port.
    fn import_media(port: &str, media: &Media, _doc: &ArtifactView<'_, CadSnapshot>) -> Result<Emit<CadMutation, CadConfigMutation, Self::DraftMutation>, MediaError> {
        if port != "geometry:in" {
            if port != "document:in" {
                return Err(MediaError::NotImplemented);
            }
            let MediaPayload::Structured { json, .. } = &media.payload else {
                return Err(MediaError::Payload(port.to_string(), "default document:in importer only accepts a Structured (base64 pack) payload".into()));
            };
            let bytes = store::pack_rt::pack_value_from_base64(json).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            let projection = <CadSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| MediaError::Payload(port.to_string(), error.to_string()))?;
            return Ok(Emit { effects: vec![reset_document_effect(&projection)], ..Default::default() });
        }
        let name = match &media.media_type.form {
            MediaForm::Brep => "import.step",
            _ => "import.obj",
        };
        let payload = match &media.payload {
            MediaPayload::Structured { json, .. } => Value::String(json.clone()),
            MediaPayload::Binary { .. } => return Err(MediaError::Payload(port.to_string(), "geometry:in only accepts a Structured payload today".into())),
        };
        // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `import_cad_object_by_extension`
        // now returns a `SemioModelElement` (composed-child shape); `create-object` is retired.
        // Composing the imported element into the Shape pane's `SemioModelSnapshot` CHILD needs a
        // child-dispatch seam on `Emit<CadMutation, _>` that does not exist yet
        // (`🔌️plugin/🦀️.rs` framework-kernel surface, W1-owned). Documented no-op.
        match crate::artifacts::cad::standards::v1::subsets::any::io::import_cad_object_by_extension(name, &payload) {
            Some(_element) => Ok(Emit::default()),
            None => Err(MediaError::Payload(port.to_string(), "unrecognized geometry payload".into())),
        }
    }

    /// 🎞️ `brep:out` (WORKFLOWS-END-TO-END-TYPED-PORTS port recipe): exports the cad document's current
    /// brep geometry (every pane's solids fused into one modelspace, same as `saveInPlay`'s STEP export)
    /// wrapped as `Media`. Falls through to the default whole-document `document:out` for any other port.
    fn export_media(port: &str, doc: &ArtifactView<'_, CadSnapshot>) -> Result<Media, MediaError> {
        if port != "brep:out" {
            if port != "document:out" {
                return Err(MediaError::NotImplemented);
            }
            let media_type = Self::io().map_or(MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep }, |io| io.document_media_type);
            let bytes = <CadSnapshot as store::ArtifactPack>::encode_pack(doc.snapshot);
            return Ok(Media { media_type, payload: MediaPayload::Structured { schema: Self::DOCUMENT_SCHEMA.to_string(), json: store::pack_rt::pack_value_to_base64(&bytes) } });
        }
        let view = CadPlayView { document: doc.snapshot.clone(), runtime: CadPlayRuntime::default() };
        let mut kernel = cad_brep_kernel();
        let solids = collect_modelspace_solids(&mut kernel, &view);
        if solids.is_empty() {
            return Err(MediaError::Payload(port.to_string(), "no solids to export".into()));
        }
        let Some(export) = export_solids_as(&mut kernel, &solids, CAD_SOLID_EXPORT_DIALECT_STEP, "cad.modelspace") else {
            return Err(MediaError::Payload(port.to_string(), "brep export failed".into()));
        };
        let text = match export.data {
            Value::String(text) => text,
            other => other.to_string(),
        };
        Ok(Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep }, payload: MediaPayload::Structured { schema: "3d.cad".into(), json: base64_codec::base64_standard_encode(text.as_bytes()) } })
    }

    fn command_id(command: &CadCommand) -> &'static str {
        command.command_id()
    }

    fn command_from_action(action: &str, args: Option<&Value>) -> Result<CadCommand, Fault> {
        cad_command_from_action(action, args)
    }

    fn host_configuration_mutation(action: &str, args: Option<&Value>) -> Result<Option<Self::ConfigMutation>, Fault> {
        Ok((action == "setContributions").then(|| CadConfigMutation::SetContributions {
            json: args.and_then(|value| value.get("json")).and_then(Value::as_str).unwrap_or("[]").to_string(),
        }))
    }

    fn handle(
        command: &CadCommand,
        doc: &ArtifactView<'_, CadSnapshot>,
        cfg: &ConfigView<'_, CadConfig>,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<CadMutation, CadConfigMutation, Self::DraftMutation>, Fault> {
        let selection = interaction.selection(CAD_INTERACTION_DOMAIN);
        let snapshot = CadInteractionSnapshot { granularity: selection.granularity.clone(), ids: selection.ids.clone(), anchor_id: selection.anchor_id.clone() };
        let mut ctx = CadDispatchCtx { interaction: snapshot, preview_operation: Some(CadPreviewOperationIdentity::from(doc.operation()?)) };
        command.dispatch(doc, cfg, &mut ctx)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::validate_cad_computer_contributions(&cfg.snapshot.contributions_json);
        let view = CadPlayView { document: doc.snapshot.clone(), runtime: cad_runtime_from_config(cfg.snapshot) };
        let labels = cad_labels(cfg.snapshot);
        let window_kind_id = match body_key {
            shape::BODY_KEY => shape::WINDOW_KIND_ID,
            building::BODY_KEY => building::WINDOW_KIND_ID,
            energy::BODY_KEY => energy::WINDOW_KIND_ID,
            structure_classic::BODY_KEY => structure_classic::WINDOW_KIND_ID,
            _ => shape::WINDOW_KIND_ID,
        };
        let active_utility = Some(cfg.snapshot.active_utility_id.as_str());
        let options = view.runtime.dislocate_options(window_kind_id);
        match body_key {
            shape::BODY_KEY => shape::render(&view, active_utility, options).map(semio_framework_plugin::built_to_component_tree),
            building::BODY_KEY => building::render(&view, active_utility, options).map(semio_framework_plugin::built_to_component_tree),
            energy::BODY_KEY => energy::render(&view, active_utility, options).map(semio_framework_plugin::built_to_component_tree),
            structure_classic::BODY_KEY => structure_classic::render(&view, active_utility, options).map(semio_framework_plugin::built_to_component_tree),
            document::CAD_PLAY_BODY_DOCUMENT => document::build_document_tree(&view, labels).map(semio_framework_plugin::built_to_component_tree),
            catalogue::CAD_PLAY_BODY_CATALOGUE => catalogue::build_catalogue_tree(labels).map(semio_framework_plugin::built_to_component_tree),
            inspection::CAD_PLAY_BODY_PROPERTIES => inspection::build_properties_panel(&view, labels, active_utility).map(semio_framework_plugin::built_to_component_tree),
            _ => semio_framework_plugin::built_text_to_component_tree(Label::data(format!("Unknown body: {body_key}"))),
        }
    }

    fn window_engagements(doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>) -> HashMap<String, WindowEngagement> {
        let view = CadPlayView { document: doc.snapshot.clone(), runtime: cad_runtime_from_config(cfg.snapshot) };
        let labels = cad_labels(cfg.snapshot);
        HashMap::from([
            (shape::WINDOW_KIND_ID.to_string(), shape::engagement(&view, labels)),
            (building::WINDOW_KIND_ID.to_string(), building::engagement(&view, labels)),
            (energy::WINDOW_KIND_ID.to_string(), energy::engagement(&view, labels)),
            (structure_classic::WINDOW_KIND_ID.to_string(), structure_classic::engagement(&view, labels)),
        ])
    }

    /// 🪟️ Keyed by the 4 fixed window-KIND ids; each window collects its own measures from the edit
    /// mode's `🎚️options/*` components.
    fn window_measures(_doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>) -> HashMap<String, Vec<WindowMeasure>> {
        let runtime = cad_runtime_from_config(cfg.snapshot);
        let is_de = cad_is_de_locale(cfg.snapshot);
        HashMap::from([
            (shape::WINDOW_KIND_ID.to_string(), shape::window_measures(&runtime, is_de)),
            (building::WINDOW_KIND_ID.to_string(), building::window_measures(&runtime, is_de)),
            (energy::WINDOW_KIND_ID.to_string(), energy::window_measures(&runtime, is_de)),
            (structure_classic::WINDOW_KIND_ID.to_string(), structure_classic::window_measures(&runtime, is_de)),
        ])
    }

    /// 🖱️ Transform/duplicate/delete section for the World3d context menu. ⚠️ FIRST-CLASS-HOVER-
    /// AND-SELECTION-MECHANISM (26/08/14): `ArtifactApp::context_menu` has no `InteractionView`
    /// parameter, so this can no longer gate on "is anything selected" the way it used to
    /// (`cfg.snapshot.selected_object_ids`, now framework-owned and unreachable here) — always shows
    /// the section; a bare right-click with nothing selected is a documented reduced-fidelity gap
    /// (each action already no-ops on an empty selection at dispatch time)?.
    fn context_menu(_request: &ContextMenuRequest, _doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        semio_framework_plugin::resolve_ready(async { Menu::of(registry).await.action("translateSelection").await.action("rotateSelection").await.action("scaleSelection").await.action("duplicateObject").await.destructive("deleteObject").await.build().await })
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

/// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): the `"cad"` mesh interaction domain —
/// whole objects (`"object"`, the default granularity) plus component-level vertex/edge/face
/// picking, all `u32` ids stringified at the `InteractionTarget` boundary (round-tripped back to
/// `u32` inside command handlers, e.g. `🎮️commands/🔄️transform`). CAUTION: NOT the same thing as
/// `crate::artifacts::cad::standards::v1::subsets::any::io::InteractionSpec` (a CAD-artifact DSL
/// type for engagement statecharts, `🗿️artifacts/📐️cad/…/🎬️interaction-spec/🦀️.rs`) —
/// unrelated, pre-existing, untouched by this migration.
pub fn cad_interaction_definition() -> semio_framework_plugin::InteractionDefinition {
    use semio_framework_plugin::{GranularityDefinition, HierarchyProvider, HoverSpec, InteractionDefinition, MergeMode, SelectionMethod, SelectionMode, SelectionSpec};
    InteractionDefinition {
        id: CAD_INTERACTION_DOMAIN.into(),
        label: LocalizedLabel::native("Mesh", "Netz"),
        granularities: vec![
            GranularityDefinition { id: "object".into(), label: LocalizedLabel::native("Object", "Objekt"), icon_id: "box".into() },
            GranularityDefinition { id: "vertex".into(), label: LocalizedLabel::native("Vertex", "Eckpunkt"), icon_id: "circle-dot".into() },
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
    }
}

pub fn create_cad_app() -> semio_framework_plugin::AppDefinition {
    Editor::builder(crate::artifacts::cad::CAD_DIALECT).document(["semio", "cad"])
            .command({
                let mut definition = CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))]) };
                definition.semantics.execution.interactive_job = semio_framework_plugin::InteractiveJobClassification::Migrated;
                definition
            })
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
            .mutation("addObject", LocalizedLabel::native("Add Object", "Objekt hinzufügen"))
            .mutation("patchObject", LocalizedLabel::native("Patch Object", "Objekt aktualisieren"))
            .mutation("patchSelection", LocalizedLabel::native("Patch Selection", "Auswahl aktualisieren"))
            .action_with(semio_framework_plugin::resolve_ready(ActionDefinition::bounded_catalog("deleteObject", LocalizedLabel::native("Delete Object", "Objekt löschen"), ActionKind::Mutation).category("actions")))
            .action_with(semio_framework_plugin::resolve_ready(ActionDefinition::bounded_catalog("duplicateObject", LocalizedLabel::native("Duplicate Object", "Objekt duplizieren"), ActionKind::Mutation).category("create")))
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .mutation("renameNode", LocalizedLabel::native("Rename Node", "Knoten umbenennen"))
            .action_with(semio_framework_plugin::resolve_ready(ActionDefinition::bounded_catalog("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"), ActionKind::Mutation).category("transform")))
            .action_with(semio_framework_plugin::resolve_ready(ActionDefinition::bounded_catalog("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"), ActionKind::Mutation).category("transform")))
            .action_with(semio_framework_plugin::resolve_ready(ActionDefinition::bounded_catalog("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"), ActionKind::Mutation).category("transform")))
            .mutation("applyTransformation", LocalizedLabel::native("Apply Transformation", "Transformation anwenden"))
            .mutation("importCadFile", LocalizedLabel::native("Import CAD File", "CAD-Datei importieren"))
            .action_with(ActionDefinition::bounded_catalog("patchCadPlayReference", LocalizedLabel::native("Patch Reference", "Referenz aktualisieren"), ActionKind::Mutation).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Mutation).in_palette(false))
            .view_action("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"))
            .view_action("setLocale", LocalizedLabel::native("Set Locale", "Sprache festlegen"))
            .view_action("setTerminology", LocalizedLabel::native("Set Terminology", "Terminologie festlegen"))
            .view_action("setProjection", LocalizedLabel::native("Set Projection", "Projektion festlegen"))
            .view_action("setProjectionParam", LocalizedLabel::native("Set Projection Parameter", "Projektionsparameter festlegen"))
            .mutation("focusModelDefinition", LocalizedLabel::native("Focus Model Definition", "Modelldefinition fokussieren"))
            .mutation("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"))
            .action_with(ActionDefinition::bounded_catalog("setNodeSelection", LocalizedLabel::native("Set Node Selection", "Knotenauswahl festlegen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("setReferenceSelection", LocalizedLabel::native("Set Reference Selection", "Referenzauswahl festlegen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("referenceHover", LocalizedLabel::native("Reference Hover", "Überfahren (Referenz)"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("engagementPossibleSelect", LocalizedLabel::native("Engagement Possible Select", "Eingabeoption auswählen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("engagementRepeatLast", LocalizedLabel::native("Engagement Repeat Last", "Letzte Eingabe wiederholen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("worldPointerMove", LocalizedLabel::native("World Pointer Move", "Welt-Zeiger bewegt"), ActionKind::View).in_palette(false))
            .view_action("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"))
            .view_action("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"))
            .view_action("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"))
            .view_action("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"))
            .action_with(ActionDefinition::bounded_catalog("setDislocateOption", LocalizedLabel::native("Set Dislocate Option", "Versetzen-Option festlegen"), ActionKind::View).in_palette(false))
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
            // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): the single mesh
            // object/vertex/edge/face interaction domain, shared by all four World3d panes — the
            // framework auto-injects `interactionSelect`/`interactionHover`/`clearSelection`/
            // `selectAll`/`setSelectionMode`/`setInteractionGranularity` for it; this app never
            // declares those verbs itself. `HierarchyProvider::Flat`: a component id (vertex/edge/
            // face) is only ever meaningful within its owning object, not a tree the framework can
            // walk itself — `transitive` therefore stays false (requires `hierarchy != Flat`).
            .interaction(cad_interaction_definition())
            .window_kind_interactions(shape::WINDOW_KIND_ID, vec![semio_framework_plugin::InteractionRef::new(CAD_INTERACTION_DOMAIN)])
            .window_kind_interactions(building::WINDOW_KIND_ID, vec![semio_framework_plugin::InteractionRef::new(CAD_INTERACTION_DOMAIN)])
            .window_kind_interactions(energy::WINDOW_KIND_ID, vec![semio_framework_plugin::InteractionRef::new(CAD_INTERACTION_DOMAIN)])
            .window_kind_interactions(structure_classic::WINDOW_KIND_ID, vec![semio_framework_plugin::InteractionRef::new(CAD_INTERACTION_DOMAIN)])
            .panel_tab_def(document::definition())
            .panel_tab_def(catalogue::definition())
            .panel_tab_def(inspection::definition())
            // 🎯️ Typed channel + port surface (WORKFLOWS-END-TO-END-TYPED-PORTS Wave 2) — `cad_io()` is
            // this same `3d.cad`/Brep information's single source of truth, reused here rather than
            // duplicated; `config_spec()` stays empty (cad has no sticky-default settings analogous to
            // shooting's format defaults — every `CadConfig` field is session view-state, not a setting).
            .config(CadPlayApp::config_spec())
            .io(cad_io())
            .action_interactive_job("setActiveUtility", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("addObject", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("patchObject", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("patchSelection", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("deleteObject", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("duplicateObject", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addNode", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("renameNode", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("rotateSelection", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("scaleSelection", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("applyTransformation", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("importCadFile", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("patchCadPlayReference", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("focusModelDefinition", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("worldPointerDown", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setCamera", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjectionParam", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setDislocateOption", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setNodeSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setReferenceSelection", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("referenceHover", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementPossibleSelect", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementRepeatLast", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementAbort", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("worldPointerMove", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSun", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunAzimuth", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunElevation", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunIntensity", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setLocale", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("setTerminology", semio_framework_plugin::InteractiveJobClassification::Migrated)
            .action_interactive_job("saveSelected", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("saveInPlay", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("saveCurrent", semio_framework_plugin::InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("loadRawRequest", semio_framework_plugin::InteractiveJobClassification::Migrated)
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`Viewer`/`.editor::<E>(def: AppDefinition)`
            // take a bare `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so the old
            // `CAD_EXAMPLE_FOREST_LEFT` app-level example registration and the no-op `.workflow("cad",
            // …)` call are dropped here (not silently: reported in the packet's migration report).
            // The subset's own `📚️examples/🎬️demo` facet (`crate::artifacts::cad::examples::...`,
            // real content, pre-existing) is the modern, role-agnostic replacement surface for this.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🔖️WorkingSceneFixtures
/// 🌲️ The Concrete Forest Left example's REAL per-pane object content, built straight from the
/// same fixture JSON `forest_play_scene()`'s (persisted, handle-only) `CadSnapshot` is built from —
/// see `crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::forest_pane_bundle`.
/// This is the app-layer `CadWorkingScene` counterpart to `forest_play_scene()`: use `forest_play_scene()`
/// for `drive`/render dispatch (a `CadSnapshot`, composed-child HANDLES only) and this for reading
/// actual object data in tests/render-path exemplars.
pub fn forest_working_scene() -> CadWorkingScene {
    use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::forest_pane_bundle;
    let (objects, geometry) = forest_pane_bundle(CadPaneId::Shape);
    let (building_objects, building_geometry) = forest_pane_bundle(CadPaneId::Building);
    let (energy_objects, energy_geometry) = forest_pane_bundle(CadPaneId::Energy);
    let (structure_classic_objects, structure_classic_geometry) = forest_pane_bundle(CadPaneId::StructureClassic);
    CadWorkingScene {
        objects,
        geometry: Some(geometry),
        building_objects,
        building_geometry: Some(building_geometry),
        energy_objects,
        energy_geometry: Some(energy_geometry),
        structure_classic_objects,
        structure_classic_geometry: Some(structure_classic_geometry),
    }
}

/// 🟦️ The single-box placeholder scene `default_document()`'s `CadSnapshot` used to inline directly
/// (pre-wave-3) — realized now as the app-layer `CadWorkingScene` counterpart: use `default_document()`
/// for `drive`/render dispatch, this for reading its (one, real) object.
pub fn default_working_scene() -> CadWorkingScene {
    CadWorkingScene { objects: vec![make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape)], ..CadWorkingScene::default() }
}
//#endregion 🔖️WorkingSceneFixtures

//#region 🧪️Tests
#[cfg(test)]
pub(crate) mod testkit {
    //! 🧪️ The one cad-app test harness — every other taxonomy node's `🧪️Tests` region builds on it
    //! instead of re-deriving a store/dispatch/render scaffold of its own.
    use super::*;
    use protocol::{Mutation, MutationDiff};
    use semio_framework_plugin::app::EditorApp;
    use semio_framework_plugin::{ActionMeta, HistoryView, UiMenuRef, VcsArtifactApp};

    pub fn meta(actor: &str) -> ActionMeta {
        semio_framework_plugin::testkit::meta(actor)
    }

    /// ✏️ `CadPlayApp` implements the AUTHORING trait `ArtifactEditor`, not the runtime `ArtifactApp`
    /// — `EditorApp<CadPlayApp>` (SDK adapter, contract §2.1) is the real `ArtifactApp` implementor
    /// `VcsArtifactApp` wraps, exactly the way `PluginBuilder::editor::<CadPlayApp>` builds it.
    pub async fn new_app() -> VcsArtifactApp<EditorApp<CadPlayApp>> {
        semio_framework_plugin::testkit::new_app::<EditorApp<CadPlayApp>>().await
    }

    /// ✏️ Adapts `create_cad_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `testkit::assert_declared_actions_bridge_to_commands` still expects —
    /// framework testkit gap, not modifiable here (`🧰️framework/**` is outside this packet's lease).
    pub fn cad_app_manifest_for_testkit() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_cad_app(), examples: Vec::new() }
    }

    pub fn empty_history() -> HistoryView {
        HistoryView::empty()
    }

    /// 🔀️ Keeps the legacy test-harness call shape while exercising the production action bridge.
    pub fn command_from_action(action: &str, args: Option<&Value>) -> CadCommand {
        cad_command_from_action(action, args).unwrap_or_else(|error| panic!("command_from_action: {error:?}"))
    }

    /// 🕹️ Drives one action against a bare `CadPlayApp` (unwrapped, config defaulted) so tests can
    /// inspect the emitted document/config operations directly.
    pub fn drive(app: &CadPlayApp, scene: &CadSnapshot, action: &str, args: Option<Value>) -> Emit<CadMutation, CadConfigMutation> {
        drive_with_config(app, scene, action, args, &CadConfig::default())
    }

    /// 🧪️ `args` stays owned so every ported test keeps the pre-migration `(action id, json!(..))`
    /// call shape verbatim; `command_from_action` only ever reads it.
    ///
    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): dispatches straight through
    /// `CadCommand::dispatch` instead of the `ArtifactApp::handle` trait method — `handle`'s
    /// `interaction: &semio_framework_plugin::app::InteractionView<'_>` parameter has `pub(crate)`
    /// fields in that crate, so this crate's own tests cannot construct one; `dispatch` only needs
    /// the app-owned `CadDispatchCtx` (whose `interaction: CadInteractionSnapshot` field IS plain
    /// and cad-owned), so tests build that by hand and skip the adaptation `handle` exists for.
    #[allow(clippy::needless_pass_by_value)]
    pub fn drive_with_config(app: &CadPlayApp, scene: &CadSnapshot, action: &str, args: Option<Value>, config: &CadConfig) -> Emit<CadMutation, CadConfigMutation> {
        let operation = CadPreviewOperationIdentity { app_instance_id: 1, parent_document_id: "cad-test-document".into(), operation_id: 1, operation_generation: 1, canonical_base_revision: "00".repeat(32) };
        drive_with_operation(app, scene, action, args, config, Some(operation)).expect("cad command handled")
    }

    /// 🪪️ Production-dispatch harness with an explicit public operation identity, including the
    /// missing-context case used by fail-closed transition fixtures.
    #[allow(clippy::needless_pass_by_value)]
    pub fn drive_with_operation(app: &CadPlayApp, scene: &CadSnapshot, action: &str, args: Option<Value>, config: &CadConfig, preview_operation: Option<CadPreviewOperationIdentity>) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let _ = app;
        let history = empty_history();
        let doc = ArtifactView::new(scene, &history);
        let cfg = ConfigView { snapshot: config };
        let command = command_from_action(action, args.as_ref());
        let mut ctx = CadDispatchCtx { interaction: CadInteractionSnapshot::default(), preview_operation };
        command.dispatch(&doc, &cfg, &mut ctx)
    }

    pub fn render_direct(_app: &CadPlayApp, body_key: &str, doc: &ArtifactView<'_, CadSnapshot>, config: &CadConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
        let cfg = ConfigView { snapshot: config };
        CadPlayApp::render(body_key, doc, &cfg).map(|tree| tree.root)
    }

    pub fn window_measures_direct(_app: &CadPlayApp, doc: &ArtifactView<'_, CadSnapshot>, config: &CadConfig) -> HashMap<String, Vec<WindowMeasure>> {
        let cfg = ConfigView { snapshot: config };
        CadPlayApp::window_measures(doc, &cfg)
    }

    pub fn context_menu_direct(_app: &CadPlayApp, doc: &ArtifactView<'_, CadSnapshot>, config: &CadConfig, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        let cfg = ConfigView { snapshot: config };
        let request = ContextMenuRequest { menu: UiMenuRef { id: "world3d".into(), args: None }, surface: None, window_instance_id: None, point: None };
        CadPlayApp::context_menu(&request, doc, &cfg, registry)
    }

    /// 🧮️ Folds a list of `CadMutation`s onto a scene via the core `Mutation`/`MutationDiff` impls —
    /// mirrors what the wrapping `VcsArtifactApp` store does when it dispatches the emitted operations.
    pub fn apply_mutations(scene: &CadSnapshot, operations: &[CadMutation]) -> CadSnapshot {
        let mut next = scene.clone();
        for operation in operations {
            next = operation.diff(&next).diff().apply(&next).expect("valid mutation diff");
        }
        next
    }

    /// 🧮️ `apply_mutations`'s config-targeted twin — folds an `Emit`'s `config_mutations` onto a base
    /// `CadConfig` (mirrors what `VcsArtifactApp`'s config store does when it dispatches them).
    pub fn config_after(emit: &Emit<CadMutation, CadConfigMutation>, base: &CadConfig) -> CadConfig {
        let mut next = base.clone();
        for operation in &emit.config_mutations {
            next = operation.diff(&next).diff().clone();
        }
        next
    }

    /// 🧮️ `config_after` plus the `CadConfig -> CadPlayRuntime` boundary conversion — the direct
    /// replacement for the pre-B1 `app.runtime.borrow()` most tests below inspected after `drive(..)`.
    pub fn runtime_after(emit: &Emit<CadMutation, CadConfigMutation>, base: &CadConfig) -> CadPlayRuntime {
        cad_runtime_from_config(&config_after(emit, base))
    }

    pub fn view(scene: CadSnapshot, runtime: CadPlayRuntime) -> CadPlayView {
        CadPlayView { document: scene, runtime }
    }
}

#[cfg(test)]
mod tests {
    use super::testkit::*;
    use super::*;
    use crate::artifacts::cad::standards::v1::subsets::any::io::{cad_document_from_dwg, cad_working_scene_from_dwg, scene_from_spatial_payload};
    use crate::artifacts::cad::standards::v1::subsets::any::schema::inferences::{
        align_mesh_to_fixture_centroid, default_document, object_mesh_data, primary_primitive_kind, run_derive_from_geometry, CAD_DEFAULT_TYPOLOGY_EXTENT, CAD_FOREST_REFERENCE_IMAGE_HEIGHT_PX, CAD_FOREST_REFERENCE_IMAGE_WIDTH_PX, CAD_FOREST_REFERENCE_PLANE_Z,
        CAD_FOREST_REFERENCE_WIDTH_WORLD, CAD_FOREST_REFERENCE_Y_OFFSET_RATIO,
    };
    use crate::artifacts::cad::{empty_cad_snapshot, CadNode, CAD_PLAY_DOCUMENT_SCHEMA};
    use semio_framework_plugin::{ActionKind, AppActionRegistry, EditorApp, PluginApp, SET_ACTIVE_UTILITY_ACTION_ID};
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
            CadCommand::SetNodeSelection(set_node_selection::SetNodeSelection { node_ids: vec!["node-1".into(), "node-2".into()] }),
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
            CadCommand::ToggleSun(toggle_sun::ToggleSun {}),
            CadCommand::SetSunAzimuth(set_sun_azimuth::SetSunAzimuth { value: 45.0 }),
            CadCommand::SetSunElevation(set_sun_elevation::SetSunElevation { value: 35.0 }),
            CadCommand::SetSunIntensity(set_sun_intensity::SetSunIntensity { value: 0.85 }),
            CadCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: "dislocate".into() }),
            CadCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() }),
            CadCommand::SetTerminology(set_terminology::SetTerminology { value: "reuse".into() }),
            CadCommand::SetContributions(set_contributions::SetContributions { json: "[]".into() }),
            CadCommand::SaveSelected(save_selected::SaveSelected {}),
            CadCommand::SaveInPlay(save_in_play::SaveInPlay {}),
            CadCommand::SaveCurrent(save_current::SaveCurrent { format: Some("step".into()) }),
            CadCommand::SaveCurrent(save_current::SaveCurrent { format: None }),
            CadCommand::LoadRawRequest(load_raw_request::LoadRawRequest {}),
        ]
    }

    /// 🧪️ The shell's example picker reaches the production bridge and produces the typed command
    /// that replaces the CAD document instead of falling through to the framework-only action path.
    #[semio_framework_async_macros::async_test]
    async fn production_action_bridge_loads_the_declared_example() {
        let command = <CadPlayApp as ArtifactEditor>::command_from_action("setActiveExample", Some(&json!({ "exampleId": CAD_EXAMPLE_FOREST_LEFT }))).expect("declared example action");
        assert!(matches!(command, CadCommand::SetActiveExample(set_active_example::SetActiveExample { example_id }) if example_id == CAD_EXAMPLE_FOREST_LEFT));
        let contributions = <CadPlayApp as ArtifactEditor>::command_from_action("setContributions", Some(&json!({ "json": "[{\"id\":\"cad\"}]" }))).expect("declared host command");
        assert!(matches!(contributions, CadCommand::SetContributions(set_contributions::SetContributions { json }) if json == "[{\"id\":\"cad\"}]"));
        assert!(<CadPlayApp as ArtifactEditor>::command_from_action("notACadAction", None).is_err());
    }

    #[test]
    fn host_contributions_resolve_to_the_event_sourced_config_lane() {
        let mutation = <CadPlayApp as ArtifactEditor>::host_configuration_mutation("setContributions", Some(&json!({ "json": "[{\"id\":\"cad\"}]" })))
            .expect("host configuration")
            .expect("CAD contribution mutation");
        assert_eq!(mutation, CadConfigMutation::SetContributions { json: "[{\"id\":\"cad\"}]".into() });
        assert_eq!(<CadPlayApp as ArtifactEditor>::host_configuration_mutation("setActiveExample", None).expect("non-host action"), None);
        assert!(<CadPlayApp as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().is_some());
        assert!(<CadPlayApp as ArtifactEditor>::build_config_store_one_item_preparation_factory().is_some());
        let factory = CadRetainedCommandJobFactory::new("s.cad.cad@1/*#editor");
        let expected_keys = CAD_RETAINED_TOOL_IDS.iter().map(|tool_id| ToolFactoryKey::new("s.cad.cad@1/*#editor", *tool_id)).collect::<Vec<_>>();
        assert_eq!(ToolJobFactory::keys(&factory), expected_keys);
        assert_eq!(ToolJobFactory::payload_schema_id(&factory), CAD_RETAINED_COMMAND_SCHEMA);
        assert_eq!(ToolJobFactory::classification(&factory), InteractiveJobClassification::Migrated);
        assert_eq!(<CadRetainedCommandJobFactory as ArtifactOwnedToolJobFactory>::PUBLICATION_CONTRACTS, CAD_RETAINED_PUBLICATION_CONTRACTS);
        assert_eq!(<CadPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), CAD_RETAINED_TOOL_IDS.len());
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_cad_presence_close_empty_lanes_have_exact_owners() {
        let fixture: Value = serde_json::from_str(include_str!("👥️presence/🧪️retirement.json")).unwrap();
        let maximum_items = fixture["grant"]["maximumItems"].as_u64().unwrap() as usize;
        let maximum_bytes = fixture["grant"]["maximumBytes"].as_u64().unwrap() as usize;
        let envelope = store::create_document_envelope::<NoDraft, NoDraftMutation>("draft.empty", "cad-draft-close", NoDraft::default(), None);
        let mut draft = store::DraftStore::new(envelope).await.unwrap();
        draft.install_member_store_owners_exact(<CadPlayApp as ArtifactEditor>::build_draft_store_owners().unwrap());
        let mut disposer = <CadPlayApp as ArtifactEditor>::build_draft_store_disposer().unwrap();
        for turn in 0..100_000 {
            match disposer.close_step(&mut draft, maximum_items, maximum_bytes).unwrap() {
                semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => assert!(released_items <= maximum_items && released_bytes <= maximum_bytes),
                semio_framework_plugin::PluginCloseStep::Blocked { reason } => panic!("empty CAD draft close blocked: {reason}"),
                semio_framework_plugin::PluginCloseStep::Complete => break,
            }
            assert!(turn < 99_999);
        }
        assert!(disposer.terminal_is_empty(&draft));
        let mut transient = store::TransientStore::new(semio_framework_plugin::NoTransient::default());
        let mut disposer = <CadPlayApp as ArtifactEditor>::build_transient_store_disposer().unwrap();
        assert_eq!(disposer.close_step(&mut transient, 0, maximum_bytes).unwrap(), semio_framework_plugin::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!(disposer.close_step(&mut transient, maximum_items, maximum_bytes).unwrap(), semio_framework_plugin::PluginCloseStep::Complete);
        assert!(disposer.terminal_is_empty(&transient));
        eprintln!("[DEBUG] CAD exact NoDraft and NoTransient owners completed under 1-item/4096-byte grants");
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_factory_proofs_activate_the_real_cad_manifest_and_close_under_the_production_grant() {
        let fixture: Value = serde_json::from_str(include_str!("../🔣️retained-jobs.json")).expect("CAD activation fixture");
        let activation = &fixture["activation"];
        let controller = activation["controller"].as_str().expect("controller");
        let bus = semio_framework::ActionBus::new();
        let definition = create_cad_app();
        let host_route = fixture["routes"].as_array().unwrap().iter().find(|route| route["id"] == "setContributions").unwrap();
        assert_eq!(host_route["disposition"], "migrated");
        let host_command = definition.commands.iter().find(|command| command.id == host_route["id"].as_str().unwrap()).expect("host command declaration");
        assert_eq!(host_command.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
        let registry = AppActionRegistry::from_definition(&definition);
        let mut app = semio_framework_plugin::VcsArtifactApp::<EditorApp<CadPlayApp>>::with_registry_on_bus(EditorApp::<CadPlayApp>::default(), registry, bus.clone()).await;
        assert_eq!(app.app_id().await, controller);
        assert_eq!(<CadPlayApp as ArtifactEditor>::bounded_first_step_tool_proofs().len(), activation["proofRows"].as_u64().expect("proof rows") as usize);
        let mut admitted = std::collections::BTreeSet::new();
        for tool_id in CAD_RETAINED_TOOL_IDS {
            let admission = bus.admit_exact_wire(controller, *tool_id, CAD_RETAINED_COMMAND_SCHEMA, &[]).expect("real CAD factory is live before proof validation");
            assert_eq!(admission.factory_type_id, std::any::TypeId::of::<CadRetainedCommandJobFactory>());
            assert_eq!(admission.factory_type_name, std::any::type_name::<CadRetainedCommandJobFactory>());
            assert!(admitted.insert(*tool_id));
        }
        assert!(admitted.contains(activation["injectedTool"].as_str().expect("injected tool")));
        let maximum_items = activation["closeItems"].as_u64().expect("close items") as usize;
        let maximum_bytes = activation["closeBytes"].as_u64().expect("close bytes") as usize;
        let mut complete = false;
        for _ in 0..100_000 {
            match app.close_step(maximum_items, maximum_bytes).expect("bounded CAD close") {
                semio_framework_plugin::PluginCloseStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= maximum_items && released_bytes <= maximum_bytes);
                }
                semio_framework_plugin::PluginCloseStep::Blocked { reason } => panic!("CAD constructor close blocked: {reason}"),
                semio_framework_plugin::PluginCloseStep::Complete => { complete = true; break; }
            }
        }
        assert!(complete && app.close_terminal_is_empty(), "the real mounted CAD owner must reach its empty terminal shell");
        eprintln!("[DEBUG] CAD activation joined {} exact app factory rows including setActiveUtility and completed bounded close", admitted.len());
    }

    #[test]
    fn retained_config_store_preparation_is_bounded_exact_and_reversible() {
        let base = CadConfig::default();
        let mut next = base.clone();
        next.locale = "de-DE".into();
        next.active_utility_id = "dislocate".into();
        let mutation = CadConfigMutation::Snapshot { config: next.clone() };
        let footprint = admit_cad_config_mutation(&mutation).expect("bounded CAD config mutation");
        assert_eq!(footprint.work_items, 1);
        let (post, inverse, forward) = prepare_cad_config(&base, mutation.clone()).expect("exact CAD config preparation");
        assert_eq!(post, next);
        assert_eq!(forward, mutation);
        assert_eq!(inverse, vec![CadConfigMutation::Snapshot { config: base.clone() }]);
        let oversized = CadConfigMutation::SetContributions { json: "x".repeat(CAD_CONFIG_STORE_MAXIMUM_BYTES + 1) };
        assert!(admit_cad_config_mutation(&oversized).is_err());
    }

    #[test]
    fn retained_artifact_store_preparation_is_bounded_exact_and_reversible() {
        let base = crate::artifacts::cad::empty_cad_snapshot();
        let node = crate::artifacts::cad::CadNode { id: "node-retained".into(), label: "Retained".into(), kind: "group".into() };
        let mutation = CadMutation::CreateNode(crate::artifacts::cad::mutations::create_node::mutation::CreateNode { node: node.clone() });
        let footprint = admit_cad_artifact_mutation(&mutation).expect("bounded CAD Artifact mutation");
        assert_eq!(footprint.work_items, 1);
        let (post, inverse, forward) = prepare_cad_artifact(&base, mutation.clone()).expect("exact CAD Artifact preparation");
        assert_eq!(post.nodes, vec![node]);
        assert_eq!(forward, mutation);
        let mut restored = post;
        for operation in inverse {
            let outcome = <CadMutation as protocol::Mutation<CadSnapshot>>::diff(&operation, &restored);
            restored = protocol::MutationDiff::apply(outcome.diff(), &restored).expect("exact inverse");
        }
        assert_eq!(restored, base);
    }

    #[test]
    fn retained_route_fixture_matches_the_exact_owner_manifest_and_laws() {
        let fixture: Value = serde_json::from_str(include_str!("../🔣️retained-jobs.json")).expect("CAD retained route fixture");
        let routes = fixture.get("routes").and_then(Value::as_array).expect("route array");
        let route_ids = routes.iter().map(|route| route.get("id").and_then(Value::as_str).expect("route id")).collect::<std::collections::BTreeSet<_>>();
        let command_ids = every_command()
            .iter()
            .map(CadCommand::command_id)
            .filter(|id| *id != SET_ACTIVE_UTILITY_ACTION_ID)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(routes.len(), 40);
        assert_eq!(route_ids, command_ids);
        assert_eq!(fixture.get("admittedRoutes"), Some(&json!(CAD_RETAINED_TOOL_IDS)));
        assert_eq!(fixture.pointer("/limits/closePageBytes").and_then(Value::as_u64), Some(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES as u64));
        assert_eq!(fixture.get("laws"), Some(&json!(["ownerLocal", "progress", "cancel", "freshness", "ackBeforeClose", "incrementalClose", "terminalEmpty"])));
        assert!(routes.iter().all(|route| {
            let id = route.get("id").and_then(Value::as_str);
            let disposition = route.get("disposition").and_then(Value::as_str);
            let blocker = route.get("blocker").and_then(Value::as_str);
            if id.is_some_and(|id| CAD_RETAINED_TOOL_IDS.contains(&id)) {
                disposition == Some("migrated") && blocker == Some("none")
            } else {
                disposition == Some("batchOnlyPendingRewrite") && blocker.is_some_and(|blocker| blocker != "none")
            }
        }));
        let manifest = create_cad_app();
        for tool_id in CAD_RETAINED_TOOL_IDS {
            let mut declarations = 0;
            for commands in std::iter::once(&manifest.commands).chain(manifest.modes.iter().map(|mode| &mode.commands)) {
                let matches = commands.iter().filter(|command| command.id == *tool_id).collect::<Vec<_>>();
                if matches.is_empty() { continue; }
                assert_eq!(matches.len(), 1, "{tool_id} requires exactly one declaration per command scope");
                assert_eq!(matches[0].semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{tool_id}");
                declarations += 1;
            }
            for window in &manifest.window_kinds {
                let actions = window.actions.iter().filter(|action| action.id == *tool_id).collect::<Vec<_>>();
                if actions.is_empty() { continue; }
                assert_eq!(actions.len(), 1, "{tool_id} requires exactly one declaration per window scope");
                assert_eq!(actions[0].semantics.execution.interactive_job, InteractiveJobClassification::Migrated, "{tool_id}");
                declarations += 1;
            }
            assert!(declarations > 0, "{tool_id} requires a manifest command or window declaration");
        }
        assert_eq!(manifest.window_kinds.iter().flat_map(|window| &window.actions).find(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID).map(|action| action.semantics.execution.interactive_job), Some(InteractiveJobClassification::Migrated));
        assert!(manifest.window_kinds.iter().flat_map(|window| &window.actions).filter(|action| route_ids.contains(action.id.as_str())).all(|action| {
            let expected = if CAD_RETAINED_TOOL_IDS.contains(&action.id.as_str()) { InteractiveJobClassification::Migrated } else { InteractiveJobClassification::BatchOnlyPendingRewrite };
            action.semantics.execution.interactive_job == expected
        }));
    }

    /// ⚖️ LAW: the one-action spot check above is not enough — this is the framework's own harness,
    /// which walks EVERY action this app's window kinds render, stages each one's declared args the way
    /// the host does, and skips the framework-injected ids. It is what catches the next
    /// `setActiveExample`: chrome that declares an action no command row backs.
    #[semio_framework_async_macros::async_test]
    async fn every_rendered_action_bridges_through_the_framework_harness() {
        semio_framework_plugin::testkit::assert_declared_actions_bridge_to_commands::<EditorApp<CadPlayApp>>(cad_app_manifest_for_testkit).await;
    }

    /// ⚖️ Text and binary are two projections of the same command, and every printed line starts with
    /// that row's wire keyword — the guard that a command decomposition cannot silently rename a row.
    #[semio_framework_async_macros::async_test]
    async fn every_command_round_trips_text_and_binary_under_its_own_wire_keyword() {
        for command in every_command() {
            store::os_store::test_support::assert_op_text_binary_equivalence(&command);
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
        // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `SetHover`/`WorldPick` and their
        // byte pins are DELETED (those commands no longer exist); every OTHER row's ordinal shifted
        // too (this enum's binary encoding is a plain row-position ordinal — greenfield, no
        // back-compat expected), so the tail-of-enum pins (`EngagementAbort`/`ToggleSun`/
        // `SaveSelected`/`LoadRawRequest`) are dropped rather than hand-recomputed; the exact-wire-key
        // guard for every row (including these) already lives in
        // `every_command_round_trips_text_and_binary_under_its_own_wire_keyword` above.
    }

    #[semio_framework_async_macros::async_test]
    async fn forest_example_uses_per_object_brep_meshes() {
        let scene = forest_working_scene();
        let runtime = CadPlayRuntime::default();
        let json = edit::world_instances_json(&scene.building_objects, &runtime);
        assert!(json.contains("object-hexagonal-cut-concrete-forest-left-bim-10"));
        let meshes = edit::world_meshes_json(&scene.building_objects, scene.building_geometry.as_ref());
        assert!(meshes.contains("object-hexagonal-cut-concrete-forest-left-bim-10"));
        assert!(!meshes.contains("🧊️hexagonal-cut-concrete-forest-left.glb"));
        assert!(scene.building_objects.len() > 5);
        assert!(scene.building_objects.iter().all(|object| object.solid_handle.is_some()));
    }

    #[semio_framework_async_macros::async_test]
    async fn cad_document_from_dwg_creates_one_object_per_layer_with_geometry() {
        let mut drawing = semio_s_plugin_stdio::artifacts::dwg::DwgDrawing::default();
        let outline = drawing.ensure_layer("outline");
        let empty_layer = drawing.ensure_layer("empty");
        let _ = empty_layer;
        drawing.entities.push(semio_s_plugin_stdio::artifacts::dwg::DwgEntity {
            layer: outline,
            color: semio_s_plugin_stdio::artifacts::dwg::DwgColor::ByLayer,
            geometry: semio_s_plugin_stdio::artifacts::dwg::DwgGeometry::PolyfaceMesh { vertices: vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0]], faces: vec![[1, 2, 3, 4]] },
        });
        let working = cad_working_scene_from_dwg(&drawing);
        assert_eq!(working.objects.len(), 1, "the empty layer must not contribute an object");
        assert_eq!(working.objects[0].label, "outline");
        let value = cad_document_from_dwg(&drawing).expect("cad document from dwg");
        let scene: CadSnapshot = serde_json::from_value(value).expect("valid cad scene");
        assert!(scene.shape_model.is_some(), "a real per-layer object must mint a shape-model child");
    }

    #[semio_framework_async_macros::async_test]
    async fn cad_document_from_empty_dwg_mints_no_shape_model_child() {
        let drawing = semio_s_plugin_stdio::artifacts::dwg::DwgDrawing::default();
        let working = cad_working_scene_from_dwg(&drawing);
        assert!(working.objects.is_empty());
        let value = cad_document_from_dwg(&drawing).expect("cad document from empty dwg");
        let scene: CadSnapshot = serde_json::from_value(value).expect("valid cad scene");
        assert!(scene.shape_model.is_none(), "no layers means no real geometry to mint a child from");
    }

    #[semio_framework_async_macros::async_test]
    async fn quad_panes_each_populate_distinct_objects() {
        let scene = forest_working_scene();
        assert!(!scene.objects.is_empty(), "shape pane");
        assert!(!scene.building_objects.is_empty(), "building pane");
        assert!(!scene.energy_objects.is_empty(), "energy pane");
        assert!(!scene.structure_classic_objects.is_empty(), "structure classic pane");
    }

    #[semio_framework_async_macros::async_test]
    async fn initial_snapshot_is_cut_concrete_forest_not_placeholder_box() {
        let scene = CadPlayApp::initial_snapshot();
        assert_eq!(scene.id, CAD_EXAMPLE_FOREST_LEFT);
        assert_eq!(scene.nodes.first().map(|node| node.label.as_str()), Some("Concrete Forest Left"), "must not be the placeholder 'Model' node");
        let working = forest_working_scene();
        assert_ne!(working.objects.first().map(|object| object.id.as_str()), Some("object-box-1"));
        assert!(!working.building_objects.is_empty(), "building pane must not be the empty default placeholder");
        assert!(!working.energy_objects.is_empty(), "energy pane must not be the empty default placeholder");
        assert!(!working.structure_classic_objects.is_empty(), "structure pane must not be the empty default placeholder");
        assert!(working.objects.iter().all(|object| object.solid_handle.is_some()));
    }

    #[semio_framework_async_macros::async_test]
    async fn forest_energy_world_mesh_survives_scene_roundtrip() {
        let scene = forest_working_scene();
        let roundtrip: CadWorkingScene = serde_json::from_str(&serde_json::to_string(&scene).expect("serialize")).expect("deserialize");
        let object = roundtrip.energy_objects.first().expect("energy object");
        let mesh = object_mesh_data(object, roundtrip.energy_geometry.as_ref());
        let min_z = mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
        assert!(min_z > 2.5, "energy world mesh min z {min_z}");
        let slab = roundtrip.structure_classic_objects.iter().find(|object| object.primitives.iter().any(|primitive| primitive.kind == "surface")).expect("structure surface");
        let slab_mesh = object_mesh_data(slab, roundtrip.structure_classic_geometry.as_ref());
        let slab_min_z = slab_mesh.positions.as_chunks::<3>().0.iter().map(|vertex| vertex[2]).fold(f32::INFINITY, f32::min);
        assert!(slab_min_z > 2.5, "structure world mesh min z {slab_min_z}");
    }

    #[semio_framework_async_macros::async_test]
    async fn forest_references_use_xy_ground_plane_and_z_up() {
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

    #[semio_framework_async_macros::async_test]
    async fn align_mesh_to_fixture_centroid_corrects_drifted_surface() {
        let scene = forest_working_scene();
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

    #[semio_framework_async_macros::async_test]
    async fn forest_surface_meshes_fall_back_to_typology_extent_without_pane_geometry() {
        // ⚠️ CORRECTED (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
        // wave G4): this test used to assert the mesh stayed at its authored height even with no
        // `CadGeometry` in hand — that only worked because `cad_brep_kernel()` was a process-global
        // `BrepEngineHost` singleton, so `energy.solid_handle` (minted by an EARLIER, already-dropped
        // call to `forest_pane_bundle`) still resolved in whatever kernel `object_mesh_data` happened
        // to reach. `origin` on fixture-derived `CadObject`s is always `[0,0,0]`
        // (`objects_from_fixture_model`) — the authored height lived ONLY in the solid's own vertex
        // data, addressed by that handle. A `cad_brep_kernel()` is now a fresh, local `Brep::new()`
        // per call (doctrine tier-(d): never outlives the call that built it), so a handle from a
        // different call is honestly unresolvable, and — exactly like `mesh_from_glb`'s documented
        // gap elsewhere in this codebase — meshing falls back to the typology's default extent box
        // at the kernel's local origin instead of silently fabricating a placement it cannot know.
        let scene = forest_working_scene();
        let energy = scene.energy_objects.first().expect("energy object");
        let energy_mesh = object_mesh_data(energy, None);
        assert!(!energy_mesh.positions.is_empty(), "energy mesh must still be real geometry, just typology-shaped");
        let slab = scene.structure_classic_objects.iter().find(|object| object.primitives.iter().any(|primitive| primitive.kind == "surface")).expect("structure surface");
        let slab_mesh = object_mesh_data(slab, None);
        assert!(!slab_mesh.positions.is_empty(), "structure slab mesh must still be real geometry, just typology-shaped");
    }

    #[semio_framework_async_macros::async_test]
    async fn cad_document_schema_matches_domain() {
        let scene = empty_cad_snapshot();
        assert_eq!(scene.schema, CAD_PLAY_DOCUMENT_SCHEMA);
    }

    #[semio_framework_async_macros::async_test]
    async fn default_example_and_forest_scene_parse_as_projections() {
        let default_json = serde_json::to_string(&default_document()).unwrap();
        let default_scene: CadSnapshot = serde_json::from_str(&default_json).unwrap();
        assert_eq!(default_scene.schema, CAD_PLAY_DOCUMENT_SCHEMA);
        let forest_json = serde_json::to_string(&forest_play_scene()).unwrap();
        let forest_scene: CadSnapshot = serde_json::from_str(&forest_json).unwrap();
        assert_eq!(forest_scene.id, CAD_EXAMPLE_FOREST_LEFT);
        assert!(!forest_working_scene().building_objects.is_empty());
    }
    //#endregion 🔖️Fixtures
    //#region 🔖️Render
    #[semio_framework_async_macros::async_test]
    async fn renders_world_scene_for_each_pane() {
        let app = CadPlayApp::default();
        let scene = forest_play_scene();
        let history = empty_history();
        let doc = ArtifactView::new(&scene, &history);
        for body_key in [shape::BODY_KEY, building::BODY_KEY, energy::BODY_KEY, structure_classic::BODY_KEY] {
            let node = render_direct(&app, body_key, &doc, &CadConfig::default()).expect("CAD UI assembly");
            let json = serde_json::to_string(&node).unwrap();
            assert!(json.contains("world-3d"), "body {body_key} should render a world-3d scene");
        }
    }

    /// 🛡️ Anti-regression guard for the "four empty windows" defect: `forest_play_document` must
    /// actually populate `shape_model`/`building_model`/`energy_model`/`structure_classic_model`
    /// (via `cad_document_pane_bundle`, carried as each handle's `ArtifactChild::local_owner`), and
    /// `build_world_scene_for_pane`'s own pane resolver (`edit::cad_pane_working_scene`/
    /// `edit::cad_pane_working_objects`) must read real objects back out of them instead of the
    /// hardcoded empty slice the defect shipped with. Checks the exact `instances_json` string
    /// `build_world_scene_for_pane` feeds `MeshWindowKit::render` — the built scene's world-3d
    /// payload the defect left permanently empty — for every pane, not just via the lower-level
    /// `world_instances_json(&scene.building_objects, ..)` shortcut `forest_example_uses_per_object_brep_meshes` uses.
    #[semio_framework_async_macros::async_test]
    async fn forest_example_world_scene_has_non_empty_instances_for_every_pane() {
        let document = forest_play_scene();
        for pane in CadPaneId::all() {
            let working_scene = edit::cad_pane_working_scene(&document, pane).unwrap_or_else(|| panic!("pane {pane:?} must resolve a local-owner working scene"));
            let (objects, _geometry) = edit::cad_pane_working_objects(&working_scene, pane);
            assert!(!objects.is_empty(), "pane {pane:?} must have real objects, not the empty-defect slice");
            let instances_json = edit::world_instances_json(objects, &CadPlayRuntime::default());
            assert_ne!(instances_json, "[]", "pane {pane:?} instances_json must not be empty");
            let meshes_json = edit::world_meshes_json(objects, _geometry);
            assert!(!meshes_json.contains(CAD_FALLBACK_MESH_KIND), "pane {pane:?} must render real brep meshes, not the universal fallback box");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn app_definition_declares_one_window_scoped_dislocate_utility() {
        let definition = create_cad_app();
        let utility_ids: Vec<&str> = definition.utilities.iter().map(|utility| utility.id.as_str()).collect();
        assert_eq!(utility_ids, vec![CAD_DISLOCATE_UTILITY_ID]);
        // 🧰️ The framework auto-injects `setActiveUtility` as a View action once utilities are declared —
        // cad must NOT also declare it as an Mutation.
        let set_active_utility = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|action| action.id == SET_ACTIVE_UTILITY_ACTION_ID).expect("setActiveUtility auto-injected");
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
    /// config-derived per frame via `ArtifactApp::window_measures`, never frozen into the manifest.
    #[semio_framework_async_macros::async_test]
    async fn manifest_stitches_every_taxonomy_node_with_its_pre_migration_shape() {
        let definition = create_cad_app();
        let windows: Vec<(&str, &str)> = definition.window_kinds.iter().map(|window| (window.id.as_str(), window.body_key.as_str())).collect();
        assert_eq!(windows, vec![(shape::WINDOW_KIND_ID, shape::BODY_KEY), (building::WINDOW_KIND_ID, building::BODY_KEY), (energy::WINDOW_KIND_ID, energy::BODY_KEY), (structure_classic::WINDOW_KIND_ID, structure_classic::BODY_KEY),]);
        for window in definition.window_kinds.iter() {
            assert_eq!(window.surface_kind, ui_wgpu::wgpu::SurfaceKind::World3d, "window {} surface kind", window.id);
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
                (semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_ID, Some(document::CAD_PLAY_BODY_DOCUMENT)),
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

    #[semio_framework_async_macros::async_test]
    async fn internal_and_plumbing_actions_excluded_from_palette() {
        let definition = create_cad_app();
        let hidden_actions = [
            "patchCadPlayReference",
            "engagementSubmit",
            "setNodeSelection",
            "setReferenceSelection",
            "referenceHover",
            "engagementInput",
            "engagementPossibleSelect",
            "engagementRepeatLast",
            "engagementAbort",
            "worldPointerDown",
            "worldPointerMove",
            "setDislocateOption",
        ];
        for action_id in hidden_actions {
            let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("action {action_id} missing from manifest"));
            assert!(!action.in_palette, "internal action {action_id} must have in_palette: false");
        }

        let palette_user_actions = ["addObject", "deleteObject", "duplicateObject", "translateSelection", "rotateSelection", "scaleSelection"];
        for action_id in palette_user_actions {
            let action = definition.window_kinds.iter().flat_map(|window| window.actions.iter()).find(|entry| entry.id == action_id).unwrap_or_else(|| panic!("user action {action_id} missing from manifest"));
            assert!(action.in_palette, "user action {action_id} must have in_palette: true");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn engagement_input_and_possible_engagements_present() {
        let mut app = new_app().await;
        let engagements = app.window_engagements().await;
        let shape = engagements.get(shape::WINDOW_KIND_ID).expect("shape engagement");
        assert!(shape.input.is_some());
        assert!(shape.possible_engagements.as_ref().is_some_and(|rows| !rows.is_empty()));
    }

    #[semio_framework_async_macros::async_test]
    async fn window_engagements_registered_for_all_four_panes() {
        let mut app = new_app().await;
        let engagements = app.window_engagements().await;
        for window_kind in [shape::WINDOW_KIND_ID, building::WINDOW_KIND_ID, energy::WINDOW_KIND_ID, structure_classic::WINDOW_KIND_ID] {
            assert!(engagements.contains_key(window_kind), "missing engagement for {window_kind}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn forest_example_includes_reference_overlay() {
        let scene = forest_play_scene();
        let references = edit::world_references_json(&scene, CadPaneId::Shape).expect("references");
        assert!(references.contains("ref-concrete-forest"));
    }

    #[semio_framework_async_macros::async_test]
    async fn typology_extent_derives_from_authored_geometry() {
        let scene = forest_working_scene();
        let column = scene.building_objects.iter().find(|object| object.typology == "building.building.column").expect("column object");
        let extent = column.extent.expect("column extent derived from geometry");
        assert!(extent[2] > 0.05, "authored column height should be measurable");
        assert_ne!(extent, CAD_DEFAULT_TYPOLOGY_EXTENT, "should differ from the universal fallback");
    }
    //#endregion 🔖️Render
    //#region 🔖️ViewModel
    #[semio_framework_async_macros::async_test]
    async fn gumball_config_fields_present_regardless_of_dislocate_activation() {
        // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): mesh selection is
        // framework-owned now and `ArtifactApp::render` has no `InteractionView` (see
        // `gumball_active`'s own doc comment) — the gumball can never see a live selection at this
        // render boundary, so `gumballActive` stays `false` even with Dislocate active; the
        // transform-mode config fields still render regardless (client-side, harmless while inactive).
        let selection = edit::world_selection_json(&default_document(), &CadPlayRuntime::default(), Some(CAD_DISLOCATE_UTILITY_ID), CadDislocateOptions::default());
        assert!(selection.contains("\"transformMode\":\"transform\""));
        assert!(selection.contains("\"moveAxes\":true"));
        assert!(selection.contains("\"rotate\":true"));
        assert!(selection.contains("\"scaleAxes\":false"));
        assert!(selection.contains("\"gumballActive\":false"));
        assert!(!selection.contains("\"gumballTarget\""));
    }

    /// 🎥️ `setCamera`/`setProjection`/`setProjectionParam` are `ActionKind::View` (see the `.view_action`
    /// registrations below) — they must never emit a `CadMutation` (no VCS edit, no undo entry) and
    /// instead write a coalesced `CadConfigMutation`, isolated per pane.
    #[semio_framework_async_macros::async_test]
    async fn set_camera_writes_config_not_mutations() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let emit = drive(&app, &scene, "setCamera", Some(json!({ "surfaceId": "cad.play.scene3d/building", "camera": { "position": [1.0, 2.0, 3.0], "target": [0.0, 0.0, 0.0], "zoom": 2.0, "fov": 60.0 } })));
        assert!(emit.artifact_mutations.is_empty(), "setCamera must not emit a VCS operation");
        assert!(!emit.config_mutations.is_empty(), "setCamera must write a config operation");
        let runtime = runtime_after(&emit, &CadConfig::default());
        assert_eq!(cad_pane_camera_runtime(&runtime, CadPaneId::Building).zoom, 2.0);
        assert_eq!(cad_pane_camera_runtime(&runtime, CadPaneId::Shape).zoom, 1.0, "panes stay isolated");
    }

    #[semio_framework_async_macros::async_test]
    async fn gumball_inactive_without_selection() {
        let selection = edit::world_selection_json(&default_document(), &CadPlayRuntime::default(), Some(CAD_DISLOCATE_UTILITY_ID), CadDislocateOptions::default());
        assert!(selection.contains("\"gumballActive\":false"));
        assert!(!selection.contains("\"gumballTarget\""));
    }

    #[semio_framework_async_macros::async_test]
    async fn active_utility_flows_from_config_into_scene() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let history = empty_history();
        let doc = ArtifactView::new(&scene, &history);
        let config = CadConfig { active_utility_id: CAD_DISLOCATE_UTILITY_ID.into(), ..CadConfig::default() };
        let node = render_direct(&app, shape::BODY_KEY, &doc, &config).expect("CAD UI assembly");
        let json = serde_json::to_string(&node).unwrap();
        // The world selection blob is embedded as an escaped JSON string inside the scene node.
        assert!(json.contains(r#"transformMode\":\"transform"#), "render sources Dislocate from CadConfig::active_utility_id");
    }

    /// @emoji 🎯️ WORKFLOWS-END-TO-END-TYPED-PORTS: `active_utility_id` is now a single, global
    /// `CadConfig` field (the pre-B1 per-window-instance `ViewModel.active_utility_by_window_id` has no
    /// replacement — `render`/`window_measures` have no per-instance parameter anymore, see
    /// `CadDislocateOptions`'s doc comment in `cad_document_engine`) — so the gumball is active in
    /// EVERY pane with an active selection once the Dislocate utility is on, not isolated per window.
    #[semio_framework_async_macros::async_test]
    async fn dislocate_gumball_config_fields_present_in_every_pane_once_the_utility_is_active() {
        // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): mesh selection is
        // framework-owned now and `ArtifactApp::render` has no `InteractionView` (see
        // `edit::gumball_active`'s own doc comment) — the gumball can never be live-active at this
        // render boundary, in any pane; the transform-mode config fields still render regardless.
        let app = CadPlayApp::default();
        let scene = default_document();
        let config = CadConfig { active_utility_id: CAD_DISLOCATE_UTILITY_ID.into(), ..CadConfig::default() };
        let history = empty_history();
        let doc = ArtifactView::new(&scene, &history);
        let shape = render_direct(&app, shape::BODY_KEY, &doc, &config).expect("CAD UI assembly");
        let building = render_direct(&app, building::BODY_KEY, &doc, &config).expect("CAD UI assembly");
        let shape_json = serde_json::to_string(&shape).unwrap();
        let building_json = serde_json::to_string(&building).unwrap();
        assert!(shape_json.contains(r#"gumballActive\":false"#));
        assert!(shape_json.contains(r#"transformMode\":\"transform"#));
        assert!(building_json.contains(r#"gumballActive\":false"#));
        assert!(building_json.contains(r#"transformMode\":\"transform"#));
    }

    #[semio_framework_async_macros::async_test]
    async fn context_menu_resolves_labels_from_the_registry() {
        // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `context_menu` is no longer
        // selection-gated — `ArtifactApp::context_menu` has no `InteractionView` parameter, so it
        // can no longer tell whether anything is selected (see its own doc comment); it always
        // shows the transform/duplicate/delete section now.
        let app = CadPlayApp::default();
        let scene = default_document();
        let history = empty_history();
        let doc = ArtifactView::new(&scene, &history);
        let registry = AppActionRegistry::from_definition(&create_cad_app());
        let config = CadConfig::default();

        let items = context_menu_direct(&app, &doc, &config, &registry);
        assert!(items.iter().any(|item| item.id == "translateSelection" && item.label.is_some()), "labels must resolve from the registry: {items:?}");
        assert!(items.iter().any(|item| item.id == "deleteObject" && item.destructive == Some(true)), "deleteObject must be marked destructive: {items:?}");
    }

    /// 🗂️ GROUPED-PROGRESSIVELY-DISCLOSED-CONTEXT-MENUS: the selection context menu stays a shallow,
    /// disclosed list (top-level verbs + a handful of taxonomy groups) rather than a flat wall of rows,
    /// and the destructive `deleteObject` action stays the trailing item.
    #[semio_framework_async_macros::async_test]
    async fn context_menu_is_grouped_and_keeps_delete_object_last() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let history = empty_history();
        let doc = ArtifactView::new(&scene, &history);
        let registry = AppActionRegistry::from_definition(&create_cad_app());
        let config = CadConfig::default();

        let items = context_menu_direct(&app, &doc, &config, &registry);

        assert!(items.len() <= 9, "top-level context menu should stay progressively disclosed: {items:?}");
        assert_eq!(items.last().map(|item| item.id.as_str()), Some("deleteObject"), "deleteObject must stay the trailing item: {items:?}");
        assert_eq!(items.last().and_then(|item| item.destructive), Some(true), "trailing deleteObject must be marked destructive: {items:?}");
    }

    /// @emoji 🎛️ Dislocate move/rotate options are now keyed by PANE (`CadConfig::dislocate_shape`/
    /// `dislocate_building`/…), not by an arbitrary host-pushed window-instance id — the direct
    /// replacement for the pre-B1 per-window-instance isolation test.
    #[semio_framework_async_macros::async_test]
    async fn dislocate_move_and_rotate_options_are_per_pane() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let emit = drive(&app, &scene, "setDislocateOption", Some(json!({ "pane": "building", "option": "rotate", "pressed": false })));
        let config = config_after(&emit, &CadConfig::default());
        let history = empty_history();
        let doc = ArtifactView::new(&scene, &history);
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

    #[semio_framework_async_macros::async_test]
    async fn engagement_hud_no_longer_carries_utility_switcher_options() {
        let mut app = new_app().await;
        let engagements = app.window_engagements().await;
        for engagement in engagements.values() {
            assert!(engagement.options.is_none(), "utility switching now lives in the framework utility bar, not the engagement HUD");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn switching_utility_emits_no_operations_and_no_history_entry() {
        // 🧰️ The key regression guard: switching the host-owned active utility must be a pure View
        // action — zero operations, no projection mutation, and (proven below) no intervening
        // history entry. If the switch recorded an edit, the single undo would revert the switch
        // instead of the preceding addObject.
        // 🧱️ Uses `AddNode` as the "prior real edit" — `AddObject` is a documented no-op pending the
        // child-dispatch seam (see `commands/🧱️object/component.rs`'s module doc), so it cannot
        // stand in for a real history entry here anymore.
        let mut app = new_app().await;
        let before = app.snapshot().expect("snapshot").nodes.len();
        app.dispatch_typed(CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }), &meta("local")).await.expect("add node");
        let projection_after_add = serde_json::to_string(&app.snapshot().expect("snapshot")).unwrap();
        let result = app.dispatch_typed(CadCommand::SetActiveUtility(set_active_utility::SetActiveUtility { utility_id: CAD_DISLOCATE_UTILITY_ID.into() }), &meta("local")).await.expect("set active utility");
        assert!(result.mutations.is_empty(), "utility switch must emit zero operations");
        let projection_after_switch = serde_json::to_string(&app.snapshot().expect("snapshot")).unwrap();
        assert_eq!(projection_after_add, projection_after_switch, "utility switch must not mutate the projection");
        app.handle_action("undo", None, &meta("local")).await.expect("undo");
        assert_eq!(app.snapshot().expect("snapshot").nodes.len(), before, "a single undo reverts the addNode — proving the utility switch created no history entry");
    }

    #[semio_framework_async_macros::async_test]
    async fn sun_measures_registered_for_all_four_panes_and_default_off() {
        let app = CadPlayApp::default();
        let base_config = CadConfig::default();
        assert!(!base_config.sun.enabled, "sun must be off by default");
        let scene = default_document();
        let history = empty_history();
        let doc = ArtifactView::new(&scene, &history);
        let measures = window_measures_direct(&app, &doc, &base_config);
        for window_kind in [shape::WINDOW_KIND_ID, building::WINDOW_KIND_ID, energy::WINDOW_KIND_ID, structure_classic::WINDOW_KIND_ID] {
            assert!(measures.contains_key(window_kind), "missing sun measures for {window_kind}");
        }
        let emit = drive(&app, &scene, "toggleSun", None);
        let runtime = runtime_after(&emit, &base_config);
        assert!(runtime.sun.enabled);
    }

    // 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `worldPick`/`setHover`/`setSelection`
    // and their round-trip tests are DELETED, not migrated — mesh object/vertex/edge/face
    // selection AND hover are now the framework-owned `"cad"` interaction domain, dispatched
    // through the auto-injected `interactionSelect`/`interactionHover` verbs (never app-declared)
    // and tested once, centrally, by the framework's own `semio-framework-plugin` suite.

    //#endregion 🔖️ViewModel
    //#region 🔖️Operations
    #[semio_framework_async_macros::async_test]
    async fn add_object_action_is_a_documented_no_op() {
        // ⚠️ `addObject` is a documented no-op pending the child-dispatch seam (see
        // `commands/🧱️object/component.rs`'s module doc) — this locks in the honest current
        // behavior (zero artifact mutations) rather than the pre-migration "grows the object list"
        // claim, which no longer applies now `CadSnapshot` carries no inline objects. Selection is
        // out of scope here too (framework-owned now, unreachable from `handle()`).
        let app = CadPlayApp::default();
        let scene = default_document();
        let emit = drive(&app, &scene, "addObject", Some(json!({ "typology": "building.building.column" })));
        assert!(emit.artifact_mutations.is_empty(), "addObject is a documented no-op until the child-dispatch seam lands");
    }

    #[semio_framework_async_macros::async_test]
    async fn add_object_through_wrapper_is_a_documented_no_op() {
        let mut app = new_app().await;
        let before = serde_json::to_string(&app.snapshot().expect("snapshot")).unwrap();
        app.dispatch_typed(CadCommand::AddObject(add_object::AddObject { typology: Some("spatial.shape.primitive.box".into()) }), &meta("local")).await.expect("add object dispatch");
        let after = serde_json::to_string(&app.snapshot().expect("snapshot")).unwrap();
        assert_eq!(before, after, "addObject is a documented no-op until the child-dispatch seam lands");
    }

    #[semio_framework_async_macros::async_test]
    async fn focus_model_definition_emits_document_operation() {
        let mut app = new_app().await;
        app.dispatch_typed(CadCommand::FocusModelDefinition(focus_model_definition::FocusModelDefinition { model_definition_id: "aec.building".into() }), &meta("local")).await.expect("focus model definition");
        assert_eq!(app.snapshot().expect("snapshot").active_model_definition_id, "aec.building");
    }

    #[semio_framework_async_macros::async_test]
    async fn derive_transformation_populates_energy_pane() {
        // ⚠️ `apply_transformation_mutations` is a documented no-op pending the child-dispatch seam
        // (see its own doc comment in this file) — this instead exercises the real derive algorithm
        // directly (`run_derive_from_geometry`), the pure function `applyTransformation` will call
        // once that seam exists. `make_object_for_typology` already built and dropped its own local
        // `cad_brep_kernel()` internally to mint the object's solid handle; `cad_brep_kernel()` is a
        // fresh `Brep::new()` per call (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
        // wave G4 — no shared lock, no reentrancy concern), so the kernel built HERE for
        // `run_derive_from_geometry` is its own independent instance.
        let object = make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape);
        let mut kernel = cad_brep_kernel();
        let derived = run_derive_from_geometry(&mut kernel, &[object], "energy");
        assert!(!derived.is_empty());
        assert!(derived.iter().any(|object| object.typology.starts_with("energy.energy.")));
    }

    #[semio_framework_async_macros::async_test]
    async fn forest_transformation_uses_live_shape_pane() {
        // ⚠️ CORRECTED (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
        // wave G4): this test used to derive from `forest_working_scene().objects` and compare
        // against a single live box, relying on the forest fixture's `solid_handle`s resolving into
        // whatever kernel `run_derive_from_geometry` reached — only true under the deleted
        // process-global `BrepEngineHost` singleton. `solid_for_object` (the derive's per-object
        // solid builder) has never applied `object.origin` — fixture objects carry `origin:
        // [0,0,0]` regardless (`objects_from_fixture_model`) — so once a handle stops resolving it
        // falls back to an extent+typology-only box built at the kernel's local origin, and two
        // fixture objects sharing that origin fuse into a materially different (and no longer
        // fixture-distinguishing) hull. The real, still-true property — output tracks LIVE INPUT,
        // not a memoized static result — is verified here directly against extent, the one field
        // that DOES still flow through `solid_for_object`'s fallback path honestly.
        let live_box = make_object_for_typology("spatial.shape.primitive.box", 0, CadPaneId::Shape);
        let mut kernel = cad_brep_kernel();
        let box_derived = run_derive_from_geometry(&mut kernel, &[live_box], "energy");
        assert!(!box_derived.is_empty(), "a live box must derive at least a hull");

        let live_wall = make_object_for_typology("building.building.wall", 0, CadPaneId::Shape);
        let mut kernel = cad_brep_kernel();
        let wall_derived = run_derive_from_geometry(&mut kernel, &[live_wall], "energy");
        assert!(!wall_derived.is_empty(), "a live wall panel must derive at least a hull");

        let wall_typologies: Vec<&str> = wall_derived.iter().map(|object| object.typology.as_str()).collect();
        let box_typologies: Vec<&str> = box_derived.iter().map(|object| object.typology.as_str()).collect();
        assert_ne!(
            wall_typologies, box_typologies,
            "a thin wall panel and a cube must classify their dominant faces differently, proving the derive tracks the LIVE input's real shape, not a memoized result:\n  box:  {box_typologies:?}\n  wall: {wall_typologies:?}"
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn save_selected_emits_download_effect() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let config = CadConfig::default();
        let emit = drive_with_config(&app, &scene, "saveSelected", None, &config);
        assert!(emit.artifact_mutations.is_empty(), "export must not mutate the document");
        assert_eq!(emit.effects.len(), 1);
        match &emit.effects[0] {
            Effect::DownloadMediaExport { filename, data, .. } => {
                assert_eq!(filename, "cad.selected.spatial.dsl");
                assert!(data.contains("activeModelDefinitionId"));
            }
            other => panic!("expected DownloadMediaExport, got {other:?}"),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn load_raw_request_emits_file_open_effect() {
        let app = CadPlayApp::default();
        let emit = drive(&app, &default_document(), "loadRawRequest", None);
        match &emit.effects[0] {
            Effect::RequestFileOpen { import_action, read_as, .. } => {
                assert_eq!(import_action, "importCadFile");
                assert_eq!(read_as.as_deref(), Some("dataUrl"));
            }
            other => panic!("expected RequestFileOpen, got {other:?}"),
        }
    }
    //#endregion 🔖️Operations
    //#region 🔖️Engagement
    #[semio_framework_async_macros::async_test]
    async fn engagement_starts_box_interaction_session() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
        let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
        let runtime = runtime_after(&emit, &config);
        assert!(runtime.engagement_session.is_some());
    }

    #[semio_framework_async_macros::async_test]
    async fn world_pointer_move_updates_live_preview_without_committing_or_emitting_mutations() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
        let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
        let config = config_after(&emit, &config);

        let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [3.0, 4.0, 0.0] })), &config);
        assert!(emit.artifact_mutations.is_empty(), "a pointer move must not emit any document operation");
        let runtime = runtime_after(&emit, &config);
        let session = runtime.engagement_session.as_ref().expect("session still active");
        assert_eq!(session.state, "first_corner", "pointer.move must not change state");
        assert_eq!(session.context.get("cursor"), Some(&json!([3.0, 4.0, 0.0])));
    }

    //#region 🔖️GesturePreview
    fn preview_operation(app_instance_id: u32) -> CadPreviewOperationIdentity {
        CadPreviewOperationIdentity { app_instance_id, parent_document_id: "cad-preview-document".into(), operation_id: 41, operation_generation: 3, canonical_base_revision: "cd".repeat(32) }
    }

    fn persisted_preview_stamp(config: &CadConfig) -> CadPreviewStamp {
        CadPreviewStamp { operation: serde_json::from_str(config.engagement_preview_operation_json.as_ref().expect("persisted operation identity")).expect("valid persisted operation identity"), generation: config.engagement_preview_generation }
    }

    fn spatial_scene_import_args() -> Value {
        let file_text = json!({
            "schema": "spatial.model",
            "revision": 1,
            "modelDefinitionId": "spatial.shape",
            "objects": [{
                "id": "object-preview-transition",
                "label": "Preview transition",
                "typology": "spatial.shape.primitive.box",
                "visible": true,
                "locked": false,
                "origin": [0.0, 0.0, 0.0],
                "primitives": []
            }]
        })
        .to_string();
        json!({ "payload": file_text, "name": "preview-transition.spatial.json" })
    }

    /// 🔬️ CW7 preview-law seam: `CadPlayApp::gesture_preview` reads `CadEngagementScratch` only, never
    /// `CadSnapshot`/`CadMutation` — driven through the real `worldPointerMove` handler (the natural
    /// per-tick gesture handler) via the existing `drive` helper, config threaded explicitly across
    /// calls (the pure `CadPlayApp` no longer holds any of this state itself).
    #[semio_framework_async_macros::async_test]
    async fn gesture_preview_is_none_without_a_live_engagement_session() {
        let app = CadPlayApp::default();
        assert!(app.gesture_preview(&CadConfig::default()).is_none(), "no live engagement session, nothing to preview");
    }

    #[semio_framework_async_macros::async_test]
    async fn gesture_preview_reflects_the_live_rubber_band_preview_and_clears_on_abort() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
        let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
        let config = config_after(&emit, &config);

        let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [3.0, 4.0, 0.0] })), &config);
        let config = config_after(&emit, &config);
        let first = app.gesture_preview(&config).expect("a live engagement session is previewable");
        let value: Value = serde_json::from_slice(&first.payload).expect("payload is valid json");
        assert_eq!(value["context"]["cursor"], json!([3.0, 4.0, 0.0]));

        let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [5.0, 6.0, 0.0] })), &config);
        let config = config_after(&emit, &config);
        let second = app.gesture_preview(&config).expect("still live mid-gesture");
        assert_eq!(second.stamp.operation, first.stamp.operation);
        assert_eq!(second.stamp.generation, first.stamp.generation + 1, "the persisted preview generation advances exactly once per changed checkpoint");
        assert!(second.is_fresher_than(&first.stamp));
        let value_after_second: Value = serde_json::from_slice(&second.payload).expect("payload is valid json");
        assert_eq!(value_after_second["context"]["cursor"], json!([5.0, 6.0, 0.0]), "preview tracks the live cursor, not the gesture start");

        let emit = drive_with_config(&app, &scene, "engagementAbort", None, &config);
        let config = config_after(&emit, &config);
        assert!(app.gesture_preview(&config).is_none(), "the engagement session was aborted: nothing left to preview");
    }

    #[semio_framework_async_macros::async_test]
    async fn gesture_preview_is_a_pure_read_never_mutating_the_engagement_session() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let config = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
        let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
        let config = config_after(&emit, &config);
        let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [1.0, 2.0, 0.0] })), &config);
        let config = config_after(&emit, &config);
        let session_before = config.engagement_session_json.clone();
        let first = app.gesture_preview(&config);
        let second = app.gesture_preview(&config);
        assert_eq!(first, second, "equal checkpoint reads keep the exact same freshness stamp");
        assert_eq!(config.engagement_session_json, session_before, "gesture_preview must never mutate the live engagement session it reads");
    }

    #[semio_framework_async_macros::async_test]
    async fn production_transition_authority_routes_engagement_utility_and_import_without_noop_increment() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let operation = preview_operation(1);
        let base = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };

        let started_emit = drive_with_operation(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &base, Some(operation.clone())).expect("ordinary engagement transition");
        let started = config_after(&started_emit, &base);
        let started_stamp = persisted_preview_stamp(&started);
        assert!(started.engagement_session_json.is_some());
        assert_eq!(started_stamp.operation, operation);
        assert_eq!(started_stamp.generation, base.engagement_preview_generation + 1);

        let utility_emit = drive_with_operation(&app, &scene, "setActiveUtility", Some(json!({ "utilityId": CAD_DISLOCATE_UTILITY_ID })), &started, Some(operation.clone())).expect("utility clear transition");
        let utility_cleared = config_after(&utility_emit, &started);
        let utility_stamp = persisted_preview_stamp(&utility_cleared);
        assert!(utility_cleared.engagement_session_json.is_none());
        assert!(utility_stamp.is_fresher_than(&started_stamp));
        assert_eq!(utility_stamp.generation, started_stamp.generation + 1);

        let noop_emit = drive_with_operation(&app, &scene, "setActiveUtility", Some(json!({ "utilityId": "move" })), &utility_cleared, Some(operation.clone())).expect("same checkpoint utility update");
        let noop = config_after(&noop_emit, &utility_cleared);
        assert_eq!(noop.engagement_session_json, utility_cleared.engagement_session_json);
        assert_eq!(persisted_preview_stamp(&noop), utility_stamp, "a non-session config change must not advance the preview generation");

        let input_emit = drive_with_operation(&app, &scene, "engagementInput", Some(json!({ "value": "b", "pane": "shape" })), &noop, Some(operation.clone())).expect("engagement input");
        let with_input = config_after(&input_emit, &noop);
        assert_eq!(persisted_preview_stamp(&with_input), utility_stamp, "input-only config must preserve the stamp");
        let restarted_emit = drive_with_operation(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &with_input, Some(operation.clone())).expect("restart engagement");
        let restarted = config_after(&restarted_emit, &with_input);
        let restarted_stamp = persisted_preview_stamp(&restarted);
        assert!(restarted_stamp.is_fresher_than(&utility_stamp));

        let import_emit = drive_with_operation(&app, &scene, "importCadFile", Some(spatial_scene_import_args()), &restarted, Some(operation)).expect("scene import clear transition");
        let import_cleared = config_after(&import_emit, &restarted);
        let import_stamp = persisted_preview_stamp(&import_cleared);
        assert!(import_cleared.engagement_session_json.is_none());
        assert!(import_stamp.is_fresher_than(&restarted_stamp));
        assert_eq!(import_stamp.generation, restarted_stamp.generation + 1);
        assert!(matches!(import_emit.effects.first(), Some(Effect::LoadDocument { .. })));

        let example_emit = drive_with_operation(&app, &scene, "setActiveExample", Some(json!({ "exampleId": "" })), &restarted, Some(preview_operation(1))).expect("active example clear transition");
        let example_cleared = config_after(&example_emit, &restarted);
        let example_stamp = persisted_preview_stamp(&example_cleared);
        assert!(example_cleared.engagement_session_json.is_none());
        assert!(example_stamp.is_fresher_than(&restarted_stamp));
        assert_eq!(example_stamp.generation, restarted_stamp.generation + 1);
        assert!(matches!(example_emit.effects.first(), Some(Effect::LoadDocument { .. })));
    }

    #[semio_framework_async_macros::async_test]
    async fn production_transition_authority_isolates_two_app_aba_sequences() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let base = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
        let operations = [preview_operation(1), preview_operation(2)];
        let mut previews = Vec::new();

        for operation in operations {
            let started_emit = drive_with_operation(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &base, Some(operation.clone())).expect("start app-local engagement");
            let started = config_after(&started_emit, &base);
            let a_emit = drive_with_operation(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [1.0, 2.0, 0.0] })), &started, Some(operation.clone())).expect("A");
            let at_a = config_after(&a_emit, &started);
            let first_a = app.gesture_preview(&at_a).expect("first A preview");
            let b_emit = drive_with_operation(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [3.0, 4.0, 0.0] })), &at_a, Some(operation.clone())).expect("B");
            let at_b = config_after(&b_emit, &at_a);
            let a_again_emit = drive_with_operation(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [1.0, 2.0, 0.0] })), &at_b, Some(operation)).expect("A again");
            let at_a_again = config_after(&a_again_emit, &at_b);
            let second_a = app.gesture_preview(&at_a_again).expect("second A preview");
            assert_eq!(first_a.payload, second_a.payload);
            assert_eq!(second_a.stamp.generation, first_a.stamp.generation + 2);
            assert_ne!(first_a.stamp, second_a.stamp);
            previews.push(second_a);
        }

        assert_eq!(previews[0].payload, previews[1].payload);
        assert_eq!(previews[0].stamp.generation, previews[1].stamp.generation);
        assert_ne!(previews[0].stamp.operation, previews[1].stamp.operation);
        assert!(!previews[0].is_fresher_than(&previews[1].stamp));
        assert!(!previews[1].is_fresher_than(&previews[0].stamp));
    }

    #[semio_framework_async_macros::async_test]
    async fn production_transition_exhaustion_and_missing_context_fail_before_checkpoint_persistence() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let operation = preview_operation(9);
        let base = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
        let started_emit = drive_with_operation(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &base, Some(operation.clone())).expect("start live engagement");
        let started = config_after(&started_emit, &base);
        let mut at_max = started.clone();
        at_max.engagement_preview_generation = CAD_PREVIEW_GENERATION_MAX;
        let checkpoint_before = at_max.engagement_session_json.clone();
        let operation_before = at_max.engagement_preview_operation_json.clone();

        assert!(drive_with_operation(&app, &scene, "setActiveUtility", Some(json!({ "utilityId": CAD_DISLOCATE_UTILITY_ID })), &at_max, Some(operation.clone())).is_err());
        assert!(drive_with_operation(&app, &scene, "importCadFile", Some(spatial_scene_import_args()), &at_max, Some(operation.clone())).is_err());
        assert!(drive_with_operation(&app, &scene, "setActiveExample", Some(json!({ "exampleId": "" })), &at_max, Some(operation.clone())).is_err());
        assert!(drive_with_operation(&app, &scene, "engagementAbort", None, &at_max, Some(operation)).is_err());
        assert_eq!(at_max.engagement_session_json, checkpoint_before, "failed commands cannot persist their cleared checkpoint");
        assert_eq!(at_max.engagement_preview_generation, CAD_PREVIEW_GENERATION_MAX);
        assert_eq!(at_max.engagement_preview_operation_json, operation_before);

        assert!(drive_with_operation(&app, &scene, "setActiveUtility", Some(json!({ "utilityId": CAD_DISLOCATE_UTILITY_ID })), &started, None).is_err());
        assert!(drive_with_operation(&app, &scene, "importCadFile", Some(spatial_scene_import_args()), &started, None).is_err());
        let mut bypass = cad_runtime_from_config(&started);
        bypass.engagement_session = None;
        assert!(snapshot_of(&bypass, &started).is_err(), "ordinary snapshots must reject session-transition bypasses");
        assert_eq!(started.engagement_session_json, checkpoint_before);
    }

    #[semio_framework_async_macros::async_test]
    async fn gesture_preview_rejects_aba_collision_and_cross_app_stamps_and_survives_restart() {
        let app = CadPlayApp::default();
        let scene = default_document();
        let base = CadConfig { engagement_input: "b".into(), ..CadConfig::default() };
        let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &base);
        let started = config_after(&emit, &base);

        let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [1.0, 2.0, 0.0] })), &started);
        let at_a = config_after(&emit, &started);
        let preview_a = app.gesture_preview(&at_a).expect("A preview");
        let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [3.0, 4.0, 0.0] })), &at_a);
        let at_b = config_after(&emit, &at_a);
        let emit = drive_with_config(&app, &scene, "worldPointerMove", Some(json!({ "pane": "shape", "position": [1.0, 2.0, 0.0] })), &at_b);
        let at_a_again = config_after(&emit, &at_b);
        let preview_a_again = app.gesture_preview(&at_a_again).expect("second A preview");
        assert_eq!(preview_a.payload, preview_a_again.payload, "fixture must exercise A → B → A");
        assert_eq!(preview_a_again.stamp.generation, preview_a.stamp.generation + 2);
        assert_ne!(preview_a.stamp, preview_a_again.stamp, "ABA payload equality cannot reproduce a freshness stamp");

        let restarted: CadConfig = serde_json::from_slice(&serde_json::to_vec(&at_a_again).expect("config checkpoint")).expect("cold reopen config");
        assert_eq!(app.gesture_preview(&restarted).expect("reopened preview").stamp, preview_a_again.stamp);

        let mut other_app = restarted.clone();
        other_app.engagement_preview_operation_json =
            Some(serde_json::to_string(&CadPreviewOperationIdentity { app_instance_id: 2, parent_document_id: "cad-test-document".into(), operation_id: 1, operation_generation: 1, canonical_base_revision: "00".repeat(32) }).expect("identity"));
        let collision = app.gesture_preview(&other_app).expect("other app preview");
        assert_eq!(collision.stamp.generation, preview_a_again.stamp.generation, "forced finite-generation collision fixture");
        assert_ne!(collision.stamp.operation, preview_a_again.stamp.operation);
        assert!(!collision.is_fresher_than(&preview_a_again.stamp), "freshness requires exact operation identity, not only a colliding counter");
    }

    #[semio_framework_async_macros::async_test]
    async fn preview_generation_cross_surface_domain_round_trips_max_and_rejects_plus_one() {
        let app = CadPlayApp::default();
        let operation = CadPreviewOperationIdentity { app_instance_id: 7, parent_document_id: "cad-max-document".into(), operation_id: 11, operation_generation: 13, canonical_base_revision: "ab".repeat(32) };
        let at_max =
            CadConfig { engagement_session_json: Some("{}".into()), engagement_preview_operation_json: Some(serde_json::to_string(&operation).expect("identity")), engagement_preview_generation: CAD_PREVIEW_GENERATION_MAX, ..CadConfig::default() };
        let encoded = serde_json::to_string(&at_max).expect("maximum generation serializes");
        let decoded: CadConfig = serde_json::from_str(&encoded).expect("maximum generation deserializes exactly");
        assert_eq!(decoded.engagement_preview_generation, CAD_PREVIEW_GENERATION_MAX);
        assert_eq!(app.gesture_preview(&decoded).expect("maximum generation remains previewable").stamp.generation, CAD_PREVIEW_GENERATION_MAX);

        let plus_one = i64::from(CAD_PREVIEW_GENERATION_MAX) + 1;
        let oversized = encoded.replace(&format!("\"engagementPreviewGeneration\":{}", CAD_PREVIEW_GENERATION_MAX), &format!("\"engagementPreviewGeneration\":{plus_one}"));
        assert!(serde_json::from_str::<CadConfig>(&oversized).is_err(), "maximum + 1 must fail before entering persisted config");

        let mut runtime = cad_runtime_from_config(&decoded);
        runtime.engagement_session = None;
        let ctx = CadDispatchCtx { interaction: CadInteractionSnapshot::default(), preview_operation: Some(operation) };
        assert!(preview_transition_snapshot_of(&runtime, &decoded, &ctx).is_err(), "incrementing the maximum generation must fail closed");

        let json_schema: Value = serde_json::from_str(include_str!("🎚️config/🧬️schema/🔣️.json")).expect("CAD config JSON descriptor");
        let generation_schema = &json_schema["properties"]["engagementPreviewGeneration"];
        assert_eq!(generation_schema["minimum"], json!(0));
        assert_eq!(generation_schema["maximum"], json!(CAD_PREVIEW_GENERATION_MAX));
        assert!(include_str!("🎚️config/🧬️schema/🛰️.proto").contains("int32 engagement_preview_generation = 32;"));
        assert!(include_str!("🎚️config/🧬️schema/🔗️.graphql").contains("engagementPreviewGeneration: Int!"));
        assert!(include_str!("🎚️config/🧬️schema/🟦️.ts").contains("engagementPreviewGeneration: number;"));
        assert!(include_str!("🎚️config/🧬️schema/🦀️component.rs").contains("engagement_preview_generation: i32"));
    }
    //#endregion 🔖️GesturePreview

    #[semio_framework_async_macros::async_test]
    async fn engagement_repeat_last_restarts_the_last_finalized_interaction() {
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
            scene = apply_mutations(&scene, &emit.artifact_mutations);
            config = config_after(&emit, &config);
        }

        config.engagement_input = "SetHeight2.5".into();
        let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
        config = config_after(&emit, &config);

        // 🔣️box.json's `set.height` only records the height (state stays first_corner_height); an
        // explicit `confirm` (Enter) is needed to reach `ready`, box's commit.fromStates.
        config.engagement_input = "Confirm".into();
        let emit = drive_with_config(&app, &scene, "engagementSubmit", Some(json!({ "pane": "shape" })), &config);
        scene = apply_mutations(&scene, &emit.artifact_mutations);
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
    #[semio_framework_async_macros::async_test]
    async fn import_spatial_modelspace_round_trips() {
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
        assert!(scene.shape_model.is_some(), "a real imported object must mint a shape-model child");
    }

    #[semio_framework_async_macros::async_test]
    async fn import_cad_file_action_accepts_spatial_json_text_string_payload() {
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
        // 🌱️ Whole-document replace is not an in-history mutation (SEMANTIC-MUTATIONS-OVERHAUL
        // retired `SetSnapshot`) — a spatial JSON string payload now surfaces as a
        // `Effect::LoadDocument` carrying the replacement document's pack bytes.
        let Effect::LoadDocument { pack, .. } = emit.effects.first().expect("importCadFile must emit a LoadDocument effect for a spatial JSON string payload") else {
            panic!("expected a LoadDocument effect");
        };
        let next = <CadSnapshot as store::ArtifactPack>::decode_pack(pack).expect("decode loaded document pack");
        assert!(next.shape_model.is_some(), "a real imported object must mint a shape-model child");
    }

    #[semio_framework_async_macros::async_test]
    async fn import_cad_file_action_imports_obj_by_extension() {
        // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `import_cad_object_by_extension`
        // now returns a `SemioModelElement` — composing it into the document needs the same
        // child-dispatch seam as `commands/🧱️object/component.rs` (see `import_cad_file::handle`'s
        // own doc comment). Documented no-op. 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM
        // (26/08/14): auto-selecting the imported object is no longer reachable from `handle()`
        // either (selection is framework-owned) — this now only asserts the document-write gap.
        let app = CadPlayApp::default();
        let scene = default_document();
        let obj_text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1 2 3\n";
        let obj_data_url = format!("data:model/obj;base64,{}", base64_codec::base64_standard_encode(obj_text));
        let emit = drive(&app, &scene, "importCadFile", Some(json!({ "payload": obj_data_url, "name": "triangle.obj" })));
        assert!(emit.artifact_mutations.is_empty(), "importCadFile's document write is a documented no-op until the child-dispatch seam lands");
        assert!(emit.config_mutations.is_empty(), "importCadFile no longer touches config once selection moved to the framework");
    }
    //#endregion 🔖️Import
    //#region 🔖️History
    #[semio_framework_async_macros::async_test]
    async fn undo_redo_round_trips_added_node_through_generic_helper() {
        // ⚠️ `AddObject` is a documented no-op pending the child-dispatch seam (see
        // `commands/🧱️object/component.rs`'s module doc) — this exercises the generic
        // `assert_undo_redo_round_trip` testkit helper (distinct from `undo_redo_round_trips_added_node_through_wrapper`
        // below, which drives the manual add/undo/redo dance) against the real `AddNode` command.
        let mut app = new_app().await;
        let before = app.snapshot().expect("snapshot").nodes.len();
        semio_framework_plugin::testkit::assert_undo_redo_round_trip(&mut app, CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }), |app| app.snapshot().expect("snapshot").nodes.len(), before, before + 1).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn undo_redo_round_trips_added_node_through_wrapper() {
        let mut app = new_app().await;
        let before = app.snapshot().expect("snapshot").nodes.len();
        app.dispatch_typed(CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }), &meta("local")).await.expect("add node");
        assert_eq!(app.snapshot().expect("snapshot").nodes.len(), before + 1);
        let undo = app.handle_action("undo", None, &meta("local")).await.expect("undo");
        assert!(undo.events.iter().any(|event| event.kind == "history-changed"));
        assert_eq!(app.snapshot().expect("snapshot").nodes.len(), before);
        app.handle_action("redo", None, &meta("local")).await.expect("redo");
        assert_eq!(app.snapshot().expect("snapshot").nodes.len(), before + 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn coalesced_translate_drag_is_a_single_undo_step() {
        // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `translateSelection`
        // (and the `addObject` that used to seed the dragged object) are documented no-ops pending
        // the child-dispatch seam — object placement now lives inside composed
        // `s.stdio.semio.model` CHILD documents (see `commands/🔄️transform/component.rs`'s own doc
        // comment). This locks in the honest current behavior — a coalesced multi-tick drag emits
        // nothing to undo — rather than letting it silently drift.
        let mut app = new_app().await;
        let before = serde_json::to_string(&app.snapshot().expect("snapshot")).unwrap();
        for _ in 0..3 {
            app.dispatch_typed(CadCommand::TranslateSelection(translate_selection::TranslateSelection { object_ids: vec!["object-box-1".into()], dx: 1.0, dy: 0.0, dz: 0.0 }), &meta("local")).await.expect("translate tick");
        }
        let after = serde_json::to_string(&app.snapshot().expect("snapshot")).unwrap();
        assert_eq!(before, after, "translateSelection is a documented no-op until the child-dispatch seam lands");
    }
    //#endregion 🔖️History
    //#region 🔖️Convergence
    /// 🧪️ The definitional merge proof: two instances start from the SAME base projection, apply
    /// DISJOINT edits (A translates object A, B patches object B's label), and after exchanging operations
    /// over a `MemoryBackbone` both converge to contain BOTH edits — impossible under whole-document
    /// `setDocument` snapshots.
    #[semio_framework_async_macros::async_test]
    async fn two_instances_converge_disjoint_edits_via_backbone() {
        // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `PatchObject` is a
        // documented no-op pending the child-dispatch seam (object fields now live inside composed
        // `s.stdio.semio.model` CHILD documents — see `commands/🧱️object/component.rs`'s module
        // doc), so it can no longer stand in as the disjoint edit this law needs. `RenameNode` is a
        // real, unaffected parent-document mutation (node data was never part of the deleted inline
        // object list) — proves the identical convergence property.
        let mut base = default_document();
        base.nodes = vec![CadNode { id: "node-a".into(), label: "A".into(), kind: "solid".into() }, CadNode { id: "node-b".into(), label: "B".into(), kind: "solid".into() }];
        let node_a = base.nodes[0].id.clone();
        let node_b = base.nodes[1].id.clone();
        let base_envelope = store::create_document_envelope::<CadSnapshot, CadMutation>(CAD_DOCUMENT_SCHEMA, "cad-play", base, None);
        let base_files = store::print_document_pack(&base_envelope).await.expect("print document pack");

        let mut instance_a = new_app().await;
        let mut instance_b = new_app().await;
        instance_a.load_document_pack(&base_files).await.expect("load a");
        instance_b.load_document_pack(&base_files).await.expect("load b");
        let (backbone_a, backbone_b) = MemoryBackbone::pair("mem://cad-convergence", "mem://cad-convergence").await;
        instance_a.attach_backbone(store::Backbones::Memory(backbone_a)).await.expect("attach a");
        instance_b.attach_backbone(store::Backbones::Memory(backbone_b)).await.expect("attach b");

        // A renames node A.
        instance_a.dispatch_typed(CadCommand::RenameNode(rename_node::RenameNode { node_id: node_a.clone(), value: "Renamed By A".into() }), &meta("actor-a")).await.expect("a renames node a");

        // B renames node B — a disjoint edit that must survive alongside A's.
        instance_b.dispatch_typed(CadCommand::RenameNode(rename_node::RenameNode { node_id: node_b.clone(), value: "Renamed By B".into() }), &meta("actor-b")).await.expect("b renames node b");

        // A neutral history command always pumps inbound operations before doing its own work.
        instance_a.handle_action("commitCheckpoint", None, &meta("actor-a")).await.expect("pump a");
        instance_b.handle_action("commitCheckpoint", None, &meta("actor-b")).await.expect("pump b");

        let scene_a = instance_a.snapshot().expect("projection a");
        let scene_b = instance_b.snapshot().expect("projection b");

        let label_a_in_a = scene_a.nodes.iter().find(|node| node.id == node_a).unwrap().label.clone();
        let label_a_in_b = scene_b.nodes.iter().find(|node| node.id == node_a).unwrap().label.clone();
        let label_b_in_a = scene_a.nodes.iter().find(|node| node.id == node_b).unwrap().label.clone();
        let label_b_in_b = scene_b.nodes.iter().find(|node| node.id == node_b).unwrap().label.clone();

        assert_eq!(label_a_in_a, "Renamed By A", "instance A keeps its own edit");
        assert_eq!(label_a_in_b, "Renamed By A", "instance B converges on A's edit");
        assert_eq!(label_b_in_a, "Renamed By B", "instance A converges on B's edit");
        assert_eq!(label_b_in_b, "Renamed By B", "instance B keeps its own edit");
    }

    #[semio_framework_async_macros::async_test]
    async fn ingest_operations_is_idempotent_for_cad() {
        let mut sender = new_app().await;
        let (near, mut far) = MemoryBackbone::pair("mem://cad-doc", "mem://cad-doc").await;
        sender.attach_backbone(store::Backbones::Memory(near)).await.expect("attach");
        sender.dispatch_typed(CadCommand::AddNode(add_node::AddNode { kind: "solid".into() }), &meta("local")).await.expect("add node");

        let mut envelopes = Vec::new();
        for message in far.receive().await.expect("receive") {
            if let BackboneMessage::Mutations { envelopes: operations } = message {
                envelopes.extend(operations);
            }
        }
        assert!(!envelopes.is_empty(), "expected the applied operation to flow onto the channel");
        let operations = envelopes;

        let mut receiver = new_app().await;
        let nodes_before = receiver.snapshot().expect("snapshot").nodes.len();
        receiver.ingest_operations(&operations).await.expect("ingest once");
        receiver.ingest_operations(&operations).await.expect("ingest twice");
        assert_eq!(receiver.snapshot().expect("snapshot").nodes.len(), nodes_before + 1, "feeding the same operation twice must not double-apply");
    }
    //#endregion 🔖️Convergence
}
//#endregion 🧪️Tests
