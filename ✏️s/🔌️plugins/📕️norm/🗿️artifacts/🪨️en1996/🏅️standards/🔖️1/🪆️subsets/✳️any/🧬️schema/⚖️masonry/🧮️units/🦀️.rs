//! 🧮️ EN 1996 check units: one global unit (integrity and verdict) and one unit per wall and one per design case, in exactly the order of `evaluate_building`, `evaluate_design_cases` and `detailing_checks`.

use super::{detailing_checks, evaluate_design_cases, evaluate_scoped, Scope};
use semio_s_artifact_norm_en1996_model::En1996Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, CostTier, Rows, UnitSource};

/// 🧮️ The unit source of the EN 1996 masonry subject: `SharedEvaluation<En1996Units>` is the one evaluation path of editor and viewer.
pub struct En1996Units;

fn scoped(d: &En1996Snapshot, scope: Scope<'_>) -> Vec<semio_s_plugin_norm_compliance::CheckResult> {
    evaluate_scoped(d.annex, d.masonry_class, d.design_situation, d.reliability_class, d.storeys, &d.walls, scope).checks
}

impl UnitSource for En1996Units {
    type Document = En1996Snapshot;

    const OWNERS: semio_s_plugin_norm_compliance::Owners = semio_s_plugin_norm_compliance::Owners::new(&["walls", "cases"], &["en1996.subject.empty", "en1996.integrity.", "en1996.m-95.", "en1996.8.chase-wall.", "en1996.8.joint-wall."]);

    fn units(d: &En1996Snapshot) -> Vec<CheckUnit<En1996Snapshot>> {
        let mut rows = Rows::new(["annex", "masonryClass", "designSituation", "storeys"]).global_reading(&["annex", "walls"], |d: &En1996Snapshot| scoped(d, Scope::Globals));
        for (index, wall) in d.walls.iter().enumerate() {
            rows = rows.row("walls", "walls", index, &wall.id, CostTier::T1, &[], move |d: &En1996Snapshot| scoped(d, Scope::Row("walls", index)));
        }
        for (index, case) in d.cases.iter().enumerate() {
            rows = rows.row("cases", "cases", index, &case.id, CostTier::T0, &["cases.*.id"], move |d: &En1996Snapshot| evaluate_design_cases(d.annex, &d.cases, Scope::Row("cases", index)).checks);
        }
        rows.part_reading("detailing", &["annex", "walls", "chases", "movementJoints"], |d: &En1996Snapshot| detailing_checks(d.annex, &d.walls, &d.chases, &d.movement_joints).checks).finish()
    }
}
