//! 🏗️ VcsBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::vcs::{VcsDiff, VcsMutation, VcsSnapshot};
use crate::artifacts::vcs::standards::v1::subsets::any::builder::VcsBuilder as VcsAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct VcsBuilder(VcsAnyBuilder);

impl ArtifactBuilder for VcsBuilder {
    type Snapshot = VcsSnapshot;
    type Mutation = VcsMutation;
    type Diff = VcsDiff;
    fn empty() -> Self { Self(VcsAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(VcsAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(VcsAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(VcsAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
