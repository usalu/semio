//! 🗺️ `change-masonry-class` payload — changes the En1996 document's `masonry_class` (masonry manufacturing-control class).

use crate::artifacts::en1996::diff::En1996Diff;
use crate::artifacts::en1996::mutations::En1996Mutation;
use crate::artifacts::en1996::En1996Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeMasonryClass
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeMasonryClass {
    pub new_masonry_class: crate::artifacts::en1996::MasonryClass,
}

impl protocol::MutationKind<En1996Snapshot, En1996Mutation> for ChangeMasonryClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "masonry-class", kind: "change-masonry-class", record: "ChangedMasonryClass" };

    fn diff(&self, base: &En1996Snapshot) -> En1996Diff {
        crate::artifacts::en1996::mutations::change_masonry_class::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1996Snapshot) -> Vec<En1996Mutation> {
        crate::artifacts::en1996::mutations::change_masonry_class::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change masonry manufacturing-control class to {:?}", self.new_masonry_class)
    }
}
//#endregion 🔖️ChangeMasonryClass
