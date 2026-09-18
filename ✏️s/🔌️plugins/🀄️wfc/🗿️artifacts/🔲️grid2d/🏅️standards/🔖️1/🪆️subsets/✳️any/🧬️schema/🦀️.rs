//! 🧬️ `s.wfc.grid2d` artifact schema — the persisted WFC problem for a regular 2D grid, and the
//! four handcrafted descriptor facets (artifact/snapshot/diff/mutations) the declaration tree binds.

use crate::schema::mutations::Grid2dMutation;
use crate::schema::snapshot::Grid2dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Grid2dArtifact
/// 🧬️ Grid2dArtifact facet — the persisted problem spec IS the artifact; the solved assignment is
/// an inference over it, never a field.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.wfc.grid2d")]
pub struct Grid2dArtifact {
    #[state(artifact)]
    pub snapshot: Grid2dSnapshot,
}

impl Grid2dArtifact {
    pub fn to_snapshot(&self) -> Grid2dSnapshot {
        self.snapshot.clone()
    }

    pub fn from_snapshot(snapshot: Grid2dSnapshot) -> Self {
        Self { snapshot }
    }
}
//#endregion 🔖️Grid2dArtifact

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.wfc.grid2d` — twenty handcrafted schema leaves, four facets by five
/// languages, every one of them `include_str!`-backed and hand-authored (there is no generator).
pub fn grid2d_artifact_schema_descriptor() -> ::semio_framework_schema::ArtifactSchemaDescriptor {
    ::semio_framework_schema::ArtifactSchemaDescriptor {
        id: "s.wfc.grid2d",
        artifact: ::semio_framework_schema::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor

//#region 🏗️Construction
/// 🏗️ This subset needs no custom build/analysis logic beyond the ordinary `Mutation`/
/// `MutationDiff` algebra, so the generic `SnapshotBuilder<S, M>` is the whole construction facet;
/// all io goes exclusively through the `io_mechanism` registry (`🚪️io/🦀️.rs`'s `io()`), never the
/// retired `ComposerEntry` channel.
pub type Construction = semio_framework_plugin::app::SnapshotBuilder<Grid2dSnapshot, Grid2dMutation>;
//#endregion 🏗️Construction
