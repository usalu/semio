//! En1997Builder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::en1997::{En1997Diff, En1997Mutation, En1997Snapshot};

#[derive(Clone, Debug, Default)]
pub struct En1997Builder {
    snapshot: En1997Snapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for En1997Builder {
    type Snapshot = En1997Snapshot;
    type Mutation = En1997Mutation;
    type Diff = En1997Diff;
    fn empty() -> Self { Self { snapshot: En1997Snapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<En1997Snapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<En1997Snapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <En1997Mutation as protocol::Mutation<En1997Snapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = <En1997Diff as protocol::MutationDiff<En1997Snapshot>>::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <En1997Diff as protocol::MutationDiff<En1997Snapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
