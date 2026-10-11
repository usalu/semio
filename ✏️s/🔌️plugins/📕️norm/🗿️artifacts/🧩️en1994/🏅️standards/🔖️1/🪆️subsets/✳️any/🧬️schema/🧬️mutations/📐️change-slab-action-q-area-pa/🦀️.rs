//! `change-slab-action-q-area-pa` mutation leaf.

use crate::{En1994Mutation, En1994Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSlabActionQAreaPa {
    pub index: usize,
    pub action_index: usize,
    pub new_q_area_pa: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeSlabActionQAreaPa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "slab-action-q-area-pa", kind: "change-slab-action-q-area-pa", record: "ChangedSlabActionQAreaPa" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Result<Vec<En1994Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change imposed area load on the slab", "Flächenlast auf der Verbunddecke ändern")
    }
}
//#endregion 🔖️Payload
