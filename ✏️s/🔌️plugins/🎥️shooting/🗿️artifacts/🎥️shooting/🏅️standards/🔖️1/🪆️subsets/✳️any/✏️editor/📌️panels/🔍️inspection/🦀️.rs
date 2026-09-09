//! 🔍️ Shooting inspector — edits the selected or active shot through semantic field controls.

use crate::editor::shooting::config::ShootingConfig;
use crate::editor::shooting::terminology::ShootingLabels;
use crate::editor::shooting::{shooting_action, ui_capacity_error, ui_children, ui_label, ui_node, ui_text, ui_value_list, ui_value_map, ui_value_text};
use crate::{ShootingShot, ShootingSnapshot, SHOOTING_DOCUMENT_SCHEMA};
use semio_framework_plugin::{LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_ui_contract::{column, field, input, section, text, BuiltNode, HasBase, InputKind, Trigger};

//#region 🔖️Constants
pub const SHOOTING_PLAY_BODY_INSPECTION: &str = "shooting.play.inspection";
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
fn shot_field(shot: &ShootingShot, name: &'static str, label: &str, value: &str, kind: Option<InputKind>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let id = format!("shooting-play-inspector.shot.{name}");
    let child = if let Some(kind) = kind {
        let args = ui_value_map([("field", ui_value_text(name)?), ("shotIds", ui_value_list([ui_value_text(&shot.id)?])?)])?;
        let (action, args) = shooting_action("patchShots", Some(args))?;
        let builder = input(kind).value(ui_text(value)?);
        let builder = match args {
            Some(args) => builder.try_on_with(Trigger::Change, action, args),
            None => builder.try_on(Trigger::Change, action),
        }
        .map_err(|_| ui_capacity_error())?;
        ui_node(builder, &format!("{id}.input"))?
    } else {
        ui_node(text(ui_label(value)?), &format!("{id}.value"))?
    };
    ui_node(ui_children(field(ui_label(label)?), [child])?, &id)
}

fn shot_inspector_group(shot: &ShootingShot, labels: &ShootingLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let fields = [
        shot_field(shot, "label", labels.field_label.as_str(), &shot.label, Some(InputKind::Text))?,
        shot_field(shot, "format", labels.field_format.as_str(), &shot.format, None)?,
        shot_field(shot, "shape", labels.field_shape.as_str(), &shot.shape, None)?,
        shot_field(shot, "width", labels.field_width.as_str(), &shot.width.to_string(), Some(InputKind::Number))?,
        shot_field(shot, "height", labels.field_height.as_str(), &shot.height.to_string(), Some(InputKind::Number))?,
    ];
    ui_node(ui_children(section(ui_label(labels.shot.as_str())?).default_open(true), fields)?, "shooting-play-inspector.shot")
}

/// 🔍️ Resolves shot configuration selection, then the document's active shot, then a localized summary.
pub fn render(snapshot: &ShootingSnapshot, cfg: &ShootingConfig, labels: &ShootingLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let selected = cfg.selected_shot_ids.first().and_then(|id| snapshot.shots.iter().find(|shot| &shot.id == id));
    let group = if let Some(shot) = selected.or_else(|| crate::standards::v1::subsets::any::schema::active_shot(snapshot)) {
        shot_inspector_group(shot, labels)?
    } else {
        let summary = [
            ui_node(text(ui_label(SHOOTING_DOCUMENT_SCHEMA)?), "shooting-play-inspector.schema")?,
            ui_node(text(ui_label(&format!("{}: {}", labels.shots.as_str(), snapshot.shots.len()))?), "shooting-play-inspector.shots")?,
            ui_node(text(ui_label(&format!("{}: {}", labels.assets.as_str(), snapshot.assets.len()))?), "shooting-play-inspector.assets")?,
        ];
        ui_node(ui_children(section(ui_label(labels.inspection_title.as_str())?).default_open(true), summary)?, "shooting-play-inspector.empty")?
    };
    ui_node(ui_children(column(), [group])?, "shooting-play-inspector")
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
