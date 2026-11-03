//! ↩️ `insert-reliability-analysis` inverse: removes the inserted row again.

use crate::insert_reliability_analysis::InsertReliabilityAnalysis;
use crate::remove_reliability_analysis::RemoveReliabilityAnalysis;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(payload: &InsertReliabilityAnalysis, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_reliability_analysis::RemoveReliabilityAnalysis> {
    Ok(previous(payload, base).map(|restore| vec![Op::from(restore)]).unwrap_or_default())
}

fn previous(payload: &InsertReliabilityAnalysis, base: &En1990Snapshot) -> Option<RemoveReliabilityAnalysis> {
    let index = payload.index.unwrap_or(usize::MAX).min(base.reliability_analyses.len());
    Some(RemoveReliabilityAnalysis { index })
}
