use crate::events::KitEvent;

#[test]
fn group_set_color_emits() {
    let (kit, dg, gg) = super::common::kit_with_group();
    let mut rx = kit.read().unwrap().subscribe();
    {
        let kr = kit.read().unwrap();
        let d = kr.design(dg.as_str()).unwrap();
        let g = d.read().unwrap().group(gg.as_str()).unwrap();
        g.write().unwrap().set_color(Some("#000".into()));
    }
    let evs = super::common::drain(&mut rx);
    assert!(evs.iter().any(|e| matches!(e, KitEvent::FieldChanged { field: "color", .. })));
}
