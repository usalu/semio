//! ✒️ Direct `change-schema` payload and behavior owner.

use crate::standards::v1::subsets::any::schema::{diff::PlaygroundDiff, mutations::PlaygroundMutation, snapshot::PlaygroundSnapshot};
use dsl::os_pack::json::{array, from_dsl_value, from_json_str, object, to_string, Value};

//#region 🔖️Mutation
/// ✒️ Changes the playground document's schema identity.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
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
    let snapshot = from_json_str(snapshot_json).map_err(|error| format!("the committed playground snapshot JSON does not decode: {error}"))?;
    let mutation = from_json_str(mutation_json).map_err(|error| format!("the committed playground mutation JSON does not decode: {error}"))?;
    Ok((snapshot, mutation))
}

fn bridge_step(snapshot: &PlaygroundSnapshot, mutation: &PlaygroundMutation) -> Result<(PlaygroundSnapshot, Vec<String>), String> {
    use protocol::{Mutation, MutationDiff};
    let outcome = <PlaygroundMutation as Mutation<PlaygroundSnapshot>>::diff(mutation, snapshot);
    let messages = outcome.messages().iter().map(|message| message.code.0.clone()).collect();
    MutationDiff::apply(outcome.diff(), snapshot).map(|next| (next, messages)).map_err(|error| format!("{error:?}"))
}

fn bridge_render(snapshot: &PlaygroundSnapshot, messages: Vec<String>) -> Result<String, String> {
    let value = object([("snapshot".to_string(), from_dsl_value(&dsl::ToValue::to_value(snapshot))), ("messages".to_string(), array(messages.into_iter().map(Value::String)))]);
    Ok(to_string(&value))
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
    let value = object([("printed".to_string(), Value::String(printed)), ("snapshot".to_string(), from_dsl_value(&dsl::ToValue::to_value(&parsed))), ("reparsed".to_string(), from_dsl_value(&dsl::ToValue::to_value(&reparsed)))]);
    Ok(to_string(&value))
}
//#endregion 🌉️ExternalCodecBridge

//#region 🧪️Behavior
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Behavior
