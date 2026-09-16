use super::*;
use crate::editor::fem2d::interaction::canvas_gesture::FEM2D_UTILITY_TRANSFORM;
use crate::Viewport2d;
use store::ArtifactDsl;

type Fem2dSnapshot = crate::Fem2dSnapshot;

fn demo() -> Fem2dSnapshot {
    Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).expect("demo document parses")
}

#[test]
fn gumball_meta_layer_emitted_when_transform_utility_and_selection() {
    let doc = demo();
    let camera = Viewport2d::default();
    let meta = fem2d_gumball_meta_layer(&doc, &["n1".into()], &camera, FEM2D_UTILITY_TRANSFORM, Some("w")).expect("meta");
    assert_eq!(meta.get("role").and_then(|value| value.as_str()), Some("meta"));
    assert!(meta.get("gumball").is_some());
    assert_eq!(meta.pointer("/gumball/config/moveAxes").and_then(|value| value.as_bool()), Some(true));
}

#[test]
fn gumball_meta_absent_without_selection() {
    let doc = demo();
    let camera = Viewport2d::default();
    assert!(fem2d_gumball_meta_layer(&doc, &[], &camera, FEM2D_UTILITY_TRANSFORM, None).is_none());
}

#[test]
fn translate_selection_moves_node_coordinates() {
    let doc = demo();
    let mutations = fem2d_translate_selection_mutations(&doc, &["n1".into()], 1.0, 2.0);
    assert_eq!(mutations.len(), 1);
    let crate::standards::v1::subsets::any::schema::mutations::text::Fem2dMutation::ReplaceNode(replace) = &mutations[0] else { panic!("replace node") };
    assert_eq!(replace.id, "n1");
    let n1 = doc.nodes.iter().find(|node| node.id == "n1").expect("n1");
    assert!((replace.new_node.x - (n1.x + 1.0)).abs() < 1e-9);
    assert!((replace.new_node.y - (n1.y + 2.0)).abs() < 1e-9);
}
