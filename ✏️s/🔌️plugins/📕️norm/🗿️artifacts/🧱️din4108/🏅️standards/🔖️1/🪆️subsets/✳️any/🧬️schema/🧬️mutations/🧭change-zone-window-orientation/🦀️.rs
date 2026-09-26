//! `change-zone-window-orientation`.

use crate::{Din4108Mutation, Din4108Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeZoneWindowOrientation {
    pub zone_id: String,
    pub window_id: String,
    pub new_orientation: String,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeZoneWindowOrientation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "zone-window-orientation",
        kind: "change-zone-window-orientation",
        record: "ChangedZoneWindowOrientation",
    };
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-zone-window-orientation", "change-zone-window-orientation")
    }
}
