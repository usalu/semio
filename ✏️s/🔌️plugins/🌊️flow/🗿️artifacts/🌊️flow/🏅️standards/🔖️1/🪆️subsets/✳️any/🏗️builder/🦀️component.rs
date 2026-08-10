//! FlowBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::flow::{FlowDiff, FlowMutation, FlowSnapshot};

#[derive(Clone, Debug, Default)]
pub struct FlowBuilder {
    snapshot: FlowSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for FlowBuilder {
    type Snapshot = FlowSnapshot;
    type Mutation = FlowMutation;
    type Diff = FlowDiff;
    fn empty() -> Self { Self { snapshot: FlowSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<FlowSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<FlowSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::flow::schema::mutations::apply_flow_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <FlowDiff as protocol::MutationDiff<FlowSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
