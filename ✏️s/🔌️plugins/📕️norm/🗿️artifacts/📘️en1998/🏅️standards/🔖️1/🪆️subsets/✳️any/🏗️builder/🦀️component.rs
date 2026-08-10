//! En1998Builder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1998::{En1998Diff, En1998Mutation, En1998Snapshot};

#[derive(Clone, Debug, Default)]
pub struct En1998Builder {
    snapshot: En1998Snapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for En1998Builder {
    type Snapshot = En1998Snapshot;
    type Mutation = En1998Mutation;
    type Diff = En1998Diff;
    fn empty() -> Self { Self { snapshot: En1998Snapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<En1998Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<En1998Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <En1998Mutation as protocol::Mutation<En1998Snapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <En1998Diff as protocol::MutationDiff<En1998Snapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
