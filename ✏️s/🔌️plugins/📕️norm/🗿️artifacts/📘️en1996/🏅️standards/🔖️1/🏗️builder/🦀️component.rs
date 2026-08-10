//! 🏗️ En1996Builder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1996::{En1996Diff, En1996Mutation, En1996Snapshot};
use crate::artifacts::en1996::standards::v1::subsets::any::builder::En1996Builder as En1996AnyBuilder;

#[derive(Clone, Debug)]
pub struct En1996Builder(En1996AnyBuilder);

impl ArtifactBuilder for En1996Builder {
    type Snapshot = En1996Snapshot;
    type Mutation = En1996Mutation;
    type Diff = En1996Diff;
    fn empty() -> Self { Self(En1996AnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(En1996AnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(En1996AnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(En1996AnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
