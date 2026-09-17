use super::*;
use crate::editor::process3d::unit_tests::context;

/// 🪵️ Both wood machines' capabilities render as tree items (labels present) whether the current
/// stock satisfies their rules or not — `stock_validation_context` now checks real dimensions, so
/// this only asserts presence; the pass/fail split itself is covered by
/// `catalogue_flags_a_violated_max_rule_and_not_a_satisfied_one` below with a fixture this test
/// doesn't have to depend on the shared example document's own stock for.
#[semio_framework_async_macros::async_test]
async fn catalogue_lists_workshop_wood_machines() {
    let mut app = context::app();
    let rendered = context::render(&mut app, PROCESS_3D_PLAY_BODY_CATALOGUE);
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
    let labels = crate::editor::process3d::terminology::process3d_labels(&semio_framework_plugin::ViewModel::default());
    // 🚚️ Read through the retiring PROJECTION, never `serde_json::to_string` on a `BuiltNode`: a built
    // node's `BuiltChildren` only serialises through the retained page transport.
    let node = render(&fixture, "[]", labels, &semio_framework_plugin::TreeWindows::unhosted()).expect("catalogue renders");
    let rendered = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("catalogue projection");
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

//#region 🪟️WindowLaws
use semio_framework_plugin::{TreeWindowRequest, TreeWindows, ViewModel};

/// 🔎️ `(total, offset, row keys)` of one container, read off a projected body.
fn container_of(json: &str, key: &str) -> (u64, u64, Vec<String>) {
    fn walk(node: &serde_json::Value, key: &str) -> Option<serde_json::Value> {
        if node["key"].as_str() == Some(key) {
            return Some(node.clone());
        }
        node["children"].as_array().and_then(|children| children.iter().find_map(|child| walk(child, key)))
    }
    let projection: serde_json::Value = serde_json::from_str(json).expect("catalogue projection json");
    let node = walk(&projection, key).unwrap_or_else(|| panic!("{key} is not in the rendered catalogue: {json}"));
    let window = node["component"]["window"].as_object().unwrap_or_else(|| panic!("{key} stamps no window: {json}"));
    (
        window.get("total").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        window.get("offset").and_then(serde_json::Value::as_u64).unwrap_or_default(),
        node["children"].as_array().cloned().unwrap_or_default().iter().map(|row| row["key"].as_str().unwrap_or_default().to_string()).collect(),
    )
}

/// 🌾️ A workshop an order of magnitude past any viewport: one uncataloged machine carrying 80
/// capabilities, every one of them satisfiable (no rules), so the section's window is what bounds
/// the tree and nothing else.
fn oversized_workshop() -> crate::Process3dSnapshot {
    use crate::{Capability, MeasureRecipe, Workshop, WorkshopMachine};
    let mut fixture = crate::empty_process3d_snapshot();
    fixture.workshop = Workshop {
        machines: vec![WorkshopMachine {
            id: "saw".into(),
            label: "Saw".into(),
            icon_id: "scissors".into(),
            catalog_id: None,
            capabilities: (0..80)
                .map(|index| Capability {
                    id: format!("cut-{index:03}"),
                    label: format!("Cut {index}"),
                    icon_id: "scissors".into(),
                    recipe: MeasureRecipe::DiscCut { diameter: "bladeDiameter".into(), kerf: "kerf".into() },
                    parameters: Vec::new(),
                    rules: Vec::new(),
                })
                .collect(),
        }],
    };
    fixture
}

/// 🚚️ Reads through the retiring PROJECTION, never `serde_json::to_string` on a `BuiltNode`.
fn project(fixture: &crate::Process3dSnapshot, windows: &TreeWindows<'_>) -> String {
    let labels = crate::editor::process3d::terminology::process3d_labels(&ViewModel::default());
    let node = render(fixture, "[]", labels, windows).expect("catalogue renders");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("catalogue projection")
}

fn window_view(node_key: &str, open: Option<bool>, offset: u32, rows: u32) -> ViewModel {
    ViewModel { tree_windows: vec![TreeWindowRequest { body_key: PROCESS_3D_PLAY_BODY_CATALOGUE.into(), node_key: node_key.into(), open, offset, rows }], ..Default::default() }
}

/// ⚖️ LAW (a): the capability section stamps its whole extent, materialises at most its slice, and
/// never summarises the remainder as a `+n` / `…more` continuation row.
#[test]
fn an_oversized_catalogue_stamps_its_full_capability_count() {
    let json = project(&oversized_workshop(), &TreeWindows::unhosted());
    let (total, offset, rows) = container_of(&json, PROCESS_3D_PLAY_CATALOGUE_WORKSHOP);
    assert_eq!(total, 80, "the workshop section stamps every capability: {json}");
    assert_eq!(offset, 0, "a first paint starts at zero");
    assert!(rows.len() <= 80, "the section materialises at most its slice: {json}");
    assert!(!json.contains(".more\""), "a windowed catalogue has no continuation row: {json}");
    assert!(!json.contains(r#""label":"+"#), "a windowed catalogue publishes no `+n` label: {json}");
}

/// ⚖️ LAW (b): a closed section states its extent and materialises nothing — the quick-swap stock
/// section authors itself CLOSED, so it is already the closed case on a first paint.
#[test]
fn a_closed_catalogue_section_stamps_its_total_with_no_rows() {
    let json = project(&oversized_workshop(), &TreeWindows::unhosted());
    let (total, offset, rows) = container_of(&json, PROCESS_3D_PLAY_CATALOGUE_STOCK);
    assert_eq!(total, 4, "the author-closed stock section still states its four kinds: {json}");
    assert_eq!(offset, 0);
    assert!(rows.is_empty(), "an author-closed section materialises nothing: {json}");

    let view = window_view(PROCESS_3D_PLAY_CATALOGUE_WORKSHOP, Some(false), 0, 0);
    let closed = project(&oversized_workshop(), &TreeWindows::for_body(&view, PROCESS_3D_PLAY_BODY_CATALOGUE));
    let (total, _, rows) = container_of(&closed, PROCESS_3D_PLAY_CATALOGUE_WORKSHOP);
    assert_eq!(total, 80, "a host-closed section still states its extent: {closed}");
    assert!(rows.is_empty(), "a host-closed section materialises nothing: {closed}");
}

/// ⚖️ LAW (c): a host window request materialises exactly `[offset, offset + rows)`, keyed by the
/// row's own `{machine}.{capability}` id.
#[test]
fn a_catalogue_window_request_materialises_exactly_its_slice() {
    let view = window_view(PROCESS_3D_PLAY_CATALOGUE_WORKSHOP, Some(true), 40, 6);
    let json = project(&oversized_workshop(), &TreeWindows::for_body(&view, PROCESS_3D_PLAY_BODY_CATALOGUE));
    let (total, offset, rows) = container_of(&json, PROCESS_3D_PLAY_CATALOGUE_WORKSHOP);
    assert_eq!(total, 80);
    assert_eq!(offset, 40, "the stamped offset is the requested one: {json}");
    let expected: Vec<String> = (40..46).map(|index| format!("process3d-catalogue.saw.cut-{index:03}")).collect();
    assert_eq!(rows, expected, "exactly entries [40, 46) are materialised: {json}");
}

/// ⚖️ LAW (d), as it applies to the catalogue: its rows are install/add ACTIONS, not domain pick
/// targets, so the tree declares no `interactionDomain` and every materialised row keeps its own
/// `addStep` binding (the domain-pick form of law (d) is pinned on `📌️panels/🗿️artifact`).
#[test]
fn catalogue_rows_are_action_rows_not_domain_pick_targets() {
    let view = window_view(PROCESS_3D_PLAY_CATALOGUE_WORKSHOP, Some(true), 0, 4);
    let json = project(&oversized_workshop(), &TreeWindows::for_body(&view, PROCESS_3D_PLAY_BODY_CATALOGUE));
    let projection: serde_json::Value = serde_json::from_str(&json).expect("catalogue projection json");
    assert!(projection["component"]["interactionDomain"].as_str().is_none(), "the catalogue declares no interaction domain: {json}");
    assert!(projection["bindings"].as_array().map(|bindings| bindings.is_empty()).unwrap_or(true), "the catalogue carries no tree-level interactionSelect: {json}");
    for section in projection["children"].as_array().cloned().unwrap_or_default() {
        if section["key"].as_str() != Some(PROCESS_3D_PLAY_CATALOGUE_WORKSHOP) {
            continue;
        }
        let rows = section["children"].as_array().cloned().unwrap_or_default();
        assert_eq!(rows.len(), 4, "the requested four rows: {json}");
        for row in rows {
            assert!(row["component"]["granularity"].as_str().is_none(), "an action row declares no pick granularity: {row}");
            assert!(!row["bindings"].as_array().cloned().unwrap_or_default().is_empty(), "an action row keeps its own addStep binding: {row}");
        }
    }
}
//#endregion 🪟️WindowLaws
