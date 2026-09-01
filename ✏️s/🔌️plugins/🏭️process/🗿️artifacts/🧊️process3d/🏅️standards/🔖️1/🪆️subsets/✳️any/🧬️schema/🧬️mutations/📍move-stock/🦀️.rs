//! 🧱 Process3d mutation — `MoveStock` (repurposes the pre-migration `🧱set-stock/` triad dir —
//! glue.rs path-includes this exact directory outside this facet's writable boundary, so the
//! directory name stays `🧱set-stock`; see the migration report's `sharedFileRequests` for the
//! rename once a later pass can touch `📦️glue.rs`).

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::move_stock::MoveStock;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::{Pose, Process3dSnapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️MoveStock
/// 🧱 Absolute spatial reposition of the document's single [`crate::artifacts::process3d::Stock`]
/// workpiece — the `stock` field's `pose` sub-value, addressed implicitly (the document has exactly
/// one stock, so `target()` is empty per `MutationKind::target`'s whole-artifact-scope default).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct MoveStock {
    pub new_pose: Pose,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for MoveStock {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "stock", kind: "move-stock", record: "MovedStock" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        "Move stock".to_string()
    }
}
//#endregion 🔖️MoveStock
