//! 🏗️ FlowBuilder (final, artifact-level) — delegates to the 1 standard.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::flow::{FlowDiff, FlowMutation, FlowSnapshot};
use crate::artifacts::flow::standards::v1::builder::FlowBuilder as FlowRawBuilder;

#[derive(Clone, Debug)]
pub struct FlowBuilder(FlowRawBuilder);

impl ArtifactBuilder for FlowBuilder {
    type Snapshot = FlowSnapshot;
    type Mutation = FlowMutation;
    type Diff = FlowDiff;
    fn empty() -> Self { Self(FlowRawBuilder::empty()) }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self(FlowRawBuilder::from_snapshot(snapshot)) }
    fn from_text(text: &str) -> Result<Self, store::TextError> { Ok(Self(FlowRawBuilder::from_text(text)?)) }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> { Ok(Self(FlowRawBuilder::from_binary(bytes)?)) }
    fn mutate(self, mutation: Self::Mutation) -> Self { Self(self.0.mutate(mutation)) }
    fn absorb(self, diff: Self::Diff) -> Self { Self(self.0.absorb(diff)) }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> { self.0.build() }
}
