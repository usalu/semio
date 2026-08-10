//! 🧬️ Cad artifact schema — every field of the artifact with its state class.

use crate::artifacts::cad::{
    CadCamera, CadGeometry, CadNode, CadObject, CadReferenceList, CadSnapshot,
};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

//#region 🔖️ArtifactHelpers
/// 🎯️ Component-level selection for World3d overlays (artifact-owned mirror of app config).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadSelectionTargets {
    pub mesh: bool,
    pub vertex: bool,
    pub edge: bool,
    pub face: bool,
}

impl Default for CadSelectionTargets {
    fn default() -> Self {
        Self { mesh: true, vertex: false, edge: true, face: false }
    }
}

/// 🎯️ Component selection record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadComponentSelection {
    pub targets: CadSelectionTargets,
    pub mode: String,
    pub ids: Vec<u32>,
}

impl Default for CadComponentSelection {
    fn default() -> Self {
        Self { targets: CadSelectionTargets::default(), mode: "mesh".into(), ids: Vec::new() }
    }
}

/// 🎛️ Per-pane dislocate handle groups.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CadDislocateOptions {
    pub move_enabled: bool,
    pub rotate_enabled: bool,
}

impl Default for CadDislocateOptions {
    fn default() -> Self {
        Self { move_enabled: true, rotate_enabled: true }
    }
}
//#endregion 🔖️ArtifactHelpers

//#region 🔖️Artifact
/// 🧬️ Full cad artifact state across persistent, shared-ui, local-ui and preview classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.cad.cad")]
pub struct CadArtifact {
    #[state(persistent)] pub schema: String,
    #[state(persistent)] pub id: String,
    #[state(persistent)] pub objects: Vec<CadObject>,
    #[state(persistent)] pub building_objects: Vec<CadObject>,
    #[state(persistent)] pub energy_objects: Vec<CadObject>,
    #[state(persistent)] pub structure_classic_objects: Vec<CadObject>,
    #[state(persistent)] pub references_by_model_definition_id: BTreeMap<String, CadReferenceList>,
    #[state(persistent)] pub nodes: Vec<CadNode>,
    #[state(persistent)] pub shape_geometry: Option<CadGeometry>,
    #[state(persistent)] pub building_geometry: Option<CadGeometry>,
    #[state(persistent)] pub energy_geometry: Option<CadGeometry>,
    #[state(persistent)] pub structure_classic_geometry: Option<CadGeometry>,
    #[state(persistent)] pub active_model_definition_id: String,
    #[state(shared_ui)] pub selected_object_ids: Vec<String>,
    #[state(shared_ui)] pub selected_node_ids: Vec<String>,
    #[state(shared_ui)] pub active_object_id: Option<String>,
    #[state(shared_ui)] pub component_selection: CadComponentSelection,
    #[state(shared_ui)] pub selected_reference_model_definition_id: Option<String>,
    #[state(shared_ui)] pub selected_reference_id: Option<String>,
    #[state(shared_ui)] pub selected_primitive_id: Option<String>,
    #[state(shared_ui)] pub selected_primitive_kind: Option<String>,
    #[state(shared_ui)] pub active_utility_id: String,
    #[state(shared_ui)] pub active_example_id: Option<String>,
    #[state(local_ui)] pub selection_method: String,
    #[state(local_ui)] pub engagement_input: String,
    #[state(local_ui)] pub engagement_step: String,
    #[state(local_ui)] pub engagement_pane: Option<String>,
    #[state(local_ui)] pub engagement_session_json: Option<String>,
    #[state(local_ui)] pub last_finalized_interaction_id: Option<String>,
    #[state(local_ui)] pub sun_enabled: bool,
    #[state(local_ui)] pub sun_azimuth: f64,
    #[state(local_ui)] pub sun_elevation: f64,
    #[state(local_ui)] pub sun_intensity: f64,
    #[state(local_ui)] pub sun_color: String,
    #[state(local_ui)] pub camera: CadCamera,
    #[state(local_ui)] pub camera_building: CadCamera,
    #[state(local_ui)] pub camera_energy: CadCamera,
    #[state(local_ui)] pub camera_structure_classic: CadCamera,
    #[state(local_ui)] pub dislocate_shape: CadDislocateOptions,
    #[state(local_ui)] pub dislocate_building: CadDislocateOptions,
    #[state(local_ui)] pub dislocate_energy: CadDislocateOptions,
    #[state(local_ui)] pub dislocate_structure_classic: CadDislocateOptions,
    #[state(local_ui)] pub locale: String,
    #[state(local_ui)] pub terminology: String,
    #[state(local_ui)] pub contributions_json: String,
    #[state(preview)] pub hovered_object_id: Option<String>,
    #[state(preview)] pub hovered_target_object_id: Option<String>,
    #[state(preview)] pub hovered_target_mode: Option<String>,
    #[state(preview)] pub hovered_target_id: Option<u32>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for CadArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::artifacts::cad::empty_cad_snapshot())
    }
}

impl CadArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> CadSnapshot {
        CadSnapshot {
            schema: self.schema.clone(),
            id: self.id.clone(),
            objects: self.objects.clone(),
            building_objects: self.building_objects.clone(),
            energy_objects: self.energy_objects.clone(),
            structure_classic_objects: self.structure_classic_objects.clone(),
            references_by_model_definition_id: self.references_by_model_definition_id.clone(),
            nodes: self.nodes.clone(),
            shape_geometry: self.shape_geometry.clone(),
            building_geometry: self.building_geometry.clone(),
            energy_geometry: self.energy_geometry.clone(),
            structure_classic_geometry: self.structure_classic_geometry.clone(),
            active_model_definition_id: self.active_model_definition_id.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: CadSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            id: snapshot.id,
            objects: snapshot.objects,
            building_objects: snapshot.building_objects,
            energy_objects: snapshot.energy_objects,
            structure_classic_objects: snapshot.structure_classic_objects,
            references_by_model_definition_id: snapshot.references_by_model_definition_id,
            nodes: snapshot.nodes,
            shape_geometry: snapshot.shape_geometry,
            building_geometry: snapshot.building_geometry,
            energy_geometry: snapshot.energy_geometry,
            structure_classic_geometry: snapshot.structure_classic_geometry,
            active_model_definition_id: snapshot.active_model_definition_id,
            selected_object_ids: Vec::new(),
            selected_node_ids: Vec::new(),
            active_object_id: None,
            component_selection: CadComponentSelection::default(),
            selected_reference_model_definition_id: None,
            selected_reference_id: None,
            selected_primitive_id: None,
            selected_primitive_kind: None,
            active_utility_id: "dislocate".into(),
            active_example_id: None,
            selection_method: "rectangle".into(),
            engagement_input: String::new(),
            engagement_step: "Idle".into(),
            engagement_pane: None,
            engagement_session_json: None,
            last_finalized_interaction_id: None,
            sun_enabled: false,
            sun_azimuth: 45.0,
            sun_elevation: 35.0,
            sun_intensity: 0.85,
            sun_color: "#ffffff".into(),
            camera: CadCamera::default(),
            camera_building: CadCamera::default(),
            camera_energy: CadCamera::default(),
            camera_structure_classic: CadCamera::default(),
            dislocate_shape: CadDislocateOptions::default(),
            dislocate_building: CadDislocateOptions::default(),
            dislocate_energy: CadDislocateOptions::default(),
            dislocate_structure_classic: CadDislocateOptions::default(),
            locale: "en-US".into(),
            terminology: "native".into(),
            contributions_json: "[]".into(),
            hovered_object_id: None,
            hovered_target_object_id: None,
            hovered_target_mode: None,
            hovered_target_id: None,
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: CadSnapshot) {
        self.schema = snapshot.schema;
        self.id = snapshot.id;
        self.objects = snapshot.objects;
        self.building_objects = snapshot.building_objects;
        self.energy_objects = snapshot.energy_objects;
        self.structure_classic_objects = snapshot.structure_classic_objects;
        self.references_by_model_definition_id = snapshot.references_by_model_definition_id;
        self.nodes = snapshot.nodes;
        self.shape_geometry = snapshot.shape_geometry;
        self.building_geometry = snapshot.building_geometry;
        self.energy_geometry = snapshot.energy_geometry;
        self.structure_classic_geometry = snapshot.structure_classic_geometry;
        self.active_model_definition_id = snapshot.active_model_definition_id;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.cad.cad` — fifteen handcrafted schema leaves.
pub fn cad_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.cad.cad",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto"),
        },
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto"),
        },
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
