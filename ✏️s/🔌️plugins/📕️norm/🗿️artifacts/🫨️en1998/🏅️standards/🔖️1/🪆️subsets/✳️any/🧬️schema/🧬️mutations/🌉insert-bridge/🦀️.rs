//! 🌉 `insert-bridge` mutation leaf.

use crate::{En1998Mutation, En1998Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertBridge {
    pub index: Option<usize>,
    pub bridge: crate::En1998Bridge,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for InsertBridge {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "bridge",
        kind: "insert-bridge",
        record: "InsertBridge",
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
        semio_framework_ui_locale::LocalizedLabel::native("Insert bridge", "Brücke einfügen")
    }
    fn target(&self) -> Vec<String> {
        vec!["insert-bridge".into()]
    }
}
