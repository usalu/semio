//! 🧬️ VCS artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️DocumentHelpers
/// 🌱️ The artifact's empty/default snapshot — used as `VcsPlayApp::initial_snapshot()` and by every
/// test fixture that needs a base document (was: `⚙️engine::empty_vcs_snapshot()`, dissolved per ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
pub fn empty_vcs_snapshot() -> crate::artifacts::vcs::VcsSnapshot {
    crate::artifacts::vcs::VcsSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Artifact
/// 🧬️ Full VCS demo artifact state across the artifact, presence and config lanes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.vcs.vcs")]
pub struct VcsArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub title: String,
    #[state(artifact)]
    pub counter: i64,
    #[state(artifact)]
    pub notes: String,
    #[state(artifact)]
    pub status: String,
    #[state(artifact)]
    #[serde(default)]
    pub tags: Vec<String>,
    #[state(presence)]
    #[serde(default)]
    pub selected_checkpoint_ids: Vec<String>,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for VcsArtifact {
    fn default() -> Self {
        Self { schema: crate::artifacts::vcs::VCS_DOCUMENT_SCHEMA.into(), title: "VCS Demo".into(), counter: 0, notes: String::new(), status: "new".into(), tags: Vec::new(), selected_checkpoint_ids: Vec::new(), locale: "en-US".into() }
    }
}

impl VcsArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::artifacts::vcs::VcsSnapshot {
        crate::artifacts::vcs::VcsSnapshot { schema: self.schema.clone(), title: self.title.clone(), counter: self.counter, notes: self.notes.clone(), status: self.status.clone(), tags: self.tags.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub fn from_snapshot(snapshot: crate::artifacts::vcs::VcsSnapshot) -> Self {
        Self { schema: snapshot.schema, title: snapshot.title, counter: snapshot.counter, notes: snapshot.notes, status: snapshot.status, tags: snapshot.tags, ..Self::default() }
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
//#region 🏗️Construction
/// 🏗️ W1-C's generic `SnapshotBuilder<Snapshot, Mutation>` (design.md §5 step 3) — replaces the
/// deleted `derive_artifact_facets!`-generated `VcsBuilder`/`VcsAnalyzer`/`VcsComposer` cluster
/// (`derived_construction`/`derived_analysis`/`derived_composition`, all confirmed dead —
/// zero repo-wide references outside this plugin) with the ordinary `Mutation`/`MutationDiff`
/// algebra this subset needs; all io now goes exclusively through `io::io()` (design.md rule 3).
pub type Construction = semio_framework_plugin::app::SnapshotBuilder<crate::artifacts::vcs::VcsSnapshot, crate::artifacts::vcs::VcsDemoMutation>;
//#endregion 🏗️Construction

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_vcs_snapshot();
        assert_eq!(snapshot.schema, crate::artifacts::vcs::VCS_DOCUMENT_SCHEMA);
        assert_eq!(snapshot.status, "new");
    }
}
//#endregion 🧪️Tests
