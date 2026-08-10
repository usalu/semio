//! 🏗️ MathematicalBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalMutation, MathematicalSnapshot};
use crate::artifacts::mathematical::standards::v1::builder::MathematicalBuilder as MathematicalRawBuilder;

#[derive(Clone, Debug)]
pub struct MathematicalBuilder(MathematicalRawBuilder);

impl ArtifactBuilder for MathematicalBuilder {
    type Snapshot = MathematicalSnapshot;
    type Mutation = MathematicalMutation;
    type Diff = MathematicalDiff;
    fn empty() -> Self { Self(MathematicalRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(MathematicalRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(MathematicalRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(MathematicalRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
