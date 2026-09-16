use super::*;
use crate::FemAxis;

fn demo() -> Fem3dSnapshot {
    crate::standards::v1::subsets::any::schema::snapshot::text::fem3d_boot_snapshot()
}

fn node(doc: &Fem3dSnapshot, mutations: &[Fem3dMutation], id: &str) -> [f64; 3] {
    let mut snapshot = doc.clone();
    for mutation in mutations {
        crate::standards::v1::subsets::any::schema::mutations::apply_fem3d_mutation(&mut snapshot, mutation).expect("applies");
    }
    let node = snapshot.nodes.iter().find(|node| node.id == id).expect("node");
    [node.x, node.y, node.z]
}

#[test]
fn targets_resolve_members_supports_and_loads_to_their_geometry() {
    let doc = demo();
    let targets = fem3d_transform_targets(&doc, &["fb1_0".into(), "s_20".into(), "l1".into(), "steel".into()]);
    assert_eq!(targets.node_ids.iter().cloned().collect::<Vec<_>>(), vec!["n00_l1", "n20_g", "n20_l1"]);
    assert_eq!(targets.solid_ids.iter().cloned().collect::<Vec<_>>(), vec!["sol1"]);
    assert!(fem3d_transform_targets(&doc, &["steel".into()]).is_empty());
}

#[test]
fn translate_moves_nodes_and_solids_by_the_delta() {
    let doc = demo();
    let mutations = fem3d_translate_selection_mutations(&doc, &["n20_l1".into(), "sol1".into()], [1.0, 2.0, 3.0]);
    assert_eq!(mutations.len(), 2);
    assert_eq!(node(&doc, &mutations, "n20_l1"), [9.0, 2.0, 5.8]);
    let Fem3dMutation::ReplaceSolid(replaced) = &mutations[1] else { panic!("solid") };
    assert_eq!(replaced.new_solid.outline[0], [11.0, 2.0]);
    assert!((replaced.new_solid.base_z - 3.0).abs() < 1e-12);
    assert_eq!(replaced.new_solid.height, doc.solids[0].height);
    assert!(fem3d_translate_selection_mutations(&doc, &["n20_l1".into()], [0.0, 0.0, 0.0]).is_empty());
}

#[test]
fn rotate_turns_nodes_about_the_pivot_and_solids_only_about_their_axis() {
    let doc = demo();
    let quarter = std::f64::consts::FRAC_PI_2;
    let mutations = fem3d_rotate_selection_mutations(&doc, &["n20_g".into(), "n00_g".into()], [0.0, 0.0, 1.0], quarter);
    let moved = node(&doc, &mutations, "n20_g");
    assert!((moved[0] - 4.0).abs() < 1e-9 && (moved[1] - 4.0).abs() < 1e-9, "n20_g swings a quarter turn about the pivot (4, 0, 0): {moved:?}");
    let about_z = fem3d_rotate_selection_mutations(&doc, &["sol1".into()], [0.0, 0.0, 1.0], quarter);
    assert_eq!(about_z.len(), 1, "a Z solid rotates about a Z axis");
    let about_x = fem3d_rotate_selection_mutations(&doc, &["sol1".into()], [1.0, 0.0, 0.0], quarter);
    assert!(about_x.is_empty(), "a Z solid cannot spell a rotation about X");
    assert!(fem3d_rotate_selection_mutations(&doc, &["n20_g".into()], [0.0, 0.0, 0.0], quarter).is_empty());
}

#[test]
fn scale_stretches_about_the_pivot_and_solids_along_their_axis() {
    let mut doc = demo();
    doc.solids[0].axis = FemAxis::Y;
    let mutations = fem3d_scale_selection_mutations(&doc, &["sol1".into()], [1.0, 2.0, 1.0]);
    let Fem3dMutation::ReplaceSolid(replaced) = &mutations[0] else { panic!("solid") };
    assert!((replaced.new_solid.height - 1.0).abs() < 1e-9, "the Y extrusion doubled: {}", replaced.new_solid.height);
    let nodes = fem3d_scale_selection_mutations(&doc, &["n00_g".into(), "n20_g".into()], [2.0, 1.0, 1.0]);
    assert_eq!(node(&doc, &nodes, "n20_g"), [12.0, 0.0, 0.0]);
    assert!(fem3d_scale_selection_mutations(&doc, &["n20_g".into()], [1.0, 1.0, 1.0]).is_empty());
}

#[test]
fn the_selection_record_arms_the_gumball_only_with_the_transform_utility() {
    let doc = demo();
    let config = Fem3dGumballConfig::default();
    let selected = Fem3dInteractionSnapshot::selecting(["n20_l1"]);
    assert!(fem3d_gumball_active(&doc, &selected, true, &config));
    assert!(!fem3d_gumball_active(&doc, &selected, false, &config));
    assert!(!fem3d_gumball_active(&doc, &Fem3dInteractionSnapshot::selecting(["steel"]), true, &config));
    let json = fem3d_selection_json(&doc, &selected, true, &config);
    assert!(json.contains("\"gumballLiveDispatch\":true"), "{json}");
    assert!(json.contains("\"gumballTarget\":[8"), "{json}");
    let off = Fem3dGumballConfig { move_axes: false, move_planes: false, rotate: false, scale_axes: false, scale_uniform: false };
    assert!(!fem3d_gumball_active(&doc, &selected, true, &off));
}
