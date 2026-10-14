use crate::modes::view::windows::report::*;

#[semio_framework_async_macros::async_test]
async fn report_renders_default_document() {
    let document = semio_s_artifact_norm_en1990_model::En1990Snapshot::default();
    let _node = render(&document, semio_framework_ui_locale::Locale::En).expect("render");
}
