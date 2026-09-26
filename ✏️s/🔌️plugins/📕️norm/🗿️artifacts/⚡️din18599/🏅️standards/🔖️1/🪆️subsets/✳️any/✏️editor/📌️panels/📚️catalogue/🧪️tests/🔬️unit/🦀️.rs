use super::*;
use crate::editor::din18599::unit_tests::context;

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

