//! 🗑️ `delete-selection` — removes every frame in the live `"elements"` domain selection.

use crate::editor::layout::LAYOUT_INTERACTION_ELEMENTS;
use crate::mutations::delete_frame::DeleteFrame;
use crate::mutations::LayoutMutation;
use crate::LayoutSnapshot;
use semio_framework_plugin::app::InteractionView;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, NoConfig, NoConfigMutation};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "delete-selection")]
pub struct DeleteSelection {}

fn frame_page_id(doc: &LayoutSnapshot, frame_id: &str) -> Option<String> {
    doc.pages.iter().find(|page| page.frames.iter().any(|frame| frame.id() == frame_id)).map(|page| page.id.clone())
}

/// 🕹️ `app_commands!`'s generated `dispatch` has no `interaction` slot — `LayoutPlayApp::handle` and
/// the retained window work route here instead (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
pub fn handle(_payload: &DeleteSelection, _doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    Ok(Emit::default())
}

pub fn apply(_payload: &DeleteSelection, doc: &ArtifactView<'_, LayoutSnapshot>, _cfg: &ConfigView<'_, NoConfig>, interaction: &InteractionView<'_>) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    apply_frame_ids(doc, &interaction.selection(LAYOUT_INTERACTION_ELEMENTS).ids)
}

pub fn apply_frame_ids(doc: &ArtifactView<'_, LayoutSnapshot>, selected: &[String]) -> Result<Emit<LayoutMutation, NoConfigMutation>, Fault> {
    let mutations = selected
        .iter()
        .filter_map(|frame_id| frame_page_id(doc.snapshot, frame_id).map(|page_id| LayoutMutation::DeleteFrame(DeleteFrame { page_id, frame_id: frame_id.clone() })))
        .collect();
    Ok(Emit::mutations(mutations))
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
