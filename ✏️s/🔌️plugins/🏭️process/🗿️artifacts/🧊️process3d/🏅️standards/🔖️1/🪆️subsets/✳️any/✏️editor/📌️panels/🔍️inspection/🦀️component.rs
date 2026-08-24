//! 🔍️ Process 3d play app panel — the field inspector for whatever is selected: the stock, a process
//! step, or a workshop machine.
//!
//! 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): the per-item inspector (stock/step/
//! machine field groups, keyed by the old `Process3dConfig::selected_id`) is unreachable at this
//! `render` boundary now — mesh/geometry object selection is the framework-owned `"geometry"`
//! interaction domain, and `ArtifactEditor::render` carries no `InteractionView` parameter (a known SDK
//! gap flagged in the ticket's `w3c-summary.md`, mirrors `📐️cad`'s own
//! `document_tree_selected_ids`/inspector precedent). This panel falls back to its always-empty
//! state unconditionally until a render-time interaction read exists.

use crate::artifacts::process3d::Process3dSnapshot;
use crate::editor::process3d::config::Process3dConfig;
use crate::editor::process3d::terminology::Process3dLabels;
use semio_framework_plugin::{tree_item, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const PROCESS_3D_PLAY_BODY_INSPECTION: &str = "process.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(PROCESS_3D_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(_fixture: &Process3dSnapshot, _cfg: &Process3dConfig, labels: &Process3dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let items = crate::editor::process3d::ui_node_list([tree_item("process3d-play-inspector.empty", crate::editor::process3d::ui_label(labels.no_selection.as_str())?)])?;
    PanelTreeBuilder::new("process3d-play-inspector")?
        .section("process3d-play-inspector.section", Some(crate::editor::process3d::ui_label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)?), true, items)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::process3d::commands::step::add_step;
    use crate::editor::process3d::testkit;
    use crate::editor::process3d::Process3dCommand;

    #[semio_framework_async_macros::async_test]
    async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
        assert_eq!(definition.body_key.as_deref(), Some(PROCESS_3D_PLAY_BODY_INSPECTION));
    }

    //#region 🔖️AddStepIsADocumentedNoOp
    /// 🌉️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM wave 4: `AddStep` dispatches a
    /// `CreateStep` mutation, a documented no-op now (`steps` composes an `s.stdio.semio.flow`
    /// CHILD HANDLE — no resolver, see `ProcessWorkingScene`'s doc comment), so the persisted
    /// `document.steps` this test suite used to read is gone. `add_step::handle`'s own capability-
    /// dimension VALIDATION gate is also a documented gap now (no resolvable stock extent — see
    /// its own doc comment), so every resolvable machine/capability pair succeeds unconditionally.
    /// These tests assert the honest post-migration behavior: the command still dispatches its
    /// (no-op) mutation.
    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): the inspector no longer selects
    /// the new step (selection is framework-owned now, unreachable from `Emit`), so this only
    /// asserts the still-real mutation dispatch, not a rendered inspector state.
    #[semio_framework_async_macros::async_test]
    async fn add_step_dispatches_its_no_op_mutation() {
        let mut app = testkit::app();
        let result = testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("drill".into()), machine_id: None, capability_id: None, position: None }));
        assert!(!result.mutations.is_empty(), "AddStep must still dispatch its (no-op) CreateStep mutation");
    }

    /// 🌉️ Same documented gap as above, from the catalogue-routed (machine/capability-addressed)
    /// entry point: even a stock the pre-migration code would have rejected (circular saw needs
    /// height ≤ 0.065m; the default timber beam is 0.24m) now succeeds, since the dimension gate
    /// can no longer read real stock extents.
    #[semio_framework_async_macros::async_test]
    async fn add_step_via_catalogue_no_longer_gates_on_stock_dimensions() {
        let mut app = testkit::app();
        let result = testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: None, machine_id: Some("circularSaw".into()), capability_id: Some("crosscut".into()), position: None }));
        assert!(!result.mutations.is_empty(), "documented gap: the dimension-validation gate can no longer reject an oversized stock");
    }

    #[semio_framework_async_macros::async_test]
    async fn measure_arg_routes_to_generic_machine_and_dispatches() {
        let mut app = testkit::app();
        let result = testkit::dispatch(&mut app, Process3dCommand::AddStep(add_step::AddStep { measure: Some("cut".into()), machine_id: None, capability_id: None, position: None }));
        assert!(!result.mutations.is_empty());
    }

    /// 🕹️ FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM (26/08/14): `render` carries no `InteractionView`
    /// (a known SDK gap — see this file's own header comment), so the inspector always renders its
    /// empty state now, regardless of framework-owned selection.
    #[semio_framework_async_macros::async_test]
    async fn inspector_always_renders_the_empty_state() {
        let mut app = testkit::app();
        let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_INSPECTION);
        assert!(rendered.contains("process3d-play-inspector.empty"));
    }
    //#endregion 🔖️AddStepIsADocumentedNoOp
}
//#endregion 🧪️Tests
