use crate::events::KitEvent;

#[test]
fn port_set_family_emits_field_changed() {
    let (kit, type_guid, port_g) = super::common::kit_with_port();
    let mut rx = kit.read().unwrap().subscribe();
    let p = {
        let kr = kit.read().unwrap();
        let t = kr.semio_type(type_guid.as_str()).unwrap();
        let tr = t.read().unwrap();
        tr.port(port_g.as_str()).unwrap().clone()
    };
    p.write().unwrap().set_family(Some("f".into()));
    let evs = super::common::drain(&mut rx);
    assert!(evs.iter().any(|e| matches!(e, KitEvent::FieldChanged { field: "family", .. })));
}
