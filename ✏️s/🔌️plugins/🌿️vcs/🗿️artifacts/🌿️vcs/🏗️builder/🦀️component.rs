//! 🏗️ VcsBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::vcs::{VcsDiff, VcsMutation, VcsSnapshot};
use crate::artifacts::vcs::standards::v1::builder::VcsBuilder as VcsRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct VcsBuilder(VcsRawBuilder);

impl ArtifactBuilder for VcsBuilder {
    type Snapshot = VcsSnapshot;
    type Mutation = VcsMutation;
    type Diff = VcsDiff;
    fn empty() -> Self { Self(VcsRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(VcsRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(VcsRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(VcsRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
