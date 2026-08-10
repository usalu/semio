//! 🏗️ CadBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::cad::{CadDiff, CadMutation, CadSnapshot};
use crate::artifacts::cad::standards::v1::subsets::any::builder::CadBuilder as CadAnyBuilder;

#[derive(Clone, Debug)]
pub struct CadBuilder(CadAnyBuilder);

impl ArtifactBuilder for CadBuilder {
    type Snapshot = CadSnapshot;
    type Mutation = CadMutation;
    type Diff = CadDiff;
    fn empty() -> Self { Self(CadAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(CadAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(CadAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(CadAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
