//! 🧮️ Din4108 editor evaluation: the family delegates its check units to the shared `Din4108Units`; the retained `SharedEvaluation` lives in `inferences::live`, read by editor and viewer alike, so a paint after an edit re-runs only the units whose read set changed.

use crate::Din4108Family;
use semio_s_artifact_norm_din4108_inferences::Din4108Units;
use semio_s_artifact_norm_din4108_model::Din4108Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, EntityEdge, UnitSource};

impl UnitSource for Din4108Family {
    type Document = Din4108Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = <Din4108Units as UnitSource>::OWNERS;

    fn units(document: &Din4108Snapshot) -> Vec<CheckUnit<Din4108Snapshot>> {
        Din4108Units::units(document)
    }

    fn references(document: &Din4108Snapshot) -> Vec<EntityEdge> {
        Din4108Units::references(document)
    }

    fn summarise(document: &Din4108Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<semio_s_plugin_norm_compliance::CheckResult> {
        Din4108Units::summarise(document, report)
    }
}

impl semio_s_plugin_norm_compliance::IncrementalNormFamily for Din4108Family {}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧭️ownership/🦀️.rs"]
mod ownership;
