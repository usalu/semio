//! 🏗️ GisMapBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::gismap::{GisMapDiff, GisMapMutation, GisMapSnapshot};
use crate::artifacts::gismap::standards::v1::subsets::any::builder::GismapBuilder as GisMapAnyBuilder;

#[derive(Clone, Debug)]
pub struct GisMapBuilder(GisMapAnyBuilder);

impl ArtifactBuilder for GisMapBuilder {
    type Snapshot = GisMapSnapshot;
    type Mutation = GisMapMutation;
    type Diff = GisMapDiff;
    fn empty() -> Self { Self(GisMapAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(GisMapAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(GisMapAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(GisMapAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
