use super::*;
use crate::app_surface::CatalogueCell;
use crate::editor::en1998::unit_tests::context;
use crate::standards::v1::subsets::any::schema::na_de::{GroundCombo, SeismicZone};

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
fn reference_tables_cells_match_na4_spectrum_params() {
    let tables = reference_tables();
    assert!(tables.iter().any(|t| t.id == "table-na-4-ground-combos"));
    assert!(tables.iter().any(|t| t.id == "table-na-1-seismic-zones"));
    let na4 = tables.iter().find(|t| t.id == "table-na-4-ground-combos").expect("NA.4");
    assert_ne!(na4.title_en, na4.title_de);
    let cs = na4.rows.iter().find(|r| r.id == "C-S").expect("C-S row");
    let CatalogueCell::Number { value: s, .. } = &cs.cells[1] else {
        panic!("C-S S cell must be Number");
    };
    let (expected_s, expected_tb, expected_tc, expected_td) = GroundCombo::CS.spectrum_params();
    assert!((*s - expected_s).abs() <= 0.005 * expected_s.max(1.0));
    let CatalogueCell::Number { value: tb, .. } = &cs.cells[2] else { panic!("TB") };
    let CatalogueCell::Number { value: tc, .. } = &cs.cells[3] else { panic!("TC") };
    let CatalogueCell::Number { value: td, .. } = &cs.cells[4] else { panic!("TD") };
    assert!((*tb - expected_tb).abs() <= 1e-9);
    assert!((*tc - expected_tc).abs() <= 1e-9);
    assert!((*td - expected_td).abs() <= 1e-9);
    let zones = tables.iter().find(|t| t.id == "table-na-1-seismic-zones").expect("NA.1");
    assert_ne!(zones.title_en, zones.title_de);
    let z2 = zones.rows.iter().find(|r| r.id == "zone-2").expect("zone-2");
    let CatalogueCell::Number { value: agr, .. } = &z2.cells[1] else { panic!("a_gR") };
    assert_eq!(*agr, SeismicZone::Zone2.a_gr());
}

