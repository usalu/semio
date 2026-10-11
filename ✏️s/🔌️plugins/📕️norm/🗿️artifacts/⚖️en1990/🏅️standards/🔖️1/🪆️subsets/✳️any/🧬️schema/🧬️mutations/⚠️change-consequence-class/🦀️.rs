//! `change-consequence-class` mutation for EN 1990.

use crate::{En1990Mutation, En1990Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeConsequenceClass {
    pub new_consequence_class: u8,
}

impl protocol::MutationKind<En1990Snapshot, En1990Mutation> for ChangeConsequenceClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "consequence-class",
        kind: "change-consequence-class",
        record: "ChangedConsequenceClass",
    };

    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<En1990Mutation as protocol::Mutation<En1990Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Result<Vec<En1990Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change consequence class", "Schadensfolgeklasse ändern")
    }
}
//#endregion 🔖️Payload
