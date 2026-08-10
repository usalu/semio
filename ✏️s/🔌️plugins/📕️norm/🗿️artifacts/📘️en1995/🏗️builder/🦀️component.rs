//! En1995Builder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1995::{En1995Diff, En1995Mutation, En1995Snapshot};

#[derive(Clone, Debug, Default)]
pub struct En1995Builder {
    snapshot: En1995Snapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for En1995Builder {
    type Snapshot = En1995Snapshot;
    type Mutation = En1995Mutation;
    type Diff = En1995Diff;
    fn empty() -> Self { Self { snapshot: En1995Snapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<En1995Snapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<En1995Snapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <En1995Mutation as protocol::Mutation<En1995Snapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = <En1995Diff as protocol::MutationDiff<En1995Snapshot>>::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <En1995Diff as protocol::MutationDiff<En1995Snapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
