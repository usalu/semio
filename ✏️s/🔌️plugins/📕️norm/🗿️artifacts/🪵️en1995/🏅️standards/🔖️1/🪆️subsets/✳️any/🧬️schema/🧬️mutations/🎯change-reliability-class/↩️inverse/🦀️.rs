//! ↩️ `change-reliability-class` inverse — restores the pre-change `reliability_class` from BASE state; `change` is its
//! own inverse partner (per `📓️taxonomy.md`).

use crate::change_reliability_class::ChangeReliabilityClass;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &ChangeReliabilityClass, base: &En1995Snapshot) -> Option<crate::change_reliability_class::ChangeReliabilityClass> {
    Some(ChangeReliabilityClass { new_reliability_class: base.reliability_class })
}
//#endregion 🔖️Inverse
