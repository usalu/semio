//! 🧬️ GIS map diff schema — sparse field delta over the artifact.

use crate::{MapFeature, MapFeaturePatch};
use ::semio_framework_schema::ArtifactSchema;
use semio_framework_value_derive::{FromValue, ToValue};

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
protocol::list_delta! {
    /// 🧩 Positional delta for a feature list (`protocol::list_delta`): `removed` rows carry their BASE index, `inserted` rows their AFTER
    /// index, `moved` rows both coordinates, `modified` rows a sparse patch keyed by id. No order list is ever carried.
    pub GisMapFeaturesDelta { removal: GisMapFeatureRemoval, insertion: GisMapFeatureInsertion, relocation: GisMapFeatureRelocation, modification: GisMapFeatureModification, row: MapFeature, patch: MapFeaturePatch, key: id }
}
//#endregion 🔹DeltaHelpers

//#region 🔹Rows
use crate::GisMapSnapshot;
use protocol::list_delta::RowPatch;
use protocol::MutationDiff;

impl RowPatch<MapFeature> for MapFeaturePatch {
    fn commit_into(&self, row: &mut MapFeature, _capability: protocol::ApplyCapability) -> Result<(), protocol::MutationApplyError> {
        patch_feature(row, self);
        Ok(())
    }

    fn absorb(&mut self, later: Self) {
        merge_feature_patch(self, later);
    }

    fn inverse(&self, row: &MapFeature) -> Self {
        invert_feature_patch(self, row)
    }

    fn is_empty(&self) -> bool {
        *self == MapFeaturePatch::default()
    }
}

fn patch_feature(feature: &mut MapFeature, patch: &MapFeaturePatch) {
    if let Some(data) = &patch.data {
        feature.data = data.clone();
    }
    crate::schema::feature::apply_property_edits(&mut feature.data, &patch.properties);
}

fn merge_feature_patch(existing: &mut MapFeaturePatch, incoming: MapFeaturePatch) {
    if incoming.data.is_some() {
        existing.data = incoming.data;
        existing.properties = incoming.properties;
    } else {
        existing.properties.extend(incoming.properties);
    }
}

fn invert_feature_patch(patch: &MapFeaturePatch, base: &MapFeature) -> MapFeaturePatch {
    if patch.data.is_some() {
        return MapFeaturePatch { data: Some(base.data.clone()), properties: Vec::new() };
    }
    MapFeaturePatch { data: None, properties: crate::schema::feature::invert_property_edits(&base.data, &patch.properties) }
}

fn absorb_features_delta(target: &mut Option<GisMapFeaturesDelta>, incoming: Option<GisMapFeaturesDelta>) {
    match (target.as_mut(), incoming) {
        (Some(dst), Some(src)) => dst.absorb(src),
        (None, Some(src)) => *target = Some(src),
        (_, None) => {}
    }
    if target.as_ref().is_some_and(GisMapFeaturesDelta::is_empty) {
        *target = None;
    }
}
//#endregion 🔹Rows

//#region 🔹Apply
impl MutationDiff<GisMapSnapshot> for GisMapDiff {
    fn apply(&self, snapshot: &GisMapSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<GisMapSnapshot> {
        let mut next = snapshot.clone();
        if let Some(delta) = &self.positions {
            next.positions = delta.commit_onto(&next.positions, capability).map_err(|error| error.under(["positions"]))?;
        }
        if let Some(delta) = &self.routes {
            next.routes = delta.commit_onto(&next.routes, capability).map_err(|error| error.under(["routes"]))?;
        }
        if let Some(delta) = &self.regions {
            next.regions = delta.commit_onto(&next.regions, capability).map_err(|error| error.under(["regions"]))?;
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
            positions: self.positions.as_ref().map(|delta| delta.inverse(&base.positions)),
            routes: self.routes.as_ref().map(|delta| delta.inverse(&base.routes)),
            regions: self.regions.as_ref().map(|delta| delta.inverse(&base.regions)),
        }
    }
    fn is_empty(&self) -> bool {
        [&self.positions, &self.routes, &self.regions].into_iter().all(|delta| delta.as_ref().is_none_or(GisMapFeaturesDelta::is_empty))
    }
}
//#endregion 🔹Apply

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
