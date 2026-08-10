//! 🏗️ Din18599Builder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::din18599::{Din18599Diff, Din18599Mutation, Din18599Snapshot};
use crate::artifacts::din18599::standards::v1::subsets::any::builder::Din18599Builder as Din18599AnyBuilder;

#[derive(Clone, Debug)]
pub struct Din18599Builder(Din18599AnyBuilder);

impl ArtifactBuilder for Din18599Builder {
    type Snapshot = Din18599Snapshot;
    type Mutation = Din18599Mutation;
    type Diff = Din18599Diff;
    fn empty() -> Self { Self(Din18599AnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Din18599AnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Din18599AnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Din18599AnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
