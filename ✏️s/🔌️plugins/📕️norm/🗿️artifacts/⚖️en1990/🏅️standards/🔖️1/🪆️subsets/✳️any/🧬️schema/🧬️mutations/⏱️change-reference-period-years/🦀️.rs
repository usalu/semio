//! `change-reference-period-years` mutation for EN 1990.

use crate::{En1990Mutation, En1990Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeReferencePeriodYears {
    pub new_reference_period_years: f64,
}

impl protocol::MutationKind<En1990Snapshot, En1990Mutation> for ChangeReferencePeriodYears {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "reference-period-years",
        kind: "change-reference-period-years",
        record: "ChangedReferencePeriodYears",
    };

    fn diff(&self, base: &En1990Snapshot) -> protocol::MutationOutcome<<En1990Mutation as protocol::Mutation<En1990Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1990Snapshot) -> Vec<En1990Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("Change reference-period-years", "Ändern: reference-period-years")
    }
}
//#endregion 🔖️Payload
