//! 📄️ Lowpoly play app panel — the document tree: mesh objects and, per object, its vertex/edge/face
//! component groups.

use crate::editor::lowpoly::engine::LowpolyDocument;
use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::{document_object_row_id, document_target_row_id, mesh_select_action, resolve_active_object_id, LowpolyView, MESH_GRANULARITY_OBJECT, MESH_INTERACTION_DOMAIN};
use crate::editor::lowpoly::{lowpoly_action, ui_label, ui_value_list, ui_value_map, ui_value_number};
use semio_framework_plugin::plugin_app_close_prelude::{ActionBinding, Buildable, BuiltNode, HasBase, HasChildren, RowAction, RowActionPlacement, Trigger};
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const LOWPOLY_PLAY_BODY_DOCUMENT: &str = "lowpoly.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(LOWPOLY_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(view: LowpolyView<'_>, doc: &LowpolyDocument, labels: &LowpolyLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let active_id = resolve_active_object_id(view.snapshot, view.config);
    let mut items = UiFixedList::<BuiltNode>::default();
    for (object_index, object) in view.snapshot.objects.iter().enumerate() {
        let mesh = doc.object_index(&object.id).ok().and_then(|index| doc.mesh_at(index));
        let counts = [
            ("vertex", labels.vertices, "circle", mesh.as_ref().map_or(0, |entry| entry.vertex_count())),
            ("edge", labels.edges, "minus", mesh.as_ref().map_or(0, |entry| entry.edge_count())),
            ("face", labels.faces, "square", mesh.as_ref().map_or(0, |entry| entry.face_count())),
        ];
        let mut groups = UiFixedList::<BuiltNode>::default();
        for (mode, label, icon, count) in counts {
            let mut leaves = UiFixedList::<BuiltNode>::default();
            for id in 0..count {
                let row_id = document_target_row_id(&object.id, object_index, mode, id as u32);
                let (action, args) = mesh_select_action(mode, &row_id, "invertive")?;
                let mut item = ui::tree_item(ui_label(format!("{} {id}", label.as_str()))?)
                    .try_id(&row_id)
                    .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly component row id admission failed"))?
                    .icon(UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly component icon admission failed"))?);
                item = match args {
                    Some(args) => item.try_on_with(Trigger::Activate, action, args),
                    None => item.try_on(Trigger::Activate, action),
                }
                .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly component action admission failed"))?;
                if mode == "face" {
                    let face_ids = ui_value_list([ui_value_number(id as f64)])?;
                    let (action, args) = lowpoly_action("flipFaces", Some(ui_value_map([("faceIds", face_ids)])?))?;
                    item = item
                        .try_row_action(RowAction {
                            icon: UiText::try_from_str("flip-vertical").ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly face action icon admission failed"))?,
                            label: Some(ui_label(labels.flip_normal.as_str())?),
                            action: ActionBinding { trigger: Trigger::Activate, action, args, capability: None },
                            placement: RowActionPlacement::Menu,
                        })
                        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly face row action admission failed"))?;
                }
                let item = item.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly component row admission failed"))?;
                leaves.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly component list admission failed"))?;
            }
            let group = ui::tree_item(ui_label(label.as_str())?)
                .try_id(format!("lowpoly-document.{}.{mode}.group", object.id))
                .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly component group id admission failed"))?
                .icon(UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly component group icon admission failed"))?)
                .description(UiText::try_from_string(count.to_string()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly component count admission failed"))?)
                .try_children(leaves)
                .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly component group children admission failed"))?
                .try_build()
                .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly component group admission failed"))?;
            groups.try_push(group).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly component group list admission failed"))?;
        }
        let object_row_id = document_object_row_id(&object.id);
        let (action, args) = mesh_select_action(MESH_GRANULARITY_OBJECT, &object_row_id, "invertive")?;
        let mut item = ui::tree_item(ui_label(object.name.clone())?)
            .try_id(&object_row_id)
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly object row id admission failed"))?
            .icon(UiText::try_from_str("box").ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly object icon admission failed"))?)
            .default_open(object.id == active_id)
            .description(UiText::try_from_str(&object.id).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly object description admission failed"))?)
            .try_children(groups)
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly object children admission failed"))?;
        item = match args {
            Some(args) => item.try_on_with(Trigger::Activate, action, args),
            None => item.try_on(Trigger::Activate, action),
        }
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly object action admission failed"))?;
        let item = item.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly object row admission failed"))?;
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly object list admission failed"))?;
    }
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `.interaction_domain` binds this tree
    // to the "mesh" domain — the framework OVERWRITES every row's `presence.selected`/`.hovered` from the
    // live `InteractionState` right after render, so this app never calls `.selected()?`/`.highlighted()?`
    // again (dead code the wrapper would silently discard anyway).
    PanelTreeBuilder::new("lowpoly-play-document")?.section("lowpoly-play-document.meshes", Some(ui_label(labels.meshes.as_str())?), true, items)?.interaction_domain(MESH_INTERACTION_DOMAIN)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::lowpoly::testkit::render;

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_document_tab_to_this_body_key() {
        use semio_framework_plugin::PanelTabDefinition;
        let definition: PanelTabDefinition = super::definition();
        assert_eq!(definition.id(), semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(super::LOWPOLY_PLAY_BODY_DOCUMENT));
    }

    #[semio_framework_async_macros::async_test]
    async fn document_tree_lists_active_object() {
        let mut a = crate::editor::lowpoly::testkit::app().await;
        assert!(render(&mut a, super::LOWPOLY_PLAY_BODY_DOCUMENT).await.contains("lowpoly-document."));
    }
}
//#endregion 🧪️Tests
