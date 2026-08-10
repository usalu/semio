//! Vdi3805Builder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::vdi3805::{Vdi3805Diff, Vdi3805Mutation, Vdi3805Snapshot};

#[derive(Clone, Debug, Default)]
pub struct Vdi3805Builder {
    snapshot: Vdi3805Snapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for Vdi3805Builder {
    type Snapshot = Vdi3805Snapshot;
    type Mutation = Vdi3805Mutation;
    type Diff = Vdi3805Diff;
    fn empty() -> Self { Self { snapshot: Vdi3805Snapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<Vdi3805Snapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<Vdi3805Snapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = <Vdi3805Diff as protocol::MutationDiff<Vdi3805Snapshot>>::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <Vdi3805Diff as protocol::MutationDiff<Vdi3805Snapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
