use crate::events::KitEvent;

#[test]
fn layer_set_order_emits() {
    let (kit, dg, lg) = super::common::kit_with_layer();
    let mut rx = kit.read().unwrap().subscribe();
    let l = {
        let kr = kit.read().unwrap();
        let d = kr.design(dg.as_str()).unwrap();
        let dr = d.read().unwrap();
        dr.layer(lg.as_str()).unwrap().clone()
    };
    l.write().unwrap().set_order(Some(2));
    let evs = super::common::drain(&mut rx);
    assert!(evs.iter().any(|e| matches!(e, KitEvent::FieldChanged { field: "order", .. })));
}
