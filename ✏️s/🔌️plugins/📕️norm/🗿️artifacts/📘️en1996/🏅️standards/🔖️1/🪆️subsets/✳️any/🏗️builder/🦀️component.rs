//! En1996Builder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1996::{En1996Diff, En1996Mutation, En1996Snapshot};

#[derive(Clone, Debug, Default)]
pub struct En1996Builder {
    snapshot: En1996Snapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for En1996Builder {
    type Snapshot = En1996Snapshot;
    type Mutation = En1996Mutation;
    type Diff = En1996Diff;
    fn empty() -> Self { Self { snapshot: En1996Snapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<En1996Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<En1996Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = <En1996Diff as protocol::MutationDiff<En1996Snapshot>>::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <En1996Diff as protocol::MutationDiff<En1996Snapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
