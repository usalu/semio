//! 🧬️ schema leaf

use schema::ArtifactSchema;

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.bim.model.presence")]
pub struct BimPresence {
    #[state(presence)]
    pub engagement_input: String,
    #[state(presence)]
    pub storey: String,
    #[state(presence)]
    pub camera: store::Viewport2d,
}

impl Default for BimPresence {
    fn default() -> Self {
        Self { engagement_input: String::new(), storey: String::new(), camera: store::Viewport2d { x: 0.0, y: 0.0, zoom: 1.0 } }
    }
}
