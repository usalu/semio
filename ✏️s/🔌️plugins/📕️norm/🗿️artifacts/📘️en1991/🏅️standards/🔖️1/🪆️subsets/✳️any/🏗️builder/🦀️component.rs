//! En1991Builder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};

#[derive(Clone, Debug, Default)]
pub struct En1991Builder {
    snapshot: En1991Snapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for En1991Builder {
    type Snapshot = En1991Snapshot;
    type Mutation = En1991Mutation;
    type Diff = En1991Diff;
    fn empty() -> Self { Self { snapshot: En1991Snapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<En1991Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<En1991Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <En1991Mutation as protocol::Mutation<En1991Snapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = <En1991Diff as protocol::MutationDiff<En1991Snapshot>>::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <En1991Diff as protocol::MutationDiff<En1991Snapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
