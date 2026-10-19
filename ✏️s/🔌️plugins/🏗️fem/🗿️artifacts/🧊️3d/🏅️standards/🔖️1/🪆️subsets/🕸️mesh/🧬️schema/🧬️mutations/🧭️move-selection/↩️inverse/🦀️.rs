//! ↩️ Restores exactly the geometry field mask authored by the central forward diff.
use super::MoveSelection;
use crate::standards::v1::subsets::any::schema::mutations::change_node::ChangeNode;
use crate::standards::v1::subsets::any::schema::mutations::change_solid::ChangeSolid;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceNode> + From<ReplaceSolid>>(payload: &MoveSelection, base: &Fem3dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    let forward = super::diff::diff(payload, base);
    let inverse = protocol::DiffAlgebra::inverse(forward.diff(), base);
    let nodes = inverse.nodes.into_iter().flat_map(|delta| delta.modified).map(|row| Op::from(ChangeNode { id: row.id, patch: row.patch }));
    let solids = inverse.solids.into_iter().flat_map(|delta| delta.modified).map(|row| Op::from(ChangeSolid { id: row.id, patch: row.patch }));
    let mut rows: Vec<_> = nodes.chain(solids).collect();
    rows.reverse();
    Ok(rows)
}
//#endregion 🔖️Inverse
