//! 🏗️ FlowBuilder (1 standard) — delegates to its ✳️any subset.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::flow::{FlowDiff, FlowMutation, FlowSnapshot};
use crate::artifacts::flow::standards::v1::subsets::any::builder::FlowBuilder as FlowAnyBuilder;

#[derive(Clone, Debug, Default)]
pub struct FlowBuilder(FlowAnyBuilder);

impl ArtifactBuilder for FlowBuilder {
    type Snapshot = FlowSnapshot;
    type Mutation = FlowMutation;
    type Diff = FlowDiff;
    fn empty() -> Self { Self(FlowAnyBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(FlowAnyBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(FlowAnyBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(FlowAnyBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
