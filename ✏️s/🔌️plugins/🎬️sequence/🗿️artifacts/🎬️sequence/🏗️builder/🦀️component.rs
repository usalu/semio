//! 🏗️ SequenceBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::sequence::{SequenceDiff, SequenceMutation, SequenceSnapshot};
use crate::artifacts::sequence::standards::v1::builder::SequenceBuilder as SequenceRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct SequenceBuilder(SequenceRawBuilder);

impl ArtifactBuilder for SequenceBuilder {
    type Snapshot = SequenceSnapshot;
    type Mutation = SequenceMutation;
    type Diff = SequenceDiff;
    fn empty() -> Self { Self(SequenceRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(SequenceRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(SequenceRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(SequenceRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
