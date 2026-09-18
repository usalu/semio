//! 🧬️ Bitmap artifact schema — the persisted overlapping-model problem and its descriptor leaves.

use crate::schema::snapshot::BitmapSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️BitmapArtifact
/// 🧬️ BitmapArtifact facet — the persisted problem spec is the artifact; the solved output bitmap,
/// the contradiction verdict and the entropy map are inferences, never state.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.wfc.bitmap")]
pub struct BitmapArtifact {
    #[state(artifact)]
    pub snapshot: BitmapSnapshot,
}

impl BitmapArtifact {
    pub fn to_snapshot(&self) -> BitmapSnapshot {
        self.snapshot.clone()
    }

    pub fn from_snapshot(snapshot: BitmapSnapshot) -> Self {
        Self { snapshot }
    }
}
//#endregion 🔖️BitmapArtifact

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.wfc.bitmap` — twenty handcrafted schema leaves.
pub fn bitmap_artifact_schema_descriptor() -> ::semio_framework_schema::ArtifactSchemaDescriptor {
    ::semio_framework_schema::ArtifactSchemaDescriptor {
        id: "s.wfc.bitmap",
        artifact: ::semio_framework_schema::FacetLeaves {
            rust: include_str!("🦀️.rs"),
            typescript: include_str!("🟦️.ts"),
            graphql: include_str!("🔗️.graphql"),
            json_schema: include_str!("🔣️.json"),
            proto: include_str!("🛰️.proto"),
        },
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
/// 🏗️ The framework's own generic snapshot builder is this artifact's construction facet. The old
/// hand-rolled `derived_construction`/`derived_analysis`/`derive_artifact_facets!` chain, and the
/// `ArtifactComposition` dispatch it fed, are not used here at all: every io hop goes through the
/// `io_mechanism` registry `🚪️io/🦀️.rs`'s `io()` returns, which is the one channel a subset
/// declaration reads.
pub type Construction = semio_framework_plugin::app::SnapshotBuilder<BitmapSnapshot, crate::BitmapMutation>;
//#endregion 🏗️Construction
