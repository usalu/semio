//! 🏗️ Process3dBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::process3d::{Process3dDiff, Process3dMutation, Process3dSnapshot};
use crate::artifacts::process3d::standards::v1::subsets::any::builder::Process3dBuilder as Process3dAnyBuilder;

#[derive(Clone, Debug)]
pub struct Process3dBuilder(Process3dAnyBuilder);

impl ArtifactBuilder for Process3dBuilder {
    type Snapshot = Process3dSnapshot;
    type Mutation = Process3dMutation;
    type Diff = Process3dDiff;
    fn empty() -> Self { Self(Process3dAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Process3dAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Process3dAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Process3dAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
