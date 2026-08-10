//! 🏗️ WriterBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::writer::{WriterDiff, WriterMutation, WriterSnapshot};
use crate::artifacts::writer::standards::v1::builder::WriterBuilder as WriterRawBuilder;

#[derive(Clone, Debug)]
pub struct WriterBuilder(WriterRawBuilder);

impl ArtifactBuilder for WriterBuilder {
    type Snapshot = WriterSnapshot;
    type Mutation = WriterMutation;
    type Diff = WriterDiff;
    fn empty() -> Self { Self(WriterRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(WriterRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(WriterRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(WriterRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
