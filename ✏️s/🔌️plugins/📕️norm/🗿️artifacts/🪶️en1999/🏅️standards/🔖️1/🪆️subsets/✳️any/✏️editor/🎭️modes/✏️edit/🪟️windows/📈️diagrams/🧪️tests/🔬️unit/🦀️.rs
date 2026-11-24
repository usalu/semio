use crate::modes::edit::windows::diagrams::*;
use crate::unit_tests::context;
use semio_s_artifact_norm_en1999_scene::diagrams;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {
    assert_eq!(definition().body_key, BODY_DIAGRAMS);
    assert_eq!(definition().id, WINDOW_DIAGRAMS);
}

#[semio_framework_async_macros::async_test]
async fn lists_the_diagrams_of_the_default_document_as_gallery_cards() {
    let mut app = context::app_with_registry().await;
    let rendered = context::render(&mut app, BODY_DIAGRAMS).await;
    let total = rendered.split("\"total\":").nth(1).and_then(|rest| rest.split(|c: char| !c.is_ascii_digit()).next()).and_then(|digits| digits.parse::<usize>().ok());
    assert!(total.is_some_and(|total| total > 0), "the diagrams body must list the diagram cards of the default document: {rendered}");
    context::close(&mut app);
}
