use crate::diff::DesignDiff;
use crate::events::{EntityKind, EntityRef, KitEvent};
use crate::guid::Guid;
use crate::piece::PieceFullDto;
use crate::typ::TypeIdDto;

#[test]
fn apply_design_diff_add_piece_emits_child_added_and_hashes() {
    let (kit, tg, dg, _) = super::common::kit_with_piece();
    let new_piece = Guid::new_v7();
    let diff = DesignDiff {
        added_pieces: vec![PieceFullDto {
            guid: new_piece.clone(),
            r#type: Some(TypeIdDto {
                guid: tg.clone(),
            }),
            ..Default::default()
        }],
        ..Default::default()
    };
    let mut rx = kit.read().unwrap().subscribe();
    kit.write()
        .unwrap()
        .apply_design_diff(dg.as_str(), &diff)
        .unwrap();
    let evs = super::common::drain(&mut rx);
    let child = EntityRef::new(EntityKind::Piece, new_piece);
    assert!(evs.iter().any(|e| matches!(
        e,
        KitEvent::ChildAdded { child: c, .. } if *c == child
    )));
    assert!(evs.iter().filter(|e| matches!(e, KitEvent::HashInvalidated { .. })).count() >= 1);
}
