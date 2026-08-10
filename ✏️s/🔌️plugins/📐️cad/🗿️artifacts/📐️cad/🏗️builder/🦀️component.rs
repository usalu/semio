//! 🏗️ CadBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::cad::{CadDiff, CadMutation, CadSnapshot};
use crate::artifacts::cad::standards::v1::builder::CadBuilder as CadRawBuilder;

#[derive(Clone, Debug)]
pub struct CadBuilder(CadRawBuilder);

impl ArtifactBuilder for CadBuilder {
    type Snapshot = CadSnapshot;
    type Mutation = CadMutation;
    type Diff = CadDiff;
    fn empty() -> Self { Self(CadRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(CadRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(CadRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(CadRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
