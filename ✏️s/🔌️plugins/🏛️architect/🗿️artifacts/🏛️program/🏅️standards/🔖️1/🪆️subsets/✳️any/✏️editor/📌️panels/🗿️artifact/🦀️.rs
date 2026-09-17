//! 📄️ Architect document panel — program meta, the register roster and the element list.
//!
//! 🪟️ The register roster and the element list are **virtualised**, not chunked. One `registers`
//! section spans the whole roster and one `elements` section spans the whole document; each reports its
//! full `total` and materialises only the row window the host asked for (`TreeWindows::for_body`,
//! threaded in from `ArchitectPlayApp::render`). The `.chunks(UI_FIXED_LIST_ITEMS)`-into-
//! "Registers 1–32"/"33–64"/"65–66"-sections idiom this panel used to carry is gone with it, and so is
//! the hard `ui.fixed-capacity` fault a program with more than 32 elements used to hit.

use crate::editor::architect::ui_label;
use crate::editor::architect::ARCHITECT_INTERACTION_PROGRAM;
use crate::editor::architect::{architect_action, ui_node, ui_text, ui_value_map, ui_value_text, ARCHITECT_APP_ID, ARCHITECT_INTERACTION_GRANULARITY_ENTITY};
use crate::standards::v1::subsets::any::schema::inferences::status_summary;
use crate::standards::v1::subsets::any::schema::registers::ProgramElement;
use crate::ProgramSnapshot;
use semio_framework_plugin::{tree_item_desc, tree_item_with_action, ui_node_list, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, UiAssemblyResult, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const ARCHITECT_BODY_ARTIFACT: &str = "architect.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(ARCHITECT_BODY_ARTIFACT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: element rows are the "program"
/// interaction domain's sole real pick surface — keyed by the RAW `EntityId` (already globally unique,
/// unlike note's nested block ids, so no row-id prefix/mapping is needed) and carrying nothing but
/// their granularity. The activation a click needs is the ONE tree-level `interactionSelect` binding
/// `PanelTreeBuilder::interaction_domain` stamps, so a pick row costs zero `UiValue` arena — which is
/// what lets a whole window of them exist. Register rows keep their own `selectRegister` action
/// (switching the active register is unrelated to entity selection) and sit in the SAME tree.
fn element_row(element: &ProgramElement) -> UiAssemblyResult<BuiltNode> {
    let id = element.header.id.to_string();
    let row = semio_framework_ui_contract::tree_item(ui_label(format!("{} ({:?})", element.header.name, element.kind))?).description(ui_text(&id)?).granularity(ui_text(ARCHITECT_INTERACTION_GRANULARITY_ENTITY)?);
    ui_node(row, &id)
}

/// 🪟️ The 66-row register summary is authored CLOSED: the first-paint row budget is shared across the
/// body in document order, so an open register roster would swallow all of it and the elements section
/// — this panel's actual subject and its only pick surface — would materialise zero rows on a cold
/// paint. Closed, it still stamps its full `total`, so the host can open and stream it on demand.
pub fn render(program: &ProgramSnapshot, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let summary = status_summary(program);
    let meta = ui_node_list([
        tree_item_desc("architect-document.meta.title", ui_label(format!("Title: {}", program.meta.title))?, None),
        tree_item_desc("architect-document.meta.project", ui_label(format!("Project: {} ({})", program.project.client_name, program.project.code))?, None),
        tree_item_desc("architect-document.meta.entities", ui_label(format!("Entities tracked: {}", summary.total_entities))?, None),
    ])?;
    PanelTreeBuilder::new("architect-document")?
        .section("architect-document.meta", Some(ui_label("ProgramSnapshot")?), true, meta)?
        .window_section(windows, "architect-document.registers", Some(ui_label("Registers")?), false, &summary.by_register, |row| {
            let args = ui_value_map([("registerId", ui_value_text(&row.register)?)])?;
            tree_item_with_action(format!("architect-document.register.{}", row.register), ui_label(format!("{} ({})", row.register, row.count))?, None, architect_action("selectRegister", Some(args))?)
        })?
        .window_section_or_placeholder(windows, "architect-document.elements", Some(ui_label("Elements")?), true, &program.elements, element_row, ui_label("(none)")?)?
        .interaction_domain(ARCHITECT_APP_ID, ARCHITECT_INTERACTION_PROGRAM)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
