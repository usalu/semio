//! 🧬️ VCS artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Artifact
/// 🧬️ Full VCS demo artifact state across persistent, shared-ui and local-ui classes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.vcs.vcs")]
pub struct VcsArtifact {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    pub title: String,
    #[state(persistent)]
    pub counter: i64,
    #[state(persistent)]
    pub notes: String,
    #[state(persistent)]
    pub status: String,
    #[state(persistent)]
    #[serde(default)]
    pub tags: Vec<String>,
    #[state(shared_ui)]
    #[serde(default)]
    pub selected_checkpoint_ids: Vec<String>,
    #[state(local_ui)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for VcsArtifact {
    fn default() -> Self {
        Self {
            schema: crate::artifacts::vcs::VCS_DOCUMENT_SCHEMA.into(),
            title: "VCS Demo".into(),
            counter: 0,
            notes: String::new(),
            status: "new".into(),
            tags: Vec::new(),
            selected_checkpoint_ids: Vec::new(),
            locale: "en-US".into(),
        }
    }
}

impl VcsArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::vcs::VcsSnapshot {
        crate::artifacts::vcs::VcsSnapshot {
            schema: self.schema.clone(),
            title: self.title.clone(),
            counter: self.counter,
            notes: self.notes.clone(),
            status: self.status.clone(),
            tags: self.tags.clone(),
        }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::vcs::VcsSnapshot) -> Self {
        Self {
            schema: snapshot.schema,
            title: snapshot.title,
            counter: snapshot.counter,
            notes: snapshot.notes,
            status: snapshot.status,
            tags: snapshot.tags,
            ..Self::default()
        }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::vcs::VcsSnapshot) {
        self.schema = snapshot.schema;
        self.title = snapshot.title;
        self.counter = snapshot.counter;
        self.notes = snapshot.notes;
        self.status = snapshot.status;
        self.tags = snapshot.tags;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.vcs.vcs` — twenty handcrafted schema leaves.
pub fn vcs_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
    schema::ArtifactSchemaDescriptor {
        id: "s.vcs.vcs",
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
