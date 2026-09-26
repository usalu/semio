use super::*;
use crate::app_surface::CatalogueCell;
use crate::artifact_schema::inferences::evaluate;
use crate::document::AnnexChoice;
use crate::editor::en1991::unit_tests::context;
use crate::standards::v1::subsets::any::schema::part_1_1;
use crate::{En1991Snapshot, FloorArea};

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
    assert!(!tables.is_empty(), "catalogue must publish normative reference tables");
    for table in &tables {
        assert!(json.contains(&format!("norm-catalogue.table-{}", table.id)) || json.contains(table.title_en) || json.contains(table.id), "missing table {}: {json}", table.id);
        assert_ne!(table.title_en, table.title_de, "table {} en/de titles must differ", table.id);
    }
}

#[test]
fn reference_tables_cells_match_imposed_and_evaluate_limit() {
    let tables = reference_tables();
    assert!(!tables.is_empty());
    let de = tables.iter().find(|t| t.id == "table-6-1-imposed-qk-de").expect("DE q_k table");
    let b1 = de.rows.iter().find(|r| r.id == "B1").expect("B1 row");
    let CatalogueCell::Number { value: cell_qk, .. } = &b1.cells[1] else {
        panic!("B1 q_k cell must be Number");
    };
    let source = part_1_1::imposed_qk_pa("B1", AnnexChoice::De);
    assert_eq!(*cell_qk, source);
    assert_eq!(*cell_qk, 2000.0);

    let mut doc = En1991Snapshot::default();
    doc.storey_count = 1;
    doc.floors = vec![FloorArea {
        id: "b1".into(),
        category: "B1".into(),
        area: 20.0,
        assumed_qk: 2000.0,
        assumed_qk_concentrated: 2000.0,
        assumed_partitions: 800.0,
    }];
    let report = evaluate(&doc);
    let check = report.checks.iter().find(|c| c.id == "en1991.1-1.imposed.b1").expect("imposed B1 check");
    assert!((check.limit.value - *cell_qk).abs() < 1e-9, "evaluated limit {} != table cell {}", check.limit.value, cell_qk);
}
