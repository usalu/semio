//! 🏗️ PlaybookBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookMutation, PlaybookSnapshot};
use crate::artifacts::playbook::standards::v1::subsets::any::builder::PlaybookBuilder as PlaybookAnyBuilder;

#[derive(Clone, Debug)]
pub struct PlaybookBuilder(PlaybookAnyBuilder);

impl ArtifactBuilder for PlaybookBuilder {
    type Snapshot = PlaybookSnapshot;
    type Mutation = PlaybookMutation;
    type Diff = PlaybookDiff;
    fn empty() -> Self { Self(PlaybookAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(PlaybookAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(PlaybookAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(PlaybookAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
