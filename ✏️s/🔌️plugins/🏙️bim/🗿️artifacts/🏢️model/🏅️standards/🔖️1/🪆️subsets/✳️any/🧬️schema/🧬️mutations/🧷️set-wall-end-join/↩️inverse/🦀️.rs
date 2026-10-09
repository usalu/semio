//! ↩️ Inverse of `SetWallEndJoin`: an absolute `SetWallEndJoin` back to the base preference of that end (absent when the end joined
//! automatically), none when the wall is absent.

use super::super::modify::WallEnd;
use super::SetWallEndJoin;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SetWallEndJoin, base: &ModelSnapshot) -> Vec<ModelMutation> {
    match base.walls.get(&payload.id) {
        Some(wall) => vec![ModelMutation::SetWallEndJoin(SetWallEndJoin { id: payload.id.clone(), end: payload.end, join: if payload.end == WallEnd::Start { wall.start_join } else { wall.end_join } })],
        None => Vec::new(),
    }
}
