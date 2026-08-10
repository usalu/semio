//! Din18599Builder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::din18599::{Din18599Diff, Din18599Mutation, Din18599Snapshot};

#[derive(Clone, Debug, Default)]
pub struct Din18599Builder {
    snapshot: Din18599Snapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for Din18599Builder {
    type Snapshot = Din18599Snapshot;
    type Mutation = Din18599Mutation;
    type Diff = Din18599Diff;
    fn empty() -> Self { Self { snapshot: Din18599Snapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<Din18599Snapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<Din18599Snapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <Din18599Diff as protocol::MutationDiff<Din18599Snapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
