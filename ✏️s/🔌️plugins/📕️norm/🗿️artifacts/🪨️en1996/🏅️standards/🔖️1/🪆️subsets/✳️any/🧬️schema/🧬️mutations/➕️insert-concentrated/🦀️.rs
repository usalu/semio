//! ➕️insert-concentrated
use crate::{En1996Mutation, En1996Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct InsertConcentrated {
    pub wall_index: usize,
    pub load_case_index: usize,
    pub index: usize,
    pub load: crate::ConcentratedLoad,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for InsertConcentrated {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "insert",
        entity: "concentrated",
        kind: "insert-concentrated",
        record: "InsertedConcentrated",
    };
    fn diff(&self, base: &En1996Snapshot) -> protocol::MutationOutcome<<En1996Mutation as protocol::Mutation<En1996Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1996Snapshot) -> Result<Vec<En1996Mutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert concentrated action", "Einzellast einfügen")
    }
}

#[cfg(test)]
#[path = "🧪️tests/✅apply/🦀️.rs"]
mod named_test;
