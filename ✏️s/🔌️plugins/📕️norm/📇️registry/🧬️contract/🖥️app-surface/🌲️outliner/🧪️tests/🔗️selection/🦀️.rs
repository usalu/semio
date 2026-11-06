//! 🎯️ Outliner and selection-mirror integration at their common inspection owner.

use crate::*;
use semio_framework_value::DslValue;

fn entity(id: &str, name: Option<&str>) -> DslValue {
    let mut entries = vec![("id".to_string(), DslValue::String(id.into()))];
    if let Some(name) = name { entries.push(("name".into(), DslValue::String(name.into()))); }
    DslValue::Object(entries)
}

fn project(node: BuiltNode) -> String {
    semio_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_plugin::ComponentTree::new(node)).expect("projection")
}

#[test]
fn rows_of_an_interaction_domain_are_scoped_pick_targets_the_selection_mirror_resolves() {
    let tree = DslValue::object([("members".into(), DslValue::Array(vec![entity("b1", Some("Main beam")), entity("c2", None)])), ("loads".into(), DslValue::Array(vec![entity("b1", None)]))]);
    let entities = [OutlinedCollection { path: "members", label_en: "Members", label_de: "Stäbe" }, OutlinedCollection { path: "loads", label_en: "Loads", label_de: "Lasten" }];
    let state = OutlinerState { interaction_domain: Some("norm.mock"), ..Default::default() };
    let windows = TreeWindows::unhosted();
    let report = CheckReport::default();
    let context = OutlinerContext { report: &report, locale: Locale::En, controller_id: "norm.mock", windows: &windows, state: &state };
    let json = project(render_outliner_document(&tree, &entities, None, &context).expect("outliner"));
    let member = crate::protocol::wire::encode_scoped_identity(&["members", "b1"]).expect("address");
    let load = crate::protocol::wire::encode_scoped_identity(&["loads", "b1"]).expect("address");
    assert_ne!(member, load, "equal ids of different collections stay distinct");
    assert!(json.contains(&member) && json.contains(&load) && json.contains("\"entity\""), "{json}");
    let row = EntityRow { path: "members[id=b1]".into(), id: "b1".into(), title: "b1".into(), index: 0, value: &entity("b1", None) };
    assert_eq!(row_address(&row), Some(member.clone()));
    let picked = NormSelection::resolve(&report, None, &[member]);
    assert_eq!(picked.entities, ["b1"], "the mirror resolves the scoped row address to the entity");
    assert_eq!(picked.outliner_state_in("norm.mock").interaction_domain, Some("norm.mock"));
}

