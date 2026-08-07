//! 🔺️ GIS terrain artifact — the operation diff and its `OperationDiff` law (split out of the old
//! constitutional `op` crate).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::gisterrain::Gis3dTerrainDocument;
use protocol::OperationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
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
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_diffs_absorb_last_writer_wins_and_apply_onto_the_projection() {
        let base = Gis3dTerrainDocument { exaggeration: 1.0, imported_features_json: String::new() };
        let mut diff = Gis3dTerrainDiff { exaggeration: Some(2.0), ..Default::default() };
        diff.absorb(Gis3dTerrainDiff { exaggeration: Some(3.0), imported_features_json: Some("null".into()), ..Default::default() });
        let next = diff.apply(&base);
        assert_eq!(next.exaggeration, 3.0);
        assert_eq!(next.imported_features_json, "null");
    }

    #[test]
    fn a_whole_document_diff_wins_over_every_field_diff() {
        let base = Gis3dTerrainDocument { exaggeration: 1.0, imported_features_json: String::new() };
        let replacement = Gis3dTerrainDocument { exaggeration: 9.0, imported_features_json: "{}".into() };
        let mut diff = Gis3dTerrainDiff { exaggeration: Some(2.0), ..Default::default() };
        diff.absorb(Gis3dTerrainDiff { document: Some(replacement.clone()), ..Default::default() });
        assert_eq!(diff.apply(&base), replacement);
    }
}
//#endregion 🧪️Tests
