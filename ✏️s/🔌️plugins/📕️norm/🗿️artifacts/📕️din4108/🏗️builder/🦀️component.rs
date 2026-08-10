//! 🏗️ Din4108Builder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::din4108::{Din4108Diff, Din4108Mutation, Din4108Snapshot};
use crate::artifacts::din4108::standards::v1::builder::Din4108Builder as Din4108RawBuilder;

#[derive(Clone, Debug)]
pub struct Din4108Builder(Din4108RawBuilder);

impl ArtifactBuilder for Din4108Builder {
    type Snapshot = Din4108Snapshot;
    type Mutation = Din4108Mutation;
    type Diff = Din4108Diff;
    fn empty() -> Self { Self(Din4108RawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Din4108RawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Din4108RawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Din4108RawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
