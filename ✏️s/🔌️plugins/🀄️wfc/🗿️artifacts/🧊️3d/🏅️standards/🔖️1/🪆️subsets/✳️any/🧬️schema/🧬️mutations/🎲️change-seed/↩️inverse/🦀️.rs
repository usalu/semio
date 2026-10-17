//! ↩️ Inverse for `ChangeSeed` — restores the PRIOR seed from a real BASE lookup (the seed field
//! always exists, so this is never a no-op).

use crate::standards::v1::subsets::any::schema::mutations::change_seed::change_seed;
use crate::schema::snapshot::Wfc3dSnapshot;

pub fn inverse<Op>(_payload: &super::ChangeSeed, base: &Wfc3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_seed::ChangeSeed>,
{
    Ok((|| {
    vec![change_seed(base.seed)]

    })())
}
