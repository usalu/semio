use crate::artifacts::gismap::schema::diff::*;
//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar



use crate::artifacts::gismap::schema::GisMapArtifact;
use crate::artifacts::gismap::{GisMapSnapshot, MapFeature};
use protocol::{MutationDiff, Patchable};

//#region 🔹Apply
/// Applies an identified-collection delta to a feature list.
pub fn apply_features_delta(items: &[MapFeature], delta: &GisMapFeaturesDelta) -> Vec<MapFeature> {
    let mut next = items.to_vec();
    for id in &delta.removed {
        next.retain(|item| &item.id != id);
    }
    for item in &delta.added {
        next.push(item.clone());
    }
    for entry in &delta.patched {
        if let Some(item) = next.iter_mut().find(|item| item.id == entry.id) {
            item.apply_patch(&entry.patch);
        }
    }
    if let Some(order) = &delta.reordered {
        let mut by_id: std::collections::BTreeMap<_, _> =
            next.into_iter().map(|item| (item.id.clone(), item)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(item) = by_id.remove(id) {
                ordered.push(item);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

fn absorb_features_delta(target: &mut Option<GisMapFeaturesDelta>, incoming: Option<GisMapFeaturesDelta>) {
    if let Some(src) = incoming {
        match target {
            Some(dst) => {
                dst.added.extend(src.added);
                dst.removed.extend(src.removed);
                dst.patched.extend(src.patched);
                if src.reordered.is_some() {
                    dst.reordered = src.reordered;
                }
            }
            None => *target = Some(src),
        }
    }
}

impl GisMapDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &GisMapArtifact) -> GisMapArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(delta) = &self.positions {
            next.positions = apply_features_delta(&next.positions, delta);
        }
        if let Some(delta) = &self.routes {
            next.routes = apply_features_delta(&next.routes, delta);
        }
        if let Some(delta) = &self.regions {
            next.regions = apply_features_delta(&next.regions, delta);
        }
        if let Some(delta) = &self.layer_visibility {
            for (key, value) in &delta.entries {
                match value {
                    Some(v) => { next.layer_visibility.insert(key.clone(), *v); }
                    None => { next.layer_visibility.remove(key); }
                }
            }
        }
        if let Some(delta) = &self.layer_stroke_scale {
            for (key, value) in &delta.entries {
                match value {
                    Some(v) => { next.layer_stroke_scale.insert(key.clone(), *v); }
                    None => { next.layer_stroke_scale.remove(key); }
                }
            }
        }
        if let Some(value) = &self.camera_json { next.camera_json = value.clone(); }
        if let Some(value) = &self.render_mode { next.render_mode = value.clone(); }
        if let Some(value) = &self.vector_style { next.vector_style = value.clone(); }
        if let Some(value) = &self.lod_mode { next.lod_mode = value.clone(); }
        if let Some(value) = &self.locale { next.locale = value.clone(); }
        next
    }
}

impl MutationDiff<GisMapSnapshot> for GisMapDiff {
    fn apply(&self, snapshot: &GisMapSnapshot) -> GisMapSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(delta) = &self.positions {
            next.positions = apply_features_delta(&next.positions, delta);
        }
        if let Some(delta) = &self.routes {
            next.routes = apply_features_delta(&next.routes, delta);
        }
        if let Some(delta) = &self.regions {
            next.regions = apply_features_delta(&next.regions, delta);
        }
        // 🕸️ Keep `drawing`/`value` a pure function of `(positions, routes, regions)` — mirrors
        // `apply_gis_map_mutation`'s identical re-derivation (see `GisMapSnapshot`'s doc comment).
        next = crate::artifacts::gismap::gis_map_snapshot_with_derived_children(next);
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        absorb_features_delta(&mut self.positions, other.positions);
        absorb_features_delta(&mut self.routes, other.routes);
        absorb_features_delta(&mut self.regions, other.regions);
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(camera_json);
        take!(render_mode);
        take!(vector_style);
        take!(lod_mode);
        take!(locale);
        match (&mut self.layer_visibility, other.layer_visibility) {
            (Some(dst), Some(src)) => dst.entries.extend(src.entries),
            (None, Some(src)) => self.layer_visibility = Some(src),
            _ => {}
        }
        match (&mut self.layer_stroke_scale, other.layer_stroke_scale) {
            (Some(dst), Some(src)) => dst.entries.extend(src.entries),
            (None, Some(src)) => self.layer_stroke_scale = Some(src),
            _ => {}
        }
    }
}
//#endregion 🔹Apply

//#region 🔹Helpers
pub fn diff_set_snapshot(snapshot: &GisMapSnapshot) -> GisMapDiff {
    GisMapDiff {
        artifact: Some(Box::new(GisMapArtifact::from_snapshot(snapshot.clone()))),
        ..Default::default()
    }
}
//#endregion 🔹Helpers

//#region 🔹Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn feature(id: &str) -> MapFeature {
        MapFeature { id: id.into(), data: dsl::DslValue::String(id.into()) }
    }

    #[test]
    fn a_whole_artifact_diff_wins_over_every_collection_diff() {
        let base = GisMapSnapshot { positions: vec![feature("p1")], ..Default::default() };
        let replacement = crate::artifacts::gismap::gis_map_snapshot_with_derived_children(GisMapSnapshot { routes: vec![feature("r1")], ..Default::default() });
        let mut diff = GisMapDiff {
            positions: Some(GisMapFeaturesDelta { removed: vec!["p1".into()], ..Default::default() }),
            ..Default::default()
        };
        diff.absorb(diff_set_snapshot(&replacement));
        assert_eq!(diff.apply(&base), replacement);
    }

    #[test]
    fn collection_diffs_absorb_and_apply_add_remove_patch() {
        let base = GisMapSnapshot { positions: vec![feature("p1")], ..Default::default() };
        let mut diff = GisMapDiff {
            positions: Some(GisMapFeaturesDelta { removed: vec!["p1".into()], ..Default::default() }),
            ..Default::default()
        };
        diff.absorb(GisMapDiff {
            positions: Some(GisMapFeaturesDelta { added: vec![feature("p2")], ..Default::default() }),
            ..Default::default()
        });
        let next = diff.apply(&base);
        assert_eq!(next.positions.len(), 1);
        assert_eq!(next.positions[0].id, "p2");
    }
}
//#endregion 🔹Tests
