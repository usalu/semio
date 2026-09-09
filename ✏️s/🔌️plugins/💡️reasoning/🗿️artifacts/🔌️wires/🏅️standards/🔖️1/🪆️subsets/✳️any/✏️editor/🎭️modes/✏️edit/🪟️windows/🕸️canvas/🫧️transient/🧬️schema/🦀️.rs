//! 🧬️ schema leaf
use framework_schema::ArtifactSchema;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
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
