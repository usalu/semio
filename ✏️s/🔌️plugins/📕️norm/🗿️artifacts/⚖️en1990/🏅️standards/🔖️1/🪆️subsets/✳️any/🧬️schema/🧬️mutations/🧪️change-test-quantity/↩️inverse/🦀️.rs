//! ↩️ `change-test-quantity` inverse: restores the previous `quantity`.

use crate::change_test_quantity::ChangeTestQuantity;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeTestQuantity, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_test_quantity::ChangeTestQuantity> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTestQuantity, base: &En1990Snapshot) -> Option<ChangeTestQuantity> {
    let r0 = base.test_series.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTestQuantity { id: payload.id.clone(), new_quantity: r0.quantity.clone() })
}
