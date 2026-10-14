//! 🧰️ CAD editor kit: the play runtime and view, the dispatch context, the shared object/engagement helpers and the
//! ui-value plumbing every command, mode and panel builds on.

extern crate semio_framework_os_kernel as store;

//#region 🧩️Modules
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod cad {
        #[path = "../🗣️terminology/🦀️.rs"]
        pub mod terminology;
    }
}
//#endregion 🧩️Modules

use semio_s_artifact_cad_cad_model::{snapshot::CadCamera, snapshot::CadPaneId, snapshot::CadSnapshot, geometry::CAD_OBJECT_PSET, geometry::CadObject, geometry::cad_object_from_model_element, geometry::model_element_from_cad_object, geometry::objects_from_model_snapshot};
use semio_s_artifact_cad_cad_play::CadComposedPanes;
use semio_s_artifact_cad_cad_config::{CadConfig, CadConfigMutation, CadDislocateOptions, SetEdit, cad_sun_config_to_world};
use semio_s_artifact_cad_cad_interaction::{self, CadEngagementScratch, apply_event, can_commit, keyed_transitions, resolve_interaction_key, start_session};
use semio_s_artifact_cad_cad_transform::{CadPaneModels, CadToolEntry, CadToolLeaf, CadTransformRecord};
use semio_s_artifact_cad_cad_transient::CadWorldWindowTransient;
use semio_s_artifact_cad_cad_io::{CadSolidExport, export_solids_as};
use semio_s_artifact_cad_cad_inferences::{CAD_MODEL_DEFINITION_BUILDING, CAD_MODEL_DEFINITION_ENERGY, CAD_MODEL_DEFINITION_SHAPE, CAD_MODEL_DEFINITION_STRUCTURE_CLASSIC, cad_brep_kernel, cad_camera_projection_config, ensure_object_solid_handle, next_cad_id};
use semio_s_artifact_stdio_semio_model::{schema::mutations::SemioModelMutation, schema::mutations::remove_element::RemoveElement, schema::mutations::set_element::SetElement, schema::snapshot::SemioModelElement};
use semio_framework::kernel::Effect;
use semio_framework_plugin::{ActionArgDef, ActionDefinition, ActionDescriptor, ActionKind, ChildContentView, ConfigView, Fault, MediaClass, MediaForm, MediaType, PluginAssemblyError, UiText, UiValue, ViewModel, WorldSunConfig, tree_item as framework_tree_item, tree_item_with_action, world3d_camera_projection_json};
use semio_framework_ui_locale::{Label, LocalizedLabel};
use semio_framework_3d_brep_engine::{Brep, GeometryHandle};
use semio_framework_value_derive::{FromValue, ToValue};
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
/// 🕹️ `"cad"` — the single FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM interaction domain this app
/// declares (granularities object/vertex/edge/face, `HierarchyProvider::Flat`).
pub const CAD_INTERACTION_DOMAIN: &str = "cad";
/// 🕹️ Owned snapshot of the framework's `"cad"` domain — `InteractionView::selection` plus the
/// `"pointer"` hover — read once per dispatch by `ArtifactApp::handle` (threaded through
/// `CadDispatchCtx` to every command handler) and once per render by
/// `ArtifactApp::render_with_request_context` (threaded through [`CadPlayView`] to the world scene,
/// the engagement HUD, the inspection panel and the document tree). Decouples both from
/// `semio_framework_plugin::app::InteractionView` itself — whose fields are `pub(crate)` to that
/// crate, so this crate's own tests cannot construct one — tests build this plain, cad-owned struct
/// directly instead (see `🎮️commands/🔄️transform`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CadInteractionSnapshot {
    pub granularity: String,
    pub ids: Vec<String>,
    pub anchor_id: Option<String>,
    pub hovered_ids: Vec<String>,
}
impl CadInteractionSnapshot {
    /// 🐁️ The hover channel `World3dHost` publishes instance hover on (`world3dHoverActionArgs`).
    pub const POINTER_CHANNEL: &'static str = "pointer";

    pub fn from_interaction(interaction: &semio_framework_plugin::app::InteractionView<'_>) -> Self {
        let selection = interaction.selection(CAD_INTERACTION_DOMAIN);
        Self {
            granularity: selection.granularity.clone(),
            ids: selection.ids.clone(),
            anchor_id: selection.anchor_id.clone(),
            hovered_ids: interaction.hover(CAD_INTERACTION_DOMAIN, Self::POINTER_CHANNEL).ids.clone(),
        }
    }

    pub fn is_selected(&self, id: &str) -> bool {
        self.ids.iter().any(|selected| selected == id)
    }

    pub fn is_hovered(&self, id: &str) -> bool {
        self.hovered_ids.iter().any(|hovered| hovered == id)
    }
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
/// 🔁️ Encodes any `ToValue` type to its JSON-text wire form via `protocol::json` — the window transient's
/// `engagement_session_json` carries real JSON text (not a `DslValue`, which never touches the wire directly).
fn json_string_of(value: &impl semio_framework_value::ToValue) -> String {
    semio_framework_pack_json::to_json_string(value)
}
/// 🔁️ The `json_string_of` inverse: parses JSON text straight into `T` via `protocol::json`.
/// `None` on either a JSON syntax error or a shape mismatch — callers already treat a
/// missing/invalid persisted session as "no session".
fn json_string_to<T: semio_framework_value::FromValue>(json: &str) -> Option<T> {
    semio_framework_pack_json::from_json_str::<T>(json, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()
}
/// 🔀️ Unpacks artifact-wide `CadConfig` and the addressed window's engagement transient (design §17.4) into the
/// ergonomic runtime scratch record. Exact window-owned camera, sun, and Dislocate state is overlaid by `runtime_of`.
pub fn cad_runtime_from(cfg: &CadConfig, transient: &CadWorldWindowTransient) -> CadPlayRuntime {
    CadPlayRuntime {
        selected_node_ids: cfg.selected_node_ids.clone(),
        hovered_reference_id: cfg.hovered_reference_id.clone(),
        engagement_input: transient.engagement_input.clone(),
        engagement_step: transient.engagement_step.clone(),
        active_example_id: cfg.active_example_id.clone(),
        selected_reference_model_definition_id: cfg.selected_reference_model_definition_id.clone(),
        selected_reference_id: cfg.selected_reference_id.clone(),
        engagement_pane: transient.engagement_pane.clone(),
        engagement_session: transient.engagement_session_json.as_deref().and_then(json_string_to),
        last_finalized_interaction_id: transient.last_finalized_interaction_id.clone(),
        sun: WorldSunConfig::default(),
        camera: CadCamera::default(),
        camera_building: CadCamera::default(),
        camera_energy: CadCamera::default(),
        camera_structure_classic: CadCamera::default(),
        dislocate_options_by_window_id: HashMap::new(),
    }
}
/// 🔀️ The `cad_runtime_from` boundary's outbound twin: repacks the (possibly mutated)
/// `CadPlayRuntime` scratch struct back into a real `CadConfig` snapshot. Kept private so production
/// command modules cannot bypass the checked snapshot authorities below.
fn cad_config_from_runtime(runtime: &CadPlayRuntime, base: &CadConfig) -> CadConfig {
    CadConfig {
        contributions_json: base.contributions_json.clone(),
        selected_node_ids: runtime.selected_node_ids.clone(),
        hovered_reference_id: runtime.hovered_reference_id.clone(),
        active_example_id: runtime.active_example_id.clone(),
        selected_reference_model_definition_id: runtime.selected_reference_model_definition_id.clone(),
        selected_reference_id: runtime.selected_reference_id.clone(),
    }
}
/// 🫧️ The engagement state of the (possibly mutated) runtime, as the addressed window's transient (design §17.4).
pub fn cad_transient_from_runtime(runtime: &CadPlayRuntime) -> CadWorldWindowTransient {
    CadWorldWindowTransient {
        engagement_input: runtime.engagement_input.clone(),
        engagement_step: runtime.engagement_step.clone(),
        engagement_pane: runtime.engagement_pane.clone(),
        engagement_session_json: runtime.engagement_session.as_ref().map(json_string_of),
        last_finalized_interaction_id: runtime.last_finalized_interaction_id.clone(),
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
/// 🎛️ Ephemeral read/render view assembled per call from the store's `CadSnapshot` projection, the panes its composed
/// model children hold now ([`CadComposedPanes`], design §20.15) and the app's `CadPlayRuntime` view-state. Its history
/// lives in the wrapping `VcsArtifactApp`'s stores, its runtime view-state in config and the window transient.
pub struct CadPlayView {
    pub document: CadSnapshot,
    pub(crate) panes: CadComposedPanes,
    pub runtime: CadPlayRuntime,
    /// 🕹️ The live `"cad"` domain at this render — empty on the interaction-less `render` path
    /// (exports, tests) and on every `handle`-side view.
    pub interaction: CadInteractionSnapshot,
}
impl CadPlayView {
    /// 🎛️ The view over `document` composed with its live `children`.
    pub fn of(document: &CadSnapshot, children: &ChildContentView, runtime: CadPlayRuntime, interaction: CadInteractionSnapshot) -> Self {
        Self { document: document.clone(), panes: CadComposedPanes::compose(document, children), runtime, interaction }
    }
}
pub fn cad_action(action: &str, args: Option<UiValue>) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<UiValue>)> {
    semio_framework_plugin::ActionFactory::new(CAD_PLAY_CONTROLLER_ID).action(action, args)
}
/// 🪟️ Bridges window chrome, which still carries the retained WGPU action descriptor.
pub fn cad_window_action(action: &str, args: Option<semio_framework_value::DslValue>) -> ActionDescriptor {
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
/// 🗺️ Admits one fixed CAD UI map value. 🔑️ `UiMapBuilder::push` admits keys in strictly ascending
/// order only — a later key that does not exceed the previous one is refused — so callers list
/// entries sorted by key.
pub fn ui_value_map(values: impl IntoIterator<Item = (&'static str, UiValue)>) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    let mut builder = semio_framework_plugin::UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "cad UI map admission failed"))?;
    for (key, value) in values {
        builder.push(key.to_owned(), value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "cad UI map entry admission failed"))?;
    }
    Ok(UiValue::Map(builder.finish()))
}
/// 🏷️ Admits resolved CAD text into the semantic UI contract.
pub fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    semio_framework_ui_contract::Label::try_from(value.as_ref()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "cad UI label admission failed"))
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
/// 🙈️ A reference set-verb's definition: the `{modelDefinitionId, referenceId}` a row names and the REQUIRED boolean `flag`
/// it sets — a missing value is refused, never defaulted.
pub fn cad_reference_flag_action(id: &str, flag: &str, label: LocalizedLabel, value: LocalizedLabel) -> ActionDefinition {
    ActionDefinition::bounded_catalog(id, label, ActionKind::Mutation)
        .with_args([
            ActionArgDef::text("modelDefinitionId", LocalizedLabel::native("Model Definition", "Modelldefinition")).required(),
            ActionArgDef::text("referenceId", LocalizedLabel::native("Reference", "Referenz")).required(),
            ActionArgDef::toggle(flag, value).required(),
        ])
        .in_palette(false)
}
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
/// 🌳️ Builds a presentational CAD tree item without inventing a plugin focus command.
pub fn cad_tree_item_static(id: impl Into<String>, label: impl AsRef<str>, icon_id: Option<&str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut item = framework_tree_item(id.into(), ui_label(label)?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = match icon_id {
            Some(value) => Some(UiText::try_from_str(value).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "cad tree icon admission failed"))?),
            None => None,
        };
    }
    Ok(item)
}



/// 🔀️ The artifact-wide config snapshot of the (possibly mutated) runtime.
pub fn snapshot_of(runtime: &CadPlayRuntime, base: &CadConfig) -> Result<CadConfigMutation, Fault> {
    Ok(CadConfigMutation::Set(SetEdit { config: Box::new(cad_config_from_runtime(runtime, base)) }))
}
/// 🫧️ Hands the runtime's engagement state to the addressed window's transient when it changed (design §17.4): an
/// action line, a REPL step, a pointer move or a possible-select is per-frame window state — never config, never history.
pub fn publish_engagement(runtime: &CadPlayRuntime, ctx: &mut CadDispatchCtx) {
    let next = cad_transient_from_runtime(runtime);
    if next != ctx.window_transient {
        ctx.next_window_transient = Some(next);
    }
}
//#endregion 🔖️Runtime
//#region 🔖️Helpers
/// 🧱️ `pane`'s objects read losslessly out of its composed model child — what every object writer decides against; a
/// pane without a composed model has none.
pub(crate) fn cad_model_objects(models: &CadPaneModels, pane: CadPaneId) -> Vec<CadObject> {
    models.model(pane).map(objects_from_model_snapshot).unwrap_or_default()
}
/// 🆕️ The entry that lands `object` in `pane`'s model child: one `insert-element` the transform tool yields.
pub fn create_object_entry(pane: CadPaneId, object: &CadObject) -> CadToolEntry {
    CadToolEntry::Create { pane, element: model_element_from_cad_object(object) }
}
/// 📄️ `source` under a fresh id and a `copy` label, offset by `offset` so the copy is visibly its own instance.
pub(crate) fn cad_object_copy(source: &CadObject, offset: [f64; 3]) -> CadObject {
    CadObject { id: next_cad_id("object"), label: format!("{} copy", source.label), origin: [source.origin[0] + offset[0], source.origin[1] + offset[1], source.origin[2] + offset[2]], ..source.clone() }
}
/// ❌️ `deleteObject`'s leaf: one `remove-element` on the pane child holding `object_id` — an id no pane holds produces
/// nothing rather than a fabricated target.
pub fn delete_object_leaves(models: &CadPaneModels, object_id: &str) -> Vec<CadToolLeaf> {
    models.pane_of(object_id).map_or_else(Vec::new, |pane| vec![CadToolLeaf { pane, leaf: SemioModelMutation::RemoveElement(RemoveElement { id: object_id.to_string() }) }])
}
/// 📄️ `duplicateObject`'s entry: the source object re-inserted in its own pane as [`cad_object_copy`].
pub fn duplicate_object_entries(models: &CadPaneModels, object_id: &str) -> Vec<CadToolEntry> {
    let Some(pane) = models.pane_of(object_id) else {
        return Vec::new();
    };
    cad_model_objects(models, pane).iter().find(|object| object.id == object_id).map_or_else(Vec::new, |source| vec![create_object_entry(pane, &cad_object_copy(source, [CAD_DUPLICATE_OFFSET, 0.0, 0.0]))])
}
/// 📄️ World-space offset a duplicate is placed at so it never hides inside its source.
pub const CAD_DUPLICATE_OFFSET: f64 = 1.0;
/// 🔁️ Re-deriving building/energy/structure typologies from shape geometry is expressible as `insert-element` entries
/// on the target pane's model child, but the derivation RULES (which typology each source solid becomes, how openings and
/// storeys map) are not written anywhere yet — a modelling decision, not a missing seam. Yields nothing until they are.
pub fn apply_transformation_entries(_models: &CadPaneModels, _qid: &str) -> Vec<CadToolEntry> {
    Vec::new()
}
/// 📦️ The pane's visible objects as world-space kernel solids, read from the composed pane ([`CadComposedPanes`]) and
/// built as the viewport builds them ([`crate::standards::v1::subsets::any::schema::inferences::pane_world_solids`]).
pub fn collect_pane_solids(kernel: &mut Brep, envelope: &CadPlayView, pane: CadPaneId) -> Vec<GeometryHandle> {
    let composed = envelope.panes.pane(pane);
    semio_s_artifact_cad_cad_inferences::pane_world_solids(kernel, &composed.objects, composed.geometry(pane))
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
/// ⬇️ Converts a staged native-geometry export into a download host effect emitted directly
/// to the shell (no document mutation, no pending-export runtime slot).
pub fn cad_solid_export_effect(export: CadSolidExport) -> Effect {
    let data = match export.data {
        semio_framework_value::DslValue::String(text) => text,
        other => semio_framework_pack_json::to_json_string(&other),
    };
    Effect::DownloadMediaExport { filename: export.filename, mime_type: export.mime_type, data, encoding: export.encoding }
}
/// ⬇️ Wraps a spatial-JSON export document into a download host effect.
pub fn cad_spatial_export_effect(value: &semio_framework_value::DslValue, filename: &str) -> Effect {
    Effect::DownloadMediaExport { filename: filename.into(), mime_type: "text/plain".into(), data: semio_framework_pack_json::to_json_string(value), encoding: None }
}
/// ⚠️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3: exporting per-pane objects as
/// spatial JSON used to read `CadSnapshot`'s inline `objects` field directly. That data now lives
/// inside composed `s.stdio.semio.model` CHILD documents (unresolved at this boundary — see
/// `🔖️Composition` in `🏪️store/🦀️.rs`). Returns an empty `objects` array per pane;
/// documented reduced-fidelity gap, not silently wrong.
pub fn export_spatial_json(_envelope: &CadPlayView, mode: &str, pane: Option<CadPaneId>) -> Result<semio_framework_value::DslValue, Fault> {
    let object = |entries: Vec<(&str, semio_framework_value::DslValue)>| semio_framework_value::DslValue::object(entries.into_iter().map(|(key, value)| (key.to_string(), value)));
    let text = |value: &str| semio_framework_value::DslValue::String(value.to_string());
    let empty_model = || object(vec![("schema", text("spatial.model")), ("revision", semio_framework_value::DslValue::uint(1)), ("objects", semio_framework_value::DslValue::Array(Vec::new()))]);
    let models: Vec<semio_framework_value::DslValue> = CadPaneId::all().into_iter().map(|pane| object(vec![("id", text(pane.model_definition_id())), ("model", empty_model())])).collect();
    Ok(match mode {
        "selected" => {
            let pane = pane.ok_or_else(|| Fault::from("cad.window.invalid: selected export requires an addressed CAD pane"))?;
            let model = empty_model();
            let model_space =
                object(vec![("schema", text("spatial.modelspace")), ("revision", semio_framework_value::DslValue::uint(1)), ("models", semio_framework_value::DslValue::Array(vec![object(vec![("id", text(pane.model_definition_id())), ("model", model.clone())])]))]);
            object(vec![("model", model), ("modelSpace", model_space)])
        }
        "current" => {
            let pane = pane.ok_or_else(|| Fault::from("cad.window.invalid: current export requires an addressed CAD pane"))?;
            object(vec![("schema", text("spatial.model")), ("revision", semio_framework_value::DslValue::uint(1)), ("modelDefinitionId", text(pane.model_definition_id())), ("objects", semio_framework_value::DslValue::Array(Vec::new()))])
        }
        _ => object(vec![("schema", text("spatial.modelspace")), ("revision", semio_framework_value::DslValue::uint(1)), ("models", semio_framework_value::DslValue::Array(models))]),
    })
}
/// 🌱️ Builds a `Effect::LoadDocument` that swaps the live document to `scene` OUTSIDE history —
/// the sanctioned non-mutation path for a whole-document replace (file import, load-example). Per
/// `📓️taxonomy.md`, whole-document replace has NO mutation-enum representative (`SetSnapshot` is
/// banned outright); every former "replace the whole document" gesture builds this effect instead
/// of an `Emit::mutations([...])`. The spr is a fresh, edit-free op-log for `scene`'s own
/// `schema`/`id` — a genesis envelope with no history to encode.
pub fn reset_document_effect(scene: &CadSnapshot) -> Effect {
    let pack = <CadSnapshot as pack::snapshot::ArtifactPack>::encode_pack(scene);
    // 🪪️ An edit-free history log, never a throwaway `create_document_envelope`: an envelope dropped
    // without its bounded retirement authority traps (`terminal shell reached Drop …`). The host
    // re-stamps the log's identity with the live store's before archive hydration.
    let spr = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::empty_document_spr(&scene.id, &scene.schema));
    Effect::LoadDocument { pack, spr }
}
pub fn resolve_number_edit(current: f64, value: Option<&semio_framework_value::DslValue>, delta: Option<&semio_framework_value::DslValue>) -> Option<f64> {
    if let Some(absolute) = value.and_then(semio_framework_value::DslValue::as_f64) {
        return Some(absolute);
    }
    delta.and_then(semio_framework_value::DslValue::as_f64).map(|delta| current + delta)
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
/// 🎛️ The `set-element` that brings `element` to `next`'s authoring state: only the facets that differ — the class for a
/// typology, the placement for an origin/orientation/scale, the [`CAD_OBJECT_PSET`] property set (replaced in place,
/// every foreign set kept) for a label/visibility/lock — `None` when nothing differs.
pub(crate) fn cad_set_element_leaf(element: &SemioModelElement, next: &CadObject) -> Option<SemioModelMutation> {
    let current = cad_object_from_model_element(element);
    let target = model_element_from_cad_object(next);
    let placed = next.origin != current.origin || next.orientation != current.orientation || next.scale != current.scale;
    let authored = placed || next.label != current.label || next.visible != current.visible || next.locked != current.locked;
    let psets = authored.then(|| {
        let set = target.psets.iter().find(|set| set.name == CAD_OBJECT_PSET).cloned();
        let mut psets = element.psets.clone();
        match (psets.iter_mut().find(|candidate| candidate.name == CAD_OBJECT_PSET), set) {
            (Some(slot), Some(set)) => *slot = set,
            (None, Some(set)) => psets.push(set),
            (_, None) => {}
        }
        psets
    });
    let leaf = SetElement { id: element.id.clone(), class: (next.typology != current.typology).then_some(target.class), placement: placed.then_some(target.placement), geometry: None, spatial_id: None, psets: psets.filter(|psets| *psets != element.psets) };
    (leaf.class.is_some() || leaf.placement.is_some() || leaf.psets.is_some()).then_some(SemioModelMutation::SetElement(leaf))
}
/// 🎯️ The child leaves that apply `field`'s edit across `object_ids`: one `set-element` per object that changes, on the
/// pane child holding it. Pose fields (`origin.<axis>`, `scale.<axis>`, `orientation.<axis>`) read each object's own
/// current component, so `value` (absolute) or `delta` (relative) applies per object and keeps the other axes across a
/// multi-select; whole-value fields (`label`, `typology`, `visible`/`hidden`, `locked`) replace that facet. An inspector
/// origin DELTA never reaches here: it is the transform tool's relative `drag-elements`.
pub fn patch_objects_leaves(models: &CadPaneModels, object_ids: &[String], field: &str, value: Option<&semio_framework_value::DslValue>, delta: Option<&semio_framework_value::DslValue>) -> Vec<CadToolLeaf> {
    let patched = |object: &CadObject| -> Option<CadObject> {
        let mut next = object.clone();
        if let Some(axis) = axis3_index(field, "origin") {
            next.origin[axis] = resolve_number_edit(object.origin[axis], value, delta)?;
        } else if let Some(axis) = axis3_index(field, "scale") {
            let mut scale = object.scale.unwrap_or([1.0, 1.0, 1.0]);
            scale[axis] = resolve_number_edit(scale[axis], value, delta)?;
            next.scale = Some(scale);
        } else if let Some(axis) = axis4_index(field, "orientation") {
            let mut orientation = object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]);
            orientation[axis] = resolve_number_edit(orientation[axis], value, delta)?;
            next.orientation = Some(quat_normalize(orientation));
        } else {
            match field {
                "label" => next.label = value.and_then(semio_framework_value::DslValue::as_str)?.to_string(),
                "typology" => next.typology = value.and_then(semio_framework_value::DslValue::as_str)?.to_string(),
                "visible" => next.visible = value.and_then(semio_framework_value::DslValue::as_bool)?,
                "hidden" => next.visible = !value.and_then(semio_framework_value::DslValue::as_bool)?,
                "locked" => next.locked = value.and_then(semio_framework_value::DslValue::as_bool)?,
                _ => return None,
            }
        }
        Some(next)
    };
    models
        .0
        .iter()
        .flat_map(|(pane, _, model)| {
            model.elements.iter().filter(|element| object_ids.contains(&element.id)).filter_map(move |element| {
                let next = patched(&cad_object_from_model_element(element))?;
                Some(CadToolLeaf { pane: *pane, leaf: cad_set_element_leaf(element, &next)? })
            })
        })
        .collect()
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
    let mut kernel = cad_brep_kernel();
    ensure_object_solid_handle(&mut kernel, &mut object);
    let _ = pane;
    object
}
/// 🧭️ Commits `session` if it satisfies `can_commit`: clears the session runtime state and answers what the
/// transform tool yields for it on the panes' composed `models` — a construction's created elements, a copy's duplicated
/// objects, or the relative transform of `transform.move`/`rotate`/`scale*` — as ONE tool request (empty when no commit
/// happened). Used by both the direct-event and keyed-transition REPL paths in `engagement_submit_entries` (a state
/// reached via either path can be commit-ready, e.g. box's explicit `confirm` step reachable via a keyed transition).
pub fn try_commit_session_entries(models: &CadPaneModels, runtime: &mut CadPlayRuntime, pane: CadPaneId, session: &CadEngagementScratch) -> Vec<CadToolEntry> {
    if !can_commit(session) {
        return Vec::new();
    }
    let mut kernel = cad_brep_kernel();
    let label_count = models.model(session.pane).map_or(0, |model| model.elements.len());
    let outcome = interaction::commit_session(&mut kernel, session, label_count, next_cad_id);
    let interaction_id = session.interaction_id.clone();
    let pane = if session.pane == pane { pane } else { session.pane };
    runtime.engagement_input.clear();
    runtime.last_finalized_interaction_id = Some(interaction_id.clone());
    runtime.engagement_session = None;
    let (entries, step): (Vec<CadToolEntry>, String) = match outcome {
        Some(interaction::CommitOutcome::Objects(objects)) => {
            let count = objects.len();
            (objects.iter().map(|object| create_object_entry(pane, object)).collect(), format!("Committed {count} object(s)"))
        }
        Some(interaction::CommitOutcome::Move { targets, delta }) => (vec![CadToolEntry::Transform(CadTransformRecord::drag(targets, delta))], "Moved".to_string()),
        Some(interaction::CommitOutcome::Copy { targets, delta }) => {
            let entries = targets
                .iter()
                .filter_map(|target| {
                    let source_pane = models.pane_of(target)?;
                    let source = cad_model_objects(models, source_pane).into_iter().find(|object| &object.id == target)?;
                    Some(create_object_entry(source_pane, &cad_object_copy(&source, delta)))
                })
                .collect();
            (entries, "Copied".to_string())
        }
        Some(interaction::CommitOutcome::Rotate { targets, axis, angle }) => (vec![CadToolEntry::Transform(CadTransformRecord::rotate(targets, axis, angle))], "Rotated".to_string()),
        Some(interaction::CommitOutcome::Scale { targets, factors }) => (vec![CadToolEntry::Transform(CadTransformRecord::scale(targets, factors))], "Scaled".to_string()),
        Some(interaction::CommitOutcome::Unsupported(action)) => (Vec::new(), format!("Unsupported: {action}")),
        None => (Vec::new(), "Nothing to commit".to_string()),
    };
    runtime.engagement_step = if entries.is_empty() && step.starts_with("Committed") { "Nothing to commit".into() } else { step };
    entries
}
/// ⌨️ Advances the engagement REPL for the current `engagement_input`, mutating runtime
/// session state and returning the transform-tool entries its commit yields on `models` (empty without one).
pub fn engagement_submit_entries(models: &CadPaneModels, runtime: &mut CadPlayRuntime, pane: CadPaneId) -> Vec<CadToolEntry> {
    let input = runtime.engagement_input.trim().to_string();
    if input.is_empty() {
        // ⏎️ An empty line during a session is the shell's Enter/Space: it is the state's own
        // `confirm` when the spec declares one ("Accept height", "Finalize"), else a no-op that keeps
        // the session.
        if let Some(session) = runtime.engagement_session.as_mut() {
            if apply_event(session, "confirm", None) {
                runtime.engagement_step = session.state.clone();
                let session_snapshot = session.clone();
                return try_commit_session_entries(models, runtime, pane, &session_snapshot);
            }
            runtime.engagement_step = session.state.clone();
            return Vec::new();
        }
        runtime.engagement_step = "Idle".into();
        return Vec::new();
    }
    let model_definition_id = pane.model_definition_id();
    if let Some((event_kind, payload)) = interaction::parse_repl_line_for(&input, runtime.engagement_session.as_ref()) {
        // An active session's own events/keyed-transitions always take priority over starting an
        // unrelated interaction by key — otherwise a mid-flow keypress that happens to collide
        // with another interaction's top-level key (e.g. box's "d" for diagonal mode vs. length's
        // top-level key "d") would silently abandon the current session.
        if let Some(session) = runtime.engagement_session.as_mut() {
            if apply_event(session, &event_kind, payload.as_ref()) {
                runtime.engagement_step = session.state.clone();
                // 🧹️ The line was consumed: a scalar entry that keeps its state ("2" in the height
                // prompt) must not linger as the published line, or the shell's next Enter re-submits it.
                runtime.engagement_input.clear();
                let session_snapshot = session.clone();
                return try_commit_session_entries(models, runtime, pane, &session_snapshot);
            }
            for transition in keyed_transitions(session) {
                if (transition.key.eq_ignore_ascii_case(&input) || transition.event_kind.eq_ignore_ascii_case(&input)) && apply_event(session, &transition.event_kind, None) {
                    runtime.engagement_step = session.state.clone();
                    runtime.engagement_input.clear();
                    let session_snapshot = session.clone();
                    return try_commit_session_entries(models, runtime, pane, &session_snapshot);
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
/// 🔀️ WORKFLOWS-END-TO-END-TYPED-PORTS: the typed-command counterpart of the pre-B1
/// `mesh_selection_ids` (JSON-args) helper — falls back to the current selection when the command
/// carries no explicit ids.
pub fn ids_or_selection(ids: &[String], fallback: &[String]) -> Vec<String> {
    if ids.is_empty() {
        fallback.to_vec()
    } else {
        ids.to_vec()
    }
}
/// 🩹️ Typed-command counterpart of a raw JSON patch value: `CadCommand::PatchObject`/
/// `PatchSelection`/`PatchCadPlayReference` all carry `value: Option<String>` (the typed channel has no
/// single Rust type spanning "maybe a string, maybe a number, maybe a bool") — this recovers the
/// `DslValue` shape `object_patch_from_field`/`resolve_number_edit` already expect, dispatching
/// on the same field-name vocabulary those helpers use (bool fields by name, everything else tried as a
/// number first, falling back to a string).
pub fn command_value_json(field: &str, value: &str) -> semio_framework_value::DslValue {
    match field {
        "hidden" | "locked" => value.parse::<bool>().map_or(semio_framework_value::DslValue::Null, semio_framework_value::DslValue::Bool),
        _ => value.parse::<f64>().map_or_else(|_| semio_framework_value::DslValue::String(value.into()), semio_framework_value::DslValue::float),
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
        artifact_schema: "cad.scene".into(),
        artifact_media_type: MediaType { class: MediaClass::ThreeD, form: MediaForm::Brep },
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
/// 🧵️ Per-dispatch app-struct state: a read-only [`CadInteractionSnapshot`] of the framework's `"cad"` domain — the
/// `semio_framework_plugin::app_commands!`-generated `dispatch` has no way to thread `InteractionView` itself (see that
/// macro's own doc comment on `ctx`), so the dispatch builds the snapshot once and hands it down through this app-owned
/// context — the addressed view, and the addressed window's engagement transient in (`window_transient`) and out
/// (`next_window_transient`, design §17.4).
pub struct CadDispatchCtx {
    pub interaction: CadInteractionSnapshot,
    pub view_state: Option<ViewModel>,
    pub window_transient: CadWorldWindowTransient,
    pub next_window_transient: Option<CadWorldWindowTransient>,
}
impl CadDispatchCtx {
    /// 🧵️ A dispatch over `interaction` from `view_state`, whose window holds `window_transient`.
    pub fn new(interaction: CadInteractionSnapshot, view_state: Option<ViewModel>, window_transient: CadWorldWindowTransient) -> Self {
        Self { interaction, view_state, window_transient, next_window_transient: None }
    }
}
//#endregion 🔖️Commands
