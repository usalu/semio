//! 🧬️ schema leaf
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.process.3d.config")]
pub struct Process3dConfig {
    #[state(config)]
    pub engagement_input: String,
    #[state(config)]
    pub camera_position: [f64; 3],
    #[state(config)]
    pub camera_target: [f64; 3],
    #[state(config)]
    pub camera_fov: f64,
    #[state(config)]
    pub sun_enabled: bool,
    #[state(config)]
    pub sun_azimuth: f64,
    #[state(config)]
    pub sun_elevation: f64,
    #[state(config)]
    pub sun_intensity: f64,
    #[state(config)]
    pub sun_color: String,
    #[state(config)]
    pub active_utility_id: String,
    #[state(config)]
    pub locale: String,
    #[state(config)]
    pub contributions_json: String,
}

//#region 🔖️AppSchemaDescriptor
/// 📎 `s.process.3d`'s config and presence schema, owned by this leaf.
pub fn app_schema_descriptor() -> schema::AppSchemaDescriptor {
    schema::AppSchemaDescriptor {
        id: "s.process.3d",
        config: schema::FacetLeaves {
            rust: include_str!("./🦀️component.rs"),
            typescript: include_str!("./🟦️component.ts"),
            graphql: include_str!("./🔗️component.graphql"),
            json_schema: include_str!("./🔣️component.json"),
            proto: include_str!("./🛰️component.proto"),
        },
        presence: schema::FacetLeaves {
            rust: include_str!("../../👥️presence/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../../👥️presence/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../../👥️presence/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../../👥️presence/🧬️schema/🔣️component.json"),
            proto: include_str!("../../👥️presence/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️AppSchemaDescriptor
