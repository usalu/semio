/// 🎥️ Persisted local viewport transform for one concrete Equation graph window.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, dsl::ToValue, dsl::FromValue)]
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

#[derive(Clone, Debug, Default, PartialEq, dsl::DslArtifact, dsl::ToValue, dsl::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[dsl(id = "mathematical.equationgraphwindowconfig", layout = "lines")]
pub struct EquationGraphWindowConfig {
    #[dsl(block)]
    pub camera: EquationCamera,
}
