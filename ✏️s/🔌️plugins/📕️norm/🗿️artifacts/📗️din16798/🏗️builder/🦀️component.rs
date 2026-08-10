//! 🏗️ Din16798Builder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::din16798::{Din16798Diff, Din16798Mutation, Din16798Snapshot};
use crate::artifacts::din16798::standards::v1::builder::Din16798Builder as Din16798RawBuilder;

#[derive(Clone, Debug)]
pub struct Din16798Builder(Din16798RawBuilder);

impl ArtifactBuilder for Din16798Builder {
    type Snapshot = Din16798Snapshot;
    type Mutation = Din16798Mutation;
    type Diff = Din16798Diff;
    fn empty() -> Self { Self(Din16798RawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Din16798RawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Din16798RawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Din16798RawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
