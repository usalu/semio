use crate::artifacts::gisterrain::GisTerrainSnapshot;
use crate::artifacts::gisterrain::mutations::GisTerrainMutation;

pub fn inverse(base: &GisTerrainSnapshot, mutation: &GisTerrainMutation) -> Vec<GisTerrainMutation> {
    <GisTerrainMutation as protocol::Mutation<GisTerrainSnapshot>>::inverse(mutation, base)
}
