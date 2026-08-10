//! 🏗️ Procedural3dBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::procedural3d::{Procedural3dDiff, Procedural3dMutation, Procedural3dSnapshot};
use crate::artifacts::procedural3d::standards::v1::builder::Procedural3dBuilder as Procedural3dRawBuilder;

#[derive(Clone, Debug)]
pub struct Procedural3dBuilder(Procedural3dRawBuilder);

impl ArtifactBuilder for Procedural3dBuilder {
    type Snapshot = Procedural3dSnapshot;
    type Mutation = Procedural3dMutation;
    type Diff = Procedural3dDiff;
    fn empty() -> Self { Self(Procedural3dRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Procedural3dRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Procedural3dRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Procedural3dRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
