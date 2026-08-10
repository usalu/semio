//! 🏗️ LowpolyBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::lowpoly::{LowpolyDiff, LowpolyMutation, LowpolySnapshot};
use crate::artifacts::lowpoly::standards::v1::subsets::any::builder::LowpolyBuilder as LowpolyAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct LowpolyBuilder(LowpolyAnyBuilder);

impl ArtifactBuilder for LowpolyBuilder {
    type Snapshot = LowpolySnapshot;
    type Mutation = LowpolyMutation;
    type Diff = LowpolyDiff;
    fn empty() -> Self { Self(LowpolyAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(LowpolyAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(LowpolyAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(LowpolyAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
