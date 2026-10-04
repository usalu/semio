//! 🧊️ `add-geometry` — brings a new id-keyed parametric geometry definition into existence.

use crate::{ParametricGeometry, Vdi3805Mutation, Vdi3805Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct AddGeometry {
    pub geometry: ParametricGeometry,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for AddGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "add", entity: "geometry", kind: "add-geometry", record: "AddedGeometry" };

    fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Vdi3805Snapshot) -> Result<Vec<Vdi3805Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create geometry \"{}\"", self.geometry.id), &format!("Geometrie \"{}\" erstellen", self.geometry.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.geometry.id.clone()]
    }
}
//#endregion 🔖️Payload
