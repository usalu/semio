//! En1993Builder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1993::{En1993Diff, En1993Mutation, En1993Snapshot};

#[derive(Clone, Debug, Default)]
pub struct En1993Builder {
    snapshot: En1993Snapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for En1993Builder {
    type Snapshot = En1993Snapshot;
    type Mutation = En1993Mutation;
    type Diff = En1993Diff;
    fn empty() -> Self { Self { snapshot: En1993Snapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<En1993Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<En1993Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <En1993Mutation as protocol::Mutation<En1993Snapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = <En1993Diff as protocol::MutationDiff<En1993Snapshot>>::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <En1993Diff as protocol::MutationDiff<En1993Snapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
