//! 🏗️ PresentBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::present::{PresentDiff, PresentMutation, PresentSnapshot};
use crate::artifacts::present::standards::v1::subsets::any::builder::PresentBuilder as PresentAnyBuilder;

#[derive(Clone, Debug)]
pub struct PresentBuilder(PresentAnyBuilder);

impl ArtifactBuilder for PresentBuilder {
    type Snapshot = PresentSnapshot;
    type Mutation = PresentMutation;
    type Diff = PresentDiff;
    fn empty() -> Self { Self(PresentAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(PresentAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(PresentAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(PresentAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
