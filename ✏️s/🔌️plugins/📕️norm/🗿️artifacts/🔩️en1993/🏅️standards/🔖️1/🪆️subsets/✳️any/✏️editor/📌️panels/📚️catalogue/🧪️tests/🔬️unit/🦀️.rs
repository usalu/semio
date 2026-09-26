use super::*;
use crate::app_surface::CatalogueCell;
use crate::editor::en1993::unit_tests::context;
use crate::standards::v1::subsets::any::schema::{check_full_steel_structure, AnnexParams, GAMMA_M0_DE, GAMMA_M1_DE};
use crate::En1993Snapshot;

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
    assert!(!tables.is_empty(), "reference_tables must publish normative lookup tables");
    for table in &tables {
        assert!(!table.id.is_empty());
        assert!(!table.title_en.is_empty());
        assert!(!table.title_de.is_empty());
        assert_ne!(table.title_en, table.title_de);
        assert!(json.contains(&format!("norm-catalogue.table-{}", table.id)) || json.contains(table.title_en) || json.contains(table.id), "missing table {}: {json}", table.id);
    }
}

#[test]
fn reference_table_gamma_and_area_match_evaluated_axial_limit() {
    let tables = reference_tables();
    assert!(!tables.is_empty());
    let gamma = tables.iter().find(|t| t.id == "table-en1993-gamma-m").expect("γ_M table");
    let row = gamma.rows.iter().find(|r| r.id == "gammaM1").expect("γ_M1");
    let CatalogueCell::Number { value: gamma_m1_de, .. } = &row.cells[2] else {
        panic!("γ_M1 DE cell must be Number");
    };
    assert_eq!(*gamma_m1_de, GAMMA_M1_DE);
    assert_eq!(*gamma_m1_de, AnnexParams::de().gamma_m1);

    let sections = tables.iter().find(|t| t.id == "table-en1993-heb-sections").expect("section table");
    let heb = sections.rows.iter().find(|r| r.id == "sec-heb240").expect("HEB240");
    let CatalogueCell::Number { value: area_cm2, .. } = &heb.cells[1] else {
        panic!("area cell must be Number");
    };

    let doc = En1993Snapshot::compliant_heb240_frame();
    let section = doc.sections.iter().find(|s| s.id == "sec-heb240").expect("section");
    let material = doc.materials.iter().find(|m| m.id == "mat-s355").expect("material");
    assert!((*area_cm2 * 1.0e-4 - section.area).abs() < 1e-9);
    let expected = section.area * material.fy / GAMMA_M0_DE;
    let report = check_full_steel_structure(&doc);
    let matched = report.checks.iter().any(|c| (c.limit.value - expected).abs() / expected.max(1.0) < 1e-6);
    assert!(matched, "no evaluated limit equals A·fy/γ_M0 from shared consts; checks={:?}", report.checks.iter().map(|c| (c.id.clone(), c.limit.value)).collect::<Vec<_>>());
}
