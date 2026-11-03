//! ↩️ `change-test-variation-known` inverse: restores the previous `variation_known`.

use crate::change_test_variation_known::ChangeTestVariationKnown;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &ChangeTestVariationKnown, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_test_variation_known::ChangeTestVariationKnown> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &ChangeTestVariationKnown, base: &En1990Snapshot) -> Option<ChangeTestVariationKnown> {
    let r0 = base.test_series.iter().find(|row| row.id == payload.id)?;
    Some(ChangeTestVariationKnown { id: payload.id.clone(), new_variation_known: r0.variation_known })
}
