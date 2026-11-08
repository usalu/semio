//! 🧮️ Vdi3805 editor evaluation: the family delegates its check units to the shared `Vdi3805Units`; the retained `SharedEvaluation` lives in `inferences::live`, read by editor and viewer alike, so a paint after an edit re-runs only the units whose read set changed.

use crate::Vdi3805Family;
use semio_s_artifact_norm_vdi3805_inferences::Vdi3805Units;
use semio_s_artifact_norm_vdi3805_model::Vdi3805Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, EntityEdge, UnitSource};

impl UnitSource for Vdi3805Family {
    type Document = Vdi3805Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = <Vdi3805Units as UnitSource>::OWNERS;

    fn units(document: &Vdi3805Snapshot) -> Vec<CheckUnit<Vdi3805Snapshot>> {
        Vdi3805Units::units(document)
    }

    fn references(document: &Vdi3805Snapshot) -> Vec<EntityEdge> {
        Vdi3805Units::references(document)
    }

    fn summarise(document: &Vdi3805Snapshot, report: &semio_s_plugin_norm_compliance::CheckReport) -> Vec<semio_s_plugin_norm_compliance::CheckResult> {
        Vdi3805Units::summarise(document, report)
    }
}

impl semio_s_plugin_norm_compliance::IncrementalNormFamily for Vdi3805Family {}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧭️ownership/🦀️.rs"]
mod ownership;
