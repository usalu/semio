//! ⚡️ GIS 3D app — operation enum + laws (constitutional: op).

use gis3d::Gis3dTerrainDocument;
use gis3d_engine::Gis3dConfig;
use protocol::{Operation, OperationDiff};
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

//#region 🔖️Types
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gis3dTerrainDiff {
    pub document: Option<Gis3dTerrainDocument>,
    pub exaggeration: Option<f64>,
    pub imported_features_json: Option<String>,
}

impl OperationDiff<Gis3dTerrainDocument> for Gis3dTerrainDiff {
    fn apply(&self, projection: &Gis3dTerrainDocument) -> Gis3dTerrainDocument {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        if let Some(exaggeration) = self.exaggeration {
            next.exaggeration = exaggeration;
        }
        if let Some(imported_features_json) = &self.imported_features_json {
            next.imported_features_json = imported_features_json.clone();
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            *self = Gis3dTerrainDiff { document: other.document, ..Default::default() };
            return;
        }
        if other.exaggeration.is_some() {
            self.exaggeration = other.exaggeration;
        }
        if other.imported_features_json.is_some() {
            self.imported_features_json = other.imported_features_json;
        }
    }
}

/// 🗺️ Typed, invertible terrain operation. `SetExaggeration` is the one interactively-edited field
/// (the transform-gumball-equivalent slider control); `SetImportedFeatures` writes `map:in`'s overlay
/// pin layer (see `Gis3dTerrainDocument::imported_features_json`); `SetDocument` replaces the whole
/// document (`whole_document_operation`/`document:in` import, example load, reset) — mirrors
/// `gis2d_op::GisMapOperation::SetDocument`'s identical "whole-projection replace" shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum Gis3dTerrainOperation {
    SetExaggeration {
        exaggeration: f64,
    },
    SetImportedFeatures {
        #[dsl(key = "features-json")]
        features_json: String,
    },
    SetDocument {
        #[dsl(block)]
        document: Gis3dTerrainDocument,
    },
}

impl Operation<Gis3dTerrainDocument> for Gis3dTerrainOperation {
    type Diff = Gis3dTerrainDiff;

    fn diff(&self, _projection: &Gis3dTerrainDocument) -> Gis3dTerrainDiff {
        match self {
            Gis3dTerrainOperation::SetExaggeration { exaggeration } => Gis3dTerrainDiff { exaggeration: Some(*exaggeration), ..Default::default() },
            Gis3dTerrainOperation::SetImportedFeatures { features_json } => Gis3dTerrainDiff { imported_features_json: Some(features_json.clone()), ..Default::default() },
            Gis3dTerrainOperation::SetDocument { document } => Gis3dTerrainDiff { document: Some(document.clone()), ..Default::default() },
        }
    }

    fn backwards(&self, projection: &Gis3dTerrainDocument) -> Vec<Self> {
        match self {
            Gis3dTerrainOperation::SetExaggeration { .. } => vec![Gis3dTerrainOperation::SetExaggeration { exaggeration: projection.exaggeration }],
            Gis3dTerrainOperation::SetImportedFeatures { .. } => vec![Gis3dTerrainOperation::SetImportedFeatures { features_json: projection.imported_features_json.clone() }],
            Gis3dTerrainOperation::SetDocument { .. } => vec![Gis3dTerrainOperation::SetDocument { document: projection.clone() }],
        }
    }
}

pub type Gis3dTerrainEnvelope = DocumentEnvelope<Gis3dTerrainDocument, Gis3dTerrainOperation>;
pub type Gis3dTerrainStore = DocumentStore<Gis3dTerrainDocument, Gis3dTerrainOperation>;
//#endregion 🔖️Types

//#region 🔖️ConfigOperations
/// @emoji 🧮️ B1: `gis3d_engine::Gis3dConfig`'s operation enum — one variant per settled interaction,
/// plus a generic `Snapshot` every variant's `backwards()` returns — mirrors
/// `gis2d_op::Gis2dConfigOperation`/`shooting_op::ShootingConfigOperation`'s identical shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslOps)]
pub enum Gis3dConfigOperation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Gis3dConfig,
    },
    #[dsl(key = "camera")]
    SetCamera { camera_json: String },
    #[dsl(key = "selection")]
    SetSelection { ids: Vec<String> },
    #[dsl(key = "locale")]
    SetLocale { value: String },
}

impl Operation<Gis3dConfig> for Gis3dConfigOperation {
    type Diff = Gis3dConfig;

    fn diff(&self, base: &Gis3dConfig) -> Gis3dConfig {
        let mut next = base.clone();
        match self {
            Gis3dConfigOperation::Snapshot { config } => return config.clone(),
            Gis3dConfigOperation::SetCamera { camera_json } => next.camera_json = camera_json.clone(),
            Gis3dConfigOperation::SetSelection { ids } => next.selected_ids = ids.clone(),
            Gis3dConfigOperation::SetLocale { value } => next.locale = value.clone(),
        }
        next
    }

    fn backwards(&self, base: &Gis3dConfig) -> Vec<Self> {
        vec![Gis3dConfigOperation::Snapshot { config: base.clone() }]
    }
}
//#endregion 🔖️ConfigOperations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gis3d_terrain_set_exaggeration_op_line_round_trips() {
        store::test_support::assert_op_line_round_trip(&Gis3dTerrainOperation::SetExaggeration { exaggeration: 3.0 });
    }

    #[test]
    fn gis3d_terrain_set_imported_features_op_line_round_trips() {
        store::test_support::assert_op_line_round_trip(&Gis3dTerrainOperation::SetImportedFeatures { features_json: r#"{"positions":[]}"#.into() });
    }

    #[test]
    fn gis3d_terrain_set_document_op_line_round_trips() {
        store::test_support::assert_op_line_round_trip(&Gis3dTerrainOperation::SetDocument { document: Gis3dTerrainDocument { exaggeration: 2.0, imported_features_json: "null".into() } });
    }

    #[test]
    fn gis3d_terrain_set_document_backwards_restores_the_exact_prior_projection() {
        let projection = Gis3dTerrainDocument { exaggeration: 1.5, imported_features_json: "null".into() };
        let operation = Gis3dTerrainOperation::SetDocument { document: Gis3dTerrainDocument { exaggeration: 4.0, imported_features_json: r#"{"positions":[]}"#.into() } };
        let next = operation.diff(&projection).apply(&projection);
        assert_eq!(next.exaggeration, 4.0);
        let backwards = operation.backwards(&projection);
        let restored = backwards[0].diff(&next).apply(&next);
        assert_eq!(restored, projection);
    }

    #[test]
    fn gis3d_config_operation_backwards_restores_the_pre_operation_snapshot() {
        let base = Gis3dConfig::default();
        let operation = Gis3dConfigOperation::SetSelection { ids: vec!["p1".into()] };
        let next = operation.diff(&base);
        assert_eq!(next.selected_ids, vec!["p1".to_string()]);
        let backwards = operation.backwards(&base);
        assert_eq!(backwards, vec![Gis3dConfigOperation::Snapshot { config: base.clone() }]);
        assert_eq!(backwards[0].diff(&next), base);
    }

    #[test]
    fn gis3d_config_operation_lines_round_trip() {
        store::test_support::assert_op_line_round_trip(&Gis3dConfigOperation::SetCamera { camera_json: r#"{"position":[1.0,2.0,3.0]}"#.into() });
        store::test_support::assert_op_line_round_trip(&Gis3dConfigOperation::SetSelection { ids: vec!["p1".into()] });
        store::test_support::assert_op_line_round_trip(&Gis3dConfigOperation::SetLocale { value: "de-DE".into() });
        store::test_support::assert_op_line_round_trip(&Gis3dConfigOperation::Snapshot { config: Gis3dConfig::default() });
    }
}
//#endregion 🧪️Tests
