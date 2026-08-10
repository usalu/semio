//! 🏗️ Process3dBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::process3d::{Process3dDiff, Process3dMutation, Process3dSnapshot};
use crate::artifacts::process3d::standards::v1::builder::Process3dBuilder as Process3dRawBuilder;

#[derive(Clone, Debug)]
pub struct Process3dBuilder(Process3dRawBuilder);

impl ArtifactBuilder for Process3dBuilder {
    type Snapshot = Process3dSnapshot;
    type Mutation = Process3dMutation;
    type Diff = Process3dDiff;
    fn empty() -> Self { Self(Process3dRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Process3dRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Process3dRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Process3dRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
