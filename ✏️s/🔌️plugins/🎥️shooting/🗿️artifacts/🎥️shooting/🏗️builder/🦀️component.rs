//! ShootingBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::shooting::schema::diff::ShootingDiff;
use crate::artifacts::shooting::schema::mutations::ShootingMutation;
use crate::artifacts::shooting::schema::snapshot::ShootingSnapshot;

#[derive(Clone, Debug, Default)]
pub struct ShootingBuilder {
    snapshot: ShootingSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for ShootingBuilder {
    type Snapshot = ShootingSnapshot;
    type Mutation = ShootingMutation;
    type Diff = ShootingDiff;
    fn empty() -> Self { Self { snapshot: ShootingSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<ShootingSnapshot as store::DocumentDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<ShootingSnapshot as store::DocumentPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let d = <ShootingMutation as protocol::Mutation<ShootingSnapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = protocol::MutationDiff::apply(&d, &self.snapshot);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <ShootingDiff as protocol::MutationDiff<ShootingSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
