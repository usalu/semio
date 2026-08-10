//! 🏗️ LayoutBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::layout::{LayoutDiff, LayoutMutation, LayoutSnapshot};
use crate::artifacts::layout::standards::v1::subsets::any::builder::LayoutBuilder as LayoutAnyBuilder;

#[derive(Clone, Debug)]
pub struct LayoutBuilder(LayoutAnyBuilder);

impl ArtifactBuilder for LayoutBuilder {
    type Snapshot = LayoutSnapshot;
    type Mutation = LayoutMutation;
    type Diff = LayoutDiff;
    fn empty() -> Self { Self(LayoutAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(LayoutAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(LayoutAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(LayoutAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
