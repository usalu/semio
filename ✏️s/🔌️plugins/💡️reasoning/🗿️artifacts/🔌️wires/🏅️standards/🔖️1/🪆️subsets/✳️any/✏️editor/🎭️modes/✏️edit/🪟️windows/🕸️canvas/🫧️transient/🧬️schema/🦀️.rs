//! 🧬️ schema leaf
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.reasoning.wires.canvas-window-transient")]
pub struct WiresCanvasTransient {
    #[state(transient)]
    pub drag_node_id: Option<String>,
    #[state(transient)]
    pub drag_start_x: f64,
    #[state(transient)]
    pub drag_start_y: f64,
    #[state(transient)]
    pub drag_last_x: f64,
    #[state(transient)]
    pub drag_last_y: f64,
    #[state(transient)]
    pub drag_zoom: f64,
}

/// 🧱️ Carries an optional value as a present slot, so clearing it stays distinct from leaving it untouched on every wire.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct WiresCanvasOptionalNode {
    pub value: Option<String>,
}

/// 🔺️ Sparse field delta over [`WiresCanvasTransient`]: every present slot is the new value of exactly that field.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct WiresCanvasTransientDiff {
    #[value(skip_serializing_if = "Option::is_none")]
    pub drag_node_id: Option<WiresCanvasOptionalNode>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub drag_start_x: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub drag_start_y: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub drag_last_x: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub drag_last_y: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub drag_zoom: Option<f64>,
}

impl protocol::MutationDiff<WiresCanvasTransient> for WiresCanvasTransientDiff {
    fn apply(&self, base: &WiresCanvasTransient, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<WiresCanvasTransient> {
        let mut next = base.clone();
        if let Some(value) = &self.drag_node_id {
            next.drag_node_id = value.value.clone();
        }
        if let Some(value) = &self.drag_start_x {
            next.drag_start_x = value.clone();
        }
        if let Some(value) = &self.drag_start_y {
            next.drag_start_y = value.clone();
        }
        if let Some(value) = &self.drag_last_x {
            next.drag_last_x = value.clone();
        }
        if let Some(value) = &self.drag_last_y {
            next.drag_last_y = value.clone();
        }
        if let Some(value) = &self.drag_zoom {
            next.drag_zoom = value.clone();
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        if other.drag_node_id.is_some() {
            self.drag_node_id = other.drag_node_id;
        }
        if other.drag_start_x.is_some() {
            self.drag_start_x = other.drag_start_x;
        }
        if other.drag_start_y.is_some() {
            self.drag_start_y = other.drag_start_y;
        }
        if other.drag_last_x.is_some() {
            self.drag_last_x = other.drag_last_x;
        }
        if other.drag_last_y.is_some() {
            self.drag_last_y = other.drag_last_y;
        }
        if other.drag_zoom.is_some() {
            self.drag_zoom = other.drag_zoom;
        }
    }
}

impl protocol::DiffAlgebra<WiresCanvasTransient> for WiresCanvasTransientDiff {
    fn inverse(&self, base: &WiresCanvasTransient) -> Self {
        Self {
            drag_node_id: self.drag_node_id.as_ref().map(|_| WiresCanvasOptionalNode { value: base.drag_node_id.clone() }),
            drag_start_x: self.drag_start_x.as_ref().map(|_| base.drag_start_x.clone()),
            drag_start_y: self.drag_start_y.as_ref().map(|_| base.drag_start_y.clone()),
            drag_last_x: self.drag_last_x.as_ref().map(|_| base.drag_last_x.clone()),
            drag_last_y: self.drag_last_y.as_ref().map(|_| base.drag_last_y.clone()),
            drag_zoom: self.drag_zoom.as_ref().map(|_| base.drag_zoom.clone()),
        }
    }
    fn between(base: &WiresCanvasTransient, other: &WiresCanvasTransient) -> Self {
        Self {
            drag_node_id: (base.drag_node_id != other.drag_node_id).then(|| WiresCanvasOptionalNode { value: other.drag_node_id.clone() }),
            drag_start_x: (base.drag_start_x != other.drag_start_x).then(|| other.drag_start_x.clone()),
            drag_start_y: (base.drag_start_y != other.drag_start_y).then(|| other.drag_start_y.clone()),
            drag_last_x: (base.drag_last_x != other.drag_last_x).then(|| other.drag_last_x.clone()),
            drag_last_y: (base.drag_last_y != other.drag_last_y).then(|| other.drag_last_y.clone()),
            drag_zoom: (base.drag_zoom != other.drag_zoom).then(|| other.drag_zoom.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.drag_node_id.is_none() && self.drag_start_x.is_none() && self.drag_start_y.is_none() && self.drag_last_x.is_none() && self.drag_last_y.is_none() && self.drag_zoom.is_none()
    }
}
