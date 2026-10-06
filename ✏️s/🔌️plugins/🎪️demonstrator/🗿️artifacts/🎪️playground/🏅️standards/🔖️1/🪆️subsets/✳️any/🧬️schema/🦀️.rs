//! 🧬️ Playground artifact schema — every field of the artifact with its state class.

use schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full playground artifact state (artifact-lane fields only today).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.demonstrator.playground")]
pub struct PlaygroundArtifact {
    #[state(artifact)]
    pub schema: String,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for PlaygroundArtifact {
    fn default() -> Self {
        Self { schema: crate::PLAYGROUND_DOCUMENT_SCHEMA.into() }
    }
}

impl PlaygroundArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot {
        crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot { schema: self.schema.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    pub fn from_snapshot(snapshot: crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot) -> Self {
        Self { schema: snapshot.schema }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot) {
        self.schema = snapshot.schema;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️DocumentHelpers
/// 🏗️ Empty default playground snapshot (relocated from the deleted `⚙️engine`, ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES: pure document helper, no `&mut self`,
/// no app type — belongs beside the snapshot it builds).
pub fn empty_playground_snapshot() -> crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot {
    crate::standards::v1::subsets::any::schema::snapshot::PlaygroundSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.demonstrator.playground` — twenty handcrafted schema leaves.
pub fn playground_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.demonstrator.playground",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"), typescript: include_str!("🔺️diff/🟦️.ts"), graphql: include_str!("🔺️diff/🔗️.graphql"), json_schema: include_str!("🔺️diff/🔣️.json"), proto: include_str!("🔺️diff/🛰️.proto")
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
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
