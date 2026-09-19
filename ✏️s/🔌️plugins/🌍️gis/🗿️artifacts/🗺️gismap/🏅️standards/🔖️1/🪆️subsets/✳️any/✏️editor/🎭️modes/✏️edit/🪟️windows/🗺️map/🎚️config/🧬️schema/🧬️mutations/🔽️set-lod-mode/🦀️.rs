//! 🔽️ Level-of-detail payload and sparse configuration change.

use super::super::{MapWindowConfig, MapWindowConfigDelta, MapWindowConfigDiff, MapWindowConfigMutation};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-lod-mode")]
pub struct SetLodMode {
    pub value: String,
}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<MapWindowConfig, MapWindowConfigMutation> for SetLodMode {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "lod-mode", kind: "set-lod-mode", record: "SetLodMode" };
    fn diff(&self, base: &MapWindowConfig) -> MutationOutcome<MapWindowConfigDiff> {
        if base.lod_mode == self.value {
            return MutationOutcome::empty().warn("mutation.no-op", format!("LOD mode is already \"{}\".", self.value));
        }
        MutationOutcome::new(MapWindowConfigDelta { lod_mode: Some(self.value.clone()), ..Default::default() }.into())
    }
    fn inverse(&self, base: &MapWindowConfig) -> Vec<MapWindowConfigMutation> {
        vec![Self { value: base.lod_mode.clone() }.into()]
    }
    fn label(&self) -> String {
        "Set LOD mode".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["lodMode".into()]
    }
}
//#endregion ⚙️Behavior

//#region 🧪️Contracts
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Contracts
