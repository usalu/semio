//! 🏗️ En1996Builder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1996::{En1996Diff, En1996Mutation, En1996Snapshot};
use crate::artifacts::en1996::standards::v1::builder::En1996Builder as En1996RawBuilder;

#[derive(Clone, Debug)]
pub struct En1996Builder(En1996RawBuilder);

impl ArtifactBuilder for En1996Builder {
    type Snapshot = En1996Snapshot;
    type Mutation = En1996Mutation;
    type Diff = En1996Diff;
    fn empty() -> Self { Self(En1996RawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(En1996RawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(En1996RawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(En1996RawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
