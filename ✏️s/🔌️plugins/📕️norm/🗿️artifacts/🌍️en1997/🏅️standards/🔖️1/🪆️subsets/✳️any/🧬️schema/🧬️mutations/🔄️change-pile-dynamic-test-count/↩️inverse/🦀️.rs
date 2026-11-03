//! ↩️ `change-pile-dynamic-test-count` inverse: restores the previous `dynamic_test_count`.

use crate::change_pile_dynamic_test_count::ChangePileDynamicTestCount;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn inverse<Op>(payload: &ChangePileDynamicTestCount, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_pile_dynamic_test_count::ChangePileDynamicTestCount> {
    Ok(match base.piles.iter().find(|row| row.id == payload.id) {
        Some(row) => vec![Op::from(ChangePileDynamicTestCount { id: payload.id.clone(), new_dynamic_test_count: row.dynamic_test_count })],
        None => Vec::new(),
    })
}
