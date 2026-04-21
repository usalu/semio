use crate::attribute::AttributeFullDto;
use crate::events::KitEvent;
use crate::guid::Guid;
use crate::kit::{KitFullDto, KitStore};

#[test]
fn attribute_set_value_emits() {
    let g = Guid::new_v7();
    let kit = KitStore::from_full_dto(KitFullDto {
        guid: Guid::new_v7(),
        name: "k".into(),
        attributes: vec![AttributeFullDto {
            guid: g.clone(),
            key: "k".into(),
            value: "v".into(),
            definition: None,
        }],
        ..Default::default()
    });
    let mut rx = kit.read().unwrap().subscribe();
    kit.read().unwrap().attributes[0]
        .write()
        .unwrap()
        .set_value("v2".into());
    let evs = super::common::drain(&mut rx);
    assert!(evs.iter().any(|e| matches!(e, KitEvent::FieldChanged { field: "value", .. })));
}
