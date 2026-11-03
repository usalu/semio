//! 🧮️ EN 1993 editor evaluation: the family delegates its check units to the shared `En1993Units`; the retained `SharedEvaluation` lives in `inferences::live_report`, read by editor and viewer alike, so a paint after an edit re-runs only the units whose read set changed.

use crate::En1993Family;
use semio_s_artifact_norm_en1993_checks::En1993Units;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, EntityEdge, UnitSource};

impl UnitSource for En1993Family {
    type Document = En1993Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = <En1993Units as UnitSource>::OWNERS;

    fn units(document: &En1993Snapshot) -> Vec<CheckUnit<En1993Snapshot>> {
        En1993Units::units(document)
    }

    fn references(document: &En1993Snapshot) -> Vec<EntityEdge> {
        En1993Units::references(document)
    }
}

impl semio_s_plugin_norm_compliance::IncrementalNormFamily for En1993Family {}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧭️ownership/🦀️.rs"]
mod ownership;
