//! ✒️ Direct `change-schema` payload and behavior owner.

use crate::standards::v1::subsets::any::schema::{diff::PlaygroundDiff, mutations::PlaygroundMutation, snapshot::PlaygroundSnapshot};

//#region 🔖️Mutation
/// ✒️ Changes the playground document's schema identity.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeSchema {
    pub new_schema: String,
}

impl protocol::MutationKind<PlaygroundSnapshot, PlaygroundMutation> for ChangeSchema {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "schema", kind: "change-schema", record: "ChangedSchema" };

    fn diff(&self, base: &PlaygroundSnapshot) -> protocol::MutationOutcome<PlaygroundDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &PlaygroundSnapshot) -> Result<Vec<PlaygroundMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change playground schema to \"{}\"", self.new_schema), &format!("Playground-Schema auf \"{}\" ändern", self.new_schema))
    }

    fn target(&self) -> Vec<String> {
        vec!["schema".into()]
    }
}

/// 🏷️ Direct semantic roster exported for the language-neutral test adapter.
pub const KINDS: &[&str] = &["change-schema"];
//#endregion 🔖️Mutation

//#region 🌉️ExternalCodecBridge










//#endregion 🌉️ExternalCodecBridge

//#region 🧪️Behavior
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Behavior
