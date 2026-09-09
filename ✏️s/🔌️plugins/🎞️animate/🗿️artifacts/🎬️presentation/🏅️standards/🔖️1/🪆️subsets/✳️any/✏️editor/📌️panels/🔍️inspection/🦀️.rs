//! 🔍️ Animate presentation app panel — the inspector: field editors for the selected tile(s).

use crate::editor::animate::terminology::AnimatePresentationLabels;
use crate::editor::animate::{ui_children, ui_label, ui_node};
use crate::{PresentationSnapshot, PRESENTATION_DOCUMENT_SCHEMA};
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_ui_contract::{column, field, section, text, BuiltNode};

//#region 🔖️Constants
pub const PRESENTATION_PLAY_BODY_DETAILS: &str = "animate.presentation.play.details";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(PRESENTATION_PLAY_BODY_DETAILS.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// ⚠️ Ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the per-selected-tile field group
/// (crop x/y/width/height, name, delete) this panel used to build from `config.selected_ids` is
/// deleted along with that field — selection is framework-owned state now and
/// `ArtifactApp::render(body_key, doc, cfg, view_state)` is never given an `InteractionView` (only
/// `handle`/`copy_fragment`/`cut_operations` are). Documented reduced-fidelity gap, same shape as
/// `🖍️draw`'s `properties` panel (`🎛️apps/🖍️draw/📌️panels/🔍️properties/🦀️.rs`): falls through
/// to a schema/tile-count summary until a resolved-selection render path exists.
pub fn render(deck: &PresentationSnapshot, labels: &AnimatePresentationLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (_, tiles) = crate::presentation_working_scene(deck);
    let schema = ui_node(text(ui_label(PRESENTATION_DOCUMENT_SCHEMA)?), "animate-presentation-play-inspector.schema.value")?;
    let tile_count = ui_node(text(ui_label(tiles.len().to_string())?), "animate-presentation-play-inspector.tiles.value")?;
    let schema = ui_node(ui_children(field(ui_label(labels.details_schema_field.as_str())?), [schema])?, "animate-presentation-play-inspector.schema")?;
    let tile_count = ui_node(ui_children(field(ui_label(labels.details_tiles_field.as_str())?), [tile_count])?, "animate-presentation-play-inspector.tiles")?;
    let summary = ui_node(ui_children(section(ui_label(labels.details_title.as_str())?).default_open(true), [schema, tile_count])?, "animate-presentation-play-inspector.empty")?;
    ui_node(ui_children(column(), [summary])?, "animate-presentation-play-inspector")
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semantic-contract/🦀️.rs"]
mod semantic_contract;
