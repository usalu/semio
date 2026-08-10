//! 🏗️ Vdi3805Builder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};
use crate::artifacts::vdi3805::standards::v1::builder::Vdi3805Builder as Vdi3805RawBuilder;

#[derive(Clone, Debug)]
pub struct Vdi3805Builder(Vdi3805RawBuilder);

impl ArtifactBuilder for Vdi3805Builder {
    type Snapshot = Vdi3805Snapshot;
    type Mutation = Vdi3805Mutation;
    type Diff = Vdi3805Diff;
    fn empty() -> Self { Self(Vdi3805RawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Vdi3805RawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Vdi3805RawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Vdi3805RawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
