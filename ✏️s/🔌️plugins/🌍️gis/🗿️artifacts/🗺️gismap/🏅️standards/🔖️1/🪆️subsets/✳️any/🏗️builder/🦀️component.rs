//! GismapBuilder
use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::gismap::{GisMapDiff, GisMapMutation, GisMapSnapshot};

#[derive(Clone, Debug, Default)]
pub struct GismapBuilder {
    snapshot: GisMapSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for GismapBuilder {
    type Snapshot = GisMapSnapshot;
    type Mutation = GisMapMutation;
    type Diff = GisMapDiff;
    fn empty() -> Self { Self { snapshot: GisMapSnapshot::default(), diagnostics: Vec::new() } }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self { Self { snapshot, diagnostics: Vec::new() } }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<GisMapSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<GisMapSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> (Self, Self::Diff) {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(&mutation, &self.snapshot);
        crate::artifacts::gismap::schema::mutations::apply_gis_map_mutation(&mut self.snapshot, &mutation);
        (self, diff)
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <GisMapDiff as protocol::MutationDiff<GisMapSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
