use crate::events::KitEvent;

#[test]
fn connector_set_code_emits() {
    let (kit, tg, cg) = super::common::kit_with_type_connector();
    let mut rx = kit.read().unwrap().subscribe();
    {
        let kr = kit.read().unwrap();
        let t = kr.semio_type(tg.as_str()).unwrap();
        let c = t.read().unwrap().connector(cg.as_str()).unwrap();
        c.write().unwrap().set_code("C2".into());
    }
    let evs = super::common::drain(&mut rx);
    assert!(evs.iter().any(|e| matches!(e, KitEvent::FieldChanged { field: "code", .. })));
}
