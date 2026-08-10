//! 🎛 Gis3dTerrain mutation — `SetImportedFeatures` apply delegate.
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use crate::artifacts::gisterrain::mutations::GisTerrainMutation;

pub fn apply(snapshot: &mut GisTerrainSnapshot, mutation: &GisTerrainMutation) {
    crate::artifacts::gisterrain::mutations::apply_gis_terrain_mutation(snapshot, mutation);
}
