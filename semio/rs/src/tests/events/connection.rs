use crate::events::KitEvent;

#[test]
fn connection_set_gap_triggers_flatten_and_validation() {
    let (kit, _, dg, _, _, cg) = super::common::kit_with_connection();
    let mut rx = kit.read().unwrap().subscribe();
    let c = {
        let kr = kit.read().unwrap();
        let d = kr.design(dg.as_str()).unwrap();
        let dr = d.read().unwrap();
        dr.connection(cg.as_str()).unwrap().clone()
    };
    c.write().unwrap().set_gap(Some(1.0));
    let evs = super::common::drain(&mut rx);
    assert!(evs.iter().any(|e| matches!(e, KitEvent::FieldChanged { field: "gap", .. })));
    assert!(evs.iter().any(|e| matches!(e, KitEvent::FlattenInvalidated { .. })));
    assert!(evs.iter().any(|e| matches!(e, KitEvent::ValidationInvalidated)));
}
