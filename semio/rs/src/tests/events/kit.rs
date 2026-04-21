macro_rules! kit_meta_test {
    ($fn:ident, $field:literal, $op:expr) => {
        #[test]
        fn $fn() {
            let kit = std::sync::Arc::new(std::sync::RwLock::new(crate::KitStore::new("i")));
            let kref = super::common::kit_entity_ref(&kit);
            let mut rx = kit.read().unwrap().subscribe();
            {
                let mut g = kit.write().unwrap();
                $op(&mut *g);
            }
            let evs = super::common::drain(&mut rx);
            super::common::assert_kit_metadata_core(&evs, kref, $field);
        }
    };
}

kit_meta_test!(kit_set_name, "name", |k: &mut crate::KitStore| {
    k.set_name("a".into());
});
kit_meta_test!(kit_set_description, "description", |k: &mut crate::KitStore| {
    k.set_description(Some("d".into()));
});
kit_meta_test!(kit_set_icon, "icon", |k: &mut crate::KitStore| {
    k.set_icon(Some("ic".into()));
});
kit_meta_test!(kit_set_image, "image", |k: &mut crate::KitStore| {
    k.set_image(Some("im".into()));
});
kit_meta_test!(kit_set_preview, "preview", |k: &mut crate::KitStore| {
    k.set_preview(Some("pr".into()));
});
kit_meta_test!(kit_set_version, "version", |k: &mut crate::KitStore| {
    k.set_version(Some("1".into()));
});
kit_meta_test!(kit_set_remote, "remote", |k: &mut crate::KitStore| {
    k.set_remote(Some("r".into()));
});
kit_meta_test!(kit_set_homepage, "homepage", |k: &mut crate::KitStore| {
    k.set_homepage(Some("h".into()));
});
kit_meta_test!(kit_set_license, "license", |k: &mut crate::KitStore| {
    k.set_license(Some("l".into()));
});
kit_meta_test!(kit_set_uri, "uri", |k: &mut crate::KitStore| {
    k.set_uri(Some("u".into()));
});
kit_meta_test!(kit_set_created, "created", |k: &mut crate::KitStore| {
    k.set_created(Some("c".into()));
});
kit_meta_test!(kit_set_updated, "updated", |k: &mut crate::KitStore| {
    k.set_updated(Some("u2".into()));
});

#[test]
fn kit_set_name_idempotent_no_events() {
    let kit = std::sync::Arc::new(std::sync::RwLock::new(crate::KitStore::new("same")));
    let mut rx = kit.read().unwrap().subscribe();
    kit.write().unwrap().set_name("same".into());
    assert!(super::common::drain(&mut rx).is_empty());
}
