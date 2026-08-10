//! 🏗️ PlaygroundBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::playground::{PlaygroundDiff, PlaygroundMutation, PlaygroundSnapshot};
use crate::artifacts::playground::standards::v1::builder::PlaygroundBuilder as PlaygroundRawBuilder;

#[derive(Clone, Debug)]
pub struct PlaygroundBuilder(PlaygroundRawBuilder);

impl ArtifactBuilder for PlaygroundBuilder {
    type Snapshot = PlaygroundSnapshot;
    type Mutation = PlaygroundMutation;
    type Diff = PlaygroundDiff;
    fn empty() -> Self { Self(PlaygroundRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(PlaygroundRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(PlaygroundRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(PlaygroundRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
