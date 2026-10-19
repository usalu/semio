use semio_s_artifact_fem_2d_editor_panels::editor::fem2d::panels::inspection::*;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_combination::ADD_TERM_FIELD;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::patch_combination::TERM_FIELD_PREFIX;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_entity_kind;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::fem2d_load_owner;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::Fem2dInteractionSnapshot;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::FEM2D_GRANULARITY_COMBINATION;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::FEM2D_GRANULARITY_ELEMENT;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::FEM2D_GRANULARITY_LOAD;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::FEM2D_GRANULARITY_LOAD_CASE;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::FEM2D_GRANULARITY_MATERIAL;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::FEM2D_GRANULARITY_NODE;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::FEM2D_GRANULARITY_REGION;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::FEM2D_GRANULARITY_SECTION;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::FEM2D_GRANULARITY_SUPPORT;
use semio_s_artifact_fem_2d_editor_interaction::editor::fem2d::interaction::FEM2D_INTERACTION_DOMAIN;
use semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::Fem2dLabels;
use semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::fem2d_action;
use semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::ui_label;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::element_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::load_id;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemCombination;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemCombinationTerm;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemDof;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemElement;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoad;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemLoadCase;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemMaterial;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemNode;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemRegion;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemSection;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FemSupport;
use semio_framework_plugin::plugin_app_close_prelude::Buildable;
use semio_framework_plugin::plugin_app_close_prelude::HasBase;
use semio_framework_plugin::plugin_app_close_prelude::HasChildren;
use semio_framework_plugin::plugin_app_close_prelude::InputKind;
use semio_framework_plugin::tree_item;
use semio_framework_plugin::tree_item_desc;
use semio_framework_plugin::tree_item_with_action;
use semio_framework_plugin::ActionId;
use semio_framework_plugin::BuiltNode;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::Trigger;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::UiFixedList;
use semio_framework_plugin::UiMapBuilder;
use semio_framework_plugin::UiText;
use semio_framework_plugin::UiValue;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_ID;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_INSPECTION_LABEL;
use semio_framework_plugin::INTERACTION_SELECT_ACTION_ID;
use semio_framework_ui_contract as ui;
use semio_s_artifact_fem_2d_editor_commands::editor::fem2d::commands::set_active_example::SetActiveExample;
use semio_s_artifact_fem_2d_editor_terminology::editor::fem2d::terminology::fem2d_labels;
use crate::context::dispatch;
use crate::context::fem2d_app;
use crate::context::render as render_body;
use semio_s_artifact_fem_2d_editor::editor::fem2d::Fem2dCommand;
use semio_framework_plugin::ComponentTree;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::Terminology;
use semio_framework_plugin::ViewModel;
use pack::snapshot::ArtifactDsl;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::io::text::snapshot::FEM2D_EXAMPLE_TEXT).expect("the bundled demo fixture parses")
}

fn selecting(ids: &[&str]) -> Fem2dInteractionSnapshot {
    Fem2dInteractionSnapshot { selected_ids: ids.iter().map(|id| (*id).to_string()).collect(), ..Default::default() }
}

fn panel(doc: &Fem2dSnapshot, ids: &[&str], view_state: &ViewModel) -> String {
    let built = render(doc, &selecting(ids), fem2d_labels(view_state), &TreeWindows::unhosted()).expect("fem2d inspector assembly");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(ComponentTree::new(built)).expect("fem2d inspector projection")
}

fn english(doc: &Fem2dSnapshot, ids: &[&str]) -> String {
    panel(doc, ids, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native))
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

/// 🌲️ Every inspector body is a `Component::Tree` of sections and `treeItem` rows.
fn carries_a_tree(json: &str) -> bool {
    json.contains("\"tree\"") || json.contains("\"treeSection\"") || json.contains("\"treeItem\"")
}

#[semio_framework_async_macros::async_test]
async fn a_selected_node_renders_bound_ordinate_inputs_2d() {
    let json = english(&demo(), &["scope:1:4:node2:n1"]);
    assert!(json.contains("fem2d-play-inspection.node.x.input"), "{json}");
    assert!(json.contains("fem2d-play-inspection.node.y.input"));
    assert!(json.contains("patchNode"), "both ordinate inputs dispatch the node patch command");
    assert!(json.contains("\"n1\""), "the argument map addresses the selected node");
    assert!(!json.contains("fem2d-play-inspection.summary.nodes"), "a live selection replaces the summary");
    assert_eq!(component_at(&json, "fem2d-play-inspection.node"), "treeSection", "a group is a tree section");
    assert!(carries_a_tree(&json), "{json}");
}

/// 🧾️ Editable fields are `treeItem` rows whose inline control stays bound to its patch command.
#[semio_framework_async_macros::async_test]
async fn a_node_section_carries_a_number_input_bound_to_patch_node_2d() {
    let json = english(&demo(), &["scope:1:4:node2:n1"]);
    let tree: serde_json::Value = serde_json::from_str(&json).expect("the inspector projection is JSON");
    let control = node_at(&tree, "fem2d-play-inspection.node.x.input").unwrap_or_else(|| panic!("{json}"));
    assert_eq!(control["component"]["type"].as_str(), Some("input"), "the X row is an input, not a tree item: {control}");
    assert_eq!(control["component"]["kind"].as_str(), Some("number"), "{control}");
    let bindings = control["bindings"].to_string();
    assert!(bindings.contains("patchNode"), "{bindings}");
    assert!(bindings.contains("\"x\""), "the binding names the field it patches: {bindings}");
    let row = node_at(&tree, "fem2d-play-inspection.node.x").unwrap_or_else(|| panic!("{json}"));
    assert_eq!(row["component"]["type"].as_str(), Some("treeItem"), "the input sits inside its labelled tree row: {row}");
}

#[semio_framework_async_macros::async_test]
async fn a_selected_element_offers_its_reference_selects_2d() {
    let json = english(&demo(), &["scope:1:7:element2:e3"]);
    for row in ["element.kind", "element.start", "element.end", "element.material", "element.section"] {
        assert!(json.contains(&format!("fem2d-play-inspection.{row}.select")), "{row} missing from {json}");
        assert_eq!(component_at(&json, &format!("fem2d-play-inspection.{row}.select")), "select", "{row} is a real select");
    }
    assert!(json.contains("patchElement"));
    assert!(json.contains("Steel S235"), "a material option reads as its name, not its id");
}

#[semio_framework_async_macros::async_test]
async fn a_selected_material_mixes_inputs_and_a_bounded_slider_2d() {
    let json = english(&demo(), &["scope:1:8:material6:timber"]);
    assert!(json.contains("fem2d-play-inspection.material.nu.slider"), "{json}");
    assert!(json.contains("fem2d-play-inspection.material.e.input"));
    assert!(json.contains("patchMaterial"));
    assert_eq!(component_at(&json, "fem2d-play-inspection.material.nu.slider"), "slider");
    assert_eq!(component_at(&json, "fem2d-play-inspection.material.e.input"), "input");
}

#[semio_framework_async_macros::async_test]
async fn a_selected_support_renders_one_toggle_per_planar_dof_2d() {
    let json = english(&demo(), &["scope:1:7:support2:s1"]);
    for row in ["support.tx", "support.ty", "support.rz"] {
        assert!(json.contains(&format!("fem2d-play-inspection.{row}.toggle")), "{row} missing from {json}");
        assert_eq!(component_at(&json, &format!("fem2d-play-inspection.{row}.toggle")), "toggle", "{row} is a real toggle");
    }
    assert!(json.contains("patchSupport"));
}

#[semio_framework_async_macros::async_test]
async fn a_selected_region_shows_its_polygon_read_only_2d() {
    let json = english(&demo(), &["scope:1:6:region2:r1"]);
    assert!(json.contains("fem2d-play-inspection.region.thickness.slider"), "{json}");
    assert!(json.contains("fem2d-play-inspection.region.outline"), "the polygon is reported, never offered as a text box");
    assert_eq!(component_at(&json, "fem2d-play-inspection.region.outline"), "treeItem", "a read-only value is a tree row with a description");
    assert!(json.contains("patchRegion"));
}

#[semio_framework_async_macros::async_test]
async fn a_selected_load_names_its_owning_case_2d() {
    let json = english(&demo(), &["scope:1:4:load4:live2:l6"]);
    assert!(json.contains("fem2d-play-inspection.load.dof.select"), "{json}");
    assert!(json.contains("fem2d-play-inspection.load.value.input"));
    assert!(json.contains("Live Load"), "a load row states the case it is applied in");
    assert!(json.contains("patchLoad"));
    let area = english(&demo(), &["scope:1:4:load4:dead2:l5"]);
    assert!(area.contains("fem2d-play-inspection.load.pressure.input"), "{area}");
    assert!(area.contains("Dead Load"));
}

#[semio_framework_async_macros::async_test]
async fn a_selected_load_case_lists_its_loads_as_picks_2d() {
    let json = english(&demo(), &["scope:1:8:loadCase4:live"]);
    assert!(json.contains("fem2d-play-inspection.load-case.self-weight.toggle"), "{json}");
    assert!(json.contains("patchLoadCase"));
    assert!(json.contains("interactionSelect"), "a load row hands the load to the framework-owned selection");
    assert!(json.contains("scope:1:4:load4:live2:l6") && json.contains("scope:1:4:load4:live2:l7"), "every load of the case is listed");
    assert_eq!(component_at(&json, "scope:1:4:load4:live2:l6"), "treeItem", "a pick row is an activatable tree item");
    assert!(carries_a_tree(&json), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn a_selected_combination_renders_one_factor_input_per_term_2d() {
    let json = english(&demo(), &["scope:1:11:combination3:uls"]);
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
    let json = english(&doc, &["scope:1:11:combination3:uls"]);
    assert!(json.contains("fem2d-play-inspection.combination.add-term.select"), "{json}");
    assert!(json.contains("addTerm"));
    assert_eq!(component_at(&json, "fem2d-play-inspection.combination.add-term.select"), "select");
}

#[semio_framework_async_macros::async_test]
async fn a_multi_selection_headers_the_count_and_inspects_the_first_2d() {
    let json = english(&demo(), &["scope:1:4:node2:n1", "scope:1:4:node2:n2", "scope:1:4:node2:p8"]);
    assert!(json.contains("3 Selected"), "{json}");
    for id in ["scope:1:4:node2:n1", "scope:1:4:node2:n2", "scope:1:4:node2:p8"] {
        assert!(json.contains(&format!("\"{id}\"")));
    }
    assert!(json.contains("fem2d-play-inspection.node.x.input"), "the fields belong to the first selected id");
}

/// 🎯️ Focus and delete are activatable tree rows in the Actions section.
#[semio_framework_async_macros::async_test]
async fn the_actions_group_binds_focus_and_delete_as_tree_items_2d() {
    let json = english(&demo(), &["scope:1:4:node2:n1"]);
    assert_eq!(component_at(&json, "fem2d-play-inspection.actions.focus"), "treeItem", "{json}");
    assert_eq!(component_at(&json, "fem2d-play-inspection.actions.delete"), "treeItem");
    assert!(json.contains("focusEntity") && json.contains("removeSelection"), "{json}");
    let material = english(&demo(), &["scope:1:8:material6:timber"]);
    assert!(!material.contains("fem2d-play-inspection.actions.focus"), "a material has no viewport geometry to focus: {material}");
    assert_eq!(component_at(&material, "fem2d-play-inspection.actions.delete"), "treeItem");
}

#[semio_framework_async_macros::async_test]
async fn nothing_selected_renders_the_document_summary_2d() {
    let doc = demo();
    let json = english(&doc, &[]);
    assert!(json.contains("fem2d-play-inspection.summary.nodes"), "{json}");
    assert!(json.contains(&doc.nodes.len().to_string()));
    assert!(json.contains(semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::structure::FEM_2D_SCHEMA));
    assert!(json.contains("fem2d-play-inspection.summary.deformation-scale"), "the analysis settings are reported, not edited, here");
    assert!(!json.contains("patchNode"));
    assert!(carries_a_tree(&json), "the empty-selection summary is still a tree panel: {json}");
}

#[semio_framework_async_macros::async_test]
async fn an_id_no_collection_owns_falls_back_to_the_summary_2d() {
    let json = english(&demo(), &["deleted-yesterday"]);
    assert!(json.contains("fem2d-play-inspection.summary.nodes"), "{json}");
}

#[semio_framework_async_macros::async_test]
async fn german_resolves_every_field_label_the_inspector_binds_2d() {
    let view_state = ViewModel { locale: Locale::De, terminology: Terminology::Native, ..ViewModel::new(Locale::De, Terminology::Native) };
    let doc = demo();
    assert!(panel(&doc, &["scope:1:8:material6:timber"], &view_state).contains("Querdehnzahl"));
    assert!(panel(&doc, &["scope:1:6:region2:r1"], &view_state).contains("Netzweite"));
    assert!(panel(&doc, &["scope:1:8:loadCase4:live"], &view_state).contains("Eigengewicht"));
    assert!(panel(&doc, &[], &view_state).contains("Übersicht"));
}

/// 🖼️ The panel reaches the shell through its body key, on the live app, with the real label
/// resolution — `render` with no request context carries an empty `"fem2d"` domain, which is the
/// summary path.
#[semio_framework_async_macros::async_test]
async fn the_body_key_routes_to_this_panel_on_the_live_app_2d() {
    let mut app = fem2d_app();
    dispatch(&mut app, Fem2dCommand::SetActiveExample(SetActiveExample { example_id: semio_s_artifact_fem_2d_io::examples::demo::ID.into() })).await;
    let json = render_body(&mut app, BODY_KEY);
    assert!(json.contains("fem2d-play-inspection.summary.nodes"), "{json}");
    assert!(json.contains("12"), "the demo's twelve nodes are counted");
}
