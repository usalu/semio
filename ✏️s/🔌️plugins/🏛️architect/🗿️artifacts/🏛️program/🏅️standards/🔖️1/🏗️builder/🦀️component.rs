//! 🏗️ ProgramBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::program::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use crate::artifacts::program::standards::v1::subsets::any::builder::ProgramBuilder as ProgramAnyBuilder;

#[derive(Clone, Debug)]
pub struct ProgramBuilder(ProgramAnyBuilder);

impl ArtifactBuilder for ProgramBuilder {
    type Snapshot = ProgramSnapshot;
    type Mutation = ProgramMutation;
    type Diff = ProgramDiff;
    fn empty() -> Self { Self(ProgramAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(ProgramAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(ProgramAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(ProgramAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
