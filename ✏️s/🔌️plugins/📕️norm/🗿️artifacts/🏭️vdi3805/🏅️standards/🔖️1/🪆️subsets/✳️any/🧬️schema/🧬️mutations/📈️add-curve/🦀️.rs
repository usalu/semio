//! 📈️ `add-curve` — brings a new id-keyed characteristic curve into existence.

use crate::{CharacteristicCurve, Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct AddCurve {
    pub curve: CharacteristicCurve,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for AddCurve {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "curve", kind: "add-curve", record: "AddedCurve" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Result<Vec<Vdi3805Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create curve \"{}\"", self.curve.id), &format!("Kurve \"{}\" erstellen", self.curve.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.curve.id.clone()]
    }
}
//#endregion 🔖️Payload
