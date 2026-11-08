//! 🧮️ Din18599 editor evaluation: the family delegates its check units to the shared `Din18599Units`; the retained `SharedEvaluation` lives in `inferences::live`, read by editor and viewer alike, so a paint after an edit re-runs only the units whose read set changed.

use crate::DinV18599Family;
use semio_s_artifact_norm_din18599_inferences::Din18599Units;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, EntityEdge, UnitSource};

impl UnitSource for DinV18599Family {
    type Document = Din18599Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = <Din18599Units as UnitSource>::OWNERS;

    fn units(document: &Din18599Snapshot) -> Vec<CheckUnit<Din18599Snapshot>> {
        Din18599Units::units(document)
    }

    fn references(document: &Din18599Snapshot) -> Vec<EntityEdge> {
        Din18599Units::references(document)
    }

    fn summarise(document: &Din18599Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<semio_s_plugin_norm_compliance::CheckResult> {
        Din18599Units::summarise(document, report)
    }
}

impl semio_s_plugin_norm_compliance::IncrementalNormFamily for DinV18599Family {}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧭️ownership/🦀️.rs"]
mod ownership;
