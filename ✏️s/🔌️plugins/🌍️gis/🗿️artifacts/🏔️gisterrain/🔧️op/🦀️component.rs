//! ⚡️ GIS terrain artifact — the operation enum, its `Operation` law and the store aliases
//! (constitutional: op).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::gisterrain::diff::Gis3dTerrainDiff;
use crate::artifacts::gisterrain::Gis3dTerrainDocument;
use protocol::Operation;
use serde::{Deserialize, Serialize};
use store::{DocumentEnvelope, DocumentStore};

//#region 🔖️Operation
/// 🗺️ Typed, invertible terrain operation. `SetExaggeration` is the one interactively-edited field
/// (the transform-gumball-equivalent slider control); `SetImportedFeatures` writes `map:in`'s overlay
/// pin layer (see `Gis3dTerrainDocument::imported_features_json`); `SetDocument` replaces the whole
/// document (`whole_document_operation`/`document:in` import, example load, reset) — mirrors
/// `crate::artifacts::gismap::op::GisMapOperation::SetDocument`'s identical "whole-projection replace"
/// shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
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
//#region 🔖️HandcraftedOpCodecs
/// ⚡️ P6 handcrafted OpText/OpBinary (derive no longer emits these traits).
impl protocol::OpText for Gis3dTerrainOperation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for Gis3dTerrainOperation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs




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
//#endregion 🔖️Operation

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::OperationDiff;

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
    fn set_exaggeration_and_set_imported_features_invert_to_the_prior_field_value() {
        let projection = Gis3dTerrainDocument { exaggeration: 1.5, imported_features_json: "null".into() };
        assert_eq!(Gis3dTerrainOperation::SetExaggeration { exaggeration: 9.0 }.backwards(&projection), vec![Gis3dTerrainOperation::SetExaggeration { exaggeration: 1.5 }]);
        assert_eq!(
            Gis3dTerrainOperation::SetImportedFeatures { features_json: "{}".into() }.backwards(&projection),
            vec![Gis3dTerrainOperation::SetImportedFeatures { features_json: "null".into() }]
        );
    }
}
//#endregion 🧪️Tests
