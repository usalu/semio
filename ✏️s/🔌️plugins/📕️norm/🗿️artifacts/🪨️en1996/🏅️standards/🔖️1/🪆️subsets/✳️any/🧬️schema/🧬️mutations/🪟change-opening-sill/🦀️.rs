//! 🪟change-opening-sill
use crate::{En1996Mutation, En1996Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeOpeningSill {
    pub wall_index: usize,
    pub index: usize,
    pub new_sill_height_m: f64,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeOpeningSill {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "opening-sill",
        kind: "change-opening-sill",
        record: "ChangedOpeningSill",
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
        semio_framework_ui_locale::LocalizedLabel::native("Change opening sill height", "Brüstungshöhe der Öffnung ändern")
    }
}

#[cfg(test)]
#[path = "🧪️tests/✅apply/🦀️.rs"]
mod named_test;
