//! ↩️ Inverse of `DeleteCurtainPanelOverride`: the concrete create of the removed override, in storage order (see the shared cascade).

use super::super::cascade;
use super::DeleteCurtainPanelOverride;
use crate::standards::v1::subsets::any::schema::{snapshot::ModelSnapshot};

pub fn inverse<Op>(payload: &DeleteCurtainPanelOverride, base: &ModelSnapshot) -> Vec<Op>
where
    Op: crate::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse,
{
    cascade::inverse(base, std::slice::from_ref(&payload.id))
}
