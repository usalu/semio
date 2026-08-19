//! 📄️ VCS play app panel — the document tree: checkpoints and alternatives of the seeded history.

use crate::editor::vcs::terminology::VcsPlayLabels;
use crate::editor::vcs::{vcs_action, VCS_INTERACTION_HISTORY};
use semio_framework_plugin::{
    tree_item_with_action, HistoryView, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const VCS_PLAY_BODY_DOCUMENT: &str = "vcs.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(VCS_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🌳️ Builds the document tree's checkpoint + alternative sections from `HistoryView` alone — the
/// swimlane graph's own `HistoryColumn`s carry everything needed (checkpoint id/description/timestamp,
/// and which alternative ids reference each row); alternative rows are labeled by id since
/// `HistoryColumn` doesn't carry alternative display names (`vcs_kernel::Alternative.name` isn't part of
/// the `ArtifactApp`-visible `HistoryView` contract — a real gap, noted for whoever revisits this).
///
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the tree is bound to the "history"
/// interaction domain (`VCS_INTERACTION_HISTORY`'s doc comment) — the framework now owns and stamps
/// checkpoint multi-select highlighting, replacing the deleted `selected`/`setSelection` plumbing.
/// Per-row `checkoutCheckpoint`/`switchAlternative` clicks stay app actions (navigation, not selection).
pub async fn render(history: &HistoryView, labels: &VcsPlayLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("vcs-play-document");
    let checkpoint_items: Vec<UiTreeItemNode> = history
        .columns
        .iter()
        .rev()
        .map(|column| UiTreeItemNode {
            icon_id: Some("git-commit".into()),
            menu: None,
            ..tree_item_with_action(
                builder.item_id("checkpoint", &column.checkpoint_id),
                Label::data(column.description.clone().unwrap_or_else(|| column.checkpoint_id.clone())),
                Some(column.timestamp.clone()),
                vcs_action("checkoutCheckpoint", Some(json!({ "checkpointId": column.checkpoint_id }))),
            )
        })
        .collect();
    let mut alternative_ids: Vec<String> = Vec::new();
    for column in &history.columns {
        for alternative_id in &column.alternative_ids {
            if !alternative_ids.contains(alternative_id) {
                alternative_ids.push(alternative_id.clone());
            }
        }
    }
    let alternative_items: Vec<UiTreeItemNode> = alternative_ids
        .iter()
        .map(|alternative_id| {
            let count = history.columns.iter().filter(|column| column.alternative_ids.contains(alternative_id)).count();
            UiTreeItemNode {
                icon_id: Some("git-branch".into()),
                menu: None,
                ..tree_item_with_action(
                    builder.item_id("alternative", alternative_id),
                    Label::data(alternative_id.clone()),
                    Some(format!("{count} {}", labels.checkpoints.as_str())),
                    vcs_action("switchAlternative", Some(json!({ "alternativeId": alternative_id }))),
                )
            }
        })
        .collect();
    builder
        .section_or_placeholder("vcs-play-document.checkpoints", Some(labels.document.into()), true, checkpoint_items, labels.no_checkpoints)
        .section("vcs-play-document.alternatives", Some(labels.alternatives.into()), true, alternative_items)
        .interaction_domain(VCS_INTERACTION_HISTORY)
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::vcs::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn document_lists_checkpoints() {
        let mut instance = app();
        let json = render_body(&mut instance, VCS_PLAY_BODY_DOCUMENT);
        assert!(json.contains("vcs-play-document.checkpoint"));
    }

    #[semio_framework_async_macros::async_test]
    async fn vcs_labels_resolve_native_english_by_default() {
        let mut instance = app();
        let json = render_body(&mut instance, VCS_PLAY_BODY_DOCUMENT);
        assert!(json.contains("Alternatives"));
        assert!(json.contains("checkpoints"));
    }
}
//#endregion 🧪️Tests
