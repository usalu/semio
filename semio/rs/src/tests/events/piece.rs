use crate::events::{EntityKind, EntityRef};
use crate::geom::{Coord, Plane};

macro_rules! piece_geom_test {
    ($fn:ident, $field:literal, $op:expr) => {
        #[test]
        fn $fn() {
            let (kit, _, dg, pg) = super::common::kit_with_piece();
            let pre = EntityRef::new(EntityKind::Piece, pg.clone());
            let dre = EntityRef::new(EntityKind::Design, dg.clone());
            let kre = super::common::kit_entity_ref(&kit);
            let mut rx = kit.read().unwrap().subscribe();
            {
                let kr = kit.read().unwrap();
                let d = kr.design(dg.as_str()).unwrap();
                let p = d.read().unwrap().piece(pg.as_str()).unwrap();
                let mut pw = p.write().unwrap();
                $op(&mut *pw);
            }
            let evs = super::common::drain(&mut rx);
            super::common::assert_piece_geometry_change(&evs, pre, dre, kre, &pg, $field);
        }
    };
}

piece_geom_test!(piece_set_plane, "plane", |p: &mut crate::PieceStore| {
    p.set_plane(Some(Plane::world_xy()));
});
piece_geom_test!(piece_set_center, "center", |p: &mut crate::PieceStore| {
    p.set_center(Some(Coord::new(1.0, 2.0, 3.0)));
});
piece_geom_test!(piece_set_mirror_plane, "mirrorPlane", |p: &mut crate::PieceStore| {
    p.set_mirror_plane(Some(Plane::world_xy()));
});
piece_geom_test!(piece_set_scale, "scale", |p: &mut crate::PieceStore| {
    p.set_scale(Some(2.0));
});
piece_geom_test!(piece_set_hidden, "hidden", |p: &mut crate::PieceStore| {
    p.set_hidden(Some(true));
});
piece_geom_test!(piece_set_locked, "locked", |p: &mut crate::PieceStore| {
    p.set_locked(Some(true));
});
piece_geom_test!(piece_set_id, "id", |p: &mut crate::PieceStore| {
    p.set_id(Some("id1".into()));
});
piece_geom_test!(piece_set_name, "name", |p: &mut crate::PieceStore| {
    p.set_name(Some("p".into()));
});
piece_geom_test!(piece_set_description, "description", |p: &mut crate::PieceStore| {
    p.set_description(Some("pd".into()));
});

#[test]
fn piece_set_color_hash_only() {
    let (kit, _, dg, pg) = super::common::kit_with_piece();
    let pre = EntityRef::new(EntityKind::Piece, pg.clone());
    let dre = EntityRef::new(EntityKind::Design, dg.clone());
    let kre = super::common::kit_entity_ref(&kit);
    let mut rx = kit.read().unwrap().subscribe();
    {
        let kr = kit.read().unwrap();
        let d = kr.design(dg.as_str()).unwrap();
        let p = d.read().unwrap().piece(pg.as_str()).unwrap();
        let mut pw = p.write().unwrap();
        pw.set_color(Some("#fff".into()));
    }
    let evs = super::common::drain(&mut rx);
    super::common::assert_piece_scalar_hash_only(&evs, pre, dre, kre, "color");
}

#[test]
fn piece_set_type_weak_geometry() {
    let (kit, tg, dg, pg) = super::common::kit_with_piece();
    let pre = EntityRef::new(EntityKind::Piece, pg.clone());
    let dre = EntityRef::new(EntityKind::Design, dg.clone());
    let kre = super::common::kit_entity_ref(&kit);
    let mut rx = kit.read().unwrap().subscribe();
    let tw = kit
        .read()
        .unwrap()
        .semio_type(tg.as_str())
        .map(|t| std::sync::Arc::downgrade(&t))
        .unwrap();
    {
        let kr = kit.read().unwrap();
        let d = kr.design(dg.as_str()).unwrap();
        let p = d.read().unwrap().piece(pg.as_str()).unwrap();
        let mut pw = p.write().unwrap();
        pw.set_type_weak(Some(tw));
    }
    let evs = super::common::drain(&mut rx);
    super::common::assert_piece_geometry_change(&evs, pre, dre, kre, &pg, "type");
}
