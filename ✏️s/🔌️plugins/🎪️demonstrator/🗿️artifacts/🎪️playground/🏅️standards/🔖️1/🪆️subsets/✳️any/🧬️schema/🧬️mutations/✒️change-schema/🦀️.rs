//! ✒️ Direct `change-schema` payload and behavior owner.

use crate::artifacts::playground::standards::v1::subsets::any::schema::{diff::PlaygroundDiff, mutations::PlaygroundMutation, snapshot::PlaygroundSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// ✒️ Changes the playground document's schema identity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSchema {
    pub new_schema: String,
}

impl protocol::MutationKind<PlaygroundSnapshot, PlaygroundMutation> for ChangeSchema {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "schema", kind: "change-schema", record: "ChangedSchema" };

    fn diff(&self, base: &PlaygroundSnapshot) -> protocol::MutationOutcome<PlaygroundDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &PlaygroundSnapshot) -> Vec<PlaygroundMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change playground schema to \"{}\"", self.new_schema)
    }

    fn target(&self) -> Vec<String> {
        vec!["schema".into()]
    }
}

/// 🏷️ Direct semantic roster exported for the language-neutral test adapter.
pub const KINDS: &[&str] = &["change-schema"];
//#endregion 🔖️Mutation

//#region 🌉️ExternalCodecBridge
fn bridge_decode_pair(snapshot_json: &str, mutation_json: &str) -> Result<(PlaygroundSnapshot, PlaygroundMutation), String> {
    let snapshot = serde_json::from_str(snapshot_json).map_err(|error| format!("the committed playground snapshot JSON does not decode: {error}"))?;
    let mutation = serde_json::from_str(mutation_json).map_err(|error| format!("the committed playground mutation JSON does not decode: {error}"))?;
    Ok((snapshot, mutation))
}

fn bridge_step(snapshot: &PlaygroundSnapshot, mutation: &PlaygroundMutation) -> Result<(PlaygroundSnapshot, Vec<String>), String> {
    use protocol::{Mutation, MutationDiff};
    let outcome = <PlaygroundMutation as Mutation<PlaygroundSnapshot>>::diff(mutation, snapshot);
    let messages = outcome.messages().iter().map(|message| message.code.0.clone()).collect();
    MutationDiff::apply(outcome.diff(), snapshot).map(|next| (next, messages)).map_err(|error| format!("{error:?}"))
}

fn bridge_render(snapshot: &PlaygroundSnapshot, messages: Vec<String>) -> Result<String, String> {
    serde_json::to_string(&serde_json::json!({ "snapshot": snapshot, "messages": messages })).map_err(|error| error.to_string())
}

/// 🌉️ Applies one committed language-neutral mutation payload to a playground snapshot.
pub fn apply_playground_mutation_json(snapshot_json: &str, mutation_json: &str) -> Result<String, String> {
    let (snapshot, mutation) = bridge_decode_pair(snapshot_json, mutation_json)?;
    let (applied, messages) = bridge_step(&snapshot, &mutation)?;
    bridge_render(&applied, messages)
}

/// ↩️ Applies one mutation and every step of its inverse plan.
pub fn undo_playground_mutation_json(snapshot_json: &str, mutation_json: &str) -> Result<String, String> {
    use protocol::Mutation;
    let (base, mutation) = bridge_decode_pair(snapshot_json, mutation_json)?;
    let (mut current, mut messages) = bridge_step(&base, &mutation)?;
    for undo in <PlaygroundMutation as Mutation<PlaygroundSnapshot>>::inverse(&mutation, &base) {
        let (next, raised) = bridge_step(&current, &undo)?;
        current = next;
        messages.extend(raised);
    }
    bridge_render(&current, messages)
}

/// 🔁️ Parses, prints, and reparses one language-neutral playground document.
pub fn round_trip_playground_dsl(text: &str) -> Result<String, String> {
    use store::ArtifactDsl;
    let parsed = <PlaygroundSnapshot as ArtifactDsl>::parse_dsl(text).map_err(|error| format!("the committed playground example does not parse: {error:?}"))?;
    let printed = <PlaygroundSnapshot as ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <PlaygroundSnapshot as ArtifactDsl>::parse_dsl(&printed).map_err(|error| format!("the reprinted playground document does not parse: {error:?}"))?;
    serde_json::to_string(&serde_json::json!({ "printed": printed, "snapshot": parsed, "reparsed": reparsed })).map_err(|error| error.to_string())
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️Behavior
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mutation, MutationDiff, SemanticMutation};

    fn mutation(schema: &str) -> PlaygroundMutation {
        PlaygroundMutation::ChangeSchema(ChangeSchema { new_schema: schema.into() })
    }

    #[test]
    fn committed_json_bridge_round_trips() {
        let before = include_str!("🧪️tests/retags-the-playground-document-schema/📸️snapshot/⬅️before/🔣️.json");
        let operation = include_str!("🧪️tests/retags-the-playground-document-schema/🦠️mutation/🔣️.json");
        let after = include_str!("🧪️tests/retags-the-playground-document-schema/📸️snapshot/➡️after/🔣️.json");
        let applied: serde_json::Value = serde_json::from_str(&apply_playground_mutation_json(before, operation).expect("apply committed mutation")).expect("decode bridge answer");
        let expected: serde_json::Value = serde_json::from_str(after).expect("decode committed after snapshot");
        assert_eq!(applied["snapshot"], expected);
        let undone: serde_json::Value = serde_json::from_str(&undo_playground_mutation_json(before, operation).expect("undo committed mutation")).expect("decode undo bridge answer");
        let expected: serde_json::Value = serde_json::from_str(before).expect("decode committed before snapshot");
        assert_eq!(undone["snapshot"], expected);
    }

    #[test]
    fn descriptor_inverse_and_outcome_are_complete() {
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
        protocol::os_spr::testkit::assert_outcome_deterministic(&base, &operation);
        let no_op = mutation("playground.base").diff(&base);
        assert_eq!(no_op.worst_level(), Some(protocol::os_dsl::Severity::Warning));
        assert!(no_op.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
    }

    #[test]
    fn inverse_and_absorb_laws_hold() {
        let base = PlaygroundSnapshot { schema: "playground.base".into() };
        let operation = mutation("playground.changed");
        protocol::os_spr::testkit::assert_mutation_inverse_law(&base, &operation);
        let first = operation.diff(&base).into_parts().0;
        let after = first.apply(&base).expect("valid mutation diff");
        let second = mutation("playground.changed-again").diff(&after).into_parts().0;
        protocol::os_spr::testkit::assert_mutation_diff_absorb_law(&base, first, second);
    }

    #[test]
    fn kinds_match_the_language_neutral_catalog() {
        let descriptors = PlaygroundMutation::kinds();
        assert_eq!(KINDS.len(), descriptors.len());
        let manifest = include_str!("../../../🧪️oracle/🔣️.json");
        for (kind, descriptor) in KINDS.iter().zip(descriptors.iter()) {
            assert_eq!(*kind, descriptor.kind);
            assert!(manifest.contains(&format!("\"{kind}\"")));
        }
    }
}
//#endregion 🧪️Behavior
