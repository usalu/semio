//! 🖼️ Render-mode payload and sparse configuration change.

use super::super::{Gis2dConfig, Gis2dConfigDelta, Gis2dConfigDiff, Gis2dConfigMutation};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-render-mode")]
pub struct SetRenderMode { pub value: String }
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<Gis2dConfig, Gis2dConfigMutation> for SetRenderMode {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "render-mode", kind: "set-render-mode", record: "SetRenderMode" };
    fn diff(&self, base: &Gis2dConfig) -> MutationOutcome<Gis2dConfigDiff> {
        if base.render_mode == self.value { return MutationOutcome::empty().warn("mutation.no-op", format!("Render mode is already \"{}\".", self.value)); }
        MutationOutcome::new(Gis2dConfigDelta { render_mode: Some(self.value.clone()), ..Default::default() }.into())
    }
    fn inverse(&self, base: &Gis2dConfig) -> Vec<Gis2dConfigMutation> { vec![Self { value: base.render_mode.clone() }.into()] }
    fn label(&self) -> String { "Set render mode".into() }
    fn target(&self) -> Vec<String> { vec!["renderMode".into()] }
}
//#endregion ⚙️Behavior

//#region 🧪️Contracts
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_payload_metadata_codecs_and_inverse_match_the_neutral_fixture() {
        super::super::super::direct_mutation_tests::assert_leaf::<SetRenderMode>(3, Gis2dConfigMutation::SetRenderMode, include_str!("🔣️.json"));
    }
}
//#endregion 🧪️Contracts
