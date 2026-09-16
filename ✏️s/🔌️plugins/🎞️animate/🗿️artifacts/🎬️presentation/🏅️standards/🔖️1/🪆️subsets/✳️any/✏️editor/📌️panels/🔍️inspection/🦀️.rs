//! 🔍️ Animate presentation app panel — the inspector: field editors for the selected tile(s).

use crate::editor::animate::terminology::AnimatePresentationLabels;
use crate::editor::animate::{ui_label, ui_node_list};
use crate::{PresentationSnapshot, PRESENTATION_DOCUMENT_SCHEMA};
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

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
pub fn render(deck: &PresentationSnapshot, labels: &AnimatePresentationLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let (_, tiles) = crate::presentation_working_scene(deck);
    let items = ui_node_list([
        tree_item_desc("animate-presentation-play-inspector.schema", ui_label(labels.details_schema_field.as_str())?, Some(PRESENTATION_DOCUMENT_SCHEMA.into())),
        tree_item_desc("animate-presentation-play-inspector.tiles", ui_label(labels.details_tiles_field.as_str())?, Some(tiles.len().to_string())),
    ])?;
    PanelTreeBuilder::new("animate-presentation-play-inspector")?
        .section("animate-presentation-play-inspector.summary", Some(ui_label(labels.details_title.as_str())?), true, items)?
        .build()
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
