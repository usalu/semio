use super::kit::{items, scheduled};
use super::*;
use crate::standards::v1::subsets::any::io::text::inferences::schedules::table_json;
use crate::standards::v1::subsets::any::schema::inferences::model_graph::ModelInferenceSession;
use crate::{Entry, ModelDiff, ModelInference, OpeningPatch, SchedulePatch, TopConstraint, WallPatch};
use protocol::Inference;

fn scheduled_count() -> usize {
    scheduled().schedules.len()
}

fn height_of(table: &ScheduleTable, name: &str) -> f64 {
    let row = items(table).into_iter().find(|row| row.cells[0].display() == name).unwrap_or_else(|| panic!("a row for {name}"));
    match row.cells[4] {
        ScheduleCell::Number { value } => value,
        ref other => panic!("a height, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn the_schedules_are_cache_transparent_warm_equals_cold_equals_uncached() {
    let snapshot = scheduled();
    let uncached = ModelInference::infer(&snapshot).expect("infers");
    let mut session = ModelInferenceSession::new();
    let cold = session.refresh(&snapshot).clone();
    assert!(session.report().computed_by_kind.get("schedule").copied().unwrap_or(0) == scheduled_count(), "the first run computes every schedule once: {:?}", session.report().computed_by_kind);
    let warm = session.refresh(&snapshot).clone();
    assert_eq!(session.report().computed, 0, "a second refresh is all cache hits");
    assert_eq!(cold.schedules, uncached.schedules);
    assert_eq!(warm.schedules, uncached.schedules);
    assert_eq!(table_json(&warm.schedules), table_json(&uncached.schedules));
}

#[semio_framework_async_macros::async_test]
async fn a_quantity_edit_reaches_the_schedule_through_the_gate_and_spares_the_schedules_it_does_not_feed() {
    let snapshot = scheduled();
    let mut session = ModelInferenceSession::new();
    let before = session.update(&snapshot, &ModelDiff::default()).clone();
    assert_eq!(height_of(&before.schedules["sch-wall"], "South"), 3.0, "the wall reaches the top of the storey");
    let edit = ModelDiff::walls("w-south", Entry::Patched(WallPatch { top: Some(TopConstraint::Unconnected { height: 2.5 }), ..Default::default() }));
    let edited = protocol::apply_diff(&edit, &snapshot).expect("the edit applies");
    let after = session.update(&edited, &edit).clone();
    let report = session.report().clone();
    assert!(!report.gated, "the schedules read the walls");
    assert_eq!(height_of(&after.schedules["sch-wall"], "South"), 2.5, "the quantity edit shows in the schedule");
    assert_ne!(before.schedules["sch-wall"], after.schedules["sch-wall"]);
    let recomputed = report.computed_by_kind.get("schedule").copied().unwrap_or(0);
    assert!((1..scheduled_count()).contains(&recomputed), "only the schedules fed by the edited wall are recomputed, not all {}: {:?}", scheduled_count(), report.computed_by_kind);
    assert_eq!(after.schedules["sch-door"], before.schedules["sch-door"], "the door schedule stays what it was");
    assert_eq!(after, ModelInference::infer(&edited).expect("infers"), "and the incremental result equals a fresh inference");
}

#[semio_framework_async_macros::async_test]
async fn a_rename_recomputes_only_the_schedules_that_list_the_renamed_element() {
    let snapshot = scheduled();
    let mut session = ModelInferenceSession::new();
    session.update(&snapshot, &ModelDiff::default());
    let edit = ModelDiff::openings("o-door-1", Entry::Patched(OpeningPatch { name: Some("Entrance".into()), ..Default::default() }));
    let edited = protocol::apply_diff(&edit, &snapshot).expect("the edit applies");
    let after = session.update(&edited, &edit).clone();
    let recomputed = session.report().computed_by_kind.get("schedule").copied().unwrap_or(0);
    assert_eq!(recomputed, 2, "the door schedule and the material take-off list the door: {:?}", session.report().computed_by_kind);
    assert_eq!(items(&after.schedules["sch-door"])[0].cells[0].display(), "Entrance");
    assert_eq!(after, ModelInference::infer(&edited).expect("infers"));
}

#[semio_framework_async_macros::async_test]
async fn a_definition_edit_recomputes_its_own_schedule_alone() {
    let snapshot = scheduled();
    let mut session = ModelInferenceSession::new();
    let before = session.update(&snapshot, &ModelDiff::default()).clone();
    let edit = ModelDiff::schedules("sch-wall", Entry::Patched(SchedulePatch { itemize: Some(false), ..Default::default() }));
    let edited = protocol::apply_diff(&edit, &snapshot).expect("the edit applies");
    let after = session.update(&edited, &edit).clone();
    assert_eq!(session.report().computed_by_kind.get("schedule"), Some(&1), "{:?}", session.report().computed_by_kind);
    assert_ne!(before.schedules["sch-wall"], after.schedules["sch-wall"]);
    assert_eq!(after.schedules["sch-door"], before.schedules["sch-door"]);
    assert_eq!(after, ModelInference::infer(&edited).expect("infers"));
}

#[semio_framework_async_macros::async_test]
async fn a_schedule_leaves_the_graph_with_its_definition() {
    let mut snapshot = scheduled();
    let mut session = ModelInferenceSession::new();
    session.refresh(&snapshot);
    snapshot.schedules.remove("sch-door");
    let after = session.refresh(&snapshot).clone();
    assert!(!after.schedules.contains_key("sch-door") && after.schedules.len() == scheduled_count() - 1);
    assert_eq!(after, ModelInference::infer(&snapshot).expect("infers"));
}

#[semio_framework_async_macros::async_test]
async fn a_schedule_over_a_deleted_storey_is_removed_with_it_and_comes_back_with_it() {
    use crate::standards::v1::subsets::any::schema::mutations::cascade;
    let mut snapshot = scheduled();
    snapshot.schedules.get_mut("sch-door").expect("the door schedule").storeys = vec!["st-first".into()];
    let removal = cascade::closure(&snapshot, &["st-first".to_string()]).unwrap_or_else(|refusal| panic!("{}", refusal.message));
    assert!(removal.schedules.contains("sch-door") && !removal.schedules.contains("sch-wall"), "only the schedule scoped to the storey leaves");
    let diff = removal.diff();
    let after = protocol::apply_diff(&diff, &snapshot).expect("the removal applies");
    assert!(!after.schedules.contains_key("sch-door") && after.schedules.contains_key("sch-wall"));
    let mut session = ModelInferenceSession::new();
    let table = session.refresh(&after).clone();
    assert!(!table.schedules.contains_key("sch-door"));
    assert_eq!(table, ModelInference::infer(&after).expect("infers"));
}
