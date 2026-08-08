use crate::artifacts::gismap::diff::GisMapDiff;
use crate::artifacts::gismap::GisMapDocument;
use crate::artifacts::gismap::mutations::GisMapMutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &GisMapMutation, base: &GisMapDocument) -> GisMapDiff {
    <GisMapMutation as protocol::Mutation<GisMapDocument>>::diff(mutation, base)
}
