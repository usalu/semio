//! 🏗️ En1999Builder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1999::{En1999Diff, En1999Mutation, En1999Snapshot};
use crate::artifacts::en1999::standards::v1::builder::En1999Builder as En1999RawBuilder;

#[derive(Clone, Debug)]
pub struct En1999Builder(En1999RawBuilder);

impl ArtifactBuilder for En1999Builder {
    type Snapshot = En1999Snapshot;
    type Mutation = En1999Mutation;
    type Diff = En1999Diff;
    fn empty() -> Self { Self(En1999RawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(En1999RawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(En1999RawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(En1999RawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
