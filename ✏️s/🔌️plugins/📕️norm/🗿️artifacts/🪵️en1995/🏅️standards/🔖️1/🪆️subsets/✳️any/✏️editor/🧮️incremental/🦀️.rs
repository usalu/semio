//! 🧮️ En1995 editor evaluation: the family delegates its check units to the shared `En1995Units`; the retained `SharedEvaluation` lives in `inferences::live`, read by editor and viewer alike, so a paint after an edit re-runs only the units whose read set changed.

use crate::En1995Family;
use semio_s_artifact_norm_en1995_schema::checks::timber::En1995Units;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, EntityEdge, UnitSource};

impl UnitSource for En1995Family {
    type Document = En1995Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = <En1995Units as UnitSource>::OWNERS;

    fn units(document: &En1995Snapshot) -> Vec<CheckUnit<En1995Snapshot>> {
        En1995Units::units(document)
    }

    fn references(document: &En1995Snapshot) -> Vec<EntityEdge> {
        En1995Units::references(document)
    }

    fn summarise(document: &En1995Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<semio_s_plugin_norm_compliance::CheckResult> {
        En1995Units::summarise(document, report)
    }
}

impl semio_s_plugin_norm_compliance::IncrementalNormFamily for En1995Family {}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧭️ownership/🦀️.rs"]
mod ownership;
