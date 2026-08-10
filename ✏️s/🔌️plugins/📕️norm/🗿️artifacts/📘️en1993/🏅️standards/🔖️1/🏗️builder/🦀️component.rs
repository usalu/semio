//! 🏗️ En1993Builder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1993::{En1993Diff, En1993Mutation, En1993Snapshot};
use crate::artifacts::en1993::standards::v1::subsets::any::builder::En1993Builder as En1993AnyBuilder;

#[derive(Clone, Debug)]
pub struct En1993Builder(En1993AnyBuilder);

impl ArtifactBuilder for En1993Builder {
    type Snapshot = En1993Snapshot;
    type Mutation = En1993Mutation;
    type Diff = En1993Diff;
    fn empty() -> Self { Self(En1993AnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(En1993AnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(En1993AnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(En1993AnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
