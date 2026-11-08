//! 🧮️ DIN EN 16798 check units in exactly the order of [`check_full_environment`]: the unique-id integrity, the empty-zones verdict, one unit per zone, one unit per ventilation system and the building envelope.

use super::*;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, CostTier, Rows, UnitSource};

/// 🧮️ The unit source of the DIN EN 16798 environment subject: `SharedEvaluation<Din16798Units>` is the one evaluation path of editor and viewer.
pub struct Din16798Units;

fn run(scope: Scope<'static>) -> impl Fn(&semio_s_artifact_norm_din16798_model::Din16798Snapshot) -> Vec<CheckResult> + Send + Sync + 'static {
    move |document| check_scoped(document, scope).checks
}

/// 📖️ The document leaves the envelope verdicts read: the envelope scalars, the night set-back and the usage of the first zone.
const ENVELOPE_READS: [&str; 7] = ["annex", "envelopeN50HInv", "envelopeVolumeM3", "cellarAreaM2", "cellarVentilationM3H", "nightSetbackK", "zones.0.usageType"];

impl UnitSource for Din16798Units {
    type Document = semio_s_artifact_norm_din16798_model::Din16798Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = semio_s_plugin_norm_compliance::Owners::new(&["zones", "ventSystems"], &["din16798.zones.empty", "din16798-7.", "din16798-5-1."]);

    fn units(document: &Self::Document) -> Vec<CheckUnit<Self::Document>> {
        let mut rows = Rows::except(document, &["zones", "ventSystems"]).global_reading(&["annex", "zones.*.id", "ventSystems.*.id"], run(Scope::Part("integrity")));
        rows = rows.part_reading("zones-none", &["annex", "zones.*.id"], run(Scope::Part("zones-none")));
        for (index, zone) in document.zones.iter().enumerate() {
            rows = rows.row("zones", "zones", index, &zone.id, CostTier::T1, &["ventSystems"], move |d| check_scoped(d, Scope::Row("zones", index)).checks);
        }
        for (index, vent) in document.vent_systems.iter().enumerate() {
            rows = rows.row("ventSystems", "ventSystems", index, &vent.id, CostTier::T0, &["zones"], move |d| check_scoped(d, Scope::Row("ventSystems", index)).checks);
        }
        rows.part_reading("envelope", &ENVELOPE_READS, run(Scope::Part("envelope"))).finish()
    }
}
