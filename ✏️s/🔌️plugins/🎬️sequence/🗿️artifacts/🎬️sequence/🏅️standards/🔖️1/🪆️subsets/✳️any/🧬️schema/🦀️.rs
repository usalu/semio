//! 🧬️ Sequence artifact schema — every field of the artifact with its state class.

use crate::{default_snapshot, SequenceContentChild, SequenceMutation, SequenceSnapshot, SEQUENCE_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;


//#region 🔖️Artifact
/// 🧬️ sequence document artifact state. Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`sequence→C:flow`): `steps`/`edges` are replaced
/// by the same composed `content` CHILD slot `SequenceSnapshot` carries, mirroring `WriterArtifact`/
/// `FlowArtifact`'s precedent so `to_snapshot`/`from_snapshot`/`set_snapshot` stay consistent.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.sequence.sequence")]
pub struct SequenceArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub content: SequenceContentChild,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for SequenceArtifact {
    fn default() -> Self {
        Self { schema: SEQUENCE_DOCUMENT_SCHEMA.into(), content: crate::sequence_content_child_with_owner(Vec::new(), Vec::new()) }
    }
}

impl SequenceArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> SequenceSnapshot {
        SequenceSnapshot { schema: self.schema.clone(), content: self.content.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: SequenceSnapshot) -> Self {
        Self { schema: snapshot.schema, content: snapshot.content }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: SequenceSnapshot) {
        self.schema = snapshot.schema;
        self.content = snapshot.content;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.sequence.sequence` — twenty handcrafted schema leaves.
pub fn sequence_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.sequence.sequence",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🔖️Example

//#endregion 🔖️Example

//#region 🏗️Construction
/// 🏗️ W1-C's generic `SnapshotBuilder<Snapshot, Mutation>` (design.md §5 step 3) — replaces the
/// deleted `derive_artifact_facets!`-generated `SequenceBuilder`/`SequenceAnalyzer`/
/// `SequenceComposer` cluster outright: construction is a plain snapshot+mutation build (no custom
/// analysis/composition logic this subset needs beyond the ordinary `Mutation`/`MutationDiff`
/// algebra), so the trivial-subset shape applies verbatim.
pub type Construction = semio_framework_plugin::app::SnapshotBuilder<SequenceSnapshot, SequenceMutation>;
//#endregion 🏗️Construction

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::SequenceCamera;
//#endregion 🔁️Re-exports
