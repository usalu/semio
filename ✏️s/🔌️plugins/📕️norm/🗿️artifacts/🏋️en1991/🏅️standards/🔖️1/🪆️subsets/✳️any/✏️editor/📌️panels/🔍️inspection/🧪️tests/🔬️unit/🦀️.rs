use crate::panels::inspection::*;
use crate::En1991Family;
use semio_s_plugin_norm_compliance::NormHost;

#[semio_framework_async_macros::async_test]
async fn an_out_of_range_selected_index_falls_back_to_the_first_check() {
    let host = NormHost::<En1991Family>::live(semio_s_artifact_norm_en1991_model::En1991Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
    let first = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::ComponentTree::new(render(&host, None, semio_ui_locale::Locale::En, None).expect("node assembly"))).expect("json");
    let clamped = semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::ComponentTree::new(render(&host, Some(9_999), semio_ui_locale::Locale::En, None).expect("node assembly"))).expect("json");
    assert_eq!(first, clamped);
}
