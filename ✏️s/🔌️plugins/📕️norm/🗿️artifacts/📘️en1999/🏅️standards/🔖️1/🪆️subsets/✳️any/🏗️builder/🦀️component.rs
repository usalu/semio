//! En1999Builder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1999::{En1999Diff, En1999Mutation, En1999Snapshot};

#[derive(Clone, Debug, Default)]
pub struct En1999Builder {
    snapshot: En1999Snapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for En1999Builder {
    type Snapshot = En1999Snapshot;
    type Mutation = En1999Mutation;
    type Diff = En1999Diff;
    fn empty() -> Self { Self { snapshot: En1999Snapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<En1999Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<En1999Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <En1999Mutation as protocol::Mutation<En1999Snapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = <En1999Diff as protocol::MutationDiff<En1999Snapshot>>::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <En1999Diff as protocol::MutationDiff<En1999Snapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
