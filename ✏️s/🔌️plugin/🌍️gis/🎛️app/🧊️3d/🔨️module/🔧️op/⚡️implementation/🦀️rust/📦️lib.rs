//! ⚡️ GIS 3D app — operation enum + laws (constitutional: op).

use gis3d::Gis3dTerrainDocument;
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

//#region 🔖️Types
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gis3dTerrainDiff {
    pub exaggeration: Option<f64>,
}

impl OperationDiff<Gis3dTerrainDocument> for Gis3dTerrainDiff {
    fn apply(&self, projection: &Gis3dTerrainDocument) -> Gis3dTerrainDocument {
        Gis3dTerrainDocument { exaggeration: self.exaggeration.unwrap_or(projection.exaggeration) }
    }

    fn absorb(&mut self, other: Self) {
        if other.exaggeration.is_some() {
            self.exaggeration = other.exaggeration;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Gis3dTerrainOperation {
    SetExaggeration { exaggeration: f64 },
}

impl Operation<Gis3dTerrainDocument> for Gis3dTerrainOperation {
    type Diff = Gis3dTerrainDiff;

    fn diff(&self, _projection: &Gis3dTerrainDocument) -> Gis3dTerrainDiff {
        match self {
            Gis3dTerrainOperation::SetExaggeration { exaggeration } => Gis3dTerrainDiff { exaggeration: Some(*exaggeration) },
        }
    }

    fn backwards(&self, projection: &Gis3dTerrainDocument) -> Vec<Self> {
        match self {
            Gis3dTerrainOperation::SetExaggeration { .. } => vec![Gis3dTerrainOperation::SetExaggeration { exaggeration: projection.exaggeration }],
        }
    }
}

pub type Gis3dTerrainEnvelope = DocumentEnvelope<Gis3dTerrainDocument, Gis3dTerrainOperation>;
pub type Gis3dTerrainStore = DocumentStore<Gis3dTerrainDocument, Gis3dTerrainOperation>;
//#endregion 🔖️Types

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gis3d_terrain_set_exaggeration_op_line_round_trips() {
        store::test_support::assert_op_line_round_trip(&Gis3dTerrainOperation::SetExaggeration { exaggeration: 3.0 });
    }
}
//#endregion 🧪️Tests
