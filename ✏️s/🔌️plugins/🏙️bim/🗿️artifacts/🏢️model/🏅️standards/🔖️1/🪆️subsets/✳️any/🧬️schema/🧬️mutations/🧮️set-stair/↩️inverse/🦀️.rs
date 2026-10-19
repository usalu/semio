//! ↩️ Inverse of `SetStair`: an absolute `SetStair` restoring the base values of exactly the provided fields, none when the stair is
//! absent.

use crate::standards::v1::subsets::any::schema::mutations::set_stair::SetStair;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetStair, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_stair::SetStair>,
{
    let Some(stair) = base.stairs.get(&payload.id) else {
        return Vec::new();
    };
    vec![Op::from(SetStair {
        id: payload.id.clone(),
        start: payload.start.map(|_| stair.start),
        direction: payload.direction.map(|_| stair.direction),
        width: payload.width.map(|_| stair.width),
        flight: payload.flight.as_ref().map(|_| stair.flight.clone()),
        top: payload.top.as_ref().map(|_| stair.top.clone()),
        max_riser: payload.max_riser.map(|_| stair.max_riser),
        min_tread: payload.min_tread.map(|_| stair.min_tread),
        stringer: payload.stringer.map(|_| stair.stringer),
        nosing: payload.nosing.map(|_| stair.nosing),
        tread_thickness: payload.tread_thickness.map(|_| stair.tread_thickness),
        riser: payload.riser.map(|_| stair.riser),
        landing_depth: payload.landing_depth.map(|_| stair.landing_depth),
        name: payload.name.as_ref().map(|_| stair.name.clone()),
    })]
}
