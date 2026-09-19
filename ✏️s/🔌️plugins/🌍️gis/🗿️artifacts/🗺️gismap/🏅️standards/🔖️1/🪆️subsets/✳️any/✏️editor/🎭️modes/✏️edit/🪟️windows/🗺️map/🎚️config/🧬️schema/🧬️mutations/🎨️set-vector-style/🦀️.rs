//! 🎨️ Vector-style payload and sparse configuration change.

use super::super::{MapWindowConfig, MapWindowConfigDelta, MapWindowConfigDiff, MapWindowConfigMutation};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-vector-style")]
pub struct SetVectorStyle {
    pub value: String,
}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<MapWindowConfig, MapWindowConfigMutation> for SetVectorStyle {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "vector-style", kind: "set-vector-style", record: "SetVectorStyle" };
    fn diff(&self, base: &MapWindowConfig) -> MutationOutcome<MapWindowConfigDiff> {
        if base.vector_style == self.value {
            return MutationOutcome::empty().warn("mutation.no-op", format!("Vector style is already \"{}\".", self.value));
        }
        MutationOutcome::new(MapWindowConfigDelta { vector_style: Some(self.value.clone()), ..Default::default() }.into())
    }
    fn inverse(&self, base: &MapWindowConfig) -> Vec<MapWindowConfigMutation> {
        vec![Self { value: base.vector_style.clone() }.into()]
    }
    fn label(&self) -> String {
        "Set vector style".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["vectorStyle".into()]
    }
}
//#endregion ⚙️Behavior

//#region 🧪️Contracts
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Contracts
