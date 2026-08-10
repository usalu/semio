//! 🏗️ LayoutBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::layout::{LayoutDiff, LayoutMutation, LayoutSnapshot};
use crate::artifacts::layout::standards::v1::builder::LayoutBuilder as LayoutRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct LayoutBuilder(LayoutRawBuilder);

impl ArtifactBuilder for LayoutBuilder {
    type Snapshot = LayoutSnapshot;
    type Mutation = LayoutMutation;
    type Diff = LayoutDiff;
    fn empty() -> Self { Self(LayoutRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(LayoutRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(LayoutRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(LayoutRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
