//! 🧬️ Sequence artifact schema — every field of the artifact with its state class.

use crate::artifacts::sequence::{default_snapshot, SequenceCamera, SequenceContentChild, SequenceMutation, SequenceSnapshot, SEQUENCE_DOCUMENT_SCHEMA};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};
use store::ArtifactDsl;

//#region 🔖️Artifact
/// 🧬️ Full sequence artifact state across the artifact, presence and config lanes. Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`sequence→C:flow`): `steps`/`edges` are replaced
/// by the same composed `content` CHILD slot `SequenceSnapshot` carries, mirroring `WriterArtifact`/
/// `FlowArtifact`'s precedent so `to_snapshot`/`from_snapshot`/`set_snapshot` stay consistent.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.sequence.sequence")]
pub struct SequenceArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio.flow")]
    pub content: SequenceContentChild,
    #[state(config)]
    pub last_run_json: String,
    #[state(config)]
    pub orientation: String,
    #[state(config)]
    pub camera: SequenceCamera,
    #[state(config)]
    pub locale: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for SequenceArtifact {
    fn default() -> Self {
        Self {
            schema: SEQUENCE_DOCUMENT_SCHEMA.into(),
            content: crate::artifacts::sequence::sequence_content_child_with_owner(Vec::new(), Vec::new()),
            last_run_json: String::new(),
            orientation: "leftRight".into(),
            camera: SequenceCamera::default(),
            locale: "en-US".into(),
        }
    }
}

impl SequenceArtifact {
    /// 📸️ Persisted subset.
    pub async fn to_snapshot(&self) -> SequenceSnapshot {
        SequenceSnapshot { schema: self.schema.clone(), content: self.content.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot, leaving UI fields at defaults.
    pub async fn from_snapshot(snapshot: SequenceSnapshot) -> Self {
        Self { schema: snapshot.schema, content: snapshot.content, last_run_json: String::new(), orientation: "leftRight".into(), camera: SequenceCamera::default(), locale: "en-US".into() }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub async fn set_snapshot(&mut self, snapshot: SequenceSnapshot) {
        self.schema = snapshot.schema;
        self.content = snapshot.content;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.sequence.sequence` — twenty handcrafted schema leaves.
pub async fn sequence_artifact_schema_descriptor() -> schema::ArtifactSchemaDescriptor {
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

//#region 🔖️Example
/// 📄️ JSON re-serialization of `default_snapshot()`, round-tripped through its own `.sequence` DSL
/// first (see `crate::artifacts::sequence::dsl`), to prove the fixture is fully expressible in text —
/// for the framework-generic call site that contractually requires JSON (`App::example`'s manifest
/// `document_json` is loaded via `serde_json::from_str` by `ArtifactApp::load_document`'s default impl)
/// — out of scope to change, since both are defined in `framework/plugin`.
pub async fn sequence_example_json() -> String {
    let fixture = <SequenceSnapshot as ArtifactDsl>::parse_dsl(&default_snapshot().print_dsl()).expect("default_snapshot round-trips through its own DSL");
    serde_json::to_string(&fixture).expect("default_snapshot is a static, hand-built value with no non-finite floats or non-UTF8 keys")
}
//#endregion 🔖️Example

//#region 🏗️Construction
/// 🏗️ W1-C's generic `SnapshotBuilder<Snapshot, Mutation>` (design.md §5 step 3) — replaces the
/// deleted `derive_artifact_facets!`-generated `SequenceBuilder`/`SequenceAnalyzer`/
/// `SequenceComposer` cluster outright: construction is a plain snapshot+mutation build (no custom
/// analysis/composition logic this subset needs beyond the ordinary `Mutation`/`MutationDiff`
/// algebra), so the trivial-subset shape applies verbatim.
pub type Construction = semio_framework_plugin::app::SnapshotBuilder<SequenceSnapshot, SequenceMutation>;
//#endregion 🏗️Construction
