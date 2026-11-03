//! 🛰️ Live scene: the retained [`SceneProjector`] of one open artifact. A drag preview or an edit re-projects only the entities whose fingerprint, evaluation or dependencies changed; every other fragment is served from the cache, and the delta against the previous scene is what the viewport applies.

use super::*;
use crate::projection_kit::{station_values, status as scene_status};
use norm::{CheckReport, CheckStatus as NormStatus};

/// 🚦️ The per-entity [`Evaluation`] of a compliance report: each check lands on its subject entity, checks without one on the global slot; informative rows are left out.
pub fn evaluation_of(report: &CheckReport, report_seq: u64) -> Evaluation {
    Evaluation { reports: ReportIndex::of(report), ..rollup(report, report_seq) }
}

fn rollup(report: &CheckReport, report_seq: u64) -> Evaluation {
    let mut evaluation = Evaluation { report_seq, ..Evaluation::default() };
    for check in report.checks.iter().filter(|check| check.status != NormStatus::Info) {
        let slot = match check.subject.entity_id.as_str() {
            "" => &mut evaluation.global,
            id => evaluation.entities.entry(id.to_owned()).or_default(),
        };
        slot.checks.push(CheckSample { check_id: check.id.clone(), utilisation: check.utilization.max(0.0), status: scene_status(check.status) });
        let stations = station_values(check);
        if !stations.is_empty() && slot.stations.as_ref().is_none_or(|known| known.len() < stations.len()) {
            slot.stations = Some(stations);
        }
    }
    evaluation
}

/// 🛰️ Retained projector plus the last evaluation; `report_seq` advances only when the evaluation really changed, so an unchanged report keeps every fragment key.
pub struct LiveScene<P: StructuralProjection> {
    projector: SceneProjector<P>,
    evaluation: Evaluation,
    index: ReportIndex,
}

impl<P: StructuralProjection> LiveScene<P> {
    pub fn new(projection: P) -> Self {
        Self { projector: SceneProjector::new(projection), evaluation: Evaluation::default(), index: ReportIndex::default() }
    }

    pub fn projector(&self) -> &SceneProjector<P> {
        &self.projector
    }

    fn follow(&mut self, report: &CheckReport) {
        let seen = self.index.version();
        self.index.follow(report);
        if self.index.version() == seen {
            return;
        }
        let next = Evaluation { reports: self.index.clone(), ..rollup(report, self.evaluation.report_seq) };
        if next != self.evaluation {
            self.evaluation = Evaluation { report_seq: self.evaluation.report_seq + 1, ..next };
        }
    }

    /// 🎬️ The full scene of `doc` against `report`; unchanged entities come from the fragment cache. The scene is not kept as the base of the next [`Self::preview`] delta.
    pub fn scene(&mut self, doc: &P::Document<'_>, report: &CheckReport, ctx: &ProjectionContext) -> Result<StructuralScene, Vec<Violation>> {
        self.follow(report);
        self.projector.project_untracked(doc, &self.evaluation, ctx)
    }

    /// 🩹️ The scene and the delta against the previous one; `changed` names the entities whose document subtree changed (empty when only the evaluation moved).
    pub fn preview(&mut self, doc: &P::Document<'_>, changed: &ChangedSet, report: &CheckReport, ctx: &ProjectionContext) -> Result<(StructuralScene, SceneDelta), Vec<Violation>> {
        self.follow(report);
        self.projector.project_delta(doc, changed, &self.evaluation, ctx)
    }
}

/// 🛰️ Declares `mod $module` with the retained [`LiveScene`] of one artifact projection (`$projection: StructuralProjection + Default + Send`): `scene` and `preview` serve editor and viewer from the same cache.
#[macro_export]
macro_rules! live_scene {
    ($vis:vis mod $module:ident = $projection:ty) => {
        $vis mod $module {
            use super::*;

            fn slot() -> &'static ::std::sync::Mutex<$crate::projection::LiveScene<$projection>> {
                static SLOT: ::std::sync::OnceLock<::std::sync::Mutex<$crate::projection::LiveScene<$projection>>> = ::std::sync::OnceLock::new();
                SLOT.get_or_init(|| ::std::sync::Mutex::new($crate::projection::LiveScene::new(<$projection as ::std::default::Default>::default())))
            }

            pub fn scene(document: &<$projection as $crate::projection::StructuralProjection>::Document<'_>, report: &$crate::norm::CheckReport, ctx: &$crate::projection::ProjectionContext) -> ::std::result::Result<$crate::StructuralScene, ::std::vec::Vec<$crate::Violation>> {
                slot().lock().unwrap_or_else(::std::sync::PoisonError::into_inner).scene(document, report, ctx)
            }

            pub fn preview(document: &<$projection as $crate::projection::StructuralProjection>::Document<'_>, changed: &$crate::projection::ChangedSet, report: &$crate::norm::CheckReport, ctx: &$crate::projection::ProjectionContext) -> ::std::result::Result<($crate::StructuralScene, $crate::SceneDelta), ::std::vec::Vec<$crate::Violation>> {
                slot().lock().unwrap_or_else(::std::sync::PoisonError::into_inner).preview(document, changed, report, ctx)
            }
        }
    };
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
