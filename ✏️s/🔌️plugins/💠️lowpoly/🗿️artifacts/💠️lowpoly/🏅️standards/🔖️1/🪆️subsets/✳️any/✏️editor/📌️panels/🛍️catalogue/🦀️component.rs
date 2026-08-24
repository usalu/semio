//! 🛍️ Lowpoly play app panel — the primitive catalogue (box / plane / cylinder / cone / ico sphere).

use crate::editor::lowpoly::lowpoly_action;
use crate::editor::lowpoly::terminology::{primitive_catalog_label, LowpolyLabels};
use semio_framework_plugin::{tree_item_with_action, IconName, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub(crate) const LOWPOLY_PLAY_BODY_CATALOGUE: &str = "lowpoly.play.catalogue";

const PRIMITIVE_CATALOG: &[(&str, &str, &str)] = &[("box", "Cube", "box"), ("plane", "Plane", "square"), ("cylinder", "Cylinder", "cylinder"), ("cone", "Cone", "triangle"), ("ico_sphere", "Ico Sphere", "globe")];
//#endregion 🔖️Constants

//#region 🔖️Definition
pub(crate) async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(LOWPOLY_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub(crate) async fn render(labels: &LowpolyLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let items: Vec<UiTreeItemNode> = PRIMITIVE_CATALOG
        .iter()
        .map(|(kind, label, icon)| UiTreeItemNode {
            icon_id: IconName::from_str(icon),
            ..tree_item_with_action(format!("lowpoly-play-catalogue.{kind}"), primitive_catalog_label(kind, label, labels), Some((*kind).to_string()), lowpoly_action("addPrimitive", Some(json!({ "kind": kind }))))?
        })
        .collect();
    PanelTreeBuilder::new("lowpoly-play-catalogue")?.section("lowpoly-play-catalogue.primitives", Some(labels.primitives.into()), true, items)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::lowpoly::testkit::{app, render};

    #[semio_framework_async_macros::async_test]
    async fn catalogue_lists_primitives() {
        let mut a = app();
        let json = render(&mut a, super::LOWPOLY_PLAY_BODY_CATALOGUE);
        assert!(json.contains("lowpoly-play-catalogue.box"));
        assert!(json.contains("Cube"));
        assert!(json.contains("Ico Sphere"));
    }
}
//#endregion 🧪️Tests
