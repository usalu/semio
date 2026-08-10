//! Process3dBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::process3d::schema::diff::Process3dDiff;
use crate::artifacts::process3d::schema::mutations::Process3dMutation;
use crate::artifacts::process3d::schema::snapshot::Process3dSnapshot;

#[derive(Clone, Debug, Default)]
pub struct Process3dBuilder {
    snapshot: Process3dSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for Process3dBuilder {
    type Snapshot = Process3dSnapshot;
    type Mutation = Process3dMutation;
    type Diff = Process3dDiff;
    fn empty() -> Self { Self { snapshot: Process3dSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<Process3dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<Process3dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let d = <Process3dMutation as protocol::Mutation<Process3dSnapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
        (self, d)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <Process3dDiff as protocol::MutationDiff<Process3dSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
