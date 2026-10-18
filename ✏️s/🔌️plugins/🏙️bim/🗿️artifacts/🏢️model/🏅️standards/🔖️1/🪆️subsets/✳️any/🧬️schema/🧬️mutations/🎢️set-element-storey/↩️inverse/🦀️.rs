//! ↩️ Inverse of `SetElementStorey`: an absolute `SetElementStorey` back to the storey the element stood on in the base, none when the element is absent or stands on no storey.

use crate::standards::v1::subsets::any::schema::mutations::elements;
use crate::standards::v1::subsets::any::schema::mutations::set_element_storey::SetElementStorey;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SetElementStorey, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::set_element_storey::SetElementStorey>,
{
    match elements::storey_of(base, &payload.id) {
        Some(storey) => vec![Op::from(SetElementStorey { id: payload.id.clone(), storey })],
        None => Vec::new(),
    }
}
