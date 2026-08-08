use crate::artifacts::gismap::GisMapDocument;
use crate::artifacts::gismap::mutations::GisMapMutation;

pub fn inverse(base: &GisMapDocument, mutation: &GisMapMutation) -> Vec<GisMapMutation> {
    <GisMapMutation as protocol::Mutation<GisMapDocument>>::inverse(mutation, base)
}
