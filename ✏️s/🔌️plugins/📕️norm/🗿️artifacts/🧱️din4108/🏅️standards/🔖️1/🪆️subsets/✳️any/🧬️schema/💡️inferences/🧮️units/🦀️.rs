//! 🧮️ DIN 4108 check units in exactly the order of [`super::evaluate`]: reference integrity, one unit per zone, the empty-zones verdict, one unit per envelope element, the empty-elements verdict, airtightness, one unit per thermal bridge and the bridge equivalence.

use super::{evaluate_scoped, Din4108Snapshot};
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, CostTier, Rows, Scope, UnitSource};
use semio_s_plugin_norm_compliance::CheckResult;

/// 🧮️ The unit source of the DIN 4108 envelope subject: `SharedEvaluation<Din4108Units>` is the one evaluation path of editor and viewer.
pub struct Din4108Units;

fn run(scope: Scope<'static>) -> impl Fn(&Din4108Snapshot) -> Vec<CheckResult> + Send + Sync + 'static {
    move |document| evaluate_scoped(document, scope).checks
}

impl UnitSource for Din4108Units {
    type Document = Din4108Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = semio_s_plugin_norm_compliance::Owners::new(&["zones", "elements", "thermalBridges"], &["din4108-2.summer.none", "din4108-2.table3.none", "din4108-7.", "din4108-bb2.delta-u"]);

    fn units(document: &Din4108Snapshot) -> Vec<CheckUnit<Din4108Snapshot>> {
        let mut rows = Rows::except(document, &["zones", "elements", "thermalBridges"]).global_reading(&["zones", "elements", "thermalBridges"], run(Scope::Part("integrity")));
        for (index, zone) in document.zones.iter().enumerate() {
            rows = rows.row("zones", "zones", index, &zone.id, CostTier::T0, &["elements"], move |d: &Din4108Snapshot| evaluate_scoped(d, Scope::Row("zones", index)).checks);
        }
        rows = rows.part_reading("zones-none", &["zones"], run(Scope::Part("zones-none")));
        for (index, element) in document.elements.iter().enumerate() {
            rows = rows.row("elements", "elements", index, &element.id, CostTier::T1, &["elements", "thermalBridges"], move |d: &Din4108Snapshot| evaluate_scoped(d, Scope::Row("elements", index)).checks);
        }
        rows = rows.part_reading("elements-none", &["elements"], run(Scope::Part("elements-none")));
        rows = rows.part_reading("airtightness", &["airtightnessN50", "hasMechanicalVentilation"], run(Scope::Part("airtightness")));
        for (index, bridge) in document.thermal_bridges.iter().enumerate() {
            rows = rows.row("thermalBridges", "thermalBridges", index, &bridge.id, CostTier::T0, &[], move |d: &Din4108Snapshot| evaluate_scoped(d, Scope::Row("thermalBridges", index)).checks);
        }
        rows.part_reading("equivalence", &["thermalBridges", "elements", "bb2DetailsConform"], run(Scope::Part("equivalence"))).finish()
    }
}
