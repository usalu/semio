//! SequenceBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::sequence::schema::diff::SequenceDiff;
use crate::artifacts::sequence::schema::mutations::SequenceMutation;
use crate::artifacts::sequence::schema::snapshot::SequenceSnapshot;

#[derive(Clone, Debug, Default)]
pub struct SequenceBuilder {
    snapshot: SequenceSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for SequenceBuilder {
    type Snapshot = SequenceSnapshot;
    type Mutation = SequenceMutation;
    type Diff = SequenceDiff;
    fn empty() -> Self { Self { snapshot: SequenceSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<SequenceSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<SequenceSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let d = <SequenceMutation as protocol::Mutation<SequenceSnapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
        (self, d)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <SequenceDiff as protocol::MutationDiff<SequenceSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
