//! 🔧 `change-section-depth-mm` payload — changes the En1995 document's `section_depth_mm` (EN 1995 input).

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeSectionDepthMm
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSectionDepthMm {
    pub new_section_depth_mm: f64,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeSectionDepthMm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "section-depth-mm", kind: "change-section-depth-mm", record: "ChangedSectionDepthMm" };

    fn diff(&self, base: &En1995Snapshot) -> En1995Diff {
        crate::artifacts::en1995::mutations::change_section_depth_mm::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        crate::artifacts::en1995::mutations::change_section_depth_mm::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change section depth mm to {:?}", self.new_section_depth_mm)
    }
}
//#endregion 🔖️ChangeSectionDepthMm
