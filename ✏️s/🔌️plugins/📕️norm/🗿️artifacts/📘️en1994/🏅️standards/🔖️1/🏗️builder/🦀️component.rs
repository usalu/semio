//! 🏗️ En1994Builder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};
use crate::artifacts::en1994::standards::v1::subsets::any::builder::En1994Builder as En1994AnyBuilder;

#[derive(Clone, Debug)]
pub struct En1994Builder(En1994AnyBuilder);

impl ArtifactBuilder for En1994Builder {
    type Snapshot = En1994Snapshot;
    type Mutation = En1994Mutation;
    type Diff = En1994Diff;
    fn empty() -> Self { Self(En1994AnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(En1994AnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(En1994AnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(En1994AnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
