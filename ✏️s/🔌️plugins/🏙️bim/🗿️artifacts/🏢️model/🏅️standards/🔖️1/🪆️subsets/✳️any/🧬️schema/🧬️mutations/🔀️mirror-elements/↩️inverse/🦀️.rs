//! ↩️ Inverse of `MirrorElements`, exact because it restores absolute base states and never mirrors back. As copies, one
//! `DeleteElements` of the minted ids. In place, one `PlaceElements` carrying the base placement of every changed element and the base
//! offset and flips of every changed opening (one diff, so no restored opening can collide with one that is still mirrored), and one
//! `SetWallEndJoin` per wall end whose join preference the mirror traded. Empty when the mirror is refused or changes nothing.

use crate::standards::v1::subsets::any::schema::mutations::delete_elements::DeleteElements;
use crate::standards::v1::subsets::any::schema::mutations::modify::{self, WallEnd};
use crate::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements;
use crate::standards::v1::subsets::any::schema::mutations::set_wall_end_join::SetWallEndJoin;
use crate::standards::v1::subsets::any::schema::mutations::mirror_elements::diff::map_of;
use crate::standards::v1::subsets::any::schema::mutations::mirror_elements::MirrorElements;
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &MirrorElements, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_elements::DeleteElements> + From<crate::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements> + From<crate::standards::v1::subsets::any::schema::mutations::set_wall_end_join::SetWallEndJoin>,
{
    let map = map_of(payload);
    if !map.finite() || map.degenerate() {
        return Vec::new();
    }
    if let Some(prefix) = &payload.prefix {
        return match modify::duplicate(base, &payload.ids, prefix, &[map]) {
            Ok(built) if !built.roots.is_empty() => vec![Op::from(DeleteElements { ids: built.roots })],
            _ => Vec::new(),
        };
    }
    let Ok(placed) = modify::in_place(base, &payload.ids, &map) else {
        return Vec::new();
    };
    let places = (!placed.before.is_empty()).then(|| Op::from(PlaceElements { placements: placed.before.clone() }));
    let joins = placed.joins.iter().flat_map(|(id, (start, end))| {
        let row = |end_of: WallEnd, join| Op::from(SetWallEndJoin { id: id.clone(), end: end_of, join });
        [row(WallEnd::Start, *start), row(WallEnd::End, *end)]
    });
    places.into_iter().chain(joins).collect()
}
