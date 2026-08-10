//! 🏗️ PresentBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::present::{PresentDiff, PresentMutation, PresentSnapshot};
use crate::artifacts::present::standards::v1::builder::PresentBuilder as PresentRawBuilder;

#[derive(Clone, Debug)]
pub struct PresentBuilder(PresentRawBuilder);

impl ArtifactBuilder for PresentBuilder {
    type Snapshot = PresentSnapshot;
    type Mutation = PresentMutation;
    type Diff = PresentDiff;
    fn empty() -> Self { Self(PresentRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(PresentRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(PresentRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(PresentRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
