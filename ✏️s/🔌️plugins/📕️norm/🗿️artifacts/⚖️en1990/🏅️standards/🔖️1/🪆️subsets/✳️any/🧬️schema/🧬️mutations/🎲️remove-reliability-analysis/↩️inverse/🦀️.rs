//! ↩️ `remove-reliability-analysis` inverse: inserts the removed row back at its position.

use crate::remove_reliability_analysis::RemoveReliabilityAnalysis;
use crate::insert_reliability_analysis::InsertReliabilityAnalysis;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &RemoveReliabilityAnalysis, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_reliability_analysis::InsertReliabilityAnalysis> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &RemoveReliabilityAnalysis, base: &En1990Snapshot) -> Option<InsertReliabilityAnalysis> {
    let index = payload.index;
    let row = base.reliability_analyses.get(index)?;
    Some(InsertReliabilityAnalysis { index: Some(index), item: row.clone() })
}
