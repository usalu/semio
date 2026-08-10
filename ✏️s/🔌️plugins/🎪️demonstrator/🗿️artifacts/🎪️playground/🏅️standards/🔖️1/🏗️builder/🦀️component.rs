//! 🏗️ PlaygroundBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::playground::{PlaygroundDiff, PlaygroundMutation, PlaygroundSnapshot};
use crate::artifacts::playground::standards::v1::subsets::any::builder::PlaygroundBuilder as PlaygroundAnyBuilder;

#[derive(Clone, Debug)]
pub struct PlaygroundBuilder(PlaygroundAnyBuilder);

impl ArtifactBuilder for PlaygroundBuilder {
    type Snapshot = PlaygroundSnapshot;
    type Mutation = PlaygroundMutation;
    type Diff = PlaygroundDiff;
    fn empty() -> Self { Self(PlaygroundAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(PlaygroundAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(PlaygroundAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(PlaygroundAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
