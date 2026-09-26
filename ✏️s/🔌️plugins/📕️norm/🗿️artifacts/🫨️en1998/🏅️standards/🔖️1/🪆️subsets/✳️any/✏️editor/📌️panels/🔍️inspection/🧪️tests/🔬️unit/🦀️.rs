use super::*;

#[semio_framework_async_macros::async_test]
async fn definition_binds_the_framework_inspection_tab_to_this_body_key() {
    assert_eq!(definition().body_key.as_deref(), Some(BODY_INSPECTION));
    assert_eq!(definition().id(), FRAMEWORK_PANEL_TAB_INSPECTION_ID);
    assert!(matches!(definition().group, PanelGroup::Details));
}

#[semio_framework_async_macros::async_test]
async fn an_out_of_range_selected_index_falls_back_to_the_first_check() {
    let mut host = NormHost::<En1998Family>::from_artifact(crate::En1998Snapshot::default());
    host.evaluate();
    let first = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree {
        root: render(&host, None, Default::default(), None).expect("node assembly"),
    })
    .expect("json");
    let clamped = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree {
        root: render(&host, Some(9_999), Default::default(), None).expect("node assembly"),
    })
    .expect("json");
    assert_eq!(first, clamped);
}
