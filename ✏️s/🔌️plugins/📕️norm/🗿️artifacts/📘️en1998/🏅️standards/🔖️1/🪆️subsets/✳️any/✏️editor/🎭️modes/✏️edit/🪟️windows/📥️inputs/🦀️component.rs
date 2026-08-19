//! 📥️ EN 1998 play app — the inputs window: the raw compliance document, rendered as JSON.

use crate::artifacts::en1998::En1998Snapshot;
use semio_framework_plugin::{LocalizedLabel, UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_INPUTS: &str = "norm-en1998-inputs";
pub const BODY_INPUTS: &str = "norm.en1998.play.inputs";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::en1998::create_en1998_app`.
pub async fn definition() -> WindowKindDefinition {
    crate::app_surface::window_definition(WINDOW_INPUTS, LocalizedLabel::native("Inputs", "Eingaben"), BODY_INPUTS, "download")
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub async fn render(document: &En1998Snapshot) -> UiNode {
    crate::app_surface::render_document_json(document)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::en1998::testkit;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_this_windows_body_key() {
        assert_eq!(definition().body_key, BODY_INPUTS);
        assert_eq!(definition().id, WINDOW_INPUTS);
    }

    #[semio_framework_async_macros::async_test]
    async fn renders_the_document_as_json() {
        let mut app = testkit::new_app();
        assert!(testkit::render(&mut app, BODY_INPUTS).contains(':'), "the inputs body renders the document json");
    }
}
//#endregion 🧪️Tests
