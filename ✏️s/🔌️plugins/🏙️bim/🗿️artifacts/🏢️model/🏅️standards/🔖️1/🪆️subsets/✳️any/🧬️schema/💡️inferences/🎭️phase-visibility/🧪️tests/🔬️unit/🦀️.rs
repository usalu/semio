use super::*;
use crate::mutations::set_element_phase::SetElementPhase;
use crate::mutations::set_element_storey::SetElementStorey;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::ModelInferenceSession;
use crate::{ModelDiff, ModelInference, ModelMutation};
use protocol::{Inference, Mutation};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use std::collections::BTreeSet;

const BUILDING: &str = include_str!("../../../../../🧫️fixtures/💡️inferences/🧮️quantities/🏗️building/📸️snapshot/🔣️.json");

fn building() -> ModelSnapshot {
    from_json_str(BUILDING, JsonMemberPolicy::Reject).expect("the building decodes")
}

fn step(snapshot: &ModelSnapshot, mutation: ModelMutation) -> (ModelSnapshot, ModelDiff) {
    let (diff, messages) = mutation.diff(snapshot).into_parts();
    assert!(messages.iter().all(|message| !matches!(message.level, semio_framework_diagnostic::Severity::Error | semio_framework_diagnostic::Severity::Fatal)), "{messages:?}");
    (protocol::apply_diff(&diff, snapshot).expect("the diff applies"), diff)
}

fn rephase(id: &str, phase: Phase) -> ModelMutation {
    ModelMutation::SetElementPhase(SetElementPhase { id: id.into(), phase })
}

fn restorey(id: &str, storey: &str) -> ModelMutation {
    ModelMutation::SetElementStorey(SetElementStorey { id: id.into(), storey: storey.into() })
}

fn ids(visibility: &PhaseVisibility, view: ViewPhase) -> Vec<&str> {
    visibility.ids(view).iter().map(String::as_str).collect()
}

#[semio_framework_async_macros::async_test]
async fn a_view_phase_shows_exactly_its_own_phase_and_all_shows_every_phase() {
    for view in ViewPhase::ALL {
        for phase in Phase::ALL {
            assert_eq!(view.shows(phase), view == ViewPhase::All || view.filter() == Some(phase), "{view:?} / {phase:?}");
        }
        assert_eq!(ViewPhase::parse(view.key()), Some(view));
        assert_eq!(ViewPhase::of(view.filter()), view);
    }
    assert_eq!(ViewPhase::parse(""), Some(ViewPhase::All));
    assert_eq!(ViewPhase::parse(" Demolished "), Some(ViewPhase::Demolished));
    assert_eq!(ViewPhase::parse("planned"), None);
}

#[semio_framework_async_macros::async_test]
async fn every_element_of_a_storey_is_shown_by_all_and_by_exactly_one_phase() {
    let mut model = building();
    model.walls.get_mut("w-l1").expect("wall").phase = Phase::Demolished;
    model.walls.get_mut("w-iso").expect("wall").phase = Phase::Temporary;
    model.columns.get_mut("c-sq").expect("column").phase = Phase::Existing;
    let visibility = visibility_of(&model, "st-0");
    let all: BTreeSet<&str> = ids(&visibility, ViewPhase::All).into_iter().collect();
    let mut seen = BTreeSet::new();
    for view in ViewPhase::ALL.into_iter().filter(|view| *view != ViewPhase::All) {
        for id in ids(&visibility, view) {
            assert!(seen.insert(id), "{id} is shown by two phases");
        }
    }
    assert_eq!(seen, all);
    assert_eq!(ids(&visibility, ViewPhase::Demolished), ["w-l1"]);
    assert_eq!(ids(&visibility, ViewPhase::Existing), ["c-sq"]);
    assert!(ids(&visibility, ViewPhase::Temporary).contains(&"w-iso"));
    assert!(ids(&visibility, ViewPhase::New).contains(&"sl-holed") && !ids(&visibility, ViewPhase::New).contains(&"w-l1"));
}

#[semio_framework_async_macros::async_test]
async fn an_opening_takes_the_phase_of_its_host() {
    let model = building();
    let (model, _) = step(&model, rephase("w-iso", Phase::Demolished));
    let visibility = visibility_of(&model, "st-0");
    assert_eq!(ids(&visibility, ViewPhase::Demolished), ["o-door", "o-window", "w-iso"]);
    assert_eq!(phase_of(&model, "o-window"), Some(Phase::Demolished));
    assert_eq!(phase_of(&model, "o-void"), Some(Phase::New));
    assert!(visibility.hides(ViewPhase::New, "o-window") && !visibility.hides(ViewPhase::Demolished, "o-window"));
    assert!(!visibility.hides(ViewPhase::New, "not-an-element") && !visibility.knows("not-an-element"));
}

#[semio_framework_async_macros::async_test]
async fn a_moved_element_and_its_openings_change_the_storey_they_are_visible_on() {
    let model = building();
    let before = (visibility_of(&model, "st-0"), visibility_of(&model, "st-1"));
    assert!(before.0.knows("w-iso") && before.0.knows("o-window") && !before.1.knows("w-iso"));
    let (moved, _) = step(&model, restorey("w-iso", "st-1"));
    let (ground, first) = (visibility_of(&moved, "st-0"), visibility_of(&moved, "st-1"));
    assert!(!ground.knows("w-iso") && !ground.knows("o-window") && !ground.knows("o-door"));
    assert!(first.knows("w-iso") && first.knows("o-window") && first.knows("o-door"));
    assert!(ground.knows("o-void"), "the openings of the walls that stay do not move");
}

#[semio_framework_async_macros::async_test]
async fn the_node_is_recomputed_by_a_phase_or_storey_edit_and_by_no_geometry_edit() {
    let model = building();
    let mut session = ModelInferenceSession::new();
    session.refresh(&model);
    assert_eq!(session.inference().phase_visibility.len(), model.storeys.len());

    let (phased, diff) = step(&model, rephase("w-l1", Phase::Demolished));
    session.update(&phased, &diff);
    let report = session.report();
    assert_eq!(report.computed_by_kind.get("phase-visibility"), Some(&1), "{report:?}");
    assert!(!report.computed_by_kind.contains_key("wall-layout") && !report.computed_by_kind.contains_key("solid") && !report.computed_by_kind.contains_key("plan"), "a phase edit leaves the geometry alone: {report:?}");
    assert_eq!(session.inference().phase_visibility["st-0"].ids(ViewPhase::Demolished), ["w-l1"]);

    let (moved, diff) = step(&phased, restorey("w-up", "st-0"));
    session.update(&moved, &diff);
    assert_eq!(session.report().computed_by_kind.get("phase-visibility"), Some(&2), "{:?}", session.report());

    let (shifted, diff) = step(&moved, ModelMutation::SetStoreyHeight(crate::mutations::set_storey_height::SetStoreyHeight { id: "st-1".into(), height: 3.4 }));
    session.update(&shifted, &diff);
    assert!(!session.report().computed_by_kind.contains_key("phase-visibility"), "a storey height edit leaves the visibility alone: {:?}", session.report());
}

#[semio_framework_async_macros::async_test]
async fn the_session_agrees_with_a_fresh_inference_after_a_move() {
    let model = building();
    let mut session = ModelInferenceSession::new();
    session.refresh(&model);
    let (moved, diff) = step(&model, restorey("w-iso", "st-1"));
    let fresh = ModelInference::infer(&moved).expect("infers");
    assert_eq!(session.update(&moved, &diff).phase_visibility, fresh.phase_visibility);
}

#[semio_framework_async_macros::async_test]
async fn the_default_and_determinism_laws_hold() {
    let model = building();
    assert_eq!(visibility_of(&ModelSnapshot::default(), "st-0"), PhaseVisibility { visible: ViewPhase::ALL.into_iter().map(|view| (view.key().to_string(), Vec::new())).collect() });
    assert_eq!(visibility_of(&model, "st-0"), visibility_of(&model, "st-0"));
    assert_eq!(dependency(&model, "st-0"), dependency(&model, "st-0"));
}

use crate::{View, ViewKind};

fn phase_view(model: &ModelSnapshot, phase: Option<Phase>) -> ModelSnapshot {
    let mut model = model.clone();
    let building = model.storeys["st-0"].building.clone();
    model.views.insert("v-phase".into(), View { phase, ..View::of_storey(&building, "Ground plan", ViewKind::Plan, "st-0") });
    model
}

fn drawn_elements(model: &ModelSnapshot) -> BTreeSet<String> {
    let drawing = ModelInference::infer(model).expect("infers").view_linework.remove("v-phase").expect("the view is drawn");
    drawing.lines.regions.iter().map(|region| region.element.clone()).chain(drawing.lines.polylines.iter().map(|line| line.element.clone())).chain(drawing.lines.texts.iter().map(|text| text.element.clone())).filter(|element| phase_of(model, element).is_some()).collect()
}

fn phased_building() -> ModelSnapshot {
    let mut model = building();
    model.walls.get_mut("w-l1").expect("wall").phase = Phase::Demolished;
    model.columns.get_mut("c-sq").expect("column").phase = Phase::Demolished;
    model.slabs.get_mut("sl-holed").expect("slab").phase = Phase::Existing;
    model.spaces.get_mut("sp-hall").expect("space").phase = Phase::Existing;
    model
}

#[semio_framework_async_macros::async_test]
async fn a_view_phase_draws_exactly_the_elements_of_that_phase_for_every_phase_bearing_kind() {
    let model = phased_building();
    let all = drawn_elements(&phase_view(&model, None));
    for id in ["w-l1", "c-sq", "sl-holed", "sp-hall"] {
        assert!(all.contains(id), "{id} is drawn without a phase filter: {all:?}");
    }
    let mut union = BTreeSet::new();
    for phase in Phase::ALL {
        let shown = drawn_elements(&phase_view(&model, Some(phase)));
        assert!(shown.iter().all(|element| phase_of(&model, element) == Some(phase)), "{phase:?} draws only its own elements: {shown:?}");
        union.extend(shown);
    }
    assert_eq!(union, all, "the phases together draw what no filter draws");
    let demolished = drawn_elements(&phase_view(&model, Some(Phase::Demolished)));
    assert!(demolished.contains("w-l1") && demolished.contains("c-sq") && !demolished.contains("sl-holed"));
    let existing = drawn_elements(&phase_view(&model, Some(Phase::Existing)));
    assert!(existing.contains("sl-holed") && existing.contains("sp-hall") && !existing.contains("w-l1"));
}

#[semio_framework_async_macros::async_test]
async fn an_opening_is_drawn_exactly_when_its_host_wall_is() {
    let model = phased_building();
    let demolished = drawn_elements(&phase_view(&model, Some(Phase::Demolished)));
    let new = drawn_elements(&phase_view(&model, Some(Phase::New)));
    for (id, opening) in &model.openings {
        let host = phase_of(&model, &opening.host);
        assert!(!demolished.contains(id) || host == Some(Phase::Demolished), "{id} is drawn by the demolished view only with a demolished host");
        assert!(!new.contains(id) || host == Some(Phase::New), "{id} is drawn by the new view only with a new host");
    }
}

#[semio_framework_async_macros::async_test]
async fn the_view_drawing_is_cache_transparent_and_follows_a_phase_edit_of_any_kind_and_a_storey_move() {
    let mut base = building();
    base.walls.get_mut("w-iso").expect("wall").phase = Phase::Existing;
    let model = phase_view(&base, Some(Phase::Existing));
    let drawn = |model: &ModelInference, element: &str| model.view_linework["v-phase"].lines.regions.iter().any(|region| region.element == element) || model.view_linework["v-phase"].lines.polylines.iter().any(|line| line.element == element);
    let mut session = ModelInferenceSession::new();
    session.refresh(&model);
    assert!(drawn(session.inference(), "w-iso") && !drawn(session.inference(), "c-sq"), "the existing wall is drawn, the new column is not");
    let (edited, diff) = step(&model, rephase("c-sq", Phase::Existing));
    let warm = session.update(&edited, &diff).clone();
    let cold = ModelInference::infer(&edited).expect("infers");
    assert_eq!(warm.view_linework, cold.view_linework, "warm = cold");
    assert_eq!(warm.phase_visibility, cold.phase_visibility);
    assert!(drawn(&warm, "c-sq"), "the column joined the existing phase and is drawn now");
    let (moved, diff) = step(&edited, restorey("w-iso", "st-1"));
    let after = session.update(&moved, &diff).clone();
    assert_eq!(after.view_linework, ModelInference::infer(&moved).expect("infers").view_linework);
    assert!(!drawn(&after, "w-iso"), "the plan of the ground storey no longer holds the moved wall");
}
