//! 🎹️ View steps: the stepping shortcuts (next and previous check, governing check, combination) as events of the ephemeral local view lane. A key becomes `viewportAct`, the action becomes a step intent, and the intent resolves against the settled report and the scene into `ViewCheck` / `ViewTrack` / `ViewSeek` / `ViewItem` events; the document is never touched.
//!
//! 🔗️ Keys: [`shortcuts`](crate::accessibility::focus_order::shortcuts); walking order: [`step_index`](crate::accessibility::focus_order::step_index), [`step_governing`](crate::accessibility::focus_order::step_governing); see `visual-editor-architecture.md` §8.2.

use crate::*;
use crate::accessibility::focus_order::{shortcut_action_id, step_governing, step_index, ShortcutAction};
use semio_s_plugin_norm_structural_scene::{StructuralScene, TrackKind};

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

fn list_order(report: &CheckReport) -> Vec<usize> {
    plan_checks(report, &ChecksViewState::default(), Locale::En).into_iter().flat_map(|group| group.rows).collect()
}

/// 🔢️ The events one step intent makes: `Check` walks the checks in the order of the default checks list (worst entity first), `Governing` from the highest utilisation down (not applicable last), both wrapping and starting from the check the view selected; `Combination` selects the next keyframe of the scene's combination track (track, value and index), nothing when the scene has none. The scene is only projected for a combination step.
pub fn resolve_view_step(step: ViewStep, forward: bool, report: &CheckReport, scene: impl FnOnce() -> StructuralScene, view: &ViewLane) -> Vec<LaneEvent> {
    let current = view.check.map(|index| index as usize);
    let check = |index: Option<usize>| index.map(|index| vec![LaneEvent::ViewCheck(Some(index as u32))]).unwrap_or_default();
    match step {
        ViewStep::Check => {
            let order = list_order(report);
            let at = current.and_then(|check| order.iter().position(|index| *index == check));
            check(step_index(order.len(), at, forward).map(|position| order[position]))
        }
        ViewStep::Governing => {
            let utilisations: Vec<Option<f64>> = report.checks.iter().map(|check| (check.status != CheckStatus::NotApplicable).then_some(check.utilization)).collect();
            check(step_governing(&utilisations, current, forward))
        }
        ViewStep::Combination => {
            let scene = scene();
            let Some(track) = scene.tracks.iter().find(|track| matches!(track.kind, TrackKind::Combination)) else { return Vec::new() };
            step_index(track.keyframes.len(), view.item.map(|item| item as usize), forward)
                .map(|item| vec![LaneEvent::ViewTrack(Some(track.id.clone())), LaneEvent::ViewSeek(track.keyframes[item].at), LaneEvent::ViewItem(Some(item as u32))])
                .unwrap_or_default()
        }
    }
}

/// 🎹️ Replaces every step intent of `events` by its resolved events, keeps all other events.
pub fn resolve_view_steps(events: Vec<LaneEvent>, report: &CheckReport, scene: impl Fn() -> StructuralScene, view: &ViewLane) -> Vec<LaneEvent> {
    events
        .into_iter()
        .flat_map(|event| match event {
            LaneEvent::ViewStep { step, forward } => resolve_view_step(step, forward, report, &scene, view),
            other => vec![other],
        })
        .collect()
}

/// 👁️ The read-only step of a viewer: the next [`ViewLane`] after one stepping intent, folded from the resolved events exactly like the editor transient does. It reads the report and the scene only and returns a view lane, so a viewer that keeps this lane in its host view state steps checks and combinations without any document, config or draft mutation.
pub fn step_view(view: &ViewLane, step: ViewStep, forward: bool, report: &CheckReport, scene: impl FnOnce() -> StructuralScene) -> ViewLane {
    resolve_view_step(step, forward, report, scene, view).into_iter().fold(NormTransient::default().with_view(view.clone()), NormTransient::apply).view
}
