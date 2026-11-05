//! ↩️ Captures the prior pin sequence once, preserving its exact order.
use crate::schema::snapshot::BitmapSnapshot;

pub fn inverse<Op>(payload: &super::RestorePins, base: &BitmapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::restore_pins::RestorePins>,
{
    Ok(if payload.pins == base.pinned { Vec::new() } else { vec![crate::schema::mutations::restore_pins::restore_pins(base.pinned.clone())] })
}
