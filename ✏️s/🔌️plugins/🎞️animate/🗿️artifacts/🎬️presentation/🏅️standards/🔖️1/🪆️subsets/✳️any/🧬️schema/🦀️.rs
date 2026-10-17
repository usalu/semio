//! 🧬️ Presentation artifact schema — every field of the artifact with its state class.

use crate::{AnimationChild, PresentationChild};
use schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ presentation document artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.animate.presentation")]
pub struct PresentationArtifact {
    #[state(artifact)]
    pub schema: String,
    /// 🖼️ See `PresentationSnapshot::source` — the persisted payload the composed `presentation`
    /// deck is derived from.
    #[state(artifact)]
    pub source: crate::FigureTileSource,
    /// 🧱 See `PresentationSnapshot::tiles`.
    #[state(artifact)]
    pub tiles: Vec<crate::FigureTileDraft>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub presentation: PresentationChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub animation: AnimationChild,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for PresentationArtifact {
    fn default() -> Self {
        Self::from_snapshot(crate::default_snapshot())
    }
}

impl PresentationArtifact {
    /// 📸️ Persisted subset.
    pub fn to_snapshot(&self) -> crate::PresentationSnapshot {
        crate::PresentationSnapshot { schema: self.schema.clone(), source: self.source.clone(), tiles: self.tiles.clone(), presentation: self.presentation.clone(), animation: self.animation.clone() }
    }

    /// 🧬️ Builds the document artifact from its snapshot.
    pub fn from_snapshot(snapshot: crate::PresentationSnapshot) -> Self {
        Self { schema: snapshot.schema, source: snapshot.source, tiles: snapshot.tiles, presentation: snapshot.presentation, animation: snapshot.animation }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    pub fn set_snapshot(&mut self, snapshot: crate::PresentationSnapshot) {
        self.schema = snapshot.schema;
        self.source = snapshot.source;
        self.tiles = snapshot.tiles;
        self.presentation = snapshot.presentation;
        self.animation = snapshot.animation;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.animate.presentation` — twenty handcrafted schema leaves.
pub fn presentation_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.animate.presentation",
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
//#region 🏗️Construction
/// 🏗️ Replaces the deleted `derive_artifact_facets!`-generated `PresentationBuilder`/`PresentationAnalyzer`/
/// `PresentationComposer` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §2) — the
/// generic construction/mutation-application path every trivial subset (no custom analysis/
/// composition logic beyond ordinary `Mutation`/`MutationDiff` algebra) now uses, mirroring `🎬️sequence`'s identical shape.
pub type Construction = semio_framework_plugin::app::SnapshotBuilder<crate::PresentationSnapshot, crate::PresentationMutation>;
//#endregion 🏗️Construction

//#region 🔖️Error
/// 🎞️ Errors from presentation deck envelope materialization (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES: split from the former engine-tree
/// `PresentationError`, which mixed this schema-tier envelope-replay concern with app-tier video-export
/// concerns — an artifact must never depend on an app, so the video-only variants
/// (`NoSceneHashes`/`Compile`) moved to `crate::editor::animate::engine`'s own `PresentationVideoExportError`
/// instead of being kept here).
#[derive(Debug)]
pub enum PresentationError {
    /// 🧾️ The stored envelope JSON was malformed.
    DeserializeEnvelope(semio_framework_value::ValueError),
    /// 🧬️ Whole-buffer envelope ingress was rejected in favor of the persistent fixed-page decoder.
    EnvelopeIngress(store::os_store::owned_schema_decode::ArtifactEnvelopeWholeBufferIngressError),
    /// 📐️ VCS replay failed while materializing the projection.
    Vcs(vcs::VcsError),
}

impl std::fmt::Display for PresentationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeserializeEnvelope(error) => write!(formatter, "deserialize envelope: {error}"),
            Self::EnvelopeIngress(error) => write!(formatter, "{error}"),
            Self::Vcs(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for PresentationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DeserializeEnvelope(error) => Some(error),
            Self::EnvelopeIngress(error) => Some(error),
            Self::Vcs(error) => std::error::Error::source(error),
        }
    }
}

impl From<semio_framework_value::ValueError> for PresentationError {
    fn from(error: semio_framework_value::ValueError) -> Self {
        Self::DeserializeEnvelope(error)
    }
}

impl From<store::os_store::owned_schema_decode::ArtifactEnvelopeWholeBufferIngressError> for PresentationError {
    fn from(error: store::os_store::owned_schema_decode::ArtifactEnvelopeWholeBufferIngressError) -> Self {
        Self::EnvelopeIngress(error)
    }
}

impl From<vcs::VcsError> for PresentationError {
    fn from(error: vcs::VcsError) -> Self {
        Self::Vcs(error)
    }
}
//#endregion 🔖️Error

//#region 🔖️DocumentHelpers
/// 📄️ Empty presentation deck — the wasm VCS bridge's default projection for a fresh envelope.
pub fn empty_presentation_snapshot() -> crate::PresentationSnapshot {
    crate::presentation_snapshot_with_tiles(&crate::default_figure_tile_source(), &[])
}

//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
#[path = "🧪️tests/🪪️document-contract/🦀️.rs"]
mod document_contract_tests;
