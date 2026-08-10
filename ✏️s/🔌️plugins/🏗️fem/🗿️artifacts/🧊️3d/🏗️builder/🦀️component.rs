//! 🏗️ Fem3dBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::fem3d::{Fem3dDiff, Fem3dMutation, Fem3dSnapshot};
use crate::artifacts::fem3d::standards::v1::builder::Fem3dBuilder as Fem3dRawBuilder;

#[derive(Clone, Debug)]
pub struct Fem3dBuilder(Fem3dRawBuilder);

impl ArtifactBuilder for Fem3dBuilder {
    type Snapshot = Fem3dSnapshot;
    type Mutation = Fem3dMutation;
    type Diff = Fem3dDiff;
    fn empty() -> Self { Self(Fem3dRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Fem3dRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Fem3dRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Fem3dRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
