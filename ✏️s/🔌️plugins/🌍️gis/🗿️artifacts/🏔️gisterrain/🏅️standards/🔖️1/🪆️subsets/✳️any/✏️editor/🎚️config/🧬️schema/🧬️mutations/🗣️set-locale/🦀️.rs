//! 🗣️ Locale payload and sparse GIS 3D configuration behavior.
use super::super::{Gis3dConfig, Gis3dConfigDelta, Gis3dConfigDiff, Gis3dConfigMutation};
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🧬️Payload
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(keyword = "set-locale")]
pub struct SetLocale { pub value: String }
//#endregion 🧬️Payload
//#region ⚙️Behavior
impl MutationKind<Gis3dConfig, Gis3dConfigMutation> for SetLocale {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "locale", kind: "set-locale", record: "SetLocale" };
    fn diff(&self, base: &Gis3dConfig) -> MutationOutcome<Gis3dConfigDiff> { if base.locale == self.value { MutationOutcome::empty().warn("mutation.no-op", format!("Locale is already \"{}\".", self.value)) } else { MutationOutcome::new(Gis3dConfigDelta { locale: Some(self.value.clone()), ..Default::default() }.into()) } }
    fn inverse(&self, base: &Gis3dConfig) -> Vec<Gis3dConfigMutation> { vec![Self { value: base.locale.clone() }.into()] }
    fn label(&self) -> String { "Set locale".into() }
    fn target(&self) -> Vec<String> { vec!["locale".into()] }
}
//#endregion ⚙️Behavior
//#region 🧪️Contracts
#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{Mutation, MutationDiff};
    #[test]
    fn direct_leaf_neutral_schema_codec_and_outcome_laws() {
        super::super::super::direct_leaf_contracts::assert_leaf_contract::<SetLocale>("locale", include_str!("🔣️.json"));
    }
    #[test]
    fn sparse_locale_inverse_and_codecs_preserve_camera() {
        let base = Gis3dConfig { camera_json: "default".into(), locale: "en-US".into() };
        let mutation = Gis3dConfigMutation::SetLocale(SetLocale { value: "de-DE".into() });
        let next = mutation.diff(&base).diff().apply(&base).expect("apply");
        assert_eq!(next.camera_json, base.camera_json);
        assert_eq!(mutation.inverse(&base)[0].diff(&next).diff().apply(&next).expect("inverse"), base);
        store::os_store::test_support::assert_op_line_round_trip(&mutation);
    }
}
//#endregion 🧪️Contracts
