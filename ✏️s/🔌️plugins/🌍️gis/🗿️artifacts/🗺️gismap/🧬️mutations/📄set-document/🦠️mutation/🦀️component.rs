//! 📄 GisMap mutation — `SetDocument` apply delegate.
use crate::artifacts::gismap::GisMapDocument;
use crate::artifacts::gismap::mutations::GisMapMutation;

pub fn apply(projection: &mut GisMapDocument, mutation: &GisMapMutation) {
    crate::artifacts::gismap::mutations::apply_gis_map_mutation(projection, mutation);
}
