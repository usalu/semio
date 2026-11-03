//! 🧮️ EN 1994 check units in exactly the order of `evaluate`: the empty verdict, beams, columns, slabs, the fire verdict and fire rows (beams, slabs, columns), the bridge verdict, bridge rows and the design-case rows.

use super::evaluate_scoped;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, CostTier, Rows, Scope, UnitSource};

/// 🧮️ The unit source of the EN 1994 composite subject: `SharedEvaluation<En1994Units>` is the one evaluation path of editor and viewer.
pub struct En1994Units;

fn run(scope: Scope<'static>) -> impl Fn(&En1994Snapshot) -> Vec<semio_s_plugin_norm_compliance::CheckResult> + Send + Sync + 'static {
    move |d| evaluate_scoped(d, scope).checks
}

const STUD_READS: &[&str] = &["studs"];

/// 📖️ Verdict units read only the annex, their one scalar and which collections are populated (the ids), not `*`: a load drag or a leaf edit of a row leaves them clean.
const VERDICT_READS: [&str; 5] = ["annex", "reliabilityClass", "beams.*.id", "columns.*.id", "slabs.*.id"];

fn verdict_reads(scalar: &str) -> Vec<&str> {
    VERDICT_READS.iter().copied().chain([scalar]).collect()
}

fn rows<T>(mut rows: Rows<En1994Snapshot>, items: &[T], tag: &'static str, collection: &'static str, id: impl Fn(&T) -> &str, tier: CostTier, extra: &[&str]) -> Rows<En1994Snapshot> {
    for (index, item) in items.iter().enumerate() {
        rows = rows.row(tag, collection, index, id(item), tier, extra, move |d: &En1994Snapshot| evaluate_scoped(d, Scope::Row(tag, index)).checks);
    }
    rows
}

impl UnitSource for En1994Units {
    type Document = En1994Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = semio_s_plugin_norm_compliance::Owners::new(&["beams", "columns", "slabs", "cases"], &["en1994.subject.empty", "en1994.integrity."]);

    fn units(d: &En1994Snapshot) -> Vec<CheckUnit<En1994Snapshot>> {
        let r = Rows::except(d, &["beams", "columns", "slabs", "placements", "studs", "cases"]).global_reading(&VERDICT_READS, run(Scope::Part("empty")));
        let r = rows(r, &d.beams, "beams", "beams", |e| e.id.as_str(), CostTier::T1, STUD_READS);
        let r = rows(r, &d.columns, "columns", "columns", |e| e.id.as_str(), CostTier::T1, &[]);
        let r = rows(r, &d.slabs, "slabs", "slabs", |e| e.id.as_str(), CostTier::T1, &[]);
        let r = r.part_reading("fire-rating", &verdict_reads("fireRating"), run(Scope::Part("fire-rating")));
        let r = rows(r, &d.beams, "fireBeams", "beams", |e| e.id.as_str(), CostTier::T2, &[]);
        let r = rows(r, &d.slabs, "fireSlabs", "slabs", |e| e.id.as_str(), CostTier::T0, &[]);
        let r = rows(r, &d.columns, "fireColumns", "columns", |e| e.id.as_str(), CostTier::T2, &[]);
        let r = r.part_reading("bridge-kind", &verdict_reads("structureKind"), run(Scope::Part("bridge-kind")));
        let r = rows(r, &d.beams, "bridgeBeams", "beams", |e| e.id.as_str(), CostTier::T0, STUD_READS);
        rows(r, &d.cases, "cases", "cases", |e| e.id.as_str(), CostTier::T0, &[]).finish()
    }
}
