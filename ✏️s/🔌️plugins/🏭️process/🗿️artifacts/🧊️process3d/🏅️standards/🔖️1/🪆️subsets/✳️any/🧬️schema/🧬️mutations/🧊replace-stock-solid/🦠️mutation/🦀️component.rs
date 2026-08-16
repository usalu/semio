//! 🔁 `replace-stock-solid` payload — whole-value swap of the document's composed
//! `s.stdio.semio.brep` stock-solid CHILD HANDLE.
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `new_solid` used to carry a
//! literal `SolidSpec` (now deleted — duplicated `brep` content); it now carries the real, already-
//! minted `store::ArtifactChild<SemioBrepSnapshot>` HANDLE the caller mints via
//! `crate::artifacts::process3d::brep_child_handle` from real content (e.g.
//! `brep_snapshot_for_working_solid`). This stays a REAL mutation (unlike the step-content triads)
//! because it is a pure handle SWAP — no read of the child's prior content is needed to compute
//! either the forward diff or the inverse (both only ever touch `stock_solid` on `base`/the payload,
//! never the child's resolved content).

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::Process3dMutation;
use crate::artifacts::process3d::Process3dSnapshot;
use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ReplaceStockSolid
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceStockSolid {
    pub new_solid: store::ArtifactChild<SemioBrepSnapshot>,
}

impl protocol::MutationKind<Process3dSnapshot, Process3dMutation> for ReplaceStockSolid {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "replace", entity: "stock", kind: "replace-stock-solid", record: "ReplacedStockSolid" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        crate::artifacts::process3d::mutations::replace_stock_solid::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Vec<Process3dMutation> {
        crate::artifacts::process3d::mutations::replace_stock_solid::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        "Replace stock solid".to_string()
    }
}
//#endregion 🔖️ReplaceStockSolid
