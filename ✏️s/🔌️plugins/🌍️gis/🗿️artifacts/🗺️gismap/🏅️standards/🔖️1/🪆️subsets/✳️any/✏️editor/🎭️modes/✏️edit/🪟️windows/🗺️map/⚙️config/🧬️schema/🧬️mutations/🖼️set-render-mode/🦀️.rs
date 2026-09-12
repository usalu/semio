//! 🖼️ Render-mode payload and sparse configuration change.

use super::super::{MapWindowConfig, MapWindowConfigDelta, MapWindowConfigDiff, MapWindowConfigMutation};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-render-mode")]
pub struct SetRenderMode {
    pub value: String,
}
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<MapWindowConfig, MapWindowConfigMutation> for SetRenderMode {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "render-mode", kind: "set-render-mode", record: "SetRenderMode" };
    fn diff(&self, base: &MapWindowConfig) -> MutationOutcome<MapWindowConfigDiff> {
        if base.render_mode == self.value {
            return MutationOutcome::empty().warn("mutation.no-op", format!("Render mode is already \"{}\".", self.value));
        }
        MutationOutcome::new(MapWindowConfigDelta { render_mode: Some(self.value.clone()), ..Default::default() }.into())
    }
    fn inverse(&self, base: &MapWindowConfig) -> Vec<MapWindowConfigMutation> {
        vec![Self { value: base.render_mode.clone() }.into()]
    }
    fn label(&self) -> String {
        "Set render mode".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["renderMode".into()]
    }
}
//#endregion ⚙️Behavior

//#region 🧪️Contracts
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Contracts
