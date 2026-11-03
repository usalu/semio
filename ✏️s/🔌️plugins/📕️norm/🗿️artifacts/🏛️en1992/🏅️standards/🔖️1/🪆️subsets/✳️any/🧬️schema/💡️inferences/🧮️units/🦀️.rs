//! 🧮️ EN 1992 check units: one global unit (integrity) and one unit per member, anchor and design case, in exactly the order of `evaluate`; members with a fire design run as slow (T2) units on the worker.

use super::{evaluate_scoped, Scope};
use semio_s_artifact_norm_en1992_model::En1992Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, CostTier, Rows, UnitSource};

/// 🧮️ The unit source of the EN 1992 concrete subject: `SharedEvaluation<En1992Units>` is the one evaluation path of editor and viewer.
pub struct En1992Units;

/// 📖️ What the global unit reads: the annex, the id column of every collection it checks for duplicates and the material collections of the catalogue verdicts; a member or anchor edit leaves it clean.
const GLOBAL_READS: [&str; 8] = ["annex", "members.*.id", "anchors.*.id", "placements.*.id", "cases.*.id", "concreteGrades", "reinforcementGrades", "prestressSteels"];

const SHARED_READS: [&str; 9] = ["annex", "reliabilityClass", "title", "designWorkingLifeYears", "deltaCDev", "cementType", "concreteGrades", "reinforcementGrades", "prestressSteels"];

impl UnitSource for En1992Units {
    type Document = En1992Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = semio_s_plugin_norm_compliance::Owners::new(&["members", "anchors", "cases"], &["en1992.annex.", "en1992.integrity."]);

    fn units(d: &En1992Snapshot) -> Vec<CheckUnit<En1992Snapshot>> {
        let mut rows = Rows::new(SHARED_READS).global_reading(&GLOBAL_READS, |d: &En1992Snapshot| evaluate_scoped(d, Scope::Globals).checks);
        for (index, member) in d.members.iter().enumerate() {
            let tier = if member.fire.is_some() { CostTier::T2 } else { CostTier::T1 };
            rows = rows.slow("members", "members", index, &member.id, tier, &[], move |d: &En1992Snapshot, progress, token| {
                progress.report(0.0);
                if token.is_cancelled() {
                    return Vec::new();
                }
                let checks = evaluate_scoped(d, Scope::Row("members", index)).checks;
                progress.report(1.0);
                checks
            });
        }
        for (index, anchor) in d.anchors.iter().enumerate() {
            rows = rows.row("anchors", "anchors", index, &anchor.id, CostTier::T0, &[], move |d: &En1992Snapshot| evaluate_scoped(d, Scope::Row("anchors", index)).checks);
        }
        for (index, case) in d.cases.iter().enumerate() {
            rows = rows.row("cases", "cases", index, &case.id, CostTier::T0, &[], move |d: &En1992Snapshot| evaluate_scoped(d, Scope::Row("cases", index)).checks);
        }
        rows.finish()
    }
}
