//! WriterBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::writer::{WriterDiff, WriterMutation, WriterSnapshot};

#[derive(Clone, Debug, Default)]
pub struct WriterBuilder {
    snapshot: WriterSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for WriterBuilder {
    type Snapshot = WriterSnapshot;
    type Mutation = WriterMutation;
    type Diff = WriterDiff;
    fn empty() -> Self { Self { snapshot: WriterSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<WriterSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<WriterSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        apply_writer_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <WriterDiff as protocol::MutationDiff<WriterSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
