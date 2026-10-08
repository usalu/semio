/// 🎥️ Persisted local viewport transform for one concrete Equation graph window.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct EquationCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for EquationCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, semio_framework_os_kernel::DslArtifact, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[dsl(layout = "lines")]
#[artifact(id = "mathematical.equationgraphwindowconfig")]
pub struct EquationGraphWindowConfig {
    #[dsl(block)]
    pub camera: EquationCamera,
}

semio_framework_os_kernel::config_diff! { record: EquationGraphWindowConfig, diff: EquationGraphWindowConfigDiff, fields: { camera: EquationCamera } }
