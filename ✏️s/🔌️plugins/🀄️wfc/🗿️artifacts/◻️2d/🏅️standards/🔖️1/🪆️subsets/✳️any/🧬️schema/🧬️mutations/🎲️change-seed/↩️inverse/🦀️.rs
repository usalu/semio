//! ↩️ Inverse for `ChangeSeed` — built from a real BASE lookup, so a target the base never held
//! yields an empty inverse (nothing to undo) rather than a fabricated one.

use crate::standards::v1::subsets::any::schema::mutations::change_seed::change_seed;
use crate::schema::snapshot::Wfc2dSnapshot;

pub fn inverse<Op>(_payload: &super::ChangeSeed, base: &Wfc2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_seed::ChangeSeed>,
{
    Ok((|| {
    vec![change_seed(base.seed)]

    })())
}
