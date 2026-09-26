use super::*;
use crate::editor::iso16757::unit_tests::context;

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
    assert_eq!(definition().body_key.as_deref(), Some(BODY_CATALOGUE));
    assert_eq!(definition().id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
}

#[semio_framework_async_macros::async_test]
async fn renders_this_standards_catalogue_headline() {
    let mut app = context::app_with_registry().await;
    assert!(context::render(&mut app, BODY_CATALOGUE).await.contains("catalogue"));
    context::close(&mut app);
}

#[test]
fn renders_reference_tables_with_examples() {
    let node = render(Vec::new(), semio_framework_plugin::Locale::En, "norm.catalogue").expect("catalogue");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project");
    assert!(json.contains("norm-catalogue.examples") || json.contains("Examples") || json.contains("Beispiele") || json.contains("catalogue"), "{json}");
    let tables = reference_tables();
    if tables.is_empty() {
        assert!(!json.contains("norm-catalogue.table-"), "empty tables must not invent sections: {json}");
    } else {
        for table in &tables {
            assert!(json.contains(&format!("norm-catalogue.table-{}", table.id)) || json.contains(table.title_en) || json.contains(table.id), "missing table {}: {json}", table.id);
        }
    }
}

#[test]
fn reference_tables_are_populated_from_evaluate_constants() {
    let tables = reference_tables();
    assert!(
        !tables.is_empty(),
        "reference_tables() must expose normative catalogue cells (edition profiles, lifecycle statuses, exchange stages, …) sourced from the same consts evaluate() reads — CORRECTION 14:54"
    );
    for table in &tables {
        assert!(!table.id.is_empty());
        assert!(!table.title_en.is_empty() && !table.title_de.is_empty());
        assert!(!table.rows.is_empty(), "table {} must have at least one row", table.id);
    }
    let clearance = tables
        .iter()
        .find(|t| t.id == "iso16757-2-installation-clearance")
        .expect("installation clearance table");
    let cell = clearance.rows[0].cells.get(1).expect("clearance value cell");
    match cell {
        crate::app_surface::CatalogueCell::Number { value, .. } => {
            assert!(
                (*value - crate::artifact_schema::part_2::INSTALL_CLEARANCE_M).abs() < 1e-12,
                "clearance cell must equal evaluate() INSTALL_CLEARANCE_M limit"
            );
        }
        other => panic!("expected number cell, got {other:?}"),
    }
}

