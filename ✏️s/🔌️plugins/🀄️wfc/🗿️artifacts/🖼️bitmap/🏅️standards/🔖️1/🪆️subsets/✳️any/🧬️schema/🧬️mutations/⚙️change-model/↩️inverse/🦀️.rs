//! ↩️ Inverse for `ChangeModel` — the prior parameter record; the model always exists, so this is
//! only empty when the forward mutation was itself a no-op.

use crate::standards::v1::subsets::any::schema::mutations::change_model::change_model;
use crate::schema::snapshot::BitmapSnapshot;

pub fn inverse<Op>(_payload: &super::ChangeModel, base: &BitmapSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_model::ChangeModel>,
{
    Ok((|| {
    vec![change_model(base.model.pattern_size, base.model.symmetry, base.model.periodic_input, base.model.ground)]

    })())
}
