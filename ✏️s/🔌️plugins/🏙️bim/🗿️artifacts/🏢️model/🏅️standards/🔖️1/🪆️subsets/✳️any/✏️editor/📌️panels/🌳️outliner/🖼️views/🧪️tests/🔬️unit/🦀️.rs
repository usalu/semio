use super::*;
use crate::editor::bim::panels::outliner::{render, BODY_KEY};
use crate::{ModelInference, View};
use semio_framework_plugin::ViewModel;
use semio_framework_ui_locale::{Locale, Terminology};

fn demo() -> (ModelSnapshot, String, Vec<String>) {
    let mut snapshot = crate::standards::v1::subsets::any::io::text::snapshot::default_snapshot();
    snapshot.views.clear();
    let building = snapshot.buildings.keys().next().cloned().expect("a building");
    let mut storeys: Vec<(&String, i32)> = snapshot.storeys.iter().filter(|(_, storey)| storey.building == building).map(|(id, storey)| (id, storey.level)).collect();
    storeys.sort_by_key(|row| row.1);
    let storeys: Vec<String> = storeys.into_iter().map(|row| row.0.clone()).collect();
    (snapshot, building, storeys)
}

fn plan(building: &str, name: &str, kind: ViewKind, storey: &str) -> View {
    View { storey: Some(storey.into()), ..View::standard(building, name, kind) }
}

fn text(snapshot: &ModelSnapshot, locale: Locale) -> String {
    let view = ViewModel::new(locale, Terminology::Native);
    let node = render(snapshot, &ModelInference::default(), crate::editor::bim::terminology::bim_labels(&view), &semio_framework_plugin::TreeWindows::for_body(&view, BODY_KEY)).expect("the outliner renders");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).expect("projects")
}

#[semio_framework_async_macros::async_test]
async fn kinds_group_as_plans_ceiling_plans_sections_elevations_and_cameras() {
    let kinds = [ViewKind::Plan, ViewKind::CeilingPlan, ViewKind::Section, ViewKind::Elevation, ViewKind::Orthographic, ViewKind::Perspective];
    assert_eq!(kinds.map(group_of), [0, 1, 2, 3, 4, 4]);
}

#[semio_framework_async_macros::async_test]
async fn the_groups_of_a_building_follow_the_kind_then_the_level_of_the_storey_then_the_name() {
    let (mut snapshot, building, storeys) = demo();
    assert!(storeys.len() >= 2, "the demo house has two storeys");
    snapshot.views.insert("v-high-a".into(), plan(&building, "A", ViewKind::Plan, &storeys[1]));
    snapshot.views.insert("v-low-z".into(), plan(&building, "Z", ViewKind::Plan, &storeys[0]));
    snapshot.views.insert("v-low-b".into(), plan(&building, "B", ViewKind::Plan, &storeys[0]));
    snapshot.views.insert("v-ceiling".into(), plan(&building, "Ceiling", ViewKind::CeilingPlan, &storeys[0]));
    snapshot.views.insert("v-camera".into(), View::standard(&building, "Camera", ViewKind::Perspective));
    snapshot.views.insert("v-ortho".into(), View::standard(&building, "Axonometric", ViewKind::Orthographic));
    let groups = groups(&snapshot, &building);
    assert_eq!(groups.iter().map(|group| group.0).collect::<Vec<_>>(), vec![0, 1, 4], "groups without a view are left out");
    assert_eq!(groups[0].1, vec!["v-low-b", "v-low-z", "v-high-a"], "ground plans by name, then the first floor");
    assert_eq!(groups[2].1, vec!["v-ortho", "v-camera"]);
    assert_eq!(super::groups(&snapshot, &building), groups, "the order is deterministic");
}

#[semio_framework_async_macros::async_test]
async fn only_the_views_of_the_building_count_and_no_view_means_no_row() {
    let (mut snapshot, building, _) = demo();
    let windows_view = ViewModel::new(Locale::En, Terminology::Native);
    let windows = semio_framework_plugin::TreeWindows::for_body(&windows_view, BODY_KEY);
    let labels = crate::editor::bim::terminology::bim_labels(&windows_view);
    assert!(groups(&snapshot, &building).is_empty());
    assert!(views_row(&windows, &snapshot, labels, &building).is_none());
    snapshot.views.insert("v-elsewhere".into(), View::standard("another-building", "Elsewhere", ViewKind::Perspective));
    assert!(groups(&snapshot, &building).is_empty() && views_row(&windows, &snapshot, labels, &building).is_none());
    snapshot.views.insert("v-here".into(), View::standard(&building, "Here", ViewKind::Perspective));
    assert!(views_row(&windows, &snapshot, labels, &building).is_some_and(|row| row.is_ok()));
}

#[semio_framework_async_macros::async_test]
async fn the_scale_text_reads_the_authored_scale_of_the_view() {
    let (mut snapshot, building, _) = demo();
    snapshot.views.insert("v".into(), View { scale: 50, ..View::standard(&building, "Scaled", ViewKind::Perspective) });
    assert_eq!(scale_text(&snapshot, "v").as_deref(), Some("1:50"));
    assert_eq!(scale_text(&snapshot, "missing"), None);
}

#[semio_framework_async_macros::async_test]
async fn the_outliner_shows_the_views_in_both_languages() {
    let (mut snapshot, building, storeys) = demo();
    snapshot.views.insert("v-plan".into(), plan(&building, "Entrance plan", ViewKind::Plan, &storeys[0]));
    let english = text(&snapshot, Locale::En);
    assert!(english.contains("Views") && english.contains("Plans") && english.contains("Entrance plan"), "{english}");
    let german = text(&snapshot, Locale::De);
    assert!(german.contains("Ansichten") && german.contains("Grundrisse") && german.contains("Entrance plan"), "{german}");
    assert!(!text(&demo().0, Locale::En).contains("Views"), "a building without a view has no Views node");
}
