//! 🔍️ Sequence play app panel — inspection: the selected step's kind and params.

use crate::editor::sequence::terminology::SequenceLabels;
use crate::editor::sequence::ui_label;
use crate::{SequenceFixture, SequenceStep};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase};
use semio_framework_plugin::{BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiAssemblyResult, UiFixedList, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};
use semio_framework_ui_contract as ui;

//#region 🔖️Constants
pub const SEQUENCE_PLAY_BODY_INSPECTOR: &str = "sequence.play.inspection";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(SEQUENCE_PLAY_BODY_INSPECTOR.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(fixture: &SequenceFixture, selected: &[String], labels: &SequenceLabels) -> UiAssemblyResult<BuiltNode> {
    let steps: Vec<&SequenceStep> = selected.iter().filter_map(|id| fixture.steps.iter().find(|step| &step.id == id)).collect();
    let mut children = UiFixedList::default();
    let (section_id, heading) = if let Some(step) = steps.first() {
        let mut fields = UiFixedList::<(&str, String)>::default();
        if steps.len() == 1 {
            fields.try_push((labels.id.as_str(), step.id.clone())).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector id admission failed"))?;
        }
        fields.try_push((labels.kind.as_str(), step.kind.clone())).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector kind admission failed"))?;
        fields.try_push((labels.params.as_str(), dsl::os_pack::to_json_string(&step.params))).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector parameters admission failed"))?;
        for (index, (label, value)) in fields.into_iter().enumerate() {
            let field = ui::text(ui_label(format!("{label}: {value}"))?)
                .try_id(format!("sequence-play-inspector.field.{index}"))
                .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector field id admission failed"))?
                .try_build()
                .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector field admission failed"))?;
            children.try_push(field).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector child admission failed"))?;
        }
        ("sequence-play-inspector.step", labels.step.as_str())
    } else {
        let text = if selected.is_empty() { labels.select_prompt.as_str() } else { labels.step_not_found.as_str() };
        let child = ui::text(ui_label(text)?)
            .try_id("sequence-play-inspector.message")
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector message id admission failed"))?
            .try_build()
            .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector message admission failed"))?;
        children.try_push(child).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "sequence inspector message child admission failed"))?;
        (if selected.is_empty() { "sequence-play-inspector.empty" } else { "sequence-play-inspector.missing" }, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)
    };
    PanelTreeBuilder::new("sequence-play-inspector")?.section(section_id, Some(ui_label(heading)?), true, children)?.build()
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
