//! 📄️ Lowpoly play app panel — the document tree: mesh objects and, per object, its vertex/edge/face
//! component groups.

use crate::apps::lowpoly::lowpoly_action;
use crate::apps::lowpoly::terminology::LowpolyLabels;
use crate::apps::lowpoly::view::{document_target_row_id, highlighted_document_ids, resolve_active_object_id, selected_document_ids, LowpolyView};
use crate::artifacts::lowpoly::engine::LowpolyDocument;
use semio_framework_plugin::{
    IconName, Label, LabelText, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiTreeActionPlacement, UiTreeItemAction, UiTreeItemNode, UiNode, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const LOWPOLY_PLAY_BODY_DOCUMENT: &str = "lowpoly.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_DOCUMENT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(LOWPOLY_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(view: LowpolyView<'_>, doc: &LowpolyDocument, labels: &LowpolyLabels) -> UiNode {
    let active_id = resolve_active_object_id(view.projection, view.config);
    let selected_ids = selected_document_ids(view);
    let highlighted_ids = highlighted_document_ids(view);
    let items: Vec<UiTreeItemNode> = view
        .projection
        .objects
        .iter()
        .enumerate()
        .map(|(object_index, object)| {
            let object_id = object.id.clone();
            let mesh = doc.object_index(&object.id).ok().and_then(|index| doc.mesh_at(index));
            let vertex_count = mesh.as_ref().map(|entry| entry.vertex_count()).unwrap_or(0);
            let edge_count = mesh.as_ref().map(|entry| entry.edge_count()).unwrap_or(0);
            let face_count = mesh.as_ref().map(|entry| entry.face_count()).unwrap_or(0);
            let component_group = |mode: &str, label: LabelText, icon: &str, count: usize| {
                let leaves: Vec<UiTreeItemNode> = (0..count)
                    .map(|id| {
                        let row_id = document_target_row_id(&object.id, object_index, mode, id as u32);
                        let hover_args = json!({
                            "objectId": object.id,
                            "mode": mode,
                            "id": id,
                        });
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
                            action: Some(lowpoly_action(
                                "toggleSelectionTarget",
                                Some(json!({
                                    "objectId": object.id,
                                    "mode": mode,
                                    "id": id,
                                    "merge": "invertive",
                                })),
                            )),
                            hover_action: Some(lowpoly_action("setHover", Some(hover_args.clone()))),
                            unhover_action: Some(lowpoly_action("setHover", None)),
                            actions,
                            menu: None,
                            ..UiTreeItemNode::base(row_id, Label::data(format!("{} {id}", label.as_str())))
                        }
                    })
                    .collect();
                UiTreeItemNode { icon_id: IconName::from_str(icon), items: Some(leaves), description: Some(format!("{count}")), menu: None, ..UiTreeItemNode::base(format!("lowpoly-document.{object_id}.{mode}.group"), label) }
            };
            UiTreeItemNode {
                icon_id: Some("box".into()),
                action: Some(lowpoly_action(
                    "toggleSelectionTarget",
                    Some(json!({
                        "objectId": object.id,
                        "mode": "mesh",
                        "id": 0,
                        "merge": "invertive",
                    })),
                )),
                items: Some(vec![component_group("vertex", labels.vertices, "circle", vertex_count), component_group("edge", labels.edges, "minus", edge_count), component_group("face", labels.faces, "square", face_count)]),
                default_open: Some(object.id == active_id),
                description: Some(object.id.clone()),
                menu: None,
                ..UiTreeItemNode::base(format!("lowpoly-document.{object_id}"), Label::data(object.name.clone()))
            }
        })
        .collect();
    let mut builder = PanelTreeBuilder::new("lowpoly-play-document").section("lowpoly-play-document.meshes", Some(labels.meshes.into()), true, items);
    if !selected_ids.is_empty() {
        builder = builder.selected(selected_ids);
    }
    if !highlighted_ids.is_empty() {
        builder = builder.highlighted(highlighted_ids);
    }
    builder.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::apps::lowpoly::testkit::render;

    #[test]
    fn definition_binds_the_framework_document_tab_to_this_body_key() {
        use semio_framework_plugin::PanelTabDefinition;
        let definition: PanelTabDefinition = super::definition();
        assert_eq!(definition.id(), semio_framework_plugin::FRAMEWORK_PANEL_TAB_DOCUMENT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(super::LOWPOLY_PLAY_BODY_DOCUMENT));
    }

    #[test]
    fn document_tree_lists_active_object() {
        let mut a = crate::apps::lowpoly::testkit::app();
        assert!(render(&mut a, super::LOWPOLY_PLAY_BODY_DOCUMENT).contains("lowpoly-document."));
    }
}
//#endregion 🧪️Tests
