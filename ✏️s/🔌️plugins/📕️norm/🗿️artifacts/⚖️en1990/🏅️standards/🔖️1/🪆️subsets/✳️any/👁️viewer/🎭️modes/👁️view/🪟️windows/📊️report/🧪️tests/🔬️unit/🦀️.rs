use super::*;

#[semio_framework_async_macros::async_test]
async fn report_renders_default_document() {
    let document = crate::En1990Snapshot::default();
    let _node = render(&document, semio_framework_plugin::Locale::En).expect("render");
}
