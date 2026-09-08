
use super::*;

#[test]
fn definition_declares_a_tree_window() {
    let def = definition();
    assert_eq!(def.id, WINDOW_KIND_ID);
    assert_eq!(def.body_key, BODY_KEY);
}

#[test]
fn render_lists_every_collection_branch() {
    let mut document = AssemblySnapshot::default();
    document.slots.push(AssemblySlot { id: "s1".into(), x: 1.0, y: 2.0, z: 0.0, pinned_module_id: None });
    document.rules.push(AssemblyRule { id: "r1".into(), module_a_id: "a".into(), module_b_id: "b".into(), allowed: true, ..Default::default() });
    let UiNode::Tree(node) = render(&document) else { panic!("expected Tree") };
    let root = &node.sections[0].items[0];
    let root_children = root.items.as_ref().expect("root has children");
    assert!(root_children.iter().any(|item| item.id == "slots"));
    assert!(root_children.iter().any(|item| item.id == "rules"));
}
