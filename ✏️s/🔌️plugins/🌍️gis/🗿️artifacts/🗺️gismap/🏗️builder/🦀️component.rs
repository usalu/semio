//! 🏗️ GisMapBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::gismap::{GisMapDiff, GisMapMutation, GisMapSnapshot};
use crate::artifacts::gismap::standards::v1::builder::GisMapBuilder as GisMapRawBuilder;

#[derive(Clone, Debug)]
pub struct GisMapBuilder(GisMapRawBuilder);

impl ArtifactBuilder for GisMapBuilder {
    type Snapshot = GisMapSnapshot;
    type Mutation = GisMapMutation;
    type Diff = GisMapDiff;
    fn empty() -> Self { Self(GisMapRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(GisMapRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(GisMapRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(GisMapRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
