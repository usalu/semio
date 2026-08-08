//! 🎛 Gis3dTerrain mutation — `SetExaggeration` apply delegate.
use crate::artifacts::gisterrain::Gis3dTerrainDocument;
use crate::artifacts::gisterrain::mutations::Gis3dTerrainMutation;

pub fn apply(projection: &mut Gis3dTerrainDocument, mutation: &Gis3dTerrainMutation) {
    crate::artifacts::gisterrain::mutations::apply_gis_3d_terrain_mutation(projection, mutation);
}
