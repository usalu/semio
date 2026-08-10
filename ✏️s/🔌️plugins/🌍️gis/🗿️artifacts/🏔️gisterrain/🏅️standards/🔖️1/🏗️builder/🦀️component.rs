//! 🏗️ GisTerrainBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::gisterrain::{GisTerrainDiff, GisTerrainMutation, GisTerrainSnapshot};
use crate::artifacts::gisterrain::standards::v1::subsets::any::builder::GisterrainBuilder as GisTerrainAnyBuilder;

#[derive(Clone, Debug)]
pub struct GisTerrainBuilder(GisTerrainAnyBuilder);

impl ArtifactBuilder for GisTerrainBuilder {
    type Snapshot = GisTerrainSnapshot;
    type Mutation = GisTerrainMutation;
    type Diff = GisTerrainDiff;
    fn empty() -> Self { Self(GisTerrainAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(GisTerrainAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(GisTerrainAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(GisTerrainAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
