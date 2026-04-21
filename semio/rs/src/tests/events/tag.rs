use crate::events::KitEvent;
use crate::guid::Guid;
use crate::kit::{KitFullDto, KitStore};
use crate::tag::TagFullDto;

#[test]
fn tag_set_order_emits() {
    let g = Guid::new_v7();
    let kit = KitStore::from_full_dto(KitFullDto {
        guid: Guid::new_v7(),
        name: "k".into(),
        tags: vec![TagFullDto {
            guid: g.clone(),
            name: "t".into(),
            order: None,
        }],
        ..Default::default()
    });
    let mut rx = kit.read().unwrap().subscribe();
    let t = {
        let kr = kit.read().unwrap();
        kr.tags[0].clone()
    };
    t.write().unwrap().set_order(Some(1));
    let evs = super::common::drain(&mut rx);
    assert!(evs.iter().any(|e| matches!(e, KitEvent::FieldChanged { field: "order", .. })));
}
