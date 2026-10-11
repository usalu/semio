//! 🎨️ Vector-style payload and sparse configuration change.

use super::super::{MapWindowConfig, MapWindowConfigDelta, MapWindowConfigDiff, MapWindowConfigMutation};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
            return MutationOutcome::empty().warning("mutation.no-op", format!("Vector style is already \"{}\".", self.value));
        }
        MutationOutcome::new(MapWindowConfigDelta { vector_style: Some(self.value.clone()), ..Default::default() }.into())
    }
    fn inverse(&self, base: &MapWindowConfig) -> Result<Vec<MapWindowConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { value: base.vector_style.clone() }.into()]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set vector style", "Vektorstil setzen")
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
