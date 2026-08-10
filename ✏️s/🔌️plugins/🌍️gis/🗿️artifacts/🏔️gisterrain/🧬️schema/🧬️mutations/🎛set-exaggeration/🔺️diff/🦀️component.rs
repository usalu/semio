use crate::artifacts::gisterrain::diff::GisTerrainDiff;
use crate::artifacts::gisterrain::GisTerrainSnapshot;
use crate::artifacts::gisterrain::mutations::GisTerrainMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &GisTerrainMutation, base: &GisTerrainSnapshot) -> GisTerrainDiff {
    <GisTerrainMutation as protocol::Mutation<GisTerrainSnapshot>>::diff(mutation, base)
}
