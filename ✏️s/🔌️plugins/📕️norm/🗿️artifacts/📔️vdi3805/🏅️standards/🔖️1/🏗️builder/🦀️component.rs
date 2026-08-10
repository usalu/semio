//! 🏗️ Vdi3805Builder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};
use crate::artifacts::vdi3805::standards::v1::subsets::any::builder::Vdi3805Builder as Vdi3805AnyBuilder;

#[derive(Clone, Debug)]
pub struct Vdi3805Builder(Vdi3805AnyBuilder);

impl ArtifactBuilder for Vdi3805Builder {
    type Snapshot = Vdi3805Snapshot;
    type Mutation = Vdi3805Mutation;
    type Diff = Vdi3805Diff;
    fn empty() -> Self { Self(Vdi3805AnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Vdi3805AnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Vdi3805AnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Vdi3805AnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
