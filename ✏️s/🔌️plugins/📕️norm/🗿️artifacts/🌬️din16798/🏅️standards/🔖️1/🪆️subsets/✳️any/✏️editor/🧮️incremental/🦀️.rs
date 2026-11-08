//! 🧮️ Din16798 editor evaluation: the family delegates its check units to the shared `Din16798Units`; the retained `SharedEvaluation` lives in `inferences::live`, read by editor and viewer alike, so a paint after an edit re-runs only the units whose read set changed.

use crate::DinEn16798Family;
use semio_s_artifact_norm_din16798_checks::Din16798Units;
use semio_s_artifact_norm_din16798_model::Din16798Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, EntityEdge, UnitSource};

impl UnitSource for DinEn16798Family {
    type Document = Din16798Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = <Din16798Units as UnitSource>::OWNERS;

    fn units(document: &Din16798Snapshot) -> Vec<CheckUnit<Din16798Snapshot>> {
        Din16798Units::units(document)
    }

    fn references(document: &Din16798Snapshot) -> Vec<EntityEdge> {
        Din16798Units::references(document)
    }

    fn summarise(document: &Din16798Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<semio_s_plugin_norm_compliance::CheckResult> {
        Din16798Units::summarise(document, report)
    }
}

impl semio_s_plugin_norm_compliance::IncrementalNormFamily for DinEn16798Family {}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧭️ownership/🦀️.rs"]
mod ownership;
