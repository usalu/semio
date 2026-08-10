//! 🏗️ Fem2dBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::fem2d::{Fem2dDiff, Fem2dMutation, Fem2dSnapshot};
use crate::artifacts::fem2d::standards::v1::subsets::any::builder::Fem2dBuilder as Fem2dAnyBuilder;

#[derive(Clone, Debug)]
pub struct Fem2dBuilder(Fem2dAnyBuilder);

impl ArtifactBuilder for Fem2dBuilder {
    type Snapshot = Fem2dSnapshot;
    type Mutation = Fem2dMutation;
    type Diff = Fem2dDiff;
    fn empty() -> Self { Self(Fem2dAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Fem2dAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Fem2dAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Fem2dAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
