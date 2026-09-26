use super::*;
use crate::app_surface::CatalogueCell;
use crate::artifact_schema::{evaluate_document, geg_anlage2_ht_prime_limits};
use crate::editor::din18599::unit_tests::context;
use semio_framework_plugin::{TreeWindows, ViewModel};

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_catalogue_tab_to_this_body_key() {
    assert_eq!(definition().body_key.as_deref(), Some(BODY_CATALOGUE));
    assert_eq!(definition().id(), FRAMEWORK_PANEL_TAB_CATALOGUE_ID);
}

#[semio_framework_async_macros::async_test]
async fn renders_declared_examples_in_the_catalogue_panel() {
    let mut app = context::app_with_registry().await;
    let rendered = context::render(&mut app, BODY_CATALOGUE).await;
    assert!(rendered.contains("compliant-detached") || rendered.contains("Compliant") || rendered.contains("Konformes") || rendered.to_lowercase().contains("catalogue"), "{rendered}");
    assert!(rendered.contains("noncompliant") || rendered.contains("Non-Compliant") || rendered.contains("Nicht") || rendered.contains("demo") || rendered.contains("Demo"), "{rendered}");
    context::close(&mut app);
}

#[test]
fn renders_reference_tables_with_examples() {
    let view = ViewModel::default();
    let windows = TreeWindows::for_body(&view, BODY_CATALOGUE);
    let node = render(Vec::new(), semio_framework_plugin::Locale::En, "norm.catalogue", &windows).expect("catalogue");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project");
    assert!(json.contains("norm-catalogue.examples") || json.contains("Examples") || json.contains("Beispiele") || json.contains("catalogue"), "{json}");
    let tables = reference_tables();
    assert!(!tables.is_empty(), "reference tables must be published");
    for table in &tables {
        assert!(json.contains(&format!("norm-catalogue.table-{}", table.id)) || json.contains(table.title_en) || json.contains(table.id), "missing table {}: {json}", table.id);
    }
}

#[test]
fn reference_tables_cells_match_evaluate_sources() {
    let tables = reference_tables();
    let ht = tables.iter().find(|t| t.id == "geg-anlage2-ht-prime").expect("H′T table");
    let detached = ht.rows.iter().find(|r| r.id == "detached-an-le-350").expect("detached ≤350 row");
    let CatalogueCell::Number { value: cell_limit, .. } = &detached.cells[1] else {
        panic!("detached H′T cell must be Number");
    };
    assert_eq!(*cell_limit, geg_anlage2_ht_prime_limits::DETACHED_AN_LE_350);

    let doc = crate::subjects::compliant_detached_house();
    let report = evaluate_document(&doc);
    let check = report.checks.iter().find(|c| c.id == "din18599.geg.ht-prime").expect("ht-prime check");
    assert_eq!(check.limit.value, *cell_limit);
}
