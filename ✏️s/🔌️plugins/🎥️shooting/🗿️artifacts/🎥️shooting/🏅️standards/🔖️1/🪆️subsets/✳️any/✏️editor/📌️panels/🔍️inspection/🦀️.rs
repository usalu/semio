//! 🔍️ Shooting inspector — edits the selected or active shot through semantic field controls.

use crate::editor::shooting::config::ShootingConfig;
use crate::editor::shooting::terminology::ShootingLabels;
use crate::editor::shooting::{shooting_action, ui_capacity_error, ui_label, ui_text, ui_value_list, ui_value_map, ui_value_text};
use semio_framework_plugin::ui_node_list;
use crate::{ShootingShot, ShootingSnapshot, SHOOTING_DOCUMENT_SCHEMA};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, HasChildren, InputKind, Trigger};
use semio_framework_plugin::{tree_item, tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_ui_contract::{input, BuiltNode};

//#region 🔖️Constants
pub const SHOOTING_PLAY_BODY_INSPECTION: &str = "shooting.play.inspection";
const ROOT: &str = "shooting-play-inspector";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(SHOOTING_PLAY_BODY_INSPECTION.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn control_row(row_id: &str, label: &str, control: BuiltNode) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    semio_framework_ui_contract::tree_item(ui_label(label)?)
        .try_id(row_id)
        .map_err(|_| ui_capacity_error())?
        .try_child(control)
        .map_err(|_| ui_capacity_error())?
        .try_build()
        .map_err(|_| ui_capacity_error())
}

fn shot_field(shot: &ShootingShot, name: &'static str, label: &str, value: &str, kind: Option<InputKind>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let id = format!("{ROOT}.shot.{name}");
    if let Some(kind) = kind {
        let args = ui_value_map([("field", ui_value_text(name)?), ("shotIds", ui_value_list([ui_value_text(&shot.id)?])?)])?;
        let (action, args) = shooting_action("patchShots", Some(args))?;
        let builder = input(kind).value(ui_text(value)?);
        let builder = match args {
            Some(args) => builder.try_on_with(Trigger::Change, action, args),
            None => builder.try_on(Trigger::Change, action),
        }
        .map_err(|_| ui_capacity_error())?;
        let control = builder
            .try_id(format!("{id}.input"))
            .map_err(|_| ui_capacity_error())?
            .try_build()
            .map_err(|_| ui_capacity_error())?;
        return control_row(&id, label, control);
    }
    tree_item_desc(&id, ui_label(label)?, Some(value.to_string()))
}

fn shot_inspector_group(shot: &ShootingShot, labels: &ShootingLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let fields = ui_node_list([
        shot_field(shot, "label", labels.field_label.as_str(), &shot.label, Some(InputKind::Text)),
        shot_field(shot, "format", labels.field_format.as_str(), &shot.format, None),
        shot_field(shot, "shape", labels.field_shape.as_str(), &shot.shape, None),
        shot_field(shot, "width", labels.field_width.as_str(), &shot.width.to_string(), Some(InputKind::Number)),
        shot_field(shot, "height", labels.field_height.as_str(), &shot.height.to_string(), Some(InputKind::Number)),
    ])?;
    PanelTreeBuilder::new(ROOT)?.section(format!("{ROOT}.shot"), Some(ui_label(labels.shot.as_str())?), true, fields)?.build()
}

/// 🔍️ Resolves shot configuration selection, then the document's active shot, then a localized summary.
pub fn render(snapshot: &ShootingSnapshot, cfg: &ShootingConfig, labels: &ShootingLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let selected = cfg.selected_shot_ids.first().and_then(|id| snapshot.shots.iter().find(|shot| &shot.id == id));
    if let Some(shot) = selected.or_else(|| crate::standards::v1::subsets::any::schema::active_shot(snapshot)) {
        return shot_inspector_group(shot, labels);
    }
    let summary = ui_node_list([
        tree_item(format!("{ROOT}.schema"), SHOOTING_DOCUMENT_SCHEMA),
        tree_item(format!("{ROOT}.shots"), format!("{}: {}", labels.shots.as_str(), snapshot.shots.len())),
        tree_item(format!("{ROOT}.assets"), format!("{}: {}", labels.assets.as_str(), snapshot.assets.len())),
    ])?;
    PanelTreeBuilder::new(ROOT)?.section(format!("{ROOT}.empty"), Some(ui_label(labels.inspection_title.as_str())?), true, summary)?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🔬️semantic-contract/🦀️.rs"]
mod semantic_contract;
