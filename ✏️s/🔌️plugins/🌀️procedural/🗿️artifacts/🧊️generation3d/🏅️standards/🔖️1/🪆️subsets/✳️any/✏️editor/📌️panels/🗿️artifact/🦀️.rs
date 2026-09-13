//! 📄️ Generation3d play app panel — the document tree: widgets of the current fixture.

use crate::editor::generation3d::modes::edit::windows::flow::graph_targets_json;
use crate::editor::generation3d::terminology::Generation3dLabels;
use crate::editor::generation3d::GENERATION_3D_INTERACTION_CHANNEL;
use crate::editor::generation3d::GENERATION_3D_INTERACTION_DOMAIN;
use crate::editor::generation3d::GENERATION_3D_PLAY_APP_ID;
use crate::widget_id;
use semio_framework_artifact_flow_flow::FlowFixture;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, Trigger};
use semio_framework_plugin::{ActionFactory, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiAssemblyResult, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const GENERATION_3D_PLAY_BODY_DOCUMENT: &str = "procedural.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(GENERATION_3D_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn document_error(scope: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.document", scope)
}

/// 🌳️ One document row: the widget's own id verbatim — the `graph` domain's `node` target — carrying
/// the same two interaction verbs the flow window's outline rows carry, so a click here picks exactly
/// the node a click on the canvas picks and the Inspection panel leaves its "no selection" branch.
///
/// 🪪️ `.interaction_domain(…)` on the tree is NOT a click contract: it carries topology and
/// post-render presence stamping only, and the React target never turns it into a dispatch (the
/// `ui_tree_stamp_presence` path it was written against is `🎯️targets/🧊️wgpu`-only). Rows without
/// these bindings are inert on the React renderer — a real click reached the row, `aria-selected`
/// stayed `false` and nothing was invoked (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The same
/// explicit pair is what `🎭️modes/✏️edit/🪟️windows/🕸️flow`'s `node_row` and the puzzle3d document
/// panel author.
fn widget_row(id: &str, icon_id: &str, factory: &ActionFactory) -> UiAssemblyResult<BuiltNode> {
    let targets = graph_targets_json("node", id);
    let (select, select_args) = factory.action(
        semio_framework_plugin::INTERACTION_SELECT_ACTION_ID,
        Some(crate::ui_value_map([
            ("domainId", crate::ui_value_text(GENERATION_3D_INTERACTION_DOMAIN)?),
            ("merge", crate::ui_value_text("replace")?),
            ("method", crate::ui_value_text("pick")?),
            ("targets", crate::ui_value_text(&targets)?),
        ])?),
    )?;
    let (hover, hover_args) = factory.action(
        semio_framework_plugin::INTERACTION_HOVER_ACTION_ID,
        Some(crate::ui_value_map([
            ("channel", crate::ui_value_text(GENERATION_3D_INTERACTION_CHANNEL)?),
            ("domainId", crate::ui_value_text(GENERATION_3D_INTERACTION_DOMAIN)?),
            ("targets", crate::ui_value_text(&targets)?),
        ])?),
    )?;
    let mut builder = semio_framework_ui_contract::tree_item(crate::ui_label(id)?).try_id(id).map_err(|_| document_error("row.id"))?.icon(crate::ui_text(icon_id)?);
    builder = builder.try_on_with(Trigger::Activate, select, select_args.ok_or_else(|| document_error("row.select-args"))?).map_err(|_| document_error("row.select"))?;
    builder = builder.try_on_with(Trigger::HoverPreview, hover, hover_args.ok_or_else(|| document_error("row.hover-args"))?).map_err(|_| document_error("row.hover"))?;
    builder.try_build().map_err(|_| document_error("row.build"))
}

/// 🕹️ Item ids are the RAW widget id (no namespace prefix) — they must equal the `graph` interaction
/// domain's target ids one-for-one so `.interaction_domain("graph")?`'s post-render presence stamping
/// (`ui_tree_stamp_presence`) can match them by plain string membership (ticket
/// 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), and so the per-row `interactionSelect`/
/// `interactionHover` args [`widget_row`] binds name the very same targets.
pub fn render(fixture: &FlowFixture, labels: &Generation3dLabels) -> UiAssemblyResult<BuiltNode> {
    let factory = ActionFactory::new(GENERATION_3D_PLAY_APP_ID);
    let items = crate::ui_node_list(fixture.widgets.iter().map(|widget| widget_row(widget_id(widget), "cpu", &factory)))?;
    PanelTreeBuilder::new("procedural-play-document")?
        .section("procedural-play-document.widgets", Some(crate::ui_label(labels.widgets.as_str())?), true, items)?
        .interaction_domain(GENERATION_3D_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
