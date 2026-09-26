//! Inverse for `change-wind-face-assumed-wp`.
use super::ChangeWindFaceAssumedWp;
use crate::{En1991Mutation, En1991Snapshot};
pub fn inverse(payload: &ChangeWindFaceAssumedWp, base: &En1991Snapshot) -> Vec<En1991Mutation> {
    if payload.index >= base.wind_faces.len() { return Vec::new(); }
    vec![En1991Mutation::ChangeWindFaceAssumedWp(ChangeWindFaceAssumedWp { index: payload.index, new_assumed_wp: base.wind_faces[payload.index].assumed_wp })]
}
