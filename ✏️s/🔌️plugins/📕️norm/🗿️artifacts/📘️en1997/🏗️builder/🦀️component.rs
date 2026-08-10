//! 🏗️ En1997Builder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1997::{En1997Diff, En1997Mutation, En1997Snapshot};
use crate::artifacts::en1997::standards::v1::builder::En1997Builder as En1997RawBuilder;

#[derive(Clone, Debug)]
pub struct En1997Builder(En1997RawBuilder);

impl ArtifactBuilder for En1997Builder {
    type Snapshot = En1997Snapshot;
    type Mutation = En1997Mutation;
    type Diff = En1997Diff;
    fn empty() -> Self { Self(En1997RawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(En1997RawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(En1997RawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(En1997RawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
