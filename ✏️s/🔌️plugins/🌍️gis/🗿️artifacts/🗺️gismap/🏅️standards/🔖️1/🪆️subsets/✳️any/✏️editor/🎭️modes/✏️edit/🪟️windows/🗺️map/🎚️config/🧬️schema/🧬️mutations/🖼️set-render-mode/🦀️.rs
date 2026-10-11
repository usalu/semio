//! 🖼️ Render-mode payload and sparse configuration change.

use super::super::{MapWindowConfig, MapWindowConfigDelta, MapWindowConfigDiff, MapWindowConfigMutation};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
            return MutationOutcome::empty().warning("mutation.no-op", format!("Render mode is already \"{}\".", self.value));
        }
        MutationOutcome::new(MapWindowConfigDelta { render_mode: Some(self.value.clone()), ..Default::default() }.into())
    }
    fn inverse(&self, base: &MapWindowConfig) -> Result<Vec<MapWindowConfigMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { value: base.render_mode.clone() }.into()]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set render mode", "Darstellungsmodus setzen")
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
