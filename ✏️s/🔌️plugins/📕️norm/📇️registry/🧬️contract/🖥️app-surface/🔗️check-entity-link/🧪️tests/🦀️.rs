//! 🧪️ Check and entity link tests: hand-derived owners of a four-check report and a three-entity scene.

use super::*;
use semio_s_plugin_norm_compliance::{LocalizedCopy, SubjectRef};
use semio_s_plugin_norm_structural_scene::{Aabb, Appearance, EntityKind, EntityRef, Geometry, LocalizedCopy as Text, MaterialRole, PickPolicy, SceneEntity, SceneRevision, Vec3, SCENE_SCHEMA_ID};

fn check(id: &str, entity: &str, utilization: f64, status: CheckStatus) -> CheckResult {
    CheckResult::assess(id, "ULS", ClauseId::new("EN 1993", "1-1", "6.2.9"), SubjectRef::new(entity, "path", LocalizedCopy::new(entity, entity)), LocalizedCopy::new(id, id))
        .utilization(Quantity::new(QuantityKind::Length, utilization * 0.0125), Quantity::new(QuantityKind::Length, 0.0125))
        .status(status)
        .build()
}

fn report() -> CheckReport {
    let mut report = CheckReport::default();
    for (id, entity, utilization, status) in [("a", "B1", 0.64, CheckStatus::Pass), ("b", "B2", 1.12, CheckStatus::Fail), ("c", "B1:flange", 0.95, CheckStatus::Warning), ("g", "", 0.30, CheckStatus::Pass)] {
        report.push(check(id, entity, utilization, status));
    }
    report
}

fn entity(id: &str) -> SceneEntity {
    SceneEntity {
        entity: EntityRef::new(EntityKind::Member, id, format!("members[id={}]", id.split(':').next().unwrap_or(id))),
        geometry: Geometry::Group { members: Vec::new() },
        appearance: Appearance::role(MaterialRole::Steel),
        utilisation: None,
        label: None,
        accessible_name: Text::new(id, id),
        accessible_detail: Text::new("", ""),
        layer: "members".into(),
        pick: PickPolicy::Whole,
        handles: Vec::new(),
        content_hash: String::new(),
    }
    .sealed()
}

fn scene() -> StructuralScene {
    StructuralScene {
        schema: SCENE_SCHEMA_ID.into(),
        revision: SceneRevision { document_seq: 1, report_seq: 1 },
        bounds: Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(4.0, 1.0, 1.0)),
        entities: vec![entity("B1"), entity("B1:flange"), entity("B2")],
        annotations: Vec::new(),
        legends: Vec::new(),
        layers: Vec::new(),
        tracks: Default::default(),
        owners: Vec::new(),
        description: Text::new("scene", "Szene"),
    }
}

fn ids(focus: &CheckFocus) -> Vec<String> {
    focus.targets.iter().map(|t| t.id.clone()).collect()
}

#[test]
fn a_check_owner_is_its_entity_by_base_id_or_the_document() {
    let report = report();
    assert_eq!(check_owner(&report, 0), Some(CheckOwner::Entity("B1".into())));
    assert_eq!(check_owner(&report, 2), Some(CheckOwner::Entity("B1".into())), "a part id belongs to its base entity");
    assert_eq!(check_owner(&report, 3), Some(CheckOwner::Global));
    assert_eq!(check_owner(&report, 9), None);
}

#[test]
fn selecting_a_check_selects_the_owner_entity_and_frames_it() {
    let (report, scene) = (report(), scene());
    let focus = focus_of_check(&report, &scene, 1, CameraPreset::Iso).expect("focus");
    assert_eq!(focus.owner, CheckOwner::Entity("B2".into()));
    assert_eq!(ids(&focus), vec![crate::viewport3d::picking::entity_address(EntityKind::Member, "B2").expect("address")]);
    assert!(focus.camera.is_some());
    let part = focus_of_check(&report, &scene, 2, CameraPreset::Iso).expect("focus");
    assert_eq!(ids(&part), ids(&focus_of_check(&report, &scene, 0, CameraPreset::Iso).expect("focus")), "a part check selects the entity whose id is the owner");
    let effect = focus_effect("norm.en9101", &focus).expect("a selection effect");
    match effect {
        semio_plugin::Effect::ReplayShellCommand { action_id, args: Some(args) } => {
            assert_eq!(action_id, semio_framework::INTERACTION_SELECT_ACTION_ID);
            let text = |key: &str| args.get(key).and_then(DslValue::as_str).map(str::to_owned).unwrap_or_default();
            assert_eq!((text("domainId").as_str(), text("merge").as_str(), text("method").as_str()), ("norm.en9101", "replace", "pick"));
            assert_eq!(serde_json::from_str::<Vec<ShellSelectTarget>>(&text("targets")).expect("targets json"), focus.targets);
        }
        other => panic!("expected an interactionSelect replay, got {other:?}"),
    }
}

#[test]
fn a_document_level_check_selects_the_global_badge_and_keeps_the_camera() {
    let (report, scene) = (report(), scene());
    let focus = focus_of_check(&report, &scene, 3, CameraPreset::Iso).expect("focus");
    assert_eq!((focus.owner.clone(), focus.camera), (CheckOwner::Global, None));
    assert!(focus_effect("norm.en9101", &focus).is_none(), "a document-level check selects no entity");
    assert_eq!(owner_of_scene_id(GLOBAL_BADGE), CheckOwner::Global);
    assert_eq!(owner_of_scene_id("B1:flange"), CheckOwner::Entity("B1".into()));
}

#[test]
fn selecting_an_entity_narrows_the_checks_to_its_own_and_scrolls_to_the_governing_one() {
    let report = report();
    let mut state = ChecksViewState::default();
    let b1 = checks_of_owner(&report, &CheckOwner::Entity("B1".into()), &state, Locale::En);
    assert_eq!(b1.rows.iter().copied().collect::<std::collections::BTreeSet<_>>(), [0, 2].into(), "both the entity and its part checks");
    assert_eq!(entity_selected_event(&b1), Some(LaneEvent::ViewCheck(Some(2))));
    state.filter.warning = false;
    assert_eq!(checks_of_owner(&report, &CheckOwner::Entity("B1".into()), &state, Locale::En).rows, vec![0], "the status filter still applies");
    assert_eq!(checks_of_owner(&report, &CheckOwner::Entity("B1".into()), &state, Locale::En).scroll_to, Some(0));
    assert_eq!(checks_of_owner(&report, &CheckOwner::Global, &state, Locale::En).rows, vec![3]);
    let none = checks_of_owner(&report, &CheckOwner::Entity("nope".into()), &state, Locale::En);
    assert_eq!((none.rows.len(), entity_selected_event(&none)), (0, None));
}

#[test]
fn both_directions_agree() {
    let (report, scene) = (report(), scene());
    for index in 0..3 {
        let CheckOwner::Entity(owner) = check_owner(&report, index).expect("owner") else { panic!("entity owner") };
        let back = checks_of_owner(&report, &owner_of_scene_id(&owner), &ChecksViewState::default(), Locale::En);
        assert!(back.rows.contains(&index) && focus_of_check(&report, &scene, index, CameraPreset::Iso).is_some());
    }
}

#[semio_framework_async_macros::async_test]
async fn pick_step_and_fresh_pick_follow_the_real_interaction_machine() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🎞️sequence.json")).expect("selection sequence");
    let mut report = CheckReport::default();
    for row in fixture["checks"].as_array().unwrap() {
        report.push(check(row["id"].as_str().unwrap(), row["entity"].as_str().unwrap(), row["utilization"].as_f64().unwrap(), CheckStatus::Pass));
    }
    let scene = scene();
    let topology = crate::domain_topology(&DslValue::object(Vec::new()), &scene, &report, None);
    let spec = crate::protocol::SelectionSpec { modes: vec![crate::protocol::SelectionMode::Multiple], methods: vec![crate::protocol::SelectionMethod::Pick], merges: vec![crate::protocol::MergeMode::Replace], transitive: false, broadcast: false };
    let mut interaction = crate::protocol::DomainSelection::default();
    let mut lane = crate::ViewLane::default();
    for row in fixture["events"].as_array().unwrap() {
        let targets = if row["kind"] == "pick" {
            vec![crate::protocol::InteractionTarget { granularity: "member".into(), id: entity_address(EntityKind::Member, row["entity"].as_str().unwrap()).unwrap() }]
        } else {
            let forward = row["forward"].as_bool().unwrap();
            let effect = check_step_selection_effect("norm.sequence", &report, &lane, &interaction.ids, LaneEvent::ViewStep { step: ViewStep::Check, forward }, || scene.clone()).expect("step selection effect");
            let semio_framework_plugin::Effect::ReplayShellCommand { action_id, args: Some(args) } = effect else { panic!("interaction selection replay") };
            assert_eq!(action_id, semio_framework::INTERACTION_SELECT_ACTION_ID);
            let targets: Vec<ShellSelectTarget> = serde_json::from_str(args.get("targets").unwrap().as_str().unwrap()).expect("framework targets");
            lane = crate::step_view(&lane, ViewStep::Check, forward, &report, || scene.clone());
            targets.into_iter().map(|target| crate::protocol::InteractionTarget { granularity: target.granularity, id: target.id }).collect()
        };
        interaction = crate::protocol::next_selection(&spec, &interaction, &topology, &crate::protocol::SelectionInput { targets, merge: crate::protocol::MergeMode::Replace, mode: crate::protocol::SelectionMode::Multiple }).await;
        let selected = crate::NormSelection::from_interaction(&report, lane.check, &interaction.ids);
        assert_eq!(selected.check, row["check"].as_u64().map(|index| index as usize));
        assert_eq!(selected.primary(&report).as_deref(), row["focus"].as_str());
        let checks = crate::plan_checks(&report, &selected.checks_state(&report, ChecksViewState::default()), Locale::En);
        assert!(checks.iter().any(|group| group.rows.contains(&selected.check.unwrap())));
    }
}
