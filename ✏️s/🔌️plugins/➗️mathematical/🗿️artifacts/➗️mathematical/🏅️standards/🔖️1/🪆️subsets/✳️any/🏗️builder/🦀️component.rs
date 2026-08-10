//! MathematicalBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::mathematical::{MathematicalDiff, MathematicalMutation, MathematicalSnapshot};

#[derive(Clone, Debug, Default)]
pub struct MathematicalBuilder {
    snapshot: MathematicalSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for MathematicalBuilder {
    type Snapshot = MathematicalSnapshot;
    type Mutation = MathematicalMutation;
    type Diff = MathematicalDiff;
    fn empty() -> Self { Self { snapshot: MathematicalSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<MathematicalSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<MathematicalSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::mathematical::schema::mutations::apply_mathematical_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <MathematicalDiff as protocol::MutationDiff<MathematicalSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
