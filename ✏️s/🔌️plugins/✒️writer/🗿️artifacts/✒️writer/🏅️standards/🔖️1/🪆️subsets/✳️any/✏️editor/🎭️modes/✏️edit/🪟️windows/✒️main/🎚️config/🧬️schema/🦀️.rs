/// 📷️ Persisted local viewport transform for one concrete Writer main window.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct WriterCamera {
    #[serde(default)]
    #[value(default)]
    pub x: f64,
    #[serde(default)]
    #[value(default)]
    pub y: f64,
    #[serde(default = "default_zoom")]
    #[value(default = "default_zoom")]
    pub zoom: f64,
}

impl Default for WriterCamera {
    fn default() -> Self {
        default_camera()
    }
}

/// ⚙️ Persisted local editor chrome for one concrete Writer main window.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
pub struct WriterEditorSettings {
    pub show_line_numbers: bool,
    pub font_px: u32,
    pub line_height: u32,
    pub tab_size: u32,
}

impl Default for WriterEditorSettings {
    fn default() -> Self {
        Self { show_line_numbers: true, font_px: 14, line_height: 22, tab_size: 2 }
    }
}

pub fn default_zoom() -> f64 {
    1.0
}

pub fn default_camera() -> WriterCamera {
    WriterCamera { x: 0.0, y: 0.0, zoom: 1.0 }
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, semio_framework_os_kernel::DslArtifact, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
#[artifact(extension = "writer.mainwindowconfig")]
#[dsl(layout = "lines")]
pub struct WriterMainWindowConfig {
    #[dsl(block)]
    pub camera: WriterCamera,
    #[dsl(block)]
    pub editor_settings: WriterEditorSettings,
}

impl Default for WriterMainWindowConfig {
    fn default() -> Self {
        Self { camera: WriterCamera::default(), editor_settings: WriterEditorSettings::default() }
    }
}

/// 🔺️ Sparse field delta over [`WriterMainWindowConfig`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct WriterMainWindowConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera: Option<WriterCamera>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub editor_settings: Option<WriterEditorSettings>,
}

impl protocol::MutationDiff<WriterMainWindowConfig> for WriterMainWindowConfigDiff {
    fn apply(&self, base: &WriterMainWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<WriterMainWindowConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.camera {
            next.camera = value.clone();
        }
        if let Some(value) = &self.editor_settings {
            next.editor_settings = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.camera.is_some() {
            self.camera = other.camera;
        }
        if other.editor_settings.is_some() {
            self.editor_settings = other.editor_settings;
        }
    }
}

impl protocol::DiffAlgebra<WriterMainWindowConfig> for WriterMainWindowConfigDiff {
    fn inverse(&self, base: &WriterMainWindowConfig) -> Self {
        Self {
            camera: self.camera.as_ref().map(|_| base.camera.clone()),
            editor_settings: self.editor_settings.as_ref().map(|_| base.editor_settings.clone()),
        }
    }
    fn between(base: &WriterMainWindowConfig, other: &WriterMainWindowConfig) -> Self {
        Self {
            camera: (base.camera != other.camera).then(|| other.camera.clone()),
            editor_settings: (base.editor_settings != other.editor_settings).then(|| other.editor_settings.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.camera.is_none() && self.editor_settings.is_none()
    }
}
