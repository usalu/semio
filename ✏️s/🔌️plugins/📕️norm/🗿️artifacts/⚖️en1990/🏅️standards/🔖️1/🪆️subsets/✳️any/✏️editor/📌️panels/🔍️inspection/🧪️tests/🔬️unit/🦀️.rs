use crate::panels::inspection::*;
use semio_s_plugin_norm_compliance::NormHost;
use crate::En1990Family;

#[semio_framework_async_macros::async_test]
async fn inspection_renders_with_locale() {
    let host = NormHost::<En1990Family>::from_artifact(semio_s_artifact_norm_en1990_model::En1990Snapshot::default());
    let _node = render(&host, None, semio_framework_ui_locale::Locale::En, None).expect("node assembly");
}
