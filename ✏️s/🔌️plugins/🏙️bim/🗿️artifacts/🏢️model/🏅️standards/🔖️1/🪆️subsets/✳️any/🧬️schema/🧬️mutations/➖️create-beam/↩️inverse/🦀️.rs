//! ↩️ Inverse of `CreateBeam`: the concrete `DeleteBeam` of the id it created, none when the id was already taken.

use crate::standards::v1::subsets::any::schema::mutations::delete_beam::DeleteBeam;
use crate::standards::v1::subsets::any::schema::mutations::create_beam::CreateBeam;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &CreateBeam, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_beam::DeleteBeam>,
{
    if base.beams.contains_key(&payload.id) {
        return Vec::new();
    }
    vec![Op::from(DeleteBeam { id: payload.id.clone() })]
}
