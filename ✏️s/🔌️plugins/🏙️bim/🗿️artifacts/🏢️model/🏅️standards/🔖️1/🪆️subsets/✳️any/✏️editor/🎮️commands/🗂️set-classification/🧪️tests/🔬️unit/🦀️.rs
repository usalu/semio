use super::*;
use crate::editor::bim::unit_tests::support::{applied, ctx, demo, run};
use crate::{ClassificationItem, ClassificationSystem};

fn with_system() -> ModelSnapshot {
    let mut snapshot = demo();
    snapshot.classification_systems.insert("cs-uni".into(), ClassificationSystem { name: "Uniclass 2015".into(), edition: "2024".into(), source: None, entries: vec![ClassificationItem { code: "EF_25_10".into(), title: "Walls".into(), parent: None }] });
    snapshot
}

fn classify(snapshot: &ModelSnapshot, ids: &[&str], system: &str, code: &str, selected: &[&str]) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let mut ctx = ctx(selected);
    run(snapshot, |doc, cfg| handle(&SetClassification { ids: ids.iter().map(|id| id.to_string()).collect(), system: system.into(), code: code.into() }, doc, cfg, &mut ctx))
}

#[semio_framework_async_macros::async_test]
async fn classifying_sets_the_code_in_the_system_on_each_target() {
    let snapshot = with_system();
    let emit = classify(&snapshot, &["w-south", "w-east"], " cs-uni ", " EF_25_10 ", &[]).expect("classifies");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::SetElementClassification(_), ModelMutation::SetElementClassification(_)]));
    let after = applied(&snapshot, &emit);
    for id in ["w-south", "w-east"] {
        assert_eq!(after.classifications[id].get("cs-uni").map(String::as_str), Some("EF_25_10"));
    }
}

#[semio_framework_async_macros::async_test]
async fn the_selection_is_classified_when_no_id_is_given_and_an_unchanged_classification_is_skipped() {
    let snapshot = with_system();
    let first = classify(&snapshot, &[], "cs-uni", "EF_25", &["w-south"]).expect("classifies the selection");
    let after = applied(&snapshot, &first);
    assert!(after.classifications.contains_key("w-south"));
    let again = classify(&after, &["w-south", "w-east"], "cs-uni", "EF_25", &[]).expect("only the unclassified one changes");
    assert_eq!(again.artifact_mutations.len(), 1);
    assert!(classify(&after, &["w-south"], "cs-uni", "EF_25", &[]).expect("nothing to do").artifact_mutations.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn a_type_is_a_holder_and_a_missing_target_system_or_code_is_refused() {
    let snapshot = with_system();
    let code = |result: Result<Emit<ModelMutation, NoConfigMutation>, Fault>| result.err().map(|fault| fault.code.0);
    let typed = classify(&snapshot, &["wt-300"], "cs-uni", "EF_25_10", &[]).expect("a type carries classifications");
    assert_eq!(applied(&snapshot, &typed).classifications["wt-300"]["cs-uni"], "EF_25_10");
    assert_eq!(code(classify(&snapshot, &["w-south"], "", "EF_25", &[])), Some("bim.classification.invalid".to_string()));
    assert_eq!(code(classify(&snapshot, &["w-south"], "cs-uni", "  ", &[])), Some("bim.classification.invalid".to_string()));
    assert_eq!(code(classify(&snapshot, &["nobody"], "cs-uni", "EF_25", &[])), Some("bim.classification.target-missing".to_string()));
    assert_eq!(code(classify(&snapshot, &["w-south"], "cs-none", "EF_25", &[])), Some("bim.classification.system-missing".to_string()));
}
