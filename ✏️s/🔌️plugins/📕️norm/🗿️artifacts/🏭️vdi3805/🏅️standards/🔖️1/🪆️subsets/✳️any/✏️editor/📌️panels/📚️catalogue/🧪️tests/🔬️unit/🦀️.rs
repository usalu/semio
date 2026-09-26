use super::*;
use crate::editor::vdi3805::unit_tests::context;
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
    assert!(!tables.is_empty(), "VDI 3805 catalogue must expose Blatt code-list tables");
    for table in &tables {
        assert!(
            json.contains(&format!("norm-catalogue.table-{}", table.id)) || json.contains(table.title_en) || json.contains(table.id),
            "missing table {}: {json}",
            table.id
        );
    }
}
