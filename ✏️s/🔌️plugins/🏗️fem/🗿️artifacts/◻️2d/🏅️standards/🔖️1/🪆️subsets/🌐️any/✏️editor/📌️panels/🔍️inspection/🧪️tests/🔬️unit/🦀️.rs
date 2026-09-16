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

#[semio_framework_async_macros::async_test]
async fn a_selected_node_renders_bound_ordinate_inputs_2d() {
    let json = english(&demo(), &["n1"]);
    assert!(json.contains("fem2d-play-inspection.node.x.input"), "{json}");
    assert!(json.contains("fem2d-play-inspection.node.y.input"));
    assert!(json.contains("patchNode"), "both ordinate inputs dispatch the node patch command");
    assert!(json.contains("\"n1\""), "the argument map addresses the selected node");
    assert!(!json.contains("fem2d-play-inspection.summary.nodes"), "a live selection replaces the summary");
}

#[semio_framework_async_macros::async_test]
async fn a_selected_element_offers_its_reference_selects_2d() {
    let json = english(&demo(), &["e3"]);
    for row in ["element.kind", "element.start", "element.end", "element.material", "element.section"] {
        assert!(json.contains(&format!("fem2d-play-inspection.{row}.select")), "{row} missing from {json}");
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
}

#[semio_framework_async_macros::async_test]
async fn a_selected_support_renders_one_toggle_per_planar_dof_2d() {
    let json = english(&demo(), &["s1"]);
    for row in ["support.tx", "support.ty", "support.rz"] {
        assert!(json.contains(&format!("fem2d-play-inspection.{row}.toggle")), "{row} missing from {json}");
    }
    assert!(json.contains("patchSupport"));
}

#[semio_framework_async_macros::async_test]
async fn a_selected_region_shows_its_polygon_read_only_2d() {
    let json = english(&demo(), &["r1"]);
    assert!(json.contains("fem2d-play-inspection.region.thickness.slider"), "{json}");
    assert!(json.contains("fem2d-play-inspection.region.outline"), "the polygon is reported, never offered as a text box");
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
}

#[semio_framework_async_macros::async_test]
async fn a_selected_combination_renders_one_factor_input_per_term_2d() {
    let json = english(&demo(), &["uls"]);
    assert!(json.contains("fem2d-play-inspection.combination.term.dead.input"), "{json}");
    assert!(json.contains("fem2d-play-inspection.combination.term.live.input"));
    assert!(json.contains("term:dead"), "a factor input binds the term field its case names");
    assert!(json.contains("patchCombination"));
    assert!(!json.contains("combination.add-term"), "every case is already superposed, so nothing is left to add");
}

#[semio_framework_async_macros::async_test]
async fn a_combination_missing_a_case_offers_the_add_term_select_2d() {
    let mut doc = demo();
    doc.combinations[0].terms.retain(|term| term.case_id == "dead");
    let json = english(&doc, &["uls"]);
    assert!(json.contains("fem2d-play-inspection.combination.add-term.select"), "{json}");
    assert!(json.contains("addTerm"));
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

#[semio_framework_async_macros::async_test]
async fn nothing_selected_renders_the_document_summary_2d() {
    let doc = demo();
    let json = english(&doc, &[]);
    assert!(json.contains("fem2d-play-inspection.summary.nodes"), "{json}");
    assert!(json.contains(&doc.nodes.len().to_string()));
    assert!(json.contains(crate::FEM_2D_SCHEMA));
    assert!(json.contains("fem2d-play-inspection.summary.deformation-scale"), "the analysis settings are reported, not edited, here");
    assert!(!json.contains("patchNode"));
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
