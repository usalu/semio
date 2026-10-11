//! ⚖️ `change-storey-permanent-gk-n` mutation leaf.

use crate::{En1998Mutation, En1998Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeStoreyPermanentGkN {
    pub building_index: usize,
    pub storey_index: usize,
    pub new_permanent_gk_n: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeStoreyPermanentGkN {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "storey-permanent-gk-n",
        kind: "change-storey-permanent-gk-n",
        record: "ChangeStoreyPermanentGkN",
    };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<<En1998Mutation as protocol::Mutation<En1998Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1998Snapshot) -> Result<Vec<En1998Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Change permanent storey load", "Ständige Geschosslast ändern")
    }
    fn target(&self) -> Vec<String> {
        vec!["change-storey-permanent-gk-n".into()]
    }
}
