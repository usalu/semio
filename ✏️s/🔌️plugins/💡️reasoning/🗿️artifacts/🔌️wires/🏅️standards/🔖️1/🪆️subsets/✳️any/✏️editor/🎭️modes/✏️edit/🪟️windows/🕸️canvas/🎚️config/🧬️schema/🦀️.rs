/// 🎥️ Persisted local viewport transform for one concrete Wires canvas.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct WiresCanvasCamera {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl Default for WiresCanvasCamera {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, zoom: 1.0 }
    }
}

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, semio_framework_os_kernel::DslArtifact, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase", default)]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[dsl(layout = "lines")]
#[artifact(id = "reasoning.wirescanvaswindowconfig")]
pub struct WiresCanvasWindowConfig {
    #[dsl(block)]
    pub camera: WiresCanvasCamera,
}

/// 🔺️ Sparse field delta over [`WiresCanvasWindowConfig`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct WiresCanvasWindowConfigDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub camera: Option<WiresCanvasCamera>,
}

impl protocol::MutationDiff<WiresCanvasWindowConfig> for WiresCanvasWindowConfigDiff {
    fn apply(&self, base: &WiresCanvasWindowConfig, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<WiresCanvasWindowConfig> {
        let mut next = base.clone();
        if let Some(value) = &self.camera {
            next.camera = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.camera.is_some() {
            self.camera = other.camera;
        }
    }
}

impl protocol::DiffAlgebra<WiresCanvasWindowConfig> for WiresCanvasWindowConfigDiff {
    fn inverse(&self, base: &WiresCanvasWindowConfig) -> Self {
        Self {
            camera: self.camera.as_ref().map(|_| base.camera.clone()),
        }
    }
    fn between(base: &WiresCanvasWindowConfig, other: &WiresCanvasWindowConfig) -> Self {
        Self {
            camera: (base.camera != other.camera).then(|| other.camera.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.camera.is_none()
    }
}
