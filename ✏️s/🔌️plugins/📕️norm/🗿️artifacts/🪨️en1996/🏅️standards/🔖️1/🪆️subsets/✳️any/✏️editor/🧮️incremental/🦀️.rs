//! 🧮️ En1996 editor evaluation: the family delegates its check units to the shared `En1996Units`; the retained `SharedEvaluation` lives in `inferences::live`, read by editor and viewer alike, so a paint after an edit re-runs only the units whose read set changed.

use crate::En1996Family;
use semio_s_artifact_norm_en1996_schema::checks::masonry::En1996Units;
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, EntityEdge, UnitSource};

impl UnitSource for En1996Family {
    type Document = En1996Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = <En1996Units as UnitSource>::OWNERS;

    fn units(document: &En1996Snapshot) -> Vec<CheckUnit<En1996Snapshot>> {
        En1996Units::units(document)
    }

    fn references(document: &En1996Snapshot) -> Vec<EntityEdge> {
        En1996Units::references(document)
    }

    fn summarise(document: &En1996Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<semio_s_plugin_norm_compliance::CheckResult> {
        En1996Units::summarise(document, report)
    }
}

impl semio_s_plugin_norm_compliance::IncrementalNormFamily for En1996Family {}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧭️ownership/🦀️.rs"]
mod ownership;
