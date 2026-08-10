//! VcsBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::vcs::{VcsDiff, VcsDemoMutation, VcsSnapshot};

#[derive(Clone, Debug, Default)]
pub struct VcsBuilder {
    snapshot: VcsSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for VcsBuilder {
    type Snapshot = VcsSnapshot;
    type Mutation = VcsDemoMutation;
    type Diff = VcsDiff;
    fn empty() -> Self { Self { snapshot: VcsSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<VcsSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<VcsSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::vcs::schema::mutations::apply_vcs_demo_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <VcsDiff as protocol::MutationDiff<VcsSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
