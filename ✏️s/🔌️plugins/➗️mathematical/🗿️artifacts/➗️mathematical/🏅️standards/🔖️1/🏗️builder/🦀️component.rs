//! 🏗️ MathematicalBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalMutation, MathematicalSnapshot};
use crate::artifacts::mathematical::standards::v1::subsets::any::builder::MathematicalBuilder as MathematicalAnyBuilder;

#[derive(Clone, Debug)]
pub struct MathematicalBuilder(MathematicalAnyBuilder);

impl ArtifactBuilder for MathematicalBuilder {
    type Snapshot = MathematicalSnapshot;
    type Mutation = MathematicalMutation;
    type Diff = MathematicalDiff;
    fn empty() -> Self { Self(MathematicalAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(MathematicalAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(MathematicalAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(MathematicalAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
