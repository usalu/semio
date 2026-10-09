use super::*;
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, run};
use crate::{ClassificationItem, ClassificationSystem};
use std::collections::BTreeMap;

fn classified() -> ModelSnapshot {
    let mut snapshot = demo();
    let table = |name: &str, code: &str| ClassificationSystem { name: name.into(), edition: String::new(), source: None, entries: vec![ClassificationItem { code: code.into(), title: String::new(), parent: None }] };
    snapshot.classification_systems.insert("cs-uni".into(), table("Uniclass", "Ss_25"));
    snapshot.classification_systems.insert("cs-din".into(), table("DIN 276", "331"));
    snapshot.classifications.insert("w-south".into(), BTreeMap::from([("cs-uni".to_string(), "Ss_25".to_string()), ("cs-din".to_string(), "331".to_string())]));
    snapshot
}

fn remove(snapshot: &ModelSnapshot, ids: &[&str], system: &str, selected: &[&str]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let mut ctx = ctx(selected);
    run(snapshot, |doc, cfg| handle(&RemoveClassification { ids: ids.iter().map(|id| id.to_string()).collect(), system: system.into() }, doc, cfg, &mut ctx))
}

#[semio_framework_async_macros::async_test]
async fn removing_drops_one_system_of_the_targets_and_skips_the_unclassified() {
    let snapshot = classified();
    let emit = remove(&snapshot, &["w-south", "w-east"], "cs-uni", &[]).expect("removes from the one that has it");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::RemoveElementClassification(_)]));
    let after = applied(&snapshot, &emit);
    assert_eq!(after.classifications["w-south"].keys().collect::<Vec<_>>(), ["cs-din"]);
    let rest = remove(&after, &["w-south"], "cs-din", &[]).expect("removes the last");
    assert!(!applied(&after, &rest).classifications.contains_key("w-south"));
}

#[semio_framework_async_macros::async_test]
async fn the_selection_is_used_without_ids_and_none_classified_is_refused() {
    let snapshot = classified();
    assert!(remove(&snapshot, &[], "cs-uni", &["w-south"]).is_ok());
    assert_eq!(remove(&snapshot, &["w-east"], "cs-uni", &[]).err().map(|fault| fault.code.0), Some("bim.classification.missing".to_string()));
    assert_eq!(remove(&snapshot, &[], "cs-uni", &[]).err().map(|fault| fault.code.0), Some("bim.classification.missing".to_string()));
}
