//! 🧮️ En1994 editor evaluation: the family delegates its check units to the shared `En1994Units`; the retained `SharedEvaluation` lives in `inferences::live`, read by editor and viewer alike, so a paint after an edit re-runs only the units whose read set changed.

use crate::En1994Family;
use semio_s_artifact_norm_en1994_inferences::En1994Units;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, EntityEdge, UnitSource};

impl UnitSource for En1994Family {
    type Document = En1994Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = <En1994Units as UnitSource>::OWNERS;

    fn units(document: &En1994Snapshot) -> Vec<CheckUnit<En1994Snapshot>> {
        En1994Units::units(document)
    }

    fn references(document: &En1994Snapshot) -> Vec<EntityEdge> {
        En1994Units::references(document)
    }

    fn summarise(document: &En1994Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<semio_s_plugin_norm_compliance::CheckResult> {
        En1994Units::summarise(document, report)
    }
}

impl semio_s_plugin_norm_compliance::IncrementalNormFamily for En1994Family {}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧭️ownership/🦀️.rs"]
mod ownership;
