use super::*;
use crate::editor::layout::unit_tests::context::{dispatch, layout_app};
use crate::editor::layout::LayoutCommand;

#[semio_framework_async_macros::async_test]
async fn patch_document_renames_sets_print_target_and_clears_it() {
    let mut app = layout_app().await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "name".into(), value: "Press sheet".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "printTarget".into(), value: "cmyk".into() })).await;
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "dataFields".into(), value: "{\"title\":\"A\"}".into() })).await;
    let snapshot = app.snapshot().expect("projection");
    assert_eq!(snapshot.name, "Press sheet");
    assert_eq!(snapshot.print_target.as_deref(), Some("cmyk"));
    assert_eq!(snapshot.data_fields_json.as_deref(), Some("{\"title\":\"A\"}"));
    dispatch(&mut app, LayoutCommand::PatchDocument(PatchDocument { field: "printTarget".into(), value: "  ".into() })).await;
    assert_eq!(app.snapshot().expect("projection").print_target, None);
}
