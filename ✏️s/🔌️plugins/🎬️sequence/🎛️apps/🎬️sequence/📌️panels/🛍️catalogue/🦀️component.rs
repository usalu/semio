//! 🛍️ Sequence play app panel — the step-kind catalogue, plus per-slot "add to" shortcuts for
//! expanded control-flow steps.

use crate::apps::sequence::sequence_action;
use crate::apps::sequence::terminology::SequenceLabels;
use crate::apps::sequence::{control_slots, is_control_kind};
use crate::artifacts::sequence::SequenceSnapshot;
use semio_framework_plugin::{tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use serde_json::json;

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
pub fn render(fixture: &SequenceSnapshot, labels: &SequenceLabels) -> UiNode {
    let actions = [("state.set", labels.action_set_state), ("log.print", labels.action_log_print), ("control.if", labels.action_if), ("control.while", labels.action_while), ("math.add", labels.action_add)];
    let mut items: Vec<UiTreeItemNode> = actions.iter().map(|(kind, label)| tree_item_with_action(format!("sequence-play-catalogue.action.{kind}"), *label, Some((*kind).into()), sequence_action("addStep", Some(json!({ "kind": kind }))))).collect();
    for owner in fixture.steps.iter().filter(|step| is_control_kind(&step.kind)) {
        for slot_name in control_slots(&owner.kind) {
            items.push(tree_item_with_action(
                format!("sequence-play-catalogue.slot.{}.{}", owner.id, slot_name),
                Label::data(format!("{} {} → {slot_name}", labels.add_to.as_str(), owner.id)),
                Some(format!("{slot_name} @ {}", owner.id)),
                sequence_action(
                    "addStepToSlot",
                    Some(json!({
                        "kind": "log.print",
                        "owner": owner.id,
                        "slotName": slot_name,
                    })),
                ),
            ));
        }
    }
    PanelTreeBuilder::new("sequence-play-catalogue").section("sequence-play-catalogue.actions", Some(Label::data(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL)), true, items).selected(vec![]).build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::sequence::testkit::{new_app, render as render_body};

    #[test]
    fn catalogue_lists_step_kind_actions() {
        let mut app = new_app();
        assert!(render_body(&mut app, SEQUENCE_PLAY_BODY_CATALOGUE).contains("sequence-play-catalogue.action.log.print"));
    }
}
//#endregion 🧪️Tests
