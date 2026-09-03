//! 🧬️ schema leaf
use schema::ArtifactSchema;

#[derive(Clone, Debug, Default, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.reasoning.wires.presence")]
pub struct WiresPresence {
    #[state(presence)]
    pub drag_node_id: Option<String>,
    #[state(presence)]
    pub drag_last_x: f64,
    #[state(presence)]
    pub drag_last_y: f64,
}
