//! PlaygroundBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::playground::schema::diff::PlaygroundDiff;
use crate::artifacts::playground::schema::mutations::PlaygroundMutation;
use crate::artifacts::playground::schema::snapshot::PlaygroundSnapshot;

#[derive(Clone, Debug, Default)]
pub struct PlaygroundBuilder {
    snapshot: PlaygroundSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for PlaygroundBuilder {
    type Snapshot = PlaygroundSnapshot;
    type Mutation = PlaygroundMutation;
    type Diff = PlaygroundDiff;
    fn empty() -> Self { Self { snapshot: PlaygroundSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<PlaygroundSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<PlaygroundSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let d = <PlaygroundMutation as protocol::Mutation<PlaygroundSnapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
        (self, d)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <PlaygroundDiff as protocol::MutationDiff<PlaygroundSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
