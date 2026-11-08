//! 🧮️ DIN V 18599 check units in exactly the order of `evaluate_scoped`: the integrity part, the volume, climate and area part, one linkage unit per envelope element, the envelope part, one lighting unit per zone, the systems part, one cooling-setpoint unit per zone, the cooling-demand part and the energy part.

use semio_s_artifact_norm_din18599_checks::evaluate_scoped;
use semio_s_artifact_norm_din18599_model::Din18599Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, CostTier, Rows, Scope, UnitSource};
use semio_s_plugin_norm_compliance::{CheckResult, Owners};

/// 🧮️ The unit source of the DIN V 18599 building subject: `SharedEvaluation<Din18599Units>` is the one evaluation path of editor and viewer.
pub struct Din18599Units;

/// 🌐️ The checks that concern the whole building and have no owning zone or element.
const DOCUMENT_LEVEL: &[&str] = &[
    "din18599.1.heated-volume",
    "din18599.1.climate-composition",
    "din18599.1.net-floor-area",
    "din18599.geg.ht-prime",
    "din18599.geg.mean-u.",
    "din18599.geg.qp",
    "din18599.2.heating-demand",
    "din18599.5.heating-efficiency",
    "din18599.6.heat-recovery",
    "din18599.7.cooling",
    "din18599.8.dhw",
    "din18599.11.automation",
    "din18599.12.tabular",
];

fn run(scope: Scope<'static>) -> impl Fn(&Din18599Snapshot) -> Vec<CheckResult> + Send + Sync + 'static {
    move |document| evaluate_scoped(document, scope).checks
}

fn rows<T>(mut rows: Rows<Din18599Snapshot>, items: &[T], tag: &'static str, collection: &'static str, id: impl Fn(&T) -> &str, extra: &[&str]) -> Rows<Din18599Snapshot> {
    for (index, item) in items.iter().enumerate() {
        rows = rows.row(tag, collection, index, id(item), CostTier::T0, extra, move |document: &Din18599Snapshot| evaluate_scoped(document, Scope::Row(tag, index)).checks);
    }
    rows
}

impl UnitSource for Din18599Units {
    type Document = Din18599Snapshot;

    const OWNERS: Owners = Owners::new(&["zones", "elements"], DOCUMENT_LEVEL);

    fn units(document: &Din18599Snapshot) -> Vec<CheckUnit<Din18599Snapshot>> {
        let r = Rows::except(document, &["zones", "elements"]).part_reading("integrity", &["zones.*.id", "elements.*.id"], run(Scope::Part("integrity")));
        let r = r.part("volume-climate-area", run(Scope::Part("volume-climate-area")));
        let r = rows(r, &document.elements, "linkage", "elements", |e| e.id.as_str(), &["zones.*.id"]);
        let r = r.part("envelope", run(Scope::Part("envelope")));
        let r = rows(r, &document.zones, "lighting", "zones", |z| z.id.as_str(), &[]);
        let r = r.part("systems", run(Scope::Part("systems")));
        let r = rows(r, &document.zones, "cooling-need", "zones", |z| z.id.as_str(), &["zones", "elements"]);
        r.part("cooling", run(Scope::Part("cooling"))).part("energy", run(Scope::Part("energy"))).finish()
    }
}
