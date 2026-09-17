use super::*;
use crate::editor::fem3d::commands::set_active_example::SetActiveExample;
use crate::editor::fem3d::terminology::fem3d_labels;
use crate::editor::fem3d::unit_tests::context::{dispatch, fem3d_app, render as render_body};
use crate::editor::fem3d::Fem3dCommand;
use semio_framework_plugin::{ComponentTree, Locale, Terminology, TreeWindows, ViewModel};

fn demo() -> Fem3dSnapshot {
    crate::standards::v1::subsets::any::schema::snapshot::text::fem3d_boot_snapshot()
}

fn selecting(ids: &[&str]) -> Fem3dInteractionSnapshot {
    Fem3dInteractionSnapshot { selected_ids: ids.iter().map(|id| (*id).to_string()).collect(), ..Default::default() }
}

fn panel(doc: &Fem3dSnapshot, ids: &[&str], view_state: &ViewModel) -> String {
    let built = render(doc, &selecting(ids), fem3d_labels(view_state), &TreeWindows::unhosted()).expect("fem3d inspector assembly");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(ComponentTree { root: built }).expect("fem3d inspector projection")
}

fn english(doc: &Fem3dSnapshot, ids: &[&str]) -> String {
    panel(doc, ids, &ViewModel::default())
}

/// 🔎️ The projected node carrying `key`, anywhere under the body.
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

/// 🌲️ Every inspector body is a `Component::Tree` of sections and `treeItem` rows.
fn carries_a_tree(json: &str) -> bool {
    json.contains("\"tree\"") || json.contains("\"treeSection\"") || json.contains("\"treeItem\"")
}

#[semio_framework_async_macros::async_test]
async fn a_selected_node_renders_bound_ordinate_inputs_3d() {
    let json = english(&demo(), &["n20_l1"]);
    for axis in ["x", "y", "z"] {
        let key = format!("fem3d-play-inspection.node.{axis}.input");
        assert_eq!(component_at(&json, &key), "input", "{axis} is a real input");
    }
    assert!(json.contains("patchNode"), "the ordinate inputs dispatch the node patch command");
    assert!(json.contains("\"n20_l1\""), "the argument map addresses the selected node");
    assert!(!json.contains("fem3d-play-inspection.summary.nodes"), "a live selection replaces the summary");
    assert_eq!(component_at(&json, "fem3d-play-inspection.node"), "treeSection", "a group is a tree section");
    assert!(carries_a_tree(&json), "{json}");
}

/// 🧾️ Editable fields are `treeItem` rows whose inline control stays bound to its patch command.
#[semio_framework_async_macros::async_test]
async fn a_node_section_carries_a_number_input_bound_to_patch_node_3d() {
    let json = english(&demo(), &["n20_l1"]);
    let tree: serde_json::Value = serde_json::from_str(&json).expect("the inspector projection is JSON");
    let control = node_at(&tree, "fem3d-play-inspection.node.x.input").unwrap_or_else(|| panic!("{json}"));
    assert_eq!(control["component"]["type"].as_str(), Some("input"), "the X row is an input, not a tree item: {control}");
    assert_eq!(control["component"]["kind"].as_str(), Some("number"), "{control}");
    let bindings = control["bindings"].to_string();
    assert!(bindings.contains("patchNode"), "{bindings}");
    assert!(bindings.contains("\"x\""), "the binding names the field it patches: {bindings}");
    let row = node_at(&tree, "fem3d-play-inspection.node.x").unwrap_or_else(|| panic!("{json}"));
    assert_eq!(row["component"]["type"].as_str(), Some("treeItem"), "the input sits inside its labelled tree row: {row}");
}

#[semio_framework_async_macros::async_test]
async fn a_selected_frame_offers_its_reference_selects_and_roll_3d() {
    let json = english(&demo(), &["e1"]);
    for row in ["element.kind", "element.start", "element.end", "element.material", "element.section"] {
        assert_eq!(component_at(&json, &format!("fem3d-play-inspection.{row}.select")), "select", "{row} is a real select");
    }
    assert_eq!(component_at(&json, "fem3d-play-inspection.element.roll.input"), "input", "a frame exposes its roll");
    assert!(json.contains("patchElement"));
    assert!(json.contains("Steel S235"), "a material option reads as its name, not its id");
}

#[semio_framework_async_macros::async_test]
async fn a_selected_material_mixes_inputs_and_a_bounded_slider_3d() {
    let json = english(&demo(), &["concrete"]);
    assert_eq!(component_at(&json, "fem3d-play-inspection.material.nu.slider"), "slider");
    assert_eq!(component_at(&json, "fem3d-play-inspection.material.e.input"), "input");
    assert_eq!(component_at(&json, "fem3d-play-inspection.material.g.input"), "input");
    assert!(json.contains("patchMaterial"));
}

#[semio_framework_async_macros::async_test]
async fn a_selected_support_renders_one_toggle_per_dof_3d() {
    let json = english(&demo(), &["s_00"]);
    for row in ["support.tx", "support.ty", "support.tz", "support.rx", "support.ry", "support.rz"] {
        assert_eq!(component_at(&json, &format!("fem3d-play-inspection.{row}.toggle")), "toggle", "{row} is a real toggle");
    }
    assert!(json.contains("patchSupport"));
}

#[semio_framework_async_macros::async_test]
async fn a_selected_solid_edits_its_extrusion_and_shows_its_polygon_read_only_3d() {
    let json = english(&demo(), &["sol1"]);
    assert_eq!(component_at(&json, "fem3d-play-inspection.solid.axis.select"), "select", "{json}");
    assert_eq!(component_at(&json, "fem3d-play-inspection.solid.height.input"), "input");
    assert_eq!(component_at(&json, "fem3d-play-inspection.solid.mesh-size.input"), "input");
    assert_eq!(component_at(&json, "fem3d-play-inspection.solid.outline"), "treeItem", "a read-only value is a tree row with a description");
    assert!(json.contains("patchSolid"));
}

#[semio_framework_async_macros::async_test]
async fn a_selected_load_names_its_owning_case_3d() {
    let json = english(&demo(), &["l2"]);
    assert_eq!(component_at(&json, "fem3d-play-inspection.load.dof.select"), "select");
    assert_eq!(component_at(&json, "fem3d-play-inspection.load.value.input"), "input");
    assert!(json.contains("Live Load"), "a load row states the case it is applied in");
    assert!(json.contains("patchLoad"));
    let area = english(&demo(), &["l1"]);
    assert_eq!(component_at(&area, "fem3d-play-inspection.load.pressure.input"), "input");
    assert_eq!(component_at(&area, "fem3d-play-inspection.load.solid.select"), "select");
    assert!(area.contains("Dead Load"));
}

#[semio_framework_async_macros::async_test]
async fn a_selected_load_case_lists_its_loads_as_picks_3d() {
    let json = english(&demo(), &["live"]);
    assert_eq!(component_at(&json, "fem3d-play-inspection.load-case.self-weight.toggle"), "toggle");
    assert!(json.contains("patchLoadCase"));
    assert!(json.contains("interactionSelect"), "a load row hands the load to the framework-owned selection");
    assert_eq!(component_at(&json, "q_l_spine"), "treeItem", "a pick row is an activatable tree item");
    assert_eq!(component_at(&json, "q_l_b0"), "treeItem");
    assert!(carries_a_tree(&json), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn a_selected_combination_renders_one_factor_input_per_term_3d() {
    let json = english(&demo(), &["uls"]);
    assert_eq!(component_at(&json, "fem3d-play-inspection.combination.term.dead.input"), "input");
    assert_eq!(component_at(&json, "fem3d-play-inspection.combination.term.live.input"), "input");
    assert!(json.contains("term:dead"), "a factor input binds the term field its case names");
    assert!(json.contains("patchCombination"));
    assert!(!json.contains("combination.add-term"), "every case is already superposed, so nothing is left to add");
    assert_eq!(component_at(&json, "fem3d-play-inspection.combination.remove-term.select"), "select", "a term can be dropped");
    let mut doc = demo();
    doc.combinations[0].terms.remove("live");
    let json = english(&doc, &["uls"]);
    assert_eq!(component_at(&json, "fem3d-play-inspection.combination.add-term.select"), "select");
    assert!(json.contains("addTerm"));
}

#[semio_framework_async_macros::async_test]
async fn a_multi_selection_headers_the_count_and_inspects_the_first_3d() {
    let json = english(&demo(), &["lc1b", "lc2b", "l_col1"]);
    assert!(json.contains("3 Selected"), "{json}");
    assert!(json.contains("fem3d-play-inspection.node.x.input"), "the fields belong to the first selected id");
}

/// 🎯️ Focus and delete are activatable tree rows in the Actions section.
#[semio_framework_async_macros::async_test]
async fn the_actions_group_binds_focus_and_delete_as_tree_items_3d() {
    let json = english(&demo(), &["sol1"]);
    assert_eq!(component_at(&json, "fem3d-play-inspection.actions.focus"), "treeItem", "{json}");
    assert_eq!(component_at(&json, "fem3d-play-inspection.actions.delete"), "treeItem");
    assert!(json.contains("focusEntity") && json.contains("removeSelection"), "{json}");
    let material = english(&demo(), &["steel"]);
    assert!(!material.contains("fem3d-play-inspection.actions.focus"), "a material has no viewport geometry to focus: {material}");
    assert_eq!(component_at(&material, "fem3d-play-inspection.actions.delete"), "treeItem");
}

#[semio_framework_async_macros::async_test]
async fn nothing_selected_renders_the_document_summary_3d() {
    let doc = demo();
    let json = english(&doc, &[]);
    assert!(json.contains("fem3d-play-inspection.summary.nodes"), "{json}");
    assert!(json.contains("fem3d-play-inspection.summary.solids"));
    assert!(json.contains(crate::FEM_3D_SCHEMA));
    assert!(json.contains("fem3d-play-inspection.summary.deformation-scale"), "the analysis settings are reported, not edited, here");
    assert!(!json.contains("patchNode"));
    assert!(carries_a_tree(&json), "the empty-selection summary is still a tree panel: {json}");
    assert!(english(&doc, &["deleted-yesterday"]).contains("fem3d-play-inspection.summary.nodes"), "an unowned id falls back to the summary");
}

#[semio_framework_async_macros::async_test]
async fn german_resolves_every_field_label_the_inspector_binds_3d() {
    let view_state = ViewModel { locale: Locale::De, terminology: Terminology::Native, ..Default::default() };
    let doc = demo();
    assert!(panel(&doc, &["concrete"], &view_state).contains("Querdehnzahl"));
    assert!(panel(&doc, &["sol1"], &view_state).contains("Netzweite"));
    assert!(panel(&doc, &["live"], &view_state).contains("Eigengewicht"));
    assert!(panel(&doc, &[], &view_state).contains("Übersicht"));
}

/// 🖼️ The panel reaches the shell through its body key, on the live app — `render` with no request
/// context carries an empty `"fem3d"` domain, which is the summary path.
#[semio_framework_async_macros::async_test]
async fn the_body_key_routes_to_this_panel_on_the_live_app_3d() {
    let mut app = fem3d_app();
    dispatch(&mut app, Fem3dCommand::SetActiveExample(SetActiveExample { example_id: crate::examples::demo::ID.into() })).await;
    let json = render_body(&mut app, BODY_KEY);
    assert!(json.contains("fem3d-play-inspection.summary.nodes"), "{json}");
    assert!(json.contains("16"), "the demo's sixteen nodes are counted");
}
