//! En1992Builder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1992::{En1992Diff, En1992Mutation, En1992Snapshot};

#[derive(Clone, Debug, Default)]
pub struct En1992Builder {
    snapshot: En1992Snapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for En1992Builder {
    type Snapshot = En1992Snapshot;
    type Mutation = En1992Mutation;
    type Diff = En1992Diff;
    fn empty() -> Self { Self { snapshot: En1992Snapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<En1992Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<En1992Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <En1992Mutation as protocol::Mutation<En1992Snapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <En1992Diff as protocol::MutationDiff<En1992Snapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
