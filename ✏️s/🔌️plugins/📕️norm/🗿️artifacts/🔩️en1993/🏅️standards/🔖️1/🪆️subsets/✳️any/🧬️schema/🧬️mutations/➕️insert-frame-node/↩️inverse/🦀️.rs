use crate::insert_frame_node::InsertFrameNode;
use crate::remove_frame_node;
use semio_s_artifact_norm_en1993_model::En1993Snapshot;
pub fn inverse<Op>(payload: &InsertFrameNode, base: &En1993Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_frame_node::RemoveFrameNode> {
    Ok((|| {
    let at = payload.index.unwrap_or(usize::MAX).min(base.frame_nodes.len());
    vec![Op::from(remove_frame_node::RemoveFrameNode { index: at })]

    })())
}
