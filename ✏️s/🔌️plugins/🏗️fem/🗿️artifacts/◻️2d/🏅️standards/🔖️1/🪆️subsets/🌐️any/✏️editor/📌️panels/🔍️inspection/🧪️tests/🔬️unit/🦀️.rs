use super::*;
use crate::editor::fem2d::commands::set_active_example::SetActiveExample;
use crate::editor::fem2d::terminology::fem2d_labels;
use crate::editor::fem2d::unit_tests::context::{dispatch, fem2d_app, render as render_body};
use crate::editor::fem2d::Fem2dCommand;
use semio_framework_plugin::{ComponentTree, Locale, Terminology, ViewModel};
use store::ArtifactDsl;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("the bundled demo fixture parses")
}

fn selecting(ids: &[&str]) -> Fem2dInteractionSnapshot {
    Fem2dInteractionSnapshot { selected_ids: ids.iter().map(|id| (*id).to_string()).collect(), ..Default::default() }
}

fn panel(doc: &Fem2dSnapshot, ids: &[&str], view_state: &ViewModel) -> String {
    let built = render(doc, &selecting(ids), fem2d_labels(view_state)).expect("fem2d inspector assembly");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(ComponentTree { root: built }).expect("fem2d inspector projection")
}

fn english(doc: &Fem2dSnapshot, ids: &[&str]) -> String {
    panel(doc, ids, &ViewModel::default())
}

/// 🔎️ The projected node carrying `key`, anywhere under the body — the projection keys every node by
/// the id its builder was given.
fn node_at<'a>(node: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
    if node["key"].as_str() == Some(key) {
        return Some(node);
    }
    node["children"].as_array()?.iter().find_map(|child| node_at(child, key))
}

/// 🧩️ The `Component::…` discriminant of the node keyed `key`.
fn component_at(json: &str, key: &str) -> String {
    let tree: serde_json::Value = serde_json::from_str(json).expect("the inspector projection is JSON");
    let node = node_at(&tree, key).unwrap_or_else(|| panic!("no node keyed {key} in {json}"));
    node["component"]["type"].as_str().expect("every projected component names its type").to_owned()
}

/// 🌲️ A tree node, section or item anywhere in the body. The React `Interpreter` maps a
/// `Component::Tree` onto `TreeDataItem`s and DROPS every non-tree-item child, so a form control
/// nested in one reaches the host and is never drawn — this panel renders none.
fn carries_a_tree(json: &str) -> bool {
    json.contains("\"tree\"") || json.contains("\"treeSection\"") || json.contains("\"treeItem\"")
}

#[semio_framework_async_macros::async_test]
async fn a_selected_node_renders_bound_ordinate_inputs_2d() {
    let json = english(&demo(), &["n1"]);
    assert!(json.contains("fem2d-play-inspection.node.x.input"), "{json}");
    assert!(json.contains("fem2d-play-inspection.node.y.input"));
    assert!(json.contains("patchNode"), "both ordinate inputs dispatch the node patch command");
    assert!(json.contains("\"n1\""), "the argument map addresses the selected node");
    assert!(!json.contains("fem2d-play-inspection.summary.nodes"), "a live selection replaces the summary");
    assert_eq!(component_at(&json, "fem2d-play-inspection.node"), "container", "a group is a section container");
    assert!(!carries_a_tree(&json), "{json}");
}

/// 🧾️ The law the tree layout broke. A field row is not "some node with the right id": it has to
/// reach the host as the CONTROL component the renderer knows how to draw, bound to its own patch
/// command. Under the old `PanelTreeBuilder` body every one of these was a `treeItem` the React
/// renderer dropped, and the whole panel was read-only in the browser.
#[semio_framework_async_macros::async_test]
async fn a_node_section_carries_a_number_input_bound_to_patch_node_2d() {
    let json = english(&demo(), &["n1"]);
    let tree: serde_json::Value = serde_json::from_str(&json).expect("the inspector projection is JSON");
    let control = node_at(&tree, "fem2d-play-inspection.node.x.input").unwrap_or_else(|| panic!("{json}"));
    assert_eq!(control["component"]["type"].as_str(), Some("input"), "the X row is an input, not a tree item: {control}");
    assert_eq!(control["component"]["kind"].as_str(), Some("number"), "{control}");
    let bindings = control["bindings"].to_string();
    assert!(bindings.contains("patchNode"), "{bindings}");
    assert!(bindings.contains("\"x\""), "the binding names the field it patches: {bindings}");
    let row = node_at(&tree, "fem2d-play-inspection.node.x").unwrap_or_else(|| panic!("{json}"));
    assert_eq!(row["component"]["type"].as_str(), Some("container"), "the input sits inside its labelled field row: {row}");
}

#[semio_framework_async_macros::async_test]
async fn a_selected_element_offers_its_reference_selects_2d() {
    let json = english(&demo(), &["e3"]);
    for row in ["element.kind", "element.start", "element.end", "element.material", "element.section"] {
        assert!(json.contains(&format!("fem2d-play-inspection.{row}.select")), "{row} missing from {json}");
        assert_eq!(component_at(&json, &format!("fem2d-play-inspection.{row}.select")), "select", "{row} is a real select");
    }
    assert!(json.contains("patchElement"));
    assert!(json.contains("Steel S235"), "a material option reads as its name, not its id");
}

#[semio_framework_async_macros::async_test]
async fn a_selected_material_mixes_inputs_and_a_bounded_slider_2d() {
    let json = english(&demo(), &["timber"]);
    assert!(json.contains("fem2d-play-inspection.material.nu.slider"), "{json}");
    assert!(json.contains("fem2d-play-inspection.material.e.input"));
    assert!(json.contains("patchMaterial"));
    assert_eq!(component_at(&json, "fem2d-play-inspection.material.nu.slider"), "slider");
    assert_eq!(component_at(&json, "fem2d-play-inspection.material.e.input"), "input");
}

#[semio_framework_async_macros::async_test]
async fn a_selected_support_renders_one_toggle_per_planar_dof_2d() {
    let json = english(&demo(), &["s1"]);
    for row in ["support.tx", "support.ty", "support.rz"] {
        assert!(json.contains(&format!("fem2d-play-inspection.{row}.toggle")), "{row} missing from {json}");
        assert_eq!(component_at(&json, &format!("fem2d-play-inspection.{row}.toggle")), "toggle", "{row} is a real toggle");
    }
    assert!(json.contains("patchSupport"));
}

#[semio_framework_async_macros::async_test]
async fn a_selected_region_shows_its_polygon_read_only_2d() {
    let json = english(&demo(), &["r1"]);
    assert!(json.contains("fem2d-play-inspection.region.thickness.slider"), "{json}");
    assert!(json.contains("fem2d-play-inspection.region.outline"), "the polygon is reported, never offered as a text box");
    assert_eq!(component_at(&json, "fem2d-play-inspection.region.outline.value"), "text", "a read-only value is display text, not a control");
    assert!(json.contains("patchRegion"));
}

#[semio_framework_async_macros::async_test]
async fn a_selected_load_names_its_owning_case_2d() {
    let json = english(&demo(), &["l6"]);
    assert!(json.contains("fem2d-play-inspection.load.dof.select"), "{json}");
    assert!(json.contains("fem2d-play-inspection.load.value.input"));
    assert!(json.contains("Live Load"), "a load row states the case it is applied in");
    assert!(json.contains("patchLoad"));
    let area = english(&demo(), &["l5"]);
    assert!(area.contains("fem2d-play-inspection.load.pressure.input"), "{area}");
    assert!(area.contains("Dead Load"));
}

#[semio_framework_async_macros::async_test]
async fn a_selected_load_case_lists_its_loads_as_picks_2d() {
    let json = english(&demo(), &["live"]);
    assert!(json.contains("fem2d-play-inspection.load-case.self-weight.toggle"), "{json}");
    assert!(json.contains("patchLoadCase"));
    assert!(json.contains("interactionSelect"), "a load row hands the load to the framework-owned selection");
    assert!(json.contains("\"l6\"") && json.contains("\"l7\""), "every load of the case is listed");
    assert_eq!(component_at(&json, "l6"), "button", "a pick row is an activatable button, not a tree item");
    assert!(!carries_a_tree(&json), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn a_selected_combination_renders_one_factor_input_per_term_2d() {
    let json = english(&demo(), &["uls"]);
    assert!(json.contains("fem2d-play-inspection.combination.term.dead.input"), "{json}");
    assert!(json.contains("fem2d-play-inspection.combination.term.live.input"));
    assert!(json.contains("term:dead"), "a factor input binds the term field its case names");
    assert!(json.contains("patchCombination"));
    assert_eq!(component_at(&json, "fem2d-play-inspection.combination.term.dead.input"), "input");
    assert!(!json.contains("combination.add-term"), "every case is already superposed, so nothing is left to add");
}

#[semio_framework_async_macros::async_test]
async fn a_combination_missing_a_case_offers_the_add_term_select_2d() {
    let mut doc = demo();
    doc.combinations[0].terms.retain(|term| term.case_id == "dead");
    let json = english(&doc, &["uls"]);
    assert!(json.contains("fem2d-play-inspection.combination.add-term.select"), "{json}");
    assert!(json.contains("addTerm"));
    assert_eq!(component_at(&json, "fem2d-play-inspection.combination.add-term.select"), "select");
}

#[semio_framework_async_macros::async_test]
async fn a_multi_selection_headers_the_count_and_inspects_the_first_2d() {
    let json = english(&demo(), &["n1", "n2", "p8"]);
    assert!(json.contains("3 Selected"), "{json}");
    for id in ["n1", "n2", "p8"] {
        assert!(json.contains(&format!("\"{id}\"")));
    }
    assert!(json.contains("fem2d-play-inspection.node.x.input"), "the fields belong to the first selected id");
}

/// 🎯️ The selected entity's verbs stay one group of activatable buttons: focus for a kind with
/// viewport geometry, delete for every kind.
#[semio_framework_async_macros::async_test]
async fn the_actions_group_binds_focus_and_delete_as_buttons_2d() {
    let json = english(&demo(), &["n1"]);
    assert_eq!(component_at(&json, "fem2d-play-inspection.actions.focus"), "button", "{json}");
    assert_eq!(component_at(&json, "fem2d-play-inspection.actions.delete"), "button");
    assert!(json.contains("focusEntity") && json.contains("removeSelection"), "{json}");
    let material = english(&demo(), &["timber"]);
    assert!(!material.contains("fem2d-play-inspection.actions.focus"), "a material has no viewport geometry to focus: {material}");
    assert_eq!(component_at(&material, "fem2d-play-inspection.actions.delete"), "button");
}

#[semio_framework_async_macros::async_test]
async fn nothing_selected_renders_the_document_summary_2d() {
    let doc = demo();
    let json = english(&doc, &[]);
    assert!(json.contains("fem2d-play-inspection.summary.nodes"), "{json}");
    assert!(json.contains(&doc.nodes.len().to_string()));
    assert!(json.contains(crate::FEM_2D_SCHEMA));
    assert!(json.contains("fem2d-play-inspection.summary.deformation-scale"), "the analysis settings are reported, not edited, here");
    assert!(!json.contains("patchNode"));
    assert!(!carries_a_tree(&json), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn an_id_no_collection_owns_falls_back_to_the_summary_2d() {
    let json = english(&demo(), &["deleted-yesterday"]);
    assert!(json.contains("fem2d-play-inspection.summary.nodes"), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn german_resolves_every_field_label_the_inspector_binds_2d() {
    let view_state = ViewModel { locale: Locale::De, terminology: Terminology::Native, ..Default::default() };
    let doc = demo();
    assert!(panel(&doc, &["timber"], &view_state).contains("Querdehnzahl"));
    assert!(panel(&doc, &["r1"], &view_state).contains("Netzweite"));
    assert!(panel(&doc, &["live"], &view_state).contains("Eigengewicht"));
    assert!(panel(&doc, &[], &view_state).contains("Übersicht"));
}

/// 🖼️ The panel reaches the shell through its body key, on the live app, with the real label
/// resolution — `render` with no request context carries an empty `"fem2d"` domain, which is the
/// summary path.
#[semio_framework_async_macros::async_test]
async fn the_body_key_routes_to_this_panel_on_the_live_app_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetActiveExample(SetActiveExample { example_id: crate::examples::demo::ID.into() })).await;
    let json = render_body(&mut app, BODY_KEY);
    assert!(json.contains("fem2d-play-inspection.summary.nodes"), "{json}");
    assert!(json.contains("12"), "the demo's twelve nodes are counted");
}
