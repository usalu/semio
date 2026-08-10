//! ProgramBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::program::schema::diff::ProgramDiff;
use crate::artifacts::program::schema::mutations::ProgramMutation;
use crate::artifacts::program::schema::snapshot::ProgramSnapshot;

#[derive(Clone, Debug, Default)]
pub struct ProgramBuilder {
    snapshot: ProgramSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for ProgramBuilder {
    type Snapshot = ProgramSnapshot;
    type Mutation = ProgramMutation;
    type Diff = ProgramDiff;
    fn empty() -> Self { Self { snapshot: ProgramSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<ProgramSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<ProgramSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <ProgramMutation as protocol::Mutation<ProgramSnapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <ProgramDiff as protocol::MutationDiff<ProgramSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
