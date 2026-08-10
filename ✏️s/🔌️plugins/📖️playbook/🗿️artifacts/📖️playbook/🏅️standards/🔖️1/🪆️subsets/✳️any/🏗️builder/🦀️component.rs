//! PlaybookBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::playbook::{PlaybookDiff, PlaybookMutation, PlaybookSnapshot};

#[derive(Clone, Debug, Default)]
pub struct PlaybookBuilder {
    snapshot: PlaybookSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for PlaybookBuilder {
    type Snapshot = PlaybookSnapshot;
    type Mutation = PlaybookMutation;
    type Diff = PlaybookDiff;
    fn empty() -> Self { Self { snapshot: PlaybookSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<PlaybookSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<PlaybookSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        self.snapshot = crate::artifacts::playbook::schema::mutations::apply_playbook_mutation(&self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <PlaybookDiff as protocol::MutationDiff<PlaybookSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
