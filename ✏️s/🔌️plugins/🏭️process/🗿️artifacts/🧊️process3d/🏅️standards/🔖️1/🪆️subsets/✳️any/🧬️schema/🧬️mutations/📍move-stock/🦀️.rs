//! 🧱 Process3d mutation — `MoveStock` (repurposes the pre-migration `🧱set-stock/` triad dir —
//! glue.rs path-includes this exact directory outside this facet's writable boundary, so the
//! directory name stays `🧱set-stock`; see the migration report's `sharedFileRequests` for the
//! rename once a later pass can touch `🦀️.rs`).

use semio_s_artifact_process_process3d_model_diff::Process3dDiff;
use semio_s_artifact_process_process3d_model::{snapshot::Pose, snapshot::Process3dSnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️MoveStock
/// 🧱 Absolute spatial reposition of the document's single [`crate::Stock`]
/// workpiece — the `stock` field's `pose` sub-value, addressed implicitly (the document has exactly
/// one stock, so `target()` is empty per `MutationKind::target`'s whole-artifact-scope default).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct MoveStock {
    pub new_pose: Pose,
}

impl<Op> protocol::MutationKind<Process3dSnapshot, Op> for MoveStock
where
    Op: protocol::Mutation<Process3dSnapshot, Diff = semio_s_artifact_process_process3d_model_diff::Process3dDiff> + From<crate::move_stock::MoveStock>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "stock", kind: "move-stock", record: "MovedStock" };

    fn diff(&self, base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Process3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Move stock", "Rohteil verschieben")
    }
}
//#endregion 🔖️MoveStock
