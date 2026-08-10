//! En1990Builder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1990::{En1990Diff, En1990Mutation, En1990Snapshot};

#[derive(Clone, Debug, Default)]
pub struct En1990Builder {
    snapshot: En1990Snapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for En1990Builder {
    type Snapshot = En1990Snapshot;
    type Mutation = En1990Mutation;
    type Diff = En1990Diff;
    fn empty() -> Self { Self { snapshot: En1990Snapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<En1990Snapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<En1990Snapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <En1990Mutation as protocol::Mutation<En1990Snapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <En1990Diff as protocol::MutationDiff<En1990Snapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
