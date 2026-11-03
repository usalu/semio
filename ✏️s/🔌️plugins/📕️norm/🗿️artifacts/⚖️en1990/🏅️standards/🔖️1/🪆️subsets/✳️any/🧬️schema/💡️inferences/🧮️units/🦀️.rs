//! 🧮️ EN 1990 check units in exactly the order of `evaluate`: one global unit (validation, integrity and the document-level verdicts), then one unit per member row; a member reads every other member only through its id.

use super::{evaluate_gated, evaluate_scoped, RowGate, Scope};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, CostTier, Gate, Rows, UnitSource};
use semio_s_plugin_norm_compliance::CheckResult;

fn row(gate: &Gate<RowGate>, tag: &'static str, index: usize) -> impl Fn(&En1990Snapshot) -> Vec<CheckResult> + Send + Sync + 'static {
    let gate = gate.clone();
    move |d| evaluate_gated(d, Scope::Row(tag, index), &gate).checks
}

/// 📖️ What the global unit reads of the row collections besides the shared fields: only their id columns (duplicate ids and the member wiring), so a drag of a member geometry or a case value leaves it clean.
const GLOBAL_ROW_READS: [&str; 5] = ["members.*.id", "reliabilityAnalyses.*.id", "testSeries.*.id", "cases.*.id", "placements.*.id"];

/// 🧮️ The unit source of the EN 1990 basis-of-design subject: `SharedEvaluation<En1990Units>` is the one evaluation path of editor and viewer.
pub struct En1990Units;

impl UnitSource for En1990Units {
    type Document = En1990Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = semio_s_plugin_norm_compliance::Owners::new(&["members", "permanents", "variables", "accidentals", "seismics", "bridgeSls", "fatigueSpectra", "reliabilityAnalyses", "testSeries", "factorOverrides", "trafficGroups", "cases"], &["en1990.table-2.1.", "en1990.a2.working-life", "en1990.a2.traffic-groups", "en1990.members", "en1990.annex-b.", "en1990.annex-c.beta", "en1990.input.", "en1990.integrity."]);

    fn units(d: &En1990Snapshot) -> Vec<CheckUnit<En1990Snapshot>> {
        let gate = Gate::new();
        let mut rows = Rows::except(d, &["members", "reliabilityAnalyses", "testSeries", "cases", "placements"]).global_sharing(&GLOBAL_ROW_READS, |d: &En1990Snapshot| evaluate_scoped(d, Scope::Globals).checks);
        for (index, member) in d.members.iter().enumerate() {
            rows = rows.row("members", "members", index, &member.id, CostTier::T1, &["members.*.id", "fatigueSpectra.*", "factorOverrides.*", "trafficGroups.*"], row(&gate, "members", index));
        }
        for (index, analysis) in d.reliability_analyses.iter().enumerate() {
            rows = rows.row("reliabilityAnalyses", "reliabilityAnalyses", index, &analysis.id, CostTier::T1, &[], row(&gate, "reliabilityAnalyses", index));
        }
        for (index, series) in d.test_series.iter().enumerate() {
            rows = rows.row("testSeries", "testSeries", index, &series.id, CostTier::T0, &[], row(&gate, "testSeries", index));
        }
        for (index, case) in d.cases.iter().enumerate() {
            rows = rows.row("cases", "cases", index, &case.id, CostTier::T0, &[], row(&gate, "cases", index));
        }
        rows.finish()
    }
}
