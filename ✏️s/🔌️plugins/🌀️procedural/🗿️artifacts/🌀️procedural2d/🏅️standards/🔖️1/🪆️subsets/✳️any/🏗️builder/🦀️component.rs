//! Procedural2dBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::procedural2d::{Procedural2dDiff, Procedural2dMutation, Procedural2dSnapshot};

#[derive(Clone, Debug, Default)]
pub struct Procedural2dBuilder {
    snapshot: Procedural2dSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for Procedural2dBuilder {
    type Snapshot = Procedural2dSnapshot;
    type Mutation = Procedural2dMutation;
    type Diff = Procedural2dDiff;
    fn empty() -> Self { Self { snapshot: Procedural2dSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<Procedural2dSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<Procedural2dSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::procedural2d::schema::mutations::apply_procedural2d_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <Procedural2dDiff as protocol::MutationDiff<Procedural2dSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
