//! 🔽️ Level-of-detail payload and sparse configuration change.

use super::super::{Gis2dConfig, Gis2dConfigDelta, Gis2dConfigDiff, Gis2dConfigMutation};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-lod-mode")]
pub struct SetLodMode { pub value: String }
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<Gis2dConfig, Gis2dConfigMutation> for SetLodMode {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "lod-mode", kind: "set-lod-mode", record: "SetLodMode" };
    fn diff(&self, base: &Gis2dConfig) -> MutationOutcome<Gis2dConfigDiff> {
        if base.lod_mode == self.value { return MutationOutcome::empty().warn("mutation.no-op", format!("LOD mode is already \"{}\".", self.value)); }
        MutationOutcome::new(Gis2dConfigDelta { lod_mode: Some(self.value.clone()), ..Default::default() }.into())
    }
    fn inverse(&self, base: &Gis2dConfig) -> Vec<Gis2dConfigMutation> { vec![Self { value: base.lod_mode.clone() }.into()] }
    fn label(&self) -> String { "Set LOD mode".into() }
    fn target(&self) -> Vec<String> { vec!["lodMode".into()] }
}
//#endregion ⚙️Behavior

//#region 🧪️Contracts
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_payload_metadata_codecs_and_inverse_match_the_neutral_fixture() {
        super::super::super::direct_mutation_tests::assert_leaf::<SetLodMode>(5, Gis2dConfigMutation::SetLodMode, include_str!("🔣️.json"));
    }
}
//#endregion 🧪️Contracts
