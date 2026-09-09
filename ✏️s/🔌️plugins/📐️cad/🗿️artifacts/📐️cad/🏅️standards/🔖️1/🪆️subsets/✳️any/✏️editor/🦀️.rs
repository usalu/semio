//! 📐️ CAD play app — the plugin's play app: its `ArtifactApp` impl (dispatch-only), the
//! `CadPlayRuntime` scratch mirror of `CadConfig`, the shared view/export helpers its command,
//! panel and window nodes build on, and the manifest that stitches those nodes together.
//!
//! 🧭️ Every behavioural arm lives in `🎮️commands/<group>/🦀️.rs`; every rendered surface in
//! `📌️panels/<panel>` or `🎭️modes/✏️edit/🪟️windows/<window>`. This file dispatches and stitches.

use crate::editor::cad::commands::camera::{set_camera, set_projection, set_projection_param};
use crate::editor::cad::commands::contribution::set_contributions;
use crate::editor::cad::commands::engagement::{engagement_abort, engagement_input, engagement_possible_select, engagement_repeat_last, engagement_submit, world_pointer_down, world_pointer_move};
use crate::editor::cad::commands::io::{import_cad_file, load_raw_request, save_current, save_in_play, save_selected};
use crate::editor::cad::commands::model_definition::{focus_model_definition, set_active_example};
use crate::editor::cad::commands::node::{add_node, rename_node, set_node_selection};
use crate::editor::cad::commands::object::{add_object, delete_object, duplicate_object, patch_object, patch_selection};
use crate::editor::cad::commands::reference::{patch_cad_play_reference, reference_hover, set_reference_selection};
use crate::editor::cad::commands::sun::{set_sun_azimuth, set_sun_elevation, set_sun_intensity, toggle_sun};
use crate::editor::cad::commands::transform::{apply_transformation, rotate_selection, scale_selection, translate_selection};
use crate::editor::cad::commands::utility::set_dislocate_option;
use crate::editor::cad::config::{cad_sun_config_from_world, cad_sun_config_to_world, deserialize_cad_preview_generation, CadConfig, CadConfigMutation, CadDislocateOptions, CAD_PREVIEW_GENERATION_MAX};
use crate::editor::cad::engine::interaction::{self, apply_event, can_commit, commit_object, keyed_transitions, parse_repl_line, resolve_interaction_key, start_session, CadEngagementScratch};
use crate::editor::cad::modes::edit;
use crate::editor::cad::modes::edit::windows::{building, energy, shape, structure_classic};
use crate::editor::cad::panels::{catalogue, document, inspection};
use crate::editor::cad::terminology::{cad_is_de_locale, cad_labels};
use crate::op::CadMutation;
use crate::standards::v1::subsets::any::io::{export_solids_as, CadSolidExport, CAD_SOLID_EXPORT_DIALECT_STEP};
use crate::standards::v1::subsets::any::schema::inferences::{
    cad_brep_kernel, cad_camera_projection_config, ensure_object_solid_handle, forest_play_scene, next_cad_id, CAD_EXAMPLE_FOREST_LEFT, CAD_MODEL_DEFINITION_BUILDING, CAD_MODEL_DEFINITION_ENERGY, CAD_MODEL_DEFINITION_SHAPE,
    CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC,
};
use crate::{artifact_kind, cad_pane_from_model_definition_id, CadCamera, CadPaneId, CadSnapshot, CadWorkingScene, CAD_DOCUMENT_SCHEMA};
use dsl::json;
use semio_framework::kernel::Effect;
use semio_framework::{InteractiveJobClassification, ToolExecutionContract, ToolFactoryKey, ToolJobFactory, ToolJobFactoryError};
use semio_framework_plugin::retained_command::{ArtifactCommandWork, ArtifactRetainedCommandJob, ArtifactRetainedCommandPayload, BoundedArtifactCommandWork};
use semio_framework_plugin::{
    tree_item_with_action, world3d_camera_projection_json, ActionArgDef, ActionArgOption, ActionDefinition, ActionDescriptor, ActionKind, AppActionRegistry, AppOperationContext, ArtifactOwnedToolJobFactory, ArtifactOwnedToolJobRequest,
    ArtifactToolFactoryRegistry, ArtifactToolPublicationContract, ArtifactToolPublicationLane, ArtifactView, CommandDefinition, ConfigView, ContextMenuItemSpec, ContextMenuRequest, DraftView, EditorApp, Emit, Fault, Label, LocalizedLabel, Media,
    MediaClass, MediaError, MediaForm, MediaPayload, MediaType, Menu, NoDraft, NoDraftMutation, PluginAssemblyError, UiText, UiValue, UtilityCategory, UtilityDefinition, ViewModel, WindowEngagement, WindowMeasure, WorldSunConfig,
};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::{Brep, GeometryHandle};
// 🚧️ SDK GAP: `ArtifactEditor`/`Editor`/`Dialect` (ticket 26/08/16 contract §2.1/§2.4)? are not yet
// in `semio_framework_plugin`'s curated crate-root re-export list (`🔌️plugin/🦀️.rs:17858`)
// — only reachable through the `app` submodule they're actually declared in. Not fixable here
// (`🧰️framework/**` is outside this packet's lease); flagged for W1-A in the migration report.
use semio_framework_plugin::app::{ArtifactEditor, Dialect, Editor};
use semio_framework_value_derive::{FromValue, ToValue};
// 🌉️ Test harness only — `protocol::json` (the `pack::json` re-export) carries its own first-party
// `json!` macro (`Value`-producing, same object/array literal syntax as `serde_json::json!`) and
// `Value` type (`Index`/`PartialEq<&str>`/`as_*` parity with `serde_json::Value`), so no
// `serde_json` dependency survives even here (ticket 26/09/01/
// RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS).
#[cfg(test)]
use protocol::os_pack::json::Value;
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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct CadPlayRuntime {
    /// 👁️ Document-tree node selection — app-owned (not a mesh-geometry granularity).
    #[value(default)]
    pub selected_node_ids: Vec<String>,
    /// 🐁️ Hovered reference-overlay id — app-owned, distinct from the framework `"cad"` domain hover.
    #[value(default)]
    pub hovered_reference_id: Option<String>,
    #[value(default)]
    pub engagement_input: String,
    #[value(default)]
    pub engagement_step: String,
    #[value(default)]
    pub active_example_id: Option<String>,
    #[value(default)]
    pub selected_reference_model_definition_id: Option<String>,
    #[value(default)]
    pub selected_reference_id: Option<String>,
    #[value(default)]
    pub engagement_pane: Option<String>,
    #[value(default)]
    pub engagement_session: Option<CadEngagementScratch>,
    #[value(default)]
    pub engagement_preview_operation_json: Option<String>,
    #[value(default, deserialize_with = "deserialize_cad_preview_generation")]
    pub engagement_preview_generation: i32,
    #[value(default)]
    pub last_finalized_interaction_id: Option<String>,
    #[value(default)]
    pub sun: WorldSunConfig,
    /// 🎥️ Per-pane camera pose — session-only view state (never a VCS-tracked document field): see
    /// `"setCamera"`/`"setProjection"`/`"setProjectionParam"` in `handle_action` below.
    #[value(default)]
    pub camera: CadCamera,
    #[value(default)]
    pub camera_building: CadCamera,
    #[value(default)]
    pub camera_energy: CadCamera,
    #[value(default)]
    pub camera_structure_classic: CadCamera,
    #[value(default)]
    pub dislocate_options_by_window_id: HashMap<String, CadDislocateOptions>,
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
        }
    }
}

impl CadPlayRuntime {
    /// 🪟️ Reads the Dislocate handle configuration for one window instance without sharing it with siblings.
    pub fn dislocate_options(&self, window_id: &str) -> CadDislocateOptions {
        self.dislocate_options_by_window_id.get(window_id).copied().unwrap_or_default()
    }
}

/// 🔁️ Encodes any `ToValue` type to its JSON-text wire form via `protocol::json` — the
/// `engagement_session_json`/`engagement_preview_operation_json` persisted-string fields need real
/// JSON text (not a `DslValue`, which never touches the wire directly).
fn json_string_of(value: &impl protocol::ToValue) -> String {
    json::to_json_string(value)
}

/// 🔁️ The `json_string_of` inverse: parses JSON text straight into `T` via `protocol::json`.
/// `None` on either a JSON syntax error or a shape mismatch — callers already treat a
/// missing/invalid persisted session as "no session".
fn json_string_to<T: protocol::FromValue>(json: &str) -> Option<T> {
    json::from_json_str::<T>(json).ok()
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
        engagement_session: cfg.engagement_session_json.as_deref().and_then(json_string_to),
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
        engagement_session_json: runtime.engagement_session.as_ref().map(json_string_of),
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

pub fn cad_action(action: &str, args: Option<UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<UiValue>)> {
    semio_framework_plugin::ActionFactory::new(CAD_PLAY_CONTROLLER_ID).action(action, args)
}

/// 🪟️ Bridges window chrome, which still carries the retained WGPU action descriptor.
pub fn cad_window_action(action: &str, args: Option<protocol::DslValue>) -> ActionDescriptor {
    ActionDescriptor { controller_id: CAD_PLAY_CONTROLLER_ID.into(), action: action.into(), args }
}

/// 🧱️ Admits one fixed CAD UI text value.
pub fn ui_value_text(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    UiText::try_from_str(value.as_ref()).map(UiValue::Text).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "cad UI text admission failed"))
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
pub fn ui_node_list(values: impl IntoIterator<Item = semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode>>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
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
pub fn cad_tree_item(id: impl Into<String>, label: impl AsRef<str>, icon_id: Option<&str>, action: (semio_framework_plugin::ActionId, Option<UiValue>)) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
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
        config.engagement_preview_generation = base.engagement_preview_generation.checked_add(1).ok_or_else(|| Fault::from("cad.preview.conflict: engagement preview generation exhausted"))?;
        config.engagement_preview_operation_json = Some(json_string_of(operation));
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
        protocol::DslValue::String(text) => text,
        other => json::to_json_string(&other),
    };
    Effect::DownloadMediaExport { filename: export.filename, mime_type: export.mime_type, data, encoding: export.encoding }
}

/// @emoji ⬇️ Wraps a spatial-JSON export document into a download host effect.
pub fn cad_spatial_export_effect(value: &protocol::DslValue, filename: &str) -> Effect {
    Effect::DownloadMediaExport { filename: filename.into(), mime_type: "text/plain".into(), data: json::to_json_string(value), encoding: None }
}

/// ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: exporting per-pane objects as
/// spatial JSON used to read `CadSnapshot`'s inline `objects` field directly. That data now lives
/// inside composed `s.stdio.semio.model` CHILD documents (unresolved at this boundary — see
/// `🔖️Composition` in `🏪️store/🦀️.rs`). Returns an empty `objects` array per pane;
/// documented reduced-fidelity gap, not silently wrong.
pub fn export_spatial_json(envelope: &CadPlayView, mode: &str) -> protocol::DslValue {
    let object = |entries: Vec<(&str, protocol::DslValue)>| protocol::DslValue::object(entries.into_iter().map(|(key, value)| (key.to_string(), value)));
    let text = |value: &str| protocol::DslValue::String(value.to_string());
    let empty_model = || object(vec![("schema", text("spatial.model")), ("revision", protocol::DslValue::uint(1)), ("objects", protocol::DslValue::Array(Vec::new()))]);
    let models: Vec<protocol::DslValue> = CadPaneId::all().into_iter().map(|pane| object(vec![("id", text(pane.model_definition_id())), ("model", empty_model())])).collect();
    match mode {
        "selected" => {
            let pane = cad_pane_from_model_definition_id(&envelope.document.active_model_definition_id).unwrap_or(CadPaneId::Shape);
            let model = empty_model();
            let model_space =
                object(vec![("schema", text("spatial.modelspace")), ("revision", protocol::DslValue::uint(1)), ("models", protocol::DslValue::Array(vec![object(vec![("id", text(pane.model_definition_id())), ("model", model.clone())])]))]);
            object(vec![("model", model), ("modelSpace", model_space), ("activeModelDefinitionId", text(pane.model_definition_id()))])
        }
        "current" => {
            let pane = cad_pane_from_model_definition_id(&envelope.document.active_model_definition_id).unwrap_or(CadPaneId::Shape);
            object(vec![("schema", text("spatial.model")), ("revision", protocol::DslValue::uint(1)), ("modelDefinitionId", text(pane.model_definition_id())), ("objects", protocol::DslValue::Array(Vec::new()))])
        }
        _ => object(vec![("schema", text("spatial.modelspace")), ("revision", protocol::DslValue::uint(1)), ("activeModelDefinitionId", text(&envelope.document.active_model_definition_id)), ("models", protocol::DslValue::Array(models))]),
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
pub fn object_field_mutation(_pane: CadPaneId, _object_id: &str, _field: &str, _value: Option<&protocol::DslValue>) -> Option<CadMutation> {
    None
}

pub fn resolve_number_edit(current: f64, value: Option<&protocol::DslValue>, delta: Option<&protocol::DslValue>) -> Option<f64> {
    if let Some(absolute) = value.and_then(protocol::DslValue::as_f64) {
        return Some(absolute);
    }
    delta.and_then(protocol::DslValue::as_f64).map(|delta| current + delta)
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
pub fn patch_objects_mutations(_document: &CadSnapshot, _object_ids: &[String], _field: &str, _value: Option<&protocol::DslValue>, _delta: Option<&protocol::DslValue>) -> Vec<CadMutation> {
    Vec::new()
}

pub(crate) fn make_object_for_typology(typology: &str, label_count: usize, pane: CadPaneId) -> crate::standards::v1::subsets::any::io::geometry_import::CadObject {
    use crate::standards::v1::subsets::any::io::geometry_import::CadObject;
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
/// `DslValue` shape `object_patch_from_field`/`resolve_number_edit` already expect, dispatching
/// on the same field-name vocabulary those helpers use (bool fields by name, everything else tried as a
/// number first, falling back to a string).
pub fn command_value_json(field: &str, value: &str) -> protocol::DslValue {
    match field {
        "hidden" | "locked" => value.parse::<bool>().map_or(protocol::DslValue::Null, protocol::DslValue::Bool),
        _ => value.parse::<f64>().map_or_else(|_| protocol::DslValue::String(value.into()), protocol::DslValue::float),
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
    pub view_state: Option<ViewModel>,
}

/// 🪪️ Collision-free public-operation identity attached to every persisted preview generation.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct CadPreviewOperationIdentity {
    pub app_instance_id: u32,
    pub parent_document_id: String,
    pub operation_id: u64,
    pub operation_generation: u64,
    pub canonical_base_revision: String,
}

impl From<&AppOperationContext> for CadPreviewOperationIdentity {
    fn from(operation: &AppOperationContext) -> Self {
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
fn cad_command_from_action(action: &str, args: Option<&protocol::DslValue>) -> Result<CadCommand, Fault> {
    let str_field = |key: &str| args.and_then(|value| value.get(key)).and_then(protocol::DslValue::as_str).map(str::to_string);
    let f64_field = |key: &str| args.and_then(|value| value.get(key)).and_then(protocol::DslValue::as_f64);
    let bool_field = |key: &str| args.and_then(|value| value.get(key)).and_then(protocol::DslValue::as_bool);
    let str_vec_field = |key: &str| -> Vec<String> { args.and_then(|value| value.get(key)).and_then(|value| protocol::FromValue::from_value(value.clone()).ok()).unwrap_or_default() };
    let value_string = || -> Option<String> {
        args.and_then(|value| value.get("value")).and_then(|value| match value {
            protocol::DslValue::String(text) => Some(text.clone()),
            protocol::DslValue::Bool(flag) => Some(flag.to_string()),
            protocol::DslValue::Number(number) => Some(match number {
                protocol::Number::UInt(number) => number.to_string(),
                protocol::Number::Int(number) => number.to_string(),
                protocol::Number::Float(number) => number.to_string(),
            }),
            _ => None,
        })
    };
    let position_axis = |index: usize| args.and_then(|value| value.get("position")).and_then(protocol::DslValue::as_array).and_then(|array| array.get(index)).and_then(protocol::DslValue::as_f64);
    Ok(match action {
        "setActiveExample" => CadCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: str_field("exampleId").unwrap_or_default() }),
        "setDislocateOption" => CadCommand::SetDislocateOption(set_dislocate_option::SetDislocateOption { pane: str_field("pane"), option: str_field("option").unwrap_or_default(), pressed: bool_field("pressed") }),
        "setNodeSelection" => CadCommand::SetNodeSelection(set_node_selection::SetNodeSelection { node_ids: str_vec_field("nodeIds") }),
        "setCamera" => CadCommand::SetCamera(set_camera::SetCamera { pane: str_field("surfaceId"), camera: args.and_then(|value| value.get("camera")).and_then(|value| protocol::FromValue::from_value(value.clone()).ok()).unwrap_or_default() }),
        "setProjection" => CadCommand::SetProjection(set_projection::SetProjection {
            pane: str_field("surfaceId"),
            field: str_field("field"),
            value_str: args.and_then(|value| value.get("value")).and_then(protocol::DslValue::as_str).map(String::from),
            value_num: args.and_then(|value| value.get("value")).and_then(protocol::DslValue::as_f64),
            param: str_field("param"),
        }),
        "setProjectionParam" => CadCommand::SetProjectionParam(set_projection_param::SetProjectionParam {
            pane: str_field("surfaceId"),
            field: str_field("field"),
            value_str: args.and_then(|value| value.get("value")).and_then(protocol::DslValue::as_str).map(String::from),
            value_num: args.and_then(|value| value.get("value")).and_then(protocol::DslValue::as_f64),
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
                Some(protocol::DslValue::String(text)) => text,
                Some(other) => json::to_json_string(&other),
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
        let operation = json_string_to(config.engagement_preview_operation_json.as_ref()?)?;
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
    context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<CadPlayApp>>>,
    operation: &AppOperationContext,
) -> Result<Emit<CadMutation, CadConfigMutation, NoDraftMutation>, Fault> {
    let doc = ArtifactView::with_operation(snapshot, history, operation.clone());
    let cfg = ConfigView { snapshot: config, window: None };
    let selection = interaction.selection.get(CAD_INTERACTION_DOMAIN).cloned().unwrap_or_default();
    let retained_interaction = CadInteractionSnapshot { granularity: selection.granularity.clone(), ids: selection.ids.clone(), anchor_id: selection.anchor_id };
    let mut ctx = CadDispatchCtx { interaction: retained_interaction, preview_operation: Some(CadPreviewOperationIdentity::from(operation)), view_state: context.and_then(|context| context.view_state.clone()) };
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

impl ToolJobFactory for CadRetainedCommandJobFactory {
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

impl ArtifactOwnedToolJobFactory for CadRetainedCommandJobFactory {
    type Owner = EditorApp<CadPlayApp>;
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

fn cad_projection_retained_bytes(projection: &crate::CadProjectionDsl) -> usize {
    projection
        .kind
        .len()
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
    config
        .selected_node_ids
        .iter()
        .fold(0usize, |bytes, value| bytes.saturating_add(value.len()))
        .saturating_add(option_bytes)
        .saturating_add(config.engagement_input.len())
        .saturating_add(config.engagement_step.len())
        .saturating_add(config.sun.color.len())
        .saturating_add(cad_camera_retained_bytes(&config.camera))
        .saturating_add(cad_camera_retained_bytes(&config.camera_building))
        .saturating_add(cad_camera_retained_bytes(&config.camera_energy))
        .saturating_add(cad_camera_retained_bytes(&config.camera_structure_classic))
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

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<CadConfig, CadConfigMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<CadConfig, CadConfigMutation>>, store::ArtifactStoreOneItemPreparationRequest<CadConfig, CadConfigMutation>> {
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

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<CadConfig, CadConfigMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<CadConfig, CadConfigMutation>> {
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
        references.iter().fold(bytes.saturating_add(model_definition_id.len()), |bytes, reference| bytes.saturating_add(reference.id.len()).saturating_add(reference.source_url.len()).saturating_add(reference.media_kind.len()))
    });
    let node_bytes = snapshot.nodes.iter().fold(0usize, |bytes, node| bytes.saturating_add(node.id.len()).saturating_add(node.label.len()).saturating_add(node.kind.len()));
    snapshot.schema.len().saturating_add(snapshot.id.len()).saturating_add(snapshot.active_model_definition_id.len()).saturating_add(fixed_children).saturating_add(drawing_bytes).saturating_add(reference_bytes).saturating_add(node_bytes)
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
    let retained_bytes = json::to_json_string(mutation).len();
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

    fn begin(
        &self,
        request: store::ArtifactStoreOneItemPreparationRequest<CadSnapshot, CadMutation>,
    ) -> Result<Box<dyn store::ArtifactStoreOneItemPreparation<CadSnapshot, CadMutation>>, store::ArtifactStoreOneItemPreparationRequest<CadSnapshot, CadMutation>> {
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

    fn checkpoint(&self) -> store::ArtifactStoreOneItemCheckpoint {
        self.checkpoint
    }

    fn prepared(&self) -> Option<&store::ArtifactStoreOneItemPrepared<CadSnapshot, CadMutation>> {
        self.prepared.as_ref()
    }

    fn take_prepared(&mut self) -> Option<store::ArtifactStoreOneItemPrepared<CadSnapshot, CadMutation>> {
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
        assert_eq!(size_of::<semio_framework_plugin::NoTransient>(), 0);
        Ok(semio_framework_plugin::PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self, _owner: &store::TransientStore<semio_framework_plugin::NoTransient, semio_framework_plugin::NoTransientMutation>) -> bool {
        size_of::<semio_framework_plugin::NoTransient>() == 0
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
        assert_eq!(size_of::<NoDraft>(), 0);
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

    const DIALECT: Dialect = crate::CAD_DIALECT;
    const DOCUMENT_SCHEMA: &'static str = CAD_DOCUMENT_SCHEMA;

    fn build_artifact_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(std::sync::Arc::new(CadArtifactStorePreparationFactory))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<std::sync::Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(std::sync::Arc::new(CadConfigStorePreparationFactory))
    }

    semio_framework_plugin::bounded_first_step_tool_proofs! {
        owner: EditorApp<CadPlayApp>,
        owner_file: "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        controller: "s.cad.cad@1/*#editor",
        document_schema: "cad.scene",
        factory: "CadRetainedCommandJobFactory",
        factory_type: CadRetainedCommandJobFactory,
        contract: ToolExecutionContract::bounded_first_step(8_192, 64, 1, 16_384, 7_500),
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
        let payload = ArtifactRetainedCommandPayload::try_new(
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
            CadCommand::command_id,
            CAD_RETAINED_RAW_BYTES,
            CAD_RETAINED_WORK_ITEMS,
            work,
        )?;
        Ok(Some(semio_framework::ToolOperationSpec::new(request.controller_id, request.tool_id, request.payload_schema_id, payload, request.operation)))
    }

    fn app_schema() -> Option<::framework_schema::AppSchemaDescriptor> {
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
            MediaPayload::Structured { json, .. } => protocol::DslValue::String(json.clone()),
            MediaPayload::Binary { .. } => return Err(MediaError::Payload(port.to_string(), "geometry:in only accepts a Structured payload today".into())),
        };
        // ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: `import_cad_object_by_extension`
        // now returns a `SemioModelElement` (composed-child shape); `create-object` is retired.
        // Composing the imported element into the Shape pane's `SemioModelSnapshot` CHILD needs a
        // child-dispatch seam on `Emit<CadMutation, _>` that does not exist yet
        // (`🔌️plugin/🦀️.rs` framework-kernel surface, W1-owned). Documented no-op.
        match crate::standards::v1::subsets::any::io::import_cad_object_by_extension(name, &payload) {
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
            protocol::DslValue::String(text) => text,
            other => json::to_json_string(&other),
        };
        Ok(Media { media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep }, payload: MediaPayload::Structured { schema: "3d.cad".into(), json: base64_codec::base64_standard_encode(text.as_bytes()) } })
    }

    fn command_id(command: &CadCommand) -> &'static str {
        command.command_id()
    }

    fn command_from_action(action: &str, args: Option<&protocol::DslValue>) -> Result<CadCommand, Fault> {
        cad_command_from_action(action, args)
    }

    fn host_configuration_mutation(action: &str, args: Option<&protocol::DslValue>) -> Result<Option<Self::ConfigMutation>, Fault> {
        Ok((action == "setContributions").then(|| CadConfigMutation::SetContributions { json: args.and_then(|value| value.get("json")).and_then(protocol::DslValue::as_str).unwrap_or("[]").to_string() }))
    }

    fn handle(
        command: &CadCommand,
        doc: &ArtifactView<'_, CadSnapshot>,
        cfg: &ConfigView<'_, CadConfig>,
        interaction: &semio_framework_plugin::app::InteractionView<'_>,
        view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, Self::Draft>,
        _engines: &EngineHandles,
    ) -> Result<Emit<CadMutation, CadConfigMutation, Self::DraftMutation>, Fault> {
        let selection = interaction.selection(CAD_INTERACTION_DOMAIN);
        let snapshot = CadInteractionSnapshot { granularity: selection.granularity.clone(), ids: selection.ids.clone(), anchor_id: selection.anchor_id.clone() };
        let mut ctx = CadDispatchCtx { interaction: snapshot, preview_operation: Some(CadPreviewOperationIdentity::from(doc.operation()?)), view_state: view_state.cloned() };
        command.dispatch(doc, cfg, &mut ctx)
    }

    fn render(body_key: &str, doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, view_state: &ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {
        crate::standards::v1::subsets::any::schema::inferences::validate_cad_computer_contributions(&cfg.snapshot.contributions_json);
        let view = CadPlayView { document: doc.snapshot.clone(), runtime: cad_runtime_from_config(cfg.snapshot) };
        let labels = cad_labels(view_state);
        let window_kind_id = match body_key {
            shape::BODY_KEY => shape::WINDOW_KIND_ID,
            building::BODY_KEY => building::WINDOW_KIND_ID,
            energy::BODY_KEY => energy::WINDOW_KIND_ID,
            structure_classic::BODY_KEY => structure_classic::WINDOW_KIND_ID,
            _ => shape::WINDOW_KIND_ID,
        };
        let active_utility = view_state.active_utility_id.as_deref();
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

    fn window_engagements(doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, view_state: &ViewModel) -> HashMap<String, WindowEngagement> {
        let view = CadPlayView { document: doc.snapshot.clone(), runtime: cad_runtime_from_config(cfg.snapshot) };
        let labels = cad_labels(view_state);
        HashMap::from([
            (shape::WINDOW_KIND_ID.to_string(), shape::engagement(&view, labels)),
            (building::WINDOW_KIND_ID.to_string(), building::engagement(&view, labels)),
            (energy::WINDOW_KIND_ID.to_string(), energy::engagement(&view, labels)),
            (structure_classic::WINDOW_KIND_ID.to_string(), structure_classic::engagement(&view, labels)),
        ])
    }

    /// 🪟️ Keyed by the 4 fixed window-KIND ids; each window collects its own measures from the edit
    /// mode's `☑️options/*` components.
    fn window_measures(_doc: &ArtifactView<'_, CadSnapshot>, cfg: &ConfigView<'_, CadConfig>, view_state: &ViewModel) -> HashMap<String, Vec<WindowMeasure>> {
        let runtime = cad_runtime_from_config(cfg.snapshot);
        let is_de = cad_is_de_locale(view_state);
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
    fn context_menu(_request: &ContextMenuRequest, _doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _view_state: &ViewModel, registry: &AppActionRegistry) -> Vec<ContextMenuItemSpec> {
        Menu::of(registry).action("translateSelection").action("rotateSelection").action("scaleSelection").action("duplicateObject").destructive("deleteObject").build()
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
/// `crate::standards::v1::subsets::any::io::InteractionSpec` (a CAD-artifact DSL
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
    Editor::builder(crate::CAD_DIALECT).document(["semio", "cad"])
            .command({
                let mut definition = CommandDefinition { in_palette: false, ..CommandDefinition::bounded_catalog("setContributions", LocalizedLabel::native("Set Contributions", "Beiträge festlegen"), "host", ActionKind::View).with_args([ActionArgDef::text("json", LocalizedLabel::native("Contributions", "Beiträge"))]) };
                definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated;
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
            .action_with(ActionDefinition::bounded_catalog("deleteObject", LocalizedLabel::native("Delete Object", "Objekt löschen"), ActionKind::Mutation).category("actions"))
            .action_with(ActionDefinition::bounded_catalog("duplicateObject", LocalizedLabel::native("Duplicate Object", "Objekt duplizieren"), ActionKind::Mutation).category("create"))
            .mutation("addNode", LocalizedLabel::native("Add Node", "Knoten hinzufügen"))
            .mutation("renameNode", LocalizedLabel::native("Rename Node", "Knoten umbenennen"))
            .action_with(ActionDefinition::bounded_catalog("translateSelection", LocalizedLabel::native("Translate Selection", "Auswahl verschieben"), ActionKind::Mutation).category("transform"))
            .action_with(ActionDefinition::bounded_catalog("rotateSelection", LocalizedLabel::native("Rotate Selection", "Auswahl drehen"), ActionKind::Mutation).category("transform"))
            .action_with(ActionDefinition::bounded_catalog("scaleSelection", LocalizedLabel::native("Scale Selection", "Auswahl skalieren"), ActionKind::Mutation).category("transform"))
            .mutation("applyTransformation", LocalizedLabel::native("Apply Transformation", "Transformation anwenden"))
            .mutation("importCadFile", LocalizedLabel::native("Import CAD File", "CAD-Datei importieren"))
            .action_with(ActionDefinition::bounded_catalog("patchCadPlayReference", LocalizedLabel::native("Patch Reference", "Referenz aktualisieren"), ActionKind::Mutation).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("engagementSubmit", LocalizedLabel::native("Engagement Submit", "Eingabe bestätigen"), ActionKind::Mutation).in_palette(false))
            .action_with(ActionDefinition::new("setCamera", LocalizedLabel::native("Set Camera", "Kamera festlegen"), ActionKind::View, "camera"))
            .action_with(ActionDefinition::new("setProjection", LocalizedLabel::native("Set Projection", "Projektion festlegen"), ActionKind::View, "scan"))
            .action_with(ActionDefinition::new("setProjectionParam", LocalizedLabel::native("Set Projection Parameter", "Projektionsparameter festlegen"), ActionKind::View, "scan"))
            .mutation("focusModelDefinition", LocalizedLabel::native("Focus Model Definition", "Modelldefinition fokussieren"))
            .action_with(ActionDefinition::new("setActiveExample", LocalizedLabel::native("Set Active Example", "Aktives Beispiel festlegen"), ActionKind::Mutation, "panel-left"))
            .action_with(ActionDefinition::bounded_catalog("setNodeSelection", LocalizedLabel::native("Set Node Selection", "Knotenauswahl festlegen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("setReferenceSelection", LocalizedLabel::native("Set Reference Selection", "Referenzauswahl festlegen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("referenceHover", LocalizedLabel::native("Reference Hover", "Überfahren (Referenz)"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new("engagementInput", LocalizedLabel::native("Engagement Input", "Eingabe"), ActionKind::View, "hand").in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("engagementPossibleSelect", LocalizedLabel::native("Engagement Possible Select", "Eingabeoption auswählen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("engagementRepeatLast", LocalizedLabel::native("Engagement Repeat Last", "Letzte Eingabe wiederholen"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new("engagementAbort", LocalizedLabel::native("Engagement Abort", "Eingabe abbrechen"), ActionKind::View, "hand").in_palette(false))
            .action_with(ActionDefinition::new("worldPointerDown", LocalizedLabel::native("World Pointer Down", "Welt-Zeiger gedrückt"), ActionKind::View, "mouse-pointer").in_palette(false))
            .action_with(ActionDefinition::bounded_catalog("worldPointerMove", LocalizedLabel::native("World Pointer Move", "Welt-Zeiger bewegt"), ActionKind::View).in_palette(false))
            .action_with(ActionDefinition::new("toggleSun", LocalizedLabel::native("Toggle Sun", "Sonne umschalten"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunAzimuth", LocalizedLabel::native("Set Sun Azimuth", "Sonnenazimut festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunElevation", LocalizedLabel::native("Set Sun Elevation", "Sonnenhöhe festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::new("setSunIntensity", LocalizedLabel::native("Set Sun Intensity", "Sonnenintensität festlegen"), ActionKind::View, "sun"))
            .action_with(ActionDefinition::bounded_catalog("setDislocateOption", LocalizedLabel::native("Set Dislocate Option", "Versetzen-Option festlegen"), ActionKind::View).in_palette(false))
            .shell_action("saveSelected", LocalizedLabel::native("Save Selected", "Auswahl speichern"))
            .shell_action("saveInPlay", LocalizedLabel::native("Save In Play", "Im Play speichern"))
            .shell_action("saveCurrent", LocalizedLabel::native("Save Current", "Aktuelles speichern"))
            .shell_action("loadRawRequest", LocalizedLabel::native("Load Raw Request", "Rohdaten laden"))
            .action_args("saveCurrent", vec![ActionArgDef::select("format", LocalizedLabel::native("Format", "Format"), vec![
                ActionArgOption::new("step", LocalizedLabel::native("STEP", "STEP")),
                ActionArgOption::new("obj", LocalizedLabel::native("OBJ", "OBJ")),
                ActionArgOption::new("stl", LocalizedLabel::native("STL", "STL")),
            ]).default_value(&"step")])
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
            .action_interactive_job("addObject", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("patchObject", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("patchSelection", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("deleteObject", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("duplicateObject", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("addNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("renameNode", InteractiveJobClassification::Migrated)
            .action_interactive_job("translateSelection", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("rotateSelection", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("scaleSelection", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("applyTransformation", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("importCadFile", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("patchCadPlayReference", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementSubmit", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("focusModelDefinition", InteractiveJobClassification::Migrated)
            .action_interactive_job("setActiveExample", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("worldPointerDown", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("setCamera", InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setProjectionParam", InteractiveJobClassification::Migrated)
            .action_interactive_job("setDislocateOption", InteractiveJobClassification::Migrated)
            .action_interactive_job("setNodeSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("setReferenceSelection", InteractiveJobClassification::Migrated)
            .action_interactive_job("referenceHover", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementInput", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementPossibleSelect", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementRepeatLast", InteractiveJobClassification::Migrated)
            .action_interactive_job("engagementAbort", InteractiveJobClassification::Migrated)
            .action_interactive_job("worldPointerMove", InteractiveJobClassification::Migrated)
            .action_interactive_job("toggleSun", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunAzimuth", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunElevation", InteractiveJobClassification::Migrated)
            .action_interactive_job("setSunIntensity", InteractiveJobClassification::Migrated)
            .action_interactive_job("saveSelected", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("saveInPlay", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("saveCurrent", InteractiveJobClassification::BatchOnlyPendingRewrite)
            .action_interactive_job("loadRawRequest", InteractiveJobClassification::Migrated)
            // 🚧️ SDK GAP (contract §2.4): `EditorBuilder`/`Viewer`/`.editor::<E>(def: AppDefinition)`
            // take a bare `AppDefinition`, not the old `App { definition, examples }` — there is no
            // `.example(...)`/`.workflow(...)` on this builder, so the old
            // `CAD_EXAMPLE_FOREST_LEFT` app-level example registration and the no-op `.workflow("cad",
            // …)` call are dropped here (not silently: reported in the packet's migration report).
            // The subset's own `📚️examples/🎬️demo` facet (`crate::examples::...`,
            // real content, pre-existing) is the modern, role-agnostic replacement surface for this.
            .build_definition()
}
//#endregion 🔖️Manifest

//#region 🔖️WorkingSceneFixtures
/// 🌲️ The Concrete Forest Left example's REAL per-pane object content, built straight from the
/// same fixture JSON `forest_play_scene()`'s (persisted, handle-only) `CadSnapshot` is built from —
/// see `crate::standards::v1::subsets::any::schema::inferences::forest_pane_bundle`.
/// This is the app-layer `CadWorkingScene` counterpart to `forest_play_scene()`: use `forest_play_scene()`
/// for `drive`/render dispatch (a `CadSnapshot`, composed-child HANDLES only) and this for reading
/// actual object data in tests/render-path exemplars.
pub fn forest_working_scene() -> CadWorkingScene {
    use crate::standards::v1::subsets::any::schema::inferences::forest_pane_bundle;
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
#[path = "🧪️tests/🔬️testkit/🦀️.rs"]
pub(crate) mod testkit;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
