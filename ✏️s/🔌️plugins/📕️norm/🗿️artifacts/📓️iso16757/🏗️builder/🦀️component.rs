//! Iso16757Builder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::iso16757::{Iso16757Diff, Iso16757Mutation, Iso16757Snapshot};

#[derive(Clone, Debug, Default)]
pub struct Iso16757Builder {
    snapshot: Iso16757Snapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for Iso16757Builder {
    type Snapshot = Iso16757Snapshot;
    type Mutation = Iso16757Mutation;
    type Diff = Iso16757Diff;
    fn empty() -> Self { Self { snapshot: Iso16757Snapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<Iso16757Snapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<Iso16757Snapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = <Iso16757Diff as protocol::MutationDiff<Iso16757Snapshot>>::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <Iso16757Diff as protocol::MutationDiff<Iso16757Snapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
