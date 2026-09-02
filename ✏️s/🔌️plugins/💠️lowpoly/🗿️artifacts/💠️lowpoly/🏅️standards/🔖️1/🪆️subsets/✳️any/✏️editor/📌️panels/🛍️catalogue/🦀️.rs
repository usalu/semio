//! 🛍️ Lowpoly play app panel — the primitive catalogue (box / plane / cylinder / cone / ico sphere).

use crate::editor::lowpoly::terminology::{primitive_catalog_label, LowpolyLabels};
use crate::editor::lowpoly::{lowpoly_action, ui_label, ui_node_list, ui_value_map, ui_value_text};
use semio_framework_plugin::{tree_item_with_action, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiText, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const LOWPOLY_PLAY_BODY_CATALOGUE: &str = "lowpoly.play.catalogue";

const PRIMITIVE_CATALOG: &[(&str, &str, &str)] = &[("box", "Cube", "box"), ("plane", "Plane", "square"), ("cylinder", "Cylinder", "cylinder"), ("cone", "Cone", "triangle"), ("ico_sphere", "Ico Sphere", "globe")];
//#endregion 🔖️Constants

//#region 🔖️Definition
pub(crate) fn definition() -> PanelTabDefinition {
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
pub(crate) fn render(labels: &LowpolyLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let items = ui_node_list(PRIMITIVE_CATALOG.iter().map(|(kind, label, icon)| {
        let args = ui_value_map([("kind", ui_value_text(kind)?)])?;
        let mut node = tree_item_with_action(format!("lowpoly-play-catalogue.{kind}"), primitive_catalog_label(kind, label, labels).into_string(), Some((*kind).to_string()), lowpoly_action("addPrimitive", Some(args))?)?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
            props.icon = Some(UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly primitive icon admission failed"))?);
        }
        Ok(node)
    }))?;
    PanelTreeBuilder::new("lowpoly-play-catalogue")?.section("lowpoly-play-catalogue.primitives", Some(ui_label(labels.primitives.as_str())?), true, items)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use crate::editor::lowpoly::testkit::{app, render};

    #[semio_framework_async_macros::async_test]
    async fn catalogue_lists_primitives() {
        let mut a = app().await;
        let json = render(&mut a, super::LOWPOLY_PLAY_BODY_CATALOGUE).await;
        assert!(json.contains("lowpoly-play-catalogue.box"));
        assert!(json.contains("Cube"));
        assert!(json.contains("Ico Sphere"));
    }
}
//#endregion 🧪️Tests
