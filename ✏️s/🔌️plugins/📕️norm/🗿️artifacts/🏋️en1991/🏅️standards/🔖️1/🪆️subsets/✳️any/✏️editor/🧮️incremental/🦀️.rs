//! 🧮️ En1991 editor evaluation: the family delegates its check units to the shared `En1991Units`; the retained `SharedEvaluation` lives in `inferences::live`, read by editor and viewer alike, so a paint after an edit re-runs only the units whose read set changed.

use crate::En1991Family;
use semio_s_artifact_norm_en1991_inferences::En1991Units;
use semio_s_artifact_norm_en1991_model::En1991Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, EntityEdge, UnitSource};

impl UnitSource for En1991Family {
    type Document = En1991Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = <En1991Units as UnitSource>::OWNERS;

    fn units(document: &En1991Snapshot) -> Vec<CheckUnit<En1991Snapshot>> {
        En1991Units::units(document)
    }

    fn references(document: &En1991Snapshot) -> Vec<EntityEdge> {
        En1991Units::references(document)
    }

    fn summarise(document: &En1991Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<semio_s_plugin_norm_compliance::CheckResult> {
        En1991Units::summarise(document, report)
    }
}

impl semio_s_plugin_norm_compliance::IncrementalNormFamily for En1991Family {}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧭️ownership/🦀️.rs"]
mod ownership;
