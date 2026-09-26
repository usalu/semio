use super::*;
use crate::artifact_schema::{resolve_params, DesignApproach};
use crate::document::AnnexChoice;
use crate::editor::en1997::unit_tests::context;
use crate::app_surface::CatalogueCell;

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
    let node = render(
        Vec::new(),
        semio_framework_plugin::Locale::En,
        "norm.catalogue",
        &semio_framework_plugin::TreeWindows::unhosted(),
    )
    .expect("catalogue");
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project");
    assert!(json.contains("norm-catalogue.examples") || json.contains("Examples") || json.contains("Beispiele") || json.contains("catalogue"), "{json}");
    let tables = reference_tables();
    assert!(!tables.is_empty(), "reference_tables must publish normative tables");
    for table in &tables {
        assert!(!table.id.is_empty());
        assert_ne!(table.title_en, table.title_de, "table {} needs distinct en/de titles", table.id);
        assert!(json.contains(&format!("norm-catalogue.table-{}", table.id)) || json.contains(table.title_en) || json.contains(table.id), "missing table {}: {json}", table.id);
    }
}

#[test]
fn reference_tables_partial_factors_match_resolve_params() {
    let tables = reference_tables();
    let partial = tables.iter().find(|t| t.id == "din1054-partial-factors").expect("partial factors table");
    let row = partial.rows.iter().find(|r| r.id == "de-da2-bsp").expect("DE DA2* BS-P row");
    let expected = resolve_params(DesignApproach::Da2, AnnexChoice::De, "bsP");
    let gamma_g = match &row.cells[1] {
        CatalogueCell::Number { value, .. } => *value,
        other => panic!("expected γ_G number cell, got {other:?}"),
    };
    let gamma_q = match &row.cells[2] {
        CatalogueCell::Number { value, .. } => *value,
        other => panic!("expected γ_Q number cell, got {other:?}"),
    };
    let gamma_r_v = match &row.cells[3] {
        CatalogueCell::Number { value, .. } => *value,
        other => panic!("expected γ_R,v number cell, got {other:?}"),
    };
    let gamma_r_h = match &row.cells[4] {
        CatalogueCell::Number { value, .. } => *value,
        other => panic!("expected γ_R,h number cell, got {other:?}"),
    };
    assert!((gamma_g - expected.gamma_g).abs() < 1e-12);
    assert!((gamma_q - expected.gamma_q).abs() < 1e-12);
    assert!((gamma_r_v - expected.gamma_r_v).abs() < 1e-12);
    assert!((gamma_r_h - expected.gamma_r_h).abs() < 1e-12);
}
