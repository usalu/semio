//! 🏗️ Fem3dBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::fem3d::{Fem3dDiff, Fem3dMutation, Fem3dSnapshot};
use crate::artifacts::fem3d::standards::v1::subsets::any::builder::Fem3dBuilder as Fem3dAnyBuilder;

#[derive(Clone, Debug)]
pub struct Fem3dBuilder(Fem3dAnyBuilder);

impl ArtifactBuilder for Fem3dBuilder {
    type Snapshot = Fem3dSnapshot;
    type Mutation = Fem3dMutation;
    type Diff = Fem3dDiff;
    fn empty() -> Self { Self(Fem3dAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Fem3dAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Fem3dAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Fem3dAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
