use super::*;
use super::retiring::Retiring;
use semio_plugin::ViewModel;

fn demo() -> Retiring<Fem3dSnapshot> {
    Retiring::new(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot().expect("demo snapshot"))
}

#[test]
fn the_domain_declares_every_entity_kind_once() {
    let definition = fem3d_interaction_definition();
    assert_eq!(definition.id, FEM3D_INTERACTION_DOMAIN);
    let ids: Vec<&str> = definition.granularities.iter().map(|granularity| granularity.id.as_str()).collect();
    assert_eq!(ids, vec![FEM3D_GRANULARITY_ELEMENT, FEM3D_GRANULARITY_NODE, FEM3D_GRANULARITY_SOLID, FEM3D_GRANULARITY_SUPPORT, FEM3D_GRANULARITY_LOAD, FEM3D_GRANULARITY_MATERIAL, FEM3D_GRANULARITY_SECTION, FEM3D_GRANULARITY_LOAD_CASE, FEM3D_GRANULARITY_COMBINATION, FEM3D_GRANULARITY_ANALYSIS]);
    assert!(definition.selection.methods.contains(&SelectionMethod::Rectangle), "the host marquee needs the rectangle method");
}

#[test]
fn entity_kinds_resolve_every_id_of_the_demo() {
    let doc = demo();
    assert_eq!(fem3d_entity_kind(&doc, &fem3d_entity_target("node", "n00_g").unwrap()), Some(FEM3D_GRANULARITY_NODE));
    assert_eq!(fem3d_entity_kind(&doc, &fem3d_entity_target("element", "e1").unwrap()), Some(FEM3D_GRANULARITY_ELEMENT));
    assert_eq!(fem3d_entity_kind(&doc, &fem3d_entity_target("solid", "sol1").unwrap()), Some(FEM3D_GRANULARITY_SOLID));
    assert_eq!(fem3d_entity_kind(&doc, &fem3d_entity_target("support", "s_00").unwrap()), Some(FEM3D_GRANULARITY_SUPPORT));
    assert_eq!(fem3d_entity_kind(&doc, &fem3d_load_target("live", "l2").unwrap()), Some(FEM3D_GRANULARITY_LOAD));
    assert_eq!(fem3d_entity_kind(&doc, &fem3d_entity_target("material", "steel").unwrap()), Some(FEM3D_GRANULARITY_MATERIAL));
    assert_eq!(fem3d_entity_kind(&doc, &fem3d_entity_target("section", "hea200").unwrap()), Some(FEM3D_GRANULARITY_SECTION));
    assert_eq!(fem3d_entity_kind(&doc, &fem3d_entity_target("loadCase", "dead").unwrap()), Some(FEM3D_GRANULARITY_LOAD_CASE));
    assert_eq!(fem3d_entity_kind(&doc, &fem3d_entity_target("combination", "uls").unwrap()), Some(FEM3D_GRANULARITY_COMBINATION));
    assert_eq!(fem3d_entity_kind(&doc, &fem3d_entity_target(FEM3D_GRANULARITY_ANALYSIS, FEM3D_ANALYSIS_ID).unwrap()), Some(FEM3D_GRANULARITY_ANALYSIS));
    assert_eq!(fem3d_entity_kind(&doc, &fem3d_entity_target(FEM3D_GRANULARITY_ANALYSIS, "other").unwrap()), None);
    assert_eq!(fem3d_entity_kind(&doc, &fem3d_entity_target("node", "ghost").unwrap()), None);
    assert_eq!(fem3d_load_owner(&doc, &fem3d_load_target("live", "l3").unwrap()).map(|(case, _)| case), Some("live"));
}

#[test]
fn entity_points_frame_geometry_bearing_entities() {
    let doc = demo();
    assert_eq!(fem3d_entity_point(&doc, &fem3d_entity_target("node", "n20_l1").unwrap()), Some([8.0, 0.0, 2.8]));
    assert_eq!(fem3d_entity_point(&doc, &fem3d_entity_target("element", "fb1_0").unwrap()), Some([4.0, 0.0, 2.8]));
    assert_eq!(fem3d_entity_point(&doc, &fem3d_entity_target("support", "s_20").unwrap()), Some([8.0, 0.0, 0.0]));
    assert_eq!(fem3d_entity_point(&doc, &fem3d_load_target("live", "l2").unwrap()), Some([8.0, 0.0, 2.8]));
    assert_eq!(fem3d_entity_point(&doc, &fem3d_entity_target("solid", "sol1").unwrap()), Some([11.0, 1.0, 0.25]));
    assert_eq!(fem3d_entity_point(&doc, &fem3d_load_target("dead", "l1").unwrap()), Some([11.0, 1.0, 0.5]));
    assert_eq!(fem3d_entity_point(&doc, &fem3d_entity_target("material", "steel").unwrap()), None);
}

#[test]
fn the_transform_utility_is_read_from_the_addressed_window() {
    let mut view = Retiring::new(ViewModel { window_id: Some("model-left".into()), ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native) });
    assert!(!fem3d_transform_armed(&view));
    view.active_utility_by_window_id.insert("model-left".into(), Some(FEM3D_UTILITY_TRANSFORM.into()));
    assert!(fem3d_transform_armed(&view));
    view.active_utility_id = Some("brush".into());
    assert!(fem3d_transform_armed(&view), "a global utility never disarms an addressed window that owns its own");
    view.active_utility_by_window_id.insert("model-left".into(), Some("brush".into()));
    assert!(!fem3d_transform_armed(&view));
}

#[test]
fn a_select_effect_replays_the_framework_action_with_json_targets() {
    let target = fem3d_entity_target(FEM3D_GRANULARITY_NODE, "n1").unwrap();
    let effect = interaction_select_effect(&[(FEM3D_GRANULARITY_NODE, target.as_str())], "replace", "pick").unwrap();
    let Effect::ReplayShellCommand { action_id, args } = effect else { panic!("replay") };
    assert_eq!(action_id, semio_framework::INTERACTION_SELECT_ACTION_ID);
    let args = args.expect("args");
    assert_eq!(args.get("domainId").and_then(semio_framework_value::DslValue::as_str), Some(FEM3D_INTERACTION_DOMAIN));
    assert!(args.get("targets").and_then(semio_framework_value::DslValue::as_str).expect("targets").contains(&target));
}

/// 🖼️ LAW: every granularity names a registered icon id; an unregistered one silently falls back to the generic circle.
#[test]
fn every_granularity_icon_is_registered_3d() {
    for granularity in fem3d_interaction_definition().granularities {
        assert_ne!(granularity.icon_id, semio_plugin::IconName::Circle, "{} falls back to the generic circle", granularity.id);
    }
}

/// 🗂️ LAW: the topology `selectAll` walks lists every selectable entity of the demo exactly once, each under the granularity `fem3d_entity_kind` resolves it to, elements first (the default Select All scope) — an empty topology made Select All a silent no-op.
#[test]
fn select_all_topology_lists_every_selectable_entity_3d() {
    let doc = demo();
    let topology = fem3d_interaction_topology(&doc);
    let ordered = &topology.domains.get(FEM3D_INTERACTION_DOMAIN).expect("fem3d domain topology").ordered;
    let count = |granularity: &str| ordered.iter().filter(|node| node.granularity == granularity).count();
    assert_eq!(count(FEM3D_GRANULARITY_ELEMENT), doc.elements.len());
    assert_eq!(count(FEM3D_GRANULARITY_NODE), doc.nodes.len());
    assert_eq!(count(FEM3D_GRANULARITY_SOLID), doc.solids.len());
    assert_eq!(count(FEM3D_GRANULARITY_SUPPORT), doc.supports.len());
    assert_eq!(count(FEM3D_GRANULARITY_LOAD), doc.load_cases.iter().map(|case| case.loads.len()).sum::<usize>());
    assert_eq!(count(FEM3D_GRANULARITY_LOAD_CASE), doc.load_cases.len());
    assert_eq!(count(FEM3D_GRANULARITY_COMBINATION), doc.combinations.len());
    assert_eq!(count(FEM3D_GRANULARITY_ANALYSIS), 1);
    assert!(!ordered.is_empty() && ordered[0].granularity == FEM3D_GRANULARITY_ELEMENT, "elements lead the topology");
    assert!(ordered.iter().all(|node| fem3d_entity_kind(&doc, &node.id) == Some(node.granularity.as_str())), "every listed id resolves to its own granularity");
    let unique: std::collections::BTreeSet<&str> = ordered.iter().map(|node| node.id.as_str()).collect();
    assert_eq!(unique.len(), ordered.len(), "no id is listed twice");
}
