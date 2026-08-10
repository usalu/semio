//! Fem3dBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::fem3d::{Fem3dDiff, Fem3dMutation, Fem3dSnapshot};

#[derive(Clone, Debug, Default)]
pub struct Fem3dBuilder {
    snapshot: Fem3dSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for Fem3dBuilder {
    type Snapshot = Fem3dSnapshot;
    type Mutation = Fem3dMutation;
    type Diff = Fem3dDiff;
    fn empty() -> Self { Self { snapshot: Fem3dSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<Fem3dSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
        crate::artifacts::fem3d::schema::mutations::apply_fem3d_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
