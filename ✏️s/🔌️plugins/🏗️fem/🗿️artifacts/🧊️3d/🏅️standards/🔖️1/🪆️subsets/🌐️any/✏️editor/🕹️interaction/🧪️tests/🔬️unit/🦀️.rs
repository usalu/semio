use super::*;
use semio_framework_plugin::ViewModel;

fn demo() -> Fem3dSnapshot {
    crate::standards::v1::subsets::any::schema::snapshot::text::fem3d_boot_snapshot()
}

#[test]
fn the_domain_declares_every_entity_kind_once() {
    let definition = fem3d_interaction_definition();
    assert_eq!(definition.id, FEM3D_INTERACTION_DOMAIN);
    let ids: Vec<&str> = definition.granularities.iter().map(|granularity| granularity.id.as_str()).collect();
    assert_eq!(ids, vec![FEM3D_GRANULARITY_NODE, FEM3D_GRANULARITY_ELEMENT, FEM3D_GRANULARITY_SOLID, FEM3D_GRANULARITY_SUPPORT, FEM3D_GRANULARITY_LOAD, FEM3D_GRANULARITY_MATERIAL, FEM3D_GRANULARITY_SECTION, FEM3D_GRANULARITY_LOAD_CASE, FEM3D_GRANULARITY_COMBINATION]);
    assert!(definition.selection.methods.contains(&SelectionMethod::Rectangle), "the host marquee needs the rectangle method");
}

#[test]
fn entity_kinds_resolve_every_id_of_the_demo() {
    let doc = demo();
    assert_eq!(fem3d_entity_kind(&doc, "n00_g"), Some(FEM3D_GRANULARITY_NODE));
    assert_eq!(fem3d_entity_kind(&doc, "e1"), Some(FEM3D_GRANULARITY_ELEMENT));
    assert_eq!(fem3d_entity_kind(&doc, "sol1"), Some(FEM3D_GRANULARITY_SOLID));
    assert_eq!(fem3d_entity_kind(&doc, "s_00"), Some(FEM3D_GRANULARITY_SUPPORT));
    assert_eq!(fem3d_entity_kind(&doc, "l2"), Some(FEM3D_GRANULARITY_LOAD));
    assert_eq!(fem3d_entity_kind(&doc, "steel"), Some(FEM3D_GRANULARITY_MATERIAL));
    assert_eq!(fem3d_entity_kind(&doc, "hea200"), Some(FEM3D_GRANULARITY_SECTION));
    assert_eq!(fem3d_entity_kind(&doc, "dead"), Some(FEM3D_GRANULARITY_LOAD_CASE));
    assert_eq!(fem3d_entity_kind(&doc, "uls"), Some(FEM3D_GRANULARITY_COMBINATION));
    assert_eq!(fem3d_entity_kind(&doc, "ghost"), None);
    assert_eq!(fem3d_load_owner(&doc, "l3").map(|(case, _)| case), Some("live"));
}

#[test]
fn entity_points_frame_geometry_bearing_entities() {
    let doc = demo();
    assert_eq!(fem3d_entity_point(&doc, "n20_l1"), Some([8.0, 0.0, 2.8]));
    assert_eq!(fem3d_entity_point(&doc, "fb1_0"), Some([4.0, 0.0, 2.8]));
    assert_eq!(fem3d_entity_point(&doc, "s_20"), Some([8.0, 0.0, 0.0]));
    assert_eq!(fem3d_entity_point(&doc, "l2"), Some([8.0, 0.0, 2.8]));
    assert_eq!(fem3d_entity_point(&doc, "sol1"), Some([11.0, 1.0, 0.25]));
    assert_eq!(fem3d_entity_point(&doc, "l1"), Some([11.0, 1.0, 0.5]));
    assert_eq!(fem3d_entity_point(&doc, "steel"), None);
}

#[test]
fn the_transform_utility_is_read_from_the_addressed_window() {
    let mut view = ViewModel { window_id: Some("model-left".into()), ..Default::default() };
    assert!(!fem3d_transform_armed(&view));
    view.active_utility_by_window_id.insert("model-left".into(), FEM3D_UTILITY_TRANSFORM.into());
    assert!(fem3d_transform_armed(&view));
    view.active_utility_id = Some("brush".into());
    assert!(!fem3d_transform_armed(&view));
}

#[test]
fn a_select_effect_replays_the_framework_action_with_json_targets() {
    let effect = interaction_select_effect(&[(FEM3D_GRANULARITY_NODE, "n1")], "replace", "pick");
    let Effect::ReplayShellCommand { action_id, args } = effect else { panic!("replay") };
    assert_eq!(action_id, semio_framework::INTERACTION_SELECT_ACTION_ID);
    let args = args.expect("args");
    assert_eq!(args.get("domainId").and_then(dsl::DslValue::as_str), Some(FEM3D_INTERACTION_DOMAIN));
    assert!(args.get("targets").and_then(dsl::DslValue::as_str).expect("targets").contains("\"n1\""));
}
