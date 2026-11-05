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

fn projected(context: &str, node: semio_framework_plugin::BuiltNode) -> serde_json::Value {
    let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap_or_else(|error| panic!("{context} IR: {error}"));
    serde_json::from_str(&json).expect("projected IR JSON")
}

fn contains_selected(value: &serde_json::Value, label: &str) -> bool {
    match value {
        serde_json::Value::Object(fields) => (fields.get("selected").and_then(serde_json::Value::as_bool) == Some(true) && value.to_string().contains(label)) || fields.values().any(|value| contains_selected(value, label)),
        serde_json::Value::Array(values) => values.iter().any(|value| contains_selected(value, label)),
        serde_json::Value::String(text) => serde_json::from_str::<serde_json::Value>(text).ok().is_some_and(|value| contains_selected(&value, label)),
        _ => false,
    }
}

#[test]
fn readonly_windows_render_the_same_picked_owner_without_edit_actions() {
    use semio_s_plugin_norm_structural_scene::{Appearance, Axis, AxisScale, DataColumn, DataTable, Diagram2d, EntityKind, EntityRef, Frame, Geometry, Layer, MaterialRole, PickPolicy, SceneEntity, SolidGeometry, SolidShape, TickRule};
    let report = report();
    let mut scene = scene(&report, &ViewLane::default());
    scene.layers = vec![Layer { id: "members".into(), name: Text::new("Members", "Bauteile"), visible: true, locked: false }];
    scene.entities = ["B1", "B2"].into_iter().map(|id| SceneEntity {
        entity: EntityRef::new(EntityKind::Solid, id, format!("members[id={id}]")),
        geometry: Geometry::Solid(SolidGeometry { shape: SolidShape::Box { frame: Frame::WORLD, size: Vec3::new(1.0, 1.0, 1.0) } }),
        appearance: Appearance::role(MaterialRole::Steel), utilisation: None, label: None,
        accessible_name: Text::new(id, id), accessible_detail: Text::new("detail", "Detail"), layer: "members".into(), pick: PickPolicy::Whole, handles: Vec::new(), content_hash: String::new(),
    }.sealed()).collect();
    let picked = [crate::viewport3d::picking::entity_address(EntityKind::Solid, "B2").unwrap()];
    let frame = NormViewerFrame::new(report, ViewLane { check: Some(0), ..Default::default() }, &picked);
    assert_eq!((frame.selection.check, frame.diagram_focus().as_deref()), (Some(1), Some("B2")));
    let viewport = projected("viewport", frame.render_viewport("norm.en1993.view.viewport", &scene, "norm.en1993", "s.norm.en1993@1/*#viewer", semio_framework_ui_locale::Locale::En).expect("viewer viewport"));
    assert!(contains_selected(&viewport, "B2"), "{viewport}");
    assert!(!viewport.to_string().contains("setFields"));
    let checks = projected("checks", frame.render_checks(&semio_framework_plugin::TreeWindows::unhosted(), "norm.en1993", "s.norm.en1993@1/*#viewer", semio_framework_ui_locale::Locale::En).expect("viewer checks"));
    assert!(contains_selected(&checks, "check b"), "{checks}");
    assert!(!checks.to_string().contains("setSelectedCheckIndex") && !checks.to_string().contains("applyRemedy"));
    assert!(checks.to_string().contains(&crate::check_address("b").unwrap()));
    let focus = frame.diagram_focus().unwrap();
    let diagram = Diagram2d {
        id: format!("diagram-{focus}"), title: Text::new(format!("Diagram {focus}"), format!("Diagramm {focus}")), description: Text::new("owner diagram", "Bauteildiagramm"),
        axes: ["x", "y"].into_iter().map(|name| Axis { label: Text::new(name, name), quantity: semio_s_plugin_norm_structural_scene::QuantityKind::Dimensionless, unit: "1".into(), scale: AxisScale::Linear, domain: None, ticks: TickRule::Auto }).collect(),
        series: Vec::new(), markers: Vec::new(), annotations: Vec::new(), focus: Some(EntityRef::new(EntityKind::Solid, &focus, "")), legend: false,
        table_twin: DataTable { columns: vec![DataColumn { header: Text::new("Value", "Wert"), unit: "1".into(), values: vec![0.0, 1.0] }] },
    };
    let diagrams = projected("diagrams", frame.render_diagrams("norm.en1993.view.diagrams", &[diagram], &semio_framework_plugin::TreeWindows::unhosted(), semio_framework_ui_locale::Locale::En).expect("viewer diagrams"));
    assert!(diagrams.to_string().contains("Diagram B2"));
    assert!(!diagrams.to_string().contains("setFields"));
}

#[test]
fn shortcut_help_groups_the_complete_neutral_table_without_exceeding_fixed_lists() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../🧪️tests/🧫️fixtures/shortcut-help/🔣️.json")).expect("shortcut help law");
    for (locale, language) in [(semio_framework_ui_locale::Locale::En, "en"), (semio_framework_ui_locale::Locale::De, "de")] {
        let help = projected("shortcut help", crate::toolbar::render_shortcut_help(locale, false).expect("complete shortcut help"));
        let groups = help["children"][0]["children"].as_array().expect("scope groups");
        assert_eq!(groups.len(), law["scopes"].as_array().expect("scopes").len());
        for (group, expected) in groups.iter().zip(law["scopes"].as_array().expect("scopes")) {
            assert!(group["key"].as_str().expect("scope key").ends_with(expected["id"].as_str().expect("id")));
            assert_eq!(group["component"]["label"], expected[language]);
            assert_eq!(group["children"].as_array().expect("shortcut rows").len() as u64, expected["rows"].as_u64().expect("row count"));
        }
    }
}
