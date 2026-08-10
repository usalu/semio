//! 🏗️ En1995Builder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1995::{En1995Diff, En1995Mutation, En1995Snapshot};
use crate::artifacts::en1995::standards::v1::subsets::any::builder::En1995Builder as En1995AnyBuilder;

#[derive(Clone, Debug)]
pub struct En1995Builder(En1995AnyBuilder);

impl ArtifactBuilder for En1995Builder {
    type Snapshot = En1995Snapshot;
    type Mutation = En1995Mutation;
    type Diff = En1995Diff;
    fn empty() -> Self { Self(En1995AnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(En1995AnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(En1995AnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(En1995AnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
