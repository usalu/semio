//! 🏗️ FormsBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::forms::{FormsDiff, FormsMutation, FormsSnapshot};
use crate::artifacts::forms::standards::v1::builder::FormsBuilder as FormsRawBuilder;

#[derive(Clone, Debug, Default)]
pub struct FormsBuilder(FormsRawBuilder);

impl ArtifactBuilder for FormsBuilder {
    type Snapshot = FormsSnapshot;
    type Mutation = FormsMutation;
    type Diff = FormsDiff;
    fn empty() -> Self { Self(FormsRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(FormsRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(FormsRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(FormsRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
