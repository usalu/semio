//! 🏗️ En1991Builder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};
use crate::artifacts::en1991::standards::v1::subsets::any::builder::En1991Builder as En1991AnyBuilder;

#[derive(Clone, Debug)]
pub struct En1991Builder(En1991AnyBuilder);

impl ArtifactBuilder for En1991Builder {
    type Snapshot = En1991Snapshot;
    type Mutation = En1991Mutation;
    type Diff = En1991Diff;
    fn empty() -> Self { Self(En1991AnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(En1991AnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(En1991AnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(En1991AnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
