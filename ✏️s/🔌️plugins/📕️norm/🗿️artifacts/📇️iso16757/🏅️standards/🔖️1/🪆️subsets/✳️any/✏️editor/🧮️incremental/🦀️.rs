//! 🧮️ Iso16757 editor evaluation: the family delegates its check units to the shared `Iso16757Units`; the retained `SharedEvaluation` lives in `inferences::live`, read by editor and viewer alike, so a paint after an edit re-runs only the sections whose read set changed.

use crate::Iso16757Family;
use semio_s_artifact_norm_iso16757_inferences::Iso16757Units;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, EntityEdge, UnitSource};

impl UnitSource for Iso16757Family {
    type Document = Iso16757Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = <Iso16757Units as UnitSource>::OWNERS;

    fn units(document: &Iso16757Snapshot) -> Vec<CheckUnit<Iso16757Snapshot>> {
        Iso16757Units::units(document)
    }

    fn references(document: &Iso16757Snapshot) -> Vec<EntityEdge> {
        Iso16757Units::references(document)
    }

    fn summarise(document: &Iso16757Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<semio_s_plugin_norm_compliance::CheckResult> {
        Iso16757Units::summarise(document, report)
    }
}

impl semio_s_plugin_norm_compliance::IncrementalNormFamily for Iso16757Family {}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
