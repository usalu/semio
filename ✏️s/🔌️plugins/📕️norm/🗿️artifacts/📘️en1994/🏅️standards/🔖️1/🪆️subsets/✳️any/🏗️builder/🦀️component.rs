//! En1994Builder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};

#[derive(Clone, Debug, Default)]
pub struct En1994Builder {
    snapshot: En1994Snapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for En1994Builder {
    type Snapshot = En1994Snapshot;
    type Mutation = En1994Mutation;
    type Diff = En1994Diff;
    fn empty() -> Self { Self { snapshot: En1994Snapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<En1994Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<En1994Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let d = <En1994Mutation as protocol::Mutation<En1994Snapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = <En1994Diff as protocol::MutationDiff<En1994Snapshot>>::apply(&d, &self.snapshot);
        (self, d)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <En1994Diff as protocol::MutationDiff<En1994Snapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
