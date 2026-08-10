//! 🏗️ En1998Builder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1998::{En1998Diff, En1998Mutation, En1998Snapshot};
use crate::artifacts::en1998::standards::v1::subsets::any::builder::En1998Builder as En1998AnyBuilder;

#[derive(Clone, Debug)]
pub struct En1998Builder(En1998AnyBuilder);

impl ArtifactBuilder for En1998Builder {
    type Snapshot = En1998Snapshot;
    type Mutation = En1998Mutation;
    type Diff = En1998Diff;
    fn empty() -> Self { Self(En1998AnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(En1998AnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(En1998AnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(En1998AnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
