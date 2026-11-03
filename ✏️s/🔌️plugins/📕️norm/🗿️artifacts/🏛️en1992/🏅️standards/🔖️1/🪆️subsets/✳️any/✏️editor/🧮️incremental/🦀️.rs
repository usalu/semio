//! 🧮️ En1992 editor evaluation: the family delegates its check units to the shared `En1992Units`; the retained `SharedEvaluation` lives in `inferences::live`, read by editor and viewer alike, so a paint after an edit re-runs only the units whose read set changed.

use crate::En1992Family;
use semio_s_artifact_norm_en1992_inferences::En1992Units;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, EntityEdge, UnitSource};

impl UnitSource for En1992Family {
    type Document = En1992Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = <En1992Units as UnitSource>::OWNERS;

    fn units(document: &En1992Snapshot) -> Vec<CheckUnit<En1992Snapshot>> {
        En1992Units::units(document)
    }

    fn references(document: &En1992Snapshot) -> Vec<EntityEdge> {
        En1992Units::references(document)
    }

    fn summarise(document: &En1992Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<semio_s_plugin_norm_compliance::CheckResult> {
        En1992Units::summarise(document, report)
    }
}

impl semio_s_plugin_norm_compliance::IncrementalNormFamily for En1992Family {}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧭️ownership/🦀️.rs"]
mod ownership;
