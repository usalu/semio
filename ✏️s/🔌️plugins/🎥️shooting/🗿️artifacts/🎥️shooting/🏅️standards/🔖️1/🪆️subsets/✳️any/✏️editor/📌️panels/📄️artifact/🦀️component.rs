//! 📄️ Shooting play app panel — the document tree: shots and assets of the current snapshot.

use crate::artifacts::shooting::ShootingSnapshot;
use crate::editor::shooting::terminology::ShootingLabels;
use crate::editor::shooting::SHOOTING_INTERACTION_DOMAIN;
use semio_framework_plugin::{Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const SHOOTING_PLAY_BODY_DOCUMENT: &str = "shooting.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(SHOOTING_PLAY_BODY_DOCUMENT.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ Builds an `interactionSelect` dispatch for one `"assets"`-domain target — replaces the deleted
/// `setSelection` action's asset half. This tree mixes shot AND asset rows under namespaced ids
/// (`"shooting-shot:…"`/`"shooting-asset:…"`), not the domain's raw ids, so it stays un-bound to
/// `.interaction_domain(...)?` (matches `cad`'s `document_tree_selected_ids` precedent) — the asset row's
/// click action is built manually instead, one target per click (`"replace"` merge, matching the old
/// `setSelection` row-click semantics).
fn asset_select_action(asset_id: &str) -> semio_framework_plugin::UiAssemblyResult<(semio_framework_plugin::ActionId, Option<semio_framework_plugin::UiValue>)> {
    let targets = serde_json::to_string(&[semio_framework_plugin::InteractionTarget { granularity: "asset".into(), id: asset_id.into() }])
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.action.targets", "selection target encoding failed"))?;
    let args = crate::editor::shooting::ui_value_map([
        ("domainId", crate::editor::shooting::ui_value_text(SHOOTING_INTERACTION_DOMAIN)?),
        ("merge", crate::editor::shooting::ui_value_text("replace")?),
        ("targets", crate::editor::shooting::ui_value_text(&targets)?),
    ])?;
    crate::editor::shooting::shooting_action("interactionSelect", Some(args))
}

pub async fn render(snapshot: &ShootingSnapshot, labels: &ShootingLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut shot_items = semio_framework_plugin::UiFixedList::default();
    for shot in &snapshot.shots {
        let ids = crate::editor::shooting::ui_value_list([crate::editor::shooting::ui_value_text(&shot.id)?])?;
        let args = crate::editor::shooting::ui_value_map([("shotIds", ids)])?;
        let item = crate::editor::shooting::tree_item_with_icon(
            format!("shooting-shot:{}", shot.id),
            Label::data(shot.label.clone()),
            "camera",
            crate::editor::shooting::shooting_action("setShotSelection", Some(args)),
        )?;
        shot_items.try_push(item).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.document.shots", "fixed shot list admission failed"))?;
    }
    let mut asset_items = semio_framework_plugin::UiFixedList::default();
    for asset in &snapshot.assets {
        let item = crate::editor::shooting::tree_item_with_icon(format!("shooting-asset:{}", asset.id), Label::data(asset.name.clone()), "box", asset_select_action(&asset.id))?;
        asset_items.try_push(item).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.document.assets", "fixed asset list admission failed"))?;
    }
    PanelTreeBuilder::new("shooting-play-document")?.section("shooting-play-document.shots", Some(labels.shots.into()), true, shot_items)?.section("shooting-play-document.assets", Some(labels.assets.into()), true, asset_items)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::shooting::testkit::{render as render_body, shooting_app};

    #[semio_framework_async_macros::async_test]
    async fn document_lists_shots_and_assets() {
        let mut app = shooting_app();
        let json = render_body(&mut app, SHOOTING_PLAY_BODY_DOCUMENT);
        assert!(json.contains("Overview Svg"));
        assert!(json.contains("Base"));
    }
}
//#endregion 🧪️Tests
