//! 🏗️ En1991Builder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};
use crate::artifacts::en1991::standards::v1::builder::En1991Builder as En1991RawBuilder;

#[derive(Clone, Debug)]
pub struct En1991Builder(En1991RawBuilder);

impl ArtifactBuilder for En1991Builder {
    type Snapshot = En1991Snapshot;
    type Mutation = En1991Mutation;
    type Diff = En1991Diff;
    fn empty() -> Self { Self(En1991RawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(En1991RawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(En1991RawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(En1991RawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
