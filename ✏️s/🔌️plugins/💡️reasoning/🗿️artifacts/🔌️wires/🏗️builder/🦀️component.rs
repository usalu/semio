//! WiresBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::wires::schema::diff::WiresDiff;
use crate::artifacts::wires::schema::mutations::WiresMutation;
use crate::artifacts::wires::schema::snapshot::WiresSnapshot;

#[derive(Clone, Debug)]
pub struct WiresBuilder {
    snapshot: WiresSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for WiresBuilder {
    type Snapshot = WiresSnapshot;
    type Mutation = WiresMutation;
    type Diff = WiresDiff;
    fn empty() -> Self { Self { snapshot: crate::artifacts::wires::empty_wires_snapshot(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<WiresSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<WiresSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <WiresMutation as protocol::Mutation<WiresSnapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <WiresDiff as protocol::MutationDiff<WiresSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
