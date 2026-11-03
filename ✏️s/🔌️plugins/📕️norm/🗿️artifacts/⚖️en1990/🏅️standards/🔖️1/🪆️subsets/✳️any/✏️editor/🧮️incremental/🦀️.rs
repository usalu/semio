//! 🧮️ En1990 editor evaluation: the family delegates its check units to the shared `En1990Units`; the retained `SharedEvaluation` lives in `inferences::live`, read by editor and viewer alike, so a paint after an edit re-runs only the units whose read set changed.

use crate::En1990Family;
use semio_s_artifact_norm_en1990_inferences::En1990Units;
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, EntityEdge, UnitSource};

impl UnitSource for En1990Family {
    type Document = En1990Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = <En1990Units as UnitSource>::OWNERS;

    fn units(document: &En1990Snapshot) -> Vec<CheckUnit<En1990Snapshot>> {
        En1990Units::units(document)
    }

    fn references(document: &En1990Snapshot) -> Vec<EntityEdge> {
        En1990Units::references(document)
    }

    fn summarise(document: &En1990Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<semio_s_plugin_norm_compliance::CheckResult> {
        En1990Units::summarise(document, report)
    }
}

impl semio_s_plugin_norm_compliance::IncrementalNormFamily for En1990Family {}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧭️ownership/🦀️.rs"]
mod ownership;
