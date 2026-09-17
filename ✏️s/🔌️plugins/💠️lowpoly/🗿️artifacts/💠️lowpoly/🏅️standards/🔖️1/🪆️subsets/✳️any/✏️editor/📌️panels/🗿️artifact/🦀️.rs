//! 📄️ Lowpoly play app panel — the document tree: mesh objects and, per object, its vertex/edge/face
//! component groups.
//!
//! 🪟️ All THREE nesting levels are **virtualised**, not built eagerly. The `meshes` section windows the
//! objects, each object row windows its three component groups, and each group row windows one leaf per
//! raw mesh element id. A real lowpoly mesh has hundreds of vertices per object; building one row each
//! used to overflow the fixed child list and fail the whole panel with `ui.fixed-capacity`. Only the
//! entry ids are materialised eagerly (a `Vec<u32>` per group) — the row closure runs once per row the
//! host's window actually asks for. Picks cost zero argument arena: the tree carries ONE
//! `interactionSelect` binding and each row names only its granularity.

use crate::editor::lowpoly::engine::LowpolyDocument;
use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::{document_object_row_id, document_target_row_id, resolve_active_object_id, LowpolyView, MESH_GRANULARITY_OBJECT, MESH_INTERACTION_DOMAIN};
use crate::editor::lowpoly::{lowpoly_action, ui_label, ui_value_list, ui_value_map, ui_value_number, LOWPOLY_PLAY_CONTROLLER_ID};
use crate::LowpolyObject;
use semio_framework_plugin::plugin_app_close_prelude::{ActionBinding, Buildable, BuiltNode, HasBase, RowAction, RowActionPlacement, Trigger};
use semio_framework_plugin::{
    tree_window_item, LabelText, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const LOWPOLY_PLAY_BODY_ARTIFACT: &str = "lowpoly.play.artifact";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(LOWPOLY_PLAY_BODY_ARTIFACT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Rows
fn admission(reason: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", reason)
}

fn ui_key(value: &str, reason: &'static str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| admission(reason))
}

/// 🕹️ One pick row of the "mesh" domain: keyed by the RAW target id the domain itself uses
/// (`document_object_row_id`/`document_target_row_id`) and carrying only its granularity — the
/// activation a click needs is the ONE tree-level `interactionSelect` binding
/// `PanelTreeBuilder::interaction_domain` stamps, so the per-row `mesh_select_action` argument map
/// (a domain id, a merge and a JSON target list, per row) is gone.
fn pick_item(id: &str, label: impl AsRef<str>, icon: &str, granularity: &str) -> UiAssemblyResult<ui::TreeItemBuilder> {
    Ok(ui::tree_item(ui_label(label)?)
        .try_id(id)
        .map_err(|_| admission("lowpoly row id admission failed"))?
        .icon(ui_key(icon, "lowpoly row icon admission failed")?)
        .granularity(ui_key(granularity, "lowpoly row granularity admission failed")?))
}

/// 🧩️ One object's component group: the mesh granularity it picks at, and the raw element ids a window
/// indexes into. Only the ids are materialised here — a row is built per element the host asks for.
struct ComponentGroup {
    mode: &'static str,
    label: LabelText,
    icon: &'static str,
    ids: Vec<u32>,
}

fn component_groups(object: &LowpolyObject, doc: &LowpolyDocument, labels: &LowpolyLabels) -> [ComponentGroup; 3] {
    let mesh = doc.object_index(&object.id).ok().and_then(|index| doc.mesh_at(index));
    let ids = |count: usize| (0..count as u32).collect::<Vec<u32>>();
    [
        ComponentGroup { mode: "vertex", label: labels.vertices, icon: "circle", ids: ids(mesh.map_or(0, |entry| entry.vertex_count())) },
        ComponentGroup { mode: "edge", label: labels.edges, icon: "minus", ids: ids(mesh.map_or(0, |entry| entry.edge_count())) },
        ComponentGroup { mode: "face", label: labels.faces, icon: "square", ids: ids(mesh.map_or(0, |entry| entry.face_count())) },
    ]
}

/// 🔁️ A face row keeps its own `flipFaces` menu action — a row action is not a pick, so it survives
/// the domain migration untouched.
fn with_flip_normal(item: ui::TreeItemBuilder, id: u32, labels: &LowpolyLabels) -> UiAssemblyResult<ui::TreeItemBuilder> {
    let face_ids = ui_value_list([ui_value_number(f64::from(id))])?;
    let (action, args) = lowpoly_action("flipFaces", Some(ui_value_map([("faceIds", face_ids)])?))?;
    item.try_row_action(RowAction {
        icon: ui_key("flip-vertical", "lowpoly face action icon admission failed")?,
        label: Some(ui_label(labels.flip_normal.as_str())?),
        action: ActionBinding { trigger: Trigger::Activate, action, args, capability: None },
        placement: RowActionPlacement::Menu,
    })
    .map_err(|_| admission("lowpoly face row action admission failed"))
}

fn element_row(object_id: &str, group: &ComponentGroup, id: u32, labels: &LowpolyLabels) -> UiAssemblyResult<BuiltNode> {
    let row_id = document_target_row_id(object_id, group.mode, id);
    let item = pick_item(&row_id, format!("{} {id}", group.label.as_str()), group.icon, group.mode)?;
    let item = if group.mode == "face" { with_flip_normal(item, id, labels)? } else { item };
    item.try_build().map_err(|_| admission("lowpoly component row admission failed"))
}

fn group_row(object_id: &str, group: &ComponentGroup, labels: &LowpolyLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let id = format!("lowpoly-document.{object_id}.{}.group", group.mode);
    let item = ui::tree_item(ui_label(group.label.as_str())?)
        .try_id(&id)
        .map_err(|_| admission("lowpoly component group id admission failed"))?
        .icon(ui_key(group.icon, "lowpoly component group icon admission failed")?)
        .description(UiText::try_from_string(group.ids.len().to_string()).map_err(|_| admission("lowpoly component count admission failed"))?);
    tree_window_item(windows, item, &id, false, &group.ids, |element| element_row(object_id, group, *element, labels))
}

fn object_row(object: &LowpolyObject, doc: &LowpolyDocument, active_id: &str, labels: &LowpolyLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let row_id = document_object_row_id(&object.id);
    let item = pick_item(&row_id, object.name.clone(), "box", MESH_GRANULARITY_OBJECT)?.description(ui_key(&object.id, "lowpoly object description admission failed")?);
    let groups = component_groups(object, doc, labels);
    tree_window_item(windows, item, &row_id, object.id == active_id, &groups, |group| group_row(&object.id, group, labels, windows))
}
//#endregion 🔖️Rows

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `.interaction_domain` binds this tree
/// to the "mesh" domain — the framework OVERWRITES every row's `presence.selected`/`.hovered` from the
/// live `InteractionState` right after render, so this app never calls `.selected()?`/`.highlighted()?`
/// again (dead code the wrapper would silently discard anyway).
pub fn render(view: LowpolyView<'_>, doc: &LowpolyDocument, labels: &LowpolyLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let active_id = resolve_active_object_id(view.snapshot, view.config);
    PanelTreeBuilder::new("lowpoly-play-document")?
        .window_section(windows, "lowpoly-play-document.meshes", Some(ui_label(labels.meshes.as_str())?), true, &view.snapshot.objects, |object| object_row(object, doc, &active_id, labels, windows))?
        .interaction_domain(LOWPOLY_PLAY_CONTROLLER_ID, MESH_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
