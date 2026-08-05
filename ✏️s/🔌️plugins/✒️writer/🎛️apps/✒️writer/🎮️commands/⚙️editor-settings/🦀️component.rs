//! ⚙️ Writer play app commands — editor chrome settings: line-number toggle, font size, line height,
//! tab size. All config-only View commands, all patching `WriterConfig::editor_settings`.

use crate::apps::writer::config::{WriterConfig, WriterConfigOperation};
use crate::artifacts::writer::op::WriterOperation;
use crate::artifacts::writer::WriterProjection;
use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};
use serde::{Deserialize, Serialize};

//#region 🔖️ToggleLineNumbers
pub mod toggle_line_numbers {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "toggle-line-numbers")]
    pub struct ToggleLineNumbers {}

    pub fn handle(_payload: &ToggleLineNumbers, _doc: &DocumentView<'_, WriterProjection>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterOperation, WriterConfigOperation>, Fault> {
        let config = cfg.projection;
        let mut settings = config.editor_settings.clone();
        settings.show_line_numbers = !settings.show_line_numbers;
        Ok(Emit::config(vec![WriterConfigOperation::SetEditorSettings { settings }, WriterConfigOperation::SetRevision { value: config.revision + 1 }]))
    }
}
//#endregion 🔖️ToggleLineNumbers

//#region 🔖️SetFontPx
pub mod set_font_px {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "font-px")]
    pub struct SetFontPx {
        pub value: u32,
    }

    pub fn handle(payload: &SetFontPx, _doc: &DocumentView<'_, WriterProjection>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterOperation, WriterConfigOperation>, Fault> {
        let config = cfg.projection;
        let mut settings = config.editor_settings.clone();
        settings.font_px = payload.value;
        Ok(Emit::config(vec![WriterConfigOperation::SetEditorSettings { settings }, WriterConfigOperation::SetRevision { value: config.revision + 1 }]))
    }
}
//#endregion 🔖️SetFontPx

//#region 🔖️SetLineHeight
pub mod set_line_height {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "line-height")]
    pub struct SetLineHeight {
        pub value: u32,
    }

    pub fn handle(payload: &SetLineHeight, _doc: &DocumentView<'_, WriterProjection>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterOperation, WriterConfigOperation>, Fault> {
        let config = cfg.projection;
        let mut settings = config.editor_settings.clone();
        settings.line_height = payload.value;
        Ok(Emit::config(vec![WriterConfigOperation::SetEditorSettings { settings }, WriterConfigOperation::SetRevision { value: config.revision + 1 }]))
    }
}
//#endregion 🔖️SetLineHeight

//#region 🔖️SetTabSize
pub mod set_tab_size {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "tab-size")]
    pub struct SetTabSize {
        pub value: u32,
    }

    pub fn handle(payload: &SetTabSize, _doc: &DocumentView<'_, WriterProjection>, cfg: &ConfigView<'_, WriterConfig>) -> Result<Emit<WriterOperation, WriterConfigOperation>, Fault> {
        let config = cfg.projection;
        let mut settings = config.editor_settings.clone();
        settings.tab_size = payload.value.max(1);
        Ok(Emit::config(vec![WriterConfigOperation::SetEditorSettings { settings }, WriterConfigOperation::SetRevision { value: config.revision + 1 }]))
    }
}
//#endregion 🔖️SetTabSize

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::toggle_line_numbers;
    use crate::apps::writer::testkit::new_app;
    use crate::apps::writer::WriterCommand;
    use semio_framework_plugin::PluginApp;

    #[test]
    fn view_action_emits_no_operations() {
        let mut app = new_app();
        let result = app.dispatch_typed(WriterCommand::ToggleLineNumbers(toggle_line_numbers::ToggleLineNumbers {}), &semio_framework_plugin::testkit::meta("local")).expect("toggle");
        assert!(result.operations.is_empty());
    }
}
//#endregion 🧪️Tests
