use super::{WriterMainWindowConfig, WriterMainWindowConfigDiff};

#[path = "📷️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;
#[path = "⚙️set-editor-settings/🦀️.rs"]
mod set_editor_settings;
pub use set_editor_settings::SetEditorSettings;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations)]
#[value(tag = "kind", rename_all = "kebab-case")]
#[mutations(snapshot = WriterMainWindowConfig, diff = WriterMainWindowConfigDiff, schema = "writer.mainwindowconfig")]
pub enum WriterMainWindowConfigMutation {
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
    #[dsl(key = "set-editor-settings")]
    SetEditorSettings(SetEditorSettings),
}





//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
