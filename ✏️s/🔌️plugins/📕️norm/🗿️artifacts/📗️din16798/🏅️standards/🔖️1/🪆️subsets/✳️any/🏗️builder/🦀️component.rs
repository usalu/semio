//! Din16798Builder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::din16798::{Din16798Diff, Din16798Mutation, Din16798Snapshot};

#[derive(Clone, Debug, Default)]
pub struct Din16798Builder {
    snapshot: Din16798Snapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for Din16798Builder {
    type Snapshot = Din16798Snapshot;
    type Mutation = Din16798Mutation;
    type Diff = Din16798Diff;
    fn empty() -> Self { Self { snapshot: Din16798Snapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<Din16798Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<Din16798Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let d = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = <Din16798Diff as protocol::MutationDiff<Din16798Snapshot>>::apply(&d, &self.snapshot);
        (self, d)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <Din16798Diff as protocol::MutationDiff<Din16798Snapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
