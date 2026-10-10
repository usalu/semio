use super::*;
use crate::editor::bim::unit_tests::support::{ctx, demo, run};
use crate::ClassificationItem;

fn system() -> ClassificationSystem {
    let entry = |code: &str, title: &str, parent: Option<&str>| ClassificationItem { code: code.into(), title: title.into(), parent: parent.map(str::to_string) };
    ClassificationSystem { name: "Uniclass".into(), edition: String::new(), source: None, entries: vec![entry("EF_25", "Walls and barriers", None), entry("EF_25_10", "Walls", Some("EF_25")), entry("Pr_20", "Structure", None)] }
}

#[semio_framework_async_macros::async_test]
async fn the_matches_are_the_entries_whose_code_or_title_contains_the_text_without_regard_to_case() {
    let table = system();
    assert_eq!(matches(&table, "walls"), ["EF_25", "EF_25_10"]);
    assert_eq!(matches(&table, " PR_ "), ["Pr_20"]);
    assert_eq!(matches(&table, "zzz"), Vec::<String>::new());
    assert_eq!(matches(&table, "").len(), 3, "an empty text matches every entry");
    assert_eq!(entry_id("cs-uni", "EF_25"), "cs-uni:EF_25");
}

#[semio_framework_async_macros::async_test]
async fn a_search_selects_the_matches_in_the_library_domain_and_an_empty_one_clears_them() {
    let mut snapshot = demo();
    snapshot.classification_systems.insert("cs-uni".into(), system());
    let mut context = ctx(&[]);
    let search = |query: &str, context: &mut BimDispatchCtx| run(&snapshot, |doc, cfg| handle(&SearchClassification { system: "cs-uni".into(), query: query.into() }, doc, cfg, context));
    let found = search("walls", &mut context).expect("selects");
    assert!(found.artifact_mutations.is_empty() && found.effects.len() == 1, "selection is framework state, no document byte");
    let cleared = search("  ", &mut context).expect("clears");
    assert!(cleared.artifact_mutations.is_empty() && cleared.effects.len() == 1);
    assert_eq!(search("zzz", &mut context).err().map(|fault| fault.code.0), Some("bim.classification.no-match".to_string()));
    let missing = run(&snapshot, |doc, cfg| handle(&SearchClassification { system: "cs-none".into(), query: "a".into() }, doc, cfg, &mut context));
    assert_eq!(missing.err().map(|fault| fault.code.0), Some("bim.classification.system-missing".to_string()));
}
