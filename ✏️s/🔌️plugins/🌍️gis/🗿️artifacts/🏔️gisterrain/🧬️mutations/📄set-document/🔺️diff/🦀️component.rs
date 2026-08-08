use crate::artifacts::gisterrain::diff::Gis3dTerrainDiff;
use crate::artifacts::gisterrain::Gis3dTerrainDocument;
use crate::artifacts::gisterrain::mutations::Gis3dTerrainMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &Gis3dTerrainMutation, base: &Gis3dTerrainDocument) -> Gis3dTerrainDiff {
    <Gis3dTerrainMutation as protocol::Mutation<Gis3dTerrainDocument>>::diff(mutation, base)
}
