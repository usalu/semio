//! 🏗️ En1990Builder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1990::{En1990Diff, En1990Mutation, En1990Snapshot};
use crate::artifacts::en1990::standards::v1::subsets::any::builder::En1990Builder as En1990AnyBuilder;

#[derive(Clone, Debug)]
pub struct En1990Builder(En1990AnyBuilder);

impl ArtifactBuilder for En1990Builder {
    type Snapshot = En1990Snapshot;
    type Mutation = En1990Mutation;
    type Diff = En1990Diff;
    fn empty() -> Self { Self(En1990AnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(En1990AnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(En1990AnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(En1990AnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
