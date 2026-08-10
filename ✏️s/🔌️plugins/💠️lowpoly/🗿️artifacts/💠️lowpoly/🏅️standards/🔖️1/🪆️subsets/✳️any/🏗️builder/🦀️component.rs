//! LowpolyBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::lowpoly::schema::diff::LowpolyDiff;
use crate::artifacts::lowpoly::schema::mutations::LowpolyMutation;
use crate::artifacts::lowpoly::schema::snapshot::LowpolySnapshot;

#[derive(Clone, Debug, Default)]
pub struct LowpolyBuilder {
    snapshot: LowpolySnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for LowpolyBuilder {
    type Snapshot = LowpolySnapshot;
    type Mutation = LowpolyMutation;
    type Diff = LowpolyDiff;
    fn empty() -> Self { Self { snapshot: LowpolySnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<LowpolySnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<LowpolySnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let d = <LowpolyMutation as protocol::Mutation<LowpolySnapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
        (self, d)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <LowpolyDiff as protocol::MutationDiff<LowpolySnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
