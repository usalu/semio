//! Procedural3dBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::procedural3d::{Procedural3dDiff, Procedural3dMutation, Procedural3dSnapshot};

#[derive(Clone, Debug, Default)]
pub struct Procedural3dBuilder {
    snapshot: Procedural3dSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for Procedural3dBuilder {
    type Snapshot = Procedural3dSnapshot;
    type Mutation = Procedural3dMutation;
    type Diff = Procedural3dDiff;
    fn empty() -> Self { Self { snapshot: Procedural3dSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<Procedural3dSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<Procedural3dSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::procedural3d::schema::mutations::apply_procedural3d_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <Procedural3dDiff as protocol::MutationDiff<Procedural3dSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
