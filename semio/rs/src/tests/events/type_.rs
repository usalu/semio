use crate::events::{EntityKind, EntityRef};

macro_rules! type_meta_test {
    ($fn:ident, $field:literal, $op:expr) => {
        #[test]
        fn $fn() {
            let (kit, tg) = super::common::kit_with_type_only();
            let tre = EntityRef::new(EntityKind::Type, tg.clone());
            let kre = super::common::kit_entity_ref(&kit);
            let mut rx = kit.read().unwrap().subscribe();
            {
                let kr = kit.read().unwrap();
                let t = kr.semio_type(tg.as_str()).unwrap();
                let mut tw = t.write().unwrap();
                $op(&mut *tw);
            }
            let evs = super::common::drain(&mut rx);
            super::common::assert_type_metadata_core(&evs, tre, kre, $field);
        }
    };
}

type_meta_test!(type_set_name, "name", |t: &mut crate::TypeStore| t.set_name("tn".into()));
type_meta_test!(type_set_description, "description", |t: &mut crate::TypeStore| {
    t.set_description(Some("td".into()));
});
type_meta_test!(type_set_icon, "icon", |t: &mut crate::TypeStore| {
    t.set_icon(Some("i".into()));
});
type_meta_test!(type_set_image, "image", |t: &mut crate::TypeStore| {
    t.set_image(Some("m".into()));
});
type_meta_test!(type_set_variant, "variant", |t: &mut crate::TypeStore| {
    t.set_variant(Some("v".into()));
});
type_meta_test!(type_set_stock, "stock", |t: &mut crate::TypeStore| {
    t.set_stock(Some(3));
});
type_meta_test!(type_set_virtual, "virtual", |t: &mut crate::TypeStore| {
    t.set_virtual(Some(true));
});
type_meta_test!(type_set_unit, "unit", |t: &mut crate::TypeStore| {
    t.set_unit(Some("u".into()));
});
type_meta_test!(type_set_location, "location", |t: &mut crate::TypeStore| {
    t.set_location(Some(crate::geom::Location::new(1.0, 2.0)));
});
type_meta_test!(type_set_created, "created", |t: &mut crate::TypeStore| {
    t.set_created(Some("c".into()));
});
type_meta_test!(type_set_updated, "updated", |t: &mut crate::TypeStore| {
    t.set_updated(Some("u".into()));
});
