//! ↩️ Exact sparse restoration from the central forward diff's inverse, preserving independent peer fields.
use super::MoveSelection;
use crate::standards::v1::subsets::any::schema::mutations::change_node::ChangeNode;
use semio_s_artifact_fem_2d_mesh_regions::standards::v1::subsets::any::schema::mutations::change_region::ChangeRegion;
use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshotValue;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeNode> + From<ChangeRegion>>(payload: &MoveSelection, base: &Fem2dSnapshotValue) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    let outcome = super::diff::diff(payload, base);
    let negative = protocol::ColdDiffAlgebra::inverse(outcome.diff(), base);
    let nodes = negative.nodes.into_iter().flat_map(|delta| delta.modified).map(|row| Op::from(ChangeNode { id: row.id, patch: row.patch }));
    let regions = negative.regions.into_iter().flat_map(|delta| delta.modified).map(|row| Op::from(ChangeRegion { id: row.id, patch: row.patch }));
    let mut rows: Vec<_> = nodes.chain(regions).collect();
    rows.reverse();
    Ok(rows)
}
//#endregion 🔖️Inverse
