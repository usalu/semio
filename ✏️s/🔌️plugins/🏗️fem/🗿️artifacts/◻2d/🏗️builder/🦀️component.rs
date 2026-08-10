//! Fem2dBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::fem2d::{Fem2dDiff, Fem2dMutation, Fem2dSnapshot};

#[derive(Clone, Debug, Default)]
pub struct Fem2dBuilder {
    snapshot: Fem2dSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for Fem2dBuilder {
    type Snapshot = Fem2dSnapshot;
    type Mutation = Fem2dMutation;
    type Diff = Fem2dDiff;
    fn empty() -> Self { Self { snapshot: Fem2dSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<Fem2dSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<Fem2dSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::fem2d::schema::mutations::apply_fem2d_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <Fem2dDiff as protocol::MutationDiff<Fem2dSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
