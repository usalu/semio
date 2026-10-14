use crate::panels::inspection::*;
use crate::En1991Family;
use semio_s_plugin_norm_compliance::NormHost;

#[semio_framework_async_macros::async_test]
async fn an_out_of_range_selected_index_falls_back_to_the_first_check() {
    let host = NormHost::<En1991Family>::from_artifact(semio_s_artifact_norm_en1991_model::En1991Snapshot::default());
    let first = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(render(&host, None, semio_framework_ui_locale::Locale::En, None).expect("node assembly"))).expect("json");
    let clamped = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::ComponentTree::new(render(&host, Some(9_999), semio_framework_ui_locale::Locale::En, None).expect("node assembly"))).expect("json");
    assert_eq!(first, clamped);
}
