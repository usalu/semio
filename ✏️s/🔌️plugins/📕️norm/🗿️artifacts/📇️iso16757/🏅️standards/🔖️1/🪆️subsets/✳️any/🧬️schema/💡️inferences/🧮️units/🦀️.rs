//! 🧮️ ISO 16757 check units in exactly the order of [`semio_s_artifact_norm_iso16757_checks::evaluate_all`]: one unit per evaluation section of Parts 1, 2, 4 and 5, each reading only the document facets its section declares, so an edit re-runs the sections that read the edited facet and leaves the others settled.

use semio_s_artifact_norm_iso16757_checks::{sections, Section};
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use semio_s_plugin_norm_compliance::incremental_evaluation::{CheckUnit, Rows, UnitSource};
use semio_s_plugin_norm_compliance::{CheckReport, CheckResult, Owners};

/// 🧮️ The unit source of the ISO 16757 catalogue: `SharedEvaluation<Iso16757Units>` is the one evaluation path of editor and viewer.
pub struct Iso16757Units;

fn run(section: Section) -> impl Fn(&Iso16757Snapshot) -> Vec<CheckResult> + Send + Sync + 'static {
    move |document| {
        let mut report = CheckReport::default();
        (section.run)(document, &mut report);
        report.checks
    }
}

impl UnitSource for Iso16757Units {
    type Document = Iso16757Snapshot;

    const OWNERS: Owners = Owners::AS_EMITTED;

    fn units(_document: &Self::Document) -> Vec<CheckUnit<Self::Document>> {
        let mut all = sections().into_iter();
        let first = all.next().expect("the evaluation has sections");
        all.fold(Rows::new(Vec::<String>::new()).global_reading(first.reads, run(first)), |rows, section| rows.part_reading(section.id, section.reads, run(section))).finish()
    }
}
