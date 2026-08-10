//! 🏗️ Procedural2dBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::procedural2d::{Procedural2dDiff, Procedural2dMutation, Procedural2dSnapshot};
use crate::artifacts::procedural2d::standards::v1::builder::Procedural2dBuilder as Procedural2dRawBuilder;

#[derive(Clone, Debug)]
pub struct Procedural2dBuilder(Procedural2dRawBuilder);

impl ArtifactBuilder for Procedural2dBuilder {
    type Snapshot = Procedural2dSnapshot;
    type Mutation = Procedural2dMutation;
    type Diff = Procedural2dDiff;
    fn empty() -> Self { Self(Procedural2dRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(Procedural2dRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(Procedural2dRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(Procedural2dRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
