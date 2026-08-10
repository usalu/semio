//! 🏗️ En1992Builder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1992::{En1992Diff, En1992Mutation, En1992Snapshot};
use crate::artifacts::en1992::standards::v1::subsets::any::builder::En1992Builder as En1992AnyBuilder;

#[derive(Clone, Debug)]
pub struct En1992Builder(En1992AnyBuilder);

impl ArtifactBuilder for En1992Builder {
    type Snapshot = En1992Snapshot;
    type Mutation = En1992Mutation;
    type Diff = En1992Diff;
    fn empty() -> Self { Self(En1992AnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(En1992AnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(En1992AnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(En1992AnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
