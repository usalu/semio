//! 🏗️ FormsBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::forms::{FormsDiff, FormMutation, FormsSnapshot};
use crate::artifacts::forms::standards::v1::subsets::any::builder::FormsBuilder as FormsAnyBuilder;

#[derive(Clone, Debug)]
pub struct FormsBuilder(FormsAnyBuilder);

impl ArtifactBuilder for FormsBuilder {
    type Snapshot = FormsSnapshot;
    type Mutation = FormMutation;
    type Diff = FormsDiff;
    fn empty() -> Self { Self(FormsAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(FormsAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(FormsAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(FormsAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) { let (inner, diff) = self.0.mutate(mutation); (Self(inner), diff) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
