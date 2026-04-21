use crate::events::{EntityKind, EntityRef};
use crate::geom::{Camera, Coord, Location};

macro_rules! design_meta_test {
    ($fn:ident, $field:literal, $op:expr) => {
        #[test]
        fn $fn() {
            let (kit, _, dg, pg) = super::common::kit_with_piece();
            let dre = EntityRef::new(EntityKind::Design, dg.clone());
            let kre = super::common::kit_entity_ref(&kit);
            let mut rx = kit.read().unwrap().subscribe();
            let d = {
                let kr = kit.read().unwrap();
                kr.design(dg.as_str()).expect("design").clone()
            };
            let mut dw = d.write().unwrap();
            $op(&mut *dw);
            let evs = super::common::drain(&mut rx);
            super::common::assert_design_scalar_metadata_events(&evs, dre, kre, &pg, $field);
        }
    };
}

design_meta_test!(design_set_name, "name", |d: &mut crate::DesignStore| {
    d.set_name("x".into());
});
design_meta_test!(design_set_description, "description", |d: &mut crate::DesignStore| {
    d.set_description(Some("d".into()));
});
design_meta_test!(design_set_icon, "icon", |d: &mut crate::DesignStore| {
    d.set_icon(Some("i".into()));
});
design_meta_test!(design_set_image, "image", |d: &mut crate::DesignStore| {
    d.set_image(Some("m".into()));
});
design_meta_test!(design_set_variant, "variant", |d: &mut crate::DesignStore| {
    d.set_variant(Some("v".into()));
});
design_meta_test!(design_set_view, "view", |d: &mut crate::DesignStore| {
    d.set_view(Some("vw".into()));
});
design_meta_test!(design_set_location, "location", |d: &mut crate::DesignStore| {
    d.set_location(Some(Location::new(1.0, 2.0)));
});
design_meta_test!(design_set_camera, "camera", |d: &mut crate::DesignStore| {
    let mut cam = Camera::default();
    cam.position = Coord::new(0.0, 0.0, 1.0);
    d.set_camera(Some(cam));
});
design_meta_test!(design_set_unit, "unit", |d: &mut crate::DesignStore| {
    d.set_unit(Some("mm".into()));
});
design_meta_test!(design_set_created, "created", |d: &mut crate::DesignStore| {
    d.set_created(Some("c".into()));
});
design_meta_test!(design_set_updated, "updated", |d: &mut crate::DesignStore| {
    d.set_updated(Some("u".into()));
});
