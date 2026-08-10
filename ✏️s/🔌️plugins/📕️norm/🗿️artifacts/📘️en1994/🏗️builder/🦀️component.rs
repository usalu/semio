//! 🏗️ En1994Builder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};
use crate::artifacts::en1994::standards::v1::builder::En1994Builder as En1994RawBuilder;

#[derive(Clone, Debug)]
pub struct En1994Builder(En1994RawBuilder);

impl ArtifactBuilder for En1994Builder {
    type Snapshot = En1994Snapshot;
    type Mutation = En1994Mutation;
    type Diff = En1994Diff;
    fn empty() -> Self { Self(En1994RawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(En1994RawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(En1994RawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(En1994RawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
