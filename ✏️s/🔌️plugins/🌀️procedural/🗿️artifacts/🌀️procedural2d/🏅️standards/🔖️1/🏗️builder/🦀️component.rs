//! 🏗️ Procedural2dBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::procedural2d::{Procedural2dDiff, Procedural2dMutation, Procedural2dSnapshot};
use crate::artifacts::procedural2d::standards::v1::subsets::any::builder::Procedural2dBuilder as Procedural2dAnyBuilder;

#[derive(Clone, Debug)]
pub struct Procedural2dBuilder(Procedural2dAnyBuilder);

impl ArtifactBuilder for Procedural2dBuilder {
    type Snapshot = Procedural2dSnapshot;
    type Mutation = Procedural2dMutation;
    type Diff = Procedural2dDiff;
    fn empty() -> Self { Self(Procedural2dAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Procedural2dAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Procedural2dAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Procedural2dAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
