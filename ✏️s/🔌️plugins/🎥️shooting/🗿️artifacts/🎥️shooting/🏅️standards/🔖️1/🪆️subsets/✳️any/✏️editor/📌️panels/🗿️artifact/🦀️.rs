//! 📄️ Shooting play app panel — the document tree: shots and assets of the current snapshot.

use crate::editor::shooting::terminology::ShootingLabels;
use crate::editor::shooting::SHOOTING_INTERACTION_DOMAIN;
use crate::ShootingSnapshot;
use semio_framework_plugin::{Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const SHOOTING_PLAY_BODY_ARTIFACT: &str = "shooting.play.artifact";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(SHOOTING_PLAY_BODY_ARTIFACT.into()),
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
    let targets = dsl::os_pack::json::to_json_string(&[semio_framework_plugin::InteractionTarget { granularity: "asset".into(), id: asset_id.into() }]);
    let args = crate::editor::shooting::ui_value_map([
        ("domainId", crate::editor::shooting::ui_value_text(SHOOTING_INTERACTION_DOMAIN)?),
        ("merge", crate::editor::shooting::ui_value_text("replace")?),
        ("targets", crate::editor::shooting::ui_value_text(&targets)?),
    ])?;
    crate::editor::shooting::shooting_action("interactionSelect", Some(args))
}

pub fn render(snapshot: &ShootingSnapshot, labels: &ShootingLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    PanelTreeBuilder::new("shooting-play-document")?
        .window_section(windows, "shooting-play-document.shots", Some(crate::editor::shooting::ui_label(labels.shots.as_str())?), true, &snapshot.shots, |shot| {
            let ids = crate::editor::shooting::ui_value_list([crate::editor::shooting::ui_value_text(&shot.id)?])?;
            let args = crate::editor::shooting::ui_value_map([("shotIds", ids)])?;
            crate::editor::shooting::tree_item_with_icon(format!("shooting-shot:{}", shot.id), Label::data(shot.label.clone()), "camera", crate::editor::shooting::shooting_action("setShotSelection", Some(args)))
        })?
        .window_section(windows, "shooting-play-document.assets", Some(crate::editor::shooting::ui_label(labels.assets.as_str())?), true, &snapshot.assets, |asset| {
            crate::editor::shooting::tree_item_with_icon(format!("shooting-asset:{}", asset.id), Label::data(asset.name.clone()), "box", asset_select_action(&asset.id))
        })?
        .build()
}

//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
