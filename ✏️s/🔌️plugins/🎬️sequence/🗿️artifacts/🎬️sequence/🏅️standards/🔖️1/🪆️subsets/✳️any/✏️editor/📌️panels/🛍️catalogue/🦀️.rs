//! 🛍️ Sequence play app panel — the step-kind catalogue, plus per-slot "add to" shortcuts for
//! expanded control-flow steps.

use crate::editor::sequence::terminology::SequenceLabels;
use crate::editor::sequence::{control_slots, is_control_kind};
use crate::editor::sequence::{sequence_action, ui_label};
use crate::SequenceFixture;
use semio_framework_plugin::{tree_item_with_action, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const SEQUENCE_PLAY_BODY_CATALOGUE: &str = "sequence.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(SEQUENCE_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(fixture: &SequenceFixture, labels: &SequenceLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let actions = [("state.set", labels.action_set_state), ("log.print", labels.action_log_print), ("control.if", labels.action_if), ("control.while", labels.action_while), ("math.add", labels.action_add)];
    let mut items = semio_framework_plugin::UiFixedList::default();
    for (kind, label) in actions {
        let args = crate::editor::sequence::ui_value_map([("kind", crate::editor::sequence::ui_value_text(kind)?)])?;
        let item = tree_item_with_action(format!("sequence-play-catalogue.action.{kind}"), label.as_str(), Some(kind.into()), sequence_action("addStep", Some(args))?)?;
        items.try_push(item).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.items", "fixed catalogue admission failed"))?;
    }
    for owner in fixture.steps.iter().filter(|step| is_control_kind(&step.kind)) {
        for slot_name in control_slots(&owner.kind) {
            let args =
                crate::editor::sequence::ui_value_map([("kind", crate::editor::sequence::ui_value_text("log.print")?), ("owner", crate::editor::sequence::ui_value_text(&owner.id)?), ("slotName", crate::editor::sequence::ui_value_text(slot_name)?)])?;
            let item = tree_item_with_action(
                format!("sequence-play-catalogue.slot.{}.{}", owner.id, slot_name),
                format!("{} {} → {slot_name}", labels.add_to.as_str(), owner.id),
                Some(format!("{slot_name} @ {}", owner.id)),
                sequence_action("addStepToSlot", Some(args))?,
            )?;
            items.try_push(item).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.catalogue.items", "fixed catalogue admission failed"))?;
        }
    }
    PanelTreeBuilder::new("sequence-play-catalogue")?.section("sequence-play-catalogue.actions", Some(ui_label(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)?), true, items)?.selected([])?.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
