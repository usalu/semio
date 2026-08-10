//! 🏗️ En1998Builder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1998::{En1998Diff, En1998Mutation, En1998Snapshot};
use crate::artifacts::en1998::standards::v1::builder::En1998Builder as En1998RawBuilder;

#[derive(Clone, Debug)]
pub struct En1998Builder(En1998RawBuilder);

impl ArtifactBuilder for En1998Builder {
    type Snapshot = En1998Snapshot;
    type Mutation = En1998Mutation;
    type Diff = En1998Diff;
    fn empty() -> Self { Self(En1998RawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(En1998RawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(En1998RawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(En1998RawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
