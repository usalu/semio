//! 🧮️ En1999 editor evaluation: the family delegates its check units to the shared `En1999Units`; the retained `SharedEvaluation` lives in `inferences::live`, read by editor and viewer alike, so a paint after an edit re-runs only the units whose read set changed.

use crate::En1999Family;
use semio_s_artifact_norm_en1999_checks::En1999Units;
use semio_s_artifact_norm_en1999_model::En1999Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, EntityEdge, UnitSource};

impl UnitSource for En1999Family {
    type Document = En1999Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = <En1999Units as UnitSource>::OWNERS;

    fn units(document: &En1999Snapshot) -> Vec<CheckUnit<En1999Snapshot>> {
        En1999Units::units(document)
    }

    fn references(document: &En1999Snapshot) -> Vec<EntityEdge> {
        En1999Units::references(document)
    }

    fn summarise(document: &En1999Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<semio_s_plugin_norm_compliance::CheckResult> {
        En1999Units::summarise(document, report)
    }
}

impl semio_s_plugin_norm_compliance::IncrementalNormFamily for En1999Family {}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧭️ownership/🦀️.rs"]
mod ownership;
