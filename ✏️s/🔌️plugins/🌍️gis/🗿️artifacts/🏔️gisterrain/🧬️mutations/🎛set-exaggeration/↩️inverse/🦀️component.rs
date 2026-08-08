use crate::artifacts::gisterrain::Gis3dTerrainDocument;
use crate::artifacts::gisterrain::mutations::Gis3dTerrainMutation;

pub fn inverse(base: &Gis3dTerrainDocument, mutation: &Gis3dTerrainMutation) -> Vec<Gis3dTerrainMutation> {
    <Gis3dTerrainMutation as protocol::Mutation<Gis3dTerrainDocument>>::inverse(mutation, base)
}
