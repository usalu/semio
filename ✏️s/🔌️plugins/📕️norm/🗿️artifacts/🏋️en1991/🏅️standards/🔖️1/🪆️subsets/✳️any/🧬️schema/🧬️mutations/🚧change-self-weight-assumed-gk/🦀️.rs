//! 🚧 `change-self-weight-assumed-gk`.

use crate::{En1991Mutation, En1991Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSelfWeightAssumedGk {
    pub index: usize,
    pub new_assumed_gk: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeSelfWeightAssumedGk {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "self-weight-assumed-gk",
        kind: "change-self-weight-assumed-gk",
        record: "ChangedSelfWeightAssumedGk",
    };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Result<Vec<En1991Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change assumed self-weight", "Angenommene Eigenlast ändern")
    }
}
//#endregion 🔖️Payload
