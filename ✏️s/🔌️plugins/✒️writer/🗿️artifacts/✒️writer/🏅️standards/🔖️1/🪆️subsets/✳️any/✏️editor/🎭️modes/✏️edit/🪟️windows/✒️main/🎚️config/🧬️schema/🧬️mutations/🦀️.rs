use super::{WriterMainWindowConfig, WriterMainWindowConfigDiff};

#[path = "📷️set-camera/🦀️.rs"]
mod set_camera;
pub use set_camera::SetCamera;
#[path = "⚙️set-editor-settings/🦀️.rs"]
mod set_editor_settings;
pub use set_editor_settings::SetEditorSettings;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslEnum, dsl::Mutations, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "kebab-case")]
#[mutations(snapshot = WriterMainWindowConfig, diff = WriterMainWindowConfigDiff, schema = "writer.mainwindowconfig")]
pub enum WriterMainWindowConfigMutation {
    #[dsl(key = "set-camera")]
    SetCamera(SetCamera),
    #[dsl(key = "set-editor-settings")]
    SetEditorSettings(SetEditorSettings),
}





impl store::snapshot_clone_preparation::ConfigApplyMutation<WriterMainWindowConfig> for WriterMainWindowConfigMutation {
    fn exchange(self, post: &mut WriterMainWindowConfig) -> Result<Self, (semio_framework_value::ValueError, Self)> {
        Ok(match self {
            Self::SetCamera(SetCamera { camera }) => Self::SetCamera(SetCamera { camera: std::mem::replace(&mut post.camera, camera) }),
            Self::SetEditorSettings(SetEditorSettings { settings }) => Self::SetEditorSettings(SetEditorSettings { settings: std::mem::replace(&mut post.editor_settings, settings) }),
        })
    }
}

//#region 🌉️TestBridge

//#endregion 🌉️TestBridge
