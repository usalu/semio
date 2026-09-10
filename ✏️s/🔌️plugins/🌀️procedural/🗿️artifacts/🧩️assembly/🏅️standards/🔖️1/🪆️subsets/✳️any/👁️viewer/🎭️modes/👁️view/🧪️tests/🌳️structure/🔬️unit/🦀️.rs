
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
    let tree = render(&document).expect("the assembly tree window assembles");
    assert_eq!(tree.key.as_str(), WINDOW_KIND_ID);
    let root = &tree.children[0].children[0];
    assert!(root.children.iter().any(|item| item.key.as_str() == "slots"));
    assert!(root.children.iter().any(|item| item.key.as_str() == "rules"));
}
