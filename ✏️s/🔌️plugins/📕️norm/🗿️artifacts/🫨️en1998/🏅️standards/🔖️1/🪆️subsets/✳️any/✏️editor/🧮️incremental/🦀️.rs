//! 🧮️ En1998 editor evaluation: the family delegates its check units to the shared `En1998Units`; the retained `SharedEvaluation` lives in `inferences::live`, read by editor and viewer alike, so a paint after an edit re-runs only the units whose read set changed.

use crate::En1998Family;
use semio_s_artifact_norm_en1998_inferences::En1998Units;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, EntityEdge, UnitSource};

impl UnitSource for En1998Family {
    type Document = En1998Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = <En1998Units as UnitSource>::OWNERS;

    fn units(document: &En1998Snapshot) -> Vec<CheckUnit<En1998Snapshot>> {
        En1998Units::units(document)
    }

    fn references(document: &En1998Snapshot) -> Vec<EntityEdge> {
        En1998Units::references(document)
    }

    fn summarise(document: &En1998Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<semio_s_plugin_norm_compliance::CheckResult> {
        En1998Units::summarise(document, report)
    }
}

impl semio_s_plugin_norm_compliance::IncrementalNormFamily for En1998Family {}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧭️ownership/🦀️.rs"]
mod ownership;
