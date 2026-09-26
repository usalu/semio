//! 🏔 `change-exceptional-snow-north-german-lowlands`.

use crate::{En1991Mutation, En1991Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeExceptionalSnowNorthGermanLowlands {
    pub new_exceptional_snow_north_german_lowlands: bool,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeExceptionalSnowNorthGermanLowlands {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "exceptional-snow-north-german-lowlands",
        kind: "change-exceptional-snow-north-german-lowlands",
        record: "ChangedExceptionalSnowNorthGermanLowlands",
    };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-exceptional-snow-north-german-lowlands", "change-exceptional-snow-north-german-lowlands")
    }
}
//#endregion 🔖️Payload
