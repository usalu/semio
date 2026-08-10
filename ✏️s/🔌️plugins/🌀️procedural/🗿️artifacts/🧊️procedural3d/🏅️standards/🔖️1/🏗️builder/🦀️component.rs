//! 🏗️ Procedural3dBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::procedural3d::{Procedural3dDiff, Procedural3dMutation, Procedural3dSnapshot};
use crate::artifacts::procedural3d::standards::v1::subsets::any::builder::Procedural3dBuilder as Procedural3dAnyBuilder;

#[derive(Clone, Debug)]
pub struct Procedural3dBuilder(Procedural3dAnyBuilder);

impl ArtifactBuilder for Procedural3dBuilder {
    type Snapshot = Procedural3dSnapshot;
    type Mutation = Procedural3dMutation;
    type Diff = Procedural3dDiff;
    fn empty() -> Self { Self(Procedural3dAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Procedural3dAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Procedural3dAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Procedural3dAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
