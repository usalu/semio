//! ✏️ ✏️ Layout play app commands command — `add-page`.

use crate::editor::layout::config::{LayoutConfig, LayoutConfigMutation};
use crate::artifacts::layout::schema::text_to_rgba;
use crate::artifacts::layout::mutations::change_frame_columns::mutation::ChangeFrameColumns;
use crate::artifacts::layout::mutations::change_frame_fill::mutation::ChangeFrameFill;
use crate::artifacts::layout::mutations::change_frame_stroke::mutation::ChangeFrameStroke;
use crate::artifacts::layout::mutations::change_frame_wrap_mode::mutation::ChangeFrameWrapMode;
use crate::artifacts::layout::mutations::change_link_path::mutation::ChangeLinkPath;
use crate::artifacts::layout::mutations::change_page_height::mutation::ChangePageHeight;
use crate::artifacts::layout::mutations::change_page_width::mutation::ChangePageWidth;
use crate::artifacts::layout::mutations::create_frame::mutation::CreateFrame;
use crate::artifacts::layout::mutations::create_page::mutation::CreatePage;
use crate::artifacts::layout::mutations::edit_story::mutation::EditStory;
use crate::artifacts::layout::mutations::move_frame::mutation::MoveFrame;
use crate::artifacts::layout::mutations::rename_page::mutation::RenamePage;
use crate::artifacts::layout::mutations::resize_frame::mutation::ResizeFrame;
use crate::artifacts::layout::mutations::update_page_columns::mutation::UpdatePageColumns;
use crate::artifacts::layout::mutations::update_page_margins::mutation::UpdatePageMargins;
use crate::artifacts::layout::mutations::LayoutMutation;
use crate::artifacts::layout::{Frame, LayoutSnapshot, Page, PageColumns, PageMargins};
use semio_framework_plugin::{ConfigView, ArtifactView, Emit, Fault};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "add-page")]
pub struct AddPage {}

pub async fn handle(_payload: &AddPage, doc: &ArtifactView<'_, LayoutSnapshot>, cfg: &ConfigView<'_, LayoutConfig>) -> Result<Emit<LayoutMutation, LayoutConfigMutation>, Fault> {
    let document = doc.snapshot;
    let config = cfg.snapshot;
    let template = document.pages.iter().find(|page| page.id == config.active_page_id).or_else(|| document.pages.first());
    let (width, height, spread_id, parent_page_id, margins, columns) = template.map_or(
        (595.0, 842.0, "spread-1".into(), None, PageMargins { top: 48.0, right: 36.0, bottom: 48.0, left: 36.0 }, PageColumns { count: 1, gutter: 0.0 }),
        |page| (page.width, page.height, page.spread_id.clone(), page.parent_page_id.clone(), page.margins.clone(), page.columns.clone()),
    );
    let page_id = format!("page-{}", document.pages.len() + 1);
    let layer_id = format!("layer-{page_id}");
    let index = document.pages.len();
    let page = crate::artifacts::layout::Page {
        id: page_id.clone(),
        name: format!("Page {}", document.pages.len() + 1),
        spread_id,
        parent_page_id,
        width,
        height,
        margins,
        columns,
        guides: Vec::new(),
        layer_ids: vec![layer_id.clone()],
        layers: vec![crate::artifacts::layout::Layer { id: layer_id, name: "Content".into(), visible: true, locked: false, object_ids: Vec::new() }],
        frames: Vec::new(),
        overrides: Vec::new(),
    };
    // 🕹️ Used to also `SetSelection { ids: vec![page_id] }` — pages were never real "elements" domain
    // targets (canvas hit-testing only ever resolves frame ids), so selecting one meant nothing beyond
    // coincidentally matching a document-tree page row's id; dropped with the deleted config field
    // rather than reproduced as a meaningless `interactionSelect` (ticket
    // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM). `setActivePage` still switches the Blueprint
    // surface to the new page.
    Ok(Emit {
        artifact_mutations: vec![LayoutMutation::CreatePage(CreatePage { page, index: Some(index) })],
        config_mutations: vec![LayoutConfigMutation::SetActivePage { page_id: page_id.clone() }],
        ..Default::default()
    })
}
