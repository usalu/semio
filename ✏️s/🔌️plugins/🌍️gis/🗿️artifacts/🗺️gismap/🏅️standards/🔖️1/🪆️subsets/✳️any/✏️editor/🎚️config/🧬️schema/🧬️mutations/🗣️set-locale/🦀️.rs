//! 🗣️ Locale payload and sparse configuration change.

use super::super::{Gis2dConfig, Gis2dConfigDelta, Gis2dConfigDiff, Gis2dConfigMutation};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-locale")]
pub struct SetLocale { pub value: String }
//#endregion 🧬️Payload

//#region ⚙️Behavior
impl MutationKind<Gis2dConfig, Gis2dConfigMutation> for SetLocale {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "locale", kind: "set-locale", record: "SetLocale" };
    fn diff(&self, base: &Gis2dConfig) -> MutationOutcome<Gis2dConfigDiff> {
        if base.locale == self.value { return MutationOutcome::empty().warn("mutation.no-op", format!("Locale is already \"{}\".", self.value)); }
        MutationOutcome::new(Gis2dConfigDelta { locale: Some(self.value.clone()), ..Default::default() }.into())
    }
    fn inverse(&self, base: &Gis2dConfig) -> Vec<Gis2dConfigMutation> { vec![Self { value: base.locale.clone() }.into()] }
    fn label(&self) -> String { "Set locale".into() }
    fn target(&self) -> Vec<String> { vec!["locale".into()] }
}
//#endregion ⚙️Behavior

//#region 🧪️Contracts
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_payload_metadata_codecs_and_inverse_match_the_neutral_fixture() {
        super::super::super::direct_mutation_tests::assert_leaf::<SetLocale>(8, Gis2dConfigMutation::SetLocale, include_str!("🔣️.json"));
    }
}
//#endregion 🧪️Contracts
