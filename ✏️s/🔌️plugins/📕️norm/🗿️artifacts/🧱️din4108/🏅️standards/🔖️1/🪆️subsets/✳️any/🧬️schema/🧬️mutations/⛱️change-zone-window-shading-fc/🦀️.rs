//! ⛱️ `change-zone-window-shading-fc`.

use crate::{Din4108Mutation, Din4108Snapshot};

#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeZoneWindowShadingFc {
    pub zone_id: String,
    pub window_id: String,
    pub new_shading_fc: f64,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeZoneWindowShadingFc {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor {
        verb: "change",
        entity: "zone-window-shading-fc",
        kind: "change-zone-window-shading-fc",
        record: "ChangedZoneWindowShadingFc",
    };
    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> protocol::LocalizedLabel {
        protocol::LocalizedLabel::native("change-zone-window-shading-fc", "change-zone-window-shading-fc")
    }
}
