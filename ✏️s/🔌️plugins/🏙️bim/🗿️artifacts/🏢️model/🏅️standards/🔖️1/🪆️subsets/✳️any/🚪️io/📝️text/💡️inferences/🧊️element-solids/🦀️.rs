//! 📊️ Native JSON inference table projection.
use std::collections::BTreeMap;
use crate::ModelSnapshot;
use crate::standards::v1::subsets::any::schema::inferences::element_solids::{ElementSolid, SolidFamily};
use crate::standards::v1::subsets::any::schema::inferences::element_solids::plan_kit::is_curved;

/// 📋️ The canonical JSON table `{element id: {volume, min, max}}` of the planar column, beam, slab, roof, stair and railing solids: the projection the third-party case compares.
pub fn planar_projection_json(snapshot: &ModelSnapshot, solids: &std::collections::BTreeMap<String, ElementSolid>) -> String {
    let ours = |family: SolidFamily| matches!(family, SolidFamily::Column | SolidFamily::Beam | SolidFamily::Slab | SolidFamily::Roof | SolidFamily::Stair | SolidFamily::Railing);
    let rows: Vec<String> = solids
        .iter()
        .filter(|(id, solid)| ours(solid.family) && !is_curved(snapshot, id))
        .map(|(id, solid)| {
            let (lo, hi) = (solid.bounds.min, solid.bounds.max);
            format!("{}:{{\"volume\":{},\"min\":[{},{},{}],\"max\":[{},{},{}]}}", semio_framework_pack_json::to_json_string(id), solid.volume, lo.x, lo.y, lo.z, hi.x, hi.y, hi.z)
        })
        .collect();
    format!("{{{}}}", rows.join(","))
}
