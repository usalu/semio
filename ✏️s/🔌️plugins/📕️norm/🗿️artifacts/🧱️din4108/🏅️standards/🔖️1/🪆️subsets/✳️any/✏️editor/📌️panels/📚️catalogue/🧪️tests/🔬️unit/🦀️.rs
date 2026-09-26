use super::*;
use crate::editor::din4108::unit_tests::context;
use semio_framework_plugin::{TreeWindows, ViewModel};

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
    let view = ViewModel::default();
    let windows = TreeWindows::for_body(&view, BODY_CATALOGUE);
    let node = render(Vec::new(), semio_framework_plugin::Locale::En, "norm.catalogue", &windows).expect("catalogue");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project");
    assert!(json.contains("norm-catalogue.examples") || json.contains("Examples") || json.contains("Beispiele") || json.contains("catalogue"), "{json}");
    let tables = reference_tables();
    assert!(!tables.is_empty(), "din4108 must publish reference tables");
    for table in &tables {
        assert!(!table.id.is_empty());
        assert!(json.contains(&format!("norm-catalogue.table-{}", table.id)) || json.contains(table.title_en) || json.contains(table.id), "missing table {}: {json}", table.id);
    }
}

#[test]
fn design_lambda_table_matches_part4_const() {
    let tables = reference_tables();
    let table = tables.iter().find(|t| t.id == "din4108-4-design-lambda").expect("λ table");
    assert!(!table.rows.is_empty());
    let eps = table.rows.iter().find(|r| r.id == "eps").expect("eps");
    let cell = eps.cells.iter().find_map(|c| match c {
        crate::app_surface::CatalogueCell::Number { value, .. } => Some(*value),
        _ => None,
    }).expect("number");
    assert!((cell - crate::artifact_schema::part_4::design_lambda("eps").unwrap()).abs() < 1e-12);
}
