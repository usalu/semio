use super::*;
use crate::editor::en1995::unit_tests::context;

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

fn projected(locale: Locale) -> String {
    let node = render(crate::examples(), locale, "norm.catalogue").expect("catalogue");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("project")
}

#[test]
fn renders_reference_tables_with_examples() {
    let json = projected(Locale::En);
    assert!(json.contains("norm-catalogue.examples"), "{json}");
    for example in crate::examples() {
        assert!(json.contains(&format!("norm-catalogue.{}", example.id())), "missing example {}: {json}", example.id());
    }
    for table in reference_tables(Locale::En) {
        assert!(json.contains(&format!("norm-catalogue.table-{}", table.id)), "missing table {}: {json}", table.id);
        assert!(!table.rows.is_empty(), "table {} must not be empty", table.id);
        for row in &table.rows {
            assert_eq!(row.cells.len(), table.columns.len(), "table {} row {} must fill every column", table.id, row.id);
        }
    }
}

#[test]
fn strength_class_table_lists_every_tabulated_class_with_values() {
    let tables = reference_tables(Locale::En);
    let classes = tables.iter().find(|t| t.id == "strength-classes").expect("strength table");
    assert_eq!(classes.rows.len(), strength_class_options().len());
    let gl28h = classes.rows.iter().find(|r| r.id == "gl28h").expect("GL28h row");
    assert_eq!(gl28h.cells[2], CatalogueCell::number(28.0, 1));
    assert_eq!(gl28h.cells[8], CatalogueCell::number(425.0, 0));
    let fasteners = tables.iter().find(|t| t.id == "fastener-types").expect("fastener table");
    let nail = fasteners.rows.iter().find(|r| r.id == "nail").expect("nail row");
    assert_eq!(nail.cells[3..], [CatalogueCell::number(5.0, 0), CatalogueCell::number(10.0, 0), CatalogueCell::number(5.0, 0)]);
    let k_mod = tables.iter().find(|t| t.id == "k-mod").expect("k_mod table");
    assert_eq!(k_mod.rows.len(), 3);
    assert_eq!(k_mod.rows[2].cells[1], CatalogueCell::number(0.5, 2));
    let roles = tables.iter().find(|t| t.id == "member-roles").expect("role table");
    assert_eq!(roles.rows.len(), 4);
}

#[test]
fn reference_tables_are_localized_in_english_and_german() {
    let en = projected(Locale::En);
    let de = projected(Locale::De);
    assert!(en.contains("Strength classes") && en.contains("Glulam (EN 14080)") && en.contains("Bridge"), "{en}");
    assert!(de.contains("Festigkeitsklassen") && de.contains("Brettschichtholz (EN 14080)") && de.contains("Brücke") && de.contains("Stabdübel"), "{de}");
    assert!(!de.contains("Glulam (EN 14080)"), "German catalogue must not leak English product names: {de}");
}
