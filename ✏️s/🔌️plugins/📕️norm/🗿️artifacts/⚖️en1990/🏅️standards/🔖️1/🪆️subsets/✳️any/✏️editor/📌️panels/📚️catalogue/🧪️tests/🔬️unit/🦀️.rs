use super::*;
use crate::app_surface::CatalogueCell;
use crate::artifact_schema::{psi_for_category, NaEn};
use crate::editor::en1990::unit_tests::context;
use crate::ImportanceClass;
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
    if tables.is_empty() {
        assert!(!json.contains("norm-catalogue.table-"), "empty tables must not invent sections: {json}");
    } else {
        for table in &tables {
            assert!(json.contains(&format!("norm-catalogue.table-{}", table.id)) || json.contains(table.title_en) || json.contains(table.id), "missing table {}: {json}", table.id);
        }
    }
}

#[test]
fn reference_tables_cells_match_psi_and_gamma_i_sources() {
    let tables = reference_tables();
    let en_psi = tables.iter().find(|t| t.id == "table-a1-1-psi-en").expect("EN ψ table");
    let office = en_psi.rows.iter().find(|r| r.id == "office").expect("office row");
    let CatalogueCell::Number { value: psi_0, .. } = &office.cells[1] else {
        panic!("office ψ₀ cell must be Number");
    };
    assert_eq!(*psi_0, psi_for_category(&NaEn, "office").psi_0);

    let gamma = tables.iter().find(|t| t.id == "importance-gamma-i").expect("γ_I table");
    let class_iii = gamma.rows.iter().find(|r| r.id == "III").expect("class III row");
    let CatalogueCell::Number { value: gamma_i, .. } = &class_iii.cells[1] else {
        panic!("class III γ_I cell must be Number");
    };
    assert_eq!(*gamma_i, ImportanceClass::III.gamma_i());
}
