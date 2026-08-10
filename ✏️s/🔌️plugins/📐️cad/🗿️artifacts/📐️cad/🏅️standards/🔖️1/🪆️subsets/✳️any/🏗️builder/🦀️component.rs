//! CadBuilder — ArtifactBuilder for cad.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::cad::diff::schema::CadDiff;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::{CadSnapshot, CAD_PLAY_DOCUMENT_SCHEMA};
use std::collections::BTreeMap;

//#region Builder
fn empty_snapshot() -> CadSnapshot {
    CadSnapshot {
        schema: CAD_PLAY_DOCUMENT_SCHEMA.into(),
        id: String::new(),
        objects: Vec::new(),
        building_objects: Vec::new(),
        energy_objects: Vec::new(),
        structure_classic_objects: Vec::new(),
        references_by_model_definition_id: BTreeMap::new(),
        nodes: Vec::new(),
        shape_geometry: None,
        building_geometry: None,
        energy_geometry: None,
        structure_classic_geometry: None,
        active_model_definition_id: String::new(),
    }
}

/// Builds a `cad` snapshot.
#[derive(Clone, Debug)]
pub struct CadBuilder {
    snapshot: CadSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for CadBuilder {
    type Snapshot = CadSnapshot;
    type Mutation = CadMutation;
    type Diff = CadDiff;

    fn empty() -> Self {
        Self { snapshot: empty_snapshot(), diagnostics: Vec::new() }
    }

    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }

    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<CadSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }

    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<CadSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }

    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        let diff = <CadMutation as protocol::Mutation<CadSnapshot>>::diff(&mutation, &self.snapshot);
        self.snapshot = <CadDiff as protocol::MutationDiff<CadSnapshot>>::apply(&diff, &self.snapshot);
        self
    }

    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <CadDiff as protocol::MutationDiff<CadSnapshot>>::apply(&diff, &self.snapshot);
        self
    }

    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion Builder
