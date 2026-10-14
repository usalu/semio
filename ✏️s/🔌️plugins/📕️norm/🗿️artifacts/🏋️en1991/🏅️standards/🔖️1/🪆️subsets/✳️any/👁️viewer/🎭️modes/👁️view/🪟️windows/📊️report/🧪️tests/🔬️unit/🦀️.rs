use crate::modes::view::windows::report::*;

#[semio_framework_async_macros::async_test]
async fn render_assembles() {
    let document = semio_s_artifact_norm_en1991_model::En1991Snapshot::default();
    let _node = render(&document, semio_framework_ui_locale::Locale::En);
}
