//! PresentBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::present::schema::diff::PresentDiff;
use crate::artifacts::present::schema::mutations::PresentMutation;
use crate::artifacts::present::schema::snapshot::PresentSnapshot;

#[derive(Clone, Debug, Default)]
pub struct PresentBuilder {
    snapshot: PresentSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for PresentBuilder {
    type Snapshot = PresentSnapshot;
    type Mutation = PresentMutation;
    type Diff = PresentDiff;
    fn empty() -> Self { Self { snapshot: PresentSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<PresentSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<PresentSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <PresentMutation as protocol::Mutation<PresentSnapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <PresentDiff as protocol::MutationDiff<PresentSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
