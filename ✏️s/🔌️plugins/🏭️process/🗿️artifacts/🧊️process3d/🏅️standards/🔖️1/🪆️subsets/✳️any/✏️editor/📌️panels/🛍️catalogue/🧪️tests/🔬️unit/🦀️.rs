
use super::*;
use crate::editor::process3d::testkit;

/// 🪵️ Both wood machines' capabilities render as tree items (labels present) whether the current
/// stock satisfies their rules or not — `stock_validation_context` now checks real dimensions, so
/// this only asserts presence; the pass/fail split itself is covered by
/// `catalogue_flags_a_violated_max_rule_and_not_a_satisfied_one` below with a fixture this test
/// doesn't have to depend on the shared example document's own stock for.
#[semio_framework_async_macros::async_test]
async fn catalogue_lists_workshop_wood_machines() {
    let mut app = testkit::app();
    let rendered = testkit::render(&mut app, PROCESS_3D_PLAY_BODY_CATALOGUE);
    assert!(rendered.contains("Circular Saw"), "expected wood's circular saw in the catalogue: {rendered}");
    assert!(rendered.contains("Table Saw"), "expected wood's table saw in the catalogue: {rendered}");
}

/// 🪚️ Real per-variant dimensions (`stock_validation_context`) must flag a capability whose `Max`
/// rule a `0.2m`-tall stock violates (`maxCutDepth = 0.1`) and leave one it satisfies
/// (`maxCutDepth = 0.5`) alone: the violated item renders `validation_reason`'s "needs stock…"
/// text and no `addStep` action binding; the satisfied one binds `addStep` and carries no reason.
#[semio_framework_async_macros::async_test]
async fn catalogue_flags_a_violated_max_rule_and_not_a_satisfied_one() {
    use crate::{Capability, CapabilityParameter, CapabilityRule, MeasureRecipe, Stock, StockQuantity, WorkingSolid, Workshop, WorkshopMachine};
    let mut fixture = crate::empty_process3d_snapshot();
    fixture.stock_payload = Stock { id: "stock".into(), label: "Stock".into(), solid: WorkingSolid::Box { width: 0.5, depth: 0.5, height: 0.2 }, pose: Default::default() };
    fixture.workshop = Workshop {
        machines: vec![WorkshopMachine {
            id: "saw".into(),
            label: "Saw".into(),
            icon_id: "scissors".into(),
            catalog_id: None,
            capabilities: vec![
                Capability {
                    id: "shallowCrosscut".into(),
                    label: "Shallow Crosscut".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                    parameters: vec![CapabilityParameter { id: "maxCutDepth".into(), label: "Max Cut Depth".into(), value: 0.1 }],
                    rules: vec![CapabilityRule::Max { quantity: StockQuantity::Height, parameter: "maxCutDepth".into(), margin: 0.0 }],
                },
                Capability {
                    id: "deepCrosscut".into(),
                    label: "Deep Crosscut".into(),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                    parameters: vec![CapabilityParameter { id: "maxCutDepth".into(), label: "Max Cut Depth".into(), value: 0.5 }],
                    rules: vec![CapabilityRule::Max { quantity: StockQuantity::Height, parameter: "maxCutDepth".into(), margin: 0.0 }],
                },
            ],
        }],
    };
    let labels = crate::editor::process3d::terminology::process3d_labels(&crate::editor::process3d::config::Process3dConfig::default());
    let node = render(&fixture, "[]", labels).expect("catalogue renders");
    let rendered = serde_json::to_string(&node).expect("render json");
    assert!(rendered.contains("process3d-catalogue.saw.shallowCrosscut"), "expected the violated capability item: {rendered}");
    assert!(rendered.contains("process3d-catalogue.saw.deepCrosscut"), "expected the satisfied capability item: {rendered}");
    assert!(rendered.contains("needs stock height"), "expected the violated rule's reason text: {rendered}");
    assert!(rendered.contains("addStep"), "expected the satisfied capability to still bind addStep: {rendered}");
}

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
    let definition = definition();
    assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
    assert_eq!(definition.body_key.as_deref(), Some(PROCESS_3D_PLAY_BODY_CATALOGUE));
}
