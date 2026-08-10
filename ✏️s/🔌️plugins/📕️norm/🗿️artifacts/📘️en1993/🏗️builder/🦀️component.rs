//! 🏗️ En1993Builder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1993::{En1993Diff, En1993Mutation, En1993Snapshot};
use crate::artifacts::en1993::standards::v1::builder::En1993Builder as En1993RawBuilder;

#[derive(Clone, Debug)]
pub struct En1993Builder(En1993RawBuilder);

impl ArtifactBuilder for En1993Builder {
    type Snapshot = En1993Snapshot;
    type Mutation = En1993Mutation;
    type Diff = En1993Diff;
    fn empty() -> Self { Self(En1993RawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(En1993RawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(En1993RawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(En1993RawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
