use super::*;
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, run};
use crate::ClassificationSystem;

fn edit(op: &str, code: &str, title: &str, parent: &str) -> EditClassification {
    EditClassification { id: "cs-uni".into(), op: op.into(), code: code.into(), title: title.into(), parent: parent.into() }
}

fn table() -> Vec<ClassificationItem> {
    let list = apply(&[], &edit("add", "Pr", "Products", "")).expect("a root");
    apply(&list, &edit("add", "Pr_20", "Structure", "Pr")).expect("a child")
}

#[semio_framework_async_macros::async_test]
async fn entries_are_added_retitled_moved_and_removed() {
    let list = table();
    assert_eq!(list.iter().map(|entry| (entry.code.as_str(), entry.parent.as_deref())).collect::<Vec<_>>(), [("Pr", None), ("Pr_20", Some("Pr"))]);
    let titled = apply(&list, &edit("retitle", "Pr_20", "Structure and general products", "")).expect("retitles");
    assert_eq!(titled[1].title, "Structure and general products");
    let moved = apply(&titled, &edit("reparent", "Pr_20", "", "")).expect("moves to the root");
    assert_eq!(moved[1].parent, None);
    let removed = apply(&moved, &edit("remove", "Pr_20", "", "")).expect("removes a leaf");
    assert_eq!(removed.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn a_code_that_exists_a_code_that_does_not_and_an_entry_with_children_are_refused() {
    let list = table();
    assert_eq!(apply(&list, &edit("add", "Pr", "Again", "")), Err("bim.classification.edit-invalid"));
    assert_eq!(apply(&list, &edit("add", "", "Nothing", "")), Err("bim.classification.edit-invalid"));
    assert_eq!(apply(&list, &edit("retitle", "Zz", "Nothing", "")), Err("bim.classification.edit-invalid"));
    assert_eq!(apply(&list, &edit("remove", "Pr", "", "")), Err("bim.classification.entry-has-children"));
    assert_eq!(apply(&list, &edit("frobnicate", "Pr", "", "")), Err("bim.classification.edit-invalid"));
}

#[semio_framework_async_macros::async_test]
async fn the_typed_line_of_the_panel_carries_code_title_and_parent() {
    let add = normalised(&edit("add", "", "Pr_20_93 | Structural units | Pr_20", ""));
    assert_eq!((add.code.as_str(), add.title.as_str(), add.parent.as_str()), ("Pr_20_93", "Structural units", "Pr_20"));
    let retitle = normalised(&edit("retitle", "", "Pr_20 | Structure", ""));
    assert_eq!((retitle.code.as_str(), retitle.title.as_str()), ("Pr_20", "Structure"));
    let reparent = normalised(&edit("reparent", "", "Pr_20 | Pr", ""));
    assert_eq!((reparent.code.as_str(), reparent.parent.as_str()), ("Pr_20", "Pr"));
    let remove = normalised(&edit("remove", "", " Pr_20 ", ""));
    assert_eq!(remove.code, "Pr_20");
    assert_eq!(normalised(&edit("add", "Pr", "Products", "")), edit("add", "Pr", "Products", ""), "an explicit code is kept");
}

#[semio_framework_async_macros::async_test]
async fn the_command_becomes_one_set_classification_system_mutation() {
    let mut snapshot = demo();
    snapshot.classification_systems.insert("cs-uni".into(), ClassificationSystem { name: "Uniclass".into(), edition: String::new(), source: None, entries: Vec::new() });
    let mut context = ctx(&[]);
    let emit = run(&snapshot, |doc, cfg| handle(&edit("add", "", "Pr | Products | ", ""), doc, cfg, &mut context)).expect("adds an entry from the typed line");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::SetClassificationSystem(_)]));
    assert_eq!(applied(&snapshot, &emit).classification_systems["cs-uni"].entries.len(), 1);
    let missing = EditClassification { id: "cs-none".into(), ..edit("add", "Pr", "Products", "") };
    assert_eq!(run(&snapshot, |doc, cfg| handle(&missing, doc, cfg, &mut context)).err().map(|fault| fault.code.0), Some("bim.classification.system-missing".to_string()));
}
