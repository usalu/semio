//! ↩️ Inverse of `MirrorElements`, exact because it restores absolute base states and never mirrors back. As copies, one
//! `DeleteElements` of the minted ids. In place, one `PlaceElements` carrying the base placement of every changed element and the base
//! offset and flips of every changed opening (one diff, so no restored opening can collide with one that is still mirrored), and one
//! `SetWallEndJoin` per wall end whose join preference the mirror traded. Empty when the mirror is refused or changes nothing.

use super::super::delete_elements::DeleteElements;
use super::super::modify::{self, WallEnd};
use super::super::place_elements::PlaceElements;
use super::super::set_wall_end_join::SetWallEndJoin;
use super::diff::map_of;
use super::MirrorElements;
use crate::{ModelMutation, ModelSnapshot};

pub fn inverse(payload: &MirrorElements, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let map = map_of(payload);
    if !map.finite() || map.degenerate() {
        return Vec::new();
    }
    if let Some(prefix) = &payload.prefix {
        return match modify::duplicate(base, &payload.ids, prefix, &[map]) {
            Ok(built) if !built.roots.is_empty() => vec![ModelMutation::DeleteElements(DeleteElements { ids: built.roots })],
            _ => Vec::new(),
        };
    }
    let Ok(placed) = modify::in_place(base, &payload.ids, &map) else {
        return Vec::new();
    };
    let places = (!placed.before.is_empty()).then(|| ModelMutation::PlaceElements(PlaceElements { placements: placed.before.clone() }));
    let joins = placed.joins.iter().flat_map(|(id, (start, end))| {
        let row = |end_of: WallEnd, join| ModelMutation::SetWallEndJoin(SetWallEndJoin { id: id.clone(), end: end_of, join });
        [row(WallEnd::Start, *start), row(WallEnd::End, *end)]
    });
    places.into_iter().chain(joins).collect()
}
