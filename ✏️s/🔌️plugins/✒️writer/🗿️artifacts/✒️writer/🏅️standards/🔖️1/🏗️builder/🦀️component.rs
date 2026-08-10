//! 🏗️ WriterBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::writer::{WriterDiff, WriterMutation, WriterSnapshot};
use crate::artifacts::writer::standards::v1::subsets::any::builder::WriterBuilder as WriterAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct WriterBuilder(WriterAnyBuilder);

impl ArtifactBuilder for WriterBuilder {
    type Snapshot = WriterSnapshot;
    type Mutation = WriterMutation;
    type Diff = WriterDiff;
    fn empty() -> Self { Self(WriterAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(WriterAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(WriterAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(WriterAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
