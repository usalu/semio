//! 🛍️ Lowpoly play app panel — the primitive catalogue (box / plane / cylinder / cone / ico sphere).

use crate::editor::lowpoly::terminology::{primitive_catalog_label, LowpolyLabels};
use crate::editor::lowpoly::{lowpoly_action, ui_label, ui_value_map, ui_value_text};
use semio_framework_plugin::{
    tree_item_with_action, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText, FRAMEWORK_PANEL_TAB_CATALOGUE_ID,
    FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};

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
/// 🛍️ One primitive row: it keeps its own `addPrimitive` binding — a catalogue row creates geometry,
/// it never picks any, so this tree binds no interaction domain and stamps no granularity.
fn primitive_row(entry: &(&'static str, &'static str, &'static str), labels: &LowpolyLabels) -> UiAssemblyResult<BuiltNode> {
    let (kind, label, icon) = *entry;
    let args = ui_value_map([("kind", ui_value_text(kind)?)])?;
    let mut node = tree_item_with_action(format!("lowpoly-play-catalogue.{kind}"), primitive_catalog_label(kind, label, labels).into_string(), Some(kind.to_string()), lowpoly_action("addPrimitive", Some(args))?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut node.component {
        props.icon = Some(UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "lowpoly primitive icon admission failed"))?);
    }
    Ok(node)
}

pub(crate) fn render(labels: &LowpolyLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    PanelTreeBuilder::new("lowpoly-play-catalogue")?
        .window_section(windows, "lowpoly-play-catalogue.primitives", Some(ui_label(labels.primitives.as_str())?), true, PRIMITIVE_CATALOG, |entry| primitive_row(entry, labels))?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
