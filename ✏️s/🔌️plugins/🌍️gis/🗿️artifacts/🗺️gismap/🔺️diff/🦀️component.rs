//! 🔺️ GIS map artifact — the operation diff and its `MutationDiff` law (split out of the old
//! constitutional `op` crate).


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::gismap::{GisMapDocument, MapFeature, MapFeaturePatch};
use protocol::{CollectionDiff, MutationDiff, Patchable};
use serde::{Deserialize, Serialize};

//#region 🔖️Helpers
fn apply_map_collection_diff(items: &mut Vec<MapFeature>, diff: &CollectionDiff<String, MapFeaturePatch, MapFeature>) {
    for id in &diff.removed {
        items.retain(|item| &item.id != id);
    }
    for patch in &diff.modified {
        if let Some(item) = items.iter_mut().find(|item| item.id == patch.id) {
            item.apply_patch(&patch.patch);
        }
    }
    for added in &diff.added {
        items.push(added.clone());
    }
}

fn absorb_map_collection_diff(target: &mut Option<CollectionDiff<String, MapFeaturePatch, MapFeature>>, incoming: Option<CollectionDiff<String, MapFeaturePatch, MapFeature>>) {
    if let Some(next) = incoming {
        match target {
            Some(existing) => {
                existing.removed.extend(next.removed);
                existing.modified.extend(next.modified);
                existing.added.extend(next.added);
            }
            None => *target = Some(next),
        }
    }
}
//#endregion 🔖️Helpers

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GisMapDiff {
    pub document: Option<GisMapDocument>,
    pub positions: Option<CollectionDiff<String, MapFeaturePatch, MapFeature>>,
    pub routes: Option<CollectionDiff<String, MapFeaturePatch, MapFeature>>,
    pub regions: Option<CollectionDiff<String, MapFeaturePatch, MapFeature>>,
}

impl MutationDiff<GisMapDocument> for GisMapDiff {
    fn apply(&self, projection: &GisMapDocument) -> GisMapDocument {
        if let Some(document) = &self.document {
            return document.clone();
        }
        let mut next = projection.clone();
        if let Some(diff) = &self.positions {
            apply_map_collection_diff(&mut next.positions, diff);
        }
        if let Some(diff) = &self.routes {
            apply_map_collection_diff(&mut next.routes, diff);
        }
        if let Some(diff) = &self.regions {
            apply_map_collection_diff(&mut next.regions, diff);
        }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.document.is_some() {
            *self = GisMapDiff { document: other.document, ..Default::default() };
            return;
        }
        absorb_map_collection_diff(&mut self.positions, other.positions);
        absorb_map_collection_diff(&mut self.routes, other.routes);
        absorb_map_collection_diff(&mut self.regions, other.regions);
    }
}
//#endregion 🔖️Diff

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn feature(id: &str) -> MapFeature {
        MapFeature { id: id.into(), data: dsl::DslValue::String(id.into()) }
    }

    #[test]
    fn a_whole_document_diff_wins_over_every_collection_diff() {
        let base = GisMapDocument { positions: vec![feature("p1")], ..Default::default() };
        let replacement = GisMapDocument { routes: vec![feature("r1")], ..Default::default() };
        let mut diff = GisMapDiff { positions: Some(CollectionDiff { removed: vec!["p1".into()], modified: Vec::new(), added: Vec::new() }), ..Default::default() };
        diff.absorb(GisMapDiff { document: Some(replacement.clone()), ..Default::default() });
        assert_eq!(diff.apply(&base), replacement);
    }

    #[test]
    fn collection_diffs_absorb_and_apply_add_remove_patch() {
        let base = GisMapDocument { positions: vec![feature("p1")], ..Default::default() };
        let mut diff = GisMapDiff { positions: Some(CollectionDiff { removed: vec!["p1".into()], modified: Vec::new(), added: Vec::new() }), ..Default::default() };
        diff.absorb(GisMapDiff { positions: Some(CollectionDiff { removed: Vec::new(), modified: Vec::new(), added: vec![feature("p2")] }), ..Default::default() });
        let next = diff.apply(&base);
        assert_eq!(next.positions.len(), 1);
        assert_eq!(next.positions[0].id, "p2");
    }
}
//#endregion 🧪️Tests
