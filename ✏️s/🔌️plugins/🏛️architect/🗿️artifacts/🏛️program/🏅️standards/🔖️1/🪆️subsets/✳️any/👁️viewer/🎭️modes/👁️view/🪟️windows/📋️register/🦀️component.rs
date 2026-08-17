//! 📋️ Architect viewer — the Register window: a read-only, document-wide register overview (entity
//! counts + draft/approved split per non-empty register). Built from the SAME artifact-level pure
//! `status_summary` inference the sibling editor surface's own document panel uses — this file itself
//! imports nothing from that sibling surface (`policyViewerPurityBreaches` forbids it outright). A
//! viewer has no per-session config (`Config = NoConfig`, contract §2.2), so unlike the editor's own
//! Register window (which reads `ArchitectConfig::active_register` to show ONE selected register) this
//! window shows every register at once — a genuinely useful, config-free read-only equivalent, not a
//! narrower stand-in for the one it mirrors.

use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::status_summary;
use crate::artifacts::program::ProgramSnapshot;
use semio_framework_plugin::{ui_text, Label, LocalizedLabel, SurfaceKind, UiNode, UiPresence, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, WindowKindDefinition, WindowOptions};

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
/// 👁️ Local tree-node helpers, mirroring the sibling surface's own presentation factories in shape —
/// deliberately NOT reused from there (a viewer must never depend on the sibling surface, see this
/// file's own doc comment); this is intentional, minimal duplication, not an oversight.
fn view_tree_item(id: impl Into<String>, label: impl Into<String>) -> UiTreeItemNode {
    UiTreeItemNode::base(id, Label::data(label.into()))
}

fn view_tree_section(id: impl Into<String>, label: Option<String>, items: Vec<UiTreeItemNode>) -> UiTreeSectionNode {
    UiTreeSectionNode { id: id.into(), label: label.map(Label::data), default_open: Some(true), presence: UiPresence::default(), items }
}

fn view_tree_node(sections: Vec<UiTreeSectionNode>) -> UiNode {
    UiNode::Tree(UiTreeNode { sections, presence: UiPresence::default(), interaction_domain: None, drop_action: None, menu: None })
}

/// 👁️ Pure `ProgramSnapshot -> UiNode` read: every non-empty register's entity count plus its
/// draft/approved split, one tree section per register, sourced entirely from the shared artifact-level
/// `status_summary` inference (no config, no selection state).
pub fn render(program: &ProgramSnapshot) -> UiNode {
    let summary = status_summary(program);
    if summary.total_entities == 0 {
        return ui_text(Label::data("No entities in this program yet."));
    }
    let sections: Vec<UiTreeSectionNode> = summary
        .by_register
        .iter()
        .filter(|register| register.count > 0)
        .map(|register| {
            let items = vec![
                view_tree_item(format!("architect-view-register.{}.total", register.register), format!("Total: {}", register.count)),
                view_tree_item(format!("architect-view-register.{}.draft", register.register), format!("Draft: {}", register.draft_count)),
                view_tree_item(format!("architect-view-register.{}.approved", register.register), format!("Approved: {}", register.approved_count)),
            ];
            view_tree_section(format!("architect-view-register.{}", register.register), Some(register.register.clone()), items)
        })
        .collect();
    view_tree_node(sections)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::{empty_plugin, sample_plugin};

    #[test]
    fn definition_declares_the_table_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, ARCHITECT_VIEW_BODY_REGISTER);
        assert!(matches!(definition.surface_kind, SurfaceKind::Table));
    }

    #[test]
    fn the_overview_lists_every_non_empty_register_with_its_counts() {
        let json = serde_json::to_string(&render(&sample_plugin())).expect("json");
        assert!(json.contains("\"elements\""));
        assert!(json.contains("Total: 2"));
    }

    #[test]
    fn an_empty_program_renders_the_placeholder() {
        let json = serde_json::to_string(&render(&empty_plugin())).expect("json");
        assert!(json.contains("No entities in this program yet."));
    }
}
//#endregion 🧪️Tests
