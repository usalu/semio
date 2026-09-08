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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Contracts
