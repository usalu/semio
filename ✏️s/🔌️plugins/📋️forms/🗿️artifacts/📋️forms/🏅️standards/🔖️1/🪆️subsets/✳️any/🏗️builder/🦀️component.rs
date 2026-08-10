//! FormsBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::forms::{FormsDiff, FormMutation, FormsSnapshot};

#[derive(Clone, Debug, Default)]
pub struct FormsBuilder {
    snapshot: FormsSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for FormsBuilder {
    type Snapshot = FormsSnapshot;
    type Mutation = FormMutation;
    type Diff = FormsDiff;
    fn empty() -> Self { Self { snapshot: FormsSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<FormsSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<FormsSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        self.snapshot = crate::artifacts::forms::schema::mutations::apply_form_edit_mutation(&self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <FormsDiff as protocol::MutationDiff<FormsSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
