use crate::remove_frame_node::RemoveFrameNode;
use crate::insert_frame_node;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &RemoveFrameNode, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_frame_node::InsertFrameNode> {
    Ok((|| {
    if payload.index >= base.frame_nodes.len() { return Vec::new(); }
    vec![Op::from(insert_frame_node::InsertFrameNode { index: Some(payload.index), frame_node: base.frame_nodes[payload.index].clone() })]

    })())
}
