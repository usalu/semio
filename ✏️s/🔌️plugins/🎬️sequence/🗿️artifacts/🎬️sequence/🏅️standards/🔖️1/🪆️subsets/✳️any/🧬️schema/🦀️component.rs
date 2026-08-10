//! 🧬️ Sequence artifact schema — every field of the artifact with its state class.

use crate::artifacts::sequence::{SequenceCamera, SequenceEdge, SequenceStep, SEQUENCE_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full sequence artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.sequence.sequence")]
pub struct SequenceArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub steps: Vec<SequenceStep>,
    #[state(persistent)]
    pub edges: Vec<SequenceEdge>,
    #[state(shared_ui)]
    pub selected_step_ids: Vec<String>,
    #[state(local_ui)]
    pub last_run_json: String,
    #[state(local_ui)]
    pub orientation: String,
    #[state(local_ui)]
    pub camera: SequenceCamera,
    #[state(local_ui)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for SequenceArtifact {
    fn default() -> Self {
        Self {
            schema: SEQUENCE_DOCUMENT_SCHEMA.into(),
            steps: Vec::new(),
            edges: Vec::new(),
            selected_step_ids: Vec::new(),
            last_run_json: String::new(),
            orientation: "leftRight".into(),
            camera: SequenceCamera::default(),
            locale: "en-US".into(),
        }
    }
}

impl SequenceArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::sequence::SequenceSnapshot {
        crate::artifacts::sequence::SequenceSnapshot {
            schema: self.schema.clone(),
            steps: self.steps.clone(),
            edges: self.edges.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::sequence::SequenceSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            steps: snapshot.steps,
            edges: snapshot.edges,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::sequence::SequenceSnapshot) {
        self.schema = snapshot.schema;
        self.steps = snapshot.steps;
        self.edges = snapshot.edges;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.sequence.sequence` — twenty handcrafted schema leaves.
pub fn sequence_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.sequence.sequence",
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
        mutations: schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️component.rs"),
            typescript: include_str!("🧬️mutations/🟦️component.ts"),
            graphql: include_str!("🧬️mutations/🔗️component.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️component.json"),
            proto: include_str!("🧬️mutations/🛰️component.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
