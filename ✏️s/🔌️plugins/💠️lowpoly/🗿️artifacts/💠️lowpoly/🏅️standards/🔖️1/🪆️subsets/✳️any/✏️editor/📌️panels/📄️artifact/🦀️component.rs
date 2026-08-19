//! 📄️ Lowpoly play app panel — the document tree: mesh objects and, per object, its vertex/edge/face
//! component groups.

use crate::editor::lowpoly::lowpoly_action;
use crate::editor::lowpoly::terminology::LowpolyLabels;
use crate::editor::lowpoly::view::{document_object_row_id, document_target_row_id, mesh_select_action, resolve_active_object_id, MESH_INTERACTION_DOMAIN, MESH_GRANULARITY_OBJECT, LowpolyView};
use crate::editor::lowpoly::engine::LowpolyDocument;
use semio_framework_plugin::{
    IconName, Label, LabelText, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiTreeActionPlacement, UiTreeItemAction, UiTreeItemNode, UiNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const LOWPOLY_PLAY_BODY_DOCUMENT: &str = "lowpoly.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
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
pub async fn render(view: LowpolyView<'_>, doc: &LowpolyDocument, labels: &LowpolyLabels) -> UiNode {
    let active_id = resolve_active_object_id(view.snapshot, view.config);
    let items: Vec<UiTreeItemNode> = view
        .snapshot
        .objects
        .iter()
        .enumerate()
        .map(|(object_index, object)| {
            let object_id = object.id.clone();
            let mesh = doc.object_index(&object.id).ok().and_then(|index| doc.mesh_at(index));
            let vertex_count = mesh.as_ref().map_or(0, |entry| entry.vertex_count());
            let edge_count = mesh.as_ref().map_or(0, |entry| entry.edge_count());
            let face_count = mesh.as_ref().map_or(0, |entry| entry.face_count());
            let component_group = |mode: &str, label: LabelText, icon: &str, count: usize| {
                let leaves: Vec<UiTreeItemNode> = (0..count)
                    .map(|id| {
                        let row_id = document_target_row_id(&object.id, object_index, mode, id as u32);
                        let mut actions = None;
                        if mode == "face" {
                            actions = Some(vec![UiTreeItemAction {
                                icon_id: "flip-vertical".into(),
                                label: Some(labels.flip_normal.into()),
                                action: lowpoly_action("flipFaces", Some(json!({ "faceIds": [id] }))),
                                placement: Some(UiTreeActionPlacement::Menu),
                            }]);
                        }
                        UiTreeItemNode {
                            icon_id: IconName::from_str(icon),
                            action: Some(mesh_select_action(mode, &row_id, "invertive")),
                            actions,
                            menu: None,
                            ..UiTreeItemNode::base(row_id.clone(), Label::data(format!("{} {id}", label.as_str())))
                        }
                    })
                    .collect();
                UiTreeItemNode { icon_id: IconName::from_str(icon), items: Some(leaves), description: Some(format!("{count}")), menu: None, ..UiTreeItemNode::base(format!("lowpoly-document.{object_id}.{mode}.group"), label) }
            };
            let object_row_id = document_object_row_id(&object.id);
            UiTreeItemNode {
                icon_id: Some("box".into()),
                action: Some(mesh_select_action(MESH_GRANULARITY_OBJECT, &object_row_id, "invertive")),
                items: Some(vec![component_group("vertex", labels.vertices, "circle", vertex_count), component_group("edge", labels.edges, "minus", edge_count), component_group("face", labels.faces, "square", face_count)]),
                default_open: Some(object.id == active_id),
                description: Some(object.id.clone()),
                menu: None,
                ..UiTreeItemNode::base(object_row_id, Label::data(object.name.clone()))
            }
        })
        .collect();
    // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: `.interaction_domain` binds this tree
    // to the "mesh" domain — the framework OVERWRITES every row's `presence.selected`/`.hovered` from the
    // live `InteractionState` right after render, so this app never calls `.selected()`/`.highlighted()`
    // again (dead code the wrapper would silently discard anyway).
    PanelTreeBuilder::new("lowpoly-play-document").section("lowpoly-play-document.meshes", Some(labels.meshes.into()), true, items).interaction_domain(MESH_INTERACTION_DOMAIN).build()
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
        let mut a = crate::editor::lowpoly::testkit::app();
        assert!(render(&mut a, super::LOWPOLY_PLAY_BODY_DOCUMENT).contains("lowpoly-document."));
    }
}
//#endregion 🧪️Tests
