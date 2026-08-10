//! LayoutBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::layout::schema::mutations::apply_layout_mutation;
use crate::artifacts::layout::{LayoutDiff, LayoutMutation, LayoutSnapshot};

#[derive(Clone, Debug)]
pub struct LayoutBuilder {
    snapshot: LayoutSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for LayoutBuilder {
    type Snapshot = LayoutSnapshot;
    type Mutation = LayoutMutation;
    type Diff = LayoutDiff;
    fn empty() -> Self {
        Self {
            snapshot: crate::artifacts::layout::engine::default_document(),
            diagnostics: Vec::new(),
        }
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<LayoutSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<LayoutSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
        apply_layout_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <LayoutDiff as protocol::MutationDiff<LayoutSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
