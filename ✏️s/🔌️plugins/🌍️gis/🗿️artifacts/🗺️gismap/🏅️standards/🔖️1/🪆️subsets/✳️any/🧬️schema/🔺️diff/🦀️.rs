//! 🧬️ GIS map diff schema — sparse field delta over the artifact.

use crate::{MapFeature, MapFeaturePatch};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::{BTreeMap, BTreeSet};

//#region 🔹Diff
/// 🔺️ Sparse field delta for the GIS map artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, ToValue, FromValue)]
#[value(rename_all = "camelCase", default, deny_unknown_fields)]
#[artifact_schema(id = "s.gis.gismap")]
pub struct GisMapDiff {
    #[state(artifact)]
    pub positions: Option<GisMapFeaturesDelta>,
    #[state(artifact)]
    pub routes: Option<GisMapFeaturesDelta>,
    #[state(artifact)]
    pub regions: Option<GisMapFeaturesDelta>,
}
//#endregion 🔹Diff

//#region 🔹DeltaHelpers
/// Identified-collection delta for feature lists: removed ids, appended rows, keyed data patches and, only when the final order is
/// not "survivors then appended", the complete final id order.
#[derive(Clone, Debug, Default, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct GisMapFeaturesDelta {
    pub added: Vec<MapFeature>,
    pub removed: Vec<String>,
    pub patched: Vec<GisMapFeaturePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched feature entry.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct GisMapFeaturePatchEntry {
    pub id: String,
    pub patch: MapFeaturePatch,
}
//#endregion 🔹DeltaHelpers

//#region 🔹Rows
use crate::GisMapSnapshot;
use protocol::MutationDiff;

trait RowKey {
    fn row_key(&self) -> &str;
}

/// 🧮️ The one keyed-collection algebra every feature collection delta shares: removed ids, appended rows, keyed field patches and an
/// optional complete final order — apply, absorb (create∘delete cancels, patch∘patch merges, a later order supersedes), the
/// negative delta and the state delta all work on this shape.
#[derive(Clone, Debug, PartialEq)]
struct Rows<T, P> {
    added: Vec<T>,
    removed: Vec<String>,
    patched: Vec<(String, P)>,
    reordered: Option<Vec<String>>,
}

impl<T, P> Default for Rows<T, P> {
    fn default() -> Self {
        Self { added: Vec::new(), removed: Vec::new(), patched: Vec::new(), reordered: None }
    }
}

fn keys_of<T: RowKey>(rows: &[T]) -> Vec<String> {
    rows.iter().map(|row| row.row_key().to_string()).collect()
}

impl<T: Clone + RowKey, P: Clone + Default + PartialEq> Rows<T, P> {
    fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.patched.iter().all(|(_, patch)| *patch == P::default()) && self.reordered.is_none()
    }

    fn normalize(&mut self) {
        self.removed.sort();
        self.removed.dedup();
        self.patched.sort_by(|left, right| left.0.cmp(&right.0));
    }

    fn apply(&self, rows: &[T], patch_row: impl Fn(&mut T, &P)) -> protocol::MutationApplyResult<Vec<T>> {
        for (index, id) in self.removed.iter().enumerate() {
            if !rows.iter().any(|row| row.row_key() == id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "removed row does not exist").at(["removed".to_string(), index.to_string()]));
            }
            if self.removed[..index].contains(id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "row is removed more than once").at(["removed".to_string(), index.to_string()]));
            }
        }
        for (index, item) in self.added.iter().enumerate() {
            let id = item.row_key();
            if rows.iter().any(|row| row.row_key() == id && !self.removed.iter().any(|removed| removed == id)) || self.added[..index].iter().any(|prior| prior.row_key() == id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added row identity already exists").at(["added".to_string(), index.to_string()]));
            }
        }
        for (index, (id, _)) in self.patched.iter().enumerate() {
            if !rows.iter().any(|row| row.row_key() == id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "patched row does not exist").at(["patched".to_string(), index.to_string()]));
            }
            if self.removed.contains(id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.conflicting-target", "row cannot be removed and patched").at(["patched".to_string(), index.to_string()]));
            }
            if self.patched[..index].iter().any(|(prior, _)| prior == id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "row is patched more than once").at(["patched".to_string(), index.to_string()]));
            }
        }
        let mut next: Vec<T> = rows.iter().filter(|row| !self.removed.iter().any(|id| id == row.row_key())).cloned().collect();
        for (id, patch) in &self.patched {
            if let Some(row) = next.iter_mut().find(|row| row.row_key() == id) {
                patch_row(row, patch);
            }
        }
        next.extend(self.added.iter().cloned());
        if let Some(order) = &self.reordered {
            if order.len() != next.len() || order.iter().enumerate().any(|(index, id)| order[..index].contains(id) || !next.iter().any(|row| row.row_key() == id)) {
                return Err(protocol::MutationApplyError::new("mutation.apply.invalid-order", "reorder must be a complete unique permutation").at(["reordered"]));
            }
            let mut by_id: BTreeMap<String, T> = next.into_iter().map(|row| (row.row_key().to_string(), row)).collect();
            let mut ordered = Vec::with_capacity(order.len());
            for id in order {
                ordered.push(by_id.remove(id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "reordered row does not exist").at(["reordered".to_string(), id.clone()]))?);
            }
            next = ordered;
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self, patch_row: impl Fn(&mut T, &P), merge: impl Fn(&mut P, P)) {
        let Self { added, removed, patched, reordered } = other;
        for id in &removed {
            if let Some(at) = self.added.iter().position(|row| row.row_key() == id) {
                self.added.remove(at);
                continue;
            }
            self.patched.retain(|(key, _)| key != id);
            if !self.removed.contains(id) {
                self.removed.push(id.clone());
            }
        }
        for (id, incoming) in patched {
            if let Some(row) = self.added.iter_mut().find(|row| row.row_key() == id) {
                patch_row(row, &incoming);
            } else if let Some((_, existing)) = self.patched.iter_mut().find(|(key, _)| *key == id) {
                merge(existing, incoming);
            } else {
                self.patched.push((id, incoming));
            }
        }
        match reordered {
            Some(order) => self.reordered = Some(order),
            None => {
                if let Some(order) = self.reordered.as_mut() {
                    order.retain(|id| !removed.contains(id));
                    order.extend(added.iter().map(|row| row.row_key().to_string()));
                }
            }
        }
        self.added.extend(added);
        self.normalize();
    }

    fn inverse(&self, base: &[T], invert: impl Fn(&P, &T) -> P) -> Self {
        let base_keys = keys_of(base);
        let added_keys: BTreeSet<&str> = self.added.iter().map(RowKey::row_key).collect();
        let restored: Vec<T> = base.iter().filter(|row| self.removed.iter().any(|id| id == row.row_key())).cloned().collect();
        let patched = self.patched.iter().filter(|(id, _)| !added_keys.contains(id.as_str())).filter_map(|(id, patch)| base.iter().find(|row| row.row_key() == id).map(|row| (id.clone(), invert(patch, row)))).collect();
        let mut after: Vec<String> = base_keys.iter().filter(|id| !self.removed.contains(id)).cloned().chain(self.added.iter().map(|row| row.row_key().to_string())).collect();
        if let Some(order) = &self.reordered {
            after = order.clone();
        }
        let natural: Vec<String> = after.into_iter().filter(|id| !added_keys.contains(id.as_str())).chain(restored.iter().map(|row| row.row_key().to_string())).collect();
        let reordered = (natural != base_keys).then_some(base_keys);
        let mut inverse = Self { added: restored, removed: self.added.iter().map(|row| row.row_key().to_string()).collect(), patched, reordered };
        inverse.normalize();
        inverse
    }

    fn between(base: &[T], other: &[T], compare: impl Fn(&T, &T) -> Option<Option<P>>) -> Self {
        let base_keys = keys_of(base);
        let other_keys = keys_of(other);
        let mut delta = Self::default();
        let mut replaced = BTreeSet::new();
        for row in other {
            let Some(source) = base.iter().find(|candidate| candidate.row_key() == row.row_key()) else { continue };
            match compare(source, row) {
                Some(Some(patch)) => delta.patched.push((row.row_key().to_string(), patch)),
                Some(None) => {}
                None => {
                    replaced.insert(row.row_key().to_string());
                }
            }
        }
        delta.removed = base.iter().filter(|row| !other_keys.iter().any(|id| id == row.row_key()) || replaced.contains(row.row_key())).map(|row| row.row_key().to_string()).collect();
        delta.added = other.iter().filter(|row| !base_keys.iter().any(|id| id == row.row_key()) || replaced.contains(row.row_key())).cloned().collect();
        let natural: Vec<String> = base_keys.iter().filter(|id| !delta.removed.contains(id)).cloned().chain(delta.added.iter().map(|row| row.row_key().to_string())).collect();
        delta.reordered = (natural != other_keys).then_some(other_keys);
        delta.normalize();
        delta
    }
}

fn patch_feature(feature: &mut MapFeature, patch: &MapFeaturePatch) {
    if let Some(data) = &patch.data {
        feature.data = data.clone();
    }
}

fn merge_feature_patch(existing: &mut MapFeaturePatch, incoming: MapFeaturePatch) {
    if incoming.data.is_some() {
        existing.data = incoming.data;
    }
}

fn invert_feature_patch(patch: &MapFeaturePatch, base: &MapFeature) -> MapFeaturePatch {
    MapFeaturePatch { data: patch.data.as_ref().map(|_| base.data.clone()) }
}

fn compare_features(base: &MapFeature, other: &MapFeature) -> Option<Option<MapFeaturePatch>> {
    Some((base.data != other.data).then(|| MapFeaturePatch { data: Some(other.data.clone()) }))
}

impl RowKey for MapFeature {
    fn row_key(&self) -> &str {
        &self.id
    }
}

impl GisMapFeaturesDelta {
    fn rows(&self) -> Rows<MapFeature, MapFeaturePatch> {
        Rows { added: self.added.clone(), removed: self.removed.clone(), patched: self.patched.iter().map(|entry| (entry.id.clone(), entry.patch.clone())).collect(), reordered: self.reordered.clone() }
    }
    fn from_rows(rows: Rows<MapFeature, MapFeaturePatch>) -> Self {
        Self { added: rows.added, removed: rows.removed, patched: rows.patched.into_iter().map(|(id, patch)| GisMapFeaturePatchEntry { id, patch }).collect(), reordered: rows.reordered }
    }
    /// 🕳️ Whether the delta changes nothing.
    pub fn is_empty(&self) -> bool {
        self.rows().is_empty()
    }
}

/// Applies an identified-collection delta to a feature list.
pub fn apply_features_delta(items: &[MapFeature], delta: &GisMapFeaturesDelta) -> protocol::MutationApplyResult<Vec<MapFeature>> {
    delta.rows().apply(items, patch_feature)
}

fn absorb_features_delta(target: &mut Option<GisMapFeaturesDelta>, incoming: Option<GisMapFeaturesDelta>) {
    match (target.as_mut(), incoming) {
        (Some(dst), Some(src)) => {
            let mut rows = dst.rows();
            rows.absorb(src.rows(), patch_feature, merge_feature_patch);
            *dst = GisMapFeaturesDelta::from_rows(rows);
        }
        (None, Some(src)) => *target = Some(src),
        _ => {}
    }
    if target.as_ref().is_some_and(GisMapFeaturesDelta::is_empty) {
        *target = None;
    }
}
//#endregion 🔹Rows

//#region 🔹Apply
impl MutationDiff<GisMapSnapshot> for GisMapDiff {
    fn apply(&self, snapshot: &GisMapSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GisMapSnapshot> {
        let mut next = snapshot.clone();
        if let Some(delta) = &self.positions {
            next.positions = apply_features_delta(&next.positions, delta).map_err(|error| error.under(["positions"]))?;
        }
        if let Some(delta) = &self.routes {
            next.routes = apply_features_delta(&next.routes, delta).map_err(|error| error.under(["routes"]))?;
        }
        if let Some(delta) = &self.regions {
            next.regions = apply_features_delta(&next.regions, delta).map_err(|error| error.under(["regions"]))?;
        }
        Ok(next)
    }
    fn absorb(&mut self, other: Self) {
        absorb_features_delta(&mut self.positions, other.positions);
        absorb_features_delta(&mut self.routes, other.routes);
        absorb_features_delta(&mut self.regions, other.regions);
    }
}

impl protocol::DiffAlgebra<GisMapSnapshot> for GisMapDiff {
    fn inverse(&self, base: &GisMapSnapshot) -> Self {
        Self {
            positions: self.positions.as_ref().map(|delta| GisMapFeaturesDelta::from_rows(delta.rows().inverse(&base.positions, invert_feature_patch))),
            routes: self.routes.as_ref().map(|delta| GisMapFeaturesDelta::from_rows(delta.rows().inverse(&base.routes, invert_feature_patch))),
            regions: self.regions.as_ref().map(|delta| GisMapFeaturesDelta::from_rows(delta.rows().inverse(&base.regions, invert_feature_patch))),
        }
    }
    fn between(base: &GisMapSnapshot, other: &GisMapSnapshot) -> Self {
        let delta = |left: &[MapFeature], right: &[MapFeature]| Some(GisMapFeaturesDelta::from_rows(Rows::between(left, right, compare_features))).filter(|delta| !delta.is_empty());
        Self { positions: delta(&base.positions, &other.positions), routes: delta(&base.routes, &other.routes), regions: delta(&base.regions, &other.regions) }
    }
    fn is_empty(&self) -> bool {
        [&self.positions, &self.routes, &self.regions].into_iter().all(|delta| delta.as_ref().is_none_or(GisMapFeaturesDelta::is_empty))
    }
}
//#endregion 🔹Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
