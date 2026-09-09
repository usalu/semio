use super::*;
use crate::editor::forms::testkit::{dispatch, forms_app};
use crate::editor::forms::FormsCommand;
use ExportFixture;

#[semio_framework_async_macros::async_test]
async fn export_fixture_downloads_the_forms_dsl_text() {
    let mut app = forms_app().await;
    let result = dispatch(&mut app, FormsCommand::ExportFixture(ExportFixture {})).await;
    assert!(!result.requested_effects.is_empty(), "exportFixture must emit a host effect");
}
