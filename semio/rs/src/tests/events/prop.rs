use crate::events::KitEvent;
use crate::guid::Guid;
use crate::kit::{KitFullDto, KitStore};
use crate::prop::PropFullDto;

#[test]
fn prop_set_unit_emits() {
    let g = Guid::new_v7();
    let kit = KitStore::from_full_dto(KitFullDto {
        guid: Guid::new_v7(),
        name: "k".into(),
        props: vec![PropFullDto {
            guid: g.clone(),
            key: "k".into(),
            value: "v".into(),
            unit: None,
        }],
        ..Default::default()
    });
    let mut rx = kit.read().unwrap().subscribe();
    let p = {
        let kr = kit.read().unwrap();
        kr.props[0].clone()
    };
    p.write().unwrap().set_unit(Some("u".into()));
    let evs = super::common::drain(&mut rx);
    assert!(evs.iter().any(|e| matches!(e, KitEvent::FieldChanged { field: "unit", .. })));
}
