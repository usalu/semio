/// 📷️ Persisted local viewport transform for one concrete Writer main window.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord, dsl::ToValue, dsl::FromValue)]
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
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord, dsl::ToValue, dsl::FromValue)]
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

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslArtifact, dsl::ToValue, dsl::FromValue)]
#[serde(rename_all = "camelCase", default)]
#[value(rename_all = "camelCase", default)]
#[dsl(extension = "writer.mainwindowconfig")]
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
