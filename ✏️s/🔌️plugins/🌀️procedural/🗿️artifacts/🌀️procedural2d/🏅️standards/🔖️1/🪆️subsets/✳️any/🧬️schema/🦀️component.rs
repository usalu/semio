//! 🧬️ Procedural2d artifact schema — every field of the artifact with its state class.

use crate::artifacts::procedural2d::snapshot::schema::Procedural2dSnapshot;
use flow::CameraJson;
use flow::FlowFixture;
use flow::playbook::GenerationPlayState;
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Procedural2dArtifact
/// 🧬️ Procedural2dArtifact facet type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.procedural.procedural2d")]

pub struct Procedural2dArtifact {
    #[state(persistent)] pub fixture: FlowFixture,
    #[state(persistent)] pub generation: GenerationPlayState,
    #[state(shared_ui)] pub selected_ids: Vec<String>,
    #[state(local_ui)] pub graph_camera: CameraJson,
    #[state(local_ui)] pub show_mode: String,
    #[state(shared_ui)] pub selected_generation_id: Option<String>,
    #[state(preview)] pub generation_preview_text: Option<String>,
    #[state(local_ui)] pub locale: String}
//#endregion 🔖️Procedural2dArtifact

impl Default for Procedural2dArtifact {
    fn default() -> Self {
        Self {
            fixture: FlowFixture::default(),
            generation: GenerationPlayState::default(),
            selected_ids: Vec::new(),
            graph_camera: CameraJson { x: 0.0, y: 0.0, zoom: 1.0 },
            show_mode: "preview".into(),
            selected_generation_id: None,
            generation_preview_text: None,
            locale: "en-US".into()}
    }
}

impl Procedural2dArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> Procedural2dSnapshot {
        Procedural2dSnapshot {
            fixture: self.fixture.clone(),
            generation: self.generation.clone()}
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: Procedural2dSnapshot) -> Self {
        Self {
            fixture: snapshot.fixture,
            generation: snapshot.generation,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: Procedural2dSnapshot) {
        self.fixture = snapshot.fixture;
        self.generation = snapshot.generation;
    }
}

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.procedural.procedural2d` — fifteen handcrafted schema leaves.
pub fn procedural2d_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.procedural.procedural2d",
        artifact: schema::FacetLeaves {
            rust: include_str!("🦀️component.rs"),
            typescript: include_str!("🟦️component.ts"),
            graphql: include_str!("🔗️component.graphql"),
            json_schema: include_str!("🔣️component.json"),
            proto: include_str!("🛰️component.proto")},
        snapshot: schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️component.rs"),
            typescript: include_str!("📸️snapshot/🟦️component.ts"),
            graphql: include_str!("📸️snapshot/🔗️component.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️component.json"),
            proto: include_str!("📸️snapshot/🛰️component.proto")},
        diff: schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️component.rs"),
            typescript: include_str!("🔺️diff/🟦️component.ts"),
            graphql: include_str!("🔺️diff/🔗️component.graphql"),
            json_schema: include_str!("🔺️diff/🔣️component.json"),
            proto: include_str!("🔺️diff/🛰️component.proto")}}
}
//#endregion 🔖️Descriptor
