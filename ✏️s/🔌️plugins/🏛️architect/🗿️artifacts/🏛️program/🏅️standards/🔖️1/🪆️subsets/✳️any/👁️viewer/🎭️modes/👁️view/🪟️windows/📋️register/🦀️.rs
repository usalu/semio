//! 📋️ Architect viewer — the Register window: a read-only, document-wide register overview (entity
//! counts + draft/approved split per non-empty register). Built from the SAME artifact-level pure
//! `status_summary` inference the sibling editor surface's own document panel uses — this file itself
//! imports nothing from that sibling surface (`policyViewerPurityBreaches` forbids it outright). A
//! viewer has no per-session config (`Config = NoConfig`, contract §2.2), so unlike the editor's own
//! Register window (which reads `ArchitectConfig::active_register` to show ONE selected register) this
//! window shows every register at once — a genuinely useful, config-free read-only equivalent, not a
//! narrower stand-in for the one it mirrors.

use crate::standards::v1::subsets::any::schema::inferences::status_summary;
use crate::ProgramSnapshot;
use semio_framework_plugin::{tree_item_desc, Label, LocalizedLabel, PanelTreeBuilder, PluginAssemblyError, SurfaceKind, UiFixedList, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const ARCHITECT_VIEW_WINDOW_REGISTER: &str = "architect-view-register";
pub const ARCHITECT_VIEW_BODY_REGISTER: &str = "architect.view.register";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 👁️ Stitched into the viewer manifest by `crate::viewer::architect::create_architect_viewer`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: ARCHITECT_VIEW_WINDOW_REGISTER.into(),
        label: LocalizedLabel::native("Register Overview", "Register-Übersicht"),
        body_key: ARCHITECT_VIEW_BODY_REGISTER.into(),
        surface_kind: SurfaceKind::Table,
        icon_id: "list".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        // 👁️ Read-only overview — no `.window_kind_interactions(..)` reference for this window.
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn ui_label(value: impl AsRef<str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_ui_contract::Label> {
    value.as_ref().try_into().map_err(|_| PluginAssemblyError::new("architect.viewer.label.capacity", "register label admission failed"))
}

/// 👁️ Renders the artifact's non-empty registers and their total, draft and approved counts.
pub fn render(program: &ProgramSnapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let summary = status_summary(program);
    if summary.total_entities == 0 {
        return semio_framework_plugin::built_text_node(Label::data("No entities in this program yet."))
            .map_err(|_| PluginAssemblyError::new("architect.viewer.label.capacity", "register placeholder admission failed"));
    }
    let mut tree = PanelTreeBuilder::new("architect-view-register")?;
    for register in summary.by_register.iter().filter(|register| register.count > 0) {
        let mut items = UiFixedList::default();
        for (key, label, count) in [("total", "Total", register.count), ("draft", "Draft", register.draft_count), ("approved", "Approved", register.approved_count)] {
            let node = tree_item_desc(format!("architect-view-register.{}.{key}", register.register), ui_label(format!("{label}: {count}"))?, None)?;
            items.try_push(node).map_err(|_| PluginAssemblyError::new("architect.viewer.items.capacity", "register item admission failed"))?;
        }
        tree = tree.section(format!("architect-view-register.{}", register.register), Some(ui_label(&register.register)?), true, items)?;
    }
    tree.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{empty_plugin, sample_plugin};

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_table_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, ARCHITECT_VIEW_BODY_REGISTER);
        assert!(matches!(definition.surface_kind, SurfaceKind::Table));
    }

    #[semio_framework_async_macros::async_test]
    async fn the_overview_lists_every_non_empty_register_with_its_counts() {
        let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(render(&sample_plugin()).expect("render"))).expect("retire viewer tree");
        assert!(json.contains("\"elements\""));
        assert!(json.contains("Total: 2"));
    }

    #[semio_framework_async_macros::async_test]
    async fn an_empty_program_renders_the_placeholder() {
        let json = semio_framework_plugin::testkit::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(render(&empty_plugin()).expect("render"))).expect("retire viewer tree");
        assert!(json.contains("No entities in this program yet."));
    }
}
//#endregion 🧪️Tests
