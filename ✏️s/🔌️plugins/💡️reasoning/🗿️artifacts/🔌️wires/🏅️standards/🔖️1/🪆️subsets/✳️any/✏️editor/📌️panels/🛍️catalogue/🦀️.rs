//! 🛍️ Wires play app panel — the identity/relationship kind catalogue (click to add).

use crate::editor::wires::terminology::WiresLabels;
use crate::editor::wires::{ui_value_map, ui_value_text, wires_action};
use dsl::DslValue;
use semio_framework_plugin::{tree_item_with_action, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const WIRES_PLAY_BODY_CATALOGUE: &str = "reasoning.wires.catalogue";
const WIRES_PLAY_CATALOGUE_TAB_ID: &str = "framework.panel.catalogue";
const WIRES_PLAY_KINDS_NAMESPACE: &str = "wires-play-kinds";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(WIRES_PLAY_CATALOGUE_TAB_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(WIRES_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn wires_kind_catalog_entries(wires: &DslValue, key: &str) -> Vec<DslValue> {
    wires
        .get("kindCatalogs")
        .and_then(|value| value.get(key))
        .and_then(|value| value.as_array())
        .map(|values| values.to_vec())
        .or_else(|| wires.get("board").and_then(|value| value.get("meta")).and_then(|value| value.get("kindCatalogs")).and_then(|value| value.get(key)).and_then(|value| value.as_array()).map(|values| values.to_vec()))
        .unwrap_or_default()
}

fn catalog_kind_label(entry: &DslValue) -> String {
    entry.get("name").and_then(|value| value.as_str()).filter(|value| !value.is_empty()).or_else(|| entry.get("id").and_then(|value| value.as_str())).unwrap_or("kind").into()
}

/// 🛍️ One kind row: it keeps its own `addNode`/`addRelationship` binding — a catalogue row creates
/// graph elements, it never picks any, so this tree binds no interaction domain.
fn kind_catalog_row(kind: &str, index: usize, entry: &DslValue) -> UiAssemblyResult<BuiltNode> {
    let kind_id = entry.get("id").and_then(|value| value.as_str()).ok_or_else(|| PluginAssemblyError::new("ui.catalogue", "wires catalogue kind id missing"))?;
    let args = ui_value_map([("kind", ui_value_text(kind_id)?)])?;
    let action = match kind {
        "relationship-kinds" => wires_action("addRelationship", Some(args))?,
        _ => wires_action("addNode", Some(args))?,
    };
    tree_item_with_action(format!("{WIRES_PLAY_KINDS_NAMESPACE}.{kind}.{index}.{kind_id}"), crate::editor::wires::ui_label(catalog_kind_label(entry))?, Some(kind_id.into()), action)
}

pub fn render(wires: &DslValue, labels: &WiresLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let identity_entries: Vec<_> = wires_kind_catalog_entries(wires, "identityKinds").into_iter().enumerate().collect();
    let relationship_entries: Vec<_> = wires_kind_catalog_entries(wires, "relationshipKinds").into_iter().enumerate().collect();
    PanelTreeBuilder::new(WIRES_PLAY_KINDS_NAMESPACE)?
        .window_section_or_placeholder(
            windows,
            "wires-play-kinds.identity-kinds",
            Some(crate::editor::wires::ui_label(labels.identity_kinds.as_str())?),
            true,
            &identity_entries,
            |(index, entry)| kind_catalog_row("identity-kinds", *index, entry),
            crate::editor::wires::ui_label("(none)")?,
        )?
        .window_section_or_placeholder(
            windows,
            "wires-play-kinds.relationship-kinds",
            Some(crate::editor::wires::ui_label(labels.relationship_kinds.as_str())?),
            true,
            &relationship_entries,
            |(index, entry)| kind_catalog_row("relationship-kinds", *index, entry),
            crate::editor::wires::ui_label("(none)")?,
        )?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
