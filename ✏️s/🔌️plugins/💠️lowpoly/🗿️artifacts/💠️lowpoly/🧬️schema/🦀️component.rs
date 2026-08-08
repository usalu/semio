//! 🧬️ Lowpoly artifact schema — every field of the artifact with its state class.

use crate::artifacts::lowpoly::{LowpolyObject, LowpolySelection};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full lowpoly artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.lowpoly.lowpoly")]
pub struct LowpolyArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub objects: Vec<LowpolyObject>,
    #[state(shared_ui)] pub active_object_id: Option<String>,
    #[state(shared_ui)] pub selection: LowpolySelection,
    #[state(shared_ui)] pub selected_object_ids: Vec<String>,
    #[state(shared_ui)] pub paint_utility: String,
    #[state(shared_ui)] pub active_paint_layer: u32,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(local_ui)] pub show_edges: bool,
    #[state(local_ui)] pub sun_enabled: bool,
    #[state(local_ui)] pub sun_azimuth: f64,
    #[state(local_ui)] pub sun_elevation: f64,
    #[state(local_ui)] pub sun_intensity: f64,
    #[state(local_ui)] pub sun_color: String,
    #[state(local_ui)] pub world_camera_position_x: f64,
    #[state(local_ui)] pub world_camera_position_y: f64,
    #[state(local_ui)] pub world_camera_position_z: f64,
    #[state(local_ui)] pub world_camera_target_x: f64,
    #[state(local_ui)] pub world_camera_target_y: f64,
    #[state(local_ui)] pub world_camera_target_z: f64,
    #[state(local_ui)] pub world_camera_fov: f64,
    #[state(local_ui)] pub utility_params_json: String,
    #[state(local_ui)] pub paint_color_r: u32,
    #[state(local_ui)] pub paint_color_g: u32,
    #[state(local_ui)] pub paint_color_b: u32,
    #[state(local_ui)] pub paint_color_a: u32,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub selection_mode_default: String,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub locale: String,
    #[state(preview)] pub hovered_object_id: Option<String>,
    #[state(preview)] pub hovered_target_object_id: Option<String>,
    #[state(preview)] pub hovered_target_mode: Option<String>,
    #[state(preview)] pub hovered_target_id: Option<u32>,
    #[state(preview)] pub stroke_drag_active: bool,
    #[state(preview)] pub transform_drag_active: bool,
    #[state(preview)] pub preview_seq: i64,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for LowpolyArtifact {
    fn default() -> Self {
        Self {
            schema: crate::artifacts::lowpoly::LOWPOLY_DOCUMENT_SCHEMA.into(),
            objects: Vec::new(),
            active_object_id: None,
            selection: crate::artifacts::lowpoly::LowpolySelection::default(),
            selected_object_ids: Vec::new(),
            paint_utility: "brush".into(),
            active_paint_layer: 0,
            active_utility_id: "move".into(),
            show_edges: true,
            sun_enabled: false,
            sun_azimuth: 45.0,
            sun_elevation: 35.0,
            sun_intensity: 0.85,
            sun_color: "#ffffff".into(),
            world_camera_position_x: 18.0,
            world_camera_position_y: -18.0,
            world_camera_position_z: 12.0,
            world_camera_target_x: 0.0,
            world_camera_target_y: 0.0,
            world_camera_target_z: 0.0,
            world_camera_fov: 45.0,
            utility_params_json: String::new(),
            paint_color_r: 255,
            paint_color_g: 64,
            paint_color_b: 64,
            paint_color_a: 255,
            selection_method: "rectangle".into(),
            selection_mode_default: "default".into(),
            engagement_input: String::new(),
            locale: "en-US".into(),
            hovered_object_id: None,
            hovered_target_object_id: None,
            hovered_target_mode: None,
            hovered_target_id: None,
            stroke_drag_active: false,
            transform_drag_active: false,
            preview_seq: 0,
        }
    }
}

impl LowpolyArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::lowpoly::LowpolySnapshot {
        crate::artifacts::lowpoly::LowpolySnapshot {
            schema: self.schema.clone(),
            objects: self.objects.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::lowpoly::LowpolySnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            objects: snapshot.objects,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::lowpoly::LowpolySnapshot) {
        self.schema = snapshot.schema;
        self.objects = snapshot.objects;
    }
}
//#endregion 🔖️Conversions


//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.lowpoly.lowpoly` — fifteen handcrafted schema leaves.
pub fn lowpoly_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.lowpoly.lowpoly",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("../📸️snapshot/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../📸️snapshot/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../📸️snapshot/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../📸️snapshot/🧬️schema/🔣️component.json"),
            proto: include_str!("../📸️snapshot/🧬️schema/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("../🔺️diff/🧬️schema/🦀️component.rs"),
            typescript: include_str!("../🔺️diff/🧬️schema/🟦️component.ts"),
            graphql: include_str!("../🔺️diff/🧬️schema/🔗️component.graphql"),
            json_schema: include_str!("../🔺️diff/🧬️schema/🔣️component.json"),
            proto: include_str!("../🔺️diff/🧬️schema/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
