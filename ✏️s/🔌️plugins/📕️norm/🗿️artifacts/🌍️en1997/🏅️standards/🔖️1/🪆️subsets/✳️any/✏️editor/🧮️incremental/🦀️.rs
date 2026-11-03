//! 🧮️ En1997 editor evaluation: the family delegates its check units to the shared `En1997Units`; the retained `SharedEvaluation` lives in `inferences::live`, read by editor and viewer alike, so a paint after an edit re-runs only the units whose read set changed.

use crate::En1997Family;
use semio_s_artifact_norm_en1997_checks::En1997Units;
use semio_s_artifact_norm_en1997_model::En1997Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, EntityEdge, UnitSource};

impl UnitSource for En1997Family {
    type Document = En1997Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = <En1997Units as UnitSource>::OWNERS;

    fn units(document: &En1997Snapshot) -> Vec<CheckUnit<En1997Snapshot>> {
        En1997Units::units(document)
    }

    fn references(document: &En1997Snapshot) -> Vec<EntityEdge> {
        En1997Units::references(document)
    }

    fn summarise(document: &En1997Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<semio_s_plugin_norm_compliance::CheckResult> {
        En1997Units::summarise(document, report)
    }
}

impl semio_s_plugin_norm_compliance::IncrementalNormFamily for En1997Family {}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧭️ownership/🦀️.rs"]
mod ownership;
