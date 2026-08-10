//! 🏗️ Iso16757Builder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Mutation, Iso16757Snapshot};
use crate::artifacts::iso16757::standards::v1::builder::Iso16757Builder as Iso16757RawBuilder;

#[derive(Clone, Debug)]
pub struct Iso16757Builder(Iso16757RawBuilder);

impl ArtifactBuilder for Iso16757Builder {
    type Snapshot = Iso16757Snapshot;
    type Mutation = Iso16757Mutation;
    type Diff = Iso16757Diff;
    fn empty() -> Self { Self(Iso16757RawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Iso16757RawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Iso16757RawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Iso16757RawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
