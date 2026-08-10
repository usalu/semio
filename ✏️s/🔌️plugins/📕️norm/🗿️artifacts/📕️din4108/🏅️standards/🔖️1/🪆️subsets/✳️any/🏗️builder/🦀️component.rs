//! Din4108Builder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::din4108::{Din4108Diff, Din4108Mutation, Din4108Snapshot};

#[derive(Clone, Debug, Default)]
pub struct Din4108Builder {
    snapshot: Din4108Snapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for Din4108Builder {
    type Snapshot = Din4108Snapshot;
    type Mutation = Din4108Mutation;
    type Diff = Din4108Diff;
    fn empty() -> Self { Self { snapshot: Din4108Snapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<Din4108Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<Din4108Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = <Din4108Diff as protocol::MutationDiff<Din4108Snapshot>>::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <Din4108Diff as protocol::MutationDiff<Din4108Snapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
