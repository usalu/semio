//! 🧪️ Viewer surface tests: a read-only viewer steps checks and combinations through its ephemeral local view lane. The independent oracle is the hand-derived order of the example report (worst entity first) and `std` iteration; the lane is a plain struct, so the test needs no host.

use super::*;
use semio_s_plugin_norm_compliance::{AnnexChoice, LocalizedCopy, SubjectRef};
use semio_s_plugin_norm_structural_scene::projection_kit::tracks::indexed;
use semio_s_plugin_norm_structural_scene::{Aabb, LocalizedCopy as Text, SceneRevision, TrackKind, Vec3, SCENE_SCHEMA_ID};

struct Lane(NormTransient);

impl LaneTransient for Lane {
    type Mutation = NormTransient;
    fn lane(&self) -> NormTransient {
        self.0.clone()
    }
    fn replace(lane: &NormTransient) -> NormTransient {
        lane.clone()
    }
}

fn check(id: &str, entity: &str, utilization: f64, status: CheckStatus) -> CheckResult {
    CheckResult::assess(id, "ULS", ClauseId::new("EN 1993", "1-1", "6.2.9"), SubjectRef::new(entity, format!("members[{entity}].tM"), LocalizedCopy::new(format!("Member {entity}"), format!("Stab {entity}"))), LocalizedCopy::new(format!("check {id}"), format!("Nachweis {id}")))
        .utilization(Quantity::new(QuantityKind::Length, utilization * 0.0125), Quantity::new(QuantityKind::Length, 0.0125))
        .explanation(LocalizedCopy::new("explained", "erläutert"))
        .annex(AnnexChoice::De)
        .status(status)
        .build()
}

fn report() -> NormViewReport {
    let mut report = CheckReport::default();
    for (id, entity, utilization, status) in [("a", "B1", 0.64, CheckStatus::Pass), ("b", "B2", 1.12, CheckStatus::Fail), ("c", "B1", 0.95, CheckStatus::Warning), ("d", "B3", 0.0, CheckStatus::NotApplicable), ("e", "B2", 0.3, CheckStatus::Pass)] {
        report.push(check(id, entity, utilization, status));
    }
    std::sync::Arc::new(report)
}

fn scene(report: &NormViewReport, _lane: &ViewLane) -> StructuralScene {
    let stops = (1..=3).map(|n| (format!("Combination {n}"), format!("Kombination {n}"), None, None)).collect();
    assert_eq!(report.checks.len(), 5);
    StructuralScene {
        schema: SCENE_SCHEMA_ID.into(),
        revision: SceneRevision { document_seq: 1, report_seq: 1 },
        bounds: Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
        entities: Vec::new(),
        annotations: Vec::new(),
        legends: Vec::new(),
        layers: Vec::new(),
        tracks: indexed("combination", TrackKind::Combination, ("Combination", "Kombination"), stops).into_iter().collect(),
        owners: Vec::new(),
        description: Text::new("scene", "Szene"),
    }
}

fn act(lane: &Lane, action: &str, direction: f64) -> Lane {
    let event = view_event(action, None, Some(direction)).expect("a view event");
    let mutations = view_lane_mutations(event, lane, report, scene);
    assert!(mutations.len() <= 1, "one replace mutation at most");
    Lane(mutations.into_iter().next().unwrap_or_else(|| lane.0.clone()))
}

#[test]
fn a_viewer_steps_checks_through_its_ephemeral_lane() {
    let mut lane = Lane(NormTransient::default());
    let mut visited = Vec::new();
    for _ in 0..5 {
        lane = act(&lane, VIEW_ACTION_STEP_CHECK, 1.0);
        visited.push(lane.0.view.check.expect("a selected check"));
    }
    assert_eq!(visited, [1, 4, 2, 0, 3], "list order: worst entity first");
    assert_eq!(act(&lane, VIEW_ACTION_STEP_CHECK, 1.0).0.view.check, Some(1), "wraps");
}

#[test]
fn a_viewer_steps_the_governing_check_and_the_combination_keyframes() {
    let mut lane = Lane(NormTransient::default());
    let mut visited = Vec::new();
    for _ in 0..5 {
        lane = act(&lane, VIEW_ACTION_STEP_GOVERNING, 1.0);
        visited.push(lane.0.view.check.expect("a selected check"));
    }
    assert_eq!(visited, [1, 2, 0, 4, 3], "1.12, 0.95, 0.64, 0.30, not applicable last");
    let first = act(&Lane(NormTransient::default()), VIEW_ACTION_STEP_COMBINATION, 1.0);
    assert_eq!(first.0.view.item, Some(0));
    assert_eq!(first.0.view.track.as_deref(), Some("combination"));
    assert_eq!(act(&first, VIEW_ACTION_STEP_COMBINATION, 1.0).0.view.item, Some(1));
    assert_eq!(act(&first, VIEW_ACTION_STEP_COMBINATION, -1.0).0.view.item, Some(2), "wraps backwards");
}

#[test]
fn an_unchanged_lane_publishes_no_mutation_and_other_view_events_fold_directly() {
    let lane = Lane(NormTransient::default());
    assert!(view_lane_mutations(LaneEvent::ViewCheck(None), &lane, report, scene).is_empty());
    assert_eq!(view_lane_mutations(LaneEvent::ViewCheck(Some(2)), &lane, report, scene)[0].view.check, Some(2));
    assert!(view_event("nope", None, Some(1.0)).is_none());
}

#[test]
fn the_viewer_tool_is_published_on_the_transient_lane_only() {
    assert_eq!(NORM_VIEWER_TOOL_IDS, ["viewportAct"]);
    assert_eq!(NORM_VIEWER_PUBLICATION_CONTRACTS.len(), 1);
    assert!(matches!(NORM_VIEWER_PUBLICATION_CONTRACTS[0].lanes, [ArtifactToolPublicationLane::Transient]));
    assert_eq!(view_selected_check(&ViewLane { check: Some(3), ..ViewLane::default() }), Some(3));
}

#[test]
fn the_live_scene_of_a_viewer_is_projected_under_the_view_lane() {
    let seen = std::cell::RefCell::new(Vec::new());
    let lane = Lane(NormTransient { view: ViewLane { group: Some(1), item: Some(0), track: Some("fire".into()), value: Some(1800.0), ..ViewLane::default() }, ..NormTransient::default() });
    let event = view_event(VIEW_ACTION_STEP_COMBINATION, None, Some(1.0)).expect("a combination step");
    view_lane_mutations(event, &lane, report, |report, view| {
        seen.borrow_mut().push(view_options_of(view));
        scene(report, view)
    });
    let seen = seen.into_inner();
    assert_eq!(seen.len(), 1, "the scene is projected once, for the combination step");
    assert_eq!((seen[0].traffic_group_item(), seen[0].combination_item(), seen[0].track_value("fire")), (Some(1), Some(0), Some(1800.0)));
    assert_eq!(view_options_of(&ViewLane::default()).stamp_text(), "", "an untouched lane follows the governing choices");
}
