//! 🛍️ Layout play app panel — the catalogue: draggable page/frame-kind creation items.
//!
//! 🪟️ The roster is a windowed section like every other container: it stamps its FULL extent and
//! materialises only the host's slice (`ViewModel::tree_windows`), never a `+N` row — the same law
//! whether the roster stays a compile-time array or becomes document-derived.
//!
//! 🕹️ Deliberately UNBOUND to an interaction domain: a catalogue row is not a pick, it is its own
//! `addPage`/`addFrame` command, so the rows keep their action, their drag payload and their
//! catalogue-namespaced keys, and stamp no `granularity`.

use crate::terminology::catalogue_kind_label;
use crate::terminology::LayoutLabels;
use semio_s_artifact_layout_layout_editor_commands::{layout_action, ui_value_map, ui_value_text};
use semio_framework_plugin::tree_item_with_action;
use semio_framework_ui_locale::Label;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiFixedMap;
use semio_framework_plugin::UiText;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_ID;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL;

//#region 🔖️Constants
pub const LAYOUT_PLAY_BODY_CATALOGUE: &str = "layout.play.catalogue";
pub const LAYOUT_CATALOGUE_ROOT: &str = "layout-catalogue";
pub const LAYOUT_CATALOGUE_KINDS_SECTION: &str = "layout-catalogue.kinds";


/// 🛍️ The full creation roster this catalogue windows over — the page item plus every frame kind.
const LAYOUT_CATALOGUE_ROSTER: &[(&str, &str)] = &[("page", "file"), ("rect", "square"), ("text", "type"), ("image", "image"), ("png", "image"), ("jpg", "image"), ("gif", "image"), ("bmp", "image"), ("tiff", "image"), ("pdf", "file-text"), ("svg", "spline"), ("drawing", "pen-tool"), ("dwg", "pen-tool"), ("dxf", "pen-tool"), ("raster", "image"), ("bitmap", "image"), ("cad", "pen-tool"), ("map", "map"), ("fem2d", "spline"), ("note", "file-text"), ("writer", "type"), ("forms", "file-text"), ("flow", "spline"), ("equation", "type"), ("puzzle2d", "square"), ("block2d", "square"), ("generation2d", "spline"), ("terrain", "map"), ("grid2d", "image"), ("wfc2d", "image"), ("presentation", "file"), ("sequence", "file"), ("wires", "spline")];
pub const LAYOUT_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";
pub const LAYOUT_CATALOGUE_KIND_MIME_PREFIX: &str = "application/x-semio-catalogue-kind.";

pub fn catalogue_kind(kind: &str) -> Option<&'static str> {
    LAYOUT_CATALOGUE_ROSTER.iter().find_map(|(candidate, _)| (*candidate == kind).then_some(*candidate))
}
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(LAYOUT_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn catalogue_tree_item(kind: &str, label: impl Into<Label>, icon: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let action = if kind == "page" { layout_action("addPage", None)? } else { layout_action("addFrame", Some(ui_value_map([("kind", ui_value_text(kind)?)])?))? };
    let mut item = tree_item_with_action(format!("layout-catalogue.{kind}"), label.into().as_str(), Some(kind.into()), action)?;
    let mut drag_data = UiFixedMap::default();
    let payload = UiText::try_from_string(serde_json::json!({ "kind": kind }).to_string()).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout catalogue drag payload admission failed"))?;
    drag_data
        .try_push(UiText::try_from_str(LAYOUT_CATALOGUE_DRAG_MIME).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "layout catalogue drag mime admission failed"))?, payload)
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout catalogue drag entry admission failed"))?;
    drag_data
        .try_push(
            UiText::try_from_string(format!("{LAYOUT_CATALOGUE_KIND_MIME_PREFIX}{kind}")).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout catalogue kind mime admission failed"))?,
            UiText::try_from_str("").ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "layout catalogue empty drag value admission failed"))?,
        )
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "layout catalogue kind drag entry admission failed"))?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = Some(UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "layout catalogue icon admission failed"))?);
        props.draggable = Some(true);
        props.drag_data = Some(drag_data);
    }
    Ok(item)
}

pub fn render(labels: &LayoutLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    PanelTreeBuilder::new(LAYOUT_CATALOGUE_ROOT)?
        .window_section(windows, LAYOUT_CATALOGUE_KINDS_SECTION, Some(semio_s_artifact_layout_layout_editor_commands::ui_label(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)?), true, LAYOUT_CATALOGUE_ROSTER, |(kind, icon)| {
            let label = if *kind == "page" { labels.catalogue_page.into() } else { catalogue_kind_label(kind, labels) };
            catalogue_tree_item(kind, label, icon)
        })?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
