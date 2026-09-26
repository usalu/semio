use super::*;
use crate::app_surface::CatalogueCell;
use crate::artifact_schema::{part_1_1, part_en1990};
use crate::document::AnnexChoice;
use crate::editor::en1994::unit_tests::context;
use crate::CompositeBeam;

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
    assert!(!tables.is_empty(), "reference_tables must publish normative rows");
    for table in &tables {
        assert!(!table.id.is_empty());
        assert!(!table.title_en.is_empty());
        assert!(!table.title_de.is_empty());
        assert_ne!(table.title_en, table.title_de, "en/de titles must differ for {}", table.id);
        assert!(json.contains(&format!("norm-catalogue.table-{}", table.id)) || json.contains(table.title_en) || json.contains(table.id), "missing table {}: {json}", table.id);
    }
}

#[test]
fn reference_tables_cells_match_psi_and_gamma_i_sources() {
    let tables = reference_tables();
    let gamma = tables.iter().find(|t| t.id == "en1990-partial-factors-gamma").expect("γ table");
    let de = gamma.rows.iter().find(|r| r.id == "de").expect("de row");
    let CatalogueCell::Number { value: gamma_g, .. } = &de.cells[1] else {
        panic!("γ_G cell must be Number");
    };
    assert_eq!(*gamma_g, part_en1990::gamma_g(AnnexChoice::De));
    assert_eq!(*gamma_g, part_en1990::GAMMA_G_DE);

    let psi = tables.iter().find(|t| t.id == "en1990-psi-factors").expect("ψ table");
    let office = psi.rows.iter().find(|r| r.id == "imposed-B").expect("imposed-B");
    let CatalogueCell::Number { value: psi_0, .. } = &office.cells[1] else {
        panic!("ψ₀ cell must be Number");
    };
    assert_eq!(*psi_0, part_en1990::psi_factors("imposed", "B").0);

    let stud = tables.iter().find(|t| t.id == "en1994-stud-spacing-limits").expect("stud table");
    let s_max_row = stud.rows.iter().find(|r| r.id == "s-max").expect("s-max");
    let CatalogueCell::Number { value: s_max_cell, .. } = &s_max_row.cells[1] else {
        panic!("s_max cell must be Number");
    };
    let beam = CompositeBeam::default_placeholder();
    let (_s_min, s_max) = part_1_1::stud_spacing_limits_m(&beam);
    assert!((s_max_cell - s_max).abs() < 1e-12, "catalogue s_max {s_max_cell} != evaluate {s_max}");
}
