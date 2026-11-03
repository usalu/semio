//! 🎲 `insert-reliability-analysis` diff: inserts the reliability analysis row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_reliability_analysis::InsertReliabilityAnalysis;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990ReliabilityAnalysisDelta};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &InsertReliabilityAnalysis, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if base.reliability_analyses.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Reliability analysis {key} already exists."), [key]);
    }
    let index = payload.index.unwrap_or(usize::MAX).min(base.reliability_analyses.len());
    let outcome = protocol::MutationOutcome::new(En1990Diff { reliability_analyses: En1990ReliabilityAnalysisDelta::insertion(index, payload.item.clone()), ..Default::default() });
    if payload.index.is_none_or(|requested| requested == index) {
        return outcome;
    }
    outcome.warning("mutation.clamped", format!("Position {} is past the end of the reliability analysis list; inserted at {index}.", payload.index.unwrap_or(index)))
}
