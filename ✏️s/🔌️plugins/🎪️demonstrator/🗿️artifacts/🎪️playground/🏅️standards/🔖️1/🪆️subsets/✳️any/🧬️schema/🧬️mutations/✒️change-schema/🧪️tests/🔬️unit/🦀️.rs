use super::*;
use protocol::{Mutation, MutationDiff, SemanticMutation};

fn mutation(schema: &str) -> PlaygroundMutation {
    PlaygroundMutation::ChangeSchema(ChangeSchema { new_schema: schema.into() })
}

#[test]
fn committed_json_bridge_round_trips() {
    let before = include_str!("../📅️retags-the-4f474c/📸️snapshot/⬅️before/🔣️.json");
    let operation = include_str!("../📅️retags-the-4f474c/🦠️mutation/🔣️.json");
    let after = include_str!("../📅️retags-the-4f474c/📸️snapshot/➡️after/🔣️.json");
    let applied: serde_json::Value = serde_json::from_str(&apply_playground_mutation_json(before, operation).expect("apply committed mutation")).expect("decode bridge answer");
    let expected: serde_json::Value = serde_json::from_str(after).expect("decode committed after snapshot");
    assert_eq!(applied["snapshot"], expected);
    let undone: serde_json::Value = serde_json::from_str(&undo_playground_mutation_json(before, operation).expect("undo committed mutation")).expect("decode undo bridge answer");
    let expected: serde_json::Value = serde_json::from_str(before).expect("decode committed before snapshot");
    assert_eq!(undone["snapshot"], expected);
}

#[semio_framework_async_macros::async_test]
async fn descriptor_inverse_and_outcome_are_complete() {
    let base = PlaygroundSnapshot { schema: "playground.base".into() };
    let operation = mutation("playground.changed");
    assert_eq!(operation.semantics().kind, "change-schema");
    assert_eq!(operation.semantics().record, "ChangedSchema");
    assert_eq!(PlaygroundMutation::kinds().len(), KINDS.len());
    let after = operation.diff(&base).diff().apply(&base).expect("valid mutation diff");
    assert_eq!(after.schema, "playground.changed");
    let mut restored = after;
    for back in operation.inverse(&base) {
        restored = back.diff(&restored).diff().apply(&restored).expect("valid inverse diff");
    }
    assert_eq!(restored, base);
    protocol::os_spr::testkit::assert_outcome_deterministic(&base, &operation).await;
    let no_op = mutation("playground.base").diff(&base);
    assert_eq!(no_op.worst_level(), Some(protocol::os_dsl::Severity::Warning));
    assert!(no_op.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
}

#[semio_framework_async_macros::async_test]
async fn inverse_and_absorb_laws_hold() {
    let base = PlaygroundSnapshot { schema: "playground.base".into() };
    let operation = mutation("playground.changed");
    protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &operation).await;
    let first = operation.diff(&base).into_parts().0;
    let after = first.apply(&base).expect("valid mutation diff");
    let second = mutation("playground.changed-again").diff(&after).into_parts().0;
    protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, first, second).await;
}

#[test]
fn kinds_match_the_language_neutral_catalog() {
    let descriptors = PlaygroundMutation::kinds();
    assert_eq!(KINDS.len(), descriptors.len());
    let manifest = include_str!("../../../../../🔮️oracle/🔣️.json");
    for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
        assert_eq!(*kind, descriptor.kind);
        assert!(manifest.contains(&format!("\"{kind}\"")));
    }
}
