//! 🏗️ GisTerrainBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::gisterrain::{GisTerrainDiff, GisTerrainMutation, GisTerrainSnapshot};
use crate::artifacts::gisterrain::standards::v1::builder::GisTerrainBuilder as GisTerrainRawBuilder;

#[derive(Clone, Debug)]
pub struct GisTerrainBuilder(GisTerrainRawBuilder);

impl ArtifactBuilder for GisTerrainBuilder {
    type Snapshot = GisTerrainSnapshot;
    type Mutation = GisTerrainMutation;
    type Diff = GisTerrainDiff;
    fn empty() -> Self { Self(GisTerrainRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(GisTerrainRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(GisTerrainRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(GisTerrainRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
