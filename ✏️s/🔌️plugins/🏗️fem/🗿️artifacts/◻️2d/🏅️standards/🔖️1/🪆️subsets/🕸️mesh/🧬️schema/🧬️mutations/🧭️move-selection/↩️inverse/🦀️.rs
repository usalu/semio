//! ↩️ Inverse for `MoveSelection` — the whole-record replacements restoring every BASE node and region the
//! transform moves (exact, never an inverted transform that would accumulate float error). Nothing moved ⇒
//! `Vec::new()`.
use super::MoveSelection;
use semio_s_artifact_fem_2d_mesh_topology::standards::v1::subsets::any::schema::mutations::replace_node::ReplaceNode;
use crate::standards::v1::subsets::any::schema::mutations::replace_region::ReplaceRegion;

use semio_s_artifact_fem_2d_model::standards::v1::subsets::any::schema::snapshot::Fem2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ReplaceNode> + From<ReplaceRegion>>(payload: &MoveSelection, base: &Fem2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    if payload.breach().is_some() {
        return Ok(Vec::new());
    }
    let nodes = base.nodes.iter().filter(|node| payload.node_ids.contains(&node.id) && payload.moved_node(node).is_some()).map(|node| Op::from(ReplaceNode { id: node.id.clone(), new_node: node.clone() }));
    let regions = base.regions.iter().filter(|region| payload.region_ids.contains(&region.id) && payload.moved_region(region).is_some()).map(|region| Op::from(ReplaceRegion { id: region.id.clone(), new_region: region.clone() }));
    Ok(nodes.chain(regions).collect())
}
//#endregion 🔖️Inverse
