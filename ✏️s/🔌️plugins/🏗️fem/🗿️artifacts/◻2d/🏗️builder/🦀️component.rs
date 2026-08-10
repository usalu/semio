//! 🏗️ Fem2dBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::fem2d::{Fem2dDiff, Fem2dMutation, Fem2dSnapshot};
use crate::artifacts::fem2d::standards::v1::builder::Fem2dBuilder as Fem2dRawBuilder;

#[derive(Clone, Debug)]
pub struct Fem2dBuilder(Fem2dRawBuilder);

impl ArtifactBuilder for Fem2dBuilder {
    type Snapshot = Fem2dSnapshot;
    type Mutation = Fem2dMutation;
    type Diff = Fem2dDiff;
    fn empty() -> Self { Self(Fem2dRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Fem2dRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Fem2dRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Fem2dRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
